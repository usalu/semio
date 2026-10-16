// 🛬️ sf3 target 3/4: the three framework test helpers that plugin and os tests mounted by `#[path]` become framework test crates; every consumer mounts nothing and dev-depends on the owner.
// usage: bun g1-framework-helpers.ts <scan.tsv> [--write] [--only=owners,typed,backing]
import {readFileSync, existsSync} from "node:fs";
import {dirname} from "node:path";
import {log, read, put, addToSection, packageNameOf, propagateEdges, createCrate, membersInsert, depLine, abs, allLocks, isMember, pathBlock} from "../dm2-tools/lib.ts";
import {blocksOf, nameOf, depsOf, lockEnsure, DRY} from "../dm2-tools/dd.ts";
import {relative} from "node:path";

const FW = "🧰️framework";
const rel = (from: string, to: string) => relative(abs(from), abs(to)).replaceAll("\\", "/");
const scan = process.argv[2]!;
const only = (process.argv.find(a => a.startsWith("--only="))?.slice(7) ?? "owners,typed,backing").split(",");
const rows = readFileSync(scan, "utf8").split("\n").filter(Boolean).map(l => l.split("\t"));
const KERNEL = `${FW}/🛍️products/💻️os/📦️packages/🦀️rust`;
const VALUE = `${FW}/🔨️modules/🌱️value/📦️packages/🦀️rust`;
const TRACE = `${FW}/🔨️modules/⏱️trace/📦️packages/🦀️rust`;

type Helper = {key: string; dir: string; file: string; name: string; id: string; description: string; ident: string; deps: {name: string; dir: string; extra?: string}[]; moduleAlias: RegExp};
const helpers: Helper[] = [
  {
    key: "owners", dir: `${FW}/🔨️modules/🧪️test/🛬️original-snapshot-owners`, file: "🦀️.rs", name: "semio-framework-test-original-snapshot-owners", id: "test-original-snapshot-owners",
    description: "Original caller authorities for artifact tests: the native snapshot owners and the SQLite recipient every snapshot codec receives from its caller.",
    ident: "semio_framework_test_original_snapshot_owners", deps: [{name: "semio-framework-os-kernel", dir: KERNEL}, {name: "semio-framework-value", dir: VALUE}], moduleAlias: /mod original_owners;/u,
  },
  {
    key: "typed", dir: `${FW}/🔨️modules/🧪️test/⚖️typed-input-law`, file: "🦀️.rs", name: "semio-framework-test-typed-input-law", id: "test-typed-input-law",
    description: "Typed-command input retirement law: the gate every typed command payload passes before its original owner is retired under quoted grants.",
    ident: "semio_framework_test_typed_input_law", deps: [{name: "semio-framework-os-kernel", dir: KERNEL}, {name: "semio-framework-trace", dir: TRACE}, {name: "semio-framework-value", dir: VALUE}], moduleAlias: /mod typed_input_law;/u,
  },
  {
    key: "backing", dir: `${FW}/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️testing/💰️backing`, file: "🦀️.rs", name: "semio-framework-io-sqlite-snapshot-testing-backing", id: "io-sqlite-snapshot-testing-backing",
    description: "Test-only SQLite snapshot backing observation: the request-observing allocator and the snapshot backing laws every sqlite snapshot suite shares.",
    ident: "semio_framework_io_sqlite_snapshot_testing_backing", deps: [{name: "semio-framework-os-kernel", dir: KERNEL}], moduleAlias: /mod (observed|observer|backing_observer|snapshot_test_backing);/u,
  },
];

const created: {name: string; version: string; deps: string[]}[] = [];
const edges: {consumer: string; dep: string; dev: boolean}[] = [];
const problems: string[] = [];

const reachFromLock = (blocks: string[], starts: string[]): Set<string> => {
  const graph = new Map<string, string[]>();
  for (const b of blocks) graph.set(nameOf(b), [...(graph.get(nameOf(b)) ?? []), ...depsOf(b).map(d => d.split(" ")[0]!)]);
  const seen = new Set<string>();
  const stack = [...starts];
  while (stack.length) { const n = stack.pop()!; if (seen.has(n)) continue; seen.add(n); for (const d of graph.get(n) ?? []) stack.push(d); }
  return seen;
};
const lockTexts = new Map<string, string>(allLocks().map(l => [l, read(l)] as [string, string]));

