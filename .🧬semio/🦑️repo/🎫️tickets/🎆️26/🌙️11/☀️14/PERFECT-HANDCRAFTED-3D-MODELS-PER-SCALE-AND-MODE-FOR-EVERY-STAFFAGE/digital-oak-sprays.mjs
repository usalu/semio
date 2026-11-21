/** 🌿 Hand-place eight current-year oak shoots with alternate, cupped, round-lobed leaves. */
import {readFileSync,writeFileSync} from 'node:fs';
import {Vector3,Quaternion} from 'three';
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),v=doc.asset.volumes[50],sites=JSON.parse(readFileSync(new URL('./digital-oak-sprays.json',import.meta.url)));
const A=(...q)=>new Vector3(...q),round=p=>p.toArray().map(x=>+x.toFixed(7)),rad=a=>a*Math.PI/180;
const contour=[[0,.03],[.035,.26],[.075,.50],[.12,.52],[.17,.30],[.22,.36],[.25,.65],[.30,.78],[.34,.75],[.39,.45],[.445,.55],[.49,.91],[.55,1],[.60,.87],[.64,.50],[.69,.55],[.73,.84],[.785,.87],[.82,.70],[.85,.43],[.9,.48],[.94,.52],[.97,.38],[1,.025]];
const nodes=[.14,.27,.43,.56,.71,.86,.98],tilts=[34,-22,28,-32,23,-18,13],widths=[.89,1,.95,.96,.84,.78,.69];
for(const [part,az,el,reach,length,width] of sites){
 const parent=v.body.find(s=>s.id===part),bone=parent.bone,q=new Quaternion(...parent.transform.rotation);
 const axis=A(Math.cos(rad(az))*Math.cos(rad(el)),Math.sin(rad(el)),Math.sin(rad(az))*Math.cos(rad(el))).normalize();
 const side=A(0,1,0).cross(axis).normalize(),up=axis.clone().cross(side).normalize(),offset=A(...parent.transform.translation);
 const verts=parent.vertices.map(x=>A(...x).applyQuaternion(q).add(offset));
 const anchor=verts.reduce((best,p)=>p.dot(axis)>best.dot(axis)?p:best).clone().addScaledVector(axis,-.031);
 const station=u=>anchor.clone().addScaledVector(axis,u*reach).addScaledVector(up,.024*u*u).addScaledVector(side,.014*Math.sin(u*Math.PI));
 const prefix=part.replace('fol-','shoot-'),region=v.thermal.find(r=>r.of===part).region;
 function add(s,field){v.body.push(s);v.thermal.push({id:'th-'+s.id,of:s.id,region:field});}
 const shank={id:prefix,kind:'capsule',p0:round(station(0)),p1:round(station(1)),r0:.0047,r1:.0021,bone,material:'oak-bark',role:'detail',tone:.44,slender:true};
 add(shank,'limb-bark');
 for(const [j,node] of nodes.entries()){
  const sign=j%2?-1:1,origin=station(node),dir=axis.clone().multiplyScalar(.32+.03*j).addScaledVector(side,sign).addScaledVector(up,-.23+.035*j).normalize();
  const transverse=dir.clone().cross(up).normalize().applyAxisAngle(dir,rad(tilts[j])),normal=transverse.clone().cross(dir).normalize();
  const len=length*widths[j],wide=width*widths[j],points=[],tris=[];
  for(const [u,r] of contour){
   const along=origin.clone().addScaledVector(dir,len*u).addScaledVector(normal,len*(.16*Math.sin(Math.PI*u)-.12*u*u)).addScaledVector(transverse,.003*Math.sin(2.3*u));
   for(const [frac,down] of [[-1,-.17],[0,0],[1,-.13],[0,-.035]]){
    const lateral=frac*wide*.5*r*(frac<0?1-.07*Math.sin(7*u):1);
    points.push(round(along.clone().addScaledVector(transverse,lateral).addScaledVector(normal,wide*r*down)));
   }
  }
  for(let r=0;r<contour.length-1;r++)for(let i=0;i<4;i++){const l=r*4+i,k=r*4+(i+1)%4;tris.push([l,k,l+4],[k,k+4,l+4]);}
  tris.push([0,3,1],[3,2,1]);const end=points.length-4;tris.push([end,end+1,end+3],[end+1,end+2,end+3]);
  add({id:prefix+'-leaf-'+(j+1),bone,material:parent.material,role:'detail',tone:Math.min(.8,(parent.tone??.55)+.025*(j%3)),kind:'polyhedron',vertices:points,triangles:tris},region);
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc));
