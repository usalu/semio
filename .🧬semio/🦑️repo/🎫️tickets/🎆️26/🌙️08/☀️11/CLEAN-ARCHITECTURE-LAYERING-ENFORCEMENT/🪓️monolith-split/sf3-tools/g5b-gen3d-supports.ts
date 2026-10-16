// 🧰️ sf3 target 2, step 2: registers the four generation3d geometry test-support crates and re-points every consumer (mount -> use, dev-dependency, catalogue and registry arguments).
// usage: bun g5b-gen3d-supports.ts <scan.tsv> [--write]
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {dirname, join, normalize, relative} from "node:path";
import {log, read, put, addToSection, packageNameOf, propagateEdges, createCrate, membersInsert, workspaceDependencyTable, abs, ROOT} from "../dm2-tools/lib.ts";
import {DRY} from "../dm2-tools/dd.ts";

const rel = (from: string, to: string) => relative(abs(from), abs(to)).replaceAll("\\", "/");
const A = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d";
const G = `${A}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry`;
const rows = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean).map(l => l.split("\t"));
const PFX = "semio-s-artifact-procedural-3d-geometry-";

type Spec = {key: "O" | "C" | "P" | "H"; slug: string; dir: string; domain: string; suffix: string; description: string};
const specs: Spec[] = [
  {key: "O", slug: "oracle-support", dir: `${A}/🧪️tests/🔮️geometry-oracle-support`, domain: `${G}/🧪️tests/🧰️oracle-support/🦀️.rs`, suffix: "oracle_support", description: "Fixture-driven oracle of the geometry compute lanes: decodes language-agnostic cases into typed widget inputs, drives started jobs at several fuel grants and compares typed outputs; the catalogue is passed in by the caller"},
  {key: "C", slug: "construction-support", dir: `${A}/🧪️tests/🏗️geometry-construction-support`, domain: `${G}/🧪️tests/🧰️construction-support/🦀️.rs`, suffix: "construction_support", description: "Fixture-driven support of the construction computes: shape inputs from other widgets and analytic expectations; the catalogue and the registry lookup are passed in by the caller"},
  {key: "P", slug: "phased-job-support", dir: `${A}/🧪️tests/⏱️geometry-phased-job-support`, domain: `${G}/⏱️phased-job/🧪️tests/🧰️support/🦀️.rs`, suffix: "phased_job_support", description: "Fixture-driven support of the phased B-Rep categories: decodes cases into widget inputs, drives started jobs at several fuel grants and checks shapes against analytic expectations; the catalogue is passed in by the caller"},
  {key: "H", slug: "mesh-harness", dir: `${A}/🧪️tests/🧱️geometry-mesh-harness`, domain: `${G}/🧱️mesh-support/🧪️tests/🧰️mesh-harness/🦀️.rs`, suffix: "mesh_harness", description: "Fixture harness of the mesh unit tests: meshes and selections from language-agnostic cases, every compute at fuel one and unbounded, bit-identical repeated runs; the catalogue is passed in by the caller"},
];
const by = (k: string) => specs.find(s => s.key === k)!;
const pkgName = (s: Spec) => PFX + s.slug;
const ident = (s: Spec) => PFX.replaceAll("-", "_") + s.suffix;
const crateDirOf = (s: Spec) => `${s.dir}/📦️packages/🦀️rust`;
const wsText = read(`${A}/Cargo.toml`);
const inWorkspace = (n: string) => new RegExp(String.raw`^\s*\[workspace\.dependencies\.${n.replace(/[-_]/g, "[-_]")}\]`, "m").test(wsText);
const problems: string[] = [];
const created: {name: string; version: string; deps: string[]}[] = [];
const edges: {consumer: string; dep: string; dev: boolean}[] = [];

const externalNames = (file: string): string[] => {
  const text = read(file).replace(/\/\/[^\n]*/g, "");
  const found = [...text.matchAll(/(?<![\w:])((?:semio_[a-z0-9_]+)|serde_json)\s*::/g), ...text.matchAll(/\{\s*(semio_[a-z0-9_]+)::/g)].map(m => m[1]!.replaceAll("_", "-"));
  return [...new Set(found)].sort();
};
let manifests: string[] | undefined;
const geometryManifests = (): string[] => manifests ??= (execFileSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "-z", "--", `${G}/*/📦️packages/🦀️rust/Cargo.toml`, `${G}/*/*/📦️packages/🦀️rust/Cargo.toml`], {cwd: ROOT}).toString("utf8").split("\0").filter(Boolean));
const pathLine = (dep: string, targetDir: string): string => {
  for (const m of geometryManifests()) {
    const hit = new RegExp(String.raw`^${dep.replace(/-/g, "[-_]")}\s*=\s*\{\s*path\s*=\s*"([^"]+)"`, "m").exec(read(m));
    if (hit) return `${dep} = { path = "${relative(abs(targetDir), normalize(join(dirname(abs(m)), hit[1]!))).replaceAll("\\", "/")}" }`;
  }
  throw new Error(`no path dependency line for ${dep}`);
};
const own = new Set(specs.map(pkgName));
const depsOf = (s: Spec): {lines: string[]; names: string[]} => {
  const lines: string[] = [], names: string[] = [];
  for (const n of externalNames(s.domain).filter(x => !own.has(x))) {
    const key = n === "serde-json" ? "serde_json" : n;
    lines.push(inWorkspace(n) ? `${key} = { workspace = true }` : pathLine(n, crateDirOf(s)));
    names.push(n === "serde-json" ? "serde_json" : n);
  }
  if (s.key !== "O") { lines.push(`${pkgName(by("O"))} = { workspace = true }`); names.push(pkgName(by("O"))); }
  return {lines: lines.sort(), names: names.sort()};
};

