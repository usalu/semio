import { readFileSync, writeFileSync } from "fs";

const pluginPath = process.argv[2];
const channelPath = process.argv[3];
const logPath = process.argv[4];

let src = readFileSync(pluginPath, "utf8");
const origLen = src.length;

const draftTypes = `
    /// @emoji 📝 Read-only view of the volatile draft-lane projection (gestures, hover, chrome scratch).
    pub struct DraftView<'a, D> {
        pub projection: &'a D,
    }

    /// @emoji 🫙 Empty draft projection for apps that do not use the draft lane.
    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct NoDraft {}

    /// @emoji 🫙 No-op draft operation twin of {@link NoConfigOperation}.
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum NoDraftOperation {
        Noop,
    }

    impl ::protocol::Operation<NoDraft> for NoDraftOperation {
        fn apply(&self, draft: &mut NoDraft) {
            let _ = draft;
        }

        fn backwards(&self, _before: &NoDraft) -> Vec<Self> {
            vec![NoDraftOperation::Noop]
        }
    }
`;

if (!src.includes("pub struct DraftView")) {
  const marker = "pub struct ConfigView<'a, C> {";
  const idx = src.indexOf(marker);
  if (idx < 0) throw new Error("ConfigView not found");
  const after = src.indexOf("\n    }\n", idx);
  if (after < 0) throw new Error("ConfigView end not found");
  const insertAt = after + "\n    }\n".length;
  src = src.slice(0, insertAt) + draftTypes + src.slice(insertAt);
}

src = src.replace(
  /pub struct Emit<Operation, ConfigOperation = NoConfigOperation> \{[\s\S]*?pub ui_scope: semio_framework_core::kernel::UiDirtyScope,\n    \}/,
  `pub struct Emit<Operation, ConfigOperation = NoConfigOperation, DraftOperation = NoDraftOperation> {
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
    }`,
);

src = src.replace(
  /impl<Operation, ConfigOperation> Default for Emit<Operation, ConfigOperation> \{\n        fn default\(\) -> Self \{\n            Self \{ document_operations: Vec::new\(\), config_operations: Vec::new\(\), description: None, coalesce_key: None, effects: Vec::new\(\), events: Vec::new\(\), ui_scope: semio_framework_core::kernel::UiDirtyScope::default\(\) \}\n        \}\n    \}/,
  `impl<Operation, ConfigOperation, DraftOperation> Default for Emit<Operation, ConfigOperation, DraftOperation> {
        fn default() -> Self {
            Self { document_operations: Vec::new(), config_operations: Vec::new(), draft_operations: Vec::new(), description: None, coalesce_key: None, effects: Vec::new(), events: Vec::new(), ui_scope: semio_framework_core::kernel::UiDirtyScope::default() }
        }
    }`,
);

src = src.replace(
  /impl<Operation, ConfigOperation> Emit<Operation, ConfigOperation> \{/,
  `impl<Operation, ConfigOperation, DraftOperation> Emit<Operation, ConfigOperation, DraftOperation> {`,
);

if (!src.includes("pub fn draft(")) {
  src = src.replace(
    /pub fn event\(event: AppEvent\) -> Self \{\n            Self \{ events: vec!\[event\], \.\.Default::default\(\) \}\n        \}\n    \}/,
    `pub fn event(event: AppEvent) -> Self {
            Self { events: vec![event], ..Default::default() }
        }

        /// @emoji 📝 A draft-lane emission carrying \`draft_operations\` and nothing else.
        pub fn draft(draft_operations: Vec<DraftOperation>) -> Self {
            Self { draft_operations, ..Default::default() }
        }

        /// @emoji 🔁️ Draft-lane amend (coalesced gesture ticks).
        pub fn amend_draft(draft_operations: Vec<DraftOperation>, coalesce_key: impl Into<String>) -> Self {
            Self { draft_operations, coalesce_key: Some(coalesce_key.into()), ..Default::default() }
        }

        /// @emoji 📌️ Draft-lane commit (described, non-coalesced).
        pub fn commit_draft(draft_operations: Vec<DraftOperation>, description: impl Into<String>) -> Self {
            Self { draft_operations, description: Some(description.into()), ..Default::default() }
        }
    }`,
  );
}

