import assert from "node:assert/strict";
import { appendFileSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { connect } from "node:net";
import { execFileSync } from "node:child_process";

const root = process.cwd(), ticket = resolve(import.meta.dir, ".."), generated = join(ticket, "🗑️generated");
const tree = JSON.parse(readFileSync(join(generated, "command-tree.json"), "utf8"));
const [renderer = "react", example = "🎬️demo", ...extra] = process.argv.slice(2), resume = extra[0] === "--resume" ? extra.splice(0, 2)[1] : undefined, id = resume ?? `dashboard-real-${renderer}-${Date.now()}`;
const menuRenderer=renderer==="native-smoke"?"wgpu-native":renderer;
let node = tree;
if (renderer === "target") {
  function find(branch) { return branch.leaf?.args?.includes(example) ? branch : (branch.children ?? []).map(find).find(Boolean); }
  node = find(tree);
  if (!node) {
    const graph = JSON.parse(readFileSync(join(root, ".nx/workspace-data/project-graph.json"), "utf8")), split = example.lastIndexOf(":"), project = example.slice(0,split), target = example.slice(split+1);
    assert.ok(graph.nodes[project]?.data.targets[target], `Missing actual Nx target ${example}`);
    node = {leaf:{cmd:"bun",args:["nx","run",example],env:[]}};
    console.log(`[DEBUG] verified Nx inferred target ${example}`);
  }
} else for (const key of ["dev", "draw", "draw", menuRenderer, ...(example === "all" ? ["all"] : ["examples", example]), "language", "en", "terminology", "native"]) { node = node.children.find(child => child.key === key); assert.ok(node, `Missing ${key}`); }
if(renderer==="native-smoke"){const target="@semio-tech/framework-os-dev:smoke-draw-native-dev",graph=JSON.parse(readFileSync(join(root,".nx/workspace-data/project-graph.json"),"utf8"));assert.ok(graph.nodes["@semio-tech/framework-os-dev"].data.targets["smoke-draw-native-dev"]);node={leaf:{...node.leaf,args:["nx","run",target,"--verbose","--output-style=stream","--","--example",example]}};}
const leaf = node.leaf, command = { cmd: leaf.cmd, args: [...leaf.args, ...extra], cwd: root, env: [...leaf.env.map(value => [value.name, value.value]), ...Object.entries(process.env).filter(([key]) => key.startsWith("SEMIO_TEST_") || key === "CARGO_BUILD_JOBS" || key === "RUST_MIN_STACK" || key === "RUSTC_WRAPPER" || key === "SEMIO_RUNTIME_DIAGNOSTICS")], cols: 160, rows: 42 };
let buffer = Buffer.alloc(0), session, text = "", daemon, disconnected = false;
const log = join(generated, `${id}.log`), socket = connect(readFileSync(join(root, ".🧬semio/🦑️repo/⚡️cache/🎛️dashboard/daemon.pipe.name"), "utf8").trim());
function send(message): void { const payload = Buffer.from(JSON.stringify(message)), prefix = Buffer.alloc(5); prefix.writeUInt32LE(payload.length + 1); prefix[4] = 1; socket.write(Buffer.concat([prefix, payload])); }
socket.on("error", error => { disconnected = true; console.error(error); });
socket.on("close", () => { disconnected = true; });
socket.on("data", chunk => {
  buffer = Buffer.concat([buffer, chunk]);
  while (buffer.length >= 4 && buffer.length >= buffer.readUInt32LE(0) + 4) {
    const size = buffer.readUInt32LE(0), kind = buffer[4], payload = buffer.subarray(5, size + 4); buffer = buffer.subarray(size + 4);
    if (kind === 1) {
      const message = JSON.parse(payload.toString());
      if (message.type === "attached") daemon = message.daemon_pid;
      if (message.type === "sessions") session = message.sessions.find(value => value.session_id === id) ?? session;
      if (message.type === "session_changed" && message.session.session_id === id) { session = message.session; console.log(`[DEBUG] ${id} ${session.status} pid=${session.pid} exit=${session.code}`); }
      if (message.type === "error") throw new Error(message.message);
    } else if (kind === 2) {
      const length = payload.readUInt16LE(0);
      if (payload.subarray(2, length + 2).toString() === id) { const output = payload.subarray(length + 2).toString(); appendFileSync(log, output); text = (text + output).slice(-256 * 1024); }
    }
  }
});
async function until(predicate, limit: number, label: string): Promise<void> {
  const started = Date.now(); let progress = 0;
  while (!await predicate()) {
    assert.ok(!disconnected, "daemon disconnected");
    if (Date.now() - started > limit) throw new Error(`${label} timed out\n${text.slice(-5000)}`);
    if (Date.now() - progress > 20000) { console.log(`[DEBUG] ${label} elapsed=${Math.round((Date.now() - started) / 1000)}s ${text.slice(-350).replace(/\s+/g, " ")}`); progress = Date.now(); }
    await new Promise(accept => setTimeout(accept, 500));
  }
}
async function ready(): Promise<boolean> {
  if (renderer === "target" || renderer === "native-smoke") {
    if (session && ["exited", "failed"].includes(session.status)) { assert.equal(session.code, 0, text.slice(-8000)); return true; }
    return false;
  }
  if (session && ["exited", "failed"].includes(session.status)) throw new Error(`Real ${renderer} task exited ${session.code}\n${text.slice(-8000)}`);
  if (renderer === "wgpu-native") {
    assert.ok(!text.includes("boot_runtime failed:"),text.slice(-4000));
    if (!session?.pid || !text.includes("native renderer boot ready")) return false;
    const rows = JSON.parse(execFileSync("powershell.exe", ["-NoProfile", "-NonInteractive", "-Command", "Get-CimInstance Win32_Process | ForEach-Object { $p = Get-Process -Id $_.ProcessId -ErrorAction SilentlyContinue; [pscustomobject]@{ pid=$_.ProcessId; parent=$_.ParentProcessId; window=if($p -and $p.MainWindowHandle){$p.MainWindowHandle.ToInt64()}else{0}; title=$p.MainWindowTitle } } | ConvertTo-Json -Compress"], { encoding: "utf8", windowsHide: true }));
    const owned = new Set([session.pid]); let changed = true;
    while (changed) { changed = false; for (const row of rows) if (owned.has(row.parent) && !owned.has(row.pid)) { owned.add(row.pid); changed = true; } }
    const window = rows.find(row => owned.has(row.pid) && row.window > 0);
    if (window) console.log(`[DEBUG] actual native window pid=${window.pid} hwnd=${window.window} title=${window.title}`);
    assert.ok(!text.includes("boot_runtime failed:"),text.slice(-4000));
    return Boolean(window) && text.includes("native renderer boot ready");
  }
  const port = command.env.find(([key]) => key === "S_OS_PORT")[1];
  try { const response = await fetch(`http://127.0.0.1:${port}/`, { signal: AbortSignal.timeout(2000) }); return response.ok && (await response.text()).includes("<html"); } catch { return false; }
}
async function browserRuntime(): Promise<void> {
  const { chromium } = await import("@playwright/test");
  const browser = await chromium.launch({ channel: "chrome", headless: true, ...(renderer==="wgpu-wasm"?{args:["--enable-unsafe-webgpu","--ignore-gpu-blocklist",`--use-angle=${process.platform==="win32"?"d3d11":process.platform==="darwin"?"metal":"vulkan"}`]}:{}) });
  const page = await browser.newPage(), messages: string[] = [], errors: string[] = [];
  page.on("console", message => { messages.push(message.type()+": "+message.text()); if(message.type()==="error") errors.push(message.text()); });
  page.on("pageerror", error => errors.push(error.message));
  try {
    const port = command.env.find(([key]) => key === "S_OS_PORT")[1];
    await page.goto(`http://127.0.0.1:${port}/`, {waitUntil:"domcontentloaded"});
    await page.waitForFunction(() => document.documentElement.dataset.semioOsReady || document.documentElement.dataset.semioOsError || document.documentElement.dataset.semioOsNotFound, undefined, {timeout:120000});
    const state = await page.evaluate(() => ({ready:document.documentElement.dataset.semioOsReady,error:document.documentElement.dataset.semioOsError,missing:document.documentElement.dataset.semioOsNotFound,body:document.body.innerText.slice(0,2000)}));
    console.log(`[DEBUG] real browser runtime ${JSON.stringify(state)}`);
    assert.ok(state.ready, JSON.stringify({state,errors}));
    assert.deepEqual(errors.filter(message => !/favicon|404/.test(message)), []);
  } finally { appendFileSync(log, "\n[DEBUG] browser console\n"+messages.join("\n")+"\n"+errors.join("\n")); await browser.close(); }
}

try {
  await until(() => Boolean(daemon), 10000, "attach");
  send({ type: "attach", client_id: id });
  send(resume ? { type: "list" } : { type: "spawn", session_id: id, command });
  await until(ready, 3600000, "real renderer readiness");
  if (renderer === "react" || renderer === "wgpu-wasm") await browserRuntime();
  if(renderer==="native-smoke"){const plain=text.replace(/\x1b\[[0-9;?]*[A-Za-z]/g, "");const match=plain.replace(/@semio-tech\/framework-os-dev:\s*/g,"").match(/\{\s*"booted"\s*:\s*true,[\s\S]*?"windowDocuments"\s*:\s*\[[\s\S]*?\]\s*\}/);assert.ok(match,plain.slice(-6000));const report=JSON.parse(match[0]);assert.equal(report.booted,true);assert.equal(report.session?.pluginId,"draw");console.log(`[DEBUG] native smoke ${JSON.stringify(report)}`);}
  const first = session.pid; assert.ok(first);
  console.log(`[DEBUG] real ${renderer} ${example} ${renderer === "wgpu-native" ? "window opened" : "ready"} through daemon ${daemon}, task ${first}`);
  if (renderer !== "target" && renderer !== "native-smoke") {
    text="";
    send({ type: "restart", session_id: id });
    await until(() => session?.status === "running" && session.pid !== first, 20000, "restart pid");
    await until(ready, 3600000, "restart readiness");
    if(renderer!=="wgpu-native")await browserRuntime();
    console.log(`[DEBUG] real renderer restarted ${first} -> ${session.pid}`);
  }
} finally {
  if (session && !["exited", "failed"].includes(session.status)) { send({ type: "stop", session_id: id }); await until(() => ["exited", "failed"].includes(session?.status), 20000, "stop"); }
  send({ type: "detach" }); socket.end();
}
