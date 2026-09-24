#!/usr/bin/env bun
/** 💼️ G10 live proof of the generic inference quartet on its GUEST site, over the real stdio semio MCP
 * with a folder workspace and the staged wfc guest: `inference_list` shows the declared commit action,
 * `inference_submit` starts a wfc solve in the plugin's guest, `inference_events` streams it to an
 * offered proposal, `inference_cancel` stops or withdraws a second job, `inference_approve` commits
 * the proposal through the normal edit path (`pin-solution`), a second approval of the same hash is
 * refused, and the commit is an ordinary, undoable edit.
 * usage: bun g10-quartet-folder-live.ts [captureDir] */
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { requireMcpBinary, spawnRawMcp } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";

const OUT = process.argv[2] ?? "/Users/ueli/Documents/semio/.tmp-ticket/wp-g10/generated";
const T0 = Date.now();
const at = () => `${((Date.now() - T0) / 1000).toFixed(1)}s`;
const rows: { step: string; ok: boolean; detail: string }[] = [];
const transcript: unknown[] = [];
const row = (step: string, ok: boolean, detail: string) => {
  rows.push({ step, ok, detail });
  console.log(`${ok ? "PASS" : "FAIL"}  [${at()}] ${step} — ${detail}`);
};
const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

process.env.S_AGENT_BRIDGE_DIR = mkdtempSync(join(tmpdir(), "g10-quartet-bridge-"));
const folder = mkdtempSync(join(tmpdir(), "g10-quartet-folder-"));
const mcp = spawnRawMcp(requireMcpBinary("/Users/ueli/Documents/semio"), ["stdio", "--folder", folder, "--scopes", "workspace.read,artifact.write,inference.execute", "--auto-approve", "all", "--no-bridge"]);
const call = async (name: string, args: Record<string, unknown>, budgetMs = 600_000) => {
  const response = await mcp.request("tools/call", { name, arguments: args, _meta: { progressToken: `g10-${name}-${Date.now()}` } }, budgetMs);
  const result = (response.result ?? {}) as { isError?: boolean; structuredContent?: any };
  transcript.push({ at: at(), tool: name, args, isError: result.isError === true, structured: result.structuredContent });
  return result;
};
const read = async (uri: string) => String(((await mcp.request("resources/read", { uri }, 120_000)).result as any)?.contents?.[0]?.text ?? "");
const follow = async (handle: string, until: (job: any) => boolean, budgetMs: number) => {
  const deadline = Date.now() + budgetMs;
  let cursor = 0;
  let job: any;
  const events: string[] = [];
  while (Date.now() < deadline) {
    const page = await call("inference_events", { jobHandle: handle, after: cursor });
    job = page.structuredContent?.job ?? page.structuredContent;
    for (const event of job?.events ?? []) events.push(String(event.kind ?? event));
    if (typeof job?.nextCursor === "number") cursor = job.nextCursor;
    if (!job || until(job)) break;
    await sleep(300);
  }
  return { job, events };
};

