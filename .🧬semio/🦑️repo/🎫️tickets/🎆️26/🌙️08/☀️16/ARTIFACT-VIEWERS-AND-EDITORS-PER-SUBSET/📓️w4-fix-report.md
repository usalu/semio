# W4-FIX Report — VcsArtifactApp Role Guard + Demonstrator Manifest Fix

Lane W4-FIX, ticket `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`. Closes the two gaps found by
`📓️w4-audit-taxonomy.md` (demonstrator manifest compile break) and `📓️w4-audit-readonly.md` (contract
§2.3 role guard, parked at W0 per `📌️important.md` §2).

## Unpark check (done first)

`📌️important.md` §2 parked the role guard because the peer ticket
`26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS` held
`🔌️plugin/🦀️component.rs`'s `VcsArtifactApp`/`🧪️testkit` regions and `🔌️plugin/🏗️builder/🦀️component.rs`,
deferred "to be made once the peer's `📓️w1-b-report.md` exists." Checked: that peer ticket's
`📓️w1-b-report.md` exists (03:38) **and** its `🎫️ticket.json` status is `"closed"`. Unpark condition
satisfied — proceeded.

## Job 1 — `VcsArtifactApp` role guard (contract §2.3)

Files: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs`.

**Root gap found**: the runtime `trait ArtifactApp` (line 9081) had no `ROLE` const at all — only the
NEW authoring traits `ArtifactEditor`/`ArtifactViewer` (added by an earlier wave) carry
`const ROLE: AppRole`. `EditorApp<E>`/`ViewerApp<V>` (the adapters implementing the runtime
`ArtifactApp`) never forwarded it. `VcsArtifactApp<A: ArtifactApp>` therefore had no way to read a
role at all — clause 5 was the actual missing foundation the other four clauses depend on.

### Clause 5 — read role from `ArtifactApp::ROLE`

- `trait ArtifactApp`: added `const ROLE: AppRole = AppRole::Editor;` (defaulted, so the pre-existing
  hand-written direct impls — `DummyApp`/`TxnApp`/`TestApp` in this file's own tests, plus
  `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️component.rs::SpaceApp` and
  `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs::ModuleApp` — keep compiling
  unchanged with their existing full-mutation behavior; only `EditorApp`/`ViewerApp` ever produce a
  `Viewer`-role instance).
- `impl<E: ArtifactEditor> ArtifactApp for EditorApp<E>`: `const ROLE: AppRole = E::ROLE;`
- `impl<V: ArtifactViewer> ArtifactApp for ViewerApp<V>`: `const ROLE: AppRole = V::ROLE;`

Every other clause below reads `A::ROLE` directly — no field added to `VcsArtifactApp` itself.

### Clause 1 — reject the eight verbs

New region `//#region 🔖️ViewerGuard` (next to `HISTORY_ACTION_IDS`/`CLIPBOARD_ACTION_IDS`):

```rust
const VIEWER_REJECTED_ACTION_IDS: [&str; 7] =
    ["undo", "redo", "commitCheckpoint", "createAlternative", REVERT_TO_COMMAND_ACTION_ID, "cut", "paste"];
fn viewer_read_only_fault(verb: &str) -> Fault {
    Fault::new(FaultOrigin::Framework, FaultCode::new("viewer.read-only"), format!("'{verb}' is not permitted on a viewer instance (contract §2.3)"))
}
```

- `dispatch_action` (the `handle_action` body): checked FIRST, before `INTERACTION_ACTION_IDS`.
- `dispatch_import_media` (the `PluginApp::import_media` body — `import` is a method, not a string
  action, so it needed its own check): checked first.
- `checkoutCheckpoint`/`switchAlternative` are deliberately **not** in the rejected set — they move the
  read cursor across already-existing history, they never create new content, so a viewer may still
  browse checkpoints/alternatives (documented inline).

### Clause 2 — hard backstop on non-empty `artifact_mutations`

`dispatch_emit`, right after destructuring `Emit`:

