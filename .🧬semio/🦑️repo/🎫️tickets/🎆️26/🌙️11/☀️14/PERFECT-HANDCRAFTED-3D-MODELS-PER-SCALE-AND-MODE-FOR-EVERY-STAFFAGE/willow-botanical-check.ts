/** 🌳 Measure folded willow blades, their parent switch and the wind travel of every pendant group. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Triangle,Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./willow-botanical-case.json',import.meta.url),'utf8'))
const V=p=>new Vector3(...p),p3=p=>new Vector3(p.x,p.y,p.z),world=(b,p)=>p3(b.o).addScaledVector(p3(b.u),p[0]).addScaledVector(p3(b.v),p[1]).addScaledVector(p3(b.w),p[2])
const states=[...f.poses.map(pose=>({kind:'pose',pose})),...[0,1.3,2.6,3.9,5.2].map(timeS=>({kind:'clip',clip:'wind-sway',timeS}))].map(o=>evaluateDrawing(a,o).bones)
for(const k of f.scales){
 const v=a.volumes[k],bladeSets=v.body.filter(q=>q.id.startsWith('blade-')),twigs=new Map(v.body.filter(q=>q.id.startsWith('twig-')).map(q=>[q.id,q]))
 assert.ok(bladeSets.length>=108);const branches=new Map(),sizes=[],breadths=[];let leafCount=0,maxGap=0
 for(const s of bladeSets){
  const id=s.id.slice(6).replace(/-\d+$/,'');assert.ok(twigs.has('twig-'+id));assert.ok(v.thermal.some(t=>t.of===s.id&&t.region==='leaf-curtain'))
  assert.ok(s.vertices.length===8)
  for(let j=0;j<s.vertices.length;j+=8){
   const base=V(s.vertices[j]),tip=V(s.vertices[j+3]),L=base.distanceTo(tip),W=V(s.vertices[j+1]).distanceTo(V(s.vertices[j+5]))
   const twig=twigs.get('twig-'+id),gap=Math.min(...twig.triangles.map(t=>new Triangle(...t.map(i=>V(twig.vertices[i]))).closestPointToPoint(base,new Vector3()).distanceTo(base)))
   assert.ok(gap<=f.maximumLeafGapM,s.id+' attachment '+gap);maxGap=Math.max(maxGap,gap)
   assert.ok(L>=f.bladeLengthM[0]&&L<=f.bladeLengthM[1],s.id+' '+L);assert.ok(W>=f.bladeWidthM[0]&&W<=f.bladeWidthM[1]);sizes.push(L);breadths.push(W);leafCount++
  }
  if(!branches.has(s.bone)||s.vertices.at(-1)[0]>branches.get(s.bone)[0])branches.set(s.bone,s.vertices.at(-1))
 }
 assert.equal(branches.size,f.minimumMovingBranches);const travels=[]
 for(const [bone,p] of branches){
  const positions=states.map(b=>world(b[bone].frame,p)),storm=positions[0].distanceTo(positions[2]),cycle=Math.max(...positions.slice(3).map(v=>v.distanceTo(positions[3])))
  assert.ok(storm>f.minimumStemTravelM&&cycle>f.minimumStemTravelM);travels.push([storm,cycle])
 }
 console.log(JSON.stringify({scale:k,leaves:leafCount,maximumAttachmentGapM:maxGap,bladeM:[Math.min(...sizes),Math.max(...sizes)],widthM:[Math.min(...breadths),Math.max(...breadths)],minimumStormTravelM:Math.min(...travels.map(v=>v[0])),minimumCycleTravelM:Math.min(...travels.map(v=>v[1]))}))
}
