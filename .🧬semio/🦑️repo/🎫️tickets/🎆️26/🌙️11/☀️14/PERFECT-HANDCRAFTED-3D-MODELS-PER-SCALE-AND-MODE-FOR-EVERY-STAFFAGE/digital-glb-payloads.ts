/** 🧾 Measure the exact byte cost of repeated accessors and local index widths in the published GLB. */
import {createHash} from 'node:crypto'
import {readFileSync} from 'node:fs'
import {writeGlb} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,bytes=Buffer.from(await writeGlb(a,{pose:a.presentation.defaultPose}))
const jsonLength=bytes.readUInt32LE(12),j=JSON.parse(bytes.subarray(20,20+jsonLength).toString('utf8')),base=jsonLength+28
let duplicates=0,repeatCount=0,indexBytes=0
const seen=new Set()
for(const ac of j.accessors){
 const v=j.bufferViews[ac.bufferView],dat=bytes.subarray(base+v.byteOffset,base+v.byteOffset+v.byteLength)
 const key=[ac.type,ac.componentType,!!ac.min,v.target,createHash('sha256').update(dat).digest('hex')].join('/')
 if(seen.has(key)){duplicates+=Math.ceil(v.byteLength/4)*4;repeatCount++}else seen.add(key)
 if(v.target===34963&&ac.componentType===5123){
  let max=0;for(let i=0;i<dat.length;i+=2)max=Math.max(max,dat.readUInt16LE(i))
  if(max<255)indexBytes+=dat.length/2
 }
}
console.log(JSON.stringify({asset:a.id,bytes:bytes.length,jsonLength,accessors:j.accessors.length,duplicates,repeatCount,indexBytes}))
