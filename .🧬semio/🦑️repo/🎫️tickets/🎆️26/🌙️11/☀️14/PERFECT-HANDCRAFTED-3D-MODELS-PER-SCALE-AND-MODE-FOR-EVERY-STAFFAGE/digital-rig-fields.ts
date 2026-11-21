/** 🦴 Independently trace every named pose's joint frames, mass centres and three-dimensional load resultants. */
import {Euler,Matrix4,Vector3} from 'three'
import {createHash} from 'node:crypto'
import {mkdirSync,readFileSync,renameSync,writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {catalogEntries} from './volumes-lib.ts'
import {paperInput} from './living-paper-input.ts'
import {frameOf,momentOf} from './digital-rig-math.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {solveStructure,freeBody} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts'
const limits=JSON.parse(readFileSync(join(import.meta.dir,'digital-rig-contract.json'),'utf8'))
const dir=join(import.meta.dir,'🗑️generated','digital-rigs');mkdirSync(dir,{recursive:true})
const radians=Math.PI/180,vec=p=>new Vector3(p.x,p.y,p.z)
const frame=f=>frameOf(vec(f.o),vec(f.u),vec(f.v),vec(f.w))
for(const entry of catalogEntries().filter(e=>process.argv.slice(2).includes(e.slug))){
 let raw=readFileSync(entry.path,'utf8')
 const hash=createHash('sha256').update(raw).digest('hex'),a=paperInput(raw);raw=''
 const findings:string[]=[],fold=new Set(a.rig.radialFolds?.flatMap(f=>f.facets))
 let bones=0,loads=0,members=0,unsupported=0
 for(const pose of a.poses){
  const state=evaluateDrawing(a,{kind:'pose',pose:pose.id}),result=solveStructure(state)
  const check=(distance:number,bound:number,label:string)=>{if(!(distance<=bound))findings.push(pose.id+' '+label+' '+distance)}
  const tol=limits.frameTolerance
  const root=new Matrix4().makeRotationZ(state.root.angleDeg*radians).setPosition(state.root.x,state.root.y,state.root.z)
  let kg=0;const centre=new Vector3()
  for(const b of a.rig.bones){
   const evaluated=state.bones[b.id],f=evaluated.frame,o=evaluated.offset,matrix=frame(f),parent=b.parent?frame(state.bones[b.parent].frame):root
   const rel=new Vector3(b.x+o.dx,b.y+o.dy,b.z??0)
   check(rel.clone().applyMatrix4(parent).distanceTo(vec(f.o)),tol,b.id+' origin')
   check(new Vector3(b.lengthM,0,0).applyMatrix4(matrix).distanceTo(vec(evaluated.tip)),tol,b.id+' tip')
   check(Math.abs(matrix.determinant()-1),tol,b.id+' basis')
   if(!fold.has(b.id)){
    const angles=new Euler(-o.twistDeg*radians,-((b.abductionDeg??0)+o.abductionDeg)*radians,(b.angleDeg+o.angleDeg)*radians,'ZYX')
    const expected=parent.clone().multiply(new Matrix4().makeRotationFromEuler(angles).setPosition(rel))
    check(Math.max(...matrix.elements.map((v,i)=>Math.abs(v-expected.elements[i]))),tol,b.id+' rotation')
   }
   if(b.mass){
    const x=b.mass.distribution==='uniform'&&b.lengthM>0?b.lengthM/2:b.mass.comX
    centre.addScaledVector(new Vector3(x,b.mass.comY,0).applyMatrix4(matrix),b.mass.kg);kg+=b.mass.kg
   }
   bones++
  }
  if(kg)centre.divideScalar(kg)
  check(centre.distanceTo(vec(result.centreOfMass)),tol,'centre of mass');check(Math.abs(kg-result.totalMassKg),tol,'mass')
  const force=new Vector3(),moment=new Vector3()
  for(const part of freeBody(result)){
   const f=new Vector3(part.fxN,part.fyN,part.fzN),m=new Vector3(part.mxNm,part.myNm,part.mzNm)
   force.add(f);moment.add(momentOf(vec(part.at),f,m));loads++
  }
  const r=result.residual,budget=limits.balanceTolerance*Math.max(1,kg*result.gravityMS2)
  check(force.distanceTo(new Vector3(r.fxN,r.fyN,r.fzN)),budget,'force resultant')
  check(moment.distanceTo(new Vector3(r.mxNm,r.myNm,r.mzNm)),budget*Math.max(1,a.dimensions.heightM,a.dimensions.widthM,a.dimensions.depthM),'moment resultant')
  for(const [id,m] of Object.entries(result.members)){
   const f=state.bones[id].frame,matrix=frame(f)
   for(const s of m.stations){
    check(new Vector3(s.s,0,0).applyMatrix4(matrix).distanceTo(new Vector3(s.at.x,s.at.y,s.z)),tol,id+' station')
    const mWorld=vec(s.momentVector)
    check(Math.abs(mWorld.dot(vec(f.u))-s.torsionNm)+Math.abs(mWorld.dot(vec(f.w))-s.momentNm)+Math.abs(mWorld.dot(vec(f.v))-s.momentLateralNm),budget,id+' member moment')
   }
   members++
  }
  if(!result.reactions.some(r=>r.active))unsupported++
 }
 const row={id:entry.slug,hash,signature:process.env.STAFFAGE_RIG_SIGNATURE,poses:a.poses.length,bones,loads,members,unsupported,findings}
 const dest=join(dir,entry.slug+'.json'),tmp=dest+'.'+process.pid
 writeFileSync(tmp,JSON.stringify(row));renameSync(tmp,dest)
 console.log(entry.slug,bones,members,findings.length);Bun.gc(true)
}
