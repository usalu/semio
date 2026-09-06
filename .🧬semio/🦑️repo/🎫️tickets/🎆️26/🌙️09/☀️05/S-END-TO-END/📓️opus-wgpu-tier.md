# Lane L — `wgpu-tier` (Wave 3): `🛠️dev🖥️s🧊️wgpu🌐️wasm` boot + `parity verify s`

Opus 5 implementer, 2026-09-06 05:20 → (in progress). Spec: `📓️explore-wgpu-tier-readiness.md` (8-step
checklist), `📓️explore-wgpu-and-native-shells.md`, plan entry 05:20 (`PackInteger` carriers).
Private cargo target `target-s-e2e-wgpu`, `RUSTC_WRAPPER=""`. Logs: `🗑️generated/lane-l/`.

## 1. Cheap RED gates (checklist steps 1-3)

### 1.1 `check-frame-worker` — was RED, now green (and re-goes-RED on every OS-TS edit)

Lane K's report said the deployment-catalog rename made it green again; **it was RED again at 05:24**:

```
$ bun ./📜️script.ts check-frame-worker
error: 🎞️frame-worker.js is stale; run the generate-frame-worker target
    at checkFrameWorker (…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:238:105)
```

`bun ./📜️script.ts generate-frame-worker` rewrote
`…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🎞️frame-worker.js` (+22/-3 lines vs HEAD) and
`check-frame-worker` then reported `🎞️frame-worker.js is fresh`.

