import{requireCapturedBuildCommandV1,adoptCapturedBuildCommandV1,type CapturedBuildCommandProofV1}from"../../📥️source/🔏️closure/🚪️command/🟦️.ts";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1,withIssuedResourceOperationV1,type ScriptInvocation}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type{ResourceLeaseControl}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{issueRepositoryScriptProvidersV1}from"../../../🏃️process/🔌️providers/🟦️.ts";
import{withImplementationPinV1}from"../../📥️inputs/🔏️revision/🟨️.mjs";
import{exitIfStaleImplementationV1}from"../../🧷️format-stamps/🟨️.mjs";
export type{CapturedBuildCommandProofV1}from"../../📥️source/🔏️closure/🚪️command/🟦️.ts";
/** 🔏️ Enters one authenticated verb kernel under the inherited Original without another capacity grant. */
export async function runCapturedRepositoryKernelV1(args:readonly string[],proof:CapturedBuildCommandProofV1,identity:unknown,loader:unknown,body:(root:string,args:readonly string[],original:ScriptInvocation,control:ResourceLeaseControl)=>Promise<void>):Promise<void>{requireCapturedBuildCommandV1(proof,identity,loader);adoptCapturedBuildCommandV1(proof);proof.recheck();issueRepositoryScriptProvidersV1(process.env);try{await receiveScriptProcessInvocation(process.env,original=>{const control=createIssuedResourceControlV1(original,"repository:command",0);return withIssuedResourceOperationV1(control,()=>withImplementationPinV1(()=>body(proof.root,args,original,control)));});}catch(error){exitIfStaleImplementationV1(error);throw error;}proof.recheck();}
