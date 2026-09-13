# React Example-Switch Regression — Resident Credit Exhaustion

Lane `react-example-switch-regression`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END, 2026-09-13 (into 09-14).

## TL;DR

The failure is **not** geometry and **not** the Nth switch in the sense of a counter: it is the **process-wide
resident BYTE ledger running out** while the resident SLOT ledger still has 38 of 64 slots free.

- A React generation3d editor session holds **26 concurrent reconcile reservations** and peaks at
  **33 483 491 bytes of a 33 554 432-byte aggregate — 99.79 %** (measured, §3).
- On the 4th example switch (`Sphere Cut With Torus`, the heaviest graph) the flow window asks to grow its
  credit to **2 517 394 B**; **5 549 B** are left. `admit_credit` refuses with
  `SurfaceReconcileFault::ResidentCredit` — a **terminal** fault by design.
- `window:procedural-main` is retired for good. Its published graph freezes at the Sphere Cut widget list and
  **fourteen consecutive later switches render the stale example** while the preview windows keep working
  perfectly (that is why the viewer role and generate mode look healthy).
- Root cause: `UI_RESIDENT_AGGREGATE_BYTES` was `4 * UI_RESIDENT_SURFACE_BYTES` — four times the per-surface
  **ceiling**, i.e. the imagined concurrency of a four-surface session — while `UI_RESIDENT_SLOTS` admits
  **64** holders at once. Wave B58 had already established that the per-surface ceiling is "a maximum a
  surface may reach, never a price"; deriving the **aggregate** from that ceiling repeats the same category
  error one level up.
- Fix: `UI_RESIDENT_AGGREGATE_BYTES = UI_RESIDENT_SLOTS * UI_RESIDENT_DOCUMENT_BYTES` (64 × 821 248 =
  **52 559 872 B**), where `UI_RESIDENT_DOCUMENT_BYTES = UI_DOCUMENT_NODES * size_of::<UiNodeRecord>()` is
  what one surface costs when its body fills the document contract. The two ledgers now refuse together.
- Native laws green. **Runtime re-verification is BLOCKED** by an unrelated peer break that wedges both vite
  dev servers — see §7. This is the one deliverable I could not complete.

## 1. Reproduction

`T/🐍️example-switch-regression-probe.mjs` (new). It varies only the **count** of switches: boot, then walk the
picker's own order round after round, recording per switch whether every preview settled **and** whether the
flow window republished the graph the picker names. `SEMIO_PROBE_DIAGNOSTICS` (default on) arms
`localStorage.SEMIO_RUNTIME_DIAGNOSTICS`, which makes the guest print `PatchTracker::debug_state()` every
more-work turn — the only live read of the resident ledger a browser session can get.

Run: `SEMIO_PROBE_ROUNDS=2 SEMIO_PROBE_BUDGET=18 bun 🐍️example-switch-regression-probe.mjs`
(`T/🗑️generated/example-switch/results.json`).

| # | switch | converged | flow-window graph |
|---|---|---|---|
| 0 | boot | ✅ 8 s | Hexagonal |
| 1 | No example | ✅ 2 s | (empty) |
| 2 | Hexagonal Mushroom Column | ✅ 3 s | Hexagonal |
| 3 | Rectangle Extrude Volume | ✅ 7 s | Rectangle Extrude |
| 4 | Sphere Cut With Torus | ✅ 6 s | Sphere Cut — **ResidentCredit fault raised here** |
| 5 | Box Fillet Preview | ❌ | Sphere Cut (stale) |
| 6…18 | every later switch | ❌ | Sphere Cut (stale), 14 in a row |

Step 13 (`r2:Sphere Cut With Torus`) reports `converged=true` only because the picker returned to the example
the window is frozen on — a false positive worth knowing about when reading the journey results.

The previews are **settled and correct** at every failing step (`meshes` tracks the picked example: 0, 3, 1,
1 …, `phase=idle`, `ratio=1`). Only the flow window is dead.

## 2. The fault, with file:line

```
37393 error typed-operation completion effects failed
      SemioFaultError: 1:procedural-main: ResidentCredit { required: 2517394, aggregate: 33548883 }
  at retainedUiRefreshEffects (…/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1775)
  at ownedUiRefreshResponse  (…/🔌️PluginRuntime/🟦️.tsx:1964)
```

