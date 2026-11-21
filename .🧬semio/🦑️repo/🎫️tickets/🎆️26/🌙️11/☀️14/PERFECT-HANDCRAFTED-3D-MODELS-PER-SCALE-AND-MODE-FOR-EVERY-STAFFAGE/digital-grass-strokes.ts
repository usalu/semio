/** 🌾 Draw the articulated grass blades along their measured spatial midribs in each projection. */
import {readFileSync,writeFileSync} from 'node:fs';
import {poseState,projectWorld} from './volumes-lib.ts';
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts';
import {centreLineOf,clusterPoints} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧵️centreline/🟦️.ts';
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,pose=evaluateDrawing(a,{kind:'pose',pose:a.presentation.defaultPose});
for(const d of [200,500]){
 const v=a.representations[d],world=worldSolids(a,pose.bones,d).filter(q=>q.id.startsWith('blade-')),byid=new Map(world.map(q=>[q.id,q]));
 for(const view of ['elevation','front','plan']){
  const stems=v.geometry[view].filter(s=>byid.has(s.id));
  const probes=stems.map(s=>({...s,kind:'poly',points:[[0,0],[.1,0],[0,.1]],closed:true,filled:true}));
  const frames=new Map(poseState({...a,representations:{...a.representations,[d]:{...v,geometry:{...v.geometry,[view]:probes}}}},d)[view].map(q=>[q.shape.id,q.rings[0]]));
  for(const s of stems){
   const mesh=byid.get(s.id),line=centreLineOf(mesh.positions,mesh.positions,mesh.indices);if(!line)continue;
   const [o,x,y]=frames.get(s.id),ux=(x.x-o.x)*10,uy=(x.y-o.y)*10,vx=(y.x-o.x)*10,vy=(y.y-o.y)*10,det=ux*vy-uy*vx;if(Math.abs(det)<1e-8)continue;
   const pts=clusterPoints(line,mesh.positions,0).map(p=>{
    const q=projectWorld(view,p),dx=q.x-o.x,dy=q.y-o.y;
    return [ +((vy*dx-vx*dy)/det).toFixed(6),+((ux*dy-uy*dx)/det).toFixed(6)];
   });
   const chains:number[][]=[];
   for(const [j,k] of line.edges){
    const chain=chains.findLast(seq=>seq.at(-1)===j);
    if(chain)chain.push(k);else chains.push([j,k]);
   }
   if(!chains.length)continue;
   if(chains.length===1){s.kind='poly';s.points=chains[0].map(i=>pts[i]);}
   else{s.kind='path';s.d=chains.map(seq=>'M '+seq.map(i=>pts[i].join(' ')).join(' L ')).join(' ');delete s.points;delete s.closed;}
  }
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc));
