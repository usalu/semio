from pathlib import Path
ROOT=Path(__file__).resolve().parent
exec((ROOT/'scan-reference.py').read_text(encoding='utf8').split("canvas=Image.new")[0])
from scipy.ndimage import sobel,map_coordinates
S=46.81675036113457;ppm=16000;results=[]
cases=[('B11-P',2,0,1,-.0615,(abs(centres[:,1]-.6955)<.005)&(unit[:,1]>.9)&(centres[:,0]>-.11)&(centres[:,0]<-.075)&(centres[:,2]<-.03),(-.155,-.05),.6955),('B10-S',0,2,-1,-.116,(abs(centres[:,1]-.6755)<.004)&(unit[:,1]>.9)&(centres[:,2]>-.10)&(centres[:,2]<-.08)&(centres[:,0]<-.09),(-.22,-.115),.6755)]
for label,longaxis,cameraaxis,side,face,roofmask,fallback,roof in cases:
 rr=tri[roofmask].reshape(-1,3)
 bounds=[float(rr[:,longaxis].min()),float(rr[:,longaxis].max())]if len(rr)else None
 print('[DEBUG] roof',label,int(roofmask.sum()),bounds,flush=True)
 # A patch must approach both observed physical ends; otherwise it is explicitly incomplete.
 complete=bounds is not None and (bounds[1]-bounds[0])*S>3.8
 ends=bounds if complete else list(fallback);mid=sum(ends)/2
 u0,u1=ends[0]-.006,ends[1]+.006;y0,y1=roof-.065,roof+.008
 width,height=round((u1-u0)*ppm),round((y1-y0)*ppm);pixels=np.full((height,width,3),245,dtype=np.uint8);depth=np.full((height,width),-100.,dtype=float)
 selected=np.flatnonzero((abs(centres[:,cameraaxis]-face)<.009)&(unit[:,cameraaxis]*side>.80)&(centres[:,longaxis]>u0-.003)&(centres[:,longaxis]<u1+.003)&(centres[:,1]>y0)&(centres[:,1]<y1))
 for number in selected:
  ids=indices[number];p=points[ids];xx=(p[:,longaxis]-u0)*ppm;yy=(y1-p[:,1])*ppm;xmin,xmax=max(0,int(np.floor(xx.min()))),min(width-1,int(np.ceil(xx.max())));ymin,ymax=max(0,int(np.floor(yy.min()))),min(height-1,int(np.ceil(yy.max())))
  if xmin>xmax or ymin>ymax:continue
  den=(yy[1]-yy[2])*(xx[0]-xx[2])+(xx[2]-xx[1])*(yy[0]-yy[2])
  if abs(den)<1e-12:continue
  gx,gy=np.meshgrid(np.arange(xmin,xmax+1)+.5,np.arange(ymin,ymax+1)+.5);a=((yy[1]-yy[2])*(gx-xx[2])+(xx[2]-xx[1])*(gy-yy[2]))/den;b=((yy[2]-yy[0])*(gx-xx[2])+(xx[0]-xx[2])*(gy-yy[2]))/den;c=1-a-b;dist=side*(a*p[0,cameraaxis]+b*p[1,cameraaxis]+c*p[2,cameraaxis]);current=depth[ymin:ymax+1,xmin:xmax+1];mask=(a>=0)&(b>=0)&(c>=0)&(dist>current)
  if not mask.any():continue
  tuv=np.mod(np.stack((a[mask],b[mask],c[mask]),axis=1)@uv[ids],1);tx=np.minimum((tuv[:,0]*tex.shape[1]).astype(int),tex.shape[1]-1);ty=np.minimum((tuv[:,1]*tex.shape[0]).astype(int),tex.shape[0]-1);pixels[ymin:ymax+1,xmin:xmax+1][mask]=tex[ty,tx];current[mask]=dist[mask]
 gray=np.asarray(Image.fromarray(pixels).convert('L')).astype(float);mag=np.hypot(sobel(gray,0),sobel(gray,1));angles=np.linspace(0,2*np.pi,128,endpoint=False);xxguess=(mid-.7/S-u0)*ppm;yyguess=(y1-roof+1.25/S)*ppm;best=None
 for yy in np.arange(yyguess-.25/S*ppm,yyguess+.25/S*ppm+1,8):
  for xx in np.arange(xxguess-.4/S*ppm,xxguess+.4/S*ppm+1,8):
   for radius in np.arange(.5/S*ppm,.78/S*ppm,8):
    score=float(np.quantile(map_coordinates(mag,[yy+radius*np.sin(angles),xx+radius*np.cos(angles)],order=1,mode='constant',cval=0),.35))
    if best is None or score>best['score']:best={'score':score,'raw_longitudinal':float(u0+xx/ppm),'raw_y':float(y1-yy/ppm),'radius_m':float(radius/ppm*S),'offset_from_end_midpoint_m':float(abs(u0+xx/ppm-mid)*S)if complete else None}
 Image.fromarray(pixels).save(OUT/f'gluon-raw-side-{label}.png')
 results.append({'label':label,'long_axis':longaxis,'camera_axis':cameraaxis,'camera_sign':side,'roof_triangles':int(roofmask.sum()),'roof_end_bounds':bounds,'full_length_sample':complete,'roof_span_m':(bounds[1]-bounds[0])*S if bounds else None,'fit':best,'selected_wall_triangles':len(selected)})
(OUT/'gluon-raw-B-side-centres.json').write_text(json.dumps(results,indent=2));print('[DEBUG]',json.dumps(results),flush=True)
