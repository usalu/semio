import os

io = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io"
sqlname = next(n for n in os.listdir(io) if n.endswith("sqlite"))
sdir = os.path.join(io, sqlname, next(n for n in os.listdir(os.path.join(io, sqlname)) if "snapshot" in n))
rs = next(n for n in os.listdir(sdir) if n.endswith(".rs") and n.count(".") == 1)
text = open(os.path.join(sdir, rs), encoding="utf-8").read()
key = 'insert_float("en1993_joint",&['
i = text.find(key)
j = text.find("],", i)
chunk = text[i + len(key) : j]
print("beam_web", "beam_web" in chunk)
print("weld_force", "weld_force" in chunk)
print("cells", chunk.count("Cell::"))
print(chunk[-500:])
