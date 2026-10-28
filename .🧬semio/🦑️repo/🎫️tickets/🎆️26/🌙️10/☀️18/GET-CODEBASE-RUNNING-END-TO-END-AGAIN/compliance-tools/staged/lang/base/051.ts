/** 🏷️ Contract: when the loader or the snapshot a captured command proves differs from the one this process loaded (a peer saved the loader while the front door waited for capacity), the command ends as a stale implementation (the outermost command starts again, exit 75), while a malformed proof stays a plain refusal. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {requireCapturedBuildCommandV1,type CapturedBuildCommandProofV1} from "../../🟦️.ts";
import {buildSourceClosureStreamLoaderV1} from "../../../🟦️.ts";
import {STALE_IMPLEMENTATION_CODES_V1,concurrentEditOfV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";

const sha=(text:string)=>createHash("sha256").update(text).digest("hex"),loader=sha(buildSourceClosureStreamLoaderV1),entry=sha("entry");
const proof=(change:Partial<{root:string;sha256:string;loaderSha256:string}>={}):CapturedBuildCommandProofV1=>({version:1,root:process.cwd(),snapshot:null,sha256:entry,loaderSha256:loader,recheck:()=>{},...change});
const failure=(run:()=>void):Error&{code?:string}=>{try{run();}catch(error){return error as Error;}throw Error("A refusal was expected");};

test("a proof that matches the loaded loader and snapshot is accepted",()=>{expect(()=>requireCapturedBuildCommandV1(proof(),entry,loader)).not.toThrow();});

test("a loader or snapshot that differs from the loaded one is a stale implementation, never a plain refusal",()=>{
 for(const[name,run]of[["loader identity",()=>requireCapturedBuildCommandV1(proof(),entry,sha("other loader"))],["snapshot identity",()=>requireCapturedBuildCommandV1(proof(),sha("other entry"),loader)],["loaded constant",()=>requireCapturedBuildCommandV1(proof({loaderSha256:sha("peer saved")}),entry,sha("peer saved"))]] as const){
  const error=failure(run);expect(STALE_IMPLEMENTATION_CODES_V1.has(error.code!),name).toBe(true);expect(error.message,name).toContain("restart required");expect(concurrentEditOfV1(error),name).toBeUndefined();
 }
});

test("a malformed proof stays a plain refusal",()=>{
 for(const bad of[proof({root:"relative/root"}),{...proof(),recheck:undefined as never},proof({sha256:"short"})]){const error=failure(()=>requireCapturedBuildCommandV1(bad,entry,loader));expect(error.message).toBe("Authenticated captured build command entry required");expect(error.code).toBeUndefined();}
});
