/** 🌱 Draw each slender epicormic shoot along its own tapered wooden centreline. */
import {readFileSync,writeFileSync} from 'node:fs';
import {centreLineOf,clusterPoints} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧵️centreline/🟦️.ts';
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts';
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
import {poseState} from './volumes-lib.ts';
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,v=a.volumes[50],rep=a.representations[50];
const solved=evaluateDrawing(a,{kind:'pose',pose:a.presentation.defaultPose});
const limbs=worldSolids(a,solved.bones,50).filter(s=>s.slender);
const probes=limbs.map(s=>({kind:'poly',closed:true,filled:true,role:'body',tone:.37,id:s.id,bone:s.bone,z:2,points:[[0,0],[.1,0],[0,.1]]}));
const pa={...a,representations:{...a.representations,50:{...rep,geometry:{...rep.geometry,elevation:probes}}}};
const maps=poseState(pa,50).elevation;
const items=rep.geometry.elevation.filter(s=>!(s.role==='body'&&!s.filled&&s.id.endsWith('-line')));
for(const limb of limbs){
 const line=centreLineOf(limb.positions,limb.positions,limb.indices);if(!line)throw Error(limb.id);
 const [o,u,v]=maps.find(s=>s.shape.id===limb.id).rings[0];
 const ax=(u.x-o.x)*10,ay=(u.y-o.y)*10,bx=(v.x-o.x)*10,by=(v.y-o.y)*10,det=ax*by-ay*bx;
 const pts=clusterPoints(line,limb.positions,0).map(([x,y,z])=>[+((by*(x-o.x)-bx*(y-o.y))/det).toFixed(6),+((ax*(y-o.y)-ay*(x-o.x))/det).toFixed(6)]);
 items.push({kind:'path',id:'axis-'+limb.id,bone:limb.bone,role:'body',z:2.5,tone:.45,filled:false,material:'plane-twig',region:'shoot-bark',penMm:.20,d:line.edges.map(([i,j])=>'M '+pts[i].join(' ')+' L '+pts[j].join(' ')).join(' ')});
}
rep.geometry.elevation=items.sort((a,b)=>a.z-b.z);writeFileSync(process.argv[3],JSON.stringify(doc));
