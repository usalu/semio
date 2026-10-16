# UI Device Accepted Work And Lifetime Design

## Status And Decision

This is the requested Source trace and concrete API/adoption proposal, before implementation. The current provider is not qualified to release its issuing Session through accepted GPU completion after its public submission witness is dropped. The current callback only signals an atomic cell; production draw/readback callers discard the returned `Submission`. A private opaque resource wrapper alone does not establish the required asynchronous lifetime.

The corrected ownership direction is an actual external lifecycle owner, installed before the first backend work, retaining a bounded accepted-work scope independently of caller witnesses. Backend callbacks retain only outcome cells. Actual owner-driven polling/draining removes accepted entries only after real backend outcome. No callback/session cycle, detached polling thread, blocking Drop, elapsed-time completion, global authority registry, integer-handle enrollment, copied Original or marker grant is proposed.

There are real unresolved adoption paths below. A normal-close-only scope must not be reported as a complete fix. The public API and caller adoption should land together after review, with Source and native qualification kept distinct.

## Source Checkpoint

The actual private `GrowBuffer::upload<T: Pod>` correction has a genuine independent AST RED75501. Successor prototype81074/release28889 both passed16 tests/2694 expectations with1 separately routed census skip; strict90062/33816 exited0. All handles were collected. No Rust compiler or GPU backend ran. The earlier actual native91733 Source refusal is retained without retry. The nine-owner Source subdivision and98-owner conservative census remain described in `ui-wgpu-render-subdivision-implementation-current.md`.

## Exact Current Execution Surface

Within `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw`:

| Owner/File | Current Effect | Required Enrollment |
|---|---|---|
| `🎮️execution/🔌️device/🖌️commands/🦀️.rs:13` | Actual queue submit, then callback; public witness is immediately discardable | Reserve an actual scope entry before submission; retain issuing Session through actual outcome even if witness disappears |
| `🎮️execution/🔌️device/🦀️.rs:129` | Buffer mapping callback; Buffer is borrowed only for the initial call | Entry retains exact Buffer and Session through actual mapping callback, including cancellation/error |
| `🎮️execution/🔌️device/🦀️.rs:133` | Queue writes buffer/texture | Retain staged write resources; flush through an actual queue submission/barrier before closing; never call a write complete merely because the synchronous method returned |
| `🎮️execution/🔌️device/📥️platform/🦀️.rs:54` | Canvas renderer receives private raw device/queue and may issue backend work internally | Reserve scope work before the backend call and register actual queue completion on both successful and ordinary error returns |
| `🎮️execution/🦀️.rs:643,651,700,734...923` | Prepared draw phases discard submission witnesses | Poll/retire completed accepted entries while continuing normal frames; retain the scope through frame cancellation and final close |
| `🎮️execution/📤️readback/🦀️.rs:64` | Readback submits, then maps | Keep separate accepted submission/map entries; dropping or cancelling the user readback must not drop their actual retained buffer/session |

Surface present remains inside its existing one-task native/browser presentation phase. A GPU completion barrier must not defer a browser current texture beyond that task. The current `prepared_present_step` result describes submitted/presented work, not CPU-observed GPU completion. Keep that semantic distinction explicit: normal asynchronous frames can return after accepted submission while the external lifecycle owner retains pending work; final retirement and readback consumption require their own real outcome.

`create_buffer_init` uses the backend's initialized-buffer construction rather than this explicit public Queue seam. Its exact backend behavior needs native characterization; do not assume every resource constructor submits queue work or that every constructor is synchronous hardware completion. Device error scopes are asynchronous too: either enroll their accepted future while it owns Session or keep the exact scope future under the same external lifecycle owner; dropping a witness is not an outcome.

## Proposed First-Party API

The neutral contract remains finite data only: work kind (`submission`, `mapping`, `staged-write`, `backend-render`, `error-scope`), finite pending-slot budget, progress and outcome/refusal data. Schema/corpus may describe states for cross-language validation, but cannot mint an owner or authorize work. Existing `ResourceFault` and `OwnedFailure<T>::Returned/Retained` remain canonical; ordinary error data is not proof of completion.

The concrete provider owns these opaque types:

