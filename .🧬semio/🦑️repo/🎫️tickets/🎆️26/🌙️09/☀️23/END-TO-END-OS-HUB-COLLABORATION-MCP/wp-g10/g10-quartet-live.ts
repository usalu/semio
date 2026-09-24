#!/usr/bin/env bun
/** 💼️ G10 live proof (G9's `g9-quartet-live.ts`, re-pointed): the generic inference quartet over the stdio semio MCP against a hub.
 * A human signs in, creates a wfc grid3d document and a gis map on the hub, and delegates two agents. Agent A
 * submits a grid3d solve (guest-executed, no commit binding: a result, never a proposal) and gis inferences
 * (hub-executed) through `inference_submit`, streams them with `inference_events`, cancels a second solve and
 * withdraws one gis offer with `inference_cancel`, and approves the other gis offer with `inference_approve`, which
 * commits through the normal edit path. Agent B, a second MCP client, then observes the committed edit, and the
 * human reads the advanced ledger head. The run owns a fresh space. Usage: bun g10-quartet-live.ts <hubOrigin> [binary] */
import { spawn, type ChildProcessWithoutNullStreams } from "node:child_process";
import { randomBytes } from "node:crypto";
import { chmodSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const ORIGIN = process.argv[2] ?? "http://127.0.0.1:8030";
const BINARY = process.argv[3] ?? "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp";
const OUT = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s11-g10-logs";
const EMAIL = process.env.OS_MCP_HUB_EMAIL ?? "user1@semio.dev";
const PASSWORD = process.env.OS_MCP_HUB_PASSWORD ?? "gm1-local-dev-pass-1";
const T0 = Date.now();
const at = () => `${((Date.now() - T0) / 1000).toFixed(1)}s`;
const rows: Array<{ step: string; ok: boolean; detail: string }> = [];
const transcript: unknown[] = [];
const row = (step: string, ok: boolean, detail: string) => {
  rows.push({ step, ok, detail });
  console.log(`${ok ? "PASS" : "FAIL"}  [${at()}] ${step} — ${detail}`);
};
const hub = async (method: string, path: string, token?: string, body?: unknown) => {
  const response = await fetch(`${ORIGIN}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body: JSON.stringify(body) }), signal: AbortSignal.timeout(120_000) });
  const text = await response.text();
  let json: any;
  try {
    json = JSON.parse(text);
  } catch {}
  return { status: response.status, text, json };
};
const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

/** 🔌️ One stdio semio MCP client with its own delegated credential. */
class Agent {
  private child: ChildProcessWithoutNullStreams;
  private buffer = "";
  private next = 1;
  private pending = new Map<number, (message: any) => void>();
  stderr = "";
  progress: unknown[] = [];
  constructor(readonly name: string, spaceId: string, credentialPath: string) {
    this.child = spawn(BINARY, ["stdio", "--hub", ORIGIN, "--space", spaceId, "--credential-file", credentialPath, "--scopes", "workspace.read,artifact.write,inference.execute", "--auto-approve", "all", "--no-bridge"], { stdio: ["pipe", "pipe", "pipe"] });
    this.child.stderr.on("data", (chunk: Buffer) => (this.stderr += chunk.toString("utf8")));
    this.child.stdout.on("data", (chunk: Buffer) => {
      this.buffer += chunk.toString("utf8");
      for (let newline = this.buffer.indexOf("\n"); newline >= 0; newline = this.buffer.indexOf("\n")) {
        const line = this.buffer.slice(0, newline).trim();
        this.buffer = this.buffer.slice(newline + 1);
        if (!line) continue;
        const message = JSON.parse(line);
        if (message.method === "notifications/progress") this.progress.push(message.params);
        this.pending.get(message.id)?.(message);
        this.pending.delete(message.id);
      }
    });
  }
  request(method: string, params: unknown): Promise<any> {
    return new Promise((resolve) => {
      const id = this.next++;
      this.pending.set(id, resolve);
      this.child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
    });
  }
  async call(name: string, args: unknown): Promise<any> {
    const result = (await this.request("tools/call", { name, arguments: args, _meta: { progressToken: `${this.name}-${this.next}` } })).result ?? {};
    transcript.push({ at: at(), agent: this.name, tool: name, args, isError: result.isError === true, structured: result.structuredContent });
    return result;
  }
  async start(): Promise<void> {
    await this.request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: `g10-${this.name}`, version: "1" } });
    this.child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`);
  }
  async documents(): Promise<any[]> {
    const listed = await this.request("resources/read", { uri: "semio://workspace/artifacts" });
    return JSON.parse(String(listed.result?.contents?.[0]?.text ?? "{}")).artifacts ?? [];
  }
  stop(): void {
    this.child.kill();
  }
}

