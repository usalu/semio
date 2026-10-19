import json,sys
a=json.load(open(sys.argv[1],encoding='utf8')); b=json.load(open(sys.argv[2],encoding='utf8'))
bad=[k for k in sorted(set(a)|set(b)) if a.get(k)!=b.get(k)]
for k in bad: print("DIFF",a.get(k),"->",b.get(k),k[:200])
print("differences:",len(bad))
