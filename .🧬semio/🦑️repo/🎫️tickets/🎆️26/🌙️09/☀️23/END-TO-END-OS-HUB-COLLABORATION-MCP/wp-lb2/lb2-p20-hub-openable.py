#!/usr/bin/env python3
"""🌐️ LB2 p20 (stdio + framework + hub pins): every stdio kind with an editor is openable and creatable on the hub.

H14's census (📓️wp-h14.md § Session 15) and LB2's p17 answers (`s14-lb2-captures/p17-s2-codec-answers`), measured
2026-09-29: stdio editors open 56 `(kind, document schema)` rows, but the stdio package publishes only its LINKED native
codecs as hub codec rows (29), and a hosted row binds the owner's row. 28 opened rows (counting pdf 1.7) had no owner row, so they were neither
openable nor creatable: binary, bmp, epw, wav, gif (87a `stdio.gif`, 89a `stdio.gif.89a`), ifc (4 `stdio.ifc`, 2x3
`stdio.ifc.2x3`), the 19 semio subsets, and pdf 1.7. pdf's linked row named `stdio.pdf`, the 1.4 model that no app opens since
p12. And a kind whose standards share one schema (dwg `ac1018`/`ac1024`, pdf `1.4`/`1.7`) had no single creation surface.

Fix, one rule per layer:
- stdio: every `(kind, schema)` an editor opens has exactly ONE linked native codec, declared schema-first. Each of the 8
  roots declares a `codecs[]` entry per schema: `native-document` codec of the schema's standard, factory
  `stdio.native.<schema without stdio.>.v1`, the existing runtime codec capability, and the pack hash = SHA-256 of that
  snapshot's protocol (p17's identity). Each root also gets `native_codecs()` factories (`ArtifactCodec::of::<Snapshot, Mutation>`
  of the declaration's own document codec) and `definition()` binds them as executables. The committed receipts projection
  grows 29 → 56: 28 new rows, and pdf's row moves to `stdio.pdf.1.7`. pdf's `ArtifactKindSpec` names `stdio.pdf.1.7`.
  `native_codec_artifact_kinds` answers each kind once (36), while the factory count is 56.
- framework: `codec.*` may be asked by a dialect coordinate `<kind>@<standard>/<subset>`, which names exactly that surface. The
  hub's per-dialect creation entry (H14 row 36) passes `parent_dialect.to_coordinate()`, so genesis is dialect-aware without a
  WIT change.
- hub: provider pins 29 → 56 (stdio) and 32 → 59 (set), and the publisher's native-provider / bootstrap oracles plus their
  fixtures (frontier 59, provider fixture 56, stdio+gis bootstrap 56 / limit 58). The stdio native-catalog-surface fixture
  names its kind count `kindCount` (36), because the kinds are no longer as many as the codecs. The publisher's linked-ownership rule matches a declared
  `(kind, schema)` against all of the kind's linked rows, not the first one: a kind may carry several schemas.
- laws: stdio `shipped_fleet` LAW (g) — per package in its own process, every editor's `(kind, schema)` has one linked owner
  row whose hash the package answers, and genesis by the editor's coordinate creates a zero-history document of that schema;
  framework fixture law `a_dialect_coordinate_selects_exactly_one_creation_surface`; hub TS twin in
  `🧪️tests/⛓️linked-codec-ownership` (committed descriptors: every editor's pair is owned) + a multi-schema fixture case.

ORDER: after T7d (p17): LAW (g) extends p17's `describe()`, so `--dry-run` passes only on a tree where p17 is written.
Every other edit anchors identically before and after p17.
usage: python3 lb2-p20-hub-openable.py --dry-run | --write | --revert [--root <tree>]
Backups (byte-exact, per root) under `.🧬semio/🌐hub/s14-lb2-backup/p20/<root-hash>/`.
"""
import hashlib, json, os, re, shutil, sys

TREE = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
BACKUP = f"/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-backup/p20/{hashlib.sha256(TREE.encode()).hexdigest()[:12]}"
STDIO = "✏️s/🔌️plugins/🗄️stdio"
ART = f"{STDIO}/🗿️artifacts"
PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
FIXTURE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs"
REGISTRY_JSON = f"{STDIO}/📇️registry/📜️native-codec-factories.json"
REGISTRY_RS = f"{STDIO}/📇️registry/🦀️.rs"
STDIO_PROVIDER_TEST = f"{STDIO}/🧪️tests/📇️native-openable-provider/🦀️.rs"
LAW = f"{STDIO}/🧪️tests/🚢️shipped-fleet/🦀️.rs"
HUB_PROVIDER = "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs"
PUBLISHER = "🌎️hub/📦️packages/🦀️rust/📜️script.ts"
OWNERSHIP_FIXTURE = "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/⛓️linked-codec-ownership/🔣️.json"
OWNERSHIP_LAW = "🌎️hub/🧪️tests/⛓️linked-codec-ownership/🟦️.ts"
PROTOCOL = "💾️binary/📡️.protocol.semio"
problems = []

SEMIO_SUBSETS = [
    ("base", "SemioSnapshot", "SemioMutation", "STDIO_SEMIO_DOCUMENT_SCHEMA"),
    ("animation", "SemioAnimationSnapshot", "SemioAnimationMutation", "STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA"),
    ("audio", "SemioAudioSnapshot", "SemioAudioMutation", "STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA"),
    ("brep", "SemioBrepSnapshot", "SemioBrepMutation", "STDIO_SEMIOBREP_DOCUMENT_SCHEMA"),
    ("cad", "SemioCadSnapshot", "SemioCadMutation", "STDIO_SEMIOCAD_DOCUMENT_SCHEMA"),
    ("document", "SemioDocumentSnapshot", "SemioDocumentMutation", "STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA"),
    ("drawing", "SemioDrawingSnapshot", "SemioDrawingMutation", "STDIO_SEMIODRAWING_DOCUMENT_SCHEMA"),
    ("flow", "SemioFlowSnapshot", "SemioFlowMutation", "STDIO_SEMIOFLOW_DOCUMENT_SCHEMA"),
    ("graph", "SemioGraphSnapshot", "SemioGraphMutation", "STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA"),
    ("image", "SemioImageSnapshot", "SemioImageMutation", "STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA"),
    ("kit", "SemioKitSnapshot", "SemioKitMutation", "STDIO_SEMIOKIT_DOCUMENT_SCHEMA"),
    ("mesh", "SemioMeshSnapshot", "SemioMeshMutation", "STDIO_SEMIOMESH_DOCUMENT_SCHEMA"),
    ("model", "SemioModelSnapshot", "SemioModelMutation", "STDIO_SEMIOMODEL_DOCUMENT_SCHEMA"),
    ("object", "SemioObjectSnapshot", "SemioObjectMutation", "STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA"),
    ("presentation", "SemioPresentationSnapshot", "SemioPresentationMutation", "STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA"),
    ("table", "SemioTableSnapshot", "SemioTableMutation", "STDIO_SEMIOTABLE_DOCUMENT_SCHEMA"),
    ("text", "SemioTextSnapshot", "SemioTextMutation", "STDIO_SEMIOTEXT_DOCUMENT_SCHEMA"),
    ("value", "SemioValueSnapshot", "SemioValueMutation", "STDIO_SEMIOVALUE_DOCUMENT_SCHEMA"),
    ("video", "SemioVideoSnapshot", "SemioVideoMutation", "STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA"),
]


