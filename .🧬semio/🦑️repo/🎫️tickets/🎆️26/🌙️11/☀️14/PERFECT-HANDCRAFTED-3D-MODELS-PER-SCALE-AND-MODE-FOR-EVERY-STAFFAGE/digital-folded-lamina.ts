/** 🌾 Keep the fine stations wherever shortening a twisted blade would make its two faces intersect. */
import {readFileSync,writeFileSync} from 'node:fs';
import {polyhedronFault} from '../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/✅️validation/🪨️polyhedron/🟦️.ts';
const path=process.argv[2],doc=JSON.parse(readFileSync(path,'utf8')),v=doc.asset.volumes;
const fine=new Map(v[100].body.map(q=>[q.id,q]));
let kept=0;
v[200].body=v[200].body.map(q=>{
 if(!q.id.startsWith('blade-')||polyhedronFault(q.vertices,q.triangles)===null)return q;
 const leaf=fine.get(q.id);
 if(!leaf||polyhedronFault(leaf.vertices,leaf.triangles)!==null)throw Error(q.id);
 kept++;return leaf;
});
writeFileSync(process.argv[3],JSON.stringify(doc));console.log('[DEBUG] retained twisted blades',kept);
