# W2 Packet P9 (architect) — Notes

Lane: W2 packet P9, plugin `🏛️architect`, subset `s.architect.program@1/*`. Scope: migrate the retired
`✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/` app (3 modes, 5 windows — the most complex single-plugin
packet in this ticket) into `✏️editor`, author a real `👁️viewer`, rewire `📦️glue.rs`/plugin root/artifact
root, delete `🎛️apps/`, and record results. Followed `📓️w2-cad-report.md`'s recipe, `📋️contract-freeze.md`,
and the closed SDK gaps (`📓️w0-f-report.md`, `📓️w2-p8-report.md`).

## DIALECT derivation

Verified against the schema's own attribute, not guessed: `grep "artifact_schema(id" 🧬️schema/**` shows
`#[artifact_schema(id = "s.architect.program")]` on the snapshot/diff/schema files, and the artifact's own
`definition()` schema-capability claim already keys off the identical string
(`ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.architect.program")`). Added at
`🗿️artifacts/🏛️program/🦀️component.rs`:

```rust
pub const ARCHITECT_DIALECT: semio_framework_plugin::app::Dialect =
    semio_framework_plugin::app::Dialect { artifact_kind: "s.architect.program", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
```

Canonical surface ids: `s.architect.program@1/*#editor` / `s.architect.program@1/*#viewer`.

## How the 3 modes were preserved

The app's three modes (`✏️edit`, `🔍️review`, `📊️report`) moved as a unit into
`✳️any/✏️editor/🎭️modes/`, unchanged in shape:

- `✏️edit` — the app's default mode and the ONLY one carrying a window layout (all 5 window kinds are
  declared at APP level via `window_kind_def`, never at mode level, and no mode declares its own
  layout — `✏️edit` is simply the shallowest common ancestor and the sole owner of a layout referencing
  them, exactly as the pre-migration app's own doc comment already stated). Its 5 windows
  (`↔️adjacency`, `🕸️graph`, `📋️register`, `📄️report`, `🧭️trace`) moved with their real `🦀️component.rs`
  bodies intact.
- `🔍️review` and `📊️report` — windowless mode-level facets, each with its own real `🦀️component.rs`
  (`ModeDefinition` only, no layout) and the taxonomy's empty `🪟️windows/📌️empty.md` placeholder,
  exactly as they were pre-migration. Left untouched beyond the `crate::apps::architect::` →
  `crate::editor::architect::` mechanical rename. The completeness policy only requires ≥1 mode with
  ≥1 window across the whole surface (satisfied by `✏️edit`), not every mode — these two stay
  intentionally windowless.

