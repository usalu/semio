import {test} from "bun:test";
import {receiveScriptProcessInvocation} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runProcessTestSuiteV1,withProcessTestOutputV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🏃️run/🟦️.ts";
import {buildTestRootV1,readBuildTestTableV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🟦️.ts";
const root="/Users/ueli/Documents/semio";
test("a build suite runs through the shared runner under a genuine invocation",()=>receiveScriptProcessInvocation(process.env,invocation=>withProcessTestOutputV1(root,"suite-smoke",output=>runProcessTestSuiteV1({invocation,root:buildTestRootV1,checkout:root,table:readBuildTestTableV1(),suite:process.env.SMOKE_SUITE??"build-memory",owner:"build",output,environment:process.env}))),600000);
