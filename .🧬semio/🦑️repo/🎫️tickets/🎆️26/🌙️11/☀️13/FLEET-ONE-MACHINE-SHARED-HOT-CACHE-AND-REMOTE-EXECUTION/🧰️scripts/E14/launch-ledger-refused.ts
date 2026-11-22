import {launchGraphOptionalCollectorV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🚀️launch/🟦️.ts";
const lines:string[]=[];
const decision=await launchGraphOptionalCollectorV1("/tmp/none",0,{report:line=>lines.push(line),environment:{SEMIO_LANGUAGE:"en"}});
console.log(JSON.stringify({decision,lines:lines.map(l=>l.slice(0,200))}));
