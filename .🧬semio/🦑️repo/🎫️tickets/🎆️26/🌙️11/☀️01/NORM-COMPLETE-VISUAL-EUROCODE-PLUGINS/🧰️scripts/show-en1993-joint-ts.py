import os

io = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io"
sqlname = next(n for n in os.listdir(io) if n.endswith("sqlite"))
sdir = os.path.join(io, sqlname, next(n for n in os.listdir(os.path.join(io, sqlname)) if "snapshot" in n))
ts = next(n for n in os.listdir(sdir) if n.endswith(".ts") and "test" not in n)
text = open(os.path.join(sdir, ts), encoding="utf-8").read()
for key in ["beamWebThickness", "beamWebFy", "weldForceLongitudinal", "weldForceTransverse"]:
    print(key, text.count(key))
i = text.find("baseLeverRight")
print(text[i:i+700])
