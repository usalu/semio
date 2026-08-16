# Artifact Viewers and Editors — per-subset, plugin-contributable, user-configurable

## Context

Today the OS has one general **app** system: a plugin ships `🎛️apps/<app>/🎭️modes/<mode>/🪟️windows/<window>/🦀️component.rs`, an `App`/`AppDefinition` (manifest) bound to exactly one document schema through `ArtifactApp::{APP_ID, DOCUMENT_SCHEMA, Snapshot, Mutation}`, and the shells (React `ShellHost`, wgpu shell) open "the app" for a document. There is **no** notion of viewer vs editor, no read-only enforcement, no "open with…", no default app per artifact kind, and library plugins (`🗄️stdio` 36 kinds, `🔋️energy`, `🎪️demonstrator`) have artifacts with **zero** UI. Modes/windows/apps are enforced by `🔣️taxonomy.json` (`appsDirName`, `appChildDirs`, `modeChildDirs`, `windowChildDirs`, `windowComponentLangs`) via three validators (root `📜️script.ts` policies, `📇️registry/📜️script.ts`, Rust taxonomy gate in `🔌️plugin/🦀️component.rs`).

Direction already settled by live tickets: an artifact = `🧬️schema` + `🚪️io` (+ examples), never an engine (26/08/12 ENGINELESS); plugin = `🎛️apps` + `🗿️artifacts` (26/08/12 APA); four state lanes `🗿️artifacts / 🎚️config / 👥️presence / 🫧️transient` on every state-owning scope (26/08/13 UNIFIED-STATE); plugin dependencies + contributions on foreign artifacts + host routers (26/08/16 PLUGIN-DEPENDENCIES… — **open, in flight**, W0 landed: `PluginManifest.dependencies/contributions`, `.depends_on`, `ArtifactContribution` builder skeleton, policy gates).

**Goal of this ticket:** replace the general app system by a specific one — every artifact **subset** has a **viewer** (read-only) and an **editor**; plugins can register viewers/editors for subsets they do not own; the user can open an artifact in any registered viewer/editor and configure per-subset defaults; everything schema-first, event-sourced, both hosts (Rust wasmtime + TS browser) and both shells (React + wgpu) in parity, en/de labels, launch.json regenerated, gates green.

