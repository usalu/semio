# W0-F Report — SDK Gap Closure

Lane: 0-F (SDK gap closure), critical path for the eight remaining W2 plugin-migration packets.
Contract: `📋️contract-freeze.md` §2 (esp. §2.5), §1. Pilot: `📓️w2-cad-report.md` ("SDK gaps found").

## Gap 1 — crate-root re-exports

Added `ArtifactEditor`, `ArtifactViewer`, `Editor`, `EditorApp`, `ViewEmit`, `Viewer`, `ViewerApp` to
the existing curated `pub use app::{ … };` block in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, in the same idiom as the
pre-existing `ArtifactApp`/`Emit`/`App` entries (alphabetically placed among neighbors). No other
name added — `Dialect`/`StandardId`/`SubsetId` were already reachable via `semio_framework::*` and
were left alone per "do not widen internals beyond the contract's named types."

Insertions (line numbers as of the last edit in this lease): `ArtifactEditor` :18211, `ArtifactViewer`
:18233, `Editor`/`EditorApp` :18246–18247, `ViewEmit`/`Viewer`/`ViewerApp` :18295–18297, inside the
`pub use app::{ … };` block starting :18177. Verified no naming collisions before editing.

Downstream effect: every W2 packet can now `use semio_framework_plugin::{ArtifactEditor,
ArtifactViewer, Editor, Viewer, EditorApp, ViewerApp, ViewEmit};` directly, instead of the pilot's
workaround `use semio_framework_plugin::app::{ … };`.

## Gap 2 — testkit helpers (contract §2.5)

Re-read the peer ticket's `📓️w1-b-report.md` (26/08/16/PLUGIN-DEPENDENCIES, W1-B) first, which
confirms it owns `🔖️Emit`, `VcsArtifactApp`, `🔖️Exchange` and a `🧪️testkit` subregion nested inside
the pre-existing `pub mod testkit { … }` (`//#region 🔖️Testkit`, same file). Its subregion (three
`assert_transaction_*` helpers + `transaction_testkit_tests`) was left completely untouched.

Appended a new, clearly delimited subregion `//#region 👁️✏️SurfaceTestkit` /
`//#endregion 👁️✏️SurfaceTestkit` at `🔌️plugin/🦀️component.rs:6739`–`7056`, directly after the
peer's subregion closes and before `pub mod testkit`'s own closing brace — no existing line moved or
edited.

Three functions, exact contract §2.5 signatures:

```rust
pub fn assert_viewer_never_mutates<V: ArtifactViewer>() where V::Command: Default;
pub fn assert_editor_and_viewer_share_dialect<E: ArtifactEditor, V: ArtifactViewer>();
pub fn new_viewer<V: ArtifactViewer>() -> VcsArtifactApp<ViewerApp<V>>;
```

