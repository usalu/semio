# 📓️ lease — P8-agent-descriptor

**From**: terra, packet P8-agent-spi, ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY`.
**To**: the peer ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`'s sol (files below are
their A2/E1/E2 `path_scope`, per `important.md`'s collision table), applied together as ONE atomic
bundle (§0 explains why atomicity matters). None of these diffs have been applied by terra — every
file below is unmodified by this packet except `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`,
whose OWN edit (the new `//#region 🔖️AgentContributions`, already applied, see
`📓️terra-P8-report.md`) is additive and independent of this lease landing.

---

## 0. Why this must land as one bundle, not piecemeal

`AgentContributions{capabilities: Vec<String>, promoted: Vec<String>}` (already defined, additive,
zero call sites, in `🛂️manifest/🦀️component.rs` — see that region's own header comment for the
full reasoning) needs an attachment point on `PackageDescriptor`. `PackageDescriptor`'s only two
construction sites (`describe_plugin()`/`describe_extension()`, both in
`🔌️plugin/🛂️describe/🦀️component.rs`) build it as a **full explicit struct literal** — verified
by reading the file (§4 quotes it verbatim) — so adding a field to `PackageDescriptor` and
NOT updating those two literals in the same edit breaks `cargo check -p semio-framework-plugin
--lib` immediately. The five diffs below (manifest field, `PluginDescriptorExtras` field,
`ExtensionManifest` field, `PluginBuilder`/`ExtensionBundle` builder methods, `describe_plugin()`/
`describe_extension()` assembly) must be applied **together**; applying any strict subset leaves
the tree red. The sixth (registry `check`) and seventh (note proof) diffs are independent additive
follow-ons that only need the first five.

---

## 1. `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` — `PackageDescriptor.agent` field

