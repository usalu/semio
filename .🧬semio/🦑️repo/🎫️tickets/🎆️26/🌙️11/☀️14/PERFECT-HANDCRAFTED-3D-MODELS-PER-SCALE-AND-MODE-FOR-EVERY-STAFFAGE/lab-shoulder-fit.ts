/** 🐕 Fit the costal coat, deltoid, triceps and caudal cervical mantle. Reproject the bounded forequarter into each printed view. */
import {readFileSync,writeFileSync} from 'node:fs'
import {featureM,meshesAt,simplifyRing,area} from './volumes-lib.ts'
import {projectMask,contours,inverse} from './r3-project2d.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,round=x=>+x.toFixed(6)
if(a.id!=='labrador-dog')throw Error(a.id)
const weights=(x,anchors)=>{
 const i=anchors.findIndex(p=>p[0]>=x)
 if(i===0)return anchors[0][1]
 if(i<0)return anchors.at(-1)[1]
 const [u,l]=anchors[i-1],[v,h]=anchors[i],frac=(x-u)/(v-u)
 return l+(h-l)*frac
}
const neck=[[-.05,.04],[-.032,.16],[-.017,.23],[.035,.27],[.075,.24],[.12,.15],[.16,.05],[.2,0]]
const costal=[[.03,0],[.105,.07],[.155,.104],[.192,.12],[.233,.23],[.256,.27],[.278,.20],[.292,0]]
const scapula=[[-.01,0],[.012,.001],[.054,.006],[.091,.015],[.13,.026],[.159,.030],[.19,.025]]
const triceps=[[-.06,.014],[-.035,.021],[-.012,.027],[.032,.03],[.07,.024],[.1,.016],[.14,.007],[.178,0]]
const gains={50:1,100:.99,200:.95,500:1.02}
const shapeId=(part,k)=>{
 if(['neck','chest'].includes(part))return part
 const near=part.endsWith('-n'),prefix=k===500?(near?'n':'f'):(near?'near':'far')
 return prefix+'-'+(part.startsWith('shoulder')?(k===50?'scapula':'shoulder'):part.startsWith('upper-arm')?(k===50?'humerus':'upper-arm'):'fore-upper')
}
for(const [s,v] of Object.entries(a.volumes)){
 const k=+s,gain=gains[s]
 const touched=v.body.filter(q=>/^(chest|neck|shoulder-[nf]|upper-arm-[nf]|fore-upper-[nf])$/.test(q.id))
 for(const solid of touched)for(const station of solid.sections){
  const x=station.at[0],near=solid.id.endsWith('-n')?1:-1
  if(solid.id==='neck'){
   const f=weights(x,neck)*gain
   station.ry=round(station.ry*(1+f));station.rx=round(station.rx*(1+.20*f))
  }else if(solid.id==='chest'){
   const f=weights(x,costal)*gain
   station.rx=round(station.rx*(1+f));station.at[0]=round(x+weights(x,[[.13,0],[.19,.004],[.23,.016],[.263,.027],[.3,.031]])*gain);station.ry=round(station.ry*(1+.18*f))
  }else{
   const d=weights(x,solid.id.startsWith('shoulder')?scapula:triceps)*gain
   station.at[2]=round(station.at[2]+near*d);station.ry=round(station.ry*(1-3.2*d))
  }
 }
 for(const q of touched){const surface=v.thermal.find(t=>t.of===q.id);surface.layer='fur'}
 const byShape=new Map(meshesAt(a,k,touched).map((m,i)=>[shapeId(touched[i].id,k),m]))
 const feature=featureM(k,a)
 for(const [view,shapes] of Object.entries(a.representations[s].geometry)){
  const jobs=shapes.filter(q=>byShape.has(q.id)).map(meta=>{
   const mask=projectMask(view,[byShape.get(meta.id)],feature/16,3)
   const loops=contours(mask.cells,mask.width,mask.height,mask.box,mask.cell)
   let ring=loops.sort((l,r)=>Math.abs(area(r))-Math.abs(area(l)))[0]
   for(let i=0;i<6;i++)ring=ring.map((p,j)=>{const l=ring[(j+ring.length-1)%ring.length],r=ring[(j+1)%ring.length];return {x:(l.x+2*p.x+r.x)/4,y:(l.y+2*p.y+r.y)/4}})
   return {meta,target:simplifyRing(ring,feature*.18)}
  })
  const projected=inverse(a,k,view,jobs)
  jobs.forEach(({meta},i)=>{if(projected[i].residualM>1e-5)throw Error(meta.id+' '+view);meta.points=projected[i].points.map(p=>[round(p.x),round(p.y)])})
 }
}
const measured={'coat-trunk':.4981,'coat-forelimbs':.1503}
for(const region of a.thermal.regions)if(Object.hasOwn(measured,region.id))region.areaM2=measured[region.id]
a.thermal.assumptions.push({en:'Standing exchange areas for trunk and forelimb fur use the exposed 1:50 surface. At the shoulder, a buried coat surface exchanges no heat with the air.',de:'Die Austauschflächen von Rumpf- und Vorderlauffell entsprechen der freien Oberfläche im Stand bei 1:50. Überdecktes Fell an der Schulter tauscht keine Wärme mit der Luft aus.'})
writeFileSync(process.argv[3],JSON.stringify(doc))
