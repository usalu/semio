/**
 * 🚦 Applies the integrated apply series (final/manifest.json) to a repository root, group by group.
 *
 *   bun apply-all.ts                       # dry run of every group (default): verifies every anchor, writes nothing
 *   bun apply-all.ts --yes --group A       # applies group A (A, B, C in order; a later group needs the earlier ones applied)
 *   options: --series DIR (default final; round 2 is final2)
 *   bun apply-all.ts --undo A              # restores the files of an applied group from its backup
 *   options: --root DIR (default: the checkout this file lives in), --force-while-running (skip the dependency sync guard)
 *
 * Every row carries the content hash of the live preimage and of the result. A target whose live hash equals the result is already applied (skipped); equal to the preimage it is replaced; anything else is DRIFT and refuses the whole group before the first write.
 * Writes are temp-file plus rename; the replaced files are backed up first (flat names, index.json) so a group can be undone while the tree still holds exactly its result.
 */
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, join, resolve } from "node:path";

const here = import.meta.dir;
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.includes(name) ? args[args.indexOf(name) + 1] : undefined);
const root = resolve(flag("--root") ?? join(here, "../../../../../../../.."));
const yes = args.includes("--yes");
const undo = flag("--undo");
const only = flag("--group");
const series = flag("--series") ?? "final";
const manifest = JSON.parse(readFileSync(join(here, series, "manifest.json"), "utf8")) as {
  groups: { id: string; title: string; sets: string[]; verify: string[]; commands: string[][]; files: { rel: string; op: "create" | "replace"; baseSha256: string | null; resultSha256: string; file: string }[] }[];
};
const sha = (data: Buffer): string => createHash("sha256").update(data).digest("hex");
const backupDirectory = (id: string): string => join(here, "../🗑️generated/integrate-apply", id);

const chain: { series: string; group: string; rel: string; result: string }[] = [];
for (const name of ["final", "final2", "final3"]) {
  const path = join(here, name, "manifest.json");
  if (!existsSync(path)) continue;
  for (const group of (JSON.parse(readFileSync(path, "utf8")) as typeof manifest).groups) for (const row of group.files) chain.push({ series: name, group: group.id, rel: row.rel, result: row.resultSha256 });
}
const supersededBy = (rel: string, series: string, group: string): Set<string> => {
  const own = chain.findIndex((row) => row.rel === rel && row.series === series && row.group === group);
  return new Set(chain.filter((row, index) => row.rel === rel && index > own).map((row) => row.result));
};

if (!existsSync(join(root, "bunfig.toml"))) throw new Error(`not a repository root: ${root}`);

function running(): string[] {
  if (args.includes("--force-while-running")) return [];
  const listing = spawnSync("ps", ["-axo", "pid=,command="], { encoding: "utf8" }).stdout ?? "";
  return listing.split("\n").filter((line) => /(dependencies (sync|lock)|show projects)/.test(line) && !line.includes("apply-all") && !line.includes("grep")).map((line) => line.trim().slice(0, 160));
}

function plan(group: (typeof manifest.groups)[number]) {
  const todo: { row: (typeof group.files)[number]; data: Buffer; live: Buffer | undefined }[] = [], skipped: string[] = [], drift: string[] = [];
  for (const row of group.files) {
    const target = join(root, row.rel), data = readFileSync(join(here, series, row.file));
    if (sha(data) !== row.resultSha256) throw new Error(`staged copy changed since finalize: ${row.file}`);
    if (target.length > 255 && process.platform === "win32") drift.push(`path too long: ${row.rel}`);
    const live = existsSync(target) ? readFileSync(target) : undefined;
    if (live !== undefined && (sha(live) === row.resultSha256 || supersededBy(row.rel, series, group.id).has(sha(live)))) { skipped.push(row.rel); continue; }
    if (row.op === "create") { if (live === undefined) todo.push({ row, data, live }); else drift.push(`exists with other content: ${row.rel}`); continue; }
    if (live === undefined) drift.push(`missing: ${row.rel}`);
    else if (sha(live) !== row.baseSha256) drift.push(`DRIFT ${row.rel} live ${sha(live).slice(0, 12)} expected ${row.baseSha256!.slice(0, 12)}`);
    else todo.push({ row, data, live });
  }
  return { todo, skipped, drift };
}

