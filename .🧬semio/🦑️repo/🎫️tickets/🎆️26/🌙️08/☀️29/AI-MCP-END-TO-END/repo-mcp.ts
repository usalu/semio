import { spawn } from "node:child_process";
import { createInterface } from "node:readline";
import { appendFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const generated = join(import.meta.dirname, "🗑️generated");
mkdirSync(generated, { recursive: true });
const child = spawn("repo", ["mcp"], { cwd: process.cwd(), env: { ...process.env, SEMIO_TEST_ARTIFACT_DIR: generated }, shell: false, stdio: ["pipe", "pipe", "pipe"] });
const progressLog = join(generated, `mcp-${process.pid}.jsonl`);
const stderrLog = join(generated, `mcp-${process.pid}.stderr.txt`);
child.stderr!.on("data", chunk => { appendFileSync(stderrLog, chunk); process.stderr.write(chunk); });
const drained = new Promise<void>(resolve => child.once("exit", () => resolve()));
const pending = new Map<number, (value: any) => void>();
let sequence = 0;
createInterface({ input: child.stdout! }).on("line", line => {
  try {
    appendFileSync(progressLog, line + "\n");
    const response = JSON.parse(line);
    if (response.id !== undefined) pending.get(response.id)?.(response);
    else if (response.method === "notifications/progress") process.stderr.write("[DEBUG] " + JSON.stringify(response.params) + "\n");
  } catch {}
});
const request = (method: string, params: unknown) => new Promise<any>((resolve, reject) => {
  const id = ++sequence;
  const timer = setTimeout(() => { child.stdin!.write(JSON.stringify({ jsonrpc: "2.0", method: "notifications/cancelled", params: { requestId: id, reason: "ticket delivery deadline" } }) + "\n"); reject(new Error(`Timed out: ${method}; awaiting owned cancellation and drain`)); }, method === "initialize" ? 30000 : Number(process.env.SEMIO_TICKET_TIMEOUT_MS ?? 1800000));
  pending.set(id, value => { clearTimeout(timer); pending.delete(id); resolve(value); });
  child.stdin!.write(JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n");
});
try {
  await request("initialize", { protocolVersion: "2024-11-05", capabilities: {}, clientInfo: { name: "codex", version: "1" } });
  child.stdin!.write(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }) + "\n");
  const [method, params] = process.argv.slice(2);
  const input = JSON.parse(params?.startsWith("@") ? readFileSync(params.slice(1), "utf8") : params ?? "{}");
  const response = await request(method, method === "tools/call" ? { ...input, _meta: { progressToken: "semio-ticket" } } : input);
  const saved = join(generated, `mcp-${process.pid}.json`);
  if (!input.name?.startsWith("ticket_")) writeFileSync(saved, JSON.stringify(response));
  const result = response.result?.structuredContent;
  console.log(JSON.stringify(method === "tools/list" ? response.result.tools.filter((tool: any) => new RegExp(process.env.SEMIO_TICKET_TOOL_FILTER??"ticket|cargo").test(tool.name)) : result?.operation ? { saved, ...result, stdout: result.stdout?.slice(-16000), stderr: result.stderr?.slice(-16000) } : response));
} finally {
  child.stdin!.end();
  await drained;
}
