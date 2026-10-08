import {test,expect} from "bun:test";
import {readFileSync,writeFileSync,readdirSync} from "node:fs";
import {resolve,join} from "node:path";
import {createHash} from "node:crypto";
const root=resolve(import.meta.dir,"../../../../../../..");
const {runtimeCargoProvenanceV1,runtimeCargoUnitInputsV1,runtimeRetainedBuildResourceInputsV1}=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts"));
test("real completed guest receipts retain strict exact bases and measured current input/resource status",()=>{
 const paths=JSON.parse(readFileSync(join(import.meta.dir,"🗑️generated/oct8-plugin-artifacts/plugin-four-guest-corrected2-historical-index.json"),"utf8")),reports=[];
 const digest=(path:string)=>{try{return createHash("sha256").update(readFileSync(resolve(root,path))).digest("hex")}catch{return undefined}};
 for(const path of paths){const[observation]=runtimeCargoProvenanceV1(readFileSync(path,"utf8"));expect(observation.status).toBe(0);
  const units=observation.units.map(unit=>runtimeCargoUnitInputsV1(observation,unit,root,digest));
  const resources=(observation.buildResources??[]).map(resource=>runtimeRetainedBuildResourceInputsV1(observation,resource,{root,cwd:observation.cwd,outDirectory:resource.out_dir,read:path=>{try{return readFileSync(resolve(root,path))}catch{return undefined}},directory:path=>{try{return readdirSync(resolve(root,path)).map(name=>path+"/"+name)}catch{return undefined}}}));
  reports.push({path,receiptSha256:digest(path),args:observation.args,units:units.length,currentUnits:units.filter(unit=>unit.verified).length,unitFindings:units.flatMap(unit=>unit.findings),resources:resources.map(resource=>({verified:resource.verified,inputs:resource.inputs.length,findings:resource.findings})),testCapabilities:observation.units.flatMap(unit=>unit.message.features.filter(feature=>["mutation-testing","artifact-app-testing"].includes(feature))),fixtureInputs:units.flatMap(unit=>unit.inputs.filter(path=>/🧫️fixtures|🧪️fixtures/.test(path)))});
 }
 writeFileSync(join(import.meta.dir,"🗑️generated/oct8-real-guest-strict-input-review.json"),JSON.stringify(reports,null,2)+"\n");
 expect(reports.length).toBe(4);expect(reports.every(report=>report.testCapabilities.length===0)).toBe(true);expect(reports.every(report=>report.fixtureInputs.length===0)).toBe(true);
 console.log("[DEBUG] actual guest strict retained observation parser accepted4; current unit/resource verdicts measured separately, no source freshness inferred");
});
