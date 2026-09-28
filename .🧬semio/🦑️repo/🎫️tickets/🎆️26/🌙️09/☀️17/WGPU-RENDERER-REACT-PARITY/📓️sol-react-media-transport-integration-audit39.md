# React Media Transport Integration Audit 39

Read-only audit of the live tree on 2026-09-28. Source and tests were not edited and no broad build was run. The implementation was changing concurrently, so line numbers identify the inspected revision.

## Accepted contract and host seam

- The single reserved identity is `framework.media.transport@1`, declared with `framework.window.media` and output port `playback:out` in `🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts:5-8`.
- `parseMediaTransportProps` (`:74-125`) is the authority for host admission. It requires exact keys, schema version 1, audio/video, a nonempty MIME type of at most 128 characters, decimal u64 revision/generation, safe integer time values, paired and ordered selection bounds, explicit `en|de` labels, exact resource/capability objects, resource revision equality, ready-with-resource, and finite `hostContentHeight` in `[0,4096]`. The React host should import this parser and should not duplicate its checks.
- The shared wire authority already exists in `💻️os/🟦️.ts:2635-2638`: `MediaExportHandle`, `MediaExportState`, and `MediaExportStatus`. `MediaTransportHost/🟦️.tsx:7-29` currently duplicates these types. Import the shared types and keep only a local chunk result type if the shared API does not yet export one.
- `PluginWasmHandle` now exposes submit/poll/cancel/take in `PluginRuntime/🟦️.tsx:194-201`. Its adapter (`:3966-3985`) preflights the instance, preserves bigint authority, rejects error/missing frames, and compares all returned handle fields. Cancel correctly expects the real `Done` frame and returns `Promise<void>`.
- `resolveExternalSlots` already supports `hostExtensions`: an exact retained ID bypasses plugin contribution while near matches use the ordinary external-slot path. `ShellHost/🟦️.tsx:5828-5845` now supplies the reserved set. Keep this exact-ID behavior and the neutral host-slots fixture test; do not add prefix matching.
- `Interpreter/🟦️.tsx:2259-2268` now routes the exact ID to `MediaTransportHost` and leaves generic extensions unchanged. Wrap the reserved branch in a per-node fault boundary (`record.id`/`record.key` in the boundary identity), because its current early return bypasses the boundary used by generic extensions.

## Required ownership route

The packet is a request, not authority. Bind each rendered body to one live runtime owner and check the packet against it before submit:

1. Exact plugin handle and `instanceId` from the body that produced the tree.
2. Exact app `controllerId`.
3. Exact live parent document ID and revision where Shell has them; the guest also enforces these in `submit_media_export_checked`, but the shell must not infer them from a UI node key or controller ID.
4. Exact returned handle: app instance, parent document, base revision, operation ID, and generation. `MediaTransportHost/🟦️.tsx:49-59,133-145` currently checks the resource controller/app instance and the returned handle authority, including generation.

`ShellHost/🟦️.tsx:12171-12172,12197-12198` currently constructs owner object literals inside the broad `modeWindows` memo. Unrelated memo invalidation changes the context object and restarts the host effect because it depends on `[owner, props]` (`MediaTransportHost/🟦️.tsx:203`). Cache/memoize owners by exact plugin handle + instance + controller, or make the effect depend on those stable scalar fields and `port`.

The base-window branch currently wraps `browserActorStore ?? localStore` with `primaryMediaOwner` (`ShellHost/🟦️.tsx:12270`). This can route a remote/browser-actor packet into the local guest. Supply a null owner whenever `browserActorStore` is present until the browser actor has its own authenticated media-export command route. Spawned windows must use their spawned plugin/instance owner, which the current branch does. Panel bodies (`ShellHelpers/🟦️.tsx:2698`) currently have no owner provider; keep them unsupported unless a panel-specific document owner is threaded through the panel cache and cache identity.

## Runtime result defect resolved during the audit

The inspected revision originally set the completion reply MIME to `result.schema`, although schema values can be semantic forms such as `2d`. This has now been corrected at the source: `ArtifactMediaExportResult` owns a distinct bounded `mime_type: String` (`🔌️plugin/🦀️.rs:15221-15235`) and `media_export_complete_status` publishes that field (`:41286-41295`). `media_type` remains the semantic media class and must not be substituted for MIME. The host should still require the completed status MIME to equal the validated `props.mediaType` before publishing a Blob.

## Single-owner lifecycle

