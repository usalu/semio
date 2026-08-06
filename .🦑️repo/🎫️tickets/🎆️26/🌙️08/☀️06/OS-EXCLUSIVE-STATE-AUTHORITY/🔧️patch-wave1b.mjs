import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join } from "path";

function walk(d, pred, acc = []) {
  for (const n of readdirSync(d)) {
    if (["node_modules", "target", ".git"].includes(n)) continue;
    const p = join(d, n);
    try {
      const s = statSync(p);
      if (s.isDirectory()) walk(p, pred, acc);
      else if (pred(p)) acc.push(p);
    } catch {}
  }
  return acc;
}

const pluginRs = walk(".", (x) => x.endsWith("🔌️plugin/🦀️component.rs") && x.includes("🛍️products"))[0];
const hostRs = walk(".", (x) => x.includes("🔌️plugin/🖥️host/🦀️component.rs"))[0];
const wit = walk(".", (x) => x.includes("🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit"))[0];
console.log({ pluginRs, hostRs, wit });

function mustReplace(src, old, neu, label) {
  if (!src.includes(old)) throw new Error(`MISSING (${label}):\n${old.slice(0, 200)}`);
  const parts = src.split(old);
  if (parts.length !== 2) throw new Error(`COUNT ${parts.length - 1} for (${label})`);
  return parts.join(neu);
}

function replaceAllOnce(src, old, neu, label) {
  if (!src.includes(old)) throw new Error(`MISSING (${label})`);
  return src.split(old).join(neu);
}

// ========== PLUGIN COMPONENT ==========
let s = readFileSync(pluginRs, "utf8");

// 1) Import EngineHandles
s = mustReplace(
  s,
  "use store::{build_history_columns, create_config_envelope, create_document_envelope, ConfigStore, DocumentCommand, DocumentPack, DocumentStore, HistoryColumn, SpaceConflict};",
  "use store::{build_history_columns, create_config_envelope, create_document_envelope, ConfigStore, DocumentCommand, DocumentPack, DocumentStore, EngineHandles, HistoryColumn, SpaceConflict};",
  "engine import"
);

// 2) DraftView after ConfigView
s = mustReplace(
  s,
  `    pub struct ConfigView<'a, C> {
        pub projection: &'a C,
    }

    //#region 🔖️NoConfig`,
  `    pub struct ConfigView<'a, C> {
        pub projection: &'a C,
    }

    /// @emoji 📝️ Read-only view of an app's volatile draft projection — same role as {@link ConfigView}
    /// for the draft {@link store::DraftStore} (ephemeral; never checkpoints).
    pub struct DraftView<'a, D> {
        pub projection: &'a D,
    }

    //#region 🔖️NoConfig`,
  "DraftView"
);

// 3) NoDraft aliases after NoConfig region
s = mustReplace(
  s,
  `    }
    //#endregion 🔖️NoConfig

    //#region 🔖️CommandLog`,
  `    }
    //#endregion 🔖️NoConfig

    //#region 🔖️NoDraft
    /// @emoji 📝️ Default \`DocumentApp::Draft\` for apps with no draft lane yet.
    pub type NoDraft = NoConfig;
    /// @emoji 📝️ Default \`DocumentApp::DraftOperation\` twin of {@link NoDraft}.
    pub type NoDraftOperation = NoConfigOperation;
    //#endregion 🔖️NoDraft

    //#region 🔖️CommandLog`,
  "NoDraft"
);

// 4) Emit struct
s = mustReplace(
  s,
  `    pub struct Emit<Operation, ConfigOperation = NoConfigOperation> {
        pub document_operations: Vec<Operation>,
        pub config_operations: Vec<ConfigOperation>,
        pub description: Option<String>,
        pub coalesce_key: Option<String>,
        pub effects: Vec<HostEffect>,
        pub events: Vec<AppEvent>,
        /// 🐢️ Which rendered UI sections this action actually invalidates — \`Full\` (the default) preserves
        /// today's whole-shell-refresh behavior for every app that doesn't opt in to narrower scopes.
        pub ui_scope: semio_framework_core::kernel::UiDirtyScope,
    }

    impl<Operation, ConfigOperation> Default for Emit<Operation, ConfigOperation> {
        fn default() -> Self {
            Self { document_operations: Vec::new(), config_operations: Vec::new(), description: None, coalesce_key: None, effects: Vec::new(), events: Vec::new(), ui_scope: semio_framework_core::kernel::UiDirtyScope::default() }
        }
    }

    impl<Operation, ConfigOperation> Emit<Operation, ConfigOperation> {`,
  `    pub struct Emit<Operation, ConfigOperation = NoConfigOperation, DraftOperation = NoDraftOperation> {
        pub document_operations: Vec<Operation>,
        pub config_operations: Vec<ConfigOperation>,
        pub draft_operations: Vec<DraftOperation>,
        pub description: Option<String>,
        pub coalesce_key: Option<String>,
        pub effects: Vec<HostEffect>,
        pub events: Vec<AppEvent>,
        /// 🐢️ Which rendered UI sections this action actually invalidates — \`Full\` (the default) preserves
        /// today's whole-shell-refresh behavior for every app that doesn't opt in to narrower scopes.
        pub ui_scope: semio_framework_core::kernel::UiDirtyScope,
    }

    impl<Operation, ConfigOperation, DraftOperation> Default for Emit<Operation, ConfigOperation, DraftOperation> {
        fn default() -> Self {
            Self { document_operations: Vec::new(), config_operations: Vec::new(), draft_operations: Vec::new(), description: None, coalesce_key: None, effects: Vec::new(), events: Vec::new(), ui_scope: semio_framework_core::kernel::UiDirtyScope::default() }
        }
    }

    impl<Operation, ConfigOperation, DraftOperation> Emit<Operation, ConfigOperation, DraftOperation> {`,
  "Emit"
);

