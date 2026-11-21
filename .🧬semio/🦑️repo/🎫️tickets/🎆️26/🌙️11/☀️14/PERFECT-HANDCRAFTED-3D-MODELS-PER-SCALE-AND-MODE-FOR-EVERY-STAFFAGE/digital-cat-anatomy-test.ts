/** 🐈 Verify the corneal aperture, iris boundary and seated caudal chain against the actual paws. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-cat-anatomy.json',import.meta.url),'utf8')),state=evaluateDrawing(a,{kind:'pose',pose:'sitting-tail-wrapped'})
const V=p=>new Vector3(p.x,p.y,p.z),bone=id=>a.rig.bones.find(b=>b.id===id),frame=id=>state.bones[id].frame
const tip=id=>V(frame(id).o).addScaledVector(V(frame(id).u),bone(id).lengthM)
assert.ok(tip('tail-2').z<tip('paw-fn').z-f.sittingOuterClearanceM)
assert.ok(tip('tail-5').x>tip('paw-fn').x+f.earliestTipXBeyondForepawM)
assert.ok(tip('tail-5').z>tip('tail-3').z+.03)
for(const k of [50,100,200,500]){
 const v=a.volumes[k]
 for(let i=1;i<=5;i++)assert.ok(v.body.some(s=>s.bone==='tail-'+i)&&v.structure.some(s=>s.member==='tail-'+i))
 for(const side of ['n','f']){
  const thermal=v.thermal.find(s=>s.id==='th-eye-'+side)
  assert.equal(thermal.kind,'loft');assert.equal(thermal.capStart,false)
  if(k>100)continue
  const eye=v.body.find(s=>s.id==='eye-'+side);assert.equal(eye.radii[2]*2,f.eyeWidthM);assert.equal(eye.radii[1]*2,f.eyeHeightM)
  assert.ok(v.body.some(s=>s.id==='pupil-'+side))
 }
}
console.log('Wrapped tail endpoints in millimetres',...Array.from({length:5},(_,i)=>tip('tail-'+(i+1)).toArray().map(v=>Math.round(v*1000)).join('/')))
