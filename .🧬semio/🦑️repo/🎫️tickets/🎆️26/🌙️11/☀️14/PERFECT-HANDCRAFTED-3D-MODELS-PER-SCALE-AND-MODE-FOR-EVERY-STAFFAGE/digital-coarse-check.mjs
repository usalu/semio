import {readFileSync} from 'node:fs';
import {strict as assert} from 'node:assert';
import {Box2,Vector2} from 'three';
const tests=JSON.parse(readFileSync(new URL('./digital-coarse-case.json',import.meta.url),'utf8'));
const near=(p,q)=>assert(Math.abs(p-q)<1e-7,`${p} versus ${q}`);
for(const file of process.argv.slice(2)){
 const {asset:a}=JSON.parse(readFileSync(file,'utf8')),f=tests[a.id];
 if(a.id==='gas-meter'){
  const solids=a.volumes[200].body,casework=solids.find(s=>s.id==='housing'),seam=solids.find(s=>s.id==='seam');
  const outline=new Box2().setFromPoints(casework.sections.flatMap(s=>[-1,1].map(k=>new Vector2(k*s.rx,s.at[1]))));
  near(outline.min.y,f.lowerM);near(outline.max.y,f.upperM);near(Math.max(...seam.sections.map(s=>s.rx)),f.seamHalfWidthM);
 }else if(a.id==='waste-bin'){
  for(const d of [50,100,200,500]){
   const solids=a.volumes[d].body,feet=solids.filter(x=>x.id.startsWith('foot-'));assert.equal(feet.length,4);
   for(const foot of feet)near(foot.to-foot.from,f.footM);
   const floor=solids.find(q=>q.id==='floor'),shoulder=solids.find(q=>q.id==='pedestal-shoulder');
   near(shoulder.to,f.pedestalTopM);near(floor.to-floor.from,f.floorM);assert.equal(shoulder.holes.length,1);
   const bounds=new Box2().setFromPoints(floor.profile.map(([z,x])=>new Vector2(z,x)));
   near(bounds.getSize(new Vector2()).x,.396);near(bounds.getCenter(new Vector2()).y,.2);
  }
 }else if(a.id==='vacuum-cleaner'){
  for(const d of [50,100,200,500]){
   const b=a.volumes[d].body,hoses=b.filter(q=>/^hose-\d+-(tube|bend-[ab])$/.test(q.id));assert(hoses.length>=10);
   for(const h of hoses){near(2*h.r0,f.hoseBoreEnvelopeM);near(2*h.r1,f.hoseBoreEnvelopeM)}
   const tube=b.find(q=>q.id==='wand-tube');assert.deepEqual(tube.profile[0],tube.profile.at(-1));near(Math.min(...tube.profile.map(p=>p[1])),.0165);
   const head=b.filter(q=>q.id==='nozzle'||q.id.startsWith('nozzle-cap-'));
   near(Math.max(...head.map(s=>s.to))-Math.min(...head.map(s=>s.from)),f.headWidthM);
   for(const name of ['brush','brush-rear'])near(b.find(s=>s.id===name).size[1],f.brushHeightM);
  }
 }
 console.log(a.id,'verified against independent working dimensions');
}
