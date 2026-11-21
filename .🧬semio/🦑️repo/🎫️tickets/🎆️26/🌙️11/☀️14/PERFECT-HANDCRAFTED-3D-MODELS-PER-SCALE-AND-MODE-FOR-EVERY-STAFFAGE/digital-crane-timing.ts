/** ⏱ Identify the active phase of the full-sized crane geometry measurement. */
import {plugin} from 'bun'
import {accelerate} from './digital-containment-study.ts'
import {readFileSync} from 'node:fs'
plugin({name:'staffage-measurement-timing',setup(b){
 b.onLoad({filter:/🧊️volumes\/🟦️\.ts$/},a=>{
  const source=readFileSync(a.path,'utf8')
  return {loader:'ts',contents:source.replace('const shares = exposedShares(picked,','console.error("[DEBUG] begin outer area",region.id,performance.now());const shares = exposedShares(picked,').replace('const exposed = exposedArea(picked,','console.error("[DEBUG] begin exposed area",region.id,performance.now());const exposed = exposedArea(picked,')}
 })
 b.onLoad({filter:/🧮️measures\/🟦️\.ts$/},a=>{
  const raw=readFileSync(a.path,'utf8'),source=process.env.STAFFAGE_COPIES?accelerate(raw):raw
  return {loader:'ts',contents:source.replace('const outward = orientationOf(solid).normals;\n    const reachM','console.error("[DEBUG] surface",solid.id,performance.now()); const outward = orientationOf(solid).normals;\n    const reachM').replace('const weights = subdivision(Math.min', 'if (solid.id==="th-mast-ladder-rail" && t%9===0) console.error("[DEBUG] triangle",t,solid.indices.length,around.length,performance.now());const weights = subdivision(Math.min')}
 })
 b.onLoad({filter:/🌊️flood\/🟦️\.ts$/},a=>{
  const source=readFileSync(a.path,'utf8')
  return {loader:'ts',contents:source.replace('const slot = new Int32Array(nx * ny * nz)','console.error("[DEBUG] flood grid", nx,ny,nz,h,performance.now()); const slot = new Int32Array(nx * ny * nz)').replace('const start = new Int32Array(count + 1)','console.error("[DEBUG] raster complete",count,performance.now()); const start = new Int32Array(count + 1)').replace('if (typeof built !== "number") return built;', 'if (typeof built !== "number") { console.error("[DEBUG] ready for samples", performance.now()); return built; }')}
 })
}})
