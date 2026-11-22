/** 🖌️ Interpolate authored vertex lighting over a covered orthographic triangle with the shared depth convention. */
import type {Canvas,RGB} from './volumes-raster.ts'
export function shadedTriangle(canvas:Canvas,corners:number[][],rgb:RGB,light:number[],epsilon:number){
 const [a,b,c]=corners,det=(b[1]-c[1])*(a[0]-c[0])+(c[0]-b[0])*(a[1]-c[1])
 if(Math.abs(det)<1e-12)return
 const low=corners.reduce((p,v)=>[Math.min(p[0],v[0]),Math.min(p[1],v[1])],[canvas.width,canvas.height])
 const high=corners.reduce((p,v)=>[Math.max(p[0],v[0]),Math.max(p[1],v[1])],[0,0])
 for(let y=Math.max(0,Math.floor(low[1]));y<=Math.min(canvas.height-1,Math.ceil(high[1]));y++){
  for(let x=Math.max(0,Math.floor(low[0]));x<=Math.min(canvas.width-1,Math.ceil(high[0]));x++){
   const dx=x+.5-c[0],dy=y+.5-c[1],u=((b[1]-c[1])*dx+(c[0]-b[0])*dy)/det,v=((c[1]-a[1])*dx+(a[0]-c[0])*dy)/det,w=1-u-v
   if(Math.min(u,v,w)<-1e-9)continue
   const i=y*canvas.width+x,z=u*a[2]+v*b[2]+w*c[2]
   if(z<=canvas.depth[i]+epsilon)continue
   canvas.depth[i]=z
   const f=u*light[0]+v*light[1]+w*light[2]
   for(let channel=0;channel<3;channel++)canvas.data[3*i+channel]=rgb[channel]*f
  }
 }
}
