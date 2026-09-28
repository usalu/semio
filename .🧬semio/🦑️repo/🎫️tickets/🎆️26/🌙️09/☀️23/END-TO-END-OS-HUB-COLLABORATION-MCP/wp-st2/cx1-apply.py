#!/usr/bin/env python3
"""🧷️ CX1 prepared patch: stdio txt/tsv/html own hub-native document codecs (stdio registry, receipts projection, hub
fence 26 → 29 stdio codecs / 31 stdio+GIS / 32 linked provider set), the linked-ownership and census laws, and the Nx
registry targets' declared dist-wasm inputs.

Usage: cx1-apply.py [--dry-run | --write] [--root <tree>]   (default: --dry-run on the repository this ticket lives in)

Every hunk is anchored on text that must occur exactly `count` times; a hunk whose replacement is already present and
whose anchor is gone reports `applied` (idempotent re-run). New files are created from `payload/` and must not exist
with other content. Nothing is written unless every hunk and file of the whole set is clean."""
import argparse
import hashlib
import json
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
PAYLOAD = HERE / "cx1-payload"
REPO = next(parent for parent in HERE.parents if (parent / "Cargo.toml").exists() and (parent / ".git").exists())

STDIO = "✏️s/🔌️plugins/🗄️stdio"
ART = f"{STDIO}/🗿️artifacts"
REG = f"{STDIO}/📇️registry"
HUB = "🌎️hub"
HUB_SCRIPT = f"{HUB}/📦️packages/🦀️rust/📜️script.ts"
TRUSTED = f"{HUB}/🗿️artifact-authority/🔏️trusted-catalog"
PROVIDER = f"{HUB}/🗿️artifact-authority/📇️native-openable-provider"
PLUGIN_REG = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry"

BOOTSTRAP_GENERATION_OLD = "bda0b90f3ef9a8e253c4ee54c6e7ea2595b8377330d4044f7e9d6400abbead51"
BOOTSTRAP_GENERATION_NEW = (PAYLOAD / "bootstrap-generation.txt").read_text().strip() if (PAYLOAD / "bootstrap-generation.txt").exists() else None

EDITS = []
NEW_FILES = []
PROTOCOL_DIGESTS = []


def edit(path, old, new, count=1):
    EDITS.append((path, old, new, count))


