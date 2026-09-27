#!/usr/bin/env python3
"""🧬️ H12: codec calls answer through a per-app table of plain functions recorded at registration; they construct
no app. usage: codec-table.py [--dry-run] [--root <repo>]"""
import pathlib, sys
ROOT = pathlib.Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else pathlib.Path("/Users/ueli/Documents/semio")
DRY = "--dry-run" in sys.argv
PLUGIN = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
BUILDER = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs"
LAW = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs"

TRAIT_OLD = '''        /// 🧬️ Structural fingerprint of this app's snapshot record shape — the same 32 bytes
        /// `store::ArtifactCodec::pack_schema_hash` carries, read straight off the app's own
        /// `Snapshot::record_spec()`. `None` when the snapshot is a hand-written `ArtifactPack` with
        /// no `RecordSpec`, exactly as the native codec table reports `[0; 32]` for that case.
        async fn artifact_pack_schema_hash(&self) -> Option<[u8; 32]>;
        /// 🌱️ The canonical empty document of this app's kind at `document_id`: this app's initial
        /// snapshot, its own dialect, and an exactly zero history (no edits, changes, checkpoints or
        /// alternatives, and an empty cursor). It is a pure function of `(Self, document_id)` — the
        /// instance's own live document is neither read nor replaced.
        async fn artifact_genesis_pair(&self, document_id: &str) -> Result<store::ArtifactPackFiles, Fault>;
        /// 📥️ `(pack, spr) -> (dsl, ops)` mirror of a pair belonging to this app's kind, without
        /// loading it into this instance. A host uses it as the pair-validation fence for a package
        /// whose codec it does not link.
        async fn artifact_print_mirror(&self, pack: &[u8], spr: &[u8]) -> Result<store::ArtifactTextFiles, Fault>;
        /// 🧩️ Applies one `os_spr::encode_ops_vec` batch to a pair of this app's kind and returns the
        /// next pair, again without touching this instance's own document.
        async fn artifact_apply_ops(&self, pack: &[u8], spr: &[u8], ops: &[u8]) -> Result<store::ArtifactPackFiles, Fault>;
        /// 📜️ Folds one `os_spr::encode_envelopes` ledger stream onto a pair of this app's kind through
        /// the replica merge gate (`store::replay_envelopes_onto_pair`), again without touching this
        /// instance's own document.
        async fn artifact_replay_envelopes(&self, pack: &[u8], spr: &[u8], envelopes: &[u8]) -> Result<store::ArtifactPackFiles, Fault>;
'''

IMPL_OLD = '''        async fn artifact_pack_schema_hash(&self) -> Option<[u8; 32]> {
            match <A::Snapshot as store::ArtifactPack>::record_spec() {
                Some(spec) => Some(store::os_pack::schema_hash(&spec)),
                None => None,
            }
        }

        async fn artifact_genesis_pair(&self, document_id: &str) -> Result<store::ArtifactPackFiles, Fault> {
            artifact_app_genesis_pair::<A>(document_id).await.map_err(|error| error.into_fault())
        }

        async fn artifact_print_mirror(&self, pack: &[u8], spr: &[u8]) -> Result<store::ArtifactTextFiles, Fault> {
            let parsed: store::ParsedDocumentText<A::Snapshot, A::Mutation> = store::parse_document_pack(pack, spr).await.map_err(|error| error.into_fault())?;
            let envelope = parsed.into_envelope();
            let mirror = store::print_document_text(&envelope).await;
            drop(envelope.into_owners());
            mirror.map_err(|error| error.into_fault())
        }

        async fn artifact_apply_ops(&self, pack: &[u8], spr: &[u8], ops: &[u8]) -> Result<store::ArtifactPackFiles, Fault> {
            artifact_app_apply_ops::<A>(pack, spr, ops).await.map_err(|error| error.into_fault())
        }

        async fn artifact_replay_envelopes(&self, pack: &[u8], spr: &[u8], envelopes: &[u8]) -> Result<store::ArtifactPackFiles, Fault> {
            artifact_app_replay_envelopes::<A>(pack, spr, envelopes).await.map_err(|error| error.into_fault())
        }

'''

