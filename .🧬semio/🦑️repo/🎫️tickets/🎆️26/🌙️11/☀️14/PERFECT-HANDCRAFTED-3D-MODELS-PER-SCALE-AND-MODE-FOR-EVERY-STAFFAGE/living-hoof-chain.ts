/** 🐾 Verify the true separation of every pair of claws under the native joint poses and independent Three.js frame geometry. */
import {readFileSync} from 'node:fs'
import {join} from 'node:path'
import {Matrix4,Vector3} from 'three'
import {catalogEntries} from './volumes-lib.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const test=JSON.parse(readFileSync(join(import.meta.dir,'living-hoof-chain.json'),'utf8'))
let samples=0,worst=0
for(const sp of test.animals){
 const file=catalogEntries().find(e=>e.slug===sp.id)!.path,a=JSON.parse(readFileSync(file,'utf8')).asset
 const states=[...new Set([a.presentation.defaultPose,sp.pose])].map(pose=>evaluateDrawing(a,{kind:'pose',pose}))
 for(const d of test.scales){const vol=a.volumes[d],solids=new Map(vol.body.map(s=>[s.id,s]))
  test.feet.forEach((foot,i)=>{const bone='hoof-'+foot,ref=a.rig.bones.find(b=>b.id===bone)
   if(!ref?.parent)throw Error(sp.id+' '+bone+' missing joint')
   const pair=sp.sides.map(side=>sp.pattern.replace('{foot}',foot).replace('{near}',foot[1]==='n'?'near':'far').replace('{end}',foot[0]==='h'?'hind':'fore').replace('{side}',side)).map(id=>{
    const s=solids.get(id);if(!s||s.bone!==bone||s.kind!=='loft'||!vol.thermal.some(t=>t.of===id&&t.region==='hooves'))throw Error(sp.id+' '+d+' '+id)
    const pt=s.sections.at(-1).at
    const inside=s.sections.every(q=>q.rx>0&&q.ry>0)
    if(!inside)throw Error('Invalid claw gauge')
    return new Vector3(...pt)
   })
   for(const state of states){const f=state.bones[bone].frame,m=new Matrix4().set(f.u.x,f.v.x,f.w.x,f.o.x,f.u.y,f.v.y,f.w.y,f.o.y,f.u.z,f.v.z,f.w.z,f.o.z,0,0,0,1)
    const actual=pair[0].clone().applyMatrix4(m).distanceTo(pair[1].clone().applyMatrix4(m))*1000
    const error=Math.abs(actual-sp.distances[i]);if(error>.001)throw Error([sp.id,d,foot,actual,sp.distances[i]].join(' '))
    samples++;worst=Math.max(worst,error)
   }
  })
 }
}
console.log('[DEBUG]',test.animals.length,'species',samples,'posed claw pairs, maximum error in mm',worst)
