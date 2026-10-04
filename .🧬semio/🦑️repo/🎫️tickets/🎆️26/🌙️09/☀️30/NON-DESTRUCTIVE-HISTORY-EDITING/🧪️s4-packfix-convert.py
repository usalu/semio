import difflib,re,sys,json
K,F,OUT,REVIEW=sys.argv[1:5]
VARIANTS='BadMagic|UnsupportedVersion|UnknownRequiredFlags|Truncated|ChecksumMismatch|ContentHashMismatch|LimitExceeded|RetainedMalformed|RetainedAllocation|Malformed|NonCanonical|UnsupportedCodec|ValueRefusal|TextRefusal|Io|Schema|TransportAdmission'
def norm(s):
    s=re.sub(r'semio_framework_dsl_record::(Shape|FieldValue|FieldSpec|RecordValue|RecordSpec|WireNode|WireValue|WireEdgeLabel|RecordLayout|RecordSpecProducer)\b',r'\1',s)
    s=s.replace('crate::os_pack::','crate::').replace('protocol::value::','semio_framework_value::')
    s=re.sub(r'semio_framework_value::(DslValue|Number)\b',r'\1',s)
    s=s.replace('PackError','PackRefusal')
    s=re.sub(r'kind\s*:\s*ValueRefusalKind::\w+\s*,\s*','',s)
    s=re.sub(r'\s+','',s)
    s=re.sub(r'LimitExceeded\{limit:("[^"]*")\}',r'LimitExceeded(\1)',s)
    return s
def close(s,i):
    o=s[i];c={'(' :')','{':'}','[':']'}[o];d=0
    for j in range(i,len(s)):
        if s[j]==o:d+=1
        elif s[j]==c:
            d-=1
            if d==0:return j
    raise ValueError(s)
def wrap(s):
    s=re.sub(r'map_err\(PackRefusal::(ValueRefusal|TextRefusal|from)\)','map_err(PackError::from)',s)
    s=s.replace('PackRefusal::from(','PackError::from(')
    out='';i=0
    for m in re.finditer(r'PackRefusal::(\w+)',s):
        pass
    while True:
        m=re.compile(r'(?<!Refusal\()PackRefusal::(\w+)').search(s,i)
        if not m:out+=s[i:];break
        out+=s[i:m.start()]
        j=m.end()
        k=j
        while k<len(s) and s[k]==' ':k+=1
        if k<len(s) and s[k] in '({':
            e=close(s,k);expr=s[m.start():e+1];i=e+1
        else:
            expr=s[m.start():j];i=j
        out+='PackError::Refusal('+expr+')'
    s=out
    out='';i=0
    while True:
        m=re.compile(r'([\w.]+)\.into_pack_refusal\(').search(s,i)
        if not m:out+=s[i:];break
        e=close(s,m.end()-1)
        out+=s[i:m.start()]+'PackError::Refusal('+s[m.start():e+1]+')';i=e+1
    s=out
    s=re.sub(r'\bPackRefusal\b(?!::)(?!\()','PackError',s)
    s=re.sub(r'PackError::Refusal\(PackError\b','PackError::Refusal(PackRefusal',s)
    return s
def denorm(t,k):
    if 'crate::os_pack::' in k: t=re.sub(r'crate::(?!os_pack)','crate::os_pack::',t)
    return t
k=open(K).read().split('\n');f=open(F).read().split('\n')
kn=[norm(x) for x in k];fn=[norm(x) for x in f]
sm=difflib.SequenceMatcher(None,kn,fn,autojunk=False)
m={}
for tag,i1,i2,j1,j2 in sm.get_opcodes():
    for i in range(i1,i2):
        if tag=='equal' or j2-j1==i2-i1: m[i]=[j1+(i-i1)]
        else: m[i]=list(range(j1,j2))
res=list(k);rev=[]
for i,line in enumerate(k):
    if not re.search(r'PackError::('+VARIANTS+r')\b',line): continue
    js=m.get(i,[])
    if len(js)==1 and kn[i]==fn[js[0]] and 'PackRefusal' in f[js[0]]:
        try:
            cand=wrap(denorm(f[js[0]],line))
        except ValueError:
            rev.append(('MANUAL',i+1,line.strip(),f[js[0]].strip()));continue
        res[i]=cand
        rev.append(('AUTO',i+1,line.strip(),cand.strip()))
    else:
        rev.append(('MANUAL',i+1,line.strip(),' || '.join(f[j].strip() for j in js[:6])))
open(OUT,'w').write('\n'.join(res))
with open(REVIEW,'w') as o:
    for r in rev: o.write(f'{r[0]} K{r[1]}\n  OLD {r[2]}\n  NEW {r[3]}\n')
print(sum(1 for r in rev if r[0]=='AUTO'),sum(1 for r in rev if r[0]=='MANUAL'))
