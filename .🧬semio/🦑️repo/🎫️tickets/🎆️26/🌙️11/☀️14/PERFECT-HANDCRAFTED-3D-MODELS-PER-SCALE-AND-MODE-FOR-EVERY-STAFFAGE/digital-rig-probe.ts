/** 📏 Check the independent frame, segment endpoint and world moment against a hand-calculated asymmetric rig. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Vector3} from 'three'
import {frameOf,momentOf} from './digital-rig-math.ts'
const p=JSON.parse(readFileSync(new URL('./digital-rig-contract.json',import.meta.url),'utf8')).probe
const v=xyz=>new Vector3(...xyz)
const equal=(a,b)=>assert.ok(a.distanceTo(v(b))<1e-9,JSON.stringify([a,b]))
const f=frameOf(v(p.origin),...p.axes.map(v))
equal(v(p.point).applyMatrix4(f),p.world)
equal(v([p.length,0,0]).applyMatrix4(f),p.tip)
equal(momentOf(v(p.world),v(p.force),v(p.couple)),p.momentAboutOrigin)
console.log('The asymmetric frame and resultant moment passed.')
