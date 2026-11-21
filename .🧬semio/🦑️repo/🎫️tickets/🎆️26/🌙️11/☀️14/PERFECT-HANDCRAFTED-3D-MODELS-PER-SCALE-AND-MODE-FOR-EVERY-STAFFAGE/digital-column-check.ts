import { readFileSync } from 'node:fs';
import assert from 'node:assert/strict';
import { BoxGeometry } from 'three';
import { unionVolume } from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts';
const f=JSON.parse(readFileSync(new URL('./digital-column-case.json',import.meta.url),'utf8'));
const boxes=f.verticalIntervalsM.map(([lo,hi],i)=>{
  const geometry=new BoxGeometry(f.widthM,hi-lo,f.depthM);
  geometry.translate(.5,(lo+hi)/2,.5);
  return {id:String(i),positions:Float64Array.from(geometry.getAttribute('position').array),indices:Uint32Array.from(geometry.index!.array)};
});
const start=performance.now();
const result=unionVolume(boxes,1/f.gridColumns);
assert(Math.abs(result-f.unionM3)<1e-10);
console.log(JSON.stringify({result,ms:Math.round(performance.now()-start),peakKiB:process.resourceUsage().maxRSS}));
