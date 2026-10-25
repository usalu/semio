import { spawn } from "node:child_process";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, resolve } from "node:path";

const here = import.meta.dir;
const root = resolve(here, "../../../../../../../..");
const server = process.argv[2] ?? "repo";
const outPath = process.argv[3] ?? `${here}/../🗑️generated/mcp-b/probe-${server}.json`;
const overrideCmd = process.env.PROBE_COMMAND ? JSON.parse(process.env.PROBE_COMMAND) as string[] : undefined;
const spec = JSON.parse(readFileSync(`${root}/.mcp.json`, "utf8")).mcpServers[server];
const exp = JSON.parse(readFileSync(`${here}/expectations.json`, "utf8"))[server];
const [command, ...args] = overrideCmd ?? [spec.command, ...spec.args];
const timeoutMs = Number(process.env.PROBE_TIMEOUT_MS ?? 120000);
const t0 = Date.now();
const checks: { name: string; ok: boolean; detail?: string }[] = [];
const check = (name: string, ok: boolean, detail?: string) => { checks.push({ name, ok, detail }); console.log(`${ok ? "PASS" : "FAIL"} ${name}${detail ? " " + detail : ""}`); };
mkdirSync(dirname(outPath), { recursive: true });
const child = spawn(command, args, { cwd: root, env: { ...process.env, ...(spec.env ?? {}) }, stdio: ["pipe", "pipe", "pipe"] });
let stderr = "";
child.stderr.on("data", (d) => (stderr += d.toString()));
let buf = "";
let pollution = 0;
const waiters = new Map<number, (m: any) => void>();
child.stdout.on("data", (d) => {
  buf += d.toString();
  let i: number;
  while ((i = buf.indexOf("\n")) >= 0) {
    const line = buf.slice(0, i).trim();
    buf = buf.slice(i + 1);
    if (!line) continue;
    try {
      const m = JSON.parse(line);
      if (m.id !== undefined && waiters.has(m.id)) waiters.get(m.id)!(m);
    } catch { pollution++; console.log("STDOUT-POLLUTION", line.slice(0, 200)); }
  }
});
let exited: string | undefined;
child.on("exit", (c, s) => { exited = `code=${c} signal=${s}`; });
let nextId = 1;
const send = (method: string, params?: unknown) =>
  new Promise<any>((res, rej) => {
    const id = nextId++;
    const to = setTimeout(() => rej(new Error(`timeout ${method}`)), timeoutMs);
    waiters.set(id, (m) => { clearTimeout(to); res(m); });
    child.stdin.write(JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n");
  });
const record: Record<string, unknown> = { server, command: [command, ...args] };
try {
  const init = await send("initialize", { protocolVersion: exp.initialize.protocolVersion, capabilities: {}, clientInfo: { name: "mcp-b-probe", version: "0" } });
  const ms = Date.now() - t0;
  record.initialize = init;
  check("initialize result", !!init.result && !init.error, init.error ? JSON.stringify(init.error) : `${ms} ms`);
  check("initialize latency", ms <= exp.initialize.maxMs, `${ms} <= ${exp.initialize.maxMs}`);
  check("serverInfo.name", init.result?.serverInfo?.name === exp.initialize.serverInfoName, String(init.result?.serverInfo?.name));
  child.stdin.write(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }) + "\n");
  const caps = init.result?.capabilities ?? {};
  const list = await send("tools/list", {});
  const tools: any[] = list.result?.tools ?? [];
  record.tools = tools.map((t) => t.name);
  check("tools/list", !list.error && tools.length >= exp.tools.min, `${tools.length} tools`);
  for (const r of exp.tools.required ?? []) check(`tool ${r}`, tools.some((t) => t.name === r));
  check("tools have inputSchema", tools.every((t) => t.inputSchema && t.inputSchema.type === "object"));
  if (exp.resources) {
    const rl = await send("resources/list", {});
    const res: any[] = rl.result?.resources ?? [];
    record.resources = res.map((r) => r.uri);
    check("resources/list", !rl.error && res.length >= exp.resources.min, rl.error ? JSON.stringify(rl.error) : `${res.length} resources`);
    for (const u of exp.resources.requiredUris ?? []) check(`resource ${u}`, res.some((r) => r.uri === u));
    if (exp.read) {
      const rr = await send("resources/read", { uri: exp.read.uri });
      const contents: any[] = rr.result?.contents ?? [];
      record.read = { uri: exp.read.uri, count: contents.length, head: String(contents[0]?.text ?? "").slice(0, 300) };
      check(`resources/read ${exp.read.uri}`, !rr.error && contents.length >= exp.read.contentsMin, rr.error ? JSON.stringify(rr.error) : `${contents.length} contents`);
    }
  }
  const callName = process.env.PROBE_CALL;
  if (callName) {
    const r = await send("tools/call", { name: callName, arguments: JSON.parse(process.env.PROBE_ARGS ?? "{}") });
    record.call = r;
    const sc = r.result?.structuredContent;
    check(`tools/call ${callName}`, !r.error && Array.isArray(r.result?.content), r.error ? JSON.stringify(r.error) : `isError=${r.result?.isError}`);
    check("structuredContent omitted-or-object", sc === undefined || (sc !== null && typeof sc === "object" && !Array.isArray(sc)));
  }
} catch (e) {
  check("probe", false, String(e));
}
check("stdout pollution", pollution === exp.stdoutPollution, String(pollution));
record.exited = exited ?? "running";
record.stderrHead = stderr.slice(0, 2000);
record.checks = checks;
record.ok = checks.every((c) => c.ok);
writeFileSync(outPath, JSON.stringify(record, null, 2));
child.kill("SIGTERM");
setTimeout(() => { try { child.kill("SIGKILL"); } catch {} process.exit(record.ok ? 0 : 1); }, 1500);
