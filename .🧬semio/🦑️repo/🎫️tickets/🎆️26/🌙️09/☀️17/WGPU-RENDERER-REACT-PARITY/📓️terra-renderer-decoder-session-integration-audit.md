# Renderer Decoder Session Integration Audit

## Scope

Read-only source audit of the portable decoder integration added for Native 116. No build or test was run. The reviewed path is:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
  - `RendererAssetDecodeJob` and `RendererAssetDecodeSession` (`:2889–:3070`)
  - `RuntimeMailbox::pump_asset_decode_session_step` (`:11493`)
  - restore and publication (`:11470`, `:11540`)
  - native reference and transport (`:11916`, `:15024`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:876–:1005`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:342`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:401`

## Confirmed defects

### 1. Cancellation can lose the publication fence after decoder Ready

`RendererAssetDecodeSession::pump_one` takes a terminal job's probe and records `(probe, Some(Ready))` in `boundary` (`🧊️renderer/🦀️.rs:3058`). `pump_asset_decode_session_step` then restores that boundary at `:11504` without validating its request again. `restore_asset_decode_boundary` accepts `Ready` as `Pending` at `:11477` and restores the Ready probe.

The current publication path for shared images/maps, terrain, and GLB (`:11548–:11664`) does not call the already-existing `renderer_asset_current` / `renderer_asset_cancelled` authority at `:11831`. A request retired after terminal extraction but before restore/publication can therefore still apply bytes or a mesh.

This applies to `Rejected` too: restoration records a shared image/map miss through `record_rejected_asset` even if the request became stale after decoder terminal. A cancellation should have no publication side effect, including a stale missing-asset marker.

**Required boundary:** distinguish `Current`, `Cancelled`, and `Busy` at both handback and immediate pre-publication. Only `Current` may restore a Ready/Rejected boundary for application. `Cancelled` must convert every not-yet-published boundary to `Cancelled`, call `probe.begin_close()`, and retain it for ordinary exact owner return. `Busy` must retain the boundary without applying it; it is neither a cancellation nor a permission to publish.

**Fail-first law:** create a real sealed World GLB or shared map/image owner, drive its `RendererAssetDecodeSession` through terminal Ready, cancel its exact request before `take_boundary`, then step to terminal close. Assert no mesh/bytes/miss is published, the exact request token returns to its original authority, and every session/probe owner is terminal empty. Repeat with a rejected shared image and assert no stale empty-byte/miss projection.

### 2. Native treats an ordinary interaction checkout as cancellation

`renderer_asset_current` returns `None` when it cannot inspect the runtime or when `runtime.interaction` is temporarily absent (`🧊️renderer/🦀️.rs:11831`). That absence is a normal, explicit `AppRuntime::check_out_interaction` state (`:12566–:12582`), used by dispatch and deferred frame work.

`renderer_asset_cancelled` turns both an actual retired token and that unknown state into `true` (`:11842`). Native consumers use it as a destructive verdict:

- `pump_native_reference_decode` cancels its decode job (`:11940`);
- local-file streaming aborts before a page read (`:15035`);
- HTTP streaming aborts between chunks (`:15097`).

The browser deliberately has the opposite temporary-state rule: `browser_renderer_asset_current` accepts `None` and rejects only `Some(false)` (`:11846`). This makes the native use of the same tri-state source an actual cross-target semantic mismatch.

`pump_native_asset` then handles every stream error alike: it begins close on the fetch and records `native renderer asset fetch failed` (`:12039–:12052`). Thus an ordinary interaction checkout can both cancel a valid request and raise a renderer fault.

**Required boundary:** replace boolean cancellation with a small liveness result owned by `RuntimeMailbox`:

| Liveness | Decode/session action | Native transport action | Publication action |
| --- | --- | --- | --- |
| `Current` | advance | read/seal | apply |
| `Cancelled` | begin exact close | cancel and return owner | never apply |
| `Busy` | preserve owner/boundary | preserve owner; do not fault | defer |

The existing host-completion waker is the wake for `Busy`: `RuntimeMailboxInner::finish` invokes it on interaction return (`🧊️renderer/🦀️.rs:11054–:11063`); native maps it to `HostUserEvent::Wake` (`🪟️winit-app/🦀️.rs:876–:910`). Do not turn a `Busy` retry into an autonomous redraw loop. `asset_decode_step` currently self-wakes for a non-session `Pending` turn (`🧊️renderer/🦀️.rs:11534`), so the new result needs an explicit parked/blocked outcome or equivalent suppression until the existing owner wake arrives.

**Fail-first law:** with a current World request and a Ready probe/native reference job, check out `AppRuntime.interaction` using its real checkout seam, then drive one decoder/transport opportunity. Assert that it does not cancel, fault, publish, or consume a response page. Return the exact interaction and invoke its existing wake; the next opportunity publishes or resumes normally. A separate cancel variant must return the exact owner without setting `frame_fault`.

### 3. Native asset transport is deliberately nonfunctional for every nonempty response

`stream_native_renderer_asset` obtains `NativeIoValue::Page { bytes, eof }` and passes any populated page to `push_renderer_asset_page` (`🧊️renderer/🦀️.rs:15038–:15047`). That function closes the `RetainedJobPayload` and always returns an error when populated (`:15060–:15067`). The HTTP path likewise refuses its first nonempty chunk (`:15105–:15113`). Consequently no native GLB, terrain, map tile, UI image, or reference source can reach seal/decode through this integration.

There is **no current zero-copy transfer API** from `RetainedJobPayload` to `WorldAssetResponsePage`:

- `RetainedJobPayload` only exposes borrowed `page`/`reader` access and incremental `close_step` (`🧰️framework/🔨️modules/🧵️job/🦀️.rs:563–:642`);
- `WorldAssetResponsePage` owns a `Box<[u8]>` and only provides `try_from_owned(Vec<u8>)` (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:14758–:14770`).

