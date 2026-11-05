import {Quaternion,Vector3} from "three";
import {readFileSync,writeFileSync} from "node:fs";
const root=process.cwd(),ticket=root+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/GET-PUZZLE-ARTIFACTS-WORKING-END-TO-END";
const lines=readFileSync(ticket+"/🗑️generated/native3d.log","utf8").split("\n");
const records=[];for(let i=0;i<lines.length;i++)if(lines[i].startsWith("committed=")&&lines[i+1]?.startsWith("actual="))records.push({path:lines[i-1],actual:JSON.parse(JSON.parse(lines[i+1].slice(7))),committed:JSON.parse(JSON.parse(lines[i].slice(10)))});
const scalar={
 add:(a,b)=>a.map((x,i)=>x+b[i]),sub:(a,b)=>a.map((x,i)=>x-b[i]),scale:(a,s)=>a.map(x=>x*s),
 dot:(a,b)=>a[0]*b[0]+a[1]*b[1]+a[2]*b[2],
 cross:(a,b)=>[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]],
 mul:(a,b)=>[a[3]*b[0]+a[0]*b[3]+a[1]*b[2]-a[2]*b[1],a[3]*b[1]-a[0]*b[2]+a[1]*b[3]+a[2]*b[0],a[3]*b[2]+a[0]*b[1]-a[1]*b[0]+a[2]*b[3],a[3]*b[3]-a[0]*b[0]-a[1]*b[1]-a[2]*b[2]],
 conj:q=>[-q[0],-q[1],-q[2],q[3]],
 axis:(a,t)=>{const n=Math.sqrt(a[0]*a[0]+a[1]*a[1]+a[2]*a[2]);return n<1e-8?[0,0,0,1]:[a[0]/n*Math.sin(t*.5),a[1]/n*Math.sin(t*.5),a[2]/n*Math.sin(t*.5),Math.cos(t*.5)]},
 norm:q=>{const n=Math.sqrt(q[0]*q[0]+q[1]*q[1]+q[2]*q[2]+q[3]*q[3]);return n<1e-12?[0,0,0,1]:q.map(x=>x/n)},
 rotate:(q,v)=>{const[x,y,z,w]=q,[vx,vy,vz]=v,ix=w*vx+y*vz-z*vy,iy=w*vy+z*vx-x*vz,iz=w*vz+x*vy-y*vx,iw=-x*vx-y*vy-z*vz;return[ix*w+iw*-x+iy*-z-iz*-y,iy*w+iw*-y+iz*-x-ix*-z,iz*w+iw*-z+ix*-y-iy*-x]}
};
const V=a=>new Vector3(...a),Q=a=>new Quaternion(...a);
const third={...scalar,add:(a,b)=>V(a).add(V(b)).toArray(),sub:(a,b)=>V(a).sub(V(b)).toArray(),scale:(a,s)=>V(a).multiplyScalar(s).toArray(),dot:(a,b)=>V(a).dot(V(b)),cross:(a,b)=>V(a).cross(V(b)).toArray(),mul:(a,b)=>Q(a).multiply(Q(b)).toArray(),conj:q=>Q(q).conjugate().toArray(),axis:(a,t)=>new Quaternion().setFromAxisAngle(V(a).normalize(),t).toArray(),norm:q=>Q(q).normalize().toArray(),rotate:(q,v)=>V(v).applyQuaternion(Q(q)).toArray()};
function kernel(k){
 const vn=a=>{const l=Math.sqrt(k.dot(a,a));return l<1e-12?a:k.scale(a,1/l)},un=(a,b)=>{const r=k.dot(a,b)+1;let q;if(r<.000001)q=Math.abs(a[0])>Math.abs(a[2])?[-a[1],a[0],0,0]:[0,-a[2],a[1],0];else q=[...k.cross(a,b),r];return k.norm(q)};
 const align=(a,b)=>{const cross=k.cross(a,k.scale(b,-1));if(Math.sqrt(k.dot(cross,cross))<.01){if(Math.abs(a[2])<.01)return un([0,1,0],[0,0,-1]);let axis=k.cross([0,0,1],a);if(Math.sqrt(k.dot(axis,axis))<1e-9)axis=k.cross([1,0,0],a);axis=vn(axis);return k.norm([...k.scale(axis,Math.sin(Math.PI/2)),Math.cos(Math.PI/2)])}return un(k.scale(b,-1),a)};
 const basis=(da,db)=>{const a=vn(da),b=vn(db),aq=align(a,b),pq=un([0,1,0],a);return{a,aq,pq,g:k.rotate(pq,[0,1,0]),s:k.rotate(pq,[1,0,0]),r:k.rotate(pq,[0,0,1])}};
 const child=(ta,qa,pa,da,pb,db,c)=>{const b=basis(da,db),rq=k.axis(b.a,-c.rotation*Math.PI/180),tq=k.axis(k.rotate(rq,b.r),c.turn*Math.PI/180),iq=k.axis(k.rotate(rq,b.s),c.tilt*Math.PI/180);let q=k.conj(b.aq);for(const x of[rq,tq,iq])q=k.mul(q,k.conj(x));q=k.norm(q);const off=k.add(k.add(ta,pa),k.add(k.add(k.scale(b.g,c.gap),k.scale(b.s,c.shift)),k.scale(b.r,c.rise)));return{origin:k.sub(k.rotate(q,off),pb),orientation:k.norm(k.mul(q,qa))}};
 const derive=(ta,qa,pa,da,pb,db,tb,qb)=>{const b=basis(da,db),local=k.norm(k.mul(qb,k.conj(qa))),off=k.rotate(k.conj(local),k.add(tb,pb)),d=k.sub(k.sub(off,ta),pa),res=k.mul(b.aq,local),m=k.mul(k.mul(k.conj(b.pq),res),b.pq),x=k.rotate(m,[1,0,0]),y=k.rotate(m,[0,1,0]),z=k.rotate(m,[0,0,1]),tilt=-Math.asin(Math.max(-1,Math.min(1,y[2]))),singular=Math.abs(Math.abs(y[2])-1)<1e-6;return{gap:k.dot(d,b.g),shift:k.dot(d,b.s),rise:k.dot(d,b.r),rotation:(singular?Math.atan2(x[1],x[0]):Math.atan2(-x[2],z[2]))*180/Math.PI,turn:(singular?0:Math.atan2(y[0],y[1]))*180/Math.PI,tilt:tilt*180/Math.PI}};
 return{child,derive};
}
function solve(before,m,k){
 const result=structuredClone(before),poses=new Map(),queue=[],ports=new Map(),placing=new Set(),math=kernel(k),idq=[0,0,0,1];
 before.objects.forEach((o,i)=>o.vortices.forEach(v=>ports.set(v.id.includes(":")?v.id:o.id+":"+v.id,[i,v])));
 const ends=before.attractions.map(c=>[ports.get(c.attracting),ports.get(c.attracted)]);
 for(const id of m.targets){const i=before.objects.findIndex(o=>o.id===id&&!o.locked);if(i<0||poses.has(i))continue;const o=before.objects[i];poses.set(i,{origin:m.offset?k.add(o.origin,m.offset):o.origin,orientation:m.offset?o.orientation:k.mul(k.axis(m.axis,m.angle),o.orientation??idq)});queue.push(i)}
 while(queue.length){const parent=queue.shift();ends.forEach(([from,to],ix)=>{if(!from||!to||from[0]!==parent||poses.has(to[0])||before.objects[to[0]].locked)return;const p=poses.get(parent),c=before.attractions[ix],pose=math.child(p.origin,p.orientation??idq,from[1].position,from[1].direction??[0,0,-1],to[1].position,to[1].direction??[0,0,-1],c);poses.set(to[0],pose);placing.add(ix);queue.push(to[0])})}
 for(const[i,p]of poses){result.objects[i].origin=p.origin;if(p.orientation)result.objects[i].orientation=p.orientation;else delete result.objects[i].orientation}
 ends.forEach(([from,to],ix)=>{if(!from||!to||placing.has(ix)||!poses.has(from[0])&&!poses.has(to[0]))return;const a=result.objects[from[0]],b=result.objects[to[0]],d=math.derive(a.origin,a.orientation??idq,from[1].position,from[1].direction??[0,0,-1],to[1].position,to[1].direction??[0,0,-1],b.origin,b.orientation??idq);Object.assign(result.attractions[ix],d)});
 return result;
}
function differences(a,b,p=""){if(a&&b&&typeof a==="object"&&typeof b==="object"){const keys=new Set([...Object.keys(a),...Object.keys(b)]);return[...keys].flatMap(key=>differences(a[key],b[key],p+"/"+key))}return a===b?[]:[{path:p,reference:a,native:b,delta:typeof a==="number"&&typeof b==="number"?Math.abs(a-b):null}]}
const out=[];for(const r of records.filter(r=>r.path.includes("/📸️snapshot/"))){const scenario=r.path.slice(0,r.path.indexOf("/📸️snapshot/")),before=JSON.parse(readFileSync(scenario+"/📸️snapshot/⬅️before/🔣️.json","utf8")),mutation=JSON.parse(readFileSync(scenario+"/🦠️mutation/🔣️.json","utf8")),s=solve(before,mutation,scalar),t=solve(before,mutation,third);out.push({scenario,scalarNative:differences(s,r.actual),threeNative:differences(t,r.actual),threeCommitted:differences(t,r.committed)})}
writeFileSync(ticket+"/🗑️generated/fixture-math-reference.json",JSON.stringify(out,null,2));for(const r of out)process.stdout.write(JSON.stringify({scenario:r.scenario.split("/").at(-1),scalarNative:r.scalarNative,threeNative:r.threeNative})+"\n");
