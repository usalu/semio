const root="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/🟦️.ts";
const {runCurrentFileRecheckChecksV1,runFileObservationChecksV1}=await import(root);
console.log("recheck",await runCurrentFileRecheckChecksV1());
console.log("observation",await runFileObservationChecksV1());
