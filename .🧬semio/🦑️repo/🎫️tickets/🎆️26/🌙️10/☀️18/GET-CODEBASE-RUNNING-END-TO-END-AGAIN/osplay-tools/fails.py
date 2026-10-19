import sys
lines=open(sys.argv[1],encoding="utf-8").read().split("\n")
n=int(sys.argv[2]) if len(sys.argv)>2 else 4
for k,l in enumerate(lines):
    if l.startswith(" FAIL "):
        print(l.split("| ",1)[-1][-230:])
        for m in lines[k+1:k+1+n]: print("    ",m[:300])
