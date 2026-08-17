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

function mustReplace(src, old, neu, label) {
  if (!src.includes(old)) throw new Error(`MISSING (${label}):\n${old.slice(0, 240)}`);
  const parts = src.split(old);
  if (parts.length !== 2) throw new Error(`COUNT ${parts.length - 1} for (${label})`);
  return parts.join(neu);
}

const pluginRs = walk(".", (x) => x.endsWith("🔌️plugin/🦀️component.rs") && x.includes("🛍️products"))[0];
const hostRs = walk(".", (x) => x.includes("🔌️plugin/🖥️host/🦀️component.rs"))[0];
console.log({ pluginRs, hostRs });

let s = readFileSync(pluginRs, "utf8");

s = mustReplace(
  s,
  "use store::{build_history_columns, create_config_envelope, create_document_envelope, ConfigStore, DocumentCommand, DocumentPack, DocumentStore, HistoryColumn, SpaceConflict};",
  "use store::{build_history_columns, create_config_envelope, create_document_envelope, ConfigStore, DocumentCommand, DocumentPack, DocumentStore, EngineHandles, HistoryColumn, SpaceConflict};",
  "engine import"
);

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
        /// @emoji 📝️ Volatile draft projection — use {@link NoDraft} when the app has no draft lane.
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
        /// \`(command, document, config, draft, engines)\` to an {@link Emit}. No \`&mut self\`.
        /// \`engines\` is the host-owned {@link EngineHandles} bag (empty until WIT engine-derive/read
        /// is threaded through exchange).
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
        /// Moves to host \`DocumentSession\` when CHANNEL_VERSION 5 exchange lands.
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

s = mustReplace(
  s,
  `                $( .register_document_app(($app_fn)(), || <$app_ty as ::std::default::Default>::default()) )+`,
  `                $( .register_document_app_zst::<$app_ty>(($app_fn)()) )+`,
  "semio_plugin macro"
);

s = mustReplace(
  s,
  `            use super::super::{ConfigView, NoConfig, NoConfigOperation};
            use super::*;
            use crate::app::{DocumentView, Emit};`,
  `            use super::super::{ConfigView, DraftView, NoConfig, NoConfigOperation, NoDraft, NoDraftOperation};
            use super::*;
            use crate::app::{DocumentView, Emit};
            use store::EngineHandles;`,
  "DummyApp imports"
);

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
            type Draft = NoDraft;
            type DraftOperation = NoDraftOperation;
            type Command = TestCommand;

            fn initial_projection(&self) -> TestProjection {
                TestProjection::default()
            }`,
  "TestApp types"
);

s = mustReplace(
  s,
  `            fn handle(&self, command: &TestCommand, doc: &DocumentView<'_, TestProjection>, _cfg: &ConfigView<'_, TestConfig>) -> Result<Emit<TestOperation, TestConfigOperation>, Fault> {`,
  `            fn handle(&self, command: &TestCommand, doc: &DocumentView<'_, TestProjection>, _cfg: &ConfigView<'_, TestConfig>, _draft: &DraftView<'_, NoDraft>, _engines: &EngineHandles) -> Result<Emit<TestOperation, TestConfigOperation>, Fault> {`,
  "TestApp handle"
);

// TestApp module imports — add DraftView, NoDraft, EngineHandles if needed
s = mustReplace(
  s,
  `        use crate::app::{ui_history_panel, ActionMeta, App, AppActionRegistry, CommandView, ConfigView, DocumentApp, DocumentView, Emit, HistoryCommandFilter, HistoryView, Menu, PluginApp, VcsDocumentApp};`,
  `        use crate::app::{ui_history_panel, ActionMeta, App, AppActionRegistry, CommandView, ConfigView, DocumentApp, DocumentView, DraftView, Emit, HistoryCommandFilter, HistoryView, Menu, NoDraft, NoDraftOperation, PluginApp, VcsDocumentApp};
        use store::EngineHandles;`,
  "TestApp imports"
);

s = mustReplace(
  s,
  `    MediaClass, MediaType, Menu, ModeSpec, NoConfig, NoConfigOperation, NodeGraphDeleteDispatch, OsMediaCapability, PanelTabSpec, PanelTreeBuilder, Plugin, PluginApp, PluginBundle, VcsDocumentApp, WindowKindSpec,`,
  `    MediaClass, MediaType, Menu, ModeSpec, NoConfig, NoConfigOperation, NoDraft, NoDraftOperation, NodeGraphDeleteDispatch, OsMediaCapability, PanelTabSpec, PanelTreeBuilder, Plugin, PluginApp, PluginBundle, VcsDocumentApp, WindowKindSpec,`,
  "pub use NoDraft"
);

s = mustReplace(
  s,
  `    node_graph_delete_selection_spec, selection_count_phrase, selection_domains_from_surface, ActionMeta, App, AppActionRegistry, AppBuilder, AppInstance, ArtifactKindSpec, ConfigView, DocumentApp, DocumentView, Emit, HistoryView, KeybindingSpec,`,
  `    node_graph_delete_selection_spec, selection_count_phrase, selection_domains_from_surface, ActionMeta, App, AppActionRegistry, AppBuilder, AppInstance, ArtifactKindSpec, ConfigView, DocumentApp, DocumentView, DraftView, Emit, HistoryView, KeybindingSpec,`,
  "pub use DraftView"
);

writeFileSync(pluginRs, s);
console.log("patched plugin");

// Enhance DocumentSession docstring to name draft_store/command_log ownership seam
let h = readFileSync(hostRs, "utf8");
h = mustReplace(
  h,
  `//#region 🔖️DocumentSession
/// 🧾 Host-owned per-document generation counters and engine cache — guests hold handles only.
pub struct DocumentSession {
    pub generation: u64,
    pub command_log_len: u64,
    pub engines: store::EngineCache,
}`,
  `//#region 🔖️DocumentSession
/// 🧾 Host-owned per-instance document authority. Typed \`DocumentStore\` / \`ConfigStore\` / \`DraftStore\`
/// and the command log still live in guest \`VcsDocumentApp\` until CHANNEL_VERSION 5 moves packs onto
/// the host; this session already owns the engine cache and generation counters so the ownership seam
/// exists in-tree (\`store\` / \`config_store\` / \`draft_store\` / \`command_log\` land here next).
pub struct DocumentSession {
    pub generation: u64,
    pub command_log_len: u64,
    pub engines: store::EngineCache,
}`,
  "DocumentSession doc"
);
writeFileSync(hostRs, h);
console.log("patched host docstring");
console.log("DONE");
