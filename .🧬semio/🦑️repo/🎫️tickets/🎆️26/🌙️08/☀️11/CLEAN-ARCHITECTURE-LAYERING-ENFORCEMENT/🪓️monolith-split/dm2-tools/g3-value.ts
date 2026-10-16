// ♻️ Value test support: one allocator observer crate, one clone-source fixture crate, and the factory fixture exported by its owner under `testkit`.
import {readFileSync} from "node:fs";
import {log, read, put, replaceOnce, addToSection, packageNameOf, propagateEdges, createCrate, membersInsert, depLine, abs} from "./lib.ts";
import {relative} from "node:path";
const FW = "🧰️framework";
const V = `${FW}/🔨️modules/🌱️value`;
const ALLOC_FILE = `${V}/🧪️testing/💾️allocator/🦀️.rs`;
const ALLOC_DIR = `${V}/🧪️testing/💾️allocator/📦️packages/🦀️rust`;
const SRC_FILE = `${V}/🧬️retained-clone/🔗️source/🧪️testing/🦀️.rs`;
const SRC_DIR = `${V}/🧬️retained-clone/🔗️source/🧪️testing/📦️packages/🦀️rust`;
const rel = (from: string, to: string) => relative(abs(from), abs(to)).replaceAll("\\", "/");
const TSV = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/dm2-modowners-0.tsv";
const rows = readFileSync(TSV, "utf8").split("\n").filter(Boolean).map(l => l.split("\t"));
const consumersOf = (file: string): string[] => (rows.find(r => r[0] === file) ?? (() => { throw new Error(`no scan row ${file}`); })())[2]!.split(" ; ");
const created: {name: string; version: string; deps: string[]}[] = [];
const edges: {consumer: string; dep: string; dev: boolean}[] = [];

