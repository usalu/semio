import re,sys,os,collections
root="/Users/ueli/Documents/semio"
start=os.path.join(root,sys.argv[1]);goal=sys.argv[2]
rx=re.compile(r'''(?:from|import)\s*\(?\s*["'](\.{1,2}/[^"']+)["']''')
def resolve(base,spec):
    p=os.path.normpath(os.path.join(os.path.dirname(base),spec))
    for c in (p,p+".ts",p+".mjs",p+".cjs",p+".js"):
        if os.path.isfile(c):return c
    return None
prev={start:None};q=collections.deque([start])
while q:
    f=q.popleft()
    if goal in f:
        chain=[];
        while f: chain.append(f.replace(root+"/","")); f=prev[f]
        print("\n -> ".join(reversed(chain)));break
    try:s=open(f,encoding="utf8").read()
    except Exception:continue
    if f.endswith(".json"):continue
    for m in rx.finditer(s):
        t=resolve(f,m.group(1))
        if t and t not in prev and not t.endswith(".json"):
            prev[t]=f;q.append(t)
else:print("no chain; visited",len(prev))
