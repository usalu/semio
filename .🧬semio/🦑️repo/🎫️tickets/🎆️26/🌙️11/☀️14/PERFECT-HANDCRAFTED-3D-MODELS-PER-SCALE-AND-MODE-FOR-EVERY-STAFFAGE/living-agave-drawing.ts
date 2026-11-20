/** 🌵 Split the agave's curved blades into short drawing plates and derive each plate's edge thickness from the authored leaf. */
import {readFileSync,writeFileSync} from "node:fs"
import {join} from "node:path"
import {entryOf} from "./volumes-lib.ts"
import {checkVolumes} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"
const cuts=Number(process.argv[2] ?? 4)
const thick=Number(process.argv[3] ?? .76)
const generated=join(import.meta.dir,"🗑️generated")
const {entry,asset}=entryOf("vegetation/agave")
let src=readFileSync(join(import.meta.dir,"r3-agave-model.ts"),"utf8")
src=src.replace("{ 50: 2, 100: 2, 200: 1, 500: 1 }","{ 50: CUTS, 100: CUTS, 200: CUTS, 500: CUTS }".replaceAll("CUTS",String(cuts)))
src=src.replace("0.008 : 0","0.001 : 0").replace("0.008 : 0","0.001 : 0")
src=src.replace("const solid = { kind: \"extruded\"",`const mid=(hi+lo)/2, slope=(kh+ko)/2; const z0=mid+(lo-mid)*${thick}, z1=mid+(hi-mid)*${thick}, k0=slope+(ko-slope)*${thick}, k1=slope+(kh-slope)*${thick}; const solid = { kind: "extruded"`)
src=src.replace("z0: r5(lo), z1: r5(hi), zTip: [r5(lo + ko * Lb), r5(hi + kh * Lb)]","z0: r5(z0), z1: r5(z1), zTip: [r5(z0+k0*Lb),r5(z1+k1*Lb)]")
src=src.slice(0,src.indexOf("const cand ="))+"export {drawScale};"
src=src.replaceAll(/from "(\.\/[^"]+)"/gu,(_,p)=>"from "+JSON.stringify(join(import.meta.dir,p)))
const js=new Bun.Transpiler({loader:"ts"}).transformSync(src)
const {drawScale}=await import("data:text/javascript;base64,"+Buffer.from(js).toString("base64"))
for(const den of [50,100,200,500]){
    asset.representations[den].geometry=drawScale(den)
}
const test=checkVolumes(asset,{rules:["volumes-silhouette"]})
console.log(JSON.stringify(test.map(i=>[i.scale,i.message.en])))
writeFileSync(join(generated,"living-agave-views.json"),JSON.stringify(asset.representations))

