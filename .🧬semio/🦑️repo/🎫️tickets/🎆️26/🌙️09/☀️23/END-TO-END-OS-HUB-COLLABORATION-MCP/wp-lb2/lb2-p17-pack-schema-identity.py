#!/usr/bin/env python3
"""🧬️ LB2 p17 (T7b, stdio + framework store/plugin): ONE pack-schema identity per stdio kind — its schema-first binary protocol.

G12 (live MCP coverage, 2026-09-29 18:2x): every stdio kind it opened (md, txt, …) failed `codec.pack-schema-hash` with
"artifact codec schema has no structural record specification". Cause, measured on the tree:
- the guest's codec table answers `A::Snapshot::record_spec()` hashed — `None` for 54 of the 57 hand-written stdio
  `ArtifactPack` impls (only txt/dwg/mp4 override it), so the guest refuses;
- the hub-LINKED native codecs (29 factories) never used `record_spec` at all: each overrides
  `codec.pack_schema_hash = sha256(💾️binary/📡️.protocol.semio)` by hand, and `📜️artifact-definition.json` declares exactly
  that value. So even txt (TC4's `record_spec` override) answers `schema_hash(__dsl_spec)` in the guest but the protocol
  digest natively — two identities for one kind.

Fix (schema-first, one source of truth):
- framework store: `ArtifactPack::pack_schema_hash()` — the kind's pack-schema identity, default `schema_hash(record_spec)`
  (every derive-based kind unchanged); `store::protocol_pack_schema_hash(protocol)` names the protocol-declared identity;
  `ArtifactCodec::of` and `test_support::assert_pack_schema_identity` read `P::pack_schema_hash()`.
- plugin SDK: the codec table's `pack_schema_hash` reads `A::Snapshot::pack_schema_hash()` — guest and native codec now
  compute one function of the type.
- stdio: every snapshot's `ArtifactPack` declares `pack_schema_hash()` = the SHA-256 of its own
  `📸️snapshot/💾️binary/📡️.protocol.semio` (57 impls, all families); txt's hash-only `record_spec` override goes; the 29 native
  codec factories drop their hand-written digest (the same bytes now come from `ArtifactCodec::of`; each removed path is
  verified to BE the snapshot's protocol, so every linked value is unchanged); the 26 artifact crates whose only
  `semio_framework_hash` use was that digest drop the dependency (+ their `Cargo.lock` edges).
- law (f) `shipped_fleet`: every stdio package, assembled alone in its own process, answers `codec.pack-schema-hash` for
  every kind it opens with a nonzero hash, equal to the linked native codec receipt where the hub links one; with
  `SEMIO_STDIO_CODEC_HASH_OUT=<dir>` each child writes its answers for the oracle `lb2-p17-pack-schema-oracle.py`
  (hashlib: every answer is the SHA-256 of exactly its snapshot's committed protocol).

usage: python3 lb2-p17-pack-schema-identity.py --dry-run | --write | --revert [--root <tree>]
Backups (byte-exact, per root) under `.🧬semio/🌐hub/s14-lb2-backup/p17/<root-hash>/`.
"""
import hashlib, os, re, shutil, subprocess, sys

TREE = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
BACKUP = f"/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-backup/p17/{hashlib.sha256(TREE.encode()).hexdigest()[:12]}"
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
LAW = "✏️s/🔌️plugins/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs"
TXT = f"{ART}/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"
PROTOCOL = "💾️binary/📡️.protocol.semio"

problems = []


def once(text, old, new, label):
    count = text.count(old)
    if count != 1:
        problems.append(f"{label}: expected 1 anchor, found {count}")
        return text
    return text.replace(old, new)