- Raised at `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs:1882`
  (`SurfaceReconcileReconciler::admit_credit`, the `refusal` closure).
- Delivered to the host by `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs:724`
  (`take_render_fault`, `format!("{surface}: {fault:?}")`), which also sets `terminal.close = true`.
- Thrown host-side at `…/🔌️PluginRuntime/🟦️.tsx:1775` and swallowed by
  `…/🏛️ShellHost/🟦️.tsx:5764` (`.catch((error) => console.error("typed-operation completion effects failed", error))`).

The terminal behaviour is **deliberate**: `admit_credit`'s docstring records that a *busy* ledger
(`Contended`) is retried on the next step while a *full* one faults, precisely because wave B56's
refuse/defer/re-dirty loop (`more-work` streak 1 008 with `effects=0`) was worse. So the correct fix is to
stop the aggregate ever being full, not to make the refusal retryable.

## 3. Measurement — the aggregate is not leaking, it is undersized

Per-step **peak** of `registry=resident=<slots>s/<items>i/<bytes>B of <aggregate>B`
(`T/🗑️generated/example-switch/census/console.txt`, 2 374 samples):

| step | peak bytes | peak slots |
|---|---|---|
| boot | 28 490 398 | 26 |
| No example | 26 564 779 | 26 |
| Hexagonal | 30 533 220 | 26 |
| Rectangle Extrude | 31 492 239 | 26 |
| **Sphere Cut** | **33 483 491** | **26** ← 99.79 % of 33 554 432 |
| Box Fillet … Box Shell | ~7.6–8.7 M | 19 (flow window already dead) |

Credit is returned correctly — the settled reading falls back to 6–22 MB between steps — so there is **no
leak**. The session simply needs more than the budget at its peak, and the budget funds four ceiling-sized
surfaces against a slot ledger that admits sixty-four.

Sizing facts (printed by the new law):

```
aggregate=33554432B  fixed-backing=566352B  floor=132978B  open=1906B
flat=6520B  nodes=128  ceiling=8388608B  document=821248B  slots=64
```

The reconcile **floor** (132 978 B) is fine — 248 of them fit. The problem is exclusively the aggregate's
derivation.

## 4. Which change introduced it

The defect in `UI_RESIDENT_AGGREGATE_BYTES` is **old** — the 17:15 baseline run shows the same saturation
(`2:framework.panel.catalogue: ResidentCredit { required: 4905993, aggregate: 33470871 }`,
`T/🗑️generated/s4-journey-1/console.txt:577`). What changed between 18:20 green and 22:34 red is **which**
surface lost the race: at 17:15 it was a framework panel, whose lost refresh the journey oracle does not
read; at 22:34 it was `window:procedural-main`, which the oracle reads directly.

The session peak sat at 31.5 MB **before** the Sphere Cut step, i.e. a hair under the cliff. The lanes that
landed between 18:20 and 22:34 and grew the flow window's body pushed it over — the strongest candidates are
`📓️graph-keyboard-nav-appearance-boot` (five traversal verbs plus `Emit.interaction_writes` on every node)
and `📓️node-graph-wire-drag` (DagHost port geometry). **I did not bisect this** (see §8); what is proven is
the mechanism and that it is a cliff, not a new bug.

