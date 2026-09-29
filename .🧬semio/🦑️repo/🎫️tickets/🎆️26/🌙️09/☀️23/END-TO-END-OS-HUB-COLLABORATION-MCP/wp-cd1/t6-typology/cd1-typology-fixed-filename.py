#!/usr/bin/env python3
"""🗂️ CD1 T6 set — CAD typology assets back on their fixed filename.

The 09-05 path codemod (3a6a9d6bfc) renamed 7 `🗂️typologies/<X>/🔣️typology.json` to `🔣️typology-<hash6>.json`; the TS
runtime glob only matches the fixed name, so TS registered 31 of 38 typologies (Rust embeds explicit paths → 38) and every
`structure.*` typology lookup, style and `from_building` target missed (geometry ×3 + renderer ×2 vitest reds). This set:
  1. renames the 7 files back to `🔣️typology.json` (the `fixedSourceFilename` the artifact projection golden declares);
  2. repoints the CAD Rust registry's 7 `include_str!` paths;
  3. turns the golden's 7 `liveBindings` rows into identity rows and re-registers its sha256 in the library taxonomy
     (`📚️library/🔣️taxonomy.json` — FROZEN during the chain) and in `❄️frozen-coordinate-evidence/🔣️.json`;
  4. adds the census laws: Rust (every typology folder holds exactly the fixed filename and is embedded byte for byte)
     and TS (the registry registers exactly the typology folders on disk; replaces the masking `>= 27`).
Set root: $CD1_ROOT (default: the live repo). Usage: cd1-typology-fixed-filename.py [--dry-run (default) | --write | --revert]"""
import hashlib, json, os, shutil, sys, time

REPO = os.environ.get("CD1_ROOT", "/Users/ueli/Documents/semio")
TAG = "live" if REPO == "/Users/ueli/Documents/semio" else "overlay"
BACKUPS = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-cd1-t6/typology"
MD = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions"
RENAMES = [
    ("🌉️aec.building.structure.classic/🗂️typologies/🏛️ReinforcedConcreteColumn", "🔣️typology-20ac89.json"),
    ("🌉️aec.building.structure.classic/🗂️typologies/🚧️ReinforcedConcreteInternalWall", "🔣️typology-87a300.json"),
    ("🌉️aec.building.structure.classic/🗂️typologies/🛡️ReinforcedConcreteExternalWall", "🔣️typology-9f56a1.json"),
    ("🌉️aec.building.structure.classic/🗂️typologies/🧱️OneWayReinforcedConcreteSlab", "🔣️typology-6a95a6.json"),
    ("📏️aec.building.structure.fem.line/🗂️typologies/📏️LineElement", "🔣️typology-3ad563.json"),
    ("🗺️aec.building.structure.fem.surface/🗂️typologies/🗺️SurfaceElement", "🔣️typology-b35a36.json"),
    ("🧊️aec.building.structure.fem.solid/🗂️typologies/🧊️SolidElement", "🔣️typology-4e128f.json"),
]
FIXED = "🔣️typology.json"
RUST = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🧬️typology/🦀️.rs"
RUST_TESTS = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🧬️typology/🧪️tests/🔬️unit/🦀️.rs"
GOLDEN = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json"
TAXONOMY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
FROZEN = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/❄️frozen-coordinate-evidence/🔣️.json"
TS_TESTS = "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🧪️tests/🧪️semio-tech-cad-js-core-vec/🟦️.ts"

