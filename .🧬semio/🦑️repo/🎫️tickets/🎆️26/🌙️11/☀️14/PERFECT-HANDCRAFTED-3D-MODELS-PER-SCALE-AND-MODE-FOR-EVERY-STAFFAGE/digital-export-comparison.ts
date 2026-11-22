/** 🪝 Compare the binary geometry, primitive names and JSON overhead of two crane exports. */
import {readFileSync} from 'node:fs'
import {writeGlb} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts'
for(const p of process.argv.slice(2)){
 const a=JSON.parse(readFileSync(p,'utf8')).asset,bytes=Buffer.from(await writeGlb(a,{pose:a.presentation.defaultPose})),n=bytes.readUInt32LE(12)
 const doc=JSON.parse(bytes.subarray(20,20+n).toString()),bins=doc.bufferViews.reduce((sum,v)=>sum+v.byteLength,0)
 console.log(JSON.stringify({p:p.split('/').at(-1),bytes:bytes.length,json:n,bin:bins,nodes:doc.nodes.length,meshes:doc.meshes.length,accessors:doc.accessors.length,primitives:doc.meshes.reduce((n,m)=>n+m.primitives.length,0)}))
 const metric=m=>m.primitives.reduce((sum,p)=>sum+[p.indices,...Object.values(p.attributes)].reduce((z,i)=>z+doc.bufferViews[doc.accessors[i].bufferView].byteLength,0),0)
 console.log(JSON.stringify(doc.meshes.map(m=>[m.name,metric(m)]).sort((a,b)=>b[1]-a[1]).slice(0,10)))
 Bun.gc(true)
}
