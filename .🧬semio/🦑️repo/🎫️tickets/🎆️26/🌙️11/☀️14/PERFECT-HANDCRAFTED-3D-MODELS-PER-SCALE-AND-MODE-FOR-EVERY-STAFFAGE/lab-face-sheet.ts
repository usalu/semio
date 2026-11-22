/** 🐶 Inspect the actual presentation normals and materials close to the eyes, stop and muzzle. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
import {posedMesh} from './living-world-mesh.ts'
import {eachSolid,sceneInput} from './living-volume-input.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {solidLook} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🎨️looks/🟦️.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {Canvas} from './volumes-raster.ts'
import {drawAxon,PART_COLOR} from './volumes-render.ts'
const raw=readFileSync(process.argv[2],'utf8'),a=sceneInput(raw),pose=process.argv[4]||'sitting-waiting',asMesh=posedMesh(a,pose,true)
const fr=evaluateDrawing(JSON.parse(raw).asset,{kind:'pose',pose}).bones.head.frame,V=p=>new Vector3(p.x,p.y,p.z),u=V(fr.u),v=V(fr.v),w=V(fr.w)
const size=220,sheet=new Canvas(4*size,4*(size+15),[254,254,250])
for(const [row,k] of [50,100,200,500].entries()){
 const visit=eachSolid(raw,String(k)),q=qualityOf(a,k)
 for(const [column,[yawDeg,mode]] of [[1,'body'],[34,'body'],[89,'body'],[47,'structure']].entries()){
  const yaw=yawDeg*Math.PI/180,tilt=.11,d=u.clone().multiplyScalar(Math.cos(yaw)).addScaledVector(w,Math.sin(yaw))
  const side=u.clone().multiplyScalar(-Math.sin(yaw)).addScaledVector(w,Math.cos(yaw)),up=v.clone().multiplyScalar(Math.cos(tilt)).addScaledVector(d,-Math.sin(tilt)),depth=d.multiplyScalar(Math.cos(tilt)).addScaledVector(v,Math.sin(tilt))
  const center=V(fr.o).addScaledVector(u,.113),cam={r:side.toArray(),u:up.toArray(),e:depth.toArray()}
  const panel=new Canvas(size,size,[239,240,235]),ax={cam,scale:684,cx:side.dot(center),cy:up.dot(center),cz:0,ox:size/2,oy:size/2,zc:0}
  for(const s of visit(mode)){
   if(!['head','jaw','ear-n','ear-f','neck'].includes(s.bone))continue
   let rgb
   if(mode==='body'){const look=solidLook('light','light-solid',[s]),paint=look.paint().get(look.keyOf(s));rgb=[1,3,5].map(i=>parseInt(paint.hex.slice(i,i+2),16))}
   else rgb=PART_COLOR[s.part]??[110,110,110]
   drawAxon(panel,ax,[{mesh:asMesh(s,q),color:rgb}],{clearDepth:false})
  }
  const top=row*(size+15);sheet.blit(panel,column*size,top);sheet.text(column*size+4,top+size+2,'1/'+k+' '+mode+' '+yawDeg,[40,40,40],1)
 }
}
writeFileSync(process.argv[3],sheet.png())
