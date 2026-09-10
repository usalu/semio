# 🧊️ wgpu retained-UI intake budget — 2026-09-10

Lane: wgpu retained-UI intake (`🐚️plugin-bridge.ts`, `🐚️Shell/🎯️targets/🧊️wgpu`). Ticket
`26/09/09/PROCEDURAL-3D-END-TO-END`.

Live finding this lane answers (wgpu boot #4, 11:05, `http://localhost:6118/?plugin=generation3d`,
hidden pane 1440×900):

```
shell-boot 86% — worker-boot-failed: shell-boot: renderDocument promise failed: wgpu-ui.intake-budget-exhausted
Worker terminated: yes
UI turns over budget 1/0 (worst 11 ms @ progress-hook)
Worker steps over budget 1/0 (worst 8.1 ms @ plugin-graph:order)
```

Two separable defects, both fixed here:

1. **The intake budget was a fixed constant.** `🐚️plugin-bridge.ts` capped every retained-UI intake
   drive at `WGPU_UI_CONTINUATION_LIMIT = 4_096` steps regardless of the patch's size.
2. **A `renderDocument` failure terminated the Worker.** `ShellState::refresh_ui` propagated a
   per-surface render error with `?`; that became `shell-boot: …`, then `worker-boot-failed`, which
   closed the Worker and took its transferred `OffscreenCanvas` with it.

## 1. Budget before

| site (`🐚️plugin-bridge.ts`) | guard before | fault raised |
|---|---|---|
| patch drive to acknowledgement | `step > 4_096` | `wgpu-ui.intake-budget-exhausted` |
| publication close | `step > 4_096` | `wgpu-ui.publication-close-budget-exhausted` |
| intake retirement | `step > 4_096` | `wgpu-ui.intake-close-budget-exhausted` |
| owner close | `step > 4_096` | `wgpu-ui.owner-close-budget-exhausted` |
| lifecycle close | `step > 4_096` | `wgpu-ui.lifecycle-close-budget-exhausted` |

Why 4 096 could never work: `OwnedUiPatchIntake.advance(grant)` advances **one phase per call** — a
LEB128 byte, a text body, an attach — so the grant (`{maxItems: 1, maxBytes: 4_096}`) never buys more
phases. The budget was therefore a cap on the patch's *wire length*, at roughly a few KiB.

Measured here (`🧪️tests/📥️wgpu-intake-budget`), a 300 KB retained patch built from 2 KiB text leaves
encodes to **323 328 wire bytes across 150 nodes** through the same `pack` encoder the guest publishes
with — 79× the retired fixed budget. generation3d's first document (flow window scene + preview +
catalogue pages + measures) is of exactly this order, which is what `shell-boot 86 %` was reporting.

React had the identical defect on 2026-09-09 and had already been fixed; the wgpu target simply never
picked the fix up.

## 2. Formula and shared law

The law is now declared **once, language-agnostically**, in the retained-UI contract's own intake
fixture — `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧫️fixtures/📥️intake/🔣️.json`:

```json
"budget": {
  "stepsPerNode": 8192,
  "sliceSteps": 4096,
  "ceilings": [
    { "maxNodes": 1,     "steps": 8192      },
    { "maxNodes": 145,   "steps": 1187840   },
    { "maxNodes": 20000, "steps": 163840000 }
  ]
}
```

with two new laws on the fixture's own `laws` list: `budget-scales-with-node-quota` and
`budget-slice-is-resumable`. The fixture's `$defs` entry (`IntakeFixture`, in
`🧵️retained/🧬️schema/🔣️.json`) was extended to require and constrain `budget`, so the numbers are
schema-checked, not merely present.

```
ceiling(limits) = limits.maxNodes × 8 192        // whole-document liveness backstop, terminal
slice           = 4 096 steps                     // per-frame drive slice, a YIELD, never terminal
```

