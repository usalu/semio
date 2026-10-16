// 🧊️ The 3d test allocation observer has one owner crate; the nine brep crates depend on it as a dev-dependency.
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {log, read, put, addToSection, packageNameOf, propagateEdges, createCrate, membersInsert, depLine, abs} from "./lib.ts";
import {relative} from "node:path";
const FW = "🧰️framework";
const FILE = `${FW}/🔨️modules/🧊️3d/🧪️tests/🧰️support/🦀️.rs`;
const DIR = `${FW}/🔨️modules/🧊️3d/🧪️tests/🧰️support/📦️packages/🦀️rust`;
const TRACE = `${FW}/🔨️modules/⏱️trace/📦️packages/🦀️rust`;
const rel = (from: string, to: string) => relative(abs(from), abs(to)).replaceAll("\\", "/");
const TSV = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/dm2-modowners-0.tsv";
const row = readFileSync(TSV, "utf8").split("\n").filter(Boolean).map(l => l.split("\t")).find(r => r[0] === FILE)!;
const consumers = row[2]!.split(" ; ");
const name = "semio-framework-3d-testing-support";
createCrate({dir: DIR, name, description: "Test-only allocation observer shared by every 3d crate's test binary: the one global allocator of that binary.", role: "test", libPath: "../../🦀️.rs", workspaceRel: rel(DIR, FW), id: "3d-testing-support", dependencies: [depLine("semio-framework-trace", DIR, TRACE)], framework: true, ownerName: "@semio-tech/3d-testing-support-rs"});
membersInsert(`${FW}/Cargo.toml`, DIR.slice(FW.length + 1));
{
  let text = read(FILE);
  const pairs: [string, string][] = [["pub(crate) struct TessellationAllocationObserver", "pub struct TessellationAllocationObserver"], ["pub(crate) fn observe_tessellation_system", "pub fn observe_tessellation_system"]];
  for (const stat of ["OBSERVED_SAMPLE", "OBSERVED_MESH", "SAMPLE_RELEASES", "MESH_RELEASES", "OBSERVED_RELEASE_BYTES"]) pairs.push([`pub(crate) static ${stat}`, `pub static ${stat}`]);
  for (const [from, to] of pairs) {
    if (text.split(from).length !== 2) throw new Error(`anchor ${from}`);
    text = text.replace(from, to);
  }
  text = text.replace("//! compiled into a library.", "//! compiled into a library.");
  put(FILE, text);
}
const mountRe = /#\[path\s*=\s*"[^"]*🧪️tests\/🧰️support\/🦀️\.rs"\]\npub\(crate\) mod test_support;/u;
const files = execFileSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "grep", "-l", "-F", "🧪️tests/🧰️support/🦀️.rs", "--", "🧰️framework/🔨️modules/🧊️3d/*.rs"], {cwd: "/Users/ueli/Documents/semio", encoding: "utf8"}).split("\n").filter(Boolean);
if (files.length !== consumers.length) throw new Error(`files ${files.length} != consumers ${consumers.length}`);
for (const file of files) {
  const text = read(file);
  if (!mountRe.test(text)) throw new Error(`no mount in ${file}`);
  put(file, text.replace(mountRe, "pub(crate) use semio_framework_3d_testing_support as test_support;"));
}
const edges = [];
for (const dir of consumers) {
  addToSection(`${dir}/Cargo.toml`, "[dev-dependencies]", depLine(name, dir, DIR));
  edges.push({consumer: packageNameOf(`${dir}/Cargo.toml`), dep: name, dev: true});
}
const result = propagateEdges(edges, [{name, version: "0.1.0", deps: ["semio-framework-trace"]}]);
log(`locks changed: ${result.changed.length}; skipped: ${result.skipped.length}`);