// 5) Emit::draft helper after Emit::config
s = mustReplace(
  s,
  `        pub fn config(config_operations: Vec<ConfigOperation>) -> Self {
            Self { config_operations, ..Default::default() }
        }

        /// @emoji 🔁️ \`amend\`'s CONFIG-targeted twin`,
  `        pub fn config(config_operations: Vec<ConfigOperation>) -> Self {
            Self { config_operations, ..Default::default() }
        }

        /// @emoji 📝️ A draft-operation emission carrying \`draft_operations\` and nothing else.
        pub fn draft(draft_operations: Vec<DraftOperation>) -> Self {
            Self { draft_operations, ..Default::default() }
        }

        /// @emoji 🔁️ \`amend\`'s CONFIG-targeted twin`,
  "Emit::draft"
);

// 6) DocumentApp trait — associated types + consts + handle
s = mustReplace(
  s,
  `    pub trait DocumentApp: Send + 'static {
        type Projection: Clone + PartialEq + Serialize + DeserializeOwned + Send + store::DocumentDsl + DocumentPack;
        type Operation: ::protocol::Operation<Self::Projection> + PartialEq + Send + ::protocol::OpText + ::protocol::OpBinary;
        type Config: Clone + Default + PartialEq + Serialize + DeserializeOwned + Send + store::ConfigRecord + DocumentPack;
        type ConfigOperation: ::protocol::Operation<Self::Config> + PartialEq + Send + ::protocol::OpText + ::protocol::OpBinary;
        /// @emoji 🎯️ B1: this app's closed, typed command enum — the SOLE dispatch surface for
        /// \`handle\` below, replacing the deleted stringly-typed \`handle_action\`/\`handle_command\`/
        /// \`handle_typed_command\` trio. Decoded off the wire once, by \`VcsDocumentApp::dispatch_typed_command\`,
        /// via \`OpBinary::decode_op\`; framework-reserved verbs (undo/redo/checkpoint/alternative/clipboard/
        /// revert/history-filter/noteShellCommand) never reach here — the wrapper intercepts those itself
        /// (see \`VcsDocumentApp::dispatch_framework_action\`) since they are host mechanics, not app behavior.
        type Command: ::protocol::OpBinary + Send;

        fn app_id(&self) -> &str;
        fn document_schema(&self) -> &str;
        fn config_schema(&self) -> &str {
            "config.empty"
        }
        fn initial_projection(&self) -> Self::Projection;
        fn initial_config(&self) -> Self::Config {
            Self::Config::default()
        }
        /// @emoji 🧩️ B1: the pure heart of the app — a total, side-effect-free function from
        /// \`(command, document, config)\` to an {@link Emit}. No \`&mut self\`, no \`ViewState\` (ephemeral
        /// per-window/locale/selection state now lives in \`Self::Config\`, keyed by window-instance id where
        /// it varies per window). \`View\`-kind interactions from the pre-B1 world (camera/selection/hover/…)
        /// emit \`config_operations\` here instead of mutating an app-struct field — the config store computes
        /// their real \`backwards\`, so undo/redo works without any ad hoc \`InverseAction\`.
        fn handle(&self, command: &Self::Command, doc: &DocumentView<'_, Self::Projection>, cfg: &ConfigView<'_, Self::Config>) -> Result<Emit<Self::Operation, Self::ConfigOperation>, Fault>;`,
  `    pub trait DocumentApp: Send + 'static {
        /// @emoji 🪪 Stable app id — prefer this over \`app_id(&self)\` on the path to receiverless ZSTs.
        const APP_ID: &'static str;
        /// @emoji 📜️ Stable document schema id — prefer this over \`document_schema(&self)\`.
        const DOCUMENT_SCHEMA: &'static str;
        type Projection: Clone + PartialEq + Serialize + DeserializeOwned + Send + store::DocumentDsl + DocumentPack;
        type Operation: ::protocol::Operation<Self::Projection> + PartialEq + Send + ::protocol::OpText + ::protocol::OpBinary;
        type Config: Clone + Default + PartialEq + Serialize + DeserializeOwned + Send + store::ConfigRecord + DocumentPack;
        type ConfigOperation: ::protocol::Operation<Self::Config> + PartialEq + Send + ::protocol::OpText + ::protocol::OpBinary;
        /// @emoji 📝️ Volatile draft projection — defaults to {@link NoDraft} for apps without a draft lane.
        type Draft: Clone + Default + PartialEq + Serialize + DeserializeOwned + Send + store::DocumentDsl + DocumentPack;
        /// @emoji 📝️ Draft-lane operations applied to {@link store::DraftStore}.
        type DraftOperation: ::protocol::Operation<Self::Draft> + PartialEq + Send + ::protocol::OpText + ::protocol::OpBinary;
        /// @emoji 🎯️ B1: this app's closed, typed command enum — the SOLE dispatch surface for
        /// \`handle\` below, replacing the deleted stringly-typed \`handle_action\`/\`handle_command\`/
        /// \`handle_typed_command\` trio. Decoded off the wire once, by \`VcsDocumentApp::dispatch_typed_command\`,
        /// via \`OpBinary::decode_op\`; framework-reserved verbs (undo/redo/checkpoint/alternative/clipboard/
        /// revert/history-filter/noteShellCommand) never reach here — the wrapper intercepts those itself
        /// (see \`VcsDocumentApp::dispatch_framework_action\`) since they are host mechanics, not app behavior.
        type Command: ::protocol::OpBinary + Send;

        fn app_id(&self) -> &str {
            Self::APP_ID
        }
        fn document_schema(&self) -> &str {
            Self::DOCUMENT_SCHEMA
        }
        fn config_schema(&self) -> &str {
            "config.empty"
        }
        fn initial_projection(&self) -> Self::Projection;
        fn initial_config(&self) -> Self::Config {
            Self::Config::default()
        }
        fn initial_draft(&self) -> Self::Draft {
            Self::Draft::default()
        }
        /// @emoji 🧩️ B1: the pure heart of the app — a total, side-effect-free function from
        /// \`(command, document, config, draft, engines)\` to an {@link Emit}. No \`&mut self\`, no \`ViewState\`
        /// (ephemeral per-window/locale/selection state lives in \`Self::Config\` or \`Self::Draft\`).
        /// \`View\`-kind interactions emit \`config_operations\` / \`draft_operations\` here — stores compute
        /// real \`backwards\`. \`engines\` is the host-owned {@link EngineHandles} bag (empty until WIT
        /// engine-derive/read is wired through exchange).
        fn handle(
            &self,
            command: &Self::Command,
            doc: &DocumentView<'_, Self::Projection>,
            cfg: &ConfigView<'_, Self::Config>,
            draft: &DraftView<'_, Self::Draft>,
            engines: &EngineHandles,
        ) -> Result<Emit<Self::Operation, Self::ConfigOperation, Self::DraftOperation>, Fault>;`,
  "DocumentApp trait"
);

