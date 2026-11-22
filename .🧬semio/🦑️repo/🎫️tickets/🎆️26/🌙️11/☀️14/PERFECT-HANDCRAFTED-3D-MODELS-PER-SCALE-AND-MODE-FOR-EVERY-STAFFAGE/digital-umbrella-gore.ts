/** ☔ Cut eight nylon gores with two alternating radial pleats per gore around the moving umbrella ferrule. */
import {readFileSync,writeFileSync} from 'node:fs'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const [source,target]=process.argv.slice(2),doc=JSON.parse(readFileSync(source,'utf8')),a=doc.asset
const sides=['front','front-l','l','back-l','back','back-r','r','front-r'],count=4,length=.56,gauge=.00016
const rad=x=>x*Math.PI/180,deg=x=>x*180/Math.PI,round=x=>+x.toFixed(9)
const phases=[...a.poses.map(p=>p.bones),...a.clips.flatMap(c=>c.keys.filter(k=>k.bones).map(k=>({...a.poses.find(p=>p.id===k.pose).bones,...k.bones})))]
const direction=(b,q)=>{const pitch=rad(b.angleDeg+(q.angleDeg??0)),spread=rad((b.abductionDeg??0)+(q.abductionDeg??0));return [Math.cos(pitch)*Math.cos(spread),Math.sin(pitch)*Math.cos(spread),Math.sin(spread)]}
a.rig.bones=a.rig.bones.filter(b=>!b.id.startsWith('pleat-'))
delete a.rig.radialFolds
const bones=new Map(a.rig.bones.map(b=>[b.id,b])),facets=[]
const states=a.clips.flatMap(c=>Array.from({length:81},(_,i)=>evaluateDrawing(a,{kind:'clip',clip:c.id,timeS:c.durationS*i/80}).bones))
a.rig.radialFolds=sides.map((side,j)=>{
 const left='rib-'+side,right='rib-'+sides[(j+1)%sides.length],cut=Math.max(...phases.map(q=>{
  const u=direction(bones.get(left),q[left]),v=direction(bones.get(right),q[right]);return Math.acos(Math.min(1,u.reduce((s,x,i)=>s+x*v[i],0)))
 }),...states.map(b=>{const p=b[left].frame.u,q=b[right].frame.u;return Math.acos(Math.min(1,p.x*q.x+p.y*q.y+p.z*q.z))}))+rad(.03)
 const ids=[]
 for(let i=0;i<count;i++){
  const bone='pleat-'+side+'-'+(i+1),id='canopy-'+side+'-'+(i+1)
  a.rig.bones.push({id:bone,parent:'umbrella',title:{en:'Gore '+(j+1)+', pleat face '+(i+1),de:'Bahn '+(j+1)+', Faltenfläche '+(i+1)},x:.78,y:0,angleDeg:0,lengthM:length,joint:{type:'fixed'}})
  const profile=[[0,0]]
  for(let n=0;n<=3;n++){
   const at=(i+n/3)/count,phi=cut*at,hem=length*Math.cos(cut/2)/Math.cos(phi-cut/2)-.003*4*at*(1-at)
   profile.push([round(-hem*Math.sin(phi-i*cut/count)),round(hem*Math.cos(phi-i*cut/count))])
  }
  ids.push(bone);facets.push({id,kind:'extrude',bone,material:'umbrella-fabric',role:'body',tone:.285+.014*(j%3)+.004*(i%2),plane:'zx',profile,from:-gauge/2,to:gauge/2})
 }
 return {left,right,facets:ids,cutAngleDeg:round(deg(cut))}
})
const surface=a.thermal.regions.find(r=>r.id==='canopy')
surface.layers=[{id:'pongee',title:{en:'Coated nylon pongee',de:'Beschichteter Nylon-Pongee'},kind:'polymer',material:'umbrella-fabric',thicknessM:gauge,resistanceM2KW:gauge/.25,source:'authored-estimate'}]
const midArea=facets.reduce((total,p)=>total+p.profile.slice(2).reduce((sum,[z,x],j)=>sum+Math.abs(p.profile[j+1][1]*z-p.profile[j+1][0]*x)/2,0),0)
surface.areaM2=round(2*midArea)
function replace(items,match,parts){
 const at=items.findIndex(match);if(at<0)throw Error('Missing canopy')
 return [...items.slice(0,at),...parts,...items.slice(at).filter(s=>!match(s))]
}
for(const [s,vol] of Object.entries(a.volumes)){
 vol.body=replace(vol.body,p=>p.material==='umbrella-fabric',facets)
 vol.thermal=replace(vol.thermal,p=>p.region==='canopy',facets.map(p=>({id:'th-'+p.id,of:p.id,region:'canopy',layer:'pongee'})))
 vol.structure=vol.structure.filter(s=>!s.id.startsWith('hem-canopy-'))
 for(const p of facets){
  const at=([z,x])=>[x,0,z]
  vol.structure.push({id:'hem-'+p.id,kind:'capsule',bone:p.bone,member:p.bone,part:'member',material:p.material,role:'body',tone:.26,slender:true,p0:at(p.profile[1]),p1:at(p.profile.at(-1)),r0:.0009,r1:.0009})
 }
 const geo=a.representations[s].geometry
 for(const [v,shapes] of Object.entries(geo)){
  const template=shapes.find(p=>p.id.startsWith('canopy-'))
  if(!template)continue
  geo[v]=replace(shapes,p=>p.id.startsWith('canopy-'),facets.map((p,i)=>({...template,id:p.id,bone:p.bone,z:template.z+i*.0001,tone:p.tone,region:'canopy',kind:'poly',penMm:.22,points:p.profile.map(([z,x])=>[x,z]),closed:true,solid:{kind:'extruded',sweep:'v',z0:p.from,z1:p.to}})))
 }
 const n=vol.decisions[0]
 const english=Math.max(n.en.lastIndexOf('; the umbrella'),n.en.lastIndexOf('. Eight nylon gores')),german=n.de.search(/; (der Schirm|das Schirmdach)|\. Acht Nylonbahnen/)
 const en=english>=0?english:n.en.lastIndexOf('; the canopy');if(en<0||german<0)throw Error('Unrecognized body note '+s)
 n.en=n.en.slice(0,en)+'. Eight nylon gores meet at the ferrule. Two accordion pleats in each gore carry the sewn hem between the moving rib tips.'
 n.de=n.de.slice(0,german)+'. Acht Nylonbahnen treffen an der Zwinge zusammen. Zwei gegenläufige Falten je Bahn führen den vernähten Saum zwischen den beweglichen Rippenspitzen.'
}
writeFileSync(target,JSON.stringify(doc))
