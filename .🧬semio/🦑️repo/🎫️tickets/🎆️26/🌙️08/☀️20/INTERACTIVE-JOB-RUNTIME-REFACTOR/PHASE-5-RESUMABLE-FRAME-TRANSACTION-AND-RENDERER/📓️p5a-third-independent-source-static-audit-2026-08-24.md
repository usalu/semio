# P5a Third Independent Source/Static Audit — 2026-08-24

## Verdict

**RED — P5a is not source/static acceptable.** The isolated P5a text verifier passes, but live
production child bodies still perform complete, unbounded work in a single claimed opportunity.
The verifier does not faithfully cover that call graph. This audit was read-only; this report is its
only created artifact.

## Materials Read

- Root `AGENTS.md`; the Phase-5 portion of the governing interactive-runtime master plan; the P5a
  mounted-frame repair contract; the current P5a implementation report; all three coordinator P5a
  RED reports, especially `📓️coordinator-third-p5a-pre-acceptance-counterexamples-2026-08-24.md`.
- Final P5b and coordinator P5c source acceptances were retained as prior claims only and rechecked
  with their current isolated verifiers. See the preservation result below.
- Live mounted source, local Rust laws, and root `📜️script.ts` P5a verifier/mutations.

## Acceptance Blockers

### B1 — P5a non-text paint is still a complete hidden callee

`paint_node_step` treats every non-`Text` node as one bounded child after reserving 256 output
items, but then calls the legacy complete `paint_node_self` and immediately finishes the cursor.

- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️paint.rs:119-124`:
  `UiNode::Text` is the only retained path; the else arm calls `paint_node_self(...)`.
- `…/🦀️paint.rs:417-480`: that callee dispatches all non-text variants, including `Input`,
  `Select`, `KeyValue`, `Tree`, `Field`, `Section`, and `Group`.
- `…/🦀️paint.rs:644-674` scans every select item and allocates `row_children: Vec<NodeId>`;
  `…:698-706` scans every key/value entry; `…:857-878` traverses every tree section/item; and
  `…:892-965` recursively paints nested tree items.
- `…/🦀️paint.rs:535-549` and `…/🦀️widgets.rs:517-552` retain complete recursive/line and
  glyph loops. The 256-item reservation is a preflight check, not an output limiter: these paths
  can emit more than 256 draw items and grow/shape arbitrary text before return.

Thus a single large `Select`, `KeyValue`, `Tree`, field/input value, or nested component-derived
widget remains a complete hidden callee. It violates the third-RED requirement that **all** text,
node, scene, image, and component work advance one independently admitted scalar/item/page per
grant. The direct retained `Text` path does not cure its non-text siblings.

### B2 — Shell retained children still run unbounded text and allocation work

The claimed Shell child cursor is not a semantic bound for its text leaves.

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:10743-10759`
  invokes `chrome_text` once for each dialog title/body/label.
- `…/Shell/🧊️component.rs:6234-6240` allocates three fresh `HashMap`s and invokes complete
  `draw_text` per call.
- `…/🦀️widgets.rs:539-552` iterates every scalar and calls `ensure_glyph`/`push_glyph` without a
  byte, glyph, or output credit cursor.

Dialog and tour strings have no mounted P5a length or output cap. A multi-megabyte title/body is
therefore enough to cross the one-worker-opportunity ceiling even though the outer Shell cursor
has a phase scalar. This is a live Shell callee-body counterexample, not a test-only oracle.

### B3 — Shell I/O maintenance has no exact task refusal, generation, cancellation, or close contract

The implementation does put maintenance on `Lane::Io` on native, but it does not satisfy the
third-RED exact refusal/cancel/stale/close requirement.

- `…/📦️glue.rs:8350-8379` reserves a generic interaction future, removes the sole
  `AppInteractionState`, takes the cursor, and starts maintenance. No maintenance generation,
  cancellation token, freshness check, rejected task owner, or close cursor is carried.
- `…/📦️glue.rs:9117-9128` discards the result of
  `KernelPoolFuture::spawn(...)` (`let _ =`); a saturated/contended/shutdown submission has no
  exact handback or retry/close path despite the interaction/cursor already being removed.
- `…/Shell/🧊️component.rs:9821-9838` advances mutable maintenance without a step context or a
  stale/cancel guard. `…:9841-9950` has only phase scalars.
- The supposed bounded preference primitive remains a whole OS-config parse/serialize under a
  blocking global mutex: `…/Shell/🧊️component.rs:13150-13165` parses and serializes the full config,
  and `…:13175-13187` takes `std::sync::Mutex::lock`. The 4 KiB field filter at `:13191-13201`
  does not page that complete configuration work or make lock contention nonblocking.

The implementation report's statement that each I/O opportunity is exactly one bounded field/page
is therefore false in the production caller path. A typed retained maintenance job must own its
generation/cancel/freshness/rejection/close state, use a nonblocking submission result, and split
the backing/configuration work itself.

