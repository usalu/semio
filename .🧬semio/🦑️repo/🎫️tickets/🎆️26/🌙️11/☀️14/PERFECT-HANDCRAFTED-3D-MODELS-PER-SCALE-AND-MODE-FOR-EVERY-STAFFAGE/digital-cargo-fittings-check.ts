/** 🚲 Measure the cargo cycle's contact hardware in every digital scale. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Vector3} from 'three'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-cargo-fittings-fixture.json',import.meta.url),'utf8'))
for(const v of Object.values(a.volumes)){
 const get=id=>v.body.find(q=>q.id===id)
 for(const id of f.fittings)assert.ok(get(id),id+' at '+v.denominator)
 assert.equal(get('handlebar-l').sections[0].rx,f.barOuterRadiusM)
 assert.equal(get('stand-leg-l').sections[0].rx,f.standOuterRadiusM)
 assert.equal(get('saddle').sections[0].rx,f.saddleNoseHalfDepthM)
 const p=id=>new Vector3(...get(id).sections[0].at)
 assert.ok(Math.abs(p('stand-leg-l').distanceTo(p('stand-leg-r'))-f.standSideSeparationM)<1e-4)
 assert.ok(v.structure.some(q=>q.member==='bar'&&q.kind==='revolve'&&q.profile.some(([,r])=>Math.abs(r-.0109)<1e-5)))
}
