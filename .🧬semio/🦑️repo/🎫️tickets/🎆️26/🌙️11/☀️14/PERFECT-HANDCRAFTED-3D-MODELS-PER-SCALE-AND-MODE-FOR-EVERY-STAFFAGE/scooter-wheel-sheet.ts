/** 🛴 Render the wheel train at equal scale from the side and from above each axle. */
import {readFileSync} from 'node:fs'
import {Canvas} from './volumes-raster.ts'
import {Panel,cameraAt,listsIn,savePng} from './r3-lib.ts'
import {bodyItems,thermalItems,structureItems} from './volumes-render.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,w=296,h=135
for(const k of [50,100,200,500]){
 const v=a.volumes[k],poses={frame:{o:[0,0,0],u:[1,0,0],v:[0,1,0],w:[0,0,1]}}
 for(const end of ['rear','front'])poses['wheel-'+end]={...poses.frame,o:[end==='rear'?0:.56,.0725,0]}
 const sub={...a,volumes:{[k]:{...v,body:v.body.filter(s=>poses[s.bone]),thermal:v.thermal.filter(s=>v.body.some(p=>p.id===s.of&&poses[p.bone])),structure:v.structure.filter(s=>poses[s.bone]||v.body.some(p=>p.id===s.of&&poses[p.bone]))}}}
 const meshes=listsIn(sub,k,poses),sheet=new Canvas(3*w,2*(h+16),[255,255,255])
 for(const [i,items] of [bodyItems(a,meshes,k),thermalItems(a,meshes,k),structureItems(a,meshes,k)].entries()){
  for(const [j,[yaw,tilt]] of [[12,13],[2,70]].entries()){
   const panel=new Panel(w,h,[242,243,241],2),cam=cameraAt(yaw,tilt)
   const center=[.28,.104,0],dot=(a,b)=>a.reduce((s,n,q)=>s+n*b[q],0)
   const place={cam,scale:380,cx:dot(cam.r,center),cy:dot(cam.u,center),ox:w/2,oy:h/2}
   panel.draw(place,items);panel.shade(place);sheet.blit(panel.canvas(),i*w,j*(h+16))
   sheet.text(i*w+3,j*(h+16)+h+2,['BODY','THERMAL REGIONS','STRUCTURE'][i]+' 1/'+k+' '+(j?'HIGH':'SIDE'),[25,25,25],1)
  }
 }
 savePng(process.argv[3]+'-'+k+'.png',sheet)
}
