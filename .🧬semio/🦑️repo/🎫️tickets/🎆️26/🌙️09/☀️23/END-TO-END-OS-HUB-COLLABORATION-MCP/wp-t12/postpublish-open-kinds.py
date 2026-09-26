#!/usr/bin/env python3
"""🪪️ T12 post-publish patch set (session 12, coordinator decisions 23:0x/23:3x): every editor that edits a persisted
document is creatable and openable over the hub through the ONE open-target rule (`app_opens_kind`).

Parts (each hunk is anchored and must match exactly once; `--dry-run` proves every anchor, `--write` applies):
  A  stdio kind ids `stdio.<x>` → `s.stdio.<x>` (the id its dialects, artifact definition and codec ids already use, gltf's
     precedent): 35 `artifact_kind()` ids, 25 native-factory rows in the artifact definitions, the 25 receipts of
     `📜️native-codec-factories.json`, the receipt schema's kind enum, and every test/fixture naming a stdio KIND.
     Document schemas (`stdio.<x>`), format kinds (`export/import_stdio_kinds`), codec/factory/language ids stay.
  B  txt/tsv/html editors declare the kind they edit (no plugin-level kind), one law each. csv/json/xml/md then pair
     through their dialect with the plugin-level native-codec kind.
  C  gis terrain: `semio_s_artifact_gis_gisterrain::artifact_kind()` (`s.gis.gisterrain`, schema `gis.terrain`, the
     native codec it already has), declared at plugin level like the map, with its activation event.
  D  hub fence `local-stdio-gis-open-v1`: keeps the exact two-package native-codec closure, drops the hard-coded target
     list (every target is already validated against its verified descriptor by the one rule at load, grants by role).
  E  census law over every committed descriptor with the allow-list of shells (reasons in the law).
  F  bootstrap package table: stdio opens documents.
  G  the stdio+GIS rotation precondition mirrors the fence (it still demanded exactly one target).
Usage: postpublish-open-kinds.py --dry-run | --write"""
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
STDIO = ROOT / "✏️s/🔌️plugins/🗄️stdio"
HUB = ROOT / "🌎️hub"
write = "--write" in sys.argv
if not write and "--dry-run" not in sys.argv:
    sys.exit(__doc__)
edits: dict[Path, str] = {}
problems: list[str] = []
counts: dict[str, int] = {}


def text(path: Path) -> str:
    if path not in edits:
        edits[path] = path.read_text(encoding="utf-8")
    return edits[path]


def replace(part: str, path: Path, old: str, new: str, count: int = 1) -> None:
    source = text(path)
    found = source.count(old)
    if found != count:
        problems.append(f"{part}: {path.relative_to(ROOT)}: expected {count}× {old[:90]!r}, found {found}")
        return
    edits[path] = source.replace(old, new)
    counts[part] = counts.get(part, 0) + count


def regex(part: str, path: Path, pattern: str, new, count: int) -> None:
    source = text(path)
    updated, found = re.subn(pattern, new, source)
    if found != count:
        problems.append(f"{part}: {path.relative_to(ROOT)}: expected {count}× /{pattern[:80]}/, found {found}")
        return
    edits[path] = updated
    counts[part] = counts.get(part, 0) + count


# A1 — every stdio artifact's own `artifact_kind()` id.
kind_fn = re.compile(r'(fn artifact_kind\(\) -> (?:semio_framework_plugin::)?ArtifactKindSpec \{\s*(?:semio_framework_plugin::)?ArtifactKindSpec \{\s*id: )"stdio\.([a-z0-9]+)"(\.into\(\),)')
roots = sorted(path for path in (STDIO / "🗿️artifacts").glob("*/🦀️.rs"))
for root in roots:
    source = text(root)
    if "fn artifact_kind()" not in source or "GLTF_ARTIFACT_KIND_ID" in source:
        continue
    regex("A1 artifact_kind ids", root, kind_fn.pattern, r'\1"s.stdio.\2"\3', 1)

# A2 — the native factory row of each artifact definition (one-line and multi-line forms).
for definition in sorted((STDIO / "🗿️artifacts").glob("*/📜️artifact-definition.json")):
    source = text(definition)
    found = len(re.findall(r'"artifact_kind": "stdio\.[a-z0-9]+"', source))
    if found:
        regex("A2 definition native factories", definition, r'"artifact_kind": "stdio\.([a-z0-9]+)"', r'"artifact_kind": "s.stdio.\1"', found)

