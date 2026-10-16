"""usage: renameproject.py <old-project> <new-project> <package-root-rel>  — move the owner row (sorted insert), the project.json name and the six routes of both launch files to a new project name."""
import sys
sys.path.insert(0, ".")
import reg
old, new, root = sys.argv[1:4]
s = reg.read(reg.OWNERS)
row = f'    {{\n      "name": "{old}",\n      "root": "{root}",\n      "manifest": "{root}/📋️project.json",\n      "packages": []\n    }},\n'
assert s.count(row) == 1
reg.write_atomic(reg.OWNERS, s.replace(row, ""))
reg.insert_owner(new, root)
for p in (reg.LAUNCH, reg.SEED):
    t = reg.read(p)
    assert t.count(old) == 6, (p, t.count(old))
    reg.write_atomic(p, t.replace(old, new))
pj = reg.ROOT + "/" + root + "/📋️project.json"
t = reg.read(pj)
assert old in t
reg.write_atomic(pj, t.replace(old, new))
print("renamed", old, "->", new)
