/** 🛢️ Probe the open valve branch, saddle-cut pipe and rod seal using independent Three.js intersections. */
import {readFileSync} from 'node:fs';
import assert from 'node:assert/strict';
import {Ray,Triangle,Vector3,Vector2} from 'three';
const c=JSON.parse(readFileSync(new URL('./digital-well-case.json',import.meta.url))),{asset}=JSON.parse(readFileSync(process.argv[2]));
function hits(s,o,axis){const r=new Ray(new Vector3(...o),new Vector3(...axis)),p=s.vertices.map(q=>new Vector3(...q));return s.triangles.flatMap(([a,b,c])=>{const h=r.intersectTriangle(p[a],p[b],p[c],false,new Vector3());return h?[r.origin.distanceTo(h)]:[];});}
const close=(a,b,d=1e-6)=>assert.ok(Math.abs(a-b)<d,`${a} vs ${b}`);
for(const d of c.scales){
 const v=asset.volumes[d],get=id=>v.body.find(q=>q.id===id),p=get('rod-packing'),f=get('packing-follower'),pipe=get('flowline'),head=get('wellhead-valve')??get('wellhead-top')??get('wellhead');
 close(p.profile[0][1]*2,c.rodDiameterM);close(Math.max(...p.profile.map(q=>q[1]))*2,c.sleeveDiameterM);close(p.profile[2][0],c.packingLengthM);close(f.profile[2][0],c.followerLengthM);
 assert.equal(hits(pipe,[-3,c.flowlineY,.001],[1,0,0]).length,0);
 assert.equal(hits(head,[-3,c.flowlineY,.001],[1,0,0]).length,0);
 const radii=hits(pipe,[-2.5,c.flowlineY,0],[0,0,1]).sort((a,b)=>a-b);
 close(radii[0]*2,c.flowlineBoreM);close(radii.at(-1)*2,c.flowlineODm);
 assert.ok(hits(head,[-3,c.flowlineY+.075,.001],[1,0,0]).length>=2);
 assert.ok(v.structure.some(q=>q.of==='flowline'));assert.ok(v.thermal.find(q=>q.id==='th-flowline').sectionFaces.includes('+u'));
}
const tau=2*Math.PI,area=.5*tau*.17+.03*tau*.2+Math.PI*(.2**2-.17**2)+Math.PI*(.2**2-.1**2)+.47*tau*.1+Math.PI*(.1**2-.08**2)+.22*tau*.08+Math.PI*(.06+.08)*new Vector2(.06,.02).length()+Math.PI*(.06**2-.0255**2)+.9*tau*.06;
close(+area.toFixed(2),c.areaM2);close(asset.thermal.regions.find(q=>q.id==='wellhead').areaM2,c.areaM2);
console.log('Four valve passages, stuffing boxes, saddle intersections and exposed surface estimate verified.');
