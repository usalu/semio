from pathlib import Path
import json,math
ROOT=Path(__file__).resolve().parent
exec((ROOT/'scan-reference.py').read_text(encoding='utf8').split("view=data['bufferViews'][data['images'][0]['bufferView']]")[0])
from scipy.ndimage import sobel,map_coordinates
alts=json.loads((OUT/'gluon-aperture-alternatives.json').read_text())['rows']
original=json.loads((OUT/'gluon-all-capsule-coverage.json').read_text())['rows']
chosen=[2,5,13,16,21,23,24,27,28,31,32,34,35,40,41,46,47,49,50,52,56,61,62,63]
result=[]
for index in chosen:
 row=alts[index];old=next(r for r in original if(r['tower'],r['floor'],r['facade'],r['item'])==(row['tower'],row['floor'],row['facade'],row['item']))
 x,y,z=row['center'];ang=math.radians(row['rotation']);c,s=math.cos(ang),math.sin(ang)
 mx=-46.25*(centres[:,0]+.12);my=46.25*centres[:,2]
 lx=(mx-x)*c+(my-y)*s;ly=-(mx-x)*s+(my-y)*c
 inside=(abs(lx)<1.10)&(abs(ly)<1.84)
 scores=[]
 for phase in range(3):
  roof=(.2186 if row['tower']=='A' else .2182)+(row['floor']-3+phase/3)*.05965845070422535
  up=inside&(unit[:,1]>.9)&(abs(centres[:,1]-roof)<.005)
  down=inside&(unit[:,1]<-.9)&(abs(centres[:,1]-roof+.058)<.005)
  scores.append({'phase':phase,'roof_area':float(areas[up].sum()),'underside_area':float(areas[down].sum())})
 page=index//20+1;cell=index%20;image=np.asarray(Image.open(OUT/f'gluon-aperture-alternative-{page}.png').convert('L'))[(cell//5)*160+23:(cell//5)*160+23+132,(cell%5)*200+5:(cell%5)*200+195].astype(float)
 gy,gx=sobel(image,0),sobel(image,1)
 # Circle-like radial image contrast; corroboration of centre height, not surveyed diameter.
 scale=min(190/(2.8 if row['window_wall']=='front' else 4.3),132/2.9)
 expected_x=95+(0 if row['window_wall']=='front' else .95)*scale
 expected_y=(2.7-1.25)*scale
 best=None
 angles=np.linspace(0,2*np.pi,96,endpoint=False)
 for yy in np.arange(expected_y-16,expected_y+17,2):
  for xx in np.arange(expected_x-12,expected_x+13,2):
   for rr in np.arange(.5*scale,.82*scale,2):
    xs=xx+rr*np.cos(angles);ys=yy+rr*np.sin(angles)
    values=map_coordinates(np.hypot(gx,gy),[ys,xs],order=1,mode='constant',cval=0)
    score=float(np.quantile(values,.35))
    if best is None or score>best['edge_score']:best={'cx':float(xx),'cy':float(yy),'radius':float(rr),'edge_score':score,'height_from_nominal_floor':float(2.7-yy/scale),'expected_height':1.25,'height_convention':'above roof-minus-nominal2.5m outer shell baseline; finished floor is +.2m','finished_floor_equivalent_expected_height':1.05}
 dup=[(r['facade'],r['item'])for r in original if r['tower']==row['tower']and r['floor']==row['floor']and(r['facade'],r['item'])!=(row['facade'],row['item'])and max(abs(r['center'][j]-row['center'][j])for j in range(2))<.01 and(r['rotation']-row['rotation'])%180==0 and r['phase']==row['phase']]
 result.append({'tower':row['tower'],'floor':row['floor'],'facade':row['facade'],'item':row['item'],'old_family':old['family'],'family':row['family'].removesuffix(' ALT'),'center':row['center'],'rotation':row['rotation'],'retained_phase':row['phase'],'phase_scores':scores,'circle_height_fit':best,'other_occupied_duplicate':dup,'positive_wall_evidence':{'image':f'gluon-aperture-alternative-{page}.png','cell_zero_based':cell,'manual':'clear actual circular rim; B11N1 is partial capture','original_coverage':old['texture_contact_sheet']['coverage_fraction'],'candidate_coverage':row['texture_contact_sheet']['coverage_fraction']}})
(OUT/'gluon-confirmed-pose-candidates.json').write_text(json.dumps({'rows':result,'count':len(result),'method':'actual UV triangles plus signed horizontal surfaces; candidate metric XY from native canonical pose, not surveyed exact; phase retained and circle centre corroboration bounded'},indent=2))
for r in result:
 print(r['tower'],r['floor'],r['facade'],r['item'],r['family'],'p',r['retained_phase'],'scores',[round(q['roof_area']+q['underside_area'],5)for q in r['phase_scores']],'windowh',round(r['circle_height_fit']['height_from_nominal_floor'],2),'edge',round(r['circle_height_fit']['edge_score']),'dup',r['other_occupied_duplicate'],flush=True)
