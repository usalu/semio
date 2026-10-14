import {resolve,join} from "node:path";
const root=process.env.SEMIO_WORKSPACE_ROOT??process.cwd(),library=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
if(process.env.SEMIO_SCRIPT_PROCESS_INVOCATION!==undefined)throw Error("Original standalone sibling fixture required");
if(process.env.SEMIO_TEST_ARTIFACT_DIR!==resolve(import.meta.dir,"../🗑️generated"))throw Error("Original ticket artifact root required");
const routing=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts")),{createBuildBootstrapCapabilitiesV1}=await import(join(library,"🏗️build/📜️script.ts")),{proveJavascriptDependencySiblingScopeV1}=await import(join(library,"⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📥️runtime/🟦️.ts"));
await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"sibling-dependency-staging-test",maximumElapsedMilliseconds:120000},await createBuildBootstrapCapabilitiesV1(root,["dependencies","lock","--all"],process.env),Date.now()),original=>proveJavascriptDependencySiblingScopeV1(root,original));
