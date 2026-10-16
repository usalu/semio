"""Runs the path rewriting over every remaining test file and lists the root heads that have no mapping."""
import collections, sys
from build import *
import inventory as INV
X.CURRENT[0] = "probe"
inv = INV.inventory()
bad = collections.defaultdict(list)
for f in inv:
    t = read(f)
    for _ in range(40):
        try:
            B.transform_test_probe(t)
            break
        except KeyError as e:
            msg = str(e)
            bad[msg].append(os.path.relpath(f, E)[-60:])
            m = re.search(r"crate::([A-Za-z_0-9]+)|(?:interpreter|scenes|engine_canvas|shell)::([A-Za-z_0-9]+)", msg)
            name = (m.group(1) or m.group(2)) if m else None
            # register a dummy mapping so the scan can continue
            if name:
                EXTRA_ROOT[name] = ["__unmapped", name]
                for fac in EXTRA_FACADE:
                    EXTRA_FACADE[fac][name] = ["__unmapped", name]
            else:
                break
for k, v in sorted(bad.items()):
    print(k, "<-", len(v), v[:3])
print("done", len(inv))
