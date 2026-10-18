/** 🔁️ Level B: regenerates the carve of one artifact in a sandbox whose artifact tree is the live tree with the carve reverted from the slice's backup, and compares every write of the generator with the live file. Writes only below /Users/ueli/.cache/gtk/<tag>/ and to the report path. Usage: bun gtk-regen.ts <tag> <artifact-dir> <report.json> <backup-dir>[,<backup-dir>...] [--flags=a,b] [--package=<dir>] */
import { cpSync, existsSync, lstatSync, mkdirSync, readdirSync, readFileSync, realpathSync, rmSync, statSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { carveWrites, planArtifactCrates } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";

const repo = "/Users/ueli/Documents/semio";
const [tag, artifact, report, backupsArg, ...rest] = process.argv.slice(2) as [string, string, string, string, ...string[]];
const flags = new Set((rest.find((arg) => arg.startsWith("--flags="))?.slice(8) ?? "").split(",").filter(Boolean));
const packageDir = rest.find((arg) => arg.startsWith("--package="))?.slice(10) ?? `${artifact}/📦️packages/🦀️rust`;
const sandboxBase = resolve("/Users/ueli/.cache/gtk", tag);
const sandbox = join(sandboxBase, "root");

mkdirSync("/Users/ueli/.cache/gtk", { recursive: true });
const allowed = join(realpathSync("/Users/ueli/.cache/gtk"), tag);
function assertInside(path: string): string {
  const target = resolve(path);
  if (target !== sandboxBase && !target.startsWith(`${sandboxBase}${sep}`)) throw new Error(`sandbox write escapes: ${target}`);
  let probe = target;
  while (!existsSync(probe)) probe = dirname(probe);
  const physical = resolve(realpathSync(probe), relative(probe, target));
  if (physical !== allowed && !physical.startsWith(`${allowed}${sep}`)) throw new Error(`sandbox write resolves outside the sandbox: ${target} -> ${physical}`);
  return target;
}

rmSync(assertInside(sandboxBase), { recursive: true, force: true });
mkdirSync(assertInside(sandbox), { recursive: true });
const artifactParts = artifact.split("/");
const keepReal = new Set<string>();
for (let depth = 1; depth <= artifactParts.length; depth += 1) keepReal.add(artifactParts.slice(0, depth).join("/"));
for (const entry of readdirSync(repo)) {
  if (entry === ".git" || entry === "node_modules") continue;
  if (entry === artifactParts[0]) continue;
  symlinkSync(join(repo, entry), assertInside(join(sandbox, entry)));
}
for (let depth = 1; depth < artifactParts.length; depth += 1) {
  const directory = artifactParts.slice(0, depth).join("/");
  mkdirSync(assertInside(join(sandbox, directory)), { recursive: true });
  for (const entry of readdirSync(join(repo, directory))) {
    const child = `${directory}/${entry}`;
    if (keepReal.has(child)) continue;
    symlinkSync(join(repo, child), assertInside(join(sandbox, child)));
  }
}
cpSync(join(repo, artifact), assertInside(join(sandbox, artifact)), { recursive: true, filter: (source) => !/\/(node_modules|target)(\/|$)/u.test(source) });

const artifactCopy = join(sandbox, artifact);
const walk = (directory: string, visit: (path: string) => void): void => {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) { if (!/^(node_modules|target)$/u.test(entry.name)) walk(path, visit); } else visit(path);
  }
};

const crateFilter = rest.find((arg) => arg.startsWith("--prefix="))?.slice(9);
const removed: string[] = [];
walk(artifactCopy, (path) => {
  if (!path.endsWith("/Cargo.toml")) return;
  let toml: any;
  try { toml = Bun.TOML.parse(readFileSync(path, "utf8")); } catch { return; }
  if (!String(toml.package?.description ?? "").startsWith("Component crate of ")) return;
  if (crateFilter !== undefined && !String(toml.package?.name ?? "").startsWith(crateFilter)) return;
  const directory = dirname(path);
  for (const file of [path, join(directory, "🦀️.rs"), join(directory, "🪞️mirror/🦀️.rs")]) if (existsSync(file)) { rmSync(assertInside(file)); removed.push(relative(sandbox, file)); }
});

