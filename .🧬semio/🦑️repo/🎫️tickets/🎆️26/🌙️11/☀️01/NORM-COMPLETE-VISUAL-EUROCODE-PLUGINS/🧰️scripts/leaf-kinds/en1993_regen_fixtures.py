"""Rewrites the generated EN 1993 leaf and row vectors from the realistic base document (examples + showcase rows)."""
import os, sys, shutil
os.environ["EN1993_REGEN"] = "1"
sys.path.insert(0, os.path.dirname(__file__))
import en1993 as X, en1993_wire as Wr
shutil.rmtree(X.STAGE93, ignore_errors=True)
c = X.main()
c.F.written = set()
c.leaf_by_kind = {l.kind: l for l in c.new_leaves}
c.row_mods = {}
Wr.fixtures(c.F, c)
print(len(c.F.written), len(c.new_leaves), c.row_kinds[:2])
if "--apply" in sys.argv:
    print("applied", c.F.apply())
