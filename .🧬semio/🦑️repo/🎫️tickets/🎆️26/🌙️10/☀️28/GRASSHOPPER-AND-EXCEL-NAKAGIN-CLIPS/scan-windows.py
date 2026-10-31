from pathlib import Path
ROOT=Path(__file__).resolve().parent
source=(ROOT/'scan-reference.py').read_text(encoding='utf-8')
exec(source.split("canvas=Image.new")[0])
z0,z1,y0,y1=-.002,.065,.17,.86
scale=10000
width,height=round((z1-z0)*scale),round((y1-y0)*scale)
pixels=np.full((height,width,3),255,dtype=np.uint8)
depth=np.full((height,width),-1.,dtype=float)
selected=np.flatnonzero((centres[:,0]>.255)&(centres[:,2]>z0-.01)&(centres[:,2]<z1+.01)&(centres[:,1]>y0-.01)&(centres[:,1]<y1+.01))
for number in selected:
    ids=indices[number]
    p=points[ids]
    xx=(p[:,2]-z0)*scale
    yy=(y1-p[:,1])*scale
    xmin,xmax=max(0,int(np.floor(xx.min()))),min(width-1,int(np.ceil(xx.max())))
    ymin,ymax=max(0,int(np.floor(yy.min()))),min(height-1,int(np.ceil(yy.max())))
    if xmin>xmax or ymin>ymax:
        continue
    den=(yy[1]-yy[2])*(xx[0]-xx[2])+(xx[2]-xx[1])*(yy[0]-yy[2])
    if abs(den)<1e-12:
        continue
    gx,gy=np.meshgrid(np.arange(xmin,xmax+1)+.5,np.arange(ymin,ymax+1)+.5)
    a=((yy[1]-yy[2])*(gx-xx[2])+(xx[2]-xx[1])*(gy-yy[2]))/den
    b=((yy[2]-yy[0])*(gx-xx[2])+(xx[0]-xx[2])*(gy-yy[2]))/den
    c=1-a-b
    dist=a*p[0,0]+b*p[1,0]+c*p[2,0]
    current=depth[ymin:ymax+1,xmin:xmax+1]
    mask=(a>=0)&(b>=0)&(c>=0)&(dist>current)
    if not mask.any():
        continue
    weights=np.stack((a[mask],b[mask],c[mask]),axis=1)
    tuv=np.mod(weights@uv[ids],1)
    tx=np.minimum((tuv[:,0]*tex.shape[1]).astype(int),tex.shape[1]-1)
    ty=np.minimum((tuv[:,1]*tex.shape[0]).astype(int),tex.shape[0]-1)
    pixels[ymin:ymax+1,xmin:xmax+1][mask]=tex[ty,tx]
    current[mask]=dist[mask]
Image.fromarray(pixels).save(OUT/'gluon-window-strip.png')
(OUT/'gluon-window-strip.json').write_text(json.dumps({'z0':z0,'z1':z1,'y0':y0,'y1':y1,'pixels_per_raw':scale,'camera':'+X','selected_triangles':len(selected)}))
print('[DEBUG] Texture-registered window strip saved',flush=True)
