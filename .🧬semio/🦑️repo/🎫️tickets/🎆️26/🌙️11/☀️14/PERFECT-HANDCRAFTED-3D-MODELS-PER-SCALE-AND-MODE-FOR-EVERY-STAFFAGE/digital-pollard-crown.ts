/** 🌳 Trim the outermost leaf sprays to the measured crown and shape the interior wood against its own cambium. */
import {readFileSync,writeFileSync} from 'node:fs';
import {Matrix4,Vector3} from 'three';
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
import {catalogEntries} from './volumes-lib.ts';
const src=catalogEntries().find(q=>q.slug==='london-plane-pollarded')!.path;
const doc=JSON.parse(readFileSync(src,'utf8')),a=doc.asset;
const frames=evaluateDrawing(a,{kind:'pose',pose:a.presentation.defaultPose}).bones;
const matrix=bone=>{
 const {u,v,w,o}=frames[bone].frame;
 return new Matrix4().set(u.x,v.x,w.x,o.x,u.y,v.y,w.y,o.y,u.z,v.z,w.z,o.z,0,0,0,1);
};
for(const [d,vol] of Object.entries(a.volumes) as [string,any][]){
 const leaves=vol.body.filter(s=>s.material.startsWith('plane-leaf')),point=new Vector3();
 let left=Infinity,right=-Infinity;
 for(const leaf of leaves){
  const m=matrix(leaf.bone);
  for(const v of leaf.vertices){const x=point.fromArray(v).applyMatrix4(m).x;left=Math.min(left,x);right=Math.max(right,x);}
 }
 if(!Number.isFinite(left) || right-left<6)throw Error('Unmeasured crown envelope');
 const centre=(left+right)/2,half=(right-left)/2,shoulder=2.70,ratio=(a.dimensions.widthM/2-shoulder)/(half-shoulder);
 for(const leaf of leaves){
  const m=matrix(leaf.bone),inv=m.clone().invert();
  for(const v of leaf.vertices){
   point.fromArray(v).applyMatrix4(m);const x=point.x-centre;
   if(Math.abs(x)<=shoulder)continue;
   point.x=centre+Math.sign(x)*(shoulder+(Math.abs(x)-shoulder)*ratio);
   point.applyMatrix4(inv);v.splice(0,3,...point.toArray().map(q=>+q.toFixed(8)));
  }
 }
 for(const [id,bodyId] of [['member-trunk','stem'],['joint-k-c','knuckle-c']]){
  const member=vol.structure.find(s=>s.id===id),skin=vol.body.find(s=>s.id===bodyId);
  if(!member||!skin)continue;
  const centre=new Vector3();for(const p of skin.vertices)centre.add(point.fromArray(p));centre.divideScalar(skin.vertices.length);
  const factor=id==='member-trunk'?new Vector3(.985,.955,.955):new Vector3(.91,.91,.91);
  member.vertices=skin.vertices.map(v=>point.fromArray(v).sub(centre).multiply(factor).add(centre).toArray().map(q=>+q.toFixed(8)));
  member.triangles=structuredClone(skin.triangles);
 }
 console.log(d,left,right,ratio);
}
writeFileSync(process.argv[2],JSON.stringify(doc));
