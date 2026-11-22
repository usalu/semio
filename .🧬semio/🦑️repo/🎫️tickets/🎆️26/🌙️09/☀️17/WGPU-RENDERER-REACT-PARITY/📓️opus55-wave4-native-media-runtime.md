# Wave 4: native media playback and PluginRuntime handle contract

Paths: `F` = `🧰️framework`, `E` = `F/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`, `PB` = `E/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu`, `SH` = `F/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard`, `N` = `E/🧱️elements/🎬️MediaTransportHost/🎯️targets/🧊️native`.
STATUS: WRITTEN, NOT COMPILED OR RUN (build policy: integration lane E compiles). Every "law" below is a test in the tree that has never executed.

## 1. Native media parity

### Audio survey
Searched the whole framework for CoreAudio, AudioToolbox, AudioQueue, WASAPI, winmm, ALSA, libasound, cpal: no hit in any `.rs` or `.toml`. There is no in-repo platform audio module.

### Decision (AGENTS external-library rules)
First-party port `AudioOutputPort` in the host layer of `semio-renderer-media-native`; platform backends use SYSTEM libraries only through direct `extern "C"` declarations, no new crates, and every open failure falls back to `NullAudioOutput` carrying the reason.
| Platform | Backend | Library |
|---|---|---|
| macOS | `N/🔈️audio-output/🍎️audio-queue` | AudioToolbox `AudioQueue` (framework link) |
| Windows | `N/🔈️audio-output/🪟️waveout` | `winmm` waveOut |
| Linux | `N/🔈️audio-output/🐧️alsa` | `libasound` through `dlopen` |
| tests | `RecordingAudioOutput`, `NullAudioOutput` | none |
`write` never blocks and returns the frames accepted; the presenter carries the unwritten tail to the next tick.

### Presenter
- `N/🖼️presenter/🦀️.rs`: `MediaSlotPresenter` owns one `NativeMediaPlayer` and one audio stream per slot, parses `framework.media.transport@1` props, ticks the clock, tops up the audio queue, drops audio queued before a seek, pauses the device only after the queue drains.
- Controls identical to the browser overlay: root group with `Space ArrowLeft ArrowRight Home End`, play/pause, seek, position, duration, selection start and end, cut, loading/status/cancel rows; labels and spoken times en and de; keys through `media_key_command`, cut through `trim_value_for_selection`.
- Video frames go out as keyed raster uploads (`media-frame:{token}`) plus one contained `MediaQuad`.
- New crate `semio-renderer-wgpu-media-presenter` (`E/🎯️targets/🧊️wgpu/🎛️media-presenter`): `NativeMediaHost::present(slots, delta, uploads)` is the one per-frame call; its `CanvasRasterUploads` uses `queue_decoded_raster_upload_retaining_source`.
- Files: `N/🔈️audio-output/**`, `N/🖼️presenter/🦀️.rs`, `N/🧫️fixtures/🖼️presenter.json`, `E/🎯️targets/🧊️wgpu/🎬️media-native/🧪️tests/{🖼️presenter.rs,🦀️.rs}`, the new crate, edits to `N/🦀️.rs`, `N/🎞️player/🦀️.rs`, media-native `Cargo.toml`, root `Cargo.toml` (member).

### Laws
Controls equal the pinned fixture row for row in en and de; invalid, unsupported, loading, cancelled read as in the overlay; playing the Python-`wave` fixture gives `RecordingAudioOutput` exactly the decoded samples (third-party oracle); back-pressure loses nothing and a seek flushes the queue; one upload per distinct picture; key rules and quad rectangle pinned; selection and cut follow the window kit; spoken time matches the window kit plural fixture; presenter-crate: slot to descriptor conversion and a host run that plays audio and closes a vanished slot's stream.

