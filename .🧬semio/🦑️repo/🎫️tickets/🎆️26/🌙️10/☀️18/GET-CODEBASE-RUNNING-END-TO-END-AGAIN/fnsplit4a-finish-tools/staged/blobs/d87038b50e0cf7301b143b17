/** 💥️ The child-process control of the transaction-v2 suite: the active reference fixture, spawned children and crash drivers. */
import { terminateOwnedProcessTree } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { afterAll, expect } from "bun:test";
import { spawn, type ChildProcess } from "node:child_process";
import { appendFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { inventoryTaxonomy } from "../../../🧹️normalization/🔍️inventory-planning/🟦️.ts";
import { planTaxonomy } from "../../../🧹️normalization/🧠️planning/🟦️.ts";
import { type TaxonomyPlan } from "../../../🧹️normalization/📜️plan-contracts/🟦️.ts";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import { type BoundaryNode, type Fixture, NORMALIZATION_MODULE, type Snapshot, childEvidence, fixtureTemplates, registerChild, transactionGolden } from "../📜️boundary-contracts/🟦️.ts";
import { cloneFixture, fixtureRunRoot, journalPaths, plan, referenceFixture, retainFixture, snapshot, transactionRoot, writePlan } from "../🏗️fixture-builders/🟦️.ts";

export let activeReferenceTemplate: Fixture | undefined, activeReferenceTemplatePromise: Promise<Fixture> | undefined;

export let activeReferenceTemplateProcessOwned = true;

export async function activeReferenceFixture(name: string): Promise<Fixture> {
  activeReferenceTemplatePromise ??= (async () => {
    const sharedRoot = process.env.SEMIO_TRANSACTION_V2_RUN_ID ? join(fixtureRunRoot(), "🧪️active-reference") : undefined;
    if (sharedRoot && existsSync(sharedRoot)) {
      retainFixture(sharedRoot);
      const head = readFileSync(join(sharedRoot, ".git", "HEAD"), "utf8").trim(), baselineCommit = head.startsWith("ref: ") ? readFileSync(join(sharedRoot, ".git", head.slice(5)), "utf8").trim() : head;
      const ticketDir = join(sharedRoot, "🧪️tests"), workspace = join(ticketDir, "🧪️fixture"), scope = relative(sharedRoot, workspace).replaceAll("\\", "/");
      activeReferenceTemplateProcessOwned = false;
      activeReferenceTemplate = { baselineCommit, options: { repoRoot: sharedRoot, scope, ticketDir, workers: 1 }, planCacheKey: "active-reference-v2", repoRoot: sharedRoot, root: sharedRoot, scope, ticketDir, workspace };
      return activeReferenceTemplate;
    }
    const lock = sharedRoot ? `${sharedRoot}-preparing` : undefined;
    let ownsShared = false;
    if (lock) {
      try { mkdirSync(lock); ownsShared = true; }
      catch (error) { if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error; }
      if (!ownsShared) {
        const deadline = Date.now() + 10_000;
        while (!existsSync(sharedRoot!)) {
          if (Date.now() >= deadline) throw new Error("Timed out waiting for the shared active reference fixture");
          await Bun.sleep(2);
        }
        retainFixture(sharedRoot!);
        const head = readFileSync(join(sharedRoot!, ".git", "HEAD"), "utf8").trim(), baselineCommit = head.startsWith("ref: ") ? readFileSync(join(sharedRoot!, ".git", head.slice(5)), "utf8").trim() : head;
        const ticketDir = join(sharedRoot!, "🧪️tests"), workspace = join(ticketDir, "🧪️fixture"), scope = relative(sharedRoot!, workspace).replaceAll("\\", "/");
        activeReferenceTemplateProcessOwned = false;
        activeReferenceTemplate = { baselineCommit, options: { repoRoot: sharedRoot!, scope, ticketDir, workers: 1 }, planCacheKey: "active-reference-v2", repoRoot: sharedRoot!, root: sharedRoot!, scope, ticketDir, workspace };
        return activeReferenceTemplate;
      }
    }
    const row = referenceFixture("active-template"), value = plan(row);
    await killedAt(row, value, "transaction-wal-prepared");
    if (!sharedRoot) { activeReferenceTemplate = row; return row; }
    renameSync(row.root, sharedRoot);
    retainFixture(lock!);
    const ticketDir = join(sharedRoot, "🧪️tests"), workspace = join(ticketDir, "🧪️fixture"), scope = relative(sharedRoot, workspace).replaceAll("\\", "/");
    activeReferenceTemplateProcessOwned = false;
    activeReferenceTemplate = { ...row, options: { ...row.options, repoRoot: sharedRoot, scope, ticketDir }, planCacheKey: "active-reference-v2", repoRoot: sharedRoot, root: sharedRoot, scope, ticketDir, workspace };
    return activeReferenceTemplate;
  })();
  return cloneFixture(await activeReferenceTemplatePromise, name);
}

afterAll(() => { for (const template of fixtureTemplates.values()) if (template.processOwned) retainFixture(template.root); if (activeReferenceTemplate && activeReferenceTemplateProcessOwned) retainFixture(activeReferenceTemplate.root); });

const CHILD_SOURCE = `process.env.NX_INVOCATION_ROOT_PID=String(process.pid);const [modulePath,planPath,repoRoot,ticketDir,baseline,resumeJournal,phase,marker,inject]=process.argv.slice(1);const {applyTaxonomyPlan}=await import(modulePath);const plan=JSON.parse(await Bun.file(planPath).text());applyTaxonomyPlan(plan,{repoRoot,ticketDir,expectedBaselineCommit:baseline,resumeJournal:resumeJournal||undefined,injectFailureAt:inject||undefined,progress:(row)=>{if(row.phase===phase){require("node:fs").writeFileSync(marker,"ready\\n");Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,30000);}}});`;

function child(row: Fixture, value: TaxonomyPlan, phase: string, marker: string, resumeJournal = "", inject = "", env: Partial<NodeJS.ProcessEnv> = {}): ChildProcess {
  const planPath = writePlan(row, value);
  return registerChild(spawn(process.execPath, ["-e", CHILD_SOURCE, NORMALIZATION_MODULE, planPath, row.repoRoot, row.ticketDir, row.baselineCommit, resumeJournal, phase, marker, inject], { detached: process.platform !== "win32", env: { ...process.env, ...env }, stdio: ["ignore", "pipe", "pipe"] }));
}

function assertChildWaiting(childProcess: ChildProcess | undefined, path: string): void {
  if (!childProcess) return;
  const evidence = childEvidence.get(childProcess);
  if (evidence?.error || childProcess.exitCode !== null || childProcess.signalCode !== null) throw new Error(`Child ended before marker ${path}: ${evidence?.error?.message ?? childProcess.signalCode ?? childProcess.exitCode}\n${evidence?.output ?? ""}`);
}

export async function waitFor(path: string, timeoutMs = 10_000, childProcess?: ChildProcess): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (!existsSync(path)) {
    assertChildWaiting(childProcess, path);
    if (Date.now() >= deadline) throw new Error(`Timed out waiting for ${path}`);
    await Bun.sleep(2);
  }
}

