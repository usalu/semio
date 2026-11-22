/** 🛞 Form the alloy, pressed-steel and period wheels on their actual hubs and maintain the body frame. */
import {readFileSync,writeFileSync} from 'node:fs'
import {ShapeUtils,Vector2} from 'three'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,base=structuredClone(a.volumes['50']),city=a.id==='city-car',van=a.id==='delivery-van',vintage=a.id==='vintage-car-1929'
if(!city&&!van&&!vintage)throw Error(a.id)
const snap=x=>+x.toFixed(6),wheel=q=>q.bone==='wheel-f'||q.bone==='wheel-r',wheelMember=q=>q.member==='wheel-f'||q.member==='wheel-r'
const corners=['fl','fr','rl','rr'],loc=(c,name)=>vintage?name+'-'+c:c+'-'+name
const ring=(radius,n)=>Array.from({length:n},(_,i)=>[radius*Math.cos(2*Math.PI*i/n),radius*Math.sin(2*Math.PI*i/n)])
function castSpoke(q,c){
 const sign=c[1]==='l'?1:-1,hub=base.body.find(s=>s.id===loc(c,'hub')),z=hub.origin[2]
 const stations=city?[[.037,-.009,.023,.011,.049],[.061,-.008,.024,.012,.058],[.093,-.003,.019,.010,.063],[.134,.007,.015,.008,.065],[.169,.015,.018,.008,.063],[.188,.016,.022,.009,.061]]:
 [[.054,0,.014,.009,.017],[.071,.001,.014,.009,.018],[.105,.002,.011,.008,.015],[.158,.002,.009,.006,.009],[.205,.001,.008,.005,.003],[.229,0,.011,.006,0]]
 const sections=stations.map(([r,t,w,d,h])=>({at:city?[snap(t),snap(r),snap(z+sign*h)]:[snap(r),snap(t),snap(z+sign*h)],rx:w,ry:d,exponent:city?3.3:2.7,rotation:city?[-.7071068,0,0,.7071068]:[.5,.5,.5,.5]}))
 return {id:q.id,kind:'loft',bone:q.bone,material:q.material,role:q.role,tone:q.tone,sections,capStart:true,capEnd:true,repeat:q.repeat}
}
function dishedWheel(q,c){
 const side=c[1]==='l'?1:-1,hub=base.body.find(s=>s.id===loc(c,'hub')),z=hub.origin[2],verts=[],triangles=[],N=48
 const direction=side
 function loop(xy,h){return xy.map(([x,y])=>{verts.push([snap(x),snap(y),snap(z+side*h)]);return verts.length-1})}
 function face(x,y,z){triangles.push(direction>0?[x,y,z]:[x,z,y])}
 function bridge(outer,inner,reverse=false){for(let j=0;j<outer.length;j++){const n=(j+1)%outer.length;if(reverse){face(outer[j],inner[n],outer[n]);face(outer[j],inner[j],inner[n])}else{face(outer[j],outer[n],inner[n]);face(outer[j],inner[n],inner[j])}}}
 function band(out,holes,rev){const loops=[out,...holes],idx=loops.flat(),xy=ls=>ls.map(i=>new Vector2(verts[i][0],verts[i][1]));for(const [i,j,k] of ShapeUtils.triangulateShape(xy(out),holes.map(xy)))rev?face(idx[i],idx[k],idx[j]):face(idx[i],idx[j],idx[k])}
 const windows=Array.from({length:8},(_,i)=>Array.from({length:16},(_,j)=>{const b=2*Math.PI*i/8,t=-2*Math.PI*j/16,radial=.124+.022*Math.cos(t),tangent=.035*Math.sin(t),p=b+tangent/.124;return [radial*Math.cos(p),radial*Math.sin(p)]}))
 const data=[[.044,.062],[.087,.062],[.098,.072],[.152,.048],[.171,.018],[.179,.016]]
 const heights=r=>.072-(r-.098)*(.024/.054)
 const sets=[0,-.008].map((dz,i)=>{
  const rings=data.map(([r,h])=>loop(ring(r,N),h+dz))
  const holes=windows.map(points=>points.map(([x,y])=>loop([[x,y]],heights(Math.hypot(x,y))+dz)[0]))
  for(let j=1;j<rings.length;j++){
   if(j===3){band(rings[j],[rings[j-1],...holes],i===1);continue}
   bridge(rings[j],rings[j-1],i===1)
  }
  return {rings,holes}
 })
 const [front,back]=sets
 bridge(front.rings[0],back.rings[0]);bridge(back.rings.at(-1),front.rings.at(-1))
 for(let i=0;i<8;i++)bridge(back.holes[i],front.holes[i])
 return {id:q.id,kind:'polyhedron',bone:q.bone,material:q.material,role:'detail',tone:q.tone,vertices:verts,triangles}
}
const material=city?'alloy':'steel-rim',measured={"city-car":{"brakes":[0.8872,0.8597],"wheel-metal":[2.1191,0.6452]},"delivery-van":{"brakes":[0.8698,0.8929],"wheel-metal":[2.9441,0.6929]},"vintage-car-1929":{"brakes":[0.8042,0.6251],"wheel-metal":[1.8439,0.7466]}}[a.id]
a.thermal.regions.push({id:'wheel-metal',title:{en:'Exposed rims, centres and hubs',de:'Freiliegende Felgen, Radscheiben und Naben'},kind:'surface',model:'passive',material,areaM2:measured['wheel-metal'][0],characteristicLengthM:city?.38:van?.42:.48,geometry:'vertical-plate',skyViewFactor:.36,layers:[],heatCapacityJM2K:city?9400:van?12600:11200,source:'authored-estimate'})
a.thermal.regions.find(q=>q.id==='brakes').areaM2=measured.brakes[0]
for(const p of a.poses){
 p.thermal.regions['wheel-metal']={exposedFraction:measured['wheel-metal'][1],sunlitFraction:.32}
 p.thermal.regions.brakes.exposedFraction=measured.brakes[1]
}
for(const [k,rep] of Object.entries(a.representations))if(rep.status==='authored'){
 for(const [view,shapes] of Object.entries(rep.geometry)){
  for(const shape of shapes.filter(wheel)){
   if(shape.material!==material&&!(vintage&&shape.material==='nickel-plate'))continue
   if(shape.region==='brakes'||shape.regions?.includes('brakes')){delete shape.region;shape.regions=['brakes','wheel-metal']}
   else {delete shape.regions;shape.region='wheel-metal'}
  }
  if(k==='500'&&view==='elevation')for(const axle of ['f','r']){
   const tyre=shapes.find(q=>q.bone==='wheel-'+axle&&q.kind==='ellipse'&&q.material==='rubber')
   const radius=city?.195:van?.213:.242
   const centre={...structuredClone(tyre),id:'wheel-centre-'+axle,material,region:'wheel-metal',tone:.58,z:tyre.z+.1,rx:radius,ry:radius}
   delete centre.regions
   shapes.splice(shapes.indexOf(tyre)+1,0,centre)
  }
 }
}
const siteText=city?{en:'Five gently swept cast spokes join each alloy well to the hub. The brake rotor sits inboard of the spokes. A continuous steel roof rail ties the four pillars on each side to the headers.',de:'Fünf leicht geschwungene Gussspeichen verbinden jede Leichtmetallfelge mit ihrer Nabe. Die Bremsscheibe sitzt innenseitig. Durchgehende Dachholme verbinden die vier Säulen jeder Seite mit den Dachquerträgern.'}:van?{en:'Each pressed-steel wheel has an inset mounting face and eight open cooling windows in its dished web. The paired cant rails and pillars enclose the sliding door aperture.',de:'Jedes gepresste Stahlrad hat eine eingezogene Anlagefläche und acht offene Kühlschlitze in seiner gewölbten Radscheibe. Dachholme und Säulen umschließen die Schiebetüröffnung.'}:{en:'Twelve tapered spokes rise from each period wheel hub to its narrow steel rim. The drum stays inboard of the spokes. Sills, upper rails, roof bows and four pairs of pillars carry the body.',de:'Zwölf verjüngte Speichen führen von jeder Nabe zur schmalen Stahlfelge. Die Bremstrommel sitzt innenseitig. Schweller, Dachholme, Spriegel und vier Säulenpaare tragen den Aufbau.'}
for(const [s,v] of Object.entries(a.volumes)){
 const scale=+s,old=new Set(v.body.filter(wheel).map(q=>q.id))
 v.thermal=v.thermal.filter(q=>!wheel(q)&&!old.has(q.of))
 const masses=(vintage&&scale===500?a.volumes['200']:v).structure.filter(q=>wheelMember(q)&&q.part==='mass')
 v.structure=v.structure.filter(q=>!wheelMember(q))
 const wheels=[]
 for(const c of corners){
  const originals=base.body.filter(q=>wheel(q)&&(vintage?q.id.endsWith('-'+c):q.id.startsWith(c+'-')))
  for(const previous of originals){
   if(scale>100&&previous.id===loc(c,'valve'))continue
   let q=structuredClone(previous)
   if(q.id===loc(c,city?'spoke':vintage?'spokes':'disc-wheel')){
    q=van?dishedWheel(q,c):castSpoke(q,c)
    if(city&&scale>=200)q.sections=q.sections.filter((_,i)=>i!==2&&(scale!==500||i!==4))
   }
   wheels.push(q)
   const rubber=q.material==='rubber',brake=q.material==='cast-iron'
   v.thermal.push({id:'th-'+q.id,of:q.id,region:rubber?'tyres':brake?'brakes':'wheel-metal'})
   if(!brake&&!q.id.includes('valve')&&!q.id.includes('hub-cap'))v.structure.push({id:'st-'+q.id,of:q.id,member:q.bone,part:q.id.includes('nut')?'joint':'member'})
  }
 }
 v.body=v.body.filter(q=>!wheel(q)).concat(wheels)
 v.structure.push(...structuredClone(base.structure.filter(q=>wheelMember(q)&&(q.part==='joint'||q.part==='mass'&&(!vintage||!masses.length)))))
 if(vintage&&masses.length)v.structure.push(...masses)
 const frame=city?/^(?:[abcd]-pillar-|hinge-pillar-|roof-rail-|windscreen-header$|rear-header$|roof-bow-|cowl-crossmember$|sill-)/:van?/^(?:[abcd]-pillar-|hinge-pillar-|cant-rail-|windscreen-header$|rear-header$|roof-bow-|cowl-crossmember$|sill-)/:/^(?:pillar-[abcd]-|cant-rail-|body-sill-|roof-bow-|floor-bearer-)/
 if(scale!==50){
  const own=q=>q.member==='chassis'&&(frame.test(q.id)||/^(screen-pillar-|roof-bow$)/.test(q.id))
  v.structure=v.structure.filter(q=>!own(q)).concat(structuredClone(base.structure.filter(own)))
 }
 for(const q of v.structure)if(q.material==='cast-iron'&&q.member==='chassis'&&q.part==='member'&&!vintage)q.material='paint'
 if(scale===500)v.decisions=[siteText];else v.decisions.push(siteText)
}
writeFileSync(process.argv[3],JSON.stringify(doc))