// 7) VcsDocumentApp struct — add draft_store
s = mustReplace(
  s,
  `    pub struct VcsDocumentApp<A: DocumentApp> {
        app: A,
        store: DocumentStore<A::Projection, A::Operation>,
        config_store: ConfigStore<A::Config, A::ConfigOperation>,
        /// @emoji 🗂️ Keyed on \`(store.generation(), log_generation, history_filter)\` — any of the three
        /// changing invalidates the cached projection/\`HistoryView\` pair.
        cache: Option<((u64, u64, u64, HistoryCommandFilter), A::Projection, A::Config, HistoryView)>,
        registry: AppActionRegistry,
        /// @emoji 🧾️ Append-only session command log — see \`🔖️CommandLog\`. Never persisted, never
        /// truncated: undo/redo/revert push entries, they never remove any.
        command_log: Vec<CommandLogEntry>,
        next_command_seq: u64,
        /// @emoji 🗂️ Bumped by \`push_log_entry\`/\`record_command\` on every log mutation (a push OR a fold)
        /// — part of the cache key so a folded ×count bump alone (no store-generation change) still
        /// invalidates a stale render.
        log_generation: u64,
        history_filter: HistoryCommandFilter,
    }`,
  `    pub struct VcsDocumentApp<A: DocumentApp> {
        app: A,
        store: DocumentStore<A::Projection, A::Operation>,
        config_store: ConfigStore<A::Config, A::ConfigOperation>,
        /// @emoji 📝️ Volatile draft lane — never checkpoints; prune via \`DocumentCommand::PruneDrafts\`.
        /// Moves to host {@link DocumentSession} when CHANNEL_VERSION 5 exchange lands.
        draft_store: store::DraftStore<A::Draft, A::DraftOperation>,
        /// @emoji 🗂️ Keyed on \`(store.generation(), log_generation, history_filter)\` — any of the three
        /// changing invalidates the cached projection/\`HistoryView\` pair.
        cache: Option<((u64, u64, u64, HistoryCommandFilter), A::Projection, A::Config, HistoryView)>,
        registry: AppActionRegistry,
        /// @emoji 🧾️ Append-only session command log — see \`🔖️CommandLog\`. Never persisted, never
        /// truncated: undo/redo/revert push entries, they never remove any.
        command_log: Vec<CommandLogEntry>,
        next_command_seq: u64,
        /// @emoji 🗂️ Bumped by \`push_log_entry\`/\`record_command\` on every log mutation (a push OR a fold)
        /// — part of the cache key so a folded ×count bump alone (no store-generation change) still
        /// invalidates a stale render.
        log_generation: u64,
        history_filter: HistoryCommandFilter,
    }`,
  "VcsDocumentApp draft_store"
);

