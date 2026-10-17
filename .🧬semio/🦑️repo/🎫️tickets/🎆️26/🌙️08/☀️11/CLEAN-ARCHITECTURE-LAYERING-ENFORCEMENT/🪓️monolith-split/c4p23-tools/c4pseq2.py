"""🧵 c4p23 order proof, test-mount aware: like c4pseq.py, but the root-resident `#[cfg(test)] //#region X / #[path] mod t; / //#endregion X` mounts of the tests folders are taken out of both sequences and compared as their own sequence. A root keeps them in place while the code ranges around them move into one child, so they sit after the child mount.
usage: c4pseq2.py [key...]"""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
src = open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "c4pseq.py"), encoding="utf-8").read().split("for key in keys:")[0]
g = {"__file__": os.path.abspath(__file__).replace("c4pseq2", "c4pseq"), "__name__": "seq"}
exec(compile(src, g["__file__"], "exec"), g)
from c4plib import *
from plans import PLANS, ORDER

def split(ls):
    code, tests, i = [], [], 0
    while i < len(ls):
        if ls[i] == "#[cfg(test)]" and i + 4 < len(ls) and ls[i + 1].startswith("//#region") and ls[i + 2].startswith("#[path") and re.match(r"^mod \w+;$", ls[i + 3]) and ls[i + 4].startswith("//#endregion"):
            tests.append(tuple(ls[i:i + 5])); i += 5
        else:
            code.append(ls[i]); i += 1
    return code, tests

ok = True
for key in (sys.argv[1:] or ORDER):
    spec = PLANS[key]
    orig = [re.sub(r"\bpub\(super\) ", "", l.strip(), count=1) for l in lines_of(base_text(spec["rel"])) if l.strip()]
    new = [re.sub(r"\bpub\(super\) ", "", l.strip(), count=1) for l in g["flatten"](ROOT + "/" + spec["rel"]) if l.strip()]
    (oc, ot), (nc, nt) = split(orig), split(new)
    same = oc == nc and ot == nt
    ok &= same
    print(f"[{key}] code lines {len(oc)}=={len(nc)} test mounts {len(ot)}=={len(nt)} identical={same}")
print("SEQUENCE2", "OK" if ok else "DIFFERENT")
