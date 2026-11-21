/** 🧪 Tests the actual posed meshes for six open pipe mouths, the isolated charging circuit, the dry pocket, and the suspended ash receiver. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
import {insideAny} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts'
const {asset}=JSON.parse(readFileSync(process.argv[2],'utf8')),state=evaluateDrawing(asset,{kind:'pose',pose:asset.presentation.defaultPose})
const vec=(q)=>[q.x,q.y,q.z],world=(p,f)=>vec(f.o).map((o,i)=>o+p.reduce((s,x,j)=>s+x*vec(f[['u','v','w'][j]])[i],0))
for(const scale of [50,100,200,500]){
 const solids=worldSolids(asset,state.bones,scale),by=Object.fromEntries(solids.map(q=>[q.id,q])),issues=[];let checks=0
 function test(p,expected,why,sol=solids){checks++;const got=insideAny(sol,p);if(got!==expected)issues.push([why,p.map(x=>Math.round(x*1e5)/1e5),sol.filter(q=>insideAny([q],p)).map(q=>q.id)])}
 if(asset.id==='hot-water-cylinder'){
  const f=state.bones.connections.frame
  for(const [side,y]of [[-1,1.1977],[-1,.7852],[-1,.1893],[1,.9227],[1,.5102]]){
   for(let x=.2355;x<=.332;x+=.001)for(const z of [-.006,0,.006])test(world([side*x,y,z],f),false,'open side bore')
   for(let x=.243;x<.33;x+=.007)test(world([side*x,y,.010],f),true,'steel pipe wall')
  }
  for(let y=1.242;y<=1.480;y+=.0015)for(const dx of [-.008,0,.008])test(world([.2039+dx,y,0],f),false,'open hot outlet')
  for(let y=1.244;y<1.478;y+=.009)test(world([.2039,y,.013],f),true,'outlet wall')
  for(let x=-.236;x<-.190;x+=.0025)test(world([x,.7852,0],f),false,'dry pocket')
  test(world([-.187,.7852,0],f),true,'closed pocket end')
  for(let i=0;i<=40;i++){const angle=-Math.PI/3-20/3*Math.PI*2*i/40,y=.9227+(.5102-.9227)*i/40;test(world([.16*Math.cos(angle),y,.16*Math.sin(angle)],f),false,'coil bore');test(world([.16*Math.cos(angle),y+.010,.16*Math.sin(angle)],f),true,'coil wall',[by['charging-coil']])}
 }else{
  const f=state.bones.bowl.frame
  for(let x=-.12;x<=.12;x+=.010){test(world([x,.386,0],f),true,'ash pan floor',[by['ash-pan']]);test(world([x,.409,0],f),false,'open ash pan',[by['ash-pan']])}
  for(let x=-.10;x<=.10;x+=.01){test(world([x,.584,0],f),true,'bed carried above grate',[by['charcoal-bed']]);test(world([x,.565,0],f),false,'fuel below grate',[by['charcoal-bed']])}
  const positions=(q)=>Array.from(q.positions).filter((_,i)=>i%3===1),span=q=>[Math.min(...positions(q)),Math.max(...positions(q))]
  const pan=span(by['ash-pan']),bed=span(by['charcoal-bed']),fuelBars=solids.filter(q=>q.id.startsWith('fuel-bar-')),top=Math.max(...fuelBars.map(q=>span(q)[1]));assert(Math.abs(top-bed[0])<.0003)
  assert(Math.abs(pan[0]-.385)<.0001&&Math.abs(pan[1]-.418)<.0001&&Math.abs(bed[0]-.578)<.0001&&Math.abs(bed[1]-.618)<.0001)
 }
 console.log(JSON.stringify({id:asset.id,scale,checks,problems:issues.length,first:issues.slice(0,3)}));if(issues.length)process.exitCode=1
}