// 8) with_registry construction
s = mustReplace(
  s,
  `            let envelope = create_document_envelope::<A::Projection, A::Operation>(app.document_schema(), app.app_id(), app.initial_projection(), None);
            let config_id = format!("{}-config", app.app_id());
            let config_envelope = create_config_envelope::<A::Config, A::ConfigOperation>(app.config_schema(), &config_id, app.initial_config(), None);
            let mut store = DocumentStore::new(envelope);
            let config_store = ConfigStore::new(config_envelope);
            app.seed(&mut store);
            Self { app, store, config_store, cache: None, registry, command_log: Vec::new(), next_command_seq: 0, log_generation: 0, history_filter: HistoryCommandFilter::default() }
        }`,
  `            let envelope = create_document_envelope::<A::Projection, A::Operation>(app.document_schema(), app.app_id(), app.initial_projection(), None);
            let config_id = format!("{}-config", app.app_id());
            let config_envelope = create_config_envelope::<A::Config, A::ConfigOperation>(app.config_schema(), &config_id, app.initial_config(), None);
            let draft_id = format!("{}-draft", app.app_id());
            let draft_envelope = create_document_envelope::<A::Draft, A::DraftOperation>("draft.empty", &draft_id, app.initial_draft(), None);
            let mut store = DocumentStore::new(envelope);
            let config_store = ConfigStore::new(config_envelope);
            let draft_store = store::DraftStore::new(draft_envelope);
            app.seed(&mut store);
            Self { app, store, config_store, draft_store, cache: None, registry, command_log: Vec::new(), next_command_seq: 0, log_generation: 0, history_filter: HistoryCommandFilter::default() }
        }`,
  "with_registry draft"
);

// 9) dispatch_emit signature + destructure + draft apply
s = mustReplace(
  s,
  `        fn dispatch_emit(&mut self, verb: &str, emit: Emit<A::Operation, A::ConfigOperation>, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            let Emit { document_operations, config_operations, description, coalesce_key, effects, events, ui_scope } = emit;

            // 🧮️ Config side dispatches first, independent of whether this verb ALSO touches the document
            // — captures the resulting (possibly amended) config edit id for the command-log row below.
            let mut config_edit_id: Option<String> = None;
            if !config_operations.is_empty() {
                self.config_store.set_local_actor_id(Some(meta.actor.clone()));
                let before_config_edit_id = self.config_store.envelope().vcs.edits.last().map(|edit| edit.id.clone());
                let config_command = match &coalesce_key {
                    Some(key) => DocumentCommand::AmendLast { operations: config_operations, coalesce_key: Some(format!("config:{key}")) },
                    None => DocumentCommand::Apply { operations: config_operations, description: description.clone() },
                };
                self.config_store.dispatch(config_command).map_err(|error| error.into_fault())?;
                self.cache = None;
                let amended_same_config_edit = before_config_edit_id.is_some() && self.config_store.envelope().vcs.edits.last().map(|edit| &edit.id) == before_config_edit_id.as_ref();
                config_edit_id = if amended_same_config_edit { before_config_edit_id } else { self.config_store.envelope().vcs.edits.last().map(|edit| edit.id.clone()) };
            }`,
  `        fn dispatch_emit(&mut self, verb: &str, emit: Emit<A::Operation, A::ConfigOperation, A::DraftOperation>, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            let Emit { document_operations, config_operations, draft_operations, description, coalesce_key, effects, events, ui_scope } = emit;

            // 📝️ Draft lane — ephemeral; applied without command-log rows (never checkpoints).
            if !draft_operations.is_empty() {
                self.draft_store.set_local_actor_id(Some(meta.actor.clone()));
                self.draft_store
                    .dispatch(DocumentCommand::Apply { operations: draft_operations, description: None })
                    .map_err(|error| error.into_fault())?;
            }

            // 🧮️ Config side dispatches first, independent of whether this verb ALSO touches the document
            // — captures the resulting (possibly amended) config edit id for the command-log row below.
            let mut config_edit_id: Option<String> = None;
            if !config_operations.is_empty() {
                self.config_store.set_local_actor_id(Some(meta.actor.clone()));
                let before_config_edit_id = self.config_store.envelope().vcs.edits.last().map(|edit| edit.id.clone());
                let config_command = match &coalesce_key {
                    Some(key) => DocumentCommand::AmendLast { operations: config_operations, coalesce_key: Some(format!("config:{key}")) },
                    None => DocumentCommand::Apply { operations: config_operations, description: description.clone() },
                };
                self.config_store.dispatch(config_command).map_err(|error| error.into_fault())?;
                self.cache = None;
                let amended_same_config_edit = before_config_edit_id.is_some() && self.config_store.envelope().vcs.edits.last().map(|edit| &edit.id) == before_config_edit_id.as_ref();
                config_edit_id = if amended_same_config_edit { before_config_edit_id } else { self.config_store.envelope().vcs.edits.last().map(|edit| edit.id.clone()) };
            }`,
  "dispatch_emit draft"
);