def semio_codecs():
    rows = []
    for subset, snapshot, mutation, constant in SEMIO_SUBSETS:
        base = f"standards::v1::subsets::{subset}::schema"
        rows.append({"fn": "native_codec" if subset == "base" else f"native_codec_{subset}", "snapshot": f"{base}::snapshot::{snapshot}", "mutation": f"{base}::mutations::{mutation}", "schema_expr": f"{base}::snapshot::{constant}", "type": snapshot, "standard": "v1", "leaf": "native-document" if subset == "base" else f"native-document-{subset}", "slug": None if subset == "base" else subset})
    return rows


# (dir, artifact slug, [codec rows]) — each row: Rust fn name, Snapshot and Mutation paths from the root, schema expression,
# snapshot type name (locates its protocol), standard slug, codec-id leaf, factory slug suffix (None = the kind's own schema).
ROOTS = [
    ("🪟️bmp", "bmp", [{"fn": "native_codec", "snapshot": "BmpSnapshot", "mutation": "BmpMutation", "schema_expr": "STDIO_BMP_DOCUMENT_SCHEMA", "type": "BmpSnapshot", "standard": "v3", "leaf": "native-document", "slug": None}]),
    ("🔊️wav", "wav", [{"fn": "native_codec", "snapshot": "WavSnapshot", "mutation": "WavMutation", "schema_expr": "STDIO_WAV_DOCUMENT_SCHEMA", "type": "WavSnapshot", "standard": "riff-pcm", "leaf": "native-document", "slug": None}]),
    ("🌦️epw", "epw", [{"fn": "native_codec", "snapshot": "EpwSnapshot", "mutation": "EpwMutation", "schema_expr": "STDIO_EPW_DOCUMENT_SCHEMA", "type": "EpwSnapshot", "standard": "energyplus", "leaf": "native-document", "slug": None}]),
    ("💾️binary", "binary", [{"fn": "native_codec", "snapshot": "BinarySnapshot", "mutation": "BinaryMutation", "schema_expr": "STDIO_BINARY_DOCUMENT_SCHEMA", "type": "BinarySnapshot", "standard": "raw", "leaf": "native-document", "slug": None}]),
    ("🏗️ifc", "ifc", [
        {"fn": "native_codec", "snapshot": "IfcSnapshot", "mutation": "IfcMutation", "schema_expr": "STDIO_IFC_DOCUMENT_SCHEMA", "type": "IfcSnapshot", "standard": "4", "leaf": "native-document", "slug": None},
        {"fn": "native_codec_2x3", "snapshot": "standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot", "mutation": "standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation", "schema_expr": "standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA", "type": "Ifc2x3Snapshot", "standard": "2x3", "leaf": "native-document", "slug": "2x3"},
    ]),
    ("🎞️gif", "gif", [
        {"fn": "native_codec", "snapshot": "standards::v87a::subsets::any::schema::snapshot::GifSnapshot", "mutation": "standards::v87a::subsets::any::schema::mutations::GifMutation", "schema_expr": "STDIO_GIF_DOCUMENT_SCHEMA", "type": "GifSnapshot@7️⃣87a", "standard": "87a", "leaf": "native-document", "slug": None},
        {"fn": "native_codec_89a", "snapshot": "GifSnapshot", "mutation": "GifMutation", "schema_expr": "STDIO_GIF89A_DOCUMENT_SCHEMA", "type": "GifSnapshot@9️⃣89a", "standard": "89a", "leaf": "native-document", "slug": "89a"},
    ]),
    ("🧿️semio", "semio", semio_codecs()),
]
PDF_DIR = "📖️pdf"


def once(text, old, new, label):
    count = text.count(old)
    if count != 1:
        problems.append(f"{label}: expected 1 anchor, found {count}")
        return text
    return text.replace(old, new)


def read(path):
    return open(os.path.join(TREE, path), encoding="utf-8").read()


#region Rows
def protocol_of(directory, type_label):
    name, _, folder = type_label.partition("@")
    root = os.path.join(TREE, ART, directory)
    found = []
    for base, _, files in os.walk(root):
        if "🦀️.rs" in files and base.endswith("📸️snapshot") and (not folder or f"/{folder}/" in base + "/"):
            text = open(os.path.join(base, "🦀️.rs"), encoding="utf-8").read()
            if re.search(rf"impl store::ArtifactPack for {name} \{{", text):
                found.append(os.path.join(base, PROTOCOL))
    if len(found) != 1 or not os.path.isfile(found[0]):
        problems.append(f"{directory}: {len(found)} protocols for {type_label}")
        return None
    return found[0]


def definition_of(directory):
    return json.loads(read(f"{ART}/{directory}/📜️artifact-definition.json"))


def codec_capability(definition, schema):
    matches = [capability for capability in definition["runtime_capabilities"] if capability["category"] == "codec" and any(claim["namespace"] == "codec" and claim["value"] == schema for claim in capability["claims"])]
    if len(matches) != 1:
        problems.append(f"{definition['id']}: {len(matches)} codec capabilities claim {schema}")
        return None, None
    extension = next(claim["value"] for claim in matches[0]["claims"] if claim["namespace"] == "codec-extension").split(":")[-1]
    return matches[0]["id"], extension