- `new_viewer` — thin rename over `testkit::new_app::<ViewerApp<V>>()`.
- `assert_editor_and_viewer_share_dialect` — `assert_eq!(E::DIALECT, V::DIALECT, …)`.
- `assert_viewer_never_mutates` — **real teeth, not a type-only check**: dispatches
  `V::Command::default()` through the full `VcsArtifactApp<ViewerApp<V>>::dispatch_typed` runtime
  path (the same one `PluginBuilder::viewer::<V>` wires up) and asserts the document store's
  `generation()`/`envelope().vcs.edits.len()` AND the draft store's same two values are unchanged
  before/after. This proves the runtime adapter (`ViewerApp<V>::handle`) never lets a mutation
  through, on top of the pre-existing type-level guarantee (`ViewEmit` cannot name
  `artifact_mutations`/`draft_mutations` at all).
  - Added bound `V::Command: Default`, not in the contract's literal one-line signature, because
    `ArtifactViewer` declares no such bound on `Command` (author-defined grammar) and this is the
    minimal extra constraint needed to synthesize a representative command with zero caller-supplied
    arguments. Every viewer this ticket ships (e.g. cad's `CadViewCommand::Noop`) is a no-op-only
    command that can derive `Default` for free; documented inline at the fn's doc comment.

Self-test: a self-contained `#[cfg(test)] mod surface_testkit_tests` (own `SurfaceSnapshot`/
`SurfaceDiff`/`SurfaceMutation`/`SurfaceEditorCommand`/`SurfaceViewerCommand` fixtures, mirroring the
proven `testkit_tests::Dummy*` pattern field-for-field rather than reaching into that sibling
module's private items) exercises all three functions plus a normal-mutation control case
(`editor_fixture_still_mutates_normally`, via `EditorApp<SurfaceEditorFixture>`, proving the harness
can tell a real mutation apart from none).

**Mid-session live-refactor hit my own new code once, fixed in place.** A THIRD ticket
(`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`, not previously known to this
lane — found via the trait's own doc comment) is concurrently changing `protocol::Mutation` repo-wide:
`diff` now returns `MutationOutcome<Diff>` instead of a bare `Diff`, and `validate`/`merge_strategy`/
`conflict_rule`/`reconcile` are removed from the trait entirely. My first version of
`SurfaceMutation`'s `impl Mutation<SurfaceSnapshot>` mirrored the (at-the-time-still-compiling) old
shape used by the pilot's `Dummy*`/`Txn*` fixtures; a `cargo check` mid-session caught it as a genuine
error in MY OWN code (`🔌️plugin/🦀️component.rs:6857`, confirmed by file:line, not assumed). Fixed to
`fn diff(&self, _snapshot: &SurfaceSnapshot) -> ::protocol::MutationOutcome<SurfaceDiff>`, returning
`::protocol::MutationOutcome::new(SurfaceDiff { … })` — the same pattern the live-edited framework
`NoConfig`/`NoPresence`/`NoTransient`/`InteractionState` mutation impls in this same file had already
adopted by the time I checked (`🔌️plugin/🦀️component.rs:8098`/`8194`/`8295`/`8384`). Re-ran
`cargo check`: line 6857 no longer appears in the error list.

## Gap 3 — stale ts-rs mirror

Two real, distinct bugs found and fixed (not one) via the actual generator, `bun nx run
@semio-tech/framework-rs:generate` (`cargo test --features typegen exports_typescript_bindings` under
the hood, per `🧰️framework/📦️packages/🦀️rust/📜️script.ts`) — no generated file was ever hand-edited.

1. **`ArtifactDialect` never derived `ts_rs::TS`.** `AppDefinition.dialect: ArtifactDialect` cannot
   derive `TS` if the field type itself doesn't implement it — confirmed with
   `error[E0277]: the trait bound io::ArtifactDialect: TS is not satisfied`. Fixed: added
   `#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]` to `ArtifactDialect`
   (`🧰️framework/🔨️modules/🚪️io/🦀️component.rs:49`), mirroring the exact attribute idiom `AppRole`
   already carries.
2. **`ArtifactDialect::export()` was never called** in the hand-maintained exporter test
   `exports_typescript_bindings` (`🛂️manifest/🦀️component.rs:5629`) — a large, explicit, one-call-
   per-type list; nothing in this generator config auto-recurses a referenced type's own export. Fix
   #1 alone would have made the crate compile but the generated file would still reference an
   undefined `ArtifactDialect` TS type (verified: regenerating after fix #1 alone produced
   `dialect: ArtifactDialect,` with no matching `export type ArtifactDialect = …` anywhere in the
   file — confirmed by grep before declaring done, not assumed). Fixed: added
   `crate::ArtifactDialect::export().unwrap();` right after `crate::ui::AppRole::export()`
   (`🛂️manifest/🦀️component.rs:5763`) — `ArtifactDialect` lives at the crate root
   (`pub use io::{ …, ArtifactDialect, … };`), not under `crate::ui` like `AppRole`/`AppRef`.

Regenerated twice (once per fix) and verified both times:
- `bun nx run @semio-tech/framework-rs:generate` → `framework typescript mirror refreshed ->
  🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🟦️manifest.ts`, exit 0.
- `bun nx run @semio-tech/framework-rs:check` → `framework typescript mirror is fresh.`, exit 0.
- `grep -n "role\|dialect" 🤖️generated/🟦️manifest.ts`: `AppDefinition` now carries
  `role: AppRole, dialect: ArtifactDialect,`; `export type AppRole = "viewer" | "editor";` and
  `export type ArtifactDialect = { artifactKind: string, standard: string, subset: string, };` both
  present and correctly shaped. The React shell can now read `session.app.role`/`.dialect`.

## Also fixed — demonstrator's stale `cad::apps::cad` import

`✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs` imported
`cad::apps::cad::{create_cad_app, CadPlayApp}`, a module path the cad W2 packet deleted. Derived the
real path from cad's own `📦️packages/🦀️rust/📦️glue.rs` `pub mod` nesting (`pub mod editor { pub mod
cad { … } }` at :553/555) rather than guessing — repointed the import at
`cad::editor::cad::{create_cad_app, CadPlayApp}` and changed the builder call
`.document_app::<CadPlayApp>(create_cad_app())` → `.editor::<CadPlayApp>(create_cad_app())`
(`PluginBuilder::editor::<E: ArtifactEditor>(def: AppDefinition)`, confirmed signature at
`🔌️plugin/🏗️builder/🦀️component.rs:254`). The other five `.document_app::<…>` calls in the same
chain are untouched — their plugins haven't migrated yet.

This changes cad's app id from the old hand-written `"cad-play"` to the now-derived `surface_app_id`
value `"s.cad.cad@1/*#editor"` (verified by reading `surface_app_id`'s own implementation,
`🛂️manifest/🦀️component.rs:2678`, rather than assuming the contract's example string). Two
demonstrator tests asserted the old literal id and needed updating:
`bundle_registers_the_six_demonstrator_surfaces` and
`contribution_consumers_declare_the_hidden_app_command`. Also updated two Cargo.toml
`[package.metadata.semio]` entries in `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml`
(`playground` variant "koordinator" and the `/cad-fixture` static-asset route) that keyed off the
literal string `"cad-play"` and would otherwise silently resolve to no app.

Not touched (outside this lane's authorized scope — only demonstrator was named): cad's own
`📦️packages/🦀️rust/Cargo.toml` still has two `app = "cad-play"` metadata lines (:28, :39), cad's own
remaining loose end from its W2 packet; flagged for whoever picks that up next. Checked
`♻️mit-bestand/🧺️demonstrator/🟦️brand.ts`'s `CAD_SHAPE_WINDOW_ID = "cad-play-shape"` — that is a
window-kind id (unrelated namespace, unchanged by the app-id rename), confirmed against
`…/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️shape/🦀️component.rs:13`; left alone.

## Verification

All commands and their full output are captured in `🧪️w0-f-cargo.txt` (final run):

- `RUSTC_WRAPPER="" cargo check -p semio-framework-plugin --all-targets --keep-going`
- `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-cad --all-targets --keep-going`
- `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-demonstrator --all-targets --keep-going`
- `cargo test -p semio-s-plugin-cad`
- `bun nx run @semio-tech/framework-rs:generate` / `:check`

**`semio-framework-plugin` — clean, `--all-targets`, 0 errors.** Mid-verification this went through a
transient state (8 errors, all in the foreign `testkit_tests`/`transaction_testkit_tests` fixtures and
one unrelated `Severity::Hint` reference — none inside `//#region 👁️✏️SurfaceTestkit` (:6739–7056) or
the Gap-1 re-export block (:18177–18308), checked line-by-line, not assumed) while the peer
MUTATION-OUTCOMES ticket was still landing its sweep; the LAST run in `🧪️w0-f-cargo.txt` (re-run
after that sweep completed) shows `Finished` with zero errors for both `lib` and `lib test`.

**`semio-s-plugin-cad` / `semio-s-plugin-demonstrator`** — both blocked entirely by a THIRD, unrelated
peer crate: `error: could not compile semio-s-plugin-stdio (lib) due to 1 previous error — couldn't
read …/🗄️stdio/…/🗿️artifacts/🧊️gltf/…/🧬️mutations/🚫️no-mutation/🦀️component.rs: No such file or
directory`. This is `semio-s-plugin-stdio`, owned by ticket
`26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS` — exactly the crate this
lane's brief pre-warned is "known-broken by live peer sessions." Confirmed via `git status`/`git log
--date=iso` on the missing file's containing tree before attributing it here (not this lane's file,
not this lane's ticket). `📐️cad`'s and `🎪️demonstrator`'s OWN files produce zero errors in every run
— `grep -n "📐️cad\|🎪️demonstrator" 🧪️w0-f-cargo.txt` matches only the informational `couldn't
read …/🗄️stdio/…` path text, never an error anchored in either plugin's own files.

Ran `cargo check -p semio-framework-plugin` seven times and the typegen pair twice over roughly 25
minutes while working through Gaps 1–3, polling `git status`/mtime on the actually-failing files
between attempts (protocol in memory `feedback-concurrent-cargo-workspace-churn.md`) rather than
assuming: the failure genuinely moved upstream exactly as the pilot's report predicted —
`semio-framework-os-kernel`'s `🏪️store/🦀️component.rs` (9 → 6 → 4 → 3 errors, live-edited,
confirmed via mtime deltas of seconds) resolved itself mid-session; the same `MutationOutcome`/
`validate` trait-shape refactor then surfaced in `semio-framework`'s `🔁️workflow/🦀️component.rs`
(blocked one `generate` attempt, self-resolved by the next); then in `semio-framework-plugin` itself,
where it hit my own new `SurfaceMutation` fixture (fixed above) alongside ~15 other pre-existing
impls across the same giant file (not fixed — foreign, out of lease, would be re-touching regions
owned by `testkit_tests`/`transaction_testkit_tests`/other pre-existing code this lane was told not to
revert or rewrite).

`cargo test -p semio-s-plugin-cad`'s "two testkit assertions the pilot wrote"
(`cad_viewer_never_mutates`, `cad_editor_and_viewer_share_dialect` in
`✏️s/🔌️plugins/📐️cad/🦀️component.rs:59–67`) could not be run to a pass/fail result — blocked by the
`semio-s-plugin-stdio` missing-file error above, upstream of cad in the dependency graph. Untouched by
this lane (cad's own local stand-ins — swapping them for the new framework functions is a follow-up
for whoever next touches cad, not part of this lane's scope).

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — Gap 1 (7 re-exports added to
  the existing `pub use app::{ … };` block) + Gap 2 (new `//#region 👁️✏️SurfaceTestkit` subregion,
  :6739–7056, inside the pre-existing `pub mod testkit`).