// 10) dispatch_typed_command_inner — call handle with draft + engines
s = mustReplace(
  s,
  `        fn dispatch_typed_command_inner(&mut self, command: A::Command, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            self.refresh_cache()?;
                let (verb, emit) = {
                let VcsDocumentApp { app, cache, .. } = self;
                let (_, projection, config, history) = cache.as_ref().expect("cache refreshed above");
                let doc = DocumentView { projection, history };
                let cfg = ConfigView { projection: config };
                let verb = app.command_id(&command).to_string();
                let emit = app.handle(&command, &doc, &cfg)?;
                (verb, emit)
            };`,
  `        fn dispatch_typed_command_inner(&mut self, command: A::Command, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            self.refresh_cache()?;
            let draft_projection = self.draft_store.projection().map_err(|error| error.into_fault())?;
                let (verb, emit) = {
                let VcsDocumentApp { app, cache, .. } = self;
                let (_, projection, config, history) = cache.as_ref().expect("cache refreshed above");
                let doc = DocumentView { projection, history };
                let cfg = ConfigView { projection: config };
                let draft = DraftView { projection: &draft_projection };
                let engines = EngineHandles::empty();
                let verb = app.command_id(&command).to_string();
                let emit = app.handle(&command, &doc, &cfg, &draft, &engines)?;
                (verb, emit)
            };`,
  "handle call site"
);

// 11) register_document_app_zst after register_document_app
s = mustReplace(
  s,
  `        pub fn register_document_app<A>(self, app: App, factory: impl Fn() -> A + Send + 'static) -> Self
        where
            A: DocumentApp,
        {
            let registry = AppActionRegistry::from_definition(&app.definition);
            self.register_app(app, move || Box::new(VcsDocumentApp::with_registry(factory(), registry.clone())))
        }

        pub fn create_app(&self, app_id: &str) -> Option<Box<dyn PluginApp>> {`,
  `        pub fn register_document_app<A>(self, app: App, factory: impl Fn() -> A + Send + 'static) -> Self
        where
            A: DocumentApp,
        {
            let registry = AppActionRegistry::from_definition(&app.definition);
            self.register_app(app, move || Box::new(VcsDocumentApp::with_registry(factory(), registry.clone())))
        }

        /// @emoji 🧬️ Turbofish ZST registration — \`A: Default\` constructs the (preferably zero-sized) app
        /// type without a factory closure. Preferred entry for receiverless apps; \`semio_plugin!\` uses this.
        pub fn register_document_app_zst<A: DocumentApp + Default>(self, app: App) -> Self {
            self.register_document_app(app, || A::default())
        }

        pub fn create_app(&self, app_id: &str) -> Option<Box<dyn PluginApp>> {`,
  "register_document_app_zst"
);

// 12) semio_plugin! macro
s = mustReplace(
  s,
  `                $( .register_document_app(($app_fn)(), || <$app_ty as ::std::default::Default>::default()) )+`,
  `                $( .register_document_app_zst::<$app_ty>(($app_fn)()) )+`,
  "semio_plugin macro"
);