TABLE_ANCHOR = '''    pub async fn artifact_app_replay_envelopes<A: ArtifactApp>(pack: &[u8], spr: &[u8], envelopes: &[u8]) -> Result<store::ArtifactPackFiles, store::VcsError> {
        store::replay_envelopes_onto_pair::<A::Snapshot, A::Mutation>(pack, spr, envelopes, || A::build_document_store_owners().unwrap_or_else(store::bounded_artifact_store_owners)).await
    }
'''
TABLE_NEW = TABLE_ANCHOR + '''
    /// 🧬️ The future one `codec` answer of an [`ArtifactCodecTableV1`] resolves to.
    pub type ArtifactCodecFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, Fault>> + 'a>>;

    /// 🧬️ One registered app's `world actor` `codec` answers as plain functions of its type, recorded when the app
    /// is registered. Every answer is a pure function of the app's TYPE — its snapshot record shape, its initial
    /// snapshot and dialect, its owner catalogue — never of a constructed instance, so a `codec` call reads this
    /// table and constructs no app: no store around an initial snapshot, no action registry, no window owners,
    /// nothing to drain and close afterwards.
    #[derive(Clone, Copy)]
    pub struct ArtifactCodecTableV1 {
        /// 🧬️ Structural fingerprint of the app's snapshot record shape — the same 32 bytes
        /// `store::ArtifactCodec::pack_schema_hash` carries, read off `Snapshot::record_spec()`. `None` when the
        /// snapshot is a hand-written `ArtifactPack` with no `RecordSpec`, exactly as the native codec table reports
        /// `[0; 32]` for that case.
        // 🚫️async: E4 fn-pointer slot
        pub pack_schema_hash: fn() -> Option<[u8; 32]>,
        /// 🌱️ The canonical empty document of the app's kind at `document_id`: its initial snapshot, its own
        /// dialect, and an exactly zero history (no edits, changes, checkpoints or alternatives, and an empty cursor).
        // 🚫️async: E4 fn-pointer slot
        pub genesis: for<'a> fn(&'a str) -> ArtifactCodecFuture<'a, store::ArtifactPackFiles>,
        /// 📥️ `(pack, spr) -> (dsl, ops)` mirror of a pair of the app's kind — the pair-validation fence of a host
        /// whose codec it does not link.
        // 🚫️async: E4 fn-pointer slot
        pub print_mirror: for<'a> fn(&'a [u8], &'a [u8]) -> ArtifactCodecFuture<'a, store::ArtifactTextFiles>,
        /// 🧩️ Applies one `os_spr::encode_ops_vec` batch to a pair of the app's kind and returns the next pair.
        // 🚫️async: E4 fn-pointer slot
        pub apply_ops: for<'a> fn(&'a [u8], &'a [u8], &'a [u8]) -> ArtifactCodecFuture<'a, store::ArtifactPackFiles>,
        /// 📜️ Folds one `os_spr::encode_envelopes` ledger stream onto a pair of the app's kind through the replica
        /// merge gate (`store::replay_envelopes_onto_pair`).
        // 🚫️async: E4 fn-pointer slot
        pub replay_envelopes: for<'a> fn(&'a [u8], &'a [u8], &'a [u8]) -> ArtifactCodecFuture<'a, store::ArtifactPackFiles>,
    }

    /// 🧬️ The [`ArtifactCodecTableV1`] of every app built on `A`.
    pub fn artifact_codec_table<A: ArtifactApp>() -> ArtifactCodecTableV1 {
        fn pack_schema_hash<A: ArtifactApp>() -> Option<[u8; 32]> {
            <A::Snapshot as store::ArtifactPack>::record_spec().map(|spec| store::os_pack::schema_hash(&spec))
        }
        fn genesis<A: ArtifactApp>(document_id: &str) -> ArtifactCodecFuture<'_, store::ArtifactPackFiles> {
            Box::pin(async move { artifact_app_genesis_pair::<A>(document_id).await.map_err(|error| error.into_fault()) })
        }
        fn print_mirror<'a, A: ArtifactApp>(pack: &'a [u8], spr: &'a [u8]) -> ArtifactCodecFuture<'a, store::ArtifactTextFiles> {
            Box::pin(async move {
                let parsed: store::ParsedDocumentText<A::Snapshot, A::Mutation> = store::parse_document_pack(pack, spr).await.map_err(|error| error.into_fault())?;
                let envelope = parsed.into_envelope();
                let mirror = store::print_document_text(&envelope).await;
                drop(envelope.into_owners());
                mirror.map_err(|error| error.into_fault())
            })
        }
        fn apply_ops<'a, A: ArtifactApp>(pack: &'a [u8], spr: &'a [u8], ops: &'a [u8]) -> ArtifactCodecFuture<'a, store::ArtifactPackFiles> {
            Box::pin(async move { artifact_app_apply_ops::<A>(pack, spr, ops).await.map_err(|error| error.into_fault()) })
        }
        fn replay_envelopes<'a, A: ArtifactApp>(pack: &'a [u8], spr: &'a [u8], envelopes: &'a [u8]) -> ArtifactCodecFuture<'a, store::ArtifactPackFiles> {
            Box::pin(async move { artifact_app_replay_envelopes::<A>(pack, spr, envelopes).await.map_err(|error| error.into_fault()) })
        }
        ArtifactCodecTableV1 { pack_schema_hash: pack_schema_hash::<A>, genesis: genesis::<A>, print_mirror: print_mirror::<A>, apply_ops: apply_ops::<A>, replay_envelopes: replay_envelopes::<A> }
    }
'''