**Why node-scaled and not byte-scaled.** The task framed React's fix as `base + 8 × wire bytes`. That
was React's *first* attempt and it faulted: the measurement recorded in
`📃️UiDocumentStore/📥️intake/🟦️.ts` shows a Nakagin-scale world-3d surface (145 nodes, 57 294 carried
bytes) costs **671 321 steps**, i.e. 4 630 per node — 18× what the byte-scaled budget credited and 1.3×
what its 64 KiB floor credited. Re-rooting a surface pays the authoritative whole-graph walk, whose
cost tracks the node count, not the byte count. So the working React law is node-scaled with ~1.8×
headroom, and mirroring "exactly like React's" means mirroring **that** law. It dominates the wire-byte
demand comfortably: the 300 KB patch above needs ≤ 323 328 phases against a 1 228 800-step credit for
its own 150 nodes, and 163 840 000 against the contract's own quota.

Twins:

| side | symbol | file |
|---|---|---|
| TypeScript | `RETAINED_UI_INTAKE_STEPS_PER_NODE`, `RETAINED_UI_INTAKE_SLICE_STEPS`, `retainedUiIntakeStepCeiling(limits)` | `📃️UiDocumentStore/📥️intake/🟦️.ts` |
| Rust | `RETAINED_UI_INTAKE_STEPS_PER_NODE`, `RETAINED_UI_INTAKE_SLICE_STEPS`, `UiDocumentLimits::retained_ui_intake_step_ceiling()` | `🖱️ui/🧬️contract/📦️packages/🦀️rust/🛡️limits.rs` |

Both sides carry a law test that reads the fixture and asserts every declared ceiling, so neither can
drift. The TS names lost their `plugin*` prefix (`pluginUiIntakeStepCeiling` →
`retainedUiIntakeStepCeiling`): the law is no longer React's, it is the retained contract's, and both
targets price through it.

## 3. Fix

### 3a. Proportional, resumable budget (`🐚️plugin-bridge.ts`)

`WGPU_UI_CONTINUATION_LIMIT` is deleted. All five drive sites now share one cursor:

```ts
export class WgpuUiIntakeCursor {
  #steps = 0;
  readonly #ceiling: number;
  constructor(ceiling: number = WGPU_UI_INTAKE_STEP_CEILING) { this.#ceiling = ceiling; }
  get steps(): number { return this.#steps; }
  async next(phase: string): Promise<void> {
    this.#steps += 1;
    if (this.#steps > this.#ceiling) throw new Error(`wgpu-ui.intake-budget-exhausted:${phase}:${this.#steps}`);
    if (this.#steps % RETAINED_UI_INTAKE_SLICE_STEPS === 0) await nextWgpuFrame();
    else await yieldWgpuUi(this.#steps);
  }
}
```

with `WGPU_UI_INTAKE_STEP_CEILING = retainedUiIntakeStepCeiling(DEFAULT_UI_DOCUMENT_LIMITS)`.

**Resumability.** Exhausting a slice is no longer a fault. The cursor is retained across the frame
boundary (`nextWgpuFrame()` — `requestAnimationFrame` where the target has one, which the worker's
`OffscreenCanvas` context does; a macrotask otherwise so headless tests resume too) and the *same*
intake continues from where it stopped. A document larger than one slice therefore publishes across
several frames instead of rejecting. Only the whole-document ceiling is terminal, and it now names the
phase and the crossing step (`wgpu-ui.intake-budget-exhausted:intake:163840001`) instead of a bare code.

A genuine stall is still caught, earlier and more precisely, by the intake's own rule: `advance` rejects
a phase that reports 32 consecutive steps carrying neither an item nor a byte. That byte-aware rule is
what the ceiling is explicitly *not* — a phase-name-only rule cannot work, since a legitimate Nakagin
first publication stays in `validation` for 163 284 consecutive steps.

The four close/retirement drives (`intake-close`, `publication-close`, `owner-close`,
`lifecycle-close`) share the same cursor and gained the same resumability for free.

### 3b. Per-surface `renderDocument` faults (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`)

New typed datum, mirroring the existing `ShellPluginFault`:

```rust
pub struct ShellSurfaceFault { pub surface_id: String, pub body_key: String, pub detail: String }
```

`refresh_ui`'s three `render_with_document(…).await?` / `render(…).await?` sites — window bodies, panel
tabs, spawned app — now `match` instead. A failure pushes `(surface_id, body_key, detail)` onto a local
list, the loop continues, and every other surface still publishes. `settle_surface_faults` then replaces
the live fault set with the one this refresh produced, so a surface that recovered stops showing a card
and a surface that faulted keeps the same card. `refresh_ui` itself returns `Ok(())`. The catalogue
fetch was already isolated this way; the other four were not.

