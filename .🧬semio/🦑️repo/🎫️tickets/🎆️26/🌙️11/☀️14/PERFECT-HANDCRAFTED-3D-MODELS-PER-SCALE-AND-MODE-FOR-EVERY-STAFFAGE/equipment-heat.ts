import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";

const staffage = resolve(import.meta.dir,"../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const {solveThermalAt} = await import(pathToFileURL(resolve(staffage,"🔨️modules/🌡️thermal/🟦️.ts")).href);
const read = (file:string) => JSON.parse(readFileSync(resolve(staffage,file),"utf8"));
const radiator = read("🗂️catalog/🧱️components/panel-radiator/🔣️.json").asset;
const room = read("🗂️catalog/🌤️environments/🔣️.json").presets.find((p:any) => p.id === "indoor-heated-winter").environment;
for (const pose of ["off","night","comfort","rated"]) {
  const field = solveThermalAt(radiator,{kind:"pose",pose},room);
  const watts = radiator.thermal.regions.reduce((sum:number,r:any)=>sum+r.areaM2*field.regions[r.id].conductedWM2,0);
  console.log(pose,watts);
  if (pose === "rated" && !(watts>1050 && watts<1300)) process.exitCode=1;
}

