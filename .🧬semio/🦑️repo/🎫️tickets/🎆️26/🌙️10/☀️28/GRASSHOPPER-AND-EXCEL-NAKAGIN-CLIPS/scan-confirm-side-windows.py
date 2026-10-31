from pathlib import Path
import json,math
ROOT=Path(__file__).resolve().parent
exec((ROOT/'scan-reference.py').read_text(encoding='utf8').split("view=data['bufferViews'][data['images'][0]['bufferView']]")[0])
from scipy.ndimage import sobel,map_coordinates
sides=json.loads((OUT/'gluon-unresolved-side-walls.json').read_text())['rows'];original=json.loads((OUT/'gluon-all-capsule-coverage.json').read_text())['rows']
image=np.asarray(Image.open(OUT/'gluon-unresolved-sides-1.png').convert('L')).astype(float)
result=[]
for index in range(0,18,2):
 q=sides[index];old=next(r for r in original if(r['tower'],r['floor'],r['facade'],r['item'])==(q['tower'],q['floor'],q['facade'],q['item']))
 family='P' if q['item']==0 else 'S'
 x,y,z=q['center'];ang=math.radians(q['rotation']);c,s=math.cos(ang),math.sin(ang)
 mx=-46.25*(centres[:,0]+.12);my=46.25*centres[:,2];lx=(mx-x)*c+(my-y)*s;ly=-(mx-x)*s+(my-y)*c
 inside=(abs(lx)<1.10)&(abs(ly)<1.84);scores=[]
 for phase in range(3):
  roof=.2182+(q['floor']-3+phase/3)*.05965845070422535
  up=inside&(unit[:,1]>.9)&(abs(centres[:,1]-roof)<.005);down=inside&(unit[:,1]<-.9)&(abs(centres[:,1]-roof+.058)<.005)
  scores.append({'phase':phase,'roof_area':float(areas[up].sum()),'underside_area':float(areas[down].sum())})
 pix=image[(index//5)*160+23:(index//5)*160+155,(index%5)*200+5:(index%5)*200+195]
 gy,gx=sobel(pix,0),sobel(pix,1);mag=np.hypot(gx,gy);scale=min(190/4.3,132/2.9);angles=np.linspace(0,2*np.pi,96,endpoint=False);best=None
 for yy in np.arange((2.7-1.25)*scale-16,(2.7-1.25)*scale+17,2):
  for xx in np.arange(95+.95*scale-20,95+.95*scale+21,2):
   for rr in np.arange(.5*scale,.82*scale,2):
    score=float(np.quantile(map_coordinates(mag,[yy+rr*np.sin(angles),xx+rr*np.cos(angles)],order=1,mode='constant',cval=0),.35))
    if best is None or score>best['edge_score']:best={'cx':float(xx),'cy':float(yy),'radius':float(rr),'edge_score':score,'height_above_roof_minus_nominal_shell':float(2.7-yy/scale),'longitudinal_centre_from_body_centre':float((xx-95)/scale),'expected_longitudinal_centre':.95}
 roof=.2182+(q['floor']-3+q['phase']/3)*.05965845070422535;observed=(roof-.00116284081093258)*46.81675036113457-2.5+best['height_above_roof_minus_nominal_shell'];best['world_height_residual']=observed-(z+1.25)
 result.append({'tower':'B','floor':q['floor'],'facade':q['facade'],'item':q['item'],'old_family':old['family'],'family':family,'center':q['center'],'rotation':q['rotation'],'retained_phase':q['phase'],'phase_scores':scores,'circle_height_fit':best,'positive_wall_evidence':{'image':'gluon-unresolved-sides-1.png','cell_zero_based':index,'manual':'clear actual right long-wall circular rim; unchanged body/entry'},'other_occupied_duplicate':[]})
(OUT/'gluon-additional-window-candidates.json').write_text(json.dumps({'rows':result,'count':len(result)},indent=2))
for q in result:print(q['floor'],q['item'],q['family'],'p',q['retained_phase'],'scores',[round(v['roof_area']+v['underside_area'],5)for v in q['phase_scores']],'fit',q['circle_height_fit'],flush=True)
