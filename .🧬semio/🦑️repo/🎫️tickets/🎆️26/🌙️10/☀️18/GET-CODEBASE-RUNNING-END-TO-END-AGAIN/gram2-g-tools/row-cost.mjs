// [gram2-g] bytes per observation row of the REAL record path: N readdir rows through observedFs inside one inference snapshot. usage: bun row-cost.mjs [N=60000] [kind=readdir|exists|lstat]
import{heapStats}from"bun:jsc";import{execFileSync}from"node:child_process";import{join,dirname}from"node:path";
const ROOT="/Users/ueli/Documents/semio",L=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",N=Number(process.argv[2]??"60000"),KIND=process.argv[3]??"readdir";
if(process.env.VARIANT==="ids"){const {transformObservationV1}=await import("./rowdiet/variant.mjs");Bun.plugin({name:"rowdiet",setup(b){b.onLoad({filter:/observation\/[^/]*\.mjs$/},async args=>{const text=await Bun.file(args.path).text();if(!args.path.endsWith("👁️observation/🟨️.mjs"))return {contents:text,loader:"js"};return {contents:transformObservationV1(text),loader:"js"};});}});}
const obs=await import(L+"👁️observation/🟨️.mjs");
const dirs=[...new Set(execFileSync("git",["ls-files","-z"],{cwd:ROOT,maxBuffer:1<<29}).toString().split("\0").filter(Boolean).map(f=>dirname(f)))].slice(0,N).map(d=>join(ROOT,d));
const live=()=>{Bun.gc(true);Bun.gc(true);const h=heapStats();return (h.heapSize+h.extraMemorySize)/1048576;};
const base=live();let during;
const rows=await obs.observeInferenceSnapshotV1(async()=>{
 const {captureObservedSourceInputsV1}=obs;
 const cap=await captureObservedSourceInputsV1(async()=>{for(const d of dirs){if(KIND==="readdir")obs.observedFs.readdirSync(d,{withFileTypes:true});else if(KIND==="exists")obs.observedFs.existsSync(d);else obs.observedFs.lstatSync(d);}during=live();return 1;},undefined,false);
 return cap.observations;
});
const after=live();
console.log(JSON.stringify({kind:KIND,dirs:dirs.length,rows:rows.length,baseMB:base.toFixed(0),duringMB:during.toFixed(0),afterMB:after.toFixed(0),bytesPerRowDuring:Math.round((during-base)*1048576/rows.length),bytesPerRowAfter:Math.round((after-base)*1048576/rows.length)}));
obs.releaseInferenceMemoryV1();console.log("after release MB",live().toFixed(0));
console.log("rows[0]",JSON.stringify(rows[0]).slice(0,300));