Use one serialized async owner for each run. No cleanup function or click handler should issue a media command concurrently with that owner.

- While Running, a cancellation request is a flag. After the current awaited poll returns, cancel exactly once and stop polling. Rust cancel removes the running operation and answers `Done`; a subsequent poll is invalid.
- Once poll returns Complete, the runtime has moved the bytes into retained segmented output. Cancel is then invalid. Even if the component becomes stale, drain and discard chunks through the terminal marker so the runtime releases the retained output.
- On every submit/poll/status/capacity/chunk exception, retire the exact known handle: cancel if it is still Running; if Complete was observed, drain/discard to terminal. `MediaTransportHost/🟦️.tsx:185-187` currently only changes React state and can leave a live operation or retained output.
- Apply a byte cap and a chunk/empty-page cap. The current 512 MiB byte cap (`:47`) has no shared contract ownership and there is no chunk bound, so nonterminal empty chunks can loop forever. Reuse the bounded segmented-download contract (currently 32 MiB plus maximum outstanding chunks) or define a schema-owned media limit shared by Rust and TypeScript.
- Verify exact total length even for zero-byte output and reject a MIME mismatch. `status.mime_type || props.mediaType` (`:177`) silently accepts a missing or foreign runtime MIME.
- Publish an object URL only after terminal drain and only if the run is still current. That part of the current host is structurally sound.

Object URL teardown must be a single idempotent operation: pause the element, remove `src`, call `load()`, then revoke the URL once. The effect replacement/unmount and media `onError` now follow that order (`:111-119,193-201,227-237`). Apply the same teardown when `play()` rejects; the current play rejection only changes state (`:244`), unmounting the element while leaving its object URL live until a later effect cleanup.

`hostContentHeight` is now applied to the ready host (`:241`) but not loading/unsupported/invalid states, so the reserved layout collapses while exporting or failing. Put the height on a stable outer host for every state. Running status has no total byte count, so the current indeterminate `<progress>` (`:210`) is correct; expose exact progress as text or `aria-valuetext` rather than treating `applied_progress` as a ratio.

## Smallest test packet

Keep the language-neutral lifecycle fixture/schema beside `MediaTransportHost`, but extend focused tests around the following boundaries:

1. **Adapter wire truth:** fake Rust cancel with `Done`; verify error/missing/wrong frame rejection, full foreign-handle rejection, and bigint preservation for every method. The current `PluginRuntime` test at `:1727-1790` now covers the basic `Done` shape and foreign receipts.
2. **Happy path:** exact submit fields, one poll or chunk per event-loop turn, complete-to-terminal drain, exact byte order/length/MIME, one Blob URL, no autoplay, explicit accessible controls, and stable height in all states.
3. **Cancellation races:** revision replacement and unmount while a poll promise is pending; cancellation after Running; stale completion cannot publish; Complete observed during cleanup drains/discards and never sends cancel; rapid cancel clicks send at most one command.
4. **Retirement failures:** submit/poll/status/chunk fault, foreign handle in each reply, over-cap bytes, excessive/empty pages, wrong total length, wrong MIME, cancelled/failed status. Assert no URL and exact cancel-or-drain retirement.
5. **URL ownership:** replacement, unmount, media decode error, and rejected play all detach before one revoke. A newly published URL must never be revoked by an older run.
6. **Shell authority:** exact reserved ID survives resolution while near matches do not; local base and extra windows bind the correct instance; two instances of the same controller cannot cross-route; spawned owner uses its plugin; browser-actor and panel bodies receive null until routed; unrelated Shell rerenders do not resubmit.
7. **Parity text:** malformed `locale: en/de` gets the same localized invalid-contract text as WGPU; an invalid locale gets the stable neutral code. Unsupported/resource-null packets never call the port.
8. **Real browser:** inject a small valid audio fixture into the page itself and assert metadata, play/pause, seek and replacement teardown. Copying jsdom `innerHTML` to Chromium, as the current test does (`MediaTransportHost/.../🧪️tests/♻️lifecycle/🟦️.tsx:126-153`), verifies static markup only; it does not execute the React host, Blob creation, event handlers, channel lifecycle, or object URL ownership in Chromium.

The current lifecycle test covers schema validity, basic happy-path calls, static controls, stale owner refusal, and static Chromium markup. It does not yet exercise in-flight cancellation, terminal drain on stale runs, faults/caps/MIME, decode/play URL cleanup, browser actor authority, or stable owner identity.
