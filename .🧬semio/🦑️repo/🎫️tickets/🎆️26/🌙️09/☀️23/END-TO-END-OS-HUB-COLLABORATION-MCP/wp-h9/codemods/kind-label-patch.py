#!/usr/bin/env python3
"""Per-kind localized label patch set (H9 session 12, item L) — lands in the post-publish window (ABI freeze rule 1).

`ArtifactKindSpec.name: String` (one English string) becomes `label: LocalizedLabel` (en + de), and every consumer that
presented a kind by its editor app's label presents the kind's own label: the hub creation catalog
(`artifact_creation_catalog`), the local picker (`artifact_kind_choices`) and the host's `OsArtifactDescriptor`.
Guests emit the new descriptor field, so this changes the plugin descriptor wire shape: every guest must be rebuilt
and the catalog republished after landing (W2).

usage: kind-label-patch.py [--apply]   (default: dry run — verifies every anchor, prints the plan, writes nothing)
After --apply: `bun nx run @semio-tech/framework:generate` (regenerates `🤖️generated/🪪️manifest/🟦️.ts`), then
`cargo check` of semio-framework, semio-framework-plugin, semio-framework-os-host, semio-hub (+ wasm32 for one plugin),
then fix every residual `ArtifactKindSpec.name`/`OsArtifactDescriptor.name` read the compiler names (compile-atomic).
"""
import re
import sys

ROOT = "/Users/ueli/Documents/semio/"
APPLY = "--apply" in sys.argv

GERMAN = {
    "2D": "2D", "3D": "3D",
    "2D Drawing": "2D-Zeichnung", "2D Generation": "2D-Generierung", "2D Grid": "2D-Gitter", "2D Image": "2D-Bild",
    "2D Map": "2D-Karte", "2D Puzzle": "2D-Puzzle", "2D Raster": "2D-Rasterbild", "2D Shooting": "2D-Aufnahme",
    "3D CAD": "3D-CAD", "3D Generation": "3D-Generierung", "3D Grid": "3D-Gitter", "3D Lowpoly": "3D-Lowpoly",
    "3D Process": "3D-Prozess", "3D Puzzle": "3D-Puzzle", "3D Remodeling": "3D-Umbau", "5D Puzzle": "5D-Puzzle",
    "Animate Presentation": "Animierte Präsentation", "Architect Program": "Raumprogramm", "Binary": "Binärdatei",
    "Builder Test 3D": "Builder-Test 3D", "DAG": "DAG", "Deflate": "Deflate", "Energy Model": "Energiemodell",
    "Equation": "Gleichung", "FEM 2D Results": "FEM-2D-Ergebnisse", "FEM 3D Results": "FEM-3D-Ergebnisse",
    "Fixture Number": "Fixture-Zahl", "Flow": "Fluss", "Form Dictionary": "Formularwörterbuch",
    "Kind Catalogue": "Typenkatalog", "Kit Catalog": "Bausatzkatalog", "Kit Catalogue": "Bausatzkatalog",
    "Layout": "Layout", "Node Kind": "Knotentyp", "Note": "Notiz", "Object Kind": "Objekttyp", "Part Kind": "Bauteiltyp",
    "Playbook": "Playbook", "Playground Document": "Playground-Dokument", "Procedure": "Prozedur", "S Home": "S-Start",
    "Semio": "Semio", "Sequence": "Sequenz", "Sourcing Curation": "Beschaffungskuratierung",
    "Space Artifacts": "Space-Artefakte", "Test 2D": "Test 2D", "Test 3D": "Test 3D", "Test Doc A": "Testdokument A",
    "Test Doc B": "Testdokument B", "Text Document": "Textdokument", "Trinity Graph": "Trinity-Graph",
    "Trinity Rewrite Rule": "Trinity-Umschreibregel", "VCS Document": "VCS-Dokument", "W1b Export Bug Proof": "W1b-Exportfehlernachweis",
    "WFC Bitmap": "WFC-Bitmap", "Wires Graph": "Leitungsgraph", "Format Roundtrip": "Format-Rundlauf",
}
FORMATS = ["Avi", "Bcf", "Bmp", "Csv", "Docx", "Dwg", "Dxf", "Epw", "Gif", "Gltf", "Html", "Ifc", "Jpg", "Json", "Las", "Md", "Mp3",
           "Mp4", "Obj", "Pdf", "Ply", "Png", "Pptx", "Step", "Stl", "Svg", "Tiff", "Tsv", "Txt", "Wav", "Xlsx", "Xml", "Zip"]
GERMAN.update({name: name for name in FORMATS})

FRAMEWORK_LABEL_PATH = {
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️app-router/🦀️.rs": "semio_framework_plugin::LocalizedLabel",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔬️plugin-builder-dependency/🦀️.rs": "crate::LocalizedLabel",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs": "LocalizedLabel",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs": "crate::LocalizedLabel",
    "🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🔬️workflow-unit/🦀️.rs": "semio_framework_plugin::LocalizedLabel",
    "🌎️hub/🗿️artifact-authority/🧪️tests/🔬️unit/🦀️.rs": "semio_framework_plugin::LocalizedLabel",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🦀️.rs": "semio_framework::LocalizedLabel",
}

LITERAL = re.compile(r'(?P<path>[a-z_:]*)ArtifactKindSpec \{')
NAME = re.compile(r'\bname: (?P<expr>"(?P<text>[^"]*)"\.(?:into|to_string)\(\)|id\.into\(\)|label\.into\(\)),')

