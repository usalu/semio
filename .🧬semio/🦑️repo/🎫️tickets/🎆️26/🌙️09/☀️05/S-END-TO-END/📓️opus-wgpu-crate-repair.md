# Lane N — `wgpu-crate-repair`: `semio-framework-os-renderer-wgpu` compiles again

Opus 5 implementer, 2026-09-06 ~13:00 → . Input: `📓️opus-wgpu-tier.md` §3.6 (lane L2's five-family
census), `📓️explore-wgpu-tier-readiness.md`, `📋️plan.md`. Private cargo target
`target-s-e2e-wgpu`, `RUSTC_WRAPPER=""`. Logs: `🗑️generated/lane-n/` (`native-0` = the 74-error
baseline, `native-3` = green, `wasm-1` = the 42-error intermediate, `wasm-12` = green,
`wasm-build-1-tail` = the real browser build, `trunk-build-2-blocker` = §7.2's peer breakage).

## 0. Headline

| Target | Before (lane L2) | After |
|---|---|---|
| `wasm32-unknown-unknown` (browser) | **49 errors** | **0 — `cargo check` exit 0** |
| native (`aarch64-apple-darwin`) | **74 errors** | **0 — `cargo check` exit 0** |

All four registered cheap gates green (§4). The crate's own `#[cfg(test)]` suites do **not** build —
independent, pre-existing drift that lane L2 never measured (§6.1).

## 1. Family-by-family

Counts are lane L2's census, cross-checked against the measured native run at 13:0x
(`lane-n/native-0.txt`, 74 errors: 23 E0599, 22 E0277, 6 E0624, 5 E0308, 5 E0063, 3 E0502, 3 E0499,
2 E0559, 2 E0506, 1 E0507, 1 E0004).

| Family | wasm before | native before | after |
|---|---|---|---|
| 1 serde-elimination fallout | 14 | 17 | 0 |
| 2 `dsl::Fault` lost `Display` | 0 | 14 | 0 |
| 3 struct/enum shape drift | 23 | 30 | 0 |
| 4 browser `Rc`-vs-`Send` | 6 | 0 | 0 |
| 5 borrowck / lifetimes | 6 | 13 | 0 |

### 1.1 Family 2 — `dsl::Fault` has no `Display`, on purpose

The framework offered **no** canonical accessor: three sites in
`🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs` (`:389`, `:1800`, `:1804`) each re-derived
`format!("{}: {}", fault.code.0, fault.message)` by hand, and `🔌️plugin/🦀️.rs:31409,31452` drop the
code entirely with `.map_err(|fault| fault.message)`. So the accessor was written once, in the
owning module, rather than a `Display` shim in the renderer:

- `🧰️framework/🔨️modules/⚠️diagnostic/🦀️.rs:564-573` — new `Fault::describe()` →
  `format!("{}: {}", self.code.0, self.message)`, with a docstring stating why `Fault` deliberately
  has no `Display` (a structured abort must not be silently interpolated, and `to_string()` would
  hide the `FaultCode` every triage tool keys on).
- `…/⚠️diagnostic/🦀️.rs:723-747` — new `#[cfg(test)] mod fault_describe_tests`, 3 cases
  (code+message pairing, empty message keeps the code, survives an `encode_fault_bytes` →
  `decode_fault_bytes` round trip). **Run, green** — see §5.1.
- `🧊️renderer/🦀️.rs` — 14 `fault.to_string()` → `fault.describe()` at `:5296, 5299, 5304, 5314,
  7326, 7335, 7358, 7365, 7377, 7399, 7407, 7413, 7425, 7432` (pre-edit numbering).

`🏃️run`'s three hand-rolled copies were left alone deliberately — they are outside this lane's
blast radius and a peer owns that file; the accessor is now there for them to adopt.

### 1.2 Family 1 — serde-elimination fallout

`ManifestCommandInvocation`/`ManifestActionInvocation` (= `semio_framework::manifest::{Command,
Action}Invocation`) dropped `Serialize`/`Deserialize` and re-keyed `arguments` to
`BTreeMap<String, DslValue>`; `store::to_dsl_value`/`from_dsl_value` re-bound to `ToValue`/
`FromValue`; `serde_json::Value` is deliberately not a `ToValue`/`FromValue` target.

- `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:33-37` — `use serde::{de::DeserializeOwned, Serialize}`
  replaced by `dsl::{FromValue, ToValue}`; `:47` `encode_wire<T: ToValue>`, `:53`
  `decode_wire<T: FromValue>`.
- `…/🌉️ProgramBridge/…:184, 212` — `serde_json::from_str` → `dsl::json::from_json_str`.
- `…/🌉️ProgramBridge/…:618, 628` (browser half) — `serde_json::from_str::<InvocationResult>` →
  `dsl::os_pack::json::from_json_str::<…>` (`InvocationResult` derives `ToValue, FromValue` at
  `🎠️kernel/🦀️.rs:881`).
- `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4540, 4594, 4898, 5009, 5364, 9288, 9303` — `serde_json::to_string`
  → `dsl::os_pack::json::to_json_string` (infallible, so the two `match … { Ok/Err }` wrappers at
  `:4540`/`:4594` collapsed to a straight-line call and the two `.expect("command invocation
  serializes")` disappeared); `:7279` `from_str` → `from_json_str`.
- `🐚️Shell/…:9152, 9155` — `resolved_execute_args` now bridges through the `From` impls the value
  module owns (`🌱️value/🦀️.rs:218,268`): `DslValue::from(Value::Object(staged.clone()))` and
  `Value::from(&effective).as_object().cloned()`. **Not** `pack::json::to_dsl_value`, which takes
  `pack::json::Value` (a different first-party JSON type) — the first attempt did and produced
  `expected &JsonValue, found &Value`.
- `🎞️Scenes/…:2283` and `🗣️Interpreter/…:538` — `merge_action_args`'s
  `semio_framework::from_dsl_value::<Value>(dsl.clone())` → `Value::from(dsl)`, same From impl.

The `to_dsl_value` self-call trap was avoided throughout: no `ToValue` impl in this crate calls
`store::to_dsl_value`; every bridge goes through `DslValue`/`From`.

### 1.3 Family 3 — shape drift

| Site | Drift | Repair |
|---|---|---|
| `🧊️renderer/🦀️.rs:3436` | `kernel::TurnResult` gained `lifecycle_receipt`/`ui_patch_receipt` | threaded straight through from the actor `TurnResult` |
| `🧊️renderer/🦀️.rs:7574, 9434, 9490` | `actor::TurnResult` gained the same two | `None`/`None` on the synthesized `Faulted` turn (no patch ⇒ `validate_pairing` demands `None`) |
| `🧊️renderer/🦀️.rs:7007, 9264` | `kernel::Event::InstanceOpen` swapped `instance: PluginInstanceId` for `request: ActorInstanceOpenRequest` | `ActorInstanceOpenRequest { activation_generation: 1, instance_id, request_sequence: 1 }`, matching the two existing producers (`🌉️mcp/🏠️workspace/🦀️.rs:560,782`) |
| `🧊️renderer/🦀️.rs:7304` | `Event::PatchRejected` gained `receipt: ActorUiPatchReceipt`; `SurfaceId.0` is now `UiText` | see §1.3.1 |
| `📐️surface-lane/🦀️.rs:100,125,134` | `JobPayloadStream::{Checkpoint,Commit}` renamed | `CommitState`/`CommitOutput` — matching `🧵️frame-job`'s own `CommitCandidate` |
| `🌐️browser-worker/🦀️.rs:646,673` | `AppRuntime.pending_frame_deferred`, `AppPresenter.retained_fault` | `None`/`None` at bootstrap |
| `🐚️Shell/…:12085` | `[u16; 64]` has no std `Default` (std stops at 32) | new `ShellUtilityPath([u16; 64])` newtype with a hand-written `Default`, so `ShellChromeChildCursor` stays derivable; 5 call sites take `.0` |
| `🐚️Shell/…:12745, 12851` | `UiDocumentLease` is no longer `Clone` | `lease.try_alias().ok()` — the arena-refcounted alias that replaced `Clone` |
| `🐚️Shell/…:8213` | `UiText::try_from_str` returns `Option`, not `Result` | `.ok_or(())?` |
| `🐚️Shell/…:2141, 3260-3261` | `ShellSpaceAdministrationPhaseV1`'s `impl` block and two `ShellState` initializers missed the `#[cfg(not(target_arch = "wasm32"))]` the rest of the region carries | cfgs added |
| `🪢️kernel-seam/🦀️.rs:128,133` | `SurfaceId(UiText)` vs `KernelOutcome.surface: String` | `.0.as_str().to_string()` |
| `🧊️renderer/🦀️.rs:12941…15168` (5) | `DrawList::retire_step` was `pub(crate)` **inside `ui`**, so invisible to this crate | `🖱️ui/…/🎯️targets/🧊️wgpu/📋️draw_types.rs:311` → `pub`, matching the `pub fn retire_step` its sibling `📦️prepared.rs:1569` already exposes (and which this crate's own source-shape test at `🧊️renderer/🦀️.rs:10927` asserts) |
| `🧊️renderer/🦀️.rs:13870` | `FrameDirectives::close_step` module-private | `pub(crate)` in `🧵️frame-job/🦀️.rs:69` |
| `🐚️Shell/…:16139` | `WorkerCell::{with,borrow,borrow_mut}` module-private (lane L2 widened only the struct) | `pub(crate)` in `🗣️Interpreter/…/🦀️.rs:43-53` |
| `🧊️renderer/🦀️.rs:7902` | `KernelRequest::AcknowledgeTypedOperationResult` uncovered in `shutdown_step` | joined the zero-credit arm (`command_credits` reports `(0, 0)` for it, so a shutdown slice retires it whole) |
| `⚙️EngineCanvas/…:3030` | `let Some(cursor) = cursor.as_mut()` shadowed the `&mut Option<…>` parameter, so the exhaustion reset `*cursor = …ok()` assigned an `Option` to a `MapTileRequestCursor` | parameter renamed to `slot`; the reset writes `*slot` as it was always meant to. **A real latent bug**, not just a type error |
| `🧊️renderer/🦀️.rs:14977` | `push_renderer_asset_page` takes the zero-copy `RetainedJobPayload` the native I/O path produces; the HTTP path hands it a `Vec<u8>` chunk | the HTTP path now returns that helper's own "requires a zero-copy retained-page handoff that is not mounted" verdict directly. Behaviour is identical (the helper rejects every populated page) and no payload is fabricated |
| `🧊️renderer/🦀️.rs:12371` | `record_frame_fault(&'static str)` fed a dynamic `String` | dynamic detail to `log_debug`, frozen label to `record_frame_fault` |

#### 1.3.1 `Event::PatchRejected` needs the issued-patch authority

`ActorUiPatchReceipt` is minted by the guest and pinned 1:1 to a turn's single patch by
`ActorUiPatchReceipt::validate_pairing`. The wgpu in-process kernel tracked no such authority, so the
receipt was threaded rather than invented:

- `🧊️renderer/🦀️.rs` `RetainedSurface` gains `patch_receipt: Option<ActorUiPatchReceipt>`;
- `apply_ui_patch(&mut self, instance, patch, receipt)` takes the turn's `result.ui_patch_receipt`
  (copied out before `try_transfer_one` borrows `result`) and stores it on the slot it admits into;
- `PendingSurfaceRejection` gains `receipt: Option<…>`, filled from that turn at both
  capacity-exhaustion sites and from `retained.patch_receipt` at the `advance_patch_one` site;
- the event is emitted only when a receipt exists. A rejection with none behind it (a surface the
  registry could never admit, so no turn ever reached a slot) has no authority to reject back and
  stays a local registry entry — documented inline rather than fabricated.

### 1.4 Family 4 — browser `Rc` vs `Send` (the one real design decision)

`ActiveFrameBuild` (the frame job) transitively owns `Weak<RuntimeMailboxInner>` → `Mutex<AppRuntime>`
→ `ShellState` → `Vec<ProgramBridgeEntry>` → `Rc<JsValue>` plugin handles, plus an `Rc<dyn Fn()>`
waker. `JsValue` is not `Send` and never can be. `InteractiveJob: Send` therefore made the browser
frame job structurally impossible.

`Send` was **not** asserted away with an `unsafe impl`. Instead the bound moved to the calls that
actually transfer a job:

- `🧵️job/🦀️.rs:1348-1360` — new `JobThreadTransfer` marker: `: Send` with a blanket
  `impl<T: Send + ?Sized>` everywhere except `all(target_arch = "wasm32", not(target_env = "p2"))`,
  where it is bound-free. `InteractiveJob: JobThreadTransfer` — one trait definition, no cfg pair on
  the trait body, and on every threaded target supertrait elaboration still proves `J: Send` exactly
  as before (`WorkerJobSessionInner`'s `unsafe impl<J: Send>` is unchanged).
- `🧵️job/🦀️.rs:2939` `WorkerJobSession::try_submit_step` and `:2199` `MountedWorkerJobSession::pump_one`
  gained `where J: Send` — these are the calls that hand a job to a pool thread.
- The three erasure points that really cross threads now say so:
  `🎯️action-bus/🦀️.rs:301-310` (`Box<dyn InteractiveJob + Send>`, `ErasedToolJob::new<J: … + Send>`),
  `🎯️action-bus/🦀️.rs:332-336` (`ToolJobFactory::Job: … + Send`),
  `🔌️plugin/🦀️.rs:14217` (`Box<dyn ArtifactReservedJob + Send>`), `:14229`
  (`ArtifactReservedToolJob::new<J: … + Send>`), `:19237`
  (`run_framework_reserved_job<J: … + Send>`), and `🔌️plugin/🖥️host/⚡️effects/🦀️.rs` ×5
  (`Box<dyn InteractiveJob + Send>`).
- `🧵️frame-job/🦀️.rs:554-566` — the **browser** `poll_runtime_and_resubmit` no longer submits to
  `renderer_worker_pool()`. It calls `session.try_step_on_caller()`, the in-thread drive the session
  already exposes. Justification is inline: the function's own `web_sys::window().is_some()` guard
  means it only ever runs inside the dedicated `semio-frame-worker` isolate, that isolate *is* the
  frame thread, and a `wasm32-unknown-unknown` build has no second thread at all. Contention is
  transient — the next `poll_runtime_and_resubmit` retries the same generation, exactly as a
  saturated pool submission did.

A narrower attempt (cfg-ing `semio_framework_async::Job` non-`Send` on the browser) was **reverted**:
it cascades into `OnceLock<WorkerPool>` and the timer wheel's callback type, i.e. it would have
required making the whole browser pool non-`Sync`. The call-site fix is smaller and truer.

Blast radius measured: dropping the supertrait surfaced 89 latent `Send` requirements, all in
`🔌️plugin` + `🎯️action-bus`, all resolved by the seven `+ Send` statements above. Both targets are
green with them.

### 1.5 Family 5 — borrowck

| Site | Cause | Repair |
|---|---|---|
| `🐚️Shell/…:3563→3581` | `&ProgramBridgeEntry` from `self.plugins` held across `self.retain_document_for_close` | `.cloned()` on entry and app, matching the file's own `touchArtifact` paths |
| `🧊️renderer/🦀️.rs:13657, 13691, 13725` | `app.input` resolves through `DerefMut for AppRuntime`, i.e. a **whole-`app`** borrow, conflicting with the live `app.interaction.as_mut()` binding | use `interaction.input` / `interaction.wheel_zoom_deadline_ms` — the same object, disjoint fields, no deref |
| `🧊️renderer/🦀️.rs:15127` | `self.input.update_hover(self.last_pointer_x, …)` | hoist the two scalars |
| `🧊️renderer/🦀️.rs:2563` | `self.owner()` returns a borrow of **all** of `*self` | call `self.owner.as_ref().expect(…)` directly — field-precise, so `observed_bytes`/`page_index`/`prefix` stay disjoint |
| `🧊️renderer/🦀️.rs:5685` | `if let Some(id) = state.nodes.keys().next().copied()` keeps the iterator temporary alive through the body (edition 2021) | hoist to a `let` |
| `🧊️renderer/🦀️.rs:3578` | `for field in [&mut self.plugin_id, &mut self.app_id]` live across `self.terminal_is_empty()` | record the released length, check after the loop |
| `🧊️renderer/🦀️.rs:15011` | `self.native_hot_swap_cursor` assigned while `self.shell.plugins.get(self.native_hot_swap_cursor % …)` borrow live (deref coercion to `AppInteractionState`) | compute the index first |
| `🗣️Interpreter/…:1315` | `Ok(Some(page)) if engine.apply_document_page(…, page, …)` moves out of a pattern guard | plain arm with the `if` inside |
| `🗣️Interpreter/…:1865` | `let decode = |_| …` inferred a single-lifetime `FnOnce` | annotate `|_: &[u8]|` so it is higher-ranked |
| `🐚️Shell/…:4528` | `actor` moved into the `DslValue` map then borrowed at `:4581` | `actor.clone()` |

## 2. Files changed

Renderer crate (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/`):

| File | +/− |
|---|---|
| `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` | +86 −40 |
| `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | +42 −36 |
| `🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` | +13 −7 |
| `🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` | +7 −9 |
| `🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs` | +16 −15 |
| `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` | +5 −5 |
| `🎯️targets/🧊️wgpu/📐️surface-lane/🦀️.rs` | +3 −3 |
| `🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs` | +2 −2 |
| `🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` | +2 −2 |
| `🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs` | +2 −0 |
| `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🎞️frame-worker.js` | regenerated (§4) |

Framework callees (each a genuine wrong shape, fixed once at the owner):

| File | +/− | Why |
|---|---|---|
| `🧰️framework/🔨️modules/⚠️diagnostic/🦀️.rs` | +36 −0 | `Fault::describe()` + its 3 tests |
| `🧰️framework/🔨️modules/🧵️job/🦀️.rs` | +31 −3 | `JobThreadTransfer`, `Send` on the two pool-submitting methods |
| `🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs` | +9 −3 | `ErasedToolJob`/`ToolJobFactory::Job` state `Send` |
| `🧰️framework/🔨️modules/🖱️ui/…/🎯️targets/🧊️wgpu/📋️draw_types.rs` | +4 −1 | `DrawList::retire_step` → `pub` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | +6 −3 | three `+ Send` statements |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️.rs` | +5 −5 | `Box<dyn InteractiveJob + Send>` ×5 |

No compatibility layer, no deprecation, no adapter, no `unsafe impl` was added anywhere.

## 3. Commands + real output

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/target-s-e2e-wgpu cargo check \
    -p semio-framework-os-renderer-wgpu --keep-going --message-format=short
error: could not compile `semio-framework-os-renderer-wgpu` (lib) due to 74 previous errors; 144 warnings emitted   # before
…
EXIT=0                                                                                                              # after
```

```
$ CARGO_PROFILE_WASM_DEV_DEBUG=false RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/target-s-e2e-wgpu \
    cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown --keep-going
error: could not compile `semio-framework-os-renderer-wgpu` (lib) due to 49 previous errors    # lane L2 baseline
… 42 … 15 … 2 … 89 (Send blast radius) … 38 … 9 … 3 … 2 …
EXIT=0                                                                                          # after
```

Full logs: `lane-n/native-{0,1,2,3}.txt`, `lane-n/wasm-{1..12}.txt`.

## 4. Registered cheap gates

```
$ bun ./📜️script.ts check-frame-worker
error: 🎞️frame-worker.js is stale; run the generate-frame-worker target
$ bun ./📜️script.ts generate-frame-worker
framework-renderer-wgpu: generated 🎞️frame-worker.js
$ bun ./📜️script.ts check-frame-worker
framework-renderer-wgpu: 🎞️frame-worker.js is fresh

$ bun ./📜️script.ts lint
framework-renderer-wgpu: color-literal lint passed

$ bun ./📜️script.ts check-browser-worker
EXIT=0

$ bun ./📜️script.ts test-browser-worker
 Test Files  2 passed (2)
      Tests  32 passed (32)
   Duration  253ms

$ bun ./📜️script.ts test-preview-generated
 Test Files  1 passed (1)
      Tests  15 passed (15)
   Duration  2.18s
```

`check-frame-worker` was RED again on entry from an unrelated `💻️os/🟦️.ts` edit — the **fourth**
time across three sessions. Lane L2's §6.5 recommendation stands and should be escalated: either
every OS-TS lane re-runs `generate-frame-worker`, or the committed artifact becomes a build step.

## 5. Tests

### 5.1 `Fault::describe` — run, green

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/target-s-e2e-wgpu \
    cargo test -p semio-framework-replication --lib fault_describe -- --nocapture
running 3 tests
test diagnostic::fault_describe_tests::describe_pairs_the_code_with_the_message ... ok
test diagnostic::fault_describe_tests::describe_keeps_the_code_when_the_message_is_empty ... ok
test diagnostic::fault_describe_tests::describe_survives_a_wire_round_trip ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 250 filtered out
```

(The `⚠️diagnostic` module is path-mounted into the replication crate, which is where its
`#[cfg(test)]` blocks compile — the same place `🌱️value/🦀️.rs:324`'s own tests live.)

### 5.2 The renderer crate's own suites cannot run — see §6.1.

## 6. Blockers / hand-offs

### 6.1 The wgpu crate's `#[cfg(test)]` suites are independently broken (56 errors)

`cargo check -p semio-framework-os-renderer-wgpu --tests` reports **56** errors that are **not** this
lane's and were never measured before (lane L2 checked `--lib` only). They are a second, disjoint
body of drift:

- `🧊️renderer/🦀️.rs:10152-10154` — `include_str!` of
  `🖱️ui/…/🎯️targets/🧊️wgpu/🦀️gpu.rs`, `🦀️draw.rs`, `🦀️prepared.rs`. Those files were renamed to
  `⚙️engine.rs`, `✍️draw.rs`, `📦️prepared.rs`; the source-shape assertions read nonexistent paths.
- `🗣️Interpreter/…:1209,1599` — `Ui::apply_tree` no longer exists.
- `🎞️Scenes/…:9534` — `InputState::drain_events` no longer exists.
- `🧵️frame-job/🦀️.rs:717-718` — `PreparedRenderInput::new` gone, `FrameEnginePackets` is no longer a
  `Vec`.
- `🪢️kernel-seam/🦀️.rs:230,235` — the `UiText`/`String` swap, in test code.
- ~20 `doesn't implement Debug` on `kernel_runtime::{PendingSurfaceRejection, KernelRequest,
  ResponseSlot}` and `PendingRasterCheckedOut` — test code calls `.expect()` on `Result`s carrying
  them and none of those types ever derived `Debug`.

**This needs its own lane.** Until it lands, this crate has zero runnable Rust unit tests, so the
family-3/5 repairs above are covered only by `cargo check` on both targets plus the four registered
gates. Every repair is type-checked on wasm **and** native, which is what caught the
`EngineCanvas` shadowing bug and the `AcknowledgeTypedOperationResult` gap.

### 6.2 `Send` is now a stated requirement, not an ambient one

Any new `InteractiveJob` that is dispatched through `ToolJobFactory`, `ErasedToolJob`,
`ArtifactReservedToolJob`, `WorkerJobSession::try_submit_step` or
`MountedWorkerJobSession::pump_one` must be `Send`; the compiler says so at the dispatch site rather
than at the trait. Jobs that only ever `drive_step`/`try_step_on_caller` (the browser frame job) no
longer have to be. `semio_framework_async::Job` was **left** `Send` on all targets.

### 6.3 Not done in this lane

- Browser build (`trunk build`) / headless boot on 6077 / `parity verify s` — status in §7.
- `dumpStructure` introspection pair through `🚚️browser-frame-transport` (lane L2 §5) — not started.
- `🏃️run/🦀️.rs`'s three hand-rolled `code: message` copies could adopt `Fault::describe()`.

## 7. Browser build — the crate builds; the tier is now blocked by a peer, not by this crate

### 7.1 The real `wasm32-unknown-unknown` **build** (not just a check) succeeded

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/target-s-e2e-wgpu \
    cargo build -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown --message-format=short
warning: `semio-framework-os-renderer-wgpu` (lib) generated 353 warnings
    Finished `dev` profile [unoptimized] target(s) in 24m 31s
EXIT=0

$ ls -la target-s-e2e-wgpu/wasm32-unknown-unknown/debug/semio_framework_os_renderer_wgpu.wasm
-rwxr-xr-x  116758887  Sep  6 13:56
```

0 errors, a real 116 MB debug `cdylib`. This is the first time this crate has ever produced a
browser artifact (`.🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu` is still empty, exactly as
`📓️explore-wgpu-tier-readiness.md` §3 inferred). Log: `lane-n/wasm-build-1.txt`.

### 7.2 `trunk build` — blocked by a peer's in-flight edit to `🌊️flow`, at 14:12

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/target-s-e2e-wgpu SEMIO_BUILD_BUDGET_MS=5400000 \
    bun ./📜️script.ts wasm
…
error[E0599]: no method named `retire_cold` found for struct `neural_engine::Tree` in the current scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../🖥️host/🦀️.rs:1123:18
help: trait `ColdRetire` which provides `retire_cold` is implemented but not in scope;
      perhaps you want to import it
   3 + use neural_engine::ColdRetire;
error: could not compile `semio-framework-os-flow` (lib) due to 18 previous errors
error: trunk build failed for wgpu renderer
```

Attribution — this is **not** this lane's and not a stale artefact:

- `git status` shows `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` as ` M`
  (unstaged, live edit), mtime **14:12:36**, i.e. *after* §7.1's 13:56 build in which os-flow
  compiled clean for this exact target.
- The file has 27 `retire_cold(…)` calls and no `use neural_engine::ColdRetire;`; the trait exists
  and is re-exported (`🧠️neural/⚙️engine/🦀️.rs:21`). A peer is mid-refactor.
- `semio-framework-os-flow` is a path dependency of the wgpu renderer, so every build of the tier
  fails there until it lands. Left untouched, per the fleet rule.

It was polled for ~25 min (14:15 → 14:40); the file did not change.

### 7.3 Headless boot on 6077 and `parity verify s` — not reached

`TrunkServeScript` runs the same `cargo build`, so the boot cannot be attempted while §7.2 stands.
Nothing was started on 6066/6070/6077; no plugin catalog build was run; the only cargo work was in
`target-s-e2e-wgpu`. `dumpStructure` (lane L2 §5) is therefore still unimplemented.

**Hand-off — the tier is now one peer fix plus one `trunk build` away:**
1. wait for `🌊️flow/🖥️host/🦀️.rs` to compile (`cargo check -p semio-framework-os-flow --target
   wasm32-unknown-unknown`);
2. `bun ./📜️script.ts wasm` from the wgpu rust package (budget ≥ 25 min cold; **do not** set
   `SEMIO_PARITY_QUIET_CARGO=1` on a warm private target dir — it injects `RUSTFLAGS=-Awarnings`,
   changes every fingerprint and forces a full rebuild; that mistake cost this lane one 25-minute
   cycle);
3. `bun ./📜️script.ts serve` on the catalogue port, then the coordinator's `console-dump-probe.ts`;
4. then lane L2 §5's two contract drifts: `#semio-wgpu-canvas` (already fixed by L2) and
   `window.wasmBindings.dumpStructure` (still owed) before `parity verify s` can pass.
