import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { readFileSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { createInterface } from "node:readline";
import { InitializeResultSchema, JSONRPCMessageSchema, ListToolsResultSchema } from "@modelcontextprotocol/sdk/types.js";

const root = process.cwd(), output = join(import.meta.dirname, "🗑️generated");
mkdirSync(output, { recursive: true });
const configured = JSON.parse(readFileSync(join(root, ".mcp.json"), "utf8")).mcpServers.semio;
const args: string[] = [...configured.args];
if(process.argv.includes("--bare"))for(const flag of ["--catalog","--catalog-target"]){const index=args.indexOf(flag);if(index>=0)args.splice(index,2);}
const folder = args.indexOf("--folder");
assert.ok(folder >= 0);
args[folder + 1] = join(output, "smoke-space");
const child = spawn(configured.command, args, { cwd: root, env: { ...process.env, ...configured.env, SEMIO_TEST_ARTIFACT_DIR: output, S_AGENT_BRIDGE_DIR: join(output, "smoke-bridge") }, shell: false, stdio: ["pipe", "pipe", "pipe"] });
const pending = new Map<number, (value: any) => void>();
const rejecting=new Map<number,(error:Error)=>void>();
const lines: unknown[] = [];
let stderr = "", sequence = 0, protocolFailure:Error|undefined;
const readyMode=process.argv.includes("--ready"),timeoutAt=process.argv.indexOf("--timeout-ms"),discoveryTimeout=timeoutAt<0?600000:Number(process.argv[timeoutAt+1]);
assert.ok(Number.isSafeInteger(discoveryTimeout)&&discoveryTimeout>0&&discoveryTimeout<=1800000,"Finite discovery timeout up to1800000ms required");
let readyResult:unknown;
child.stderr.on("data", chunk => { stderr = (stderr + chunk.toString()).slice(-1048576); if(readyMode)process.stderr.write(chunk); });
createInterface({ input: child.stdout! }).on("line", line => {
  try {
    const message = JSONRPCMessageSchema.parse(JSON.parse(line));
    lines.push(message);
    if("id"in message&&!("method"in message))pending.get(message.id as number)?.(message);
  } catch(error) {
    protocolFailure=Error("Invalid protocol stdout: "+line.slice(0,200),{cause:error});
    lines.push({invalidStdout:line.slice(0,4096)});
    for(const reject of rejecting.values())reject(protocolFailure);
    child.stdin!.end();
  }
});
const terminal = new Promise(resolve => child.once("exit", (code, signal) => {for(const reject of rejecting.values())reject(Error("Server exited before response"));resolve({ code, signal });}));
const request = (method: string, params: unknown, maximumMilliseconds=30000) => new Promise<any>((resolve, reject) => {
  if(protocolFailure)return reject(protocolFailure);
  const id = ++sequence, timer = setTimeout(() => fail(new Error("MCP response deadline: " + method + "\n" + stderr)), maximumMilliseconds);
  const fail=(error:Error)=>{clearTimeout(timer);pending.delete(id);rejecting.delete(id);reject(error);};
  rejecting.set(id,fail);
  pending.set(id, value => { clearTimeout(timer); pending.delete(id); rejecting.delete(id); resolve(value); });
  child.stdin!.write(JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n");
});
const interrupt=()=>{for(const reject of rejecting.values())reject(Error("Smoke cancelled"));child.stdin!.end();};
for(const signal of ["SIGINT","SIGTERM"]as const)process.on(signal,interrupt);
try {
  const response = await request("initialize", { protocolVersion: "2025-11-25", capabilities: {}, clientInfo: { name: "semio-ticket-smoke", version: "1" } });
  assert.equal(response.error, undefined);
  const initialized = InitializeResultSchema.parse(response.result);
  assert.equal(initialized.serverInfo.name, "semio");
  child.stdin!.write(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }) + "\n");
  console.log(JSON.stringify({ initialized: true, server: initialized.serverInfo, nativeReady: initialized.serverInfo.version !== "warming" }));
  if(readyMode){
    const discovered=await request("server/discover",{},discoveryTimeout);
    assert.equal(discovered.error,undefined);
    assert.equal(discovered.result?.resultType,"complete");
    const native=InitializeResultSchema.parse(discovered.result);
    assert.ok(native.serverInfo.version.trim().length>0);
    assert.equal(native.protocolVersion,"2026-07-28");
    assert.equal(native.serverInfo.name,"semio");
    assert.notEqual(native.serverInfo.version,"warming");
    const listed=await request("tools/list",{});
    assert.equal(listed.error,undefined);
    const names=ListToolsResultSchema.parse(listed.result).tools.map(tool=>tool.name);
    for(const required of ["capabilities_search","context_resolve","artifact_create","artifact_open","artifact_validate","artifact_snapshot","action_prepare","action_invoke","inference_list","inference_submit","ui_focus","history_undo","history_redo"])assert.ok(names.includes(required),"Missing native tool: "+required);
    readyResult={nativeReady:true,server:native.serverInfo,tools:names};
    console.log(JSON.stringify(readyResult));
  }
} finally {
  for(const signal of ["SIGINT","SIGTERM"]as const)process.off(signal,interrupt);
  child.stdin!.end();
  const kill = setTimeout(() => child.kill("SIGKILL"), 15000);
  const exited = await terminal;
  clearTimeout(kill);
  const evidence=JSON.stringify({ readyMode, discoveryTimeout, readyResult, messages: lines, exited, stderr }, null, 2);
  writeFileSync(join(output, "smoke.json"), evidence);
  writeFileSync(join(output, "smoke-"+process.pid+".json"), evidence);
  console.log(JSON.stringify({ exited }));
}
