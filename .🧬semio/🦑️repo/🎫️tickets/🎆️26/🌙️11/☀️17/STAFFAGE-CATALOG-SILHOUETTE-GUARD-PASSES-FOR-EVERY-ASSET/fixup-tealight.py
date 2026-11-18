import sys
P="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/tealight/🔣️.json"
v=sys.argv[1]
lines=open(P,encoding="utf8").read().split("\n")
for k in (846,850,900,904):
    assert lines[k-1].strip() in ("0.016",)+tuple(sys.argv[2:3]), (k,lines[k-1])
    lines[k-1]="                  "+v
open(P,"w",encoding="utf8").write("\n".join(lines))
