import { createHash } from "node:crypto";
import { spawn } from "node:child_process";
import { closeSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, writeFileSync, writeSync } from "node:fs";
import { join, relative, resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../../../../../..");
const ticketRelative = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION";
const ticketRoot = join(repoRoot, ticketRelative);
const runParent = join(import.meta.dir, "🧾️runs");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const baseline = "9f449b10659b95148c8bcb3f91ce583bf7446973";
const schemaPath = `${library}/🔣️taxonomy.json`;
const paths = ["📜️script.ts", schemaPath, `${library}/🔍️discovery/🟦️component.ts`, `${library}/🧹️normalization/🟦️.ts`, JSON.parse(readFileSync(join(repoRoot, schemaPath), "utf8")).semanticPackageProjectionContracts["nested-cargo-packages-v1"].authorityCatalogPath];
const fingerprint = () => paths.map((path) => {
  if (typeof path !== "string" || path.includes("\\") || path.split("/").some((segment) => ["", ".", ".."].includes(segment)) || ["compose", "temp/compose"].some((opaque) => path === opaque || path.startsWith(`${opaque}/`))) throw new Error("Unsafe capture authority path");
  const segments = path.split("/");
  for (let index = 1; index < segments.length; index++) { const parent = lstatSync(join(repoRoot, ...segments.slice(0, index))); if (!parent.isDirectory() || parent.isSymbolicLink()) throw new Error(`Unsafe capture authority ancestor: ${path}`); }
  const absolute = join(repoRoot, path), state = lstatSync(absolute);
  if (!state.isFile() || state.isSymbolicLink()) throw new Error(`Unsafe capture authority: ${path}`);
  const bytes = readFileSync(absolute);
  return { path, bytes: bytes.byteLength, sha256: createHash("sha256").update(bytes).digest("hex") };
});

if (process.cwd() !== repoRoot || import.meta.dir !== join(ticketRoot, "📓️energy-reference-capture")) throw new Error("Energy capture requires its exact repository and ticket owner");
if (process.argv[2] === "preflight") {
  console.log(JSON.stringify({ baseline, runParent, authorities: fingerprint(), mutatesWorkspace: false }));
  process.exit(0);
}
if (process.argv[2] !== "capture") throw new Error("Expected preflight or capture");
let parent = repoRoot;
for (const segment of relative(repoRoot, runParent).split(/[\\/]/u)) {
  parent = join(parent, segment);
  try { const state = lstatSync(parent); if (!state.isDirectory() || state.isSymbolicLink()) throw new Error(`Unsafe capture owner: ${parent}`); }
  catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; mkdirSync(parent); }
}
const runRoot = mkdtempSync(join(runParent, "🔖️"));
const planPath = join(runRoot, "🔣️.json"), cancelPath = join(runRoot, "⛔️cancel"), logPath = join(runRoot, "📋️output.txt");
const startedAt = new Date().toISOString(), before = fingerprint(), log = openSync(logPath, "wx");
const args = [join(repoRoot, "node_modules/nx/bin/nx.js"), "run", "workspace:clean-taxonomy-plan", "--skip-nx-cache", "--", "--ticket", "26/08/17/END-TO-END-TAXONOMY-NORMALIZATION", "--baseline", baseline, "--scope", "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model", "--plan", relative(repoRoot, planPath), "--workers", "1", "--cancel-file", relative(repoRoot, cancelPath)];
writeFileSync(join(runRoot, "🔎️before.json"), JSON.stringify({ startedAt, before, args, planPath, cancelPath }, null, 2));
writeFileSync(join(runRoot, "📝️.md"), `# Energy Reference Capture Run\n\nStarted ${startedAt}. This new observational run uses baseline ${baseline}. It does not replace any missing historical capture. The plan, cancellation sentinel, lossless log and identity records belong only to this run. All outputs remain retained while the process or transaction evidence is active.\n`, { flag: "wx" });
console.log("[DEBUG] Energy capture started", JSON.stringify({ startedAt, runRoot, planPath, cancelPath }));
let tail = "", pending = "";
const lastProgress = new Map<string, number>();
const receive = (chunk: Buffer): void => {
  writeSync(log, chunk);
  const text = chunk.toString().replace(/\x1b\[[0-9;]*m/gu, "");
  tail = (tail + text).slice(-12000);
  pending += text;
  let newline: number;
  while ((newline = pending.indexOf("\n")) >= 0) {
    const line = pending.slice(0, newline); pending = pending.slice(newline + 1);
    const progress = line.match(/\[clean taxonomy progress\] (\S+) (\S+) (\d+)\/(\d+)/u);
    if (!progress) { if (/error|failed|unresolved|cancel|digest|success/iu.test(line)) console.log(line); continue; }
    const key = `${progress[1]}/${progress[2]}`, bucket = Math.floor(Number(progress[3]) / 10000);
    if (lastProgress.get(key) !== bucket || progress[3] === progress[4]) { lastProgress.set(key, bucket); console.log(line); }
  }
};
const child = spawn(process.execPath, args, { cwd: repoRoot, env: { ...process.env, NX_DAEMON: "false" }, stdio: ["ignore", "pipe", "pipe"] });
child.stdout.on("data", receive);
child.stderr.on("data", receive);
process.on("SIGINT", () => { try { writeFileSync(cancelPath, "cancel\n", { flag: "wx" }); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error; } console.log("[DEBUG] Energy capture cancellation requested"); });
const outcome = await new Promise<{ code: number | null; signal: string | null }>((resolve, reject) => { child.once("error", reject); child.once("close", (code, signal) => resolve({ code, signal })); }).finally(() => closeSync(log));
const after = fingerprint(), finishedAt = new Date().toISOString();
writeFileSync(join(runRoot, "🔎️after.json"), JSON.stringify({ finishedAt, outcome, after, stable: JSON.stringify(before) === JSON.stringify(after) }, null, 2));
writeFileSync(join(runRoot, "📝️.md"), `# Energy Reference Capture Run\n\nStarted ${startedAt}; finished ${finishedAt}. Child exit code ${outcome.code}; signal ${outcome.signal ?? "none"}. Before/after observed authority stability: ${JSON.stringify(before) === JSON.stringify(after)}. Baseline: ${baseline}.\n\nThis was an actual uncached workspace:clean-taxonomy-plan invocation, not an apply. Success requires a separate review of the requested plan artifact; an exit code alone is not acceptance. Exact command and input observations are in the adjacent identity records; complete child output is in the adjacent log. This new run does not reconstruct missing historical evidence. No output was deleted.\n`);
console.log("[DEBUG] Energy capture finished", JSON.stringify({ finishedAt, outcome, runRoot, stable: JSON.stringify(before) === JSON.stringify(after), tail }));
process.exitCode = outcome.code ?? 1;