def schema_value(directory, expression):
    constant = expression.rsplit("::", 1)[-1]
    for base, _, files in os.walk(os.path.join(TREE, ART, directory)):
        if "🦀️.rs" in files:
            found = re.search(rf'pub const {constant}: &str = "([^"]+)";', open(os.path.join(base, "🦀️.rs"), encoding="utf-8").read())
            if found:
                return found.group(1)
    problems.append(f"{directory}: no value for {constant}")
    return None


def rows():
    """🧾️ Every new linked codec row, resolved from the tree: schema, standard, runtime capability, extension, protocol, hash."""
    out = []
    for directory, artifact, codecs in ROOTS + [(PDF_DIR, "pdf", [{"fn": "native_codec", "snapshot": "PdfSnapshot", "mutation": "PdfMutation", "schema_expr": "STDIO_PDF17_DOCUMENT_SCHEMA", "type": "PdfSnapshot@7️⃣1.7", "standard": "1-7", "leaf": "native-document", "slug": "1.7"}])]:
        definition = definition_of(directory)
        for codec in codecs:
            schema = schema_value(directory, codec["schema_expr"])
            protocol = protocol_of(directory, codec["type"])
            capability, extension = codec_capability(definition, schema)
            if schema is None or protocol is None or capability is None:
                continue
            standard = f"{definition['id']}.standard.{codec['standard']}"
            if not any(item["id"] == standard for item in definition["standards"]):
                problems.append(f"{directory}: no standard {standard}")
            factory = f"stdio.native.{artifact}.v1" if codec["slug"] is None else f"stdio.native.{artifact}.{codec['slug']}.v1"
            out.append({
                "directory": directory, "artifact": artifact, "codec": codec, "schema": schema, "kind": definition["id"], "standard": standard,
                "codec_id": f"{standard}.codec.{codec['leaf']}.v1", "factory": factory, "capability": capability, "extension": extension,
                "hash": hashlib.sha256(open(protocol, "rb").read()).hexdigest(), "protocol": os.path.relpath(protocol, os.path.join(TREE, STDIO)),
            })
    return out
#endregion Rows


#region Roots
DEFINITION_OLD = "    semio_s_artifact_stdio_contract::definition_from_schema(ARTIFACT_DEFINITION_SCHEMA)\n}\n"
DEFINITION_NEW = (
    "    let factories = native_codecs();\n"
    "    let executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;\n"
    "    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)\n"
    "}\n"
)


def codec_fn(row):
    codec = row["codec"]
    return (
        f"fn {codec['fn']}() -> store::ArtifactCodec {{\n"
        f"    let mut codec = store::ArtifactCodec::of::<{codec['snapshot']}, {codec['mutation']}>({codec['schema_expr']});\n"
        f"    codec.extension = \"{row['extension']}\";\n"
        "    codec\n"
        "}\n"
    )


def factories_fn(root_rows):
    entries = ", ".join(f"semio_s_artifact_stdio_contract::NativeCodecFactory {{ id: \"{row['factory']}\", artifact: \"{row['artifact']}\", kind: artifact_kind, codec: {row['codec']['fn']} }}" for row in root_rows)
    body = f"    vec![{entries}]\n"
    if len(body) > 251:
        body = "    vec![\n" + "".join(f"        semio_s_artifact_stdio_contract::NativeCodecFactory {{ id: \"{row['factory']}\", artifact: \"{row['artifact']}\", kind: artifact_kind, codec: {row['codec']['fn']} }},\n" for row in root_rows) + "    ]\n"
    return "pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {\n" + body + "}\n"


def root(directory, root_rows):
    def edit(text):
        text = once(text, DEFINITION_OLD, DEFINITION_NEW, f"{directory}: definition() binds its native codecs")
        empty = "pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {\n    Vec::new()\n}\n"
        return once(text, empty, "".join(codec_fn(row) + "\n" for row in root_rows) + factories_fn(root_rows), f"{directory}: native_codecs()")
    return edit


def pdf_root(row):
    def edit(text):
        found = re.search(r"fn native_codec\(\) -> store::ArtifactCodec \{\n.*?\n\}\n", text, re.S)
        if not found:
            problems.append("pdf: no native_codec()")
            return text
        text = text[: found.start()] + codec_fn(row) + text[found.end() :]
        text = once(text, 'NativeCodecFactory { id: "stdio.native.pdf.v1", artifact: "pdf", kind: artifact_kind, codec: native_codec }', f'NativeCodecFactory {{ id: "{row["factory"]}", artifact: "pdf", kind: artifact_kind, codec: native_codec }}', "pdf: factory id")
        text = once(text, "        source_format: STDIO_PDF_DOCUMENT_SCHEMA.into(),\n", "        source_format: STDIO_PDF17_DOCUMENT_SCHEMA.into(),\n", "pdf: kind source format")
        return once(text, "        schema: STDIO_PDF_DOCUMENT_SCHEMA.into(),\n", "        schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(),\n", "pdf: kind schema")
    return edit
#endregion Roots


#region Definitions
def codec_entry(row):
    return (
        "    {\n"
        f'      "id": "{row["codec_id"]}",\n'
        '      "status": "implemented",\n'
        f'      "from": "{row["standard"]}.dialect.source",\n'
        f'      "to": "{row["standard"]}.dialect.source",\n'
        '      "executable_registration": true,\n'
        '      "native_factory": {\n'
        f'        "factory_id": "{row["factory"]}",\n'
        f'        "artifact_kind": "{row["kind"]}",\n'
        f'        "artifact_schema": "{row["schema"]}",\n'
        f'        "extension": "{row["extension"]}",\n'
        f'        "pack_schema_hash": "{row["hash"]}",\n'
        f'        "runtime_capability_id": "{row["capability"]}"\n'
        "      }\n"
        "    }"
    )


def definition(directory, root_rows):
    def edit(text):
        block = '  "codecs": [\n' + ",\n".join(codec_entry(row) for row in root_rows) + "\n  ],\n"
        if directory == PDF_DIR:
            found = re.search(r'  "codecs": \[\n.*?\n  \],\n', text, re.S)
            if not found:
                problems.append("pdf: no codecs block")
                return text
            text = text[: found.start()] + block + text[found.end() :]
        else:
            text = once(text, '  "codecs": [],\n', block, f"{directory}: codecs[]")
        json.loads(text)
        return text
    return edit
#endregion Definitions


