"""📦 Compare byte-faithful GLB primitive layouts against the published equipment geometry."""
import json,math,struct,sys,hashlib
from array import array
from collections import Counter,defaultdict
from pathlib import Path
T=Path(__file__).resolve().parent
source=(T/'🗑️generated/equipment-check/mixer-base.glb').read_bytes()
js=struct.unpack_from('<I',source,12)[0]
j=json.loads(source[20:20+js]);binary=memoryview(source)[28+js:]
arrays=j['accessors'];views=j['bufferViews'];dims={'SCALAR':1,'VEC2':2,'VEC3':3,'VEC4':4,'MAT4':16};fmt={5126:'f',5125:'I',5123:'H',5121:'B'}
def raw(i):
 a=arrays[i];v=views[a['bufferView']];return binary[v['byteOffset']:v['byteOffset']+v['byteLength']]
def components(i):return raw(i).cast(fmt[arrays[i]['componentType']])
def digest(x):return hashlib.blake2b(x,digest_size=16).digest()

def ana(limit):
 attrs={};saved=Counter();lots=Counter();prims=0;extra=0;sel={};splits={};triBounds={};triShare=defaultdict(set);parentHash={i:digest(raw(i)) for i in range(len(arrays))}
 def emit(key,v):
  if key not in attrs:attrs[key]=v
 def sliceattr(i,lo,hi):
  a=arrays[i];width=dims[a['type']]*struct.calcsize(fmt[a['componentType']]);return raw(i)[lo*width:hi*width]
 for m in j['meshes']:
  for p in m['primitives']:
   if p['mode']!=4:continue
   indices=components(p['indices']);position=p['attributes']['POSITION']
   for s in p.get('extras',{}).get('solids',[]):
    ids=indices[s['first']:s['first']+s['count']];lo,hi=min(ids),max(ids)+1;triBounds[(position,s['id'])]=(lo,hi)
    for a in p['attributes'].values():
     chunk=sliceattr(a,lo,hi)
     if len(chunk)>=limit:triShare[digest(chunk)].add((parentHash[a],lo))
 for m in j['meshes']:
  for p in m['primitives']:
   ind=components(p['indices']);pos=p['attributes']['POSITION'];solids=p.get('extras',{}).get('solids',[]);groups=[];unit=[]
   for s in solids:
    first=s['first'];end=first+s['count'];ids=ind[first:end];lo,hi=min(ids),max(ids)+1;tr=triBounds.get((pos,s['id']));lo,hi=tr if tr and tr[0]<=lo and hi<=tr[1] else (lo,hi)
    attrsToCheck=[pos] if p['mode']==1 else p['attributes'].values();can=any(len(ch:=sliceattr(i,lo,hi))>=limit and len(triShare.get(digest(ch),()))>1 for i in attrsToCheck)
    if can:
     if unit:groups.append(unit);unit=[]
     groups.append([(s,lo,hi)])
    else:unit.append((s,lo,hi))
   if unit:groups.append(unit)
   if len(groups)<=1:
    groups=[[(s,0,arrays[pos]['count']) for s in solids]]
   for group in groups:
    prims+=1
    if not group or len(groups)==1:lo=0;hi=arrays[pos]['count'];idx=ind
    else:
     lo=min(x for _,x,_ in group);hi=max(y for _,_,y in group);first=group[0][0]['first'];last=group[-1][0];idx=ind[first:last['first']+last['count']]
    for sem,i in p['attributes'].items():
     data=sliceattr(i,lo,hi);key=(sem,digest(data));emit(key,len(data))
    pack='B' if hi-lo<=255 else 'H' if hi-lo<=65535 else 'I';data=array(pack,(i-lo for i in idx));key=('IDX'+pack,digest(data));emit(key,len(data)*data.itemsize)
    if len(groups)>1:extra+=1
 by=Counter()
 for (k,_),n in attrs.items():by[k]+=n
 original=sum(len(v['primitives']) for v in j['meshes']);print('minimum duplicate payload',limit,'primitives',prims,'additional',prims-original,'BIN',sum(attrs.values()),'by',dict(by),'approx json',js+(prims-original)*440+(len(attrs)-len(arrays))*165,'approx bytes',sum(attrs.values())+js+(prims-original)*440+(len(attrs)-len(arrays))*165,flush=True)
for l in (32768,8192,4096,2048,1024,512,256,65536*1024):ana(l)
