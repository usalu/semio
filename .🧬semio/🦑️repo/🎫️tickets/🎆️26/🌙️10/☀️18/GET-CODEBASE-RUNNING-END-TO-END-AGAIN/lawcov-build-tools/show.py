import sys,os
sys.argv=[sys.argv[0]]+sys.argv[1:]
FE="️"
LIB="/Users/ueli/Documents/semio/🧰"+FE+"framework/🛍"+FE+"products/🦑"+FE+"repo/🔨"+FE+"modules/📚"+FE+"library"
cur=LIB
for part in sys.argv[1].split("/"):
    for e in os.listdir(cur):
        if e.replace(FE,"")==part.replace(FE,""):
            cur=os.path.join(cur,e);break
    else: raise SystemExit("no "+part)
print("DIR",cur)
for e in sorted(os.listdir(cur)):
    p=os.path.join(cur,e)
    if os.path.isfile(p):
        print("=== FILE",e.replace(FE,""));print(open(p,encoding="utf8").read()[:int(os.environ.get("MAXC","9000"))])
    else: print("--- DIR",e.replace(FE,""))
