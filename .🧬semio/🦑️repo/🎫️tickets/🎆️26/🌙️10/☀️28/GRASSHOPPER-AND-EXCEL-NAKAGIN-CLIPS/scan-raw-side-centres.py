from pathlib import Path
ROOT=Path(__file__).resolve().parent
exec((ROOT/'scan-reference.py').read_text(encoding='utf8').split("canvas=Image.new")[0])
from scipy.ndimage import sobel,map_coordinates
components={q['component']:q for q in json.loads((OUT/'gluon-horizontal-components.json').read_text())}
results=[];ppm=16000;S=46.81675036113457
for label,component,side in (('A13-P',5046,1),('A13-S',3007,1)):
 comp=components[component];lo,hi=comp['bounds'];zmid=(lo[2]+hi[2])/2
 z0,z1=lo[2]-.006,hi[2]+.006;y0,y1=.790,.865
 width,height=round((z1-z0)*ppm),round((y1-y0)*ppm)
 pixels=np.full((height,width,3),245,dtype=np.uint8);depth=np.full((height,width),-100.,dtype=float)
 face=.1807 if label.endswith('P') else .2413
 selected=np.flatnonzero((abs(centres[:,0]-face)<.009)&(unit[:,0]*side>.80)&(centres[:,2]>z0-.003)&(centres[:,2]<z1+.003)&(centres[:,1]>y0)&(centres[:,1]<y1))
 for number in selected:
  ids=indices[number];p=points[ids];xx=(p[:,2]-z0)*ppm;yy=(y1-p[:,1])*ppm
  xmin,xmax=max(0,int(np.floor(xx.min()))),min(width-1,int(np.ceil(xx.max())));ymin,ymax=max(0,int(np.floor(yy.min()))),min(height-1,int(np.ceil(yy.max())))
  if xmin>xmax or ymin>ymax:continue
  den=(yy[1]-yy[2])*(xx[0]-xx[2])+(xx[2]-xx[1])*(yy[0]-yy[2])
  if abs(den)<1e-12:continue
  gx,gy=np.meshgrid(np.arange(xmin,xmax+1)+.5,np.arange(ymin,ymax+1)+.5)
  a=((yy[1]-yy[2])*(gx-xx[2])+(xx[2]-xx[1])*(gy-yy[2]))/den;b=((yy[2]-yy[0])*(gx-xx[2])+(xx[0]-xx[2])*(gy-yy[2]))/den;c=1-a-b
  dist=side*(a*p[0,0]+b*p[1,0]+c*p[2,0]);current=depth[ymin:ymax+1,xmin:xmax+1];mask=(a>=0)&(b>=0)&(c>=0)&(dist>current)
  if not mask.any():continue
  weights=np.stack((a[mask],b[mask],c[mask]),axis=1);tuv=np.mod(weights@uv[ids],1);tx=np.minimum((tuv[:,0]*tex.shape[1]).astype(int),tex.shape[1]-1);ty=np.minimum((tuv[:,1]*tex.shape[0]).astype(int),tex.shape[0]-1)
  pixels[ymin:ymax+1,xmin:xmax+1][mask]=tex[ty,tx];current[mask]=dist[mask]
 gray=np.asarray(Image.fromarray(pixels).convert('L')).astype(float);mag=np.hypot(sobel(gray,0),sobel(gray,1));angles=np.linspace(0,2*np.pi,128,endpoint=False)
 rawguess=zmid-.7/S;yyguess=(y1-(float(np.median([lo[1],hi[1]]))-1.25/S))*ppm
 xxguess=(rawguess-z0)*ppm;best=None
 for yy in np.arange(yyguess-.25/S*ppm,yyguess+.25/S*ppm+1,8):
  for xx in np.arange(xxguess-.4/S*ppm,xxguess+.4/S*ppm+1,8):
   for rr in np.arange(.50/S*ppm,.78/S*ppm,8):
    values=map_coordinates(mag,[yy+rr*np.sin(angles),xx+rr*np.cos(angles)],order=1,mode='constant',cval=0);score=float(np.quantile(values,.35))
    if best is None or score>best['score']:best={'score':score,'raw_z':float(z0+xx/ppm),'raw_y':float(y1-yy/ppm),'radius_m':float(rr/ppm*S),'offset_from_roof_midpoint_m':float(abs(z0+xx/ppm-zmid)*S)}
 Image.fromarray(pixels).save(OUT/f'gluon-raw-side-{label}.png')
 results.append({'label':label,'roof_component':component,'untruncated_roof_z_bounds':[lo[2],hi[2]],'raw_body_midpoint_z':zmid,'A_height_only_scale':S,'raw_roi':[z0,z1,y0,y1],'selected_triangles':len(selected),'fit':best})
(OUT/'gluon-raw-side-centres.json').write_text(json.dumps(results,indent=2));print('[DEBUG]',json.dumps(results),flush=True)
