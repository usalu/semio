# 📓️ S3-LOAD — whole-document load senders onto the stepped archive load

Owner: S3-LOAD (session 3, coordinator `⚪b7db773a…`), from S3-W2A's handoff `📓️api-stepped-document-load.md` §8 (2).
Scratch: `🗑️generated/s3-load/`. Nothing committed; no ticket or goal touched; no describe/activate/serve run.

Status: **done (11:27)**; kernel driver and TS twin (§7) verified; MCP and `🏃️run` compile/tests WRITTEN BUT UNVERIFIED (peer-red dependencies, §4).

## 1. Plan

1. One host-side driver for the archive protocol, written once (kernel channel, beside the wire types), replayed by a language-agnostic corpus.
2. MCP `PluginArtifactChannel`: `load_session_document` and `ExportMedia` load through it; progress on the activation scope
   (new phase `loading-document`), cancel = guest-side cancel with zero trace, faults decoded with their own codes.
3. `🏃️run` `SpaceRunner::compute_node`: admit in the first batch, poll per exchange, acknowledge, then the media/read batch;
   cancel through the run token; progress through an ephemeral observer the CLI prints.
4. Laws per sender, then the repo-wide list of remaining `LoadDocument` callers.

## 2. Progress log

- 06:57 kernel driver `DocumentArchiveLoadHost` (channel region `🔖️DocumentArchiveLoadHost`) + re-exports in `📡️spr/🦀️.rs`; corpus +
  schema `📡️spr/🧵️channel/🧬️fixtures/🗃️document-archive-load-host/`; Rust law in the channel unit tests; Python oracle
  `🧪️s3-load-archive-host.py` (jsonschema + independent model): **12 cases, 0 failures**; `--negative` exits 1 (2 failures).
- 07:00 kernel `cargo check --lib`: red, 11 errors, all in a peer's active ValueError/io wave (`🧬️semio/🦀️.rs:4`,
  `🏪️store/📜️space-history/…/🪶️sqlite/🚦️native`, `🏪️store/📦️codec/🪶️snapshot-capability`, `🚪️io/🦀️.rs:2654`), 0 in my regions.
- 07:03 MCP sources written (workspace, dispatch phase, schema, fixture, long-test law, quick-test command).
- 07:15 usage cut; 10:42 resumed: every edit above intact (the interrupted dangling-doc-line removal had landed).
- 10:45 `🏃️run` sources written (compute_node split, `drive_document_load`, progress observer, CLI reporter, FakeHost archive
  model, three laws).
- 10:5x corpus moved to the taxonomy open patterns (`🧫️fixtures/🧫️…`, `🧬️schema/🔣️…`); the first location (`🧬️fixtures/🗃️…`,
  created by me at 06:57) is deleted (rule 32, zero references left).

## 3. Changes

