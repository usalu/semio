import re,sys
sys.setrecursionlimit(100000)
text=open("🌎️hub/Cargo.lock").read()
deps={}
for block in text.split("[[package]]")[1:]:
    m=re.search(r'^name = "([^"]+)"',block,re.M)
    if not m: continue
    dm=re.search(r'^dependencies = \[(.*?)^\]',block,re.M|re.S)
    ds=set(re.findall(r'"([^" ]+)(?: [^"]*)?"',dm.group(1))) if dm else set()
    deps.setdefault(m.group(1),set()).update(ds)
target=sys.argv[1]
pk=open(sys.argv[2]).read().split()
memo={}
def reach(n):
    if n in memo: return memo[n]
    memo[n]=False
    r = n==target or any(reach(x) for x in deps.get(n,()))
    memo[n]=r
    return r
print("packages in lock:",len(deps),"target present:",target in deps)
for p in pk: print(p, reach(p))