# 🪛️ Exact hunks outside the literal sites: the type, the host descriptor, and the two pickers' label sources.
HUNKS = [
    ("🧰️framework/🔨️modules/🛂️manifest/🦀️.rs",
     "pub struct ArtifactKindSpec {\n    pub id: String,\n    pub name: String,\n",
     "pub struct ArtifactKindSpec {\n    pub id: String,\n    /// 🗣️ The kind's own localized name — what a picker shows for it, never its editor app's label.\n    pub label: LocalizedLabel,\n"),
    ("🧰️framework/🔨️modules/🛂️manifest/🦀️.rs",
     "            by_coordinate.entry(app.dialect.to_coordinate()).or_insert_with(|| ArtifactKindChoice { kind_id: app.dialect.artifact_kind.clone(), schema: app.io.artifact_schema.clone(), dialect: app.dialect.clone(), label: app.label.clone() });",
     "            let label = app.artifact_kinds.iter().chain(manifest.artifact_kinds.iter()).find(|kind| kind.schema == app.io.artifact_schema).map_or_else(|| app.label.clone(), |kind| kind.label.clone());\n"
     "            by_coordinate.entry(app.dialect.to_coordinate()).or_insert_with(|| ArtifactKindChoice { kind_id: app.dialect.artifact_kind.clone(), schema: app.io.artifact_schema.clone(), dialect: app.dialect.clone(), label });"),
    ("🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs",
     "                kind: spec.id.clone(),\n                name: spec.name.clone(),\n",
     "                kind: spec.id.clone(),\n                label: spec.label.clone(),\n"),
    ("🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs",
     "            let app = retained.descriptor.manifest.apps.iter().find(|app| app.id == selection.surface.app_id && app.dialect == selection.parent_dialect)?;\n",
     "            let app = retained.descriptor.manifest.apps.iter().find(|app| app.id == selection.surface.app_id && app.dialect == selection.parent_dialect)?;\n"
     "            let kind = app.artifact_kinds.iter().chain(retained.descriptor.manifest.artifact_kinds.iter()).find(|kind| kind.id == selection.artifact.kind)?;\n"),
    ("🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs",
     "                    en: app.label.resolve(semio_framework::Terminology::Native, semio_framework::Locale::En).to_string(),\n                    de: app.label.resolve(semio_framework::Terminology::Native, semio_framework::Locale::De).to_string(),\n",
     "                    en: kind.label.resolve(semio_framework::Terminology::Native, semio_framework::Locale::En).to_string(),\n                    de: kind.label.resolve(semio_framework::Terminology::Native, semio_framework::Locale::De).to_string(),\n"),
    ("🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/🗄️artifact-kind-formats.json",
     '  "name": "Format Roundtrip",\n',
     '  "label": { "native": { "de": "Format-Rundlauf", "en": "Format Roundtrip" } },\n'),
]


def literal_bodies(text):
    for match in LITERAL.finditer(text):
        if text[max(0, match.start() - 11):match.start()].endswith("pub struct "):
            continue
        index, depth = match.end(), 1
        while depth and index < len(text):
            depth += {"{": 1, "}": -1}.get(text[index], 0)
            index += 1
        yield match.group("path"), match.end(), index


def label_path(file, struct_path):
    if file in FRAMEWORK_LABEL_PATH:
        return FRAMEWORK_LABEL_PATH[file]
    if struct_path:
        return f"{struct_path}LocalizedLabel"
    return "semio_framework_plugin::LocalizedLabel"


def rewrite(file, text, plan, missing):
    edits = []
    for struct_path, start, end in literal_bodies(text):
        body = text[start:end]
        found = NAME.search(body)
        if not found:
            continue
        path = label_path(file, struct_path)
        expr = found.group("expr")
        if found.group("text") is not None:
            english = found.group("text")
            if english not in GERMAN:
                missing.add(english)
                continue
            replacement = f'label: {path}::native("{english}", "{GERMAN[english]}"),'
        elif expr == "label.into()":
            replacement = f"label: {path}::data(label),"
        else:
            replacement = f"label: {path}::data(id),"
        edits.append((start + found.start(), start + found.end(), replacement))
    for begin, finish, replacement in reversed(edits):
        text = text[:begin] + replacement + text[finish:]
    plan.append((file, len(edits)))
    return text


def main():
    files = [line for line in open(ROOT + ".tmp-ticket/wp-h9/generated/kindspec-files.txt", encoding="utf-8").read().split("\n") if line]
    plan, missing, failures = [], set(), []
    changed = {}
    for file in files:
        text = open(ROOT + file, encoding="utf-8").read()
        changed[file] = rewrite(file, text, plan, missing)
    for file, old, new in HUNKS:
        text = changed.get(file) or open(ROOT + file, encoding="utf-8").read()
        if text.count(old) != 1:
            failures.append(f"{file}: anchor matched {text.count(old)}× — {old[:80]!r}")
            continue
        changed[file] = text.replace(old, new)
    literal_sites = sum(count for _, count in plan)
    print(f"literal sites rewritten: {literal_sites} in {sum(1 for _, count in plan if count)} files; exact hunks: {len(HUNKS) - len(failures)}/{len(HUNKS)}")
    for file, count in plan:
        if count:
            print(f"  {count:3d}  {file}")
    if missing:
        print("names without a German label:", sorted(missing))
    for failure in failures:
        print("ANCHOR", failure)
    if missing or failures:
        sys.exit(1)
    if APPLY:
        for file, text in changed.items():
            open(ROOT + file, "w", encoding="utf-8").write(text)
        print("applied")
    else:
        print("dry run clean — nothing written")


main()