### 3.1 One host driver (kernel channel, region `🔖️DocumentArchiveLoadHost`)
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs`: transport-free `DocumentArchiveLoadHost` with `new(archive)`,
`step(seq) -> Send(AppCommand) | Finished(Ready|Cancelled|Fault(bytes))`, `answer(seq, frame) -> Ok(Some(status)) | Ok(None) |
Err(Refused(bytes) | Unanswered)`, `request_cancel()`, `operation()`. Admission seq = operation; a cancel before admission sends
nothing; after admission one `CancelDocumentArchiveLoad`, then polling until the guest's terminal (a refused cancel = lost race,
the next poll answers it); a cancelled terminal whose ack is refused is polled again; a refused ack of Ready/Fault is surfaced.
Re-exported from `📡️spr/🦀️.rs` (`DocumentArchiveLoadHost/Outcome/Refusal/Step`). TS twin: `AppChannelClient.loadDocumentArchive`.
- Corpus `📡️spr/🧵️channel/🧫️fixtures/🧫️document-archive-load-host/🔣️.json` (12 cases) + schema
  `📡️spr/🧵️channel/🧬️schema/🔣️document-archive-load-host/🔣️.json`; Rust law
  `the_document_archive_load_host_sends_the_scripted_commands_and_ends_in_the_scripted_outcome` (channel unit tests);
  ticket input `🧪️s3-load-archive-host.py` (third-party `jsonschema` + an independent Python model).

### 3.2 MCP (`🌉️mcp`)
- `🏠️workspace/🦀️.rs`: `PluginArtifactChannel::load_document_archive(instance, pack, spr, scope)` drives the host driver over
  `exchange_sequenced` (new; `exchange_one_real` = `take_seq` + it). Progress: every polled status →
  `scope.advance(ActivationPhase::LoadingDocument, completed/total)`. Cancel: scope cancel or `DOCUMENT_LOAD_WALL_BUDGET` (150 s)
  → guest-side cancel, polled to its restore → `job.cancelled` / `budget.exceeded`; a cancel that does not settle within one more
  `COMMAND_RESUME_WALL_BUDGET` discards the instance. Faults: the guest's own code via `decode_guest_fault`.
  - `load_session_document(.., scope)` and `seed_session_guest(.., scope)`: `activate` passes its scope (progress + cancel reach
    `inference_run`'s binding and `activate_plugin_session` jobs), `exchange` passes a detached scope.
  - `ExportMedia`: loads through the same path (detached scope: `artifact_export` is a synchronous tool with no progress channel)
    and drops the instance's session-document record first.
  - Docs that named `LoadDocument` now name the archive load (Stamped/admit_reply_frame/await_response/exchange/
    SessionDocumentPair/export_artifact_media). The only remaining mention is the exhaustive `app_command_seq_mut` arm.
- `🔀️dispatch/🦀️.rs`: `ActivationPhase::LoadingDocument` (`loading-document`) between `opening-guest` and `reading-document`
  (8 phases, positions k/8). Schema first: `🏠️workspace/🧬️schema/🔣️.json` `SessionActivationPhaseV1` enum + `SessionActivationPhasesV1`
  8 rows; fixture `💡️inference/🧫️fixtures/⏱️binding-cancellation-law.json` phase table (0, .125 … .875) and law text (the phase is
  reached only by a hub-bound session). Both law fixtures validate against the updated schemas (jsonschema, 0 errors).
- `🗿️artifact/🦀️.rs` module doc; `🏠️workspace/🧪️tests/🔬️quick/🦀️.rs` abandoned-exchange law builds its driver from `ReadDocument`.
- Law `🏠️workspace/🧪️tests/🔬️long/🦀️.rs` region `🔖️SteppedDocumentLoad`:
  `a_long_history_document_loads_through_polls_and_a_cancel_restores_the_previous_document` (real `🗒️note` guest, 240 committed
  transactions → fresh instance loads through ≥ 2 polls, monotonic progress ending at 1.0, head = source head; a cancel at the first
  polled status → `job.cancelled`, witness + `0@` history unchanged, the next load of the same instance reaches the source head).

### 3.3 `🏃️run`
- `🏃️run/🦀️.rs`: `compute_node` = batch 1 `SetMergePolicy, LoadConfig, LoadDocumentArchive` → `drive_document_load` (one
  command per exchange: polls, at most one cancel, ack) → batch 2 `MediaIn*, (MediaOut, MediaFingerprint)*, ReadDocument,
  ReadConfig`. A node with no stored document (`read_artifact` → empty pair) loads nothing and runs on its app's genesis document.
  Cancel: the run token (`ctx.cancel`) → guest-side cancel polled to restore → `RunError::Cancelled` (also for a guest-side cancel).
  Fault: `RunError::Host` with the guest's code (`dispatch_error_message`). Progress: ephemeral `NodeDocumentLoadProgress
  {node_id, completed, total}` to `SpaceRunner::with_document_load_progress(observer)` — never recorded in the run document.
  Helpers `node_reply`, `refuse_unsolicited`. Header/compute_node docs updated.
- `🏃️run/🏗️bootstrap/🦀️.rs`: the CLI installs `document_load_reporter()` — one `[os run] <node>: loading document c/t` line per
  crossed tenth.
- `🏃️run/🧪️tests/🔬️unit/🦀️.rs`: `FakeHost` models the guest's archive protocol (admission keyed by seq, pack+spr required,
  `fold_per_poll`, cancel → zero-trace restore, ack release, `document.loading` refusal of every other command while a load is live,
  `sent` log); `RecorderHost` loses its `LoadDocument` arm; the ordering law now expects `[1, 1, 2, 2]` (two exchanges per node);
  the upstream-edit law uses a real pair (`pack` + edited spr). New laws (region `🔖️SteppedDocumentLoad`):
  `a_long_history_node_document_loads_through_polls_before_any_import` (exact command script, 16 polls, progress monotonic to
  241/241, read-back = loaded document, node without document loads nothing),
  `cancelling_the_run_during_a_document_load_restores_the_node_document_and_stops_the_run`,
  `a_faulted_document_load_restores_the_node_document_and_surfaces_the_guest_code`.

### 3.4 §20.8 (pure commands)
No MCP or `🏃️run` call sends a stateless pure command with a document lane: the MCP `PureCommand` sends empty lanes and evaluates on
the live instance whose document came through the archive load (option (a), stateful). `🏃️run` sends no `PureCommand`. Nothing to
convert. The `inference_run` artifact binding injects `pack`/`spr` into a guest inference request body (not a `PureCommand`); if that
guest folds history, it is a §20.8 candidate for the inference owner.

### 3.5 Deleted (rule 32, created by me this session)
`📡️spr/🧵️channel/🧬️fixtures/🗃️document-archive-load-host/` (`🔣️.json`, `🧬️schema/🔣️.json`) — moved to the open-pattern paths above.

## 4. Verification (gated, foreground, one cargo at a time)

| Command | Result |
|---|---|
| `python3 T/🧪️s3-load-archive-host.py` | **12 cases, 0 failures**; `--negative` exits 1 (2 failures) |
| `cargo check -p semio-framework-os-kernel --lib` (10:46) | **Finished**, 392 warnings, none in my regions |
| `cargo test -p semio-framework-os-kernel --lib -- the_document_archive_load_host` (10:49) | **1 passed, 0 failed** |
| `cargo test -p semio-framework-os-kernel --lib -- os_spr::channel::` (after the corpus move) | **93 passed, 0 failed** |
| `bun ./📜️script.ts verify taxonomy report --scope` (both new dirs) | **clean ×2** |
| jsonschema: binding + create cancellation-law fixtures vs updated schemas | **0 errors ×2** |
| `cargo check -p semio-framework-os-run --lib --bins --tests` (10:55) | **red, 154 errors, all in the dependency `🔁️workflow/🗿️artifacts/🏃️run`** (peer: `semio_framework_ui_locale` import + private `LocalizedLabel`, `ValueError` trait signatures; last peer edit 09:05) — `🏃️run` itself not reached |
| `cargo check -p semio-framework-os-mcp --lib` (11:11) | **red, 42 errors, all peer IoError/ValueError wave**: `🏠️workspace/🦀️.rs` 402 (`ProbeOwnedRetirement::close_step`) and 3119–3155 (sqlite io region), `🏠️workspace/🪶️sqlite/🦀️.rs` ×21; 0 in my regions |

**WRITTEN BUT UNVERIFIED** (dependencies red): the MCP and `🏃️run` sources and laws. Owed once those crates are green:
1. `cargo check -p semio-framework-os-run --lib --bins --tests` then `cargo test -p semio-framework-os-run --lib`
   (private target `target-nde-s3-load`) — expect the three new laws plus the updated ordering/upstream laws green.
2. `cargo check -p semio-framework-os-mcp --lib --tests`, then
   `cargo test -p semio-framework-os-mcp --lib -- long::a_long_history_document_loads_through_polls quick::an_abandoned` and the
   `binding-cancellation-law` test target (8-phase table).

## 5. Remaining `LoadDocument` references (repo-wide `/usr/bin/grep -rn`, rs/ts/tsx/mts/js; git grep js/json/wit/py: none)

**Senders: none.** Every remaining reference is the variant itself, its codec, receivers, exhaustive arms, tests or docs — all for
the §20.7 bump wave:
- Variant + codec, `📡️spr/🧵️channel/🦀️.rs`: enum variant, paged field take/put 1978/1981/2007/2010, paged decoder 2154, fields 2347,
  encoder 3403, decoder tag 6 at 3791. Tests `📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs` 98, 632, 810.
- Guest receiver: `🔌️plugin/🦀️.rs:43798` (`protocol::AppCommand::LoadDocument` arm).
- Exhaustive seq arms: `🏃️run/🦀️.rs:2260` (`app_command_seq`), `🌉️mcp/🏠️workspace/🦀️.rs:2179` (`app_command_seq_mut`).
- TS twin `💻️os/🟦️.ts`: type 2617, tag 2896, encode 2959–2963, decode 3149–3153, `sendCommand` document cache 4055,
  `AppChannelClient.loadDocument` 4192–4194. Tests `💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts` 467, 665, 686 (hex `060101010102`),
  1336–1339, 1448, 1454 (document-cache law), 1486.
- Docs only: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs:603`, `🔌️plugin/🦀️.rs` 8315, 43449, `🔌️plugin/⚛️reactor/📸️checkpoint/🦀️.rs`
  8, 74, `🔌️plugin/🖥️host/🦀️.rs` 7423, 7451, `🏛️ShellHost/🟦️.tsx` 8726, 8811, `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:4733`.