async function waitForGeneratorPid(path: string, childProcess: ChildProcess, timeoutMs = 12_000): Promise<number> {
  const deadline = Date.now() + timeoutMs;
  for (;;) {
    assertChildWaiting(childProcess, path);
    if (existsSync(path)) {
      try {
        const pid = (JSON.parse(readFileSync(path, "utf8")) as { pid?: unknown }).pid;
        if (Number.isSafeInteger(pid) && (pid as number) > 0) return pid as number;
      } catch {}
    }
    if (Date.now() >= deadline) throw new Error(`Timed out waiting for a complete generator marker at ${path}\n${childEvidence.get(childProcess)?.output ?? ""}`);
    await Bun.sleep(2);
  }
}

export async function killedAt(row: Fixture, value: TaxonomyPlan, phase: string, resumeJournal = "", inject = ""): Promise<Readonly<{ transaction: Snapshot; workspace: Snapshot }>> {
  const marker = join(row.root, `marker-${phase}-${crypto.randomUUID()}`), childProcess = child(row, value, phase, marker, resumeJournal, inject);
  try {
    await waitFor(marker, 10_000, childProcess);
    const workspace = snapshot(row.workspace), transaction = snapshot(transactionRoot(row));
    expect(Object.values(transaction).some((entry) => entry.startsWith("directory|"))).toBe(true);
    killTree(childProcess);
    const exit = await boundedExit(childProcess);
    expect(exit.signal === "SIGKILL" || exit.code !== 0).toBe(true);
    return { transaction, workspace };
  } finally {
    if (childProcess.exitCode === null && childProcess.signalCode === null) { killTree(childProcess); await boundedExit(childProcess); }
  }
}

