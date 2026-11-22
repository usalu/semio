/** 🚋 Verify the daylight opening of each rubber bead against its own fixed pane. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {ShapeUtils,Vector2} from 'three'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-transit-cases.json',import.meta.url),'utf8'))[a.id]
for(const [scale,v] of Object.entries(a.volumes)){
 const windows=v.body.filter(q=>q.id.startsWith('pane-')&&q.kind==='box')
 assert.ok(windows.length>=12)
 if(f.rectangularPanes)assert.equal(windows.length,f.rectangularPanes)
 for(const p of windows){
  const bead=v.body.find(q=>q.id==='seal-'+p.id);assert.ok(bead?.holes?.length===1,p.id)
  const loop=bead.holes[0].map(x=>new Vector2(...x));assert.ok(Math.abs(ShapeUtils.area(loop))>.7*p.size[0]*p.size[1])
  const width=points=>Math.max(...points.map(x=>x[0]))-Math.min(...points.map(x=>x[0]))
  assert.ok(Math.abs((width(bead.profile)-width(bead.holes[0]))/2-f.lipWidthM)<.00001)
  assert.equal(bead.bone,p.bone);assert.ok(v.thermal.some(q=>q.of===bead.id&&q.region==='window-seals'))
 }
}
