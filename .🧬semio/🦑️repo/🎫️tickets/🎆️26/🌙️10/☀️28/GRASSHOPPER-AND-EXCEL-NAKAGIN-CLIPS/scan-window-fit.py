from pathlib import Path
import json
import numpy as np
from PIL import Image
from scipy import ndimage
from scipy.optimize import least_squares
ROOT=Path(__file__).resolve().parent
OUT=ROOT/'🗑️generated'
rgb=np.asarray(Image.open(OUT/'gluon-window-strip.png').convert('RGB'))
gray=rgb.mean(axis=2)
rows={}
for name,y0,y1 in (('top',200,850),('upper',850,1550),('middle',1950,2650),('broken',3150,3750),('lower',3800,4450),('lowest',6150,6850)):
    region=gray[y0:y1,80:600]
    results=[]
    for threshold in (70,80,90,100,110,120,130):
        labels,count=ndimage.label(region<threshold)
        sizes=np.bincount(labels.ravel()); sizes[0]=0
        mask=ndimage.binary_fill_holes(labels==sizes.argmax())
        edge=mask&~ndimage.binary_erosion(mask,iterations=2)
        yy,xx=np.nonzero(edge)
        cx,cy=xx.mean(),yy.mean()
        rr=np.hypot(xx-cx,yy-cy)
        good=(rr>np.percentile(rr,10))&(rr<np.percentile(rr,90))
        fit=least_squares(lambda p:np.hypot(xx[good]-p[0],yy[good]-p[1])-p[2],(cx,cy,np.median(rr)))
        cx,cy,r=fit.x
        results.append({'threshold':threshold,'centre':[float(cx+80),float(cy+y0)],'radius_px':float(r),'fit_rms':float(np.sqrt(np.mean(fit.fun**2))),'bbox':[int(xx.min()+80),int(yy.min()+y0),int(xx.max()+80),int(yy.max()+y0)]})
    center=np.array(results[4]['centre'])
    radii=np.arange(90,181,1.)
    angles=np.linspace(0,2*np.pi,720,endpoint=False)
    xx=center[0]+radii[:,None]*np.cos(angles)
    yy=center[1]+radii[:,None]*np.sin(angles)
    values=ndimage.map_coordinates(gray,(yy,xx),order=1)
    profile=np.median(values,axis=1)
    derivative=np.gradient(ndimage.gaussian_filter1d(profile,1))
    rows[name]={'threshold_fits':results,'radial_profile':[(float(r),round(float(v),2),round(float(d),2)) for r,v,d in zip(radii,profile,derivative)]}
(OUT/'gluon-window-fits.json').write_text(json.dumps(rows,indent=2))
for name,row in rows.items():
    print(name,row['threshold_fits'],flush=True)
    print('radial peaks',sorted(row['radial_profile'],key=lambda x:-x[2])[:8],flush=True)
