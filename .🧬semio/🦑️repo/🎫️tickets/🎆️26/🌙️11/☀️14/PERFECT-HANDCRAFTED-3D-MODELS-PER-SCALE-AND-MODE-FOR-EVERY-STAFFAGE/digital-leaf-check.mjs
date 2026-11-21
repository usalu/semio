import assert from 'node:assert/strict'
import { readFileSync, readdirSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { join } from 'node:path'
import { Vector3, Box3 } from 'three'
const here=fileURLToPath(new URL('.',import.meta.url))
const test=JSON.parse(readFileSync(join(here,'digital-leaf-case.json')))
const catalog=join(process.cwd(),'🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog')
const asset=JSON.parse(readFileSync(join(catalog,readdirSync(catalog).find(s=>s.endsWith('furniture')),test.asset,'🔣️.json'))).asset
for(const scale of ['50','100','200','500']) {
  const solids=asset.volumes[scale].body
  if(scale==='500') {
    const top=solids.find(b=>b.id==='slab-l')
    const insert=solids.find(b=>b.id==='leaf')
    assert.equal(top.to-top.from,test.dimensionsM.board)
    assert.equal(insert.to-insert.from,test.dimensionsM.board)
    assert(Math.abs(.735+top.from-(.69+insert.to))<test.gapToleranceM)
  }
  const frame=asset.rig.bones.find(b=>b.id==='frame')
  const leaf=asset.rig.bones.find(b=>b.id==='leaf')
  for(const [index,x] of test.stationsX.entries()) {
    const rod=solids.find(s=>s.id==='lift-screw-'+index)
    const nut=solids.find(s=>s.id==='lift-nut-'+index)
    assert(rod && nut, 'A screw and a bearing nut carry each end of the insert')
    const head=Math.max(...rod.profile.map(p=>p[0]))
    const foot=Math.min(...rod.profile.map(p=>p[0]))
    const levels=nut.profile.map(p=>p[0]+nut.origin[1]+frame.y)
    for(let step=0;step<test.samples;step++) {
      const lift=test.strokeM*step/(test.samples-1)
      const sheet=new Box3(new Vector3(leaf.x,leaf.y+lift,-.45),new Vector3(leaf.x+test.dimensionsM.insert,leaf.y+lift+test.dimensionsM.board,.45))
      for(const [n,z] of test.bearingsZ.entries()) {
        const p=new Vector3(...rod.origin).add(new Vector3(leaf.x,leaf.y+lift+head,0)).addScaledVector(new Vector3(...rod.repeat.translation),n)
        assert(Math.abs(p.x-x)<test.gapToleranceM && Math.abs(p.z-z)<test.gapToleranceM)
        assert(Math.abs(sheet.min.y-p.y)<test.gapToleranceM && sheet.distanceToPoint(p)<test.gapToleranceM)
        const base=p.y-head+foot
        assert(Math.min(...[p.y, Math.max(...levels)])-Math.max(base,Math.min(...levels))>=test.requiredThreadEngagementM)
      }
    }
  }
}
console.log('Four lift screws contact the insert through the entire 30 mm stroke and retain thread engagement at all four scales.')