#region Framework
def store(text):
    text = once(
        text,
        "    fn record_spec() -> Option<crate::os_dsl::RecordSpec> {\n        None\n    }\n}\n",
        "    fn record_spec() -> Option<crate::os_dsl::RecordSpec> {\n        None\n    }\n"
        "\n"
        "    /// 🪪️ This document kind's pack-schema identity — the 32 bytes `ArtifactCodec::pack_schema_hash`, the guest's\n"
        "    /// `codec.pack-schema-hash` and a hub's trusted catalog pin the kind by. Default: `schema_hash` over\n"
        "    /// [`Self::record_spec`] (every derive-based kind). A kind whose pack is declared schema-first by a binary protocol\n"
        "    /// answers [`protocol_pack_schema_hash`] of that protocol instead, so its native codec and its guest can never\n"
        "    /// disagree.\n"
        "    fn pack_schema_hash() -> Option<[u8; 32]> {\n"
        "        Self::record_spec().map(|spec| crate::os_pack::schema_hash(&spec))\n"
        "    }\n"
        "}\n"
        "\n"
        "/// 📡️ The pack-schema identity of a kind whose pack is declared by a schema-first binary protocol\n"
        "/// (`📸️snapshot/💾️binary/📡️.protocol.semio`): the SHA-256 of the protocol's bytes.\n"
        "pub fn protocol_pack_schema_hash(protocol: &[u8]) -> [u8; 32] {\n"
        "    semio_framework_hash::Sha256::digest(protocol)\n"
        "}\n",
        "store: ArtifactPack::pack_schema_hash",
    )
    text = once(
        text,
        "    /// 🧬️ This document kind's structural field spec, for `ArtifactCodec::pack_schema_hash`\n",
        "    /// 🧬️ This document kind's structural field spec, hashed by the default [`Self::pack_schema_hash`]\n",
        "store: record_spec doc",
    )
    text = once(
        text,
        "    /// 🧬️ W5.7: a structural fingerprint of this document kind's field shape —\n"
        "    /// `crate::os_pack::schema_hash(&spec)` over `P::record_spec()`, or `[0u8; 32]` when `P` has no\n"
        "    /// `RecordSpec` (hand-written `ArtifactPack` impls, see that trait method's doc). Hub actors\n",
        "    /// 🧬️ W5.7: this document kind's pack-schema identity — `P::pack_schema_hash()` (see that trait method), or\n"
        "    /// `[0u8; 32]` when `P` declares none. Hub actors\n",
        "store: ArtifactCodec field doc",
    )
    text = once(
        text,
        "            pack_schema_hash: match P::record_spec() {\n                Some(spec) => crate::os_pack::schema_hash(&spec),\n                None => [0u8; 32],\n            },\n",
        "            pack_schema_hash: P::pack_schema_hash().unwrap_or([0u8; 32]),\n",
        "store: ArtifactCodec::of",
    )
    return once(
        text,
        "    /// 🧬️ Asserts a document kind has a structural pack-schema identity: `record_spec()` is\n"
        "    /// declared, its `schema_hash` is nonzero, and `snapshot` round-trips exactly through the pack.\n"
        "    pub fn assert_pack_schema_identity<P>(snapshot: &P) -> [u8; 32]\n"
        "    where\n"
        "        P: ArtifactPack + PartialEq + std::fmt::Debug,\n"
        "    {\n"
        "        let spec = P::record_spec().unwrap_or_else(|| panic!(\"{} declares no pack record spec\", std::any::type_name::<P>()));\n"
        "        let hash = crate::os_pack::schema_hash(&spec);\n",
        "    /// 🧬️ Asserts a document kind has a pack-schema identity: `pack_schema_hash()` is declared and\n"
        "    /// nonzero, and `snapshot` round-trips exactly through the pack.\n"
        "    pub fn assert_pack_schema_identity<P>(snapshot: &P) -> [u8; 32]\n"
        "    where\n"
        "        P: ArtifactPack + PartialEq + std::fmt::Debug,\n"
        "    {\n"
        "        let hash = P::pack_schema_hash().unwrap_or_else(|| panic!(\"{} declares no pack schema identity\", std::any::type_name::<P>()));\n",
        "store: assert_pack_schema_identity",
    )


