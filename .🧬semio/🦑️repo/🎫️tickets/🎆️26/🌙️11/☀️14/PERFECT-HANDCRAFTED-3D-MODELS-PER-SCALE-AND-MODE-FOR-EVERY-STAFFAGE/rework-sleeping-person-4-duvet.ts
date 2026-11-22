/** 🧵 Copy of digital-sleeping-duvet.ts with the feet and toes among the parts the cloth lies on (rework of sleeping-person): sew down-filled baffles over the measured sleeping body and pull their ridges down as each support moves. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Matrix4,Quaternion,Vector3} from 'three'
import {sampleClip} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts'
import {posedMesh} from './living-world-mesh.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
const [src,dst]=process.argv.slice(2),doc=JSON.parse(readFileSync(src,'utf8')),a=doc.asset,round=x=>+x.toFixed(7),deg=x=>x*180/Math.PI
const around=['torso','pelvis','thigh-l','thigh-r','shank-l','shank-r','foot-l','foot-r','toe-l','toe-r']
const fine=a.volumes['50'].body.filter(p=>around.includes(p.bone)&&p.material!=='duvet-cover')
const height=new Map(),sewn=a.poses.map(pose=>({id:pose.id,bones:pose.bones,query:pose.id}))
const turnover=a.clips.find(c=>c.id==='turn-over')
a.rig.bones=a.rig.bones.filter(b=>!b.id.startsWith('quilt-'))
for(const p of a.poses)for(const k of Object.keys(p.bones))if(k.startsWith('quilt-'))delete p.bones[k]
turnover.keys=turnover.keys.filter(k=>[0,2,4.5,6.5,8].includes(k.t)).map(({t,pose})=>({t,pose,ease:'smooth'}))
const keys=[]
const samples=[...new Set([...Array.from({length:65},(_,n)=>n/8),...Array.from({length:14},(_,n)=>5+n/16)])].sort((a,b)=>a-b)
for(const [n,t] of samples.entries()){
 const base=t<=2?'supine':t<4.5?'supine':t<6.5?'side':'curled',marker=turnover.keys.find(k=>k.t===t)
 if(marker){keys.push({...marker,ease:'linear'});continue}
 const s=sampleClip(a,turnover,t),bones={}
 for(const [id,full] of Object.entries(s.offsets)){const raw=a.poses.find(p=>p.id===base).bones[id]??{},delta={}
  const link=a.rig.bones.find(b=>b.id===id)
  for(const [k,unclamped] of Object.entries(full)){const value=k==='twistDeg'?Math.max(link.joint.twistMinDeg??-180,Math.min(link.joint.twistMaxDeg??180,unclamped)):unclamped;if(Math.abs(value-(raw[k]??0))>1e-6)delta[k]=round(value)}
  if(Object.keys(delta).length)bones[id]=delta
 }
 keys.push({t,pose:base,ease:'linear',bones});sewn.push({id:'turn-'+n,bones,query:{kind:'clip',clip:turnover.id,timeS:t}})
}
for(const pose of sewn){
 const mesh=posedMesh(a,pose.query),q=qualityOf(a,50),cloud=new Map()
 for(const p of fine){const v=mesh(p,q).world;for(let i=0;i<v.length;i+=3){
  const key=Math.round(v[i]*160)+','+Math.round(v[i+2]*160),y=v[i+1]-.46
  if(y>0&&(!cloud.has(key)||y>cloud.get(key)[1]))cloud.set(key,[v[i],y,v[i+2]])
 }}
 const pts=[...cloud.values()]
 height.set(pose.id,(x,z)=>{let top=0;for(const [xx,y,zz] of pts)top=Math.max(top,y-1.04*Math.hypot(x-xx,z-zz));return top})
}
const cells=[],xs=[.595,.74,.91,1.065,1.21,1.39,1.54,1.68,1.805],zs=[-.515,-.37,-.205,-.065,.085,.23,.375,.515],pad=.052
const note=(en,de)=>({en,de}),v=(x,y,z)=>new Vector3(x,y,z)
const definition=(id,parent,title,x,y,z,joint)=>({id,parent,title,x:round(x),y:round(y),z:round(z),lengthM:.007,angleDeg:0,joint})
a.rig.bones=a.rig.bones.filter(b=>!b.id.startsWith('quilt-'))
const quilt=a.rig.bones.find(b=>b.id==='duvet')
function offsets(sx,sz,S){
 const identity=()=>Array.from({length:4},(_,i)=>Array.from({length:4},(_,j)=>+(i===j)))
 const mul=(a,b)=>a.map(row=>row.map((_,j)=>row.reduce((sum,v,k)=>sum+v*b[k][j],0)))
 const h=1/S,m=[[1+3*h,-sz*h,0,sx*h],[-sz*h,1-h,sx*h,0],[0,sx*h,1-h,sz*h],[sx*h,0,sz*h,1-h]].map(row=>row.map(n=>n/4))
 let root=m,inv=identity()
 for(let n=0;n<15;n++){const prod=mul(inv,root),t=prod.map((row,i)=>row.map((x,j)=>(3*+(i===j)-x)/2));root=mul(root,t);inv=mul(t,inv)}
 const O=[[1,1,1,1],[1,1,-1,-1],[1,-1,1,-1],[1,-1,-1,1]].map(row=>row.map(x=>x/2))
 const quad=(a,b)=>a.reduce((t,x,i)=>t+x*m[i].reduce((s,y,j)=>s+y*b[j],0),0)
 for(let n=0;n<32;n++){
  const d=O.map(o=>.25-quad(o,o));if(Math.max(...d.map(Math.abs))<1e-12)break
  const L=Array.from({length:3},(_,i)=>[0,0,0,d[i]]),edges=[]
  for(let i=0;i<4;i++)for(let j=i+1;j<4;j++){
   const c=2*quad(O[i],O[j]);edges.push([i,j,c])
   if(i<3)L[i][i]+=c*c
   if(j<3){L[j][j]+=c*c;L[i][j]-=c*c;L[j][i]-=c*c}
  }
  for(let i=0;i<3;i++){const diag=L[i][i];for(let k=i;k<4;k++)L[i][k]/=diag
   for(let j=0;j<3;j++)if(j!==i){const q=L[j][i];for(let k=i;k<4;k++)L[j][k]-=q*L[i][k]}
  }
  const x=[...L.map(row=>row[3]),0]
  for(const [i,j,v] of edges){const t=Math.max(-.16,Math.min(.16,v*(x[i]-x[j]))),c=Math.cos(t),s=Math.sin(t),a=O[i],b=O[j];O[i]=a.map((u,k)=>u*c+b[k]*s);O[j]=b.map((u,k)=>u*c-a[k]*s)}
 }
 return O.map(row=>{
  const [w,x,y,z]=root.map(r=>2*r.reduce((s,c,i)=>s+c*row[i],0))
  if(Math.abs(w*w+x*x+y*y+z*z-1)>1e-6)throw Error('Cloth weight decomposition')
  const rotation=new Matrix4().makeRotationFromQuaternion(new Quaternion(x,y,z,w))
  const u=v(1,0,0).applyMatrix4(rotation),side=v(0,0,1).applyMatrix4(rotation),v1=v(-u.y,u.x,0).normalize(),up=v(0,1,0).applyMatrix4(rotation)
  return {angleDeg:round(deg(Math.atan2(u.y,u.x))),abductionDeg:round(deg(Math.asin(Math.max(-1,Math.min(1,u.z))))),twistDeg:round(deg(Math.atan2(side.dot(v1),up.dot(v1))))}
 })
}
const heights=new Map()
for(const pose of sewn){
 const h=height.get(pose.id),point=(x,z)=>x+','+z
 const rises=new Map()
 for(let i=0;i<xs.length-1;i++)for(let j=0;j<zs.length-1;j++)for(const half of [0,1]){
  const [x0,x1,z0,z1]=[xs[i],xs[i+1],zs[j],zs[j+1]],p=half?[[x0,z0],[x1,z1],[x0,z1]]:[[x0,z0],[x1,z0],[x1,z1]],y=p.map(([x,z])=>h(x,z))
  let deficiency=0
  for(let m=0;m<=5;m++)for(let n=0;n<=5-m;n++){
   const w=[m/5,n/5,(5-m-n)/5],x=p.reduce((s,c,k)=>s+c[0]*w[k],0),z=p.reduce((s,c,k)=>s+c[1]*w[k],0)
   deficiency=Math.max(deficiency,h(x,z)-y.reduce((s,t,k)=>s+t*w[k],0)-.018)
  }
  for(const [x,z] of p){const k=point(x,z);rises.set(k,Math.max(deficiency,rises.get(k)??0))}
 }
 heights.set(pose.id,(x,z)=>([xs[0],xs.at(-1)].includes(x)&&[zs[0],zs.at(-1)].includes(z))?pad:h(x,z)+pad+rises.get(point(x,z)))
}
for(let i=0;i<xs.length-1;i++)for(let j=0;j<zs.length-1;j++)for(const half of [0,1]){
 const tag=(i+1)+'-'+(j+1)+'-'+(half+1),bone='quilt-'+tag
 const [x0,x1,z0,z1]=[xs[i],xs[i+1],zs[j],zs[j+1]],p=half?[[x0,z0],[x1,z1],[x0,z1]]:[[x0,z0],[x1,z0],[x1,z1]],x=p.reduce((s,c)=>s+c[0]/3,0),z=p.reduce((s,c)=>s+c[1]/3,0)
 const states=sewn.map(pose=>{
  const h=heights.get(pose.id),y=p.map(([u,w])=>h(u,w))
  const sx=(half?y[1]-y[2]:y[1]-y[0])/(x1-x0),sz=(half?y[2]-y[0]:y[2]-y[1])/(z1-z0)
  return {pose,sx,sz,y:y.reduce((s,c)=>s+c/3,0)}
 })
 const S=round(1.065+Math.max(...states.map(s=>Math.hypot(s.sx,s.sz))))
 a.rig.bones.push(definition(bone,'duvet',note('Quilt support','Auflagepunkt der Decke'),x-quilt.x,0,z,{type:'prismatic',axisDeg:90,minM:0,maxM:.56}))
 const weights=Array.from({length:4},(_,n)=>({bone:bone+'-'+(n+1),weight:.25}))
 for(const [n,w] of weights.entries())a.rig.bones.push(definition(w.bone,bone,note('Cloth bias '+(n+1),'Gewebediagonale '+(n+1)),0,0,0,{type:'revolute',minDeg:-180,maxDeg:180,twistMinDeg:-180,twistMaxDeg:180}))
 for(const {pose,sx,sz,y} of states){pose.bones[bone]={dy:round(y)};for(const [n,rot] of offsets(sx,sz,S).entries())pose.bones[weights[n].bone]=rot}
 const points=p.map(([u,w])=>[Math.round((u-x)*S*1.0002*65536)/65536,Math.round((w-z)*S*1.0002*65536)/65536])
 cells.push({id:'baffle-'+tag,weights,bone,S,x,z,points,coords:p,i,j,half,region:Math.abs((z0+z1)/2)>.28?'duvet-edge':(x0+x1)/2<1.02?'quilt-chest':(x0+x1)/2>1.64?'quilt-foot':'bottom'})
}
function panel(c,detail){
 const {id,bone,S,weights,points,tone=.88}=c,grad=c.coords.map(([x,z])=>{
  const ix=xs.indexOf(x),iz=zs.indexOf(z),left=xs[Math.max(0,ix-1)],right=xs[Math.min(xs.length-1,ix+1)],near=zs[Math.max(0,iz-1)],far=zs[Math.min(zs.length-1,iz+1)]
  let gx=0,gz=0
  for(const pose of a.poses){const h=heights.get(pose.id);gx+=(h(right,z)-h(left,z))/(right-left)/3;gz+=(h(x,far)-h(x,near))/(far-near)/3}
  return [gx,gz,a.poses.reduce((sum,pose)=>sum+heights.get(pose.id)(x,z)*S/3,0)]
 })
 const vertices=[],triangles=[],index=new Map(),N=detail
 function vertex(m,n){
  const u=[1-(m+n)/N,m/N,n/N],key=m+'/'+n
  if(index.has(key))return index.get(key)
  const x=u.reduce((s,q,k)=>s+q*points[k][0],0),z=u.reduce((s,q,k)=>s+q*points[k][1],0)
  const edges=[]
  let curved=0,linear=0
  for(let i=0;i<3;i++){
   const y=grad[i][2];curved+=u[i]**3*y;linear+=u[i]*y
   for(let j=0;j<3;j++)if(j!==i){
    const ctrl=y+(grad[i][0]*(points[j][0]-points[i][0])+grad[i][1]*(points[j][1]-points[i][1]))/3
    edges.push(ctrl);curved+=3*u[i]*u[i]*u[j]*ctrl
   }
  }
  const meanEdge=edges.reduce((s,c)=>s+c,0)/6,meanCorner=grad.reduce((s,c)=>s+c[2],0)/3
  curved+=6*u[0]*u[1]*u[2]*(meanEdge+.5*(meanEdge-meanCorner))
  const delta=curved-linear
  const xx=c.x+x/S,zz=c.z+z/S,stitch=.0045*Math.sin(Math.PI*(xx-xs[c.i])/(xs[c.i+1]-xs[c.i]))**2*Math.sin(Math.PI*(zz-zs[c.j])/(zs[c.j+1]-zs[c.j]))**2
  const at=vertices.length;index.set(key,at);vertices.push([x,round(delta+stitch*S),z]);return at
 }
 vertex(0,0);vertex(N,0);vertex(0,N)
 for(let m=0;m<=N;m++)for(let n=0;n<=N-m;n++)vertex(m,n)
 const up=vertices.length
 for(let i=0;i<up;i++){const [x,y,z]=vertices[i];vertices.push([x,round(y-.052*S),z])}
 for(let m=0;m<N;m++)for(let n=0;n<N-m;n++){
  const A=vertex(m,n),B=vertex(m+1,n),C=vertex(m,n+1)
  triangles.push([A,C,B],[A+up,B+up,C+up])
  if(m+n+1<N){const D=vertex(m+1,n+1);triangles.push([B,C,D],[B+up,D+up,C+up])}
 }
 const loop=[...Array.from({length:N},(_,j)=>vertex(j,0)),...Array.from({length:N},(_,j)=>vertex(N-j,j)),...Array.from({length:N},(_,j)=>vertex(0,N-j))]
 const collars=[loop,...[.00035].map(depth=>loop.map(k=>{
  const [x,y,z]=vertices[k],idx=vertices.length;vertices.push([x,round(y-depth*S),z]);return idx
 })),loop.map(k=>k+up)]
 for(let n=1;n<collars.length;n++)for(let j=0;j<loop.length;j++){const l=(j+1)%loop.length,A=collars[n-1][j],B=collars[n-1][l],C=collars[n][j],D=collars[n][l];triangles.push([A,B,D],[A,D,C])}
 return {id,bone,weights,kind:'polyhedron',material:'duvet-cover',role:'body',tone,vertices,triangles}
}
for(const key of keys){
 const base=a.poses.find(p=>p.id===key.pose).bones
 for(const [id,o] of Object.entries(key.bones??{}))if(id.startsWith('quilt-')){
  for(const [field,value] of Object.entries(o))if(Math.abs(value-(base[id]?.[field]??0))<(field==='dy'?1e-7:.00002))delete o[field]
  if(!Object.keys(o).length)delete key.bones[id]
 }
}
turnover.keys=keys
const region=a.thermal.regions.find(r=>r.id==='duvet-edge')
if(!a.thermal.regions.some(r=>r.id==='quilt-foot'))a.thermal.regions.push({...structuredClone(region),id:'quilt-foot',title:note('Foot of the quilt','Fußende der Bettdecke'),areaM2:.26})
const upper=a.thermal.regions.find(r=>r.id==='top')
if(!a.thermal.regions.some(r=>r.id==='quilt-chest'))a.thermal.regions.push({...structuredClone(region),id:'quilt-chest',title:note('Quilt over the chest','Bettdecke über dem Brustkorb'),layers:[structuredClone(upper.layers.find(s=>s.id==='duvet'))]})
upper.title=note('Cotton pyjama top and sleeves','Baumwoll-Schlafanzugoberteil mit Ärmeln');upper.material='cotton-underwear';upper.layers=upper.layers.filter(l=>l.id==='pyjama-top')
for(const [id,areaM2] of Object.entries({'quilt-chest':.572,top:.700,bottom:.706,'duvet-edge':2.362,'quilt-foot':.177}))a.thermal.regions.find(r=>r.id===id).areaM2=areaM2
for(const pose of a.poses){
 const contact=pose.id==='supine'?.32:pose.id==='side'?.19:.62,temp=pose.id==='curled'?24:29
 pose.thermal.regions['quilt-chest']=structuredClone(pose.thermal.regions['top'])
 pose.thermal.regions['quilt-foot']={exposedFraction:round(1-contact),sunlitFraction:0,contact:{fraction:contact,tempC:temp}}
}
const supports=[.02,1.18].flatMap((x,i)=>[-.516,.516].map((z,j)=>({
 id:'quilt-bearing-'+(i?'foot':'head')+'-'+(j?'l':'r'),bone:'duvet',x,y:0,z,kind:'support',title:note('Quilt hem resting on mattress','Deckenrand auf der Matratze'),support:'contact',normalDeg:90,frictionCoefficient:.45,footprintLengthM:.03,footprintWidthM:.008
})))
a.anchors=a.anchors.filter(x=>!x.id.startsWith('quilt-bearing-')).concat(supports)
a.structure.supports=[...new Set([...a.structure.supports,...supports.map(q=>q.id)])]
for(const [scale,vol] of Object.entries(a.volumes)){
 const removed=new Set(vol.body.filter(p=>p.material==='duvet-cover'&&p.id!=='pillow').map(p=>p.id))
 vol.body=vol.body.filter(p=>!removed.has(p.id));vol.thermal=vol.thermal.filter(p=>!removed.has(p.of));vol.structure=vol.structure.filter(p=>p.id!=='member-duvet'&&!p.id.startsWith('quilt-web-')&&!p.id.startsWith('support-quilt-bearing-'))
 for(const surface of vol.thermal)if(surface.region==='top'&&surface.of&&!surface.layer)surface.layer='pyjama-top'
 const geo=a.representations[scale].geometry
 for(const k in geo)geo[k]=geo[k].filter(p=>!(p.material==='duvet-cover'&&p.id!=='pillow'))
 for(const prefix of ['sleeve','foresleeve']){
  const right=prefix+'-r',left=prefix+'-l',original=vol.body.find(s=>s.id===right)
  if(!original||vol.body.some(s=>s.id===left))continue
  const mate=k=>k?.replace(/-r$/,'-l'),bone=mate(original.bone)
  vol.body.push({...structuredClone(original),id:left,bone})
  vol.thermal.push({id:'th-'+left,of:left,region:'top',layer:'pyjama-top'})
  for(const view of Object.values(geo)){
   const drawing=view.find(s=>s.id===right)
   if(drawing&&!view.some(s=>s.id===left)){
    const other=structuredClone(drawing);other.id=left;other.bone=bone
    if(original.kind==='capsule'){
     const [a,b]=[original.p0[0],original.p1[0]],{r0,r1}=original,c=Math.SQRT1_2
     other.points=[[a-r0,0],[a-r0*c,r0*c],[a,r0],[b,r1],[b+r1*c,r1*c],[b+r1,0]]
     other.solid={kind:'revolved',axisBone:bone,axis:'u',axisX:0,axisZ:0,mirror:true}
    }else if(other.solid?.axisBone)other.solid.axisBone=mate(other.solid.axisBone)
    view.push(other)
   }
  }
 }
 for(const c of cells){
  const {id,bone,weights,points,S,region}=c,tone=.88
  let body=panel(c,+scale<=100?4:2)
  if(+scale<=100&&Math.max(...body.vertices.slice(0,15).map(p=>Math.abs(p[1])/S))<(scale==='50'?.012:.014))body=panel(c,2)
  const ys=body.vertices.map(v=>v[1]),from=Math.min(...ys),to=Math.max(...ys)
  vol.body.push(body);vol.thermal.push({id:'th-'+id,of:id,region,...(['quilt-chest','bottom'].includes(region)?{layer:'duvet'}:{})})
  if(c.half===0&&((scale==='50'&&[0,2,4,6].includes(c.j))||[0,3,6].includes(c.j))){const [p,q]=points
   vol.structure.push({id:'quilt-web-'+id,bone,weights,kind:'box',member:'duvet',part:'member',material:'duvet-fill',role:'body',tone:.68,size:[q[0]-p[0],.004*S,.005*S],transform:{translation:[(p[0]+q[0])/2,-.011*S,p[1]]},slender:true})}
  if((c.half===1&&c.i===0)||(c.half===0&&c.i===xs.length-2)){
   const p=points[c.half===1?0:1],q=points[2]
   vol.structure.push({id:'quilt-web-across-'+id,bone,weights,kind:'box',member:'duvet',part:'member',material:'duvet-fill',role:'body',tone:.68,size:[.005*S,.004*S,q[1]-p[1]],transform:{translation:[p[0],-.011*S,(p[1]+q[1])/2]},slender:true})
  }
  for(const view in geo)geo[view].push({id,bone,role:'body',z:round(5+c.i*.011+c.j*.001+c.half*.0001),filled:true,tone,region,material:'duvet-cover',kind:'poly',points,closed:true,penMm:.22,solid:{kind:'extruded',sweep:'v',z0:from,z1:to,weights}})
 }
 for(const sign of [-1,1]){
  const id='side-hem-'+(sign<0?'r':'l'),z=sign*.512
  const pts=[[0,.052],[.21,.050],[.46,.052],[.68,.049],[.90,.052],[1.21,.052],[1.21,-.104],[.98,-.095],[.84,-.10],[.59,-.106],[.35,-.094],[.17,-.102],[0,-.10]],lo=Math.min(z,z+sign*.008),hi=Math.max(z,z+sign*.008)
  vol.body.push({id,bone:'duvet',material:'duvet-cover',role:'body',tone:.85,kind:'extrude',plane:'xy',profile:pts,from:lo,to:hi})
  vol.thermal.push({id:'th-'+id,of:id,region:'duvet-edge'})
  for(const view in geo)geo[view].push({id,bone:'duvet',role:'body',z:5.001,filled:true,tone:.85,region:'duvet-edge',material:'duvet-cover',kind:'poly',points:pts,closed:true,penMm:.22,solid:{kind:'extruded',sweep:'w',z0:lo,z1:hi}})
 }
 for(const q of supports)vol.structure.push({id:'support-'+q.id,bone:'duvet',kind:'box',material:'duvet-fill',role:'detail',tone:.52,size:[.03,.0075,.008],transform:{translation:[q.x,.00375,q.z]},member:'duvet',part:'support',anchor:q.id})
 const d=note('The upper and lower surfaces of the stitched duvet follow the torso, pelvis, knees and calves. The empty foot end settles onto the bed when the sleeper curls. The bound sides hang over the mattress.','Ober- und Unterseite der gesteppten Decke folgen Rumpf, Becken, Knien und Waden. Beim Einrollen sinkt das freie Fußende auf die Matratze. Die eingefassten Seiten hängen über die Matratzenkanten.')
 vol.decisions=[d,...vol.decisions.filter(s=>s.en!==d.en)]
}
writeFileSync(dst,JSON.stringify(doc))