- Not this variant: `Effect::LoadDocument` (guest→host effect) and the `🎠️kernel` effect enum.

## 6. Coordinator actions
1. §20.7 bump wave: delete the variant and every item in §5 (the TS document-cache law moves to `loadDocumentArchive`).
2. Re-run the owed commands in §4 once `🔁️workflow/🗿️artifacts/🏃️run` and the MCP io region are green (peer IoError/ValueError +
   locale waves).
3. S3-W2C: the wgpu `ProgramBridge` `load_app_document_archive` (Rust) has no cancel/progress and swallows a refused ack of a
   cancelled terminal; adopting `DocumentArchiveLoadHost` gives it both.
4. ~~TS twin diverges from the corpus~~ — DONE in §7 (coordinator RESUME 11:14, S3-LOAD owns the corpus).
5. S3-W2A (guest): the archive commit does not lift head-only mode (`time_travel.set_history_unavailable(false)` is only in
   `load_document_pack/text`); once those leave `PluginApp`, the archive Ready path must lift it.
6. No describe/activation is needed for these changes (host-side only; the guest is unchanged).

## 7. TS twin conforms to the corpus (coordinator RESUME 11:14, §6 item 4)

### 7.1 Changes
- `🧰️framework/🛍️products/💻️os/🟦️.ts` region `🗃️DocumentArchiveLoadHost` (new): `DocumentArchiveLoadHost` class, the exact twin of the Rust
  driver (`requestCancel`, `step(nextSeq)`, `answer(seq, frame)`, `operation`), types `DocumentArchiveLoadOutcome/Step/Answer`, and
  `documentArchiveLoadReplyV1` (the frame answering a seq: its refusal first, else `Done`/status).
  `AppChannelClient.loadDocumentArchive(archive, signal?, progress?)` now drives it — same public signature:
  - a signal aborted before admission sends nothing and rejects with `signal.reason` (was: admitted, then cancelled);
  - a refused cancel is the lost race: polling goes on, a `Ready` resolves (was: threw and left the operation unacknowledged);
  - every other path is unchanged (cancelled → `signal.reason` or `AbortError`, fault/refusal → the guest's own fault text,
    unanswered → typed error, a refused ack of a cancelled terminal → poll again after one macrotask, polls wait one macrotask).
- Both drivers now mint a sequence only for a command that is sent (`step(nextSeq)`, Rust `Send { seq, command }`): the TS client
  used to burn one sequence on the finishing step (caught by the existing `readDocumentArchive` law expecting `seq 4`). Rust hosts
  follow: MCP `load.step(|| self.take_seq())`, `🏃️run` `load.step(&mut next_seq)`; the Rust law mints through a closure and asserts a
  finished load mints none.
- New bun:test law `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️document-archive-load-host/🟦️.ts`: Ajv 2020 validates the corpus against its schema
  (third-party), then all 12 cases replay through `DocumentArchiveLoadHost` (commands, operations, statuses, outcome — the Rust
  law's assertions) and through `AppChannelClient.loadDocumentArchive` over a scripted channel (abort = the caller's signal,
  outcome = how the promise settles, `documentPack()` set only on ready). Registered as `test-channel-oracles`
  (`💻️os/📦️packages/🟦️typescript/📜️script.ts` `ChannelOraclesTestScript`, `📋️project.json` target, `.vscode/launch.json`
  `⚖️test-channel-oracles💻️os🟦️` after `test-store-oracles`, group `4_gate`, order 900.03555).
- `💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts`: the cancel law now pins both halves — a pre-admission abort sends nothing; an abort
  while the admission is answered sends `Load(1) → Cancel(2, op 1) → Poll(3, op 1) → Ack(4, op 1)`.