CODEC_OLD = '    /// 🧬️ Resolves the `codec` answers of the ONE app of the installed bundle that owns `artifact_schema`. This is\n    /// the guest half of `world actor`\'s `codec` interface: a host that links no Rust codec for this package selects\n    /// a document kind by the same `schema` string `store::ArtifactCodec` is keyed by, and the bundle answers from its\n    /// own registered apps. An editor is preferred over a viewer because only an editor\'s snapshot is the kind\'s\n    /// creation authority; an ambiguous schema is refused rather than resolved by order. Nothing is constructed: the\n    /// answers are the functions of the owner\'s type its registration recorded ([`crate::app::ArtifactCodecTableV1`]).\n    fn plugin_artifact_codec<PA: PluginApp>(runtime: &PluginRuntime<PA>, artifact_schema: &str) -> Result<crate::app::ArtifactCodecTableV1, Fault> {\n        if artifact_schema.is_empty() || artifact_schema.len() > 256 || artifact_schema.chars().any(char::is_control) {\n            return Err(plugin_internal_fault("artifact codec schema identity is empty, oversized or control-bearing"));\n        }\n        if let Some(fault) = runtime.plugin_assembly_error.try_borrow().map_err(|_| plugin_internal_fault("plugin assembly authority busy"))?.clone() {\n            return Err(fault);\n        }\n        let program = runtime.plugin.try_borrow().map_err(|_| plugin_internal_fault("plugin factory authority busy"))?;\n        let program = program.as_ref().ok_or_else(|| plugin_internal_fault("plugin not initialized"))?;\n        let definition = artifact_codec_owner(program, artifact_schema)?;\n        program.app_codec(&definition.id).ok_or_else(|| plugin_internal_fault("artifact codec owner has no registered codec"))\n    }\n\n    /// \U0001faaa️ The apps of `program` that own `artifact_schema`: those whose registered type opens that document\n    /// schema — the primary key `store::ArtifactCodec` and the hub\'s trusted catalog use — and those whose\n    /// dialect names it as an ARTIFACT KIND, the identity build tooling with nothing but a compiled\n    /// descriptor asks by (it reads the schema back out of `codec.genesis` and asks by schema after that; a\n    /// kind `s.<plugin>.<artifact>` and a `DOCUMENT_SCHEMA` never collide). Pure: nothing is constructed.\n    pub(crate) fn artifact_codec_candidates<\'a, PA: PluginApp>(program: &\'a Plugin<PA>, artifact_schema: &\'a str) -> impl Iterator<Item = &\'a crate::app::AppDefinition> + \'a {\n        program.manifest.apps.iter().filter(move |definition| program.app_document_schema(&definition.id) == Some(artifact_schema) || definition.dialect.artifact_kind == artifact_schema)\n    }\n\n    /// 🎯️ The ONE app whose codec answers `artifact_schema`: the owning editor — only an editor\'s snapshot is the\n    /// kind\'s creation authority — else the owning viewer. Two owners of the same role are refused rather than\n    /// resolved by order, and no owner is refused; all of it decided on the declarations.\n    pub(crate) fn artifact_codec_owner<\'a, PA: PluginApp>(program: &\'a Plugin<PA>, artifact_schema: &\'a str) -> Result<&\'a crate::app::AppDefinition, Fault> {\n        let (mut editor, mut viewer) = (None, None);\n        for definition in artifact_codec_candidates(program, artifact_schema) {\n            let slot = if definition.role == semio_framework::AppRole::Editor { &mut editor } else { &mut viewer };\n            if slot.replace(definition).is_some() {\n                return Err(plugin_internal_fault("artifact codec schema resolves more than one app of the same role"));\n            }\n        }\n        editor.or(viewer).ok_or_else(|| plugin_internal_fault("artifact codec schema is owned by no app of this bundle"))\n    }\n\n    /// 🧬️ `codec.pack-schema-hash` — the kind\'s own 32-byte snapshot-record fingerprint.\n    pub async fn plugin_artifact_pack_schema_hash<PA: PluginApp>(runtime: &PluginRuntime<PA>, artifact_schema: &str) -> Result<[u8; 32], Fault> {\n        let codec = plugin_artifact_codec(runtime, artifact_schema)?;\n        (codec.pack_schema_hash)().ok_or_else(|| plugin_internal_fault("artifact codec schema has no structural record specification"))\n    }\n\n'
CODEC_NEW = '    /// 🧬️ Runs `answer` on the installed bundle once `artifact_schema` is a bounded identity and the bundle assembled — the\n    /// guard every `world actor` `codec` answer shares.\n    fn with_codec_program<PA: PluginApp, T>(runtime: &PluginRuntime<PA>, artifact_schema: &str, answer: impl FnOnce(&Plugin<PA>) -> Result<T, Fault>) -> Result<T, Fault> {\n        if artifact_schema.is_empty() || artifact_schema.len() > 256 || artifact_schema.chars().any(char::is_control) {\n            return Err(plugin_internal_fault("artifact codec schema identity is empty, oversized or control-bearing"));\n        }\n        if let Some(fault) = runtime.plugin_assembly_error.try_borrow().map_err(|_| plugin_internal_fault("plugin assembly authority busy"))?.clone() {\n            return Err(fault);\n        }\n        let program = runtime.plugin.try_borrow().map_err(|_| plugin_internal_fault("plugin factory authority busy"))?;\n        answer(program.as_ref().ok_or_else(|| plugin_internal_fault("plugin not initialized"))?)\n    }\n\n    /// 🧬️ Resolves the `codec` answers of the ONE app of the installed bundle that owns `artifact_schema`. This is\n    /// the guest half of `world actor`\'s `codec` interface: a host that links no Rust codec for this package selects\n    /// a document kind by the same `schema` string `store::ArtifactCodec` is keyed by, and the bundle answers from its\n    /// own registered apps ([`artifact_codec_owner`]). Nothing is constructed: the answers are the functions of the\n    /// owner\'s type its registration recorded ([`crate::app::ArtifactCodecTableV1`]).\n    fn plugin_artifact_codec<PA: PluginApp>(runtime: &PluginRuntime<PA>, artifact_schema: &str) -> Result<crate::app::ArtifactCodecTableV1, Fault> {\n        with_codec_program(runtime, artifact_schema, |program| {\n            let definition = artifact_codec_owner(program, artifact_schema)?;\n            program.app_codec(&definition.id).ok_or_else(|| plugin_internal_fault("artifact codec owner has no registered codec"))\n        })\n    }\n\n    /// \U0001faaa️ The apps of `program` that own `artifact_schema`: those whose registered type opens that document\n    /// schema — the primary key `store::ArtifactCodec` and the hub\'s trusted catalog use — and those whose\n    /// dialect names it as an ARTIFACT KIND, the identity build tooling with nothing but a compiled\n    /// descriptor asks by (it reads the schema back out of `codec.genesis` and asks by schema after that; a\n    /// kind `s.<plugin>.<artifact>` and a `DOCUMENT_SCHEMA` never collide). Pure: nothing is constructed.\n    pub(crate) fn artifact_codec_candidates<\'a, PA: PluginApp>(program: &\'a Plugin<PA>, artifact_schema: &\'a str) -> impl Iterator<Item = &\'a crate::app::AppDefinition> + \'a {\n        program.manifest.apps.iter().filter(move |definition| program.app_document_schema(&definition.id) == Some(artifact_schema) || definition.dialect.artifact_kind == artifact_schema)\n    }\n\n    /// 🌳️ Whether `general` is the whole-standard dialect (`<kind>@<standard>/*`) of `subset`\'s own standard: every\n    /// document of a subset of a standard is a document of the whole standard, so the whole-standard app covers it.\n    fn dialect_covers(general: &semio_framework::ArtifactDialect, subset: &semio_framework::ArtifactDialect) -> bool {\n        general.subset == crate::SubsetId::ANY.0 && subset.subset != crate::SubsetId::ANY.0 && general.artifact_kind == subset.artifact_kind && general.standard == subset.standard\n    }\n\n    /// 🎯️ The ONE app whose codec answers `artifact_schema`: the owning editor — only an editor\'s snapshot is the\n    /// kind\'s creation authority — else the owning viewer. Within a role the most general dialect decides\n    /// ([`dialect_covers`]): subset surfaces may share their standard\'s document schema (stdio\'s `json@rfc8259/*` and\n    /// `json@rfc8259/i-json` both open `stdio.json`), and the whole-standard app answers for them. Owners of one role\n    /// that no other owner covers and that are more than one (whole-standard apps of two standards sharing a schema)\n    /// are refused rather than resolved by order, and no owner is refused; all of it decided on the declarations.\n    pub(crate) fn artifact_codec_owner<\'a, PA: PluginApp>(program: &\'a Plugin<PA>, artifact_schema: &\'a str) -> Result<&\'a crate::app::AppDefinition, Fault> {\n        let most_general = |role: semio_framework::AppRole| {\n            let owners = artifact_codec_candidates(program, artifact_schema).filter(|definition| definition.role == role).collect::<Vec<_>>();\n            let mut general = owners.iter().copied().filter(|owner| !owners.iter().any(|other| dialect_covers(&other.dialect, &owner.dialect)));\n            match (general.next(), general.next()) {\n                (Some(_), Some(_)) => Err(plugin_internal_fault("artifact codec schema resolves more than one most-general app of the same role")),\n                (owner, _) => Ok(owner),\n            }\n        };\n        match most_general(semio_framework::AppRole::Editor)? {\n            Some(editor) => Ok(editor),\n            None => most_general(semio_framework::AppRole::Viewer)?.ok_or_else(|| plugin_internal_fault("artifact codec schema is owned by no app of this bundle")),\n        }\n    }\n\n    /// 🧬️ `codec.pack-schema-hash` — the document schema\'s pack-schema identity. It belongs to the SCHEMA, not to one\n    /// app: every app that opens `artifact_schema` (editors and viewers of every subset and standard) declares it through\n    /// its snapshot (`store::ArtifactPack::pack_schema_hash`), and the one value they all agree on answers — no creation\n    /// authority is needed to identify a document. Owners that disagree are refused, a schema no app opens is refused as\n    /// unowned, and a schema whose owners declare no identity is refused as such.\n    pub async fn plugin_artifact_pack_schema_hash<PA: PluginApp>(runtime: &PluginRuntime<PA>, artifact_schema: &str) -> Result<[u8; 32], Fault> {\n        with_codec_program(runtime, artifact_schema, |program| {\n            let mut identities = artifact_codec_candidates(program, artifact_schema).map(|definition| program.app_codec(&definition.id).map(|codec| (codec.pack_schema_hash)()).ok_or_else(|| plugin_internal_fault("artifact codec owner has no registered codec")));\n            let identity = identities.next().ok_or_else(|| plugin_internal_fault("artifact codec schema is owned by no app of this bundle"))??;\n            for other in identities {\n                if other? != identity {\n                    return Err(plugin_internal_fault("artifact codec schema is owned by apps that declare different pack-schema identities"));\n                }\n            }\n            identity.ok_or_else(|| plugin_internal_fault("artifact codec schema has no structural record specification"))\n        })\n    }\n\n'


