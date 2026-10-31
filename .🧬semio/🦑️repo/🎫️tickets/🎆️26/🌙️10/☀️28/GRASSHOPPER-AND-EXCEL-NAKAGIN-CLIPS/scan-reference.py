from pathlib import Path
import json
import math
import struct
import numpy as np
from PIL import Image, ImageDraw
from scipy.signal import find_peaks
import io

ROOT=Path(__file__).resolve().parent
OUT=ROOT/'🗑️generated'
OUT.mkdir(exist_ok=True)
reference=OUT/'gluon-reference.glb'
if not reference.exists():
    from urllib.request import urlopen
    reference.write_bytes(urlopen('https://gluon.tokyo/nakagin/models/NakaginCapsuleTower.glb',timeout=30).read())
raw=reference.read_bytes()
length=struct.unpack_from('<I',raw,12)[0]
data=json.loads(raw[20:20+length])
blob=raw[28+length:]

def read(index):
    """📦 Decode a raw glTF accessor without changing reference geometry."""
    item=data['accessors'][index]
    view=data['bufferViews'][item['bufferView']]
    size={'SCALAR':1,'VEC2':2,'VEC3':3}[item['type']]
    dtype={5125:'<u4',5126:'<f4'}[item['componentType']]
    offset=view.get('byteOffset',0)+item.get('byteOffset',0)
    return np.ndarray((item['count'],size),dtype=dtype,buffer=blob,offset=offset,strides=(view.get('byteStride',size*4),4)).copy()

points=read(1)
indices=read(0).reshape(-1,3)
uv=read(2)
tri=points[indices]
centres=tri.mean(axis=1)
normals=np.cross(tri[:,1]-tri[:,0],tri[:,2]-tri[:,0])
areas=np.linalg.norm(normals,axis=1)/2
unit=normals/np.maximum(2*areas[:,None],1e-15)
vertical=np.abs(unit[:,1])<.15
angles=np.mod(np.arctan2(unit[vertical,2],unit[vertical,0]),math.pi/2)
hist,edges=np.histogram(angles,180,range=(0,math.pi/2),weights=areas[vertical])
angle=float((edges[hist.argmax()]+edges[hist.argmax()+1])/2)
horizontal=np.abs(unit[:,1])>.95
bins=np.arange(-.02,1.16,.001)
yhist,ys=np.histogram(centres[horizontal,1],bins,weights=areas[horizontal])
peaks,_=find_peaks(yhist,prominence=.00003,distance=4)
stats={'dominant_axis_deg':math.degrees(angle),'horizontal_levels':[(round(float((ys[i]+ys[i+1])/2),5),round(float(yhist[i]),6)) for i in peaks],'bounds':[points.min(axis=0).tolist(),points.max(axis=0).tolist()]}
(OUT/'gluon-analysis.json').write_text(json.dumps(stats,indent=2))
print('[DEBUG] Reference feature statistics saved',flush=True)
view=data['bufferViews'][data['images'][0]['bufferView']]
texture=Image.open(io.BytesIO(blob[view['byteOffset']:view['byteOffset']+view['byteLength']])).convert('RGB')
tex=np.asarray(texture)
coords=uv[indices].mean(axis=1)
u=np.mod(coords[:,0],1)
v=np.mod(coords[:,1],1)
colors=tex[np.minimum((v*tex.shape[0]).astype(int),tex.shape[0]-1),np.minimum((u*tex.shape[1]).astype(int),tex.shape[1]-1)]
canvas=Image.new('RGB',(1600,1500),(245,245,245))
draw=ImageDraw.Draw(canvas)
for number,theta in enumerate((angle,angle+math.pi/2,angle+math.pi,angle+3*math.pi/2)):
    cx,cy=(number%2)*800,(number//2)*750
    projected=np.column_stack((points[:,0]*math.cos(theta)+points[:,2]*math.sin(theta),points[:,1],-points[:,0]*math.sin(theta)+points[:,2]*math.cos(theta)))
    scale=min(740/np.ptp(projected[:,0]),690/np.ptp(projected[:,1]))
    x=cx+400+(projected[:,0]-(projected[:,0].min()+projected[:,0].max())/2)*scale
    y=cy+720-(projected[:,1]-projected[:,1].min())*scale
    depth=projected[indices,2].mean(axis=1)
    for k in np.argsort(depth):
        face=indices[k]
        polygon=[(float(x[i]),float(y[i])) for i in face]
        draw.polygon(polygon,fill=tuple(int(c) for c in colors[k]))
    draw.text((cx+20,cy+10),f'Reference direction {number}, axis {math.degrees(theta):.2f} deg',fill=(0,0,0))
canvas.save(OUT/'gluon-reference-exteriors.png')
print('[DEBUG] Reference projections saved',flush=True)
