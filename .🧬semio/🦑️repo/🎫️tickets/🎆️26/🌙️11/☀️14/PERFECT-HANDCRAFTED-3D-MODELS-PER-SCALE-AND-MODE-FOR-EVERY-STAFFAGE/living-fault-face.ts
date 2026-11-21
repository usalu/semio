import {readFileSync} from 'node:fs'
import {resolve,dirname} from 'node:path'
import {pathToFileURL} from 'node:url'
import {catalogEntries} from './volumes-lib.ts'
const filename=resolve('🧰️framework/🔨️modules/🧊️3d/🗿️model/✅️validation/🪨️polyhedron/🟦️.ts')
const code=readFileSync(filename,'utf8').replaceAll(/from "([^"]+)"/g,(_,s)=>'from "'+pathToFileURL(resolve(dirname(filename),s))+'"').replace('if (crosses(a.ids, b.ids, points)) return "triangles";','if(crosses(a.ids,b.ids,points)){console.log("[DEBUG]",a.ids,b.ids);return "triangles";}')
const js=new Bun.Transpiler({loader:'ts'}).transformSync(code);const {polyhedronFault}=await import('data:text/javascript;base64,'+Buffer.from(js).toString('base64'))
for(const [name,solid] of [['hosta-clump','blade-20-l'],['hosta-clump','blade-22-l']]){
 const p=catalogEntries().find(e=>e.slug===name)!.path;const s=JSON.parse(readFileSync(p,'utf8')).asset.volumes['50'].body.find((s:any)=>s.id===solid)
 console.log(name,solid,polyhedronFault(s.vertices,s.triangles))
}
