import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { appendFileSync, existsSync, lstatSync, mkdirSync, readdirSync, readFileSync, rmdirSync, unlinkSync, writeFileSync } from "node:fs";
import { join, resolve, sep } from "node:path";

const ticket = resolve(import.meta.dir, ".."), generated = join(ticket, "🗑️generated", "oct8");
if (process.argv[2] === "purge-generated") {
  const owned = join(ticket, "🗑️generated");
  assert.equal(resolve(owned), owned);
  assert.ok(!lstatSync(ticket).isSymbolicLink(), "the owned ticket root is a real directory");
  assert.ok(owned.startsWith(ticket + sep));
  const directories: string[] = [], files: string[] = [], queue = [owned];
  while (queue.length) {
    const folder = queue.pop()!;
    if (!existsSync(folder)) continue;
    const metadata = lstatSync(folder);
    assert.ok(metadata.isDirectory() && !metadata.isSymbolicLink(), `owned real directory: ${folder}`);
    directories.push(folder);
    for (const entry of readdirSync(folder, { withFileTypes: true })) {
      const path = join(folder, entry.name);
      assert.ok(path.startsWith(owned + sep));
      assert.ok(!entry.isSymbolicLink(), `no generated links: ${path}`);
      if (entry.isDirectory()) queue.push(path); else files.push(path);
    }
  }
  console.log(`[DEBUG] bounded cleanup owns ${files.length} files and ${directories.length} directories`);
  for (const [index, file] of files.entries()) {
    const metadata = lstatSync(file);
    assert.ok(metadata.isFile() && !metadata.isSymbolicLink(), `owned regular output: ${file}`);
    unlinkSync(file);
    if (index % 1000 === 0) { console.log(`[DEBUG] removed ${index + 1}/${files.length} generated files`); await Bun.sleep(0); }
  }
  for (const folder of directories.reverse()) rmdirSync(folder);
  assert.ok(!existsSync(owned));
  console.log("[DEBUG] ticket-generated tree is absent");
  process.exit(0);
}
mkdirSync(generated, { recursive: true });
const temporary = join(generated, "temporary"); mkdirSync(temporary, { recursive: true });
process.env.TMP = temporary; process.env.TEMP = temporary; process.env.TMPDIR = temporary;
if (process.argv[2] === "freshness") {
  const root = resolve(ticket, "../../../../../../..");
  const installation = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️installation/🟦️.ts"));
  const reason = await installation.staleDashboard(root);
  console.log(`[DEBUG] final dashboard installation freshness: ${reason ?? "current"}`);
  if (reason) {
    const native = await import(join(installation.dashboardPackageRoot, "📜️script.ts"));
    await native.buildAndInstallDashboard(installation.dashboardPackageRoot, root);
  }
  const after = await installation.staleDashboard(root);
  assert.equal(after, undefined);
  writeFileSync(join(generated, "freshness-receipt.json"), JSON.stringify({ before: reason ?? null, after: after ?? null, binary: installation.installedDashboard(root) }));
  console.log(`[DEBUG] current installed dashboard ${installation.installedDashboard(root)}`);
  process.exit(0);
}
if (process.argv[2] === "console-records") {
  const { dlopen, FFIType, ptr } = await import("bun:ffi");
  const native = dlopen("kernel32.dll", {
    GetStdHandle: { args: [FFIType.i32], returns: FFIType.ptr },
    GetConsoleMode: { args: [FFIType.ptr, FFIType.ptr], returns: FFIType.i32 },
    SetConsoleMode: { args: [FFIType.ptr, FFIType.u32], returns: FFIType.i32 },
    ReadConsoleInputW: { args: [FFIType.ptr, FFIType.ptr, FFIType.u32, FFIType.ptr], returns: FFIType.i32 },
  });
  const handle = native.symbols.GetStdHandle(-10), mode = new Uint32Array(1), count = new Uint32Array(1), records = new Uint8Array(20 * 128);
  assert.ok(native.symbols.GetConsoleMode(handle, ptr(mode)), "console input exists");
  assert.ok(native.symbols.SetConsoleMode(handle, (mode[0]! & ~(1 | 2 | 4)) | 0x200), "raw virtual terminal input");
  console.log("[DEBUG] console record oracle ready");
  try {
    while (native.symbols.ReadConsoleInputW(handle, ptr(records), 128, ptr(count))) {
      const view = new DataView(records.buffer);
      for (let index = 0; index < count[0]!; index++) {
        const offset = index * 20;
        if (view.getUint16(offset, true) !== 1) continue;
        const value = { down: view.getInt32(offset + 4, true), key: view.getUint16(offset + 10, true), unit: view.getUint16(offset + 14, true), control: view.getUint32(offset + 16, true) };
        console.log(`[DEBUG] console record ${JSON.stringify(value)}`);
        if (value.unit === 3) process.exit(0);
      }
    }
  } finally { native.symbols.SetConsoleMode(handle, mode[0]!); native.close(); }
  process.exit(0);
}
if (process.argv[2] === "probe") { console.log(`[DEBUG] script probe ${JSON.stringify({ cwd: process.cwd(), args: process.argv.slice(3), marker: process.env.OCT8_PROBE })}`); process.exit(0); }
if (process.argv[2] === "script-run") {
  console.log("[DEBUG] verifying the declarative script entry through an isolated workspace daemon");
  const root = resolve(ticket, "../../../../../../.."), installed = JSON.parse(readFileSync(join(root, ".🧬semio/🦑️repo/⚡️cache/🎛️dashboard/installed.json"), "utf8"));
  const runtime = join(generated, "script-runtime"); mkdirSync(runtime, { recursive: true });
  const env = { ...process.env, SEMIO_DASHBOARD_INSTANCE: "oct8-script-entry", SEMIO_DASHBOARD_RUNTIME_DIR: runtime };
  delete env.NX_WORKSPACE_ROOT_PATH; delete env.NX_WORKSPACE_ROOT;
  env.NX_WORKSPACE_DATA_DIRECTORY = join(generated, "script-fixture-nx-data"); env.NX_CACHE_DIRECTORY = join(generated, "script-fixture-nx-cache");
  const cli = async (args: string[]) => {
    const child = Bun.spawn([join(root, installed.path), ...args], { cwd: root, env, stdin: "ignore", stdout: "pipe", stderr: "pipe" });
    const timer = setTimeout(() => child.kill(), 120000);
    const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]); clearTimeout(timer);
    appendFileSync(join(generated, "script-cli.jsonl"), JSON.stringify({ args, code, stdout, stderr }) + "\n");
    assert.equal(code, 0, `${args.join(" ")}: ${stderr}`); return stdout;
  };
  const prefix = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/🧪️oct8";
  if (process.argv[3] === "logs") {
    for (const task of JSON.parse(await cli(["tasks", "--json"]))) { const logs = await cli(["logs", task.session]); writeFileSync(join(generated, "script-diagnosis.txt"), logs); console.log(`[DEBUG] authored-script diagnostic logs:\n${logs}`); }
    process.exit(0);
  }
  const selection = ["tool:workspace/run-script", "--param", `script=${prefix}/📜️script.ts`, "--param", "project=dashboard-authority-verification", "--param", `directory=${prefix}/🧰️nx`, "--env", "OCT8_PROBE=explicit", "--", "probe", "one argument with spaces"];
  const dry = JSON.parse(await cli(["run", selection[0]!, "--dry-run", ...selection.slice(1)]));
  assert.equal(dry.processes[0].cwd.replaceAll("\\", "/"), `${prefix}/🧰️nx`);
  assert.equal(resolve(dry.processes[0].args.at(-3)), join(root, prefix, "📜️script.ts"));
  assert.deepEqual(dry.processes[0].args.slice(-2), ["probe", "one argument with spaces"]);
  try {
    const session = (await cli(["run", selection[0]!, "--detach", ...selection.slice(1)])).split("\t")[0]!.trim();
    const deadline = Date.now() + 60000;
    let task: any;
    while (Date.now() < deadline) {
      const tasks = JSON.parse(await cli(["tasks", "--json"])); task = tasks.find((task: any) => task.session === session);
      if (task && !["starting", "running", "stopping"].includes(task.status)) break;
      await Bun.sleep(250);
    }
    assert.ok(task, "the authored script is listed by the daemon"); assert.equal(task.status, "exited", JSON.stringify(task)); assert.equal(task.code, 0, JSON.stringify(task));
    const logs = await cli(["logs", task.session]);
    assert.ok(logs.includes('"marker":"explicit"') && logs.includes('"args":["one argument with spaces"]'), logs);
    assert.ok(logs.replaceAll("\\\\", "/").includes(`${prefix}/🧰️nx`), logs);
    console.log(`[DEBUG] declarative script resolved, started and exited under one isolated daemon: ${task.session}`);
  } finally { await cli(["daemon", "stop"]); }
  process.exit(0);
}
if (process.argv[2] === "ui") {
  const root = resolve(ticket, "../../../../../../..");
  delete process.env.NX_WORKSPACE_ROOT_PATH; delete process.env.NX_WORKSPACE_ROOT;
  console.log(`[DEBUG] shared UI verification ${process.argv.slice(3).join(" ")}`);
  const { runRepositoryCargoTests } = await import("../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts");
  await runRepositoryCargoTests(["semio-framework-ui"], root, ["--features", "tui-terminal", "--lib", ...process.argv.slice(3)]);
  process.exit(0);
}
if (process.argv[2] === "battle") {
  const root = resolve(ticket, "../../../../../../.."), dashboard = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard");
  const installed = JSON.parse(readFileSync(join(root, ".🧬semio/🦑️repo/⚡️cache/🎛️dashboard/installed.json"), "utf8"));
  const temporary = join(generated, "battle-temporary"); mkdirSync(temporary, { recursive: true });
  const env = { ...process.env, SEMIO_TEST_CLI: join(root, installed.path), SEMIO_COVERAGE_FULL: "1", SEMIO_SMOKE_INSTANCE: process.env.SEMIO_SMOKE_INSTANCE ?? "oct8-control-plane-smoke", SEMIO_TEST_ARTIFACT_DIR: join(generated, "battle-artifacts"), TMP: temporary, TEMP: temporary, TMPDIR: temporary };
  delete env.NX_WORKSPACE_ROOT; delete env.NX_WORKSPACE_ROOT_PATH;
  env.NX_WORKSPACE_DATA_DIRECTORY = join(generated, "real-workspace-nx-data"); env.NX_CACHE_DIRECTORY = join(generated, "real-workspace-nx-cache");
  const file = { coverage: "🗺️coverage/🟦️.ts", journeys: "🧭️journeys/🟦️.ts", load: "🧭️journeys/🏋️load/🟦️.ts", smoke: "🧭️journeys/💨️smoke/🟦️.ts" }[process.argv[3] ?? ""];
  assert.ok(file, "choose an owned battle suite");
  const child = Bun.spawn([process.execPath, "test", join(dashboard, "🧪️tests", file), "--timeout", "3600000", ...(process.argv[4] ? ["--test-name-pattern", process.argv[4]] : [])], { cwd: root, env, stdin: "ignore", stdout: "pipe", stderr: "pipe" });
  const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  writeFileSync(join(generated, `battle-${process.argv[3]}-${process.argv[4] ?? "all"}-captured.log`), `${stdout}\n${stderr}\n[DEBUG] battle exit=${code}\n`);
  console.log(stdout); console.error(stderr); console.log(`[DEBUG] unsampled battle ${process.argv[3]} exit=${code}`);
  assert.match(stdout + stderr, /[1-9]\d* (?:pass|fail)\b/, "selected battle scenarios actually ran"); process.exit(code);
}
if (process.argv[2] === "pty" || process.argv[2] === "pty-load") {
  const root = resolve(ticket, "../../../../../../.."), dashboard = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard");
  const installed = JSON.parse(readFileSync(join(root, ".🧬semio/🦑️repo/⚡️cache/🎛️dashboard/installed.json"), "utf8"));
  const temporary = join(generated, "pty-temporary"); mkdirSync(temporary, { recursive: true });
  const env = { ...process.env, CARGO_TARGET_DIR: join(generated, "pty-target"), SEMIO_TEST_CLI: join(root, installed.path), SEMIO_TEST_ARTIFACT_DIR: join(generated, "pty-artifacts"), TMP: temporary, TEMP: temporary, TMPDIR: temporary };
  delete env.NX_WORKSPACE_ROOT_PATH; delete env.NX_WORKSPACE_ROOT;
  env.NX_WORKSPACE_DATA_DIRECTORY = join(generated, "real-workspace-nx-data"); env.NX_CACHE_DIRECTORY = join(generated, "real-workspace-nx-cache");
  const manifest = join(dashboard, "🧪️tests/🧭️journeys/📦️packages/🦀️rust/Cargo.toml");
  const suite = process.argv[2] === "pty-load" ? "load" : "journeys";
  const output = join(generated, "pty-compile.jsonl");
  const compiler = Bun.spawn(["cargo", "test", "--manifest-path", manifest, "--test", suite, "--no-run", "--locked", "--message-format=json"], { cwd: root, env, stdin: "ignore", stdout: Bun.file(output), stderr: "inherit" });
  const built = await compiler.exited; console.log(`[DEBUG] PTY compile exit=${built}`); if (built) process.exit(built);
  const executable = readFileSync(output, "utf8").split("\n").flatMap(line => { try { const row = JSON.parse(line); return row.executable && row.target?.name === suite && row.target?.kind?.includes("test") ? [row.executable as string] : []; } catch { return []; } }).at(-1);
  assert.ok(executable, "the journey test executable was produced");
  const filter = process.argv[2] === "pty-load" ? "all" : process.argv[3] ?? "launcher_text_editing_follows_the_visible_hardware_cursor";
  const label = process.argv[2] === "pty-load" ? "load" : filter;
  const child = Bun.spawn([executable, ...(filter === "all" ? [] : [filter, "--exact"]), "--nocapture", "--test-threads=1"], { cwd: root, env, stdin: "ignore", stdout: "pipe", stderr: "pipe" });
  const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  writeFileSync(join(generated, `pty-${label}.log`), `${stdout}\n${stderr}\n[DEBUG] independent PTY ${filter} exit=${code}\n`);
  console.log(stdout); console.error(stderr); console.log(`[DEBUG] independent PTY ${filter} exit=${code}`);
  assert.match(stdout, /running [1-9]\d* tests?/, "the selected journey actually ran"); process.exit(code);
}
if (process.argv[2] === "inspect") {
  const root = resolve(ticket, "../../../../../../.."), installed = JSON.parse(readFileSync(join(root, ".🧬semio/🦑️repo/⚡️cache/🎛️dashboard/installed.json"), "utf8"));
  const env = { ...process.env }; delete env.NX_WORKSPACE_ROOT_PATH; delete env.NX_WORKSPACE_ROOT;
  env.NX_WORKSPACE_DATA_DIRECTORY = join(generated, "real-workspace-nx-data"); env.NX_CACHE_DIRECTORY = join(generated, "real-workspace-nx-cache");
  const bin = join(root, installed.path);
  for (const [file, args] of [["registry.json", ["commands", "--json", "--all"]], ["registry-check.txt", ["commands", "--check"]]] as const) {
    console.log(`[DEBUG] inspecting ${args.join(" ")}`);
    const child = Bun.spawn([bin, ...args], { cwd: root, env, stdin: "ignore", stdout: Bun.file(join(generated, file)), stderr: "inherit" });
    const code = await child.exited; console.log(`[DEBUG] inspection exit=${code}`); if (code) process.exit(code);
  }
  process.exit(0);
}
if (process.argv[2] === "lock") {
  const root = resolve(ticket, "../../../../../../.."), relay = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts");
  const env = { ...process.env }; delete env.NX_WORKSPACE_ROOT_PATH; delete env.NX_WORKSPACE_ROOT;
  env.NX_WORKSPACE_DATA_DIRECTORY = join(generated, "real-workspace-nx-data"); env.NX_CACHE_DIRECTORY = join(generated, "real-workspace-nx-cache");
  const child = Bun.spawn([process.execPath, relay, "relay", "metadata", "--offline", "--format-version=1", "--manifest-path", join(root, "Cargo.toml")], { cwd: root, env, stdin: "ignore", stdout: Bun.file(join(generated, "cargo-metadata.json")), stderr: "inherit" });
  const code = await child.exited; console.log(`[DEBUG] offline workspace metadata exit=${code}`); process.exit(code);
}
if (process.argv[2] === "verify") {
  const root = resolve(ticket, "../../../../../../..");
  console.log(`[DEBUG] verifying ${process.argv.slice(3).join(" ")}`);
  const env = { ...process.env }; delete env.NX_WORKSPACE_ROOT_PATH; delete env.NX_WORKSPACE_ROOT;
  env.NX_WORKSPACE_DATA_DIRECTORY = join(generated, "real-workspace-nx-data"); env.NX_CACHE_DIRECTORY = join(generated, "real-workspace-nx-cache");
  const child = Bun.spawn([process.execPath, join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📜️script.ts"), "test", ...process.argv.slice(3)], { cwd: root, env, stdin: "inherit", stdout: "inherit", stderr: "inherit" });
  const code = await child.exited; console.log(`[DEBUG] verification child exit=${code}`); process.exit(code);
}
if (process.argv[2] === "audit") {
  const { existsSync } = await import("node:fs"), { parse } = await import("jsonc-parser");
  const mapping = JSON.parse(readFileSync(join(ticket, "r2-m2-mapping.json"), "utf8"));
  const root = JSON.parse(readFileSync("package.json", "utf8"));
  const { readShell } = await import("../ticket-commands.ts");
  const manifests = [...new Bun.Glob("**/🎮️commands.json").scanSync({ cwd: ".🧬semio/🦑️repo/🎫️tickets", onlyFiles: true, followSymlinks: false })].filter(path => !path.includes("🗑️generated"));
  const declarations = manifests.map(path => ({ path, document: JSON.parse(readFileSync(join(".🧬semio/🦑️repo/🎫️tickets", path), "utf8")) }));
  const normalize = (value: string) => value.replaceAll("${workspaceFolder}", "{workspace}").replaceAll("\\", "/");
  const ticketStatuses = new Map<string, string>();
  const statusOf = (script: string) => {
    if (!script.startsWith("{workspace}/.🧬semio/🦑️repo/🎫️tickets/")) return "source";
    const folder = script.slice("{workspace}/".length).split("/").slice(0, 7).join("/");
    if (!ticketStatuses.has(folder)) { const document = join(folder, "🎫️ticket.json"); ticketStatuses.set(folder, existsSync(document) ? JSON.parse(readFileSync(document, "utf8")).status : "no ticket"); }
    return ticketStatuses.get(folder);
  };
  const namedScripts = (values: string[]) => values.map(normalize).filter(value => /[.](?:ts|py|sh|ps1)$/.test(value));
  const knownScripts = new Set(declarations.flatMap(({ document }) => (document.tools ?? []).flatMap((tool: any) => namedScripts([...(tool.command ?? []), ...(tool.parameters ?? []).flatMap((parameter: any) => (parameter.values ?? []).flatMap((value: any) => value.args ?? []))]))));
  const rows = [".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc", ".claude/launch.json"].filter(existsSync).map(path => {
    const document = parse(readFileSync(path, "utf8")), configurations = document.configurations ?? document.servers ?? [];
    const known = new Set(Object.values(mapping).flatMap((value: any) => Array.isArray(value) ? value.map(row => row.original) : []));
    const scriptGaps = new Map<string, string[]>();
    for (const row of configurations) for (const script of namedScripts(readShell(row.command ?? "").argv)) if (!knownScripts.has(script)) scriptGaps.set(script, [...(scriptGaps.get(script) ?? []), row.name]);
    return { path, count: configurations.length, scriptGaps: [...scriptGaps].map(([script, names]) => ({ script, status: statusOf(script), exists: existsSync(script.replace("{workspace}", process.cwd())), count: names.length, examples: names.slice(0, 3) })), unmapped: configurations.filter((row: any) => !known.has(row.name)).map((row: any) => ({ name: row.name, command: row.command, cwd: row.cwd, env: row.env })) };
  });
  writeFileSync(join(generated, "current-authority.json"), JSON.stringify({ rootScripts: root.scripts, rows }, null, 2));
  console.log(JSON.stringify({ rootScriptCount: Object.keys(root.scripts).length, rows: rows.map(({ unmapped, scriptGaps, ...row }) => ({ ...row, presentScriptGaps: scriptGaps.filter(row => row.exists), deadScriptPaths: scriptGaps.filter(row => !row.exists).length, unmappedCount: unmapped.length })), ticketManifests: manifests.length, declaredScriptPaths: knownScripts.size, mappingKeys: Object.keys(mapping) }, null, 2));
  process.exit(0);
}
const mcpRoot = resolve(ticket, "../../../../../../..");
const child = spawn(process.execPath, [join(mcpRoot, "📜️script.ts"), "dev", "mcp", "stdio", "codex"], { cwd: mcpRoot, stdio: ["pipe", "pipe", "pipe"], windowsHide: true });
const pending = new Map<number, { resolve: (value: any) => void; reject: (error: Error) => void }>();
let buffer = "", sequence = 0;
const exited = new Promise<number | null>(accept => child.once("exit", accept));
child.stderr.on("data", chunk => appendFileSync(join(generated, "mcp.stderr.log"), chunk));
child.stdout.on("data", chunk => {
  appendFileSync(join(generated, "mcp.stdout.log"), chunk); buffer += chunk.toString();
  for (;;) {
    const end = buffer.indexOf("\n"); if (end < 0) break;
    const line = buffer.slice(0, end); buffer = buffer.slice(end + 1);
    let message; try { message = JSON.parse(line); } catch { continue; }
    const request = pending.get(message.id); if (!request) continue;
    pending.delete(message.id); if (message.error) request.reject(Error(JSON.stringify(message.error))); else request.resolve(message.result);
  }
});
function send(method: string, params: unknown, id?: number): void { child.stdin.write(JSON.stringify({ jsonrpc: "2.0", method, params, ...(id === undefined ? {} : { id }) }) + "\n"); }
async function request(method: string, params: unknown): Promise<any> {
  const id = ++sequence, timer = setTimeout(() => pending.get(id)?.reject(Error(`MCP ${method} deadline`)), 120000);
  try { return await new Promise((resolve, reject) => { pending.set(id, { resolve, reject }); send(method, params, id); }); }
  finally { clearTimeout(timer); pending.delete(id); }
}
try {
  await request("initialize", { protocolVersion: "2024-11-05", capabilities: {}, clientInfo: { name: "codex-dashboard-control-plane", version: "1.0.0" } }); send("notifications/initialized", {});
  const mode = process.argv[2] ?? "reopen";
  if (mode === "reopen") {
    const goals = await request("resources/read", { uri: "repo://goals" }); writeFileSync(join(generated, "goals.json"), JSON.stringify(goals));
    console.log(JSON.stringify(goals).slice(0, 6000));
    const result = await request("tools/call", { name: "ticket_reopen", arguments: { path: "26/09/23/DASHBOARD-LAUNCH-COCKPIT", client: "codex", goal: "🎯runningframework🎯runningproducts🎯runningrepo", prompt: "Make the dashboard the sole declarative developer control plane, dissolve editor and agent launch configurations, repair full TUI pointer/selection/cursor/window/task-tab behavior, and battle test end to end across the complete monorepo.", no_management: true } });
    assert.ok(!result.isError, JSON.stringify(result)); console.log("[DEBUG] ticket_reopen " + JSON.stringify(result));
  } else if (mode === "close") {
    const result = await request("tools/call", { name: "ticket_close", arguments: JSON.parse(readFileSync(join(import.meta.dir, "🔣️close.json"), "utf8")) });
    assert.ok(!result.isError, JSON.stringify(result)); console.log("[DEBUG] ticket_close " + JSON.stringify(result));
  } else throw Error(`Unknown mode ${mode}`);
} finally {
  child.stdin.end(); const timer = setTimeout(() => child.kill(), 5000); await exited; clearTimeout(timer);
}
