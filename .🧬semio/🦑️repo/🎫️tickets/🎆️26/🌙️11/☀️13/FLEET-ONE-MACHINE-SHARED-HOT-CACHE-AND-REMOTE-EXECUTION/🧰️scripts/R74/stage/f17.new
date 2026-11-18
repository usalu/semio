import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
const worker=readFileSync(new URL("../../🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts",import.meta.url),"utf8");
test("the slice worker loads the graph inside the Native runtime entry scope, because an unmounted fixed storage captures its helper through that provider",()=>{
 const scope=worker.indexOf("withRepositoryCompilerRuntimeEntriesV1(workerData.root,original,control"),load=worker.indexOf("loadBuildInvocationGraphV1(workerData.root");
 expect(scope).toBeGreaterThan(-1);
 expect(load).toBeGreaterThan(scope);
});

test("the slice worker is committed to the code it loaded: it opts into the process-wide pin before it imports anything of the build and runs its whole load inside an implementation pin, so an edit of a loaded Source by a peer never ends a running load with the restart code",()=>{
 const pin=worker.indexOf("useProcessWideImplementationPinV1();"),firstImport=worker.indexOf('await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts")'),scope=worker.indexOf("withImplementationPinV1(()=>routing.receiveScriptProcessInvocation("),load=worker.indexOf("loadBuildInvocationGraphV1(workerData.root");
 expect(pin).toBeGreaterThan(-1);expect(firstImport).toBeGreaterThan(pin);
 expect(scope).toBeGreaterThan(pin);expect(load).toBeGreaterThan(scope);
 expect(worker).toContain('📥️inputs/🔏️revision/🟨️.mjs');
});
