import assert from "node:assert/strict";
import {spawn} from "node:child_process";
import {mkdtemp,readFile,rm} from "node:fs/promises";
import {join,resolve} from "node:path";
import {createInterface} from "node:readline";
import {once} from "node:events";
import {setTimeout as delay} from "node:timers/promises";
import {takeMcpPluginCatalog,withMcpHostDriverPolicy} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import {serveWarmMcpGateway} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🚪️gateway/🟦️.ts";

if (process.argv[2] === "--gateway-child") {
  const [peer, ...args] = process.argv.slice(3);
  process.exitCode = await serveWarmMcpGateway({
    stage: async (progress, signal) => {
      progress("[DEBUG] native staging is waiting for its admission port");
      try { await delay(30_000, undefined, {signal}); }
      catch (error) {
        if (signal.aborted) progress("[DEBUG] outstanding staging was cancelled by client EOF");
        throw error;
      }
      return 0;
    },
    command: () => {
      process.stderr.write("[DEBUG] native peer selected\n");
      return peer!;
    },
    args, cwd:process.cwd(), env:process.env, input:process.stdin, output:process.stdout,
  });
} else await probe();

/** 🧪 Drives a real launcher or peer through the same bounded stdio EOF exchange. */
async function probe(): Promise<void> {
  const cwd = process.cwd();
  const {mcpServers:{semio:config}} = JSON.parse(await readFile(join(cwd, ".mcp.json"), "utf8"));
  const args = [...config.args];
  const [nativeExecutable, delayBeforeEof = "0", phase] = process.argv.slice(2);
  const gateway = phase === "--gateway";
  if (gateway) assert(nativeExecutable, "Real native peer required in gateway mode");
  const environment = {...process.env, ...config.env};
  delete environment.SEMIO_OS_MCP_BIN;
  const scratch = await mkdtemp(join(import.meta.dirname, "🗑️generated", "eof-"));
  args[args.indexOf("--folder") + 1] = scratch;
  const catalog = nativeExecutable && takeMcpPluginCatalog(args.slice(4), cwd);
  const nativeArgs = catalog ? withMcpHostDriverPolicy(["stdio", ...catalog.args]) : args;
  const child = spawn(gateway ? process.execPath : nativeExecutable ? resolve(nativeExecutable) : config.command,
    gateway ? [import.meta.path, "--gateway-child", resolve(nativeExecutable!), ...nativeArgs] : nativeArgs,
    {cwd,env:{...environment,...(catalog ? catalog.env : {})},stdio:["pipe","pipe","pipe"]});
  const lines = createInterface({input:child.stdout});
  const frames: Record<string, unknown>[] = [];
  const messages: string[] = [];
  const started = performance.now();
  let eofMilliseconds: number | undefined;
  child.stderr.setEncoding("utf8");
  child.stderr.on("data", (text: string) => messages.push(text));
  for (const signal of ["SIGINT", "SIGTERM"] as const) process.once(signal, () => child.kill(signal));
  const exit = once(child, "close");
  const timeout = setTimeout(() => child.kill("SIGKILL"), 20_000);
  try {
    lines.on("line", line => frames.push(JSON.parse(line)));
    child.stdin.write(JSON.stringify({jsonrpc:"2.0",id:1,method:"initialize",params:{protocolVersion:"2025-06-18",capabilities:{},clientInfo:{name:"eof-smoke",version:"1"}}}) + "\n");
    for (let i = 0; i < 200 && !frames.some(frame => frame.id === 1); i++) await delay(25);
    const response = frames.find(frame => frame.id === 1);
    if (response && child.exitCode === null) {
      child.stdin.write(JSON.stringify({jsonrpc:"2.0",method:"notifications/initialized"}) + "\n");
      await delay(Number(delayBeforeEof));
      eofMilliseconds = performance.now() - started;
      child.stdin.end();
    }
    const [code,signal] = await exit;
    const stderr = messages.join("");
    for (const line of stderr.split("\n")) if (line) console.error(line);
    const result = response?.result as {serverInfo?:unknown} | undefined;
    const stagingCancelled = stderr.includes("outstanding staging was cancelled by client EOF");
    const nativeSpawned = stderr.includes("real per-capability ArtifactChannel routing bound");
    console.log(JSON.stringify({mode:gateway ? "gateway-staging" : nativeExecutable ? "direct" : "configured",server:result?.serverInfo,code,signal,eofMilliseconds,elapsedMilliseconds:performance.now()-started,permissionDenied:/EPERM|Operation not permitted/u.test(stderr),nativeSpawned,stagingCancelled}));
    assert(response, "initialize response required");
    assert.notEqual(eofMilliseconds, undefined, "child must receive stdin EOF");
    assert.equal(code, 0);
    assert.equal(signal, null);
    if (gateway) {
      assert(stagingCancelled, "EOF must cancel the pending stage");
      assert(!stderr.includes("native peer selected"), "EOF must close without selecting the native peer");
    }
  } finally {
    clearTimeout(timeout);
    if (child.exitCode === null && child.signalCode === null) child.kill("SIGKILL");
    await rm(scratch, {recursive:true,force:true});
  }
}