#region Registry
def registry_row(row):
    return (
        "  {\n"
        f'    "artifact": "{row["artifact"]}",\n'
        f'    "factory_id": "{row["factory"]}",\n'
        f'    "descriptor_codec_id": "{row["codec_id"]}",\n'
        f'    "runtime_capability_id": "{row["capability"]}",\n'
        f'    "artifact_kind": "{row["kind"]}",\n'
        f'    "artifact_schema": "{row["schema"]}",\n'
        f'    "extension": "{row["extension"]}",\n'
        f'    "pack_schema_sha256": "{row["hash"]}",\n'
        f'    "protocol_path": "{row["protocol"]}"\n'
        "  }"
    )


def registry(new_rows):
    def edit(text):
        head, _, _ = text.partition('  "receipts":\n[\n')
        document = json.loads(text)
        existing = [receipt for receipt in document["receipts"] if receipt["factory_id"] != "stdio.native.pdf.v1"]
        rendered = {receipt["factory_id"]: registry_row({"artifact": receipt["artifact"], "factory": receipt["factory_id"], "codec_id": receipt["descriptor_codec_id"], "capability": receipt["runtime_capability_id"], "kind": receipt["artifact_kind"], "schema": receipt["artifact_schema"], "extension": receipt["extension"], "hash": receipt["pack_schema_sha256"], "protocol": receipt["protocol_path"]}) for receipt in existing}
        for row in new_rows:
            if row["factory"] in rendered:
                problems.append(f"registry: {row['factory']} already present")
            rendered[row["factory"]] = registry_row(row)
        original = head + '  "receipts":\n[\n' + ",\n".join(registry_row({"artifact": receipt["artifact"], "factory": receipt["factory_id"], "codec_id": receipt["descriptor_codec_id"], "capability": receipt["runtime_capability_id"], "kind": receipt["artifact_kind"], "schema": receipt["artifact_schema"], "extension": receipt["extension"], "hash": receipt["pack_schema_sha256"], "protocol": receipt["protocol_path"]}) for receipt in document["receipts"]) + "\n]\n}\n"
        if json.loads(original) != document:
            problems.append("registry: the projection's own rows do not re-render to the same receipts")
        after = head + '  "receipts":\n[\n' + ",\n".join(rendered[factory] for factory in sorted(rendered)) + "\n]\n}\n"
        if len(json.loads(after)["receipts"]) != 56:
            problems.append(f"registry: {len(json.loads(after)['receipts'])} receipts, expected 56")
        return after
    return edit


def registry_rs(text):
    text = once(
        text,
        "/// 🏭️ Native document codecs of the full catalog: every artifact whose documents the hub opens through a linked codec;\n/// the seven definition-only artifacts (binary, bmp, epw, gif, ifc, semio, wav) own none.\n#[cfg(feature = \"full-artifact-catalog\")]\nconst NATIVE_CODEC_FACTORY_COUNT: usize = 29;\n",
        "/// 🏭️ Native document codecs of the full catalog: one linked codec per `(kind, document schema)` an editor opens — the hub\n/// opens and creates exactly these (56 codecs over the 36 kinds: gif, ifc and semio carry one per schema).\n#[cfg(feature = \"full-artifact-catalog\")]\nconst NATIVE_CODEC_FACTORY_COUNT: usize = 56;\n\n/// 🗂️ The artifact kinds those codecs belong to, each once.\n#[cfg(feature = \"full-artifact-catalog\")]\nconst NATIVE_CODEC_KIND_COUNT: usize = 36;\n",
        "registry: codec counts",
    )
    text = once(
        text,
        "pub fn native_codec_artifact_kinds() -> Vec<semio_framework_plugin::ArtifactKindSpec> {\n    native_codec_factories().into_iter().map(|factory| (factory.kind)()).collect()\n}\n",
        "pub fn native_codec_artifact_kinds() -> Vec<semio_framework_plugin::ArtifactKindSpec> {\n    let mut kinds = BTreeMap::new();\n    for factory in native_codec_factories() {\n        let kind = (factory.kind)();\n        kinds.entry(kind.id.clone()).or_insert(kind);\n    }\n    kinds.into_values().collect()\n}\n",
        "registry: one kind per native codec kind",
    )
    text = once(text, "    if expected.len() != NATIVE_CODEC_FACTORY_COUNT || kinds.len() != expected.len() {\n", "    if expected.len() != NATIVE_CODEC_KIND_COUNT || kinds.len() != expected.len() {\n", "registry: kind count")
    return once(text, "/// 🔐️ Admits only the exact guest-committed 36-definition/29-codec semantic projection.\n", "/// 🔐️ Admits only the exact guest-committed 36-definition/56-codec semantic projection.\n", "registry: projection doc")


def stdio_provider_test(text):
    text = once(text, "    assert_eq!(receipts.len(), 29);\n    assert_eq!(receipts.iter().map(|receipt| receipt.factory_id.as_str()).collect::<BTreeSet<_>>().len(), 29);\n", "    assert_eq!(receipts.len(), 56);\n    assert_eq!(receipts.iter().map(|receipt| receipt.factory_id.as_str()).collect::<BTreeSet<_>>().len(), 56);\n", "stdio provider test: factory ids")
    text = once(text, ".collect::<BTreeSet<_>>().len(), 29);\n    assert_eq!(receipts.iter().map(|receipt| (receipt.artifact_kind.as_str(), receipt.schema.as_str()))", ".collect::<BTreeSet<_>>().len(), 56);\n    assert_eq!(receipts.iter().map(|receipt| (receipt.artifact_kind.as_str(), receipt.schema.as_str()))", "stdio provider test: codec ids")
    text = once(text, ".collect::<BTreeSet<_>>().len(), 29);\n    assert!(receipts.iter().all(", ".collect::<BTreeSet<_>>().len(), 56);\n    assert!(receipts.iter().all(", "stdio provider test: pairs")
    return once(text, '    assert_eq!(original["payload"]["codecs"].as_array().unwrap().len(), 29);\n', '    assert_eq!(original["payload"]["codecs"].as_array().unwrap().len(), 56);\n', "stdio provider test: catalog codecs")


def hub_provider(text):
    text = once(text, "pub const NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS: usize = 32;\nconst NATIVE_STDIO_PROVIDER_RECEIPTS: usize = 29;\n", "pub const NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS: usize = 59;\nconst NATIVE_STDIO_PROVIDER_RECEIPTS: usize = 56;\n", "hub provider: receipt pins")
    return text
