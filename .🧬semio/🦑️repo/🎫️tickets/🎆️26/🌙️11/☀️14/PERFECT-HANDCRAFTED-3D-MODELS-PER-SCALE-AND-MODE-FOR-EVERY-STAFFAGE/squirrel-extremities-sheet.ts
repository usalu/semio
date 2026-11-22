/** 🔍 Inspect the nose openings, orbit and digit fans using posed production normals. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
import {posedMesh} from './living-world-mesh.ts'
import {eachSolid,sceneInput} from './living-volume-input.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {solidLook} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🎨️looks/🟦️.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {Canvas} from './volumes-raster.ts'
import {drawAxon} from './volumes-render.ts'
const raw=readFileSync(process.argv[2],'utf8'),a=sceneInput(raw),pose='bound-extension',asMesh=posedMesh(a,pose,true),bones=evaluateDrawing(JSON.parse(raw).asset,{kind:'pose',pose}).bones
const V=p=>new Vector3(p.x,p.y,p.z),S=210,sheet=new Canvas(S*4,(S+15)*4,[250,250,245])
for(const [r,k] of [50,100,200,500].entries()){
 const visit=eachSolid(raw,String(k)),quality=qualityOf(a,k)
 for(const [c,[bone,name,aim,offset,zoom]] of [['head','nose',[1,.05,-.08],.052,6700],['head','profile',[.18,.06,-1],.021,2600],['hand-n','forepaw',[.07,1,.11],.018,4400],['foot-n','hindpaw',[.07,1,.11],.024,3100]].entries()){
  const fr=bones[bone].frame,u=V(fr.u),v=V(fr.v),w=V(fr.w),eye=u.clone().multiplyScalar(aim[0]).addScaledVector(v,aim[1]).addScaledVector(w,aim[2]).normalize()
  const right=(bone==='head'?v.clone().cross(eye):w.clone().cross(eye)).normalize(),up=eye.clone().cross(right).normalize(),center=V(fr.o).addScaledVector(u,offset)
  const panel=new Canvas(S,S,[235,236,230]),ax={cam:{r:right.toArray(),u:up.toArray(),e:eye.toArray()},scale:zoom,cx:right.dot(center),cy:up.dot(center),cz:0,ox:S/2,oy:S/2,zc:0}
  for(const s of visit('body')){if(s.bone!==bone)continue;const look=solidLook('light','light-solid',[s]),paint=look.paint().get(look.keyOf(s)),rgb=[1,3,5].map(i=>parseInt(paint.hex.slice(i,i+2),16));drawAxon(panel,ax,[{mesh:asMesh(s,quality),color:rgb}],{clearDepth:false})}
  const top=r*(S+15);sheet.blit(panel,c*S,top);sheet.text(c*S+3,top+S+2,'1/'+k+' '+name,[48,48,46],1)
 }
}
writeFileSync(process.argv[3],sheet.png())
