import {test,expect,afterAll} from "bun:test";
import {withBuildTestInvocationV1} from "/home/deck/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
test("first callback settles",()=>withBuildTestInvocationV1(async()=>{expect(true).toBe(true);}),2000);
test("dynamic hook in retained Original",()=>withBuildTestInvocationV1(async()=>{console.error("[DEBUG] before hook");afterAll(()=>{console.error("[DEBUG] dynamic hook ran");});console.error("[DEBUG] after hook");expect(true).toBe(true);}),2000);