The typed card renders in English and German with no default language, through the shell's existing
`shell_chrome_string` table:

```
("surface.faulted", false) => "Surface unavailable",
("surface.faulted", true)  => "Fläche nicht verfügbar",
```

`fault_status()` joins the per-plugin and per-surface statuses into the one status line the shell
paints, and `self.error` is refreshed from it. `boot()` therefore reaches `runtime-ready` with a fault
card on the failing surface, and **the Worker survives** — an unreadable surface can no longer take the
`OffscreenCanvas` with it.

## 4. Tests and tails

| gate | result |
|---|---|
| `cargo test -p semio-framework-ui-contract --lib retained_ui_intake_budget` | **1 passed** — the Rust twin carries every fixture-declared ceiling |
| `vitest run 📥️intake` (`SEMIO_TEST_LEVEL=long`) | **3 passed** — TS twin vs. fixture, ceiling shape, slice ≪ ceiling |
| `vitest run wgpu-intake-budget` (`SEMIO_TEST_LEVEL=long`) | **4 passed** (new suite) |
| `cargo test -p semio-framework-os-renderer-wgpu --lib shell_boot_isolation` | **5 passed** (2 new), 0 failed |
| Ajv validation of the extended fixture against `IntakeFixture` | **valid**, 11 laws |
| `cargo check -p semio-framework-os-renderer-wgpu --keep-going` (native) | **0 errors**, 18 crate warnings |
| `cargo check … --target wasm32-unknown-unknown --keep-going` | **0 errors**, 14 crate warnings |

New suite `🧑‍🎨engine/🧪️tests/📥️wgpu-intake-budget/🟦️.ts` (registered in the react package's
`vitest.config.ts` engine suites):

1. *prices a 300 KB retained patch under the proportional ceiling, where the retired fixed budget
   refused it* — builds the patch, encodes it through the real `pack` wire encoder, pins
   `{nodes: 150, wireBytes: 323_328}` and asserts `ceiling(nodes) = 1 228 800 > wireBytes` and
   `WGPU_UI_INTAKE_STEP_CEILING > wireBytes`, while the retired 4 096 does not admit it.
2. *carries the same numbers as the React target and the language-agnostic fixture*.
3. *resumes across the slice boundary with the retained cursor instead of faulting* — drives three whole
   slices and asserts the cursor kept counting rather than throwing.
4. *faults only past the whole-document ceiling, and names the phase and the step that crossed it*.

New Rust tests in `🐚️Shell/🧪️tests/🔬️wgpu-shell-boot-isolation/🦀️.rs`:

5. *a poisoned surface faults alone* — two poisoned surfaces produce two cards, no plugin fault, and a
   later refresh in which one recovered drops exactly that card; an empty refresh clears the status.
6. *surface fault status reads in both languages and never duplicates a surface* — en/de, cause verbatim,
   one card per surface, and the combined `fault_status()` carries plugin and surface faults together.

Both warning counts are load-bearing: a crate that aborts at module expansion reports zero errors *and*
zero warnings, so 18/14 crate-attributed warnings are the proof the wgpu crate actually type-checked.

### Peer state seen during this lane

The first native `cargo check` failed with **2 errors, both `E0061` in
`🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`** (`close_step()` called with 0 of 2 `usize` arguments) — the sibling
**poll-task-leak / reactor** lane mid-refactor, which blocks the wgpu crate natively because it depends
on the plugin host off-wasm. Re-run ~15 minutes later: clean. Kept as
`🗑️generated/intake-cargo-check-native-blocked-by-peer.txt`. The wasm gate was clean throughout and is
the one that actually compiles this lane's Rust.

Raw logs: `🗑️generated/intake-cargo-check-native.txt`,
`🗑️generated/intake-cargo-check-wasm32.txt`, `🗑️generated/intake-shell-isolation-tests.txt`,
`🗑️generated/intake-cargo-check-native-blocked-by-peer.txt`.

## 5. Bundle