def artifact(slug, directory, standard, schema_fn_path, dsl_const, snapshot, mutation, protocol, digest, kind_prefix):
    root = f"{ART}/{directory}"
    PROTOCOL_DIGESTS.append((f"{root}/{protocol}", digest))
    fq = "" if slug == "txt" else "semio_framework_plugin::"
    edit(
        f"{root}/🦀️.rs",
        f"""pub fn definition() -> Result<{fq}ArtifactDefinition, {fq}PluginAssemblyError> {{
    semio_s_artifact_stdio_contract::definition_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}}""",
        f"""pub fn definition() -> Result<{fq}ArtifactDefinition, {fq}PluginAssemblyError> {{
    let factories = native_codecs();
    let executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;
    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)
}}""",
    )
    edit(
        f"{root}/🦀️.rs",
        """pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    Vec::new()
}""",
        f"""fn native_codec() -> store::ArtifactCodec {{
    let mut codec = store::ArtifactCodec::of::<{snapshot}, {mutation}>({dsl_const});
    codec.extension = "{slug}";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("{protocol}"));
    codec
}}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {{
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory {{ id: "stdio.native.{slug}.v1", artifact: "{slug}", kind: artifact_kind, codec: native_codec }}]
}}""",
    )
    edit(
        f"{root}/🦀️.rs",
        f"""pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, {fq}PluginAssemblyError> {{
    semio_s_artifact_stdio_contract::definition_only_assembly("{slug}", definition()?)
}}""",
        f"""pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, {fq}PluginAssemblyError> {{
    semio_s_artifact_stdio_contract::runtime_assembly("{slug}", definition()?, declaration)
}}

/// 🧩️ The executable facets the hub-native document codec needs: the artifact schema, the declared representations
/// and the `stdio.{slug}` document codec the linked `stdio.native.{slug}.v1` receipt instantiates.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: {fq}ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, {fq}ArtifactDefinitionError> {{
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition).schema({schema_fn_path}()).formats(formats).document_codec_bare::<{snapshot}, {mutation}>({dsl_const}).try_build()
}}""",
    )
    edit(
        f"{root}/📜️artifact-definition.json",
        '  "codecs": [],\n',
        f"""  "codecs": [
    {{ "id": "{kind_prefix}.codec.native-document.v1", "status": "implemented", "from": "{kind_prefix}.dialect.source", "to": "{kind_prefix}.dialect.source", "executable_registration": true, "native_factory": {{ "factory_id": "stdio.native.{slug}.v1", "artifact_kind": "s.stdio.{slug}", "artifact_schema": "stdio.{slug}", "extension": "{slug}", "pack_schema_hash": "{digest}", "runtime_capability_id": "{kind_prefix}.codec.codec-stdio-{slug}-extension-{slug}.v1" }} }}
  ],
""",
    )
    codec_capability = f"""    {{
      "id": "{kind_prefix}.codec.codec-stdio-{slug}-extension-{slug}.v1",
      "category": "codec",
      "descriptor": "runtime-capability:codec:codec-extension:{len('stdio.' + slug)}:stdio.{slug}:{slug}|codec:stdio.{slug}",
      "claims": [
        {{
          "namespace": "codec",
          "value": "stdio.{slug}"
        }},
        {{
          "namespace": "codec-extension",
          "value": "{len('stdio.' + slug)}:stdio.{slug}:{slug}"
        }}
      ]
    }},"""
    schema_capability = f"""    {{
      "id": "s.stdio.{slug}.schema.schema-s-stdio-{slug}.v1",
      "category": "schema",
      "descriptor": "runtime-capability:schema:schema:s.stdio.{slug}",
      "claims": [
        {{
          "namespace": "schema",
          "value": "s.stdio.{slug}"
        }}
      ]
    }}"""
    if slug == "txt":
        representation = """    {
      "id": "s.stdio.txt.standard.utf-8.representation.mime-text-plain-extension-txt",
      "category": "representation",
      "descriptor": "runtime-capability:representation:mime:text/plain|extension:.txt",
      "claims": [
        {
          "namespace": "mime",
          "value": "text/plain"
        },
        {
          "namespace": "extension",
          "value": ".txt"
        }
      ]
    }"""
        edit(f"{root}/📜️artifact-definition.json", f'  "runtime_capabilities": [\n{representation}\n  ],\n', f'  "runtime_capabilities": [\n{codec_capability}\n{representation},\n{schema_capability}\n  ],\n')
    else:
        edit(f"{root}/📜️artifact-definition.json", '  "runtime_capabilities": [],\n', f'  "runtime_capabilities": [\n{codec_capability}\n{schema_capability}\n  ],\n')


artifact("txt", "🔤️txt", "utf-8", "standards::v_utf_8::subsets::any::schema::txt_artifact_schema_descriptor", "STDIO_TXT_DOCUMENT_SCHEMA", "TxtSnapshot", "TxtMutation", "🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio", "d416b7a0cf81eff01de894166b9c30e77106920a106bc8406807f9187b519536", "s.stdio.txt.standard.utf-8")
artifact("tsv", "📑️tsv", "iana", "standards::iana::subsets::any::schema::tsv_artifact_schema_descriptor", "STDIO_TSV_DOCUMENT_SCHEMA", "TsvSnapshot", "TsvMutation", "🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio", "8c65ab63508c1db68d9b9d67dd8482a604a71b5f8966fb4046967e5ec3a4f989", "s.stdio.tsv.standard.iana")
artifact("html", "🌐️html", "5", "standards::v5::subsets::any::schema::html_artifact_schema_descriptor", "STDIO_HTML_DOCUMENT_SCHEMA", "HtmlSnapshot", "HtmlMutation", "🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio", "563a275178326c5e528ca2c6a9c5c4b4da8b7e0c83af5d6d49b0ea5b8b0b3e0d", "s.stdio.html.standard.5")

edit(f"{ART}/🔤️txt/📦️packages/🦀️rust/Cargo.toml", "semio-framework-dispatch-macros = { workspace = true }\nsemio-framework-job", "semio-framework-dispatch-macros = { workspace = true }\nsemio-framework-hash = { workspace = true }\nsemio-framework-job")
edit(f"{ART}/📑️tsv/📦️packages/🦀️rust/Cargo.toml", "[dependencies]\nsemio-framework-job = { workspace = true }", "[dependencies]\nsemio-framework-hash = { workspace = true }\nsemio-framework-job = { workspace = true }")
edit(f"{ART}/🌐️html/📦️packages/🦀️rust/Cargo.toml", "semio-framework = { workspace = true }\nsemio-framework-job", "semio-framework = { workspace = true }\nsemio-framework-hash = { workspace = true }\nsemio-framework-job")