## 5. Fix

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🦀️.rs`:

```rust
pub const UI_RESIDENT_DOCUMENT_BYTES: usize = super::UI_DOCUMENT_NODES * size_of::<super::UiNodeRecord>();
pub const UI_RESIDENT_AGGREGATE_BYTES: usize = UI_RESIDENT_SLOTS * UI_RESIDENT_DOCUMENT_BYTES;
```

`32 MiB → 52 559 872 B` (50.1 MiB), **57 % above the measured peak**, and derived rather than magic: every
slot the ledger admits is funded at one full document. `UI_RESIDENT_SURFACE_BYTES` is unchanged — it remains
a per-surface maximum, not a price.

## 6. Laws

| law | where | counts |
|---|---|---|
| `resident_aggregate_admits_every_reconcile_slot` (new) | `🧠️runtime/🧪️tests/🔬️reconcile-unit/🦀️.rs` | fails at the old constant (34 of 64 slots fit), passes at the new one (**64 of 64**) |
| `retained_refresh_aggregate_admits_only_a_handful_of_ceiling_sized_surfaces` (renamed from `…only_three_…`) | `🧬️contract/🎟️resident/🔄️refresh/🧪️tests/🔄️refresh/🦀️.rs` | ceiling-sized roots 3 → **6**, plus a new assertion that this is `< UI_RESIDENT_SLOTS` |
| `retained_resident_permit_preserves_existing_capacity_and_paired_final_return` | `🧬️contract/🎟️resident/🧪️tests/🎟️resident/🦀️.rs` | fill generalised from a hard-coded 4 to `while remaining > 0`; asserts `slots * documentBytes == aggregate` |
| TS twin — resident arithmetic | `🧬️contract/🧪️tests/🔬️fixed-list-storage/🟦️.ts` | `surfaceBytes * 4 == aggregateBytes` → `slots * documentBytes == aggregateBytes`; pressure roots now a covering count, not a clean divisor |
| TS twin — surface ownership | `🧠️runtime/🧪️tests/🔬️surface-ownership/🟦️.ts` | `aggregate / ceiling == 4` → `aggregate / documentBytes == residentSlots`, plus `surfaces <= residentSlots`. **40 checks pass** |

Cross-language fixtures + schemas updated in lockstep (all six validate, verified with ajv):
`🎟️resident/{🧫️fixtures,🧬️schema}`, `🎟️resident/🗃️fixed/…`, `🎟️resident/🌳️root/…`,
`🎟️resident/🔄️refresh/…`, `🧠️runtime/📃️document/…`, `🧠️runtime/📤️output/…` (the `residentCapacity`
case table re-derived: capacities 4,3,3,2,0 → 6,6,5,5,0 with the boundary pair preserved).

Suite state (`--test-threads=1`):

- `semio-framework-ui-runtime --lib`: **121 passed, 2 failed**; both failures reproduce **identically at the
  old constant** (pre-existing, another lane's).
- `semio-framework-ui-contract --lib resident`: **7 passed, 5 failed**; the 5 are the pre-existing
  `resident_root_tests::*` set failing on `step.released_items <= 1 && step.released_bytes <= grant`, a
  retirement-grant law untouched by this work — identical at the old constant.
- Whole `ui-contract --lib`: pre-existing red. Measured both ways: **44 failures at the old constant, 18 with
  this fix** — the change strictly *reduces* the cascade (more headroom, fewer capacity poisonings). The
  cascade itself (one test poisons the shared arena for every later test) belongs to the suite-hygiene lanes.

## 7. Journey proof — NOT delivered, and why

`bunx nx run semio-framework-os-flow-core:wasm --skip-nx-cache` ✅ (1 m 33 s) and
`nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev` ✅ — the guest wasm restaged at
**00:49** (`semio_s_plugin_procedural_component.core.wasm`, 69 611 218 B). Two peer breaks had to be fixed
forward first (§9).

The re-run then failed at **boot, before any switch**:

```
386 pageerror TypeError: Cannot read properties of undefined (reading 'testingOpacity')
```

Diagnosis (airtight, read-only):

- `STYLING_METRICS.toolRun` is **undefined at runtime**, read by
  `…/🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx:23` (`TOOL_RUN_TRACE_METRICS = STYLING_METRICS.toolRun`).
- On **disk** it is present: `🎨️styling/🔣️.json:327` and `🎨️styling/🤖️generated/🔤️tokens/🟦️.ts:316`
  (both regenerated 23:49). The `toolRun` block is an **uncommitted peer addition** — 11 lines, absent from
  `HEAD` — from the INTERACTIVE-TOOLS-VISIBLE-PROCESS work.
- Both vite dev servers started **before** it: 6018 at **12:30**, 6118 at **12:16**. Both serve a transform of
  that exact `@fs` path containing **zero** occurrences of `toolRun`, while exporting the same const list — a
  stale in-memory transform of a module outside vite's watch root. `touch` does not invalidate it; neither
  does a `?t=` cache-bust request; there is no `node_modules/.vite` pre-bundle involved.

**Only a recycle of the vite processes clears this**, and the lane rules assign 6018 to the coordinator, so I
did not touch it. Until it is recycled **no lane can boot either React target**, so the coordinator's own
battery is blocked too, not just this verification. Recommended: kill pid **21603** (6018) and **7446** (6118)
and restart them, then re-run `T/🐍️journey-probe.mjs` and
`SEMIO_PROBE_ROUNDS=3 bun T/🐍️example-switch-regression-probe.mjs` (27 switches ≥ 3× the 8 bundled examples).

Per-step seconds for all 23 journey steps are therefore **not recorded** in this report.

## 8. What is NOT claimed

- **No journey proof.** The 23-step journey was not re-run after the fix; blocked as in §7. The only
  post-fix runtime evidence is that the guest wasm restaged successfully.
- **The endurance law is written but not executed against a fixed runtime.** The probe exists and is proven
  to reproduce the failure pre-fix; it has not been run post-fix.
- **The introducing change is named as a candidate, not bisected.** The aggregate defect predates 18:20 (§4);
  which lane's body growth crossed the cliff is inferred from the peak curve, not from a revert experiment.
- **Headroom is finite, not proven sufficient for all sessions.** 52 559 872 B against a measured peak of
  33 483 491 B is 1.57×. A heavier session (more panes, wider bodies) could still saturate, and a saturated
  aggregate is still a *terminal* fault that permanently kills the surface. I deliberately did not change the
  fault model — it is documented, intentional, and the alternative (retry) caused wave B56's livelock — but
  the "one refusal kills a surface for the session" property remains a real residual risk and deserves its own
  ticket.
- **The pre-existing ui-contract/ui-runtime reds are not fixed.** Measured as pre-existing at both constants;
  they belong to the suite lanes.
- I did not edit `📓️status.md` or `🎫️ticket.json`, and did not open/close/reopen the ticket.

## 9. Peer breaks fixed forward (minimal, noted as instructed)

1. **`Cargo.lock` out of sync.** A peer added the workspace member
   `🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust` to `Cargo.toml` without regenerating the lock, so
   every artifact build died on `cannot update the lock file … --locked was passed`. Regenerated offline with
   `cargo metadata --offline --no-deps` (+14 lines, `--locked` now passes).
2. **`semio-s-plugin-procedural` missing the `tool-run` dependency.** `dyn_enum_close!` copies
   `PluginApp::tool_run_trace_delta`'s signature verbatim into the generated impl in the *downstream* crate,
   so `semio_framework_tool_run` must resolve there (`E0433` at `✏️s/🔌️plugins/🌀️procedural/🦀️.rs:25`).
   Added `semio-framework-tool-run = { workspace = true }` to that crate's `[dependencies]`.
   `cargo check -p semio-s-plugin-procedural --target wasm32-wasip2` is clean. **Other plugin crates using the
   same macro will need the same one-liner** — that belongs to the tool-run lane.
3. Not fixed: the stale-vite wedge (§7) — coordinator's call.

## 10. Files

Changed:

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🦀️.rs` — the fix
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🧪️tests/🎟️resident/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🧫️fixtures/🔣️.json`, `…/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🗃️fixed/🧫️fixtures/🔣️.json`, `…/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🌳️root/🧫️fixtures/🔣️.json`, `…/🧬️schema/🔣️.json`, `…/🧪️tests/🌳️root/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🔄️refresh/🧫️fixtures/🔣️.json`, `…/🧬️schema/🔣️.json`, `…/🧪️tests/🔄️refresh/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️fixed-list-storage/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️reconcile-unit/🦀️.rs` — the new law
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️surface-ownership/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📃️document/🧫️fixtures/🔣️.json`, `…/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📤️output/🧫️fixtures/🔣️.json`, `…/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml` — peer fix-forward (§9.2)
- `Cargo.lock` — peer fix-forward (§9.1)

Created:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🐍️example-switch-regression-probe.mjs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/📓️react-example-switch-regression-2026-09-13.md` (this file)

Evidence under `T/🗑️generated/example-switch/`: `results.json`, `console.txt`, `divergence-5.png`,
`census/` (the ledger growth curve), `boot-check/` (the §7 pageerror), `restage-flow-core.txt`,
`restage-activate.txt`.
