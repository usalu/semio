import os,sys,importlib
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
LIVE="/Users/ueli/Documents/semio/"
out=sys.argv[1];assert os.path.realpath(out).startswith("/private/tmp/claude-501/")
by={}
for m in sys.argv[2:]:
    for rel,old,new,count in importlib.import_module(m).EDITS:by.setdefault(rel,[]).append((old,new,count))
for rel,rows in by.items():
    p=out+"/"+rel
    text=open(p if os.path.exists(p) else LIVE+rel,encoding="utf8").read()
    for old,new,count in rows:
        n=text.count(old)
        if n!=count:sys.exit("anchor %d times (want %d) in %s: %r"%(n,count,rel,old[:80]))
        text=text.replace(old,new)
    os.makedirs(os.path.dirname(p),exist_ok=True);open(p,"w",encoding="utf8").write(text)
print("edits applied",len(by))
