# 🪣️ Master plan — interactive tools with visible process

Ticket `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS`. Inputs: the five `📓️audit-*.md` reports in this folder.
Editor root `E` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.
Host root `H` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`.

## 1. Product decisions (opinionated, final for this ticket)

1. **No plan-ahead.** The fill planner plans toward the REQUESTED count only (`runtime.fill_count`), never toward a
   hidden constant. `FILL_COUNT_MAX`, `PUZZLE3D_FILL_COUNT_MAX`, `PUZZLE5D_FILL_COUNT_MAX`, `PUZZLE2D_FILL_COUNT_MAX`
   are deleted. Raising the count continues the same deterministic sequence (prefix property already holds).
   Lowering deletes the locked tail from the document and discards the planned tail from the builder.
2. **Locking is committing.** A candidate the planner accepts becomes a real `create_object` + `connect_vortices`
   document mutation on the next `fillBuildTick` (up to 8 per tick), coalesced under history key `fill-count`.
   The render-time ghost tail (`puzzle3d_fixture_with_fill_display_memo`, `append_fill_display_tail`,
   `compose_fill_display`, `FillDisplayMemo`), the slider `reveal` key and the `revealCutoffs` interaction block
   are removed. What the user sees is exactly the document.
3. **Every tried candidate is visible.** The preview publishes the current candidate ghost WITH a verdict
   (`testing | free | collision | rejected | accepted`) and a ring of the last 12 tried candidates with verdicts.
   The viewport paints `collision` ghosts with the `danger` token and `free`/`accepted`/`testing` ghosts with the
   `highlighted` style. The HUD reads `tested / locked / requested` plus a localized stage label (EN + DE).
4. **Unbounded count, default 100.** `Puzzle3dConfig::fill_count` defaults to 100 (Rust default, JSON schema
   default, TS guard). The count control is a new framework `WindowMeasure::Number` with `max: None`; typing
   any value is allowed. Capacity limits of the fixed document pages are reported as a visible stall reason
   (`stallReason`), never as a fault and never as a clamp.
5. **Generic progress measure.** A new framework `WindowMeasure::Progress` (stage, completed, total?, last steps,
   cancel action) replaces the `Toggle.text` hack. Puzzle 3d fill is the first user; 2d/5d fill and brush follow.
6. **Puzzle 2d/5d parity.** Same unbounded default-100 count semantics; 5d slider wired to the 3d session's
   progress/cancel. Brush suggestions (3d) stream "trying k of n" instead of hiding the search.

## 2. Shared contract (names every wave codes against — do not rename)

### 2.1 Schema (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/🧬️schema/🦀️.rs`, owner: wave A)

```rust
pub const FILL_TRIED_RING: usize = 12;
#[value(rename_all = "camelCase")] pub enum FillCandidateVerdict { Testing, Free, Collision, Rejected, Accepted }
pub struct FillTriedCandidate { pub sequence: u64, pub verdict: FillCandidateVerdict, pub reason: Option<String>, pub ghost: BrushPreviewState }
// FillBuildPreview additions (existing fields stay):
pub verdict: FillCandidateVerdict,            // of candidate_ghost
pub tried: [Option<FillTriedCandidate>; FILL_TRIED_RING],
pub tested_count: u64,                        // constructed previews (accepted + rejected + testing)
pub requested_count: usize,                   // replaces the meaning of total_count (total_count removed)
pub stall_reason: Option<String>,             // e.g. "no-open-vortex", "document-capacity", "no-compatible-kind"
// FillProgressSummary additions (count/applied_count/max_count/done stay; max_count == requested count):
pub tested: u64, pub rejected: u64, pub collisions: u64, pub stage: String, pub stall_reason: Option<String>
```
Wire (ghost JSON, produced by the fill preview JSON cursor, consumed by `H/🌐️World3dHost/🟦️.tsx`):
ghost gets `"verdict"`; `fillBuildPreview` gets `"verdict"`, `"tried": [{"sequence","verdict","reason"?,"ghost":{...}}]`,
`"testedCount"`, `"requestedCount"`, `"stallReason"`. Byte cap raised 4096 → 16384 everywhere it is pinned
(`FILL_PREVIEW_JSON_MAX_BYTES`, `WORLD_FILL_PREVIEW_JSON_MAX_BYTES`, fixture `maximumBytes`, policy tests).

### 2.2 Builder (`E/⏳️precompute/🪣️fill/🦀️.rs`, owner: wave A)

```rust
impl FillBuilder {
  pub(crate) fn begin_preparation(roots: FillPreparationRoots, operation: Operation, requested_count: usize) -> Self;
  /// raises max_count and un-stalls a Complete builder; lowering sets max_count and begins tail discard down to
  /// max(requested, applied_count) via the existing DiscardTail machinery.
  pub(crate) fn set_requested_count(&mut self, requested: usize);
  pub(crate) fn requested_count(&self) -> usize;
}
```

### 2.3 Session (`E/⏳️precompute/🦀️.rs`, owner: wave B1)