# A3 — the projected receipts and their schema enum.
regex("A3 receipts", STDIO / "📇️registry/📜️native-codec-factories.json", r'"artifact_kind": "stdio\.([a-z0-9]+)"', r'"artifact_kind": "s.stdio.\1"', 25)
receipt_schema = STDIO / "📇️registry/🧬️schema/🔣️.json"
source = text(receipt_schema)
enum = re.search(r'("artifactKind": \{\s*"enum": \[)(.*?)(\])', source, re.S)
if enum is None or enum.group(2).count('"stdio.') != 25:
    problems.append(f"A3 receipt schema enum: {receipt_schema.relative_to(ROOT)}: artifactKind enum with 25 stdio.<x> rows not found")
else:
    edits[receipt_schema] = source[: enum.start(2)] + re.sub(r'"stdio\.([a-z0-9]+)"', r'"s.stdio.\1"', enum.group(2)) + source[enum.end(2):]
    counts["A3 receipt schema enum"] = 25

# A4 — tests and fixtures naming a stdio KIND.
provider_fixture = HUB / "🗿️artifact-authority/📇️native-openable-provider/🧫️fixtures/🪪️v1/🔣️.json"
replace("A4 hub fixtures", provider_fixture, '"artifactKind": "stdio.json",', '"artifactKind": "s.stdio.json",')
bin_unit = HUB / "🧪️tests/🔬️bin-unit/🦀️.rs"
replace("A4 hub fixtures", bin_unit, 'semio_framework_plugin::Dialect { artifact_kind: "stdio.json", standard: semio_framework_plugin::StandardId("rfc8259")', 'semio_framework_plugin::Dialect { artifact_kind: "s.stdio.json", standard: semio_framework_plugin::StandardId("rfc8259")')
replace("A4 hub fixtures", bin_unit, '.filter(|kind| kind.id == "stdio.json")', '.filter(|kind| kind.id == "s.stdio.json")')
registry = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry"
replace("A4 TS catalog", registry / "✅️trusted-stdio-catalog/🟦️.ts", 'codecs.find((row) => row.artifactKind === "stdio.json")', 'codecs.find((row) => row.artifactKind === "s.stdio.json")')
replace("A4 TS catalog", registry / "🧪️tests/✅️trusted-stdio-catalog/🟦️.ts", 'catalog.nativeCodecs.find((row) => row.artifactKind === "stdio.json")', 'catalog.nativeCodecs.find((row) => row.artifactKind === "s.stdio.json")')
replace("A4 TS catalog", registry / "🧫️fixtures/🧬️trusted-stdio-catalog/🔣️.json", '"expectedOpenTargetKind": "stdio.json",', '"expectedOpenTargetKind": "s.stdio.json",')
catalog_schema = registry / "🧬️schema/🧬️trusted-stdio-catalog/🔣️.json"
replace("A4 TS catalog", catalog_schema, '"expectedOpenTargetKind": { "const": "stdio.json" },', '"expectedOpenTargetKind": { "const": "s.stdio.json" },')
replace("A4 TS catalog", catalog_schema, '"artifactKind": { "const": "stdio.json" },', '"artifactKind": { "const": "s.stdio.json" },')

