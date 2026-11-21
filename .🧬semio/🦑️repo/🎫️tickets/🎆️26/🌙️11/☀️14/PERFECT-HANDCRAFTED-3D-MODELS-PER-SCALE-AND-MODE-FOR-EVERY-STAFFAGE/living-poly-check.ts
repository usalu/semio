import {readFileSync} from 'node:fs'
import {catalogEntries} from './volumes-lib.ts'
import {polyhedronFault} from '../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts'
for(const name of process.argv.slice(2)){
 const path=catalogEntries().find(e=>e.slug===name)!.path
 const asset=JSON.parse(readFileSync(path,'utf8')).asset
 const cache=new Map<string,string|null>();let total=0, bad=0
 for(const [k,v] of Object.entries(asset.volumes) as [string,any][])for(const m of ['body','thermal','structure'])for(const s of v[m])if(s.kind==='polyhedron'){
  const key=JSON.stringify([s.vertices,s.triangles]); let fault=cache.get(key)
  if(fault===undefined){fault=polyhedronFault(s.vertices,s.triangles);cache.set(key,fault)}
  total++;if(fault){bad++;if(bad<=30)console.log('[DEBUG]',name,k,m,s.id,fault)}
 }
 console.log('[DEBUG]',name,total,cache.size,bad)
}