```rust
if A::ROLE == AppRole::Viewer && !artifact_mutations.is_empty() {
    return Err(viewer_read_only_fault(verb));
}
```

Unreachable through `ViewerApp` (its `handle` builds `Emit` solely from `ViewEmit`'s three fields,
contract §2.2 — audit #2 confirmed this structural closure holds). This is what makes a **hand-written**
runtime `ArtifactApp` impl with `ROLE = Viewer` safe too — not skipped as redundant, per the audit's
explicit instruction.

### Clause 3 — read-only history panel

`ui_history_panel` gained a `read_only: bool` parameter (its only 3 call sites are all in this same
file — 1 real, 2 tests; grepped repo-wide, confirmed no cross-crate callers). When `true`: undo/redo/
commitCheckpoint/createAlternative render `enabled: false` regardless of `can_undo`/`can_redo`, and the
per-command "Backwards" (`revertToCommand`) action row is omitted entirely (`entry.revertible && !read_only`).
The filter control and the Commands list itself stay fully live — browsing history is not a mutation.
`VcsArtifactApp::render`'s one real call site now passes `A::ROLE == AppRole::Viewer`; both test call
sites pass `false` (unchanged behavior).

### Clause 4 — store attaches `Rights::Read` only

`PluginBuilder::viewer::<V>()`/`::editor::<E>()` (the builder file) now each end with a
`CapabilityRequirement` push, mirroring `local_backbone_storage`'s existing pattern (the only
pre-existing `Rights::` usage in the whole crate, at plugin/Backbone scope) but at `Scope::App` /
`ArtifactKind::Document`:

- `viewer::<V>()`: `CapabilityRequirement { artifact: ArtifactKind::Document, rights: Rights::Read, scope: Scope::App }` only.
- `editor::<E>()`: both `Rights::Read` and `Rights::Write` at the same scope.

No prior mechanism declared per-app document rights at all (`AppDefinition` itself has no `capabilities`
field); this is new, additive capability declaration, not a rename of something pre-existing.

### Test — "with teeth" (ran, output captured)

Added `viewer_rejects_every_contract_mutating_verb` to the existing `surface_testkit_tests` module
(which already had `SurfaceViewerFixture`/`new_viewer::<V>()` from a prior wave). Constructs a real
`VcsArtifactApp<ViewerApp<SurfaceViewerFixture>>` via `new_viewer`, dispatches all seven string verbs
through `handle_action` and `import` through `import_media`, and asserts every one returns
`Fault { origin: FaultOrigin::Framework, code: FaultCode("viewer.read-only") }`.

```
running 1 test
test component::app::testkit::surface_testkit_tests::viewer_rejects_every_contract_mutating_verb ... ok
```

Full output: `🧪️w4-fix-cargo.txt` in this folder.

## Job 2 — demonstrator manifest compile break

File: `✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs`.

Derived the real post-migration module paths off each plugin's OWN `📦️glue.rs` (never guessed):

- `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/📦️glue.rs`: `pub mod editor { pub mod process3d { … } }`
  and `pub mod viewer { pub mod process3d { … } }` — process3d has BOTH roles.
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/📦️glue.rs`: `pub mod editor { pub mod sourcing { … } }`
  and `pub mod viewer { pub mod sourcing { … } }` — note the inner module is named `sourcing`, not
  `curate` (the artifact itself is `curate`; the surface module is `sourcing`) — this is exactly the
  `#[path]`-means-directory-≠-module-path trap the brief warned about; confirmed by reading the glue
  file, not guessed from the `🗿️artifacts/🗂️curate/` directory name.

