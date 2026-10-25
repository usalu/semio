import json,re,sys
a=json.load(open(sys.argv[1])); b=json.load(open(sys.argv[2]))
tok=lambda x: re.sub(r'SEMIO_(SOURCE_CLOSURE|CARGO_LOCK)_[0-9a-f]{64}','TOK',x)
def effective(entry):
    p=entry['project']; named=p.get('namedInputs',{})
    def expand(items,seen=()):
        out=[]
        for i in items:
            if isinstance(i,str) and i in named and i not in seen:
                out+=expand(named[i],seen+(i,))
            else: out.append(json.dumps(i,sort_keys=True))
        return out
    res={}
    for t,v in p.get('targets',{}).items():
        res[t]=sorted(set(tok(x) for x in expand(v.get('inputs',[]))))
    return res,{k:v for k,v in p.items() if k not in('namedInputs',)} 
bad=0
for k in sorted(a):
    ea,ra=effective(a[k]); eb,rb=effective(b[k])
    # remaining project (targets other than inputs) compared with inputs removed
    def strip(r):
        r=json.loads(json.dumps(r)); 
        for t in r.get('targets',{}).values(): t.pop('inputs',None)
        return tok(json.dumps(r,sort_keys=True))
    if ea!=eb or strip(ra)!=strip(rb):
        bad+=1
        if bad<=3:
            print(k,'inputs differ' if ea!=eb else 'rest differs')
            for t in ea:
                if ea[t]!=eb.get(t): print(' target',t,'only raw',[x for x in ea[t] if x not in eb.get(t,[])][:4],'only late',[x for x in eb.get(t,[]) if x not in ea[t]][:4])
print('semantically differing projects',bad,'of',len(a))