# 📇️ stdio registry: one named count instead of six literal 26s.
edit(
    f"{REG}/🦀️.rs",
    "fn native_codec_factories() -> Vec<NativeCodecFactory> {",
    """/// 🏭️ Native document codecs of the full catalog: every artifact whose documents the hub opens through a linked codec;
/// the seven definition-only artifacts (binary, bmp, epw, gif, ifc, semio, wav) own none.
#[cfg(feature = "full-artifact-catalog")]
const NATIVE_CODEC_FACTORY_COUNT: usize = 29;

fn native_codec_factories() -> Vec<NativeCodecFactory> {""",
)
edit(
    f"{REG}/🦀️.rs",
    """    if assemblies.len() != 36 || receipts.len() != 26 {
        return Err(failure("native catalog projection requires 36 definitions and 26 codecs"));""",
    """    if assemblies.len() != 36 || receipts.len() != NATIVE_CODEC_FACTORY_COUNT {
        return Err(failure(format!("native catalog projection requires 36 definitions and {NATIVE_CODEC_FACTORY_COUNT} codecs")));""",
)
edit(f"{REG}/🦀️.rs", "    if identities.len() != 36 || receipts.len() != 26 {", "    if identities.len() != 36 || receipts.len() != NATIVE_CODEC_FACTORY_COUNT {")
edit(f"{REG}/🦀️.rs", "    let mut codecs = Vec::with_capacity(26);", "    let mut codecs = Vec::with_capacity(NATIVE_CODEC_FACTORY_COUNT);")
edit(f"{REG}/🦀️.rs", "/// 🔐️ Admits only the exact guest-committed 36-definition/26-codec semantic projection.", "/// 🔐️ Admits only the exact guest-committed 36-definition/29-codec semantic projection.")
edit(f"{REG}/🦀️.rs", "    if expected.len() != 26 || kinds.len() != expected.len() {", "    if expected.len() != NATIVE_CODEC_FACTORY_COUNT || kinds.len() != expected.len() {")
edit(
    f"{REG}/🦀️.rs",
    "    if receipts.len() != 26 || factories.len() != 26 || factory_ids.len() != 26 || descriptor_ids.len() != 26 || receipt_keys.len() != 26 {",
    "    if [receipts.len(), factories.len(), factory_ids.len(), descriptor_ids.len(), receipt_keys.len()].iter().any(|count| *count != NATIVE_CODEC_FACTORY_COUNT) {",
)
edit(
    f"{REG}/🧪️tests/🔬️unit/🦀️.rs",
    """    assert_eq!(native_codec_factory_receipts().expect("native codec receipts").len(), 26);
    let ledger = capability_ledger().expect("capability ledger");
    assert_eq!(ledger.declared, CapabilityCounts { codecs: 32, mutations: 3, inferences: 67 });
    assert_eq!(ledger.registered, CapabilityCounts { codecs: 26, mutations: 3, inferences: 67 });
    assert_eq!(ledger.implemented, CapabilityCounts { codecs: 26, mutations: 0, inferences: 0 });""",
    """    assert_eq!(native_codec_factory_receipts().expect("native codec receipts").len(), NATIVE_CODEC_FACTORY_COUNT);
    let ledger = capability_ledger().expect("capability ledger");
    assert_eq!(ledger.declared, CapabilityCounts { codecs: 35, mutations: 3, inferences: 67 });
    assert_eq!(ledger.registered, CapabilityCounts { codecs: 29, mutations: 3, inferences: 67 });
    assert_eq!(ledger.implemented, CapabilityCounts { codecs: 29, mutations: 0, inferences: 0 });""",
)
edit(
    f"{REG}/🧬️schema/🔣️.json",
    """        "receipts": {
          "type": "array",
          "minItems": 26,
          "maxItems": 26,""",
    """        "receipts": {
          "type": "array",
          "minItems": 29,
          "maxItems": 29,""",
)
edit(f"{REG}/🧬️schema/🔣️.json", """        "codecCount": {
          "const": 26
        },""", """        "codecCount": {
          "const": 29
        },""")