- `DeviceWorkOwner`: non-clone owning object, installed in the actual host/actor lifecycle before resource creation or queued work. It owns private scope state and the actual issuing Session after native preparation succeeds. It has no public constructor from identifiers, resource markers or arbitrary Queue.
- `DeviceWorkScope`: an immutable borrowed view handed to the context/client; it cannot detach the owning object, expand capacity or reopen closed births. Naming a scope is not an authority upgrade.
- `Submission`/`MapCompletion`: observer handles containing outcome cells and weak scope references only. Dropping them is harmless only while the actual `DeviceWorkOwner` remains retained by the lifecycle owner.
- `WorkCloseStep`: `Pending`, real `Complete`, or typed backend refusal with retained ownership. `Complete` requires closed births and no accepted incomplete entries; it is not inferred from a poll return, device liveness, host deadline, missing callback, empty public witness list or elapsed time.

Private graph: lifecycle owner strongly owns `DeviceWorkOwner`; owner strongly owns scope state and actual Session/accepted resources; public Device/Queue/resource wrappers hold their original Session and a weak scope reference; callbacks strongly own only completion/result cells. Scope state has no strong edge back from Session, callback or Queue. This graph has no self-retaining cycle. If the external owner is lost, no claim of completed drain follows; the caller adoption contract below must prevent that while work remains.

Before an operation's first backend effect, validate actual Arc issuer identity and scope identity, verify births remain open and reserve a bounded entry. A foreign/closed/full submission returns the exact original CommandBuffer through `Returned`; no backend submit or accepted entry is created. A stale/foreign Buffer mapping likewise refuses before registration. No sequence number serves as resource authority.

After acceptance, the entry remains owned even when callback registration or the backend operation reports an error. Capture ordinary success/error in the outcome cell and preserve the first business failure while continuing cleanup. A poll failure is retained failure, not permission to release accepted entries. An unwinding backend call and callback-registration failure require an explicit provider guard/characterized backend outcome; do not insert a synthetic completed cell in an error branch. This native error-path law must be authored before implementation is published.

The entry limit is a bounded count, with exact entry/resource retention accounted from the supplied lifecycle's existing retention budget. It must be declared before allocation and charged where an actual StepContext is already supplied; no `StepContext::new`, fresh root or copied clock/grant is introduced to make preparation convenient. Lower backend methods do not receive a structural Original surrogate.

A close turn stops births once, flushes accepted staged writes with an actual submitted queue barrier, polls the actual device nonblocking, and scans/removes at most the supplied bounded work amount. Callback outcome is the only successful completion witness. The owner stays present if the supplied Original cannot currently fund another cleanup turn. Deadline/cancellation stops new work but does not manufacture completion or release roots.

Offscreen siblings need independent close domains. Existing `GpuContext::offscreen` shares backend Device/Queue clones while creating a different upper logical context. It must request a genuine child work owner from its retained parent lifecycle before construction, while the provider privately issues a separate Session/scope over the shared underlying raw backend. Closing the child cannot stop parent births or invalidate the parent's resources. Physical backend identity and logical context identity remain distinct private Arc identities; no public raw Device/Queue or integer session handle is added.

## Concrete Product Owner Adoption

Let `E` be `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`.

### Native Window And Normal Retirement

`E/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:1031` marks close pending on CloseRequested. `about_to_wait:1106` invokes the actual `frame_caller.dispatch_cleanup_step` to move the host into `OsHostRetirement`, repeatedly closes it on the original caller, and exits the event loop only after host/retirement and original frame caller finish. This is a concrete usable owner and drive loop.

`E/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:117` retains the actual presenter inside retirement state. Its `close_step:848` closes presenter world owners, then runtime dynamic owners, then finally retires runtime/presenter at872. `E/.../🧊️renderer/🖥️frame-presentation/🦀️.rs:819` currently drains frame owners, mesh uploads/table and raster table; its terminal predicate checks those real owners.

Add the actual work owner as a lifecycle field before presenter/backend initialization, carry the same move-only object through native bootstrap and `OsHost::try_into_retirement`, and close/drain it before final GPU resource table destruction/presenter release. The exact externally owned scope stays retained if AppPresenter or a per-frame cursor is rejected/dropped. The final host terminal predicate includes the real scope terminal state. Move the owner by value with the actual host; never recover it from a public number or resource pointer.

