import {expect,test} from "bun:test";
import * as execution from "../../../../🔧️tools/🏗️bazel/🏃️execution/🟦️.ts";

const host={platform:process.platform,arch:process.arch};
const flags=(environment:Record<string,string>)=>execution.bazelExecutionFlagsV1({host,hermetic:true,jobs:3,environment});

test("with an executor the Bazel flags name its instance, switch compression off and keep the strategy list that includes remote; without one nothing remote is emitted",()=>{
 const local=execution.bazelExecutionFlagsV1({host,hermetic:true,jobs:3});
 expect(local.remote).toBe(false);expect(local.command.some(flag=>/remote_/u.test(flag))).toBe(false);
 const remote=flags({SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://127.0.0.1:8980"});
 expect(remote.remote).toBe(true);expect(remote.command).toContain("--remote_executor=grpc://127.0.0.1:8980");expect(remote.command).toContain("--noremote_local_fallback");expect(remote.command).toContain("--remote_instance_name=main");expect(remote.command).toContain("--noremote_cache_compression");
 expect(flags({SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpcs://build.example:443",SEMIO_REMOTE_EXECUTION_INSTANCE:"team-a"}).command).toContain("--remote_instance_name=team-a");
 expect(execution.bazelRemoteExecutionV1({SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://h:1"})?.instance).toBe("main");
 expect(()=>flags({SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://h:1",SEMIO_REMOTE_EXECUTION_INSTANCE:"Bad Name"})).toThrow();
 expect(new Set(remote.command).size).toBe(remote.command.length);
});

test("an HTTP cache cannot stay beside a gRPC executor: the cache endpoint is dropped, the credential helper moves to the executor host and every other cache flag survives",()=>{
 const cache=["--remote_cache=http://cache.example:6090","--remote_instance_name=main","--credential_helper=cache.example=/work/helper.sh","--remote_upload_local_results","--remote_download_outputs=toplevel","--remote_timeout=30s"];
 expect(execution.bazelExecutorCacheFlagsV1(cache,"grpc://127.0.0.1:8980")).toEqual(["--remote_instance_name=main","--credential_helper=127.0.0.1=/work/helper.sh","--remote_upload_local_results","--remote_download_outputs=toplevel","--remote_timeout=30s"]);
 expect(execution.bazelExecutorCacheFlagsV1(cache,"grpcs://[::1]:8980")).toContain("--credential_helper=::1=/work/helper.sh");
 expect(execution.bazelExecutorCacheFlagsV1(cache,"unix:/run/exec.sock")).toContain("--credential_helper=localhost=/work/helper.sh");
 expect(execution.bazelExecutorCacheFlagsV1(["--remote_cache=grpc://other:1","--remote_cache=grpcs://other:2"],"grpc://h:1")).toEqual(["--remote_cache=grpc://other:1","--remote_cache=grpcs://other:2"]);
 expect(execution.bazelExecutorCacheFlagsV1([],"grpc://h:1")).toEqual([]);
});
