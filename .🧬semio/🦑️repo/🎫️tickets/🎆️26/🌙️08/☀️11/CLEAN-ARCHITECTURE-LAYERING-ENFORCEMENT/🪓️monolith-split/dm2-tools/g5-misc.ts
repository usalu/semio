// 🧩️ History test retirement helper crate, pack schema index crate, pack intrinsic-media law template crate, ui-text reference module under `testkit`.
import {log, read, put, replaceOnce, replaceRegexOnce, addToSection, packageNameOf, propagateEdges, createCrate, membersInsert, depLine, abs} from "./lib.ts";
import {relative} from "node:path";
const FW = "🧰️framework";
const OSM = `${FW}/🛍️products/💻️os/🔨️modules`;
const rel = (from: string, to: string) => relative(abs(from), abs(to)).replaceAll("\\", "/");
const created: {name: string; version: string; deps: string[]}[] = [];
const edges: {consumer: string; dep: string; dev: boolean}[] = [];
const VALUE = `${FW}/🔨️modules/🌱️value/📦️packages/🦀️rust`;
const TRACE_MEM = `${FW}/🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/📦️packages/🦀️rust`;
const DSL_RECORD = `${FW}/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust`;

const HIST = `${OSM}/📡️spr/📜️history`;
const HIST_DIR = `${HIST}/🧪️testing/♻️retirement/📦️packages/🦀️rust`;
const histName = "semio-framework-os-history-testing-retirement";
createCrate({dir: HIST_DIR, name: histName, description: "Exact original History test recipient retirement shared by the history and record-decode suites.", role: "test", libPath: "../../🦀️.rs", workspaceRel: rel(HIST_DIR, "."), id: "os-history-testing-retirement", dependencies: [depLine("semio-framework-value", HIST_DIR, VALUE), depLine("semio-framework-trace-memory-testing", HIST_DIR, TRACE_MEM)], framework: true, ownerName: "@semio-tech/os-history-testing-retirement-rs"});
membersInsert("Cargo.toml", HIST_DIR);
created.push({name: histName, version: "0.1.0", deps: ["semio-framework-trace-memory-testing", "semio-framework-value"]});
replaceOnce(`${HIST}/🧪️testing/♻️retirement/🦀️.rs`, "pub(crate) fn close_original", "pub fn close_original");
replaceOnce(`${HIST}/🦀️.rs`, `#[path="🧪️testing/♻️retirement/🦀️.rs"]\nmod test_retirement;`, `use semio_framework_os_history_testing_retirement as test_retirement;`);
replaceOnce(`${HIST}/🛬️decode/📋️record-set/🦀️.rs`, `#[path="../../🧪️testing/♻️retirement/🦀️.rs"]\nmod test_retirement;`, `use semio_framework_os_history_testing_retirement as test_retirement;`);
for (const dir of [`${HIST}/📦️packages/🦀️rust`, `${HIST}/🛬️decode/📋️record-set/📦️packages/🦀️rust`]) {
  addToSection(`${dir}/Cargo.toml`, "[dev-dependencies]", depLine(histName, dir, HIST_DIR));
  edges.push({consumer: packageNameOf(`${dir}/Cargo.toml`), dep: histName, dev: true});
}

const PACKV = `${FW}/🔨️modules/🎒️pack/🌱️value`;
const IDX = `${PACKV}/🏭️schema/🗂️index`;
const IDX_DIR = `${IDX}/📦️packages/🦀️rust`;
const idxName = "semio-framework-pack-schema-index";
createCrate({dir: IDX_DIR, name: idxName, description: "Concrete controlled slots for schema producer identities and semantic digest buckets.", role: "framework", libPath: "../../🦀️.rs", workspaceRel: rel(IDX_DIR, FW), id: "pack-schema-index", dependencies: [depLine("semio-framework-dsl-record", IDX_DIR, DSL_RECORD), depLine("semio-framework-value", IDX_DIR, VALUE)], framework: true, ownerName: "@semio-tech/pack-schema-index-rs"});
membersInsert(`${FW}/Cargo.toml`, IDX_DIR.slice(FW.length + 1));
created.push({name: idxName, version: "0.1.0", deps: ["semio-framework-dsl-record", "semio-framework-value"]});
{
  let text = read(`${IDX}/🦀️.rs`);
  text = text.replace("pub(super)trait Key", "pub trait Key").replace("pub(super)struct Index", "pub struct Index");
  const before = text.split("pub(super)fn ").length - 1;
  if (before !== 4) throw new Error(`index fns ${before}`);
  put(`${IDX}/🦀️.rs`, text.replaceAll("pub(super)fn ", "pub fn "));
}
replaceOnce(`${PACKV}/🏭️schema/🦀️.rs`, `#[path="🗂️index/🦀️.rs"]\nmod index;\nuse index::Index;`, `use semio_framework_pack_schema_index::Index;`);
replaceOnce(`${OSM}/🎒️pack/🌱️value/🏭️schema/🦀️.rs`, `#[path="../../../../../../🔨️modules/🎒️pack/🌱️value/🏭️schema/🗂️index/🦀️.rs"]\nmod index;\nuse index::Index;`, `use semio_framework_pack_schema_index::Index;`);
for (const dir of [`${PACKV}/🧬️schema/📦️packages/🦀️rust`, `${OSM}/🎒️pack/📦️packages/🦀️rust`]) {
  addToSection(`${dir}/Cargo.toml`, "[dependencies]", depLine(idxName, dir, IDX_DIR));
  edges.push({consumer: packageNameOf(`${dir}/Cargo.toml`), dep: idxName, dev: false});
}

