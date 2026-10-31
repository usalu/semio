import {test,expect} from "bun:test";
import {withBuildTestInvocationV1} from "/home/deck/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {runBudgetedTestCommand} from "/home/deck/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts";
test("genuine asynchronous control",()=>withBuildTestInvocationV1(async()=>{await new Promise(resolve=>setImmediate(resolve));expect(true).toBe(true);}),2000);
test("budgeted Node command closes its host test",()=>withBuildTestInvocationV1(async original=>{await runBudgetedTestCommand("node",["-e","process.stdout.write(\"ok\")"],{cwd:process.cwd(),budgetMs:1000,signal:original.control.signal,throwOnFailure:true});expect(true).toBe(true);}),2000);