try {
  const init = await mcp.request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "g10-quartet-folder", version: "1" } }, 900_000);
  mcp.writeRaw(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }));
  row("0 the stdio semio MCP serves a folder workspace", init.error === undefined, `server=${(init.result as any)?.serverInfo?.name}`);

  const artifactId = `g10-quartet-${Date.now().toString(36)}`;
  const created = await call("artifact_create", { artifactId, kind: "s.wfc.bitmap" }, 900_000);
  row("1 artifact_create makes a wfc bitmap document", created.isError !== true, JSON.stringify(created.structuredContent).slice(0, 200));

  const listed = await call("inference_list", { artifactId });
  const solve = (listed.structuredContent?.declared ?? []).find((item: any) => item.inferenceSchema === "s.wfc.bitmap.solve");
  row("2 inference_list declares the solve's commit action", solve?.payload?.commit?.action === "pin-solution", `commit=${JSON.stringify(solve?.payload?.commit)}`);

  const before = await call("artifact_snapshot", { artifactId });
  const historyBefore = await read(`semio://artifact/${artifactId}/history`);

  const submitted = await call("inference_submit", { documentId: artifactId });
  const handle = String(submitted.structuredContent?.jobHandle ?? "");
  row("3 inference_submit starts the solve in the plugin's guest", submitted.isError !== true && submitted.structuredContent?.job?.site === "guest" && handle.length > 0, `handle=${handle} site=${submitted.structuredContent?.job?.site} state=${submitted.structuredContent?.job?.state} ${submitted.isError ? JSON.stringify(submitted.structuredContent).slice(0, 240) : ""}`);

  const second = await call("inference_submit", { documentId: artifactId });
  const secondHandle = String(second.structuredContent?.jobHandle ?? "");
  const cancelled = await call("inference_cancel", { jobHandle: secondHandle });
  const secondFollowed = await follow(secondHandle, (job) => ["cancelled", "succeeded", "failed"].includes(job.state) && job.proposalState !== "offered", 300_000);
  row("4 inference_cancel stops a running job or withdraws its offered proposal — nothing is applied", cancelled.isError !== true && (secondFollowed.job?.state === "cancelled" || secondFollowed.job?.proposalState === "cancelled"), `state=${secondFollowed.job?.state}/${secondFollowed.job?.proposalState} events=${JSON.stringify(secondFollowed.events)}`);

  const offered = await follow(handle, (job) => job.proposalState === "offered" || ["failed", "cancelled"].includes(job.state), 900_000);
  row("5 inference_events streams the job to an offered proposal", offered.job?.proposalState === "offered" && typeof offered.job?.proposal?.hash === "string", `state=${offered.job?.state}/${offered.job?.proposalState} action=${offered.job?.proposal?.action ?? offered.job?.proposal?.capabilityId} events=${JSON.stringify(offered.events)} progress=${(offered.job?.progress ?? []).length}`);

  const approved = await call("inference_approve", { jobHandle: handle, proposalHash: offered.job?.proposal?.hash });
  const approvedJob = approved.structuredContent?.job ?? approved.structuredContent;
  row("6 inference_approve commits the proposal through the normal edit path", approved.isError !== true && approvedJob?.proposalState === "approved" && approvedJob?.state === "succeeded", `state=${approvedJob?.state}/${approvedJob?.proposalState} commit=${JSON.stringify(approvedJob?.commit ?? null).slice(0, 240)} ${approved.isError ? JSON.stringify(approved.structuredContent).slice(0, 240) : ""}`);

  const replay = await call("inference_approve", { jobHandle: handle, proposalHash: offered.job?.proposal?.hash });
  row("7 a second approval of the same proposal is refused", replay.isError === true, JSON.stringify(replay.structuredContent).slice(0, 200));

  const after = await call("artifact_snapshot", { artifactId });
  const historyAfter = await read(`semio://artifact/${artifactId}/history`);
  row("8 the committed solve changed the document", after.structuredContent?.sprBytes !== before.structuredContent?.sprBytes, `spr ${before.structuredContent?.sprBytes}→${after.structuredContent?.sprBytes} pack ${before.structuredContent?.packBytes}→${after.structuredContent?.packBytes} history resource ${historyBefore.length}→${historyAfter.length}B`);

  const undone = await call("history_undo", { artifactId, undoToken: approvedJob?.commit?.undoToken });
  const afterUndo = await call("artifact_snapshot", { artifactId });
  row("9 the committed solve is an ordinary undoable edit", undone.isError !== true && afterUndo.structuredContent?.sprBytes !== after.structuredContent?.sprBytes, `undo=${JSON.stringify(undone.structuredContent).slice(0, 160)} spr ${after.structuredContent?.sprBytes}→${afterUndo.structuredContent?.sprBytes}`);

  const progress = mcp.stdoutLines().filter((line) => line.includes("notifications/progress")).length;
  row("10 progress notifications reached the client", progress > 0, `notifications/progress lines=${progress}`);
} catch (error) {
  row("run", false, error instanceof Error ? error.stack ?? error.message : String(error));
} finally {
  writeFileSync(join(OUT, "g10-quartet-folder-transcript.json"), JSON.stringify({ rows, transcript, stderr: mcp.stderrText().slice(-6000) }, null, 2));
  await mcp.close();
}
const red = rows.filter((entry) => !entry.ok).length;
console.log(`g10-quartet-folder-live: ${rows.length - red}/${rows.length} rows green`);
process.exit(red === 0 ? 0 : 1);