// 13) DummyApp
s = mustReplace(
  s,
  `            impl DocumentApp for DummyApp {
                type Projection = DummyProjection;
                type Operation = DummyOperation;
                type Config = NoConfig;
                type ConfigOperation = NoConfigOperation;
                type Command = DummyCommand;

                fn app_id(&self) -> &str {
                    "testkit-dummy"
                }

                fn document_schema(&self) -> &str {
                    "semio.testkit/v1"
                }

                fn initial_projection(&self) -> DummyProjection {
                    DummyProjection::default()
                }

                fn handle(&self, command: &DummyCommand, doc: &DocumentView<'_, DummyProjection>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<DummyOperation>, Fault> {
                    match command {
                        DummyCommand::Increment => Ok(Emit { document_operations: vec![DummyOperation::SetCount { value: doc.projection.count + 1 }], description: Some("increment".into()), ..Default::default() }),
                    }
                }`,
  `            impl DocumentApp for DummyApp {
                const APP_ID: &'static str = "testkit-dummy";
                const DOCUMENT_SCHEMA: &'static str = "semio.testkit/v1";
                type Projection = DummyProjection;
                type Operation = DummyOperation;
                type Config = NoConfig;
                type ConfigOperation = NoConfigOperation;
                type Draft = NoDraft;
                type DraftOperation = NoDraftOperation;
                type Command = DummyCommand;

                fn initial_projection(&self) -> DummyProjection {
                    DummyProjection::default()
                }

                fn handle(&self, command: &DummyCommand, doc: &DocumentView<'_, DummyProjection>, _cfg: &ConfigView<'_, NoConfig>, _draft: &DraftView<'_, NoDraft>, _engines: &EngineHandles) -> Result<Emit<DummyOperation>, Fault> {
                    match command {
                        DummyCommand::Increment => Ok(Emit { document_operations: vec![DummyOperation::SetCount { value: doc.projection.count + 1 }], description: Some("increment".into()), ..Default::default() }),
                    }
                }`,
  "DummyApp"
);

// Need NoDraft in DummyApp scope — check imports in that module
s = mustReplace(
  s,
  `            use super::super::{ConfigView, DocumentView, Emit, NoConfig, NoConfigOperation};`,
  `            use super::super::{ConfigView, DocumentView, DraftView, Emit, EngineHandles, NoConfig, NoConfigOperation, NoDraft, NoDraftOperation};`,
  "DummyApp imports"
);

// 14) TestApp
s = mustReplace(
  s,
  `        impl DocumentApp for TestApp {
            type Projection = TestProjection;
            type Operation = TestOperation;
            type Config = TestConfig;
            type ConfigOperation = TestConfigOperation;
            type Command = TestCommand;

            fn app_id(&self) -> &str {
                "synthetic-play"
            }

            fn document_schema(&self) -> &str {
                "semio.test/v1"
            }

            fn initial_projection(&self) -> TestProjection {
                TestProjection::default()
            }`,
  `        impl DocumentApp for TestApp {
            const APP_ID: &'static str = "synthetic-play";
            const DOCUMENT_SCHEMA: &'static str = "semio.test/v1";
            type Projection = TestProjection;
            type Operation = TestOperation;
            type Config = TestConfig;
            type ConfigOperation = TestConfigOperation;
            type Draft = crate::NoDraft;
            type DraftOperation = crate::NoDraftOperation;
            type Command = TestCommand;

            fn initial_projection(&self) -> TestProjection {
                TestProjection::default()
            }`,
  "TestApp types"
);

s = mustReplace(
  s,
  `            fn handle(&self, command: &TestCommand, doc: &DocumentView<'_, TestProjection>, _cfg: &ConfigView<'_, TestConfig>) -> Result<Emit<TestOperation, TestConfigOperation>, Fault> {`,
  `            fn handle(&self, command: &TestCommand, doc: &DocumentView<'_, TestProjection>, _cfg: &ConfigView<'_, TestConfig>, _draft: &crate::DraftView<'_, crate::NoDraft>, _engines: &store::EngineHandles) -> Result<Emit<TestOperation, TestConfigOperation>, Fault> {`,
  "TestApp handle"
);

writeFileSync(pluginRs, s);
console.log("patched plugin component");

