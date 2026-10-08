import { readFile, writeFile, mkdir } from "node:fs/promises";
import { createHash } from "node:crypto";
import { join } from "node:path";
import assert from "node:assert/strict";

type V = [number, number, number];
const add=(a:V,b:V):V=>a.map((v,i)=>v+b[i]) as V;
const sub=(a:V,b:V):V=>a.map((v,i)=>v-b[i]) as V;
const scale=(a:V,s:number):V=>a.map(v=>v*s) as V;
const dot=(a:V,b:V)=>a.reduce((s,v,i)=>s+v*b[i],0);
const cross=(a:V,b:V):V=>[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]];
const norm=(a:V):V=>scale(a,1/Math.sqrt(dot(a,a)));
const hash=(b:string|Buffer)=>createHash("sha256").update(b).digest("hex");

/** 🎯️ Finite forward-ray and segment closest pair, with every constrained boundary considered. */
export function closestPairV1(o:V,d:V,a:V,b:V){
 const e=sub(b,a),w=sub(o,a),dd=dot(d,d),ee=dot(e,e),de=dot(d,e),dw=dot(d,w),ew=dot(e,w);
 if(![...o,...d,...a,...b].every(Number.isFinite)||dd<=0||ee<=0)throw Error("invalid finite ray/segment");
 const candidates=[[Math.max(0,-dw/dd),0],[Math.max(0,(de-dw)/dd),1],[0,Math.max(0,Math.min(1,ew/ee))]];
 const determinant=dot(cross(d,e),cross(d,e));
 if(determinant>0){const t=(de*ew-ee*dw)/determinant,u=(dd*ew-de*dw)/determinant;if(t>=0&&u>=0&&u<=1)candidates.push([t,u]);}
 return candidates.map(([t,u])=>{const ray=add(o,scale(d,t)),segment=add(a,scale(e,u)),gap=sub(ray,segment);return{t,u,ray,segment,distanceSquared:dot(gap,gap)};}).sort((a,b)=>a.distanceSquared-b.distanceSquared)[0];
}

function triangle(o:V,d:V,a:V,b:V,c:V){const e1=sub(b,a),e2=sub(c,a),p=cross(d,e2),det=dot(e1,p);if(Math.abs(det)<1e-10)return null;const v=sub(o,a),u=dot(v,p)/det;if(u<0||u>1)return null;const q=cross(v,e1),w=dot(d,q)/det;if(w<0||u+w>1)return null;const t=dot(e2,q)/det;return t>=0?t:null;}
function projection(point:V,camera:any,fov:number){const forward=norm(sub(camera.target,camera.position)),right=norm(cross(forward,camera.up)),up=cross(right,forward),v=sub(point,camera.position),z=dot(v,forward),extent=Math.tan(fov*Math.PI/360);return[400+300*dot(v,right)/(z*extent),300-300*dot(v,up)/(z*extent)];}
function screenGap(x:number,y:number,a:number[],b:number[]){const dx=b[0]-a[0],dy=b[1]-a[1],l=dx*dx+dy*dy,t=l===0?0:Math.max(0,Math.min(1,((x-a[0])*dx+(y-a[1])*dy)/l));return Math.hypot(x-a[0]-t*dx,y-a[1]-t*dy);}
function ownSchema(v:any){return v&&Object.keys(v).sort().join() === "cases,schema"&&v.schema==="ticket.ray-closest-depth/v1"&&Array.isArray(v.cases)&&v.cases.length===12&&v.cases.every((r:any)=>r&&Object.keys(r).sort().join()==="a,b,direction,id,origin"&&typeof r.id==="string"&&r.id.length>0&&[r.a,r.b,r.direction,r.origin].every(a=>Array.isArray(a)&&a.length===3&&a.every((n:any)=>typeof n==="number"&&Number.isFinite(n))));}

