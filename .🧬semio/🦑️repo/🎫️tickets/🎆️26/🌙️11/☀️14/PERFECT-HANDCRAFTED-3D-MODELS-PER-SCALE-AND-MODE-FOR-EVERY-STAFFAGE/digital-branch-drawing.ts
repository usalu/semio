/** 🌿 Draw the palm fronds, Scots pine pads and willow tresses in their articulated branch frames. */
import {readFileSync,writeFileSync} from 'node:fs';
import {join} from 'node:path';
import {meshesAt,poseState,projectWorld} from './volumes-lib.ts';
import {centreLineOf,clusterPoints} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧵️centreline/🟦️.ts';
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts';
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
Bun.plugin({name:'canopy-boundaries',setup(b){b.onLoad({filter:/r3-goat-draw2d\.ts$/},args=>{
 const source=readFileSync(args.path,'utf8'),end=source.indexOf('const STROKES:');if(end<0)throw Error(args.path);
 return {contents:source.slice(0,end)+'\nexport {outline}',loader:'ts'};
});}});
const {outline}=await import(join(import.meta.dir,'r3-goat-draw2d.ts'));
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset;
const species={
 'multi-stem-birch':{regions:['leaf-crown'],material:'birch-leaf',views:{100:['elevation'],200:['elevation','front','plan'],500:['elevation','front','plan']},cell:{100:.014,200:.059,500:.104}},
 'bamboo-clump':{regions:['leaves-sun','leaves-shade'],material:'bamboo-leaf',views:{50:['elevation','front']},cell:{50:.004}},
 'weeping-willow':{regions:['leaf-curtain'],material:'willow-leaf',views:{500:['elevation','front','plan']},cell:{500:.102}},
 'scots-pine':{regions:['needles-sun','needles-shade'],material:'pine-needles',views:{100:['elevation'],200:['elevation','front','plan'],500:['elevation','front','plan']},cell:{100:.041,200:.073,500:.11}},
 'date-palm':{regions:['fronds'],material:'palm-frond',views:{500:['elevation','front']},cell:{500:.032}}
}[a.id];if(!species)throw Error(a.id);
const pose=evaluateDrawing(a,{kind:'pose',pose:a.presentation.defaultPose});
for(const [key,rep] of Object.entries(a.representations)){
 const d=Number(key),targets=species.views[d]??[],v=a.volumes[d];
 if(process.argv[4] && d!==Number(process.argv[4]))continue;
 const woody=a.id==='multi-stem-birch' && process.argv[5]==='wood';
 const leaf=(q)=>woody ? ['branch-bark','stem-bark'].includes(q.region) : species.regions.some(id=>q.region===id||q.regions?.includes(id))||q.material?.startsWith(species.material);
 const thermal=new Map(v.thermal.filter(q=>q.of).map(q=>[q.of,q.region])),groups=new Map();
 const birchLeader=(q)=>a.id==='multi-stem-birch'&&d===500&&thermal.get(q.id)==='branch-bark';
 const bod=v.body.filter(q=>!q.texture&&(woody ? ['branch-bark','stem-bark'].includes(thermal.get(q.id)) : q.material.startsWith(species.material)||birchLeader(q))),meshes=targets.length?meshesAt(a,d,bod):[];
 for(const [i,q] of bod.entries()){
  const region=thermal.get(q.id),id='spray-'+q.bone+'-'+region;
  if(!groups.has(id))groups.set(id,{id,bone:q.bone,material:q.material,region,meshes:[]});
  groups.get(id).meshes.push(meshes[i]);
 }
 const tips=a.id==='weeping-willow'?worldSolids(a,pose.bones,d).filter(q=>q.id.startsWith('switch-')):[];
 for(const view of new Set([...targets,...(tips.length?['elevation','front','plan']:[])])){
  const old=rep.geometry[view],retained=old.filter(q=>(!targets.includes(view)||!leaf(q))&&!tips.some(t=>'axis-'+t.id===q.id));
  const draws=targets.includes(view)?Array.from(groups.values()).map((g,i)=>{
   const original=old.find(q=>leaf(q)&&q.bone===g.bone&&q.region===g.region)??old.find(q=>leaf(q)&&q.region===g.region)??old.find(leaf);
   return {id:g.id,bone:g.bone,region:g.region,material:g.material,kind:'poly',points:[[0,0],[.1,0],[0,.1]],filled:true,role:'body',z:g.region==='branch-bark'?1.8:original?.z??3+i*.001,tone:g.region==='branch-bark'?.42:original?.tone??.5,penMm:original?.penMm??.2};
  }):[];
  for(const tip of tips)draws.push({id:'axis-'+tip.id,bone:tip.bone,region:'limb-bark',material:'willow-bark',kind:'poly',points:[[0,0],[.1,0],[0,.1]],filled:true,role:'body',z:2.1,tone:.42,penMm:.20});
  const probes=poseState({...a,representations:{...a.representations,[d]:{...rep,geometry:{...rep.geometry,[view]:draws}}}},d)[view];
  for(const q of draws){
   const [o,x,y]=probes.find(s=>s.shape.id===q.id).rings[0],ux=(x.x-o.x)*10,uy=(x.y-o.y)*10,vx=(y.x-o.x)*10,vy=(y.y-o.y)*10,det=ux*vy-uy*vx;
   if(Math.abs(det)<1e-8){
    for(const original of old.filter(s=>s.bone===q.bone&&leaf(s)))if(!retained.includes(original))retained.push(original);
    continue;
   }
   const local=([x,y])=>[+((vy*(x-o.x)-vx*(y-o.y))/det).toFixed(5),+((ux*(y-o.y)-uy*(x-o.x))/det).toFixed(5)].join(' ');
   let path;
   const g=groups.get(q.id);
   if(g){
    const {rings}=outline(view,g.meshes,species.cell[d],undefined,true);
    path=rings.filter(r=>r.length>2).map(r=>'M '+r.map(local).join(' L ')+' Z').join(' ');
   }else{
    const mesh=tips.find(t=>'axis-'+t.id===q.id),axis=centreLineOf(mesh.positions,mesh.positions,mesh.indices);
    if(!axis)throw Error(mesh.id);
    const pts=clusterPoints(axis,mesh.positions,0).map(p=>{const q=projectWorld(view,p);return local([q.x,q.y]);});
    path=axis.edges.map(([i,j])=>'M '+pts[i]+' L '+pts[j]).join(' ');q.filled=false;
   }
   delete q.points;if(path)retained.push({...q,kind:'path',d:path});
  }
  rep.geometry[view]=retained.sort((l,r)=>l.z-r.z);
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc));