```rust
impl Puzzle3dPrecomputeSession {
  pub fn fill_requested_count(&self) -> u32;
  /// idempotent; transports the new target to the live envelope/worker (raise) or arms tail deletion (lower)
  pub fn set_fill_requested_count(&mut self, count: u32);
  /// document-side delta toward the requested count: new locked placements as create/connect, or tail deletes
  pub fn take_fill_locked_chunk(&mut self, max_delta: usize) -> Option<FillApplyChunk>;
  pub fn fill_progress_summary(&self) -> FillProgressSummary; // extended per 2.1
}
pub(crate) const FILL_LOCK_PLACEMENTS_PER_TICK: usize = 8;
```
`start_fill_preparation` seeds the builder with `scene.runtime.fill_count`. The envelope authority carries the
requested count so the isolated worker reads it every step.

### 2.4 Framework measures (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs`, owner: wave F)

```rust
WindowMeasure::Number { id: String, label: Option<String>, value: f64, min: Option<f64>, max: Option<f64>, step: Option<f64>,
  ready: Option<f64>, loading: Option<bool>, waiting: Option<bool>, disabled: Option<bool>, on_change: ActionDescriptor }
WindowMeasure::Progress { id: String, label: Option<String>, stage: Option<String>, completed: f64, total: Option<f64>,
  steps: Vec<MeasureProgressStep>, cancel: Option<ActionDescriptor>, loading: Option<bool> }
pub struct MeasureProgressStep { pub kind: MeasureProgressStepKind, pub text: String }
pub enum MeasureProgressStepKind { Info, Success, Warning, Danger }   // wire: "info"|"success"|"warning"|"danger"
```
Wire kinds: `"number"`, `"progress"`. TS mirror in `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs`, regenerated
manifest `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` via `bun nx run @semio-tech/framework-rs:generate`.

### 2.5 Interaction JSON `fillBuild` block (`E/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs`, owner: wave C)

`{ count, appliedCount, requestedCount, tested, rejected, collisions, done, stage, stallReason }` — `revealCutoffs` removed.

## 3. Waves (Opus 5 High, parallel) — file ownership

| Wave | Owns | Report |
|---|---|---|
| A planner | `E/⏳️precompute/🪣️fill/🦀️.rs`, `…/🪣️fill/🧫️fixtures/🔣️.json`, `…/🪣️fill/🧪️tests/🔬️unit/🦀️.rs`, fill types in `🧬️schema/🦀️.rs` + `🔣️.json` | `📓️wave-A-planner.md` |
| B1 session/job bridge | `E/⏳️precompute/🦀️.rs`, `E/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs`, `E/⏳️precompute/📐️geometry/**` (capacity → stall) | `📓️wave-B1-session.md` |
| B2 commands/config/editor | `E/🦀️.rs` (fill parts, Puzzle3dPrecomputeCommandWork, publication contracts, render body), `E/🎮️commands/{🧮️set-fill-count,🪣️fill-build-tick,📨️engagement-submit,🔂️engagement-repeat-last}/🦀️.rs`, `E/🎚️config/**`, `E/🪟️window/🦀️.rs`, `E/🧪️tests/**` fill assertions, `✏️s/🔌️plugins/🧩️puzzle/🧪️publication-authority/🔣️.json` if needed | `📓️wave-B2-commands.md` |
| C viewport/UI | `E/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` + `🟦️.ts`, `E/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`, `E/🗣️terminology/🦀️.rs`, `H/🌐️World3dHost/🟦️.tsx`, `H/🛠️ShellHelpers/🟦️.tsx` (reveal cutoff removal), renderer engine-contract test | `📓️wave-C-viewport.md` |
| D puzzle 2d/5d parity | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/**` and `🖐️5d/**` fill files | `📓️wave-D-2d-5d.md` |
| F framework measures | wgpu component enum + widgets, projection TS mirror, generated manifest, React Interpreter measure renderer, ui tests | `📓️wave-F-framework-measures.md` |
| G brush suggestions | `E/🎮️commands/{⏱️suggestions-tick,🔓️open-vortex-suggestions,🔒️close-vortex-suggestions}/🦀️.rs`, `E/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️.rs`, brush parts of `E/⏳️precompute/🖌️brush/🦀️.rs` | `📓️wave-G-brush.md` |
| E policy tests | root `📜️script.ts` `…Failures()` predicates, `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-*/🟦️.ts` | `📓️wave-E-policy.md` (after A/B/C) |
| H verification | probe step in `26/09/02/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts`, deploy chain, runtime evidence | `📓️wave-H-verification.md` (after all) |

## 4. Verification gates

1. `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4` → 0 errors (baseline: 0 errors, 97 warnings).
2. `cd ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust && bun ./📜️script.ts test -- fill` → all fill tests pass, including the new
   fixture-vs-serde-oracle test for the extended preview wire.
3. `bun nx run @semio-tech/framework-rs:generate` then `check` → manifest mirror in sync; ui wire-format tests pass.
4. `bun nx run workspace:verify-interactivity` → 0 blocking findings.
5. Deploy: `component-dev` → `support-dev` → `materialize-dev` → `prepare` → `activate-puzzle3d-react-dev`; served core.wasm sha256 == disk.
6. Browser: arm fill on the Concrete Forest example → within 30 s the instance count grows monotonically, the HUD shows
   `tested/locked/requested`, at least one `collision` ghost and one `free` ghost were observed (data attributes),
   typing 250 continues without restart, typing 40 deletes down to 40.