let pending = false;

function applyGroup(group: (typeof manifest.groups)[number]): boolean {
  const { todo, skipped, drift } = plan(group);
  console.log(`group ${group.id} (${group.sets.join(", ")}): ${todo.length} to write (${todo.filter((t) => t.row.op === "create").length} new), ${skipped.length} already applied, ${drift.length} drift`);
  for (const line of drift) console.error(`  REFUSED ${line}`);
  if (drift.length) return false;
  if (!yes) { for (const t of todo.slice(0, 400)) console.log(`  would ${t.row.op}\t${t.row.rel}`); if (todo.length) pending = true; return true; }
  const busy = running();
  if (busy.length) { console.error(`REFUSED a dependency sync or show projects is running (any write aborts it): ${busy[0]}`); return false; }
  const backup = backupDirectory(group.id);
  mkdirSync(backup, { recursive: true });
  const index: { rel: string; backup: string | null; resultSha256: string }[] = [];
  todo.forEach(({ row, live }, position) => {
    const name = String(position).padStart(4, "0");
    if (live !== undefined) writeFileSync(join(backup, name), live);
    index.push({ rel: row.rel, backup: live === undefined ? null : name, resultSha256: row.resultSha256 });
  });
  writeFileSync(join(backup, "index.json"), JSON.stringify(index, null, 1));
  const staged: { temp: string; target: string }[] = [];
  try {
    for (const { row, data } of todo) {
      const target = join(root, row.rel), temp = target + ".integrate.tmp";
      mkdirSync(dirname(target), { recursive: true });
      writeFileSync(temp, data);
      staged.push({ temp, target });
    }
    for (const { temp, target } of staged) renameSync(temp, target);
  } catch (error) {
    for (const { temp } of staged) rmSync(temp, { force: true });
    throw error;
  }
  for (const { row } of todo) if (sha(readFileSync(join(root, row.rel))) !== row.resultSha256) throw new Error(`written file differs from its result hash: ${row.rel}`);
  for (const command of group.commands) { const run = spawnSync(command[0]!, command.slice(1), { cwd: root, stdio: "inherit" }); if (run.status !== 0) throw new Error(`group command failed: ${command.join(" ")}`); }
  console.log(`group ${group.id}: applied ${todo.length} files; backup ${backup}`);
  return true;
}

function undoGroup(id: string): void {
  const backup = backupDirectory(id), index = JSON.parse(readFileSync(join(backup, "index.json"), "utf8")) as { rel: string; backup: string | null; resultSha256: string }[];
  const foreign = index.filter((row) => existsSync(join(root, row.rel)) && sha(readFileSync(join(root, row.rel))) !== row.resultSha256);
  if (foreign.length) throw new Error(`refused: ${foreign.length} file(s) changed after the group was applied, e.g. ${foreign[0]!.rel}`);
  for (const row of index) {
    const target = join(root, row.rel);
    if (row.backup === null) rmSync(target, { force: true });
    else { copyFileSync(join(backup, row.backup), target + ".integrate.tmp"); renameSync(target + ".integrate.tmp", target); }
  }
  console.log(`group ${id}: restored ${index.length} files`);
}

if (undo) undoGroup(undo);
else {
  let ok = true;
  for (const group of manifest.groups) {
    if (only && only !== "all" && group.id !== only) continue;
    if (!applyGroup(group)) { ok = false; break; }
    if (pending && !yes) { console.log("later groups are checked once this group is applied (their preimages are its results)"); break; }
  }
  process.exit(ok ? 0 : 1);
}