edit(
    f"{REG}/🧬️schema/🔣️.json",
    """        "codecs": {
          "type": "array",
          "minItems": 26,
          "maxItems": 26,""",
    """        "codecs": {
          "type": "array",
          "minItems": 29,
          "maxItems": 29,""",
)
edit(f"{REG}/🧬️schema/🔣️.json", """      "title": "ClaimAuthority"
    }
  }
}""", """      "title": "ClaimAuthority"
    },
""" + (PAYLOAD / "native-text-codecs.schema-def.json").read_text().rstrip("\n") + """
  }
}""")
edit(f"{REG}/🧫️fixtures/📇️native-catalog-surface/🔣️.json", '"codecCount": 26,', '"codecCount": 29,')
NEW_FILES.append((f"{REG}/🧫️fixtures/📇️native-text-codecs/🔣️.json", "native-text-codecs.fixture.json"))


def receipt_row(slug, directory, kind_prefix, digest, protocol):
    return f"""  {{
    "artifact": "{slug}",
    "factory_id": "stdio.native.{slug}.v1",
    "descriptor_codec_id": "{kind_prefix}.codec.native-document.v1",
    "runtime_capability_id": "{kind_prefix}.codec.codec-stdio-{slug}-extension-{slug}.v1",
    "artifact_kind": "s.stdio.{slug}",
    "artifact_schema": "stdio.{slug}",
    "extension": "{slug}",
    "pack_schema_sha256": "{digest}",
    "protocol_path": "🗿️artifacts/{directory}/{protocol}"
  }},"""


GLTF_TAIL = '    "protocol_path": "🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio"\n  },'
TIFF_TAIL = '    "protocol_path": "🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio"\n  },'
edit(f"{REG}/📜️native-codec-factories.json", GLTF_TAIL, GLTF_TAIL + "\n" + receipt_row("html", "🌐️html", "s.stdio.html.standard.5", "563a275178326c5e528ca2c6a9c5c4b4da8b7e0c83af5d6d49b0ea5b8b0b3e0d", "🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio"))
edit(
    f"{REG}/📜️native-codec-factories.json",
    TIFF_TAIL,
    TIFF_TAIL
    + "\n"
    + receipt_row("tsv", "📑️tsv", "s.stdio.tsv.standard.iana", "8c65ab63508c1db68d9b9d67dd8482a604a71b5f8966fb4046967e5ec3a4f989", "🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio")
    + "\n"
    + receipt_row("txt", "🔤️txt", "s.stdio.txt.standard.utf-8", "d416b7a0cf81eff01de894166b9c30e77106920a106bc8406807f9187b519536", "🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio"),
)

# 🧪️ stdio plugin laws.
PROVIDER_TEST = f"{STDIO}/🧪️tests/📇️native-openable-provider/🦀️.rs"
edit(
    PROVIDER_TEST,
    """    assert_eq!(receipts.len(), 26);
    assert_eq!(receipts.iter().map(|receipt| receipt.factory_id.as_str()).collect::<BTreeSet<_>>().len(), 26);
    assert_eq!(receipts.iter().map(|receipt| receipt.descriptor_codec_id.as_str()).collect::<BTreeSet<_>>().len(), 26);
    assert_eq!(receipts.iter().map(|receipt| (receipt.artifact_kind.as_str(), receipt.schema.as_str())).collect::<BTreeSet<_>>().len(), 26);""",
    """    assert_eq!(receipts.len(), 29);
    assert_eq!(receipts.iter().map(|receipt| receipt.factory_id.as_str()).collect::<BTreeSet<_>>().len(), 29);
    assert_eq!(receipts.iter().map(|receipt| receipt.descriptor_codec_id.as_str()).collect::<BTreeSet<_>>().len(), 29);
    assert_eq!(receipts.iter().map(|receipt| (receipt.artifact_kind.as_str(), receipt.schema.as_str())).collect::<BTreeSet<_>>().len(), 29);""",
)
edit(PROVIDER_TEST, '    assert_eq!(original["payload"]["codecs"].as_array().unwrap().len(), 26);', '    assert_eq!(original["payload"]["codecs"].as_array().unwrap().len(), 29);')
PROJECTION_TAIL = """        assert_eq!(generated, committed, "stdio native codec projection is stale: run the stdio native-codec-projection verb");
    }
}
"""
edit(PROVIDER_TEST, PROJECTION_TAIL, PROJECTION_TAIL + "\n" + (PAYLOAD / "native-text-codecs.law.rs").read_text())
edit(f"{STDIO}/🧫️fixtures/🏠️home-io-surface/🔣️.json", '  "nativeCodecCount": 26,', '  "nativeCodecCount": 29,')
edit(f"{STDIO}/🧬️schema/🔣️.json", """        "nativeCodecCount": {
          "const": 26
        },""", """        "nativeCodecCount": {
          "const": 29
        },""")
