/** 🩼 Measure the paired adjustable crutch tubes and all transverse bores with independent ray casting. */
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {Ray,Triangle,Vector3} from 'three';
const {asset}=JSON.parse(readFileSync(process.argv[2]));
const f=JSON.parse(readFileSync(new URL('./digital-crutch-fitting.json',import.meta.url)));
const near=(v,ref,tol=.00003)=>assert.ok(Math.abs(v-ref)<tol,`${v} / ${ref}`);
function probe(part,origin,direction){
 assert.equal(part.kind,'polyhedron',part.id);
 const pts=part.vertices.map(v=>new Vector3(...v)),ray=new Ray(origin,direction),hits=[];
 for(const [a,b,c] of part.triangles){
  const point=ray.intersectTriangle(pts[a],pts[b],pts[c],false,new Vector3());
  if(point)hits.push(point.distanceTo(origin));
 }
 return [...new Set(hits.map(h=>+h.toFixed(7)))].sort((a,b)=>a-b);
}
for(const scale of f.scales)for(const [suffix,side] of [['l',1],['r',-1]]){
 const v=asset.volumes[scale],piece=id=>v.body.find(q=>q.id===id+'-'+suffix);
 const upper=piece('crutch-tube'),lower=piece('crutch-lower'),pin=piece('crutch-detent');
 const axis=new Vector3(.99631,0,side*.08579).normalize(),up=new Vector3(0,1,0);
 const radial=(part,s)=>probe(part,axis.clone().multiplyScalar(s),up);
 for(const part of [upper,lower]){
  const edges=new Map();
  for(const tri of part.triangles)for(let i=0;i<3;i++){
   const a=tri[i],b=tri[(i+1)%3],k=Math.min(a,b)+'/'+Math.max(a,b);
   const e=edges.get(k)??[0,0];e[0]++;e[1]+=a<b?1:-1;edges.set(k,e);
  }
  assert.ok([...edges.values()].every(([n,d])=>n===2&&d===0),'Tube must be an oriented closed surface');
 }
 const range=part=>{const s=part.vertices.map(p=>new Vector3(...p).dot(axis));return [Math.min(...s),Math.max(...s)];};
 for(const [part,s,inner,outer] of [[upper,.33,f.outerBoreM,f.outerDiameterM],[lower,.65,f.innerBoreM,f.innerDiameterM]]){
  const hits=radial(part,s);assert.equal(hits.length,2);near(hits[0]*2,inner);near(hits[1]*2,outer);
 }
 near(range(upper)[1]-range(lower)[0],f.overlapM);
 for(const s of [f.lockStationM,...f.adjustmentStationsM])for(const offset of [-.003,0,.003]){
  const o=axis.clone().multiplyScalar(s+offset).addScaledVector(up,-.04);
  assert.equal(probe(lower,o,up).length,0);
  if(s===f.lockStationM)assert.equal(probe(upper,o,up).length,0);
 }
 for(const s of f.adjustmentStationsM)assert.equal(radial(lower,s+f.holeDiameterM/2+.0002).length,2);
 assert.ok(pin);near(Math.max(...pin.profile.map(p=>p[1]))*2,f.pinDiameterM);
 for(const s of [lower,upper,pin]){assert.ok(v.thermal.some(q=>q.of===s.id));assert.ok(v.structure.some(q=>q.of===s.id));}
}
const frame=asset.thermal.regions.find(r=>r.id==='crutch-frame'),roomArea=.16;
const innerSurface=2*Math.PI*(f.outerBoreM*(.32+.012)+f.innerBoreM*(.856-.32));
near(frame.areaM2,roomArea+innerSurface);
for(const pose of asset.poses)near(frame.areaM2*pose.thermal.regions['crutch-frame'].exposedFraction,roomArea);
console.log('Both telescoping assemblies passed all four scales, through-bores, detents and stock dimensions.');
