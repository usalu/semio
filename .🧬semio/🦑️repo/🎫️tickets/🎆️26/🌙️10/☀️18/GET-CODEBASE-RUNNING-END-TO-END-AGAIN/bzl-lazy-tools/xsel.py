#!/usr/bin/env python3
# Cross-selection cache: builds selection A, then selection B (a subset/superset sharing tasks) against the SAME disk cache but fresh servers and output bases; B's shared actions must be disk cache hits.
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import measure as M

def main():
    a_name, a_ws, a_label, b_name, b_ws, b_label = sys.argv[1:7]
    shared = M.TMP + "/dc-xsel"
    M.shutil.rmtree(shared, ignore_errors=True)
    a = M.Ws(a_name, a_ws); b = M.Ws(b_name, b_ws)
    a.dc = b.dc = shared
    M.stub(a_ws); M.stub(b_ws)
    a.wipe_ob(); b.wipe_ob()
    out = {"a": a_label, "b": b_label}
    out["a_cold"] = a.run("build", [a_label])
    out["b_after_a"] = b.run("build", [b_label])
    out["b_again"] = b.run("build", [b_label])
    a.kill(); b.kill()
    print(json.dumps(out))

if __name__ == "__main__": main()
