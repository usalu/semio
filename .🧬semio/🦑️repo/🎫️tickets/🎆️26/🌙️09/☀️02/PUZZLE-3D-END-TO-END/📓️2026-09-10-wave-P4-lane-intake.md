# Wave P4 — the paged scene-lane intake: what exists, and why the perspective refresh stalled

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-P4 (continuation of W-P / W-P2 / W-P3), 2026-09-10.
Written incrementally while the wave ran. Sibling report: `📓️2026-09-09-wave-P-paged-scene-payload.md`
(W-P/W-P2; its §3 claims are audited below because W-P3 found at least one that the tree did not carry).

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached HEAD, live tree shared with W-F4 (fill job) and W-H3
  (brush-mesh upload). No `git commit` / `stash` / `checkout` was run; the ticket is NOT closed.
- The repo MCP server did not connect this session (`repo (-32602): invalid initialize params`), so the
  ticket folder is managed on disk.
- Private cargo target `…/scratchpad/target-p3d-p`, `RUSTC_WRAPPER=""`, `CARGO_INCREMENTAL=0`.
- Machine load average ≈ 22; every cargo/vitest run was backgrounded to a scratchpad log.

## 1 Instrument caveat — `grep` needs `-a` in this tree

`🔌️PluginRuntime/🟦️.tsx` (and other emoji-bearing, very-long-line sources) are classified as BINARY by
BSD grep, so a plain `grep -rn` over the repo returns **nothing** for symbols that are present. Measured:

```
grep -c "surface" 🔌️PluginRuntime/🟦️.tsx   → no output, rc=1     (file has 100 matches)
grep -a -c "surface" 🔌️PluginRuntime/🟦️.tsx → 100
```

Every negative grep in this ticket that was run without `-a` is therefore untrustworthy. W-P3's finding
"`PluginRuntime` is missing from the `pluginUiIntakeBudget` list" is an instance of exactly this: the
call site exists, it is simply invisible without `-a` (see §2).

## 2 Audit of W-P/W-P2's §3 claims against the tree

Verified by reading the files (`git diff HEAD` shows nothing for most of them because the repo
auto-commits; the relevant squash is `6ad7b0e7bc` 🚩️607, 2026-09-10 01:31, i.e. AFTER the 00:30 runtime
observation this wave inherited).

| Claim | State in tree |
| --- | --- |
| `SceneDoc::split_lanes` / `World3dSceneLane` / lane hash | present, `🖱️ui/🎬️scene/📦️packages/🦀️rust/🎬️scenes.rs:60,262,339,462-698` |
| `scene_surface` hangs one `paged_text_carrier` per lane off the surface node | present, `💻️os/🔨️modules/🔌️plugin/🦀️.rs:406-468` |
| language-neutral contract fixture | present, `🖱️ui/🎬️scene/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json` (`leafBytes` 512, `childrenMax` 32, `docBytesMax` 32768, `measuredFaultBytes` 57281, 18 lanes) |
| Rust `scenes-unit` Nakagin law | present, `🖱️ui/🎬️scene/🧪️tests/🔬️scenes-unit/🦀️.rs:50-223` |
| mesh TS twins `WORLD3D_SCENE_LANES` / `world3dSceneFromLanes` | present, `🧰️framework/🔨️modules/🔺️mesh/🟦️.ts:339,373` |
| Interpreter `PagedSurfaceView` + `surfaceSceneLaneText` + lane cache | present, `🗣️Interpreter/🟦️.tsx:419,507,521,1262,1278` |
| Interpreter `surface-scene-lanes` suite | present, 204 lines, registered from `🗣️Interpreter/🟦️.tsx:1398` |
| `pluginUiIntakeBudget` + zero-progress reject + inline vitest | present, `📃️UiDocumentStore/📥️intake/🟦️.ts:88-166` |
| `PluginRuntime` uses the shared budget | present — `retainedUiPatchIntakeBudget` (`🔌️PluginRuntime/🟦️.tsx:1196`) is now a one-line delegate to `pluginUiIntakeBudget`, used at `:1471`; W-P3 could not see it because of §1 |
| retained `surface` staging no longer spins | present — the staging scan in `🧵️retained/🖼️surface/🟦️.ts:373-390` always advances `#scan` before the `continue`, so it is bounded by the cell count |

So the wave's Rust half AND the TS budget/staging half are in the tree. What is NOT verified anywhere is
that a real Nakagin-scale paged lane patch actually reaches an acknowledgement through
`OwnedUiPatchIntake` — the intake vitest W-P2 added only asserts an arithmetic property of the budget
function, never drives the intake. That is the gap this wave closes.