// ========== WORLD.WIT ==========
let w = readFileSync(wit, "utf8");
w = mustReplace(
  w,
  `interface host {
  log: func(level: string, message: string);
  now-ms: func() -> s64;
  /// 📦️ \`read-document\`/\`write-document\` payloads are \`store::pack_rt::encode_wire_value\` bytes.
  read-document: func(handle: u64) -> result<list<u8>, list<u8>>;
  write-document: func(handle: u64, payload: list<u8>) -> result<_, list<u8>>;
  /// 📦️ \`params\` is a wire-encoded \`DslValue\` (\`encode_wire_value\`).
  open-window: func(kind: string, params: list<u8>) -> result<u64, list<u8>>;
  /// 📦️ \`invocation\`/\`result\` are wire-encoded \`DslValue\` blobs.
  invoke-action: func(target: string, invocation: list<u8>) -> result<list<u8>, list<u8>>;
  read-asset: func(handle: u64) -> result<list<u8>, list<u8>>;
  network-fetch: func(origin: string, path: string) -> result<list<u8>, list<u8>>;
  write-blob: func(data: list<u8>, media-type: string) -> result<string, list<u8>>;
  read-blob: func(hash: string) -> result<list<u8>, list<u8>>;
  /// 📦️ \`message\` is a \`store::encode_backbone_message\` blob (a binary \`BackboneMessage\`).
  backbone-send: func(uri: string, message: list<u8>) -> result<_, list<u8>>;
  backbone-poll: func(uri: string) -> result<list<list<u8>>, list<u8>>;
  backbone-status: func(uri: string) -> result<string, list<u8>>;
}

world plugin-world {
  import host;
  export plugin;
}`,
  `interface host {
  log: func(level: string, message: string);
  now-ms: func() -> s64;
  /// 📦️ \`read-document\`/\`write-document\` payloads are \`store::pack_rt::encode_wire_value\` bytes.
  read-document: func(handle: u64) -> result<list<u8>, list<u8>>;
  write-document: func(handle: u64, payload: list<u8>) -> result<_, list<u8>>;
  /// 📦️ \`params\` is a wire-encoded \`DslValue\` (\`encode_wire_value\`).
  open-window: func(kind: string, params: list<u8>) -> result<u64, list<u8>>;
  /// 📦️ \`invocation\`/\`result\` are wire-encoded \`DslValue\` blobs.
  invoke-action: func(target: string, invocation: list<u8>) -> result<list<u8>, list<u8>>;
  read-asset: func(handle: u64) -> result<list<u8>, list<u8>>;
  network-fetch: func(origin: string, path: string) -> result<list<u8>, list<u8>>;
  write-blob: func(data: list<u8>, media-type: string) -> result<string, list<u8>>;
  read-blob: func(hash: string) -> result<list<u8>, list<u8>>;
  /// 📦️ \`message\` is a \`store::encode_backbone_message\` blob (a binary \`BackboneMessage\`).
  backbone-send: func(uri: string, message: list<u8>) -> result<_, list<u8>>;
  backbone-poll: func(uri: string) -> result<list<list<u8>>, list<u8>>;
  backbone-status: func(uri: string) -> result<string, list<u8>>;
}

/// ⚙️ Host-owned content-addressed engine derive — gated by \`ArtifactKind::Engine\` + Invoke.
interface engine-derive {
  /// 🧮 Derive (or cache-hit) a handle for \`(engine-id, input)\`. Ok bytes encode \`EngineHandle\` as
  /// \`store::pack_rt::encode_wire_value\` of \`{ key: list<u8>, engineId: string }\`; Err is fault bytes.
  derive: func(engine-id: string, input: list<u8>) -> result<list<u8>, list<u8>>;
}

/// ⚙️ Host-owned engine result read — gated by \`ArtifactKind::Engine\` + Read.
interface engine-read {
  /// 📖 Read cached output for a previously derived handle. \`handle\` is the same wire encoding as
  /// \`engine-derive.derive\`'s Ok payload.
  read: func(handle: list<u8>) -> result<list<u8>, list<u8>>;
}

world plugin-world {
  import host;
  import engine-derive;
  import engine-read;
  export plugin;
}`,
  "world.wit"
);
writeFileSync(wit, w);
console.log("patched world.wit");

// ========== HOST ==========
let h = readFileSync(hostRs, "utf8");

// Fix bindgen path to consolidated wit location
h = mustReplace(
  h,
  `bindgen!({
    world: "plugin-world",
    path: "../../../⚡️implementations/🦀️rust/📜️wit",
    async: false,
});`,
  `bindgen!({
    world: "plugin-world",
    path: "../../../📦️packages/🦀️rust/📜️wit",
    async: false,
});`,
  "bindgen path"
);

// DocumentSession + EngineCache on HostState
h = mustReplace(
  h,
  `//#region 🔖️HostState
struct HostState {
    wasi: WasiCtx,
    table: ResourceTable,
    granted_capabilities: Vec<CapabilityRequirement>,
    plugin_id: String,
    backbones: HashMap<String, Box<dyn store::Backbone>>,
    /// @emoji 📦️ Backing store for \`write-blob\`/\`read-blob\`, injected via
    /// {@link WasmPluginRuntime::register_host_blob_store} — \`None\` until a caller registers one
    /// (mirrors \`backbones\`' explicit-registration convention, not a stub-forever like \`read-asset\`).
    blob_store: Option<Arc<dyn store::BlobStore>>,
}`,
  `//#region 🔖️DocumentSession
/// @emoji 🗂️ Host-owned per-instance document authority. Stores move here fully when CHANNEL_VERSION 5
/// exchange lands (guest becomes pure \`handle\`/\`render\` over packs). Today the typed
/// \`DocumentStore\`/\`ConfigStore\`/\`DraftStore\` still live in guest \`VcsDocumentApp\`; this session owns
/// the host engine cache and a placeholder command log so the ownership seam exists in-tree.
pub struct DocumentSession {
    pub engine_cache: store::EngineCache,
    /// @emoji 🧾️ Session command-log placeholder — typed entries remain guest-side until the channel flip.
    pub command_log: Vec<()>,
}

impl DocumentSession {
    /// @emoji 🏗️ Empty session with a default engine-cache budget (64 MiB).
    pub fn new() -> Self {
        Self { engine_cache: store::EngineCache::new(64 * 1024 * 1024), command_log: Vec::new() }
    }
}

impl Default for DocumentSession {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 🔖️DocumentSession

//#region 🔖️HostState
struct HostState {
    wasi: WasiCtx,
    table: ResourceTable,
    granted_capabilities: Vec<CapabilityRequirement>,
    plugin_id: String,
    backbones: HashMap<String, Box<dyn store::Backbone>>,
    /// @emoji 📦️ Backing store for \`write-blob\`/\`read-blob\`, injected via
    /// {@link WasmPluginRuntime::register_host_blob_store} — \`None\` until a caller registers one
    /// (mirrors \`backbones\`' explicit-registration convention, not a stub-forever like \`read-asset\`).
    blob_store: Option<Arc<dyn store::BlobStore>>,
    /// @emoji 🗂️ Host-authoritative session bag (engine cache today; stores after CHANNEL 5).
    document_session: DocumentSession,
}`,
  "HostState DocumentSession"
);

