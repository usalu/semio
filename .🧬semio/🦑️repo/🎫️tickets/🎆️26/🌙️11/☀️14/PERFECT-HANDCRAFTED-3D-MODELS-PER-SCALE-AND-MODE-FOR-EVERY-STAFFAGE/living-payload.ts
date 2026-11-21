/** 🍎 Profile exact glTF buffer views and repeated attribute rows in the serialized catalog model. */
import {deflateRawSync} from "node:zlib"
import {readFileSync,writeFileSync} from "node:fs"
import {join} from "node:path"
import {catalogEntries} from "./volumes-lib.ts"
import {writeGlb} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts"

const slug = process.argv[2]
const entry = catalogEntries().find(e=>e.slug===slug)!
const {asset} = JSON.parse(readFileSync(entry.path,"utf8"))
const bytes = await writeGlb(asset,{pose:asset.presentation.defaultPose})
Bun.gc(true)
const header=new DataView(bytes.buffer,bytes.byteOffset)
const jsonBytes=header.getUint32(12,true)
const doc=JSON.parse(new TextDecoder().decode(bytes.subarray(20,20+jsonBytes)))
const binStart=28+jsonBytes
const binary=bytes.subarray(binStart)
const use=new Map<number,Set<string>>()
function mark(index:number,role:string) {
    const k=doc.accessors[index].bufferView
    if(!use.has(k)) use.set(k,new Set())
    use.get(k)!.add(role)
}
for (const mesh of doc.meshes) for (const p of mesh.primitives){
    for(const [role,i] of Object.entries(p.attributes))mark(i as number,role)
    if(p.indices!==undefined)mark(p.indices,p.mode===1?"edge indices":"triangle indices")
}
const roles:Record<string,any>={}
const values=new DataView(binary.buffer,binary.byteOffset)
const typedWidths:Record<number,number>={5120:1,5121:1,5122:2,5123:2,5125:4,5126:4}
const channels:Record<string,number>={SCALAR:1,VEC2:2,VEC3:3,VEC4:4,MAT4:16}
const unique:Map<string,Set<string>>=new Map()
for(const [i,bv] of doc.bufferViews.entries()){
    const names=use.get(i)
    const role=names?[...names].sort().join(" + "):"other"
    const data=binary.subarray(bv.byteOffset??0,(bv.byteOffset??0)+bv.byteLength)
    const r=roles[role]??={views:0,bytes:0,deflateBytes:0,strided:0,rows:0,sameAsPrevious:0,allZeroRows:0}
    r.views++;r.bytes+=bv.byteLength
    r.deflateBytes+=deflateRawSync(data,{level:1}).length
    if(bv.byteStride)r.strided++
    const attr=doc.accessors.find((a:any)=>a.bufferView===i)
    if(!attr)continue
    const width=typedWidths[attr.componentType],num=channels[attr.type],stride=bv.byteStride??width*num
    const reader=(off:number)=>width===4?values.getUint32(off,true):width===2?values.getUint16(off,true):values.getUint8(off)
    const hasVec=num>1&&num<=4
    if(hasVec&&!unique.has(role))unique.set(role,new Set())
    let prev=""
    for(let j=0;j<attr.count;j++){
        const off=(bv.byteOffset??0)+(attr.byteOffset??0)+stride*j
        const row=Array.from({length:num},(_,c)=>reader(off+width*c))
        const key=row.join(",")
        if(hasVec)unique.get(role)!.add(key)
        if(key===prev)r.sameAsPrevious++
        if(row.every((x:number)=>x===0))r.allZeroRows++
        r.rows++;prev=key
    }
}
for(const [name,seen] of unique)roles[name].distinctRows=seen.size
const report={slug,bytes:bytes.byteLength,jsonBytes,binBytes:binary.length,views:doc.bufferViews.length,accessors:doc.accessors.length,roles}
writeFileSync(join(import.meta.dir,"🗑️generated","living-current",slug+"-payload.json"),JSON.stringify(report))
console.log("[DEBUG]",JSON.stringify(report))