`🗂️catalog` and `🎨️chrome` (app-only, non-`surfaceChildDirs` facets — no taxonomy slot under a
surface, mirroring cad's `⚙️engine` precedent) were moved whole into `✏️editor/🗂️catalog/` and
`✏️editor/🎨️chrome/` since only editor-side files (windows, panels, commands) reference them. The
already-empty, already-unmounted `🎛️apps/🏛️architect/⚙️engine/` reserved stub (0 files — the app's own
former `⚙️engine`-topic behavior had already been dissolved onto the app root by ticket
26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, per that root file's own doc comment) was deleted
with the rest of `🎛️apps/`; nothing of substance was lost.

## Viewer design rationale

Genuinely independent `ArchitectViewer: ArtifactViewer` under `✳️any/👁️viewer/`, one `👁️view` mode, one
real window:

- `Snapshot = ProgramSnapshot`, `Mutation = crate::artifacts::program::op::ProgramMutation` (both
  artifact-level, shared with the editor — decode-only per contract §2.2).
- `Config`/`Presence`/`Transient` = framework `NoConfig`/`NoPresence`/`NoTransient`. `Command` = a
  one-variant `ArchitectViewCommand::Noop`; `handle` always returns `Ok(ViewEmit::default())`.
- **Window choice**: the ticket brief suggested `MeshWindowKit`, `🕸️graph`, or `📋️register` and asked
  for judgment after reading the real `render()` code. Architect's content is program-management data
  (68 flat entity registers, no 3D geometry at all — `artifact_kind()`'s own `media_capability:
  OsMediaCapability::MeshOnly` note is about a totally different, currently-unpopulated composed-mesh
  slot, not this window), so `MeshWindowKit` does not fit at all. Between `🕸️graph` (a specific
  node-layout of `elements`/`adjacencies` only) and `📋️register` (one selectable register, needs
  `ArchitectConfig::active_register` — state a `NoConfig` viewer cannot carry), NEITHER editor window's
  `render()` signature ports directly to a config-free viewer. Rather than force one arbitrary register
  or drop to a narrower stand-in, the viewer's single window is a bespoke pure render function,
  `📋️register` (renamed from the scaffold's placeholder `🪟️main`), built on the SAME artifact-level pure
  `status_summary()` inference the editor's own document panel already uses — a document-wide,
  config-free overview (per-register entity counts + draft/approved split, one tree section per
  non-empty register out of the 68). This is "a table-like register" in spirit (the ticket's own hint),
  genuinely useful on its own, and needs no per-session state — not a narrower copy of either editor
  window. `SurfaceKind::Table` (closest semantic fit for a register-count overview; the actual `UiNode`
  returned is a `Tree`, mirroring this same codebase's own `📄️report`/`🧭️trace` editor windows, which
  declare `SurfaceKind::TextEditor` while also returning `UiNode::Tree` — SurfaceKind is a window-chrome
  category here, not a strict `UiNode` variant lock).
- Local `view_tree_item`/`view_tree_section`/`view_tree_node` helpers are DUPLICATED (not imported) from
  the editor's `🎨️chrome.rs` shape — deliberate, since a viewer file must never reference the sibling
  editor module at all (see purity note below), matching the pattern the forms/cad viewers already
  established for their own small presentation helpers.
- `create_architect_viewer() -> AppDefinition` via `Viewer::builder(ARCHITECT_DIALECT)…build_definition()`.

**Viewer purity**: `grep -rl "::editor::"` and `grep -rln ".mutation(\|Emit::mutations\|artifact_mutations"`
restricted to `👁️viewer/` both return nothing (verified after writing, not assumed).

## `.example(...)`/`.workflow(...)` drop

`create_architect_app()`'s two pre-migration `.example("sample", …)`/`.example("empty", …)` calls and the
no-op `.workflow("architect", "Architect", "data")` call were dropped, not ported — `EditorBuilder` has no
such methods (contract §2.4 gap, documented inline at the manifest fn's own doc comment, matching every
prior W2 packet). The subset's own `📚️examples/🎬️demo`/`📚️examples/🎬️demo-session` facets are the modern
replacement surface.

## `📦️glue.rs`

Old `//#region 🎛️Apps` (mounting `apps::architect::*` from `../../🎛️apps/🏛️architect/…`, 92 lines)
replaced by two independent regions built programmatically (never hand-typed — derived from the file's
OWN prior `pub mod` nesting via a saved Python transform script, `🧪️w2-p9-architect-glue-rewire.py` in
this folder) so no emoji path segment was retyped by hand:

- `//#region ✏️Editor` — `pub mod editor { pub mod architect { … } }`, every leaf `#[path]` repointed at
  `../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/…`.
- `//#region 👁️Viewer` — `pub mod viewer { pub mod architect { … } }`, same base but `…/👁️viewer/…`,
  mounting only the surface root, the `view` mode, and the one `register` window.

The bottom `//#region 📚️Examples` mount for `app_architect_demo_session` was repointed at the new editor
path. Every `crate::apps::architect::` reference across the 22 moved Rust files that used it became
`crate::editor::architect::` (mechanical `sed`, editor tree only, verified 0 remaining afterward).

**Path verification** (recipe step 10's exact script, run twice): `1193` total `#[path]` attributes in the
rewired `📦️glue.rs`, **0 missing** both times. One brace-counting bug from the programmatic transform
(a dropped closing `}` for the inner `pub mod architect { … }`) was caught immediately by a brace-balance
check (open `{` == close `}`, both 346) before ever running cargo, and fixed by hand at the exact reported
line.

## Emoji-typo incidents (caught and fixed immediately)

Twice during this session, hand-typing a path directly into a tool call (rather than reusing a
just-verified string) substituted "🏅️standards" (Latin) with a visually similar "🏅️标准" (Chinese)
fragment, and once substituted the plugin dir name with "🏛️架构" — each created a stray sibling
directory/file instead of erroring. Both were caught within the same turn (`grep -rn "标准\|架构"` +
`find -iname` swept the whole `✏️s` tree afterward, confirmed clean) and `rm -rf`'d before any dependent
file referenced them. Switched strategy immediately afterward to sourcing a scratch shell-variable file
(`🧪️w2-p9-architect-paths.sh`, this folder) and using Bash heredocs against `$EDITOR`/`$VIEWER` variables
for every subsequent write, rather than retyping full paths into tool parameters.

## Plugin root (`✏️s/🔌️plugins/🏛️architect/🦀️component.rs`)

`.document_app::<crate::apps::architect::ArchitectPlayApp>(create_architect_app())` → two calls:
`.editor::<crate::editor::architect::ArchitectPlayApp>(crate::editor::architect::create_architect_app())`
and `.viewer::<crate::viewer::architect::ArchitectViewer>(crate::viewer::architect::create_architect_viewer())`.
Added `#[cfg(test)] mod surface_tests` using the LANDED `semio_framework_plugin::testkit::{assert_viewer_never_mutates,
assert_editor_and_viewer_share_dialect}` directly (contract §2.5, closed by W0-F gap 2) — no local
stand-ins, matching `🕸️dag`/`📏️layout`'s established pattern from `📓️w2-p8-report.md`.

## `🗿️artifacts/🏛️program/🦀️component.rs`

Added `ARCHITECT_DIALECT` (see above). `.document_codec::<crate::apps::architect::ArchitectPlayApp>()` →
`.document_codec::<semio_framework_plugin::EditorApp<crate::editor::architect::ArchitectPlayApp>>()` (the
runtime `ArtifactApp` bound needs the SDK adapter, matching forms's modern bare-`EditorApp` import
post-W0-F, not cad's pre-W0-F `app::EditorApp` workaround).

## Editor root testkit/tests fallout

`pub type ArchitectApp = VcsArtifactApp<ArchitectPlayApp>` → `VcsArtifactApp<EditorApp<ArchitectPlayApp>>`;
`new_app()`/`app_with_registry()` retyped to `EditorApp<ArchitectPlayApp>` generics. `new_app_with_registry`
and `testkit::assert_declared_actions_bridge_to_commands` both still take the pre-migration
`fn() -> App` shape (W0-F gap 3, unchanged) — added a local
`architect_app_manifest_for_testkit() -> App { App { definition: create_architect_app(), examples: Vec::new() } }`
wrapper and swapped both call sites onto it, matching forms's/cad's identical fix.

## Files touched

Created:
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/**` — 90
  files total: 84 real files moved from `🎛️apps/🏛️architect/**` (root `component.rs`, `🎚️config`+schema,
  `👥️presence`+schema, `🗂️catalog`, `🎨️chrome`, `🎮️commands/*` — 8 groups, `🎭️modes/{✏️edit,🔍️review,
  📊️report}` incl. all 5 `✏️edit`-mode windows, `📌️panels/*` — 3, `📚️examples/🎬️demo-session/*`) + 6 new
  real `🟦️component.ts` files (5 distinct window twins + 1 namespaced surface-root re-export).
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/**` — 19
  files (unchanged count from the W1-E scaffold — placeholders rewritten in place, `🪟️main` window
  renamed to `📋️register`): real `🦀️component.rs`/`🟦️component.ts` at surface root, mode root, and the
  one window; taxonomy facet dirs otherwise the pre-existing `📌️empty.md`.

Edited:
- `✏️s/🔌️plugins/🏛️architect/🦀️component.rs` (plugin root: `.editor()`/`.viewer()` wiring, `surface_tests`)
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🦀️component.rs` (`ARCHITECT_DIALECT`,
  `.document_codec::<EditorApp<…>>()`)
- `✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📦️glue.rs` (editor + viewer mount regions, examples path fix)

Deleted:
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/` (whole tree — the plugin's only app, incl. the already-empty
  `⚙️engine` reserved stub)

Cargo.toml / tsconfig.json / vitest.config.ts / project.json: grepped the whole `📦️packages/**` tree for
the literal old app path (`🎛️apps/🏛️architect`) — **zero matches**, no build-tool config edits needed
(unlike cad, architect's `Cargo.toml`/`tsconfig.json`/`📋️project.json` never referenced the app path
directly).

Scratch (ticket folder): `🧪️w2-p9-architect-filelist.txt`, `🧪️w2-p9-architect-paths.sh`,
`🧪️w2-p9-architect-glue-rewire.py`, `🧪️w2-p9-architect-path-verify.py`, `🧪️w2-p9-architect-cargo.txt`,
`🧪️w2-p9-architect-test.txt`.

## Outside-lease referrers (report, not fixed)

- **Zero real (non-doc-comment) Rust compile dependencies** found repo-wide on `apps::architect`,
  `ArchitectPlayApp`, or `create_architect_app` outside `✏️s/🔌️plugins/🏛️architect/**` — grepped the whole
  `✏️s` tree, confirmed empty.
- Root `📜️script.ts`'s large static path-string array (`:8138`-ish) still lists 4 of architect's own
  already-deleted `🎛️apps/🏛️architect/…` paths — same pre-existing, cross-packet, not-kept-in-sync gap the
  W2-P8 coordinator already reported for other plugins; cosmetic, not a compile or policy blocker
  (`policyTaxonomyDirsBreaches` doesn't walk surface subtrees until W3 per contract §6), not this packet's
  job to fix.
- Every other repo-wide hit on `apps::architect`/`🎛️apps/🏛️architect` is inside historical ticket scratch
  files under `.🦑️repo/🎫️tickets/**` from earlier, unrelated tickets (pre-dating this migration) — not
  live code, not referrers, no action needed.

## Cargo verification results

`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-architect --all-targets --keep-going`, three full runs,
final output in `🧪️w2-p9-architect-cargo.txt`:

- Run 1: 263 errors, **all 263 inside `semio-s-plugin-stdio`'s own files** (`error[E0053]: method
  'mutate' has an incompatible type for trait`, `E0277`, `E0308`, `E0599` across dozens of stdio artifact
  schema files) — confirmed live-edited: `git status --porcelain` showed 80+ modified files under
  `✏️s/🔌️plugins/🗄️stdio/**` right now, `git log --date=iso` on the first-failing file
  (`💾️binary/🏅️standards/🔖️raw/…/🧬️schema/🦀️component.rs`) showed its last real commit at
  2026-08-16 14:18:35 today, tagged "Mutation Outcomes, Merge Policies and First-Class Conflicts" —
  exactly the concurrent peer ticket the packet brief pre-warned about. **0 errors in `🏛️architect`
  files.**
- Run 2 (re-run after that): 2 errors, now inside `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
  itself (`E0004: non-exhaustive patterns` on `OpsHeaderLine`) — the failure moved further upstream
  exactly as the pilot's/W0-F's reports predicted. `git status --porcelain` on that exact file: `M`
  (uncommitted right now); `git log --date=iso` shows the same 2026-08-16 14:18:35 MUTATION-OUTCOMES
  commit as its last real change. **0 errors in `🏛️architect` files.**
- Run 3: identical 2 errors, same file, same blocker, still live. **0 errors in `🏛️architect` files** —
  confirmed with `grep -B2 -A8 "^error" | grep -c "🔌️plugins/🏛️architect"` → `0` all three times.
- **Zero warnings** anchored in `🏛️architect` files either, across all three runs (checked explicitly,
  not assumed).
- `cargo test -p semio-s-plugin-architect --no-run`: same blocker (`semio-framework-os-kernel`
  `OpsHeaderLine` non-exhaustive match, upstream of architect in the dependency graph), output in
  `🧪️w2-p9-architect-test.txt`. **0 errors anchored in `🏛️architect` files.**

Net: every real error surfaced across all four runs (3× check + 1× test) was inside `semio-s-plugin-stdio`
or `semio-framework-os-kernel`, both confirmed live/uncommitted via `git status`/`git log --date=iso` (never
by parsing commit message text) — not this packet's own code, not fixed, not blocking further work here.
Re-run once the MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS ticket's sweep finishes landing.

## Structural verification

- `grep -rl "SCAFFOLD"` under `✏️editor/` and `👁️viewer/` → empty (0 residue).
- `grep -rl "::editor::"` and `grep -rln "\.mutation(\|Emit::mutations\|artifact_mutations"` restricted to
  `👁️viewer/` → both empty.
- `#[path]` resolution script (recipe step 10): 1193/1193 resolve, run twice.
- `🎛️apps/` confirmed deleted (`ls` of the plugin root no longer lists it).
- Every newly-written emoji path re-`ls`'d/`find`'d immediately after creation; two hand-typing slips
  (see "Emoji-typo incidents" above) caught and removed before any file referenced them.
