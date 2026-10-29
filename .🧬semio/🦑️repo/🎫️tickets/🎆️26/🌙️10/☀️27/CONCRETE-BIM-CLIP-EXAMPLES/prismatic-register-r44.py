import json,numpy as np
from scipy.optimize import least_squares
from scipy.linalg import rq
from pathlib import Path
base=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES')
c=json.loads((base/'prismatic-register-r44.json').read_text())
X=np.array([r['world'] for r in c['controls']]);uv=np.array([r['pixel'] for r in c['controls']]);xh=np.c_[X,np.ones(len(X))]
def normal(points):
 n=points.shape[1];mean=points.mean(axis=0);s=np.sqrt(n)/np.sqrt(((points-mean)**2).sum(axis=1).mean());T=np.eye(n+1);T[:n,:n]*=s;T[:n,n]=-s*mean;return T
T,U=normal(uv),normal(X);xx=(U@xh.T).T;yy=(T@np.c_[uv,np.ones(len(uv))].T).T
A=[]
for p,q in zip(xx,yy):
 A.extend([np.r_[p,np.zeros(4),-q[0]*p],np.r_[np.zeros(4),p,-q[1]*p]])
s=np.linalg.svd(np.array(A));P=np.linalg.inv(T)@s[2][-1].reshape(3,4)@U;P/=P[-1,-1]
def proj(points,P=P):
 q=(P@np.c_[np.atleast_2d(points),np.ones(len(np.atleast_2d(points)))].T).T;return q[:,:2]/q[:,2:]
def residual(p):return(proj(X,np.r_[p,1].reshape(3,4))-uv).ravel()
fit=least_squares(residual,P.ravel()[:-1]);P=np.r_[fit.x,1].reshape(3,4)
K,R=rq(P[:,:3]);K/=K[2,2];center=-np.linalg.solve(P[:,:3],P[:,3])
print('P',P.tolist());print('camera',center.tolist(),'intrinsics',K.tolist());print('rms',float(np.sqrt(np.mean(residual(fit.x)**2))));print('control residuals',(proj(X,P)-uv).tolist())
print('boiler predicted',proj([[6.64,11.5,9.49]],P).tolist())
pts=np.array([[x,y,6.6] for x,y in [(14.25,1.25),(14.25,3),(2,3),(2,5),(6.64,11.5)]])
print('roof control projection',proj(pts,P).tolist())
(base/'🗑️generated'/'prismatic-camera-r44.json').write_text(json.dumps({'matrix':P.tolist(),'center':center.tolist(),'intrinsics':K.tolist(),'rms':float(np.sqrt(np.mean(residual(fit.x)**2))),'residuals':(proj(X,P)-uv).tolist(),'boiler_prediction':proj([[6.64,11.5,9.49]],P).tolist()},indent=2))

from itertools import product
uv6=uv[:6]
results=[]
for swap,flipx,flipy in product([False,True],repeat=3):
 def world(u,v,z):
  q=[u,v] if not swap else [v,u]
  if flipx:q[0]=1-q[0]
  if flipy:q[1]=1-q[1]
  return[-.15+19.3*q[0],-.05+21.6*q[1],z]
 X6=np.array([world(u,v,z) for z in [6.77,2.91] for u,v in [(0,0),(1,0),(0,1)]])
 xx=np.c_[X6,np.ones(6)];AA=[]
 for p,q in zip(xx,uv6):AA.extend([np.r_[p,np.zeros(4),-q[0]*p],np.r_[np.zeros(4),p,-q[1]*p]])
 PP=np.linalg.svd(np.array(AA))[2][-1].reshape(3,4);PP/=PP[2,3]
 pred=proj([[6.64,11.5,9.49],[8.4,2.25,8.475],[2.0,3.5,9.5],[14.25,2,8.5]],PP)
 results.append({'swap':swap,'flipx':flipx,'flipy':flipy,'boiler':pred[0].tolist(),'boiler_error':float(np.linalg.norm(pred[0]-[369,45])),'SC1aperture':pred[1].tolist(),'kitchen':pred[2].tolist(),'salon':pred[3].tolist(),'P':PP.tolist()})
