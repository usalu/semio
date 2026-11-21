/** 🎞 Read the exported passenger animations through Three.js and compare posed joint positions with the evaluated catalogue. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {AnimationMixer,Vector3} from 'three'
import {FBXLoader} from 'three/examples/jsm/loaders/FBXLoader.js'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {writeFbx} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎬️fbx/🟦️.ts'
const [source,clipName,k]=process.argv.slice(2),asset=JSON.parse(readFileSync(source,'utf8')).asset
const clip=asset.clips.find(q=>q.id===clipName),bytes=await writeFbx(asset,{clip:clipName,scale:Number(k),view:'light-solid'})
const scene=new FBXLoader().parse(bytes.buffer.slice(bytes.byteOffset,bytes.byteOffset+bytes.byteLength),'')
assert.equal(scene.animations.length,1)
const mixer=new AnimationMixer(scene);mixer.clipAction(scene.animations[0]).play()
const bones=asset.rig.bones.filter(b=>b.id.startsWith('child-')||b.id.startsWith('toddler-'))
let maximum=0
for(const fraction of [0,.25,.5,.75,.9]){
 const timeS=clip.durationS*fraction;mixer.setTime(timeS);scene.updateMatrixWorld(true)
 const state=evaluateDrawing(asset,{kind:'clip',clip:clipName,timeS})
 for(const b of bones){
  const expected=state.bones[b.id].frame.o,node=scene.getObjectByName(b.id)
  assert.ok(node,b.id);const error=node.getWorldPosition(new Vector3()).distanceTo(new Vector3(expected.z,expected.y,expected.x))
  maximum=Math.max(maximum,error)
 }
}
console.log(JSON.stringify({id:asset.id,clip:clipName,scale:Number(k),frames:5,bones:bones.length,bytes:bytes.length,maximumErrorM:maximum}))
assert.ok(maximum<=.006,'Exported joint displacement exceeds 6 mm.')