`E/.../🧊️renderer/🧠️app-runtime/🦀️.rs:1380` currently awaits GpuContext, writes atlases, then can fail at plugin loading/language/shell boot (`1391,1393,1398`). The owner must therefore precede those writes and survive bootstrap failure, not first appear in the successful AppPresenter. A bootstrap result must carry failed build plus original work owner into the same actual caller cleanup continuation. `Result<..., String>` that drops the GPU on `?` is insufficient once work is accepted.

### Browser Worker Normal Close

`E/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:373` starts close. `closeRuntime:557` awaits media retirement, then exact `originalFrameSource.close()`, then alternates runtime/job close steps until both and frame turns are terminal. It emits `closed` only at576 and then closes its worker scope. The exact Original adapter `🎞️frame-worker/🎟️original/🟦️.ts:40` cancels the existing source, repeatedly calls native close with its exact issued turn, receives every receipt, closes native context, then returns the actual original admission.

The native browser close body `E/.../🌐️browser-worker/🦀️.rs:129` closes outputs/assets/frame build/text/events/snapshot; phase5 moves host into retirement at194 and closes it at204. Carry the same work owner through BrowserRendererBootstrap→BrowserRendererWorker→OsHostRetirement and require actual device drain before native close returns true. This naturally delays the worker's existing `closed` response until observed completion; there is no new timeout that implies completion.

Browser bootstrap is also a real failure path: `🌐️browser-worker/🦀️.rs:760,764` queue atlas uploads; `boot_shell:791` consumes self and may return failure; `finish:807` moves GPU into AppPresenter only on success. Replace dropping partial bootstrap after accepted work with a retained bootstrap retirement variant driven on the original lifecycle. A JavaScript `bootAbort` or dropping a Rust future does not itself settle accepted backend work.

### Shell Icon Export

`E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📤️icon-export/🦀️.rs:207` currently creates a detached headless GpuContext, then stores it as a donor `source`. `retire:236` correctly waits for the detached initializer's actual result before continuing; it does not treat cancellation as initialization death. The result needs to include a genuine already retained work owner; detached result storage must retain it on both ordinary failure and cancellation until the parent consumes/drains it.

`IconGpuPngExport` at `E/🧱️elements/🖼️IconRenderHost/🎯️targets/🧊️wgpu/📤️export/🧊️gpu/🦀️.rs:54` constructs a child offscreen GPU. It must borrow an already installed child scope while the batch stores its actual owner outside the render value. Constructor rejection must return the owner/partial GPU as typed retained cleanup alongside the exact prepared packet; returning only the packet is insufficient if constructor queue writes occurred. Existing `cancel:209` cancels and drops user readback/encoder: accepted map/submission entries must remain in the batch's child owner until actual callback.

The normal export closes cursor/rasters/uploads/tables before retiring the packet (`182,187`), and batch `retire:244` repeatedly advances a cancelled render until terminal before dropping it. Insert child scope close/drain after births stop and before table/packet/root release. Normal donor completion at batch `next:175` currently does `source=None` without close; replace that transition with a bounded donor cleanup phase. Batch terminal at309 must include both actual child and donor scope owner emptiness.

Runtime normal shutdown reaches this path: `E/.../🧊️renderer/📮️mailbox-impl/🦀️.rs:149` calls `interaction.shell.close_icon_export_step(cx)` from native host retirement; the Shell method at383 cancels, advances on that supplied context, and only removes a terminal-empty batch. This is the actual final join seam. It must also handle failed creation/rejected render and saving/cancel paths before claiming complete.

No direct production GpuContext or Device/Queue caller was found in Infinite's module subtree by the targeted current Source search. Infinite consumes prepared renderer/host contracts; it must continue using the same retained presenter lifecycle, and its existing host/resource tests need rerunning after adoption. This is a Source observation, not a compiled whole-product closure claim.

## Explicit Drop And Platform Gaps

