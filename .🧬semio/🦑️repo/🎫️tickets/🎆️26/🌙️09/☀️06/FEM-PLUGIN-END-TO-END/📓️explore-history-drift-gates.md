# Explore: fem plugin — history, peer-ticket findings, API drift, external gates (no compile)

Method: read-only. Repo MCP (`repo`/`semio`) timed out at session start — bookkeeping/lookups below
are by hand on disk (`git log/status`, `grep`, `find`, a small path-resolver script). No cargo/nx/bun
was run. Target: `semio-s-plugin-fem` at `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs`.

## 1. History — 5 prior fem tickets

| # | Ticket | Status | Left open |
|---|---|---|---|
| 1 | `🌙️07/☀️17/FEM-2D-AND-FEM-3D-STRUCTURAL-ANALYSIS-APPS` | closed | Built fem_core/fem_2d/fem_3d/fem-plugin from scratch (169 tests). Deferred: 3D continuum/solid regions, nodal-averaged stress smoothing, 3D mode-shape captions, gesture-based interaction. |
| 2 | `🌙️07/☀️19/FEM-TECHNOLOGY-ABSOLUTELY-FEATURE-COMPLETE` | closed | Round 1 closed the v0/v1/v2 deferrals (189 tests). Round 2 fixed a real end-to-end bug: fem never handled the shell's `setActiveExample` boot dispatch, so both apps booted empty — added `SetDocument` op + handler, rebuilt example fixtures, fixed deformation-scale wiring, added the full UI action set, fixed the dev hot-swap watcher. No open items recorded. |
| 3 | `🌙️07/☀️19/RUST-CLEAN-REFACTOR-WAVE-12-NORM-CRATES-FEM-2D-FEM-3D` | **open** (no summary, no important.md) | Assigned fem/2d/rs + fem/3d/rs (pre-consolidation crate layout) for a warnings/clippy/thiserror pass "additive/minimal only". Ticket has never been closed and predates the crate consolidation (ticket 5, below) that deleted the very crates it names (`fem/2d/rs`, `fem/3d/rs` no longer exist as separate crates) — this ticket is stale/superseded, not a live blocker. |
| 4 | `🌙️07/☀️18/RUST-CLEAN-REFACTOR-WAVE-2-…FEM-CORE` | closed | fem/core/rs: converted hand-rolled `FemError` to `#[derive(thiserror::Error)]`, cargo-fmt reformat. No open items. |
| 5 | `🌙️08/☀️05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION` | left open on disk (`important.md`: "Close via repo MCP when available" — repo MCP was down) | Consolidated 17 old `⚡️implementations` trees into the single `semio-s-plugin-fem` crate (current shape). `cargo check -p semio-s-plugin-fem`: **GREEN** at close (2026-08-06). `cargo test`: 318 passed / 10 failed, all in `store::test_support::assert_dsl_round_trip` — a **framework-wide** DSL-preamble asymmetry (printer emits `semio fem.fem2d dsl v1`, parser wants `semio fem.fem2d.dsl v1`), reproduced on `semio-s-plugin-block` too; registrar's own words: "framework-wide printer/parser asymmetry, not a fem consolidation defect", tracked as a shared issue, not fem's to fix alone. |

git log (`--date=iso`, real dates) on `✏️s/🔌️plugins/🏗️fem` + `✏️s/🔨️modules/🏗️fem`, last 20: newest `3a6a9d6bfc` 2026-09-05 22:02:04, oldest of the 20 `82eacf7b93` 2026-08-24 16:06:17 — steady one-repo-wide-commit-per-session cadence (commit subjects are the fake `🚩️591`-style template, per memory, not descriptive).

`git status --short` on both dirs: **clean** — no uncommitted peer edits in-flight on fem itself right now.

## 2. Peer-ticket findings mentioning fem (09/*, 08/☀️2*)