RUST_LAW_ANCHOR = "//#endregion 🔖️Assets\n"
RUST_LAW = '''
/// 🗂️ LAW: every typology folder on disk (`<model definition>/🗂️typologies/<typology>/`) holds exactly the fixed
/// `🔣️typology.json` — the filename the TS runtime glob and the artifact projection golden
/// (`📚️library/🧫️fixtures/📐️cad-draw-path-projection`, `fixedSourceFilename`) declare — and this registry embeds every one
/// of them byte for byte. A renamed asset used to vanish from the TS registry silently while this one still embedded it.
#[test]
fn every_typology_folder_holds_the_fixed_filename_and_is_embedded() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions");
    let mut on_disk = Vec::new();
    for model_definition in std::fs::read_dir(&root).expect("model definitions root") {
        let Ok(folders) = std::fs::read_dir(model_definition.expect("model definition").path().join("🗂️typologies")) else {
            continue;
        };
        for folder in folders {
            let folder = folder.expect("typology folder").path();
            let names: Vec<String> = std::fs::read_dir(&folder).expect("typology folder entries").map(|entry| entry.expect("typology entry").file_name().to_string_lossy().into_owned()).collect();
            assert_eq!(names, vec!["🔣️typology.json".to_string()], "{} must hold exactly the fixed 🔣️typology.json", folder.display());
            on_disk.push(std::fs::read_to_string(folder.join("🔣️typology.json")).expect("typology asset"));
        }
    }
    assert_eq!(on_disk.len(), RAW_TYPOLOGY_ASSETS.len(), "typology folders on disk vs embedded typology assets");
    for raw in &on_disk {
        assert!(RAW_TYPOLOGY_ASSETS.iter().any(|(_, embedded)| embedded == raw), "a typology on disk is not embedded: {}", &raw[..raw.len().min(160)]);
    }
}
'''

TS_OLD = '''    it("loads geometry and AEC typology assets", () => {
      const typologies = listModelDefinitionTypologies();
      expect(typologies.length).toBeGreaterThanOrEqual(27);
'''
TS_NEW = '''    it("loads geometry and AEC typology assets", async () => {
      const census = await typologyAssetCensus(source);
      expect(census.misnamed).toEqual([]);
      expect(listModelDefinitionTypologies().map((row) => row.id).sort()).toEqual(census.ids);
'''
TS_HELPER_ANCHOR = "type TestSource = { readonly url: string };\n"
TS_HELPER = '''
const MODEL_DEFINITION_ASSETS = "../../../../🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/";

/** 🗂️ The typology assets on disk: every `<model definition>/🗂️typologies/<typology>/` must hold exactly the fixed
 * `🔣️typology.json` the runtime glob and the artifact projection golden declare — `misnamed` lists every other file, `ids`
 * the sorted ids of the fixed-name assets. */
async function typologyAssetCensus(source: TestSource): Promise<{ readonly misnamed: readonly string[]; readonly ids: readonly string[] }> {
  const { readdir, readFile } = await import("node:fs/promises");
  const root = new URL(MODEL_DEFINITION_ASSETS, source.url);
  const misnamed: string[] = [];
  const ids: string[] = [];
  for (const modelDefinition of await readdir(root, { withFileTypes: true })) {
    if (!modelDefinition.isDirectory()) continue;
    const typologies = new URL(`${encodeURIComponent(modelDefinition.name)}/🗂️typologies/`, root);
    const folders = await readdir(typologies, { withFileTypes: true }).catch(() => []);
    for (const folder of folders) {
      const folderUrl = new URL(`${encodeURIComponent(folder.name)}/`, typologies);
      for (const file of await readdir(folderUrl)) {
        if (file !== "🔣️typology.json") misnamed.push(`${modelDefinition.name}/🗂️typologies/${folder.name}/${file}`);
        else ids.push((JSON.parse(await readFile(new URL(encodeURIComponent(file), folderUrl), "utf8")) as { readonly id: string }).id);
      }
    }
  }
  return { misnamed, ids: ids.sort() };
}
'''

def path(rel): return os.path.join(REPO, rel)
def read(rel): return open(path(rel), encoding="utf-8", newline="").read()
def sha(text): return hashlib.sha256(text.encode("utf-8")).hexdigest()