**Current SHA-256** (after terra's own already-applied `AgentContributions` region):
`89fc2d7c01b298ae2423354f7e1d63e65aad3d7b12a4ec50aaabe32d88a98aa6`
**git log**: `abd29c08d0`, `153db71c51`, `1eaf87e6f5`.

### Diff — add the field to `PackageDescriptor`

```diff
 pub struct PackageDescriptor {
     pub descriptor_version: u32,
     pub role: PackageRole,
     pub manifest: PluginManifest,
     #[serde(default, skip_serializing_if = "Vec::is_empty")]
     pub activation_events: Vec<kernel::ActivationEvent>,
     #[serde(default, skip_serializing_if = "Vec::is_empty")]
     pub capability_requests: Vec<kernel::CapabilityRequest>,
     #[serde(default, skip_serializing_if = "Vec::is_empty")]
     pub extension_points: Vec<ExtensionPointDeclaration>,
     pub execution: ExecutionMode,
     #[serde(default)]
     pub quotas: kernel::QuotaSchema,
     #[serde(default)]
     pub contributions: ContributionSet,
     #[serde(default, skip_serializing_if = "Vec::is_empty")]
     pub assets: Vec<AssetDeclaration>,
     pub hashes: PackageHashes,
+    /// 🤖️ P8-agent-spi (`📋️master.md` §3.1): which of this package's own capabilities are
+    /// offered to agents, and which subset is promoted to a first-class MCP tool — see
+    /// `AgentContributions`'s own doc for the critical distinction from `capability_requests`
+    /// above. `None` (the default) means "not yet agent-enabled", never "agent-enabled with
+    /// zero capabilities" (an empty-but-`Some` value).
+    #[serde(default, skip_serializing_if = "Option::is_none")]
+    #[cfg_attr(feature = "typegen", ts(optional))]
+    pub agent: Option<AgentContributions>,
 }
 //#endregion 🔖️PackageDescriptor
```

Every OTHER field on `PackageDescriptor` stays byte-identical — this is a pure addition at the end
of the struct, `#[serde(default, skip_serializing_if = "Option::is_none")]` so any descriptor JSON
produced before this lands (there is exactly one today, note's own, see §7) still deserializes
without a wire break.

---

## 2. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` — `PluginBuilder::agent(..)`

**Current SHA-256**: `c5771f6151d6ffb71339aa8aebc92041cc4591a60d68c553f33352f903065eab`
**git log**: `abd29c08d0`, `1eaf87e6f5`, `5ac47258a6`.

**Naming note (read before applying)**: `PluginBuilder<Ready>` already has a `.capability(mut self,
capability: CapabilityRequirement) -> Self` (line 219, host-privilege capability requirement — what
this plugin NEEDS) — reusing that name for "what this plugin OFFERS to agents" would be exactly the
`capability_requests`-vs-`AgentContributions` conflation `🛂️manifest`'s new region forbids, one
altitude down. `.requests(CapabilityRequest)` already disambiguates from `.capability(..)` for the
same reason (see its own doc: "not the older kernel-level `CapabilityRequirement` `.capability(..)`
declares"). `.agent(..)` is the name `📋️master.md` §3.1 already gives `ExtensionBundle`'s
counterpart method — reused here for `PluginBuilder` too, for exactly the same collision-avoidance
reason, so both builders share one vocabulary.

**Design note**: `.agent(..)` takes **bare** action/command ids — the same bare `id` string the
plugin author already passed to `.mutation(id, ..)`/`ActionDefinition::new(id, ..)`/`.command(..)`
— never a pre-qualified `<plugin>.<app>.<id>` string. Real `AppDefinition.id` (the thing D3's
grammar calls `app_id`) is a dialect-coordinate string like `"s.note.note@1/*#editor"` (verified
against note's own real committed descriptor, `📓️terra-P8-report.md` §3.3) — asking a plugin
author to type that by hand is unreasonable and error-prone. `describe_plugin()` (§5 below) does
the qualification, by SEARCHING every declared app/window-kind for the bare id, exactly the way
`🌉️mcp/🗂️catalog::compile()` independently does (`app_id = app.id` verbatim,
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗂️catalog/🦀️component.rs:565`) — so an id declared
via `.agent(..)` and the id the catalog compiler independently derives are the SAME string, with no
second hand-maintained copy of the qualification logic.

### Diff 2a — struct fields (after the existing `assets` field)

```diff
     activation_events: Vec<ActivationEvent>,
     capability_requests: Vec<CapabilityRequest>,
     extension_points: Vec<ExtensionPointDeclaration>,
     execution: ExecutionMode,
     quotas: QuotaSchema,
     assets: Vec<AssetDeclaration>,
+    /// 🤖️ P8-agent-spi (`📋️master.md` §3.1) — bare action/command ids offered to agents /
+    /// promoted to a first-class MCP tool; `.agent(..)`'s own storage, installed into
+    /// `plugin_runtime::PluginDescriptorExtras` alongside the E2 fields above, for
+    /// `describe::describe_plugin()` to expand into fully-qualified `manifest::
+    /// AgentContributions` ids — see `.agent(..)`'s own doc.
+    agent_capabilities: Vec<String>,
+    agent_promoted: Vec<String>,
     _state: PhantomData<State>,
```

### Diff 2b — `PluginBuilder::new()` field init (mirror in `.label()`/`.version()`'s field threading)

```diff
             activation_events: Vec::new(),
             capability_requests: Vec::new(),
             extension_points: Vec::new(),
             execution: ExecutionMode::default(),
             quotas: QuotaSchema::default(),
             assets: Vec::new(),
+            agent_capabilities: Vec::new(),
+            agent_promoted: Vec::new(),
             _state: PhantomData,
```
applied at `new()`'s field literal; the byte-identical two-line insertion (before `_state:
PhantomData,`) is ALSO needed in `.label()`'s and `.version()`'s field-threading literals (each
currently ends `assets: self.assets, _state: PhantomData,` → becomes `assets: self.assets,
agent_capabilities: self.agent_capabilities, agent_promoted: self.agent_promoted, _state:
PhantomData,`) — three sites total, one per typestate transition, all in this file.

### Diff 2c — new `.agent(..)` method (placed in the `//#region 🔖️Descriptor` block, after `.asset(..)`)

```diff
         pub fn asset(mut self, declaration: AssetDeclaration) -> Self {
             if !self.assets.contains(&declaration) {
                 self.assets.push(declaration);
             }
             self
         }
+
+        /// 🤖️ P8-agent-spi (`📋️master.md` §3.1): declares which of this plugin's own already-
+        /// declared action/command ids (bare `id`, e.g. `"deleteSelection"` — never a pre-
+        /// qualified `<plugin>.<app>.<id>` string, see this method's module-level doc) are
+        /// offered to agents, and which of THOSE are further promoted to a first-class MCP
+        /// tool. Idempotent-push on both lists (mirrors `.requests`/`.asset`'s own idiom);
+        /// `try_build()` installs the raw bare-id lists into `PluginDescriptorExtras` for
+        /// `describe::describe_plugin()` to expand and validate (`promoted ⊆ capabilities`,
+        /// every id resolves to a real declared action — `📇️registry:check`'s job, not this
+        /// builder's: a builder call is infallible, `-> Self`, never `Result`).
+        pub fn agent(mut self, capabilities: impl IntoIterator<Item = impl Into<String>>, promoted: impl IntoIterator<Item = impl Into<String>>) -> Self {
+            for id in capabilities {
+                let id = id.into();
+                if !self.agent_capabilities.contains(&id) {
+                    self.agent_capabilities.push(id);
+                }
+            }
+            for id in promoted {
+                let id = id.into();
+                if !self.agent_promoted.contains(&id) {
+                    self.agent_promoted.push(id);
+                }
+            }
+            self
+        }
         //#endregion 🔖️Descriptor
```

### Diff 2d — `try_build()` destructure + install call

```diff
             activation_events,
             capability_requests,
             extension_points,
             execution,
             quotas,
             assets,
+            agent_capabilities,
+            agent_promoted,
             _state: _,
         } = self;
```
and, at the install call near the end of `try_build()`:
```diff
-        crate::plugin_runtime::install_plugin_descriptor_extras(crate::plugin_runtime::PluginDescriptorExtras { activation_events, capability_requests, extension_points, execution, quotas, assets });
+        crate::plugin_runtime::install_plugin_descriptor_extras(crate::plugin_runtime::PluginDescriptorExtras { activation_events, capability_requests, extension_points, execution, quotas, assets, agent_capabilities, agent_promoted });
```

---

## 3. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — `plugin_runtime`/`🧩️Extension`

**Current SHA-256**: `406d62589ff0872f300d99a0c881e2632ec5ee3d99348d71d23f79621cb8a803`
**git log**: `abd29c08d0`, `153db71c51`, `1eaf87e6f5`.
**Live-edit note**: at lease-writing time this file's mtime was updating every ~30s (another live
session mid-edit, unrelated presence-refactor territory — `adopt_presence`/`EphemeralSnapshot`, see
`📓️terra-P8-report.md` §4). The two sections below (`plugin_runtime`'s `PluginDescriptorExtras`,
the `//#region 🧩️Extension` block) are far from that churn (verified: neither name appears in the
`cargo check` error list terra captured), but re-diff against the file's live state before applying.

### Diff 3a — `PluginDescriptorExtras` gains `agent`

```diff
     #[derive(Clone, Default)]
     pub struct PluginDescriptorExtras {
         pub activation_events: Vec<ActivationEvent>,
         pub capability_requests: Vec<CapabilityRequest>,
         pub extension_points: Vec<ExtensionPointDeclaration>,
         pub execution: ExecutionMode,
         pub quotas: QuotaSchema,
         pub assets: Vec<AssetDeclaration>,
+        /// 🤖️ P8-agent-spi — bare action/command ids from `PluginBuilder::agent(..)`, expanded
+        /// into fully-qualified `manifest::AgentContributions` by `describe::describe_plugin()`
+        /// (this struct stores the pre-expansion bare form since expansion needs `PluginManifest.
+        /// apps`, not yet assembled at the point `.agent(..)` is called).
+        pub agent_capabilities: Vec<String>,
+        pub agent_promoted: Vec<String>,
     }
```
(`#[derive(Clone, Default)]` already covers the two new `Vec<String>` fields — no derive change
needed; `plugin_descriptor_extras()`'s `unwrap_or_default()` fallback stays correct unmodified.)

### Diff 3b — `ExtensionManifest` gains `agent_capabilities`/`agent_promoted`

```diff
         /// 🙏️ Capability asks the broker resolves at install/link/runtime, set via
         /// `ExtensionBundle::requests(..)` — the NEW broker-scoped `kernel::CapabilityRequest`, not
         /// the older kernel-level `CapabilityRequirement` `capabilities` above carries (see that
         /// field's own doc for why both exist).
         #[serde(default, skip_serializing_if = "Vec::is_empty")]
         pub capability_requests: Vec<CapabilityRequest>,
+        /// 🤖️ P8-agent-spi (`📋️master.md` §3.1) — bare invoke-handler capability names (the same
+        /// strings passed to `.handler(capability, ..)`) offered to agents / promoted to a
+        /// first-class MCP tool, set via `ExtensionBundle::agent(..)`. `describe::
+        /// describe_extension()` prefixes each with `extension_id` to get the fully-qualified
+        /// id (`<extension_id>.<capability>` — extensions have no `app_id`/`window_kind_id` to
+        /// qualify with, unlike a plugin's actions).
+        #[serde(default, skip_serializing_if = "Vec::is_empty")]
+        pub agent_capabilities: Vec<String>,
+        #[serde(default, skip_serializing_if = "Vec::is_empty")]
+        pub agent_promoted: Vec<String>,
     }
```

### Diff 3c — both `ExtensionManifest` construction sites

```diff
                 manifest: ExtensionManifest {
                     extension_id: extension_id.into(),
                     label: label.into(),
                     version: version.into(),
                     extends: String::new(),
                     capabilities: Vec::new(),
                     topic_contributions: Vec::new(),
                     dependencies: Vec::new(),
                     contributions: Vec::new(),
                     execution: ExecutionMode::default(),
                     capability_requests: Vec::new(),
+                    agent_capabilities: Vec::new(),
+                    agent_promoted: Vec::new(),
                 },
```
(`ExtensionBundle::new()`) and
```diff
             slot.borrow().as_ref().map(|bundle| bundle.manifest.clone()).unwrap_or_else(|| ExtensionManifest {
                 extension_id: String::new(),
                 label: String::new(),
                 version: String::new(),
                 extends: String::new(),
                 capabilities: Vec::new(),
                 topic_contributions: Vec::new(),
                 dependencies: Vec::new(),
                 contributions: Vec::new(),
                 execution: ExecutionMode::default(),
                 capability_requests: Vec::new(),
+                agent_capabilities: Vec::new(),
+                agent_promoted: Vec::new(),
             })
```
(`extension_manifest()`'s fallback).

### Diff 3d — `ExtensionBundle::agent(..)` (after `.requests(..)`)

```diff
         pub fn requests(mut self, request: CapabilityRequest) -> Self {
             if !self.manifest.capability_requests.contains(&request) {
                 self.manifest.capability_requests.push(request);
             }
             self
         }
+
+        /// 🤖️ P8-agent-spi (`📋️master.md` §3.1) — mirrors `PluginBuilder::agent(..)`'s idiom and
+        /// doc; ids here are bare invoke-handler capability names (see `ExtensionManifest.
+        /// agent_capabilities`'s own doc for the extension-specific qualification rule).
+        pub fn agent(mut self, capabilities: impl IntoIterator<Item = impl Into<String>>, promoted: impl IntoIterator<Item = impl Into<String>>) -> Self {
+            for id in capabilities {
+                let id = id.into();
+                if !self.manifest.agent_capabilities.contains(&id) {
+                    self.manifest.agent_capabilities.push(id);
+                }
+            }
+            for id in promoted {
+                let id = id.into();
+                if !self.manifest.agent_promoted.contains(&id) {
+                    self.manifest.agent_promoted.push(id);
+                }
+            }
+            self
+        }
     }
```

---

## 4. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️component.rs` — assembly

**Current SHA-256**: `35e44d08bc9ea3aa361f007c7c0a9c227813a806bcaefc17764b1a0b76fb6152`
**git log**: `abd29c08d0`, `1eaf87e6f5`.

### Diff 4a — new helper (placed after `plugin_contributions`, before `describe_plugin`)

```diff
+/// 🤖️ P8-agent-spi (`📋️master.md` §3.1): resolves ONE bare action/command id (as supplied to
+/// `PluginBuilder::agent(..)`) to every fully-qualified `<plugin_id>.<app.id>.<action_id>`
+/// capability id it matches across every declared app's window kinds — the SAME expansion
+/// `🌉️mcp/🗂️catalog::compile()` independently applies (`app_id = app.id` verbatim, D3 grammar,
+/// `🌉️mcp/🗂️catalog/🦀️component.rs:565`). More than one match (the same bare id declared in two
+/// apps/window kinds) is legitimate — D3 itself anticipates this ("`#<window_kind_id>` suffix
+/// only on genuine duplicates") — so every match is returned, never just the first; a bare id
+/// with ZERO matches (a typo, or an id that only exists on a `CommandDefinition` — command
+/// expansion is a documented follow-up, not yet implemented here, see this fn's own doc below)
+/// is `📇️registry:check`'s job to flag, not this function's — `describe()` is infallible.
+fn expand_agent_capability_id(plugin_id: &str, manifest: &PluginManifest, bare_id: &str) -> Vec<String> {
+    manifest
+        .apps
+        .iter()
+        .flat_map(|app| app.window_kinds.iter().map(move |window_kind| (app, window_kind)))
+        .flat_map(|(app, window_kind)| window_kind.actions.iter().map(move |action| (app, action)))
+        .filter(|(_, action)| action.id == bare_id)
+        .map(|(app, _)| format!("{plugin_id}.{}.{bare_id}", app.id))
+        .collect()
+}
+
+/// 🤖️ P8-agent-spi: expands every bare id in `capabilities`/`promoted` via
+/// `expand_agent_capability_id`, unions/dedupes into one `manifest::AgentContributions`. `None`
+/// when both bare-id lists are empty (an `AgentContributions::default()` that is never
+/// constructed at all — "not yet agent-enabled", per that type's own doc), matching the `Option`
+/// this fn's caller attaches to `PackageDescriptor.agent`.
+fn plugin_agent_contributions(plugin_id: &str, manifest: &PluginManifest, capabilities: &[String], promoted: &[String]) -> Option<manifest::AgentContributions> {
+    if capabilities.is_empty() && promoted.is_empty() {
+        return None;
+    }
+    let mut expanded_capabilities: Vec<String> = Vec::new();
+    for bare_id in capabilities {
+        for id in expand_agent_capability_id(plugin_id, manifest, bare_id) {
+            if !expanded_capabilities.contains(&id) {
+                expanded_capabilities.push(id);
+            }
+        }
+    }
+    let mut expanded_promoted: Vec<String> = Vec::new();
+    for bare_id in promoted {
+        for id in expand_agent_capability_id(plugin_id, manifest, bare_id) {
+            if !expanded_promoted.contains(&id) {
+                expanded_promoted.push(id);
+            }
+        }
+    }
+    Some(manifest::AgentContributions { capabilities: expanded_capabilities, promoted: expanded_promoted })
+}
```
(needs `manifest::AgentContributions`/`manifest::PluginManifest`-qualified access — the file's
existing `use semio_framework::{… PluginManifest, …};` import already brings `PluginManifest` in
unqualified, so inside this new function use the bare `PluginManifest` name to match the file's own
style; `manifest::AgentContributions` above refers to the `semio_framework::manifest` module path
exactly like the file's doc comments already write `manifest::PackageDescriptor` elsewhere — add
`AgentContributions` to the existing `use semio_framework::{ … };` import line.)

### Diff 4b — import line

```diff
 use semio_framework::{
-    io, kernel, AppDefinition, ComposerEntryDescriptor, ContributedInferenceMetadata, ContributionSet, FileTypeContribution, IoEntryDescriptor, IoEntryDirection, PackageDescriptor, PackageHashes,
-    PackageRole, PanelTabDefinition, PluginManifest,
+    io, kernel, AgentContributions, AppDefinition, ComposerEntryDescriptor, ContributedInferenceMetadata, ContributionSet, FileTypeContribution, IoEntryDescriptor, IoEntryDirection, PackageDescriptor,
+    PackageHashes, PackageRole, PanelTabDefinition, PluginManifest,
 };
```
(then the helper functions above reference the bare `AgentContributions` name, not `manifest::
AgentContributions` — corrects §4a's own note to match this file's real import idiom, which
brings every manifest type in unqualified rather than through a `manifest::` path prefix.)

### Diff 4c — `describe_plugin()`

```diff
 pub fn describe_plugin() -> Vec<u8> {
     let manifest = crate::plugin_runtime::plugin_manifest();
     let extras = crate::plugin_runtime::plugin_descriptor_extras();
     let contributions = plugin_contributions(&manifest);
+    let agent = plugin_agent_contributions(&manifest.plugin_id, &manifest, &extras.agent_capabilities, &extras.agent_promoted);
     let descriptor = PackageDescriptor {
         descriptor_version: 1,
         role: PackageRole::Plugin,
         manifest,
         activation_events: extras.activation_events,
         capability_requests: extras.capability_requests,
         extension_points: extras.extension_points,
         execution: extras.execution,
         quotas: extras.quotas,
         contributions,
         assets: extras.assets,
         hashes: PackageHashes { wasm_sha256: String::new(), core_wasm_sha256: String::new(), descriptor_sha256: String::new() },
+        agent,
     };
     store::pack_rt::encode_wire_value(&dsl::to_dsl_value(&descriptor).unwrap_or(dsl::DslValue::Null))
 }
```

### Diff 4d — `describe_extension()`

```diff
 pub fn describe_extension() -> Vec<u8> {
     let extension = crate::plugin_runtime::extension_manifest();
     let manifest = PluginManifest {
         plugin_id: extension.extension_id,
         label: extension.label,
         version: extension.version,
         apps: Vec::new(),
         examples: Vec::new(),
         capabilities: extension.capabilities,
         topic_contributions: extension.topic_contributions.clone(),
         commands: Vec::new(),
         artifact_kinds: Vec::new(),
         dependencies: extension.dependencies,
         contributions: extension.contributions.clone(),
     };
+    let agent = if extension.agent_capabilities.is_empty() && extension.agent_promoted.is_empty() {
+        None
+    } else {
+        let prefix = &manifest.plugin_id; // == extension_id — PluginManifest has no `extension_id` field
+        Some(AgentContributions {
+            capabilities: extension.agent_capabilities.iter().map(|id| format!("{prefix}.{id}")).collect(),
+            promoted: extension.agent_promoted.iter().map(|id| format!("{prefix}.{id}")).collect(),
+        })
+    };
     let contributions = ContributionSet {
         commands: Vec::new(),
         menus: Vec::new(),
         file_types: Vec::new(),
         panels: Vec::new(),
         themes: Vec::new(),
         topic_contributions: extension.topic_contributions,
         artifact_contributions: extension.contributions,
         inference_services: Vec::new(),
         mutation_services: Vec::new(),
         io_entries: Vec::new(),
         composer_entries: Vec::new(),
     };
     let descriptor = PackageDescriptor {
         descriptor_version: 1,
         role: PackageRole::Extension,
         manifest,
         activation_events: Vec::new(),
         capability_requests: extension.capability_requests,
         extension_points: Vec::new(),
         execution: extension.execution,
         quotas: kernel::QuotaSchema::default(),
         contributions,
         assets: Vec::new(),
         hashes: PackageHashes { wasm_sha256: String::new(), core_wasm_sha256: String::new(), descriptor_sha256: String::new() },
+        agent,
     };
     store::pack_rt::encode_wire_value(&dsl::to_dsl_value(&descriptor).unwrap_or(dsl::DslValue::Null))
 }
```
(`extension.agent_capabilities`/`.agent_promoted` are consumed by value here — `describe_extension()`
already consumes every other `extension.*` field by value/clone the same way, so this mirrors the
existing style; note `extension.agent_capabilities`/`.agent_promoted` must be read BEFORE
`extension.topic_contributions`/`.contributions` are `.clone()`d further down if the borrow checker
orders differently than shown — a mechanical, not semantic, adjustment the applier should make if
`cargo check` flags a move-after-use.)

---

## 5. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` — registry `check`

**Current SHA-256**: `39d8b18fae9898665e5d90e312e88ce0db20606b758bf825a779e2f2bdc615e3`
**git log**: `abd29c08d0`, `1eaf87e6f5`.

**Bundled prerequisite bug fix, found while reading this file (real, pre-existing, not P8's, not
yet reported anywhere else)**: `DESCRIPTOR_JSON_REL_PATH = ["..", "..", "🤖️generated",
"🔣️descriptor.json"]` (line 150) still points at the path `📓️design-abi.md` §3 originally
specified — but E2-builder-descriptor's own registrar ruling (peer `📓️status.md`, "Registrar
decision: descriptors move out of `🤖️generated/`") superseded that path months before this lease
was written, and E2 already fixed the analogous Rust-side `descriptor_is_fresh()` macro's path
(`📓️terra-E2-builder-descriptor-report.md` §4) but this TypeScript reader was never updated to
match. Confirmed empirically: note's real committed descriptor lives at
`✏️s/🔌️plugins/🗒️note/🔣️descriptor.json` (siblings of `🛂️manifest.json`, not under
`🤖️generated/`), so `readDescriptorJson()` as written today finds NOTHING for note — every check
this diff adds would silently never fire without this one-line fix, exactly the failure mode E2's
own Rust-side fix already prevented on the other side of the same pipeline. Bundled here as a
necessary prerequisite, not a scope creep — without it §5's new checks are dead code against every
real descriptor in the tree.

### Diff 5a — path fix

```diff
-const DESCRIPTOR_JSON_REL_PATH = ["..", "..", "🤖️generated", "🔣️descriptor.json"];
+const DESCRIPTOR_JSON_REL_PATH = ["..", "..", "🔣️descriptor.json"];
```
(the doc comment immediately above this constant, and `readDescriptorJson`'s own doc comment, both
still say `🤖️generated/🔣️descriptor.json` — update both prose references to drop `🤖️generated/`
in the same diff, since they'd otherwise misdescribe the corrected path.)

### Diff 5b — agent-contribution checks, appended inside `validateDescriptors`'s per-entry loop

Placed right after the existing `if (entry.hashes) { … }` hash-check block, before the loop's
closing brace:

```diff
     if (entry.hashes) {
       const builtWasm = findBuiltWasm(repoRoot, entry.wasmOut);
       if (builtWasm === undefined) {
         warnings.push(`${entry.pluginId}: has hashes.wasmSha256 but no built wasm found under ${WASM_TARGET_DIR.join("/")}/{${WASM_PROFILE_DIRS.join(",")}}/${entry.wasmOut} — skipping hash check`);
       } else {
         const actual = sha256HexOfFile(builtWasm);
         if (actual !== entry.hashes.wasmSha256) {
           errors.push(`${entry.pluginId}: hashes.wasmSha256 is ${entry.hashes.wasmSha256} but ${relative(repoRoot, builtWasm)} actually hashes to ${actual} — re-run \`describe\` after the latest build`);
         }
       }
     }
