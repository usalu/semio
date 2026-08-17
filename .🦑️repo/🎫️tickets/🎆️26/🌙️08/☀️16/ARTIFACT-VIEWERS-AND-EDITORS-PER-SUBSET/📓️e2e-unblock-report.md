# E2E-UNBLOCK Report — cad Playground Builds Native + Wasm

Lane: E2E-UNBLOCK, ticket `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`. Read first:
`CLAUDE.md`, `📋️contract-freeze.md` §1/§2, `📓️w2-cad-report.md`, `📓️gate-stdio-s4-report.md` per the
brief.

## Baseline vs result

- **Start**: `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-cad --all-targets --keep-going` → **10
  errors**, exactly the set the brief predicted (5×`E0609` "no field `definition`", 1×`E0425`
  "cannot find type `EditorApp`", 3×`E0599` "no method `snapshot`/`dispatch` … `Result<T, E>`").
- **End**: **0 errors**, confirmed on two consecutive clean runs (`🧪️e2e-unblock.txt` §1–3).
- `cargo test -p semio-s-plugin-cad --no-run` — **links clean** (§4).
- `cargo build -p semio-s-plugin-cad --target wasm32-unknown-unknown --lib` — **0 errors** after two
  fixes (§5–7): the known repo-wide `getrandom`/`wasm_js` gap, and one more `MutationApplyResult`
  fallout site the native `--all-targets` run never reaches because it's behind
  `#[cfg(target_arch = "wasm32")]`.
- cad's four playground extension crates (`semio-s-plugin-cad-aec-building`,
  `-spatial-shape`, `-aec-building-structure`, `-aec-building-energy`) — **0 errors**, both native
  `--all-targets` and `--target wasm32-unknown-unknown --lib` (§8–9).
- **The cad playground (variant `cad`, app `cad-play`) now builds end to end, native and wasm.**

## Fixes applied

### 1. `EditorApp` unresolved (`E0425`) — import from SDK crate root

`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:1383`
— the `#[cfg(test)] mod tests` block (line 1377) does `use super::testkit::*;`, but the sibling
`testkit` module's own `use semio_framework_plugin::app::EditorApp;` (line 1269) is a private `use`,
so the glob import never re-exports it into `mod tests`. Per `📓️w0-f-report.md` Gap 1, `EditorApp`
(along with `ArtifactEditor`/`ArtifactViewer`/`Editor`/`Viewer`/`ViewerApp`/`ViewEmit`) is now
re-exported at `semio_framework_plugin`'s crate root, so the fix was adding `EditorApp` to the
existing `use semio_framework_plugin::{ActionKind, AppActionRegistry, PluginApp,
SET_ACTIVE_UTILITY_ACTION_ID};` line in that module — no `::app::` path needed.

### 2. `.definition` hop one level too deep (`E0609`, 5 sites) — remove the hop

Same file, five call sites (`app_definition_declares_one_window_scoped_dislocate_utility`,
`manifest_stitches_every_taxonomy_node_with_its_pre_migration_shape`,
`internal_and_plumbing_actions_excluded_from_palette`, and two
`AppActionRegistry::from_definition(&create_cad_app().definition)` calls in
`context_menu_...`/`context_menu_is_grouped_and_keeps_delete_object_last`) all did
`create_cad_app().definition`. `create_cad_app()` already returns `AppDefinition` directly per
contract §2.4 (`Editor::builder(...).build_definition()`), so `.definition` was reaching for a field
that doesn't exist. Fixed by dropping the `.definition` hop at all five sites — `create_cad_app()`
used bare.

### 3. `store` never unwrapped (`E0599` ×3) — test fixtures, `.expect()` is in-bounds

`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`,
both test functions (`cad_projection_defaults`, `create_shape_model_round_trips_through_store`):
`ArtifactStore::new(envelope: ArtifactEnvelope<P, Mutation>) -> Result<Self, VcsError>`
(`🏪️store/🦀️component.rs:4289`) is fallible (`validate_durable_history`/`fold_history` inside it
propagate `VcsError`), and both `let store = CadStore::new(...)` bindings were never unwrapped —
`store` was `Result<CadStore, VcsError>`, so `.snapshot()`/`.dispatch()` didn't resolve. Fixed by
appending `.expect("store")` at both construction sites — test code building a known-valid fixture,
in-bounds per the brief's "tests may unwrap a known-valid fixture" carve-out. No production code
touched here.

### 4. wasm-only production site — same `Result<Self, VcsError>` gap, fixed by propagation not unwrap

`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:25,27`
— `CadArtifactVcs::new` (the `wasm-bindgen` VCS bridge constructor, gated
`#[cfg(target_arch = "wasm32")]` so `cargo check --all-targets` on the native target never compiles
it) had the identical unwrapped-`CadStore::new(...)` shape, but here it's real production code
returning `Result<CadArtifactVcs, JsValue>` already. Fixed by propagating the typed rejection through
the existing `.map_err(|e| JsValue::from_str(&e.to_string()))?` idiom already used three lines below
in the same file (`dispatch_text`/`dispatch_binary`/`snapshot_json`) — no `unwrap`/`expect`, no
discarded `Result`, diagnostic preserved end to end into the `JsValue` error the JS host sees.

### 5. wasm32 `getrandom` feature gap — same fix as the two sibling crates already carry

`✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml` `[target.'cfg(target_arch = "wasm32")'.dependencies]`
gained `getrandom = { version = "0.3.4", features = ["wasm_js"] }`, matching
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml`
and `compose/client/lib/rs/Cargo.toml`'s existing identical entries. `.cargo/config.toml` already sets
`--cfg getrandom_backend="wasm_js"` for `[target.wasm32-unknown-unknown]` repo-wide, so only the
crate-level feature flag was missing here. cad's four extension crates inherit the fix transitively
(three depend on `semio-s-plugin-cad` directly or share the same resolved dependency graph within one
`cargo build` invocation; verified by building all four for wasm32 — 0 errors, no per-extension
`Cargo.toml` edit needed).

## Verification commands (full output: `🧪️e2e-unblock.txt`)

1. `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-cad --all-targets --keep-going` — baseline 10
   errors → 0, confirmed clean twice more after.
2. `RUSTC_WRAPPER="" cargo test -p semio-s-plugin-cad --no-run` — links.
3. `RUSTC_WRAPPER="" cargo build -p semio-s-plugin-cad --target wasm32-unknown-unknown --lib` — 0
   errors (two fix iterations: getrandom, then the wasm-bridge `Result` unwrap).
4. `RUSTC_WRAPPER="" cargo build -p semio-s-plugin-cad-aec-building -p semio-s-plugin-cad-spatial-shape
   -p semio-s-plugin-cad-aec-building-structure -p semio-s-plugin-cad-aec-building-energy --target
   wasm32-unknown-unknown --lib` — 0 errors.
5. `RUSTC_WRAPPER="" cargo check` (native, `--all-targets --keep-going`) on the same four extension
   crates — 0 errors.

## Cad playground plugin set (checked, not assumed)

`✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml`'s only `[[package.metadata.semio.playground]]`
row is `variant = "cad"`, `app = "cad-play"` — a single-plugin dev variant (ports react 6020 / wgpu
6120). It does not itself list "several plugins" as sibling playground rows; what it does pull in is
four separate extension crates under `✏️s/🔌️plugins/📐️cad/🧩️extensions/` (`role = "extension",
extends = "cad", contributes = ["cad.computer"]`, found by grepping the whole tree for the literal
`cad-play` string, not by guessing) — all four now build clean, native and wasm, listed above.

No wasm32-specific nx target exists for cad in `📦️packages/🦀️rust/📋️project.json` (confirmed absent,
matching the pilot's own finding — wasm packaging for the component target is orchestrated elsewhere,
not per-plugin `nx` targets); the `--target wasm32-unknown-unknown --lib` `cargo build` above is the
closest direct check available and is the one that surfaced both wasm-specific bugs.

## Reported, not fixed — outside this lane's lease

`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-demonstrator --all-targets --keep-going` → **42
errors**, but **zero of them are inside `semio-s-plugin-demonstrator` or `semio-s-plugin-cad`
files** — every single one is inside `semio-framework-os-flow` (`🌿️vcs/🦀️component.rs:193`),
`semio-s-plugin-gis` (`🦀️component.rs:15-16`), or `semio-s-plugin-puzzle` (36 errors across its
`🧊️3d`/`🖐️5d`/`◻2d` mutation-diff leaves — parse errors on `Puzzle*Diff` plus `SemanticMutation`
trait-bound/arity mismatches). Confirmed live, not stale, via `git log --date=iso` (never commit
*message* text, which embeds a frozen fake date template — see memory
`feedback-auto-commit-message-date-is-fake.md`): `🌿️vcs/🦀️component.rs`'s own last commit is
`c8a29e4` at **2026-08-16 20:26:15** ("Refactor OS store schema mutations and SPR command resolution
with change merge policy"), roughly 80 minutes before this check ran — squarely the
`MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` ticket's sweep still landing across
`gis`/`puzzle`/`flow`, the exact pattern `📓️w2-cad-report.md` and `📓️w0-f-report.md` both already
hit and predicted would keep moving. `git status --porcelain` on all four affected trees is clean (no
local uncommitted edits) — this is committed-but-still-broken churn from a live peer session, not
something to revert or fix here.

**Scope note**: `semio-s-plugin-demonstrator` is the *koordinator* playground (a different variant
that embeds six plugin panes including cad), not the *cad* playground this lane owns — its own
compile status could not be observed at all this run (the build never got past its `gis`/`flow`
dependencies), so it is reported as blocked, not fixed and not claimed clean.

## Files touched

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs` —
  `EditorApp` import added to `mod tests`; five `.definition` hops removed.
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`
  — two test-fixture `CadStore::new(...)` calls gained `.expect("store")`.
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`
  — `CadArtifactVcs::new`'s two `CadStore::new(...)` call sites now propagate the typed `VcsError` via
  `.map_err(...)?` instead of passing an unwrapped `Result` into `RefCell::new`.
- `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml` — added
  `getrandom = { version = "0.3.4", features = ["wasm_js"] }` under the wasm32 target dependencies.

Scratch (ticket folder): `🧪️e2e-unblock.txt` (all ten verification runs, full output).

## Handoff

1. Cad playground (native + wasm) is unblocked; the browser end-to-end verification this ticket was
   waiting on can run against it now.
2. `semio-s-plugin-demonstrator`'s koordinator playground is still blocked, upstream of cad, by live
   `MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` churn in `gis`/`puzzle`/`flow` — worth
   a re-check once that ticket's sweep finishes landing, not a cad-lane action item.
