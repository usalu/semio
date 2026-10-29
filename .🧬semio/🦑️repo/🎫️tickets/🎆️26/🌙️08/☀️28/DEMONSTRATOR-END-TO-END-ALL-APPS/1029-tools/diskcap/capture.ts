import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {readNativeRepoRecoveryPlanV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/⌨️cli/🟦️.ts";
import {grantedBuildArgumentsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts";
import schema from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/⌨️cli/🧬️schema/🔣️.json" with {type:"json"};
import fixture from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/⌨️cli/🧫️fixtures/🔣️.json" with {type:"json"};
const root="/Users/ueli/Documents/semio";
console.log("case valid",new Ajv({strict:true}).validate(schema.$defs.RecoveryCaptureCase,fixture.capture));
for(const d of fixture.capture.documents){
 const def=d.kind==="cache-reset"?schema.$defs.ResetOwner:d.kind==="maintenance"?schema.$defs.MaintenanceOwner:schema.$defs.Owner;
 const ok=new Ajv({strict:true}).validate(def,JSON.parse(readFileSync(root+"/"+d.path,"utf8")));
 const plan=readNativeRepoRecoveryPlanV1(root,grantedBuildArgumentsV1(["run",d.request],"run",1));
 console.log(d.request,"owner valid",ok,"plan",plan?.kind===d.kind&&plan?.target===d.target);
}
