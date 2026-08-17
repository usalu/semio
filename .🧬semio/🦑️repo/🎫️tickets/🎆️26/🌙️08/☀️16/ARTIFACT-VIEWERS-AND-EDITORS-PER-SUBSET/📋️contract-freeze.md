# Contract Freeze — Artifact Viewers and Editors per Subset

Frozen by the coordinator at ticket start. Every lane codes against exactly these names, strings and
labels. A lane that needs a change stops and reports; it does not improvise.

Ticket: `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`. Plan: `📋️master-plan.md`.
Start commit: `63686457bdcf0e7ba57a6598a4e224ec6c739f8e` (2026-08-16 02:50:31 +0200).

## 0. Measured ground truth (enumerated, not grepped)

| thing | count | command |
|---|---:|---|
| plugins | 33 | `ls "✏️s/🔌️plugins"` |
| subsets | 143 | `find "✏️s/🔌️plugins" -type d -regex '.*🪆️subsets/[^/]*' \| wc -l` |
| `🎛️apps` dirs | 33 | `find "✏️s/🔌️plugins" -type d -name "🎛️apps" \| wc -l` |
| apps | 53 | `find "✏️s/🔌️plugins" -type d -regex '.*🎛️apps/[^/]*' \| wc -l` |
| windows | 120 | `find "✏️s/🔌️plugins" -type d -regex '.*🪟️windows/[^/]*' \| wc -l` |

Target surfaces after W2: **286** (143 subsets × 2 roles).

## 1. C1 — Identity

Home: `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` (crate `semio-framework`).

```rust
/// 👁️✏️ Whether a surface may change the artifact it is bound to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum AppRole { Viewer, Editor }
```

- Wire strings: **`"viewer"`, `"editor"`** — exactly these, lowercase, in Rust serde, TS, JSON schema
  and the `SEMIO_APP_ROLE` / `VITE_SEMIO_APP_ROLE` env values.
- `AppRole::as_str()` → `"viewer"` / `"editor"`.
- `AppRole::from_str()` accepts only those two; anything else is `Err`.

```rust
/// 🎯️ A surface addressed across plugin boundaries.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct AppRef { pub plugin_id: String, pub app_id: String }
```

`AppDefinition` gains two REQUIRED fields (no `#[serde(default)]` — a surface without a role or a
dialect is unrepresentable):

```rust
pub role: AppRole,
pub dialect: ArtifactDialect,
```

`ArtifactDialect` (owned, `String`-based) — **not** `Dialect` (`&'static str`-based), because
`AppDefinition` is serialized and sent over the wire. `Dialect` stays the compile-time form on the
SDK traits; conversion is the existing `From<Dialect> for ArtifactDialect`.

### Canonical surface id (frozen grammar)

```
<artifact_kind>@<standard>/<subset>#<role>
```

Examples (real dialects on disk today): `s.cad.cad@1/*#editor`, `s.stdio.png@1/*#viewer`.

```rust
/// 🪪️ The one canonical spelling of a surface id.
pub fn surface_app_id(dialect: &ArtifactDialect, role: AppRole) -> String
/// 🪪️ Inverse; rejects anything not matching the grammar.
pub fn parse_surface_app_id(id: &str) -> Result<(ArtifactDialect, AppRole), String>
```

Law (unit test, owned by lane 0-A): `parse_surface_app_id(surface_app_id(d, r)) == (d, r)` for every
dialect in a fixture set including subset `*`, dotted standards (`1.7`), and hyphenated kinds.
`AppBuilder::build_definition` MUST set `id` from `surface_app_id` — a hand-written `id` is rejected.

`PanelTabKind` gains one variant, appended after `SettingsTheme`:

```rust
SettingsDefaultApps,   // id_str() == "framework.settings.default-apps"
```

## 2. C2 — SDK

Home: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (crate
`semio-framework-plugin`) and `…/🔌️plugin/🏗️builder/🦀️component.rs`.

### 2.1 Names

| old | new | note |
|---|---|---|
| `trait ArtifactApp` (authoring) | `trait ArtifactEditor` | same members, plus the two consts below |
| — | `trait ArtifactViewer` | new, read-only subset of the above |
| — | `struct EditorApp<E: ArtifactEditor>` | adapter |
| — | `struct ViewerApp<V: ArtifactViewer>` | adapter |
| `trait ArtifactApp` (runtime) | `trait ArtifactApp` | **kept**, now role-carrying; the adapters implement it |
| `PluginBuilder::document_app` | `PluginBuilder::editor::<E>` + `::viewer::<V>` | `document_app` is **deleted**, not deprecated |
| `App::builder(id)` | `Editor::builder(DIALECT)` / `Viewer::builder(DIALECT)` | id is derived, never passed |

