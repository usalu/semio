/** 🎋 Check the individual folded blades, their connected axils, parent culms and wind response. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Triangle,Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./bamboo-botanical-case.json',import.meta.url),'utf8')),V=p=>new Vector3(...p)
const states=f.poses.map(pose=>evaluateDrawing(a,{kind:'pose',pose}).bones)
for(const k of f.scales){
 const v=a.volumes[k]
 for(let i=1;i<=f.axes;i++)assert.equal(v.body.filter(q=>q.id.startsWith('apex'+i+'-l')&&q.bone==='fol-'+i&&v.thermal.some(t=>t.of===q.id&&t.region.startsWith('leaves-'))).length,3)
 if(k===100){
  const leaves=v.body.filter(q=>/-f\d+-l\d+$/.test(q.id));assert.ok(leaves.length>=1400);assert.equal(v.body.filter(q=>/^b\d+-\d+[ps]-s\d+$/.test(q.id)).length,0)
  for(const q of leaves)assert.ok(v.thermal.some(s=>s.of===q.id))
  console.log(JSON.stringify({k,individualFanLeaves:leaves.length}));continue
 }
 assert.ok(v.body.some(q=>q.id==='bed')&&v.body.some(q=>q.id==='barrier'))
 const plumes=v.body.filter(q=>k===500?q.id.includes('-plume-'):/^b\d+-\d+[ps]-tuft$/.test(q.id)),lengths=[],widths=[],joins=[],axes=new Set()
 for(const q of plumes){
  assert.equal(q.kind,'polyhedron');const count=(q.vertices.length-8)/11;assert.ok(Number.isInteger(count)&&count>=f.minLeavesPerCoarseShoot);axes.add(q.bone)
  const start=3*(2*count+2),vertices=q.vertices.map(V),root=vertices.slice(0,3).reduce((a,b)=>a.add(b),new Vector3()).divideScalar(3)
  const wood=v.body.find(s=>s.id===q.bone),node=wood.vertices.map(V),closest=new Vector3()
  const gap=Math.min(...wood.triangles.map(ids=>new Triangle(...ids.map(i=>node[i])).closestPointToPoint(root,closest).distanceTo(root)))
  assert.ok(gap<=f.maxJoinDistanceM,q.id+' is '+gap+' m from its parent');joins.push(gap)
  assert.ok(v.structure.some(s=>s.bone===q.bone||s.of===q.bone))
  for(let j=0;j<count;j++){
   const coord=start+5*j,section=vertices.slice(coord,coord+4)
   const tip=vertices[coord+4],bud=section.reduce((v,p)=>v.add(p),new Vector3()).divideScalar(4),W=Math.max(...section.flatMap(p=>section.map(q=>p.distanceTo(q)))),L=tip.distanceTo(bud)/.63
   assert.ok(W>=f.leafWidthM[0]&&W<=f.leafWidthM[1],q.id+' width '+W);assert.ok(L>=f.leafLengthM[0]&&L<=f.leafLengthM[1],q.id+' length '+L)
   widths.push(W);lengths.push(L)
  }
  assert.ok(v.thermal.some(t=>t.of===q.id&&t.region.startsWith('leaves-')))
 }
 assert.ok(axes.size>=6);let maximum=0
 for(const bone of axes){const p=states[0][bone].frame.o,q=states[2][bone].frame.o;maximum=Math.max(maximum,Math.hypot(p.x-q.x,p.y-q.y,p.z-q.z))}
 assert.ok(maximum>.03);console.log(JSON.stringify({k,shoots:plumes.length,leaves:lengths.length,maximumJoinDistanceM:Math.max(...joins),rangeM:[Math.min(...lengths),Math.max(...lengths)],widthM:[Math.min(...widths),Math.max(...widths)],maximumParentTravelM:maximum}))
}