### Wired into the winit app (coordinator follow-up)
Frame path, one frame late by design (slots are collected after a frame is presented):
1. `OsHost.native_media` (`E/🎯️targets/🧊️wgpu/🏠️os-host/🎞️native-media/🦀️.rs`, native only; field dropped in the retirement destructure, which closes audio) owns a `NativeMediaHost`.
2. `present_snapshot` (`🪟️winit-app/🎞️frame-generation-and-os-host`) clones the published snapshot's `media_slots`, calls `native_media.present(slots, now)` and keeps invalidating (`ANIMATION | PAINT`) while a slot plays or loads.
3. `present_overlays` publishes one `MediaOverlay` per visible slot to the paint crate's registry (`semio_ui_wgpu_paint_overlay`, `🖌️paint/🖼️ui-image-sources-skeleton-and-overlays`): the contained picture plus the transport rows.
4. Video pixels: `UiImageUploads` queues each new picture on the existing UI-image upload queue (`submit_ui_image_upload`); the frame transaction's existing `take_ui_image_upload` drain uploads it to the raster table under the key `media-frame:{token}`.
5. Drawing: the retained paint of the reserved media-transport external slot (`paint_external_slot_node_step`, `🖼️media-and-slot-nodes`) finds the overlay by panel rectangle and draws, through the ui draw contract, the raster quad (`push_raster_quad`), then each row's box (`push_rounded`) and caption (retained text step). The "extension unavailable" caption is no longer drawn when an overlay exists.
6. Input: `OsHost::handle_event` offers events to `NativeMedia` first (`route_media_input`): primary press/release and move on controls (buttons act on release over the same control, ranges seek or set the cut ends while dragged) and `KeyDown` to the focused slot (`Space ArrowLeft ArrowRight Home End` through `media_key_command`). Taken events never reach the app.
7. Bytes: for each slot that waits, an app task (`spawn_app_task`) runs the new public `collect_media_export` (`PB/.../🎛️handle-doors`: submit, poll, take pages, cancel on the slot's cancel flag) against the slot's `appInstanceId`, `resource.outputPort`, `parentDocumentId` and `revision`; progress and bytes return through an inbox drained each frame, then `provide_bytes` decodes natively.
Files: new `🏠️os-host/🎞️native-media/🦀️.rs`; edited `🏠️os-host/🦀️.rs`, `🪟️winit-app/🎞️frame-generation-and-os-host/🦀️.rs`, host-winit `Cargo.toml` (presenter, program-bridge), media-presenter `🦀️.rs` and `Cargo.toml` (`CanvasRasterUploads` replaced by `UiImageUploads`; `media_overlay`, `present_overlays`, `pointer`, `key`, `MediaInput`), paint overlay crate (registry, `submit_ui_image_upload`, row painter, re-exports in `🖌️paint/🦀️.rs`), `🎨️node-painting-with-driver/🖼️media-and-slot-nodes`.
### Follow-up closed (coordinator items 1 to 5)
1. Cut: `NativeMediaHost::new(true)` shows the cut button when a trim is declared. A press and release (or an assistive-technology activate) answers the presenter's `CutRequest`; `NativeMedia` turns it into the page host's media-action JSON `{windowId, controllerId, action, args: {revision, value}}` (window and controller from the slot, exactly what the browser worker pushes) and `OsHost` enqueues the new `RuntimeApply::MediaAction(Some(json))` (`🧊️renderer/🛎️frame-actions/🔁️runtime-apply`, crate `runtime-mailbox`, new dependency on `shell-dock`). Its apply step parses with the shell's own `parse_media_action` and submits an interaction that runs `ActionsCtx::dispatch_action`, the route UI intents and `pump_media_actions` take, so the response's mutations commit through the normal ingest. A refusal is logged.
2. Accessibility: `media_accessibility_nodes` projects each slot's rows (root group depth 0, rows below: role, name, value min/max/now/text, shortcut, disabled, focusable, actionable, focused, rectangle; sliders are `editable` so an assistive technology can set a value). `NativeMedia::announce` publishes them as windows `media.<slot token>` through the new `publish_native_accessibility_windows` (`♿️native-accessibility`), which composes them into every publication so a shell frame that publishes without them does not drop them. Assistive-technology Focus, Blur, Activate and Value on those windows come back as `DispatchEvent::Accessibility`, which `OsHost::route_media_input` offers to `NativeMediaHost::accessibility` first (focus moves, activate presses a button or answers the cut, a value in milliseconds seeks or sets a cut end).
3. Identity: overlays carry the slot's document node id and the paint step looks the overlay up by `tree.document_id(id)` (`media_overlay_for`, replacing the rectangle lookup); the panel rectangle is used only to choose between two windows that mount the same node id.
4. Repaint: the retained paint only runs for a window whose root carries `DIRTY_PAINT` (`frame_step`). New `Ui::request_paint(window_id)` marks it; `OsHost::request_media_paint` calls it through `UI_ENGINE` for every window with a playing or loading slot on each present, and for every media window after a routed input, together with a scheduler `PAINT | ANIMATION` invalidation. This is the explicit path; whether the dirty root is accepted in the middle of an in-flight paint frame was read, not run.
5. Decode off the UI thread: the byte-transfer app task now also decodes (`load_native_media` with progress into the inbox and the slot's cancel flag) and sends the decoded medium back; the UI thread only installs it (`MediaSlotPresenter::finish_decode`, which also opens the audio stream). `provide_bytes` remains for the inline path (laws).
Files this round: `PB/.../🎛️handle-doors` (unchanged), `E/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🔁️runtime-apply/🦀️.rs`, `📨️runtime-mailbox/📦️packages/🦀️rust/Cargo.toml`, `♿️native-accessibility/🦀️.rs`, `🎛️media-presenter/{🦀️.rs,🧪️tests/🦀️.rs,Cargo.toml}`, `N/🖼️presenter/🦀️.rs`, `🏠️os-host/🎞️native-media/🦀️.rs`, `🪟️winit-app/🎞️frame-generation-and-os-host/🦀️.rs`, paint overlay crate and `🖼️media-and-slot-nodes`, `🎨️node-painting-with-driver/🦀️.rs`, `⚙️engine/📣️dispatch-and-overlay-api/🦀️.rs`.
New laws (unrun): decode finished elsewhere starts the player and a cancel stays cancelled; accessibility rows and actions follow the controls; overlays are found by node identity with a rectangle tie-break; the cut button and a press outside every control.
Remaining limits: the cut and the accessibility actions on a media window are unrun end to end; the decode job runs synchronously on a pool worker (cancel is polled by the decoders).

### Last round: page focus order and exact window identity
- Page focus order: the reserved media-transport slot node is now a tab stop of the engine's own traversal (`is_focusable`, `⚡️events/🧭️event-model-and-focus`), so Tab and Shift+Tab reach it in document order like the browser overlay's controls. `Ui::focused_document_node(window_id)` lets the host read where the engine's focus is. `NativeMedia::sync_focus` (each present) follows it: focus landing on a slot node enters the slot's rows (first stop, or last when the traversal came backward, direction taken from the last Tab), focus leaving releases them; a press on a row already holds the slot's focus and is kept. A Tab that arrives before that sync (engine focus already on the slot node) enters directly and is consumed.
- Inside a slot, Tab and Shift+Tab walk its enabled buttons and ranges in reading order (`NativeMediaHost::tab`, per slot now). Past the last or before the first stop the key is `Ignored`, so the engine's `focus_next`/`focus_prev` from the slot node moves on to the next page stop, which may be the next slot. A slot with no tab stop (error status) is passed through at once.
- A pointer press on a row still acts on the control but is no longer swallowed (`MediaRoute.pass_through`): the engine also focuses the slot node as for any focusable node (with its blur commits), so the page's Tab order continues from the slot. Moves, releases, keys, assistive-technology actions and cuts stay the media rows'.
- Exact window identity: `media_window_handle(window_id)` interns the id in a process-wide table (position + 1), replacing the hash, so two window ids can never share a handle; the paint presentation carries it (`Ui::frame_into_step`), overlays carry it, `media_overlay_for(window, node)` is an exact match.
- Files: `🎯️targets/🧊️wgpu/⚡️events/🧭️event-model-and-focus/🦀️.rs` (crate `semio-ui-wgpu-events`), `⚙️engine/{📣️dispatch-and-overlay-api,🖥️frame-into-step-and-frame}/🦀️.rs`, paint overlay crate and `🖌️paint/🦀️.rs`, `🎛️media-presenter/{🦀️.rs,🧪️tests/🦀️.rs}`, `🏠️os-host/🎞️native-media/🦀️.rs`, `🪟️winit-app/🎞️frame-generation-and-os-host/🦀️.rs`. Laws (unrun): exact window and node lookup; Tab walks, undoes and leaves; `enter`, `release` and a missing slot.
- Limits: the slot node is a tab stop even when its host cannot play the medium (one pass-through Tab); the engine and host focus are two owners kept in step once per present, so a Tab typed in the same frame as a focus change relies on the direct-enter path above; all unrun.

## 2. PluginRuntime handle contract

### Method by method: React `PluginWasmHandle` vs `ProgramBridgeEntry`
| React | wgpu | State |
|---|---|---|
| pluginId, manifest | `plugin_id`, `manifest` | had |
| createApp, destroyApp | `create_app`, `destroy_app` | had |
| handleAction, handleCommand | same names | had (dispatch hint: none, the kernel ingress is FIFO per instance) |
| refreshUi, acquireOwnedUiSurface | `render*` (retained lease) | had |
| contextMenu | `context_menu` | GAP closed: returned an empty list on native; now `AppCommand::ContextMenu` with the request in key-ordered wire form |
| readHistory | `read_history` | had |
| applyMutations (report + conflicts) | `apply_mutations` returned `()` | GAP closed: `apply_mutations_reported` |
| readAppDocumentPack | none | GAP closed: `read_app_document_pack` |
| documentPack | none | GAP closed: `document_pack` (cache filled by read and by `load_app_document_pack`, forgotten on destroy) |
| exportAppMedia, importAppMedia | none (declared save/import doors only) | GAP closed: `export_app_media`, `import_app_media` (cancel read before send) |
| submit/poll/cancelMediaExport, takeMediaExportChunk (and takeSegmentedDownloadChunk) | private to the save driver | GAP closed: public `submit_media_export`, `poll_media_export`, `cancel_media_export`, `take_media_export_chunk` |
| readAppDocumentArchive, readAppDocumentIdentity | same | had |
| loadAppDocumentArchive (signal, progress) | no signal, no progress | GAP closed: `load_app_document_archive_observed` |
| mergeAppDocumentArchive | none | GAP closed: `merge_app_document_archive` (answers `ahead`, cancel, progress) |
| readWindowConfigPacks, loadWindowConfigPack | none | GAP closed |
| bindDocumentPort | `bind/retire/receive_document_backbone` | had |
| ephemeralSnapshot, pushPresence | same | had |
| captureExtensionCompletion, invoke | `dispatch_invoke_extension` (wasm32) | had |
| transactionPrepare/Commit/Rollback/Undo/Redo | none | GAP closed (both prepare forms, commit rejection) |
| setMergePolicy | none | GAP closed |
| resolveConflict (report + conflicts) | returned the roster only | GAP closed: `resolve_conflict_reported` |
| readConflicts | same | had |
| openArtifact, setDefaultApp, clearDefaultApp | none | GAP closed |
| readLocalInteraction | none | GAP closed: paged read, ordinal-ordered ACK, cancel through the query cancel, identity authority check, 1 MiB bound |
| subscribeOperationCompletions/Progress/Diagnostics, subscribeSpawnedJobProgress | `take_operation_publications` (polled) | mapped; spawned-job progress is now in the ledger (below) |
| dispose | realm close (`KernelClient::begin_close_realm`) | mapped, not per entry |
Browser (wasm32) keeps the TypeScript handle as owner; the new doors are native only.

Files: `PB/🧳️wasm-program-exchange/🎛️handle-doors/🦀️.rs` (exchange functions, result types, 3 unit tests), `PB/🌉️program-bridge-entry/🎛️handle-doors.rs` (entry methods), edits to `PB/🦀️.rs` (mount, re-exports), `PB/🧳️wasm-program-exchange/🦀️.rs` (`drive_observed_archive` with cancel, progress and `ahead`; three helpers made `pub(super)`), `PB/🌉️program-bridge-entry/🦀️.rs` (context menu arm, pack cache hooks).

### Job progress in the ledger
- `LiveJob.reports_progress` is now real: true once a step of the job published a preview (`operation.preview_sequence > 0`), React's `progressed`. It was always false.
- Polled reconcile no longer refreshes `progress_at_ms` when nothing moved (`JobLedger::advance`).

### Child-process shards visible to the Task Manager
- Wire: process-transport framing gained `TAG_JOBS` (child to parent) and `TAG_JOB_CANCEL` (parent to child) as a side queue (`Decoder::pop_side`), so data frames are untouched.
- Child (`semio-shard`): `ShardLoop::exchange_job_side_channel`, called after every `pump_primed`, sends its process ledger over the pipe when it changed and applies received cancels as presses on its own rows.
- Parent: `mirror_child_jobs` runs on every `ShardTransports::Process` receive and after kill; it replaces that child's rows (key `p{pid}:{actor}#{job}`, owner = pid), cancel goes to the child through `ProcessTransport::job_cancel_sender`, a dead or killed child ends all its rows.
- The Task Manager needs no change: it already lists the process ledger and cancels by row key.
- Files: `SH/🚚️process-transport/🦀️.rs`, `SH/💼️spawned-job-ledger/🦀️.rs` (`advance`, `reconcile_remote_jobs`, wire, `local_live_jobs`, `mirror_child_jobs`), `SH/🛰️shard-transports-and-test-doubles/🦀️.rs`, `SH/🎬️shard-loop-frame-turns-and-job-control/🦀️.rs`, `SH/🌀️shard-loop-registry-and-pump/🦀️.rs`, `SH/📊️shard-outcome-and-loop-state/🦀️.rs` and `SH/🏗️shard-loop-construction-and-replay-drive/🦀️.rs` (field `sent_job_snapshot`).
- Known limits: no production code constructs a `ProcessTransport` yet (selection seam only), so nothing flows until a scheduler does; remote rows carry an empty plugin id until the shell maps the actor; `instance_id` stays 0.

## 3. Laws (all unrun)
| Law | Where | Oracle |
|---|---|---|
| live-job wire equals pinned hex, round trips, truncated, trailing, oversized, bad flag, bad UTF-8 refused | `SH/🧪️tests/🔬️unit/💼️child-process-job-wire` | fixture `SH/🧪️tests/🧫️fixtures/💼️live-jobs/🔣️.json` written with Python `struct` |
| remote rows list, advance without clock churn, cancel reaches the child once, end | same | none needed |
| side frames never reach the data queue | `SH/🚚️process-transport/🧪️tests/🔬️unit` | none needed |
| json objects wire in key order; merge frames; errors refuse; pack cache | `PB/.../🎛️handle-doors/🦀️.rs` tests | serde_json sorted map |
| media laws | section 1 | Python `wave` |

## 4. Compile lane (cargo, `--tests`)
`semio-renderer-media-native` (existing six plus `presenter_tests`), `semio-renderer-wgpu-media-presenter` (new, 2 tests), `semio-plugin-host-process-transport` (`job_side_channel_frames_never_reach_the_data_queue` plus the existing framing tests), `semio-plugin-host-shard` (`child_process_job_wire` three tests, existing unit suite), `semio-renderer-wgpu-host-base` (`job_ledger`, unchanged), `semio-renderer-wgpu-program-bridge` (three new tests; native and wasm32 check, the new files are native only), `semio-ui-wgpu-paint-overlay`, `semio-ui-wgpu-paint`, `semio-ui-wgpu-events` and `semio-ui-wgpu-engine` (registry, paint step, `request_paint`), `semio-renderer-wgpu-runtime-mailbox` and `semio-renderer-wgpu-native-platform` (new variant, composed accessibility windows), `semio-renderer-wgpu-host-winit` (native and wasm32 check; the media field and routes are native only), then dependants `semio-renderer-wgpu-shell-doc-open`. Also `cargo check` media-native for macOS, Windows and Linux targets; regenerate `Cargo.lock`.
Re-exports VERIFIED by reading the crates: `semio_os_kernel` glob-exports `semio_os_spr`, which exports `MergeReport`, `MergePolicy` (wire core), `decode_envelopes`, `WindowConfigPackEntry`, `MediaExportHandleWire`, `MediaExportStateWire`, `DocumentArchive*`. The `LocalInteraction*` types are NOT exported there: fixed, the program bridge now depends on `semio-framework-replication` (alias `replication`) and imports them from it. Read first if red: glob reach of `KernelClient` into the new child module; `include_str!` path in the wire test; `LiveJob` gained a field (only constructor besides the shard loop is the ledger itself); the local-interaction read pumps with `ReadDocumentIdentity` when it has nothing to send because no pump command exists (unverified that pages arrive that way).
