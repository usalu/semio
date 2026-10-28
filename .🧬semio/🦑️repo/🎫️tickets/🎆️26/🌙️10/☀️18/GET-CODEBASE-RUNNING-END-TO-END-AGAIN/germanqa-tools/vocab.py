import json, collections, re, sys
r=json.load(open(sys.argv[1]))
u={}
for x in r:
    if x["en"]!=x["de"] and x["de"]: u[(x["en"],x["de"])]=1
cnt=collections.Counter()
def clean(d):
    d=re.sub(r"\{[A-Za-z_][A-Za-z0-9_.]*\}"," ",d)
    d=re.sub(r"`[^`]*`"," ",d)
    d=re.sub(r"https?://\S+"," ",d)
    d=re.sub(r"[\w./-]*[/\\][\w./\\-]*"," ",d)
    return d
for (e,d) in u:
    for t in re.findall(r"[A-Za-zÄÖÜäöüß][A-Za-zÄÖÜäöüß'-]*",clean(d)): cnt[t]+=1
json.dump(cnt,open(sys.argv[2],"w"),ensure_ascii=False)
print(len(cnt))
