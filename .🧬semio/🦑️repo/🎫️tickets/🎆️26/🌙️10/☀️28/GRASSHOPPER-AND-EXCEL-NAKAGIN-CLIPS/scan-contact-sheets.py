from pathlib import Path
import json,math
ROOT=Path(__file__).resolve().parent
exec((ROOT/'scan-reference.py').read_text(encoding='utf8').split("canvas=Image.new")[0])
coverage=json.loads((OUT/'gluon-all-capsule-coverage.json').read_text())
ppm=48
summaries=[]
for page in range(7):
 canvas=Image.new('RGB',(1000,640),(228,228,228));draw=ImageDraw.Draw(canvas)
 for k,row in enumerate(coverage['rows'][page*20:page*20+20]):
  x,y,z=row['center'];ang=math.radians(row['rotation']);c,s=math.cos(ang),math.sin(ang)
  mx=-46.25*(points[:,0]+.12);my=46.25*points[:,2]
  lx=(mx-x)*c+(my-y)*s;ly=-(mx-x)*s+(my-y)*c
  roof=row['phase_scores'][row['phase']]['roof_raw']
  hh=(points[:,1]-roof)*46.81675+2.5
  if row['window_wall']=='front':
   uu=lx;depth=ly;plane=2;wide=2.8
   nn=unit[:,0]*s+unit[:,2]*c
  else:
   uu=ly;depth=lx;plane=1.25;wide=4.3
   nn=-unit[:,0]*c+unit[:,2]*s
  width=190;height=132;scale=min(width/wide,height/2.9)
  pix=np.full((height,width,3),245,dtype=np.uint8);zbuffer=np.full((height,width),-100.,dtype=float)
  cmdep=depth[indices].mean(axis=1)
  cmh=hh[indices].mean(axis=1);cmu=uu[indices].mean(axis=1)
  selected=np.flatnonzero((abs(cmdep-plane)<.5)&(nn>.15)&(cmh>-.25)&(cmh<2.85)&(abs(cmu)<wide/2+.2))
  for number in selected:
   ids=indices[number];xx=width/2+uu[ids]*scale;yy=(2.7-hh[ids])*scale
   xmin,xmax=max(0,int(np.floor(xx.min()))),min(width-1,int(np.ceil(xx.max())))
   ymin,ymax=max(0,int(np.floor(yy.min()))),min(height-1,int(np.ceil(yy.max())))
   if xmin>xmax or ymin>ymax:continue
   den=(yy[1]-yy[2])*(xx[0]-xx[2])+(xx[2]-xx[1])*(yy[0]-yy[2])
   if abs(den)<1e-12:continue
   gx,gy=np.meshgrid(np.arange(xmin,xmax+1)+.5,np.arange(ymin,ymax+1)+.5)
   aa=((yy[1]-yy[2])*(gx-xx[2])+(xx[2]-xx[1])*(gy-yy[2]))/den
   bb=((yy[2]-yy[0])*(gx-xx[2])+(xx[0]-xx[2])*(gy-yy[2]))/den;cc=1-aa-bb
   dd=aa*depth[ids[0]]+bb*depth[ids[1]]+cc*depth[ids[2]]
   current=zbuffer[ymin:ymax+1,xmin:xmax+1]
   mask=(aa>=0)&(bb>=0)&(cc>=0)&(dd>current)
   if not mask.any():continue
   weights=np.stack((aa[mask],bb[mask],cc[mask]),axis=1)
   tuv=np.mod(weights@uv[ids],1)
   tx=np.minimum((tuv[:,0]*tex.shape[1]).astype(int),tex.shape[1]-1)
   ty=np.minimum((tuv[:,1]*tex.shape[0]).astype(int),tex.shape[0]-1)
   pix[ymin:ymax+1,xmin:xmax+1][mask]=tex[ty,tx];current[mask]=dd[mask]
  cx,cy=(k%5)*200,(k//5)*160
  canvas.paste(Image.fromarray(pix),(cx+5,cy+23))
  label=f"{row['tower']}{row['floor']:02d} {row['facade']}/{row['item']} {row['family']} p{row['phase']}"
  draw.text((cx+5,cy+5),label,fill='black')
  row['texture_contact_sheet']={'page':page+1,'cell':k,'triangles':len(selected),'coverage_fraction':float((zbuffer>-99).mean())}
 canvas.save(OUT/f'gluon-aperture-contact-{page+1}.png')
(OUT/'gluon-all-capsule-coverage.json').write_text(json.dumps(coverage,indent=2))
print('[DEBUG] Seven actual UV contact sheets saved',flush=True)
