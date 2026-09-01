// Fail-before / pass-after verification for the two goal-chain mechanisms:
// (1) non-repo-ness surviving `.parent()/.to_path_buf()/.clone()/.as_path()/.to_owned()` chains,
//     plus tuple-destructure + qualified-hop + literal-narrowed-match-arm-elimination resolution
//     of `mutation_source_authority_tests::materialize("valid", ..)`'s tuple, clearing derive.rs
//     rows 344, 351 (x2), 1900.
// (2) a same-file `fn NAME() -> PathBuf` matching the CARGO_MANIFEST_DIR-seeded ancestor-walk-to-
//     `nx.json` idiom (`test_repo_root`/`find_repo_root`'s shared strategy) proven as a repo-root
//     base, clearing run/component.rs row 2787.
import { execSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";

const root = "/Users/ueli/Documents/semio";
const discoveryRel = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts";
const discoveryAbs = root + "/" + discoveryRel;
const oldSnapshot = root + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🗑️temp/old-discovery-component.ts";
writeFileSync(oldSnapshot, execSync(`git show HEAD:"${discoveryRel}"`, { cwd: root, maxBuffer: 1024 * 1024 * 64 }).toString());

const oldMod = await import(oldSnapshot);
const newMod = await import(discoveryAbs);

let failures = 0;
const check = (label: string, condition: boolean): void => {
  console.log(`  [${condition ? "PASS" : "FAIL"}] ${label}`);
  if (!condition) failures++;
};

function lineOf(content: string, offset: number): number {
  let line = 1;
  for (let i = 0; i < offset; i++) if (content[i] === "\n") line++;
  return line;
}

console.log("\n=== mechanism (1): derive.rs rows 344, 351 (x2), 1900 ===");
{
  const path = root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs";
  const content = readFileSync(path, "utf8");
  const oldSuppressed = oldMod.inspectRustNonRepoJoinBaseSpans(content);
  const newSuppressed = newMod.inspectRustNonRepoJoinBaseSpans(content);
  const joins = newMod.inspectRustJoinArgumentSpans(content);
  for (const target of [344, 351, 1900]) {
    for (const j of joins.filter((row: any) => lineOf(content, row.start) === target)) {
      check(`line ${target} "${j.value}": fail-before(blocked)=${!oldSuppressed.has(j.start)} pass-after(suppressed)=${newSuppressed.has(j.start)}`, !oldSuppressed.has(j.start) && newSuppressed.has(j.start));
    }
  }
  const regressed = [...oldSuppressed].filter((start) => !newSuppressed.has(start));
  check(`no regression (${oldSuppressed.size} old suppressed rows all still suppressed)`, regressed.length === 0);
}

console.log("\n=== mechanism (1) regression: correct refusals stay blocked ===");
{
  const mcpPath = root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📇️registry/🦀️component.rs";
  const mcpContent = readFileSync(mcpPath, "utf8");
  const mcpSuppressed = newMod.inspectRustNonRepoJoinBaseSpans(mcpContent);
  const mcpJoins = newMod.inspectRustJoinArgumentSpans(mcpContent);
  const row52 = mcpJoins.find((j: any) => lineOf(mcpContent, j.start) === 52);
  check(`mcp registry:52 entry.owner_root.join(..) still blocked (live struct field, not a temp base)`, Boolean(row52) && !mcpSuppressed.has(row52.start));

  const vcPath = root + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w18-mutation-fixture-completeness/🏗️vector-converter/src/main.rs";
  const vcContent = readFileSync(vcPath, "utf8");
  const vcSuppressed = newMod.inspectRustNonRepoJoinBaseSpans(vcContent);
  const vcJoins = newMod.inspectRustJoinArgumentSpans(vcContent);
  for (const target of [146, 155]) {
    const row = vcJoins.find((j: any) => lineOf(vcContent, j.start) === target);
    check(`vector-converter:${target} DirEntry::path() accessor chain still blocked (untraced dynamic base)`, Boolean(row) && !vcSuppressed.has(row.start));
  }
}

console.log("\n=== mechanism (1) synthetic: chain preservation + soundness boundaries ===");
{
  const parentChainOffTemp = `
fn make() -> std::path::PathBuf {
    let leaf = std::env::temp_dir().join("workdir");
    let target = leaf.parent().unwrap().parent().unwrap().join("marker.txt");
    target
}`;
  const oldJoins1 = oldMod.inspectRustJoinArgumentSpans(parentChainOffTemp), oldSup1 = oldMod.inspectRustNonRepoJoinBaseSpans(parentChainOffTemp);
  const newJoins1 = newMod.inspectRustJoinArgumentSpans(parentChainOffTemp), newSup1 = newMod.inspectRustNonRepoJoinBaseSpans(parentChainOffTemp);
  const oldMarker = oldJoins1.find((j: any) => j.value === "marker.txt"), newMarker = newJoins1.find((j: any) => j.value === "marker.txt");
  check(`.parent().unwrap() chain off a temp base: fail-before(blocked)=${!oldSup1.has(oldMarker.start)} pass-after(suppressed)=${newSup1.has(newMarker.start)}`, !oldSup1.has(oldMarker.start) && newSup1.has(newMarker.start));

  const parentChainOffManifest = `
fn make() -> std::path::PathBuf {
    let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    let target = base.parent().unwrap().join("sibling.txt");
    target
}`;
  const newJoins2 = newMod.inspectRustJoinArgumentSpans(parentChainOffManifest), newSup2 = newMod.inspectRustNonRepoJoinBaseSpans(parentChainOffManifest);
  const sibling = newJoins2.find((j: any) => j.value === "sibling.txt");
  check(`a .parent() chain off a CARGO_MANIFEST_DIR base is NOT suppressed by the non-repo checker (it is proven repo-relative by the OTHER prover, never both)`, Boolean(sibling) && !newSup2.has(sibling.start));

  const unknownBase = `
fn make(x: std::path::PathBuf) -> std::path::PathBuf {
    let y = some_unknown_function(x);
    let z = y.parent().unwrap().to_path_buf();
    z.join("thing.txt")
}`;
  const newJoins3 = newMod.inspectRustJoinArgumentSpans(unknownBase), newSup3 = newMod.inspectRustNonRepoJoinBaseSpans(unknownBase);
  const thing = newJoins3.find((j: any) => j.value === "thing.txt");
  check(`a genuinely unknown base (opaque callee) still blocks, chain or not`, Boolean(thing) && !newSup3.has(thing.start));
}

console.log("\n=== mechanism (2): run/component.rs row 2787 ===");
{
  const path = root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs";
  const content = readFileSync(path, "utf8");
  const oldRefs = oldMod.inspectRustManifestPathReferences(content);
  const newRefs = newMod.inspectRustManifestPathReferences(content);
  const target = "✏️s/🔌️plugins/🗒️note/🛂️descriptor.semio";
  const oldRow = oldRefs.find((r: any) => r.value === target), newRow = newRefs.find((r: any) => r.value === target);
  check(`descriptor.semio: fail-before(absent)=${!oldRow} pass-after(proven, base=[])=${Boolean(newRow) && JSON.stringify(newRow?.base) === "[]"}`, !oldRow && Boolean(newRow) && JSON.stringify(newRow.base) === "[]");
  const removed = oldRefs.filter((r: any) => !newRefs.some((n: any) => n.start === r.start && n.value === r.value));
  check(`no regression (0 previously-proven rows removed)`, removed.length === 0);
}

console.log("\n=== mechanism (2) synthetic: env-var branch is NOT matched (soundness boundary) ===");
{
  const envVarVariant = `
fn find_repo_root() -> std::path::PathBuf {
    if let Ok(value) = std::env::var("SEMIO_REPO_ROOT") {
        let candidate = std::path::PathBuf::from(value);
        if candidate.join("nx.json").is_file() { return candidate; }
    }
    let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if dir.join("nx.json").is_file() { return dir; }
        assert!(dir.pop(), "walked past the filesystem root looking for nx.json");
    }
}
fn use_it() -> std::path::PathBuf {
    let root = find_repo_root();
    root.join("some/file.txt")
}`;
  const refs = newMod.inspectRustManifestPathReferences(envVarVariant);
  check(`find_repo_root()'s own env-var-branch variant is NOT proven (different, unverified base)`, !refs.some((r: any) => r.value === "some/file.txt"));

  const bareVariant = `
use std::path::PathBuf;
fn helper_root() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if dir.join("nx.json").is_file() { return dir; }
        assert!(dir.pop(), "walked past the filesystem root looking for nx.json");
    }
}
fn use_it() -> PathBuf {
    let root = helper_root();
    root.join("some/file.txt")
}`;
  const refs2 = newMod.inspectRustManifestPathReferences(bareVariant);
  const row = refs2.find((r: any) => r.value === "some/file.txt");
  check(`a differently-named helper matching the EXACT ancestor-walk-to-nx.json shape is ALSO proven (matched by shape, not name)`, Boolean(row) && JSON.stringify(row.base) === "[]");
}

console.log("\n\nTOTAL FAILURES:", failures);
process.exit(failures > 0 ? 1 : 0);