+    // 🤖️ P8-agent-spi (`📋️master.md` §3.1): `descriptor.agent.capabilities`/`.promoted` gate —
+    // `promoted ⊆ capabilities` always; for a PLUGIN (real `manifest.apps[].windowKinds[].
+    // actions[].id` are fully serialized in the descriptor JSON) every capability id must also
+    // resolve to a real declared action, mirroring `expand_agent_capability_id`'s Rust-side
+    // expansion exactly (`${entry.pluginId}.${app.id}.${action.id}`) so a bare-id typo in
+    // `.agent(..)` is caught here rather than silently producing an unreachable capability id.
+    // For an EXTENSION there is no serialized action list to check against (its own capabilities
+    // are runtime invoke-handler closures, `ExtensionBundle::handler(..)`, which carry no wire
+    // form) — only the `promoted ⊆ capabilities` structural check applies there; documented
+    // narrower coverage, not an oversight (mirrors this gate's own "severity, deliberately
+    // asymmetric" precedent above).
+    const agentBlock = descriptor?.agent as { capabilities?: unknown; promoted?: unknown } | undefined;
+    if (agentBlock) {
+      const capabilities = Array.isArray(agentBlock.capabilities) ? agentBlock.capabilities.filter((id): id is string => typeof id === "string") : [];
+      const promoted = Array.isArray(agentBlock.promoted) ? agentBlock.promoted.filter((id): id is string => typeof id === "string") : [];
+      const capabilitySet = new Set(capabilities);
+      for (const id of promoted) {
+        if (!capabilitySet.has(id)) {
+          errors.push(`${entry.pluginId}: agent.promoted contains ${JSON.stringify(id)}, which is not in agent.capabilities — promoted must be a subset`);
+        }
+      }
+      if (entry.role === "plugin") {
+        const manifestRecord = descriptor?.manifest as Record<string, unknown> | undefined;
+        const apps = Array.isArray(manifestRecord?.apps) ? (manifestRecord!.apps as unknown[]) : [];
+        const realCapabilityIds = new Set<string>();
+        for (const app of apps) {
+          const appId = (app as { id?: unknown }).id;
+          const windowKinds = Array.isArray((app as { windowKinds?: unknown }).windowKinds) ? ((app as { windowKinds?: unknown[] }).windowKinds as unknown[]) : [];
+          if (typeof appId !== "string") continue;
+          for (const windowKind of windowKinds) {
+            const actions = Array.isArray((windowKind as { actions?: unknown }).actions) ? ((windowKind as { actions?: unknown[] }).actions as unknown[]) : [];
+            for (const action of actions) {
+              const actionId = (action as { id?: unknown }).id;
+              if (typeof actionId === "string") realCapabilityIds.add(`${entry.pluginId}.${appId}.${actionId}`);
+            }
+          }
+        }
+        for (const id of capabilities) {
+          if (!realCapabilityIds.has(id)) {
+            errors.push(`${entry.pluginId}: agent.capabilities contains ${JSON.stringify(id)}, which does not match any declared action id (\`<pluginId>.<app.id>.<action.id>\`) in this descriptor's own manifest.apps`);
+          }
+        }
+      }
+    }
```

---

## 6. `✏️s/🔌️plugins/🗒️note/🦀️component.rs` — the proof migration

**Current SHA-256**: `e29c7fc4c987d3259b55bd846067b8e32704c6df222d7fa6ef8b6aa3f91a6dd7`
**git log**: `abd29c08d0`, `0b9f1d3a04`, `07873f842a`.

### Diff — one `.agent(..)` call, added after `.requests(..)`

```diff
         .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::note::artifact_kind().id })
         .execution(ExecutionMode::Isolated)
         .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist note edits to the open document".into(), optional: false })