export function killTree(childProcess: ChildProcess): void {
  if (!childProcess.pid) throw new Error("Child process has no pid");
  terminateOwnedProcessTree(childProcess.pid);
}

export function boundedExit(childProcess: ChildProcess, timeoutMs = 10_000): Promise<Readonly<{ code: number | null; signal: NodeJS.Signals | null }>> {
  if (childProcess.exitCode !== null || childProcess.signalCode !== null) return Promise.resolve({ code: childProcess.exitCode, signal: childProcess.signalCode });
  return new Promise((resolveExit, rejectExit) => {
    let timedOut = false;
    const timer = setTimeout(() => { timedOut = true; killTree(childProcess); }, timeoutMs);
    childProcess.once("exit", (code, signal) => { clearTimeout(timer); if (timedOut) rejectExit(new Error(`Child ${childProcess.pid ?? "unknown"} exit timed out`)); else resolveExit({ code, signal }); });
    childProcess.once("error", (error) => { clearTimeout(timer); rejectExit(error); });
  });
}

async function waitForExit(pid: number): Promise<void> {
  const deadline = Date.now() + 5_000;
  for (;;) {
    try { process.kill(pid, 0); }
    catch (error) { if ((error as NodeJS.ErrnoException).code === "ESRCH") return; throw error; }
    if (Date.now() >= deadline) throw new Error(`Descendant pid ${pid} remained alive after process-tree kill`);
    await Bun.sleep(2);
  }
}

export async function killedMixedGenerator(row: Fixture, value: TaxonomyPlan): Promise<Readonly<{ journal: string; transaction: Snapshot; workspace: Snapshot }>> {
  const markerRoot = mkdtempSync(join(fixtureRunRoot(), "🧪️mixed-generator-marker-")), marker = join(markerRoot, "ready");
  const planPath = writePlan(row, value), childProcess = registerChild(spawn(process.execPath, ["-e", `process.env.NX_INVOCATION_ROOT_PID=String(process.pid);const [m,p,r,t,b]=process.argv.slice(1);const {applyTaxonomyPlan}=await import(m);applyTaxonomyPlan(JSON.parse(await Bun.file(p).text()),{repoRoot:r,ticketDir:t,expectedBaselineCommit:b});`, NORMALIZATION_MODULE, planPath, row.repoRoot, row.ticketDir, row.baselineCommit], { detached: process.platform !== "win32", env: { ...process.env, MIXED_GENERATOR_MARKER: marker }, stdio: ["ignore", "pipe", "pipe"] }));
  try {
    const generatorPid = await waitForGeneratorPid(marker, childProcess);
    if (process.env.SEMIO_TRANSACTION_V2_MIXED_READY) writeFileSync(process.env.SEMIO_TRANSACTION_V2_MIXED_READY, "ready\n");
    const workspace = snapshot(row.workspace), transaction = snapshot(transactionRoot(row));
    killTree(childProcess);
    const exit = await boundedExit(childProcess);
    expect(exit.signal === "SIGKILL" || exit.code !== 0).toBe(true);
    await waitForExit(generatorPid);
    const journals = journalPaths(row);
    expect(journals).toHaveLength(1);
    return { journal: journals[0], transaction, workspace };
  } finally {
    if (childProcess.exitCode === null && childProcess.signalCode === null) { killTree(childProcess); await boundedExit(childProcess); }
  }
}

export function attemptJournal(row: Fixture, value: TaxonomyPlan, ordinal = 1): string {
  return join(transactionRoot(row), `🔖️${value.planDigest}`, "🔂️attempts", `🔢️${String(ordinal).padStart(6, "0")}`, "🔣️.json");
}

