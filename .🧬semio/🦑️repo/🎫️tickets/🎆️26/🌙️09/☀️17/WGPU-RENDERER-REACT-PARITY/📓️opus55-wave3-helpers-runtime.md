# Wave 3: ShellHelpers, PluginRuntime, TaskManager (re-audit items 4, 5, 7)

Paths below are relative to `🧰️framework/🛍️products/💻️os/🔨️modules` (`M`); `E` is `M/📺️renderer/🧑‍🎨engine`; `W` is `E/🎯️targets/🧊️wgpu`; `SH` is `E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging`. Build status is at the end.

## 1. Verification of the claims

| Claim (re-audit item) | Verdict |
|---|---|
| Package sniff `inspectPackageV1` has no Rust twin | TRUE. Added. |
| `resolve_format` has no production caller | TRUE. Now called (through `route_picked_file`) by the native Open File task. `pick_revision` already existed in Rust with a shared vector test; the audit's "not found" for `pickRevisionV1` was a name-grep miss. |
| Media import `importOpenedFilesV1` not ported | FALSE. `file_open_import_actions` chunks by the kernel's `import_payload_chunks`, `begin_import_transfer` drains it as a cancellable, progress-reporting `ImportTransfer`, and a cancel after a delivered chunk sends the app's `importAbort` (`SH/🚶️session/🎨️theme-file-import-and-accessibility`). Fixture `🎠️kernel/🧫️fixtures/📤️file-open-import` is shared with TS and Rust. |
| Tier-2 video frames not ported | FALSE. `Effect::RequestMediaFrames` is handled (`SH/🪝️window-actions/⚙️settings-and-dock-persistence:141`), decodes through `request_media_frames` (native ffmpeg args, wasm path) and runs as `TransferLane::VideoFrames` with progress, Cancel and `importAbort`. |
| PluginRuntime handle contract has no Rust twin | PARTLY. The wgpu twin of `PluginWasmHandle` is `ProgramBridgeEntry` (`E/🧱️elements/🌉️ProgramBridge`); method names are renamed natively (`import_native_file`, `import_native_pdf_source`, `read_app_document_identity`, ...), so a name grep cannot judge it. Handle scope and instance tables are the kernel pool and shard (`W/🧊️renderer/📦️kernel-runtime`). No gap was proven; none was invented. |
| PluginRuntime job ledger has no Rust twin | TRUE for the ledger and its Task Manager feed. Closed except for the producer (below). |
| TaskManager tool-run feed and suspend/resume consumer missing | PARTLY. The feed existed (`set_tool_runs`, `tool_run_panel_tasks`, tool-run rows in the runtime task list). The consumer was missing: `take_task_manager_requests` had no caller, so Suspend, Resume and Cancel on a tool-run row went nowhere. wave3-chrome-b left no note and no edit in these files when re-read, so it was done here. |

## 2. Changes

Package sniff and picked-file routing (item 1):
- `M/🚪️io/🗜️package-sniff/🦀️.rs`: bounded zip tail, central directory and two-entry content check, Rust twin of `🛠️ShellHelpers/🗜️package-sniff`. Cancellation is a separate `Err`, never a verdict. Inflate goes through the first-party `semio-framework-deflate`, which gained `inflate_head` (bounded prefix plus "more followed" flag, `🔨️modules/🗜️deflate/🚰️inflater`, unit-tested against the miniz oracle).
- `M/🚪️io/📂️picked-file-routing/🦀️.rs`: `route_picked_file`, Rust twin of `routePickedFileV1` (resolve, revision pick, package verdict, named refusals).
- Shared vectors: the existing `E/🧪️tests/🗜️package-routing/🧫️fixtures/🔣️.json` now also drives `M/🚪️io/🧪️tests/🗜️package-routing/🦀️.rs`. Archives are written by the third-party `zip` crate, stored and deflated, against the same verdicts the TS test gets from fflate. Includes the read-bound and cancellation laws.
- Production wiring: `SH/📂️doc-open/🎨️theme-file-import-and-accessibility/📂️open.rs` `route_native_pick` runs after the picker answers and before the file is read. An ambiguous name or contradicting package content refuses with the shared `🏛️ShellHost/📂️picked-file-opening/🔣️.json` text (en and de joined, as React renders an unknown locale). `Cargo.toml` of `shell-doc-open` gained `semio-os-io` (native only).
- Known divergence, deliberate: a name the catalog does not know proceeds in the native shell. There the app's declared extension chose what the picker admitted; React uses the catalog as that authority and refuses. Also, neither shell has a producer of package or revision claims today (`packageOf` and `revisionsOf` are never supplied in React either), so the sniff refuses nothing in production until a format declares a claim.

