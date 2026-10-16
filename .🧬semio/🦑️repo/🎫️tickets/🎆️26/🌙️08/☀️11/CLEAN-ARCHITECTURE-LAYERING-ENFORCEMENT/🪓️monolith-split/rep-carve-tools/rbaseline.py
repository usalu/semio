"""🔬 Runs the baseline differential for the replication facade and pickles the error multisets."""
import os, pickle, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
import roracle as o

if __name__ == "__main__":
    root = SP + "/S0"
    shutil.rmtree(root, ignore_errors=True)
    shutil.copytree(BASE, root, symlinks=True)
    o.patch_tree(root)
    lib = root + "/" + FACADE_ROOT
    res = {}
    for tag, extra in o.CFGS.items():
        c, err = o.run(lib, "protocol", extra)
        res[tag] = c
        open(SP + "/err-S0-" + tag + ".txt", "w").write(err)
        print(tag, sum(c.values()), flush=True)
    pickle.dump(res, open(SP + "/base-rep.pkl", "wb"))