export function expectCompleteSnapshot(value: Snapshot): void {
  const entries = Object.entries(value);
  expect(entries.length).toBeGreaterThan(0);
  expect(entries.every(([path, record]) => path.normalize("NFC") === path && /^(?:directory\|[0-7]+|file\|[0-7]+\|[A-Za-z0-9+/=]*|symlink\|[0-7]+\|[A-Za-z0-9+/=]*)$/u.test(record))).toBe(true);
  expect(JSON.parse(JSON.stringify(value))).toEqual(value);
}

function normalizedBoundaryPath(path: string, root: string): string {
  return path.replaceAll(root, "<root>")
    .replace(/[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}/gu, "<uuid>")
    .replace(/-([1-9][0-9]*)-<uuid>/gu, "-<pid>-<uuid>");
}

function normalizedBoundaryJson(value: unknown, key = "", root = ""): unknown {
  if (key === "pid" && typeof value === "number") return "<pid>";
  if (key === "token" && typeof value === "string" && /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/u.test(value)) return "<uuid>";
  if (typeof value === "string") {
    if (key === "contentHash" || key === "targetHash") return value;
    return normalizedBoundaryPath(value, root);
  }
  if (Array.isArray(value)) return value.map((entry) => normalizedBoundaryJson(entry, key, root));
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value as Record<string, unknown>).sort(([left], [right]) => Buffer.from(left).compare(Buffer.from(right))).map(([childKey, entry]) => [normalizedBoundaryPath(childKey, root), normalizedBoundaryJson(entry, childKey, root)]));
  return value;
}

function normalizedBoundaryLedger(value: Snapshot, root: string): BoundaryNode[] {
  return Object.entries(value).map(([path, record]): BoundaryNode => {
    const [kind, mode = "", bytesBase64 = ""] = record.split("|");
    const normalizedPath = normalizedBoundaryPath(path, root);
    if (kind === "directory") return { path: normalizedPath, kind: "directory", mode };
    const bytes = Buffer.from(bytesBase64, "base64");
    if (kind === "symlink") {
      const target = normalizedBoundaryPath(bytes.toString("utf8"), root);
      return { path: normalizedPath, kind: "symlink", mode, target, targetHash: new Bun.CryptoHasher("sha256").update(target).digest("hex") };
    }
    if (path.endsWith(".json")) {
      const parsed = JSON.parse(bytes.toString("utf8")), normalized = normalizedBoundaryJson(parsed, "", root) as Record<string, unknown>;
      const canonical = `${canonicalJson(normalized)}\n`;
      return { path: normalizedPath, kind: "file", mode, size: Buffer.byteLength(canonical), sha256: new Bun.CryptoHasher("sha256").update(canonical).digest("hex"), bytesBase64: Buffer.from(canonical).toString("base64"), canonicalJson: bytes.toString("utf8") === `${canonicalJson(parsed)}\n`, json: normalized };
    }
    return { path: normalizedPath, kind: "file", mode, size: bytes.length, sha256: new Bun.CryptoHasher("sha256").update(bytes).digest("hex"), bytesBase64 };
  }).sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
}

function boundaryLedger(value: Snapshot, root: string): Readonly<{ digest: string; nodes: readonly BoundaryNode[] }> {
  const nodes = normalizedBoundaryLedger(value, root);
  return { digest: new Bun.CryptoHasher("sha256").update(canonicalJson(nodes)).digest("hex"), nodes };
}

export function expectBoundaryGolden(transaction: Snapshot, workspace: Snapshot, key: string, root: string): void {
  const transactionLedger = boundaryLedger(transaction, root), workspaceLedger = boundaryLedger(workspace, root), capture = process.env.SEMIO_CAPTURE_TRANSACTION_BOUNDARIES;
  if (capture) {
    mkdirSync(capture, { recursive: true });
    const filename = `${new Bun.CryptoHasher("sha256").update(key).digest("hex")}.json`;
    writeFileSync(join(capture, filename), `${canonicalJson({ key, transaction: transactionLedger, workspace: workspaceLedger })}\n`);
    return;
  }
  expect(transactionGolden.boundaries[key]).toEqual({ transaction: transactionLedger.digest, workspace: workspaceLedger.digest });
  expect(transactionGolden.transactionLedgers[transactionLedger.digest]).toEqual(transactionLedger.nodes);
  expect(transactionGolden.workspaceLedgers[workspaceLedger.digest]).toEqual(workspaceLedger.nodes);
  if (process.env.SEMIO_TRANSACTION_V2_BOUNDARY_REGISTRY) appendFileSync(process.env.SEMIO_TRANSACTION_V2_BOUNDARY_REGISTRY, `${key}\n`);
}

