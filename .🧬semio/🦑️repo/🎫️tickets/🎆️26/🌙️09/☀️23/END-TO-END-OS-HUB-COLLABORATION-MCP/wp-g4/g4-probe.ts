#!/usr/bin/env bun
/** 🔭️ G4 stdio probe of semio-os-mcp: lists, create→prepare→invoke→snapshot→undo→redo→transaction(commit+rollback)→inference_run with progress, plus one in-flight cancellation via notifications/cancelled. Writes a full JSON-RPC transcript. */
import { spawn } from "node:child_process";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createInterface } from "node:readline";
import { ensureMcpBinary } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";

const REPO = "/Users/ueli/Documents/semio";
const OUT = join(import.meta.dir, "generated");
const RUN = process.env.G4_RUN ?? "probe";
const SOLVE_BUDGET_MS = Number(process.env.G4_SOLVE_BUDGET_MS ?? 300_000);
const SCOPES = "workspace.read,artifact.write,inference.execute,ui.observe,ui.control,conversation.write";
const T0 = Date.now();
const at = () => `${((Date.now() - T0) / 1000).toFixed(1)}s`;
mkdirSync(OUT, { recursive: true });

type Envelope = { jsonrpc: "2.0"; id?: number | null; method?: string; params?: any; result?: any; error?: { code: number; message: string } };
const transcript: Array<{ t: string; dir: "→" | "←"; msg: Envelope }> = [];
const rows: Array<{ step: string; ok: boolean; detail: string }> = [];
const row = (step: string, ok: boolean, detail: string) => {
  rows.push({ step, ok, detail });
  console.log(`${ok ? "PASS" : "FAIL"}  [${at()}] ${step} — ${detail}`);
};

const bin = ensureMcpBinary(REPO);
const folder = mkdtempSync(join(tmpdir(), "semio-g4-probe-"));
const child = spawn(bin, ["stdio", "--folder", folder, "--no-bridge", "--scopes", SCOPES], { stdio: ["pipe", "pipe", "pipe"] });
let stderr = "";
child.stderr.on("data", (chunk: Buffer) => (stderr += chunk.toString("utf8")));
const pending = new Map<number, (envelope: Envelope) => void>();
const notifications: Envelope[] = [];
createInterface({ input: child.stdout }).on("line", (line) => {
  const msg = JSON.parse(line) as Envelope;
  transcript.push({ t: at(), dir: "←", msg });
  if (typeof msg.id === "number" && pending.has(msg.id)) {
    pending.get(msg.id)!(msg);
    pending.delete(msg.id);
  } else if (msg.method) notifications.push(msg);
});
const send = (msg: Envelope) => {
  transcript.push({ t: at(), dir: "→", msg });
  child.stdin.write(`${JSON.stringify(msg)}\n`);
};
let nextId = 1;
const issue = (method: string, params: unknown, timeoutMs = 180_000): { id: number; reply: Promise<Envelope> } => {
  const id = nextId++;
  const reply = new Promise<Envelope>((resolve) => {
    const timer = setTimeout(() => {
      pending.delete(id);
      resolve({ jsonrpc: "2.0", id, error: { code: -32000, message: `no reply within ${timeoutMs} ms` } });
    }, timeoutMs);
    pending.set(id, (envelope) => {
      clearTimeout(timer);
      resolve(envelope);
    });
  });
  send({ jsonrpc: "2.0", id, method, params });
  return { id, reply };
};
const request = (method: string, params: unknown = {}, timeoutMs?: number) => issue(method, params, timeoutMs).reply;
const call = async (name: string, args: unknown, timeoutMs?: number) => {
  const envelope = await request("tools/call", { name, arguments: args }, timeoutMs);
  return envelope.error ? { isError: true, structuredContent: { code: envelope.error.code, message: envelope.error.message } as any } : (envelope.result as { isError?: boolean; structuredContent?: any });
};
const brief = (value: unknown, n = 220) => JSON.stringify(value).slice(0, n);
const waitFor = async (predicate: () => boolean, timeoutMs: number) => {
  const deadline = Date.now() + timeoutMs;
  while (!predicate() && Date.now() < deadline) await new Promise((resolve) => setTimeout(resolve, 50));
  return predicate();
};

