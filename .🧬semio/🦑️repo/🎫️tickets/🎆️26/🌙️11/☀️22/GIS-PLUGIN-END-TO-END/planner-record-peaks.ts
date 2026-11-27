import {learnedHostSlicePeakV1,recordHostSlicePeakV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗓️scheduler/📈️learn/🟦️.ts";
const root="/Users/ueli/Documents/semio",ceiling=Number(process.argv[2]??8192),env=process.env;
console.log("before: scope",learnedHostSlicePeakV1(env,ceiling,Date.now,root),"class",learnedHostSlicePeakV1(env,ceiling));
for(const mb of [3418,3750,4029,4412,4472]){await recordHostSlicePeakV1(env,mb,ceiling,Date.now,root);await new Promise(r=>setTimeout(r,1100));}
console.log("after: scope",learnedHostSlicePeakV1(env,ceiling,Date.now,root),"class",learnedHostSlicePeakV1(env,ceiling));