# B — txt/tsv/html editors declare their kind; one law each.
SUBSET = "🪆️subsets/✳️any/✏️editor"
for artifact, standard, anchor, create, editor, dialect, noun in [
    ("🔤️txt", "🔖️utf-8", '        .document(["semio", "stdio", "txt"])\n', "create_txt_editor", "TxtEditor", "TXT_EDITOR_DIALECT", "a text document"),
    ("📑️tsv", "🔖️iana", '        .document(["semio", "stdio", "tsv"])\n', "create_tsv_editor", "TsvEditor", "TSV_EDITOR_DIALECT", "a TSV table"),
    ("🌐️html", "🔖️5", '        .document(["semio", "html"])\n', "create_html_editor", "HtmlEditor", "HTML_DIALECT", "an HTML document"),
]:
    base = STDIO / "🗿️artifacts" / artifact / "🏅️standards" / standard / SUBSET
    replace("B declarations", base / "🦀️.rs", anchor, anchor + "        .artifact_kind(crate::artifact_kind())\n")
    tail = f"async fn editor_dialect_matches_the_artifact_coordinate() {{\n    assert_eq!(<{editor} as ArtifactEditor>::DIALECT, {dialect});\n}}\n"
    law = (
        "\n/// 🎯️ LAW: the editor declares the artifact kind it edits (the artifact's own `artifact_kind()`), which is\n"
        "/// what the hub's one open-target rule (`app_opens_kind`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`)\n"
        f"/// pairs with this editor and the viewer of its dialect — so {noun} can be created and opened as a hub document.\n"
        "#[semio_framework_async_macros::async_test]\n"
        "async fn the_editor_declares_the_artifact_kind_it_edits() {\n"
        f"    assert_eq!({create}().artifact_kinds, vec![crate::artifact_kind()]);\n"
        "}\n"
    )
    replace("B laws", base / "🧪️tests/🔬️unit/🦀️.rs", tail, tail + law)

# C — the terrain kind, declared at plugin level beside the map (one id with its dialect and native codec).
GIS = ROOT / "✏️s/🔌️plugins/🌍️gis"
terrain = GIS / "🗿️artifacts/🏔️gisterrain/🦀️.rs"
replace(
    "C terrain kind",
    terrain,
    "//#region 🔖️MeshComposition\n",
    "//#region 🔹ArtifactKind\n"
    "/// 🏔️ The canonical GIS terrain artifact-kind declaration: one id with its dialect and its native codec receipt\n"
    "/// (`s.gis.gisterrain`), so the hub's one open-target rule pairs it with the terrain editor and viewer; the payload\n"
    "/// schema stays `gis.terrain`.\n"
    "pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {\n"
    "    semio_framework_plugin::ArtifactKindSpec {\n"
    "        id: GISTERRAIN_DIALECT.artifact_kind.into(),\n"
    '        name: "3D Terrain".into(),\n'
    "        source_format: GIS_3D_TERRAIN_SCHEMA.into(),\n"
    '        component_kind: "gisterrain".into(),\n'
    '        dimension: "3d".into(),\n'
    "        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,\n"
    "        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },\n"
    "        schema: GIS_3D_TERRAIN_SCHEMA.into(),\n"
    "        export_formats: vec![],\n"
    "        import_formats: vec![],\n"
    "        export_stdio_kinds: standards::v1::subsets::any::io::export_stdio_kinds().iter().map(|kind| (*kind).to_owned()).collect(),\n"
    "        import_stdio_kinds: standards::v1::subsets::any::io::import_stdio_kinds().iter().map(|kind| (*kind).to_owned()).collect(),\n"
    "    }\n"
    "}\n"
    "//#endregion 🔹ArtifactKind\n\n"
    "//#region 🔖️MeshComposition\n",
)
gis_root = GIS / "🦀️.rs"
replace("C terrain kind", gis_root, "        .artifact_kind(semio_s_artifact_gis_gismap::artifact_kind())\n", "        .artifact_kind(semio_s_artifact_gis_gismap::artifact_kind())\n        .artifact_kind(semio_s_artifact_gis_gisterrain::artifact_kind())\n")
replace("C terrain kind", gis_root, "        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_gis_gismap::artifact_kind().id })\n", "        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_gis_gismap::artifact_kind().id })\n        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_gis_gisterrain::artifact_kind().id })\n")

# D — the stdio+GIS fence pins its native-codec closure; the one rule decides its targets.
trusted = HUB / "🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs"
source = text(trusted)
fence = re.search(r'        if profile\.id == "local-stdio-gis-open-v1" \{\n.*?return Err\(catalog\("local stdio plus GIS profile is not its exact closed two-package map editor and viewer authority"\)\);\n            \}\n        \}\n', source, re.S)
if fence is None:
    problems.append("D fence: block not found")
