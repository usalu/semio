import {join} from "node:path";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const root=process.env.SEMIO_WORKSPACE_ROOT!,generated=process.env.SEMIO_TEST_ARTIFACT_DIR!;
if(root!==process.cwd()||!generated||process.env.SEMIO_SCRIPT_PROCESS_INVOCATION!==undefined||process.argv.length!==2)throw Error("Standalone bounded private concurrency fixture required");
const {testResourceLeaseConcurrencyV1}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/⚔️concurrency/🟦️.ts"));
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"lease-concurrency-fixture",maximumElapsedMilliseconds:60000},{workspaceRoot:root},Date.now()),original=>testResourceLeaseConcurrencyV1(generated,original,createIssuedResourceControlV1(original,"lease:private-concurrency",0)));
