import { createHash } from "node:crypto";
import { spawn, spawnSync } from "node:child_process";
import { closeSync, lstatSync, openSync, readFileSync, readSync, writeFileSync, writeSync } from "node:fs";
import { join, relative } from "node:path";
import { StringDecoder } from "node:string_decoder";

const repoRoot = process.cwd(), runRoot = import.meta.dir;
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const baseline = "9f449b10659b95148c8bcb3f91ce583bf7446973";
const schemaPath = library + "/🔣️taxonomy.json";
const scopeCatalogPath = library + "/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json";
const hash = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
const noFollowRead = (path: string): Buffer => {
  if (typeof path !== "string" || path.includes("\\") || path.split("/").some((part) => ["", ".", ".."].includes(part)) || ["compose", "temp/compose"].some((opaque) => path === opaque || path.startsWith(opaque + "/"))) throw new Error("Unsafe capture identity path");
  const parts = path.split("/");
  for (let index = 1; index <= parts.length; index++) {
    const stat = lstatSync(join(repoRoot, ...parts.slice(0, index)));
    if (stat.isSymbolicLink() || !(index === parts.length ? stat.isFile() : stat.isDirectory())) throw new Error("Unsafe capture identity ancestor: " + path);
  }
  return readFileSync(join(repoRoot, path));
};
const fingerprint = () => {
  const schema = JSON.parse(noFollowRead(schemaPath).toString("utf8"));
  const contract = schema.semanticPackageProjectionContracts["nested-cargo-packages-v1"];
  const paths = ["📜️script.ts", "📋️project.json", schemaPath, library + "/🔍️discovery/🟦️component.ts", library + "/🧹️normalization/🟦️.ts", contract.authorityCatalogPath, scopeCatalogPath, relative(repoRoot, join(runRoot, "📜️script.ts")).replaceAll("\\", "/")];
  const files = paths.map((path) => { const bytes = noFollowRead(path); return { path, bytes: bytes.byteLength, sha256: hash(bytes) }; });
  return { files, catalogPath: contract.authorityCatalogPath, declaredCatalogSha256: contract.authorityCatalogSha256, catalogMatchesDeclaration: files.find((file) => file.path === contract.authorityCatalogPath)?.sha256 === contract.authorityCatalogSha256 };
};
if (process.argv[2] !== "capture") throw new Error("Expected capture");
const baselineType = spawnSync("git", ["cat-file", "-t", baseline], { cwd: repoRoot, encoding: "utf8" });
if (baselineType.status !== 0 || baselineType.stdout.trim() !== "commit") throw new Error("Baseline is not a commit");
const scopeBytes = noFollowRead(scopeCatalogPath);
if (hash(scopeBytes) !== "1410a74ccc87561fd4a4b91db7d503614fe21ddce8bc78dee923d8237820f3e0") throw new Error("Draw scope catalog preimage drift");
const scope = JSON.parse(scopeBytes.toString("utf8")).projections[1].sourceRoot;
const planPath = join(runRoot, "🔣️.json"), cancelPath = join(runRoot, "⛔️cancel"), logPath = join(runRoot, "📋️output.log");
const startedAt = new Date().toISOString(), started = performance.now(), before = fingerprint();
if (!before.catalogMatchesDeclaration) throw new Error("Current package catalog does not match schema");
const args = [join(repoRoot, "node_modules/nx/bin/nx.js"), "run", "workspace:clean-taxonomy-plan", "--skip-nx-cache", "--", "--ticket", "26/08/17/END-TO-END-TAXONOMY-NORMALIZATION", "--baseline", baseline, "--scope", scope, "--plan", relative(repoRoot, planPath), "--workers", "1", "--cancel-file", relative(repoRoot, cancelPath)];
writeFileSync(join(runRoot, "🔎️before.json"), JSON.stringify({ schemaVersion: 1, startedAt, baseline, scope, before, args, planPath, cancelPath, logPath }, null, 2), { flag: "wx" });
const log = openSync(logPath, "wx"), lastProgress = new Map<string, number>(), progressCounts = new Map<string, { current: number; total: number }>();
console.log("[DEBUG] Draw capture started", JSON.stringify({ startedAt, runRoot, baseline, scope, planPath, cancelPath, logPath, catalogSha256: before.declaredCatalogSha256 }));
const receiver = () => {
  const decoder = new StringDecoder("utf8");
  let pending = "";
  return (chunk: Buffer): void => {
    for (let offset = 0; offset < chunk.byteLength;) offset += writeSync(log, chunk, offset, chunk.byteLength - offset);
    pending += decoder.write(chunk).replace(/\x1b\[[0-9;]*m/gu, "");
    let newline: number;
    while ((newline = pending.indexOf("\n")) >= 0) {
      const line = pending.slice(0, newline); pending = pending.slice(newline + 1);
      const progress = line.match(/\[clean taxonomy progress\] (\S+) (\S+) (\d+)\/(\d+)/u);
      if (!progress) { if (/error|failed|unresolved|cancel|digest|success/iu.test(line)) console.log(line); continue; }
      const key = progress[1] + "/" + progress[2], current = Number(progress[3]), total = Number(progress[4]), bucket = Math.floor(current / 5000);
      progressCounts.set(key, { current, total });
      if (lastProgress.get(key) !== bucket || current === total) { lastProgress.set(key, bucket); console.log(line); }
    }
  };
};
const child = spawn(process.execPath, args, { cwd: repoRoot, env: { ...process.env, NX_DAEMON: "false" }, stdio: ["ignore", "pipe", "pipe"] });
child.stdout.on("data", receiver());
child.stderr.on("data", receiver());
process.on("SIGINT", () => {
  try { writeFileSync(cancelPath, "cancel\n", { flag: "wx" }); }
  catch (error) { if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error; }
  console.log("[DEBUG] Draw capture cancellation requested");
});
const outcome = await new Promise<{ code: number | null; signal: string | null; error?: string }>((resolve) => {
  child.once("error", (error) => resolve({ code: null, signal: null, error: error.message }));
  child.once("close", (code, signal) => resolve({ code, signal }));
}).finally(() => closeSync(log));
let after: ReturnType<typeof fingerprint> | undefined, identityError: string | undefined;
try { after = fingerprint(); } catch (error) { identityError = String(error); }
const finishedAt = new Date().toISOString(), stable = after !== undefined && JSON.stringify(before) === JSON.stringify(after);
const logSize = lstatSync(logPath).size, tailOffset = Math.max(0, logSize - 16000), buffer = Buffer.alloc(logSize - tailOffset), descriptor = openSync(logPath, "r");
try { readSync(descriptor, buffer, 0, buffer.byteLength, tailOffset); } finally { closeSync(descriptor); }
const tail = buffer.subarray(tailOffset ? Math.max(0, buffer.indexOf(10) + 1) : 0).toString("utf8");
writeFileSync(join(runRoot, "🔎️after.json"), JSON.stringify({ schemaVersion: 1, finishedAt, milliseconds: performance.now() - started, outcome, after, identityError, stable, progress: Object.fromEntries(progressCounts), logBytes: logSize, logSha256: hash(readFileSync(logPath)), tail }, null, 2), { flag: "wx" });
console.log("[DEBUG] Draw capture finished", JSON.stringify({ finishedAt, outcome, runRoot, stable, identityError, progress: Object.fromEntries(progressCounts), logBytes: logSize, tail }));
process.exitCode = outcome.code ?? 1;
