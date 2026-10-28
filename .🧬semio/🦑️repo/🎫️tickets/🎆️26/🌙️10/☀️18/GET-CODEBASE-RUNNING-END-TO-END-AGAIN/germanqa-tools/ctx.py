import json,re,sys
r=json.load(open("🗑️generated/germanqa/pairs.json"))
rx=re.compile(sys.argv[1]); n=int(sys.argv[2]) if len(sys.argv)>2 else 8
w=int(sys.argv[3]) if len(sys.argv)>3 else 60
seen=set(); out=[]
for x in r:
    d=x["de"]
    if not d or d in seen or x["en"]==d: continue
    m=rx.search(d)
    if m:
        seen.add(d); out.append((x["file"].split("/")[-3]+"/"+x["file"].split("/")[-2], d[max(0,m.start()-w):m.end()+w].replace("\n"," ")))
print(len(out),"hits")
step=max(1,len(out)//n)
for f,s in out[::step][:n]: print(f"  {f}: …{s}…")
