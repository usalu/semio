/** 🪑 Samples the chair or bench and the trouser underside with Three.js rays, in world metres and thigh-local coordinates. */
import { Ray, Vector3 } from 'three';
import {entryOf} from './volumes-lib.ts';
import {listsIn,rigAt} from './r3-lib.ts';
const point=new Vector3(),a=new Vector3(),b=new Vector3(),c=new Vector3();
for(const name of ['seated-office-worker','seated-reader']){
 const asset=entryOf('humans/'+name).asset;
 for(const pose of asset.poses.filter(p=>p.contacts.includes('thigh-support'))){
 const rig=rigAt(asset,50,{kind:'pose',pose:pose.id});
 const m=listsIn(asset,50,rig.frames);
 const anchor=rig.state.anchors['thigh-support'],frame=rig.frames['thigh-r'];
 const find=(x,z)=>{
  const ray=new Ray(new Vector3(x,3,z),new Vector3(0,-1,0));let leg=Infinity,seat=-Infinity,part='';
  for(const [i,mesh] of m.body.entries()){
    const own=mesh.id==='thigh-r';if(!own&&!/^chair-seat-cushion$|^bench-seat-slat/.test(mesh.id))continue;
    for(let j=0;j<mesh.indices.length;j+=3){
      for(const [k,v] of [a,b,c].entries())v.fromArray(mesh.world,mesh.indices[j+k]*3);
      if(!ray.intersectTriangle(a,b,c,false,point))continue;
      if(own)leg=Math.min(leg,point.y);else if(point.y>seat){seat=point.y;part=mesh.id}
    }
  }
  return {leg,seat,part}
 }
 console.log(name,pose.id,JSON.stringify(anchor),JSON.stringify(frame));
 let best={d:1};const def=anchor.anchor;
 for(let dx=-.045;dx<=.0451;dx+=.0025){const localX=def.x+dx;const x=frame.o[0]+localX*frame.u[0]+def.y*frame.v[0];const z=frame.o[2];const {leg,seat,part}=find(x,z);const d=Math.abs(leg-seat)+Math.abs(dx)*.03;if(d<best.d){const r=[x-frame.o[0],(leg+seat)/2-frame.o[1],z-frame.o[2]];best={d,leg,seat,part,x:r.reduce((v,k,i)=>v+k*frame.u[i],0),y:r.reduce((v,k,i)=>v+k*frame.v[i],0),z:r.reduce((v,k,i)=>v+k*frame.w[i],0)}}}
 console.log('    ',JSON.stringify(best))
 }
}