EDITS = [
    (PLUGIN, TRAIT_OLD, ""),
    (PLUGIN, IMPL_OLD, ""),
    (PLUGIN, TABLE_ANCHOR, TABLE_NEW),
    (PLUGIN, '''            pub app_schema: fn() -> Option<::semio_framework_schema::AppSchemaDescriptor>,
            pub document_schema: &'static str,
            pub mutation_roster: Option<OwnerMutationRoster>,''', '''            pub app_schema: fn() -> Option<::semio_framework_schema::AppSchemaDescriptor>,
            pub document_schema: &'static str,
            pub codec: super::ArtifactCodecTableV1,
            pub mutation_roster: Option<OwnerMutationRoster>,'''),
    (PLUGIN, '''            SurfaceDeclaration { definition: def, factory: factory::<E, PA>, app_schema: app_schema::<E>, document_schema: E::DOCUMENT_SCHEMA, mutation_roster: None, rights: Rights::Write }''',
     '''            SurfaceDeclaration { definition: def, factory: factory::<E, PA>, app_schema: app_schema::<E>, document_schema: E::DOCUMENT_SCHEMA, codec: super::artifact_codec_table::<EditorApp<E>>(), mutation_roster: None, rights: Rights::Write }'''),
    (PLUGIN, '''            SurfaceDeclaration { definition: def, factory: factory::<V, PA>, app_schema: app_schema::<V>, document_schema: V::DOCUMENT_SCHEMA, mutation_roster: None, rights: Rights::Read }''',
     '''            SurfaceDeclaration { definition: def, factory: factory::<V, PA>, app_schema: app_schema::<V>, document_schema: V::DOCUMENT_SCHEMA, codec: super::artifact_codec_table::<ViewerApp<V>>(), mutation_roster: None, rights: Rights::Read }'''),
    (PLUGIN, '''            pub create: fn(&AppDefinition) -> PA,
            pub document_schema: &'static str,
        }''', '''            pub create: fn(&AppDefinition) -> PA,
            pub document_schema: &'static str,
            pub codec: super::ArtifactCodecTableV1,
        }'''),
    (PLUGIN, '''AppFactory { definition, create: surface.factory, document_schema: surface.document_schema }''', '''AppFactory { definition, create: surface.factory, document_schema: surface.document_schema, codec: surface.codec }'''),
    (PLUGIN, '''        pub fn app_document_schema(&self, app_id: &str) -> Option<&'static str> {
            self.apps.get(app_id).map(|factory| factory.document_schema)
        }''', '''        pub fn app_document_schema(&self, app_id: &str) -> Option<&'static str> {
            self.apps.get(app_id).map(|factory| factory.document_schema)
        }

        /// 🧬️ The registered app's `codec` answers — functions of its type, recorded at registration, so a codec
        /// call constructs nothing.
        pub fn app_codec(&self, app_id: &str) -> Option<ArtifactCodecTableV1> {
            self.apps.get(app_id).map(|factory| factory.codec)
        }'''),
]