else:
    edits[trusted] = source[: fence.start()] + (
        '        if profile.id == "local-stdio-gis-open-v1" {\n'
        "            let identities = profile.selected_closure.iter().map(|identity| (identity.plugin_id.as_str(), identity.package_id.as_str())).collect::<Vec<_>>();\n"
        '            let gis = bundle.packages.iter().find(|package| package.plugin_id == "gis");\n'
        '            let stdio = bundle.packages.iter().find(|package| package.plugin_id == "stdio");\n'
        '            if identities != [("gis", "semio:gis"), ("stdio", "semio:stdio")]\n'
        "                || bundle.packages.len() != 2\n"
        "                || gis.is_none_or(|package| {\n"
        "                    package.native_codecs.len() != 2\n"
        "                        || package.dependencies.as_slice() != std::slice::from_ref(&profile.selected_closure[1])\n"
        '                        || !package.native_codecs.iter().any(|codec| codec.artifact_kind == "s.gis.gismap" && codec.artifact_schema == "gis.map")\n'
        '                        || !package.native_codecs.iter().any(|codec| codec.artifact_kind == "s.gis.gisterrain" && codec.artifact_schema == "gis.terrain")\n'
        "                })\n"
        "                || stdio.is_none_or(|package| package.native_codecs.len() != 26 || !package.dependencies.is_empty())\n"
        "                || profile.open_targets.len() != bundle.packages.iter().map(|package| package.open_targets.len()).sum::<usize>()\n"
        "            {\n"
        '                return Err(catalog("local stdio plus GIS profile is not its exact closed two-package native-codec closure opening every package target"));\n'
        "            }\n"
        "        }\n"
    ) + source[fence.end():]
    counts["D fence"] = 1
unit = HUB / "🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs"
replace("D fence laws", unit, "fn local_stdio_gis_profile_is_exact_two_packages_twenty_eight_codecs_and_one_map_editor_and_viewer() {", "fn local_stdio_gis_profile_is_exact_two_packages_twenty_eight_codecs_and_opens_every_package_target() {")
replace(
    "D fence laws",
    unit,
    '    terrain_target.packages[0].open_targets.push(target);\n    assert!(validate_bundle(&terrain_target, "local-stdio-gis-open-v1").expect_err("Terrain target").to_string().contains("exact closed"));\n',
    '    terrain_target.packages[0].open_targets.push(target.clone());\n'
    '    assert!(validate_bundle(&terrain_target, "local-stdio-gis-open-v1").expect_err("a package target the profile does not open").to_string().contains("exact closed"));\n'
    "    terrain_target.profiles[0].open_targets.push(TrustedBundleProfileOpenTargetV1 { package: terrain_target.profiles[0].open_targets[0].package.clone(), target });\n"
    "    terrain_target.profiles[0].generation_id = trusted_profile_generation(&terrain_target, &terrain_target.profiles[0]).expect(\"profile generation\");\n"
    '    assert!(validate_bundle(&terrain_target, "local-stdio-gis-open-v1").is_ok(), "the terrain target the one rule admits is opened by the profile");\n',
)