def plan():
    """🧭️ The complete post-state, computed from the live pre-state (refuses if an anchor drifted)."""
    edits = {}
    rust = read(RUST)
    for _, hashed in RENAMES:
        if rust.count(hashed) != 1: raise SystemExit(f"anchor: {RUST} has {rust.count(hashed)}× {hashed}")
        rust = rust.replace(hashed, FIXED)
    edits[RUST] = rust
    tests = read(RUST_TESTS)
    if tests.count(RUST_LAW_ANCHOR) != 1 or "every_typology_folder_holds_the_fixed_filename_and_is_embedded" in tests: raise SystemExit(f"anchor: {RUST_TESTS}")
    edits[RUST_TESTS] = tests.replace(RUST_LAW_ANCHOR, RUST_LAW.lstrip("\n") + "\n" + RUST_LAW_ANCHOR, 1)
    golden = read(GOLDEN)
    old_sha = sha(golden)
    for _, hashed in RENAMES:
        if golden.count(hashed) != 1: raise SystemExit(f"anchor: golden has {golden.count(hashed)}× {hashed}")
        golden = golden.replace(hashed, FIXED)
    parsed = json.loads(golden)
    bindings = parsed["projections"][0]["liveBindings"]
    for folder, _ in RENAMES:
        row = next(b for b in bindings if b["source"] == f"{folder}/{FIXED}")
        if row["live"] != row["source"]: raise SystemExit(f"golden binding not identity: {row}")
    edits[GOLDEN] = golden
    new_sha = sha(golden)
    for rel in (TAXONOMY, FROZEN):
        text = read(rel)
        if text.count(old_sha) != 1: raise SystemExit(f"anchor: {rel} has {text.count(old_sha)}× the golden sha {old_sha}")
        edits[rel] = text.replace(old_sha, new_sha)
    ts = read(TS_TESTS)
    if ts.count(TS_OLD) != 1 or ts.count(TS_HELPER_ANCHOR) != 1: raise SystemExit(f"anchor: {TS_TESTS}")
    edits[TS_TESTS] = ts.replace(TS_OLD, TS_NEW).replace(TS_HELPER_ANCHOR, TS_HELPER_ANCHOR + TS_HELPER, 1)
    renames = [(f"{MD}/{folder}/{hashed}", f"{MD}/{folder}/{FIXED}") for folder, hashed in RENAMES]
    for src, dst in renames:
        if not os.path.exists(path(src)) or os.path.exists(path(dst)): raise SystemExit(f"rename precondition: {src}")
    return edits, renames, (old_sha, new_sha)

def applied():
    return all(os.path.exists(path(f"{MD}/{folder}/{FIXED}")) and not os.path.exists(path(f"{MD}/{folder}/{hashed}")) for folder, hashed in RENAMES) and "every_typology_folder_holds_the_fixed_filename_and_is_embedded" in read(RUST_TESTS)

def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else "--dry-run"
    if mode == "--revert":
        stamps = sorted(s for s in os.listdir(BACKUPS) if s.endswith(TAG)) if os.path.isdir(BACKUPS) else []
        if not stamps: raise SystemExit("no backup to revert")
        backup = os.path.join(BACKUPS, stamps[-1])
        manifest = json.load(open(os.path.join(backup, "manifest.json"), encoding="utf-8"))
        for src, dst in manifest["renames"]:
            if os.path.exists(path(dst)) and not os.path.exists(path(src)): os.rename(path(dst), path(src))
        for rel in manifest["edits"]:
            shutil.copyfile(os.path.join(backup, "files", rel), path(rel))
        print(f"reverted from {backup}")
        return
    if applied():
        print("already applied")
        return
    edits, renames, (old_sha, new_sha) = plan()
    print(f"{len(renames)} renames, {len(edits)} file edits, golden sha {old_sha[:12]} -> {new_sha[:12]}")
    for rel in edits: print("  edit", rel)
    if mode != "--write":
        print("dry-run clean")
        return
    backup = os.path.join(BACKUPS, time.strftime("%Y%m%d-%H%M%S") + "-" + TAG)
    for rel in edits:
        os.makedirs(os.path.dirname(os.path.join(backup, "files", rel)), exist_ok=True)
        shutil.copyfile(path(rel), os.path.join(backup, "files", rel))
    json.dump({"edits": list(edits), "renames": renames}, open(os.path.join(backup, "manifest.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    for src, dst in renames: os.rename(path(src), path(dst))
    for rel, text in edits.items(): open(path(rel), "w", encoding="utf-8", newline="").write(text)
    print(f"written; backup {backup}")

main()