The dist `.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu` has a **live `trunk serve` on 6118**
(pid 45306) as its single writer, so the bundle was rebuilt **through** that writer, never against it:
`generate-frame-worker` refreshed the generated input, and the running serve rebuilt and promoted. No
competing `trunk build` was ever started.

| artifact | before | after | proof |
|---|---|---|---|
| `🎞️frame-worker.js` | 1 130 598 B @ 11:15 | **1 131 119 B @ 11:26** | `WgpuUiIntakeCursor` ×5, `nextWgpuFrame` ×2, `requestAnimationFrame` ×1, `wgpu-ui.intake-budget-exhausted:${phase}:${this.#steps}`; retired `intake-close-budget-exhausted` ×**0** |
| `semio-framework-os-renderer-wgpu_bg.wasm` | 76 014 372 B @ 10:37 | **76 048 601 B @ 11:29** | string pool carries `Surface unavailable` immediately followed by `Fläche nicht verfügbar` — both halves of the new en/de fault card |
| `index.html`, `semio-framework-os-renderer-wgpu.js` | @ 10:37 | @ 11:21 | trunk-rebuilt |
| `🚀️boot.js` | 52 268 B | 52 268 B (unchanged) | this lane touched no `🚀️browser-boot` module, so it was deliberately not regenerated |
| served by 6118 | — | `GET /🎞️frame-worker.js` → **200, 1 131 119 B** (`WgpuUiIntakeCursor` ×5); `GET /semio-framework-os-renderer-wgpu_bg.wasm` → **200, 76 048 601 B**; `GET /🚀️boot.js` → **200, 52 268 B** | the running surface is serving this lane's bundle |

Generated inputs refreshed: `bun ./📜️script.ts generate-frame-worker` (in
`🎯️targets/🧊️wgpu/📦️packages/🦀️rust`). `Trunk.toml`'s `[watch]` already names `🟦️typescript`, so the
`🐚️plugin-bridge.ts` change reached trunk without a watch-list change this time.

**Not verified by this lane:** a live boot #5 in the browser. Browser tools were out of scope for this
lane, so the evidence above is bundle-level (served bytes carry the fix) plus the six unit tests, not a
rendered `runtime-ready`. The next boot run should see `shell-boot` pass 86 % and, if any surface still
fails to publish, a `Surface unavailable: … — …` card instead of `worker-boot-failed`.

## 6. Files changed

| file | change |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧫️fixtures/📥️intake/🔣️.json` | added the `budget` block (`stepsPerNode`, `sliceSteps`, three `ceilings`) and two laws |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json` | `IntakeFixture` requires and constrains `budget`; `laws` enum + count extended to 11 |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🛡️limits.rs` | Rust twin: `RETAINED_UI_INTAKE_STEPS_PER_NODE`, `RETAINED_UI_INTAKE_SLICE_STEPS`, `UiDocumentLimits::retained_ui_intake_step_ceiling` |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️limits-unit/🦀️.rs` | Rust law test against the shared fixture |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/📥️intake/🟦️.ts` | renamed the law to `retainedUiIntakeStepCeiling`/`RETAINED_UI_INTAKE_STEPS_PER_NODE`, added `RETAINED_UI_INTAKE_SLICE_STEPS`, added the fixture-pinned laws |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🧪️typedwire/🟦️.tsx` | follows the rename |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts` | deleted `WGPU_UI_CONTINUATION_LIMIT`; added `WGPU_UI_INTAKE_STEP_CEILING`, `nextWgpuFrame`, `WgpuUiIntakeCursor`; five drive sites made resumable |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | `ShellSurfaceFault`, `surface_faults`, `record_surface_fault`, `surface_fault_status`, `fault_status`, `settle_surface_faults`; `refresh_ui`'s three render sites isolated; `surface.faulted` en/de strings |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-boot-isolation/🦀️.rs` | two new per-surface fault tests |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📥️wgpu-intake-budget/🟦️.ts` | **new** suite (4 tests) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts` | registers the new engine suite |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🎞️frame-worker.js` | regenerated bundle input |
| `.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu/*` | promoted by the live `trunk serve` on 6118 |
| `$T/🐍️intake-fixture-schema-probe.mjs` | ticket-folder Ajv probe for the extended fixture |