RUNTIME_START = '''    /// 🧬️ Resolves the ONE app of the installed bundle that owns `artifact_schema`, as a fresh
    /// throwaway instance carrying nothing but its own initial document.'''
RUNTIME_END = '''    /// 🧬️ `codec.pack-schema-hash` — the kind's own 32-byte snapshot-record fingerprint.'''
RUNTIME_NEW = '''    /// 🧬️ Resolves the `codec` answers of the ONE app of the installed bundle that owns `artifact_schema`. This is
    /// the guest half of `world actor`'s `codec` interface: a host that links no Rust codec for this package selects
    /// a document kind by the same `schema` string `store::ArtifactCodec` is keyed by, and the bundle answers from its
    /// own registered apps. An editor is preferred over a viewer because only an editor's snapshot is the kind's
    /// creation authority; an ambiguous schema is refused rather than resolved by order. Nothing is constructed: the
    /// answers are the functions of the owner's type its registration recorded ([`crate::app::ArtifactCodecTableV1`]).
    fn plugin_artifact_codec<PA: PluginApp>(runtime: &PluginRuntime<PA>, artifact_schema: &str) -> Result<crate::app::ArtifactCodecTableV1, Fault> {
        if artifact_schema.is_empty() || artifact_schema.len() > 256 || artifact_schema.chars().any(char::is_control) {
            return Err(plugin_internal_fault("artifact codec schema identity is empty, oversized or control-bearing"));
        }
        if let Some(fault) = runtime.plugin_assembly_error.try_borrow().map_err(|_| plugin_internal_fault("plugin assembly authority busy"))?.clone() {
            return Err(fault);
        }
        let program = runtime.plugin.try_borrow().map_err(|_| plugin_internal_fault("plugin factory authority busy"))?;
        let program = program.as_ref().ok_or_else(|| plugin_internal_fault("plugin not initialized"))?;
        let definition = artifact_codec_owner(program, artifact_schema)?;
        program.app_codec(&definition.id).ok_or_else(|| plugin_internal_fault("artifact codec owner has no registered codec"))
    }

    /// 🪪️ The apps of `program` that own `artifact_schema`: those whose registered type opens that document
    /// schema — the primary key `store::ArtifactCodec` and the hub's trusted catalog use — and those whose
    /// dialect names it as an ARTIFACT KIND, the identity build tooling with nothing but a compiled
    /// descriptor asks by (it reads the schema back out of `codec.genesis` and asks by schema after that; a
    /// kind `s.<plugin>.<artifact>` and a `DOCUMENT_SCHEMA` never collide). Pure: nothing is constructed.
    pub(crate) fn artifact_codec_candidates<'a, PA: PluginApp>(program: &'a Plugin<PA>, artifact_schema: &'a str) -> impl Iterator<Item = &'a crate::app::AppDefinition> + 'a {
        program.manifest.apps.iter().filter(move |definition| program.app_document_schema(&definition.id) == Some(artifact_schema) || definition.dialect.artifact_kind == artifact_schema)
    }

    /// 🎯️ The ONE app whose codec answers `artifact_schema`: the owning editor — only an editor's snapshot is the
    /// kind's creation authority — else the owning viewer. Two owners of the same role are refused rather than
    /// resolved by order, and no owner is refused; all of it decided on the declarations.
    pub(crate) fn artifact_codec_owner<'a, PA: PluginApp>(program: &'a Plugin<PA>, artifact_schema: &'a str) -> Result<&'a crate::app::AppDefinition, Fault> {
        let (mut editor, mut viewer) = (None, None);
        for definition in artifact_codec_candidates(program, artifact_schema) {
            let slot = if definition.role == semio_framework::AppRole::Editor { &mut editor } else { &mut viewer };
            if slot.replace(definition).is_some() {
                return Err(plugin_internal_fault("artifact codec schema resolves more than one app of the same role"));
            }
        }
        editor.or(viewer).ok_or_else(|| plugin_internal_fault("artifact codec schema is owned by no app of this bundle"))
    }

'''

