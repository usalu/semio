import {test,expect} from "bun:test";
import {spawn} from "node:child_process";
import {withBuildTestInvocationV1} from "/home/deck/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {testSandboxEnvironmentV1} from "/home/deck/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";
test("raw spawn with actual Original",()=>withBuildTestInvocationV1(async()=>{await new Promise((resolve,reject)=>{const child=spawn("node",["-e",""],{stdio:"inherit",detached:true});child.once("error",reject);child.once("close",resolve);});expect(true).toBe(true);}),2000);
test("sandbox with actual Original",()=>withBuildTestInvocationV1(async()=>{const sandbox=testSandboxEnvironmentV1(process.env,{route:"settlement-model",checkout:process.cwd()});sandbox.release();expect(true).toBe(true);}),2000);