- `🧰️framework/🔨️modules/🚪️io/🦀️component.rs` — Gap 3 (`ArtifactDialect` gained
  `#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]`, :49).
- `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` — Gap 3 (added
  `crate::ArtifactDialect::export().unwrap();` to `exports_typescript_bindings`, :5763).
- `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🟦️manifest.ts` — regenerated (generator-owned; not
  hand-edited).
- `✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs` — import + builder call
  repointed at cad's new editor surface; two tests updated for the new derived app id.
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml` — two `app = "cad-play"` metadata
  entries updated to `"s.cad.cad@1/*#editor"`.
- NOT touched: `🔁️workflow/🦀️component.rs`, `testkit_tests`/`transaction_testkit_tests` (foreign
  leases, both left mid-refactor by the concurrent MUTATION-OUTCOMES ticket); cad's own
  `📦️packages/🦀️rust/Cargo.toml` (foreign lease, flagged not fixed); `semio-s-plugin-stdio` (foreign
  lease, FULL-STDIO ticket).

Scratch (ticket folder): `🧪️w0-f-cargo.txt`.

## Handoff

1. `semio-framework-plugin --all-targets` is clean as of this report (confirmed: the peer
   MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS sweep that transiently broke
   `testkit_tests::DummyMutation`/`transaction_testkit_tests::TxnMutation`/a `Severity::Hint`
   reference finished landing during this lane's session). Nothing further needed here.
2. Re-run `cargo check -p semio-s-plugin-cad`/`semio-s-plugin-demonstrator` and `cargo test -p
   semio-s-plugin-cad` once `semio-s-plugin-stdio`'s missing
   `🚫️no-mutation/🦀️component.rs` lands (FULL-STDIO ticket, confirmed still missing and that
   plugin's tree still under active edit as of this report) — expected clean for both plugins based
   on every run this lane saw (zero errors ever attributed to either plugin's own files).
3. Cad's own `📦️packages/🦀️rust/Cargo.toml:28,39` still reads `app = "cad-play"` — belongs to cad's
   lease, not fixed here.
4. The eight remaining W2 packets can now `use semio_framework_plugin::{ArtifactEditor,
   ArtifactViewer, Editor, Viewer, EditorApp, ViewerApp, ViewEmit};` directly (Gap 1) and
   `semio_framework_plugin::testkit::{assert_viewer_never_mutates,
   assert_editor_and_viewer_share_dialect, new_viewer}` (Gap 2) instead of the pilot's local
   stand-ins, and the React shell can read `session.app.role`/`.dialect` off the regenerated TS
   mirror (Gap 3).
