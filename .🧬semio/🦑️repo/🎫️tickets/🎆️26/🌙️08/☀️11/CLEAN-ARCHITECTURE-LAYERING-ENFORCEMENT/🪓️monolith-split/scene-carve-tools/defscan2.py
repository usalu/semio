"""🔎 Type-scoped definition scan: private fields and methods of the types a crate actually names from another crate, used by that crate."""
import re, sys, collections
from lib import *
from model import *
from synth import code_idents
import defscan
from union import owned_files

def run(tree, deps, show=True):
    keys = list(deps)
    texts = {k: {f: rd(tree + "/" + f) for f in defscan.crate_files(tree, k)} for k in keys}
    privf, privm, pubtypes = {}, {}, {}
    for d in keys:
        pf, pm, pt = {}, {}, set()
        for f, t in texts[d].items():
            for s, fl in defscan.struct_private_fields(t).items():
                pf.setdefault(s, set()).update(fl)
            for s, fl in defscan.impl_private_items(t).items():
                pm.setdefault(s, set()).update(fl)
            pt |= {n for n, k in api_names(t).items() if k in ("struct", "enum", "union", "type", "trait", "use")}
        privf[d], privm[d], pubtypes[d] = pf, pm, pt
    out = []
    for c in keys:
        for d in deps[c]:
            for f, t in texts[c].items():
                code = re.sub(r"//[^\n]*", "", t)
                ids = code_idents(t)
                for ty in pubtypes[d] & ids:
                    for fld in privf[d].get(ty, ()):
                        if re.search(r"\." + fld + r"\b(?!\s*\()", code) or re.search(r"[{,]\s*" + fld + r"\s*:(?!:)", code):
                            out.append(("field", c, d, ty, fld, f))
                    for m in privm[d].get(ty, ()):
                        if re.search(r"\." + m + r"\s*\(", code) or re.search(r"\b" + ty + r"::" + m + r"\b", code):
                            out.append(("method", c, d, ty, m, f))
    return out

DEPS = {"algebra":[],"camera":["algebra"],"mesh":["algebra"],"pass":["algebra","mesh","camera"],"picking":["algebra","camera","mesh","pass"],"pack":["camera","algebra"],"doc":["pack"],"modelling":["mesh","camera","algebra","pack"],"world3d":["doc","pack","modelling","pass","camera","algebra","mesh","picking","surface"],"node-graph":["doc","pack","surface"],"plane":["doc","pack","surface"],"views":["doc","pack","surface"],"surface":["doc","pack","world3d","plane","views","node-graph"]}

if __name__ == "__main__":
    for h in run(sys.argv[1] if len(sys.argv) > 1 else NEW, DEPS):
        print(*h)
    print("scan done")
