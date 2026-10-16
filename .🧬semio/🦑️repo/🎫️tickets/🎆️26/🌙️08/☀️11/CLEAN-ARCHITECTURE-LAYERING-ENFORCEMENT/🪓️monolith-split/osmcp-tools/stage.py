import sys, json
sys.path.insert(0, ".")
from oracle2 import *

def stage(planmod, out_json, crates_json):
    lay, em, files = generate(planmod, False)
    # facade etc. handled in files (lib roots). Manifests are not source-counted.
    json.dump(files, open(out_json, "w"), ensure_ascii=False)
    crates = []
    for ci, n in enumerate(lay.names):
        nd = node_dir(lay, ci)
        entry = os.path.join(nd, "🦀️.rs") if nd else os.path.join(MCP, "📦️packages", "🦀️rust", "🦀️.rs")
        mdir = pkg_dir(lay, ci)
        feats = ["testkit"] if (any(id(o) in em.testkit_outs for o in lay.outs if o.crate == ci) or any(id(u.atom) in em.testkit_atoms for o in lay.outs if o.crate == ci for u in o.units)) else []
        crates.append(dict(name=crate_pkg(lay, ci), entry=entry, manifestDir=mdir, features=[]))
    json.dump(crates, open(crates_json, "w"), ensure_ascii=False)
    return lay, em, files

if __name__ == "__main__":
    stage(sys.argv[1], SCR + "stage.json", SCR + "crates.json")