**Structural finding for the coordinator**: the artifact is a committed `Bun.build` of
`🎞️frame-worker/🟦️.ts`, whose import graph includes `🧰️framework/🛍️products/💻️os/🟦️.ts`. Any edit to
that file (including this lane's own §2 edit) makes the gate RED again until `generate-frame-worker`
re-runs — it went stale a second time inside this session for exactly that reason and was regenerated
again. Every lane that touches the OS TS package must re-run `generate-frame-worker`, or the wgpu
`serve`/`dev`/`parity verify` path aborts before `trunk serve` (`TrunkServeScript.run`, `📜️script.ts:262`).

### 1.2 `lint` — 3 raw colour literals → theme tokens

`🧊️renderer/🦀️.rs:15240-15242` constructed `ui_wgpu::wgpu::Rgba::from_srgb8(…)` inline for the
`JobProgressKind` overlay. The `ui_wgpu::Theme` had only `error` as a semantic outcome colour, so two
tokens were added rather than relocating the literals:

- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🎨️theme.rs:131-136` — new
  `Theme::success` / `Theme::progress` fields (docstringed, beside the existing `error`).
- `…/🎨️theme.rs:242-243` — `success: Rgba::from_srgb8(36, 158, 91, 255)`,
  `progress: Rgba::from_srgb8(67, 132, 245, 255)` in `from_chrome`, the module the lint rule scopes
  colour construction to.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:15239-15244`
  — the match arms now read `theme.success` / `theme.error` / `theme.progress`, with the theme taken
  from `self.interaction` (falling back to `Theme::default()` when no interaction state exists).

```
$ bun ./📜️script.ts lint
framework-renderer-wgpu: color-literal lint passed
```

Note the fault colour is now the theme's `error` (`Rgba::new(0.95, 0.35, 0.35, 1.0)`), not the former
inline `from_srgb8(218, 74, 74, 255)` — a deliberate token adoption, not a regression.

### 1.3 `test-preview-generated` — was RED twice over

Two independent failures, both fixed:

**(a) real test failure, not a budget artifact.** Running the suite outside the budget showed
**2 of 12 cases failing**:

```
FAIL … > activates only the current no-follow package through both independent TypeScript compilers
AssertionError: current-catalog-and-package: expected false to be true
FAIL … > validates the current package catalog with Ajv and independent WebCrypto integrity vectors
AssertionError: reviewed-catalog-bytes: expected false to be true
```

Cause: `🔣️taxonomy.json`'s `generatorContracts["wgpu-frame-worker"].packageGeneration.catalogSha256`
was `72823763f2bcf38a…`, while
`…/🎯️targets/🧊️wgpu/🪪️package-catalog.json` hashes to `fe9cc288e4f6411a…` — both the working tree and
the file's only commit (`fe7c8a8f8b`, 2026-09-05 03:53, which introduced both). The pinned digest
matches no committed version of the catalog; the catalog's own four `artifacts[].content` blocks were
verified byte-identical to the four files on disk, so the catalog is the authority and the pin was
stale. Fixed at
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:16820`.

**(b) level.** Even green the suite exceeds the 15 s `fundamental` budget: `import 13.88 s`
(the TypeScript compiler, Ajv, emoji-regex and `loadTaxonomy` — its four independent oracles) before
any case runs, plus two full browser-bundle renders; 18 s idle, 33 s under load ~60. Levelled to
`long` rather than trimmed, by giving suites a way to declare a floor:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1154-1169` —
  `resolveTestLevel(segments, minimum: TestLevel = "fundamental")`; the resolved level is
  `max(requested, minimum)`, so an explicit `test-preview-generated exhaustive` still wins and every
  existing call site is unchanged.
- `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:567-575` — `PreviewGeneratedTestScript` passes
  `"long"`.

```
$ bun ./📜️script.ts test-preview-generated
 Test Files  1 passed (1)
      Tests  15 passed (15)
   Duration  36.90s
```

(12 pre-existing + 3 new from §2.)

### 1.4 Stale ship path at dev `📜️script.ts:1373`

Confirmed: `BuildScript` resolved `…/🧑‍🎨engine/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts` — no `🎯️targets/`
segment, a path that does not exist. Three call sites carried the same literal (two correct, one not),
so they now share one constant instead of a fourth chance to drift:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:84-87` — new
  `WGPU_PACKAGE_ROOT` / `WGPU_SCRIPT_PATH`.
- `…:1325-1326` (dev `serve`), `…:1378` (ship `wasm --release`, the bug), `…:5094` (bench `native`) all
  route through them; the ship path additionally gained the correct `cwd`.

`bun build --target=bun` of the dev script: `Bundled 941 modules in 2932ms`.

## 2. `PackInteger` carriers on the wgpu wire (plan entry 05:20)

The 03:51 commit made `decodePackValue` return lossless carriers. Proof of the two shapes:

```
$ bun -e 'import {encodePackValue,decodePackValue,decodePackWire,packUInt} from "./🟦️.ts"; …'
[DEBUG] raw decodePackValue: {"children":[{"kind":"uint","value":"9n"}],"id":{"kind":"uint","value":"7n"}}
[DEBUG] projected decodePackWire: {"children":[9],"id":7}
```

The projection now lives once, beside the codec it projects, instead of a second copy per renderer:

- `🧰️framework/🛍️products/💻️os/🟦️.ts:1538-1563` — new exported `describePackWireValue`,
  `packWireNatural(raw, field)` (WIT `bigint` / `PackInteger` carrier / absent→0, everything else a
  named fault) and `decodePackWire(bytes, path)` (`packValueToExactJson ∘ decodePackValue`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:46,539-548`
  — the React shell's own `wireNatural`/`decodeWirePack`/`describeWireValue` bodies (added at 05:20)
  are now the shared implementations; all ~15 call sites unchanged.
- `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts:71` — imports `decodePackWire`
  and `packWireNatural`.
- `…/🐚️plugin-bridge.ts:127-134` — new exported `reconcileRetainedWindowPatch(previous, patch)`:
  `decodeWirePatchOps(patch.ops, decodePackWire)` (was `decodePackValue`) and both revisions through
  `packWireNatural` (was `patch.revision ?? 0` — a WIT `u64` `bigint` there fails
  `applyUiPatchToRetained`'s `baseRevision !== previous.revision` identity check forever, i.e. a
  permanent silent desync, the same class as the React fault).
- `…/🐚️plugin-bridge.ts:172-184` — new exported `decodeInvocationPayloads(frame)` replaces the four raw
  decodes the spec cited at `:171-175` (`output`, `diagnostics`, `ui_scope`, `history_patch`).
- `…/🐚️plugin-bridge.ts:198` — `wireEffectToFriendly(effect, decodePackWire)` (was `decodePackValue`),
  so `openWindow.params` and every other packed effect field is projected too.

Fault decoding (`decodeFaultFromWire`/`faultDisplayMessage`) deliberately keeps raw `decodePackValue`,
matching `PluginRuntime`'s own treatment.

Tests — `…/🎯️targets/🧊️wgpu/🧪️tests/🧩️package-integration.ts`, new describe block
`framework renderer wgpu pack integer carriers` (3 cases, run by the registered `test-preview-generated`
target):
1. carriers + `bigint` revisions off the retained-window boundary land as exact JSON and a second patch
   with `baseRevision: 1n` reconciles instead of desyncing;
2. `packUInt(2n ** 60n)` throws naming `$.id`, and a `2n ** 60n` revision throws naming
   `uiPatch.revision` — a rejection, never a rounding;
3. all four `Invocation` payloads compared against an independent `JSON.parse` oracle, plus
   `JSON.stringify(output)` must not contain `"kind"`.

---

# Lane L2 (Opus 5, 2026-09-06 11:10 → 12:35) — continuation

Lane L was cut by the fifth Opus limit after §1-2. L2 re-verified those on disk, then ran checklist
steps 4-8. **Headline: the boot is not reachable. `semio-framework-os-renderer-wgpu` does not compile
on *either* target — it is not a wasm-only or a peer-mid-edit problem, it is committed drift.**

## 2b. Re-verification of lane L's work (all green, 12:27, idle machine)

| Gate | Result |
|---|---|
| `check-frame-worker` | RED at 11:20 again (peer edit to `💻️os/🟦️.ts`), `generate-frame-worker` → **fresh**. Confirms §1.1's structural finding a third time. |
| `lint` | `color-literal lint passed` |
| `check-browser-worker` | EXIT 0 |
| `test-browser-worker` | **32/32**, 328 ms |
| `test-preview-generated` | **15/15**, 3.98 s (lane L's 3 `PackInteger` carrier cases included) |
| dev `📜️script.ts` bundle | `bun build --target=bun` clean |

## 3. `wasm32-unknown-unknown` compile check (checklist step 4) — the wave's real blocker

All runs: `RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-s-e2e-wgpu cargo check
-p semio-framework-os-renderer-wgpu [--target wasm32-unknown-unknown] --keep-going`. Full logs in
`🗑️generated/lane-l/wgpu-check-{wasm32,native}{,-4}.txt`.

### 3.1 Measured progression

| Run | Target | Errors | What it proved |
|---|---|---|---|
| 1 | wasm32-unknown-unknown | **5** | Aborted inside `semio-s-plugin-puzzle`; the renderer's own files were never reached. |
| 2 | wasm32-unknown-unknown | **5** (in the renderer crate) | Puzzle clean; the renderer crate's *manifest* was wrong. |
| 3 | wasm32-unknown-unknown | **71** | First time this crate's own browser code was ever type-checked. |
| 3n | **native** | **123** | **Decisive: not a wasm problem.** More errors natively than on wasm. |
| 4n | native, after the shared-class fixes | 101 → **26** → 74 (see 3.5) | |
| 4 | wasm32-unknown-unknown, final | **49** | |

Attribution: `git status` is clean for every file involved except lane L's own `🎨️theme.rs`; the Shell
file's last write is 2026-09-05 19:19 and committed (`b0dfa0f09b`). **This is committed-broken HEAD,
not a peer mid-edit** — nobody has compiled this crate through the drift of the last weeks, exactly as
`📓️explore-wgpu-tier-readiness.md` §3 inferred from the empty dist cache.

### 3.2 Fixed: `semio-s-plugin-puzzle` never compiled for the browser target (5 errors)

Puzzle is a path dependency of the renderer, so its `#[cfg(all(target_arch = "wasm32", not(target_env
= "p2")))]` bridges only ever compile on this target — and had rotted through two upstream changes:

- `Puzzle{2,3,5}dSnapshot: serde::Serialize` unsatisfied (`serde` is `#[cfg(test)]`-only since ticket
  `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-…`; the documented production route is `dsl::ToValue`):
  - `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:79`
  - `…/🗿️artifacts/🧊️3d/…/✏️editor/🌉️wasm/🦀️.rs:137`
  - `…/🗿️artifacts/🖐️5d/…/✏️editor/🌉️wasm/🦀️.rs:138`

    all three `serde_json::to_string(&x).map_err(…)` → `Ok(dsl::json::to_json_string(&x))`, the same
    infallible bridge `📚️examples/*/🦀️.rs:36` and `✏️editor/🎚️config/🦀️.rs:369` already use.
- `PluginCloseStep::AwaitingInput` (added with lane C's close-fault work) not covered:
  - `…/🧊️3d/…/🌉️wasm/🦀️.rs:125` and `…/🖐️5d/…/🌉️wasm/🦀️.rs:124` — joined to the `Pending | Blocked =>
    Ok(false)` arm, matching the framework's own classification at `🔌️plugin/🦀️.rs:13767,14287`.

Verified: puzzle compiles clean on `wasm32-unknown-unknown` in runs 2-4.

### 3.3 Fixed: the renderer crate's manifest never declared its browser async dependency

`🧰️framework/…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml:108-116` — `semio-framework-async` was
declared **only** under `cfg(not(target_arch = "wasm32"))`, while four browser sites name it:
`🧊️renderer/🦀️.rs:2` (`extern crate … as wasm_bindgen_futures`), `:122` (`browser::spawn_local`),
`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:19` (`browser::JsFuture`), `🧵️frame-job/🦀️.rs:13` (`Lane`).
Added to the `cfg(all(target_arch = "wasm32", target_os = "unknown"))` block **without** the
`entrypoint` feature (`block_on` must stay unreachable from a browser build). This single line is why
run 2 could not even parse the crate.

Also `🌐️browser-worker/🦀️.rs:314` — a second `impl BrowserRendererWorker` block carrying
`#[wasm_bindgen(js_name = closeStep)]` had no `#[wasm_bindgen]` on the impl itself
("the `self` argument is only allowed for functions in `impl` blocks"). Added.

### 3.4 Fixed: the shared class (broke BOTH targets; 123 → 101 native, 71 → 49 wasm)

Diffing the two error sets isolated 8 distinct target-independent faults:

- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️.rs:294-299` — `ui_wgpu::wgpu`
  never re-exported `PreparedAtlasPages`, `PreparedRenderInputRejected`, `PreparedRenderJobRejected`
  (they live in `📦️prepared.rs`), so 9 renderer sites failed `E0425`/`E0433`. Added to the existing
  unconditional `pub use prepared::{…}`.
- `…/🎯️targets/🧊️wgpu/🦀️.rs:292-298` — same for `gpu::PreparedGpuPresentCursor` (the abandonment-owner
  handle `🧊️renderer/🦀️.rs:14178` holds and `:15054` drains), added under `mod gpu`'s own
  `feature = "wgpu-engine"` gate.
- `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7902` — `chrome_text` was `#[cfg(test)]` while **eleven
  production call sites** inside `impl ShellState` (`:14818, 14843, 14847, 14855, 14873, 15058, 15063,
  15075, 15238, 15571, 15586`) call it. Gate removed, docstring added. This alone is 11 of the errors
  and had to be wrong on every target.
- `🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:29` — `ImeEvent` used at `:327` but absent from the
  `use ui_render::{…}` list.

### 3.5 Fixed: four upstream relocations the crate never followed (101 → 26 native)

Every one is a rename that landed elsewhere and left this crate as the sole unrecompiled consumer:

| Old path in the renderer | Current home | Sites |
|---|---|---|
| `store::os_store::{DslValue, ToValue, FromValue, ValueError}` | crate root `store::…` (`💻️os/📦️packages/🦀️rust/🦀️.rs:335-340` says so explicitly) | 42 |
| `store::os_store::{Mutation, MutationDiff, MutationLeafDescriptor, MutationOutcome, MutationOutcomeClass, MutationComposition, MutationInvertibility, MutationDiffParticipation, MutationLanguageSurface, MutationApplyResult, OpText, OpBinary}` | `store::os_spr::command::…` (`📡️spr/🎮️command/🦀️.rs:16` re-exports `protocol::mutation::*`) | 15 |
| `store::os_store::ProtocolError` | `store::os_spr::ProtocolError` (flat via `pub use self::wire::*`) | 3 |
| `semio_framework_async::TokioHostRuntime` | `semio_framework_os_services::TokioHostRuntime` — the crate's *own* `Cargo.toml:95-100` comment already says os-services owns it, and `🧊️renderer/🦀️.rs:16098` already imports it from there | 2 |
| `protocol::{PRESENCE_ROSTER_MAXIMUM_ITEMS, PresenceRosterWire}` (`🌉️ProgramBridge/…/🦀️.rs:262,266`) | `flow::os_spr::channel::…` | 2 |
| bare `ProgramBridgeEntry` (`🧊️renderer/🦀️.rs:11501`) | `program_bridge::ProgramBridgeEntry` | 1 |
| bare `WorkerCell` (`🐚️Shell/…/🦀️.rs:16117`) | `crate::interpreter::WorkerCell` — also widened from private to `pub(crate)` at `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:26-37` | 1 |
| `semio_framework_actor::JOB_PROGRESS_ACTIVE_CAPACITY` missing from `🧊️renderer/🦀️.rs:3160`'s import list | added | 46 |

After these the name-resolution layer is clear natively (zero `E0405/E0422/E0425/E0433/E0463`) and all
but one on wasm (the single remaining `E0425` is `ShellSpaceAdministrationPhaseV1`, family 3 below —
a type a peer's Shell edit references but never declares). Final code distribution is in
`🗑️generated/lane-l/wgpu-check-{wasm32,native}-4.txt`. Native then re-counted **74**, not 26: clearing the unresolved names unblocks
type-checking of regions rustc had been skipping, so each layer reveals the next. That is progress, not
regression — the 26 was a floor artefact.

### 3.6 Remaining census — what still blocks the build (NOT fixed, needs owners)

`wasm32-unknown-unknown`: **49 errors**; native: **74**. All are type-level, in five families:

1. **serde-elimination fallout, 12 sites** (`🐚️Shell/…/🦀️.rs:184,186,212,215,4540,4594,4898,5009,7279`
   + `🌉️ProgramBridge/…:620,630`): `ManifestCommandInvocation` / `ManifestActionInvocation` /
   `InvocationResult` no longer derive `Serialize`/`Deserialize`, and `serde_json::Value` does not
   implement `ToValue`/`FromValue`. Same class as §3.2 and as the coordinator's 05:20 React fix —
   these must move to `dsl::json::{to_json_string, from_json_str}` / the `DslValue` bridge.
   **Owner: whoever owns ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`
   for the wgpu shell** — the decision (which types get `ToValue`) is theirs, not this lane's.
2. **`dsl::Fault` lost `Display`** — 12 sites in `🧊️renderer/🦀️.rs` (`:5296,5299,5314,7326,7335,7358,
   7365,7377,7399,7407,7413,7425,7432`) still `format!`/`to_string()` it.
3. **Struct/enum shape drift** — `semio_framework_actor::TurnResult` gained `lifecycle_receipt`/
   `ui_patch_receipt` (3 initializers), `kernel::Event` gained `receipt` and lost
   `InstanceOpen.instance`, `AppRuntime` gained `pending_frame_deferred`, `AppPresenter` gained
   `retained_fault` (`🌐️browser-worker/🦀️.rs:646,673`), `JobPayloadStream` lost
   `Checkpoint`/`Commit` (`📐️surface-lane/🦀️.rs:100,125,134`), `ShellState` has no
   `space_administration`/`space_administration_epoch` and no `ShellSpaceAdministrationPhaseV1`
   (`🐚️Shell/…:2141,3260,3261`), `UiDocumentLease` is no longer `Clone` (`:12750,12856`),
   `retire_step`/`close_step` are private (5 sites), `[u16; 64]: Default` (`:12090`).
4. **Browser-only `Send` violations, 6 sites** (`🧵️frame-job/🦀️.rs:346,373,375`): `Rc<dyn Fn>` /
   `Rc<JsValue>` crossing a `Send` bound. Genuinely wasm-specific and genuinely this tier's — it is
   the `run_on_worker`/`Lane` seam meeting the browser's single-threaded JS values.
5. **Borrowck, 6 sites** (`🧊️renderer/🦀️.rs:2563,13649,13683,13717`, `🐚️Shell/…:1313,1863,3578`).

Estimated: families 1-3 are mechanical but need the owning tickets' decisions; 4-5 are real design
work. This is **not** a one-lane job and it is **not** what the readiness report budgeted (it assumed
"5-20 min for one cargo check" then a build).

## 4. Boot (checklist step 6) — NOT ATTEMPTED, and it cannot succeed

`TrunkServeScript.run` invokes `trunk serve`, whose `data-trunk rel="rust"` link runs
`cargo build --target wasm32-unknown-unknown` of exactly the crate measured above. With 49 errors the
build fails before `wasm-bindgen`, so there is no `_bg.wasm` to serve and no page to drive with
Playwright. Running it would only reproduce §3's log at higher cost. **Port 6066 was never bound; no
`trunk`/`dev` process was started by this lane.** The other four steps of `TrunkServeScript` (`ensureTrunk`,
`ensureWasmTarget`, `buildBootScript`, `checkFrameWorker`) are all individually verified green
(§2b), so the moment the crate compiles the boot attempt is one command away.

## 5. `parity verify s` (checklist step 8) — blocked, plus two contract drifts found statically

`parity verify s` cannot run: its step 3 (`startParityDevServer("wgpu", …)`) is the same
`trunk serve`. `PARITY_OUT_DIR` was therefore never exercised. Two further defects would make it fail
*even with a perfect boot*, both found by reading `triageParityBoot`
(`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:3512-3564`) against current source:

- **`#semio-wgpu-canvas` never mounts.** `:3543` waits for that id, but `🚀️browser-boot/🟦️.ts`'s
  `canvasElement()` creates an id-less canvas (`🎬️renderer-boot/🟦️.ts:100-107` documents deliberately
  dropping the fixed id so several library mounts can coexist). **Fixed** in the trunk shell only —
  `🚀️browser-boot/🟦️.ts:37-48`, new exported `WGPU_CANVAS_ID` + `canvas.id`, with a docstring saying
  why the multi-mount library path stays id-less. `🌐️.html:24`'s `#semio-wgpu-canvas` style rule was
  dead until this and is now live. (Verified only by `check-browser-worker`, not in a browser.)
- **`window.wasmBindings.dumpStructure` can never exist. NOT fixed — needs a design decision.**
  `:3550` and `dumpWgpuStructure` (`:3169-3181`) assume Trunk's old main-thread boot glue. Current
  `🌐️.html:7` builds the crate with `data-type="worker"` and `🚀️browser-boot/🟦️.ts:164-166` loads the
  bindings inside `semio-frame-worker`; the main thread never sees a `wasmBindings` global. The
  exports themselves are fine (`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2689-2701`) — their docstring
  still describes the retired boot glue. Fixing this means adding an introspection request/response
  pair to `🚚️browser-frame-transport/🟦️.ts`'s `BrowserFrameUiMessage`/`BrowserFrameWorkerMessage`
  unions (there is none today), a handler in `🎞️frame-worker/🟦️.ts`'s `receive()` (the `bindings`
  object is currently a local inside `boot()` and would have to be hoisted), and a main-thread
  `window.wasmBindings = { dumpStructure, dumpFrameStats }` async shim in `🚀️browser-boot`. It is
  testable in the existing `test-browser-worker` protocol suite. Deferred because it is unverifiable
  until the shell boots.

## 6. Blockers and hand-offs

1. **The wgpu tier is not "one cold build away"; it is ~49 wasm / 74 native type errors away.** This
   supersedes `📓️explore-wgpu-tier-readiness.md`'s cost estimate for steps 4-8. Wave 3's wgpu item
   should be re-planned as a crate-repair lane, not a boot lane.
2. **Not this lane's to decide (family 1 above):** the `ManifestCommandInvocation`/
   `ManifestActionInvocation`/`InvocationResult`/`serde_json::Value` conversions belong with the
   serde-elimination ticket; the wgpu shell is the last consumer still on the old surface.
3. **`ui_wgpu`'s re-export surface is now wider** (`PreparedAtlasPages`, `PreparedRenderInputRejected`,
   `PreparedRenderJobRejected`, `PreparedGpuPresentCursor`) — additive, no existing name changed.
4. **`chrome_text` is no longer test-gated.** If a peer intended the eleven call sites to move to the
   retained `chrome_text_step` path, that migration is still owed; un-gating only makes the crate
   compile in the meantime and does not pre-empt it.
5. **`check-frame-worker` went RED three times in two sessions** from unrelated `💻️os/🟦️.ts` edits.
   It is a committed build artifact of a shared import graph and will keep doing this. Worth a
   coordinator decision: either every OS-TS lane re-runs `generate-frame-worker`, or the gate becomes
   a build step rather than a committed file.
6. Nothing was run on ports 6066/6070/6074/6075/6076/6077. No plugin catalog build was started. The
   only cargo work was `cargo check` in `target-s-e2e-wgpu`.
