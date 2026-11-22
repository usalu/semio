/** 🚌 Fit the fixed saloon lights to the punched side panels and the load-bearing posts of each individual vehicle. */
import {readFileSync,writeFileSync} from 'node:fs'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,f=JSON.parse(readFileSync(new URL('./digital-transit-cases.json',import.meta.url),'utf8'))[a.id],bus=a.id==='city-bus',v50=structuredClone(a.volumes['50']),round=n=>+n.toFixed(6)
const select=id=>bus?/^(?:side-upper-[lr]$|pane-[lr]-|transom-[lr]-|mullion-[lr]-|glass-[lr]-)/.test(id):/^(?:(?:end|mid)-(?:wall|side)-upper-|pane-(?:[em]\d|cab)-|band-(?:end|mid)-|mullion-)/.test(id)
const frame=id=>bus?/^(?:post-[lr]-\d+(?:-lower)?$|waist-rail-|cant-rail-|door-header-)/.test(id):/^(?:end|mid)-(?:post-(?:lower-|upper-)?[lr]-|cant-rail-|door-post-|door-lintel)/.test(id)
const parts=v50.body.filter(q=>select(q.id)),ids=new Set(parts.map(q=>q.id))
function profile(w,h,r,n){
 const p=[]
 for(const [x,y,base] of [[w/2-r,h/2-r,0],[-w/2+r,h/2-r,90],[-w/2+r,-h/2+r,180],[w/2-r,-h/2+r,270]]){
  for(let i=0;i<=n;i++){const t=(base+90*i/n)*Math.PI/180;p.push([round(x+r*Math.cos(t)),round(y+r*Math.sin(t))])}
 }
 return p
}
for(const [key,v] of Object.entries(a.volumes)){
 const scale=Number(key),removed=new Set(v.body.filter(q=>select(q.id)).map(q=>q.id))
 const thermal=q=>removed.has(q.of)||select(q.id.replace(/^th-/,''))||parts.some(s=>q.id.startsWith('th-'+s.id+'-'))
 v.body=v.body.filter(q=>!removed.has(q.id));v.thermal=v.thermal.filter(q=>!thermal(q))
 v.structure=v.structure.filter(q=>!frame(q.id)&&!removed.has(q.of))
 v.structure.push(...v50.structure.filter(q=>frame(q.id)||ids.has(q.of)).map(q=>structuredClone(q)))
 for(const q of parts){
  v.body.push(structuredClone(q))
  const therm=v50.thermal.filter(s=>s.of===q.id||s.id.startsWith('th-'+q.id+'-'))
  if(key==='50'&&therm.length)v.thermal.push(...therm.map(s=>structuredClone(s)))
  else {
   const sectionFaces=therm.find(s=>s.sectionFaces)?.sectionFaces
   v.thermal.push({id:'th-'+q.id,of:q.id,region:q.material==='glass'?'glazing':'body-panels',...(sectionFaces&&{sectionFaces})})
  }
  if(q.kind!=='box'||q.material!=='glass')continue
  const [w,h,t]=q.size,dir=Math.sign(q.transform.translation[2]),l=f.lipWidthM,depth=f.lipDepthM
  const host=parts.find(wall=>wall.material==='paint'&&wall.kind==='extrude'&&wall.bone===q.bone&&wall.id.endsWith(dir>0?'-l':'-r'))
  if(!host)throw Error('No exterior panel for '+q.id)
  const angle=2*Math.atan2(host.transform.rotation[0],host.transform.rotation[3]),center=q.transform.translation,base=host.transform.translation
  const outer=-(center[1]-base[1])*Math.sin(angle)+(center[2]-base[2])*Math.cos(angle)
  const near=dir*(t/2-.0015),far=-outer+dir*depth,segments=scale===50?4:scale===100?3:2
  const seal={id:'seal-'+q.id,kind:'extrude',plane:'xy',bone:q.bone,material:'rubber',tone:.13,role:'detail',texture:true,from:Math.min(near,far),to:Math.max(near,far),profile:profile(w+.004,h+.004,f.cornerRadiusM,segments),holes:[profile(w+.004-2*l,h+.004-2*l,f.cornerRadiusM-l,segments).reverse()],transform:q.transform}
  v.body.push(seal);v.thermal.push({id:'th-'+seal.id,of:seal.id,region:'window-seals'})
 }
 v.decisions.push({en:'Each fixed pane sits within its own side-panel aperture. The exterior glazing lip and the saloon posts retain their widths as the drawing scale changes.',de:'Jede Festscheibe sitzt in einem eigenen Ausschnitt der Seitenwand. Die äußere Dichtlippe und die Wagenkastenpfosten behalten ihre Breite in allen Darstellungsmaßstäben.'})
}
a.thermal.regions.push({id:'window-seals',title:{en:'Exterior glazing beads',de:'Äußere Verglasungsdichtungen'},kind:'surface',model:'passive',material:'rubber',areaM2:f.sealAreaM2,areaBasis:'total',characteristicLengthM:f.lipWidthM,geometry:'vertical-plate',skyViewFactor:.42,layers:[],heatCapacityJM2K:8900,source:'authored-estimate'})
for(const pose of a.poses)pose.thermal.regions['window-seals']={exposedFraction:f.sealExposedFraction[pose.id],sunlitFraction:.40}
for(const rep of Object.values(a.representations))if(rep.status==='authored')for(const shapes of Object.values(rep.geometry))for(const q of shapes){
 if(q.region!=='glazing'&&!q.regions?.includes('glazing'))continue
 const r=q.regions??[q.region];delete q.region;q.regions=[...new Set([...r,'window-seals'])]
}
if(!bus){
 const ink=a.representations['50'].geometry.front
 const index=ink.findIndex(q=>q.id==='pan-upper'),look=ink[index]
 const xyz=([x,y,z])=>({x,y,z})
 ink.splice(index,1,...['pan-lower-rod','pan-upper-rod'].map(id=>{
  const q=v50.body.find(s=>s.id===id)
  return {id,bone:q.bone,kind:'member',role:'detail',filled:true,tone:q.tone,material:q.material,region:'pantograph',z:look.z,from:xyz(q.p0),to:xyz(q.p1),section:{kind:'circle',d:2*q.r0}}
 }))
 a.thermal.regions.find(q=>q.id==='doors').areaM2=3.60
}
writeFileSync(process.argv[3],JSON.stringify(doc))