CALLS = [
    ('''        let app = plugin_artifact_codec_app(runtime, artifact_schema).await?;
        let answered = app.artifact_pack_schema_hash().await;
        close_artifact_codec_app(app)?;
        answered.ok_or_else(''', '''        let codec = plugin_artifact_codec(runtime, artifact_schema)?;
        (codec.pack_schema_hash)().ok_or_else('''),
    ('''        let app = plugin_artifact_codec_app(runtime, artifact_schema).await?;
        let produced = app.artifact_genesis_pair(document_id).await;
        close_artifact_codec_app(app)?;
        produced''', '''        (plugin_artifact_codec(runtime, artifact_schema)?.genesis)(document_id).await'''),
    ('''        let app = plugin_artifact_codec_app(runtime, artifact_schema).await?;
        let mirrored = app.artifact_print_mirror(pack, spr).await;
        close_artifact_codec_app(app)?;
        mirrored''', '''        (plugin_artifact_codec(runtime, artifact_schema)?.print_mirror)(pack, spr).await'''),
    ('''        let app = plugin_artifact_codec_app(runtime, artifact_schema).await?;
        let applied = app.artifact_apply_ops(pack, spr, ops).await;
        close_artifact_codec_app(app)?;
        applied''', '''        (plugin_artifact_codec(runtime, artifact_schema)?.apply_ops)(pack, spr, ops).await'''),
    ('''        let app = plugin_artifact_codec_app(runtime, artifact_schema).await?;
        let replayed = app.artifact_replay_envelopes(pack, spr, envelopes).await;
        close_artifact_codec_app(app)?;
        replayed''', '''        (plugin_artifact_codec(runtime, artifact_schema)?.replay_envelopes)(pack, spr, envelopes).await'''),
]

LAW_OLD_START = '''    /// 🪪️ A codec call constructs exactly one app, chosen on the declarations:'''
LAW_OLD_END = '''    #[semio_framework_async_macros::async_test]
    async fn a_conflicting_declaration_leaves_zero_rows_behind() {'''
