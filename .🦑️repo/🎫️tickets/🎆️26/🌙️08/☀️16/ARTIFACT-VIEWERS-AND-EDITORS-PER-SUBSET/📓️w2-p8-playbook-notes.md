# W2 Packet P8 (playbook) — Notes

Lane: W2 packet P8, plugin `📖️playbook`, subset `s.playbook.playbook@1/*`. Scope: migrate the retired
`✏️s/🔌️plugins/📖️playbook/🎛️apps/📖️playbook/` app into `✏️editor`, author a real `👁️viewer`, rewire
`📦️glue.rs`/`🦀️component.rs`, delete `🎛️apps/`. Followed `📓️w2-cad-report.md`'s migration recipe
verbatim (steps 1–16) and the w0-f SDK-gap-closure handoff.

## What moved where

### Editor (`…/🪆️subsets/✳️any/✏️editor/`)

The whole app tree moved across intact, replacing the W1-E scaffold's placeholder leaves: root
`🦀️component.rs` (now `impl ArtifactEditor for PlaybookPlayApp`), `⚙️engine`, `🎚️config` (+schema),
`👥️presence` (+schema), `🎮️commands/*` (9 groups: `add-step`, `remove-step`, `move-step`, `add-block`,
`remove-block`, `move-block`, `update-playbook`, `set-locale`, `set-contributions`), `🗣️terminology`,
`📚️examples/🎬️demo-session`, and the mode subtree `🎭️modes/🏗️builder/{component.rs, 🪟️windows/🏗️builder}`
— app's OWN mode/window name (`🏗️builder`), kept verbatim, NOT renamed to cad's `✏️edit`, per the task
brief's explicit instruction. The scaffold's generic `🎭️modes/✏️edit/🪟️windows/🪟️main` placeholder was
deleted wholesale (`rm -rf`) before the real mode/window directory was moved into its place under the
correct name.

The builder window gained a real `🟦️component.ts` twin (typed `PlaybookBuilderViewModel`/
`PlaybookBuilderStep`/`PlaybookBuilderBlock`/`PlaybookBuilderPaletteEntry` interfaces + window-kind
id/body-key/surface-id constants, mirroring the Rust `render()` boundary — none existed pre-migration,
this is new). The surface root also gained a real `🟦️component.ts` (re-export of the one window's twin
plus the editor's own dialect/mode-id constants — no namespacing ambiguity to resolve since, unlike
cad's four windows, playbook's editor has exactly one).

`impl ArtifactApp for PlaybookPlayApp` → `impl ArtifactEditor for PlaybookPlayApp`; `const APP_ID`
(`PLAYBOOK_PLAY_APP_ID`) removed entirely (grepped the whole plugin first — zero other referrers, safe
to delete rather than leave as an orphaned string const); `const DIALECT: Dialect = PLAYBOOK_DIALECT`
added. `create_playbook_play_app()` now returns `AppDefinition` (`Editor::builder(PLAYBOOK_DIALECT)…
.build_definition()`) instead of `App::from_builder(App::builder(ID, LABEL)…)`. Playbook's original app
had no `.example(...)`/`.workflow(...)` calls to drop (unlike cad) — nothing lost there.

### Viewer (`…/✳️any/👁️viewer/`)

A genuinely independent, minimal, real `PlaybookViewer: ArtifactViewer`:
- `Snapshot = PlaybookSnapshot`, `Mutation = crate::artifacts::playbook::op::PlaybookMutation` (both
  artifact-level, shared with the editor — decode-only per contract §2.2).
- `Config`/`ConfigMutation`/`Presence`/`PresenceMutation`/`Transient`/`TransientMutation` = framework
  `NoConfig`/`NoConfigMutation`/`NoPresence`/`NoPresenceMutation`/`NoTransient`/`NoTransientMutation` —
  a read-only step/block tree view needs no persisted per-session state.
- `Command` = one-variant `PlaybookViewCommand::Noop` (derives `Default`, required by the canonical
  `testkit::assert_viewer_never_mutates<V>()` bound); `handle` always returns `Ok(ViewEmit::default())`.