## 3 The observed stall, re-derived

Runtime evidence inherited (served #26, live TS, 2026-09-10 00:30):
`plugin-ui.intake-budget-exhausted:1:puzzle3d-main-perspective:36864`, 36 864 `advance` steps in 127 ms.

`36864 = 4096 + 4096 × 8`, i.e. `max(measuredBytes, ops × 4096) === 4096` — a patch of **one** op whose
measured payload was ≤ 4 KiB. Two facts bound the interpretation:

- `scene_surface` emits ONE surface node with 18 lane-carrier children, each a 32-ary page tree of
  512-byte text leaves. At Nakagin scale that is ≈ 145 retained nodes, and the retained wire encodes one
  `upsert` op per node. A patch of one op is therefore NOT the whole scene tree — it is a delta
  (the guest slot sat at `ack1/rev2`).
- so the 36 864 steps were spent on a patch the budget formula priced at a few kilobytes, and the
  intake still did not reach `ack`.

Both candidate causes (budget under-pricing vs. a protocol stall) predict the same symptom, and the
64 KiB floor that landed in 🚩️607 changes the failure from "dies at 36 864" to "dies at 528 384" if the
cause is a stall. The decisive instrument is therefore a law that drives the real intake with a real
Nakagin-scale paged lane patch and counts the steps it actually needs.

## 4 The measurement — and the actual root cause

New law `OwnedIntake drives a Nakagin-scale paged scene-lane surface patch to acknowledgement inside the
credited budget` (`📃️UiDocumentStore/🧪️tests/🧪️typedwire/🟦️.tsx`). It builds the exact tree
`scene_surface` emits — a surface node with one `paged_text_carrier` per declared lane, 512-byte text
leaves under a 32-ary page tree, sized from the contract fixture's own `measuredFaultBytes` — pushes it
through a real `ShardClient` UI-patch authority, and drives `OwnedUiPatchIntake` with the renderer's own
grant `{maxItems: 256, maxBytes: 65 536}`. Measured (145 nodes, 57 294 carried bytes, 18 lanes):

```
steps=669403  samePhase=163284  idle=4  nodes=145  carried=57294
```

- **The intake never stalls.** The longest run of steps carrying neither an item nor a byte is **4** —
  nowhere near the 32 at which `OwnedUiPatchIntake` itself rejects. It reaches the acknowledgement token,
  accepts the receipt, publishes revision 1 and hands back the surface. Lane paging does NOT deadlock on
  a page the guest only sends after an acknowledgement, and the TS half handles the lane page kind.
- **The cost is the DOCUMENT, not the delta.** 669 403 steps = 4 617 per node. A patch's `advance` loop
  runs `OwnedUiSurfacePatch.#prepare`, which revalidates the whole graph, rehashes it, restages every
  read cell and renotifies every subscriber. A one-operation delta pays all of it.
- Therefore **both** host budgets were priced off the wrong quantity:
  - the original `4096 + max(bytes, ops × 4096) × 8` gave a one-op delta **36 864** steps → the observed
    `plugin-ui.intake-budget-exhausted:1:puzzle3d-main-perspective:36864`;
  - W-P2's 64 KiB floor raised that to **528 384** — still 21 % short of the 669 403 the same document
    needs, so the identical fault would have returned with a bigger number.
- **The 4096 same-phase guard W-P2 added to `acceptUiPatches` is a false-positive detector.** A
  legitimate Nakagin publication stays in the single phase name `validation` for **163 284** consecutive
  steps — 40× the guard. Left in place it would have converted the budget fault into
  `plugin-ui.intake-zero-progress:puzzle3d-main-perspective:validation`, i.e. the same dead refresh under
  a new name. A phase-name-only rule cannot express progress; the byte/item-aware rule already inside
  `OwnedUiPatchIntake.#step` can, and does.
- **Yield cadence.** `yieldUi` yielded a macrotask every `PLUGIN_UI_CONTINUATION_BATCH_SIZE` = 8 steps.
  That constant is the cadence for guest TURNS (one step = one whole turn); an intake step is one wire
  phase of ~3.4 µs (36 864 steps in 127 ms, measured in browser #26). 669 403 steps at a stride of 8 is
  **83 675 macrotask round-trips** for one refresh — the scheduling dwarfs the work.

## 5 Changes

- `📃️UiDocumentStore/📥️intake/🟦️.ts` — deleted `measureRetainedUiPatchBytes` and `pluginUiIntakeBudget`
  (a delta-scaled budget cannot bound a document-scaled publication, and the deep ops walk it needed ran
  on every patch). Added `PLUGIN_UI_INTAKE_STEPS_PER_NODE = 8192` (the measured 4 617 with 2× headroom)
  and `pluginUiIntakeStepCeiling(limits)` = `limits.maxNodes × PLUGIN_UI_INTAKE_STEPS_PER_NODE`, plus an
  in-source law pinning the ceiling above the measured 669 403. Both docstrings state that the ceiling is
  a liveness BACKSTOP and that the progress guarantee is the intake's own byte-aware zero-progress reject.
- `🔌️PluginRuntime/🟦️.tsx` — removed the local `retainedUiPatchIntakeBudget` alias; both intake loops now
  bound on the module-level `PLUGIN_UI_INTAKE_STEP_CEILING = pluginUiIntakeStepCeiling(DEFAULT_UI_DOCUMENT_LIMITS)`
  (163 840 000). Removed the 4096 same-phase `intake-zero-progress` throw. Added
  `PLUGIN_UI_INTAKE_YIELD_STRIDE = 1024` (a ~3.5 ms slice) for `yieldUi`, leaving the TURN continuation
  cadence `PLUGIN_UI_CONTINUATION_BATCH_SIZE` untouched — the two loops count different things.
- `📃️UiDocumentStore/🧪️tests/🧪️typedwire/🟦️.tsx` — the new law above.

## 6 Verification

All runs backgrounded to `…/scratchpad/wp4-*.txt`.

| Command | Result |
| --- | --- |
| `bun ./📜️script.ts typecheck` (react target) | **820** `error TS`, **0** in `🔌️PluginRuntime`, `📥️intake` or `🧪️typedwire` (the two Interpreter hits are pre-existing peer errors) |
| `test exhaustive --run UiDocumentStore/🟦️.tsx --testNamePattern='Nakagin-scale paged scene-lane'` | **1 passed / 0 failed** |
| — same law, before the ceiling fix | **1 failed**: `expected 669403 to be less than or equal to 528384` |
| — same law, with W-P2's phase-name guard | **1 failed**: `validation made no phase progress: expected 4097 to be less than or equal to 4096` |
| `test long --run 📥️intake/🟦️.ts 🗣️Interpreter/🟦️.tsx` | **2 files, 74 passed / 0 failed** |
| `test long --run 🔌️PluginRuntime/🟦️.tsx 🧪️tests/🔬️engine-contract/🟦️.ts` | **2 files, 551 passed / 0 failed** |
| `cargo test -p semio-framework-ui-scene -j 4 -- --test-threads=2` | **116 passed / 0 failed**, including all seven `world3d_scene_*` lane laws and `a_nakagin_scale_world3d_scene_pages_per_lane_and_reassembles_losslessly` |

No Rust source was edited by this wave, so no wasm build and no
`cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` was needed. The two puzzle-3d editor
compile errors handed over at 01:04 (`E0252` duplicate `HashMap`, `?` on `ui_value_text`) are both gone
from the tree — a peer fixed the first and the second was never an error (`ui_value_text` returns
`UiAssemblyResult`, only `ui_value_number` returns a bare `UiValue`, and the call site never `?`s it).

## 7 Not verified / open

- **No browser re-drive.** The fix is TypeScript only; it rides vite HMR or the next serve. Confirming
  the Nakagin switch actually paints needs a browser run this wave did not do (and must not: no servers).
- **`did not publish its requested UI surfaces within 4096 continuations`** was a consequence of
  `acceptUiPatches` throwing before the surface could be retained — with the intake landing, the drain
  should settle. Unconfirmed against a live guest.
- **Interactivity cost stands.** 669 403 intake steps at ~3.4 µs is ≈2.3 s of renderer work for ONE
  Nakagin world refresh, before any React render. The yield stride keeps the tab responsive across it,
  but does not make it cheaper. The real reduction is per-node publication cost
  (`OwnedUiSurfacePatch.#prepare` revalidates and rehashes the entire graph for a one-node delta) — a
  separate wave, and the natural next one for this document size.
- **The step ceiling is a backstop, not a work bound.** At the contract's 20 000-node quota it credits
  163 840 000 steps; a runaway that keeps reporting bytes would take minutes to fail. The intake's own
  32-step byte-aware zero-progress rule is the mechanism that actually catches a stall.
