/** 👁 Sculpt the commuter’s paired orbital rims and corneal surfaces onto his head in its own bone frame. */
import {readFileSync,writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {Ray,Vector3} from 'three'
import {tessellateSolid} from '../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🧱️solid/🟦️.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,f=JSON.parse(readFileSync(join(import.meta.dir,'digital-commuter-orbits.json'),'utf8'))
const round=x=>+x.toFixed(7),tau=2*Math.PI
for(const [scale,count] of Object.entries<number>(f.scales)){
 const volume=a.volumes[scale],head=volume.body.find(b=>b.id==='head')
 for(const d of volume.decisions){
  d.en=d.en.replace('Buttons, pockets, eyes, lips, cuffs, clasp and lugs','Buttons, pockets, cuffs, clasp and lugs')
  d.de=d.de.replace('Knöpfe, Taschen, Augen, Lippen, Manschetten','Knöpfe, Taschen, Manschetten')
 }
 volume.decisions.push({
  en:'The eyelids follow the curved sockets. The iris and pupil sit above the sclera inside each lid opening. The nasal wings enclose the recessed nostrils, and the upper and lower lip meet along the oral fissure. These exposed surfaces share the facial thermal field.',
  de:'Die Lider folgen den gewölbten Augenhöhlen. Iris und Pupille liegen innerhalb der Lidspalte über der Lederhaut. Die Nasenflügel fassen die vertieften Nasenlöcher ein. Ober- und Unterlippe treffen sich in der Mundspalte. Diese Oberflächen gehören zum thermischen Gesichtsfeld.'
 })
 const geo=tessellateSolid({...head,node:'head'},qualityOf(a,Number(scale)))
 const vertices=Array.from({length:geo.positions.length/3},(_,i)=>new Vector3(...geo.positions.slice(3*i,3*i+3)))
 const ray=new Ray(new Vector3(),new Vector3(0,1,0)),intersection=new Vector3(),cache=new Map<string,number>()
 function surface(x,z){
  const key=x+','+z
  if(!cache.has(key)){
   ray.origin.set(x,-1,z);let y=Infinity
   for(let j=0;j<geo.indices.length;j+=3)if(ray.intersectTriangle(...[0,1,2].map(k=>vertices[geo.indices[j+k]]),false,intersection))y=Math.min(y,intersection.y)
   if(!Number.isFinite(y))throw Error('Orbit outside the facial surface')
   cache.set(key,y)
  }
  return cache.get(key)!
 }
 function patch(id,rows,outline,back,tone,role='body',material='skin'){
  const points=[],triangles=[],bands=scale==='50'?24:16,n=bands+1
  for(const rear of [false,true])for(const row of rows)for(let i=0;i<=bands;i++){
   const [x,z,depth]=outline(row,2*i/bands-1)
   points.push([x,surface(x,z)-(rear?-back:depth),z].map(round))
  }
  const half=n*rows.length
  const quad=(a,b,c,d)=>triangles.push([a,b,c],[a,c,d])
  for(let i=0;i<rows.length-1;i++)for(let j=0;j<bands;j++){
   const a=i*n+j,b=a+1,c=b+n,d=a+n
   quad(a,b,c,d);quad(half+d,half+c,half+b,half+a)
  }
  const perimeter=[...Array.from({length:n},(_,i)=>i),...Array.from({length:rows.length-1},(_,i)=>(i+2)*n-1),
   ...Array.from({length:bands},(_,i)=>half-2-i),...Array.from({length:rows.length-2},(_,i)=>(rows.length-2-i)*n)]
  for(const [i,b] of perimeter.entries()){const c=perimeter[(i+1)%perimeter.length];quad(c,b,b+half,c+half)}
  const vectors=points.map(q=>new Vector3(...q)),cross=new Vector3()
  if(triangles.reduce((v,[a,b,c])=>v+vectors[a].dot(cross.crossVectors(vectors[b],vectors[c])),0)<0)for(const tri of triangles)tri.reverse()
  const solid={id,kind:'polyhedron',bone:'head',material,role,tone,vertices:points,triangles}
  const old=volume.body.findIndex(q=>q.id===id)
  if(old<0)volume.body.push(solid);else volume.body[old]=solid
  if(!volume.thermal.some(q=>q.of===id))volume.thermal.push({id:'th-'+id,of:id,region:'face-skin'})
 }
 const nasal=[
  [-.012,.008,.002,0],[-.009,.012,.010,.002],[-.006,.016,.018,.006],[-.0035,.018,.024,.011],
  [-.001,.0185,.029,.011],[.002,.0187,.032,.009],[.006,.0176,.034,.005],[.011,.0151,.032,.002],
  [.018,.0125,.028,0],[.030,.0105,.019,0],[.043,.009,.009,0],[.053,.0105,.002,0],[.058,.011,.0003,0]]
 const gaussian=(d,s)=>Math.exp(-.5*(d/s)**2)
 patch('head-nose',nasal,([x,w,h,wing],u)=>{
  const z=u*w,offset=Math.abs(z),sidewall=Math.max(0,1-u*u)
  const bridge=h*sidewall**1.4,ala=wing*gaussian(offset-.013,.003)*sidewall**.35
  const fossa=.0155*gaussian(x+.0035,.0022)*gaussian(offset-.0095,.00215)
  return [x,z,Math.max(.00015,bridge+ala-fossa)]
 },.002,.80)
 const lips=[[-.0084,.0003],[-.0061,.0058],[-.0035,.009],[-.0014,.0077],[0,.0045],
  [.0015,.0078],[.0032,.0086],[.0049,.0062],[.0067,.00025]]
 patch('head-lips',lips,([height,depth],u)=>{
  const taper=Math.max(.045,1-u*u),bow=height>0?.0023*gaussian(Math.abs(u)-.29,.15)*height/.0067:0
  return [-.0324+height*taper+bow+.0008*u*u,u*.0245,depth*taper+.0001]
 },.0085,.67)
 volume.body=volume.body.filter(q=>q.id!=='head-brow')
 volume.thermal=volume.thermal.filter(q=>q.of!=='head-brow')
 for(const [side,sign] of [['l',1],['r',-1]] as const){
  patch('head-brow-'+side,[-1,-.65,.0,.6,1],(b,u)=>{
   const arch=.051+.0045*(1-(u-.1)**2)-.002*u,taper=.35+.65*(1-u*u)
   return [arch+b*.00265*taper,sign*(.0325+.022*u),.00015+.00185*(1-b*b)*taper]
  },.0007,.29,'texture','hair')

  const cx=.03658,cz=sign*f.eyeSpacingM/2
  const eyeBulge=(x,z)=>.001+.0035*Math.max(0,1-((x-cx)/.0085)**2-((z-cz)/.016)**2)
  const vertex=(x,z,depth)=>[x,surface(x,z)-depth,z].map(round)
  function solid(id,tone,loops,closed,texture){
   const verts=loops.flat(),tris=[]
   const joinRows=(i,j)=>{for(let n=0;n<count;n++){
    const m=(n+1)%count,a=i*count+n,b=i*count+m,c=j*count+m,d=j*count+n
    tris.push([a,b,c],[a,c,d])
   }}
   for(let i=0;i<loops.length-1;i++)joinRows(i,i+1)
   if(closed)joinRows(loops.length-1,0)
   else{
    for(const [row,reverse] of [[0,true],[loops.length-1,false]]){
     const index=verts.length,cap=loops[row].reduce((sum,q)=>sum.map((v,k)=>v+q[k]/count),[0,0,0])
     verts.push(cap.map(round))
     for(let i=0;i<count;i++){const n=(i+1)%count;tris.push(reverse?[index,row*count+n,row*count+i]:[index,row*count+i,row*count+n])}
    }
   }
   const p=verts.map(x=>new Vector3(...x)),cross=new Vector3()
   if(tris.reduce((sum,[i,j,k])=>sum+p[i].dot(cross.crossVectors(p[j],p[k])),0)<0)for(const face of tris)face.reverse()
   const s={id:'head-'+id+'-'+side,kind:'polyhedron',bone:'head',material:'skin',role:texture?'texture':'detail',tone,vertices:verts,triangles:tris}
   const ix=volume.body.findIndex(b=>b.id===s.id)
   if(ix<0)volume.body.push(s);else volume.body[ix]=s
   if(!volume.thermal.some(q=>q.of===s.id))volume.thermal.push({id:'th-'+s.id,of:s.id,region:'face-skin'})
  }
  const ring=fn=>Array.from({length:count},(_,i)=>fn(tau*i/count))
  const almond=(width,up,down,depth)=>ring(t=>{
   const u=Math.cos(t),v=Math.sin(t),x=cx+(v>=0?up:down)*v*(.76+.24*v*v)+.0008*u,z=cz+sign*width*u
   return vertex(x,z,depth(x,z,v))
  })
  const outer=fn=>almond(.0168,.0108,.0083,fn)
  const margin=fn=>almond(f.apertureWidthM/2,f.upperApertureM,f.lowerApertureM,fn)
  solid('eyelid',.77,[outer(()=>.0001),margin((x,z)=>eyeBulge(x,z)+.00115),margin((x,z)=>eyeBulge(x,z)-.00055),outer((x,z,v)=>-.007-.006*Math.abs(v))],true,false)
  function disc(id,tone,rx,rz,offset,texture){
   const samples=scale==='50'?[.16,.5,.82,1]:[.22,.68,1]
   const outline=(fraction,depth)=>ring(t=>{
    const x=cx+rx*fraction*Math.sin(t),z=cz+rz*fraction*Math.cos(t)
    return vertex(x,z,eyeBulge(x,z)+depth)
   })
   solid(id,tone,[...samples.map(s=>outline(s,offset)),outline(1,offset-(texture?.0003:.0205))],false,texture)
  }
  disc('eye',.87,.0077,.0153,0,false)
  disc('iris',.32,f.irisDiameterM/2,f.irisDiameterM/2,.00018,true)
  disc('pupil',.10,f.pupilDiameterM/2,f.pupilDiameterM/2,.00031,true)
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc))
