/** 🧊 Compares separated and touching translated components and exact body references with explicit independent mesh expansion. */
import assert from 'node:assert/strict'
import {Triangle} from 'three'
import {readFileSync} from 'node:fs'
import {join} from 'node:path'
import {STAFFAGE} from './volumes-lib.ts'
import {worldSolids,referenceOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
import {orientationOf,outerArea,insideAny} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts'
import {evaluate} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const sample=JSON.parse(readFileSync(join(STAFFAGE,'🧫️fixtures/👤️sample-figure/🔣️.json'),'utf8')).asset
const bones=evaluate(sample,{kind:'pose',pose:sample.presentation.defaultPose}).bones, first=sample.volumes['50'].body[0]
for(const distance of [.1,.2]){
 const box={id:'links',bone:first.bone,material:first.material,role:'body',tone:.5,kind:'box',size:[.1,.1,.1],repeat:{count:38,translation:[distance,0,0]}}
 const volume={...sample.volumes['50'],body:[box],thermal:[{id:'outer',of:'links',region:'skin'}]},a={...sample,volumes:{...sample.volumes,'50':volume}}
 const body=worldSolids(a,bones,50),refs=worldSolids(a,bones,50,'thermal',body),s=body[0],r=refs[0]
 const explicit={id:s.id,positions:s.positions,indices:s.indices,frame:s.frame}
 const projection=referenceOf('elevation',body,undefined)
 assert.equal(projection.cut,projection.visible);assert.equal(projection.cut,projection.full)
 for(let i=0;i<s.indices.length;i++){
  const projected=projection.cut[Math.floor(i/3)][0][i%3],j=s.indices[i]*3
  assert.equal(projected.x,s.positions[j]);assert.equal(projected.y,s.positions[j+1])
 }
 const section=referenceOf('elevation',body,{plane:0,label:{en:'Section',de:'Schnitt'}})
 assert.equal(section.full.length,s.indices.length/3)
 assert.equal(r.positions,s.positions);assert.equal(r.indices,s.indices);assert.equal(!!s.separateCopies,distance>.1)
 assert.equal(orientationOf(r),orientationOf(s));assert.equal(orientationOf(r).closed,orientationOf(explicit).closed)
 assert.deepEqual(orientationOf(r).normals,orientationOf(explicit).normals)
 const tr=new Triangle();let threeArea=0
 for(let i=0;i<s.indices.length;i+=3){
  tr.a.fromArray(s.positions,3*s.indices[i]);tr.b.fromArray(s.positions,3*s.indices[i+1]);tr.c.fromArray(s.positions,3*s.indices[i+2]);threeArea+=tr.getArea()
 }
 assert.ok(Math.abs(threeArea-2.28)<1e-9)
 const area=outerArea([r]);assert.ok(Math.abs(area-2.28)<1e-9);assert.ok(Math.abs(area-outerArea([explicit]))<1e-9)
 assert.equal(insideAny([r],[0,0,0]),insideAny([explicit],[0,0,0]))
 console.log({distance,area,threeArea,closed:orientationOf(r).closed,independent:!!s.separateCopies})
}
