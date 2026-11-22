import {requireSlotShapeV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪢️slots/🟦️.ts";
import {compileJsonSchemaSubset} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import schema from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪢️slots/🧬️schema/🔣️.json" with {type:"json"};
const meta={version:1,slot:"a".repeat(64),storedAt:1,refs:["b".repeat(64),"c".repeat(64)]};
let t=performance.now();for(let i=0;i<5000;i++)requireSlotShapeV1("Meta",meta,"x");console.log("requireSlotShapeV1",((performance.now()-t)/5000*1000).toFixed(0),"us/call");
const c=compileJsonSchemaSubset({$ref:"#/definitions/Meta"},schema);t=performance.now();let bad=0;for(let i=0;i<5000;i++)bad+=c.validate(meta).length;console.log("precompiled",((performance.now()-t)/5000*1000).toFixed(1),"us/call",bad);
console.log("bad refs rejects:",c.validate({...meta,refs:[1]}).length>0, c.validate({...meta,slot:"zz"}).length>0);
