import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {pathToFileURL} from "node:url";
import {join,win32} from "node:path";
import fixture from "./windows-rustc-action-fixture.json" with {type:"json"};

const repo=join(process.cwd(),"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution"),moduleAt=(name:string)=>import(pathToFileURL(join(repo,name)).href);
const [{buildOffloadActionV1},{decodeProtobufV1,REAPI_V1},{validateRemoteExecutionRustcActionV1}]=await Promise.all([
 moduleAt("🚀️offload/📜️action/🟦️.ts"),moduleAt("📡️wire/📦️protobuf/🟦️.ts"),moduleAt("🧱️actions/🟦️.ts"),
]);

test("the Windows Rust action names a missing POSIX executable and cannot substitute an executable through the command",async()=>{
 const bytesByHash=new Map<string,Uint8Array>();
 const built=await buildOffloadActionV1({
  plan:{arguments:fixture.arguments,environment:{SEMIO_TOOL_RUSTC:"@tool",SEMIO_SOURCE:fixture.source,SEMIO_CWD_NAME:"src"},files:new Map()},
  content:{missing:async(_:string,rows:unknown[])=>rows,write:async(_:string,digest:{hash:string},bytes:Uint8Array)=>{bytesByHash.set(digest.hash,bytes);}},
  instance:"fixture",
  platform:[{name:"OSFamily",value:fixture.platform},{name:"semio-target",value:fixture.target}],
 });
 const action=decodeProtobufV1(REAPI_V1.action,bytesByHash.get(built.digest.hash));
 const command=decodeProtobufV1(REAPI_V1.command,bytesByHash.get(action.command_digest.hash)),launcher=command.arguments[0];
 expect(command.output_paths).toEqual(fixture.expected.outputs);
 expect(validateRemoteExecutionRustcActionV1(command)).toBeUndefined();
 expect(validateRemoteExecutionRustcActionV1({...command,arguments:[fixture.available[0],...command.arguments.slice(4)]})).toBe("only the declared rustc action may run on this worker");
 expect(launcher).toBe(fixture.expected.launcher);
 const model=spawnSync(process.platform==="win32"?"python":"python3",["-I","-c",`import json, ntpath, sys
launcher, root, installed=json.load(sys.stdin)
path=ntpath.normcase(ntpath.join(root,launcher))
installed=[ntpath.normcase(name) for name in installed]
print(json.dumps([path, any(path + suffix in installed for suffix in ('', '.exe', '.cmd', '.bat'))]))`],{input:JSON.stringify([launcher,fixture.root,fixture.available]),encoding:"utf8",timeout:10000});
 expect(model.status,model.stderr).toBe(0);
 expect(JSON.parse(model.stdout)).toEqual([fixture.expected.windowsLauncher,fixture.expected.launcherAvailable]);
 expect(win32.resolve(fixture.root,launcher).toLowerCase()).toBe(fixture.expected.windowsLauncher);
});