def plugin(text):
    text = once(
        text,
        "        /// 🧬️ Structural fingerprint of the app's snapshot record shape — the same 32 bytes\n"
        "        /// `store::ArtifactCodec::pack_schema_hash` carries, read off `Snapshot::record_spec()`. `None` when the\n"
        "        /// snapshot is a hand-written `ArtifactPack` with no `RecordSpec`, exactly as the native codec table reports\n"
        "        /// `[0; 32]` for that case.\n",
        "        /// 🧬️ The app snapshot's pack-schema identity — the same 32 bytes `store::ArtifactCodec::pack_schema_hash`\n"
        "        /// carries, read off `Snapshot::pack_schema_hash()`. `None` when the snapshot declares none, exactly as the\n"
        "        /// native codec table reports `[0; 32]` for that case.\n",
        "plugin: codec table doc",
    )
    text = once(
        text,
        "            <A::Snapshot as store::ArtifactPack>::record_spec().map(|spec| store::os_pack::schema_hash(&spec))\n",
        "            <A::Snapshot as store::ArtifactPack>::pack_schema_hash()\n",
        "plugin: codec table hash",
    )
    return once(text, CODEC_OLD, CODEC_NEW, "plugin: codec identity + most-general owner")

FIXTURE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs"


def fixture(text):
    text = once(
        text,
        "                fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {\n"
        "                    if bytes.is_empty() {\n"
        "                        return Ok(Self::default());\n"
        "                    }\n"
        "                    serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()))\n"
        "                }\n"
        "            }\n",
        "                fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {\n"
        "                    if bytes.is_empty() {\n"
        "                        return Ok(Self::default());\n"
        "                    }\n"
        "                    serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()))\n"
        "                }\n"
        "\n"
        "                /// 🪪️ Each fixture channel's own pack-schema identity: the SHA-256 of its schema id, so channels differ.\n"
        "                fn pack_schema_hash() -> Option<[u8; 32]> {\n"
        "                    Some(store::protocol_pack_schema_hash($schema.as_bytes()))\n"
        "                }\n"
        "            }\n",
        "fixture: channel pack-schema identity",
    )
    text = once(
        text,
        "            assert!(hash.as_ref().is_ok_and(|hash| *hash != [0; 32]) || format!(\"{hash:?}\").contains(\"no structural record specification\"), \"{schema}: {hash:?}\");\n",
        "            assert!(hash.as_ref().is_ok_and(|hash| *hash != [0; 32]), \"{schema}: {hash:?}\");\n",
        "fixture: nonzero identity",
    )
    return once(
        text,
        "        assert!(crate::plugin_runtime::plugin_artifact_pack_schema_hash(&runtime, \"semio.testkit.nobody/v1\").await.is_err(), \"no owner is refused\");\n    }\n",
        "        assert!(crate::plugin_runtime::plugin_artifact_pack_schema_hash(&runtime, \"semio.testkit.nobody/v1\").await.is_err(), \"no owner is refused\");\n    }\n"
        + SHARED_SCHEMA_LAW,
        "fixture: shared-schema law",
    )