#endregion Registry


#region Framework
CANDIDATES_OLD = (
    "    /// 🪪️ The apps of `program` that own `artifact_schema`: those whose registered type opens that document\n"
    "    /// schema — the primary key `store::ArtifactCodec` and the hub's trusted catalog use — and those whose\n"
    "    /// dialect names it as an ARTIFACT KIND, the identity build tooling with nothing but a compiled\n"
    "    /// descriptor asks by (it reads the schema back out of `codec.genesis` and asks by schema after that; a\n"
    "    /// kind `s.<plugin>.<artifact>` and a `DOCUMENT_SCHEMA` never collide). Pure: nothing is constructed.\n"
    "    pub(crate) fn artifact_codec_candidates<'a, PA: PluginApp>(program: &'a Plugin<PA>, artifact_schema: &'a str) -> impl Iterator<Item = &'a crate::app::AppDefinition> + 'a {\n"
    "        program.manifest.apps.iter().filter(move |definition| program.app_document_schema(&definition.id) == Some(artifact_schema) || definition.dialect.artifact_kind == artifact_schema)\n"
    "    }\n"
)
CANDIDATES_NEW = (
    "    /// 🪪️ The apps of `program` that own `artifact_schema`: those whose registered type opens that document\n"
    "    /// schema — the primary key `store::ArtifactCodec` and the hub's trusted catalog use — and those whose\n"
    "    /// dialect names it as an ARTIFACT KIND, the identity build tooling with nothing but a compiled\n"
    "    /// descriptor asks by (it reads the schema back out of `codec.genesis` and asks by schema after that; a\n"
    "    /// kind `s.<plugin>.<artifact>` and a `DOCUMENT_SCHEMA` never collide). Asked by a dialect coordinate\n"
    "    /// `<kind>@<standard>/<subset>` (`ArtifactDialect::to_coordinate`, the identity a hub's per-dialect creation entry passes),\n"
    "    /// exactly the surfaces of that dialect answer, so creation can name one standard of a kind whose standards share\n"
    "    /// one document schema (stdio dwg `ac1018`/`ac1024`, pdf `1.4`/`1.7`). Pure: nothing is constructed.\n"
    "    pub(crate) fn artifact_codec_candidates<'a, PA: PluginApp>(program: &'a Plugin<PA>, artifact_schema: &'a str) -> impl Iterator<Item = &'a crate::app::AppDefinition> + 'a {\n"
    "        let coordinate = semio_framework::ArtifactDialect::parse_coordinate(artifact_schema).ok();\n"
    "        program.manifest.apps.iter().filter(move |definition| match &coordinate {\n"
    "            Some(dialect) => definition.dialect == *dialect,\n"
    "            None => program.app_document_schema(&definition.id) == Some(artifact_schema) || definition.dialect.artifact_kind == artifact_schema,\n"
    "        })\n"
    "    }\n"
)
FIXTURE_ANCHOR = "    /// 🪪️ A codec call constructs no app:"
FIXTURE_LAW = '''    /// 🧭️ A dialect coordinate `<kind>@<standard>/<subset>` names exactly one creation surface, so a hub creation entry can pick
    /// one standard of a kind whose standards share one document schema (stdio dwg `ac1018`/`ac1024`, pdf `1.4`/`1.7`): with the
    /// fixture's `std2/*` apps re-pointed at `std1/*`'s schema, asking by the schema finds two whole-standard editors and is
    /// refused, while asking by either coordinate answers that surface's editor, and genesis by coordinate creates a document.
    #[semio_framework_async_macros::async_test]
    async fn a_dialect_coordinate_selects_exactly_one_creation_surface() {
        const SHARED: &str = "semio.testkit.w1c-fixture.std1-any/v1";
        let std2 = ArtifactDialect::from(STD2_ANY_DIALECT);
        let mut plugin = crate::app::Plugin::<FixtureApps>::new("testkit", "Testkit", "1.0.0");
        for (app, mut factory) in project_artifact_declarations(&[build_declaration()]).app_defs {
            if app.definition.dialect == std2 {
                factory.document_schema = SHARED;
            }
            plugin = plugin.register_app_factory(app, factory);
        }
        assert!(crate::plugin_runtime::artifact_codec_owner(&plugin, SHARED).is_err(), "two whole-standard editors share the schema");
        for dialect in [STD1_ANY_DIALECT, STD2_ANY_DIALECT] {
            let coordinate = ArtifactDialect::from(dialect).to_coordinate();
            let owner = crate::plugin_runtime::artifact_codec_owner(&plugin, &coordinate).expect("a coordinate names one surface");
            assert_eq!((owner.role, &owner.dialect), (AppRole::Editor, &ArtifactDialect::from(dialect)));
        }
        let runtime = crate::plugin_runtime::PluginRuntime::new();
        crate::plugin_runtime::install_plugin_bundle(&runtime, plugin);
        let document_id = format!("artifact-{}", "3".repeat(32));
        let pair = crate::plugin_runtime::plugin_artifact_genesis(&runtime, &std2.to_coordinate(), &document_id).await.expect("genesis by the std2 coordinate");
        assert!(!pair.pack.is_empty() && !pair.spr.is_empty(), "the std2 editor created its document");
    }

'''
#endregion Framework


#region Hub
PUBLISHER_OLD = '''  for (const [kind, schema] of pairs) {
    const row = linked.find((codec) => codec.artifactKind === kind);
    if (!row) unowned.add(kind);
    else if (row.artifactSchema !== schema) throw new Error(`linked codec ${kind} carries schema ${row.artifactSchema}, its descriptor declares ${schema}`);
  }
'''
PUBLISHER_NEW = '''  for (const [kind, schema] of pairs) {
    const rows = linked.filter((codec) => codec.artifactKind === kind);
    if (rows.length === 0) unowned.add(kind);
    else if (!rows.some((row) => row.artifactSchema === schema)) throw new Error(`linked codecs of ${kind} carry schemas ${rows.map((row) => row.artifactSchema).join(", ")}, its descriptor declares ${schema}`);
  }
'''
OWNERSHIP_REFUSAL_OLD = '"refusal": "linked codec s.stdio.csv carries schema stdio.csv, its descriptor declares stdio.tsv"'
OWNERSHIP_REFUSAL_NEW = '"refusal": "linked codecs of s.stdio.csv carry schemas stdio.csv, its descriptor declares stdio.tsv"'
OWNERSHIP_CASE_ANCHOR = '''    {
      "id": "schema-conflict-refuses",'''
