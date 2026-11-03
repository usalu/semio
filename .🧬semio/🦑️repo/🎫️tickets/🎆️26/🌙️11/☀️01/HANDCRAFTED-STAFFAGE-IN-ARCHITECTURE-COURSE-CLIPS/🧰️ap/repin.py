import json,sys
d=json.load(open(sys.argv[1],encoding="utf8"))
path=sys.argv[2]
text=open(path,encoding="utf8",newline="").read()
for k,v in d["result"].items():
    for f,h in v["sha256"].items():
        old=d["pinned"][k]["sha256"][f]
        assert text.count(old)==1,(k,f,text.count(old))
        text=text.replace(old,h)
with open(path,"r+b") as fh:
    fh.write(text.encode("utf8"))
print("repinned")