SHARED_SCHEMA_LAW = """
    /// 🌳️ Subset surfaces may share their standard's document schema (stdio's `json@rfc8259/*` and `…/i-json` both open
    /// `stdio.json`). The fixture's `std1/strict` apps are re-pointed at `std1/*`'s schema: the whole-standard editor is
    /// the creation authority (`artifact_codec_owner`: `std1/*` covers `std1/strict`) and answers genesis; with `std2/*`
    /// sharing it too, two whole-standard editors of two standards are refused rather than picked by order. Identity is
    /// the schema's: one channel's apps agree and answer it, owners declaring different identities are refused.
    #[semio_framework_async_macros::async_test]
    async fn a_shared_schema_is_created_by_its_most_general_editor_and_identified_only_by_agreement() {
        const SHARED: &str = "semio.testkit.w1c-fixture.std1-any/v1";
        fn bundle(sharing: &[Dialect]) -> crate::app::Plugin<FixtureApps> {
            let sharing = sharing.iter().map(|dialect| ArtifactDialect::from(*dialect)).collect::<Vec<_>>();
            let mut plugin = crate::app::Plugin::<FixtureApps>::new("testkit", "Testkit", "1.0.0");
            for (app, mut factory) in project_artifact_declarations(&[build_declaration()]).app_defs {
                if sharing.contains(&app.definition.dialect) {
                    factory.document_schema = SHARED;
                }
                plugin = plugin.register_app_factory(app, factory);
            }
            plugin
        }
        let subsets = bundle(&[STD1_ANY_DIALECT, STD1_STRICT_DIALECT]);
        assert_eq!(crate::plugin_runtime::artifact_codec_candidates(&subsets, SHARED).count(), 4, "both std1 editors and viewers open the shared schema");
        let owner = crate::plugin_runtime::artifact_codec_owner(&subsets, SHARED).expect("the whole-standard editor");
        assert_eq!((owner.role, &owner.dialect), (AppRole::Editor, &ArtifactDialect::from(STD1_ANY_DIALECT)));
        let standards = bundle(&[STD1_ANY_DIALECT, STD1_STRICT_DIALECT, STD2_ANY_DIALECT]);
        let refused = crate::plugin_runtime::artifact_codec_owner(&standards, SHARED).expect_err("two whole-standard editors");
        assert!(refused.message.contains("more than one most-general app"), "{}", refused.message);
        let runtime = crate::plugin_runtime::PluginRuntime::new();
        crate::plugin_runtime::install_plugin_bundle(&runtime, subsets);
        let document_id = format!("artifact-{}", "2".repeat(32));
        let pair = crate::plugin_runtime::plugin_artifact_genesis(&runtime, SHARED, &document_id).await.expect("the whole-standard editor answers genesis");
        assert!(!pair.pack.is_empty(), "genesis pair");
        let disagreement = crate::plugin_runtime::plugin_artifact_pack_schema_hash(&runtime, SHARED).await.expect_err("std1/* and std1/strict snapshots declare different identities");
        assert!(disagreement.message.contains("different pack-schema identities"), "{}", disagreement.message);
        let own = crate::plugin_runtime::plugin_artifact_pack_schema_hash(&runtime, "semio.testkit.w1c-fixture.std2-any/v1").await.expect("std2's editor and viewer agree");
        assert_eq!(own, store::protocol_pack_schema_hash(b"semio.testkit.w1c-fixture.std2-any/v1"));
    }
"""

#endregion Framework


#region Stdio
METHOD = (
    "\n"
    "    /// 🪪️ Pack-schema identity: the SHA-256 of this snapshot's schema-first binary protocol (`💾️binary/📡️.protocol.semio`) —\n"
    "    /// the native codec, the guest's `codec.pack-schema-hash` and the hub's trusted catalog pin the same 32 bytes.\n"
    "    fn pack_schema_hash() -> Option<[u8; 32]> {\n"
    f"        Some(store::protocol_pack_schema_hash(include_bytes!(\"{PROTOCOL}\")))\n"
    "    }\n"
)
TXT_SPEC = re.compile(r"    /// 🧬️ The structural fingerprint `ArtifactCodec::pack_schema_hash`.*?    fn record_spec\(\) -> Option<dsl::RecordSpec> \{\n        Some\(Self::__dsl_spec\(\)\)\n    \}\n", re.S)


