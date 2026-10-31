from pathlib import Path
import json,math
import numpy as np
from PIL import Image
from scipy.ndimage import sobel,map_coordinates
ROOT=Path(__file__).resolve().parent;OUT=ROOT/'🗑️generated'
rows=json.loads((OUT/'gluon-all-capsule-coverage.json').read_text())['rows'];angles=np.linspace(0,2*np.pi,96,endpoint=False);result=[]
for index,q in enumerate(rows):
 if q['tower']!='A' or q['window_wall']!='right':continue
 page=index//20+1;cell=index%20
 image=np.asarray(Image.open(OUT/f'gluon-aperture-contact-{page}.png').convert('L')).astype(float)
 pix=image[(cell//5)*160+23:(cell//5)*160+155,(cell%5)*200+5:(cell%5)*200+195]
 gy,gx=sobel(pix,0),sobel(pix,1);mag=np.hypot(gx,gy);scale=min(190/4.3,132/2.9);best=None
 for yy in np.arange((2.7-1.25)*scale-16,(2.7-1.25)*scale+17,2):
  for xx in np.arange(95+.95*scale-22,95+.95*scale+23,2):
   for rr in np.arange(.5*scale,.82*scale,2):
    score=float(np.quantile(map_coordinates(mag,[yy+rr*np.sin(angles),xx+rr*np.cos(angles)],order=1,mode='constant',cval=0),.35))
    if best is None or score>best['edge_score']:best={'u':float((xx-95)/scale),'h':float(2.7-yy/scale),'r':float(rr/scale),'edge_score':score}
 result.append({'tower':q['tower'],'floor':q['floor'],'facade':q['facade'],'item':q['item'],'family':q['family'],'fit':best,'image':f'gluon-aperture-contact-{page}.png','cell':cell})
(OUT/'gluon-side-window-centres.json').write_text(json.dumps(result,indent=2))
print('[DEBUG]',json.dumps(result),flush=True)