- **`🌙️09/☀️05/S-END-TO-END/📓️explore-per-plugin-blockers.md:49-50`** (the authoritative per-plugin blocker census, generated same day as this ticket):
  ```
  | fem | 3d | 🏗️fem/🗿️artifacts/🧊️3d | 4/0 | 0/16 | y/n | n/y | y | 0+2 | OK | catalog audit: missing mutation module | 16 viewer dead. |
  | fem | 2d | 🏗️fem/🗿️artifacts/◻️2d | 4/0 | 0/16 | y/n | y/n | n | 1+0 | OK | same | Same. |
  ```
  Reading the header row: Mig E/V=4/0, **Batch E/V=0/16** (16 viewer commands are still `BatchOnlyPendingRewrite`, i.e. hard-dead per the ticket's own migration-status classification — see memory `Interactive-Job Classification Gates Dispatch`), Fac(factory_type present) E/V = y/n (editor has it, viewer doesn't — expected, viewers don't need tool-job factories), Proof(`bounded_first_step_tool_proofs!`) = n/y for 3d / y/n for 2d (inconsistent between the two apps — worth a follow-up, not a compile blocker), `setActiveExample` reachable = y for 3d / n for 2d (2d's own source has it per §3 below — this row may be stale or means something narrower), Descriptor = OK, "catalog audit: missing mutation module" is the noted concrete blocker for BOTH — this is a **stdio full-artifact-catalog build-order** issue (see §4), not a fem source defect.
- **`🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📋️master-plan.md:353`**: "the exact FEM-root `cargo metadata --no-deps` command advances through the prior missing-manifest frontiers and exits 0" — i.e. as of that entry, fem's manifest graph resolves cleanly (Cargo.toml dependency paths are not broken); explicitly caveated as "manifest-graph evidence only, does not supersede downstream plugin source diagnostics" — so this is necessary but not sufficient for a green `cargo check`.
- **`🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/RESUMABLE-FEM-JOB-GRAPH/*`** (12+ files, `📓️p6a-job-graph.md` … `📓️p6i-fem-live-visual-publication-implementation-2026-08-24.md`): a whole sub-ticket devoted to fem's resumable job graph (incremental mesh job, deterministic assembly job, direct eigen jobs, numerical microcursor, live-visual publication) — heavily audited/re-audited (terra/codex/sol/coordinator passes) through 08-23 to 08-26. No final consolidated verdict file found in the time available; several "post-remediation"/"reaudit" files suggest this took multiple correction rounds. Relevant because fem's `pending_effects`/live-visual reconcile path (seen live in `✏️editor/🦀️.rs`) is exactly what this sub-ticket was hardening — worth reading in full before touching that code path.
- **`🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️fem-carrier-reader-retrofit.md`**: exists (not read in full — budget), title indicates fem's IO carrier readers needed an oracle retrofit as part of the wider subset-scoped-oracle campaign.
- **`🌙️08/☀️29/S-END-TO-END/📓️explore-s-app-identity.md`**, **`🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/🔍️research/📓️serde-fem-conversion-wave.md`** and `📓️serde-fanout-fem-process.md`: a dedicated serde→`ToValue`/`FromValue` conversion wave was already run against fem specifically; §3.i below confirms fem's own source has **zero** `#[derive(Serialize/Deserialize)]` or `serde_json` in real crate code today (only a standalone dev-tool `json-engine` binary under `🏭️generator/`, and safe `dsl::json::to_dsl_value` bridge calls) — consistent with that wave having already landed.
- `📓️opus-fault-discriminators.md` and `📓️stdio-check-census.md` (both `09/☀️05/S-END-TO-END`): **no "fem" hits** — fem is not called out in either, i.e. no known fault-discriminator or stdio-check-census entry specific to fem exists yet.
- `✅️acceptance-matrix.md` (`09/☀️02/COMPLETE-SEMIO-END-TO-END`): **no "fem" hits**.
- Current ticket's own `📓️status.md` confirms this explorer is one of 6 parallel fleet agents launched by the coordinating session for this exact ticket.

## 3. Framework-API drift, grep-level (no compile)

**Total `async fn` in fem: 160. `#[async_test]`-annotated: 0.** All 160 are production `async fn` — none are tests. This is expected, not a red flag: fem's `handle_action`/`handle_command`/tool-job-factory plumbing (framework-owned, invoked via `ArtifactEditor`/`EditorApp<Self>` adapter) is genuinely async at the outer dispatch layer; the count does not by itself indicate drift.

