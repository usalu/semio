/** 📏️ Exercise the shared-band vectors in a single small process. */
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {vec2} from "gl-matrix";
import {rolesAssetWith} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🧱️synthetic-figure/🟦️.ts";
import {measureSuitability,DEFAULT_PRINT_PROFILE} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const {sharedBoundary:e}=JSON.parse(readFileSync(resolve(import.meta.dir,"../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🕳️valid-range-coverage/🔣️.json"),"utf8"));
const asset=rolesAssetWith(den=>e.drawings.filter((d:any)=>d.band===den).map((d:any)=>({kind:"rect",id:d.id,bone:"ground",role:"body",z:0,filled:true,tone:.5,x:0,y:0,w:d.widthM,h:.5,penMm:e.penMm})));
for(const d of e.drawings){
  const measured=measureSuitability(asset,asset.representations[d.band],e.denominator,DEFAULT_PRINT_PROFILE).violations.filter(v=>v.check==="feature");
  const width=vec2.length(vec2.scale(vec2.create(),vec2.fromValues(d.widthM,0),1000/e.denominator))+e.penMm;
  console.log(JSON.stringify({d,measured,width}));
  assert.deepEqual(measured.map(v=>v.shapes),[[d.id]]);
  assert.ok(Math.abs(measured[0]!.measuredMm-width)<1e-6);
}
const fine=asset.representations[100],coarse=asset.representations[200];
const replacement={...fine,geometry:coarse.geometry};
const probe=measureSuitability(asset,replacement,e.denominator,DEFAULT_PRINT_PROFILE);
assert.deepEqual(probe.violations.filter(v=>v.check==="feature").map(v=>v.shapes),[[e.drawings[1].id]]);
assert.equal(asset.representations[100],fine);
