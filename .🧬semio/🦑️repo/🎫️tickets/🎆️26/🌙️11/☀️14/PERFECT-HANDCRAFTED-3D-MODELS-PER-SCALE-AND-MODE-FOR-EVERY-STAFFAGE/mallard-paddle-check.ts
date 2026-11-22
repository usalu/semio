/** 🦆 Measure web planform, ground support and submerged contact on posed triangles at each scale. */
import {readFileSync} from 'node:fs'
import {Ray,Triangle,Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
import {solveThermal} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./mallard-paddle-case.json',import.meta.url),'utf8')),bad=new Set()
const V=p=>new Vector3(p.x,p.y,p.z)
function intersectsTrunk(cuff,body){
 if(!cuff||!body)return false
 const vector=(m,i)=>new Vector3().fromArray(m.positions,3*i),step=Math.max(1,Math.floor(cuff.positions.length/60))
 for(let i=0;i<cuff.positions.length/3;i+=step){
  const ray=new Ray(vector(cuff,i),new Vector3(.507,.817,.279).normalize()),hits=[]
  for(let n=0;n<body.indices.length;n+=3){
   const [a,b,c]=Array.from(body.indices.slice(n,n+3),i=>vector(body,i)),hit=ray.intersectTriangle(a,b,c,false,new Vector3())
   if(hit)hits.push(hit.distanceTo(ray.origin))
  }
  if(hits.sort((a,b)=>a-b).filter((v,j)=>j===0||v-hits[j-1]>1e-6).length%2)return true
 }
 return false
}
for(const [pose,condition] of Object.entries(f.poses)){
 const p=a.poses.find(q=>q.id===pose);if(!p){bad.add('missing '+pose);continue}
 const state=evaluateDrawing(a,{kind:'pose',pose}),bones=state.bones,thermal=solveThermal(state)
 const field=thermal.regions['web-feet'],reg=p.thermal.regions['web-feet'],rows=[]
 if(!field||!Number.isFinite(field.surfaceTempC)||(reg.contact?.fraction??0)!==condition.contactFraction||(reg.wettedness??0)!==condition.wettedness)bad.add(pose+' thermal state')
 if(field&&(condition.contactFraction>0?Math.abs(field.contactWM2)<.0001:Math.abs(field.contactWM2)>.0001))bad.add(pose+' contact flux')
 for(const k of f.scales){
  const vol=a.volumes[k],ids=new Set(vol.body.filter(s=>s.bone.startsWith('foot-')).map(s=>s.id))
  const sample={...a,volumes:{...a.volumes,[k]:{...vol,body:vol.body.filter(s=>ids.has(s.id)||s.id==='body'||s.id.endsWith('-drumstick'))}}}
  const meshes=worldSolids(sample,bones,k),metrics=[]
  if(pose==='resting-ashore'){
   const body=meshes.find(s=>s.id==='body')||worldSolids(a,bones,k).find(s=>['body','hull','hull-thighs'].includes(s.id))
   if(!body)bad.add(k+' missing breast')
   if(body){let lowest=Infinity;for(let i=1;i<body.positions.length;i+=3)lowest=Math.min(lowest,body.positions[i]);if(Math.abs(lowest)>.009)bad.add(k+' resting breast '+lowest.toFixed(4))}
   if(!p.contacts.includes('resting-breast'))bad.add('missing breast contact')
  }
  for(const [side,label] of [['n','near'],['f','far']]){
   const bone='foot-'+side,feet=meshes.filter(m=>m.bone===bone),fr=bones[bone].frame
   if(!intersectsTrunk(meshes.find(s=>s.id===label+'-drumstick'),meshes.find(s=>s.id==='body')))bad.add(k+' '+pose+' '+label+' leg root outside trunk')
   if(feet.length<6||vol.structure.filter(q=>q.member===bone&&q.part==='member').length<4||feet.some(m=>!vol.thermal.some(q=>q.of===m.id&&q.region==='web-feet')))bad.add(k+' '+label+' physical bindings')
   let area=0,minW=Infinity,maxW=-Infinity,maxY=-Infinity,minY=Infinity,gap=Infinity
   const contact=V(fr.o).addScaledVector(V(fr.u),.030).addScaledVector(V(fr.v),-.006)
   for(const m of feet){
    const vertices=[];for(let i=0;i<m.positions.length;i+=3){const v=new Vector3().fromArray(m.positions,i);vertices.push(v);const z=v.clone().sub(V(fr.o)).dot(V(fr.w));minW=Math.min(minW,z);maxW=Math.max(maxW,z);maxY=Math.max(maxY,v.y);minY=Math.min(minY,v.y)}
    for(let n=0;n<m.indices.length;n+=3){const [i,j,l]=m.indices.slice(n,n+3),t=new Triangle(vertices[i],vertices[j],vertices[l])
     if(m.id.includes('-web-'))area+=t.getArea()*Math.abs(t.getNormal(new Vector3()).dot(V(fr.v)))/2
     gap=Math.min(gap,t.closestPointToPoint(contact,new Vector3()).distanceTo(contact))
    }
   }
   if(area<f.minimumFootprintAreaM2||maxW-minW<f.minimumPaddleWidthM)bad.add(k+' '+label+' web area or width')
   if(condition.support==='ground'&&(gap>f.maximumSupportGapM||Math.abs(minY)>.010||Math.abs(contact.y)>.006))bad.add(k+' '+pose+' support '+minY.toFixed(4)+'/'+contact.y.toFixed(4)+'/'+gap.toFixed(4))
   if(condition.support==='water'&&maxY>f.maximumSubmergedWebHeightM)bad.add(k+' '+pose+' above water '+maxY.toFixed(4))
   metrics.push([area,maxW-minW,minY,maxY,gap].map(v=>+v.toFixed(6)))
  }
  rows.push([k,metrics[0]])
 }
 console.log(JSON.stringify({pose,contactWM2:field?.contactWM2,surfaceC:field?.surfaceTempC,rows}))
}
if(bad.size){console.error([...bad]);process.exitCode=1}
