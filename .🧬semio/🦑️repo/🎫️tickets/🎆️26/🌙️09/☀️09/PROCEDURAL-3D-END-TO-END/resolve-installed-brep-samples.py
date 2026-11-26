"""🔬 Match the browser core's executable section to its linked component before naming sampled functions."""
from pathlib import Path
import gzip,hashlib,json,re,sys,collections
root=Path(__file__).parent/'🗑️generated/bridge-current'
def number(b,i):
    n=s=0
    while True:
        v=b[i];i+=1;n+=(v&127)<<s
        if v<128:return n,i
        s+=7
def sections(b,a,end):
    while a<end:
        k=b[a];size,start=number(b,a+1);a=start+size
        yield k,start,a
def sig(b):
    return next(hashlib.sha256(b[a:z]).digest() for k,a,z in sections(b,8,len(b)) if k==10)
base='semio_s_plugin_flow_brep_component'
target=sig(memoryview((root/'stable/browser'/base/(base+'.core.wasm')).read_bytes()))
sample=collections.Counter()
for row in Path(sys.argv[1]).read_text().splitlines():
    if 'Worker CPU samples ' not in row: continue
    for count,fn,url in json.loads(row.split('Worker CPU samples ',1)[1]):
        m=re.fullmatch(r'wasm-function\[(\d+)\]',fn)
        if m and base in url: sample[int(m[1])]+=count
if not sample: raise SystemExit("No BRep samples yet")
with gzip.open(root/'stable/wasm32-wasip2/debug'/(base+'.wasm.gz')) as f: component=memoryview(f.read())
cores=[component[a:z] for k,a,z in sections(component,8,len(component)) if k==1]
core=next(c for c in cores if sig(c)==target)
for k,a,z in sections(core,8,len(core)):
    if k!=0 or bytes(core[a:a+5])!=b'\x04name':continue
    for sub,pos,stop in sections(core,a+5,z):
        if sub!=1: continue
        count,pos=number(core,pos)
        for i in range(count):
            index,pos=number(core,pos);size,pos=number(core,pos)
            if index in sample: print(sample[index],index,bytes(core[pos:pos+size]).decode(),sep='\t')
            pos+=size