// has_engine_access helper + find HostState::has_backbone_access to add after
h = mustReplace(
  h,
  `    fn has_backbone_access(&self, rights: Rights) -> bool {
        self.granted_capabilities.iter().any(|cap| cap.artifact == ArtifactKind::Backbone && cap.rights == rights && matches!(cap.scope, Scope::Plugin | Scope::Global))
    }`,
  `    fn has_backbone_access(&self, rights: Rights) -> bool {
        self.granted_capabilities.iter().any(|cap| cap.artifact == ArtifactKind::Backbone && cap.rights == rights && matches!(cap.scope, Scope::Plugin | Scope::Global))
    }

    fn has_engine_access(&self, rights: Rights) -> bool {
        self.granted_capabilities.iter().any(|cap| cap.artifact == ArtifactKind::Engine && cap.rights == rights && matches!(cap.scope, Scope::Plugin | Scope::Global))
    }`,
  "has_engine_access"
);

// Find host_state constructor and add document_session
h = mustReplace(
  h,
  `            blob_store: None,
        }`,
  `            blob_store: None,
            document_session: DocumentSession::new(),
        }`,
  "host_state init"
);

// After backbone_status Host impl closing, add engine-derive and engine-read impls
h = mustReplace(
  h,
  `    fn backbone_status(&mut self, uri: String) -> Result<String, Vec<u8>> {
        Ok(if self.backbones.contains_key(&uri) { "attached".into() } else { "detached".into() })
    }
}
//#endregion 🔖️HostState`,
  `    fn backbone_status(&mut self, uri: String) -> Result<String, Vec<u8>> {
        Ok(if self.backbones.contains_key(&uri) { "attached".into() } else { "detached".into() })
    }
}

impl semio::framework::engine_derive::Host for HostState {
    fn derive(&mut self, engine_id: String, input: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        if !self.has_engine_access(Rights::Invoke) {
            return Err(host_fault_bytes("os.host.engine-derive", "engine invoke capability missing"));
        }
        let handle = self
            .document_session
            .engine_cache
            .derive(&engine_id, &input)
            .map_err(|error| host_fault_bytes("os.host.engine-derive", error.to_string()))?;
        let value = dsl::DslValue::Record(vec![
            ("key".into(), dsl::DslValue::Bytes(handle.key.0.to_vec())),
            ("engineId".into(), dsl::DslValue::String(handle.engine_id)),
        ]);
        Ok(store::pack_rt::encode_wire_value(&value))
    }
}

impl semio::framework::engine_read::Host for HostState {
    fn read(&mut self, handle: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        if !self.has_engine_access(Rights::Read) {
            return Err(host_fault_bytes("os.host.engine-read", "engine read capability missing"));
        }
        let value = store::pack_rt::decode_wire_value(&handle).map_err(|error| host_fault_bytes("os.host.engine-read", error.to_string()))?;
        let key_bytes = value
            .get("key")
            .and_then(dsl::DslValue::as_bytes)
            .ok_or_else(|| host_fault_bytes("os.host.engine-read", "handle missing key bytes"))?;
        let engine_id = value
            .get("engineId")
            .and_then(dsl::DslValue::as_str)
            .ok_or_else(|| host_fault_bytes("os.host.engine-read", "handle missing engineId"))?
            .to_string();
        if key_bytes.len() != 32 {
            return Err(host_fault_bytes("os.host.engine-read", "handle key must be 32 bytes"));
        }
        let mut key = [0u8; 32];
        key.copy_from_slice(key_bytes);
        let engine_handle = store::EngineHandle {
            key: store::EngineKey(key),
            engine_id,
        };
        self.document_session
            .engine_cache
            .read(&engine_handle)
            .map_err(|error| host_fault_bytes("os.host.engine-read", error.to_string()))
    }
}
//#endregion 🔖️HostState`,
  "engine host impls"
);

writeFileSync(hostRs, h);
console.log("patched host");
console.log("DONE");
