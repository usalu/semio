"""📥 c4p29 installer: copies the staged files of the given targets into the repo.
Before every target it re-reads the live root file and asserts it is byte-identical to the pre-edit blob (a peer touching it aborts the install); children are created only when absent.
usage: c4pinstall.py [key...]   (default: all)    journal: 🗑️generated/c4p29-installed.json"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4plib import *
from plans import PLANS, ORDER

keys = sys.argv[1:] or ORDER
jp = GEN + "/c4p29-installed.json"
journal = json.load(open(jp, encoding="utf-8")) if os.path.exists(jp) else {}
for key in keys:
    spec = PLANS[key]
    sd = STAGE
    base_dir = os.path.dirname(spec["rel"])
    staged = [spec["rel"]] + [f"{base_dir}/{c['folder']}/🦀️.rs" for c in spec["children"]]
    allowed = {os.path.abspath(ROOT + "/" + r) for r in staged}
    live_root = ROOT + "/" + spec["rel"]
    live = read(live_root)
    base = base_text(spec["rel"])
    if key in journal:
        print(f"[{key}] already installed, skipped")
        continue
    assert live == base, f"{key}: live root differs from the pre-edit blob; refusing"
    os.makedirs(WORK + "/base", exist_ok=True)
    write(f"{WORK}/base/{key}.rs", base)
    children = [r for r in staged if r != spec["rel"]]
    for r in sorted(children):
        p = ROOT + "/" + r
        assert not os.path.exists(p), f"{key}: child exists {r}"
        assert len(r.encode("utf-8")) <= 240, (r, len(r.encode("utf-8")))
        repo_write(p, read(f"{sd}/{r}"), allowed)
    repo_write(live_root, read(f"{sd}/{spec['rel']}"), allowed)
    journal[key] = dict(root=spec["rel"], base_sha=sha(base), children=sorted(children), files=sorted(staged), installed_sha={r: sha(read(ROOT + "/" + r)) for r in staged})
    json.dump(journal, open(jp, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(f"[{key}] installed root + {len(children)} children")
