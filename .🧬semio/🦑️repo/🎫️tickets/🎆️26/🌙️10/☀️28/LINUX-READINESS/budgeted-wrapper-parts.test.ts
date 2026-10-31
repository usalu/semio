import {test,expect} from "bun:test";
import {spawn} from "node:child_process";
import {testSandboxEnvironmentV1} from "/home/deck/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";
import {runBudgetedTestCommand} from "/home/deck/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts";
test("raw spawn closes",async()=>{await new Promise((resolve,reject)=>{const child=spawn("node",["-e",""],{stdio:"inherit",detached:true});child.once("error",reject);child.once("close",resolve);});expect(true).toBe(true);},2000);
test("sandbox synchronous closes",()=>{const sandbox=testSandboxEnvironmentV1(process.env,{route:"settlement-model",checkout:process.cwd()});sandbox.release();expect(true).toBe(true);},2000);
test("wrapper closes",async()=>{await runBudgetedTestCommand("node",["-e",""],{cwd:process.cwd(),budgetMs:1000,throwOnFailure:true});expect(true).toBe(true);},2000);