`ArtifactEditor` / `ArtifactViewer` both carry:

```rust
const ROLE: AppRole;          // Editor / Viewer respectively, not overridable in practice
const DIALECT: Dialect;
```

`APP_ID` is **removed** from the authoring traits — it is derived from `DIALECT` + `ROLE` via
`surface_app_id`. `DOCUMENT_SCHEMA` stays (the store needs it).

### 2.2 `ViewEmit`

```rust
/// 👁️ What a viewer may emit. Structurally cannot carry artifact or draft mutations.
pub struct ViewEmit<CM> {
    pub config_mutations: Vec<CM>,
    pub effects: Vec<HostEffect>,
    pub ui_dirty: UiDirtyScope,
}
```

There is **no** field, constructor or method on `ViewEmit` that accepts an artifact or draft
mutation. That is the read-only guarantee: it is a type property, not a runtime check.

`ArtifactViewer` members: `Snapshot`, `Mutation` (decode-only — the store's op log must still decode),
`Config`, `ConfigMutation`, `Presence`, `PresenceMutation`, `Transient`, `TransientMutation`,
`Command`; `initial_snapshot`, `initial_config`, `config_schema`, `config_spec`, `ephemeral`,
`handle(...) -> Result<ViewEmit<Self::ConfigMutation>, Fault>`, `render`, `window_engagements`,
`window_measures`, `tool_measures`, `context_menu`, `interaction_topology`, `export_media`,
`media_fingerprint`, `io`, `media_ports`, `app_schema`, `command_id`, `command_from_action`.

**Absent from `ArtifactViewer` entirely** (not defaulted-to-error — absent): `Draft`,
`DraftMutation`, `initial_draft`, `genesis`, `import_media`, `whole_document_operation`,
`clipboard_media_type`, `clipboard_accepts`, `copy_fragment`, `cut_operations`, `paste_operations`,
`pending_effects`.

### 2.3 Role guard on `VcsArtifactApp`

`VcsArtifactApp<A: ArtifactApp>` stays one wrapper. It reads `A::ROLE` (the runtime trait carries it)
and, for `Viewer`:

- rejects `undo`, `redo`, `checkpoint`, `alternative`, `revert-to-command`, `cut`, `paste`, `import`
  with `Fault { origin: FaultOrigin::Framework, code: FaultCode::new("viewer.read-only"), .. }`;
- treats any non-empty `artifact_mutations` reaching the wrapper as a hard SDK fault with the same
  code (unreachable through `ViewerApp`, but the guard makes a hand-written runtime impl safe too);
- renders the history panel read-only;
- attaches its store with `Rights::Read` only (`kernel::CapabilityRequirement`).

**Frozen fault code strings** (exact, used by both hosts, both shells and the e2e script):

| code | meaning |
|---|---|
| `viewer.read-only` | a mutating operation was attempted on a viewer instance |
| `surface.unknown-dialect` | no surface registered for `(dialect, role)` |
| `surface.contribution-not-permitted` | contributor does not list the owner in `dependencies` |
| `surface.conflict` | the same `AppRef` was registered twice |
| `surface.missing-owner-surface` | an owned subset has no owner surface for a role |

`FaultOrigin::Framework` for all five.

### 2.4 Builders

```rust
Viewer::builder(V::DIALECT)   // no .mutation(...) method exists on this builder type
Editor::builder(E::DIALECT)
PluginBuilder::viewer::<V>(def) / ::editor::<E>(def)
ArtifactContribution::builder(kind).viewer::<V>(def) / .editor::<E>(def)
```

`Viewer::builder` and `Editor::builder` are **separate types**, not one `AppBuilder` with a flag, so
`.mutation(…)` on a viewer is a compile error rather than a runtime rejection.

### 2.5 testkit

```rust
pub fn assert_viewer_never_mutates<V: ArtifactViewer>();
pub fn assert_editor_and_viewer_share_dialect<E: ArtifactEditor, V: ArtifactViewer>();
pub fn new_viewer<V: ArtifactViewer>() -> VcsArtifactApp<ViewerApp<V>>;
```

### 2.6 Window kits (new region `//#region 🔖️WindowKits`)

Six kits, frozen ids and en/de labels:

| type | kind id | en | de |
|---|---|---|---|
| `TextWindowKit` | `framework.window.text` | Text | Text |
| `TableWindowKit` | `framework.window.table` | Table | Tabelle |
| `TreeWindowKit` | `framework.window.tree` | Tree | Baum |
| `ImageWindowKit` | `framework.window.image` | Image | Bild |
| `MeshWindowKit` | `framework.window.mesh` | Mesh | Netz |
| `DocumentWindowKit` | `framework.window.document` | Document | Dokument |
| `MediaWindowKit` | `framework.window.media` | Media | Medien |

(Seven rows — the plan says "six kits" and then lists seven; the list is authoritative, all seven ship.)

Each kit exposes:
```rust
fn window_kind() -> WindowKindDefinition;                 // read-only variant
fn editable_window_kind() -> WindowKindDefinition;        // emits the typed commands below
fn render(view: &<Kit as WindowKit>::ViewModel) -> UiNode;
```
Editable command ids, frozen: `replace-text`, `set-cell`, `set-node`, `set-pixel-region`,
`set-vertex`, `set-page`, `seek-media`.

## 3. C3 — Hosts

`AppRouter`: `(ArtifactDialect, AppRole) -> Vec<AppRef>`, built from every loaded `PluginManifest`.
Deterministic order: the owner plugin's surface first, then remaining entries sorted by `plugin_id`
ascending, then `app_id` ascending. Duplicate `AppRef` ⇒ `surface.conflict`.

`OpeningResolver::resolve(dialect, role, prefs) -> Result<AppRef, Fault>`:
1. explicit user default from `OpeningPreferences`, if it is still present in the router;
2. the owner plugin's surface;
3. the first router entry;
4. otherwise `surface.unknown-dialect`.

At plugin load, every **owned** subset must resolve for **both** roles, else
`surface.missing-owner-surface`.

### OS commands (frozen ids and labels)

Home: `🧰️framework/🛍️products/💻️os/🎮️commands/<cmd>/🦀️component.rs`.

| command id | en | de |
|---|---|---|
| `os.open-artifact` | Open Artifact | Artefakt öffnen |
| `os.open-artifact-with` | Open With… | Öffnen mit… |
| `os.set-default-viewer` | Set Default Viewer | Standard-Betrachter festlegen |
| `os.set-default-editor` | Set Default Editor | Standard-Editor festlegen |
| `os.clear-default-app` | Clear Default App | Standard-App zurücksetzen |

Channel: `AppCommand` gains, **appended after the last tag the peer ticket 26/08/16
PLUGIN-DEPENDENCIES reserved (its `TransactionRedo` = 26)**, therefore starting at **tag 27**:

```
27 OpenArtifact      { seq: u64, artifact_ref: String, role: u8, plugin_id: String, app_id: String }
28 SetDefaultApp     { seq: u64, artifact_kind: String, standard: String, subset: String, role: u8, plugin_id: String, app_id: String }
29 ClearDefaultApp   { seq: u64, artifact_kind: String, standard: String, subset: String, role: u8 }
```

`role: u8` = `0` Viewer, `1` Editor (declaration order of `AppRole`). Empty `plugin_id`/`app_id` on
`OpenArtifact` means "resolve via `OpeningResolver`". Flat fields, one level deep, matching the
hand-rolled codec's existing style. `CHANNEL_VERSION` is bumped by **one** past whatever the peer
ticket leaves it at — the executor reads the live constant and does not assume `9`.

Golden vectors: `🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/` as hex `.json`, asserted from
both Rust and vitest.

WIT `📜️world.wit`: **no change**. The app id already carries role and dialect. Recorded here so no
lane opens that file (it is under concurrent edit by two peer tickets).

## 4. C4 — Preferences

New OS-level config facet `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/` with the five schema
leaves (`🔣️component.json`, `🛰️component.proto`, `🔗️component.graphql`, `🟦️component.ts`,
`🦀️component.rs`).

- Schema id: **`os.config.opening`**.
- Record: `OpeningPreferences { defaults: Vec<DefaultApp> }`,
  `DefaultApp { dialect: ArtifactDialect, role: AppRole, app: AppRef }`.
- Mutations (each a full `{🦠️mutation, 🔺️diff, ↩️inverse}` triad under
  `🎚️config/🧬️schema/🧬️mutations/<name>/`):
  - `set-default-app` — schema kind id `os.config.opening#set-default-app`
  - `clear-default-app` — schema kind id `os.config.opening#clear-default-app`
- Applied through `ConfigStore`; materialized by `OpeningResolver`. Event-sourced: the resolver reads
  a **fold over the config op log**, never a mutable map. No CRUD, no CRDT.
- Lane: persisted **local-only** (`🎚️config`), per the four-lane rule.

Taxonomy: `osChildDirs += "🎚️config"`.

## 5. C5 — UI

- A session is `(artifact_ref, AppRef)`. The role is read off the resolved `AppDefinition.role`,
  never inferred from the id string at runtime.
- Window title chip: en **"Viewer"** / de **"Betrachter"**; en **"Editor"** / de **"Editor"**.
- Read-only badge on viewer sessions; viewer chrome hides every `Mutation`-kind action/utility and
  disables undo/redo.
- Context-menu + palette entry: en **"Open with…"** / de **"Öffnen mit…"**. Palette command ids
  `open-artifact-with-viewer`, `open-artifact-with-editor`.
- "Set as default" toggle: en **"Set as default"** / de **"Als Standard festlegen"**.
- Settings sub-tab `PanelTabKind::SettingsDefaultApps`: en **"Default apps"** / de
  **"Standard-Apps"**. Table of dialect × {viewer, editor} with selects; writes go **only** through
  the OS commands in §3.
- Both shells read the role from `SEMIO_APP_ROLE` / `VITE_SEMIO_APP_ROLE`, values `viewer`|`editor`,
  default `editor`.

There is **no default language**: every label above ships as a `LocalizedLabel` with both `en` and
`de` populated, English declared first.

## 6. C6 — Taxonomy

`🔣️taxonomy.json` `schemaVersion` **4 → 5**. Additive keys land in W0; retired keys are deleted only
in W3, after the last `🎛️apps` directory is gone (APA lesson: the Rust `assert!` gate panics if a
listed facet dir is missing on any plugin, so ordering is not optional).

**Added in W0 (additive only):**

```jsonc
"viewerDirName": "👁️viewer",
"editorDirName": "✏️editor",
"surfaceRoles": ["viewer", "editor"],
"surfaceDirNames": { "viewer": "👁️viewer", "editor": "✏️editor" },
"subsetSurfaceDirs": ["👁️viewer", "✏️editor"],
"subsetRequiredSurfaceDirs": ["👁️viewer", "✏️editor"],
"contributedSubsetChildDirs": ["👁️viewer", "✏️editor"],
"surfaceChildDirs": ["🎭️modes","🎮️commands","📌️panels","🎚️config","👥️presence","🫧️transient","🗣️terminology","🌉️wasm","📚️examples"],
"surfaceRequiredChildDirs": ["🎭️modes","🎮️commands","🎚️config","👥️presence","🫧️transient"],
"surfaceComponentLangs": ["🦀️rust","🟦️typescript"],
"surfaceSchemaSpecFilenames": { "🎚️config/🧬️schema": "🔣️component.json", "👥️presence/🧬️schema": "🔣️component.json", "🫧️transient/🧬️schema": "🔣️component.json" },
"windowLeafLangs": ["🦀️rust","🟦️typescript"],
```

plus `subsetChildDirs += "👁️viewer", "✏️editor"` and `osChildDirs += "🎚️config"`.

**`semanticCollections` is NOT extended** — see §7.7.
**`semanticAllowedOwnerLevels` is NOT extended** — see §7.8.

**Deleted in W3 only:** `appsDirName`, `appChildDirs`, `appComponentDirs`, `appSchemaSpecFilenames`;
`pluginChildDirs` → `["🎮️commands"]`; `semanticCollections` drops `"🎛️apps"`.

`🎛️apps` stays in `semanticCollections` and `pluginChildDirs` through W0–W2.

### Policies (root `📜️script.ts`)

Added in W0/W1 (all **add-only**, no existing policy changes shape until W3):

| policy | rule |
|---|---|
| `policySubsetSurfaceCompletenessBreaches` | every **owned** subset has `👁️viewer` and `✏️editor`, each with ≥1 mode with ≥1 window carrying both `windowLeafLangs` leaves |
| `policyViewerPurityBreaches` | nothing under a `👁️viewer` dir contains `.mutation(`, `Emit::mutations`, `artifact_mutations`, or a `::editor::` path |
| `policyContributedSurfaceTargetBreaches` | a surface dir under a kind the plugin does not own ⇔ a `.depends_on(<owner>)` on that plugin's builder |
| `policyOsConfigShapeBreaches` | `💻️os/🎚️config` carries the five schema leaves and both mutation triads |

Changed in W3: `policyTaxonomyDirsBreaches` (descend `🏅️standards/🔖️*/🪆️subsets/*` — it does not
today, see the ENGINELESS finding at `📜️script.ts:4076+`), `policyWindowCompletenessBreaches`
(`:4835`) and `policyModeCompletenessBreaches` (`:4940`) walk the surface dirs instead of
`taxonomy.appsDirName`.

## 7. Decisions the plan left open (recorded, not silently narrowed)

1. **`AppDefinition.dialect` is `ArtifactDialect`, not `Dialect`.** The plan wrote `Dialect`;
   `Dialect` is `&'static str`-based and cannot be deserialized. The compile-time `Dialect` stays on
   the SDK traits. This is a spelling correction, not a scope change.
2. **Seven window kits, not six.** The plan's prose says six and its list names seven; the list wins.
3. **Channel tags start at 27, not 22.** Peer ticket 26/08/16 PLUGIN-DEPENDENCIES froze 22–26. The
   executor reads the live enum and appends after the true last variant rather than trusting either
   number.
4. **`APP_ID` is removed from the authoring traits**, because a hand-written id and a derived id are
   two sources of truth for the same string. The plan said the id "becomes derived and canonical";
   leaving `APP_ID` writable would make that false.
5. **`🌉️wasm` and `📚️examples` are in `surfaceChildDirs` but not required.** The plan lists them in
   the child set without marking them required; only the five in `surfaceRequiredChildDirs` are
   mandatory, matching how `appChildDirs`/`modeRequiredChildDirs` already split the two concerns.
6. **Empty required facets carry `📌️empty.md`** (`taxonomy.windowEmptyFacetFilename`), the mechanism
   that already exists, rather than an empty directory git cannot track.
7. **`semanticCollections` is NOT extended with `👁️viewer` / `✏️editor`.** The plan's C6 asked for it.
   Measured against the real validator: `semanticCollectionSpec`
   (`📚️library/🔍️discovery/🟦️component.ts:1470`) matches a collection by **path suffix**, and a
   collection root must carry a `🔣️component.json` whose `x-semio.members` are in **exact bijection
   with its direct child directories** (`semanticReadManifest`, `:1521`). A surface's direct children
   are taxonomy *vocabulary* dirs (`🎭️modes`, `🎮️commands`, `🎚️config`, …), not semantic members —
   so the addition would create **286 phantom collection roots**, each demanding a manifest listing
   words it does not own. A surface is a subset **facet**, exactly like `🧬️schema` and `🚪️io`, and
   neither of those is a `semanticCollection` either. The facet is governed by `subsetChildDirs` +
   `surfaceChildDirs` + `subsetRequiredSurfaceDirs`, which is the same mechanism `🧬️schema` uses.
8. **W2 gets a permanent `new surface` scaffolder, not a migration script.** Measured: 286 surfaces ×
   ~19 mandatory files (2 surface leaves + 1 mode leaf + 2 window leaves + 6 window facets + 4 mode
   facets + 4 surface facets) ≈ **5 400 files**, every one of them pure taxonomy shape with zero
   per-subset meaning. Hand-typing that is not "thorough", it is a guaranteed source of the exact
   defect class the ENGINELESS ticket names — a hand-encoded emoji path that looks right and does not
   exist (`🫣️fill` vs `🪣️fill`). CLAUDE.md forbids *migration* scripts and requires permanent
   scripts to live in `📜️script.ts`; a `new surface` generator is permanent repo tooling of exactly
   the kind `📇️registry/📜️script.ts` already hosts, and it derives every path from `🔣️taxonomy.json`
   rather than from a hardcoded list. Lane 1-E owns it:
   `bun ./📜️script.ts new surface <plugin> <kind> <standard> <subset> <role>`, registered in
   `.vscode/launch.json` through `📇️registry/🖥️launch.ts`.
   **The scaffolder emits shape only.** Every surface's actual render, view-model, commands and
   labels are handcrafted per subset by the W2 packet that owns it. A surface left at scaffold
   content is a W2 failure, and `policySubsetSurfaceCompletenessBreaches` is extended to catch it.
9. **`semanticAllowedOwnerLevels` is NOT extended with `"surface"`.** `semanticOwnerLevel`
   (`:1501`) derives the level from the *parent* directory name and `semanticOwnerAncestry` (`:1489`)
   only walks a fixed collection set; neither can ever produce a surface-level owner, so the entry
   would be unreachable vocabulary that three separate places (the JSON, the `SemanticOwnerLevel`
   union at `:30`, the hardcoded set at `:823`) would have to agree on for no behavioural gain.