LAW_NEW = '''    /// 🪪️ A codec call constructs no app: every registered app records the document schema and the codec answers of
    /// its own type, each schema's owners are exactly its own editor and viewer (two of the fixture's six apps), the
    /// one answering is the editor, and a bundle whose every app factory refuses to run answers every codec call of
    /// every owned schema; a schema nobody owns is refused.
    #[semio_framework_async_macros::async_test]
    async fn codec_calls_construct_no_app() {
        fn refuse_construction(_definition: &AppDefinition) -> FixtureApps {
            panic!("a codec call constructed an app")
        }
        let projected = project_artifact_declarations(&[build_declaration()]);
        let mut plugin = crate::app::Plugin::<FixtureApps>::new("testkit", "Testkit", "1.0.0");
        for (app, mut factory) in projected.app_defs {
            assert_eq!(factory.document_schema, app.definition.io.artifact_schema, "{}", app.definition.id);
            factory.create = refuse_construction;
            plugin = plugin.register_app_factory(app, factory);
        }
        let owners: Vec<(String, &'static str)> = plugin.manifest.apps.iter().map(|definition| (definition.id.clone(), plugin.app_document_schema(&definition.id).expect("recorded schema"))).collect();
        assert_eq!(owners.len(), 6);
        let schemas = ["semio.testkit.w1c-fixture.std1-any/v1", "semio.testkit.w1c-fixture.std1-strict/v1", "semio.testkit.w1c-fixture.std2-any/v1"];
        for schema in schemas {
            let expected: Vec<&str> = owners.iter().filter(|(_, owned)| *owned == schema).map(|(id, _)| id.as_str()).collect();
            assert_eq!(expected.len(), 2, "{schema}: its editor and its viewer");
            let candidates: Vec<&str> = crate::plugin_runtime::artifact_codec_candidates(&plugin, schema).map(|definition| definition.id.as_str()).collect();
            assert_eq!(candidates, expected, "{schema}");
            let owner = crate::plugin_runtime::artifact_codec_owner(&plugin, schema).expect("one owner");
            assert_eq!(owner.role, AppRole::Editor, "{schema}: the editor is the creation authority");
        }
        assert!(crate::plugin_runtime::artifact_codec_owner(&plugin, "semio.testkit.nobody/v1").is_err(), "no owner is refused");
        let runtime = crate::plugin_runtime::PluginRuntime::new();
        crate::plugin_runtime::install_plugin_bundle(&runtime, plugin);
        for schema in schemas {
            let hash = crate::plugin_runtime::plugin_artifact_pack_schema_hash(&runtime, schema).await.expect("pack schema hash without an app");
            let document_id = format!("artifact-{}", "1".repeat(32));
            let pair = crate::plugin_runtime::plugin_artifact_genesis(&runtime, schema, &document_id).await.expect("genesis without an app");
            let mirror = crate::plugin_runtime::plugin_artifact_print_mirror(&runtime, schema, &pair.pack, &pair.spr).await.expect("print mirror without an app");
            assert!(hash != [0; 32] && !pair.pack.is_empty() && !mirror.dsl.is_empty(), "{schema}");
        }
        assert!(crate::plugin_runtime::plugin_artifact_pack_schema_hash(&runtime, "semio.testkit.nobody/v1").await.is_err(), "no owner is refused");
    }

'''

def apply():
    texts = {}
    problems = []
    def get(path):
        if path not in texts:
            texts[path] = path.read_text(encoding="utf-8")
        return texts[path]
    for path, old, new in EDITS:
        text = get(path)
        if text.count(old) != 1:
            problems.append(f"{path.name}: anchor count {text.count(old)}: {old[:70]!r}")
            continue
        texts[path] = text.replace(old, new, 1)
    text = get(PLUGIN)
    start, end = text.find(RUNTIME_START), text.find(RUNTIME_END)
    if start < 0 or end < 0 or end < start:
        problems.append("runtime region not found")
    else:
        texts[PLUGIN] = text[:start] + RUNTIME_NEW + text[end:]
    for old, new in CALLS:
        text = get(PLUGIN)
        if text.count(old) != 1:
            problems.append(f"call anchor count {text.count(old)}: {old[:70]!r}")
            continue
        texts[PLUGIN] = text.replace(old, new, 1)
    law = get(LAW)
    start, end = law.find(LAW_OLD_START), law.find(LAW_OLD_END)
    if start < 0 or end < 0:
        problems.append("law region not found")
    else:
        texts[LAW] = law[:start] + LAW_NEW + law[end:]
    if problems:
        print("\n".join(problems)); sys.exit(1)
    if DRY:
        print("dry run clean"); return
    for path, text in texts.items():
        path.write_text(text, encoding="utf-8")
    print("applied")

apply()