### 7.2 Verification
| Command | Result |
|---|---|
| `bun test /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️document-archive-load-host/🟦️.ts` (also via `bun ./📜️script.ts test-channel-oracles` and `bun nx run @semio-tech/framework-os:test-channel-oracles`) | **3 pass, 0 fail, 226 expect() calls**; nx target succeeded |
| negative control (scratch copy, corpus scripting the old "refused cancel throws" outcome) | **2 fail** (driver + client), 1 pass (schema) — the law catches the old behaviour |
| `python3 T/🧪️s3-load-archive-host.py` | **12 cases, 0 failures** |
| `cargo test -p semio-framework-os-kernel --lib -- os_spr::channel::` (after the minting change, 11:23) | **93 passed, 0 failed** |
| os vitest `bun ./📜️script.ts test quick` (`@semio-tech/framework-os`) | **476 passed (7 files)**; archive subset 6 passed |
| React `bun ./📜️script.ts test long --testNamePattern="…archive…"` (renderer-react) | **7 passed**: PluginRuntime `loadAppDocumentArchive` adapter (admit → 3 polls → ack, progress), surface-document archive gate ×3, wgpu package bridge archive transfer, time-travel whole-document load progress ×2 |
| `bun ./📜️script.ts typecheck` (os tsconfig) | 112 errors, **0 in `💻️os/🟦️.ts`, the new law or the backbone test** (the 2 in my law fixed; the rest are other owners' files) |
| `bun ./📜️script.ts verify taxonomy report --scope 💻️os/🧪️tests/🧪️document-archive-load-host` | **clean** |

### 7.3 React callers (S3-W2B's) — behaviour check
- `🔌️PluginRuntime` `loadAppDocumentArchive` forwards `(archive, signal, progress)` unchanged; its law passes.
- `🏛️ShellHost` `loadDocumentArchive`/`loadDocumentPair` (document transfer, cold attach, restore, import) await the promise; the
  Tasks-window notice uses `documentLoadCancelledV1(error, signal)`. With the twin: a pre-admission cancel still rejects with the
  signal's reason → `load-cancelled` as before, without a guest round trip; a cancel that lost the race now resolves, so the shell
  shows the document that really loaded instead of a `load-failed`/`load-cancelled` notice over a guest whose operation stayed
  unacknowledged.
- Not run: the ShellHost component suites beyond the patterns above (exhaustive level).

### 7.4 Coordinator actions (added)
- MCP and `🏃️run` hosts carry the `step(next_seq)` change too; still WRITTEN BUT UNVERIFIED for the same peer-red dependencies (§4).
- wgpu `ProgramBridge` (S3-W2C) can drive the same driver; its own loop still sends unconditionally and has no cancel.

Status: **done (11:27)** — every item assigned to S3-LOAD is written; the kernel driver, the TS twin and every runnable test are green;
the MCP and `🏃️run` senders still need their compile/test run once peers turn their dependencies green.

## 8. W-b of design §20.15 — producers supply composed children (coordinator RESUME 11:5x)

Formats from S3-AGNOSTIC: (1) serialize native carrier for a composed artifact = `encode_document_archive_bytes({parent HEAD pack,
empty spr, document_archive().members})`; (2) inference: one dependency per owned member, key `inference_child_dependency(slot,
child_id)` = `child:<slot>/<childId>`, value = the member's HEAD snapshot pack.

### 8.1 Census (12:00)
- **Inference requesters in production:** only the MCP gateway (`infer_real`, `dependencies: Vec::new()`). The TS host builds no
  inference request (the kernel TS `ArtifactInferenceRouter` only orders `dependsOn`); the hub's is GIS-only.
- **Host router** (`🖥️host/🦀️.rs` `ArtifactInferenceRouter`): replaced the requester's `dependencies` with the `dependsOn` results and sent
  dependency requests with none — composed children would never reach a guest. FIXED (8.2).
- **Guest validation blocks the key:** `validate_wire_request_resources` (`🔌️plugin/🦀️.rs` ≈2123) runs `ArtifactIdentity::parse` on
  every dependency key; `child:<slot>/<childId>` is not identity-shaped, so every composed request would be refused
  (`artifact-inference.dependencies`). Sent to S3-AGNOSTIC (owner of the key and the seam).
- **No requester holds child head packs:** MCP binds parent pack+spr; archive members are envelopes (initial pack + spr, a typed
  fold away). Proposed to S3-AGNOSTIC: guest `PluginApp::child_head_packs()` + host read `ReadChildHeads → ChildHeads` (S3-LOAD).
- **Serialize producers:** no host/shell producer hands a native payload to io in production — the shell's save is the recursive
  archive (members included), shell export is guest-internal (`SubmitMediaExport`/`MediaOut` → the app's export on the live
  snapshot), `IoRouter::run_io`/TS kernel `ioRun` have test callers only. The framework's one native-head-pack producer is the guest
  default `export_media("artifact:out")` (base64 head pack; `🏃️run` MediaOut and MCP `ExportMedia` read it) — asked S3-AGNOSTIC
  whether it emits the composed carrier (guest side, W-a region).

### 8.2 Host router keeps a composed document's children (DONE, verified)
`🔌️plugin/🖥️host/🦀️.rs`: `routed_inference_dependencies(requested, resolved)` — the requester's entries first, then the
`depends_on` results; `build_dependency_inference_request` carries the requester's entries. Law
`a_composed_documents_children_reach_every_routed_inference_request` (`🖥️host/🧪️tests/🔬️artifact-inference-router/🦀️.rs`).
- `cargo check -p semio-framework-plugin-host --lib --tests` (12:02): **Finished**, 0 errors.
- `cargo test -p semio-framework-plugin-host --lib -- artifact_inference_router` (12:06): **3 passed, 0 failed**.

## Session 4 — 2026-10-04

Successor S4-LOAD (Opus, coordinator `⚪487b04ad…`, fleet rules 1–38). Owns `🏃️run`, `🌉️mcp` (archive load + W-b), the kernel
`DocumentArchiveLoadHost` driver + TS twin, the `PluginApp` load surface (W2A-6). Scratch: `🗑️generated/s4-load/`. Private test target
`…/⚡️cache/cargo/target-nde-s4-load`. Status log newest first; sections S4.1+ below it.

### S4.0 Status log

- 02:13 started. Rule 34: owned files since §8.2 (10-03 12:06) — `🏃️run/🦀️.rs` + unit tests (22:32/22:35, peer value/DSL sweep),
  MCP workspace (10-04 00:30, peer), channel (02:17, S4-BUMP active in it), plugin runtime (01:31, peers). No half-finished S3-LOAD edit:
  every §3/§7/§8 change is on disk (`drive_document_load`, `DocumentArchiveLoadHost`, `routed_inference_dependencies`). Since S3 a peer
  landed `PluginApp::child_head_packs()` + `protocol::ChildHeadPackEntry` and the guest's `child:<slot>/<id>` key acceptance
  (`inference_child_dependency_parts` in `validate_wire_request_resources`), and moved 9 plugin-crate test sites onto
  `artifact_app_laws::load_document_text` (W2A-6 direction).
- 02:15 census (rg, tracked + untracked `.rs`): `load_document_pack/text(` **92 lines** — ✏️s plugin tests ~70 sites/46 files, `✏️s/🧑‍💻dev`
  2, hub `🌀️procedural`/`🀄️wfc` close-ladder + idle-turns 4 (`plugin_load_document_text`), plugin crate 9 already on the helper + builder
  contract 1, runtime 4 (two `consume_media` document branches = P5, `plugin_load_document_pack/text`). File:
  `🗑️generated/s4-load/census-0215.txt`.
- 02:20 coordinator decisions (via `main`): `ReadChildHeads{seq}` tag 42 → `ChildHeads{in_reply_to, entries}` frame 32 and
  `PureCommand{seq, command, head}` ride S4-BUMP's channel bump (BUMP owns channel/codec/TS twin/exhaustive arms/guest bridges; I own the
  guest answers, the host requesters, `hydrate_*_lane` removal); MediaIn whole-document → stepped archive load approved.
- 02:41 **wave 1** (T2/T3): `🧪️s4-load-sweep-document-loads.py` rewrote **72 call sites in 53 files** (`✏️s` plugins: stdio 21 files, raster 4,
  fem 4, reasoning 3, …; `✏️s/🧑‍💻dev/🧩️composition` 2) from `x.load_document_pack/text(` to
  `semio_framework_plugin::artifact_app_laws::load_document[_text](&mut x | x, ` (a `&mut` parameter receiver is passed as is); `--check`
  → 0 pending. Wave 1b: 7 stdio tests `match consume_media(…) { Ok(()) =>` → `Ok(_) =>`. Log: `🗑️generated/s4-load/sweep-run-1*.txt`.