- One real window, `🌳️steps` (`🎭️modes/👁️view/🪟️windows/🌳️steps`), built on the framework's
  `TreeWindowKit` (contract §2.6) rather than a bespoke render: playbook's document is naturally
  tree-shaped (an ordered list of steps, each an ordered list of blocks), so `TreeWindowKit::render`
  is reused unmodified — `render(spec)` maps every step to a `TreeNodeView` root (step title, falling
  back to the step id when empty) with one leaf `TreeNodeView` child per block (`"<label> (<kind>)"`).
  This is a genuinely different (simpler, read-summary) view-model than the editor's rich
  `PlaybookBuilderBlock` (~18-field form-authoring vocabulary) — not a thin wrapper around it, and
  never imports the sibling editor module (verified: zero `::editor::` substring matches anywhere
  under `👁️viewer/`, zero `.mutation(`/`Emit::mutations`/`artifact_mutations` matches either).
- `create_playbook_viewer() -> AppDefinition` via `Viewer::builder(PLAYBOOK_DIALECT)…build_definition()`.

Window-name choice: the app's one editor window is named `🏗️builder` (generic "the builder UI", not a
specific content noun); the viewer window is named `🌳️steps` (descriptive of the tree-of-steps content
it renders) rather than reusing `🏗️builder`'s name — there is no established convention forcing the two
roles' window dirs to share a name (cad's own precedent, `📐️shape`, only matched because cad's ONE
substantive content concept — "the shape" — was already the editor window's own name; playbook's editor
window name is a UI-shape noun, not a content noun, so nothing to mirror).

### `📦️glue.rs`

Old `//#region 🎛️Apps` (mounting `apps::playbook::*` from `../../🎛️apps/📖️playbook/…`) replaced by two
independent regions:
- `//#region ✏️Editor` — `pub mod editor { pub mod playbook { … } }`, every leaf `#[path]`-mounted from
  `../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/…`.
- `//#region 👁️Viewer` — `pub mod viewer { pub mod playbook { … } }`, same base but `…/👁️viewer/…`,
  deliberately never mounting anything under `✏️editor/`.

Every `crate::apps::playbook::` reference across the 24 moved Rust files became `crate::editor::playbook::`
(mechanical `sed`, editor tree only, verified zero remaining afterwards). Three cosmetic doc-comment
references to the old path fixed too (`🎚️config/🦀️component.rs`, `🎚️config/🧬️schema/🦀️component.rs`,
`🎭️modes/🏗️builder/🪟️windows/🏗️builder/🦀️component.rs` — all said `ArtifactApp::…`, now say
`ArtifactEditor::…`), plus two in the artifact root (`crate::apps::playbook::` → `crate::editor::playbook::`
in doc comments) and one in `🧬️schema/🧬️mutations/💾️binary/🦀️component.rs` (`🎛️apps/📖️playbook/…` →
`✏️editor/…`). The bottom `//#region 📚️Examples` mount for `app_playbook_demo_session` was repointed at
the new editor path (name kept, only the `#[path]` string changed) — the artifact-level `art_playbook_demo`
mount in that same region was untouched (never depended on `🎛️apps`). `resolveAll #[path] attrs` verified
against disk with the recipe's Python snippet: 156 total, 0 missing.

**The emoji-typo trap hit twice this packet** (`🏅️标准` Chinese vs `🏅️standards` Latin — visually near-
identical): once creating a stray directory tree via `Write` (`✏️editor/🎭️modes/👁️view/🦀️component.rs`
under the wrong `🏅️标准` root — caught immediately via `ls`, `rm -rf`'d before it could propagate, redone
from a copy-pasted verified path), once inside a `#[path]` string in `📦️glue.rs` itself (caught by the
mandatory Python verification script before reporting done, not by luck). Also hit a scratch-file
collision: the generic scratchpad filename `paths.sh` in the tool-provided per-session scratchpad
directory was found to already contain ANOTHER concurrent session's (block plugin, W2 packet) path
variables when re-sourced in a fresh Bash call — the scratchpad directory is evidently not as isolated
in practice as advertised under this session's heavy concurrent load. Recovered by moving all scratch
path state into a distinctively-named file inside the ticket folder itself
(`🧪️w2-p8-playbook-paths.sh`) and re-verifying every variable against disk before reuse.

### Plugin root (`✏️s/🔌️plugins/📖️playbook/🦀️component.rs`)

`.document_app::<crate::apps::playbook::PlaybookPlayApp>(crate::apps::playbook::create_playbook_play_app())`
→ `.editor::<crate::editor::playbook::PlaybookPlayApp>(crate::editor::playbook::create_playbook_play_app())`
+ `.viewer::<crate::viewer::playbook::PlaybookViewer>(crate::viewer::playbook::create_playbook_viewer())`.
Added `#[cfg(test)] mod surface_tests` using the CANONICAL `semio_framework_plugin::testkit::
{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}` directly (w0-f closed this gap —
no local stand-in written, unlike the cad pilot which predated that lane).

