"""🔎 Print one section of a catalog asset JSON compactly: python3 author-show.py <path> <dotted.key> [maxchars]"""
import json,sys
d=json.load(open(sys.argv[1]))
o=d
for k in sys.argv[2].split(".") if len(sys.argv)>2 and sys.argv[2] else []:
    o=o[int(k)] if isinstance(o,list) else o[k]
n=int(sys.argv[3]) if len(sys.argv)>3 else 3000
if isinstance(o,dict):
    for k,v in o.items():
        s=json.dumps(v,ensure_ascii=False); print(f"{k} ({len(s)}): {s[:n]}")
elif isinstance(o,list):
    print("list",len(o))
    for v in o: print(json.dumps(v,ensure_ascii=False)[:n])
else: print(o)