const delegate = async (human: string, spaceId: string, label: string) => {
  const delegation = await hub("POST", "/auth/agent-delegations", human, { schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: label, audience: "edit", ttlSecs: 1800 });
  const path = join(mkdtempSync(join(tmpdir(), "semio-g10-agent-")), "agent-credential.json");
  writeFileSync(path, `${JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: ORIGIN, spaceId, audience: "edit", token: delegation.json?.token })}\n`, { mode: 0o600 });
  chmodSync(path, 0o600);
  return { delegation, path };
};

const createDocument = async (human: string, spaceId: string, want: string, name: string) => {
  const catalog = await hub("GET", `/spaces/${spaceId}/artifact-creations`, human);
  const kinds = (catalog.json?.kinds ?? catalog.json?.entries ?? catalog.json?.rows ?? []) as any[];
  const kind = kinds.find((entry) => JSON.stringify(entry).includes(want));
  if (!kind) return { documentId: "", detail: `no creatable kind matches ${want} in ${JSON.stringify(kinds.map((entry) => entry.kindId))}` };
  const { sealSpaceArtifactCreateV1 } = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts");
  const requestId = randomBytes(16).toString("hex");
  const accepted = await hub("POST", `/spaces/${spaceId}/artifact-creations`, human, sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: catalog.json?.catalogGenerationId, kindId: kind.kindId, name }));
  for (let poll = 0; poll < 300; poll++) {
    const status = (await hub("GET", `/spaces/${spaceId}/artifact-creations/${requestId}`, human)).json ?? {};
    if (status.phase !== "accepted" && status.phase !== "preparing") {
      const documentId = String(status.ready?.artifactId ?? "");
      return { documentId, detail: `kind=${kind.kindId} accepted=${accepted.status} phase=${status.phase} ${JSON.stringify(status).slice(0, 300)}` };
    }
    await sleep(1000);
  }
  return { documentId: "", detail: `creation ${requestId} never left preparing` };
};

const follow = async (agent: Agent, handle: string, until: (job: any) => boolean, budgetMs: number) => {
  let cursor = 0;
  const seen: any[] = [];
  const deadline = Date.now() + budgetMs;
  let job: any;
  while (Date.now() < deadline) {
    const page = await agent.call("inference_events", { jobHandle: handle, after: cursor });
    job = page.structuredContent?.job;
    if (!job) return { job: page.structuredContent, seen };
    seen.push(...(job.events ?? []), ...(job.progress ?? []).map((row: any) => ({ progress: row.fraction, message: row.message })));
    if (job.site === "guest") cursor = job.nextCursor;
    if (until(job)) break;
    await sleep(500);
  }
  return { job, seen };
};