Fixes:
1. `use process::apps::process3d::{…}` → `use process::editor::process3d::{create_process3d_app, Process3dPlayApp}`, plus new `use process::viewer::process3d::{create_process3d_viewer, Process3dViewer}` (the plugin exposes a viewer — confirmed by reading `…/👁️viewer/🦀️component.rs`, `pub struct Process3dViewer` / `impl ArtifactViewer` / `pub fn create_process3d_viewer()`).
2. `use sourcing::apps::curate::{…}` → `use sourcing::editor::sourcing::{create_sourcing_curate_app, SourcingCurateApp}`, plus new `use sourcing::viewer::sourcing::{create_sourcing_viewer, SourcingViewer}` (same check).
3. `.document_app::<SourcingCurateApp>(create_sourcing_curate_app())` / `.document_app::<Process3dPlayApp>(create_process3d_app())` → four calls: `.editor::<SourcingCurateApp>(…)` + `.editor_mutation_roster::<SourcingCurateApp>()` + `.viewer::<SourcingViewer>(…)` + `.viewer_mutation_roster::<SourcingViewer>()`, and the same shape for `Process3dPlayApp`/`Process3dViewer`.

`.editor_mutation_roster()`/`.viewer_mutation_roster()` chained because both `SourcingMutation`
(`✏️s/🔌️plugins/🪵️sourcing/…/🧬️mutations/🦀️component.rs`) and `Process3dMutation`
(`✏️s/🔌️plugins/🏭️process/…/🧬️mutations/🦀️component.rs`) derive
`#[derive(…, dsl::Mutations)]` + `#[mutations(snapshot = …, diff = …, schema = "…")]` — the exact
pattern `📓️w2-sdk2-report.md` documents as satisfying `SemanticMutation`, matching the already-working
`Procedural3dPlayApp`/`CadPlayApp`/`Puzzle3dPlayApp`/`Gis2dPlayApp` roster calls already in this file.
Both viewers reuse the SAME `Mutation` type as their sibling editor (decode-only, contract §2.2), so
they qualify too. Neither `Process3dViewer`'s nor `SourcingViewer`'s builder declares
`setContributions` (checked both `create_*_viewer()` bodies) — the `contribution_consumers_…` test's
4-entry list is otherwise unchanged.

