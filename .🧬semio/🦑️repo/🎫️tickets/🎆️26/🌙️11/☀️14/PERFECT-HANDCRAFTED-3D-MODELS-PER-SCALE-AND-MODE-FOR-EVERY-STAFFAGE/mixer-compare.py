"""📦 Independently compare every indexed vertex attribute, material variant, part, node and scene across two GLBs."""
from hashlib import sha256
from pathlib import Path
from struct import unpack_from,calcsize
import json
import sys

out=Path(__file__).resolve().parent/'🗑️generated/equipment-check'
formats={5120:'b',5121:'B',5122:'h',5123:'H',5125:'I',5126:'f'}
components={'SCALAR':1,'VEC2':2,'VEC3':3,'VEC4':4,'MAT4':16}

def observed(path):
    data=path.read_bytes();n=unpack_from('<I',data,12)[0];j=json.loads(data[20:20+n]);bin=memoryview(data)[28+n:];processed=0
    def access(i):
        a=j['accessors'][i];v=j['bufferViews'][a['bufferView']];width=components[a['type']]*calcsize(formats[a['componentType']]);start=v.get('byteOffset',0)+a.get('byteOffset',0)
        return bin[start:start+v['byteLength']],width,v.get('byteStride',width),a['componentType']
    def content(i):
        b,width,stride,kind=access(i)
        return [kind,width,sha256(b).hexdigest()]
    runs=[]
    for m in j['meshes']:
        for p in m['primitives']:
            b,_,_,kind=access(p['indices']);indices=b.cast(formats[kind]);attrs=[(k,*access(i)) for k,i in sorted(p['attributes'].items())]
            for span in p.get('extras',{}).get('solids',[{'id':None,'first':0,'count':len(indices)}]):
                first=span['first'];count=span['count'];tags=[]
                for k,b,width,stride,kind in attrs:
                    h=sha256()
                    for v in indices[first:first+count]:h.update(b[v*stride:v*stride+width]);
                    tags.append((k,kind,h.hexdigest()))
                runs.append((m['name'],p['mode'],p.get('material'),p.get('extensions'),span['id'],count,tags));processed+=count
    for skin in j.get('skins',[]):
        if 'inverseBindMatrices' in skin:skin['inverseBindMatrices']=content(skin['inverseBindMatrices'])
    for clip in j.get('animations',[]):
        for s in clip['samplers']:
            for field in ('input','output'):s[field]=content(s[field])
    graph={key:j.get(key) for key in ('asset','nodes','skins','animations','scene','scenes','materials','extensionsUsed','extensionsRequired','extensions')}
    return graph,runs,processed,len(data)

base,now=(observed(out/name) for name in sys.argv[1:3]);assert base[0]==now[0]
if base[1]!=now[1]:
    for i,(a,b) in enumerate(zip(base[1],now[1])):
        if a!=b:raise ValueError((i,a[:6],b[:6],a[6],b[6]))
    raise ValueError((len(base[1]),len(now[1])))
print(json.dumps({'scenes':len(base[0]['scenes']),'primitiveSolidRanges':len(base[1]),'allIndexedVertices':base[2],'before':base[3],'after':now[3],'identical':True}))
