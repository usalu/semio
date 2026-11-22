/** 🍃 Identify each invalid new shell before the larger plot, export and pose checks. */
import {readFileSync} from 'node:fs'
import {polyhedronFault} from '../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
for(const [k,v] of Object.entries(a.volumes)){
 const bad=[]
 for(const q of v.body)if(q.kind==='polyhedron'&&polyhedronFault(q.vertices,q.triangles))bad.push(q.id)
 console.log(JSON.stringify({scale:k,bad}))
}