## Five-Family Census

| Third-RED family | Current audit result |
| --- | --- |
| 1. Fixed nonblocking generation-qualified Shell find ownership | **Partial, not sufficient for GREEN.** `ShellFindItems` itself is fixed and uses a thread-local non-nesting binding (`Shell/🧊️component.rs:133-263`); no `Arc<Mutex<Vec<_>>` or whole `take` remained in that authority. The required whole mounted frame still fails B1/B2/B3. |
| 2. Shared-I/O Shell preference/introduction/layout/presence maintenance | **RED (B3).** It is nominally submitted on `Lane::Io`, but lacks exact submission refusal/handback, generation, cancellation, stale rejection and close; whole config parse/serialize remains. |
| 3. Bounded text/node/scene/image/component work | **RED (B1/B2).** Text-only and scene/image step cursors exist, but live non-text paint and Shell text immediately enter complete loops/callees. |
| 4. Atlas item/page/payload/backing ledger and abandonment drain | **Static source evidence acceptable but unexecuted.** `PreparedAtlasPages::try_new` reserves the atomic four-dimensional permit before slot/page allocation (`…/🦀️prepared.rs:321-345`); close and abandonment progress one owner/scalar (`:390-450`), and the mounted deferred phase drains one abandoned unit (`renderer …/📦️glue.rs:11319-11323`). No contrary source blocker was found in this family. |
| 5. Faithful live verifier mutations | **RED.** The verifier is a text-presence checker and does not reject B1/B2; several claimed mutations are syntactically/type-invalid or fail for unrelated global text removal rather than a semantic counterexample. |

## Verifier-Fidelity Findings

`interactivityMountedFrameTransactionSelfTests` passed on the present tree, but that result is
not an acceptance proof.

- `📜️script.ts:8501-8518` only requires text-cursor tokens and bans a few spellings. It explicitly
  permits `paint_node_self`; it neither inspects its variant bodies for unbounded lists/recursion nor
  mutates the non-text mounted call. That omission admits B1.
- `📜️script.ts:8460-8472` checks that Shell child function names exist and bans selected legacy
  names; it does not trace `chrome_text` to `draw_text` or require byte/glyph/output admission.
  That omission admits B2.
- `📜️script.ts:8709-8714` calls synchronous-maintenance and lane mutations faithful, but
  `maintenance-on-interactive-lane` uses `replaceAll` across the complete glue source. It fails by
  deleting every `Lane::Io` occurrence, including unrelated I/O paths, not by proving the
  maintenance submission itself stays on I/O.
- `📜️script.ts:8686` injects a complete `render_main_window(...)` call that is not a valid live
  production substitution; `:8699` replaces a `char` expression with a `String` expression later
  used as `len_utf8`, and `:8707` renames `pop_front` without repairing its callers. These failures
  can be caused by invalid source/required-token loss, not preserved executable semantics.
- No mutation restores the actual B1 `paint_node_self` non-text dispatch, the B2 `chrome_text` /
  `draw_text` whole-string path, or B3's ignored `KernelPoolFuture::spawn` result and absent
  maintenance generation/cancel/close state.

The reported “69 / 69 rejected” is consequently a false claim of *faithful* mutation coverage.

## Preserved P5b/P5c Check

The accepted P5b/P5c reports cannot be treated as preserved on the current live tree:

- Direct current P5b verifier invocation failed at `📜️script.ts:8084` because its baseline reports
  missing fixed generation-qualified `UiValue`/semantic cursor evidence and a missing mounted
  reactor reconcile opportunity. Current source also declares
  `SURFACE_RECONCILE_PAGE_BYTES: usize = 32 * 1_024` in
  `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1752`, whereas the
  accepted P5b verifier expects `16 * 1_024`.
- Direct current P5c verifier invocation failed at `📜️script.ts:8241`: mutation
  `paint-live-layout` (`:8230`) was falsely accepted. Therefore its present mutation suite is not
  preserved either.

These are source/static verifier regressions observed after the final P5b/P5c acceptance reports;
this audit does not attribute their cause or modify either packet.

## Evidence Run and Deferred Gates

| Check | Result |
| --- | --- |
| Isolated `interactivityMountedFrameTransactionSelfTests` | Passed (`P5a isolated verifier passed`), but rejected as insufficient for the reasons above. |
| Isolated `interactivityLiveReconcileSelfTests` | Failed baseline as recorded above. |
| Isolated `interactivityMountedLayoutTextSelfTests` | Failed: `paint-live-layout` mutation falsely accepted. |
| Cargo, Nx, Wasm, browser, native, allocation/timing, and broad gates | Not run, per audit scope. |

Required next remediation is to replace each complete non-text/Shell text call with retained
byte/item/node cursors and a hard output budget, and to make maintenance a generation-qualified,
fallible, cancellation-aware I/O job with incremental configuration pages and close handback. The
permanent verifier must then make valid, local semantic mutations in those live callee bodies and
execute them against the same production slices.
