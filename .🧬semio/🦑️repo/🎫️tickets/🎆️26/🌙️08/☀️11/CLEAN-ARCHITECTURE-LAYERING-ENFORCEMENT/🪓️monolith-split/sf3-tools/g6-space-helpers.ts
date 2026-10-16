// 🪐️ sf3 target 5 (space): the three sqlite test helpers of the space tests that store, collection, space, dag and workflow tests mounted by `#[path]` become os test crates (root workspace).
// usage: bun g6-space-helpers.ts <scan.tsv> [--write] [--only=preflight,oracle,caller]
import {readFileSync, existsSync} from "node:fs";
import {dirname} from "node:path";
import {log, read, put, addToSection, packageNameOf, propagateEdges, createCrate, membersInsert, depLine, abs, allLocks, isMember, pathBlock} from "../dm2-tools/lib.ts";
import {blocksOf, nameOf, depsOf, lockEnsure, DRY} from "../dm2-tools/dd.ts";
import {relative} from "node:path";

const FW = "🧰️framework";
const rel = (from: string, to: string) => relative(abs(from), abs(to)).replaceAll("\\", "/");
const scan = process.argv[2]!;
const only = (process.argv.find(a => a.startsWith("--only="))?.slice(7) ?? "preflight,oracle,caller").split(",");
const rows = readFileSync(scan, "utf8").split("\n").filter(Boolean).map(l => l.split("\t"));
const KERNEL = `${FW}/🛍️products/💻️os/📦️packages/🦀️rust`;
const VALUE = `${FW}/🔨️modules/🌱️value/📦️packages/🦀️rust`;
const TRACE = `${FW}/🔨️modules/⏱️trace/📦️packages/🦀️rust`;

type Helper = {key: string; dir: string; file: string; name: string; id: string; description: string; ident: string; deps: {name: string; dir: string; extra?: string; literal?: string}[]; moduleAlias: RegExp; fixes: [string, string][]};
const OS = `${FW}/🛍️products/💻️os`;
const SP = `${OS}/🔨️modules/🪐️space/🧪️tests/🪶️sqlite`;
const SNAP = `${FW}/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust`;
const TEXTIO = `${OS}/🔨️modules/🏪️store/🚪️io/📦️packages/🦀️rust`;
const OSIO = `${OS}/🔨️modules/🚪️io/📦️packages/🦀️rust`;
const helpers: Helper[] = [
  {
    key: "preflight", dir: `${SP}/📏️preflight`, file: "🦀️.rs", name: "semio-framework-os-space-testing-sqlite-preflight", id: "os-space-testing-sqlite-preflight",
    description: "Public SQLite preflight law of the space tests: borrows actual typed owner state and admits concrete traversal scratch.",
    ident: "semio_framework_os_space_testing_sqlite_preflight", deps: [{name: "semio-framework-io-sqlite-snapshot", dir: SNAP}, {name: "semio-framework-os-io", dir: OSIO}, {name: "semio-framework-os-store-text-io", dir: TEXTIO}, {name: "semio-framework-value", dir: VALUE}], moduleAlias: /mod public_preflight;/u,
    fixes: [["pub(crate) fn verify<", "pub fn verify<"]],
  },
  {
    key: "oracle", dir: `${SP}/🔬️oracle`, file: "🦀️.rs", name: "semio-framework-os-space-testing-sqlite-oracle", id: "os-space-testing-sqlite-oracle",
    description: "Independent typed SQLite fixture and Bun edit oracle for builtin persisted owners.",
    ident: "semio_framework_os_space_testing_sqlite_oracle", deps: [{name: "semio-framework-io-sqlite-snapshot", dir: SNAP}, {name: "serde_json", dir: "", literal: `serde_json = "1.0.140"`}], moduleAlias: /mod oracle;/u,
    fixes: [["use super::store;\n", ""], ["pub(super) fn renumber(", "pub fn renumber("], ["pub(super) fn database(", "pub fn database("], ["pub(super) fn edit(", "pub fn edit("]],
  },
  {
    key: "caller", dir: `${SP}/🫴️native-caller`, file: "🦀️.rs", name: "semio-framework-os-space-testing-sqlite-native-caller", id: "os-space-testing-sqlite-native-caller",
    description: "Actual original native caller for the Semio SQLite snapshot suites: independent body and close grants drive every erased conversion.",
    ident: "semio_framework_os_space_testing_sqlite_native_caller", deps: [{name: "semio-framework-os-kernel", dir: KERNEL}, {name: "semio-framework-value", dir: VALUE}, {name: "serde_json", dir: "", literal: `serde_json = "1.0.140"`}], moduleAlias: /mod native_caller;/u,
    fixes: [],
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
    dir: crateDir, name: helper.name, description: helper.description, role: "test", libPath: "../../🦀️.rs", workspaceRel: rel(crateDir, "."), id: helper.id,
    dependencies: helper.deps.map(d => d.literal ?? depLine(d.name, crateDir, d.dir, d.extra ?? "")), framework: true, ownerName: `@semio-tech/${helper.id}-rs`,
  });
  membersInsert("Cargo.toml", crateDir);
  created.push({name: helper.name, version: "0.1.0", deps: depNames});
  lockEnsure("Cargo.lock", helper.name, "0.1.0", depNames, []);
  {
    let text = read(sharedPath);
    for (const [from, to] of helper.fixes) { if (text.split(from).length !== 2) throw new Error(`fix anchor ${from} in ${sharedPath}`); text = text.replace(from, to); }
    if (helper.fixes.length) put(sharedPath, text);
  }
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