let agentA: Agent | undefined;
let agentB: Agent | undefined;
let human = "";
const delegations: string[] = [];
try {
  const signIn = await hub("POST", "/auth/sessions", undefined, { schema: "semio.hub.auth.credential-sign-in/v1", email: EMAIL, password: PASSWORD, deviceInstanceId: `g10human${randomBytes(12).toString("hex")}`, clientClass: "browser" });
  human = String(signIn.json?.token ?? "");
  const readiness = await hub("GET", "/readyz");
  row("0 the hub publishes the inference services it executes", Array.isArray(readiness.json?.features?.inferenceServices), `features.inferenceServices=${JSON.stringify(readiness.json?.features?.inferenceServices)}`);
  row("1 human signs in", signIn.status === 200 && human.length > 0, `HTTP ${signIn.status}`);
  const { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts");
  const { createSpaceCommandV1 } = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts");
  const spaceName = `G10 quartet ${randomBytes(3).toString("hex")}`;
  await hub("POST", "/directory/commands", human, JSON.parse(directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(spaceName, "atelier", "private")))));
  const spaces = await hub("GET", "/directory/spaces", human);
  const spaceId = String((spaces.json ?? []).find((entry: any) => entry?.space?.name === spaceName)?.space?.id ?? "");
  const grid = await createDocument(human, spaceId, "wfcgrid3d", "G10 wfc grid3d solve");
  row("2 human creates a wfc grid3d document on the hub", grid.documentId.length > 0, grid.detail);
  const map = await createDocument(human, spaceId, "gis.gismap", "G10 gis map");
  row("3 human creates a gis map document on the hub", map.documentId.length > 0, map.detail);

  const a = await delegate(human, spaceId, "G10 agent A");
  const b = await delegate(human, spaceId, "G10 agent B");
  delegations.push(String(a.delegation.json?.delegationId ?? ""), String(b.delegation.json?.delegationId ?? ""));
  row("4 human delegates two agents", a.delegation.status === 201 && b.delegation.status === 201, `A=${a.delegation.json?.agentPrincipalId} B=${b.delegation.json?.agentPrincipalId}`);
  agentA = new Agent("A", spaceId, a.path);
  agentB = new Agent("B", spaceId, b.path);
  await agentA.start();
  await agentB.start();
  for (const documentId of [grid.documentId, map.documentId]) {
    for (let poll = 0; poll < 60 && !(await agentA.documents()).some((entry) => entry?.scope?.documentId === documentId); poll++) await sleep(1000);
    for (let poll = 0; poll < 60 && !(await agentB.documents()).some((entry) => entry?.scope?.documentId === documentId); poll++) await sleep(1000);
  }

  const listedGrid = await agentA.call("inference_list", { artifactId: grid.documentId });
  const listedMap = await agentA.call("inference_list", { artifactId: map.documentId });
  const solve = (listedGrid.structuredContent?.declared ?? []).find((item: any) => item.inferenceSchema === "s.wfc.grid3d.solve");
  const gisService = (listedMap.structuredContent?.declared ?? [])[0];
  row("5 inference_list names each service, its execution site and its commit binding", solve !== undefined && gisService !== undefined, `grid3d=${JSON.stringify({ schema: solve?.inferenceSchema, site: solve?.site ?? solve?.executionSite, commit: solve?.payload?.commit ?? null })} gis=${JSON.stringify({ schema: gisService?.inferenceSchema, site: gisService?.site ?? gisService?.executionSite, commit: gisService?.payload?.commit ?? null })}`);

  const openedB = await agentB.call("artifact_open", { artifactId: map.documentId });
  const historyBefore = await agentB.request("resources/read", { uri: `semio://artifact/${map.documentId}/history` });
  const ledgerBefore = await hub("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(map.documentId)}`, human);
  row("6 agent B holds the gis map open before anything is committed", openedB.isError !== true, `writePath=${openedB.structuredContent?.sessionDocument?.writePath} head_seq=${ledgerBefore.json?.head_seq}`);

  const submitted = await agentA.call("inference_submit", { documentId: grid.documentId });
  const solveHandle = String(submitted.structuredContent?.jobHandle ?? "");
  row("7 inference_submit starts the grid3d solve in the plugin's guest", submitted.isError !== true && submitted.structuredContent?.job?.site === "guest", `handle=${solveHandle} state=${submitted.structuredContent?.job?.state} ${submitted.isError ? JSON.stringify(submitted.structuredContent).slice(0, 400) : ""}`);
  const cancelled = await agentA.call("inference_submit", { documentId: grid.documentId, workUnits: 64 });
  const cancelHandle = String(cancelled.structuredContent?.jobHandle ?? "");
  await follow(agentA, cancelHandle, (job) => (job.progress ?? []).length > 0 || job.state !== "accepted", 60_000);
  const cancel = await agentA.call("inference_cancel", { jobHandle: cancelHandle });
  const cancelledJob = await follow(agentA, cancelHandle, (job) => ["cancelled", "succeeded", "failed"].includes(job.state), 120_000);
  row("8 inference_cancel stops a second grid3d solve", cancel.isError !== true && cancelledJob.job?.state === "cancelled", `state=${cancelledJob.job?.state} events=${JSON.stringify((cancelledJob.job?.events ?? []).map((event: any) => event.kind))} ${cancel.isError ? JSON.stringify(cancel.structuredContent).slice(0, 300) : ""}`);
  const solved = await follow(agentA, solveHandle, (job) => ["succeeded", "failed", "cancelled"].includes(job.state), 900_000);
  row("9 inference_events streams the grid3d solve to its result; no commit binding means no proposal", solved.job?.state === "succeeded" && solved.job?.proposalState === "none", `state=${solved.job?.state}/${solved.job?.proposalState} events=${JSON.stringify(solved.seen.filter((entry: any) => entry.kind).map((entry: any) => entry.kind))} progressRows=${solved.seen.filter((entry: any) => entry.progress !== undefined).length} resultKeys=${JSON.stringify(Object.keys(solved.job?.result ?? {}))} error=${JSON.stringify(solved.job?.error ?? null).slice(0, 300)}`);
  const unbound = await agentA.call("inference_approve", { jobHandle: solveHandle, proposalHash: "0".repeat(64) });
  row("10 inference_approve refuses a job that has no proposal", unbound.isError === true, JSON.stringify(unbound.structuredContent ?? unbound.content).slice(0, 300));

  const withdrawn = await agentA.call("inference_submit", { documentId: map.documentId });
  const withdrawnHandle = String(withdrawn.structuredContent?.jobHandle ?? "");
  row("11 inference_submit relays the gis inference to the hub that executes it", withdrawn.isError !== true && withdrawn.structuredContent?.job?.site === "hub", `site=${withdrawn.structuredContent?.job?.site} state=${withdrawn.structuredContent?.job?.state}/${withdrawn.structuredContent?.job?.proposalState} ${withdrawn.isError ? JSON.stringify(withdrawn.structuredContent).slice(0, 400) : ""}`);
  const firstOffer = await follow(agentA, withdrawnHandle, (job) => job.proposalState !== "none" || !["accepted", "running"].includes(job.state), 300_000);
  const gisCancel = await agentA.call("inference_cancel", { jobHandle: withdrawnHandle });
  row("12 inference_cancel withdraws the hub job's offer", gisCancel.isError !== true && gisCancel.structuredContent?.job?.cancelRequested === true, `before=${firstOffer.job?.state}/${firstOffer.job?.proposalState} after=${gisCancel.structuredContent?.job?.state}/${gisCancel.structuredContent?.job?.proposalState} ${gisCancel.isError ? JSON.stringify(gisCancel.structuredContent).slice(0, 300) : ""}`);

  const gis = await agentA.call("inference_submit", { documentId: map.documentId });
  const gisHandle = String(gis.structuredContent?.jobHandle ?? "");
  const offered = await follow(agentA, gisHandle, (job) => job.proposalState === "offered" || ["failed", "cancelled"].includes(job.state), 300_000);
  row("13 inference_events streams the hub job's events and progress to an offered proposal", offered.job?.proposalState === "offered" && (offered.job?.events ?? []).length > 0, `state=${offered.job?.state}/${offered.job?.proposalState} events=${JSON.stringify((offered.job?.events ?? []).map((event: any) => event.kind))} progress=${JSON.stringify((offered.job?.progress ?? []).map((entry: any) => entry.fraction))} hash=${offered.job?.proposal?.hash} error=${JSON.stringify(offered.job?.error ?? null).slice(0, 300)}`);
  const approved = await agentA.call("inference_approve", { jobHandle: gisHandle, proposalHash: offered.job?.proposal?.hash });
  row("14 inference_approve commits the hub proposal through the normal edit path", approved.isError !== true && approved.structuredContent?.job?.proposalState === "approved", `state=${approved.structuredContent?.job?.state}/${approved.structuredContent?.job?.proposalState} commit=${JSON.stringify(approved.structuredContent?.job?.commit ?? approved.structuredContent).slice(0, 400)}`);
  const replay = await agentA.call("inference_approve", { jobHandle: gisHandle, proposalHash: offered.job?.proposal?.hash });
  row("15 a second approval of the same proposal is refused", replay.isError === true, JSON.stringify(replay.structuredContent ?? replay.content).slice(0, 200));

  let ledgerAfter = await hub("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(map.documentId)}`, human);
  for (let poll = 0; poll < 60 && Number(ledgerAfter.json?.head_seq ?? 0) <= Number(ledgerBefore.json?.head_seq ?? 0); poll++) {
    await sleep(1000);
    ledgerAfter = await hub("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(map.documentId)}`, human);
  }
  row("16 the hub's document ledger advanced with the committed edit", Number(ledgerAfter.json?.head_seq ?? 0) > Number(ledgerBefore.json?.head_seq ?? 0), `head_seq ${ledgerBefore.json?.head_seq}→${ledgerAfter.json?.head_seq} commit_seq ${ledgerBefore.json?.commit_seq}→${ledgerAfter.json?.commit_seq}`);

  let observed: any;
  for (let poll = 0; poll < 60; poll++) {
    await agentB.call("artifact_open", { artifactId: map.documentId });
    observed = await agentB.request("resources/read", { uri: `semio://artifact/${map.documentId}/history` });
    if (String(observed.result?.contents?.[0]?.text ?? "") !== String(historyBefore.result?.contents?.[0]?.text ?? "")) break;
    await sleep(1000);
  }
  const before = JSON.parse(String(historyBefore.result?.contents?.[0]?.text ?? "{}"));
  const after = JSON.parse(String(observed?.result?.contents?.[0]?.text ?? "{}"));
  row("17 agent B, a second client, observes the committed edit", (after.appliedEditIds ?? []).length > (before.appliedEditIds ?? []).length, `appliedEditIds ${(before.appliedEditIds ?? []).length}→${(after.appliedEditIds ?? []).length}`);
  row("18 progress notifications reached the submitting client", agentA.progress.length > 0, `notifications/progress rows=${agentA.progress.length}`);
} catch (error) {
  row("proof", false, error instanceof Error ? `${error.message}\n${error.stack}` : String(error));
} finally {
  writeFileSync(join(OUT, "g10-quartet-live-transcript.json"), JSON.stringify({ rows, transcript, progressA: agentA?.progress, stderrA: agentA?.stderr.slice(-6000), stderrB: agentB?.stderr.slice(-3000) }, null, 2));
  agentA?.stop();
  agentB?.stop();
  for (const id of delegations) await hub("DELETE", `/auth/agent-delegations/${encodeURIComponent(id)}`, human).catch(() => undefined);
  const red = rows.filter((entry) => !entry.ok).length;
  console.log(`g10-quartet-live: ${rows.length - red}/${rows.length} rows green against ${ORIGIN}`);
  process.exit(red === 0 ? 0 : 1);
}