OWNERSHIP_CASE = '''    {
      "id": "a-kind-with-several-linked-schemas-is-owned-by-its-declared-one",
      "declared": [["s.stdio.gif", "stdio.gif"]],
      "linked": [
        { "artifactKind": "s.stdio.gif", "artifactSchema": "stdio.gif.89a", "packSchemaHash": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" },
        { "artifactKind": "s.stdio.gif", "artifactSchema": "stdio.gif", "packSchemaHash": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb" }
      ],
      "unowned": []
    },
'''
OWNERSHIP_LAW_ANCHOR = '''  it("stdio's committed descriptor declares no kind its linked registry does not own", () => {'''
OWNERSHIP_LAW_NEW = '''  it("every stdio editor opens a (kind, schema) its linked registry owns, one creation surface per dialect", () => {
    const registry = JSON.parse(readFileSync(new URL("../../../✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json", import.meta.url), "utf8")) as { receipts: { artifact_kind: string; artifact_schema: string }[] };
    const owned = new Set(registry.receipts.map((row) => `${row.artifact_kind} ${row.artifact_schema}`));
    const packages = ["", "🧩️extensions/🖼️image/", "🧩️extensions/🎵️media/", "🧩️extensions/🛠️cad/", "🧩️extensions/🏠️bim/", "🧩️extensions/🔺️mesh/", "🧩️extensions/📘️pdf/", "🧩️extensions/💼️office/", "🧩️extensions/🧿️semio/", "🧩️extensions/🔢️binary/"];
    const editors = packages.flatMap((prefix) => {
      const descriptor = JSON.parse(readFileSync(new URL(`../../../✏️s/🔌️plugins/🗄️stdio/${prefix}🔣️.json`, import.meta.url), "utf8")) as { manifest: { apps: { role: string; dialect: { artifactKind: string; standard: string; subset: string }; io: { artifactSchema: string } }[] } };
      return descriptor.manifest.apps.filter((app) => app.role === "editor");
    });
    expect(editors.length).toBeGreaterThan(80);
    expect(editors.filter((app) => !owned.has(`${app.dialect.artifactKind} ${app.io.artifactSchema}`)).map((app) => `${app.dialect.artifactKind} ${app.io.artifactSchema}`)).toEqual([]);
    const coordinates = editors.map((app) => `${app.dialect.artifactKind}@${app.dialect.standard}/${app.dialect.subset}`);
    expect(new Set(coordinates).size).toBe(coordinates.length);
  });
'''
#endregion Hub


#region Law
LAW_ANCHOR = "/// 🏠️ LAW (describe side of hosting, p15):"
DESCRIBE_OLD = '''        .collect();
    (descriptor, codecs)
}
'''
DESCRIBE_NEW = '''        .collect();
    let creations = descriptor
        .manifest
        .apps
        .iter()
        .filter(|app| app.role == AppRole::Editor)
        .map(|app| {
            let coordinate = app.dialect.to_coordinate();
            let created = semio_framework_plugin::app::resolve_ready(semio_framework_plugin::plugin_runtime::plugin_artifact_genesis(&runtime, &coordinate, &format!("artifact-{}", "4".repeat(32))))
                .map_err(|fault| format!("{fault:?}"))
                .and_then(|pair| semio_framework_plugin::app::resolve_ready(semio_framework_os_kernel::os_spr::decode_history(&pair.spr, &Default::default())).map_err(|error| error.to_string()))
                .map(|history| (history.schema, history.edits.len()));
            (app.id.clone(), coordinate, created)
        })
        .collect();
    (descriptor, codecs, creations)
}
'''
NEW_LAW = r'''/// 🧪️ The environment variable naming the one package [`package_hub_probe`] assembles in its child process.
const HUB_PROBE_PACKAGE: &str = "SEMIO_STDIO_HUB_PROBE_PACKAGE";

/// 🌐️ LAW (g): every stdio kind with an editor is openable and creatable on the hub. Every stdio package, assembled ALONE in its
/// own process as its wasm guest is, has for each editor of each kind it opens exactly one linked owner row
/// (`📇️registry/📜️native-codec-factories.json`, the only codec rows the stdio package publishes; a hosted row binds it) for the
/// editor's `(kind, document schema)`, answers that row's pack-schema identity, and creates a zero-history document of that
/// schema when `codec.genesis` is asked by the editor's dialect coordinate — the hub's per-dialect creation entry. Measured before
/// (H14 census + LB2, 2026-09-29): 28 of 56 opened rows had no owner row (binary, bmp, epw, wav, gif ×2, ifc ×2, semio ×19,
/// pdf 1.7), pdf's row named `stdio.pdf`, which no app opens, and dwg's and pdf's two standards had no single creation surface.
#[test]
fn every_stdio_kind_with_an_editor_is_openable_and_creatable_on_the_hub() {
    let binary = std::env::current_exe().expect("the test binary");
    let failures = PACKAGE_IDS
        .into_iter()
        .filter_map(|id| {
            let run = std::process::Command::new(&binary).args(["package_hub_probe", "--exact", "--ignored", "--nocapture", "--test-threads", "1"]).env(HUB_PROBE_PACKAGE, id).output().expect("the hub probe runs");
            (!run.status.success()).then(|| format!("{id} alone is not openable and creatable on the hub:\n{}\n{}", String::from_utf8_lossy(&run.stdout), String::from_utf8_lossy(&run.stderr)))
        })
        .collect::<Vec<_>>();
    assert!(failures.is_empty(), "{} of {} packages fail:\n{}", failures.len(), PACKAGE_IDS.len(), failures.join("\n"));
}

/// 🔬️ The child half of LAW (g): assembles only the package [`HUB_PROBE_PACKAGE`] names and checks every editor of every kind it
/// opens against the linked owner rows, its own identity answers and its genesis by dialect coordinate.
#[test]
#[ignore = "the child process of every_stdio_kind_with_an_editor_is_openable_and_creatable_on_the_hub"]
fn package_hub_probe() {
    let id = std::env::var(HUB_PROBE_PACKAGE).expect("the parent law names one package");
    let package = shipped(PACKAGE_IDS.into_iter().find(|candidate| *candidate == id).expect("a stdio package id"));
    assert_eq!(package.descriptor.package_id, format!("semio:{id}"), "{id} assembles alone");
    let registry: serde_json::Value = serde_json::from_str(include_str!("../../📇️registry/📜️native-codec-factories.json")).expect("the linked codec registry");
    let mut owners = BTreeMap::<(String, String), Vec<String>>::new();
    for row in registry["receipts"].as_array().expect("linked codec receipts") {
        owners.entry((row["artifact_kind"].as_str().expect("kind").to_owned(), row["artifact_schema"].as_str().expect("schema").to_owned())).or_default().push(row["pack_schema_sha256"].as_str().expect("pinned hash").to_owned());
    }
    let opened = activated_kinds(&package.descriptor);
    let mut faults = Vec::new();
    let mut editors = 0;
    for app in package.descriptor.manifest.apps.iter().filter(|app| app.role == AppRole::Editor && opened.contains(&app.dialect.artifact_kind)) {
        editors += 1;
        let pair = (app.dialect.artifact_kind.clone(), app.io.artifact_schema.clone());
        match owners.get(&pair).map(Vec::as_slice) {
            Some([pinned]) => match package.codecs.iter().find(|(kind, schema, _)| (kind, schema) == (&pair.0, &pair.1)).map(|(_, _, answer)| answer) {
                Some(Ok(hash)) if hash.iter().map(|byte| format!("{byte:02x}")).collect::<String>() == *pinned => {}
                other => faults.push(format!("{}: its identity {other:?} differs from the owner row {pinned}", app.id)),
            },
            other => faults.push(format!("{}: {} {} has {} owner rows", app.id, pair.0, pair.1, other.map_or(0, <[String]>::len))),
        }
        match package.creations.iter().find(|(app_id, _, _)| *app_id == app.id).map(|(_, coordinate, created)| (coordinate, created)) {
            Some((_, Ok((schema, 0)))) if *schema == app.io.artifact_schema => {}
            Some((coordinate, created)) => faults.push(format!("{}: genesis by {coordinate} answered {created:?}", app.id)),
            None => faults.push(format!("{}: no genesis answer", app.id)),
        }
    }
    assert!(editors > 0, "{id} ships editors");
    assert!(faults.is_empty(), "{id} alone fails {} hub rows: {faults:#?}", faults.len());
}

'''


