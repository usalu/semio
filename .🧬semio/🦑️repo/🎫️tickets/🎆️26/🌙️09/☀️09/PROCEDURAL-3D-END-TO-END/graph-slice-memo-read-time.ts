import {readFileSync} from "node:fs";
const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const {readBuildOwnerAdmissionV1}=await import(G+"/🕸️graph/🚪️admission/🟦️.ts");
const path=process.argv[2];let t=performance.now();const lap=(name:string)=>{console.log(name,Math.round(performance.now()-t)+"ms","heap",Math.round(process.memoryUsage().heapUsed/1048576)+"MB");t=performance.now();};
const text=readFileSync(path,"utf8");lap("read "+Math.round(text.length/1048576)+"M chars");
const value=JSON.parse(text);lap("parse");
console.log("observations",value.observations.length,"owners",value.owners.length,"files",value.admission.files?.length,"admission keys",Object.keys(value.admission));
readBuildOwnerAdmissionV1(value.admission);lap("validate admission");