for (const s of specs) {
  const dir = crateDirOf(s);
  if (Buffer.byteLength(`${dir}/Cargo.toml`) > 240) problems.push(`path too long: ${dir}`);
  const {lines, names} = depsOf(s);
  log(`== ${pkgName(s)} (${names.length} dependencies)`);
  createCrate({dir, name: pkgName(s), description: s.description, role: "test", libPath: "🦀️.rs", workspaceRel: rel(dir, A), dependencies: lines, framework: false});
  put(`${dir}/🦀️.rs`, `//! 📦️ Package glue — wiring only. Domain lives at owner 🦀️.rs.\n\n#[path = "${rel(dir, s.domain)}"]\nmod component;\npub use component::*;\n`, true);
  membersInsert(`${A}/Cargo.toml`, rel(A, dir));
  workspaceDependencyTable(`${A}/Cargo.toml`, pkgName(s), rel(A, dir));
  created.push({name: pkgName(s), version: "0.1.0", deps: names});
}

const CAT = "use semio_s_artifact_procedural_3d_io_text::snapshot::catalogue::catalogue;\n";
const REG = "use semio_s_artifact_procedural_3d_geometry::registry;\n";
const withLookup = new Set(["run_fixture", "assert_deterministic", "assert_cancellable", "first_shape", "inputs_of", "shape_of"]);
const catalogueOnly: Record<string, string[]> = {
  O: ["kind_of", "run_fixture", "assert_category"],
  P: ["kind_of", "run_fixture", "assert_category", "refresh"],
  H: ["run_fixture", "assert_category", "kind_of"],
  C: ["kind_of", "assert_category", "run_fixture", "assert_deterministic", "assert_cancellable", "first_shape", "inputs_of", "shape_of"],
};
const files = new Map<string, string>();
const consumerPackages = new Map<string, Set<string>>();
const needs = new Map<string, Set<string>>();
const need = (pkg: string, what: string) => needs.set(pkg, (needs.get(pkg) ?? new Set<string>()).add(what));
const domainFiles = new Set(specs.map(s => abs(s.domain)));
for (const s of specs) {
  const row = rows.find(r => r[0] === s.domain);
  if (!row) { log(`${s.key}: no scan row`); continue; }
  for (const entry of row[3]!.split(" ; ")) {
    const [pkg, ...rest] = entry.split("=");
    for (const site of rest.join("=").split(",")) {
      const file = site.replace(/:\d+.*$/u, "");
      if (!file.endsWith(".rs") || domainFiles.has(abs(file))) continue;
      const mountRx = new RegExp(String.raw`((?:#\[cfg\(test\)\]\s*)?)#\[path\s*=\s*"([^"]+)"\]\s*((?:pub(?:\([^)]*\))?\s+)?)mod\s+(\w+);`, "gu");
      let text = files.get(file) ?? read(file);
      let hits = 0;
      let alias = "";
      text = text.replace(mountRx, (all, cfg: string, path: string, vis: string, name: string) => {
        if (normalize(join(dirname(abs(file)), path)) !== abs(s.domain)) return all;
        hits++;
        alias = name;
        return `${cfg}${vis}use ${ident(s)} as ${name};`;
      });
      if (hits !== 1) { problems.push(`${s.key}: ${hits} mounts of the support in ${file}`); continue; }
      const stages: string[] = [];
      for (const fn of catalogueOnly[s.key]!) {
        const rx = new RegExp(String.raw`(?<![\w])${alias}::(oracle::)?${fn}\(`, "gu");
        const lookup = s.key === "C" && withLookup.has(fn);
        let n = 0;
        text = text.replace(rx, (all) => { n++; return `${all}catalogue(), ${lookup ? "registry::lookup, " : ""}`; });
        if (n) stages.push(`${fn}x${n}`);
      }
      if (stages.some(x => x.includes("registry"))) void 0;
      const needReg = s.key === "C" && /registry::lookup/u.test(text) && !/use semio_s_artifact_procedural_3d_geometry::registry;/u.test(text);
      const needCat = /catalogue\(\)/u.test(text) && !text.includes(CAT.trim());
      if (needCat || needReg) {
        const first = text.indexOf(`use ${ident(s)} as ${alias};`);
        const lineStart = text.lastIndexOf("\n", first) + 1;
        text = text.slice(0, lineStart) + (needCat ? CAT : "") + (needReg ? REG : "") + text.slice(lineStart);
      }
      if (needCat && /\bcatalogue\b/u.test(read(file)) && !read(file).includes(CAT.trim())) problems.push(`${s.key}: ${file} already names \`catalogue\``);
      if (/catalogue\(\)/u.test(text)) need(pkg!, "semio-s-artifact-procedural-3d-io-text");
      if (/registry::lookup/u.test(text)) need(pkg!, "semio-s-artifact-procedural-3d-geometry");
      files.set(file, text);
      log(`   ${s.key} consumer ${file.split("📐️geometry/")[1] ?? file.slice(-70)}: alias ${alias}, ${stages.join(" ")}`);
      const pk = consumerPackages.get(s.key) ?? new Set<string>();
      pk.add(pkg!);
      consumerPackages.set(s.key, pk);
    }
  }
}
const IO = `${A}/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📐️geometry`;
const MS = `${G}/🧱️mesh-support`;
const extras: {key: "H"; file: string; pkg: string; alias: string; rewriteLine?: [string, string]}[] = [
  {key: "H", file: `${IO}/📼️mesh-interchange/🧪️tests/🔬️unit/🦀️.rs`, pkg: `${IO}/📦️packages/🦀️rust`, alias: "h", rewriteLine: ["use semio_s_artifact_procedural_3d_mesh_support::harness as h;", `use ${ident(by("H"))} as h;`]},
  {key: "H", file: `${MS}/🧪️tests/🔬️unit/🦀️.rs`, pkg: `${MS}/📦️packages/🦀️rust`, alias: "oracle"},
];
for (const x of extras) {
  let text = files.get(x.file) ?? read(x.file);
  if (x.rewriteLine) { if (text.split(x.rewriteLine[0]).length !== 2) { problems.push(`extra ${x.file}: anchor`); continue; } text = text.replace(x.rewriteLine[0], x.rewriteLine[1]); }
  const stages: string[] = [];
  for (const fn of catalogueOnly[x.key]!) {
    const rx = new RegExp(String.raw`(?<![\w])${x.alias}::(oracle::)?${fn}\(`, "gu");
    let n = 0;
    text = text.replace(rx, (all) => { n++; return `${all}catalogue(), `; });
    if (n) stages.push(`${fn}x${n}`);
  }
  if (!/catalogue\(\)/u.test(text)) { problems.push(`extra ${x.file}: no call rewritten`); continue; }
  if (/\bcatalogue\b/u.test(read(x.file))) problems.push(`extra ${x.file} already names catalogue`);
  const at = text.search(/^use /mu);
  text = text.slice(0, at) + CAT + text.slice(at);
  need(x.pkg, "semio-s-artifact-procedural-3d-io-text");
  files.set(x.file, text);
  const pk = consumerPackages.get(x.key) ?? new Set<string>();
  pk.add(x.pkg);
  consumerPackages.set(x.key, pk);
  log(`   extra ${x.key} consumer ${x.file.split("📐️geometry/")[1] ?? x.file.slice(-70)}: ${stages.join(" ")}`);
}
if (problems.length) { for (const p of problems) log("PROBLEM " + p); log("ABORTED before consumer writes"); process.exit(1); }
for (const [file, text] of files) put(file, text);
for (const [pkg, wanted] of needs) {
  const manifest = read(`${pkg}/Cargo.toml`);
  for (const dep of wanted) {
    if (new RegExp(String.raw`^${dep}\s*=`, "m").test(manifest)) { log(`  ${pkg.split("📐️geometry/")[1] ?? pkg.slice(-50)} already depends on ${dep}`); continue; }
    addToSection(`${pkg}/Cargo.toml`, "[dev-dependencies]", `${dep} = { workspace = true }`);
    edges.push({consumer: packageNameOf(`${pkg}/Cargo.toml`), dep, dev: true});
  }
}
for (const s of specs) {
  for (const pkg of consumerPackages.get(s.key) ?? []) {
    addToSection(`${pkg}/Cargo.toml`, "[dev-dependencies]", `${pkgName(s)} = { workspace = true }`);
    edges.push({consumer: packageNameOf(`${pkg}/Cargo.toml`), dep: pkgName(s), dev: true});
  }
}
const result = propagateEdges(edges, created);
log(`locks changed: ${result.changed.length}; skipped: ${result.skipped.length}`);
for (const l of result.changed) log(`  ${l}`);
for (const sk of result.skipped.slice(0, 40)) log(`  skipped ${sk}`);
log(DRY ? "dry run" : "written");
