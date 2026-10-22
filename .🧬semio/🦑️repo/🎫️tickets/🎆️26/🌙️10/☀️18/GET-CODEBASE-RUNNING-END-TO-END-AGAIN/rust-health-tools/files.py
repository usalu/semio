import re,sys,collections
log=open(sys.argv[1],errors='replace').read().split('\n'); crate=sys.argv[2]
errs=[];res={}
for l in log:
    if re.match(r'^[^ \[].*:\d+:\d+: error',l): errs.append(l)
    m=re.match(r'^error: could not compile .([^`]+). \(([^)]*)\)',l)
    if m: res.setdefault(m.group(1),[]).extend(errs); errs=[]
c=collections.Counter()
for e in res.get(crate,[]): c[re.sub(r':\d+:\d+: .*','',e).split('rust/../')[-1]]+=1
for k,v in c.most_common(): print(v,k)