async function main(){
 assert.equal(process.argv[2],"oracle");const base=import.meta.dir,read=async(p:string)=>readFile(p,"utf8"),inputs=JSON.parse(await read(join(base,"inputs.json"))),gui=JSON.parse(await read(join(base,"gui.json")))[0];
 const JSON5=(await import(join(inputs.root,"node_modules/json5/lib/index.js"))).default;
 for(const path of [join(inputs.root,".vscode/launch.json"),join(inputs.root,".vscode/🧩️launch.seed.jsonc")])assert.deepEqual(JSON5.parse(await read(path)).configurations.filter((r:any)=>r.name===gui.name),[gui]);
 for(const [key,value]of Object.entries(gui.env))assert.equal(process.env[key],String(value).replaceAll("${workspaceFolder}",inputs.root));
 const bindings=[...inputs.bindings];for(const f of ["📜️script.ts","inputs.json","fixtures.json","schema.json","gui.json","project.json","nx.json","package.json"]){const path=join(base,f);bindings.push({path,sha256:hash(await readFile(path))});}
 const guard=async()=>{for(const row of bindings)assert.equal(hash(await readFile(row.path)),row.sha256,`authority ${row.path}`);};await guard();
 const out=join(inputs.ticket,"🗑️generated/current-native-ray-depth-1/oracle");await mkdir(out,{recursive:true});await writeFile(join(out,"started.json"),JSON.stringify({at:new Date().toISOString(),bindings,sourceWritesOutsideTicket:false,nativeExecuted:false},null,2));
 const Three=await import(join(inputs.root,"node_modules/three/build/three.module.js")),Ajv=(await import(join(inputs.root,"node_modules/ajv/dist/ajv.js"))).default;
 const fixture=JSON.parse(await read(join(base,"fixtures.json"))),schema=JSON.parse(await read(join(base,"schema.json"))),validate=new Ajv({strict:true}).compile(schema),schemaCases=[fixture,{...fixture,extra:true},{...fixture,cases:[]},{...fixture,cases:fixture.cases.map((r:any,i:number)=>i===0?{...r,a:[0,0]}:r)},{...fixture,schema:"other"}];
 const schemaOut=schemaCases.map((v,i)=>{const own=Boolean(ownSchema(v)),oracle=Boolean(validate(v));assert.equal(own,oracle);assert.equal(own,i===0);return{id:i,own,oracle};});
 const laws=fixture.cases.map((row:any)=>{const d=norm(row.direction),own=closestPairV1(row.origin,d,row.a,row.b),ray=new Three.Ray(new Three.Vector3(...row.origin),new Three.Vector3(...d)),r=new Three.Vector3(),s=new Three.Vector3(),sq=ray.distanceSqToSegment(new Three.Vector3(...row.a),new Three.Vector3(...row.b),r,s);assert.ok(Math.abs(own.distanceSquared-sq)<1e-7,`${row.id} own/Three squared gap`);if(Math.abs(dot(d,norm(sub(row.b,row.a))))<.999999)assert.ok(Math.abs(own.t-r.distanceTo(ray.origin))<1e-7,`${row.id} own/Three ray depth`);return{id:row.id,own,three:{ray:r.toArray(),segment:s.toArray(),distanceSquared:sq,depth:r.distanceTo(ray.origin)},midpointDepth:dot(sub(scale(add(row.a,row.b),.5),row.origin),d)};});
 const scene=JSON.parse(await read(inputs.fixture)).world3d,scenes=[];
 for(const fov of [scene.cameraJson.fov,50]){
  const cam=scene.cameraJson,camera=new Three.PerspectiveCamera(fov,800/600,.2,524288);camera.position.fromArray(cam.position);camera.up.fromArray(cam.up);camera.lookAt(...cam.target);camera.updateMatrixWorld();const aimed:V=[0,0,0],screen=projection(aimed,cam,fov),ndc=new Three.Vector3(...aimed).project(camera),caster=new Three.Raycaster();caster.params.Line.threshold=.12;caster.setFromCamera(new Three.Vector2(ndc.x,ndc.y),camera);const o=cam.position as V,d=norm(sub(aimed,o)),objects=[],current=[],corrected=[];
  for(const record of scene.instancesJson){assert.deepEqual(record.position,[0,0,0]);assert.deepEqual(record.scale,[1,1,1]);assert.deepEqual(record.rotation,[0,0,0,1]);const data=scene.meshesJson.find((r:any)=>r.id===record.meshId).data,indices=data.indices??[],positions=data.positions??[];let actual:any=null;
   if(indices.length){const geometry=new Three.BufferGeometry().setAttribute("position",new Three.Float32BufferAttribute(positions,3));geometry.setIndex(indices);actual=new Three.Mesh(geometry,new Three.MeshBasicMaterial({side:Three.DoubleSide}));let distance=Infinity;for(let i=0;i<indices.length;i+=3){const p=(index:number):V=>positions.slice(index*3,index*3+3);const t=triangle(o,d,p(indices[i]),p(indices[i+1]),p(indices[i+2]));if(t!==null)distance=Math.min(distance,t);}if(Number.isFinite(distance)){current.push({id:record.interactionId,depth:distance});corrected.push({id:record.interactionId,depth:distance});}}
   else if(data.edgePositions?.length){const edges=data.edgePositions,geometry=new Three.BufferGeometry().setAttribute("position",new Three.Float32BufferAttribute(edges,3));actual=new Three.LineSegments(geometry,new Three.LineBasicMaterial());for(let i=0;i<edges.length;i+=6){const a=edges.slice(i,i+3) as V,b=edges.slice(i+3,i+6) as V,gap=screenGap(screen[0],screen[1],projection(a,cam,fov),projection(b,cam,fov));if(gap<=18){const own=closestPairV1(o,d,a,b);current.push({id:record.interactionId,depth:dot(sub(scale(add(a,b),.5),o),d),screenGap:gap});corrected.push({id:record.interactionId,depth:own.t,screenGap:gap,distanceSquared:own.distanceSquared});}}}
   if(actual){actual.name=record.interactionId;actual.updateMatrixWorld();objects.push(actual);}
  }
  const hits=caster.intersectObjects(objects,false).map((h:any)=>({id:h.object.name,distance:h.distance,point:h.point.toArray()}));current.sort((a,b)=>a.depth-b.depth);corrected.sort((a,b)=>a.depth-b.depth);assert.equal(current[0].id,"extrusion-axis@vector");assert.equal(corrected[0].id,"extrude@solid");assert.equal(hits[0].id,"extrude@solid");scenes.push({camera:cam,fov,screen,current,corrected,three:hits,scope:"fixture pose only; attached current native state not observed"});for(const object of objects){object.geometry.dispose();object.material.dispose();}
 }
 await guard();const result={at:new Date().toISOString(),ready:true,bindings,laws,schemaOut,scenes,nativeExecuted:false,sourceWritesOutsideTicket:false,publicationReady:false};await writeFile(join(out,"result.json"),JSON.stringify(result,null,2));console.log(`[DEBUG] rayDepth laws=${laws.length} schema=${schemaOut.length} scenes=${scenes.length} midpointAxis=true closestSolid=true threeSolid=true nativeExecuted=false`);
}
if(import.meta.main)await main();
