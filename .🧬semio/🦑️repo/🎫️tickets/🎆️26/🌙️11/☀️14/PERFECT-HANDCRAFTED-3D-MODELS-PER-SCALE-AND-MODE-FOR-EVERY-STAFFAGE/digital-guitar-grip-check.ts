/** 🎸 Measure the left fingertip pads against actual string and tuner triangles in instrument space. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Vector3,Triangle,Raycaster,Mesh,MeshBasicMaterial,BufferGeometry,Float32BufferAttribute,DoubleSide} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {posedMesh} from './living-world-mesh.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-guitar-grip-case.json',import.meta.url),'utf8'))
const from=p=>new Vector3(p.x,p.y,p.z),ray=new Raycaster(),mat=new MeshBasicMaterial({side:DoubleSide})
function geometry(m){const g=new BufferGeometry();g.setAttribute('position',new Float32BufferAttribute(m.world,3));g.setIndex(Array.from(m.indices));return new Mesh(g,mat)}
function range(m,p){const g=m.geometry,k=g.getIndex(),vs=g.getAttribute('position');let best=Infinity
 for(let i=0;i<k.count;i+=3){const read=n=>new Vector3().fromBufferAttribute(vs,k.getX(i+n));const t=new Triangle(read(0),read(1),read(2));best=Math.min(best,t.closestPointToPoint(p,new Vector3()).distanceTo(p))}
 return best
}
let covered=0,worst=0,thumbs=0
for(const pose of a.poses){
 const bones=evaluateDrawing(a,{kind:'pose',pose:pose.id}).bones,g=bones.guitar.frame,world=(x,y,z)=>from(g.o).addScaledVector(from(g.u),x).addScaledVector(from(g.v),y).addScaledVector(from(g.w),z)
 const m=posedMesh(a,pose.id)
 for(const d of f.scales){const vol=a.volumes[d],q=qualityOf(a,d),make=id=>{const s=vol.body.find(s=>s.id===id);assert(s,'missing body '+id);return geometry(m(s,q))}
  for(let n=1;n<=6;n++){
   const post=make('guitar-tuner-post-'+n),spindle=make('guitar-tuner-spindle-'+n),string=a.volumes[50].body.find(s=>s.id==='guitar-string-'+n),tip=world(...string.sections.at(-1).at)
   assert(range(post,tip)<=.0045,'String terminates on the winding post')
   const sh=vol.body.find(s=>s.id==='guitar-tuner-spindle-'+n),button=vol.body.find(s=>s.id===sh.id.replace('spindle-'+n,(n<=3?'b':'t')+(n<=3?n:7-n)))
   assert(button&&range(spindle,world(...button.transform.translation))<.0045,'Button connects with the transverse tuner shaft')
  }
  for(const [name,ref] of Object.entries(f.fingers)){
   const type=name==='thumb'?['metacarpal','proximal','distal']:['proximal','middle','distal'];const ids=type.map(k=>name+'-'+k+'-l')
   for(const [i,id] of ids.entries()){assert(bones[id],pose.id+' needs articulated '+id);assert(Math.abs(from(bones[id].tip).distanceTo(from(bones[id].frame.o))-ref.lengths[i])<.0001)
    if(i)assert(from(bones[ids[i-1]].tip).distanceTo(from(bones[id].frame.o))<.0002)
    const s=vol.body.find(s=>s.bone===id);assert(s&&vol.thermal.some(v=>v.of===s.id&&v.region==='hands')&&vol.structure.some(v=>v.member===id&&v.part==='member'))
   }
   const grip=f.chords[pose.id]?.[name],tuning=pose.id==='tuning'&&['index','thumb'].includes(name)
   if(name==='thumb'&&pose.id!=='tuning'){
    const p=from(bones[ids[2]].tip),wood=make('guitar-neck'),gap=range(wood,p)-ref.radii[3]
    assert(Math.abs(gap)<.0055,pose.id+' rear-neck thumb '+gap);thumbs++
   }
   if(!grip&&!tuning)continue
   const component=make(grip?'guitar-string-'+grip[1]:f.tuner),tip=make('pad-'+ids[2]),sample=from(bones[ids[2]].tip),rad=ref.radii[3]
   const distance=range(component,sample);covered++;worst=Math.max(worst,Math.abs(distance-rad));assert(Math.abs(distance-rad)<f.contactGapM,pose.id+' '+d+' '+name+' pad distance '+distance)
   if(grip){
    const fret=f.neck.nutX-f.neck.scaleLength*(1-2**(-grip[0]/12)),delta=sample.clone().sub(from(g.o)).dot(from(g.u))-fret
    assert(delta>.0005&&delta<.014,'fret spacing '+delta);ray.set(world(fret+f.neck.pressBehindFretM,-.16,sample.clone().sub(from(g.o)).dot(from(g.w))),from(g.v));assert(ray.intersectObject(tip).length,'fingertip on playable face')
   }
  }
 }
}

for(const timeS of [0,.125,.25,.375,.5]){
 const query={kind:'clip',clip:'strum-loop',timeS},bones=evaluateDrawing(a,query).bones,mesh=posedMesh(a,query),d=500
 for(const [name,[fret,number]] of Object.entries(f.chords.strumming)){
  const string=a.volumes[d].body.find(s=>s.id==='guitar-string-'+number),location=from(bones[name+'-distal-l'].tip)
  assert(Math.abs(range(geometry(mesh(string,qualityOf(a,d))),location)-f.fingers[name].radii[3])<f.contactGapM,'strum-loop '+timeS+' '+name)
 }
}
console.log(JSON.stringify({contactMeasurements:covered,opposingThumbChecks:thumbs,maximumPadGapMm:worst*1000,upDownClipSamples:5}))