+        // 🤖️ P8-agent-spi proof migration: three real editor actions offered to agents,
+        // `deleteSelection` promoted to a first-class MCP tool. `📓️design-abi.md`/`📋️master.md`
+        // §3.1 `use_when`/`semantics` themselves are declared where each action already is
+        // (`create_note_app()`, the `EditorBuilder.mutation(..)` call sites — see this packet's
+        // report §5.2 for the exact `.mutation(..)` → `.action_with(ActionDefinition::new(..)
+        // .use_when([..]).semantics(..))` diff, a DIFFERENT file/lease this bundle does not
+        // touch); `.agent(..)` here only curates WHICH of those already-semantics-bearing ids
+        // are agent-visible at all.
+        .agent(["deleteSelection", "duplicateSelection", "addBlock"], ["deleteSelection"])
         .try_build()
```

This one call is safe to add on its own (it references `PluginBuilder::agent(..)` from lease §2,
so it only compiles once §2 lands — bundle it with the rest, never apply in isolation).

### Companion diff — `use_when`/`semantics` on the three actions themselves

`create_note_app()` (`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/
✏️editor/🦀️component.rs`, **not owned by P8 either** — flagged here rather than silently
included, since it is a THIRD file this lease would need to touch, outside both this bundle's
five-file core and P8's own `path_scope`; left as a precise worked example instead, per the
packet brief's own "if you can apply … do so; if not, the lease plus a precise worked example is
the deliverable" instruction):

```diff
-            .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
-            .mutation("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"))
+            .action_with(ActionDefinition::new_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation)
+                .use_when(["delete the selection", "remove the selected blocks", "die auswahl löschen"])
+                .destructive())
+            .action_with(ActionDefinition::new_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Mutation)
+                .use_when(["duplicate the selection", "copy the selected blocks", "die auswahl duplizieren"]))
             // ➕️ Palette-visible block insertion (P1) with a staged argument form.
