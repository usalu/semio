import os

io = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io"
sqlname = next(n for n in os.listdir(io) if n.endswith("sqlite"))
sdir = os.path.join(io, sqlname, next(n for n in os.listdir(os.path.join(io, sqlname)) if "snapshot" in n))
rs = next(n for n in os.listdir(sdir) if n.endswith(".rs") and n.count(".") == 1)
text = open(os.path.join(sdir, rs), encoding="utf-8").read()
i = text.find("joints.push(SteelJoint{")
j = text.find("});", i)
chunk = text[i:j]
print(chunk[-1800:])