edit(f"{STDIO}/📦️packages/🦀️rust/📜️script.ts", "  readonly nativeCodecCount: 26;", "  readonly nativeCodecCount: 29;")

# 🌎️ hub: linked provider, fence, fixtures, laws.
edit(f"{PROVIDER}/🦀️.rs", "pub const NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS: usize = 29;\nconst NATIVE_STDIO_PROVIDER_RECEIPTS: usize = 26;", "pub const NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS: usize = 32;\nconst NATIVE_STDIO_PROVIDER_RECEIPTS: usize = 29;")
edit(f"{PROVIDER}/🧫️fixtures/🪪️v1/🔣️.json", '  "receiptCount": 26,', '  "receiptCount": 29,')
edit(f"{HUB}/🧫️fixtures/🧭️native-artifact-provider-frontier-v1/🔣️.json", '"receiptCount": 29', '"receiptCount": 32')
edit(f"{TRUSTED}/🦀️.rs", "|| stdio.is_none_or(|package| package.native_codecs.len() != 26 || !package.dependencies.is_empty())", "|| stdio.is_none_or(|package| package.native_codecs.len() != 29 || !package.dependencies.is_empty())")
TRUSTED_TEST = f"{TRUSTED}/🧪️tests/🔬️unit/🦀️.rs"
edit(TRUSTED_TEST, "    let stdio_codecs = (0u8..26).map(", "    let stdio_codecs = (0u8..29).map(")
edit(
    TRUSTED_TEST,
    """fn local_stdio_gis_profile_is_exact_two_packages_twenty_eight_codecs_and_opens_every_package_target() {
    let bundle = local_stdio_gis_profile_bundle();
    let selected = validate_bundle(&bundle, "local-stdio-gis-open-v1").expect("closed stdio+GIS profile");
    assert_eq!(selected.package_indices.len(), 2);
    assert_eq!(selected.package_indices, vec![1, 0]);
    assert_eq!(bundle.packages.iter().map(|package| package.native_codecs.len()).sum::<usize>(), 28);""",
    """fn local_stdio_gis_profile_is_exact_two_packages_thirty_one_codecs_and_opens_every_package_target() {
    let bundle = local_stdio_gis_profile_bundle();
    let selected = validate_bundle(&bundle, "local-stdio-gis-open-v1").expect("closed stdio+GIS profile");
    assert_eq!(selected.package_indices.len(), 2);
    assert_eq!(selected.package_indices, vec![1, 0]);
    assert_eq!(bundle.packages.iter().map(|package| package.native_codecs.len()).sum::<usize>(), 31);""",
)
edit(TRUSTED_TEST, "        assert_eq!(schemas.len(), 28);", "        assert_eq!(schemas.len(), 31);")
edit(TRUSTED_TEST, "                assert_eq!(catalog.codec_count(), 28);", "                assert_eq!(catalog.codec_count(), 31);")
edit(
    TRUSTED_TEST,
    """/// 🗺️ LAW (census): every editor surface of every committed, isolated package descriptor opens at least one artifact
/// kind through [`descriptor_open_targets`], so every document kind with an editor is creatable and openable over the
/// hub; the only exceptions are [`EDITORS_WITHOUT_A_DOCUMENT`], and each of those must still exist.""",
    """/// 🔗️ The `(artifact kind, schema)` pairs each hub-LINKED package owns: the committed registries the hub's linked native
/// providers and the publisher's `linkedCodecRegistry` read (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`).
fn linked_codec_registries() -> std::collections::BTreeMap<&'static str, BTreeSet<(String, String)>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../✏️s/🔌️plugins");
    let pairs = |path: &str, kind: &str, schema: &str| -> BTreeSet<(String, String)> {
        let registry: serde_json::Value = serde_json::from_slice(&std::fs::read(root.join(path)).expect("committed linked codec registry")).expect("linked codec registry JSON");
        registry["receipts"].as_array().expect("linked codec receipts").iter().map(|row| (row[kind].as_str().expect("receipt kind").to_owned(), row[schema].as_str().expect("receipt schema").to_owned())).collect()
    };
    std::collections::BTreeMap::from([("stdio", pairs("🗄️stdio/📇️registry/📜️native-codec-factories.json", "artifact_kind", "artifact_schema")), ("gis", pairs("🌍️gis/📇️native-codecs/🔣️.json", "kind", "schema"))])
}

/// 🗺️ LAW (census): every editor surface of every committed, isolated package descriptor opens at least one artifact
/// kind through [`descriptor_open_targets`], and every kind a hub-LINKED package opens binds one of its linked native
/// codecs ([`linked_codec_registries`]), so every document kind with an editor is creatable and openable over the hub;
/// the only exceptions are [`EDITORS_WITHOUT_A_DOCUMENT`], and each of those must still exist.""",
)
edit(
    TRUSTED_TEST,
    """    let (mut editors, mut unopened, mut shells) = (0usize, Vec::new(), std::collections::BTreeSet::new());""",
    """    let linked = linked_codec_registries();
    let (mut editors, mut unopened, mut unlinked, mut shells) = (0usize, Vec::new(), Vec::new(), std::collections::BTreeSet::new());""",
)
edit(
    TRUSTED_TEST,
    """        let opened = descriptor_open_targets(&descriptor).into_iter().map(|target| target.surface_id).collect::<std::collections::BTreeSet<_>>();""",
    """        let targets = descriptor_open_targets(&descriptor);
        if let Some(owned) = linked.get(descriptor.manifest.plugin_id.as_str()) {
            unlinked.extend(targets.iter().filter(|target| !owned.contains(&(target.artifact_kind.clone(), target.artifact_schema.clone()))).map(|target| format!("{} {} ({})", target.surface_id, target.artifact_kind, path.display())));
        }
        let opened = targets.into_iter().map(|target| target.surface_id).collect::<std::collections::BTreeSet<_>>();""",
)
edit(
    TRUSTED_TEST,
    """    assert!(unopened.is_empty(), "{} of {editors} editor surfaces open no kind: {unopened:#?}", unopened.len());""",
    """    assert!(unopened.is_empty(), "{} of {editors} editor surfaces open no kind: {unopened:#?}", unopened.len());
    assert!(unlinked.is_empty(), "{} open targets of linked packages bind no linked native codec: {unlinked:#?}", unlinked.len());""",
)
BIN_UNIT = f"{HUB}/🧪️tests/🔬️bin-unit/🦀️.rs"
edit(BIN_UNIT, "    assert_eq!(relocated.codec_count(), 28);", "    assert_eq!(relocated.codec_count(), 31);")
edit(BIN_UNIT, "        assert_eq!(configured.catalog.codec_count(), 26);", "        assert_eq!(configured.catalog.codec_count(), 29);")

