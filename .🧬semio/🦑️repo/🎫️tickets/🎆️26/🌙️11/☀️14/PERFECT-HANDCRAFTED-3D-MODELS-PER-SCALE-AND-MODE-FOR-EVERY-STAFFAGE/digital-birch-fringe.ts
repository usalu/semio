/** 🌿 Clip the original birch spray strokes to their branch’s posed foliage in the elevation. */
import {readFileSync,writeFileSync} from 'node:fs';
import {poseState} from './volumes-lib.ts';
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,k=100,view='elevation',rep=a.representations[k];
const drawings=rep.geometry[view],state=poseState(a,k)[view],crown=state.filter(q=>q.shape.region==='leaf-crown'&&q.shape.filled!==false);
const arcs=drawings.filter(q=>q.id.endsWith('-arcs')&&q.role==='detail');
const probes=poseState({...a,representations:{...a.representations,[k]:{...rep,geometry:{...rep.geometry,[view]:arcs.map(q=>({...q,kind:'poly',points:[[0,0],[.1,0],[0,.1]],filled:true}))}}}},k)[view];
const cross=(x,y,u,v)=>x*v-y*u;
function retained(p,q,groups){
 const dx=q.x-p.x,dy=q.y-p.y,edges=groups.flatMap(g=>g.flatMap(r=>r.map((v,i)=>[v,r[(i+1)%r.length]])));
 const hits=[0,1];
 for(const [a,b] of edges){
  const u=b.x-a.x,v=b.y-a.y,den=cross(dx,dy,u,v);if(Math.abs(den)<1e-12)continue;
  const t=cross(a.x-p.x,a.y-p.y,u,v)/den,h=cross(a.x-p.x,a.y-p.y,dx,dy)/den;
  if(t>0&&t<1&&h>=0&&h<=1)hits.push(t);
 }
 const inside=(x,y,group)=>group.reduce((hit,r)=>r.reduce((v,a,i)=>{const b=r[(i+1)%r.length];return (a.y>y)!==(b.y>y)&&x<(b.x-a.x)*(y-a.y)/(b.y-a.y)+a.x?!v:v;},hit),false);
 hits.sort((a,b)=>a-b);return hits.slice(1).flatMap((hi,i)=>{const lo=hits[i],m=(lo+hi)/2;return hi-lo>1e-8&&groups.some(g=>inside(p.x+dx*m,p.y+dy*m,g))?[[[p.x+dx*lo,p.y+dy*lo],[p.x+dx*hi,p.y+dy*hi]]]:[];});
}
for(const mark of arcs){
 const groups=crown.filter(c=>c.shape.bone===mark.bone).map(c=>c.rings);
 const [o,x,y]=probes.find(p=>p.shape.id===mark.id).rings[0],ux=(x.x-o.x)*10,uy=(x.y-o.y)*10,vx=(y.x-o.x)*10,vy=(y.y-o.y)*10,det=ux*vy-uy*vx;
 if(Math.abs(det)<1e-8)continue;
 const local=([x,y])=>[+((vy*(x-o.x)-vx*(y-o.y))/det).toFixed(5),+((ux*(y-o.y)-uy*(x-o.x))/det).toFixed(5)].join(' ');
 const segments=state.find(p=>p.shape.id===mark.id).rings.flatMap(r=>r.slice(1).flatMap((q,i)=>retained(r[i],q,groups)));
 if(!segments.length){drawings.splice(drawings.indexOf(mark),1);continue;}
 const paths=[];let last;
 for(const [from,to] of segments){const s=local(from),e=local(to);if(s===e)continue;if(s!==last)paths.push('M '+s);paths.push('L '+e);last=e;}
 mark.d=paths.join(' ');mark.region='leaf-crown';mark.material='birch-leaf';
}
writeFileSync(process.argv[3],JSON.stringify(doc));console.log(arcs.length,'native foliage stroke groups checked');
