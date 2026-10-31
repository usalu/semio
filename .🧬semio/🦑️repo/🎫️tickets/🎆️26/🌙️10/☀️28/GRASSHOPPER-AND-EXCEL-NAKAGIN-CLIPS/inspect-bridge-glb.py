"""🖼️ Project native bridge GLB triangles for read-only component inspection."""
import json
import importlib.util
import sys
from pathlib import Path
import struct
import numpy as np
from PIL import Image, ImageDraw

TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
blob=(ROOT/'🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/🌉️bridge/🧊️bridge.glb').read_bytes()
length=struct.unpack_from('<I',blob,12)[0]
gltf=json.loads(blob[20:20+length])
binary=blob[28+length:]

def accessor(index):
    a=gltf['accessors'][index]
    b=gltf['bufferViews'][a['bufferView']]
    count={'SCALAR':1,'VEC2':2,'VEC3':3,'VEC4':4}[a['type']]
    dtype={5126:'<f4',5125:'<u4',5123:'<u2',5121:'u1'}[a['componentType']]
    return np.frombuffer(binary,dtype=dtype,count=a['count']*count,offset=b.get('byteOffset',0)+a.get('byteOffset',0)).reshape(-1,count)

triangles=[]
components=[]
for i,mesh in enumerate(gltf['meshes']):
    points=[]
    for primitive in mesh['primitives']:
        if primitive.get('mode',4)!=4:
            continue
        gl=accessor(primitive['attributes']['POSITION'])
        v=np.column_stack((gl[:,0],-gl[:,2],gl[:,1]))
        points.append(v)
        ind=accessor(primitive['indices']).ravel().reshape(-1,3)
        triangles.extend((i,v[f]) for f in ind)
    if points:
        points=np.concatenate(points)
        components.append({'mesh':i,'min':points.min(0).tolist(),'max':points.max(0).tolist()})
if '--model' in sys.argv:
    spec=importlib.util.spec_from_file_location('bridge_inspection',ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin/🐍️.py')
    model=importlib.util.module_from_spec(spec)
    sys.modules[spec.name]=model
    spec.loader.exec_module(model)
    triangles=[]
    for part in model.bridge_assembly(6).parts:
        vertices=np.asarray(part.solid.vertices)
        for face in part.solid.faces:
            for i in range(1,len(face)-1):
                triangles.append((285 if part.role in ('bridge','bridge_base') else 264,vertices[[face[0],face[i],face[i+1]]]))
image=Image.new('RGB',(1800,900),'#f1f1ee')
draw=ImageDraw.Draw(image)
for index,eye in enumerate(((2,-3,2),(0,-1,.05))):
    towards=np.asarray(eye,dtype=float)
    towards/=np.linalg.norm(towards)
    right=np.cross([0,0,1],towards)
    right/=np.linalg.norm(right)
    up=np.cross(towards,right)
    points=np.concatenate([v for i,v in triangles])
    xy=np.column_stack((points@right,points@up))
    low,high=xy.min(0),xy.max(0)
    scale=min(800/(high[0]-low[0]),760/(high[1]-low[1]))
    middle=(low+high)/2
    for mesh,v in sorted(triangles,key=lambda row:np.mean(row[1]@towards)):
        pixel=(np.column_stack((v@right,v@up))-middle)*[scale,-scale]+[450+900*index,480]
        color='#d1cfc5' if mesh>=283 else '#728890'
        draw.polygon([tuple(x) for x in pixel],fill=color,outline='#455054',width=1)
    draw.text((30+index*900,25),('Reconstructed bridge: ' if '--model' in sys.argv else 'Native bridge GLB: ')+('axon' if index==0 else 'elevation'),fill='#222222',font_size=26)
image.save(TICKET/('🗑️generated/bridge-profile.png' if '--model' in sys.argv else '🗑️generated/bridge-native.png'))
(TICKET/'🗑️generated/bridge-glb.json').write_text(json.dumps(components,indent=2),encoding='utf8')
print('[DEBUG] Native bridge projections saved',flush=True)