def law(text):
    text = once(text, "    codecs: Vec<(String, String, Result<[u8; 32], String>)>,\n}\n", "    codecs: Vec<(String, String, Result<[u8; 32], String>)>,\n    creations: Vec<(String, String, Result<(String, usize), String>)>,\n}\n", "law: ShippedPackage creations")
    text = once(text, "fn describe<PA: PluginApp>(bundle: Result<Plugin<PA>, PluginAssemblyError>) -> (PackageDescriptor, Vec<(String, String, Result<[u8; 32], String>)>) {\n", "fn describe<PA: PluginApp>(bundle: Result<Plugin<PA>, PluginAssemblyError>) -> (PackageDescriptor, Vec<(String, String, Result<[u8; 32], String>)>, Vec<(String, String, Result<(String, usize), String>)>) {\n", "law: describe signature")
    text = once(text, "/// open (`io.artifactSchema`): the rows a host mounting the component registers component codecs for.\n", "/// open (`io.artifactSchema`): the rows a host mounting the component registers component codecs for — and, as its `codec.genesis`\n/// export does, for the genesis document of every editor asked by the editor's dialect coordinate (the hub's creation entry).\n", "law: describe doc")
    text = once(text, DESCRIBE_OLD, DESCRIBE_NEW, "law: describe creations")
    text = once(text, "    let (descriptor, codecs) = described;\n    ShippedPackage { id, manifest, descriptor, codecs }\n", "    let (descriptor, codecs, creations) = described;\n    ShippedPackage { id, manifest, descriptor, codecs, creations }\n", "law: shipped keeps creations")
    return once(text, LAW_ANCHOR, NEW_LAW + LAW_ANCHOR, "law (g)")
#endregion Law


#region Pins
SURFACE_FIXTURE = f"{STDIO}/📇️registry/🧫️fixtures/📇️native-catalog-surface/🔣️.json"
REGISTRY_SCHEMA = f"{STDIO}/📇️registry/🧬️schema/🔣️.json"
REGISTRY_UNIT = f"{STDIO}/📇️registry/🧪️tests/🔬️unit/🦀️.rs"
FRONTIER_FIXTURE = "🌎️hub/🧫️fixtures/🧭️native-artifact-provider-frontier-v1/🔣️.json"
PROVIDER_FIXTURE = "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧫️fixtures/🪪️v1/🔣️.json"
BOOTSTRAP_FIXTURE = "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json"


def surface_fixture(text):
    return once(text, '  "codecCount": 29,\n', '  "kindCount": 36,\n', "surface fixture: native kind count")


def registry_schema(text):
    text = once(text, '        "definitionCount",\n        "codecCount",\n        "cases"\n', '        "definitionCount",\n        "kindCount",\n        "cases"\n', "registry schema: surface required")
    text = once(text, '        "codecCount": {\n          "const": 29\n        },\n', '        "kindCount": {\n          "const": 36\n        },\n', "registry schema: surface kind count")
    return once(text, '                  "schema",\n                  "name",\n                  "source-format"\n', '                  "schema",\n                  "label",\n                  "source-format"\n', "registry schema: surface label mutation (the fixture and its law say label since ArtifactKindSpec.label)")


def registry_unit(text):
    text = once(text, "    assert_eq!(ledger.declared, CapabilityCounts { codecs: 35, mutations: 3, inferences: 67 });\n    assert_eq!(ledger.registered, CapabilityCounts { codecs: 29, mutations: 3, inferences: 67 });\n    assert_eq!(ledger.implemented, CapabilityCounts { codecs: 29, mutations: 0, inferences: 0 });\n", "    assert_eq!(ledger.declared, CapabilityCounts { codecs: 62, mutations: 3, inferences: 67 });\n    assert_eq!(ledger.registered, CapabilityCounts { codecs: 56, mutations: 3, inferences: 67 });\n    assert_eq!(ledger.implemented, CapabilityCounts { codecs: 56, mutations: 0, inferences: 0 });\n", "registry unit: capability ledger")
    return text