def impl_span(text, start):
    depth = 0
    for index in range(start, len(text)):
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return index
    return None


def snapshot_impls():
    found = subprocess.run(["/usr/bin/grep", "-rl", "--include=🦀️.rs", "impl store::ArtifactPack for", ART], cwd=TREE, capture_output=True, text=True).stdout.split("\n")
    return sorted(path for path in found if path and "/📸️snapshot/" in path)


def snapshot(text, path):
    if path == TXT:
        found = TXT_SPEC.findall(text)
        if len(found) != 1:
            problems.append(f"{path}: txt hash-only record_spec override x{len(found)}")
            return text
        text = text.replace(found[0], "")
    impls = list(re.finditer(r"impl store::ArtifactPack for (\w+) \{", text))
    if len(impls) != 1:
        problems.append(f"{path}: {len(impls)} ArtifactPack impls")
        return text
    if not os.path.isfile(os.path.join(TREE, os.path.dirname(path), PROTOCOL)):
        problems.append(f"{path}: no {PROTOCOL}")
        return text
    end = impl_span(text, impls[0].end() - 1)
    body = text[impls[0].end() : end]
    if "fn pack_schema_hash" in body:
        problems.append(f"{path}: pack_schema_hash already declared")
        return text
    return text[:end] + METHOD + text[end:]


FACTORY_LINE = re.compile(r'    codec\.pack_schema_hash = semio_framework_hash::Sha256::digest\(include_bytes!\("([^"]+)"\)\);\n')
CODEC_OF = re.compile(r"let mut codec = store::ArtifactCodec::of::<(?:[\w:]+::)?(\w+), ")


def factory_files():
    found = subprocess.run(["/usr/bin/grep", "-rl", "--include=🦀️.rs", "codec.pack_schema_hash = semio_framework_hash::Sha256::digest", ART], cwd=TREE, capture_output=True, text=True).stdout.split("\n")
    return sorted(path for path in found if path)


def factory(text, path, impls):
    lines = FACTORY_LINE.findall(text)
    types = CODEC_OF.findall(text)
    if len(lines) != 1 or len(types) != 1:
        problems.append(f"{path}: {len(lines)} digest lines, {len(types)} codec types")
        return text
    included = os.path.normpath(os.path.join(os.path.dirname(path), lines[0]))
    artifact = path.split("/")[4]
    candidates = [impl for impl in impls if impl.split("/")[4] == artifact and re.search(rf"impl store::ArtifactPack for {types[0]} \{{", open(os.path.join(TREE, impl), encoding="utf-8").read())]
    protocols = {os.path.normpath(os.path.join(os.path.dirname(impl), PROTOCOL)) for impl in candidates}
    if included not in protocols:
        problems.append(f"{path}: its digest names {included}, not the protocol of {types[0]} ({sorted(protocols)})")
        return text
    return FACTORY_LINE.sub("", text)


HASH_DEPENDENCY = "semio-framework-hash = { workspace = true }\n"


def unused_hash_crates(after):
    """🧹️ The artifact crates whose only `semio_framework_hash` use was the removed digest override — their dependency goes."""
    crates = []
    for path in factory_files():
        artifact = "/".join(path.split("/")[:5])
        users = subprocess.run(["/usr/bin/grep", "-rl", "--include=*.rs", "semio_framework_hash", artifact], cwd=TREE, capture_output=True, text=True).stdout.split()
        if not any((after[user] if user in after else open(os.path.join(TREE, user), encoding="utf-8").read()).count("semio_framework_hash") for user in users):
            crates.append(f"{artifact}/📦️packages/🦀️rust/Cargo.toml")
    return crates


def manifest(text, path):
    if text.count(HASH_DEPENDENCY) != 1:
        problems.append(f"{path}: semio-framework-hash dependency x{text.count(HASH_DEPENDENCY)}")
        return text
    return text.replace(HASH_DEPENDENCY, "")


def lock(text, names):
    for name in names:
        head = f'[[package]]\nname = "{name}"\n'
        start = text.find(head)
        end = text.find("\n\n", start)
        block = text[start:end]
        if start < 0 or block.count('\n "semio-framework-hash",') != 1:
            problems.append(f"Cargo.lock: {name} has no single semio-framework-hash edge")
            continue
        text = text[:start] + block.replace('\n "semio-framework-hash",', "", 1) + text[end:]
    return text
#endregion Stdio


