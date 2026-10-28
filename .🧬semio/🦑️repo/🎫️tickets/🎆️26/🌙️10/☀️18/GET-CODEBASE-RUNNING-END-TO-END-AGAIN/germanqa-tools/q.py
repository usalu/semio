import json,re,sys
r=json.load(open("🗑️generated/germanqa/pairs.json"))
rx=re.compile(sys.argv[1]); n=int(sys.argv[2]) if len(sys.argv)>2 else 10
side=sys.argv[3] if len(sys.argv)>3 else "de"
seen=set(); k=0
for x in r:
    s=x[side]
    if not s or (x["en"],x["de"]) in seen or x["en"]==x["de"]: continue
    if rx.search(s):
        seen.add((x["en"],x["de"])); k+=1
        if k<=n: print(f"[{x['file'].split('/')[-3]}/{x['file'].split('/')[-2]}{x['path'][:50]}]\n  EN: {x['en'][:300]}\n  DE: {x['de'][:300]}")
print(k,"hits")