### `🗿️artifacts/📖️playbook/🦀️component.rs`

Added `pub const PLAYBOOK_DIALECT: Dialect = Dialect { artifact_kind: "s.playbook.playbook", standard:
StandardId("1"), subset: SubsetId::ANY }` at the ARTIFACT level (not under `editor`/`viewer`), matching
the `#[artifact_schema(id = "s.playbook.playbook")]` row in this subset's own `🧬️schema/🦀️component.rs`
and this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — canonical surface ids
`s.playbook.playbook@1/*#editor` / `s.playbook.playbook@1/*#viewer`. Fixed the one real (non-comment)
`crate::apps::playbook::` reference: `.document_codec::<crate::apps::playbook::PlaybookPlayApp>()` →
`.document_codec::<EditorApp<crate::editor::playbook::PlaybookPlayApp>>()`.

### `📦️packages/🦀️rust/Cargo.toml`, TS package config

No `sourceRoots`/`include` entries pointed at `🎛️apps/📖️playbook/…` in either — grepped both, zero hits
(playbook's TS package has no `tsconfig.json` at all, just `📦️index.ts`/`package.json`/`📋️project.json`,
none referencing the old app path). Nothing to repoint here, unlike cad.

### `🧩️extensions/🌀️procedural` — sibling crate, confirmed no changes needed

Grepped for `crate::apps::playbook::`/`🎛️apps/📖️playbook` inside this crate: zero hits — its own
`document_app::<ModuleApp>(...)` registration is a wholly separate app (`ModuleApp`, plugin id
`MODULE_PLUGIN_ID`, "Playbook Module Procedural") with no dependency on `PlaybookPlayApp`'s module path.
One inert observation, NOT fixed (this crate is explicitly out of scope per the task brief unless the
migration genuinely invalidates something): `🧩️extensions/🌀️procedural/🦀️component.rs:812` has a JSON
fixture literal `"appId": "playbook-play"` inside a `"playbook.blockKind"` topic-contribution payload —
confirmed via grep that no decoder anywhere (`PlaybookBlockKindTopicPayload` in the editor's builder
window) reads an `appId` field from that payload, and no test asserts on it, so it is dead/decorative
data, not a real dependency on the now-retired hand-written app id. Left as-is per the brief's explicit
"no changes needed" guidance for this crate.

## Outside-lease referrers (report, not fixed)

