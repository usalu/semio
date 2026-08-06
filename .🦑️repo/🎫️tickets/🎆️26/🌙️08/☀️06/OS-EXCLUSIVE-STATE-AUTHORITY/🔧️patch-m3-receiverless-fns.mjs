import { readFileSync, writeFileSync } from "fs";

const pluginPath = process.argv[2];
const logPath = process.argv[3];
let src = readFileSync(pluginPath, "utf8");
const before = src;

// 1) Make DocumentApp require Default (for ZST registration)
src = src.replace(
  "pub trait DocumentApp: Send + 'static {",
  "pub trait DocumentApp: Default + Send + 'static {",
);

// 2) Convert common method signatures: fn foo(&self -> fn foo(
// Only inside DocumentApp trait — bounded by markers we know.
const traitStart = src.indexOf("pub trait DocumentApp: Default + Send + 'static {");
const traitEnd = src.indexOf("\n    }\n\n    /// 🎞️ Rust mirror of WIT's `media-artifact`", traitStart);
if (traitStart < 0 || traitEnd < 0) throw new Error("trait range missing");
let trait = src.slice(traitStart, traitEnd);

const methodReplacements = [
  [/fn app_id\(&self\) -> &str \{\n\s*Self::APP_ID\n\s*\}/, ""],
  [/fn document_schema\(&self\) -> &str \{\n\s*Self::DOCUMENT_SCHEMA\n\s*\}/, ""],
  [/fn config_schema\(&self\) -> &str \{[\s\S]*?\n        \}/, `fn config_schema() -> &'static str {
            "config.empty"
        }`],
  [/fn initial_projection\(&self\) -> Self::Projection;/, "fn initial_projection() -> Self::Projection;"],
  [/fn initial_config\(&self\) -> Self::Config \{[\s\S]*?\n        \}/, `fn initial_config() -> Self::Config {
            Self::Config::default()
        }`],
  [/fn initial_draft\(&self\) -> Self::Draft \{[\s\S]*?\n        \}/, `fn initial_draft() -> Self::Draft {
            Self::Draft::default()
        }`],
  [
    /fn handle\(\n\s*&self,\n\s*command: &Self::Command,\n\s*doc: &DocumentView<'_, Self::Projection>,\n\s*cfg: &ConfigView<'_, Self::Config>,\n\s*draft: &DraftView<'_, Self::Draft>,\n\s*engines: &EngineHandles,\n\s*\)/,
    `fn handle(
            command: &Self::Command,
            doc: &DocumentView<'_, Self::Projection>,
            cfg: &ConfigView<'_, Self::Config>,
            draft: &DraftView<'_, Self::Draft>,
            engines: &EngineHandles,
        )`,
  ],
  [/fn command_id\(&self, _command: &Self::Command\) -> &str \{/, "fn command_id(_command: &Self::Command) -> &'static str {"],
  [/fn command_from_action\(&self, action: &str/, "fn command_from_action(action: &str"],
  [/fn config_spec\(&self\) -> ConfigSpec \{/, "fn config_spec() -> ConfigSpec {"],
  [/fn clipboard_media_type\(&self\) -> Option<MediaType> \{/, "fn clipboard_media_type() -> Option<MediaType> {"],
  [
    /fn clipboard_accepts\(&self\) -> Vec<MediaType> \{\n\s*self\.clipboard_media_type\(\)\.into_iter\(\)\.collect\(\)\n\s*\}/,
    `fn clipboard_accepts() -> Vec<MediaType> {
            Self::clipboard_media_type().into_iter().collect()
        }`,
  ],
  [/fn copy_fragment\(&self,/, "fn copy_fragment("],
  [/fn cut_operations\(&self,/, "fn cut_operations("],
  [/fn paste_operations\(&self,/, "fn paste_operations("],
  [/fn pending_effects\(&self,/, "fn pending_effects("],
  [/fn render\(&self, body_key: &str/, "fn render(body_key: &str"],
  [/fn window_engagements\(&self,/, "fn window_engagements("],
  [/fn window_measures\(&self,/, "fn window_measures("],
  [/fn tool_measures\(&self,/, "fn tool_measures("],
  [/fn context_menu\(&self,/, "fn context_menu("],
  [/fn seed\(&self,/, "fn seed("],
  [/fn io\(&self\) -> Option<AppIo> \{/, "fn io() -> Option<AppIo> {"],
  [
    /fn media_ports\(&self\) -> Vec<MediaPortSpec> \{\n\s*self\.io\(\)\.map\(\|io\| io\.all_ports\(\)\)\.unwrap_or_default\(\)\n\s*\}/,
    `fn media_ports() -> Vec<MediaPortSpec> {
            Self::io().map(|io| io.all_ports()).unwrap_or_default()
        }`,
  ],
  [/fn export_media\(&self, port: &str/, "fn export_media(port: &str"],
  [/self\.io\(\)/g, "Self::io()"],
  [/self\.document_schema\(\)/g, "Self::DOCUMENT_SCHEMA"],
  [/fn whole_document_operation\(&self,/, "fn whole_document_operation("],
  [/fn import_media\(&self, port: &str/, "fn import_media(port: &str"],
  [/match self\.whole_document_operation\(projection\)/, "match Self::whole_document_operation(projection)"],
  [/fn media_fingerprint\(&self, port: &str/, "fn media_fingerprint(port: &str"],
  [/self\.export_media\(port, doc\)/, "Self::export_media(port, doc)"],
];

for (const [re, rep] of methodReplacements) {
  trait = trait.replace(re, rep);
}

src = src.slice(0, traitStart) + trait + src.slice(traitEnd);

// 3) VcsDocumentApp construction
src = src.replace(
  /let envelope = create_document_envelope::<A::Projection, A::Operation>\(app\.document_schema\(\), app\.app_id\(\), app\.initial_projection\(\), None\);/,
  "let envelope = create_document_envelope::<A::Projection, A::Operation>(A::DOCUMENT_SCHEMA, A::APP_ID, A::initial_projection(), None);",
);
src = src.replace(/let config_id = format!\("\{\}-config", app\.app_id\(\)\);/, "let config_id = format!(\"{}-config\", A::APP_ID);");
src = src.replace(
  /let config_envelope = create_config_envelope::<A::Config, A::ConfigOperation>\(app\.config_schema\(\), &config_id, app\.initial_config\(\), None\);/,
  "let config_envelope = create_config_envelope::<A::Config, A::ConfigOperation>(A::config_schema(), &config_id, A::initial_config(), None);",
);
src = src.replace(/let draft_id = format!\("\{\}-draft", app\.app_id\(\)\);/, "let draft_id = format!(\"{}-draft\", A::APP_ID);");
src = src.replace(/app\.initial_draft\(\)/g, "A::initial_draft()");
src = src.replace(/app\.seed\(&mut store\);/g, "A::seed(&mut store);");
src = src.replace(/self\.app\.document_schema\(\)/g, "A::DOCUMENT_SCHEMA");
src = src.replace(/self\.app\.app_id\(\)/g, "A::APP_ID");
src = src.replace(/self\.app\.command_from_action\(/g, "A::command_from_action(");
src = src.replace(/self\.app\.render\(/g, "A::render(");

// Line-based: inside `impl<A: DocumentApp>` blocks, rewrite `app.foo(` -> `A::foo(` when line has DocumentApp-ish context
{
  const lines = src.split("\n");
  let inVcs = false;
  let depth = 0;
  const methods = new Set(["command_from_action","command_id","handle","copy_fragment","cut_operations","paste_operations","import_media","export_media","tool_measures","pending_effects","context_menu","render"]);
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    if (line.includes("impl<A: DocumentApp>")) { inVcs = true; depth = 0; }
    if (inVcs) {
      depth += (line.match(/\{/g)||[]).length - (line.match(/\}/g)||[]).length;
      if (depth < 0) inVcs = false;
      if (inVcs) {
        lines[i] = line.replace(/\bapp\.(command_from_action|command_id|handle|copy_fragment|cut_operations|paste_operations|import_media|export_media|tool_measures|pending_effects|context_menu|render)\(/g, "A::$1(");
      }
    }
  }
  src = lines.join("\n");
}

// 4) Turbofish-only register_document_app — replace factory version with zst-only
src = src.replace(
  /\/\/\/ @emoji 🧬️ Registers a typed \{@link DocumentApp\}, wrapping each instance in a\n        \/\/\/ \{@link VcsDocumentApp\} so it satisfies the object-safe runtime \{@link PluginApp\} contract with\n        \/\/\/ a persistent operation store\. The only public app-registration entry point — this structurally\n        \/\/\/ guarantees every app's state lives in a `DocumentStore`\.\n        pub fn register_document_app<A>\(self, app: App, factory: impl Fn\(\) -> A \+ Send \+ 'static\) -> Self\n        where\n            A: DocumentApp,\n        \{\n            let registry = AppActionRegistry::from_definition\(&app\.definition\);\n            self\.register_app\(app, move \|\| Box::new\(VcsDocumentApp::with_registry\(factory\(\), registry\.clone\(\)\)\)\)\n        \}\n\n        \/\/\/ @emoji 🧬️ Turbofish ZST registration — `A: Default` constructs the \(preferably zero-sized\) app\n        \/\/\/ type without a factory closure\. Preferred entry for receiverless apps; `semio_plugin!` uses this\.\n        pub fn register_document_app_zst<A: DocumentApp \+ Default>\(self, app: App\) -> Self \{[\s\S]*?\n        \}/,
  `/// @emoji 🧬️ Registers a typed {@link DocumentApp} as a ZST — turbofish-only, no factory closure.
        /// Wraps each instance in {@link VcsDocumentApp}. Stateful app structs are unrepresentable.
        pub fn register_document_app<A: DocumentApp>(self, app: App) -> Self {
            let registry = AppActionRegistry::from_definition(&app.definition);
            self.register_app(app, move || Box::new(VcsDocumentApp::with_registry(A::default(), registry.clone())))
        }`,
);

// rename zst macro uses
src = src.replace(/\.register_document_app_zst::<\$app_ty>\(/g, ".register_document_app::<$app_ty>(");
src = src.replace(/register_document_app_zst/g, "register_document_app");

// 5) Fix in-crate DocumentApp impl method receivers for Dummy/Test if still &self
src = src.replace(/fn initial_projection\(&self\) -> /g, "fn initial_projection() -> ");
src = src.replace(
  /fn handle\(&self, command: &DummyCommand, doc: &DocumentView<'_, DummyProjection>, _cfg: &ConfigView<'_, NoConfig>, _draft: &DraftView<'_, NoDraft>, _engines: &EngineHandles\)/g,
  "fn handle(command: &DummyCommand, doc: &DocumentView<'_, DummyProjection>, _cfg: &ConfigView<'_, NoConfig>, _draft: &DraftView<'_, NoDraft>, _engines: &EngineHandles)",
);
src = src.replace(
  /fn handle\(&self, command: &TestCommand, doc: &DocumentView<'_, TestProjection>, _cfg: &ConfigView<'_, TestConfig>, _draft: &DraftView<'_, NoDraft>, _engines: &EngineHandles\)/g,
  "fn handle(command: &TestCommand, doc: &DocumentView<'_, TestProjection>, _cfg: &ConfigView<'_, TestConfig>, _draft: &DraftView<'_, NoDraft>, _engines: &EngineHandles)",
);
src = src.replace(/fn render\(&self, /g, "fn render(");

writeFileSync(pluginPath, src);
writeFileSync(
  logPath,
  `bytes ${before.length} -> ${src.length}\nDefault bound=${src.includes("DocumentApp: Default + Send")}\nregister factory gone=${!src.includes("factory: impl Fn() -> A")}\nA::handle=${src.includes("A::handle(")}\n`,
);
console.log("receiverless patch applied");
