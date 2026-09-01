// 🔤️ One-shot, reviewable slug-shortening tool proven on 🌍️gis (ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION).
// See 📓️goal-slug-rule.md for the rule this table encodes. Not a permanent product script — ticket-scoped tool.
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync, renameSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO_ROOT = "/Users/ueli/Documents/semio";
const PLUGIN = `${REPO_ROOT}/✏️s/🔌️plugins/🌍️gis`;

type Entry = { artifact: string; mutationDir: string; oldName: string; newName: string };

const ENTRIES: Entry[] = [
  { artifact: "🗺️gismap", mutationDir: "🆕create-position", oldName: "adds-a-lighthouse-position-after-the-harbor", newName: "adds-lighthouse-position-after-harbor" },
  { artifact: "🗺️gismap", mutationDir: "🛣️create-route", oldName: "adds-a-tram-route-after-the-ferry", newName: "adds-tram-route-after-ferry" },
  { artifact: "🗺️gismap", mutationDir: "🌐create-region", oldName: "adds-the-old-town-region-after-the-harbor-district", newName: "adds-old-town-region-after-harbor-district" },
  { artifact: "🗺️gismap", mutationDir: "🧭reorder-routes", oldName: "moves-the-bus-route-to-the-front", newName: "moves-bus-route-to-front" },
  { artifact: "🗺️gismap", mutationDir: "🔀reorder-positions", oldName: "moves-the-harbor-position-to-the-end", newName: "moves-harbor-position-to-end" },
  { artifact: "🗺️gismap", mutationDir: "🔃reorder-regions", oldName: "moves-the-park-region-between-the-two-districts", newName: "moves-park-region-between-2-districts" },
  { artifact: "🗺️gismap", mutationDir: "🗑delete-position", oldName: "removes-the-lighthouse-position", newName: "removes-lighthouse-position" },
  { artifact: "🗺️gismap", mutationDir: "🧹delete-region", oldName: "removes-the-old-town-region", newName: "removes-old-town-region" },
  { artifact: "🗺️gismap", mutationDir: "✂️delete-route", oldName: "removes-the-tram-route", newName: "removes-tram-route" },
  { artifact: "🗺️gismap", mutationDir: "♻️replace-route-data", oldName: "rewrites-the-ferry-route-payload", newName: "rewrites-ferry-route-payload" },
  { artifact: "🗺️gismap", mutationDir: "🔄replace-region-data", oldName: "rewrites-the-harbor-district-region-payload", newName: "rewrites-harbor-district-region-payload" },
  { artifact: "🗺️gismap", mutationDir: "🔁replace-position-data", oldName: "rewrites-the-harbor-position-payload", newName: "rewrites-harbor-position-payload" },
  { artifact: "🏔️gisterrain", mutationDir: "🎚change-exaggeration", oldName: "raises-the-exaggeration-from-one-to-two-and-a-half", newName: "raises-exaggeration-from-1-to-2-5" },
  { artifact: "🏔️gisterrain", mutationDir: "📥change-imported-features", oldName: "imports-a-single-harbor-position-descriptor", newName: "imports-harbor-position-descriptor" },
];

const MUTATION_ID_RE = /^[a-z0-9]+(?:-[a-z0-9]+)*$/;
for (const e of ENTRIES) if (!MUTATION_ID_RE.test(e.newName)) throw new Error(`New slug fails MUTATION_ID_RE: ${e.newName}`);

function mutationsDir(artifact: string): string {
  return `${PLUGIN}/🗿️artifacts/${artifact}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`;
}

function walk(dir: string, out: string[] = []): string[] {
  for (const name of readdirSync(dir)) {
    const full = join(dir, name);
    const st = statSync(full);
    if (st.isDirectory()) walk(full, out);
    else out.push(full);
  }
  return out;
}

function gitGrepFiles(needle: string): string[] {
  const out = execFileSync("git", ["grep", "-F", "-l", "--", needle], { cwd: PLUGIN, encoding: "utf8", maxBuffer: 1 << 26 });
  return out.split("\n").filter(Boolean).map((rel) => join(PLUGIN, rel));
}

const touchedRustFiles = new Set<string>();

// Pass 1: collect referencing files BEFORE any rename (git grep needs the old dir to still exist for git-tracked content, but the string search itself is text-based and independent of the fs move).
const referencesByEntry = new Map<Entry, string[]>();
for (const e of ENTRIES) {
  const oldDir = `${mutationsDir(e.artifact)}/${e.mutationDir}/🧪️tests/${e.oldName}`;
  if (!existsSync(oldDir)) throw new Error(`Expected directory missing: ${oldDir}`);
  const refs = gitGrepFiles(e.oldName);
  if (refs.length === 0) throw new Error(`No references found for ${e.oldName} (expected at least glue.rs + oracle json + self)`);
  referencesByEntry.set(e, refs);
}

