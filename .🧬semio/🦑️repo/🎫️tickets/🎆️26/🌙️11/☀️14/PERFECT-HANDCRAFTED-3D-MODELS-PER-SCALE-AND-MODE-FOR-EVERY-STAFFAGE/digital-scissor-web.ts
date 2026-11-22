/** ✂️ Build the transverse pivot bores and sleeves through the individual six-millimetre walls of each lifting arm. */
import {readFileSync,writeFileSync} from 'node:fs'
import {ShapeUtils,Vector2} from 'three'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,fine=structuredClone(a.volumes['50']),round=x=>+x.toFixed(7)
const exchange=JSON.parse(readFileSync(new URL('./digital-scissor-heat.json',import.meta.url),'utf8'))
Object.assign(a.thermal.regions.find(r=>r.id==='arms'),{areaBasis:'total',areaM2:exchange.areaM2})
for(const p of a.poses)p.thermal.regions.arms.exposedFraction=round(exchange.exposedM2[p.id]/exchange.areaM2)
function arm(q,n){
 const z=q.transform.translation[2],vertices=[],triangles=[],keyed=new Map()
 function index(x,y,h){const xyz=[round(x),round(y),round(z+h)],key=xyz.join('/');if(!keyed.has(key)){keyed.set(key,vertices.length);vertices.push(xyz)}return keyed.get(key)}
 function bridge(a,b,flip=false){for(let i=0;i<a.length;i++){const j=(i+1)%a.length;const pair=[[a[i],a[j],b[j]],[a[i],b[j],b[i]]];triangles.push(...(flip?pair.map(q=>q.reverse()):pair))}}
 const level=(dir,inside)=>{
  const h=dir*(inside?.019:.025),rh=inside?.039:.045,raw=[[-.045,-rh],[1.505,-rh],[1.505,rh],[-.045,rh]]
  const outline=raw.map(([x,y])=>index(x,y,h)),holes=[0,.73,1.46].map(x=>Array.from({length:n},(_,j)=>[x+.0235*Math.cos(j*2*Math.PI/n),-.0235*Math.sin(j*2*Math.PI/n)]))
  const holeIds=holes.map(r=>r.map(([x,y])=>index(x,y,h))),ids=[outline,...holeIds].flat()
  for(const face of ShapeUtils.triangulateShape(raw.map(p=>new Vector2(...p)),holes.map(r=>r.map(p=>new Vector2(...p))))){
   const t=face.map(i=>ids[i]);triangles.push(dir*(inside?-1:1)>0?t:t.reverse())
  }
  return {outline,holeIds,rh,h}
 }
 const lo=level(-1,false),li=level(-1,true),hi=level(1,true),ho=level(1,false)
 for(const [edge,inner] of [[0,false],[2,false],[0,true],[2,true]]){
  const first=inner?hi:lo,last=inner?li:ho,i=edge,j=(i+1)%4
  triangles.push([first.outline[i],first.outline[j],last.outline[j]],[first.outline[i],last.outline[j],last.outline[i]])
 }
 for(const edge of [1,3]){
  const j=(edge+1)%4
  const outer=[lo.outline[edge],lo.outline[j],ho.outline[j],ho.outline[edge]]
  const inner=[li.outline[edge],li.outline[j],hi.outline[j],hi.outline[edge]]
  bridge(outer,inner)
 }
 for(let i=0;i<3;i++){bridge(lo.holeIds[i],li.holeIds[i]);bridge(hi.holeIds[i],ho.holeIds[i])}
 return {id:q.id,kind:'polyhedron',bone:q.bone,material:q.material,role:q.role,tone:q.tone,repeat:q.repeat,vertices,triangles}
}
const parts=q=>/^[ab][1-4]$/.test(q.bone??'')
for(const [s,v] of Object.entries(a.volumes)){
 const old=new Set(v.body.filter(parts).map(q=>q.id)),base=fine.body.filter(parts)
 v.body=v.body.filter(q=>!parts(q))
 v.thermal=v.thermal.filter(q=>!old.has(q.of)&&!parts(q))
 v.structure=v.structure.filter(q=>!parts(q)&&!(q.of&&old.has(q.of)))
 for(const q of base){
  const srf=q.id.startsWith('arm-')?arm(q,16):structuredClone(q)
  if(srf.id.startsWith('tube-')){
   const L=srf.profile.at(-1)[0],R=srf.profile[0][1]
   srf.profile=[[0,R-.004],[L,R-.004],[L,R],[0,R],[0,R-.004]]
  }
  v.body.push(srf);v.thermal.push({id:'th-'+q.id,of:q.id,region:'arms'})
  v.structure.push({id:'st-'+q.id,of:q.id,member:q.bone,part:q.id.startsWith('pin-')?'joint':'member'})
  if(!q.id.startsWith('arm-'))continue
  for(const [i,x] of [0,.73,1.46].entries()){
   const z=q.transform.translation[2],sleeve={id:'sleeve-'+q.bone+'-'+i,kind:'revolve',bone:q.bone,material:q.material,role:'detail',tone:.38,axis:[0,0,1],origin:[x,0,round(z-.025)],profile:[[0,.02025],[.05,.02025],[.05,.0235],[0,.0235],[0,.02025]],repeat:q.repeat}
   v.body.push(sleeve);v.thermal.push({id:'th-'+sleeve.id,of:sleeve.id,region:'arms'});v.structure.push({id:'st-'+sleeve.id,of:sleeve.id,member:q.bone,part:'joint'})
   if(!q.bone.startsWith('a'))continue
   const pin={...sleeve,id:'pin-shaft-'+q.bone+'-'+i,origin:[x,0,.213],profile:[[0,.02],[.107,.02]],repeat:{count:2,translation:[0,0,-.533]}}
   v.body.push(pin);v.thermal.push({id:'th-'+pin.id,of:pin.id,region:'arms'});v.structure.push({id:'st-'+pin.id,of:pin.id,member:q.bone,part:'joint'})
  }
 }
 v.decisions.push({en:'Three bored cross sections pierce the opposing 6 mm walls of each 90 by 50 mm arm. A welded sleeve clears the 40 mm transverse pin. Inner and outer links retain their separation when folded.',de:'Drei Querbohrungen durchdringen die gegenüberliegenden 6-mm-Wände jedes 90 × 50 mm großen Scherenarms. Eine eingeschweißte Hülse umschließt den 40-mm-Querbolzen. Innen- und Außenlenker bleiben auch eingefahren getrennt.'})
}
writeFileSync(process.argv[3],JSON.stringify(doc))