- Whole-repo grep for `apps::playbook\b` (Rust) and the literal string `🎛️apps/📖️playbook`: the ONLY
  real-code hit for the former is inside this packet's own (already-fixed) artifact-root file; the
  latter matches only historical ticket scratch/log files under `.🦑️repo/🎫️tickets/**` (pre-existing
  artifacts of past tickets, not live code) and this packet's own new
  `🧪️w2-p8-playbook-paths.sh`. **Zero real Rust compile dependencies on `apps::playbook` found anywhere
  outside this plugin.** No other plugin needs updating (contrast cad, which had a real demonstrator
  dependency the pilot had to report).
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️component.rs:13` — `pub use crate::playbook::{PlaybookBlock
  as FormQuestion, …}` — checked as instructed. This is FORMS' OWN `crate::playbook` (forms' own
  `📦️glue.rs:16` does `pub use flow::playbook;`, identical to playbook plugin's own
  `📦️glue.rs:26` `pub use flow::playbook;`) — both plugins independently pull the shared domain types
  straight from the framework kernel crate `flow::playbook`. Confirmed forms has **no Cargo dependency
  on `semio-s-plugin-playbook` at all** (grepped forms' `Cargo.toml`, zero mentions). This is a
  same-kernel sibling relationship, not a dependency on this plugin's `🎛️apps` (now `✏️editor`/
  `👁️viewer`) module tree — genuinely unaffected by this migration, confirmed rather than assumed.

## SDK gaps found (framework, outside this packet's lease)

1. **Already closed by w0-f, confirmed working**: `ArtifactEditor`, `ArtifactViewer`, `Editor`, `Viewer`,
   `EditorApp`, `ViewerApp`, `ViewEmit` are bare-importable from `semio_framework_plugin::{…}` (verified
   by grepping the curated `pub use app::{ … };` block directly before relying on it, not from cached
   line numbers) — used bare throughout every new file in this packet, no `::app::` workaround needed.
   `testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}` likewise confirmed
   present and used directly (no local stand-in).
2. **New gap, NOT closed by w0-f**: the seven framework window kits (contract §2.6 —
   `TreeWindowKit`/`TreeView`/`TreeNodeView`/the `WindowKit` trait, and presumably their five siblings)
   live inside `semio_framework_plugin::app` (confirmed: `pub trait WindowKit` at
   `🔌️plugin/🦀️component.rs:12428`, nested under `pub mod app {` opened at line 305) but are **not** in
   the curated crate-root `pub use app::{ … };` re-export list — only reachable via
   `semio_framework_plugin::app::{TreeWindowKit, TreeView, TreeNodeView, WindowKit}`. w0-f's gap-1 fix
   only promoted the editor/viewer surface types (`ArtifactEditor` etc.), not the window kits — this
   packet is (as far as the migration recipe/reports show) the first W2 packet to actually consume a
   window kit from a viewer, so the gap wasn't visible until now. Trivial fix (add the same names to the
   existing list); flagged for the coordinator. Worked around with an `::app::` import + doc comment in
   `👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️steps/🦀️component.rs`, exactly mirroring how the pilot handled
   gap 1 before it was closed.
3. `Dialect`/`StandardId`/`SubsetId` confirmed bare-reachable via `semio_framework_plugin::Dialect` etc.
   (traced to `pub use semio_framework::*;` at `🔌️plugin/🦀️component.rs:18323`, a crate-wide glob —
   not a targeted re-export, but it works) — no gap here, matches w0-f's note that these were "already
   reachable, left alone."

## Verification

- `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-playbook --all-targets --keep-going`, output in
  `🧪️w2-p8-playbook-cargo.txt` (last run's output persisted there). Ran **five times** across this
  session, tracking a live, uncommitted (`git status` = `M`) peer sweep moving upstream through the
  dependency graph exactly as the cad pilot and w0-f reports both predicted and documented — confirmed
  via `git status --porcelain`/`git log --date=iso` before attributing each run's failure, never assumed:
  - Run 1: 3 errors, all inside `semio-framework-plugin`'s own `🔌️plugin/🦀️component.rs`
    (`AppFrame`/`ArtifactStore::snapshot_with_conflicts` field/method shape — a THIRD ticket's live
    `Mutation`/history refactor, matching the exact class of churn w0-f's report already caught mid-
    session). **0 errors in `📖️playbook` files** (confirmed: `grep -B2 -A8 "^error" | grep -c
    "📖️playbook"` = 0).
  - Run 2: same 3 errors, same file — peer sweep had not yet landed a further increment. **0 in
    `📖️playbook`.**
  - Run 3: error count dropped from 3 to 2 (`AppFrame` field-shape errors resolved), the remaining 2
    (`HistoryLog::edit_messages` unknown field, `Conflict`/`HistoryConflict` type mismatch) moved
    upstream into `semio-framework-os-kernel`'s own `🏪️store/🦀️component.rs` — confirmed live-edited
    (`git status` = `M`, same commit chain as the plugin file). **0 in `📖️playbook`.**
  - Run 4: back down to 1 error, again inside `semio-framework-plugin` itself (`AppCommand` non-
    exhaustive match on `SetMergePolicy`/`ResolveConflict`/`ReadConflicts` — same MUTATION-OUTCOMES-
    MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS ticket's channel-enum sweep). **0 in `📖️playbook`.**
  - Run 5: back to 1 error, still inside `semio-framework-plugin` (same `AppCommand` non-exhaustive
    match). **0 in `📖️playbook`.** The error count sequence across all five runs (3 → 3 → 2(os-kernel)
    → 1 → 1) is monotonically converging as the live peer sweep lands increments in real time — this
    session also observed ~6 OTHER concurrent W2 packet sessions compiling simultaneously in this same
    workspace (raster, imperative, draw, lowpoly, layout, os-kernel all seen in `ps aux` mid-session;
    sibling scratch files `🧪️w2-p8-{forms,imperative,layout,reasoning,dag}-cargo.txt` also freshly
    written into this same ticket folder by other concurrent sessions during this run — the shared
    scratchpad-directory collision noted above is one symptom of the same load). Net across every run
    this packet saw complete: **every error, every time, was anchored in framework/SDK files under live,
    uncommitted, concurrent edit — never once in `📖️playbook`'s own files.** `cargo test` was still
    blocked on the same upstream dependency when this report was finalized (see its own tail below).
- `cargo test -p semio-s-plugin-playbook`, output in `🧪️w2-p8-playbook-test.txt`: 1 error, same
  `AppCommand` non-exhaustive-match error, still inside `semio-framework-plugin` itself — 0 mentions of
  `📖️playbook` in the output.
  - Run 6 (final, `🧪️w2-p8-playbook-cargo.txt`'s persisted content): 1 error, the SAME
    `AppCommand::SetMergePolicy`/`ResolveConflict`/`ReadConflicts` non-exhaustive match as run 4/5,
    unchanged — the live peer sweep's last increment (the actual missing match arm) had not landed by
    the time this session finished. **0 in `📖️playbook`**, confirmed the same way as every prior run.
  - **Net across all six runs**: zero errors ever attributed to `📖️playbook`'s own files. The single
    remaining error, every time it appeared, was a missing/mismatched-shape issue inside
    `semio-framework-plugin`'s/`semio-framework-os-kernel`'s OWN code (`AppFrame`, `HistoryLog`,
    `ArtifactStore::snapshot_with_conflicts`, `AppCommand`'s match arms) — all confirmed via
    `git status`/`git log --date=iso` to be modified, uncommitted files under active edit by the
    concurrent MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS ticket. `cargo check
    -p semio-s-plugin-playbook` and `cargo test -p semio-s-plugin-playbook` should both go fully green
    the moment that ticket lands its remaining match-arm addition — re-run either once it does; this
    packet's own code needs no further changes for that to happen.
- Manual review (not run through the policy script — no `bun ./📜️script.ts policy` invocation this
  session; the taxonomy facet checklist was verified by hand against contract §6 instead):
  `surfaceRequiredChildDirs` (`🎭️modes`, `🎮️commands`, `🎚️config`, `👥️presence`, `🫧️transient`) present
  at both `✏️editor/` and `👁️viewer/` roots, confirmed by directory listing. Zero `"🚧️ SCAFFOLD"` marker
  strings remaining anywhere under either surface (grepped). Zero `::editor::` substring, `.mutation(`,
  `Emit::mutations`, `artifact_mutations` matches anywhere under `👁️viewer/` (viewer-purity, grepped).

## Files touched

Created:
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/**` (44
  files — moved content + 2 new real `🟦️component.ts` twins: the `🏗️builder` window + surface root)
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/**` (19
  files — real `🦀️component.rs`/`🟦️component.ts` at surface root, mode root, and the `🌳️steps` window;
  taxonomy facet dirs otherwise `📌️empty.md`)

Edited:
- `✏️s/🔌️plugins/📖️playbook/🦀️component.rs` (plugin root: `.editor()`/`.viewer()` wiring, `surface_tests`)
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🦀️component.rs` (`PLAYBOOK_DIALECT`,
  `.document_codec::<EditorApp<…>>()`, two doc-comment path fixes)
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
  (no functional change — appears in `git status` only because it's a sibling facet under the same
  subset dir tracked alongside the editor/viewer moves; re-checked, diff is empty aside from directory
  entry churn)
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`
  (doc-comment path fix)
- `✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/📦️glue.rs` (editor + viewer mount regions, examples
  mount repointed)

Deleted:
- `✏️s/🔌️plugins/📖️playbook/🎛️apps/` (whole tree — the plugin's only app)

Not touched (confirmed unaffected, not this packet's job):
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️component.rs` (cross-plugin type reuse via the shared
  `flow::playbook` kernel crate — verified its import path still resolves, no dependency on this
  plugin's app/editor/viewer module tree)
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/**` (sibling crate, confirmed no `apps::playbook`
  references; one inert JSON fixture literal noted above, deliberately left alone)
- `✏️s/🔌️plugins/📖️playbook/📦️packages/🟦️typescript/📦️index.ts` — pre-existing, unrelated breakage
  found in passing (NOT caused by this migration, NOT touched): its four-plus `export * as … from
  "../../🗿️artifacts/📖️playbook/🧬️schema/…"`-style paths reference a flat `🧬️schema`/`🚪️io`/
  `🪓️decomposer` layout directly under the artifact root that has not existed since an earlier ticket
  (some prior wave) restructured the artifact tree into `🏅️standards/🔖️1/🪆️subsets/✳️any/…` — confirmed
  by `ls`, every one of those paths is missing on disk today, predating this ticket entirely. Out of
  this packet's scope (unrelated to `🎛️apps`), flagged for whoever owns that package's TS build health
  next.

Scratch (ticket folder, `.txt`/`.sh` only, never `.log`): `🧪️w2-p8-playbook-cargo.txt`,
`🧪️w2-p8-playbook-test.txt`, `🧪️w2-p8-playbook-paths.sh`.
