import {test} from "bun:test";
import {privateRepositoryWorkspaceV1,testCheckoutRootV1} from "./../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧫️private-repository/🟦️.ts";
import {runCanonicalGoTests} from "./../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts";
import {join} from "node:path";
const checkout=testCheckoutRootV1(),moduleRoot=join(checkout,"🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree/📦️packages/🐹️go");
test("actual tree operation capture regression",async()=>{const workspace=await privateRepositoryWorkspaceV1("graph");const prior=process.env.SEMIO_WORKSPACE_ROOT;process.env.SEMIO_WORKSPACE_ROOT=workspace.root;try{await workspace.invoke(original=>runCanonicalGoTests(moduleRoot,["-run=TestCachedQueryCapturesSourceOnEveryOperation","-count=1","-v"],original,{packages:["."],env:{...process.env,GOWORK:join(checkout,"go.work")}}),{owner:"query-source-capture",maximumElapsedMilliseconds:120000});}finally{if(prior===undefined)delete process.env.SEMIO_WORKSPACE_ROOT;else process.env.SEMIO_WORKSPACE_ROOT=prior;}},120000);
