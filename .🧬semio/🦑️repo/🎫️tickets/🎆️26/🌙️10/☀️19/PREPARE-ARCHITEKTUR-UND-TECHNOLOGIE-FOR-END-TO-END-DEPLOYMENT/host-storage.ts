import {strict as assert} from "node:assert";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
const root=process.argv[2],pre=await import(pathToFileURL(resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️preflight/🟦️.ts")).href),storage=await import(pathToFileURL(resolve(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts")).href);
const host=resolve(root,".cache"),port=storage.nativeDirectorySystemPortV1();
storage.probeNativeDirectoryExchangeV1(host,port);
const capability=pre.probeBuildPreflightCaseV1(process.platform,port.kind,()=>storage.probeNativeDirectoryCaseCapabilityV1(host,port));
console.log("[DEBUG] "+JSON.stringify({platform:process.platform,port:port.kind,exchange:"supported",capability}));
assert.ok(capability==="yes"||process.platform==="win32"&&capability==="fold-safe");
