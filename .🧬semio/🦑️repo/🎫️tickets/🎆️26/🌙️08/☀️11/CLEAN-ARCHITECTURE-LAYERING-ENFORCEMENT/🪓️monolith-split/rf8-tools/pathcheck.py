"""Path check on the live tree: every #[path]/include! target of each split crate exists, each new child is reachable exactly once,
repo-relative paths stay within 240 UTF-8 bytes, children are within 1500 lines, each starts with an emoji docstring, regions balance per file."""
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(__file__))
from frslib import *
from splitter import crate_files, mask_keep_strings

ok = True
rows = []
emoji_first = re.compile(r"^//[!/]? (\S)")
for name, t in TARGETS.items():
    if not os.path.exists(f"{W}/{name}/installed.json") or (len(sys.argv) > 1 and name not in sys.argv[1:]):
        continue
    inst = f"{W}/{name}/installed.json"
    new = json.loads(read(inst))["files"] if os.path.exists(inst) else []
    lib = ROOT + "/" + t["lib"]
    reach = [os.path.relpath(p, ROOT) for p in crate_files(lib)]
    reach_set = collections.Counter(reach)
    missing = []
    from splitter import path_mounts
    for p in crate_files(lib):
        for kind, target in path_mounts(read(p), os.path.dirname(p)):
            if not os.path.isfile(target):
                missing.append((os.path.relpath(p, ROOT), kind, os.path.relpath(target, ROOT)))
    docs = {}
    for rel in new:
        full = ROOT + "/" + rel
        lines = lines_of(read(full))
        bl = len(rel.encode("utf-8"))
        n_open = sum(1 for l in lines if re.match(r"^\s*//\s?#region ", l))
        n_close = sum(1 for l in lines if re.match(r"^\s*//\s?#endregion ", l))
        fm = emoji_first.match(lines[0]) if lines else None
        first = fm.group(1) if fm else None
        rows.append(dict(target=name, file=rel, lines=len(lines), bytes=bl, regions=(n_open, n_close), first=first, reachable=reach_set.get(rel, 0)))
        if rel.endswith("🦀️.rs") and rel != t["file"]:
            docs.setdefault(first, []).append(rel)
    bad = [r for r in rows if r["target"] == name and (r["bytes"] > 240 or r["lines"] > 1500 or r["regions"][0] != r["regions"][1] or r["reachable"] != 1 or (r["first"] is None and r["file"] != t["file"]))]
    dup = {k: v for k, v in docs.items() if len(v) > 1}
    print(f"[{name}] new files={len(new)} missing path targets={missing} bad={[(b['file'].split('/')[-2], b) for b in bad]} duplicate-first-emoji={dup}")
    if missing or bad:
        ok = False
for r in rows:
    print(f"{r['target']:8} {r['lines']:5} lines {r['bytes']:3} bytes regions={r['regions']} first={r['first']} reachable={r['reachable']} {r['file'].split('/')[-2]}")
print("PATHCHECK", "OK" if ok else "FAILED")
