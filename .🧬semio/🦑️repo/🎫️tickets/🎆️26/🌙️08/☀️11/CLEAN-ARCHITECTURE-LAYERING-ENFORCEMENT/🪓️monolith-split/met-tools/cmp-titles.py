import collections,sys
def load(p):
    return [l.rstrip("\n").split("\t",1) for l in open(p,encoding="utf8") if l.strip()]
b=load(sys.argv[1]); a=load(sys.argv[2])
cb=collections.Counter(t for s,t in b); ca=collections.Counter(t for s,t in a)
print("titles only in baseline:", list((cb-ca).elements()))
print("titles only in after:", list((ca-cb).elements()))
sb=collections.defaultdict(list); sa=collections.defaultdict(list)
for s,t in b: sb[t].append(s)
for s,t in a: sa[t].append(s)
n=0
for t in sorted(set(sb)|set(sa)):
    if sorted(sb[t])!=sorted(sa[t]): print("STATUS DIFF", t[:110], sb[t], "->", sa[t]); n+=1
print("status diffs:",n, "baseline",len(b),"after",len(a))
