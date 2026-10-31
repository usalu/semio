from pathlib import Path
import json
ROOT=Path(__file__).resolve().parent
exec((ROOT/'scan-reference.py').read_text(encoding='utf8').split("view=data['bufferViews'][data['images'][0]['bufferView']]")[0])
S=46.81675036113457
mask=(abs(centres[:,0])<.05)&(centres[:,2]>-.07)&(centres[:,2]<.025)&(unit[:,1]>.85)
hist,edges=np.histogram(centres[mask,1],bins=np.arange(.25,.76,.001),weights=areas[mask])
peaks,_=find_peaks(hist,prominence=.00003,distance=8)
rows=[]
for k in peaks:
 y=(edges[k]+edges[k+1])/2
 pick=mask&(abs(centres[:,1]-y)<.005)
 if areas[pick].sum()<.0001:continue
 pp=tri[pick].reshape(-1,3)
 rows.append({'raw_y':float(y),'area':float(areas[pick].sum()),'bounds':[pp.min(axis=0).tolist(),pp.max(axis=0).tolist()],'triangles':int(pick.sum())})
(OUT/'gluon-bridge-peaks.json').write_text(json.dumps(rows,indent=2))
print(json.dumps(rows,indent=2))

exec((ROOT/'scan-reference.py').read_text(encoding='utf8').split("view=data['bufferViews'][data['images'][0]['bufferView']]")[0])
from PIL import Image,ImageDraw
im=Image.new('RGB',(1200,1100),'white');dr=ImageDraw.Draw(im)
rows=[]
for bi,(lo,hi) in enumerate(((.33,.38),(.51,.56),(.69,.74))):
 sel=(abs(centres[:,0])<.065)&(centres[:,1]>lo)&(centres[:,1]<hi)&(centres[:,2]>-.07)&(centres[:,2]<.025)
 for face in tri[sel]:
  poly=[(200+bi*400+float(q[0])*2600,350-float(q[2])*2600) for q in face]
  dr.polygon(poly,fill=(170,190,220),outline=(80,100,130))
 dr.text((20+bi*400,20),str((lo,hi)),fill='black')
 # Exact x=0 slice of every triangle, not centroid-based rectangular bounds.
 segments=[]
 for face in tri:
  if face[:,0].min()>0 or face[:,0].max()<0:continue
  pts=[]
  for a,b in zip(face,np.roll(face,-1,axis=0)):
   if a[0]*b[0]<0:
    q=a+(b-a)*(-a[0]/(b[0]-a[0]));pts.append(q)
  if len(pts)==2 and lo<np.mean(pts,axis=0)[1]<hi:
   segments.append([q.tolist() for q in pts])
   dr.line([(200+bi*400+float(q[2])*3000,950-(float(q[1])-lo)*6000) for q in pts],fill='black',width=2)
 rows.append({'band':[lo,hi],'x0_segments':segments})
im.save(OUT/'gluon-bridge-sections.png')
(OUT/'gluon-bridge-sections.json').write_text(json.dumps(rows))
