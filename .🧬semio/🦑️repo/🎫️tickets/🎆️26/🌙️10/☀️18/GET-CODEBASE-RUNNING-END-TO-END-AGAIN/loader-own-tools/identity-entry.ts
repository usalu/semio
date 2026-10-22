import {createHash} from "node:crypto";
import {writeFileSync} from "node:fs";
import {buildSourceClosureStreamLoaderV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts";
export async function runCapturedBuildSourceV1(args:string[]){
 writeFileSync(args[0]!,buildSourceClosureStreamLoaderV1);
 console.log(JSON.stringify({defined:import.meta.semioCapturedLoaderIdentity,child:createHash("sha256").update(buildSourceClosureStreamLoaderV1).digest("hex")}));
}
