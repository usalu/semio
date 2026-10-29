import json, numpy as np
from scipy.optimize import least_squares
from pathlib import Path
base=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES')
c=json.loads((base/'chimney-register-r40.json').read_text())
m=json.loads(Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json').read_text())
a,b=np.array(c['ramp']['farDeck'],float); aa,bb=np.array(c['ramp']['edgeSamples'],float)
def intersection(p,q,r,s):
 h=np.cross(np.cross([*p,1],[*q,1]),np.cross([*r,1],[*s,1]));return h[:2]/h[2]
vp=intersection(a,aa,b,bb)
contour=np.array(c['curveContour'],float)
poly=np.array(next(e for e in m['elements'] if e['tag']=='SL-STAIR-CANOPY')['geometry']['outline'])
slope=c['ramp']['slope']
def camera(f,w):
 s=(b[0]-a[0])/w;cx=vp[0];cy=vp[1]+f*slope
 C=np.array([8.8+((a[0]+b[0])/2-cx)/s,8.23+f/s,6.6+(430-cy)/s]);return cx,cy,C

def project(poly,z,f,w):
 cx,cy,C=camera(f,w);d=C[1]-poly[:,1]
 return np.c_[cx+f*(C[0]-poly[:,0])/d,cy-f*(z-C[2])/d]
def residual(p):
 f,w=p;q=project(poly,9.35,f,w);q=np.vstack([q,q[0]]);A=q[:-1];D=q[1:]-A
 v=contour[:,None,:]-A[None,:,:];t=np.clip(np.sum(v*D,axis=-1)/np.sum(D*D,axis=-1),0,1)
 dist=contour[:,None,:]-(A[None,:,:]+t[:,:,None]*D[None,:,:]);ii=np.argmin(np.sum(dist**2,axis=-1),axis=1)
 return dist[np.arange(len(contour)),ii].ravel()
rows=[]
for start in [200,400,800,1600]:
 fit=least_squares(residual,[start,1.2],bounds=([100,.8],[2500,1.6]),max_nfev=250)
 f,w=fit.x;cx,cy,C=camera(f,w);diam=.4;scale=(838-797)/diam
 X=C[0]-(818-cx)/scale;Y=C[1]-f/scale;Z=C[2]+(cy-130)/scale
 rows.append(dict(start=start,f=float(f),w=float(w),rms=float(np.sqrt(np.mean(residual(fit.x)**2))),camera=C.tolist(),principal=[float(cx),float(cy)],cylinder=[float(X),float(Y),float(Z)]))
print(json.dumps({'vanishing_point':vp.tolist(),'fits':rows},indent=2))
import importlib
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
wall=api.path_wall_geometry(m,next(e for e in m['elements'] if e['tag']=='W-STAIR-ROOF'))
stair=api.stair_geometry('ST-FR',m)
from shapely.geometry import Point, Polygon
from shapely.ops import unary_union
for r in rows:
 f,w=r['f'],r['w'];X,Y,Z=r['cylinder'];disk=Point(X,Y).buffer(.2,quad_segs=96)
 wp=unary_union([Polygon(p['outline']) for p in wall['parts'] if p['z0']<=8<=p['z1']])
 sp=unary_union([Polygon(p['outline']) for p in stair['parts'] if 'outline' in p])
 r['wall_material_overlap_area_at_z8']=disk.intersection(wp).area
 r['stair_projection_overlap_area']=disk.intersection(sp).area
 r['canopy_overlap_area']=disk.intersection(Polygon(poly)).area
 r['projectedCanopy']=project(poly,9.35,f,w).tolist()
 theta=np.linspace(0,2*np.pi,97);ring=np.c_[X+.2*np.cos(theta),Y+.2*np.sin(theta)]
 r['projectedMouth']=project(ring,Z,f,w).tolist()
 r['column19_distance']=float(np.linalg.norm(np.array([X,Y])-np.array([8.128,12.572])))
 r['nominal_plus_two_60mm_walls_sensitivity']=[float(camera(f,w)[2][0]-(818-vp[0])*.52/41),float(camera(f,w)[2][1]-f*.52/41),float(camera(f,w)[2][2]+(camera(f,w)[1]-130)*.52/41)]
(base/'🗑️generated'/'chimney-register-r40-results.json').write_text(json.dumps({'vp':vp.tolist(),'candidates':rows},indent=2))
print(json.dumps([{k:v for k,v in r.items() if not k.startswith('projected')} for r in rows],indent=2))

assert np.allclose(vp,[578,327],atol=1e-10)
assert abs(rows[0]['cylinder'][1]-rows[2]['cylinder'][1])>.8
assert rows[0]['rms']<c['curveContourUncertaintyPixels'] and rows[2]['rms']<c['curveContourUncertaintyPixels']
for row in [rows[0],rows[2]]:
 f,w=row['f'],row['w'];cx,cy,C=camera(f,w);d=C[1]-9.4
 old_x=cx+f*(C[0]-4.7)/d;old_width=f*.4/d
 print('counterfactual old trunk',old_x,old_width)
 assert abs(old_x-818)>60 and old_width<35
print('Two distinct image-compatible registrations and old-trunk counterfactual PASS')

saved_poly=poly.copy()
raw=next(e for e in m['elements'] if e['tag']=='SC-1')['geometry']['path']
sc=[]
for item in raw[:6]:
 if isinstance(item,dict):
  bez=np.array(item['bezier']);t=np.linspace(0,1,49)[:,None];sc.extend(((1-t)**3*bez[0]+3*(1-t)**2*t*bez[1]+3*(1-t)*t**2*bez[2]+t**3*bez[3]).tolist())
 else:sc.append(item)
poly=np.array([v for i,v in enumerate(sc) if not i or np.linalg.norm(np.array(v)-np.array(sc[i-1]))>1e-9])
sc_results=[]
for start in [200,400,800,1600]:
 fit=least_squares(residual,[start,1.2],bounds=([100,.8],[2500,1.6]),max_nfev=250)
 sc_results.append({'start':start,'f':float(fit.x[0]),'width':float(fit.x[1]),'rms':float(np.sqrt(np.mean(residual(fit.x)**2)))})
poly=saved_poly
print(json.dumps(sc_results,indent=2))
(base/'🗑️generated'/'chimney-sc1-counterfactual-r40.json').write_text(json.dumps(sc_results,indent=2))