const restored: string[] = [];
const skipped: string[] = [];
for (const backup of backupsArg.split(",").filter(Boolean).reverse()) {
  let base = resolve(repo, backup);
  if (!existsSync(join(base, "✏️s"))) base = readdirSync(base, { withFileTypes: true }).filter((entry) => entry.isDirectory() && existsSync(join(base, entry.name, "✏️s"))).map((entry) => join(base, entry.name))[0] ?? base;
  walk(base, (path) => {
    const rel = relative(base, path);
    if (!rel.startsWith(`${artifact}/`)) { skipped.push(rel); return; }
    const destination = assertInside(join(sandbox, rel));
    mkdirSync(dirname(destination), { recursive: true });
    writeFileSync(destination, readFileSync(path));
    restored.push(rel);
  });
}

const patchFile = rest.find((arg) => arg.startsWith("--patch="))?.slice(8);
const patched: string[] = [];
if (patchFile) for (const edit of JSON.parse(readFileSync(resolve(repo, patchFile), "utf8")) as { file: string; find: string; replace: string }[]) {
  const target = assertInside(join(sandbox, edit.file));
  const text = readFileSync(target, "utf8");
  if (text.split(edit.find).length !== 2) throw new Error(`patch anchor is not unique in ${edit.file}: ${edit.find.slice(0, 60)}`);
  writeFileSync(target, text.replace(edit.find, () => edit.replace));
  patched.push(edit.file);
}

const options = {
  mutationFamilies: flags.has("families-on") ? "on" as const : flags.has("families-off") ? "off" as const : "auto" as const,
  inlineRootModules: flags.has("inline-root-modules"),
  lift: rest.find((arg) => arg.startsWith("--lift="))?.slice(7).split(",").filter(Boolean),
  tests: { fixtures: true, aggregate: !flags.has("no-test-aggregate"), hoist: true, detach: true },
};
const plan = await planArtifactCrates(sandbox, packageDir, options);
const out: any = { tag, artifact, patched: patched.length, removed: removed.length, restored: restored.length, skipped: skipped.length, problems: plan.problems, crates: plan.crates.length };
if (plan.problems.length) { writeFileSync(report, JSON.stringify(out, null, 1)); console.log(`plan problems: ${plan.problems.length}\n${plan.problems.slice(0, 5).join("\n")}`); process.exit(1); }
const { writes } = await carveWrites(plan);
for (const write of writes) if (write.action !== "delete") { const target = assertInside(join(sandboxBase, "gen", write.path)); mkdirSync(dirname(target), { recursive: true }); writeFileSync(target, write.content); }

