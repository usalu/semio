"""rustfmt parse check of every staged/installed file, emoji admission (RGI via bun, folder prefix use via git ls-files names), slug shape, and the rustfmt diff-count comparison against the baseline."""
import json, os, re, subprocess, sys, collections
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
from spec import SPEC
import subprocess as _sp
D = f"{W}/dirnames.txt"
if not os.path.exists(D):
    _data = _sp.run(["git", "-c", "core.quotepath=false", "ls-files", "-z"], cwd=ROOT, capture_output=True).stdout.decode("utf-8", "replace").split("\0")
    _dirs = set()
    for _p in _data:
        _dirs.update(_p.split("/")[:-1])
    write(D, "\n".join(sorted(_dirs)))

which = sys.argv[1] if len(sys.argv) > 1 else "stage"
names = open(D, encoding="utf-8").read().split("\n")
pref = collections.Counter()
for n in names:
    i = 0
    while i < len(n) and ord(n[i]) > 127: i += 1
    if i: pref[n[:i]] += 1
allem = sorted({c["emoji"] for s in SPEC.values() for c in s["children"]})
js = "const c=" + json.dumps(allem, ensure_ascii=False) + ";for(const e of c){console.log(JSON.stringify([e,/^\\p{RGI_Emoji}$/v.test(e.replace(/\\uFE0F/g,''))||/^\\p{RGI_Emoji}$/v.test(e)]))}"
rgi = {}
for line in subprocess.run(["bun", "-e", js], capture_output=True, text=True).stdout.strip().split("\n"):
    e, ok = json.loads(line); rgi[e] = ok
bad = 0
print("emoji admission:")
for e in allem:
    key = e if e.endswith("️") else e + "️"
    n = pref.get(key, 0)
    ok = rgi[e] and n > 0 and e.endswith("️")
    if not ok: bad += 1
    print(f"  {e} rgi={rgi[e]} folder-prefix-uses-in-repo={n} vs16={e.endswith(chr(0xfe0f))} {'OK' if ok else 'BAD'}")
for name, s in SPEC.items():
    fs = [c["folder"] for c in s["children"]]
    for c in s["children"]:
        if not re.fullmatch(re.escape(c["emoji"]) + r"[a-z0-9]+(-[a-z0-9]+)*", c["folder"]):
            print("BAD folder shape", c["folder"]); bad += 1
    sibs = set(os.listdir(f"{ROOT}/{TARGETS[name]['pkgdir']}"))
    for f in fs:
        if f in sibs and which == "stage":
            print("folder already exists in real tree", f); bad += 1
print("emoji/shape problems:", bad)
print("rustfmt parse (nightly-2026-07-20, repo rustfmt.toml, skip_children) and diff counts:")
tot_bad = 0
for name, s in SPEC.items():
    t = TARGETS[name]
    base = f"{W}/{name}/stage/{t['pkgdir']}" if which == "stage" else f"{ROOT}/{t['pkgdir']}"
    snap = f"{W}/{name}/snap/{t['file']}"
    def run(p):
        r = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--config-path", f"{ROOT}/rustfmt.toml", "--edition", "2021", "--check", "--config", "skip_children=true", p], capture_output=True, text=True)
        return r.returncode, r.stdout.count("Diff in "), r.stderr
    rc0, d0, e0 = run(snap)
    parse_fail = 0; diffs = 0
    files = [f"{base}/🦀️.rs"] + [f"{base}/{c['folder']}/🦀️.rs" for c in s["children"]]
    for p in files:
        rc, d, e = run(p)
        diffs += d
        if rc not in (0, 1) or "error" in e.lower():
            parse_fail += 1; print("  PARSE FAIL", p.split("/rust/")[-1], e[:200])
    tot_bad += parse_fail
    print(f"  {name}: baseline diffs {d0} (rc {rc0}); new total diffs {diffs}; files {len(files)}; parse failures {parse_fail}")
print("parse failures total:", tot_bad)