- 02:46 kernel driver `DocumentArchiveLoadHost::admitted(operation)` (a load the guest admitted itself under the host's command sequence)
  + TS twin `DocumentArchiveLoadHost.admitted` + corpus field `admittedByGuest` (schema) + 2 cases (15 total) + Rust law + TS law + Python
  oracle. `python3 T/🧪️s3-load-archive-host.py` **15 cases, 0 failures** (red first: 3 failures before the model change); `--negative` 2
  failures; `bun test ./…/🧪️document-archive-load-host/🟦️.ts` **3 pass, 0 fail, 264 expects**.
- 02:54 `cargo check -p semio-framework-os-run --lib --bins --tests` (02:46–02:54) **RED, 11 errors, all in the peer dependency**
  `semio-framework-artifact-workflow-run` (`🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/📸️snapshot/🪶️sqlite/{💰️backing,📏️value}/🦀️.rs`, E0603 private
  `dsl::FieldValue/RecordValue/NativeDecodeControl/DslField/__rt`). Coordinator GO → converted both files to the peer's own pattern
  (`semio_framework_dsl_record::{FieldValue, RecordValue, DslField, __rt::DecodedFieldOwner}`, `semio_framework_value::NativeDecodeControl`),
  11 replacements.
- 02:5x `🏃️run`: a structured (`Document` wire) media input closes its own import batch (`import_media_batch`); a whole document answered
  `DocumentArchiveLoad` is driven by `drive_document_load(…, DocumentArchiveLoadHost::admitted(seq), None, …)` before any later import,
  export or read; `expect_done` became a free fn. FakeHost models the guest (`FAKE_DOCUMENT_SCHEMA` input → load keyed by the import's seq);
  2 new laws (`a_whole_document_input_loads_through_polls_before_any_export_or_read`,
  `cancelling_the_run_during_a_whole_document_input_restores_the_node_document`).
- 03:00 plugin P5 + additive helpers (my transient red ~03:00 between sequential Edits, closed 03:01 — rule 39 adopted): `MediaConsumption
  {Applied, DocumentLoad(DocumentArchivePack)}`; `PluginApp::consume_media -> Result<MediaConsumption, _>` (trait default + VcsArtifactApp:
  own-schema `Document` wire → stamped archive, no fold); `PluginApp::document_load_archive(&files)` (stamp with the live identity);
  `plugin_runtime::plugin_document_load_archive`; `plugin_consume_media(…, operation, …) -> Option<DocumentArchiveLoadStatus>` admits the
  load under the MediaIn's seq; MediaIn arm answers `DocumentArchiveLoad{Pending}`; law helpers `artifact_app_laws::plugin_load_document` +
  generic `plugin_load_document_text::<PA, S, Mutation>` (runtime level) sharing `law_load_running/law_load_outcome` with `load_document`.
  Hub tests (`🌀️procedural` close-ladder ×2 + idle-turns, `🀄️wfc` close-ladder → pack round trip) moved onto them.
- 03:30 `cargo check -p semio-framework-plugin --lib` (03:01–03:30): my P5 changes clean; 1 peer error `🔌️plugin/⏪️time-travel/🦀️.rs:4154`
  (`TreeItemBuilder::selected`, W1E-2 in flight) → `main`.
- 03:31 **wave 2** (`🧪️s4-load-wave2-plugin-surface.py`, one atomic write, rule 39): `PluginApp::load_document_pack/text` deleted (trait +
  `VcsArtifactApp` impl), `plugin_runtime::plugin_load_document_text` deleted (the last `resolve_ready(load_document_*)` panic path; the
  `AppCommand::LoadDocument` arm + `plugin_load_document_pack` went with S4-BUMP's bump), `hydrate_document/config/draft_lane` → one
  `PluginApp::hydrate_pure_head(head)` (§20.8 head-only, driven by `drive_self_waking_ready` in the PureCommand arm), docs renamed, builder
  contract test onto `artifact_app_laws::load_document`. **`cargo check -p semio-framework-plugin --lib` 03:31–03:52 exit 0, 0 errors
  ("PLUGIN GREEN 03:52")**. Census after: `load_document_pack/text(` on `PluginApp` = **0** callers repo-wide (rg).
- 03:3x W-b (d) MCP `infer_real`: `dependencies` = `inference_child_dependencies(artifact_id)` when the inference binds an artifact — the session
  guest holding it answers `ReadChildHeads` (S4-BUMP's frame, tag 42 → `ChildHeads` 32) → `child:<slot>/<childId>` → head pack; none when no
  guest holds the artifact. Guest law `a_composed_documents_child_heads_answer_each_owned_childs_current_head_snapshot` (builder contract).
- 03:43 `bun ./📜️script.ts test-channel-oracles` (os package) **3 pass / 0 fail / 264 expects**; `bun ./📜️script.ts test quick` **507 passed**,
  but 1 file FAILED "No test suite found" = `💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts` (S4-BUMP re-sealing it mid-bump, not mine); React
  `test long --testNamePattern=archive` **5 passed, 3 files**.
- 03:57 `cargo test -p semio-framework-os-kernel --lib -- os_spr::channel::` **89 passed / 1 failed**: my `the_document_archive_load_host…`
  law **ok** (15 cases); the failure `document_identity_wire_matches_the_language_neutral_v20_fixture` (`left 20 right 21`) is S4-BUMP's
  channel-version bump mid-flight (fixture re-seal), not mine.
- 04:08 `cargo check -p semio-framework-os-run --lib --bins --tests` **exit 0** (after the workflow-run sqlite conversion).
- 04:26 `cargo test -p semio-framework-os-run --lib` **25 passed / 1 failed**: all 5 stepped-load laws green (incl. the 2 new MediaIn laws);
  the failure `note_plugin_manifest_loads_from_its_committed_descriptor` read the pre-restructure path `✏️s/🔌️plugins/🗒️note/🛂️.descriptor.semio`
  (rule 22) → 06:46 fixed to `🌎️hub/🧩️compositions/🗒️note/🛂️.descriptor.semio` + `…/dist/<profile>/semio_hub_note.wasm` (re-run owed).
- 04:15 usage cut; 06:45 resumed (coordinator RESUME): all edits above on disk.
- 06:48 **wave 3** W-b (c) (`🧪️s4-load-wave3-composed-carrier.py`, one atomic write): `VcsArtifactApp::export_media("artifact:out")` of a
  document that owns children answers the composed carrier (`encode_document_archive_bytes({parent HEAD pack, empty spr, members})`, base64,
  `Structured{schema: DOCUMENT_SCHEMA}`, media type from `A::io()`), generation-fenced; a document without children keeps the app's own
  export (head pack). The member roster is one helper `archive_member_entries` shared with `document_archive`. Laws (builder contract):
  `the_artifact_out_export_of_a_composed_document_is_the_composed_carrier`,
  `a_whole_document_media_import_is_a_stepped_archive_load_and_never_folds_inside_the_call` (P5: consume answers the consumer-stamped archive,
  document untouched until admit/poll/ack lands the source head under the consumer's identity; foreign schema → `SchemaMismatch`).
- 06:5x MCP: `child_head_dependencies(entries)` (pure, `child:<slot>/<childId>` → head pack) + quick law
  `a_composed_documents_child_heads_become_child_keyed_inference_dependencies` (keys parse back through `inference_child_dependency_parts`).
- 07:15 `cargo check -p semio-framework-plugin --lib` (re-run after the 06:48 run was SIGKILLed by the deadlock breaker in a flock wait):
  **Finished, 0 errors**, 258 warnings, none in the wave-3 regions → PLUGIN GREEN 07:15 with wave 3.
- 07:16 `cargo check -p semio-framework-os-mcp --lib --tests` running.
- 07:43 the 07:17 MCP check was SIGKILLed by the deadlock breaker (25 min flock wait); 07:47 re-run (rule-42 gate): **RED, 155 errors, one
  peer cause** — `💻️os/📦️packages/🦀️rust/🦀️.rs` (05:56) made the kernel root's `DslValue/ToValue/FromValue/ValueError/Number` (+ derives)
  private. MCP is my WP → converted 11 files' `use semio_framework_os_kernel::{…}` → `use semio_framework_value::{…}` (the value crate
  re-exports the derives) + 6 `store::DslValue/FromValue/ToValue` in the quick tests (peer pattern, `main` told). 08:02 re-check: 3 left, the
  old S3 io-region breaks in `🏠️workspace/🦀️.rs` (`ProbeOwnedRetirement::close_step` → `ValueError`; two `control.checkpoint(..)?` into
  `IoError` → `.map_err(IoError::from_value_error)?`) — fixed.
- 08:0x RULE 43 (checks only, no `--tests`): stopped my queued `--tests` check before it started; `cargo check -p semio-framework-os-mcp -p
  semio-framework-os-run --lib` running. Tests **OWED (rule 43)**.
- 08:44 `cargo check -p semio-framework-os-mcp -p semio-framework-os-run --lib` (rule-42 gate) **exit 0, 0 errors** (os-run 1 warning: unused
  extern crate in the package root; MCP 32 pre-existing warnings, none in my regions).
- 09:01 `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2` **exit 0, 0 errors** (12 m 47 s; rule 41 wasm half).
- 08:55 usage cut; 11:35 resumed: all edits intact (symbol census), sync-load callers repo-wide = 0 (2 stale test doc comments in writer/note
  swept tests renamed to the stepped helper; the wgpu shell's historical doc mention is S4-WGPU's).
- 11:36 TS re-run: `bun ./📜️script.ts test-channel-oracles` **3/0, 264 expects**; `test quick` **506 passed / 2 failed + 1 file "No test suite
  found"** — all three S4-BUMP's in-flight channel work (`🧪️backbone-envelope-io/🟦️.ts` empty while re-sealing; `🎭️actor-transport` ×2
  `reactor.channelVersion is not a function` in the generated bridge), none in my files.

### S4.1 Finding (not fixed, design question): a whole document cannot cross a `🏃️run` edge today
`produce_media("artifact:out")` answers `MediaWireFormat::Document` with BINARY `encode_document_pack_bytes(pack, spr)`, but `🏃️run`'s
`media_from_document` maps a `Document` wire to `MediaPayload::Structured { json: String::from_utf8(data)? }` — binary pack bytes are not UTF-8,
so every workflow edge from the implicit `artifact:out` port fails with a UTF-8 host error before the downstream node is reached (pre-existing
since S3; P5's stepped load is therefore proven by the FakeHost laws, not by a real component edge). `MediaPayload` has no lossless carrier for a
binary document (Structured = text, Binary = foreign format via the blob store). Options for the owner of the framework media model: (a) a
`MediaPayload::Document { schema, blob_hash }` variant (schema-first, both runners + TS twin), or (b) `artifact:out` answers the framework's own
convention `Structured { schema: DOCUMENT_SCHEMA, json: base64(carrier) }` (what every app's `export_media("artifact:out")` already returns) and
`consume_media` accepts that base64 form for its own schema. Recommendation: (b) — no new payload variant, one convention for export and
transfer, and the composed carrier (W-b c) rides it unchanged. Coordinator decision needed (cross-WP: framework manifest media model).

### S4.2 Whole document on a media edge — coordinator GO 11:4x, branch (b) taken (wave 4, 11:44)
**Why (b) and not a raw-byte variant:** the channel wire (`MediaWireFormat::Document` + `data` bytes) already carries bytes, but the run edge's
host representation `MediaPayload` has no lossless byte carrier for a DOCUMENT: `Structured{json: String}` is text and `Binary{format_kind,
blob_hash}` is a registered foreign file format (`media_from_document` resolves its MIME through `format_descriptor`, which a document
schema is not). A new `MediaPayload` variant would change the framework media model under the rule-41 ABI freeze. So one convention:
`pk:` base64 text (`pack_value_to_base64`) — exactly the `Structured{schema: DOCUMENT_SCHEMA, json: pk:…}` form every app's
`export_media("artifact:out")` already returns.
- `🧪️s4-load-wave4-document-carrier.py` (one atomic write per file): plugin `whole_document_media(port, schema, archive)` /
  `whole_document_archive(data)` (`encode/decode_document_archive_bytes` of the RECURSIVE archive: parent pack + `.spr` + owned members),
  `produce_media` (trait default + `VcsArtifactApp` fallback) answers `whole_document_media(document_archive())`; `consume_media` (both)
  decodes it for its own schema into `MediaConsumption::DocumentLoad(archive)` verbatim (the source document and its members — the same
  identity semantics the deleted sync `load_document_pack` had; `document_load_archive` stays the `Effect::LoadDocument` stamp the law
  helpers use); `media_artifact_fault` maps `SchemaMismatch` to the framework code **`plugin.media.schema-mismatch`** (`{found}`,
  `{expected}`), used by `plugin_consume_media`. Kernel framework notice row (Rust table 12→13 + TS twin + fixture; schema-valid, rows
  equal in all three — python cross-check) en "This input is a {found} document, but only {expected} documents can be loaded here." / de
  "Diese Eingabe ist ein {found}-Dokument, hier lassen sich aber nur {expected}-Dokumente laden." `🏃️run` needs no code change: its
  `Structured` text round trip is now lossless.
- Laws (compile/run OWED, rule 43): kernel `a_foreign_document_schema_on_a_media_edge_names_both_schemas_in_both_locales`; contract
  `a_whole_document_media_import_is_a_stepped_archive_load_and_never_folds_inside_the_call` (updated: `pk:` text, source identity verbatim,
  refusal → code + de notice naming both schemas), new `a_composed_document_crosses_a_media_edge_with_its_children` (UTF-8 host round trip,
  member adopted, child count 7); run `a_whole_document_carrier_crosses_a_run_edge_byte_for_byte`.
- S4.1's finding is thereby resolved in source (a real component edge still needs the coordinator's rebuild to prove).
- 12:11 `cargo check -p semio-framework -p semio-framework-plugin --lib` (wave 4; 27 min incl. lock waits) **exit 0, 0 errors**, no warning in
  the wave-4 regions → FW + PLUGIN GREEN 12:11.
- 12:16 os-run + os-mcp `--lib` re-check against wave 4 started; 12:24 stopped by me (SIGTERM, children gone) on RULE 44 (cargo freeze)
  → **OWED (rule 44)**.

### S4.3 Session-4 summary (12:25)

**Done (source):** W2A-6 complete — P4 (`PluginApp::load_document_pack/text` + `plugin_load_document_text` deleted), P5 (`MediaIn` whole
document → stepped archive load via `MediaConsumption` + `DocumentArchiveLoadHost::admitted`), P6 (`hydrate_pure_head`), T1–T3 (72 ✏️s
sites + builder contract + 4 hub sites onto `artifact_app_laws::load_document[_text]` / `plugin_load_document[_text]`); 0 sync-load callers
repo-wide. D2 W-b complete — (b) guest `child_head_packs` answered on S4-BUMP's `ReadChildHeads`, MCP requester; (c) composed
`artifact:out` export carrier (head + members) and the one whole-document media carrier (wave 4); (d) `infer_real` sends
`child:<slot>/<childId>` dependencies. Peer fallout converted on coordinator GO: workflow-run sqlite owners (11 imports), MCP value imports
(11 files + quick tests) and 3 MCP io-region errors; os-run note-descriptor test path (rule 22 layout).

**Verification (counts):**

| Command | Result |
|---|---|
| `python3 T/🧪️s3-load-archive-host.py` (+ `--negative`) | 15 cases / 0 failures (negative: 2 failures, exit 1) |
| `bun test ./…/💻️os/🧪️tests/🧪️document-archive-load-host/🟦️.ts`; `bun ./📜️script.ts test-channel-oracles` (×2) | 3 pass / 0 fail / 264 expects |
| `bun ./📜️script.ts test quick` (os, 11:36) | 506 pass / 2 fail + 1 file "no suite" — all S4-BUMP's in-flight bridge/re-seal |
| React `test long --testNamePattern=archive` | 5 pass (3 files) |
| `cargo check -p semio-framework-plugin --lib` (03:52, 07:15) / `--target wasm32-wasip2` (09:01) | exit 0 / exit 0 |
| `cargo check -p semio-framework -p semio-framework-plugin --lib` (12:11, wave 4) | exit 0, 0 errors |
| `cargo check -p semio-framework-os-run --lib --bins --tests` (04:08) | exit 0 |
| `cargo check -p semio-framework-os-mcp -p semio-framework-os-run --lib` (08:44) | exit 0, 0 errors |
| `cargo test -p semio-framework-os-kernel --lib -- os_spr::channel::` (03:57) | 89 pass / 1 fail (S4-BUMP v20→21 fixture); host-driver law ok |
| `cargo test -p semio-framework-os-run --lib` (04:26) | 25 pass / 1 fail (stale note-descriptor path, fixed 06:46) |
| python cross-check framework notices (Rust table = TS twin = fixture, schema-valid) | 13 = 13 = 13, valid |

**OWED (rules 43/44 — checks/tests frozen):** `cargo check -p semio-framework-os-run -p semio-framework-os-mcp --lib` (after wave 4);
`cargo check -p semio-framework-plugin --lib --tests`; `cargo test -p semio-framework-plugin --lib -- a_composed_documents_child_heads
the_artifact_out_export_of_a_composed_document a_whole_document_media_import a_composed_document_crosses_a_media_edge`;
`cargo test -p semio-framework --lib -- framework_notices a_foreign_document_schema`; `cargo test -p semio-framework-os-run --lib` (27 laws);
`cargo check -p semio-framework-os-mcp --lib --tests` then `cargo test -p semio-framework-os-mcp --lib -- long::a_long_history_document_loads_through_polls
quick::an_abandoned quick::a_composed_documents_child_heads` + `--test binding_cancellation_law` (long law needs the rebuilt note component,
channel v21); kernel framework-notices TS twin vitest (needs `SEMIO_VITEST_POLICY`, run via the launch row); `cargo check --manifest-path ✏️s/Cargo.toml
--tests` of the 20 swept plugin crates and `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-procedural -p semio-hub-wfc --tests`.

**Coordinator actions:** (1) rebuild + describe of every component rides S4-BUMP's v21 wave anyway — after it, the MCP long law and a real
`🏃️run` `artifact:out` edge can prove P5/W-b live; (2) no descriptor/schema regeneration for my changes (no verb/manifest change; the new
framework notice row is a kernel table + fixture, not a generated artifact); (3) the stale-doc mention in the wgpu shell
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` ≈26530, "deleted with `PluginApp::load_document`") is historical — S4-WGPU may drop it.

**Files changed this session (mine):** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (regions: ArtifactAppLaws helpers,
MediaConsumption/carrier/fault helpers, PluginApp load surface, VcsArtifactApp archive member/composed carrier/consume/produce, runtime
consume/MediaIn/PureCommand arms); `…/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`; `…/📡️spr/🧵️channel/🦀️.rs`
(`DocumentArchiveLoadHost::admitted`), its corpus, schema and unit-test law; `🧰️framework/🛍️products/💻️os/🟦️.ts` (TS twin `admitted`) + its law;
`…/🏃️run/🦀️.rs` + unit tests; `…/🌉️mcp/🏠️workspace/🦀️.rs` + quick tests + 10 MCP module import lines; `…/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/📸️snapshot/🪶️sqlite/{💰️backing,📏️value}/🦀️.rs`;
`🧰️framework/🔨️modules/🎠️kernel/{🦀️.rs,🟦️.ts,🧫️fixtures/🧫️framework-notices/🔣️.json,🧪️tests/🧪️framework-notices/🦀️.rs}`; 53 ✏️s test files (wave 1/1b) + writer/note
doc comments; hub `🌀️procedural` close-ladder/idle-turns, `🀄️wfc` close-ladder tests; ticket inputs `🧪️s4-load-sweep-document-loads.py`,
`🧪️s4-load-wave2-plugin-surface.py`, `🧪️s4-load-wave3-composed-carrier.py`, `🧪️s4-load-wave4-document-carrier.py`, `🧪️s3-load-archive-host.py`;
docs `📓️api-stepped-document-load.md` §9, `📓️w2a-sync-load-paths.md` §4. **Deleted:** none (only code inside my regions).

### S4.4 AUDIT-TOOLS items F11/F12/F16/F17 (coordinator RESUME ~12:40) — PARKED, analysis done, NO patch staged, NO file changed
Parked on the coordinator's PARK NOW. Nothing was written to any source file or staged under `🗑️generated/s4-load/` for these items.
Under rules 44/45 the next step is a staged script `🗑️generated/s4-load/s4-load-wave5-audit.py` (same anchor/idempotence pattern as
`🧪️s4-load-wave4-document-carrier.py`), landed together with a gated `cargo check -p semio-framework -p semio-framework-plugin
-p semio-framework-plugin-host -p semio-framework-os-mcp --lib` at "CARGO OPEN". The findings and design below are verified by reading the code.

- **F12 (do first; it shrinks F11).** `composed_artifact_media` and its `export_media("artifact:out")` override (`PLG/🦀️.rs` ≈24305 and ≈35931,
  `if port == "artifact:out" && !self.children.is_empty()`) are dead on the wire. `produce_media` never calls `export_media` for
  `artifact:out`, and no runtime path calls `PluginApp::export_media` (rg: only `produce_media` for other ports + my law). Plan: delete both
  plus the law `the_artifact_out_export_of_a_composed_document_is_the_composed_carrier`. The ONE carrier is `produce_media`'s
  `whole_document_media(document_archive())`: recursive archive WITH parent `.spr`, loadable by `begin_document_archive_load`. Tell S4-AGNOSTIC
  that W-a serializer entries receive the archive with a non-empty parent `.spr` and must fold it (`parse_document_pack(..).into_snapshot()`,
  exactly as `ArchiveChildren` folds members). On the media wire it is `pk:` text; as an io native payload it is the raw archive bytes
  (first byte 0x01). `archive_member_entries` stays (used by `document_archive`).
- **F16.** Name the refusals of `begin_document_archive_load` (`PLG` ≈35359–35398; today all are `plugin_sdk_fault` = `plugin.internal`):
  - trait default → `plugin.document-load.unavailable`;
  - members > 1024 → `…too-many-members` `{count}{maximum}`;
  - payload bytes > `DOCUMENT_ARCHIVE_MAXIMUM_BYTES` → `…too-large` `{bytes}{maximum}`;
  - slot live/occupied → `…busy`;
  - empty pack/spr → `…incomplete`;
  - member invalid → `…member-invalid` `{ordinal}` (1-based);
  - `RetainedHistoryDecode` error → `…history-invalid`.

  Implementation:
  - Add one helper `document_load_refusal(code, message, params)` (origin Framework) beside `MEDIA_SCHEMA_MISMATCH_CODE`, plus consts.
  - Add en+de rows (same placeholders in both locales; the schema pattern admits `plugin.document-load.*`) to the kernel
    `FRAMEWORK_FAULT_NOTICE_LABELS` (13 → 20), the TS twin and the fixture. The existing mirror laws cover the rows.
  - Add a contract law with one malformed archive per code → `fault.code`.
- **F11.** Finding: a single `MediaOut` reply cannot be stepped. `plugin_produce_media` drives `produce_media` with `resolve_ready`, which
  panics on any `yield_once`, and the reply is one `AppFrame::Media{data}`. Yields between members would be fake stepping. So:
  - (a) Staged now: bound the carrier with the SAME admission rule as the load, (payload bytes ≤ `DOCUMENT_ARCHIVE_MAXIMUM_BYTES`,
    members ≤ 1024). Check before encoding. Refuse with F16's `plugin.document-load.too-large` / `too-many-members` through a new
    `MediaArtifactError::Refused(Fault)`. `media_artifact_fault` passes `Refused`/`Import` faults through unchanged.
    `plugin_produce_media` maps errors with `media_artifact_fault` instead of `plugin_internal_fault`. Add a law that calls
    `whole_document_media` on a synthetic oversized archive.
  - (b) Proposal, needs a coordinator decision + S4-RUNTIME (tool-run): real multi-turn stepping with progress and cancel means routing
    `artifact:out` through the existing stepped media-export job (`SubmitMediaExport`/`PollMediaExport`/`TakeMediaExportChunk`).
    That needs:
    - a framework-owned `export-media:artifact:out` job factory, admitted by `submit_owned_media_export`, which today requires an
      `AppOwned` proof (≈29943);
    - job steps of one member envelope each, then ≤48 KiB base64 chunks into `output_chunks`;
    - `🏃️run` and MCP `ExportMedia` switching `artifact:out` to submit → poll → take chunks.
- **F17.**
  - There is no declared child read today: `ContributedInferenceMetadata.depends_on` lists sibling inference schemas. A new metadata field
    would change the descriptor schema (rule 41 ABI freeze).
  - Design without a schema change: an inference declares `depends_on` entry `child:<slot>`. New plugin helpers
    `inference_child_slot_dependency(slot)` / `inference_child_slot(entry)` go next to `inference_child_dependency`.
  - The host router `🖥️host/🦀️.rs` (`infer_with_visited` ≈6910 dependency loop and `validate_inference_dependency_graph` adjacency
    ≈6960) skips `child:` entries; today it would refuse them as unregistered schemas.
  - MCP `infer_real` (≈1691) asks for child heads only when `declared.depends_on` has `child:` entries, and filters
    `child_head_dependencies(entries, slots)` to those slots.
  - Tie to the parent: on the same `&mut` channel, `ReadDocument` then `ReadChildHeads` on the session guest. The `(pack, spr)` answer must
    byte-equal the bound `command.artifact_document`. Both come from the same live guest printer: `read_artifact_bytes` reads the live
    session guest (≈4444), so equality is exact, not spurious. On mismatch, refuse `inference.children-stale`. The keys use the registry
    child ids, which composition validation (`insert_validated_owned`) ties to the parent's declared children.
  - Update the quick law for slot filtering and add a router law that a `child:` dependsOn entry is not routed.
- **Order after PARK:** F12 → F16 → F11a → F17 (one wave each: rule 39, staged until CARGO OPEN); F11b only on a coordinator decision.
