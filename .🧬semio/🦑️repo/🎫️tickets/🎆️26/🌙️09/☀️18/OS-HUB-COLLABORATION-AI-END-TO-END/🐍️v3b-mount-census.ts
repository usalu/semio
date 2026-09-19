/** 🕸️ V3b census of `plugin-registry check`'s `unreachable-from-cargo-manifest` findings.
 *
 * Classifies every finding by the evidence that decides its owner:
 * - `repo-test-adapter`: the leaf sits in `<owner>/🧪️tests/<case>/` beside a `🥒️.feature`, so its
 *   compilation owner is the generated cache-local host crate
 *   (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🏗️materialization/🟦️.ts:132`), never the
 *   plugin's Cargo manifest.
 * - `mounted-elsewhere`: some `#[path]` in the repo names this leaf (so it compiles under another
 *   crate root).
 * - `unmounted`: nothing mounts it — a real finding.
 *
 * Usage: `bun 🐍️v3b-mount-census.ts <repoRoot> <findings.txt>`
 */
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join, relative } from "node:path";

const repoRoot = process.argv[2]!;
const findingsPath = process.argv[3]!;

const lines = readFileSync(findingsPath, "utf8").split("\n").filter((line) => line.includes(" is not reachable from Cargo manifest "));

const pluginDirs = new Map<string, string>();
for (const name of readdirSync(join(repoRoot, "✏️s/🔌️plugins"))) {
  const abs = join(repoRoot, "✏️s/🔌️plugins", name);
  if (statSync(abs).isDirectory()) pluginDirs.set(name, abs);
}

const mountTargets = new Set<string>();
const includeTargets = new Set<string>();
function collectMounts(dir: string): void {
  for (const name of readdirSync(dir)) {
    if (name.startsWith(".") || name === "target" || name === "node_modules" || name === "dist") continue;
    const abs = join(dir, name);
    let st;
    try {
      st = statSync(abs);
    } catch {
      continue;
    }
    if (st.isDirectory()) {
      collectMounts(abs);
      continue;
    }
    if (!name.endsWith(".rs")) continue;
    const text = readFileSync(abs, "utf8");
    for (const match of text.matchAll(/#\[path\s*=\s*"([^"]+)"\]/g)) {
      const target = match[1]!;
      const resolved = target.startsWith("/") ? target : join(dirname(abs), target);
      mountTargets.add(relative(repoRoot, resolved).replaceAll("\\", "/"));
    }
    for (const match of text.matchAll(/(?<![A-Za-z0-9_])include!\s*\(\s*"([^"]+)"\s*\)/g)) {
      const target = match[1]!;
      const resolved = target.startsWith("/") ? target : join(dirname(abs), target);
      includeTargets.add(relative(repoRoot, resolved).replaceAll("\\", "/"));
    }
  }
}
for (const area of ["✏️s", "🧰️framework"]) collectMounts(join(repoRoot, area));

type Row = { plugin: string; rel: string; abs: string; klass: string };
const rows: Row[] = [];
for (const line of lines) {
  const body = line.replace(/^\s*-\s*/, "").replace(/ is not reachable from Cargo manifest .*$/, "");
  const idx = body.indexOf(": ");
  const plugin = body.slice(0, idx);
  const rel = body.slice(idx + 2);
  const pluginRoot = pluginDirs.get(plugin);
  if (!pluginRoot) {
    rows.push({ plugin, rel, abs: "", klass: "plugin-root-not-found" });
    continue;
  }
  const abs = join(pluginRoot, rel);
  const repoRel = relative(repoRoot, abs).replaceAll("\\", "/");
  const caseDir = dirname(abs);
  const isCase = existsSync(join(caseDir, "🥒️.feature")) && basenameOf(dirname(caseDir)) === "🧪️tests";
  let klass: string;
  if (!existsSync(abs)) klass = "missing-on-disk";
  else if (isCase) klass = "repo-test-adapter";
  else if (includeTargets.has(repoRel)) klass = "include-mounted";
  else if (mountTargets.has(repoRel)) klass = "mounted-elsewhere";
  else klass = "unmounted";
  rows.push({ plugin, rel, abs, klass });
}

function basenameOf(p: string): string {
  const parts = p.replaceAll("\\", "/").split("/");
  return parts[parts.length - 1] ?? "";
}

const byClass = new Map<string, number>();
for (const row of rows) byClass.set(row.klass, (byClass.get(row.klass) ?? 0) + 1);
console.log("total=" + rows.length);
for (const [klass, count] of [...byClass].sort((a, b) => b[1] - a[1])) console.log(`  ${klass}\t${count}`);

console.log("\n=== unmounted by plugin ===");
const unmountedByPlugin = new Map<string, number>();
for (const row of rows.filter((r) => r.klass === "unmounted")) unmountedByPlugin.set(row.plugin, (unmountedByPlugin.get(row.plugin) ?? 0) + 1);
for (const [plugin, count] of [...unmountedByPlugin].sort((a, b) => b[1] - a[1])) console.log(`  ${plugin}\t${count}`);

console.log("\n=== unmounted by lane shape ===");
const shapes = new Map<string, number>();
for (const row of rows.filter((r) => r.klass === "unmounted")) {
  const parts = row.rel.split("/");
  const shape = parts.slice(-3).join("/");
  shapes.set(shape, (shapes.get(shape) ?? 0) + 1);
}
for (const [shape, count] of [...shapes].sort((a, b) => b[1] - a[1]).slice(0, 40)) console.log(`  ${count}\t${shape}`);

console.log("\n=== repo-test-adapter by plugin ===");
const adapterByPlugin = new Map<string, number>();
for (const row of rows.filter((r) => r.klass === "repo-test-adapter")) adapterByPlugin.set(row.plugin, (adapterByPlugin.get(row.plugin) ?? 0) + 1);
for (const [plugin, count] of [...adapterByPlugin].sort((a, b) => b[1] - a[1])) console.log(`  ${plugin}\t${count}`);

console.log("\n=== full unmounted list ===");
for (const row of rows.filter((r) => r.klass === "unmounted")) console.log(`${row.plugin}\t${row.rel}`);
console.log("\n=== repo-test-adapter rows ===");
for (const row of rows.filter((r) => r.klass === "repo-test-adapter")) console.log(`${row.plugin}\t${row.rel}`);
console.log("\n=== include-mounted list ===");
for (const row of rows.filter((r) => r.klass === "include-mounted")) console.log(`${row.plugin}\t${row.rel}`);
console.log("\n=== mounted-elsewhere list ===");
for (const row of rows.filter((r) => r.klass === "mounted-elsewhere")) console.log(`${row.plugin}\t${row.rel}`);
console.log("\n=== missing-on-disk list ===");
for (const row of rows.filter((r) => r.klass === "missing-on-disk")) console.log(`${row.plugin}\t${row.rel}`);