edit(HUB_SCRIPT, "fixture.receiptCount !== 26 || fixture.hostileCases.length !== 13", "fixture.receiptCount !== 29 || fixture.hostileCases.length !== 13")
edit(HUB_SCRIPT, '["semio:stdio", 26],', '["semio:stdio", 29],')
edit(HUB_SCRIPT, "readonly receiptCount: 29;", "readonly receiptCount: 32;")
edit(HUB_SCRIPT, "Number(fixture.production.receiptCount) !== 29", "Number(fixture.production.receiptCount) !== 32")
edit(
    HUB_SCRIPT,
    '[...linked.values()].reduce((sum, count) => sum + count, 0) !== 29 || !provider.includes("pub const NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS: usize = 29;")',
    '[...linked.values()].reduce((sum, count) => sum + count, 0) !== 32 || !provider.includes("pub const NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS: usize = 32;")',
)
edit(HUB_SCRIPT, "linked-receipts=29", "linked-receipts=32")
edit(HUB_SCRIPT, 'providerSource.includes("NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS: usize = 29")', 'providerSource.includes("NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS: usize = 32")')
edit(HUB_SCRIPT, '"local_stdio_gis_profile_is_exact_two_packages_twenty_eight_codecs_and_one_map_editor_and_viewer",', '"local_stdio_gis_profile_is_exact_two_packages_thirty_one_codecs_and_opens_every_package_target",', count=2)
edit(HUB_SCRIPT, "packageCount: 2, codecCount: 28, openTargetCount: 1 }", "packageCount: 2, codecCount: 31, openTargetCount: 1 }")
edit(HUB_SCRIPT, 'codecs.stdio.length !== 26 || !unique(codecs.stdio)) throw new Error("stdio bootstrap closure is not exact 26");', 'codecs.stdio.length !== 29 || !unique(codecs.stdio)) throw new Error("stdio bootstrap closure is not exact 29");')
edit(HUB_SCRIPT, "profile.packages[1].codecCount !== 26 ||", "profile.packages[1].codecCount !== 29 ||")
edit(HUB_SCRIPT, "s.receipts.length !== 26) return fail();", "s.receipts.length !== 29) return fail();")
edit(
    HUB_SCRIPT,
    """(`validate_bundle`: "trusted document-open target is bound to no native codec of its own package"), so a kind the
 * descriptor declares without a linked row (Stdio's definition-only `s.stdio.txt`/`tsv`/`html`, whose editors declare
 * the kind they edit) is unowned: declared, never a hub open target.""",
    """(`validate_bundle`: "trusted document-open target is bound to no native codec of its own package"), so a kind the
 * descriptor declares without a linked row (an editor declaring a kind its package links no codec for) is unowned:
 * declared, never a hub open target; Stdio and GIS link a codec for every kind they declare.""",
)
BOOTSTRAP = f"{TRUSTED}/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json"
edit(BOOTSTRAP, '        "codecCount": 26,', '        "codecCount": 29,')
edit(BOOTSTRAP, '    "codecCount": 28,', '    "codecCount": 31,')
if BOOTSTRAP_GENERATION_NEW:
    edit(BOOTSTRAP, BOOTSTRAP_GENERATION_OLD, BOOTSTRAP_GENERATION_NEW, count=4)
