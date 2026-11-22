/** 🦴 Check each person's own shaft section and both bones of each forearm against the anatomical master. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Vector2} from 'three'
const f=JSON.parse(readFileSync(new URL('./digital-limb-gauge.json',import.meta.url),'utf8')),a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
let count=0
function section(q){
 if(q.kind==='capsule')return new Vector2(q.r0,q.r1)
 if(q.kind==='loft'){const short=q.sections.reduce((a,b)=>a.rx*a.ry<b.rx*b.ry?a:b);return new Vector2(short.rx,short.ry)}
 throw Error(q.id+' '+q.kind)
}
for(const key of f.pairedShafts)for(const side of ['l','r']){
 const id=key+'-'+side,master=a.volumes['50'].structure.find(q=>q.id===id);assert.ok(master,id)
 for(const scale of f.scales){const bone=a.volumes[scale].structure.find(q=>q.id===id);assert.ok(bone,scale+' '+id);assert.ok(section(bone).distanceTo(section(master))<=f.toleranceM,scale+' '+id);count++}
}
console.log(JSON.stringify({asset:a.id,sections:count}))
