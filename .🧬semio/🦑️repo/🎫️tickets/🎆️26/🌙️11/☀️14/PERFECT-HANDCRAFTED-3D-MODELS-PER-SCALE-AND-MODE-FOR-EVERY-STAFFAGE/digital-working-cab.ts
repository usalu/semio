/** 🚚 Fold the mixer cab around its three glazed apertures and match the excavator's digging shell to its cutting assembly. */
import {readFileSync,writeFileSync} from 'node:fs'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,base=structuredClone(a.volumes['50']),f=JSON.parse(readFileSync(new URL('./digital-working-cab-cases.json',import.meta.url),'utf8'))[a.id]
const snap=v=>+v.toFixed(7)
function bucket(v){
 const selected=q=>q.bone==='bucket',old=new Set(v.body.filter(selected).map(q=>q.id)),chosen=base.body.filter(selected)
 v.body=v.body.filter(q=>!selected(q));v.thermal=v.thermal.filter(q=>!old.has(q.of)&&q.bone!=='bucket')
 v.structure=v.structure.filter(q=>!(q.member==='bucket'&&q.part==='member'))
 for(const q of chosen){
  v.body.push(structuredClone(q))
  const heat=base.thermal.find(k=>k.of===q.id);if(heat)v.thermal.push(structuredClone(heat))
  const support=base.structure.find(k=>k.of===q.id);if(support&&!v.structure.some(k=>k.id===support.id))v.structure.push(structuredClone(support))
  if(q.id==='bucket-lip'||q.id==='bucket-tooth')v.structure.push({id:'member-'+q.id,of:q.id,member:'bucket',part:'member'})
 }
}
function cabSkin(scale){
 const stations=base.body.find(q=>q.id==='cab-shell').sections,join=Math.pow(.5,1/12)
 const ys=[1.22,1.28,1.84,1.95,2.02,2.05,2.08,2.84,2.87,2.90,2.98,3.05,3.10]
 const edges=[-1,-.90,-.86,-.60,-.02,.02,.60,.86,.90,1]
 const sections=y=>{
  let i=stations.findIndex(q=>q.at[1]>=y);if(i<0)i=stations.length-1
  const q=stations[i],p=stations[Math.max(0,i-1)],t=(y-p.at[1])/(q.at[1]-p.at[1]||1)
  return [p.at[0]+(q.at[0]-p.at[0])*t,p.rx+(q.rx-p.rx)*t,p.ry+(q.ry-p.ry)*t]
 }
 const point=([face,u,y],depth)=>{
  const [cx,rx,rz]=sections(y),lateral=u*join,forward=Math.pow(1-Math.pow(lateral,12),1/12)
  const basis=[[forward,lateral],[-lateral,forward],[-forward,-lateral],[lateral,-forward]][face]
  return [cx+(rx-depth)*basis[0],y+(y===1.22?depth:y===3.10?-depth:0),(rz-depth)*basis[1]].map(snap)
 }
 const groups=new Map(),add=(id,points)=>{if(!groups.has(id))groups.set(id,[]);groups.get(id).push(points)}
 for(let face=0;face<4;face++)for(let yi=0;yi<ys.length-1;yi++)for(let ui=0;ui<edges.length-1;ui++){
  const y0=ys[yi],y1=ys[yi+1],u0=edges[ui],u1=edges[ui+1],u=(u0+u1)/2,y=(y0+y1)/2
  let id='cab-shell'
  if(face===0&&Math.abs(u)<.90&&y>2.05&&y<2.87)id='windscreen'
  if(face===0&&Math.abs(u)<.60&&y>1.28&&y<1.84)continue
  if((face===1||face===3)&&y>2.08&&y<2.84){const orient=face===1?-u:u;if(orient>.02&&orient<.86)id=face===1?'door-glass-l':'door-glass-r'}
  add(id,[[face,u0,y0],[face,u0,y1],[face,u1,y1],[face,u1,y0]])
 }
 for(const y of [1.22,3.10]){
  const ring=[];for(let face=0;face<4;face++)for(const u of edges.slice(0,-1))ring.push([face,u,y])
  const center=[4,0,y],forward=y===1.22
  for(let i=0;i<ring.length;i++){const pair=[ring[i],ring[(i+1)%ring.length]];add('cab-shell',[center,...(forward?pair:pair.reverse())])}
 }
 const mesh=(id,faces)=>{
  const steel=id==='cab-shell',thick=steel?f.sheetM:f.glassM,vertices=[],triangles=[],index=new Map(),perimeter=new Map(),interior=new Map()
  const vertex=(p,depth)=>{
   const coord=p[0]===4?[sections(p[2])[0],snap(p[2]+(p[2]===1.22?depth:-depth)),0]:point(p,depth)
   const key=coord.join(',');if(!index.has(key)){index.set(key,vertices.length);vertices.push(coord)}return index.get(key)
  }
  for(const points of faces){
   const outer=points.map(p=>vertex(p,0)),inner=points.map(p=>vertex(p,thick))
   for(let i=1;i<outer.length-1;i++){triangles.push([outer[0],outer[i],outer[i+1]],[inner[0],inner[i+1],inner[i]])}
   for(let j=0;j<outer.length;j++){
    const a=outer[j],b=outer[(j+1)%outer.length],key=Math.min(a,b)+':'+Math.max(a,b);interior.set(a,inner[j])
    if(perimeter.has(key))perimeter.delete(key);else perimeter.set(key,[a,b])
   }
  }
  for(const [a,b] of perimeter.values())triangles.push([b,a,interior.get(a)],[b,interior.get(a),interior.get(b)])
  return {id,kind:'polyhedron',bone:'chassis',material:steel?'paint-steel':'glass',tone:steel?.50:.85,role:steel?'body':'glazing',vertices,triangles}
 }
 return [...groups].map(([id,faces])=>mesh(id,faces))
}
if(a.id==='excavator'){for(const v of Object.values(a.volumes))bucket(v)}
else{
 const trim='cab-trim'
 a.materials.push({id:'cab-vinyl',title:{en:'Grained cab upholstery',de:'Genarbter Kabinenbezugsstoff'},densityKgM3:1150,conductivityWmK:.17,solarAbsorptivity:.76,emissivity:.94,source:'authored-estimate'})
 a.thermal.regions.push({id:trim,title:{en:'Cab seats and driving controls',de:'Fahrersitze und Bedienelemente'},kind:'surface',model:'passive',material:'cab-vinyl',areaM2:3.84415204,areaBasis:'total',characteristicLengthM:.49,geometry:'horizontal-plate-up',skyViewFactor:.1,layers:[],heatCapacityJM2K:5600,hidden:{en:'The upholstered seats and controls sit inside the glazed cab.',de:'Die gepolsterten Sitze und Bedienelemente liegen im verglasten Fahrerhaus.'},source:'authored-estimate'})
 const shell=a.thermal.regions.find(q=>q.id==='cab'),glass=a.thermal.regions.find(q=>q.id==='cab-glazing')
 shell.areaM2=29.17382524;glass.areaM2=2.70033825
 for(const p of a.poses){
  p.thermal.regions[trim]={exposedFraction:0,sunlitFraction:0}
  p.thermal.regions.cab.exposedFraction=0.9098685
  p.thermal.regions['cab-glazing'].exposedFraction=.99296
 }
 const select=id=>/^(?:cab-shell|windscreen|door-glass-[lr]|front-grille|grille-bar-\d+)$/.test(id)
 for(const [key,v] of Object.entries(a.volumes)){
  const old=new Set(v.body.filter(q=>select(q.id)).map(q=>q.id))
  v.body=v.body.filter(q=>!old.has(q.id));v.thermal=v.thermal.filter(q=>!old.has(q.of)&&!select(q.id.replace(/^th-/,'')));v.structure=v.structure.filter(q=>!old.has(q.of))
  const put=(q,region,load=false)=>{
   v.body.push(q)
   const heat={id:'th-'+q.id,region}
   if(region!==trim)v.thermal.push({...heat,of:q.id})
   else if(q.kind==='box')v.thermal.push({...q,...heat,size:q.size.map(v=>snap(v-.012)),cornerRadius:snap(q.cornerRadius-.006)})
   else if(q.kind==='revolve')v.thermal.push({...q,...heat,profile:q.profile.map(([ax,r])=>[snap(ax*.65),snap(.195+(r-.195)*.65)])})
   else v.thermal.push({...q,...heat,r0:.0065,r1:.0065})
   if(load)v.structure.push({id:'member-'+q.id,of:q.id,member:'chassis',part:'member'})
  }
  for(const q of cabSkin(+key))put(q,q.id==='cab-shell'?'cab':'cab-glazing',q.id==='cab-shell')
  const metal={bone:'chassis',role:'detail',tone:.32,material:'paint-steel'}
  put({...metal,id:'front-grille',kind:'extrude',plane:'yz',from:4.47,to:4.49,profile:[[1.26,-.67],[1.86,-.67],[1.86,.67],[1.26,.67]],holes:[[[1.29,-.625],[1.29,.625],[1.83,.625],[1.83,-.625]]]},'cab')
  put({...metal,id:'intake-duct',kind:'extrude',plane:'yz',from:4.235,to:4.481,profile:[[1.269,-.665],[1.851,-.665],[1.851,.665],[1.269,.665]],holes:[[[1.273,-.661],[1.273,.661],[1.847,.661],[1.847,-.661]]]},'cab')
  put({...metal,id:'intake-bulkhead',kind:'extrude',plane:'xy',from:-.6655,to:.6655,profile:[[4.257,1.2685],[4.2335,1.2685],[4.2335,1.8515],[4.257,1.8515],[4.257,1.8465],[4.2385,1.8465],[4.2385,1.2735],[4.257,1.2735]]},'cab')
  put({...metal,id:'radiator-fold',kind:'extrude',plane:'zx',from:1.285,to:1.835,texture:true,tone:.19,profile:[[4.244,-.638],[4.271,-.636],[4.282,-.628],[4.278,-.625],[4.268,-.632],[4.244,-.634]].map(([x,z])=>[z,x]),repeat:{count:42,translation:[0,0,.0308]}},'cab')
  for(let i=0;i<5;i++){
   const y=1.36+.10*i
   put({...metal,id:'grille-bar-'+(i+1),kind:'extrude',plane:'xy',from:-.63,to:.63,profile:[[4.456,y-.012],[4.464,y-.018],[4.497,y+.009],[4.497,y+.015],[4.486,y+.014]]},'cab')
  }
  for(const [name,z] of [['driver',.49],['passenger',-.49]]){
   const chair={bone:'chassis',material:'cab-vinyl',role:'detail',texture:true,tone:.31}
   put({...chair,id:name+'-seat',kind:'box',size:[.48,.12,f.seatWidthM],cornerRadius:.045,transform:{translation:[3.42,1.71,z]}},trim)
   put({...chair,id:name+'-backrest',kind:'box',size:[.125,.55,.47],cornerRadius:.051,transform:{translation:[3.16,1.99,z],rotation:[0,0,.0713,.997455]}},trim)
   put({...metal,id:name+'-pedestal',kind:'box',size:[.28,.42,.31],cornerRadius:.013,texture:true,transform:{translation:[3.40,1.44,z]}},'cab',true)
  }
  const control={bone:'chassis',material:'cab-vinyl',role:'detail',texture:true,tone:.24}
  put({...control,id:'instrument-console',kind:'box',size:[.28,.17,1.86],cornerRadius:.052,transform:{translation:[4.21,1.98,0]}},trim)
  const c=[3.91,2.025,.49],axis=[-.70710678,.70710678,0],radius=.195,section=Array.from({length:9},(_,i)=>[snap(.016*Math.sin(i*Math.PI/4)),snap(radius+.016*Math.cos(i*Math.PI/4))])
  put({...control,id:'steering-rim',kind:'revolve',axis,origin:c,profile:section},trim)
  for(let i=0;i<3;i++){
   const t=(90+120*i)*Math.PI/180
   put({...control,id:'steering-spoke-'+i,kind:'capsule',p0:c,p1:[c[0]+radius*.70710678*Math.cos(t),c[1]+radius*.70710678*Math.cos(t),c[2]+radius*Math.sin(t)],r0:.010,r1:.010},trim)
  }
  put({...metal,id:'steering-column',kind:'capsule',p0:[4.16,1.77,.49],p1:c,r0:.024,r1:.024,texture:true},'cab',true)
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc))
