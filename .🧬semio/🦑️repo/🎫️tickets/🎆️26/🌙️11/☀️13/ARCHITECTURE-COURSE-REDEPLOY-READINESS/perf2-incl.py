import json,sys,collections,re
f=sys.argv[1]; top=int(sys.argv[2]) if len(sys.argv)>2 else 40
p=json.load(open(f)); nodes={n["id"]:n for n in p["nodes"]}; parent={}
for n in p["nodes"]:
    for c in n.get("children",[]): parent[c]=n["id"]
incl=collections.Counter(); selfc=collections.Counter()
for s,d in zip(p["samples"],p["timeDeltas"]):
    seen=set(); i=s; first=True
    while i is not None:
        cf=nodes[i]["callFrame"]; k=f'{cf["functionName"] or "(anon)"} {cf["url"].split("/")[-1][-24:]}:{cf["lineNumber"]}'
        if first: selfc[k]+=d/1000; first=False
        if k not in seen: seen.add(k); incl[k]+=d/1000
        i=parent.get(i)
for k,v in incl.most_common(top*3):
    if re.match(r"^[A-Z][A-Za-z0-9]+ |^use[A-Z]|^[a-z]+(Issues|Of|For)\b",k) and not k.startswith(("Object","Array","Map","Set")): print(f"{v:6.0f} incl  {k}")
print("-- self"); 
for k,v in selfc.most_common(12): print(f"{v:6.0f} self  {k}")