print(json.dumps(sorted(results,key=lambda r:r['boiler_error']),indent=2))
(base/'🗑️generated'/'prismatic-orientations-r44.json').write_text(json.dumps(results,indent=2))

for r in results:
 PP=np.array(r['P']);sgn=np.sign(np.linalg.det(PP[:,:3]));kk,rot=rq(PP[:,:3]*sgn);D=np.diag(np.sign(np.diag(kk)));kk=kk@D;rot=D@rot;C=-np.linalg.solve(PP[:,:3],PP[:,3]);depth=(rot@(np.array([9.5,10.75,3])-C))[2];r['positive_depth']=bool(depth>0);r['depth']=float(depth);r['camera_center']=C.tolist()
(base/'🗑️generated'/'prismatic-orientations-r44.json').write_text(json.dumps(results,indent=2))

# Native actual-building overlay continuation
import importlib
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();g=api.building_geometry(m)
lines=[]
for tag,color in [('SC-1','#ff3030'),('W-STAIR-ROOF','#00bb30')]:
 item=next(x for x in g['screens'] if x['tag']==tag)
 for z in [item['base'],item['top']]:lines.append({'tag':tag+':'+str(z),'color':color,'points':[[*p,z] for p in item['path']]})
 for o in item['openings']:
  lines.append({'tag':o['tag'],'color':'#00cfff' if tag=='SC-1' else '#b050ff','points':[[*o['p0'],o['sill_z']],[*o['p1'],o['sill_z']],[*o['p1'],o['head_z']],[*o['p0'],o['head_z']],[*o['p0'],o['sill_z']]]})
sl=next(x for x in g['slabs'] if x['tag']=='SL-STAIR-WEATHER');lines.append({'tag':'canopy','color':'#00bb30','points':[[*p,sl['top']] for p in sl['outline']+[sl['outline'][0]]]})
lines.append({'tag':'boiler','color':'#ffff00','points':[[6.64,11.5,6.6],[6.64,11.5,9.49]]})
actual_overlays=[]
for r in results:
 if not r['positive_depth']:continue
 PP=np.array(r['P']);actual_overlays.append({'case':[r['swap'],r['flipx'],r['flipy']],'camera':r['camera_center'],'lines':[{'tag':l['tag'],'color':l['color'],'points':proj(l['points'],PP).tolist()} for l in lines]})
(base/'🗑️generated'/'prismatic-native-overlays-r44.json').write_text(json.dumps(actual_overlays,indent=2))
print('Native multi-feature projections saved',len(actual_overlays))

more=[]
for item in g['railings']:
 if item['tag']!='TUBE-RM-2':continue
 for part in item['parts']:
  vertices=np.array(api.part_vertices(part));half=len(vertices)//2
  more.append({'tag':'actual-ramp-rail','color':'#ff00ff','points':[vertices[:half].mean(axis=0).tolist(),vertices[half:].mean(axis=0).tolist()]})
for cat in ['details','furnishings','beams']:
 for item in g[cat]:
  if item['tag'] not in ['SC-SILL','SC-HEAD']:continue
  for part in item['parts']:
   more.append({'tag':item['tag'],'color':'#ff8800','points':[[*p,part['z1']] for p in part['outline']+[part['outline'][0]]]})
for i,r in enumerate([x for x in results if x['positive_depth']]):
 PP=np.array(r['P']);actual_overlays[i]['lines'] += [{'tag':l['tag'],'color':l['color'],'points':proj(l['points'],PP).tolist()} for l in more]
(base/'🗑️generated'/'prismatic-native-overlays-r44.json').write_text(json.dumps(actual_overlays,indent=2))
for row in actual_overlays:
 op=np.array(next(l['points'] for l in row['lines'] if l['tag']=='OP-SC-VIEW'));rails=[l['points'] for l in row['lines'] if l['tag']=='actual-ramp-rail'];from shapely.geometry import Polygon,LineString
 aperture=Polygon(op);cross=[p for p in rails if LineString(p).intersects(aperture)]
 print(row['case'],'native aperture versus rail overlaps',len(cross),'of',len(rails))
