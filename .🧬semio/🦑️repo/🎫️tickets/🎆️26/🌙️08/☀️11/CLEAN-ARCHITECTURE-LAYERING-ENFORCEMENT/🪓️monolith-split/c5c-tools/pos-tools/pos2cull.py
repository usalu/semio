"""🔦 POS2 culprit finder for the rustc early stop. Cause (found by micro-tests, section 4 of RESULT-PRIVACY-ORACLE-SWEEP-2): a trait path whose resolution is `Res::Err` in a bound, supertrait, `dyn`, `impl Trait` or impl header makes rustc raise a fatal error before body type-check.
This tool takes the sandbox of a finished run (`pos2-work/<family>/<side>`), enumerates every trait-position path of every file (posstub.trait_position_spans), checks that rewriting all of them to a resolvable stub trait reaches the canary and then delta-debugs the minimal set of
paths whose rewrite does. Writes only inside the sandbox (originals restored at the end) and GEN/pos2-<family>-cull-*.txt.
usage: pos2cull.py <family.json> <side> <cfg> [--max-runs N]"""
import json, os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import posunion as U
from poslib import *
from posstub import mask, trait_position_spans
from pos import CFGS, parse_json

spec = json.load(open(sys.argv[1], encoding="utf-8"))
side, cfg = sys.argv[2], sys.argv[3]
maxruns = int(sys.argv[sys.argv.index("--max-runs") + 1]) if "--max-runs" in sys.argv else 80
mans = Manifests(Source(None))
mans.build_index()
pk = spec["crates"] if side == "cur" else spec["base"]["crates"]
src = Source(None) if side == "cur" else Source(spec["base"]["rev"])
crates = U.prepare(src, mans, pk, False)
en = U.feature_closure(crates, "test" in cfg, mans)
for c in crates:
    c.features = en[c.pkg]
d = f"{SANDBOX_BASE}/{spec['name']}/{side}"
sb = Sandbox(SANDBOX_BASE)
TR_DEF = "\npub mod __cull { pub trait Tr<A0 = (), A1 = (), A2 = (), A3 = (), A4 = (), A5 = ()> {} }\n"
files = {}
for c in crates:
    for rel in c.code:
        p = os.path.join(d, c.mod, rel)
        if os.path.exists(p):
            files[p] = open(p, encoding="utf-8").read()
root = os.path.join(d, "__pos_root.rs")
root_text = open(root, encoding="utf-8").read()
if "pub mod __cull" not in root_text:
    sb.write(root, root_text + TR_DEF)
    root_text += TR_DEF
cands = []
for p, tx in files.items():
    m = mask(tx)
    for s, e, path in trait_position_spans(m):
        pt = tx[s:e].strip()
        if not pt or not re.match(r"^(::)?[A-Za-z_]", pt):
            continue
        head = pt.split("<")[0].split("::")[-1].strip()
        if head in ("Sized", "Send", "Sync", "Copy", "Clone", "Debug", "Default", "Eq", "PartialEq", "Ord", "PartialOrd", "Hash", "Fn", "FnMut", "FnOnce", "Iterator", "Drop", "Into", "From", "AsRef", "ToString", "Unpin", "Future", "IntoIterator", "Display", "Error", "Any", "Borrow"):
            continue
        cands.append((p, s, e, pt))
print("candidates", len(cands), "files", len({c[0] for c in cands}), flush=True)
runs = [0]


def run_with(sel):
    by = {}
    for i in sel:
        p, s, e, pt = cands[i]
        by.setdefault(p, []).append((s, e))
    for p, tx in files.items():
        if p in by:
            out, last = [], 0
            for s, e in sorted(by[p]):
                if s < last:
                    continue
                seg = tx[s:e]
                lead = len(seg) - len(seg.lstrip())
                out.append(tx[last:s + lead])
                out.append("crate::__cull::Tr")
                last = s + lead + len(seg.strip())
            out.append(tx[last:])
            sb.write(p, "".join(out))
        else:
            sb.write(p, tx)
    out_ = d + "/__out_cull.rmeta"
    sb.check(out_)
    cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", "pos_union", "--emit=metadata", "-o", out_, "--cap-lints", "allow", "--error-format=json", "-Ztrim-diagnostic-paths=no", *U.cfg_args(crates), *CFGS[cfg], "__pos_root.rs"]
    pr = subprocess.run(cmd, capture_output=True, text=True, cwd=d)
    if os.path.exists(out_):
        os.remove(out_)
    runs[0] += 1
    _, rc = parse_json(pr.stderr, d, frozenset())
    return {"E0616", "E0624", "E0308"} <= rc


def restore():
    for p, tx in files.items():
        sb.write(p, tx)


allidx = list(range(len(cands)))
if not run_with(allidx):
    restore()
    print("NOT-A-TRAIT-POSITION-CAUSE: rewriting all", len(cands), "trait paths does not reach the canary")
    sys.exit(2)
S = allidx
n = 2
while len(S) >= 2 and runs[0] < maxruns:
    chunk = max(1, len(S) // n)
    subs = [S[i:i + chunk] for i in range(0, len(S), chunk)]
    red = False
    for sub in subs:
        if runs[0] >= maxruns:
            break
        if run_with(sub):
            S, n, red = sub, 2, True
            break
    if not red and runs[0] < maxruns:
        for sub in subs:
            comp = [x for x in S if x not in sub]
            if comp and runs[0] < maxruns and run_with(comp):
                S, n, red = comp, max(n - 1, 2), True
                break
    if not red:
        if n >= len(S):
            break
        n = min(len(S), n * 2)
    print("candidate set", len(S), "runs", runs[0], flush=True)
restore()
lines = []
for i in S:
    p, s, e, pt = cands[i]
    tx = files[p]
    ln = tx.count("\n", 0, s) + 1
    lines.append(f"{os.path.relpath(p, d)[-120:]}:{ln}: path `{pt}` :: {tx.splitlines()[ln - 1].strip()[:160]}")
log_write(f"{GEN}/{PREFIX}-{spec['name']}-cull-{side}-{cfg}.txt", "\n".join(lines) + "\n")
print("\n".join(lines))
print("minimal set", len(S), "runs", runs[0])