# E — the census: every committed editor that edits a document opens a kind; shells are named with their reason.
census = (
    "/// 🧭️ Editors of committed descriptors that author no persisted document of their own, each with the reason; every\n"
    "/// other editor surface must open at least one kind through the one rule (coordinator decision, session 12).\n"
    "const EDITORS_WITHOUT_A_DOCUMENT: [(&str, &str); 3] = [\n"
    '    ("s.space.home@1/*#editor", "the launcher: its snapshot holds only the catalog generation it lists (`SHomeSnapshot { schema, catalog_generation }`), never user content"),\n'
    '    ("s.space.studio@1/*#editor", "the `s` shell\'s studio: it edits the running shell\'s OS-owned workflow (`semio_framework_os::WorkflowSnapshot`) and has no artifact kind of its own"),\n'
    '    ("s.playbook.procedural@1/*#editor", "a `playbook.blockKind` module: its snapshot is the host playbook block\'s render payload (foreign document codec), never a document of its own"),\n'
    "];\n\n"
    "/// 🗺️ LAW (census): every editor surface of every committed, isolated package descriptor opens at least one artifact\n"
    "/// kind through [`descriptor_open_targets`], so every document kind with an editor is creatable and openable over the\n"
    "/// hub; the only exceptions are [`EDITORS_WITHOUT_A_DOCUMENT`], and each of those must still exist.\n"
    "#[test]\n"
    "fn every_committed_editor_that_edits_a_document_opens_a_kind_through_the_one_rule() {\n"
    '    let mut pending = vec![PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../✏️s/🔌️plugins")];\n'
    "    let mut descriptors = Vec::new();\n"
    "    while let Some(directory) = pending.pop() {\n"
    '        for entry in std::fs::read_dir(&directory).unwrap_or_else(|error| panic!("{}: {error}", directory.display())) {\n'
    "            let path = entry.expect(\"plugin tree entry\").path();\n"
    '            if path.is_dir() && !matches!(path.file_name().and_then(|name| name.to_str()), Some("dist" | "target" | "node_modules")) {\n'
    "                pending.push(path);\n"
    '            } else if path.file_name().and_then(|name| name.to_str()) == Some("🛂️.descriptor.semio") {\n'
    "                descriptors.push(path);\n"
    "            }\n"
    "        }\n"
    "    }\n"
    "    let (mut editors, mut unopened, mut shells) = (0usize, Vec::new(), std::collections::BTreeSet::new());\n"
    "    for path in &descriptors {\n"
    '        let descriptor = decode_package_descriptor(&std::fs::read(path).expect("committed descriptor")).unwrap_or_else(|error| panic!("{}: {error}", path.display()));\n'
    "        if descriptor.execution != semio_framework::ExecutionMode::Isolated {\n"
    "            continue;\n"
    "        }\n"
    "        let opened = descriptor_open_targets(&descriptor).into_iter().map(|target| target.surface_id).collect::<std::collections::BTreeSet<_>>();\n"
    "        for app in descriptor.manifest.apps.iter().filter(|app| app.role == semio_framework::AppRole::Editor && app.id == semio_framework::surface_app_id(&app.dialect, app.role)) {\n"
    "            editors += 1;\n"
    "            if opened.contains(&app.id) {\n"
    "                continue;\n"
    "            }\n"
    "            match EDITORS_WITHOUT_A_DOCUMENT.iter().find(|(surface, _)| *surface == app.id) {\n"
    "                Some((surface, _)) => {\n"
    "                    shells.insert(*surface);\n"
    "                }\n"
    '                None => unopened.push(format!("{} ({})", app.id, path.display())),\n'
    "            }\n"
    "        }\n"
    "    }\n"
    '    assert!(editors > 0, "no committed editor surface found");\n'
    '    assert!(unopened.is_empty(), "{} of {editors} editor surfaces open no kind: {unopened:#?}", unopened.len());\n'
    '    assert_eq!(shells.len(), EDITORS_WITHOUT_A_DOCUMENT.len(), "every allow-listed shell still exists and still opens nothing: {shells:?}");\n'
    "}\n\n"
    "mod long {\n"
)
replace("E census law", unit, "mod long {\n", census)

# F — the bootstrap package table: stdio opens documents now.
script = HUB / "📦️packages/🦀️rust/📜️script.ts"
replace("F bootstrap", script, 'linkedCodecRegistry: "✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json", opensDocuments: false }', 'linkedCodecRegistry: "✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json", opensDocuments: true }')

# G — the stdio+GIS rotation mirrors the fence (its "exactly one open target" precondition was already stale: the
# profile has opened the map editor AND viewer since session 11).
replace(
    "G rotation",
    script,
    '    if (!Array.isArray(profile.openTargets) || profile.openTargets.length !== 1) throw new Error("trusted rotation source profile does not declare exactly one open target");\n',
    "    const packageTargets = bundle.packages.reduce((total: number, record: any) => total + record.openTargets.length, 0);\n"
    '    if (!Array.isArray(profile.openTargets) || profile.openTargets.length !== packageTargets) throw new Error("trusted rotation source profile does not open every package target");\n',
)

for part, count in sorted(counts.items()):
    print(f"{part}: {count}")
print(f"files={len([path for path, source in edits.items() if source != path.read_text(encoding='utf-8')])} problems={len(problems)} write={write}")
for problem in problems:
    print("PROBLEM", problem)
if write and not problems:
    for path, source in edits.items():
        if source != path.read_text(encoding="utf-8"):
            path.write_text(source, encoding="utf-8")
