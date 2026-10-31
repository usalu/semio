from pathlib import Path
import json,math
ROOT=Path(__file__).resolve().parent
exec((ROOT/'scan-reference.py').read_text(encoding='utf8').split("view=data['bufferViews'][data['images'][0]['bufferView']]")[0])
rows=json.loads((OUT/'gluon-all-capsule-coverage.json').read_text())['rows']
for file in ('gluon-confirmed-pose-candidates.json','gluon-additional-window-candidates.json'):
 for q in json.loads((OUT/file).read_text())['rows']:
  r=next(r for r in rows if(r['tower'],r['floor'],r['facade'],r['item'])==(q['tower'],q['floor'],q['facade'],q['item']))
  r.update(center=q['center'],rotation=q['rotation'],family=q['family'])
result=[]
def peak(v,w,mask,lo,hi):
 ids=np.flatnonzero(mask&(v>lo)&(v<hi))
 if len(ids)<3:return None
 bins=np.arange(lo,hi+.01,.01);hist,ed=np.histogram(v[ids],bins=bins,weights=w[ids]);centre=(ed[hist.argmax()]+ed[hist.argmax()+1])/2
 use=ids[abs(v[ids]-centre)<.055]
 if w[use].sum()<.00004:return None
 return float(np.average(v[use],weights=w[use])),float(w[use].sum())
for row in rows:
 x,y,z=row['center'];ang=math.radians(row['rotation']);c,s=math.cos(ang),math.sin(ang)
 mx=-46.25*(centres[:,0]+.12);my=46.25*centres[:,2]
 lx=(mx-x)*c+(my-y)*s;ly=-(mx-x)*s+(my-y)*c
 nx=-unit[:,0]*c+unit[:,2]*s;ny=unit[:,0]*s+unit[:,2]*c
 roof=(.2186 if row['tower']=='A'else.2182)+(row['floor']-3+row['phase']/3)*.05965845070422535
 h=(centres[:,1]-roof)*46.81675036113457+2.5
 flat=(abs(unit[:,1])<.12)&(h>.45)&(h<2.05)
 left=peak(lx,areas,flat&(nx<-.95)&(abs(ly)<1.5),-1.65,-1.05)
 right=peak(lx,areas,flat&(nx>.95)&(abs(ly)<1.5),1.05,1.65)
 back=peak(ly,areas,flat&(ny<-.95)&(abs(lx)<1.05),-2.45,-1.75)
 front=peak(ly,areas,flat&(ny>.95)&(abs(lx)<1.05),1.75,2.45)
 result.append({'tower':row['tower'],'floor':row['floor'],'facade':row['facade'],'item':row['item'],'family':row['family'],'left':left,'right':right,'back':back,'front':front,'width':right[0]-left[0]if left and right else None,'length':front[0]-back[0]if front and back else None})
(OUT/'gluon-flat-shell-planes.json').write_text(json.dumps(result,indent=2))
for name in ('width','length'):
 values=np.array([q[name]for q in result if q[name]is not None])
 print('[DEBUG]',name,'n',len(values),'median',np.median(values),'q10/q90',np.quantile(values,[.1,.9]),flush=True)
