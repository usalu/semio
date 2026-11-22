/** 🐑 Shape the ewe’s loin and chest into staggered banks of crimped wool and draw their orthographic contours. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Quaternion,Vector3} from 'three'
import {meshesAt,poseState} from './volumes-lib.ts'
const fixture=JSON.parse(readFileSync(new URL('./digital-fleece-case.json',import.meta.url),'utf8'))
Bun.plugin({name:'fleece-contours',setup(build){build.onLoad({filter:/r3-goat-draw2d\.ts$/},args=>{
 const source=readFileSync(args.path,'utf8'),end=source.indexOf('const STROKES:')
 if(end<0)throw Error('Contour helper unavailable')
 return {contents:source.slice(0,end)+'\nexport {outline}',loader:'ts'}
})}})
const {outline}=await import('./r3-goat-draw2d.ts')
const drawings=JSON.parse(readFileSync(new URL('./digital-fleece-contours.json',import.meta.url),'utf8'))
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset
const round=x=>+x.toFixed(6),wave=p=>.5+.5*Math.cos(p),tau=2*Math.PI

function shell(s,settings){
 const {sections,...style}=s
 delete style.capStart;delete style.capEnd
 const collar=s.id==='neck',{axial,around}=settings,vertices=[],triangles=[]
 const bands=collar?Math.max(2,settings.bands-2):settings.bands,locks=collar?Math.max(4,settings.locks-3):settings.locks
 const reliefM=collar?settings.reliefM*.48:settings.reliefM
 const origin=i=>new Vector3(...sections[i].at),q=new Quaternion(...sections[0].rotation).normalize()
 for(let i=0;i<=axial;i++){
  const t=i/axial,pos=t*(sections.length-1),j=Math.min(Math.floor(pos),sections.length-2),w=pos-j
  const o=origin(j).lerp(origin(j+1),w),mix=k=>sections[j][k]*(1-w)+sections[j+1][k]*w
  const rx=mix('rx'),ry=mix('ry'),exp=2/mix('exponent'),phase=s.id==='chest'?1.93:.27
  for(let k=0;k<around;k++){
   const angle=tau*k/around,c=Math.cos(angle),u=Math.sin(angle)
   const row=tau*bands*t+.57*Math.sin(2*angle+phase)+.38*Math.cos(3*angle-3*t)
   const curl=locks*angle+.88*Math.sin(tau*1.8*t+phase)+.62*Math.sin(row*.5+angle)
   const lock=wave(curl)*wave(row+.79*Math.sin(curl))+.40*wave(row+.9*Math.cos(curl))
   const fade=Math.min(1,t*13,(1-t)*11)
   const d=reliefM*fade*(1.52*lock-.66)
   const radial=new Vector3(Math.sign(c)*Math.pow(Math.abs(c),exp)*(rx+d-(collar?0:.014)*fade),Math.sign(u)*Math.pow(Math.abs(u),exp)*(ry+d),0)
   vertices.push(radial.applyQuaternion(q).add(o).toArray().map(round))
  }
 }
 for(let i=0;i<axial;i++)for(let j=0;j<around;j++){
  const k=i*around+j,l=i*around+(j+1)%around
  triangles.push([k,k+around,l+around],[k,l+around,l])
 }
 const first=vertices.length;vertices.push(sections[0].at,sections.at(-1).at)
 for(let j=0;j<around;j++){
  const k=(j+1)%around;triangles.push([first,j,k],[first+1,axial*around+k,axial*around+j])
 }
 return {...style,kind:'polyhedron',vertices,triangles}
}
function roundDock(s,settings){
 const {around,axial}=settings,first=(axial+1)*around
 if(s.vertices.length!==first+2)return
 const center=new Vector3(...s.vertices[first]),rim=s.vertices.slice(0,around).map(p=>new Vector3(...p).sub(center))
 const back=rim[0].clone().cross(rim[1]).normalize()
 const depth=Math.min(.034,rim[0].length()*.48)
 s.triangles=s.triangles.filter(tri=>!tri.includes(first))
 let outer=0
 for(const arc of [Math.PI/6,Math.PI/3]){
  const inner=s.vertices.length
  for(const radial of rim)s.vertices.push(center.clone().addScaledVector(radial,Math.cos(arc)).addScaledVector(back,depth*Math.sin(arc)).toArray().map(round))
  for(let j=0;j<around;j++){const k=(j+1)%around;s.triangles.push([outer+j,outer+k,inner+k],[outer+j,inner+k,inner+j])}
  outer=inner
 }
 s.vertices[first]=center.addScaledVector(back,depth).toArray().map(round)
 for(let j=0;j<around;j++)s.triangles.push([outer+j,outer+(j+1)%around,first])
}
function contourDistance(points,outline){
 return Math.max(...points.map(([x,y])=>Math.min(...outline.map(([a,b],i)=>{
  const [c,d]=outline[(i+1)%outline.length],dx=c-a,dy=d-b,t=Math.max(0,Math.min(1,((x-a)*dx+(y-b)*dy)/(dx*dx+dy*dy||1)))
  return Math.hypot(x-a-t*dx,y-b-t*dy)
 }))))
}
for(const [d,settings] of Object.entries(fixture.scales)){
 const v=a.volumes[d],rep=a.representations[d]
 for(const solid of v.body)if(solid.material==='fleece')solid.tone=.82
 for(const shapes of Object.values(rep.geometry))for(const shape of shapes){
  const region=shape.region??shape.regions?.[0]
  if(['fleece-trunk','fleece-neck','wool-upper-legs','tail'].includes(region))shape.tone=.82
 }
 for(const id of fixture.parts){
  const i=v.body.findIndex(s=>s.id===id)
  if(v.body[i].kind==='loft')v.body[i]=shell(v.body[i],settings)
  const solid=v.body[i]
  if(id==='hindbody')roundDock(solid,settings)
  const meshes=meshesAt(a,+d,[solid])
  for(const mesh of meshes)mesh.world=mesh.world.map(x=>Math.round(x*100000)/100000)
  for(const [view,shapes] of Object.entries(rep.geometry)){
   const shape=shapes.find(s=>s.id===id)
   if(d==='500'&&id==='hindbody'&&view==='elevation'){shape.regions=['fleece-trunk','tail'];delete shape.region}
   const probe={...shape,points:[[0,0],[.1,0],[0,.1]]}
   const sample={...a,representations:{...a.representations,[d]:{...rep,geometry:{...rep.geometry,[view]:[probe]}}}}
   const [o,u,w]=poseState(sample,+d)[view][0].rings[0]
   const ux=(u.x-o.x)*10,uy=(u.y-o.y)*10,wx=(w.x-o.x)*10,wy=(w.y-o.y)*10,det=ux*wy-uy*wx
   const projected=outline(view,meshes,settings.outlineM).ring.map(([x,y])=>[round((wy*(x-o.x)-wx*(y-o.y))/det),round((ux*(y-o.y)-uy*(x-o.x))/det)])
   const drawing=drawings[d][view][id],distance=Math.max(contourDistance(projected,drawing),contourDistance(drawing,projected))
   if(distance>Math.max(.001,settings.outlineM*.3))throw Error('Fleece outline diverges at '+[d,view,id,distance].join('/'))
   shape.points=drawing
  }
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc))