Job ledger (item 2):
- `W/🏛️host-base/💼️spawned-job-ledger/🦀️.rs`: `JobLedger` plus a process ledger (`begin_spawned_job`, `step_spawned_job`, `end_spawned_job`, `cancel_spawned_job`, `spawned_jobs_snapshot`), the Rust twin of `🔌️PluginRuntime/💼️job-ledger` (duplicate begin refused, cooperative cancel once, oldest-first snapshot).
- Shared vectors `E/🧪️tests/🧫️fixtures/💼️job-ledger/🔣️.json`, driven by TS (`E/🧪️tests/💼️job-ledger-vectors`, ran green: 6 pass under `repo test-file`) and Rust (`W/🏛️host-base/🧪️tests/💼️job-ledger`).
- Consumer: `sync_runtime_install_tasks` now also lists the ledger's jobs (`spawned_job_tasks`), and a Cancel press on a `job:` row calls `cancel_spawned_job`.
- Producer, wired (uncompiled): the ledger core now lives in the shard crate (`M/🔌️plugin/🖥️host/🧵️shard/💼️spawned-job-ledger`, the one native producer; host-base re-exports it, native only). `ShardLoop::pump_primed` now wraps the old body (`pump_primed_inner`): first `apply_ledger_cancels`, afterwards `reconcile_job_ledger`. Reconcile makes the ledger list exactly the jobs the shard drives (`running_jobs` plus mounted, non-closing replay seeds), which covers React's `driveSpawnedJob` lifecycle: entry opens when the spawn is admitted, `steps` follows each granted `JobStep` (`job_turns.step_sequence`), the entry ends when the job completes, fails, is cancelled or its actor retires. Reserved framework jobs (undo, redo, selection) are not listed, as in React where `driveReservedToolJob` does not open an entry. Cancel: the row's cancel queues `(actor, job)`; the owning shard applies it through the same path as a guest `Effect::CancelJob` (seed closed as cancelled, or `cancel_job_funded` then removal, a failed cancel retires the actor). Owner column: the shell maps actor id to plugin id from the observed-actor snapshot.
- Known limits: `reportsProgress` is always false (the shard sees steps, not guest progress bytes); `instance_id` is 0; a shard running in a child process (`semio-shard` stdio) has its own ledger and is not visible to the shell; kind falls back to empty when the spawn's kind owner is not yet materialized.

TaskManager (item 3):
- `TaskManagerToolRun.controls` (`pause`, `resume`, `abort` as `ActionDescriptor`s) is filled by `tool_run_panel_tasks` from the Activate bindings of the run group's `toolRunPause`, `toolRunResume`, `toolRunAbort` nodes (React's `toolRunControlsRef`).
- `OpeningState::take_tool_run_control_actions` maps Suspend, Resume and Cancel presses on a `toolRun:` task id to those actions and the dispatch branch (`SH/🔀️actions/🎬️shell-actions/🎯️dispatch-action`) queues them on `settle.deferred_actions`, the lane `importAbort` already uses. A press on a run that no longer offers the control is dropped, as in React.
- Still unconsumed: Cancel or Suspend presses on `toolCall:` rows (agent tool calls). They stay queued.

## 3. Laws

- Schema-first and shared fixtures: package-routing, job-ledger and format-resolution vectors are JSON read by both languages.
- Third-party oracle: `zip` (archives), miniz_oxide (inflate head), fflate (TS).
- Progress and cancellation: the package check polls `cancelled()` before every read and returns `Err(PackageSniffCancelled)`; the native pick passes the open task's `CancelToken`, so a cancel ends the open as `Ok(None)`.
- Concise code, no comments in bodies, emoji docstrings, no legacy paths.
- Async decree: the package check and the ledger are synchronous pure logic over provided reads; the only I/O is the caller's `read`, which runs inside the already async open task.

## 4. Build status

Nothing was compiled or run by cargo (policy change: integration lane E compiles). One cargo check of `semio-os-io` was cancelled by the queue and never produced diagnostics. Only the TS job-ledger vectors ran (6 pass via `repo test-file`).

Crates for lane E to build, with `--tests`: `semio-framework-deflate` (new `inflate_head` and its unit test), `semio-os-io` (new modules `package_sniff`, `picked_file_routing`, new `zip` dev-dependency and `semio-framework-deflate` dependency, test target `package_routing`), `semio-plugin-host-shard` (ledger module, `pump_primed` wrapper), `semio-renderer-wgpu-host-base` (re-export, new dependency on the shard crate, test target `job_ledger`), `semio-renderer-wgpu-shell-doc-open` (new native dependency on `semio-os-io`; `route_native_pick`, tool-run controls, job feed), plus the shell crate that holds `dispatch-action` (`🐚️Shell` wgpu) and the one that holds `panel-records`.

Tests to run: `package_routing` (os-io), `format_resolution` (os-io, unchanged), `job_ledger` (host-base), deflate unit tests, the TS `💼️job-ledger-vectors` and `🗜️package-routing` and `💼️job-ledger` files.

Risks to read first if it does not build: `Cargo.lock` needs `zip` and the new path dependencies; os-io pulling into the shell may form a dependency cycle (not checked); `TaskManagerToolRun` gained a field (`panel-records` is the only constructor); borrow of `self.instances` next to `self.runtime` in `apply_ledger_cancels` copies the existing pattern.
