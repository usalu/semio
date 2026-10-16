"""🚦️ Exports the five GIS inference source gates from a copy of the hub script so they can run in isolation."""
import os,re,sys
SRC,out=sys.argv[1],sys.argv[2]
D="/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust"
t=open(SRC,encoding='utf-8').read()
def fix(m):
    spec=m.group(2)
    if spec.startswith('../') or spec.startswith('./'):
        spec=os.path.normpath(os.path.join(D,spec))
    return m.group(1)+spec+m.group(3)
t=re.sub(r'(from\s+")(\.{1,2}/[^"]+)(")',fix,t)
t=re.sub(r'(import\s+")(\.{1,2}/[^"]+)(")',fix,t)
t=re.sub(r'(import\(")(\.{1,2}/[^"]+)("\))',fix,t)
for fn in ['proveGisMapApprovalUndoFixture','proveGisInferenceRetainedRuntimeFixture','proveGisInferenceCheckpointControlFixture','proveGisMapProposalApprovalFixture','proveGisMapFrozenBindingFixture']:
    a='async function %s('%fn
    assert t.count(a)==1,fn
    t=t.replace(a,'export '+a)
n=t.count('if (import.meta.main) await receiveScriptProcessInvocation')
t=t.replace('if (import.meta.main) await receiveScriptProcessInvocation','if (false) await receiveScriptProcessInvocation')
open(out,'w',encoding='utf-8').write(t)
print('ok',out[-40:],len(t),'main-guards',n)