#region Law
LAW_ANCHOR = "/// 🏠️ LAW (describe side of hosting, p15):"
NEW_LAW = r'''/// 🧪️ The environment variable naming the one package [`package_codec_probe`] assembles in its child process.
const CODEC_PROBE_PACKAGE: &str = "SEMIO_STDIO_CODEC_PROBE_PACKAGE";

/// 🧪️ With a directory named here, each [`package_codec_probe`] child writes `<package>.json` — its `(kind, schema, hash)`
/// answers — for the hashlib oracle (ticket `26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP`, `wp-lb2/lb2-p17-pack-schema-oracle.py`).
const CODEC_HASH_OUT: &str = "SEMIO_STDIO_CODEC_HASH_OUT";

/// 🪪️ LAW (f): every stdio package, assembled ALONE in its own process as its wasm guest is, answers `codec.pack-schema-hash`
/// for the document schema of every app of every kind it opens with a nonzero identity, and where the hub links a native
/// codec for the kind (`📇️registry/📜️native-codec-factories.json`) with exactly that codec's pinned hash. Measured before
/// (G12 live MCP coverage + LB2, 2026-09-29): 54 of 57 stdio snapshots declared no identity, so every guest open was refused
/// "artifact codec schema has no structural record specification"; txt answered its DSL spec hash while its linked native
/// codec pinned its protocol digest; and the 13 schemas subset surfaces share (json, xml, jpg, svg, tiff, dwg, step,
/// ifc 2x3, docx, pptx, xlsx, zip, pdf 1.7) were refused "more than one app of the same role".
#[test]
fn every_package_answers_the_pack_schema_identity_of_every_kind_it_opens_in_its_own_process() {
    let binary = std::env::current_exe().expect("the test binary");
    let failures = PACKAGE_IDS
        .into_iter()
        .filter_map(|id| {
            let run = std::process::Command::new(&binary).args(["package_codec_probe", "--exact", "--ignored", "--nocapture", "--test-threads", "1"]).env(CODEC_PROBE_PACKAGE, id).output().expect("the codec probe runs");
            (!run.status.success()).then(|| format!("{id} alone does not answer every pack-schema identity:\n{}\n{}", String::from_utf8_lossy(&run.stdout), String::from_utf8_lossy(&run.stderr)))
        })
        .collect::<Vec<_>>();
    assert!(failures.is_empty(), "{} of {} packages fail:\n{}", failures.len(), PACKAGE_IDS.len(), failures.join("\n"));
}

/// 🔬️ The child half of LAW (f): assembles only the package [`CODEC_PROBE_PACKAGE`] names and asks its runtime for the
/// pack-schema identity of every `(kind, document schema)` its apps open, for every kind it opens.
#[test]
#[ignore = "the child process of every_package_answers_the_pack_schema_identity_of_every_kind_it_opens_in_its_own_process"]
fn package_codec_probe() {
    let id = std::env::var(CODEC_PROBE_PACKAGE).expect("the parent law names one package");
    let package = shipped(PACKAGE_IDS.into_iter().find(|candidate| *candidate == id).expect("a stdio package id"));
    assert_eq!(package.descriptor.package_id, format!("semio:{id}"), "{id} assembles alone");
    let registry: serde_json::Value = serde_json::from_str(include_str!("../../📇️registry/📜️native-codec-factories.json")).expect("the linked codec registry");
    let linked = registry["receipts"]
        .as_array()
        .expect("linked codec receipts")
        .iter()
        .map(|row| ((row["artifact_kind"].as_str().expect("kind").to_owned(), row["artifact_schema"].as_str().expect("schema").to_owned()), row["pack_schema_sha256"].as_str().expect("pinned hash").to_owned()))
        .collect::<BTreeMap<_, _>>();
    let opened = activated_kinds(&package.descriptor);
    let mut faults = opened.iter().filter(|kind| !package.codecs.iter().any(|(row, _, _)| row == *kind)).map(|kind| format!("{kind}: opened without a declared document schema")).collect::<Vec<_>>();
    for (kind, schema, answer) in package.codecs.iter().filter(|(kind, _, _)| opened.contains(kind)) {
        match answer {
            Err(fault) => faults.push(format!("{kind}={schema}: {fault}")),
            Ok(hash) if *hash == [0; 32] => faults.push(format!("{kind}={schema}: zero identity")),
            Ok(hash) => {
                let hex = hash.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
                if linked.get(&(kind.clone(), schema.clone())).is_some_and(|pinned| *pinned != hex) {
                    faults.push(format!("{kind}={schema}: {hex} differs from the linked native codec"));
                }
            }
        }
    }
    if let Some(out) = std::env::var_os(CODEC_HASH_OUT) {
        let rows = package
            .codecs
            .iter()
            .filter(|(kind, _, _)| opened.contains(kind))
            .filter_map(|(kind, schema, answer)| answer.as_ref().ok().map(|hash| serde_json::json!({ "kind": kind, "schema": schema, "hash": hash.iter().map(|byte| format!("{byte:02x}")).collect::<String>() })))
            .collect::<Vec<_>>();
        std::fs::write(std::path::Path::new(&out).join(format!("{id}.json")), serde_json::to_vec(&rows).expect("answers json")).expect("the oracle answers");
    }
    assert!(!opened.is_empty(), "{id} opens stdio kinds");
    assert!(faults.is_empty(), "{id} alone answers {} pack-schema identities wrongly: {faults:#?}", faults.len());
}

'''