**a. `diff`/`inverse` on mutation kinds vs `MutationKind` trait** — **no drift**. Trait def, `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs:215-224`:
```rust
pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
where Op: Mutation<P> {
    fn diff(&self, base: &P) -> MutationOutcome<<Op as Mutation<P>>::Diff>;   // sync
    fn inverse(&self, base: &P) -> Vec<Op>;                                   // sync
```
Every fem `diff`/`inverse` impl is sync, e.g. `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/⚖️change-load-case-self-weight/🦀️.rs:23,26`. No `async fn diff`/`async fn inverse` exist anywhere in fem (`grep` returned zero matches). Matches the current trait exactly.

**b. `handle` vs the app trait** — **no drift**. fem implements `ArtifactEditor` (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs:650: impl ArtifactEditor for Fem3dPlayApp`), not the older `ArtifactApp` trait directly (that trait, `🔌️plugin/🦀️.rs:11120`, is `async fn handle(...)`, framework-internal/older shape). The **current** `ArtifactEditor::handle` (`🔌️plugin/🦀️.rs:26818-26826`) is **sync**:
```rust
/// @emoji 🧩️ The pure heart of an editor — identical shape to `ArtifactApp::handle`.
fn handle(command: &Self::Command, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>,
    interaction: &InteractionView<'_>, draft: &DraftView<'_, Self::Draft>, engines: &EngineHandles,
) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault>;
```
fem3d's impl at `✏️editor/🦀️.rs:833-841` matches this signature exactly (sync, all 6 params, same return type). fem's per-command payload `handle(payload, doc, cfg)` functions under `🎮️commands/*` (e.g. `➕️add-load/🦀️.rs:26`) are a narrower macro-dispatched shape (`app_commands!`, `🔌️plugin/🦀️.rs:10727-10756`) that the editor's single `handle` fans out to via `command.dispatch(doc, cfg)` — also unchanged. fem's `🎮️commands/` top-level dir (plugin-root, not artifact-scoped) is empty (`📌️.empty.md` only) — not a defect, fem's commands live under each artifact's `✏️editor/🎮️commands/`.

**c. `print_dsl`/`parse_dsl`/`encode_op`/`decode_op`/`print_op`/`parse_op`** — **no drift**. All sync in fem (e.g. `🧬️schema/📸️snapshot/🦀️.rs:49,57`, `👁️viewer/🦀️.rs:25,28`, `🧬️mutations/📝️text/🦀️.rs:13,24,33,36`). Zero `async fn` variants of any of these six names anywhere in fem.

**d. `render` return type / `UiNode` helpers** — **mostly clean, one dead-import nit**. `render` correctly returns `UiAssemblyResult<ComponentTree>` everywhere in fem (matches `ArtifactEditor::render`, `🔌️plugin/🦀️.rs` trait body), never a bare `ComponentTree`. `UiNode`/`ui_stack_vertical`/`ui_text` are **not** retired — they're the live wgpu-target declarative-tree API, still defined at `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🧩️component.rs:3773` (`ui_stack_vertical`) and `:3821` (`ui_text`), re-exported into `semio_framework_plugin::*` via `pub use ui_wgpu::wgpu::*;`, and used by 14 other plugins. **However**: both fem viewer files import `UiNode` and never use it — `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/…/👁️viewer/🦀️.rs:11` and `…/🧊️3d/…/👁️viewer/🦀️.rs:11` list `UiNode` in their `use semio_framework_plugin::{...}` but it appears nowhere else in either file. Cosmetic (unused-import warning, not a `cargo check` error under the current workspace lints — `unused_imports` isn't in the `[workspace.lints.rust]` deny-list at root `Cargo.toml:309-314`), but worth a one-line cleanup.

**e. `command_from_action`** — **no drift, not overridden**. fem never overrides `ArtifactEditor::command_from_action`; the default (`🔌️plugin/🦀️.rs:26828-26835`, sync, `Result<Self::Command, Fault>`) is used as-is. Nothing to drift against since fem doesn't touch it.

**f. Config/Presence `DESCRIPTORS`** — present in 3 files (`🗿️artifacts/🧊️3d/…/🌐️any/🦀️.rs`, `…/✏️editor/🧵️session/🦀️.rs`, `🗿️artifacts/◻️2d/…/🌐️any/🦀️.rs`); not independently verified against the current enum-descriptor contract in the time available — flag as unverified, not a known defect.

**g. `with_ports` on async `AppIo`** — **zero matches** in fem. Not applicable to fem (this pattern belongs to a different plugin family per the block/draw/raster tickets).

**h. `ActionDescriptor::` helper / `Label: From<…>`** — **zero direct matches** for the exact literal patterns searched (`ActionDescriptor::`, `impl From<…> for Label`, `Label::from`). Either fem doesn't use this helper family at all, or it's called through a macro/builder indirection the grep didn't catch — not conclusively cleared, flag as unverified.

**i. serde vs `ToValue`/`FromValue`** — **clean**. Zero `#[derive(Serialize/Deserialize)]` or `serde_json::` in any real fem `.rs` source file. The only serde hits are (1) a fully separate standalone dev-tool crate `🗿️artifacts/{◻️2d,🧊️3d}/…/🏭️generator/🦀️json-engine/` (its own `Cargo.lock`/`target/`, a build-time fixture generator, not part of `semio-s-plugin-fem`'s own Cargo.toml dependency graph) and (2) legitimate `dsl::json::to_dsl_value(&serde_json::Value)` bridge calls in media-import code (`✏️editor/🦀️.rs:805,807` in both 2d and 3d) — this is the sanctioned external→internal `Value` conversion, **not** the risky self-referential `impl ToValue for X { … store::to_dsl_value(self) … }` recursion pattern from the memory note; no such recursive impl exists in fem.

**j. `bounded_first_step_tool_proofs!` without `factory_type`** — **no drift**. Both fem2d (`◻️2d/…/✏️editor/🦀️.rs:447,453`) and fem3d (`🧊️3d/…/✏️editor/🦀️.rs:679,685`) pair the macro with an explicit `factory_type: Fem{2,3}dRetainedCommandJobFactory,` — the bare-factory dead-action bug from the memory note does not apply here.

**k. `#[path]` mounts — CONFIRMED, SEVERE, real `cargo check` blocker.** Wrote a small Python resolver that walks every `#[path = "..."]` + `mod` pair from the crate entry (`📦️packages/🦀️rust/🦀️.rs`, 1611 lines, 525 `#[path]` attributes) recursively against the real filesystem. **43 mount targets do not exist on disk.** Root cause, verified directly: at some point every one of ~42 deeply-nested mutation-test fixture directories under fem was renamed on disk to a hash-truncated short form, but the crate's `#[path]` mod declarations (and sibling `include_str!` calls in the per-subset test aggregator files) still reference the original long descriptive names. Confirmed example:
- On disk: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🚫️removes-node-n3-without-6eab3f/` (hash-truncated).
- Referenced (does not resolve) at `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs:195`: `#[path = "../../🗿️artifacts/◻️2d/…/🕳️delete-node/🧪️tests/🚫️removes-node-n3-without-cascading-to-its-support/🦀️.rs"]` (full name — **directory doesn't exist**, this `mod` will fail with `error[E0583]`/`file not found for module`).
- Same stale full-name path also appears in `include_str!` calls (5 JSON fixture reads per fixture) inside the *sibling* per-subset aggregator files, e.g. `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/…/🕸️mesh/🧪️tests/🕸️mutate-fem2d-1-mesh/🦀️.rs:64-68` and `…/🌐️any/🧪️tests/🕸️mutate-fem2d-1-any-mesh/🦀️.rs:65-69` — these will fail at `include_str!` macro-expansion time (a hard compile error, not a lint) even independent of the `#[path]` mod failure.
- A second confirmed instance in fem3d: on disk `…/🛡️boundary/…/🕳️create-support… ` → truncated to e.g. `🔒️clamps-the-column-f801c9`; 3 such fem3d mutation-test directories under `🛡️boundary` alone, plus more under `🧱️material`, `🕸️mesh`, `🏋️load`, `📈️analysis`.
- **Scope**: `find … -regex '.*-[0-9a-f]\{6\}$'` counts **42** such truncated directories under `✏️s/🔌️plugins/🏗️fem` (mix of fem2d and fem3d, across `🕸️mesh`, `🧱️material`, `🛡️boundary`, `🏋️load`, `📈️analysis` subsets). Every one of them is a `#[path]` mod target from the crate entry and almost certainly also has stale `include_str!` siblings the way `delete-node` does. This alone is very likely enough to make `cargo check -p semio-s-plugin-fem` fail natively and on `wasm32-wasip2` with dozens of "file not found" module/macro errors, **before any real logic is even inspected**. This does not appear to be the Win32-`MAX_PATH` issue investigated separately in `🌙️09/☀️05/WINDOWS-CHECKOUT-ILLEGAL-FILENAMES/🔬️findings.md` (that audit found 0 illegal names and only flagged path length as a *Windows-checkout* risk, not an on-disk rename) — this looks like a **separate, already-executed** truncation (possibly a macOS/tool path-length limit hit during some bulk file operation, since these are the deepest-nested files in fem) that renamed directories without repointing their referrers. Root Cargo.toml itself resolves fine (per master-plan.md:353's `cargo metadata` finding in §2) because manifest resolution never looks inside these leaf `mod`/`include_str!` targets.

## 4. External gates (dependency crates)

fem's `Cargo.toml` dependencies (`✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/Cargo.toml`):
`semio-s-plugin-stdio` (features `full-artifact-catalog`), `semio-framework-os-kernel`, `semio-framework-plugin`, `semio-framework-dispatch-macros`, `semio-framework-ui-contract`, `semio-framework-ui-scene`, `semio-framework-schema`, `semio-framework-value-derive`, `semio-framework-job` (workspace), `semio-framework`; dev-deps `semio-framework-async`, `semio-framework-async-macros`.

| Dep dir | git status | Recent commits (--date=iso) |
|---|---|---|
| `✏️s/🔌️plugins/🗄️stdio` | clean | 3a6a9d6bfc 2026-09-05 22:02, b0dfa0f09b 2026-09-05 19:04, fe7c8a8f8b 2026-09-05 03:53 |
| `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust` (os-kernel) | **M** `📋️project.json`, `📜️script.ts` uncommitted | same 3 commits as above |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin` (the `ArtifactEditor` trait's own crate) | **actively in-flight**: `AM/MM` on ~18 files under `🌐️browser-bundle/` (wasi/host bundling, fixtures) and `M` on `📇️registry/✅️catalog-complete.test.ts` | same 3 commits |
| `🔀️dispatch`, `🧬️schema`, `🎬️scene`, `✨️derive` (value) | clean | — |
| `🧰️framework/📦️packages/🦀️rust` (framework root) | clean | — |

**Reading**: the trait fem is coupled to (`ArtifactEditor` in `🔌️plugin/🦀️.rs`) lives in a crate with **live uncommitted peer edits right now**, but every changed file is under `🌐️browser-bundle/` (WASI/browser guest bundling + its fixtures) and one registry test — nowhere near the `ArtifactEditor`/`EditorApp` trait bodies read in §3. Low risk of the trait shape moving under fem mid-session, but re-check `git status` on this dir immediately before any real build (memory: `Concurrent Cargo Workspace Churn`). Per memory (`Native Cargo Misses Wasm-Gated Code`, `stdio had 225 errors mid-migration on 09-05`, `os-kernel E0432 on 09-05`) stdio/os-kernel have had recent multi-hundred-error migration windows on this exact date — worth a fresh, isolated `cargo check -p semio-s-plugin-stdio` / `-p semio-framework-os-kernel` before blaming fem for any error that surfaces through them.

**Active builds right now**: `ps aux | grep -i fem` — **no fem-related process running**. No `target-fem*` directory exists at repo root (fem has never had its own isolated `CARGO_TARGET_DIR` from a prior session in this environment, based on what's currently on disk); several unrelated `target-*` dirs exist for other plugins (block, draw, energy, lowpoly, etc.) — fem is not among them, so no stale lock risk from a fem-specific target dir.

## 5. Registry/taxonomy

No repo-wide `taxonomy.json` exists (search for that literal filename found only an unrelated print-product visualization file, `🧰️framework/🛍️products/📓️print/🖼️assets/🔣️viz-taxonomy.json`). fem's own membership/shape is instead declared in its owner-root descriptor `✏️s/🔌️plugins/🏗️fem/🔣️.json` (regenerated by the `describe` tool per the ticket's definition-of-done item 3) — read its `manifest.pluginId: "fem"`, `apps: [s.fem.fem2d@1/*#editor, s.fem.fem3d@1/*#editor]` header; a full memberNames-vs-disk diff (the "Artifact Name Registry Gate" memory pattern) needs the actual `describe`/registry-check tool or repo MCP, neither of which is usable read-only from here (repo MCP timed out; the checker isn't a static file to diff by hand at this scope) — **not independently verified this pass**, flag as open verification item for whichever agent runs the actual `cargo check`/`describe` step.

## Gaps to fix — prioritized

### (A) fem-internal drift we own
1. **P0 — 42 stale `#[path]`/`include_str!` targets** (§3.k): every hash-truncated mutation-test directory under fem needs its referencing `#[path]` mod line (crate entry `🦀️.rs`) and sibling `include_str!` calls (per-subset `🧪️tests/🕸️mutate-fem*-*-*/🦀️.rs` aggregator files) repointed to the actual on-disk truncated name. This is almost certainly the actual, mechanical reason `cargo check -p semio-s-plugin-fem` is not green right now — fix this first, before anything else, and re-grep for the same truncation pattern (`-[0-9a-f]{6}$` directories) to confirm all 42 are caught, not just the `delete-node`/`create-support` ones sampled here.
2. **P3 — cosmetic**: unused `UiNode` import in both viewer files (§3.d) — one-line removal each, not a build blocker.
3. **P3 — unverified, low-confidence**: DESCRIPTORS (§3.f) and ActionDescriptor/Label::From (§3.h) contract conformance not independently confirmed — worth a quick look once P0 is fixed and a real `cargo check` can actually run far enough to surface any real mismatch here directly.
4. **Stale ticket housekeeping (not a code gap)**: `🌙️07/☀️19/RUST-CLEAN-REFACTOR-WAVE-12-…-FEM-2D-FEM-3D` is still `"status": "open"` but targets crates (`fem/2d/rs`, `fem/3d/rs`) that no longer exist after the 08/05 consolidation — someone should close it as superseded (not this session's call per CLAUDE.md — flag to the dev/coordinator, don't close it unilaterally).

### (B) external gates we wait on / must re-check fresh, not fem's to fix
1. The `🌙️09/☀️05/S-END-TO-END` per-plugin blocker table's "catalog audit: missing mutation module" note for fem (§2) is a **stdio full-artifact-catalog** build-order issue, not fem source — re-run the stdio catalog audit after P0 above lands, since fem couldn't even compile far enough to be catalog-audited cleanly before its own `#[path]` errors are fixed.
2. `🌙️08/☀️05` ticket's 10 remaining test failures (all `assert_dsl_round_trip`, DSL-preamble printer/parser asymmetry, dotted vs spaced format) are a **framework-wide** defect shared with `semio-s-plugin-block` — track/fix at the framework DSL layer, not per-plugin.
3. `🔌️plugin` crate (home of `ArtifactEditor`) has live uncommitted peer edits in `🌐️browser-bundle/` right now (§4) — re-check `git status` immediately before building; low risk to fem's own trait usage based on which files are touched, but don't assume it stays that way.
4. Registry/taxonomy memberNames-vs-disk diff for fem (§5) needs the real `describe`/registry tool, not available read-only this pass — run it as part of (or right after) a real build.