The first-party native-I/O test helper `payload_vec` is the existing safe conversion precedent: read the bounded page, copy it into a `Vec`, then close the payload one page at a time (`🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🧪️tests/🔬️native-io-unit/🦀️.rs:3`). A zero-copy claim would be false unless a new compatible page-backing owner is designed; the two existing owner representations are not transferable today.

**Minimal safe handoff:** a renderer-local bounded conversion accepts exactly one retained page whose `len <= WORLD_ASSET_RESPONSE_PAGE_BYTES`, copies it once into `Vec<u8>`, calls `WorldAssetResponsePage::try_from_owned`, pushes it onto the exact fetch, and only then performs the payload's required one-page close. On conversion/push refusal it must return the exact fetch plus still-owned payload into bounded close; it must not drop either.

The final `seal_renderer_asset_response` already has a distinct `Busy` result. It is currently converted to a stream error and then a frame fault. Preserve a successfully read, unsealed fetch in a dedicated bounded `SealPending` owner and retry sealing after the runtime wake; do not reuse the existing `native_asset_blocked` closing-only owner for it. Its two states have opposite lifecycle requirements.

**Fail-first laws:**

1. A real local multi-page GLB streams page-by-page, seals once, reaches the independent decoder, and publishes its expected mesh without a frame transaction advancing decoder bytes.
2. A response page at the exact limit transfers and terminal-closes its retained source; a limit-plus-one page is refused without an owner leak.
3. A cancel after one accepted native page returns the same request token to the authority, closes every page/fetch owner, and leaves `frame_fault` empty.
4. A transient seal `Busy` retains the unsealed fetch, makes no error record, then seals exactly once after an interaction-return wake.

## Admission rejection and close

The initial `WorkerJobSession::try_new` rejection path recovers the exact probe from the rejection guard (`RendererAssetDecodeSession::new`, `🧊️renderer/🦀️.rs:2999`) and incrementally closes the guard before exposing its boundary (`:3024`, `:3087`). This is ownership-safe as written.

It nevertheless has a cancellation semantic gap: a cancelled request while the rejected guard is closing leaves a boundary result of `None`. Once rejection cleanup completes, the probe is restored as pending and a second decoder session is admitted just to observe the same cancellation. It does not lose the owner, but violates the requested cancellation boundary and wastes a decoder admission.

Extend the pre-publication cancellation conversion above to a `None` admission boundary. The saturated-session law already exists at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:538`; add a cancellation-before-rejected-close variant. It must show zero `probe.step` calls after cancellation, a closing probe on restoration, and exact terminal handback.

Native component close correctly observes a matching live decoder session before it tries to close its World asset authority (`🧊️renderer/🦀️.rs:11745–:11770`). The Winit host pumps `asset_decode_step` before its component-close advance (`🪟️winit-app/🦀️.rs:230–:240`), so this audit found no separate native session-versus-close deadlock. Browser close relies on the World asset authority staying nonterminal while its owner is checked out; it needs the cancellation and retained-turn laws above, but no independent owner-loss path was established here.

## Wakes and bounded scheduling

The native worker session registers a `RuntimeHostWaker` before submit (`🧊️renderer/🦀️.rs:3035`), and the Winit runtime binds that to its `Wake` event. The browser exports a private `assetDecodeStep` (`🌐️browser-worker/🦀️.rs:342`) and the `FrameTurnScheduler` alternates the asset owner with frame work in one `MessageChannel` task credit (`🧵️frame-turn-scheduler/🟦️.ts:47–:120`). No lost terminal-worker wake was established by this audit.

The known risk is instead unbounded retry under `Busy`: `has_pending_asset_decode` conservatively treats lock contention as pending (`🧊️renderer/🦀️.rs:12104`) and a pending non-session turn self-wakes. The checkout law above must assert that this path parks on the existing checkout-owner wake rather than generating repeated empty decoder opportunities or input-generation changes.

## Confidence

High confidence: Ready/Rejected post-terminal publication is unfenced; native `None` liveness is misclassified; populated native responses are refused; cancellation is reported as a native frame fault. Medium confidence: the rejected-admission cancellation gap causes one unnecessary resubmission; source proves the sequence but this audit did not execute it. No source was changed.
