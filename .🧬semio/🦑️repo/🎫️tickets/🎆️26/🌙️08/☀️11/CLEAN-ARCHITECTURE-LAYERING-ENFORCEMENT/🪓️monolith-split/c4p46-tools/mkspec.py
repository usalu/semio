import json,sys,os
# usage: mkspec.py <out> <file> <importers.json> <children.json>
out,file,imp,ch=sys.argv[1:5]
spec={"file":file,"importers":json.load(open(imp,encoding="utf8")),"children":json.load(open(ch,encoding="utf8"))}
json.dump(spec,open(out,"w",encoding="utf8"),ensure_ascii=False,indent=1)
