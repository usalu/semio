import{test}from"bun:test";
import{proveAcceptedOriginalDrainV1}from"/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/📥️issuance/🧪️tests/📥️drain/🟦️.ts";
test("mode",async()=>{const mode=process.env.AUTHREV_MODE!;const r=await proveAcceptedOriginalDrainV1(mode,undefined as never);console.log("EVT "+mode+" "+JSON.stringify(r));},60000);
