import {receiveScriptProcessInvocation} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {readIssuedScriptProcessEnvelopeV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {readNativeCallerAuthorityV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
await receiveScriptProcessInvocation(process.env,async original=>{const a=readNativeCallerAuthorityV1(readIssuedScriptProcessEnvelopeV1(original).capabilities);console.log("AUTH",JSON.stringify({role:a.role,original:a.original}).slice(0,600));});