New surface ids (`surface_app_id` = `"{artifact_kind}@{standard}/{subset}#{role}"`, derived from each
plugin's own `Dialect` const, read directly rather than guessed):
- `s.sourcing.curate@1/*#editor`, `s.sourcing.curate@1/*#viewer`
- `s.process.process3d@1/*#editor`, `s.process.process3d@1/*#viewer`

Updated `bundle_registers_its_own_and_the_six_foreign_demonstrator_surfaces` (now asserts 10 ids, was
8) and `contribution_consumers_declare_the_hidden_app_command` (replaced the two legacy string ids)
accordingly. Doc comments referencing "six foreign document surfaces"/"document-app registrations"
updated to describe the editor+viewer-pair shape.

### Sweep for other surviving `apps::` references

`grep -rln "apps::"` / `"document_app"` / `"SCAFFOLD"` over the whole `🎪️demonstrator` plugin tree: only
the one manifest file, and there only in doc-comment prose (mine, and one pre-existing about `cad`'s
already-fixed history) — no other live code reference. Repo-wide sweep (required verification, run
after the fix) — every remaining `apps::` hit is either `paired_apps::<…>` (a distinct testkit function
name, false-positive substring match) or a historical doc comment in an unrelated plugin explaining a
PAST rename; none is a live import of a deleted module. Full list in `🧪️w4-fix-cargo.txt`.

## Verification actually run

1. `RUSTC_WRAPPER="" cargo check -p semio-framework-plugin --all-targets --keep-going` — **0 errors**.
2. `cargo test -p semio-framework-plugin viewer_rejects_every_contract_mutating_verb -- --nocapture` — **1 passed** (output above).
3. `cargo test -p semio-framework-plugin --lib` (whole crate, informational) — 160 passed / 59 failed.
   **All 59 failures are pre-existing and out of this lane's scope**: every one panics on either
   `"app id <x> must be a canonical surface id: … missing '#'"` (dozens of `app_builder_tests`/
   `plugin_builder_contract_tests` fixtures — e.g. `synthetic-play`, `demo-play`, `puzzle2d-play`,
   `good-app`, `history-app`, … — still using pre-migration non-canonical ids against
   `AppBuilder::build_definition`'s `parse_surface_app_id` assertion), or on unrelated
   `s.stdio.ifc.*`/VCS-conflict-fixture errors. This exact "canonical surface id" gap is already
   documented independently across ≥10 other reports in this ticket (`w2-p5-report.md`,
   `w2-cad-report.md`, `w2-p8-*-notes.md`, …) — confirmed by `grep -rl "canonical surface id"` over the
   ticket folder — so it predates this lane and touches none of the three files this lane edited. My
   diff against `HEAD` (`git diff --stat`) does not contain any of the failing tests' names or the
   `parse_surface_app_id`/`AppBuilder::new` code paths. This lane's own `surface_testkit_tests` module
   (5 tests, including the new one) is 100% green.
4. `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-demonstrator --all-targets --keep-going` — **164
   errors, 0 anchored in any file this lane touched**. `semio-s-plugin-demonstrator` itself was never
   reached: it depends on `semio-s-plugin-process`/`semio-s-plugin-sourcing`, both of which depend
   directly on `semio-s-plugin-stdio` (checked their own `Cargo.toml`), and `semio-s-plugin-stdio`
   itself fails to compile first (164 errors: `E0433` `HashSet` not found, `E0053` `absorb` incompatible
   type, `E0308` mismatched types — all anchored under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/**/🧬️schema/**`).
   Proved foreign: `git status --porcelain` on the stdio tree shows dozens of `M`/`D` entries (including
   `✏️s/🔌️plugins/🗄️stdio/🎛️apps/🦀️component.rs` deleted), `git log --date=iso` on a sampled
   error-anchored file shows its last real commit was 2026-08-15 (yesterday), and its on-disk mtime is
   2026-08-16 17:59 — recent, uncommitted, matching exactly the pattern
   `📌️important.md` §1 already documents for the live
   `FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS` peer ticket ("owns much of
   `🗄️stdio`"), and matching `📓️w2-sdk2-report.md`'s own prior finding of the identical blocker
   ("Every plugin crate depends on semio-s-plugin-stdio, which was mid-restructure … all session and
   still fails to compile"). Not this lane's regression; cannot be worked around without touching a
   file this lane does not own.
5. Repo-wide `grep -rn "apps::" ✏️s/🔌️plugins --include='🦀️component.rs'` (excluding ticket folders) —
   every hit reported and explained above; none is a compile-breaking reference.

Full command outputs: `🧪️w4-fix-cargo.txt` in this folder.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — `ArtifactApp::ROLE` (default +
  `EditorApp`/`ViewerApp` overrides), new `//#region 🔖️ViewerGuard`, guards in `dispatch_action`/
  `dispatch_import_media`/`dispatch_emit`, `ui_history_panel`'s new `read_only` param (+ 3 call sites),
  new test `viewer_rejects_every_contract_mutating_verb` and its imports.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` — `viewer::<V>()`/
  `editor::<E>()` now push a `Document`-scoped `CapabilityRequirement` (Read-only / Read+Write).
- `✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs` — repointed the two broken
  imports, converted `.document_app::<…>()` to `.editor::<…>()` + `.viewer::<…>()` (+ roster opt-in)
  for both `sourcing` and `process`, updated the two id-asserting tests and stale doc comments.

Scratch (this ticket folder): `🧪️w4-fix-cargo.txt`.

## Not done / explicitly out of scope

- The 59 pre-existing `app_builder_tests`/`plugin_builder_contract_tests` "canonical surface id"
  failures (see Verification §3) — dozens of unrelated fixtures across the whole
  `semio-framework-plugin` crate, already independently documented in ≥10 other W2 reports in this
  ticket. Not touched: fixing them is a distinct, much larger lane, not part of either job assigned
  here.
- `semio-s-plugin-stdio`'s 164 compile errors — owned by the live
  `FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS` peer ticket, not this one.
- The other 31 plugin-root files `📓️w2-sdk2-report.md` flagged (not fixed) as candidates for
  `.editor_mutation_roster()`/`.viewer_mutation_roster()` opt-in — out of scope; only `demonstrator`
  (job 2) and the framework SDK itself (job 1) were touched here.