const indices = Array.from({ length: 16 }, (_, cell) => (cell + Math.floor(cell / 4)) % 2);
const bitmap = (width: number, height: number, seed: number) => ({
  schema: "s.wfc.bitmap",
  seed,
  input: { width: 4, height: 4, palette: [{ r: 0, g: 0, b: 0, a: 255 }, { r: 255, g: 255, b: 255, a: 255 }], pixels: Buffer.from(Uint8Array.from(indices)).toString("base64") },
  output: { width, height, periodic: true },
  model: { patternSize: 2, symmetry: 1, periodicInput: true },
  pinned: [],
});

try {
  const init = await request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "g4-probe", version: "1" } });
  row("initialize", !init.error, `server=${init.result?.serverInfo?.name}@${init.result?.serverInfo?.version} protocol=${init.result?.protocolVersion}`);
  send({ jsonrpc: "2.0", method: "notifications/initialized", params: {} });
  const tools = await request("tools/list");
  row("tools/list", !tools.error, `${tools.result?.tools?.length} tools: ${tools.result?.tools?.map((tool: any) => tool.name).join(",")}`);
  const resources = await request("resources/list");
  row("resources/list", !resources.error, `${resources.result?.resources?.length} resources: ${resources.result?.resources?.map((resource: any) => resource.uri).join(",")}`);
  const prompts = await request("prompts/list");
  row("prompts/list", !prompts.error, `${prompts.result?.prompts?.length} prompts: ${prompts.result?.prompts?.map((prompt: any) => prompt.name).join(",")}`);
  const ctx = await call("context_resolve", {});
  row("context_resolve", ctx.isError !== true, `channel=${ctx.structuredContent?.channel} principal=${ctx.structuredContent?.principal}`);

  if (process.env.G4_ONLY_GENESIS_PAYLOAD) {
    const token = `g4-gen-${Date.now().toString(36)}`;
    const genesis = { schema: "s.wfc.bitmap", seed: 0, input: { width: 2, height: 2, palette: [{ r: 0, g: 0, b: 0, a: 255 }, { r: 255, g: 255, b: 255, a: 255 }], pixels: Buffer.from([0, 1, 1, 0]).toString("base64") }, output: { width: Number(process.env.G4_OUT_W ?? 2), height: Number(process.env.G4_OUT_H ?? 2), periodic: process.env.G4_OUT_PERIODIC === "1" }, model: { patternSize: 2, symmetry: 1, periodicInput: true }, pinned: [] };
    const ran = await issue("tools/call", { name: "inference_run", arguments: { artifactKind: "s.wfc.bitmap", inferenceSchema: "s.wfc.bitmap.solve", pluginId: "wfc", payload: { snapshot: genesis }, cancellationId: token }, _meta: { progressToken: token } }, SOLVE_BUDGET_MS).reply;
    row(`inference_run genesis payload out=${genesis.output.width}x${genesis.output.height} periodic=${genesis.output.periodic}`, !ran.error && ran.result?.isError !== true, JSON.stringify(ran.error ?? ran.result?.structuredContent).slice(0, 400));
    throw new Error("genesis-payload mode done");
  }
  if (process.env.G4_ONLY_ARTIFACT_SOLVE) {
    const wfcId = `g4-wfc-${Date.now().toString(36)}`;
    const createdWfc = await call("artifact_create", { artifactId: wfcId, kind: "s.wfc.bitmap" });
    row("artifact_create s.wfc.bitmap", createdWfc.isError !== true, JSON.stringify(createdWfc.structuredContent).slice(0, 200));
    const snap = await call("artifact_snapshot", { artifactId: wfcId });
    console.log(`INFO snapshot ${JSON.stringify(snap.structuredContent).slice(0, 600)}`);
    writeFileSync(join(OUT, "g4-wfc-genesis-pair.json"), JSON.stringify({ pack: snap.structuredContent?.packBase64, spr: snap.structuredContent?.sprBase64 }));
    const exported = await call("artifact_export", { artifactId: wfcId });
    console.log(`INFO export ${JSON.stringify({ ...exported.structuredContent, contentBase64: undefined })} TEXT=${Buffer.from(String(exported.structuredContent?.contentBase64 ?? ""), "base64").toString("utf8").slice(0, 1500)}`);
    if (process.env.G4_EXPORT_ONLY) throw new Error("export-only");
    const token = `g4-art-${Date.now().toString(36)}`;
    const ran = await issue("tools/call", { name: "inference_run", arguments: { artifactKind: "s.wfc.bitmap", inferenceSchema: "s.wfc.bitmap.solve", pluginId: "wfc", artifactId: wfcId, cancellationId: token, ...(process.env.G4_WORK_UNITS ? { workUnits: Number(process.env.G4_WORK_UNITS) } : {}) }, _meta: { progressToken: token } }, SOLVE_BUDGET_MS).reply;
    row("inference_run on the created artifact", !ran.error && ran.result?.isError !== true, JSON.stringify(ran.error ?? ran.result?.structuredContent).slice(0, 400));
    throw new Error("artifact-only mode done");
  }
  const artifactId = `g4-note-${Date.now().toString(36)}`;
  const created = await call("artifact_create", { artifactId, kind: "s.note.note" });
  row("artifact_create", created.isError !== true, brief(created.structuredContent));
  const opened = await call("artifact_open", { artifactId });
  row("artifact_open", opened.isError !== true, `kind=${opened.structuredContent?.kind} sizeBytes=${opened.structuredContent?.sizeBytes}`);
  const capabilityId = "note.s.note.note@1/*#editor.addBlock";
  const input = { kind: "text" };
  const before = await call("artifact_snapshot", { artifactId });
  const prepared = await call("action_prepare", { capabilityId, input });
  row("action_prepare", prepared.isError !== true, `handle=${prepared.structuredContent?.preparedHandle}`);
  const invoked = await call("action_invoke", { preparedActionHandle: prepared.structuredContent?.preparedHandle });
  const undoToken = invoked.structuredContent?.undoToken;
  row("action_invoke", invoked.isError !== true && invoked.structuredContent?.status === "SUCCEEDED", `status=${invoked.structuredContent?.status} after=${brief(invoked.structuredContent?.revisionAfter, 160)}`);
  const after = await call("artifact_snapshot", { artifactId });
  const bytes = (snap: any) => `${snap.structuredContent?.packBase64 ?? ""}|${snap.structuredContent?.sprBase64 ?? ""}`;
  row("artifact_snapshot shows the edit", after.isError !== true && bytes(after) !== bytes(before), `pack ${before.structuredContent?.packBytes}→${after.structuredContent?.packBytes} spr ${before.structuredContent?.sprBytes}→${after.structuredContent?.sprBytes}`);
  const undone = await call("history_undo", { undoToken });
  const afterUndo = await call("artifact_snapshot", { artifactId });
  row("history_undo", undone.isError !== true && (undone.structuredContent?.warnings ?? []).length === 0, `members=${undone.structuredContent?.members} spr→${afterUndo.structuredContent?.sprBytes}`);
  const redone = await call("history_redo", { undoToken });
  const afterRedo = await call("artifact_snapshot", { artifactId });
  row("history_redo", redone.isError !== true && (redone.structuredContent?.warnings ?? []).length === 0, `members=${redone.structuredContent?.members} spr→${afterRedo.structuredContent?.sprBytes}`);

  const member = await call("action_prepare", { capabilityId, input });
  const began = await call("transaction_begin", { preparedHandles: [member.structuredContent?.preparedHandle] });
  const committed = await call("transaction_commit", { transactionHandle: began.structuredContent?.transactionHandle });
  const afterCommit = await call("artifact_snapshot", { artifactId });
  row("transaction_begin→commit", began.isError !== true && committed.isError !== true && bytes(afterCommit) !== bytes(afterRedo), `txn=${began.structuredContent?.transactionHandle} commit=${brief(committed.structuredContent, 160)}`);
  const member2 = await call("action_prepare", { capabilityId, input });
  const began2 = await call("transaction_begin", { preparedHandles: [member2.structuredContent?.preparedHandle] });
  const rolled = await call("transaction_rollback", { transactionHandle: began2.structuredContent?.transactionHandle });
  const afterRollback = await call("artifact_snapshot", { artifactId });
  row("transaction_begin→rollback", began2.isError !== true && rolled.isError !== true && bytes(afterRollback) === bytes(afterCommit), `rolledBack=${rolled.structuredContent?.rolledBack}`);

  const solveToken = `g4-solve-${Date.now().toString(36)}`;
  const solve = issue("tools/call", { name: "inference_run", arguments: { artifactKind: "s.wfc.bitmap", inferenceSchema: "s.wfc.bitmap.solve", pluginId: "wfc", payload: { snapshot: bitmap(6, 4, 11) }, cancellationId: solveToken }, _meta: { progressToken: solveToken } }, SOLVE_BUDGET_MS);
  const solved = await solve.reply;
  const solveProgress = notifications.filter((n) => n.method === "notifications/progress" && n.params?.progressToken === solveToken).map((n) => n.params?.progress);
  const solvedResult = solved.result ?? {};
  row("inference_run s.wfc.bitmap.solve (progress)", !solved.error && solvedResult.isError !== true && solveProgress.length > 0, `status=${solvedResult.structuredContent?.status} progress=[${solveProgress.join(",")}] ${solved.error ? brief(solved.error) : brief(solvedResult.structuredContent, 200)}`);

  const cancelToken = `g4-cancel-${Date.now().toString(36)}`;
  const cancelRun = issue("tools/call", { name: "inference_run", arguments: { artifactKind: "s.wfc.bitmap", inferenceSchema: "s.wfc.bitmap.solve", pluginId: "wfc", payload: { snapshot: bitmap(96, 96, 7) }, cancellationId: cancelToken }, _meta: { progressToken: cancelToken } }, SOLVE_BUDGET_MS);
  const sawProgress = await waitFor(() => notifications.some((n) => n.method === "notifications/progress" && n.params?.progressToken === cancelToken && n.params?.progress >= 0.35), SOLVE_BUDGET_MS);
  await new Promise((resolve) => setTimeout(resolve, 3_000));
  const cancelSentAt = Date.now();
  send({ jsonrpc: "2.0", method: "notifications/cancelled", params: { requestId: cancelRun.id, reason: "g4 in-flight cancellation" } });
  const cancelled = await cancelRun.reply;
  const cancelResult = cancelled.result ?? {};
  const cancelProgress = notifications.filter((n) => n.method === "notifications/progress" && n.params?.progressToken === cancelToken).map((n) => n.params?.progress);
  const status = String(cancelResult.structuredContent?.status ?? cancelResult.structuredContent?.code ?? cancelled.error?.message ?? "");
  row("inference_run in-flight notifications/cancelled", sawProgress && /CANCEL/i.test(status), `sawProgressBeforeCancel=${sawProgress} returned ${Date.now() - cancelSentAt} ms after cancel status=${status} progress=[${cancelProgress.join(",")}] ${brief(cancelResult.structuredContent ?? cancelled.error, 200)}`);
  const alive = await request("ping", {}, 30_000);
  row("server answers after the cancellation", !alive.error, alive.error ? brief(alive.error) : "ping ok");
} catch (error) {
  row("probe", false, String(error));
} finally {
  child.kill();
  const failed = rows.filter((entry) => !entry.ok).length;
  console.log(`g4-probe: ${rows.length - failed}/${rows.length} rows green`);
  writeFileSync(join(OUT, `g4-${RUN}-transcript.json`), JSON.stringify({ bin, folder, rows, transcript: transcript.map((entry) => { const text = JSON.stringify(entry.msg); return text.length > 20_000 ? { ...entry, msg: { truncated: text.slice(0, 2_000), bytes: text.length } } : entry; }), stderrTail: stderr.slice(-8_000) }, null, 2));
  process.exit(failed === 0 ? 0 : 1);
}
