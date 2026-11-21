/** 🧊 Check closed-solid candidate rejection and independent box membership. */
import { readFileSync } from 'node:fs'
import assert from 'node:assert/strict'
import {Box3,BoxGeometry,Vector3} from 'three'
import {insideAny} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts'
const f=JSON.parse(readFileSync(new URL('./digital-thermal-bounds.json',import.meta.url),'utf8'))
const boxes=f.boxes.map(v=>new Box3(new Vector3(...v.slice(0,3)),new Vector3(...v.slice(3))))
const solids=boxes.map((b,id)=>{
 const s=b.getSize(new Vector3()),c=b.getCenter(new Vector3()),g=new BoxGeometry(s.x,s.y,s.z).translate(c.x,c.y,c.z)
 return {id:String(id),positions:Float64Array.from(g.attributes.position.array),indices:Uint32Array.from(g.index.array)}
})
for(const q of f.samples){
 const visited=[]
 assert.equal(boxes.some((b,i)=>!q.skip.includes(i)&&b.containsPoint(new Vector3(...q.at))),q.inside)
 assert.equal(insideAny(solids,q.at,(_,i)=>{visited.push(i);return q.skip.includes(i)}),q.inside)
 assert.deepEqual(visited,q.tested)
}
const base=solids[0],copies=f.copies,positions=new Float64Array(copies.count*base.positions.length),indices=new Uint32Array(copies.count*base.indices.length)
for(let k=0;k<copies.count;k++){
 for(let i=0;i<base.positions.length;i++)positions[k*base.positions.length+i]=base.positions[i]+(i%3===1?k*copies.stepY:0)
 for(let i=0;i<base.indices.length;i++)indices[k*base.indices.length+i]=base.indices[i]+k*base.positions.length/3
}
const merged={id:'rungs',positions,indices},separate={...merged,separateCopies:{indices:base.indices,coordinatesPerCopy:base.positions.length}}
for(const q of copies.samples){
 const inside=Array.from({length:copies.count},(_,i)=>boxes[0].containsPoint(new Vector3(q[0],q[1]-i*copies.stepY,q[2]))).some(Boolean)
 assert.equal(insideAny([merged],q),inside);assert.equal(insideAny([separate],q),inside)
}
console.log(f.samples.length,'closed-solid queries match Three.js and the selected bounding boxes.')