LINKED = f"{TRUSTED}/🧫️fixtures/⛓️linked-codec-ownership/🔣️.json"
edit(
    LINKED,
    """      "id": "definition-only-kinds-are-unowned",
      "declared": [["s.stdio.csv", "stdio.csv"], ["s.stdio.txt", "stdio.txt"], ["s.stdio.html", "stdio.html"]],
      "linked": [{ "artifactKind": "s.stdio.csv", "artifactSchema": "stdio.csv", "packSchemaHash": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" }],
      "unowned": ["s.stdio.html", "s.stdio.txt"]""",
    """      "id": "definition-only-kinds-are-unowned",
      "declared": [["s.fixture.linked", "fixture.linked"], ["s.fixture.definition-a", "fixture.definition-a"], ["s.fixture.definition-b", "fixture.definition-b"]],
      "linked": [{ "artifactKind": "s.fixture.linked", "artifactSchema": "fixture.linked", "packSchemaHash": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" }],
      "unowned": ["s.fixture.definition-a", "s.fixture.definition-b"]""",
)
edit(
    f"{HUB}/🧪️tests/⛓️linked-codec-ownership/🟦️.ts",
    """  for (const row of fixture.cases) {
    it(row.id, () => {
      const run = () => [...classify(row.declared, row.linked)].sort();
      if (row.refusal) expect(run).toThrow(row.refusal);
      else expect(run()).toEqual(row.unowned);
    });
  }
});""",
    """  for (const row of fixture.cases) {
    it(row.id, () => {
      const run = () => [...classify(row.declared, row.linked)].sort();
      if (row.refusal) expect(run).toThrow(row.refusal);
      else expect(run()).toEqual(row.unowned);
    });
  }
  it("stdio's committed descriptor declares no kind its linked registry does not own", () => {
    const registry = JSON.parse(readFileSync(new URL("../../../✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json", import.meta.url), "utf8")) as { receipts: { artifact_kind: string; artifact_schema: string; pack_schema_sha256: string }[] };
    const descriptor = JSON.parse(readFileSync(new URL("../../../✏️s/🔌️plugins/🗄️stdio/🔣️.json", import.meta.url), "utf8")) as { manifest: { artifactKinds: { id: string; schema: string }[]; apps: { artifactKinds?: { id: string; schema: string }[] }[] } };
    const linked = registry.receipts.map((row) => ({ artifactKind: row.artifact_kind, artifactSchema: row.artifact_schema, packSchemaHash: row.pack_schema_sha256 }));
    const declared = [...descriptor.manifest.artifactKinds, ...descriptor.manifest.apps.flatMap((app) => app.artifactKinds ?? [])].map((kind) => [kind.id, kind.schema] as const);
    expect(declared.some(([kind]) => kind === "s.stdio.txt") && declared.some(([kind]) => kind === "s.stdio.tsv") && declared.some(([kind]) => kind === "s.stdio.html")).toBe(true);
    expect([...classify(declared, linked)]).toEqual([]);
  });
});""",
)