const LAWS = `${PACKV}/🧪️tests/🎞️intrinsic-media`;
const LAWS_DIR = `${LAWS}/📦️packages/🦀️rust`;
const lawsName = "semio-framework-pack-intrinsic-media-laws";
createCrate({dir: LAWS_DIR, name: lawsName, description: "Neutral full-value wire laws as one template: the same law runs against the framework pack and the OS pack.", role: "test", libPath: "../../🦀️.rs", workspaceRel: rel(LAWS_DIR, FW), id: "pack-intrinsic-media-laws", dependencies: [], framework: true, ownerName: "@semio-tech/pack-intrinsic-media-laws-rs"});
membersInsert(`${FW}/Cargo.toml`, LAWS_DIR.slice(FW.length + 1));
created.push({name: lawsName, version: "0.1.0", deps: []});
{
  const text = read(`${LAWS}/🦀️.rs`);
  const split = text.indexOf("\n");
  const head = text.slice(0, split + 1);
  const body = text.slice(split + 1).replace(/include_str!\("\.\.\/\.\.\/🧪️tests(\/[^"]*)"\)/gu, 'include_str!(concat!($root,"$1"))');
  if ((body.match(/\$root/gu) ?? []).length !== 6 || body.includes("../../🧪️tests")) throw new Error("fixture path rewrite count");
  put(`${LAWS}/🦀️.rs`, `${head}\n#[macro_export]\nmacro_rules! intrinsic_media_wire_laws {\n    ($root:literal) => {\n${body.replace(/\n+$/u, "")}\n    };\n}\n`);
}
replaceOnce(`${FW}/🔨️modules/🎒️pack/🧪️tests/🔬️record-laws/🦀️.rs`, `#[path = "../../🌱️value/🧪️tests/🎞️intrinsic-media/🦀️.rs"]\nmod intrinsic_media_tests;`, `mod intrinsic_media_tests {\n    semio_framework_pack_intrinsic_media_laws::intrinsic_media_wire_laws!("../../🌱️value/🧪️tests");\n}`);
replaceOnce(`${OSM}/🎒️pack/🌱️value/🦀️.rs`, `#[path = "../../../../../🔨️modules/🎒️pack/🌱️value/🧪️tests/🎞️intrinsic-media/🦀️.rs"]\nmod intrinsic_media_tests;`, `mod intrinsic_media_tests {\n    semio_framework_pack_intrinsic_media_laws::intrinsic_media_wire_laws!("../../../../../🔨️modules/🎒️pack/🌱️value/🧪️tests");\n}`);
for (const dir of [`${FW}/🔨️modules/🎒️pack/📦️packages/🦀️rust`, `${OSM}/🎒️pack/📦️packages/🦀️rust`]) {
  addToSection(`${dir}/Cargo.toml`, "[dev-dependencies]", depLine(lawsName, dir, LAWS_DIR));
  edges.push({consumer: packageNameOf(`${dir}/Cargo.toml`), dep: lawsName, dev: true});
}

const UI = `${FW}/🔨️modules/🖱️ui`;
const TEXT_DIR = `${UI}/🔤️typography/📦️packages/🦀️rust`;
const REF = `${UI}/🧪️testing/🔤️text-reference/🦀️.rs`;
{
  let text = read(REF);
  text = text.replace("const KERNED_PANGRAM", "pub const KERNED_PANGRAM").replace("fn reference_break_indices", "pub fn reference_break_indices");
  text = text.replace("//! 🔤️ Shared original text wrapping reference for font and renderer laws.\n", "//! 🔤️ Shared original text wrapping reference for font and renderer laws.\nuse super::FontAtlas;\n\n");
  put(REF, text);
}
replaceOnce(`${UI}/🔤️typography/🧩️component/🦀️.rs`, `#[cfg(test)]\n#[path = "../../🧪️tests/🔬️targets-wgpu-text-unit/🦀️.rs"]\nmod tests;`, `#[cfg(any(test, feature = "testkit"))]\n#[path = "../../🧪️testing/🔤️text-reference/🦀️.rs"]\npub mod text_reference;\n\n#[cfg(test)]\n#[path = "../../🧪️tests/🔬️targets-wgpu-text-unit/🦀️.rs"]\nmod tests;`);
replaceOnce(`${UI}/🧪️tests/🔬️targets-wgpu-text-unit/🦀️.rs`, `include!("../../🧪️testing/🔤️text-reference/🦀️.rs");`, `use super::text_reference::{reference_break_indices,KERNED_PANGRAM};`);
replaceOnce(`${UI}/🧪️tests/🔬️targets-wgpu-text-renderer/🦀️.rs`, `include!("../../🧪️testing/🔤️text-reference/🦀️.rs");`, `use semio_framework_ui_text::text_reference::KERNED_PANGRAM;`);
replaceOnce(`${TEXT_DIR}/Cargo.toml`, "[dependencies]\n", "[features]\ntestkit = []\n\n[dependencies]\n");
addToSection(`${UI}/📦️packages/🦀️rust/Cargo.toml`, "[dev-dependencies]", `semio-framework-ui-text = { path = "../../🔤️typography/📦️packages/🦀️rust", features = ["testkit"] }`);

const result = propagateEdges(edges, created);
log(`locks changed: ${result.changed.length}; skipped: ${result.skipped.length}`);