const allocName = "semio-framework-value-testing-allocator";
createCrate({dir: ALLOC_DIR, name: allocName, description: "Original physical allocation receipt observer shared by true native owner suites: the one global allocator of each value test binary.", role: "test", libPath: "../../🦀️.rs", workspaceRel: rel(ALLOC_DIR, FW), id: "value-testing-allocator", dependencies: [], framework: true, ownerName: "@semio-tech/value-testing-allocator-rs"});
membersInsert(`${FW}/Cargo.toml`, ALLOC_DIR.slice(FW.length + 1));
created.push({name: allocName, version: "0.1.0", deps: []});
{
  let text = read(ALLOC_FILE);
  for (const [from, to] of [["pub(crate) fn observe_retirement_allocations", "pub fn observe_retirement_allocations"], ["pub(crate) static OBSERVE_ALLOCATIONS", "pub static OBSERVE_ALLOCATIONS"], ["pub(crate) static LARGEST_ALLOCATION", "pub static LARGEST_ALLOCATION"]] as const) {
    if (text.split(from).length !== 2) throw new Error(`anchor ${from}`);
    text = text.replace(from, to);
  }
  put(ALLOC_FILE, text);
}
const mountRe = /#\[path\s*=\s*"[^"]*🧪️testing\/💾️allocator\/🦀️\.rs"\]\nmod allocation_testing;/u;
const mountDirs = consumersOf(ALLOC_FILE);
const mountFiles = execFile("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "grep", "-l", "-F", "🧪️testing/💾️allocator/🦀️.rs", "--", "*.rs"]).split("\n").filter(f => f && !f.startsWith(".🧬semio/"));
function execFile(cmd: string, args: string[]): string { return require("node:child_process").execFileSync(cmd, args, {cwd: "/Users/ueli/Documents/semio", encoding: "utf8", maxBuffer: 1 << 26}); }
if (mountFiles.length !== mountDirs.length) throw new Error(`mount files ${mountFiles.length} != consumers ${mountDirs.length}`);
for (const file of mountFiles) {
  const text = read(file);
  if (!mountRe.test(text)) throw new Error(`no mount in ${file}`);
  put(file, text.replace(mountRe, "use semio_framework_value_testing_allocator as allocation_testing;"));
}
for (const dir of mountDirs) {
  addToSection(`${dir}/Cargo.toml`, "[dev-dependencies]", depLine(allocName, dir, ALLOC_DIR));
  edges.push({consumer: packageNameOf(`${dir}/Cargo.toml`), dep: allocName, dev: true});
}

const srcName = "semio-framework-value-clone-source-testing";
const CLONE_SOURCE = `${V}/🧬️retained-clone/🔗️source/📦️packages/🦀️rust`;
const RETIREMENT = `${V}/♻️retirement/📦️packages/🦀️rust`;
createCrate({dir: SRC_DIR, name: srcName, description: "Root native allocator custody creates and drains actual canonical clone-source fixtures.", role: "test", libPath: "../../🦀️.rs", workspaceRel: rel(SRC_DIR, FW), id: "value-clone-source-testing", dependencies: [depLine("semio-framework-value-clone-source", SRC_DIR, CLONE_SOURCE), depLine("semio-framework-value-retirement", SRC_DIR, RETIREMENT), depLine(allocName, SRC_DIR, ALLOC_DIR)], framework: true, ownerName: "@semio-tech/value-clone-source-testing-rs"});
membersInsert(`${FW}/Cargo.toml`, SRC_DIR.slice(FW.length + 1));
created.push({name: srcName, version: "0.1.0", deps: [packageNameOf(`${CLONE_SOURCE}/Cargo.toml`), packageNameOf(`${RETIREMENT}/Cargo.toml`), allocName].sort()});
replaceOnce(SRC_FILE, "use crate::retirement::RetireOwned;\nuse super::{RetainedCloneSource,RetainedCloneGrant};", "use semio_framework_value_retirement::RetireOwned;\nuse semio_framework_value_clone_source::{RetainedCloneSource,RetainedCloneGrant};");
replaceOnce(SRC_FILE, "pub(crate) struct FixtureSource<", "pub struct FixtureSource<");
replaceOnce(SRC_FILE, "pub(crate) trait FixtureSourceBirth<", "pub trait FixtureSourceBirth<");
replaceOnce(SRC_FILE, "crate::observe_retirement_allocations(", "semio_framework_value_testing_allocator::observe_retirement_allocations(");
const siteEdits: {file: string; dir: string; find: string; replace: string}[] = [
  {file: `${V}/🧬️retained-clone/🦀️.rs`, dir: `${V}/📦️packages/🦀️rust`, find: `#[path="🔗️source/🧪️testing/🦀️.rs"]\nmod original_source_fixture;\n#[cfg(test)]\npub(crate) use original_source_fixture::{FixtureSource,FixtureSourceBirth};`, replace: `pub(crate) use semio_framework_value_clone_source_testing::{FixtureSource,FixtureSourceBirth};`},
  {file: `${V}/🧬️retained-clone/🗺️ordered-map/📦️packages/🦀️rust/🦀️.rs`, dir: `${V}/🧬️retained-clone/🗺️ordered-map/📦️packages/🦀️rust`, find: `#[path="../../../🔗️source/🧪️testing/🦀️.rs"]\nmod original_source_fixture;\n#[cfg(test)]\npub(crate) use original_source_fixture::FixtureSourceBirth;`, replace: `pub(crate) use semio_framework_value_clone_source_testing::FixtureSourceBirth;`},
  {file: `${V}/🧬️retained-clone/🧬️contract/🦀️.rs`, dir: `${V}/🧬️retained-clone/🧬️contract/📦️packages/🦀️rust`, find: `#[path="../🔗️source/🧪️testing/🦀️.rs"]\nmod original_source_fixture;\n#[cfg(test)]\npub(crate) use original_source_fixture::{FixtureSource,FixtureSourceBirth};`, replace: `pub(crate) use semio_framework_value_clone_source_testing::{FixtureSource,FixtureSourceBirth};`},
];
for (const edit of siteEdits) {
  replaceOnce(edit.file, edit.find, edit.replace);
  addToSection(`${edit.dir}/Cargo.toml`, "[dev-dependencies]", depLine(srcName, edit.dir, SRC_DIR));
  edges.push({consumer: packageNameOf(`${edit.dir}/Cargo.toml`), dep: srcName, dev: true});
}

const FACTORY = `${V}/♻️retirement/🏭️factory`;
const FACTORY_DIR = `${FACTORY}/📦️packages/🦀️rust`;
const FIX = `${FACTORY}/🧪️testing/🦀️.rs`;
{
  let text = read(FIX);
  for (const [from, to] of [["pub(crate) struct OwnedFactory {pub(crate) payload", "pub struct OwnedFactory {pub payload"], ["pub(crate) struct ScalarFactory {pub(crate) tag:u64,pub(crate) name", "pub struct ScalarFactory {pub tag:u64,pub name"], ["pub(crate) fn grant(", "pub fn grant("]] as const) {
    if (text.split(from).length !== 2) throw new Error(`anchor ${from}`);
    text = text.replace(from, to);
  }
  put(FIX, text);
}
replaceOnce(`${FACTORY}/🦀️.rs`, "pub(crate) use allocation_testing::observe_retirement_allocations;", "pub(crate) use allocation_testing::observe_retirement_allocations;\n\n#[cfg(any(test, feature=\"testkit\"))]\n#[path=\"🧪️testing/🦀️.rs\"]\npub mod original_factory_fixture;");
replaceOnce(`${FACTORY_DIR}/Cargo.toml`, "[dependencies]\n", "[features]\ntestkit = []\n\n[dependencies]\n");
replaceOnce(`${FACTORY}/🧪️tests/🦀️.rs`, `#[path="../🧪️testing/🦀️.rs"]\nmod original_factory_fixture;\nuse original_factory_fixture::*;`, `use crate::original_factory_fixture::*;`);
replaceOnce(`${V}/🧬️retained-clone/🔗️source/🧪️tests/🏭️factory/🦀️.rs`, `#[path="../../../../♻️retirement/🏭️factory/🧪️testing/🦀️.rs"]\nmod original_factory_fixture;\nuse original_factory_fixture::*;`, `use semio_framework_value_factory::original_factory_fixture::*;`);
addToSection(`${V}/📦️packages/🦀️rust/Cargo.toml`, "[dev-dependencies]", `semio-framework-value-factory = { path = "../../♻️retirement/🏭️factory/📦️packages/🦀️rust", features = ["testkit"] }`);
const result = propagateEdges(edges, created);
log(`locks changed: ${result.changed.length}; skipped: ${result.skipped.length}`);
for (const l of result.skipped.slice(0, 6)) log(`  skip ${l}`);
