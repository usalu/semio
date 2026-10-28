import json,re,sys,collections,os
r=json.load(open("🗑️generated/germanqa/pairs.json"))
seen=set(); groups=collections.OrderedDict()
for x in r:
    d=x["de"]
    if not d or x["en"]==d: continue
    if d in seen: continue
    if "📖️help" in x["file"] or "🧫️fixtures" in x["file"]: continue
    seen.add(d)
    f=x["file"]
    gk="/".join(f.split("/")[2:4]) if "🏃️process" not in f else "process/"+"/".join(f.split("/")[3:4])
    groups.setdefault((f,),[]).append((x["path"],d))
os.makedirs("🗑️generated/germanqa/dump",exist_ok=True)
tot=0
byfile=collections.OrderedDict()
for (f,),rows in groups.items(): byfile[f]=rows
# write files in order grouped by top module chunk ~28KB
cur=[];size=0;n=0
def flush():
    global cur,size,n
    if cur:
        open(f"🗑️generated/germanqa/dump/{n:02d}.txt","w",encoding="utf-8").write("".join(cur)); n+=1
    cur=[];size=0
for f,rows in byfile.items():
    short="/".join(f.split("/")[-5:])
    s=f"\n### {short}\n"+"".join(f"{p.split('/')[-1][:28]}: {d}\n" for p,d in rows)
    if size+len(s)>30000 and size>0: flush()
    cur.append(s); size+=len(s)
flush()
print(n,"chunks",sum(len(rows) for rows in byfile.values()),"strings")
