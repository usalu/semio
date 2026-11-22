/** 🦆 Inspect the spread, free rear toe, membrane edge and supporting rays of the swimming mallard. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
import {posedMesh} from './living-world-mesh.ts'
import {eachSolid,sceneInput} from './living-volume-input.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {Canvas} from './volumes-raster.ts'
import {drawAxon} from './volumes-render.ts'
const raw=readFileSync(process.argv[2],'utf8'),a=sceneInput(raw),pose='swimming',mesh=posedMesh(a,pose,true)
const fr=evaluateDrawing(JSON.parse(raw).asset,{kind:'pose',pose}).bones['foot-n'].frame
const V=p=>new Vector3(p.x,p.y,p.z),u=V(fr.u),v=V(fr.v),w=V(fr.w),center=V(fr.o).addScaledVector(u,.023)
const width=230,height=200,sheet=new Canvas(width*4,(height+15)*4,[250,250,245])
for(const [row,k] of [50,100,200,500].entries()){
 const visit=eachSolid(raw,String(k)),quality=qualityOf(a,k)
 for(const [col,[name,dv,dw,mode]] of [['dorsal',1,.04,'body'],['plantar',-1,.08,'body'],['web edge',.29,-1,'body'],['digital rays',1,.04,'structure']].entries()){
  const eye=v.clone().multiplyScalar(dv).addScaledVector(w,dw).normalize(),right=u.clone(),up=eye.clone().cross(right).normalize()
  const panel=new Canvas(width,height,[237,238,231]),ax={cam:{r:right.toArray(),u:up.toArray(),e:eye.toArray()},scale:2700,cx:right.dot(center),cy:up.dot(center),cz:0,ox:width/2,oy:height/2,zc:0}
  for(const s of visit(mode)){
   if(s.bone!=='foot-n')continue
   const color=mode==='structure'?[108,132,159]:s.id.includes('-web-')?[191,155,112]:[154,114,75]
   drawAxon(panel,ax,[{mesh:mesh(s,quality),color}],{clearDepth:false})
  }
  const top=row*(height+15);sheet.blit(panel,col*width,top);sheet.text(col*width+4,top+height+2,'1/'+k+' '+name,[45,46,43],1)
 }
}
writeFileSync(process.argv[3],sheet.png())
