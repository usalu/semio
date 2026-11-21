/** 📐 Measures the distance of each outlet centre to its declared tissue surface with Three.js. */
import {Triangle,Vector3} from "three";
import {solidsOf} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️lists/🟦️.ts";
import {copyMesh} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import {entryOf} from "./volumes-lib.ts";
for(const name of process.argv.slice(2)){
 const {asset}=entryOf(name);
 for(const [den,vol] of Object.entries(asset.volumes) as [string,any][]){
  const thermal=new Map(solidsOf(vol,"thermal").map(s=>[s.id,s]));
  for(const src of vol.thermal.filter(s=>s.tags?.includes("emitter"))){
   const e=asset.thermal.emitters.find(e=>src.tags.includes(e.id)&&e.kind==="breath");if(!e)continue;
   const ctr=new Vector3(...src.transform.translation),p=new Vector3(),t=new Triangle();let best=Infinity;
   for(const other of vol.thermal.filter(q=>q.region===src.region&&!q.tags?.includes("emitter"))){
    const solid=thermal.get(other.id); if(!solid||solid.bone!==src.bone)continue;
    const mesh=copyMesh(solid,{chordM:.0001,minSegments:16,maxSegments:32});
    for(let i=0;i<mesh.indices.length;i+=3){
     t.a.fromArray(mesh.positions,mesh.indices[i]*3);t.b.fromArray(mesh.positions,mesh.indices[i+1]*3);t.c.fromArray(mesh.positions,mesh.indices[i+2]*3);
     t.closestPointToPoint(ctr,p);best=Math.min(best,ctr.distanceTo(p));
    }
   }
   console.log(asset.id,den,src.id,"diameter",e.diameterM,"surface distance",best.toFixed(5));
  }
 }
}