const norm = (value: unknown): unknown => {
  if (Array.isArray(value)) return value.every((entry) => typeof entry === "string") ? [...value].sort() : value.map(norm);
  if (value && typeof value === "object") return Object.fromEntries(Object.keys(value).sort().map((key) => [key, norm((value as Record<string, unknown>)[key])]));
  return value;
};
const flat = (value: unknown, prefix = ""): Map<string, string> => {
  const map = new Map<string, string>();
  const visit = (node: unknown, path: string): void => {
    if (node && typeof node === "object" && !Array.isArray(node)) { for (const [key, child] of Object.entries(node)) visit(child, `${path}/${key}`); return; }
    map.set(path, JSON.stringify(node));
  };
  visit(value, prefix);
  return map;
};
const classify = (path: string): string => path.endsWith("Cargo.lock") ? "lock" : path.endsWith("Cargo.toml") ? (path === `${packageDir}/Cargo.toml` ? "facade-manifest" : /\/Cargo\.toml$/u.test(path) && path.split("/").length > artifact.split("/").length + 1 && writes.find((w) => w.path === path)?.action === "create" ? "crate-manifest" : "manifest") : path.endsWith("📜️script.ts") ? "router" : path.endsWith("🪞️mirror/🦀️.rs") ? "mirror" : path.endsWith("🦀️.rs") && path.includes("📦️packages/🦀️rust/🦀️.rs") ? "crate-root" : path.endsWith(".rs") ? "source" : "other";
const tally: Record<string, { equal: number; byteEqualOnly: number; differ: number; missingLive: number }> = {};
const differences: any[] = [];
for (const write of writes) {
  const kind = classify(write.path);
  const slot = (tally[kind] ??= { equal: 0, byteEqualOnly: 0, differ: 0, missingLive: 0 });
  const livePath = join(repo, write.path);
  if (write.action === "delete") { if (existsSync(livePath)) { slot.differ += 1; differences.push({ path: write.path, kind, note: "live still has a file the generator deletes" }); } else slot.equal += 1; continue; }
  if (!existsSync(livePath)) { slot.missingLive += 1; differences.push({ path: write.path, kind, note: "missing in live" }); continue; }
  const live = readFileSync(livePath, "utf8");
  if (live === write.content) { slot.equal += 1; continue; }
  if (write.path.endsWith("Cargo.toml")) {
    let a: Map<string, string>, b: Map<string, string>;
    try { a = flat(norm(Bun.TOML.parse(live))); b = flat(norm(Bun.TOML.parse(write.content))); } catch { slot.differ += 1; differences.push({ path: write.path, kind, note: "unparseable" }); continue; }
    const keys = [...new Set([...a.keys(), ...b.keys()])].filter((key) => a.get(key) !== b.get(key));
    if (!keys.length) { slot.byteEqualOnly += 1; continue; }
    slot.differ += 1;
    differences.push({ path: write.path, kind, keys: keys.slice(0, 30).map((key) => `${key}: live=${a.get(key) ?? "-"} gen=${b.get(key) ?? "-"}`.slice(0, 400)), count: keys.length });
    continue;
  }
  if (write.path.endsWith("Cargo.lock")) {
    const names = (text: string): Map<string, string> => new Map(text.split(/\n(?=\[\[package\]\]\n)/u).filter((block) => block.startsWith("[[package]]") && !/^source = /mu.test(block)).map((block) => [/name = "([^"]+)"/u.exec(block)![1]!, block]));
    const a = names(live), b = names(write.content);
    const keys = [...new Set([...a.keys(), ...b.keys()])].filter((key) => a.get(key) !== b.get(key));
    if (!keys.length) { slot.byteEqualOnly += 1; continue; }
    slot.differ += 1;
    differences.push({ path: write.path, kind, count: keys.length, keys: keys.slice(0, 40).map((key) => `${key}: ${a.has(key) ? "live" : ""}${b.has(key) ? " gen" : ""}`) });
    continue;
  }
  slot.differ += 1;
  const al = live.split("\n"), bl = write.content.split("\n");
  differences.push({ path: write.path, kind, note: `text differs (${al.length} live lines, ${bl.length} generated)` });
}
const owners = (JSON.parse(readFileSync(join(repo, "📋️build-owners.json"), "utf8")) as { owners: { name: string; root: string }[] }).owners;
const liveByRoot = new Map(owners.map((owner) => [owner.root, owner.name]));
const ownerDiffs: string[] = [];
for (const crate of plan.crates) { const name = liveByRoot.get(crate.packageDir); if (name !== crate.name) ownerDiffs.push(`${crate.packageDir}: live=${name ?? "-"} gen=${crate.name}`); }
const expected = new Set(plan.crates.map((crate) => crate.packageDir));
const prefix = `${plan.artifactCrate}-`;
const staleOwners = owners.filter((owner) => owner.name.startsWith(prefix) && owner.root.startsWith(`${artifact}/`) && !expected.has(owner.root)).map((owner) => `${owner.name}@${owner.root.slice(artifact.length)}`);
Object.assign(out, { tally, differences, ownerDiffs, staleOwners, skippedBackupFiles: skipped.slice(0, 10) });
writeFileSync(report, JSON.stringify(out, null, 1));
console.log(JSON.stringify({ tag, crates: plan.crates.length, removed: removed.length, restored: restored.length, tally, ownerDiffs: ownerDiffs.length, staleOwners: staleOwners.length }));
void lstatSync; void statSync;
