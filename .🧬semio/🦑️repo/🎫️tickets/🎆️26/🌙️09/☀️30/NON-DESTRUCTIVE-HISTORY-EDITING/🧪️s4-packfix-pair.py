import difflib,re,sys
K,F,OUT=sys.argv[1],sys.argv[2],sys.argv[3]
def norm(s):
    s=re.sub(r'semio_framework_dsl_record::(Shape|FieldValue|FieldSpec|RecordValue|RecordSpec|WireNode|WireValue|WireEdgeLabel|RecordLayout|RecordSpecProducer)',r'\1',s)
    s=s.replace('crate::os_pack::','crate::').replace('protocol::value::','semio_framework_value::')
    s=re.sub(r'semio_framework_value::(DslValue|Number)',r'\1',s)
    s=re.sub(r'PackError::Refusal\(PackRefusal::','PackRefusal::',s)
    s=s.replace('PackError','PackRefusal')
    s=re.sub(r'kind\s*:\s*ValueRefusalKind::\w+\s*,\s*','',s)
    return re.sub(r'\s+','',s)
k=open(K).read().split('\n');f=open(F).read().split('\n')
kn=[norm(x) for x in k];fn=[norm(x) for x in f]
sm=difflib.SequenceMatcher(None,kn,fn,autojunk=False)
m={}
for tag,i1,i2,j1,j2 in sm.get_opcodes():
    if tag=='equal':
        for d in range(i2-i1): m[i1+d]=[j1+d]
    else:
        for i in range(i1,i2):
            if j2-j1==i2-i1: m[i]=[j1+(i-i1)]
            else: m[i]=list(range(j1,j2))
with open(OUT,'w') as o:
    for i,line in enumerate(k):
        if 'PackError' in line:
            o.write(f'K{i+1}: {line.strip()}\n')
            for j in m.get(i,[])[:6]:
                o.write(f'  F{j+1}: {f[j].strip()}\n')