def law(text):
    text = once(
        text,
        "struct ShippedPackage {\n    id: &'static str,\n    manifest: &'static str,\n    descriptor: PackageDescriptor,\n}\n",
        "struct ShippedPackage {\n    id: &'static str,\n    manifest: &'static str,\n    descriptor: PackageDescriptor,\n    codecs: Vec<(String, String, Result<[u8; 32], String>)>,\n}\n",
        "law: ShippedPackage codecs",
    )
    text = once(
        text,
        "/// 🛂️ Describes one assembled bundle through the runtime path its component's `describe` export takes.\n"
        "fn describe<PA: PluginApp>(bundle: Result<Plugin<PA>, PluginAssemblyError>) -> PackageDescriptor {\n"
        "    let runtime = PluginRuntime::<PA>::new();\n"
        "    install_plugin_bundle_result(&runtime, bundle);\n"
        "    let bytes = semio_framework_plugin::app::resolve_ready(semio_framework_plugin::describe::describe_plugin(&runtime));\n"
        "    semio_framework::from_dsl_value(semio_framework_os_kernel::pack_rt::decode_wire_value(&bytes).expect(\"descriptor bytes\")).expect(\"strict descriptor\")\n"
        "}\n",
        "/// 🛂️ Describes one assembled bundle through the runtime path its component's `describe` export takes, and asks the same\n"
        "/// runtime — as its `codec.pack-schema-hash` export does — for the identity of every `(kind, document schema)` its apps\n"
        "/// open (`io.artifactSchema`): the rows a host mounting the component registers component codecs for.\n"
        "fn describe<PA: PluginApp>(bundle: Result<Plugin<PA>, PluginAssemblyError>) -> (PackageDescriptor, Vec<(String, String, Result<[u8; 32], String>)>) {\n"
        "    let runtime = PluginRuntime::<PA>::new();\n"
        "    install_plugin_bundle_result(&runtime, bundle);\n"
        "    let bytes = semio_framework_plugin::app::resolve_ready(semio_framework_plugin::describe::describe_plugin(&runtime));\n"
        "    let descriptor: PackageDescriptor = semio_framework::from_dsl_value(semio_framework_os_kernel::pack_rt::decode_wire_value(&bytes).expect(\"descriptor bytes\")).expect(\"strict descriptor\");\n"
        "    let rows = descriptor.manifest.apps.iter().map(|app| (app.dialect.artifact_kind.clone(), app.io.artifact_schema.clone())).collect::<BTreeSet<_>>();\n"
        "    let codecs = rows\n"
        "        .into_iter()\n"
        "        .map(|(kind, schema)| {\n"
        "            let answer = semio_framework_plugin::app::resolve_ready(semio_framework_plugin::plugin_runtime::plugin_artifact_pack_schema_hash(&runtime, &schema)).map_err(|fault| format!(\"{fault:?}\"));\n"
        "            (kind, schema, answer)\n"
        "        })\n"
        "        .collect();\n"
        "    (descriptor, codecs)\n"
        "}\n",
        "law: describe with codec answers",
    )
    text = once(
        text,
        "        other => panic!(\"{other} is not a stdio package\"),\n    };\n    ShippedPackage { id, manifest, descriptor }\n}\n",
        "        other => panic!(\"{other} is not a stdio package\"),\n    };\n    let (descriptor, codecs) = described;\n    ShippedPackage { id, manifest, descriptor, codecs }\n}\n",
        "law: shipped keeps codec answers",
    )
    text = once(text, "    let (manifest, descriptor) = match id {\n", "    let (manifest, described) = match id {\n", "law: shipped binding")
    return once(text, LAW_ANCHOR, NEW_LAW + LAW_ANCHOR, "law (f)")
#endregion Law


def plan():
    impls = snapshot_impls()
    if len(impls) != 57:
        problems.append(f"{len(impls)} stdio snapshot ArtifactPack impls, expected 57")
    edits = {STORE: store, PLUGIN: plugin, FIXTURE: fixture, LAW: law}
    for path in impls:
        edits[path] = lambda text, path=path: snapshot(text, path)
    factories = factory_files()
    if len(factories) != 29:
        problems.append(f"{len(factories)} native codec digest overrides, expected 29")
    for path in factories:
        edits[path] = lambda text, path=path: factory(text, path, impls)
    after = {path: edit(open(os.path.join(TREE, path), encoding="utf-8").read()) for path, edit in edits.items() if path.endswith(".rs")}
    crates = unused_hash_crates(after)
    names = []
    for path in crates:
        edits[path] = lambda text, path=path: manifest(text, path)
        found = re.search(r'^name = "([^"]+)"', open(os.path.join(TREE, path), encoding="utf-8").read(), re.M)
        names.append(found.group(1))
    edits["Cargo.lock"] = lambda text: lock(text, names)
    return edits


def main():
    mode = next((flag for flag in ("--dry-run", "--write", "--revert") if flag in sys.argv), None)
    if mode is None:
        print(__doc__)
        sys.exit(2)
    if mode == "--revert":
        for root, _, files in os.walk(BACKUP):
            for file in files:
                source = os.path.join(root, file)
                path = os.path.relpath(source, BACKUP)
                shutil.copyfile(source, os.path.join(TREE, path))
                print("restored", path)
        return
    edits = plan()
    staged = {}
    for path, edit in edits.items():
        before = open(os.path.join(TREE, path), encoding="utf-8").read()
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
