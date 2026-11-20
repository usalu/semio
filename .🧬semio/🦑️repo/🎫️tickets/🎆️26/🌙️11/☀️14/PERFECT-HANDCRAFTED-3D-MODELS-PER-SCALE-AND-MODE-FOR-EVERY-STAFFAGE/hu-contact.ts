/** 🦶 Independently measures distance from active contact anchors to the associated body's triangle surface in the default pose. */
import { Triangle, Vector3 } from 'three';
import { catalogEntries,loadAsset } from './volumes-lib.ts';
import { listsIn,rigAt } from './r3-lib.ts';
const triangle=new Triangle(),point=new Vector3(),hit=new Vector3();
for(const entry of catalogEntries().filter(e=>e.category==='humans')){
 const a=loadAsset(entry.path);
 const rig=rigAt(a,50,{kind:'pose',pose:a.presentation.defaultPose});
 const m=listsIn(a,50,rig.frames);
 for(const id of new Set([...rig.state.contacts,...a.structure.supports])){
  const def=a.anchors.find(x=>x.id===id),p=rig.state.anchors[id];
  
  point.set(p.at.x,p.at.y,p.z??0);
  let min=Infinity,near='';
  for(const [i,mesh] of m.body.entries()){
   if(m.solids.body[i].bone!==def.bone)continue;
   for(let j=0;j<mesh.indices.length;j+=3){
    for(const [k,v] of [triangle.a,triangle.b,triangle.c].entries())v.fromArray(mesh.world,mesh.indices[j+k]*3);
    triangle.closestPointToPoint(point,hit);
    const d=hit.distanceTo(point);
    if(d<min){min=d;near=mesh.id}
   }
  }
  console.log([entry.slug,id,near,min.toFixed(4),point.y.toFixed(4)].join('\t'))
 }
}
