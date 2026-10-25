import json,sys
p="🔒️dependencies.json"
raw=open(p,encoding="utf8").read(); d=json.loads(raw)
key="🧩️vscode/"
out=[];dropped=[]
for e in d["entries"]:
  u=e.get("users")
  if u is not None:
    n=[x for x in u if key not in x]
    if len(n)!=len(u):
      if not n: dropped.append((e["ecosystem"],e["name"])); continue
      e["users"]=n
  out.append(e)
d["entries"]=out
print("dropped",dropped)
if "--write" in sys.argv: open(p,"w",encoding="utf8").write(json.dumps(d,indent=2,ensure_ascii=False)+"\n")
