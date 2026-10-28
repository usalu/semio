import sys
p=sys.argv[1]+"/overlay.py"
s=open(p,encoding="utf-8").read()
s=s.replace('paths += sorted({e["file"] for e in spec["edits"]}) + [n["path"] for n in spec["newFiles"]]','paths += sorted({e.get("file") or spec.get("file") for e in spec["edits"]}) + [n["path"] for n in spec.get("newFiles",[])]')
open(p,"w",encoding="utf-8").write(s)
