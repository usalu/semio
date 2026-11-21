/** 🌿 Attach the distributed transpiration patches to the exposed skin of the hanging foliage. */
import {readFileSync,writeFileSync} from 'node:fs';
import {Vector3,Triangle} from 'three';
const doc=JSON.parse(readFileSync(process.argv[2],'utf8'));
for(const [scale,v] of Object.entries(doc.asset.volumes)){
 for(const outlet of v.thermal.filter(q=>q.tags?.includes('emitter')&&q.bone.startsWith('curt-'))){
  const origin=new Vector3(...outlet.transform.translation);let closest=Infinity,patch;
  for(const sprig of v.body.filter(q=>q.bone===outlet.bone&&q.id.includes('-spray-'))){
   const pts=sprig.vertices.map(p=>new Vector3(...p)),n=sprig.triangles[0][2],count=(pts.length-2)/n;
   for(let row=1;row<count-2;row++)for(let j=0;j<n;j++){
    const at=(r,s)=>r*n+(s%n),ids=[at(row,j),at(row,j+1),at(row+1,j+1),at(row+1,j)];
    const tri=new Triangle(pts[ids[0]],pts[ids[1]],pts[ids[2]]),mid=tri.getMidpoint(new Vector3());
    const dist=mid.distanceToSquared(origin);
    if(dist>=closest)continue;
    const center=(i)=>pts.slice(i*n,(i+1)*n).reduce((a,b)=>a.add(b),new Vector3()).multiplyScalar(1/n);
    const skin=ids.map(i=>pts[i]),inner=ids.map(i=>pts[i].clone().lerp(center(Math.floor(i/n)),({50:.12,100:.18,200:.42,500:.78})[scale]));
    const tris=[[0,1,2],[0,2,3],[4,6,5],[4,7,6]];
    for(let e=0;e<4;e++){const f=(e+1)%4;tris.push([e,e+4,f+4],[e,f+4,f]);}
    const outward=mid.clone().sub(center(row).lerp(center(row+1),.5));
    if(tri.getNormal(new Vector3()).dot(outward)<0)for(const t of tris)t.reverse();
    closest=dist;patch={vertices:[...skin,...inner].map(p=>p.toArray()),triangles:tris};
   }
  }
  if(!patch)throw Error(outlet.id);
  delete outlet.transform;Object.assign(outlet,patch);
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc));
