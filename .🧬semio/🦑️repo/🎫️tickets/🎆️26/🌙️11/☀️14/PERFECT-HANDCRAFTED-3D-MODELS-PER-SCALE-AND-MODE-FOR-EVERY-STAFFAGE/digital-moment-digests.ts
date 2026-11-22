/** 🧮️ Recompute GLB and FBX structure digests after omitting unshown moment arcs and read each package independently. */
import {writeFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {scaledFigure} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧊️volumes/🟦️.ts'
import {checkGlb,checkFbx,expectedScenes} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts'
import {glbDigestOf,fbxDigestOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧊️digests/🟦️.ts'
import {writeGlb} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts'
import {writeFbxDetailed} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎬️fbx/🟦️.ts'
const a=scaledFigure(),[subject,out]=process.argv.slice(2)
if(subject.endsWith('.fbx')){
 const [,id,denominator,view]=subject.split('.'),scale=Number(denominator.slice(1)),clip=a.clips.find(c=>c.id===id)
 const file=await writeFbxDetailed(a,{clip:id,scale,view})
 const review=await checkFbx(file.bytes,subject,{clip:id,durationS:clip.durationS,fps:clip.fps,bones:a.rig.bones.map(b=>b.id)})
 assert.deepEqual(review.findings,[])
 writeFileSync(out,JSON.stringify(fbxDigestOf(file)))
}else{
 const bytes=await writeGlb(a,{pose:subject}),review=await checkGlb(bytes,subject+'.glb',subject,expectedScenes(a))
 assert.deepEqual(review.findings,[])
 writeFileSync(out,JSON.stringify(glbDigestOf(bytes)))
}