export function expectBoundaryTuple(transaction: Snapshot, workspace: Snapshot, phase: string, root: string): void {
  expectCompleteSnapshot(transaction);
  expectCompleteSnapshot(workspace);
  expectBoundaryGolden(transaction, workspace, `killed:${phase}`, root);
  const keys = Object.keys(transaction), files = keys.filter((path) => transaction[path].startsWith("file|"));
  for (const path of files.filter((entry) => entry.endsWith(".json"))) {
    const bytes = Buffer.from(transaction[path].split("|")[2], "base64").toString("utf8"), value = JSON.parse(bytes);
    expect(bytes).toBe(`${canonicalJson(value)}\n`);
  }
  const writeRoots = keys.filter((path) => transaction[path].startsWith("directory|") && /(?:^|\/)\S*write-[1-9][0-9]*-[0-9a-f-]+$/u.test(path));
  const descendants = (root: string): string[] => keys.filter((path) => path.startsWith(`${root}/`));
  const attemptPreparations = keys.filter((path) => transaction[path].startsWith("directory|") && /(?:^|\/)[^/]*prepare-000001-[^/]+$/u.test(path));
  if (phase === "transaction-attempt-preparation-mkdir") { expect(attemptPreparations).toHaveLength(1); expect(descendants(attemptPreparations[0])).toEqual([]); }
  if (phase === "transaction-attempt-preparation-children") { expect(attemptPreparations).toHaveLength(1); expect(descendants(attemptPreparations[0]).filter((path) => transaction[path].startsWith("directory|"))).toHaveLength(3); }
  if (phase.endsWith("write-mkdir")) { expect(writeRoots).toHaveLength(1); expect(descendants(writeRoots[0])).toEqual([]); }
  if (phase.endsWith("candidate-written") || phase.endsWith("write-mid") || phase.endsWith("write-prepared")) { expect(writeRoots).toHaveLength(1); expect(descendants(writeRoots[0]).filter((path) => transaction[path].startsWith("file|"))).toHaveLength(1); }
  if (phase === "transaction-journal-previous-exchanged") {
    expect(writeRoots).toHaveLength(1);
    expect(descendants(writeRoots[0]).filter((path) => path.endsWith(".json"))).toHaveLength(2);
    expect(keys.some((path) => /\/🔢️000001\/🔣️\.json$/u.test(path))).toBe(false);
  }
  if (phase === "transaction-journal-canonical-exchanged") {
    expect(writeRoots).toHaveLength(1);
    expect(descendants(writeRoots[0]).filter((path) => path.includes("⏮️"))).toHaveLength(1);
    expect(keys.some((path) => /\/🔢️000001\/🔣️\.json$/u.test(path))).toBe(true);
  }
  if (phase.includes("json-canonical-exchanged")) { expect(writeRoots).toHaveLength(1); expect(descendants(writeRoots[0])).toEqual([]); }
  if (phase === "transaction-initial-lease-prepared") { expect(writeRoots).toEqual([]); expect(keys.some((path) => path.includes("🔒️lease/🔣️.json"))).toBe(true); }
  if (phase === "transaction-initial-wal-mkdir") expect(keys.some((path) => path.endsWith("🚧️journal") && descendants(path).length === 0)).toBe(true);
  if (phase === "transaction-initial-journal-canonical") expect(keys.some((path) => path.includes("prepare-000001-") && path.endsWith("🔣️.json"))).toBe(true);
  if (phase === "transaction-attempt-canonical-published") expect(keys.some((path) => /\/🔢️000001\/🔣️\.json$/u.test(path))).toBe(true);
  if (phase === "transaction-wal-prepared") expect(keys.some((path) => path.endsWith("🚧️journal") && descendants(path).length === 0)).toBe(true);
  if (phase === "transaction-backup-inner-exchange") expect(keys.filter((path) => path.endsWith(".backup"))).toHaveLength(1);
  if (phase === "transaction-backup-exchange") { expect(writeRoots).toEqual([]); expect(keys.filter((path) => path.endsWith(".backup"))).toHaveLength(1); }
  if (phase === "transaction-backup-retained") { expect(writeRoots).toEqual([]); expect(keys.filter((path) => path.endsWith(".backup"))).toHaveLength(2); }
  if (phase === "transaction-edit-inner-exchange") expect(keys.filter((path) => path.endsWith(".edit"))).toHaveLength(1);
  if (phase === "transaction-edit-exchange") { expect(keys.filter((path) => path.endsWith(".edit"))).toHaveLength(1); expect(keys.filter((path) => path.endsWith(".pre"))).toHaveLength(1); }
  if (phase === "transaction-edit-canonical-exchange") { expect(keys.filter((path) => path.endsWith(".edit"))).toEqual([]); expect(keys.filter((path) => path.endsWith(".pre"))).toHaveLength(1); }
  if (phase === "transaction-restore-mkdir") expect(keys.filter((path) => path.includes("restore-")).some((path) => descendants(path).length === 0)).toBe(true);
  if (phase === "transaction-restore-prepared") expect(keys.filter((path) => path.endsWith(".backup"))).toHaveLength(2);
  if (phase === "transaction-restore-exchange") { expect(keys.filter((path) => path.endsWith(".backup"))).toHaveLength(2); expect(keys.filter((path) => path.endsWith(".post"))).toHaveLength(1); }
  if (phase === "transaction-restore-canonical-exchange") { expect(keys.filter((path) => path.endsWith(".backup"))).toHaveLength(1); expect(keys.filter((path) => path.endsWith(".post"))).toHaveLength(1); }
  if (phase === "transaction-lease-stale-quarantined") { expect(keys.some((path) => path.includes("-stale"))).toBe(true); expect(keys.some((path) => path.includes("🔒️lease/🔣️.json"))).toBe(false); }
  if (phase === "transaction-lease-preparation-mkdir") expect(keys.some((path) => path.includes("-preparing") && descendants(path).length === 0)).toBe(true);
  if (phase === "transaction-lease-prepared") expect(keys.some((path) => path.includes("-preparing/🔣️.json"))).toBe(true);
  if (phase === "transaction-lease-canonical-published") expect(keys.some((path) => path.includes("🔒️lease/🔣️.json"))).toBe(true);
  if (phase.includes("terminal-") && phase.endsWith("stage-removed")) {
    expect(keys.some((path) => path.includes("🚧️stage"))).toBe(false);
    expect(keys.some((path) => path.includes("💾️backup"))).toBe(true);
    expect(keys.some((path) => path.includes("🔒️lease/🔣️.json"))).toBe(true);
  }
}

export function expectTerminalAttempts(row: Fixture, state: "committed" | "rolled-back"): void {
  const journals = journalPaths(row);
  expect(journals.length).toBeGreaterThan(0);
  expect(JSON.parse(readFileSync(journals.at(-1)!, "utf8")).state).toBe(state);
  for (const journal of journals) expect(readdirSync(dirname(journal))).toEqual(["🔣️.json"]);
}

export function expectEmptyPlan(row: Fixture): void {
  const value = planTaxonomy(inventoryTaxonomy(row.options), { baselineCommit: row.baselineCommit, excludedTreeDigests: [] });
  expect({
    moves: value.moves,
    embeddedTicketRoots: value.embeddedTicketRoots,
    embeddedTicketRootRelocations: value.embeddedTicketRootRelocations,
    symlinkTargetEdits: value.symlinkTargetEdits,
    evidenceRemovals: value.evidenceRemovals,
    edits: value.edits,
    regenerations: value.regenerations,
    unresolved: value.unresolved,
  }).toEqual({ moves: [], embeddedTicketRoots: [], embeddedTicketRootRelocations: [], symlinkTargetEdits: [], evidenceRemovals: [], edits: [], regenerations: [], unresolved: [] });
}