Decisions taken with the dev (this session):
- **Granularity:** per **subset**, all explicit — `…/🪆️subsets/<subset>/👁️viewer` and `…/✏️editor` beside `🧬️schema/🚪️io/📚️examples` (143 subsets today → ~286 surfaces, each ≥1 window).
- **Modes stay mandatory:** `👁️viewer/🎭️modes/<mode>/🪟️windows/<window>/` and `✏️editor/🎭️modes/<mode>/🪟️windows/<window>/` (same shape as today's apps).
- **Foreign viewers/editors** mirror the owner's path under the contributor's own `🗿️artifacts/<foreign-kind>/🏅️standards/🔖️<v>/🪆️subsets/<subset>/👁️viewer|✏️editor` (no `🧬️schema/🚪️io`), marked `x-semio.contribution` in the artifact's `🔣️component.json`; runtime via `.depends_on` + contribution router.
- **SDK types split:** `ArtifactApp` (today) → `ArtifactEditor`; new `ArtifactViewer` whose `handle` returns a `ViewEmit` that cannot carry artifact mutations; both adapt into the one runtime contract via `EditorApp<E>` / `ViewerApp<V>`; `PluginBuilder::viewer::<V>()` / `.editor::<E>()` replace `document_app`.

Workforce (dev instruction): **one Fable** (this plan; on approval opens the ticket and spawns the coordinator; does not execute lanes) → **one Opus 5 coordinator** (`Agent`, `general-purpose`, `model: opus`, background) → **Sonnet 5 executors** (`model: sonnet`, one per lease) → **Haiku 4.5 explorers/auditors** (`model: haiku`, `Explore`). Never `isolation: worktree`, never git-modifying commands, subagents never call `ticket_close`; coordinator closes with explicit ticket path + full file list; `📌️important.md` cleared last; scratch logs as `.txt` inside the ticket folder.

## Target shape (SSOT: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`)

```
✏️s/🔌️plugins/<plugin>/
  🗿️artifacts/<kind>/                       (owned OR contributed — manifest fact)
    🔣️component.json                       (x-semio: {kind, owner|contribution:"s.<owner>.<kind>"})
    🏅️standards/🔖️<v>/🪆️subsets/<subset>/
      🧬️schema/ 🚪️io/ 📚️examples/          (owned subsets only — unchanged)
      👁️viewer/                             (NEW, required for every owned subset)
        🦀️component.rs                      (viewer definition + ArtifactViewer impl)
        🟦️component.ts                      (TS twin: surface binding + typed view-model exports)
        🎭️modes/<mode>/                     (≥1; layout preset + tools + lanes; never owns windows' code beyond 🪟️windows)
          🦀️component.rs
          🪟️windows/<window>/{🦀️component.rs, 🟦️component.ts, 🎬️actions, 🪛️utilities, 🎚️options, 🎚️config, 👥️presence, 🫧️transient, 🍱️panes?, 🪀️widgets?}
          🎮️commands/ 🎚️config/ 👥️presence/ 🫧️transient/
        🎮️commands/ 🎚️config/ 👥️presence/ 🫧️transient/ 📌️panels/ 🗣️terminology/ 📚️examples/ 🌉️wasm?
      ✏️editor/                             (NEW, required for every owned subset; same shape)
  🎮️commands/  📦️packages/  🦀️component.rs  AGENTS.md
```
`🎛️apps` is **deleted repo-wide** at the end (taxonomy `appsDirName/appChildDirs/appComponentDirs/pluginChildDirs` retire; `surface*` keys replace them). Every window carries **both** `🦀️component.rs` and `🟦️component.ts` (`windowComponentLangs`), extended from window facet items to the window itself.

Sharing rule (enforced by policy): pure snapshot→view-model transforms live in the subset's `🧬️schema/💡️inferences`; **editor windows may reuse viewer windows' render; a viewer never depends on an editor**; nothing under `👁️viewer` may reference `crate::…::editor::`.

## Contracts (freeze in `📋️contract-freeze.md`, wave 0)

### C1 Identity
- `AppRole { Viewer, Editor }` (manifest, ts-rs mirrored). `Dialect { artifact_kind, standard, subset }` already exists (`🔌️plugin/🦀️component.rs` `subset!` macro) — reuse as the surface's target.
- `AppDefinition` (`🛂️manifest/🦀️component.rs:2630`) gains required `role: AppRole`, `dialect: Dialect` (replaces the informal `.document([...])` + `artifact_kinds` binding for surfaces); `id` becomes derived and canonical: `"<kind>@<standard>/<subset>#<role>"` (e.g. `s.cad.cad@1/*#editor`); a contributed surface is addressed by `AppRef { plugin_id, app_id }`.
- `PluginManifest.apps` keeps carrying every surface (owned + contributed); host validates: owned dialect ⇒ owner plugin; foreign dialect ⇒ target plugin ∈ `dependencies` (mirrors `register_contributions` rules).

### C2 SDK (`🔌️plugin/🦀️component.rs`, `🏗️builder/🦀️component.rs`)
- `ArtifactEditor` = renamed `ArtifactApp` (`:7259`) with `const ROLE = Editor`, `const DIALECT: Dialect`.
- `ArtifactViewer`: `Snapshot/Mutation` (mutation type only to decode the store), `Config/ConfigMutation`, `Presence*`, `Transient*`, `Command`; `handle(...) -> Result<ViewEmit<ConfigMutation>, Fault>` (config + ephemeral + effects, **no artifact/draft mutations representable**); `render`, `window_engagements/measures`, `context_menu`, `export_media` (read side only), no `import_media/paste/cut/genesis/whole_document_operation`.
- Adapters `EditorApp<E>` / `ViewerApp<V>` implement the single runtime authoring trait `ArtifactApp` (kept as the runtime name; role-carrying) so `VcsArtifactApp<A>` (`:8011`) stays one wrapper. `VcsArtifactApp` role guard: viewer instance rejects undo/redo/checkpoint/alternative/clipboard cut+paste/import with `Fault{origin: Framework, code: "viewer.read-only"}`; any `artifact_mutations` from a viewer is a hard SDK fault; history panel renders read-only; store attached with `Rights::Read` backbone capability (`kernel::CapabilityRequirement`).
- Builders: `Viewer::builder(DIALECT)` (no `.mutation()` — unrepresentable) / `Editor::builder(DIALECT)`; `PluginBuilder::viewer::<V>(def)` / `.editor::<E>(def)` (replace `document_app`, `:148`); `ArtifactContribution::builder(kind).viewer::<V>()/.editor::<E>()` for foreign subsets (extends the in-flight builder — coordinate region ownership with ticket 26/08/16).
- `testkit`: `assert_viewer_never_mutates::<V>()`, `assert_editor_and_viewer_share_dialect::<E,V>()`, `new_viewer::<V>()`.
- Framework **window kits** (new region `🔖️WindowKits` in the SDK): `TextWindowKit`, `TableWindowKit`, `TreeWindowKit` (json/xml/dsl), `ImageWindowKit`, `MeshWindowKit` (World3d), `DocumentWindowKit` (pdf/docx/pptx pages), `MediaWindowKit` (audio/video) — each yields `WindowKindDefinition` + `render(snapshot-view-model) -> UiNode` for read-only and an editable variant emitting typed commands (`replace-text`, `set-cell`, …). Thin per-subset surfaces (stdio's 88) declare a kit + the subset's inference view-model.

### C3 Hosts (parity Rust `🔌️plugin/🖥️host/🦀️component.rs` + `💻️os/🖥️host/🦀️component.rs` ↔ TS `🎠️kernel/🟦️component.ts` + `💻️os/🟦️component.ts`)
- `AppRouter` (mirrors `ArtifactInferenceRouter`): `(dialect, role) → Vec<AppRef>` from every loaded manifest; conflict = same `AppRef` twice; contribution gate = contributor lists owner in `dependencies`; deterministic order (owner first, then plugin id).
- `OpeningResolver`: `resolve(dialect, role, prefs) → AppRef` = explicit user default → owner plugin's surface → first router entry. Every owned subset MUST resolve for both roles (assert at load; policy gate on disk).
- New OS commands (`💻️os/🎮️commands/<cmd>/🦀️component.rs`, `osChildDirs` already `🎮️commands`): `open-artifact {artifact_ref, role, app?: AppRef}`, `open-artifact-with`, `set-default-viewer {dialect, app}`, `set-default-editor`, `clear-default-app`. Wire: `AppCommand::OpenArtifact{…}` in `📡️spr/🧵️channel` + TS `AppChannelCodec` mirror + golden vectors under `💻️os/🧫️fixtures/📡️channel/`.
- WIT `📜️world.wit`: no new imports; `instantiate-app` gains nothing (app id already carries role/dialect); regenerate bindings; jco map untouched except transpile.

### C4 Preferences (persisted local-only = `🎚️config` lane, event-sourced, no CRUD)
- New OS-level config facet `💻️os/🎚️config/🧬️schema/{🔣️component.json,🛰️component.proto,🔗️component.graphql,🟦️component.ts,🦀️component.rs}`: `OpeningPreferences { defaults: Vec<DefaultApp { dialect, role, app: AppRef }> }` (schema id `os.config.opening`), mutations `set-default-app`, `clear-default-app` (each `{🦠️mutation,🔺️diff,↩️inverse}` triad), applied through `ConfigStore` (`🏪️store`), materialized by `OpeningResolver`. TS twin decodes the same pack for the browser host. Add `osChildDirs += 🎚️config` in taxonomy.

### C5 UI (React `📺️renderer/🧑️‍🎨️engine/🧱️elements/{ShellHost,ChromePanels,ShellHelpers}`, wgpu `🖱️ui/…/🧊️wgpu/🦀️shell.rs`, `🦀️chrome.rs`)
- Sessions become `(artifact_ref, AppRef)`; window title chip `Viewer`/`Editor` (en "Viewer"/"Editor", de "Betrachter"/"Editor"), read-only badge; viewer chrome hides Mutation-kind actions/utilities and disables undo/redo.
- "Open with…" (en "Open with…", de "Öffnen mit…") in artifact/document context menus, command palette (`open-artifact-with-viewer`, `open-artifact-with-editor`) and the Document panel; lists `AppRouter` entries grouped by role with plugin labels; "Set as default" toggle.
- Settings tab `SettingsGeneral` → new sub-tab **Default apps** (`PanelTabKind::SettingsDefaultApps`, en "Default apps"/de "Standard-Apps"): table dialect × {viewer, editor} with selects; writes only via the OS commands above.
- Both shells load with `role` from launch/playground (`SEMIO_APP_ROLE=viewer|editor`, default editor).

### C6 Taxonomy / gates / codegen
- `🔣️taxonomy.json` (schemaVersion 5): `viewerDirName:"👁️viewer"`, `editorDirName:"✏️editor"`, `surfaceRoles:["viewer","editor"]`, `subsetChildDirs += 👁️viewer, ✏️editor`, `subsetRequiredSurfaceDirs:[👁️viewer,✏️editor]` (owned subsets), `contributedSubsetChildDirs:[👁️viewer,✏️editor]`, `surfaceChildDirs:[🎭️modes,🎮️commands,📌️panels,🎚️config,👥️presence,🫧️transient,🗣️terminology,🌉️wasm,📚️examples]`, `surfaceRequiredChildDirs:[🎭️modes,🎮️commands,🎚️config,👥️presence,🫧️transient]`, `surfaceComponentLangs:[🦀️rust,🟦️typescript]`, `windowLeafLangs:[🦀️rust,🟦️typescript]` (window itself), `semanticCollections += "👁️viewer":{kind:"viewer"}, "✏️editor":{kind:"editor"}`; **retire** `appsDirName, appChildDirs, appComponentDirs, appSchemaSpecFilenames` (move to `surfaceSchemaSpecFilenames`), `pluginChildDirs → ["🎮️commands"]`, `osChildDirs += 🎚️config`. Sequence per APA lesson: add new keys + validators first (wave 0), delete old keys + walkers only after the last `🎛️apps` dir is gone (wave 3), never in between (Rust `assert!` gate `🔌️plugin/🦀️component.rs` `TaxonomyJson` region and `📇️registry` gate panic otherwise).
- Validators/policies to teach the shape: `📚️library/🔍️discovery/🟦️component.ts` (`validateTaxonomy`, types), `📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`, root `📜️script.ts` (`policyTaxonomyDirsBreaches` walker must descend `🏅️standards/🔖️*/🪆️subsets/*` — it currently doesn't; `policyWindowCompletenessBreaches` `:4774` and `policyModeCompleteness…` walk `👁️viewer|✏️editor` instead of `🎛️apps`; new `policySubsetSurfaceCompletenessBreaches` (every owned subset has both surfaces with ≥1 mode ≥1 window with both leaves), `policyViewerPurityBreaches` (no `.mutation(`/`Emit::mutations`/`crate::…::editor::` under `👁️viewer`), `policyContributedSurfaceTargetBreaches` (contributed dir ⇔ `.depends_on`), `policyOsConfigShapeBreaches`), `📇️registry/📜️script.ts` (discover surfaces, `mutationDirChildDirs`-style selection, playground rows: `[[package.metadata.semio.playground]] app = "<surface id>"`, `resolve_playground_app_id`, `defaultAppId`, generated `🤖️generated/🟦️plugins.ts` / `🟦️session.ts` / `🦀️hosts`), Rust taxonomy gate (`TaxonomyJson` region), `LAUNCH-JSON` generator (`📇️registry/🖥️launch.ts`) → `.vscode/launch.json` regenerated (`🛠️dev📐️cad⚛️react` keeps its slot; add `👁️viewer` variants under the same group/order as `x.1`).

## Waves, leases, workforce

Shared-tree rules: other live sessions edit `🔌️plugin/🦀️component.rs`, `🛂️manifest/🦀️component.rs`, `📜️world.wit`, root `📜️script.ts`, `🔣️taxonomy.json` (tickets 26/08/16 PLUGIN-DEPENDENCIES… and FULL-STDIO…). Every writer re-reads the region right before editing, edits by region with `Edit`, never whole-file rewrites, never reverts foreign hunks; coordinator records touched regions in `📋️ownership-and-handoffs.md`, reads the other tickets' `📌️important.md` at each barrier, and settles attribution with `git log --date=iso` vs the ticket start commit (never commit-message dates). Cargo gates serial, `RUSTC_WRAPPER="" cargo check --all-targets --keep-going`. Beware the repo-wide single-file consolidation tool: verify split trees still split after every barrier. Never `git stash/commit/checkout`.

| Wave | Lane | Model | Exclusive lease | Deliverable / acceptance |
|---|---|---|---|---|
| W0 | 0-I coordinator | Opus | ticket docs, `📋️contract-freeze.md`, `📋️ownership-and-handoffs.md`, `🔣️taxonomy.json` (add-only keys), `📚️library/🔍️discovery/🟦️component.ts` + its test | Contract frozen (C1–C6 ids/strings/labels); taxonomy schemaVersion 5 additive; `bun nx run @semio-tech/repo-lib:test` no new failures |
| W0 | 0-A manifest spine | Sonnet | `🛂️manifest/🦀️component.rs` regions `AppDefinition/ModeDefinition/PanelTabKind`, ts-rs regen | `AppRole`, `AppRef`, `AppDefinition.{role,dialect}`, canonical id fn, `PanelTabKind::SettingsDefaultApps`; unit tests |
| W0 | 0-B SDK spine | Sonnet | `🔌️plugin/🦀️component.rs` regions `app` (trait split, adapters, `VcsArtifactApp` role guard, `AppBuilder`→`Viewer/Editor` builders, testkit), `🏗️builder/🦀️component.rs` | C2 without window kits; `cargo test -p semio-framework-plugin` green (compile of plugins deferred to W2 — keep old `document_app` **absent**, not deprecated: W2 rewires every call site) |
| W0 | 0-C channel + config spine | Sonnet | `📡️spr/🧵️channel/🦀️component.rs`, `💻️os/🟦️component.ts` `AppChannelCodec`+tests, `💻️os/🧫️fixtures/📡️channel/`, new `💻️os/🎚️config/**`, `💻️os/🎮️commands/**` | C3 commands + C4 schema quadruple + triads; golden vectors decode identically Rust/TS |
| W0 | 0-D window kits | Sonnet | `🔌️plugin/🦀️component.rs` new region `🔖️WindowKits` + TS twins in `🔌️plugin/📦️packages/🟦️typescript/🪟️window-kits/` | six kits, en/de labels, unit tests rendering `UiNode` for fixture view-models |
| W0 | 0-H Haiku scouts ×3 | Haiku | read-only | (1) exact list of 143 subsets × owning/derived × snapshot type per plugin → `📓️subset-inventory.md`; (2) all consumers of `appsDirName/appChildDirs/modeChildDirs/pluginChildDirs/document_app/App::builder/create_*_app` (files:lines) → `📓️consumer-inventory.md`; (3) shells' current open/spawn/session code paths (`ShellHost` `catalog/landingAppId/spawnPluginInstance/openPluginInstance`, wgpu `🦀️shell.rs`) → `📓️shell-inventory.md` |
| W1 | 1-A Rust hosts | Sonnet | `🔌️plugin/🖥️host/🦀️component.rs` (new regions `AppRouter/OpeningResolver`), `💻️os/🖥️host/🦀️component.rs` regions `PluginRegistry/ResourceDescriptors`, `🏃️run/**` | C3 Rust; wasmtime e2e: load 2 plugins, router lists owned+contributed, resolver honors prefs; typed rejections |
| W1 | 1-B TS host | Sonnet | `🎠️kernel/🟦️component.ts` (new regions), `💻️os/🟦️component.ts` `AppChannelClient`, `💻️os/🟦️backbone-worker.ts` (config lane attach) | C3/C4 TS twins; vitest e2e with jco-materialized plugin |
| W1 | 1-C React shell | Sonnet | `📺️renderer/…/🧱️elements/{ShellHost,ChromePanels,ShellHelpers,Shell}/🟦️component.tsx`, `🖱️ui/🧱️elements/{ContextMenu,📑️PanelTabBar}` if needed | C5 React: role sessions, read-only chrome, Open with…, Default apps settings tab, en/de |
| W1 | 1-D wgpu shell | Sonnet | `🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/{🦀️shell.rs,🦀️chrome.rs,🦀️component.rs}`, `📺️renderer/…/🧊️wgpu/**` | C5 wgpu parity |
| W1 | 1-E registry/gates/launch | Sonnet | `📇️registry/{📜️script.ts,🖥️launch.ts,🟦️catalog.ts,🤖️generated}`, root `📜️script.ts` policy region (new/changed policies), `.vscode/launch.json` (generated) | C6 policies (add-only), playground rows accept surface ids, launch.json regenerated, `verify gate` runs |
| W1 | 1-H Haiku audit | Haiku | read-only | parity audit Rust↔TS↔WIT of C1/C3/C4; report to coordinator before W2 |
| W2 | 2-P<n> plugin packets (parallel, ≤6 concurrent) | Sonnet ×N | one packet = one plugin (or plugin group) — exclusive lease on `✏️s/🔌️plugins/<plugin>/**` (incl. `📦️packages/🦀️rust/📦️glue.rs`, `Cargo.toml` playground rows, TS package) | per subset: create `👁️viewer` + `✏️editor` per shape; move `🎛️apps/<app>/**` → the owning subset's `✏️editor/` (windows keep their `🎭️modes/<mode>/🪟️windows` nesting; add `🟦️component.ts` twin per window); write viewer (reuse editor render via inferences, kits for thin kinds); rewire `#[path]` in `📦️glue.rs` (parse the real module path, never copy from siblings); replace `App::builder/.document/document_app` with `Editor::builder/Viewer::builder` + `PluginBuilder::editor/viewer`; foreign surfaces (if any) under mirrored path + `.depends_on`; labels en/de; testkit assertions; delete the emptied `🎛️apps/<app>`; `cargo check -p semio-s-plugin-<x> --all-targets` + `bun nx run <pkg>:test` + one `wasm` build; report `📓️w2-<plugin>-report.md` |
| W2 packets (sizing) | P1 stdio-a (18 kinds), P2 stdio-b (18 kinds), P3 norm (15), P4 cad+extensions, P5 flow+extensions+procedural, P6 puzzle+block+lowpoly+draw+raster, P7 gis+fem+energy+space+demonstrator, P8 trinity+imperative+playbook+dag+reasoning+forms+layout, P9 writer+mathematical+vcs+animate+shooting+sequence+architect+process+remodel+note+sourcing | | | stdio uses window kits almost everywhere; demonstrator/energy get their first surfaces; space's `studio` app needs a `🗿️artifacts/🪐️space` (workflow schema) subset first |
| W3 | 3-A dissolution | Sonnet | `🔣️taxonomy.json` (delete retired keys), root `📜️script.ts` walkers, `📇️registry/📜️script.ts`, Rust taxonomy gate, discovery types/tests | zero `🎛️apps` dirs; `appsDirName` gone; `verify gate` green for new rules |
| W3 | 3-B docs/examples | Sonnet | READMEs touched by moved paths, `💻️os/📚️examples`, `🧑️‍💻️dev` boot (`VITE_SEMIO_APP_ID` → surface id + role) | dev boots editor by default, viewer via env |
| W4 | 4-A conformance & e2e | Sonnet | `💻️os/🧫️fixtures/**`, host parity tests, browser e2e script in ticket folder | see Verification |
| W4 | 4-I audits + close | Opus + Haiku×3 | ticket docs | Haiku audits: (1) taxonomy/open-closed/parity, (2) read-only guarantee + CQRS (viewer cannot mutate on either host, prefs are event-sourced), (3) evidence honesty (every claim has a `.txt` log); remediation leases; `ticket_close` with explicit path + full file list |

Sizing: W0 = 4 Sonnet + Opus + 3 Haiku; W1 = 5 Sonnet + 1 Haiku; W2 = 9 packets, ≤6 concurrent Sonnet; W3 = 2 Sonnet; W4 = 1 Sonnet + 3 Haiku. Cargo gates never in parallel.

## Verification (end to end)

1. Spine: `cargo test -p semio-framework-plugin -p semio-framework-manifest` (crate names per `Cargo.toml`), channel/config vectors `bun nx run @semio-tech/os:test`; `assert_viewer_never_mutates` on every viewer via testkit.
2. Hosts: wasmtime test loads ≥2 real plugin components: `AppRouter` lists owner + contributed surfaces; `OpeningResolver` honors `set-default-app` then `clear-default-app`; viewer instance rejects `Undo`/`Apply` with `viewer.read-only`.
3. Browser (`🛠️dev📐️cad⚛️react` via preview browser): open the cad example → editor; "Open with… → Viewer": read-only chip, mutation actions hidden, dispatching a mutation from console yields the fault (`[DEBUG]` logs captured to ticket `.txt`, then removed); set default viewer for `s.cad.cad@1/*` → reload → resolver picks it (config lane persisted); wgpu native shell repeats the same script.
4. Gates: `bun ./📜️script.ts verify gate`; `bun ./📜️script.ts check` (registry + launch.json fresh); `bun nx run-many -t test` for touched packages; sampled `wasm` builds for every plugin packet; final `RUSTC_WRAPPER="" cargo check --workspace --all-targets --keep-going` (serial, long; attribute foreign failures via git log before acting).
5. Disk: `find ✏️s/🔌️plugins -type d -name '🎛️apps'` → 0; every owned subset has both surfaces with both window leaves (`policySubsetSurfaceCompletenessBreaches` = 0).

## Execution kickoff (on approval)

1. Fable reads `repo://goals` (done), opens the ticket: goal `r2602/runningsketchpad/runningsketchpadapps` (Apps within sketchpad; fall back to `🎯aioptimizedrepo` if rejected), client `claude-code`, title "Artifact Viewers and Editors per Subset", copies this plan into the ticket as `📋️master-plan.md`.
2. Fable spawns exactly one Opus 5 coordinator (background) with: plan + ticket paths, the shared-tree/barrier rules, roster sizes, "spawn Sonnet 5 executors per lease and Haiku 4.5 `Explore` scouts/auditors; never worktrees; never git-modifying commands; subagents never `ticket_close`; close yourself with the explicit ticket path and full file list; clear `📌️important.md` last".
3. Fable monitors coordinator notifications, relays blockers, and does not execute lanes itself.
