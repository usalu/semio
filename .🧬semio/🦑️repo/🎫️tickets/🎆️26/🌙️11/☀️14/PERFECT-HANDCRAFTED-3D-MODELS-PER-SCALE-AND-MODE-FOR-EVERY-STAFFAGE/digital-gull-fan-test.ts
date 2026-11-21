/** 🪶 Check all ten primary remiges on both hands through folding and gliding, with the thermal and shaft bindings at each scale. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-gull-fan.json',import.meta.url),'utf8')),V=p=>new Vector3(p.x,p.y,p.z)
const bones=new Map(a.rig.bones.map(b=>[b.id,b])),results={}
for(const id of ['standing','gliding','landing','touchdown']){
 const state=evaluateDrawing(a,{kind:'pose',pose:id});results[id]={}
 for(const [side,label] of [['n','near'],['f','far']]){
  const frame=k=>state.bones[k].frame,hand='hand-'+side,first='p1-'+side,last='p10-'+side
  assert.ok(bones.has(first)&&bones.has(last));assert.equal(bones.get(first).parent,hand)
  const angle=V(frame(first).u).angleTo(V(frame(last).u))*180/Math.PI
  assert.ok(id==='standing'?angle<2:angle>12,id+' '+side+' '+angle)
  for(let i=1;i<=10;i++){
   const k='p'+i+'-'+side,q=bones.get(k),b=frame(k),end=V(b.o).addScaledVector(V(b.u),q.lengthM)
   assert.equal(q.parent,hand);assert.ok(Math.abs(q.mass.kg-f.weightG[i-1]/1000)<1e-8)
   for(const v of Object.values(a.volumes)){
    const vane=v.body.find(s=>s.bone===k);assert.ok(vane);assert.ok(v.thermal.some(s=>s.of===vane.id));assert.ok(v.structure.some(s=>s.member===k))
   }
   if(i===9)results[id][label]=end.toArray()
  }
  results[id][label+'Spread']=angle
 }
}
const span=Math.abs(results.gliding.near[2]-results.gliding.far[2])
assert.ok(span>1.23&&span<1.55,String(span))
console.log(JSON.stringify({spanM:span,poses:results}))