-            .mutation("addBlock", LocalizedLabel::native("Add Block", "Block hinzufügen"))
+            .action_with(ActionDefinition::new_catalog("addBlock", LocalizedLabel::native("Add Block", "Block hinzufügen"), ActionKind::Mutation)
+                .use_when(["add a new block", "insert a text block", "einen block hinzufügen"]))
```
(`.mutation(id, label)` is sugar for exactly `.action_with(ActionDefinition::new_catalog(id, label,
ActionKind::Mutation))` — `🔌️plugin/🦀️component.rs`'s own definition, quoted in
`📓️terra-P8-report.md` §5.2 — so this diff is behaviour-preserving plus the two new chained
builders `.use_when(..)`/`.destructive()`, both landed by P3 and already exercised by 153 passing
`semio-framework` tests; `.semantics(..)` itself is not called because `ActionKind::Mutation`'s
`ActionSemantics::for_kind` default (writes `artifact:{self}`, reversible, `Diff` preview, `Inverse`
undo, `documents.write` `WhenDestructive`) is already exactly right for these three actions —
overriding it with an identical literal would be the "duplicate state" CLAUDE.md forbids;
`.destructive()` on `deleteSelection` alone tightens `approval` from `WhenDestructive` (already the
default) to `Always`, the one real semantic difference worth declaring explicitly.)

---

## 7. Worked example — what note's `🔣️descriptor.json` would then contain

Once §1–§6 land and `semio-framework-plugin-describe describe` re-runs against note's rebuilt wasm
(or the native `describe_plugin()` harness E2's own report used, §6 there), the descriptor's new
top-level `agent` block, computed by hand against the REAL action ids verified present in the
REAL committed descriptor today (`📓️terra-P8-report.md` §3.3 — `deleteSelection`/
`duplicateSelection`/`addBlock` all exist in `manifest.apps[0].windowKinds[0].actions`, `apps[0].id
== "s.note.note@1/*#editor"`), would be:

```json
{
  "agent": {
    "capabilities": [
      "note.s.note.note@1/*#editor.deleteSelection",
      "note.s.note.note@1/*#editor.duplicateSelection",
      "note.s.note.note@1/*#editor.addBlock"
    ],
    "promoted": [
      "note.s.note.note@1/*#editor.deleteSelection"
    ]
  }
}
```

**Flagged, not fixed — a real, pre-existing, wider gap this worked example exposes**: these ids are
exactly what `🌉️mcp/🗂️catalog::compile()` (P2, this ticket, already closed) ALSO independently
produces for note today, because `app_id = app.id` verbatim
(`🌉️mcp/🗂️catalog/🦀️component.rs:565`) and `AppDefinition.id` is the real dialect-coordinate
string, not a clean logical name — `📋️master.md` §3.1's own prose ("`<plugin_id>.<app_id>.
<action_id>`") reads as if `app_id` were a short name like `"editor"`; P2's own fixtures (hand-built
with `app.id = "editor"` literally) never exercised the real `Editor::builder(dialect)` id shape, so
this mismatch was never caught until this packet's worked example. Not P8's to fix (P2/`🌉️mcp/
🗂️catalog` is a closed packet's path_scope, out of P8's own `path_scope`) — see
`📓️terra-P8-report.md` §4 for the full finding and a spawned follow-up task.