// Pass 2: rewrite text references (all files, including the fixture's own component.rs which is about to move).
for (const e of ENTRIES) {
  const refs = referencesByEntry.get(e)!;
  for (const file of refs) {
    const text = readFileSync(file, "utf8");
    if (!text.includes(e.oldName)) continue;
    const rewritten = text.split(e.oldName).join(e.newName);
    writeFileSync(file, rewritten, "utf8");
    if (file.endsWith(".rs")) touchedRustFiles.add(file);
  }
}

// Pass 3: physically rename the directories with plain fs.renameSync (no git mv).
for (const e of ENTRIES) {
  const oldDir = `${mutationsDir(e.artifact)}/${e.mutationDir}/🧪️tests/${e.oldName}`;
  const newDir = `${mutationsDir(e.artifact)}/${e.mutationDir}/🧪️tests/${e.newName}`;
  if (existsSync(newDir)) throw new Error(`Destination already exists: ${newDir}`);
  renameSync(oldDir, newDir);
}

// Pass 4: re-derive the moved .rs files' new paths for rustfmt (the set collected in pass 2 still points at the OLD path for files inside the renamed dir).
const rustfmtTargets = new Set<string>();
for (const file of touchedRustFiles) {
  let resolved = file;
  for (const e of ENTRIES) {
    const oldDir = `${mutationsDir(e.artifact)}/${e.mutationDir}/🧪️tests/${e.oldName}/`;
    if (resolved.startsWith(oldDir)) resolved = resolved.replace(oldDir, `${mutationsDir(e.artifact)}/${e.mutationDir}/🧪️tests/${e.newName}/`);
  }
  rustfmtTargets.add(resolved);
}

// Pass 5: realign the two mutate-*-1 Gherkin Examples tables (fixture column) so padding matches the new, shorter names.
function realignFixtureColumn(featurePath: string): void {
  const lines = readFileSync(featurePath, "utf8").split("\n");
  let i = 0;
  while (i < lines.length) {
    if (/^\s*\|\s*id\s*\|\s*dir\s*\|\s*fixture\s*\|\s*$/.test(lines[i])) {
      const headerIdx = i;
      let end = i + 1;
      while (end < lines.length && /^\s*\|/.test(lines[end])) end++;
      const block = lines.slice(headerIdx, end);
      const rows = block.map((line) => line.split("|").slice(1, -1).map((cell) => cell.trim()));
      const widths = [0, 1, 2].map((col) => Math.max(...rows.map((r) => r[col].length)));
      const indent = block[0].match(/^\s*/)![0];
      const rebuilt = rows.map((r) => `${indent}| ${r.map((cell, col) => cell.padEnd(widths[col])).join(" | ")} |`);
      lines.splice(headerIdx, block.length, ...rebuilt);
      i = headerIdx + rebuilt.length;
      continue;
    }
    i++;
  }
  writeFileSync(featurePath, lines.join("\n"), "utf8");
}
realignFixtureColumn(`${PLUGIN}/🗿️artifacts/🗺️gismap/🧪️tests/mutate-gismap-1/🥒️.feature`);
realignFixtureColumn(`${PLUGIN}/🗿️artifacts/🏔️gisterrain/🧪️tests/mutate-gisterrain-1/🥒️.feature`);

// Pass 6: rustfmt --check every touched .rs file (post-rename paths).
console.log(`rustfmt --check on ${rustfmtTargets.size} files:`);
for (const f of rustfmtTargets) {
  try {
    execFileSync("rustfmt", ["--check", "--edition", "2021", f], { encoding: "utf8" });
    console.log(`  OK    ${f}`);
  } catch (err: any) {
    console.log(`  FAIL  ${f}`);
    console.log(err.stdout?.toString() ?? err.message);
  }
}

// Pass 7: byte-length budget report, walked from the actual filesystem (git index is stale after a plain rename).
function byteLen(s: string): number {
  return Buffer.byteLength(s, "utf8");
}
const allFiles = walk(PLUGIN).map((abs) => abs.slice(REPO_ROOT.length + 1));
const overBudget = allFiles.filter((p) => byteLen(p) > 240);
const longest = allFiles.reduce((a, b) => (byteLen(a) >= byteLen(b) ? a : b));
console.log(`\ngis tracked-tree files: ${allFiles.length}`);
console.log(`over-budget (>240 bytes) AFTER: ${overBudget.length}`);
console.log(`longest path AFTER (${byteLen(longest)} bytes): ${longest}`);
for (const p of overBudget) console.log(`  ${byteLen(p)}  ${p}`);
