import {test,expect} from "bun:test";
import * as routing from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withCargoCompilerInputsV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🟦️.ts";
import {CargoInputContinuationTransactionV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🤲️continuations/🟦️.ts";
test("fixed order settles",async()=>{
 await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"idle-c:repro",maximumElapsedMilliseconds:30000},{fixture:"repro"},Date.now()),async original=>{
  const admission=routing.createIssuedResourceControlV1(original,"repro:admission",0),plan=new CargoInputContinuationTransactionV1(admission),inner=new CargoInputContinuationTransactionV1(admission);
  const request=(control:any)=>({manifest:"/m/Cargo.toml",cwd:"/m",arguments:[],environment:{},control});
  const result=plan.start(consume=>withCargoCompilerInputsV1(request(admission),undefined,consume),async view=>{
   try{await inner.start(consume=>withCargoCompilerInputsV1(request(view.control),undefined,consume),async()=>{});
   throw Error("Native build failed");}catch(error){await inner.close().catch(()=>undefined);throw error;}
  });
  const outcome=await Promise.race([result.then(()=>"resolved",(error:any)=>String(error.message)),Bun.sleep(4000).then(()=>"HANG")]);
  await inner.close().catch(()=>undefined);await plan.close().catch(()=>undefined);
  expect(outcome).toBe("Native build failed");
 });
},20000);
