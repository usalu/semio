/** 🌳 Localize each distributed crown load inside the same hanging foliage mesh used by the coarse model. */
import {readFileSync,writeFileSync} from 'node:fs'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,v=a.volumes[500],parts=new Map(v.body.map(q=>[q.id,q]))
for(const m of v.structure.filter(q=>q.part==='mass'&&q.of)){
 const foliage=parts.get(m.of),verts=foliage.vertices
 if(foliage.kind!=='polyhedron'||foliage.material!=='birch-leaf')throw Error(m.id)
 const n=foliage.triangles[0][2],r=verts.slice(2*n,3*n),mean=ring=>[0,1,2].map(j=>ring.reduce((s,p)=>s+p[j],0)/ring.length),center=mean(r),span=[0,1,2].map(j=>Math.max(...verts.map(p=>p[j]))-Math.min(...verts.map(p=>p[j])))
 const envelope=structuredClone(foliage),factor=Math.min(.48,.18/Math.hypot(...span))
 envelope.vertices=verts.map(p=>p.map((v,j)=>+(center[j]+(v-center[j])*factor).toFixed(7)))
 const {id,member,part}=m;delete m.of
 Object.assign(m,envelope,{id,member,part})
}
v.decisions.unshift({en:'The crown load stays distributed among the same foliage stations. Each station appears as a small inclusion within its hanging branch. The outline of the load-bearing wood remains visible.',de:'Die Kronenlast bleibt auf dieselben Laubstationen verteilt. Jede wird als kleiner Einschluss im zugehörigen Hängezweig dargestellt.'})
writeFileSync(process.argv[3],JSON.stringify(doc))