1. `OsHostRetirement::Drop:939` publishes the actual retained Box to an existing reserved abandonment slot. However `close_abandoned_step:917` expressly returns `UnsupportedOwner` when an abandoned host exists: “abandoned host requires its original supplied lifecycle context”. This retains an actual object, but does not prove eventual polling/drain. Reusing that slot is not a completion fix. The actual winit/browser lifecycle must retain the retirement and original caller together, so ordinary cancellation never drops it into that unsupported path. Supporting arbitrary dropped retirement needs a separate actual original-caller re-adoption contract, not a fresh source.
2. `ShellState::Drop:985` cancels directory and pending I/O tasks only; it does not close icon exports or their GPU. There is no IconExportBatch/BrowserRendererWorker/GpuContext Drop that completes these operations. Normal runtime retirement closes the batch, but directly dropping these public/host values does not. An external actual lifecycle owner must retain the scope independently of those values and own their failed/cancelled continuation. A Drop assertion alone is not sufficient.
3. Page transport `🚚️browser-frame-transport/🟦️.ts:1068` receives `closed` and then terminates the frame worker at1252; this graceful path can use actual device drain. But `requestWorkerClose:1246` catches a failed close-message post and directly calls `retireWorker`. Worker process termination/page shutdown can also prevent any future callback or poll. These are not proof of GPU completion. Keep forced retirement as failed/indeterminate lifecycle result and never mark a native/device-work scope drained on this evidence. A cross-worker transfer cannot reconstruct an opaque device scope from wire data.
4. Browser/native process destruction cannot be made callback-driven by a local Rust owner after its executor has died. A stronger platform cancellation/loss contract would need independently verified native/browser backend support and real outcome tests. No such contract was validated in this lane. This proposal does not claim cleanup progress after machine/process/browser destruction.
5. Current PreparedGpuPresentCursor abandonment holds frame/view, and `close_abandoned_step` drains the cursor. It does not retain the device work scope or poll accepted backend operations. Keep the real scope in the outer lifecycle; do not expand the cursor's global slot into new GPU authority.

These gaps define the honest supported guarantee: a real still-running lifecycle owner, once it accepts work, retains the actual issuing roots until backend completion and continues cleanup on its existing original caller despite user cancellation or observer/value drop. Arbitrary loss of that lifecycle itself remains unqualified until separately solved. If the required contract includes that loss, implementation must wait for a valid ownership primitive rather than land a misleading complete guarantee.

## Planned TDD And Adoption Order

1. Add a neutral finite accepted-work schema/corpus with enroll/refuse/close/poll/callback/error/cancel events. Independently compute expected entries/resource ownership using SQLite; schema data does not authorize GPU work. Check no success is possible without actual outcome, closed/full/foreign refusal cannot enroll, observer drop/cancel is not completion, close prevents births, and first failure survives cleanup.
2. Add independent Rust AST Source guards requiring the actual external lifecycle field and all described success/error/cancel owner moves. Genuine RED must identify the present dropped-witness/callback-only cell defect. This guards Source adoption, not actual GPU behavior.
3. Implement provider scope, pre-effect bounded enrollment, issuer checks, map resource retention, queued-write/barrier and Canvas internal-work capture. Add actual native tests dropping all public witnesses/Device/Queue/client context while retaining the external owner; drive it later to real callbacks and prove exact final root release. Test two actual devices for foreign submission, real scope exhaustion with actual command buffers, map cancellation callback and backend callback error/registration outcome. No fake device or numeric issuer shortcut.
4. Convert constructors into actual lifecycle preparation/retirement results, install owners before bootstrap work, adopt native/browser hosts and Shell donor/child paths, preserve every exact packet/admission Returned/Retained value, and extend final terminal predicates. Close must still run on cancellation/expired original cleanup; it must not mint fresh budget. Keep device completion separate from display submission semantics.
5. Resolve or explicitly limit the actual abandonment/forced-termination contract above with Root review before a production-ready claim. Ensure no accepted work can escape the selected supported scope through an independent constructor or public raw backend seam.
6. Re-run BOTH Source/strict, all nine ordinary Source closures and the full selected census; no cap change. Then request Root's managed lock/native window. Real provider compilation, native/pixel/ownership/readback/host/browser gates remain required. Backend unavailable means no GPU pass, never a fabricated authority shortcut.

No production completion implementation, Core edit, Cargo/store access, lock refresh or native retry was performed for this design.
