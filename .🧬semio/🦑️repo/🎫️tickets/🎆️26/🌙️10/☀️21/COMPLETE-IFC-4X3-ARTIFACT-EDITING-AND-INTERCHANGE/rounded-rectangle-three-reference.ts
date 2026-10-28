/** ▢ Independent Three sampled boundary and transformed prism observations. */
import {readFile,writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {EllipseCurve,ShapeUtils,Vector2,Vector3,Matrix4} from 'three';
const root=import.meta.dir,input=await readFile(resolve(root,'rounded-rectangle-profile-cases.json'),'utf8'),data=JSON.parse(input),output=[];
const cardinal=(i:number,n:number)=>i===0?[1,0]:i===n/4?[0,1]:i===n/2?[-1,0]:i===3*n/4?[0,-1]:i===n?[1,0]:[Math.cos(2*Math.PI*i/n),Math.sin(2*Math.PI*i/n)];
for(const row of data.cases){
 const result:any={id:row.id,declaredGeometry:row.geometry};output.push(result);
 if(row.formal!=='valid'){result.status='not-meshed-invalid-source-contract';continue;}
 const [x,y]=row.dimensions,r=row.radius,unit=row.unitPrefix==='MILLI'?.001:1;
 const profile=new Matrix4().makeRotationZ(row.profile.rotation),solid=new Matrix4().makeRotationX(row.solid.tilt),mapping=new Matrix4().makeScale(...row.mappingScale),linear=mapping.clone().multiply(solid).multiply(profile);
 const e=linear.elements,u=[e[0],e[1],e[2]],v=[e[4],e[5],e[6]],aa=u.reduce((s,z)=>s+z*z,0),bb=v.reduce((s,z)=>s+z*z,0),ab=u.reduce((s,z,i)=>s+z*v[i],0),sigma=Math.sqrt((aa+bb+Math.hypot(aa-bb,2*ab))/2),worldRadius=r*unit*sigma;
 result.planeColumns=[u,v];result.worldRadius=worldRadius;
 const tolerance=Math.min(1,Math.max(row.sampling.relativeChordError,row.sampling.absoluteChordErrorMetres/worldRadius));let n=4;
 while(.5*(3.1415926535897936/n)**2*(1+8*Number.EPSILON)>tolerance)n+=4;
 const a=x/2-r,b=y/2-r,centres=[[a,b],[-a,b],[-a,-b],[a,-b]],ring:number[][]=[];
 for(let corner=0;corner<4;corner++)for(let j=0;j<=n/4;j++){const curve=new EllipseCurve(centres[corner][0],centres[corner][1],r,r,corner*Math.PI/2,(corner+1)*Math.PI/2,false,0),d=cardinal(corner*n/4+j,n),sample=curve.getPoint(j/(n/4)),point=j===0||j===n/4?[centres[corner][0]+r*d[0],centres[corner][1]+r*d[1]]:[sample.x,sample.y];if(!ring.length||point.some((value,i)=>value!==ring.at(-1)![i]))ring.push(point);}
 if(ring.length>1&&ring[0].every((value,i)=>value===ring.at(-1)![i]))ring.pop();
 const M=ring.length;Object.assign(result,{curvedSegments:n,boundaryEdges:M,logicalValues:18*M-12,points:2*M,triangles:4*M-4});
 if(M>row.sampling.maxSegments){result.status='total-boundary-resource-refusal';continue;}
 const contour=ring.map(p=>new Vector2(...p)),triangles=ShapeUtils.triangulateShape(contour,[]),area=Math.abs(ShapeUtils.area(contour)),direction=new Vector3(...row.direction).normalize().multiplyScalar(row.depth),positions:Vector3[]=[];
 for(let upper=0;upper<2;upper++)for(const p of ring){const q=new Vector3(p[0],p[1],0).applyMatrix4(profile);q.x+=row.profile.translation[0];q.y+=row.profile.translation[1];if(upper)q.add(direction);q.applyMatrix4(solid);q.add(new Vector3(...row.solid.translation));q.applyMatrix4(mapping).multiplyScalar(unit);positions.push(q);}
 const indices:number[]=[];for(const t of triangles)indices.push(t[2],t[1],t[0],t[0]+M,t[1]+M,t[2]+M);for(let i=0;i<M;i++){const j=(i+1)%M;indices.push(i,j,j+M,i,j+M,i+M);}
 if(direction.z*linear.determinant()<0)for(let i=0;i<indices.length;i+=3)[indices[i+1],indices[i+2]]=[indices[i+2],indices[i+1]];
 const edges=new Map<string,number[]>();let volume=0;for(let i=0;i<indices.length;i+=3){const ids=indices.slice(i,i+3),p=ids.map(id=>positions[id]);volume+=p[0].dot(new Vector3().crossVectors(p[1],p[2]))/6;for(let j=0;j<3;j++){const a=ids[j],b=ids[(j+1)%3],key=Math.min(a,b)+':'+Math.max(a,b);const entry=edges.get(key)??[];entry.push(a<b?1:-1);edges.set(key,entry);}}
 const distinct=new Set(positions.map(p=>p.toArray().join(','))).size;
 Object.assign(result,{status:distinct===positions.length?'sampled-reference':'binary64-coordinate-collapse',area,volume,distinctVertices:distinct,closedOppositeEdges:[...edges.values()].every(v=>v.length===2&&v[0]+v[1]===0),bounds:[['min',Infinity],['max',-Infinity]].map(([op,start])=>[0,1,2].map(axis=>positions.reduce((a,p)=>op==='min'?Math.min(a,p.getComponent(axis)):Math.max(a,p.getComponent(axis)),start as number))),ring,capTriangles:triangles.length});
}
await writeFile(resolve(root,'🗑️generated/rounded-rectangle-three-reference.json'),JSON.stringify({external:'three',producerExecuted:false,rows:output},null,2)+'\n');console.log('[DEBUG]',output.map(r=>[r.id,r.status,r.boundaryEdges,r.volume]));