for (const helper of helpers.filter(h => only.includes(h.key))) {
  const sharedPath = `${helper.dir}/${helper.file}`;
  const row = rows.find(r => r[0] === sharedPath);
  if (!row) { log(`${helper.key}: no scan row (already single-owner?)`); continue; }
  const consumers = row[2]!.split(" ; ");
  const mountFiles = [...new Set(row[3]!.split(" ; ").flatMap(entry => entry.split("=").slice(1).join("=").split(",")).map(site => site.replace(/:\d+.*$/u, "")).filter(f => f.endsWith(".rs")))];
  const crateDir = `${helper.dir}/📦️packages/🦀️rust`;
  log(`== ${helper.key}: ${consumers.length} consumers, ${mountFiles.length} mount files -> ${helper.name}`);
  const depNames = helper.deps.map(d => d.name);
  const consumerNames = new Set(consumers.map(c => packageNameOf(`${c}/Cargo.toml`)));
  for (const [lock, text] of lockTexts) {
    const hits = [...consumerNames].filter(n => text.includes(`name = "${n}"\n`));
    if (!hits.length) continue;
    const blocks = blocksOf(text).blocks;
    const reach = reachFromLock(blocks, depNames);
    for (const name of hits) if (pathBlock(blocks, name) >= 0 && isMember(lock, name) && reach.has(name)) problems.push(`CYCLE ${helper.key}: ${name} reachable from ${depNames.join(",")} in ${lock}`);
  }
  for (const file of mountFiles) {
    if (!file.includes("🧪️") && !/cfg\(test\)/u.test(read(file))) problems.push(`NONTEST ${helper.key}: ${file}`);
  }
  if (problems.length) continue;
  createCrate({
    dir: crateDir, name: helper.name, description: helper.description, role: "test", libPath: "../../🦀️.rs", workspaceRel: rel(crateDir, FW), id: helper.id,
    dependencies: helper.deps.map(d => depLine(d.name, crateDir, d.dir, d.extra ?? "")), framework: true, ownerName: `@semio-tech/${helper.id}-rs`,
  });
  membersInsert(`${FW}/Cargo.toml`, crateDir.slice(FW.length + 1));
  created.push({name: helper.name, version: "0.1.0", deps: depNames});
  const frameworkLock = `${FW}/Cargo.lock`;
  if (existsSync(abs(frameworkLock))) lockEnsure(frameworkLock, helper.name, "0.1.0", depNames, []);
  for (const file of mountFiles) {
    let text = read(file);
    const re = new RegExp(String.raw`(#\[cfg\(test\)\]\s*)?#\[path\s*=\s*"[^"]*${helper.dir.split("/").slice(-1)[0]!.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}[^"]*"\]\s*((?:pub(?:\([^)]*\))?\s+)?)${helper.moduleAlias.source.replace("mod ", "mod\\s+")}`, "gu");
    const matches = [...text.matchAll(re)];
    if (matches.length < 1) { problems.push(`NOMOUNT ${helper.key}: ${file}`); continue; }
    text = text.replace(re, (_all, cfg, vis, alias) => {
      const aliasName = /mod\s+(\w+);/u.exec(_all)![1]!;
      return `${cfg ?? ""}${vis ?? ""}use ${helper.ident} as ${aliasName};`;
    });
    put(file, text);
  }
  for (const consumer of consumers) {
    addToSection(`${consumer}/Cargo.toml`, "[dev-dependencies]", depLine(helper.name, consumer, crateDir));
    edges.push({consumer: packageNameOf(`${consumer}/Cargo.toml`), dep: helper.name, dev: true});
  }
}
if (problems.length) { for (const p of problems) log(p); log("ABORTED (problems above); nothing further written"); process.exit(1); }
const result = propagateEdges(edges, created);
log(`locks changed: ${result.changed.length}; skipped: ${result.skipped.length}`);
for (const l of result.changed) log(`  ${l}`);
for (const s of result.skipped) log(`  skipped ${s}`);
log(DRY ? "dry run" : "written");
void dirname;
