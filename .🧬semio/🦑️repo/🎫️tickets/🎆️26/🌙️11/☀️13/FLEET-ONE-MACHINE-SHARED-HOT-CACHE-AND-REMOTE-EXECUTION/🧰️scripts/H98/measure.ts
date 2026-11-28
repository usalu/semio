import {preflightTargetsV1,unitPeaksFromWardensV1,formatPreflightV1} from "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/H98/stage/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️preflight/🎯️targets/🟦️.ts";
const t0=performance.now();const peaks=unitPeaksFromWardensV1("/Users/Shared/semio-store/v1");
const r=await preflightTargetsV1({root:"/Users/ueli/Documents/semio",targets:process.argv.slice(2),peaks,cacheFile:process.env.CF});
console.log(Object.keys(peaks).length,"peaks;",Math.round(performance.now()-t0),"ms total;",JSON.stringify(r.millis),r.filesRead,"files");
console.log(JSON.stringify(r.units.map(u=>[u.unit,u.verdict,u.peakMegabytes])));console.log(formatPreflightV1(r,"en").split("\n").map(l=>l.slice(0,230)).join("\n"));