def surface_test(text):
    return once(text, '    assert_eq!(expected.len(), fixture["codecCount"].as_u64().unwrap() as usize);\n', '    assert_eq!(expected.len(), fixture["kindCount"].as_u64().unwrap() as usize);\n', "stdio provider test: native kind count")


def publisher_pins(text):
    text = once(text, "    readonly receiptCount: 32;\n", "    readonly receiptCount: 59;\n", "publisher: frontier receipt type")
    text = once(text, "Number(fixture.production.receiptCount) !== 32", "Number(fixture.production.receiptCount) !== 59", "publisher: frontier receipt check")
    text = once(text, 'fixture.receiptCount !== 29 || fixture.hostileCases.length !== 13', 'fixture.receiptCount !== 56 || fixture.hostileCases.length !== 13', "publisher: provider fixture receipt count")
    text = once(text, '    ["semio:stdio", 29],\n', '    ["semio:stdio", 56],\n', "publisher: vcs profile linked stdio count")
    text = once(text, "packageCount: 2, codecCount: 31, openTargetCount: 1 }", "packageCount: 2, codecCount: 58, openTargetCount: 1 }", "publisher: bootstrap limits")
    text = once(text, 'codecs.stdio.length !== 29 || !unique(codecs.stdio)) throw new Error("stdio bootstrap closure is not exact 29");', 'codecs.stdio.length !== 56 || !unique(codecs.stdio)) throw new Error("stdio bootstrap closure is not exact 56");', "publisher: bootstrap stdio closure")
    text = once(text, "    profile.packages[1].codecCount !== 29 ||\n", "    profile.packages[1].codecCount !== 56 ||\n", "publisher: bootstrap stdio package")
    return once(text, "!Array.isArray(s.receipts) || s.receipts.length !== 29) return fail();", "!Array.isArray(s.receipts) || s.receipts.length !== 56) return fail();", "publisher: receipt source shape")


def frontier_fixture(text):
    return once(text, '    "receiptCount": 32,\n', '    "receiptCount": 59,\n', "frontier fixture: receipts")


def provider_fixture(text):
    count = text.count('"receiptCount": 29')
    if count != 1:
        problems.append(f"provider fixture: {count} receipt counts")
        return text
    return text.replace('"receiptCount": 29', '"receiptCount": 56')


def bootstrap_fixture(text):
    text = once(text, '        "codecCount": 29,\n', '        "codecCount": 56,\n', "bootstrap fixture: stdio package")
    return once(text, '    "codecCount": 31,\n', '    "codecCount": 58,\n', "bootstrap fixture: limits")
#endregion Pins


def plan():
    all_rows = rows()
    edits = {}
    for directory, _, _ in ROOTS:
        root_rows = [row for row in all_rows if row["directory"] == directory]
        edits[f"{ART}/{directory}/🦀️.rs"] = root(directory, root_rows)
        edits[f"{ART}/{directory}/📜️artifact-definition.json"] = definition(directory, root_rows)
    pdf_rows = [row for row in all_rows if row["directory"] == PDF_DIR]
    if len(pdf_rows) == 1:
        edits[f"{ART}/{PDF_DIR}/🦀️.rs"] = pdf_root(pdf_rows[0])
        edits[f"{ART}/{PDF_DIR}/📜️artifact-definition.json"] = definition(PDF_DIR, pdf_rows)
    edits[REGISTRY_JSON] = registry(all_rows)
    edits[REGISTRY_RS] = registry_rs
    edits[STDIO_PROVIDER_TEST] = lambda text: surface_test(stdio_provider_test(text))
    edits[SURFACE_FIXTURE] = surface_fixture
    edits[REGISTRY_SCHEMA] = registry_schema
    edits[REGISTRY_UNIT] = registry_unit
    edits[FRONTIER_FIXTURE] = frontier_fixture
    edits[PROVIDER_FIXTURE] = provider_fixture
    edits[BOOTSTRAP_FIXTURE] = bootstrap_fixture
    edits[HUB_PROVIDER] = hub_provider
    edits[PLUGIN] = lambda text: once(text, CANDIDATES_OLD, CANDIDATES_NEW, "plugin: coordinate candidates")
    edits[FIXTURE] = lambda text: once(text, FIXTURE_ANCHOR, FIXTURE_LAW + FIXTURE_ANCHOR, "fixture: coordinate law")
    edits[PUBLISHER] = lambda text: publisher_pins(once(text, PUBLISHER_OLD, PUBLISHER_NEW, "publisher: linked ownership over every schema of a kind"))
    edits[OWNERSHIP_FIXTURE] = lambda text: once(once(text, OWNERSHIP_REFUSAL_OLD, OWNERSHIP_REFUSAL_NEW, "ownership fixture: refusal text"), OWNERSHIP_CASE_ANCHOR, OWNERSHIP_CASE + OWNERSHIP_CASE_ANCHOR, "ownership fixture: multi-schema case")
    edits[OWNERSHIP_LAW] = lambda text: once(text, OWNERSHIP_LAW_ANCHOR, OWNERSHIP_LAW_NEW + OWNERSHIP_LAW_ANCHOR, "ownership law: every editor owned")
    edits[LAW] = law
    if len(all_rows) != 28:
        problems.append(f"{len(all_rows)} new linked rows, expected 28")
    return edits


def main():
    mode = next((flag for flag in ("--dry-run", "--write", "--revert") if flag in sys.argv), None)
    if mode is None:
        print(__doc__)
        sys.exit(2)
    if mode == "--revert":
        for root_dir, _, files in os.walk(BACKUP):
            for file in files:
                source = os.path.join(root_dir, file)
                path = os.path.relpath(source, BACKUP)
                shutil.copyfile(source, os.path.join(TREE, path))
                print("restored", path)
        return
    staged = {}
    for path, edit in plan().items():
        before = read(path)
        after = edit(before)
        if after == before:
            problems.append(f"{path}: unchanged")
        staged[path] = (before, after)
    for problem in problems:
        print("PROBLEM", problem)
    print(f"{len(staged)} files, {len(problems)} problems")
    if mode == "--write" and not problems:
        for path, (before, after) in staged.items():
            backup = os.path.join(BACKUP, path)
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            if not os.path.exists(backup):
                open(backup, "w", encoding="utf-8").write(before)
            open(os.path.join(TREE, path), "w", encoding="utf-8").write(after)
        print("written; backups under", BACKUP)
    sys.exit(1 if problems else 0)


if __name__ == "__main__":
    main()
