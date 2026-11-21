/** 🐈 Model the feline aperture and pupil, and curl the five individual caudal segments around the forepaws. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,f=JSON.parse(readFileSync(new URL('./digital-cat-anatomy.json',import.meta.url),'utf8'))
const r=x=>+x.toFixed(8),V=p=>new Vector3(p.x,p.y,p.z),p=a.poses.find(p=>p.id==='sitting-tail-wrapped')
for(const [i,xyz] of f.tailDirectionsMm.entries()){
 const id='tail-'+(i+1),bone=a.rig.bones.find(b=>b.id===id),parent=evaluateDrawing(a,{kind:'pose',pose:p.id}).bones[bone.parent].frame
 const dir=new Vector3(...xyz).normalize(),x=dir.dot(V(parent.u)),y=dir.dot(V(parent.v)),z=dir.dot(V(parent.w))
 const pitch=Math.atan2(y,x)*180/Math.PI-bone.angleDeg
 p.bones[id]={angleDeg:r((pitch+540)%360-180),abductionDeg:r(Math.atan2(z,Math.hypot(x,y))*180/Math.PI-(bone.abductionDeg??0))}
}
const fine=structuredClone(a.volumes['50']),W=f.eyeWidthM/2,H=f.eyeHeightM/2,D=f.eyeDepthRadiusM
function pupil(side,eye,count){
 const verts=[],triangles=[],curve=(y,z)=>D*Math.sqrt(Math.max(0,1-(y/H)**2-(z/W)**2))
 for(const offset of [.00017,-.0008])for(let i=0;i<count;i++){
  const phi=i*2*Math.PI/count,y=f.pupilHeightM/2*Math.sin(phi),z=f.pupilWidthM/2*Math.sign(Math.cos(phi))*Math.abs(Math.cos(phi))**1.8
  verts.push([r(curve(y,z)+offset),r(y),r(z)])
 }
 verts.push([r(D+.00017),0,0],[r(D-.0008),0,0])
 for(let i=0;i<count;i++){const j=(i+1)%count;triangles.push([2*count,j,i],[2*count+1,i+count,j+count],[i,j,j+count],[i,j+count,i+count])}
 const v=verts.map(q=>new Vector3(...q));if(triangles.reduce((s,[i,j,k])=>s+v[i].dot(new Vector3().crossVectors(v[j],v[k])),0)<0)for(const q of triangles)q.reverse()
 return {id:'pupil-'+side,kind:'polyhedron',bone:'head',material:'cornea',role:'detail',tone:.025,texture:true,vertices:verts,triangles,transform:structuredClone(eye.transform)}
}
for(const [scale,v] of Object.entries(a.volumes)){
 for(const side of ['n','f']){
  const basis=fine.body.find(q=>q.id==='eye-'+side)
  const eye=v.body.find(q=>q.id==='eye-'+side)
  if(eye){eye.radii=[D,H,W];eye.tone=.46;v.body.push(pupil(side,eye,scale==='50'?16:12))}
  const q=v.thermal.findIndex(s=>s.id==='th-eye-'+side)
  const angles=[-14,0,18,36,54,72,89.8]
  v.thermal[q]={id:'th-eye-'+side,kind:'loft',bone:'head',material:'cornea',role:'detail',tone:.46,region:'eyes',transform:structuredClone(basis.transform),capStart:false,capEnd:true,
   sections:angles.map(d=>{const t=d*Math.PI/180;return {at:[r(D*Math.sin(t)),0,0],rx:r(W*Math.cos(t)),ry:r(H*Math.cos(t)),exponent:2,rotation:[0,-.70710678,0,.70710678]}})}
 }
 if(Number(scale)>=200){
  v.body=v.body.filter(q=>!['tail-a','tail-b'].includes(q.id))
  v.thermal=v.thermal.filter(q=>!['tail-a','tail-b'].includes(q.of))
  for(let n=1;n<=5;n++){
   const id='tail-'+n,solid=structuredClone(fine.body.find(q=>q.id===id)),all=solid.sections,lo=0,hi=all.length-1
   const ids=Number(scale)===200?[lo,2,4,Math.ceil(hi*.65),hi-2,hi]:[lo,4,hi-2,hi]
   solid.sections=[...new Set(ids)].sort((x,y)=>x-y).map(i=>all[i]);v.body.push(solid)
   v.thermal.push(structuredClone(fine.thermal.find(s=>s.of===id)))
  }
 }
 const rep=a.representations[scale]
 if(rep.status==='authored')for(const [view,shapes] of Object.entries(rep.geometry)){
  if(scale==='100'&&view==='elevation')for(const pad of shapes.filter(q=>['near-fore-pad','near-hind-pad'].includes(q.id)))pad.penMm=.20
  if(Number(scale)===200){
   const idx=shapes.findIndex(s=>s.id==='tail-a'),style=shapes[idx]
   shapes.splice(idx,1);shapes.splice(shapes.findIndex(s=>s.id==='tail-b'),1)
   for(let n=1;n<=5;n++)shapes.splice(idx+n-1,0,{...structuredClone(style),id:'tail-'+n,bone:'tail-'+n})
  }
  for(const s of shapes.filter(q=>/^tail-[1-5]$/.test(q.id))){
   delete s.followTip
   const sections=v.body.find(q=>q.id===s.id).sections
   s.points=sections.map(q=>[q.at[0],r(q.at[1]+q.ry)]).concat([[sections.at(-1).at[0],0],[sections[0].at[0],0]])
   s.solid={kind:'revolved',axisBone:s.bone,axis:'u',axisX:0,axisZ:0,mirror:true}
  }
 }
 v.decisions.push({en:'Five caudal sections wrap beside the seated paws. The tear surface follows the exposed front of the cornea. The fine views distinguish the iris and vertical pupil.',de:'Fünf Schwanzabschnitte schmiegen sich seitlich um die sitzenden Pfoten. Der Tränenfilm folgt der freien Hornhautvorderseite. In den feinen Ansichten sind Iris und senkrechte Pupille getrennt.'})
}
writeFileSync(process.argv[3],JSON.stringify(doc))