src = src.replace(
  "let Emit { document_operations, config_operations, description, coalesce_key, effects, events, ui_scope } = emit;",
  "let Emit { document_operations, config_operations, draft_operations, description, coalesce_key, effects, events, ui_scope } = emit;\n            let _ = &draft_operations; // host DocumentSession applies draft lane",
);

const oldTraitStart = src.indexOf("    pub trait DocumentApp: Send + 'static {");
const oldTraitEnd = src.indexOf("\n    }\n\n    /// 🎞️ Rust mirror of WIT's `media-artifact`", oldTraitStart);
if (oldTraitStart < 0 || oldTraitEnd < 0) throw new Error("DocumentApp trait bounds not found");

const newTrait = `    pub trait DocumentApp: Default + Send + 'static {
        const APP_ID: &'static str;
        const DOCUMENT_SCHEMA: &'static str;
        const CONFIG_SCHEMA: &'static str = "config.empty";

        type Projection: Clone + PartialEq + Serialize + DeserializeOwned + Send + store::DocumentDsl + DocumentPack;
        type Operation: ::protocol::Operation<Self::Projection> + PartialEq + Send + ::protocol::OpText + ::protocol::OpBinary;
        type Config: Clone + Default + PartialEq + Serialize + DeserializeOwned + Send + store::ConfigRecord + DocumentPack;
        type ConfigOperation: ::protocol::Operation<Self::Config> + PartialEq + Send + ::protocol::OpText + ::protocol::OpBinary;
        type Draft: Clone + Default + PartialEq + Serialize + DeserializeOwned + Send + DocumentPack;
        type DraftOperation: ::protocol::Operation<Self::Draft> + PartialEq + Send + ::protocol::OpText + ::protocol::OpBinary;
        /// @emoji 🎯️ Closed, typed command enum — sole dispatch surface for \`handle\`.
        type Command: ::protocol::OpBinary + Send;

        fn initial_projection() -> Self::Projection;
        fn initial_config() -> Self::Config {
            Self::Config::default()
        }
        fn initial_draft() -> Self::Draft {
            Self::Draft::default()
        }
        /// @emoji 🧩️ Pure \`(command, document, config, draft, engines) → Emit\`. Apps are ZSTs — no \`&self\` state.
        fn handle(
            command: &Self::Command,
            doc: &DocumentView<'_, Self::Projection>,
            cfg: &ConfigView<'_, Self::Config>,
            draft: &DraftView<'_, Self::Draft>,
            engines: &store::EngineHandles,
        ) -> Result<Emit<Self::Operation, Self::ConfigOperation, Self::DraftOperation>, Fault>;
        fn command_id(_command: &Self::Command) -> &'static str {
            "typed-command"
        }
        fn command_from_action(action: &str, _args: Option<&serde_json::Value>) -> Result<Self::Command, Fault> {
            Err(Fault::new(
                FaultOrigin::App,
                FaultCode::new("app.command.unsupported"),
                format!(
                    "action '{action}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — app actions are dispatched exclusively through the typed command channel now (see \`dispatch_typed_command\`)"
                ),
            ))
        }
        fn config_spec() -> ConfigSpec {
            ConfigSpec::empty()
        }
        fn clipboard_media_type() -> Option<MediaType> {
            None
        }
        fn clipboard_accepts() -> Vec<MediaType> {
            Self::clipboard_media_type().into_iter().collect()
        }
        fn copy_fragment(_doc: &DocumentView<'_, Self::Projection>, _cfg: &ConfigView<'_, Self::Config>, _draft: &DraftView<'_, Self::Draft>) -> Result<ClipboardFragment, ClipboardError> {
            Err(ClipboardError::EmptySelection)
        }
        fn cut_operations(_doc: &DocumentView<'_, Self::Projection>, _cfg: &ConfigView<'_, Self::Config>, _draft: &DraftView<'_, Self::Draft>) -> Vec<Self::Operation> {
            Vec::new()
        }
        fn paste_operations(_doc: &DocumentView<'_, Self::Projection>, _fragment: &ClipboardFragment, _placement: &PastePlacement) -> Result<Vec<Self::Operation>, ClipboardError> {
            Ok(Vec::new())
        }
        fn pending_effects(_doc: &DocumentView<'_, Self::Projection>, _cfg: &ConfigView<'_, Self::Config>, _draft: &DraftView<'_, Self::Draft>) -> Vec<HostEffect> {
            Vec::new()
        }
        fn render(body_key: &str, doc: &DocumentView<'_, Self::Projection>, cfg: &ConfigView<'_, Self::Config>, draft: &DraftView<'_, Self::Draft>) -> UiNode;
        fn window_engagements(_doc: &DocumentView<'_, Self::Projection>, _cfg: &ConfigView<'_, Self::Config>, _draft: &DraftView<'_, Self::Draft>) -> HashMap<String, WindowEngagement> {
            HashMap::new()
        }
        fn window_measures(_doc: &DocumentView<'_, Self::Projection>, _cfg: &ConfigView<'_, Self::Config>, _draft: &DraftView<'_, Self::Draft>) -> HashMap<String, Vec<WindowMeasure>> {
            HashMap::new()
        }
        fn tool_measures(_doc: &DocumentView<'_, Self::Projection>, _cfg: &ConfigView<'_, Self::Config>, _draft: &DraftView<'_, Self::Draft>) -> HashMap<String, Vec<WindowMeasure>> {
            HashMap::new()
        }
        fn context_menu(_request: &ContextMenuRequest, _doc: &DocumentView<'_, Self::Projection>, _cfg: &ConfigView<'_, Self::Config>, _draft: &DraftView<'_, Self::Draft>, _registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
            Vec::new()
        }
        fn seed(_store: &mut DocumentStore<Self::Projection, Self::Operation>) {}
        fn io() -> Option<AppIo> {
            None
        }
        fn media_ports() -> Vec<MediaPortSpec> {
            Self::io().map(|io| io.all_ports()).unwrap_or_default()
        }
        fn export_media(port: &str, doc: &DocumentView<'_, Self::Projection>) -> Result<Media, MediaError> {
            if port != "document:out" {
                return Err(MediaError::NotImplemented);
            }
            let media_type = Self::io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
            let bytes = doc.projection.encode_pack();
            Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
        }
        fn whole_document_operation(_projection: Self::Projection) -> Option<Self::Operation> {
            None
        }
        fn import_media(port: &str, media: &Media, _doc: &DocumentView<'_, Self::Projection>) -> Result<Emit<Self::Operation, Self::ConfigOperation, Self::DraftOperation>, MediaError> {
            if port != "document:in" {
                return Err(MediaError::NotImplemented);
            }
            let MediaPayload::Structured { json, .. } = &media.payload else {
                return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
            };
            let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            let projection = <Self::Projection as DocumentPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            match Self::whole_document_operation(projection) {
                Some(operation) => Ok(Emit::operations(vec![operation])),
                None => Err(MediaError::NotImplemented),
            }
        }
        fn media_fingerprint(port: &str, doc: &DocumentView<'_, Self::Projection>) -> Result<MediaFingerprint, MediaError> {
            Self::export_media(port, doc).map(|media| MediaFingerprint::of(&media))
        }
    }`;

src = src.slice(0, oldTraitStart) + newTrait + src.slice(oldTraitEnd);

src = src.replace(
  /\.register_document_app\(\(\$app_fn\)\(\), \|\| <\$app_ty as ::std::default::Default>::default\(\)\)/g,
  ".register_document_app::<$app_ty>(($app_fn)())",
);

writeFileSync(pluginPath, src);

let channel = readFileSync(channelPath, "utf8");
channel = channel.replace("pub const CHANNEL_VERSION: u32 = 4;", "pub const CHANNEL_VERSION: u32 = 5;");
writeFileSync(channelPath, channel);

writeFileSync(
  logPath,
  `plugin ${origLen} -> ${src.length}\nCHANNEL_VERSION=5\nDraftView=${src.includes("pub struct DraftView")}\ndraft_operations=${src.includes("draft_operations")}\nreceiverless=${src.includes("pub trait DocumentApp: Default + Send")}\n`,
);
console.log("ok");
