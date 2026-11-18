import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
const worker=readFileSync(new URL("../../🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts",import.meta.url),"utf8");
test("the slice worker loads the graph inside the Native runtime entry scope, because an unmounted fixed storage captures its helper through that provider",()=>{
 const scope=worker.indexOf("withRepositoryCompilerRuntimeEntriesV1(workerData.root,original,control"),load=worker.indexOf("loadBuildInvocationGraphV1(workerData.root");
 expect(scope).toBeGreaterThan(-1);
 expect(load).toBeGreaterThan(scope);
});