# 🔌️ framework plugin registry: trusted stdio catalog + native catalog selection.
edit(f"{PLUGIN_REG}/✅️trusted-stdio-catalog/🟦️.ts", "  if (codecs.length !== 26) throw new Error(`stdio native codec catalog must contain 26 receipts, found ${codecs.length}`);", "  if (codecs.length !== 29) throw new Error(`stdio native codec catalog must contain 29 receipts, found ${codecs.length}`);")
edit(f"{PLUGIN_REG}/🧪️tests/✅️trusted-stdio-catalog/🟦️.ts", 'it("matches the language-neutral fixture schema and publishes 26 first-party codecs", () => {', 'it("matches the language-neutral fixture schema and publishes 29 first-party codecs", () => {')
edit(f"{PLUGIN_REG}/🧫️fixtures/🧬️trusted-stdio-catalog/🔣️.json", '  "expectedCodecCount": 26,', '  "expectedCodecCount": 29,')
edit(f"{PLUGIN_REG}/🧬️schema/🧬️trusted-stdio-catalog/🔣️.json", '    "expectedCodecCount": { "const": 26 },', '    "expectedCodecCount": { "const": 29 },')
edit(
    f"{PLUGIN_REG}/🧬️schema/🧬️trusted-stdio-catalog/🔣️.json",
    """        "nativeCodecs": {
          "type": "array",
          "minItems": 26,
          "maxItems": 26,""",
    """        "nativeCodecs": {
          "type": "array",
          "minItems": 29,
          "maxItems": 29,""",
)
SELECTION = f"{PLUGIN_REG}/🧫️fixtures/📦️native-catalog-selection/🔣️.json"
edit(SELECTION, '      "receiptCount": 26\n', '      "receiptCount": 29\n', count=2)
edit(SELECTION, '      "expectedReceiptCount": 29\n', '      "expectedReceiptCount": 32\n')
edit(SELECTION, '      "expectedReceiptCount": 28\n', '      "expectedReceiptCount": 31\n')
edit(SELECTION, '      "expectedReceiptCount": 27\n', '      "expectedReceiptCount": 30\n')
edit(SELECTION, '      "expectedReceiptCount": 26\n', '      "expectedReceiptCount": 29\n')

NX_EDITS = PAYLOAD / "nx-edits.py"
if NX_EDITS.exists():
    exec(compile(NX_EDITS.read_text(), str(NX_EDITS), "exec"), {"edit": edit, "NEW_FILES": NEW_FILES, "PAYLOAD": PAYLOAD})


def main():
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--dry-run", action="store_true")
    mode.add_argument("--write", action="store_true")
    parser.add_argument("--root", default=str(REPO))
    args = parser.parse_args()
    root = pathlib.Path(args.root)
    texts = {}
    problems = []
    report = []
    for path, digest in PROTOCOL_DIGESTS:
        actual = hashlib.sha256((root / path).read_bytes()).hexdigest() if (root / path).exists() else None
        if actual != digest:
            problems.append(f"snapshot protocol {path} hashes to {actual}, the pinned codec digest is {digest}: recompute it")
    for path, old, new, count in EDITS:
        file = root / path
        if not file.exists():
            problems.append(f"missing file {path}")
            continue
        text = texts.setdefault(path, file.read_text(encoding="utf-8"))
        found = text.count(old)
        if found == count:
            texts[path] = text.replace(old, new)
            report.append(f"hunk   {path}: {old.splitlines()[0][:90]!r}")
        elif found == 0 and new in text:
            report.append(f"applied {path}: {new.splitlines()[0][:90]!r}")
        else:
            problems.append(f"anchor x{found} (want {count}) in {path}: {old.splitlines()[0][:120]!r}")
    for path, payload in NEW_FILES:
        file = root / path
        content = (PAYLOAD / payload).read_text(encoding="utf-8")
        if file.exists() and file.read_text(encoding="utf-8") != content:
            problems.append(f"new file exists with other content: {path}")
        else:
            report.append(f"{'same  ' if file.exists() else 'create'} {path}")
    for path, text in texts.items():
        if path.endswith(".json"):
            try:
                json.loads(text)
            except json.JSONDecodeError as error:
                problems.append(f"result is not JSON: {path}: {error}")
    for line in report:
        print(line)
    files = sorted(set(texts) | {path for path, _ in NEW_FILES})
    print(f"cx1-apply: {len(EDITS)} hunks, {len(NEW_FILES)} new files, {len(files)} files, {len(problems)} problems, root={root}")
    for problem in problems:
        print(f"PROBLEM {problem}")
    if problems:
        sys.exit(1)
    if not args.write:
        print("dry run: nothing written")
        return
    for path, text in texts.items():
        (root / path).write_text(text, encoding="utf-8")
    for path, payload in NEW_FILES:
        file = root / path
        file.parent.mkdir(parents=True, exist_ok=True)
        file.write_text((PAYLOAD / payload).read_text(encoding="utf-8"), encoding="utf-8")
    print(f"written: {len(files)} files")
    for path in files:
        print(f"  {path}")


main()
