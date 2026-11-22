/** 🐿️ Check posed eye, nasal opening, attachment and digit surfaces against their independent geometric cases. */
import {readFileSync} from 'node:fs'
import {Ray,Triangle,Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
import {exposedArea} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./squirrel-extremities-case.json',import.meta.url),'utf8')),bad=new Set()
const V=p=>new Vector3(p.x,p.y,p.z),pos=(fr,p)=>V(fr.o).addScaledVector(V(fr.u),p[0]).addScaledVector(V(fr.v),p[1]).addScaledVector(V(fr.w),p[2])
function hits(m,o,d){
 if(!m)return []
 const ray=new Ray(o,d),near=[],v=[];for(let i=0;i<m.positions.length;i+=3)v.push(new Vector3().fromArray(m.positions,i))
 for(let i=0;i<m.indices.length;i+=3){const t=new Vector3(),s=m.indices;if(ray.intersectTriangle(v[s[i]],v[s[i+1]],v[s[i+2]],false,t))near.push(t.distanceTo(o))}
 return [...new Set(near.map(t=>+t.toFixed(7)))].sort((a,b)=>a-b)
}
for(const k of f.scales){
 const vol=a.volumes[k],count={k,minEye:1,minAir:1,maxContactGap:0}
 for(const pose of f.poses){
  const b=evaluateDrawing(a,{kind:'pose',pose}).bones,ms=worldSolids(a,b,k).filter(s=>!s.bone.startsWith('tail')),by=new Map(ms.map(s=>[s.id,s]))
  const fail=s=>bad.add(k+'/'+s)
  for(const [side,code,sign] of [['near','n',-1],['far','f',1]]){
   const eye=by.get(side+'-eye'),area=eye?(pose===f.poses[0]?exposedArea([eye],ms.filter(s=>s.bone==='head'),[],()=>false,.00016,0):count.minEye):0
   count.minEye=Math.min(count.minEye,area);if(area<f.minimumEyeAreaM2||!vol.thermal.some(s=>s.of===side+'-eye'&&s.region==='eyes'))fail('eye')
   const fr=b.head.frame,o=pos(fr,[.061,.00165,.0027*sign]),d=V(fr.u).negate()
   const depths=['nose','head','nasal-recess'].flatMap(id=>hits(by.get(id),o,d)),surface=hits(by.get('nose'),pos(fr,[.061,.00165,0]),d)[0]
   const air=Math.min(...depths)-surface;count.minAir=Math.min(count.minAir,air);if(!(air>f.minimumNarisDepthM&&air<.008))fail('naris')
   for(const [limb,parent] of [['arm','chest'],['thigh','rump']]){
    const fr=b[limb+'-'+code].frame,origin=V(fr.o),parentMesh=by.get(parent),dirs=[new Vector3(1,.187,.061).normalize(),new Vector3(-.13,.89,.29).normalize()]
    if(!dirs.every(d=>hits(parentMesh,origin,d).length%2===1))fail(limb+' origin outside '+parent+' '+pose)
   }
   for(const [part,n,region] of [['hand',f.foreDigits,'hands-skin'],['foot',f.hindDigits,'feet-skin']]){
    if(pose===f.poses[0]){
     const frame=b[part+'-'+code].frame,group=ms.filter(q=>q.bone===part+'-'+code)
     for(const anchor of a.anchors.filter(q=>['ground-'+part+'-'+code,'bark-'+part+'-'+code].includes(q.id))){
      const o=pos(frame,[anchor.x,anchor.y,anchor.z??0]),t=new Vector3(),tri=new Triangle();let gap=Infinity
      for(const m of group)for(let j=0;j<m.indices.length;j+=3){
       tri.a.fromArray(m.positions,m.indices[j]*3);tri.b.fromArray(m.positions,m.indices[j+1]*3);tri.c.fromArray(m.positions,m.indices[j+2]*3)
       gap=Math.min(gap,tri.closestPointToPoint(o,t).distanceTo(o))
      }
      count.maxContactGap=Math.max(count.maxContactGap,gap)
      if(gap>.002)fail(anchor.id+' contact '+(gap*1000).toFixed(2))
     }
    }
    const digits=vol.body.filter(s=>s.id.startsWith(side+'-'+part+'-digit-'))
    if(digits.length!==n||digits.some(s=>s.bone!==part+'-'+code||!vol.thermal.some(t=>t.of===s.id&&t.region===region)||!vol.structure.some(t=>t.id==='phalanx-'+s.id)))fail(part+' digit bindings')
    const z=digits.map(s=>s.sections.at(-1).at[2]);if(Math.max(...z)-Math.min(...z)<f.minimumFootWidthM[part])fail(part+' width')
   }
  }
 }
 console.log(JSON.stringify(count))
}
if(bad.size){console.error([...bad]);process.exitCode=1}
