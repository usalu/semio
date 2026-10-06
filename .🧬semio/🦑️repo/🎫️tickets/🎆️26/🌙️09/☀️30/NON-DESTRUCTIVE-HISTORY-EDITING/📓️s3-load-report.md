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

## Session 5 — 2026-10-05

Successor S5-LOAD (Opus), coordinator `⚪3f26aaa1…`. Brief: P1 the folder reload route as a law before the live probe's batch B,
P2 owed checks, P3 wave 5 (F12, F16, F17, audit F1 caller half), P4 design §21.6. Scratch: `🗑️generated/s5-load/`.
Repair-first (rule 46): S4.4 left no half-finished edit (no file changed, nothing staged; `git diff` of my trees shows only
peers' session-5 landings). Rule 55 (build gate v4, `CARGO_BUILD_JOBS=3`) adopted at 01:16.

### S5.0 Status log (kept current)

- 01:13–01:33 read rules 1–55, resume §0/§1.19/§2/§3.2/§5, design §20–§22, e2e R2-2/R2-4, audit clause 9 + gap 3, W2-B follow-up 3.
  Traced the route in source (S5.1). Wrote the P1 law, its fixture, schema and TS twin under `🗑️generated/s5-load/wave-p1/`.
  TS twin RAN from staging: `bun 🗑️generated/s5-load/run-twin.ts <root>` → `folder-reload-route-oracle steps=16`
  (Ajv 2020-12 strict valid, 4 hostile rows refused, derivation == fixture); 6/6 mutated expectations killed.
  Landing lock held by S5-TOOLS (01:23) → queued. Rust law NOT yet compiled.

### S5.1 P1 — the folder reload route, read from the code (2026-10-05 01:3x)

**The route, end to end (file:line as read today).**
1. Attach: React `openSyncTarget` (`🏛️ShellHost/🟦️.tsx` ≈7825) reads the program's identity (`readAppDocumentIdentity` →
   `AppCommand::ReadDocumentIdentity` → `PluginApp::document_identity`, `PLG` ≈45375/35595 = the store envelope id) and
   addresses the folder document by it (`syncAttachDocumentIdV1`); `openDocument` posts the worker `open` with a
   `persistedLocalOnly` folder binding and binds the document port (`bindDocumentBackbone` ≈3208).
2. The port bind is a HOT attach (`plugin_handle_document_backbone_binding` → `attach_hot_backbone`, `PLG` ≈43411): nothing
   already in the log is announced. Consequences: a folder holds nothing until the first published batch, a fresh program
   never overwrites its folder at bind time, and a program that only receives never writes its folder.
3. Every published batch: `send:` posts `documentBackbone` to the worker and runs `archivePersistence` (≈7614):
   `readAppDocumentArchive` → `AppCommand::ReadDocumentArchive` → `PluginApp::document_archive` (parent pack + `.spr` +
   members, generation-fenced) → `localDocumentArchive` → worker `writeFolder` (PUT, remembers `folderArchive`).
   Admission in the worker (`🏪️store/👷️worker/🟦️.ts` ≈6997) and in the Rust actor (`🏪️store/🔄️sync/🦀️.rs` ≈2655): every
   envelope's `document_id` must equal the bound document id, else `local.backbone-scope-mismatch`.
4. Re-attach after a reload: the worker's bootstrap read (`pollFolderOnce` ≈4128) emits `documentArchiveReplaced` unless
   the bytes are its own echo; ShellHost (≈3915) runs `restoreDocumentArchiveV1`: retire the port → the stepped load
   (`loadAppDocumentArchive` = `AppChannelClient.loadDocumentArchive`, Rust twin `DocumentArchiveLoadHost`: admit → poll →
   acknowledge) → bind the port again → `refreshHistorySnapshot` (`AppCommand::ReadHistory` → `history_snapshot`).
5. In the guest the load commits through `begin_document_archive_load` (`PLG` ≈35503) and every whole-document replacement
   calls `retire_displaced_document_rows` (`⏪️time-travel/🦀️.rs` ≈3271), which also clears the O(change) backfill mark.

**The W2-B fixes for R2-2 / R2-4, as found on disk (`📓️w2-b-report.md` Follow-up 3).**
| Fix | Where it is today | What it changed |
|---|---|---|
| R2-4 identity | `openSyncTarget` + `syncAttachDocumentIdV1` (`🛠️ShellHelpers/🟦️.tsx` ≈3600), plugin handle `readAppDocumentIdentity` | the folder document is addressed by the program's own store id, not `${pluginId}-${instanceId}`; `syncDocumentId` is gone |
| R2-4 admission | worker `handleLocalMsg` `documentBackbone` arm (≈6992) | the canonical-pair precondition applies to hub documents only; a folder batch is admitted when its ids match |
| own-write echo | worker `folderArchive` (≈578, 4138, 4213) | a read that returns the document's own write emits nothing |
| hydration | `restoreDocumentArchiveV1` (`🛂️admission/📄️document/🟦️.ts`), used at ShellHost ≈3314/3920 | load → re-read history → refresh every surface |
| R2-2 displaced rows | `retire_displaced_document_rows` after the archive commit, `hydrate_pure_head` and the load legs (`PLG` ≈26275, 28818, 28866, 35358) | rows of the replaced document leave the command log; the loaded document's rows backfill |
| R2-2 labels | `Edit.verb` + `backfilled_edit_label` (`PLG` ≈12282, 27814) | a row label reads persisted facts only |
All six are present on disk today (the 10-04 stash incident did not revert them). S5-STORE re-ran the worker law at 01:03:
`folder archive restore` 5 passed.

**What the existing laws do NOT cover (the gap the probe fell through).** The worker law drives a fake program whose archive
is `JSON.stringify({positions, rows})` — it proves transport, not that the real archive carries history. The Rust law
`a_document_archive_round_trip_lists_every_history_row_of_its_source` drives raw store commands, stamps the archive with
the target's identity (the `Effect::LoadDocument` path, not the folder read-back, which passes `ReadDocumentArchive` bytes
verbatim), has no bound port, no time-travel session, no warning, no second program and no edit after the reload.

**Layer split agreed through `main` (one law per layer).** STORE: store/`.spr`/vcs reload (rows, supersedes, alternatives,
`REC_VIEWER`) and the folder transport (Rust actor, TS worker). LOAD (this WP): the route on the program side of the channel.

**The new law** `…::time_travel_tests::folder_reload_route::a_folder_bound_document_reloads_with_its_whole_history_and_each_programs_viewed_alternative`
- Files (staged, landing queued): `🔌️plugin/🧪️tests/🧪️folder-reload-route/{🦀️.rs,🟦️.ts}`,
  `🔌️plugin/🧫️fixtures/🧫️folder-reload-route/{🔣️.json,🧬️schema/🔣️.json}`; registration: one `#[path]` child module at the end
  of `🧪️tests/🧪️time-travel/🦀️.rs` (it reuses that harness; S5-TOOLS registered `gesture_laws` the same way) and one oracle
  row in `🔌️plugin/📦️packages/🦀️rust/📜️script.ts`.
- Sequence (fixture `steps`): author bound to its folder → 3 edits → history edit of the first (`begin` → `input` → `accept`
  → replay with a `mutation.no-op` warning downstream) → `finalize` → overwrite → 4th edit → history edit of it → finalize as
  the new alternative "Edited history"; a reader bound to its own folder receives every batch and edits once on the main line.
- Asserted: every published envelope names `document_identity()` before and after the reload; a hot bind publishes nothing;
  one edit and one finalize leave as one batch each; before the reload both programs match the fixture (head, viewed
  alternative, alternatives listed, superseded positions; author: warnings by mutation, both history-edit rows en/de, 4 edit
  rows); after detach → fresh program holding an example → `DocumentArchiveLoadHost` → the view (document rows + history-edit
  rows with ids, kinds, labels en/de, applied, per-mutation superseded/withdrawn/editable/worst/codes; head; viewed
  alternative; alternatives) equals the saved one for both programs; the reloaded author edits on, that batch is admitted and
  its row appears. Findings are collected and reported together (one test build per run).
- Not asserted here (other layer / other owner): the actor's echo and read-back (STORE), `seq` numbers and session-only flags
  (`introduced`, `pending`, `edited`), rows naming neither an edit nor a transition (shell/config rows are not document history).

### S5.2 Staged waves (2026-10-05 01:42) — one script, anchors verified against the live tree

`python3 T/🧪️s5-load-waves.py <p1|f1|f12> <check|land|restore>` (whole-file writes from unique anchors; `land` keeps the
replaced files under `🗑️generated/s5-load/pre-<wave>/`, `restore` puts them back). `check` at 01:41: p1 6 files, f1 3 files,
f12 2 files, every anchor holds.

| Wave | What it writes | Verifying command (gated, rule 55/56) |
|---|---|---|
| `p1` | the S5.1 law: 4 new files, a `#[path]` child module at the end of `🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs`, an oracle row in `🔌️plugin/📦️packages/🦀️rust/📜️script.ts` | `cargo test -p semio-framework-plugin --lib --features artifact-app-testing folder_reload_route` |
| `f1` | audit F1 caller half: `🏃️run` `RunError::Refused(Fault)` + `activation_refusal_error` (used by `open`, ≈2133) + law `an_admission_refusal_reaches_the_runner_as_the_guests_structured_fault`; MCP `ensure_instance_scoped` passes `PluginHostError::Refused(fault)` through instead of `not_wired("instantiate", …)` | `cargo check -p semio-framework-os-run -p semio-framework-os-mcp --lib` (= the owed P2 check), then `cargo test -p semio-framework-os-run --lib an_admission_refusal` |
| `f12` | audit F12: `composed_artifact_media` (`PLG` ≈24431–24446), its `export_media("artifact:out")` override (≈36084–36086) and the law `the_artifact_out_export_of_a_composed_document_is_the_composed_carrier` are deleted | `cargo check -p semio-framework-plugin --lib` + the plugin test build of `p1` |

F12 reference proof (`git grep`, tracked files, whole repo, 01:39): `composed_artifact_media` = 2 hits (definition + the override), the
law name = 1 hit; no descriptor, taxonomy, catalog or launch row names either. The only non-test caller of the instance-level
`export_media` is `produce_media` for ports other than `artifact:out` (`PLG` ≈35999), so the override was reachable from tests
only. After F12 the ONE `artifact:out` carrier is `produce_media` → `whole_document_media(document_archive())`: the recursive
archive with the parent `.spr` (`pk:` base64 text on a media edge). trinity jack's `export_media_graph_out_matches_document_pack`
compares `export_media("artifact:out")` with `graph:out` and keeps holding (both are the app's own head pack).
For S5-AGNOSTIC (W-a serializers): a serializer entry that reads `artifact:out` receives that archive, first byte `0x01`,
with a non-empty parent `.spr`; it folds it with `parse_document_pack(..).into_snapshot()`, exactly as `ArchiveChildren` folds members.

### S5.3 Design §21.6 — the whole-document carrier as a stepped export job (wire CONFIRMED unchanged, 01:42)

Read: `submit_owned_media_export` (`PLG` ≈30064), the channel arms (≈45477–45540), `produce_media` (≈35980), kernel channel
`AppCommand`/`AppFrame` (`📡️spr/🧵️channel/🦀️.rs` ≈1655–1672, 1889–1908).

- **No new `AppCommand`, no new frame, no layout change** (told `main`/S5-CHANNEL 01:42; nothing rides wave B). The existing
  `SubmitMediaExport{port, expected_parent_document_id, expected_base_revision}` → `MediaExportSubmitted{handle}`,
  `PollMediaExport{handle}` → `MediaExportStatus{state, applied_progress, checkpoint_available, mime_type, total_bytes, detail}`,
  `TakeMediaExportChunk{handle}` → `MediaExportChunk{data, terminal}`, `CancelMediaExport{handle}` carry it.
- **What blocks it today.** `submit_owned_media_export` admits only `QualifiedToolProof::AppOwned` (≈30072) and builds the job
  through `A::build_media_export_job`; no app registers `export-media:artifact:out`, so a submit of that port is refused.
  `plugin_produce_media` drives `produce_media` with `resolve_ready` (one unstepped reply, panics on a yield).
- **Design.**
  1. A framework-owned factory `export-media:artifact:out` registered by `register_framework_reserved_factories`, with a
     contract whose `max_output_bytes` is the base64 size of `DOCUMENT_ARCHIVE_MAXIMUM_BYTES` and a per-step work unit of ONE
     member envelope; `submit_owned_media_export` admits it beside the app-owned proof for exactly this tool id.
  2. Admission BEFORE any encoding, with the load side's own rule (`begin_document_archive_load` ≈35504): members ≤
     `DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS`, parent pack + `.spr` + member envelopes ≤ `DOCUMENT_ARCHIVE_MAXIMUM_BYTES`. A refusal is
     `plugin.document-load.too-many-members {count}{maximum}` / `plugin.document-load.too-large {bytes}{maximum}` (F16 codes),
     carried as `MediaError::Refused(Fault)` and answered by the submit arm as `AppFrame::Error{fault}` unflattened.
  3. The job: step 0 prints the parent pair (generation-fenced like `document_archive`), steps 1..n take one member envelope
     each, the last steps encode the archive and append ≤ 48 KiB base64 chunks to `output_chunks` under `output_credit`.
     `applied_progress` counts members + chunks really produced (no progress without work); `total_bytes` is known at admission.
  4. Cancel: `CancelMediaExport` → the lease's token; the job drops its partial archive and chunks; the terminal status is
     `Cancelled` and a later `TakeMediaExportChunk` answers nothing — zero trace. A store generation that moves during the
     job fails it `Failed` with the generation-fence detail (the caller resubmits against the new base revision).
  5. Hosts: `🏃️run` `compute_node` and MCP `ExportMedia` switch `artifact:out` from `MediaOut` to submit → poll (progress to
     the run's observer, cancel from `OperationContext.cancel`) → take chunks → `Structured{schema: app.io.artifactSchema,
     json: pk:…}`; `AppCommand::MediaOut{port: "artifact:out"}` is then refused with a named code instead of encoding inline,
     and `produce_media`'s whole-document tail is deleted with it.
- **Schema/fixture first.** A corpus `🔌️plugin/🧫️fixtures/🧫️document-export-job/🔣️.json` (schema beside it): archives by
  (member count, payload bytes) → admitted | refusal code + params; a step script → expected `applied_progress` sequence and
  chunk sizes; a cancel at step k → `Cancelled`, zero chunks. Rust law over the real job, TS twin with its own chunker
  (third party: `Buffer.from(...).toString("base64")` for the text, Ajv for the corpus).
- **Not started in code.** The factory lives in the tool-job region of `PLG` (S5-RUNTIME/S5-TOOLS); the job request types are
  theirs. This WP stages the corpus, the admission function and the host halves; the factory registration needs their review.

### S5.4 Landing 1 — p1 + f1 + f12 (landing lock 01:47:21 → 02:00:57, rules 51/55/56/58/59)

- 01:47 `python3 T/🧪️s5-load-waves.py {p1,f1,f12} land` → 11 files written (copies under `🗑️generated/s5-load/pre-{p1,f1,f12}/`).
- 01:56:49 (the v4 gate took 9.5 min to open) `CARGO_BUILD_JOBS=3 cargo check -p semio-framework-plugin -p semio-framework-os-run
  -p semio-framework-os-mcp --lib --message-format=short` → **exit 101, 1 error, mine**: MCP `🏠️workspace/🦀️.rs:1834` `match` arms
  `semio_framework_diagnostic::Fault` vs the gateway's own `actions::Fault {code, message}`. Fixed in place and in the script: the
  refused guest fault keeps its CODE and message (`Fault { code: fault.code.0.clone(), message: fault.message.clone() }`); the
  gateway's port type has no params, so `guest`/`host` reach an MCP client in the message words only (recorded limit of that type).
- 01:59:11 → 02:00:50 same command → **exit 0** (`Finished dev profile in 1m 39s`; plugin, os-run and os-mcp `--lib` green with
  f1 and f12 on disk). This is also the owed P2 check `cargo check -p semio-framework-os-run -p semio-framework-os-mcp --lib`.
- 02:00:57 lock released. The test targets are NOT yet compiled: the p1 law, the f1 law and the builder-contract file after
  the f12 cut are verified by the test build below.
- 02:01 started `RUST_MIN_STACK=268435456 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=3 CARGO_TARGET_DIR=…/⚡️cache/cargo/target-nde-s5-load
  cargo test -p semio-framework-plugin --lib --features artifact-app-testing folder_reload_route -- --nocapture`
  (output `🗑️generated/s5-load/test-p1-1.txt`).

Live news (coordinator 01:4x): probe batch B (React/en, build B0) = 67 PASS / 0 FAIL — the 09-30 reload failure and
`local.backbone-scope-mismatch` are gone live; the p1 law pins that state at the plugin/host route.

### S5.5 Coordinator decisions taken over mid-session (design §22.22) and what they mean for this WP

1. **Hot attach is unintended product behaviour** (finding of S5.1): a folder must hold the document (a) after binding an existing
   document to an EMPTY folder without another edit and (b) on a replica that only RECEIVES. Owner of the route fix: this WP.
   - wgpu BROWSER shell: already right (`🐚️Shell/🎯️targets/🧊️wgpu/📎️local-folders/🦀️.rs` `attach_host_route_backbone` :436 writes
     when the folder is empty; `flush_host_route_folder` :470 writes once per turn when `history_cursor` moved). No edit.
   - wgpu NATIVE shell: sends no archive at all; persistence is the Rust actor's `persist_operations` → S5-STORE.
   - React: the only shell that changes. Hunks (told to S5-UI through `main`): `🏛️ShellHost/🟦️.tsx` `receiveDocumentBackbone`
     ≈3205, `send:` ≈3221, the worker-event chain ≈3915 (new `documentArchiveAbsent` arm), `OpenDocumentSession` ≈2993/7613;
     the policy is one exported class next to `restoreDocumentArchiveV1` (`🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts`).
2. **Event** `{ kind: "documentArchiveAbsent" }` / `ArtifactEvent::DocumentArchiveAbsent` (once per open, bootstrap read only,
   204/404) and "a `localDocumentArchive` equal to `state.folderArchive` is not PUT again": S5-STORE lands both twins.
3. **A folder read-back of the document the program already shows is a MERGE, not a replacement** (today it replaces the store,
   adopts the writer's `REC_VIEWER` and ends an open history edit). S5-STORE adds `ArtifactStore::merge_persisted_pair(pack, spr)
   -> PairMerge { merged, ahead }`; the route is this WP's: same document → merge, another document or a cold program → load.
4. **Persist rule** (replaces my clock proposal): write the folder iff this replica's log holds events the folder's archive lacks —
   own published batch, empty folder, hub ingest (coalesced), and after a read-back merge exactly when `ahead > 0`.
5. **Wire (approved, rides wave B / channel 22, S5-CHANNEL lands the codec):** `AppCommand::MergeDocumentArchive { seq, archive }`
   as the last command (same archive encoding as `LoadDocumentArchive`; another document is refused
   `plugin.document-load.other-document`, the host then loads) + one trailing `DocumentArchiveLoadStatus.ahead: u64` (after
   `total`, before `fault`). Poll, cancel and acknowledge are the existing three commands, so progress and cancel of a merge are
   the load's. Why two host intents: a tutorial restore or "revert to file" sends an EARLIER archive of the same document and
   must replace; the guest cannot tell that from a read-back.
6. **Order:** nothing of 1/3/4 lands in React before the merge route exists — persist-after-ingest alone would make a
   shared-folder peer REPLACE the author's document (batch H). Sequence: STORE event + `merge_persisted_pair` → wave B codec →
   this WP: `PluginApp::begin_document_archive_merge`, `DocumentArchiveLoadHost::merging` + outcome `Ready{ahead}`, TS
   `AppChannelClient.mergeDocumentArchive`, corpus rows, plugin handle `mergeAppDocumentArchive`, then the ShellHost hunks
   under `serve`.

### S5.6 Staged, not landed (2026-10-05 02:16)

**`persist` wave — the folder archive persistence policy (design §22.22 items 1 and 4).**
`python3 T/🧪️s5-load-waves.py persist check` → 4 files, anchors hold. Sources: `🗑️generated/s5-load/wave-persist/`.
- `FolderArchivePersistenceV1` (goes beside `restoreDocumentArchiveV1` in `🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts`):
  `published()`, `absent()`, `merged(ahead)` (writes iff `ahead > 0`), `ingested(fold)` (ONE write when the folds in flight
  drain and at least one succeeded); writes are the repo's `latestWins` (single flight + one trailing write).
- Corpus `…/📄️document/🧫️fixtures/🧫️folder-archive-persistence/{🔣️.json,🧬️schema/🔣️.json}`: 12 cases (empty folder, published,
  receive-only, N folds = one write, refused fold, merge ahead 0 / > 0, changes during a write, failed write, two peers on one
  folder terminate). Event vocabulary: `publish`, `absent`, `foldStart`, `foldEnd{id, ok}`, `merge`, `writeEnd`.
- Law `💻️os/🧪️tests/🧪️folder-archive-persistence/🟦️.ts` (bun): Ajv strict + 3 hostile rows; every case through the class
  (promises settled event by event) and through the law's own reducer; fast-check (150 runs) over arbitrary settled sequences:
  class == reducer, never two writes in flight, writes ≤ changes, the last write starts at or after the last change.
  RAN from staging: `bun test 🗑️generated/s5-load/wave-persist/law.staged.test.ts` → **3 pass / 0 fail / 478 expects**;
  5/5 mutated policies killed (write on `ahead == 0`, write per fold, write after a refused fold, no coalescing, no write on `absent`).
- NOT in this wave: the four ShellHost hunks (S5.5 item 1). They land with the merge route, under `serve`.
- wgpu: the browser shell already satisfies the corpus by construction (one flush per turn on a moved `history_cursor`); a
  Rust replay of the corpus against `flush_host_route_folder` is S5-WGPU's law to add (told through `main`).

**`f16` — named load refusals.** 16 en/de rows staged for S5-GATES (owner of the kernel table):
`🗑️generated/s5-load/wave-f16/framework-notice-rows.json` (placeholders equal in both locales, codes unique — checked).
Mapping of the plugin sites (all `plugin_sdk_fault(…)` today = `plugin.internal`; gate `schema fault-notices --scope
history-editing` at 01:42: 435 findings, `🔌️plugin/🦀️.rs` = 57 anonymous + 21 framework codes without a notice, 5 of them the
coded load refusals below):
| Code | Sites in `PLG` (lines of 02:00) |
|---|---|
| `plugin.document-load.unavailable` | the five `PluginApp` trait defaults ≈15170–15195 |
| `…too-many-members {count}{maximum}` | `begin_document_archive_load` ≈35505; genesis ≈26381 |
| `…too-large {bytes}{maximum}` | ≈35509; genesis ≈26383 |
| `…busy` | ≈35512 (operation id live or slot occupied) |
| `…incomplete` | ≈35515 (parent pack or `.spr` missing) |
| `…member-invalid {ordinal}` | ≈35535; member ingress ≈23669–23703 |
| `…history-invalid` | ≈35541 (`RetainedHistoryDecode` refusal) |
| `…operation-unknown` | poll / cancel / acknowledge of an unknown, terminal or unsettled operation ≈35560–35583 |
| `…changed` | generation fence of `document_archive` ≈35606 |
| `…other-document` | new, the merge route (§22.22) |
| `…failed` | the stepped machine's internal invariants ≈26315–26710 ("owner is absent", "exceeded its grant", …) |
| 5 coded, notice only | `artifact-envelope.stale-handle`, `artifact-envelope.load-stale-handle`, `artifact-store.replacement-stale-handle`, `window-config.owner`, `window-config.load-retirement` |
Order: the table rows first (S5-GATES, with stage-r45), then the site conversion as one `PLG` wave with a law (one malformed
archive per admission code → `fault.code` and its en/de notice).

**F17 (not started in code; design of S4.4 stands).** `depends_on` entry `child:<slot>`; the host router skips `child:` entries;
MCP `infer_real` reads child heads only for declared slots and refuses `inference.children-stale` when `ReadDocument` and the
bound document differ byte for byte. No descriptor schema change.

### S5.7 Runs after landing 1 (2026-10-05 02:14–02:31) — what passed, what failed, whose it is

| Command | Result |
|---|---|
| `RUST_MIN_STACK=268435456 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=3 CARGO_TARGET_DIR=…/target-nde-s5-load cargo test -p semio-framework-plugin --lib --features artifact-app-testing folder_reload_route -- --nocapture` (13 min in the gate, build 5m 02s; `test-p1-1.txt`) | **1 passed / 0 failed** (1012 filtered out): `component::app::time_travel_tests::folder_reload_route::a_folder_bound_document_reloads_with_its_whole_history_and_each_programs_viewed_alternative`. First compile of the plugin TEST target with p1 + f12 on disk: exit 0, no warning in `🧪️tests/🧪️folder-reload-route/🦀️.rs`. |
| same test binary, owed load laws by filter (`test-load-laws-1.txt`): `folder_reload_route a_composed_documents_child_heads a_whole_document_media_import a_composed_document_crosses_a_media_edge whole_document_archive a_document_archive_round_trip the_refresh_poll_lane_stamps a_history_edit_is_its_own_row_locally_remotely_and_after_reload a_warning_an_edit_introduces_stays_visible bounded_reload export_media` | **11 passed / 2 failed** |
| same binary, three sibling composed tests (`test-composed-siblings-1.txt`) | 2 passed / 3 failed — the same setup refusal |
| `… cargo test -p semio-framework-os-run --lib --message-format=short -- an_admission_refusal a_whole_document_carrier` (`test-run-1.txt`) | **2 passed / 0 failed**: the f1 law and S4's `a_whole_document_carrier_crosses_a_run_edge_byte_for_byte` |
| the run crate's whole suite on that binary (`test-run-all-1.txt`) | **26 passed / 2 failed** |
| `bun T/🗑️generated/s5-load/run-twin-live.ts <repo>` (the LANDED twin) | `folder-reload-route-oracle steps=16` |

**P1 verdict.** The reload route holds on the current tree at the plugin/host level: both programs match the fixture before
the reload and read back exactly what they saved after detach → fresh program with an example → `DocumentArchiveLoadHost`;
every published batch names the program's document identity; a hot bind publishes nothing; one edit and one finalize are one
batch each; the reloaded author's next edit is admitted and listed. With the live batch B (67/0) the 09-30 failure is closed at
both levels. Not done: a product-mutant run of the Rust law (it would cost a second plugin test build while the fixture is
shared in the live tree); the law's pins are non-empty by construction and the TS twin kills 6/6 mutated expectations.

**The passing load laws (11):** the p1 law; `a_childless_whole_document_archive_replaces_the_live_document`,
`the_refresh_poll_lane_stamps_the_load_document_it_emits`, `an_unstamped_whole_document_archive_is_refused_by_parent_hydration`,
`a_refused_whole_document_archive_names_the_leg_that_refused_it`, `a_whole_document_archive_with_supersessions_loads_its_superseded_state`,
`a_document_archive_round_trip_lists_every_history_row_of_its_source` (R2-2), `a_whole_document_media_import_is_a_stepped_archive_load_and_never_folds_inside_the_call`
(W2A-6 P5 + `plugin.media.schema-mismatch` notice), `a_history_edit_is_its_own_row_locally_remotely_and_after_reload`,
`a_warning_an_edit_introduces_stays_visible_after_finalize_and_reload`, `a_long_history_reloads_one_operation_per_initializer_step`.

**Reds, none from this WP's waves (reported to `main` 02:32):**
1. Composed test fixture: every builder-contract test that registers a child on `contract_composed_app[_raw]()` fails at SETUP
   — `ChildMemberRegistrationError … "child member identity is not declared by the current parent"` (5 of 5 run, ~20 call sites).
   My two composed load laws (`a_composed_document_crosses_a_media_edge_with_its_children` :4918,
   `a_composed_documents_child_heads_answer_each_owned_childs_current_head_snapshot` :4853) are among them and never reach
   their subject → **OWED** after the composition owner (S5-NESTED / S5-RUNTIME) fixes the fixture or the validation.
2. `🏃️run` `intrinsic_media_wire_runner_preserves_owned_tags_words_octets_and_occurrences` (`🧪️tests/🔬️unit/🦀️.rs:968`): fingerprint
   differs after the intrinsic round trip — the pack/intrinsic-media peer's law (commit 670).
3. `🏃️run` `note_plugin_manifest_loads_from_its_committed_descriptor` (:943): the note component on disk predates the owned
   actor ABI → the describe wave (coordinator).

Rule 61 (02:2x): the p1 fixture, schema and the `📜️script.ts` oracle row were written at 01:47 under the rules of that time;
none is imported by a served bundle (the fixture is read by `include_str!` and by the test twin only).

### S5.8 After the 02:40 usage cut (resumed 04:48) — repair-first, one more run, live fault F4

- **Repair-first (rule 62).** Nothing was half on disk: `🧪️s5-load-waves.py {p1,f1,f12} check` → 0 files to write each (landed
  and intact), `persist` still staged (4 files), the wave script parses, the F1 law and the p1 registration are in place.
- **Last run before the cut (02:33–02:41, `test-mcp-1.txt`):** `… cargo test -p semio-framework-os-mcp --lib --message-format=short
  -- quick::an_abandoned quick::a_composed_documents_child_heads quick::a_bound_document quick::a_named_artifact` →
  **4 passed / 0 failed** (first compile of the MCP lib tests with f1 on disk: exit 0).
- **Peer reds routed by the coordinator (02:4x):** composed fixture `register_child` → S5-NESTED (my two composed load laws
  follow its fix); `intrinsic_media_wire_runner_…` → pack/intrinsic-media peer (commit 670), not mine;
  `note_plugin_manifest_loads_from_its_committed_descriptor` → describe wave. S5-RUNTIME had 2× E0502 in the plugin TEST
  target (`🧪️time-travel/🦀️.rs:2206/:2220`) — plugin test runs resume at "PLUGIN TESTS GREEN".

**Live fault F4 (first two-peer run, React/en batch H, two tabs on ONE folder; `🗑️generated/s5-e2e/run5-react-en-H.final.txt`,
`run5/probe-react-2026-10-05T02-43-27.ndjson`).** A attaches and drags; B attaches and reads A's document; A opens a history
edit; B drags. A never lists B's edit, A's Accept reviews without it, A's Finalize → Overwrite is never written to the folder,
both end with different documents and neither is told.

*What the evidence says (read, not run).* The folder request log of the run, with `AUTO_CHECKIN_IDLE_MS = 20_000`
(`🛠️ShellHelpers/🟦️.tsx:3514`):
| t (s) | Request | Reading |
|---|---|---|
| 20.8, 20.9 | GET 204 ×2 | A attaches an empty folder (bootstrap read + fresh stream read) |
| 24.8 / 25.0 | PUT / GET | A's drag is written; A reads its own echo |
| 35.7 ×2 | GET 200 | B attaches and loads A's archive |
| 44.6 / 44.8, 44.9 | PUT / GET ×2 | A's automatic check-in 20 s after its drag; both tabs read (B replaces from it) |
| 61.1 / 61.3, 61.4 | PUT / GET ×2 | B's drag; A reads B's archive while its history edit is open |
| 81.0 / 81.2, 81.3 | PUT / GET ×2 | B's automatic check-in; A reads again |
| 130 → end (196) | — | A finalizes: NO PUT, and none 20 s later for A's check-in |
The folder's final archive (`documents.db`, 143 672 bytes, `updated_at` = +81.6 s) holds two `applyBoardEvents` edits of two
actors and no trace of A's finalize.

*Root cause of "no PUT after the finalize" (host, verified in source).* A finalize does take the send path (batch B restored
the overwrite row from the folder; the p1 law pins one finalize = one published batch). What died is A's document port:
`loadDocumentArchive` (`🏛️ShellHost/🟦️.tsx` ≈3253) retires the port, loads, and rebinds in `finally` when the load rejects —
and then `DocumentAttachmentLaneV1.replace` (`🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts` ≈192–198) catches the same
rejection and detaches again (`port.retire()`, `ports.delete`). The session stays listed as attached, `entry.port` is a
retired object and the guest has no backbone, so from A's first rejected read-back (61 s) NOTHING A publishes reaches the
worker: no `send`, no `archivePersistence`, no PUT — for the finalize, its check-in and every later edit. On a reload A
would have lost all of it. The function's own docstring promises the opposite ("a cancelled or faulted load leaves the
previous document exactly as it was and its backbone bound again").

*Not yet known.* Which leg of the guest rejects A's load while its history edit is open (S5-E2E is asked for tab A's console
line `AppChannelClient.loadDocumentArchive(…): <fault>` / the `artifact-bootstrap-failed` text). If the load hung instead of
rejecting, the helper below would not help and only the merge route would; the staged F4 replay law decides it.

*Consequences for the staged work.*
- The `persist` wave alone does NOT close symptom (1): a dead port never calls `send`, whatever the policy.
- **Wave `attach` (staged, coordinator GO, lands under `serve` after "REACT RUN 5 DONE"):** `replaceAttachedDocumentV1` beside
  `restoreDocumentArchiveV1` — the port is retired, the content loaded and a fresh port bound inside the attachment lane; a
  rejected load is caught INSIDE the lane step, the document is bound again and stays attached, and the rejection is rethrown
  to the caller afterwards. ShellHost `loadDocumentArchive` uses it (one hunk ≈3272–3283 + the import). Law
  `💻️os/🧪️tests/🧪️attached-document-replacement/🟦️.ts` over the real `DocumentAttachmentLaneV1` and
  `LatestDocumentReplacementV1`: a refused load leaves a bound port and reaches its caller; a superseded load loads and binds
  nothing; fast-check over arbitrary settled sequences (resolve / reject / overlap): exactly the newest port is bound.
  RAN from staging: `bun test 🗑️generated/s5-load/wave-attach/run/law.test.ts` → **3 pass / 0 fail / 482 expects**. It joins
  the os `test-channel-oracles` command (script + project inputs), so no new launch row.
  `python3 T/🧪️s5-load-waves.py attach check` → 5 files, anchors hold.
- **Honest interim until the merge route (accepted for build B1 by the coordinator):** with the port alive A's finalize is
  written and B then REPLACES its document with A's archive — B's concurrent drag is lost (last writer wins, the semantics
  before design §22.22). That is better than both diverging silently and A losing every later edit on reload, and it is
  still wrong: a read-back must merge.
- **Merge route (after "WAVE B LANDED" + "MERGE PAIR ON DISK").** Wave B as S5-CHANNEL built it: `AppCommand::MergeDocumentArchive
  { seq, archive }` = tag 43, the last command, body identical to `LoadDocumentArchive`; `DocumentArchiveLoadStatus.ahead`
  (varint after `total`, before `fault`); a refusal arm `app.command.unsupported` above `PollDocumentArchiveLoad` that my
  handler replaces. S5-STORE's staged API (read in `🗑️generated/s5-store/stage-p1/…/🧪️viewer-head/🦀️.rs`):
  `ArtifactStore::merge_persisted_pair(&pack, &spr).await -> Result<PairMerge { merged, ahead }, VcsError>`, another document →
  `VcsError::ValidationFailed`, a merge moves the store generation (the open session's base move), never the viewer head; its
  acceptance law `two_peers_on_one_folder_converge_through_an_open_history_edit` is F4 at the store.
  Host rule: the FIRST archive read after an attach is a load (the folder wins — batch B's reconnect and B's attach in H rely
  on it; a fresh program's example document shares the document id, so a merge would union it in); every LATER read-back
  merges WITHOUT retiring the port; a `plugin.document-load.other-document` refusal falls back to a load;
  `persistence.merged(ahead)` writes iff `ahead > 0`; `absent()` on S5-STORE's `documentArchiveAbsent`.

### S5.9 F4 root cause found (05:15, E2E's alert text + source) and the corrected `attach` wave

**Correction to S5.8.** The read-back load on tab A was never refused by the guest and never started. E2E read tab A's
alert: "Document restore failed: actor-document-control.receipt-count". The chain, read in source:
1. `loadDocumentArchive` (`🏛️ShellHost/🟦️.tsx` ≈3253) first retires the document port. A retire is a control turn.
2. The React plugin runtime's `exchange` (`🔌️PluginRuntime/🟦️.tsx` ≈3496–3503) answers EVERY shell frame of that turn as a
   receipt; `ActorDocumentBindingV1.#exchange` (`🔌️plugin/📡️backbone/🔗️binding/🟦️.ts:183`) demands exactly one.
3. With a history edit open the guest's `detach_backbone` bumps the store generation (`ArtifactStore::detach_backbone` →
   `bump()`), the session sees a base move (`watch_time_travel_base`) and the same turn carries its status frame beside the
   receipt (`route_app_frame`, `⚛️reactor/🔄️turn/🦀️.rs` ≈2072) → two frames → `actor-document-control.receipt-count`.
4. The guest HAS detached; the host throws before it records the retirement (`#remote`, `#bound` stay "bound"); the port's
   `#retirement` promise is memoized as rejected, so `bindDocumentPort` (`previous.port.retire()`) can never bind this
   instance again. The document stays listed as attached with a dead port: nothing A publishes afterwards is written.
Without an open session no extra frame exists — batch B and B's own attach pass. The wgpu plugin bridge has the same check
(`🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts` ≈1798, `settled.frames.length !== 1`); the store worker's own binding classifies its
turn effects and is not affected. The earlier reading (the attachment lane detaches after a rejected load) is a second,
real defect of the same function, fixed in the same wave; it is not what F4 hit.

**Finding for S5-STORE (told through `main`).** A port change is no event, yet `detach_backbone` / `attach_hot_backbone` move
the generation an open history edit watches: the user's next draft input is refused `timeTravel.stale` (the notice E2E saw
on tab A at 109.5 s).

**Wave `attach` as staged now** (`python3 T/🧪️s5-load-waves.py attach check` → 9 files, anchors hold; served TS → `serve`):
| Part | Files | Law (RAN from staging) |
|---|---|---|
| A control turn may carry the program's own frames: `splitDocumentBackboneControlTurnV1` (canonical receipts are receipts, a duplicate or a missing or damaged receipt is still refused, every other frame is routed like any turn's) | `🔌️plugin/📡️backbone/🔗️binding/🟦️.ts`; React `🔌️PluginRuntime/🟦️.tsx` `exchange` (+ import); wgpu `🐚️plugin-bridge/🟦️.ts` `documentBackbone` (+ import) | `💻️os/🧪️tests/🧪️document-port-control-turn/🟦️.ts`: a binding binds and retires through turns that carry a status frame and a text frame, both routed; duplicate / none / damaged refused; fast-check partition. `bun test …/wave-attach/run-binding/law.test.ts` → **3 pass / 0 fail / 806 expects** |
| A rejected load keeps the document attached and bound: `replaceAttachedDocumentV1` | `🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts`; `🏛️ShellHost/🟦️.tsx` `loadDocumentArchive` (+ import) | `💻️os/🧪️tests/🧪️attached-document-replacement/🟦️.ts` → **3 pass / 0 fail / 482 expects** |
| A read-back that cannot be restored is loud | ShellHost catch ≈3928: `console.error("[os-shell] a folder read-back could not be restored", documentId, reason)` | — (E2E's console digest) |
| Registration | os `📜️script.ts` `test-channel-oracles` runs the three laws; `📋️project.json` inputs | — |
Expected in batch H on the next build (unproven until run): A's read-back retires, loads B's archive under the open session
(a replacement whose targets exist → base move), rebinds; B's edit is listed as not applied, Accept replays it, the
finalize is written, B loads it. Lossless in that sequence because B's archive contains A's log; in general the last writer
wins until the merge route lands.

**Staged for the merge route (after "WAVE B LANDED" + S5-STORE's wave PM), all under `🗑️generated/s5-load/wave-merge/`:**
- `route.ts` + `🧫️folder-read-back/` + `law.ts`: `FolderReadBackRouteV1` (first archive after an attach loads; later
  read-backs merge without retiring the port; `merged > 0` → history + refresh; `ahead > 0` → write; another document →
  load; an empty folder takes the program's document), 9-case corpus incl. "two peers on one folder through an open history
  edit". `bun test …/wave-merge/run/law.test.ts` → **3 pass / 0 fail / 24 expects**; 5/5 mutated routes killed.
- `route-law/`: the route fixture gains `twoPeers` (schema + twin `deriveTwoPeers`; twin RAN: `steps=24`), and the Rust law
  `two_programs_on_one_folder_converge_through_an_open_history_edit` (F4 replayed with two programs through
  `DocumentArchiveLoadHost::merging`: merged/ahead per read-back, the session stays open on the same preview, the peer's
  edit is listed as not applied, the port stays bound, nothing is written while nobody is ahead, the finalize is one batch,
  both settle on the same head and rows). NOT compiled: needs `AppCommand::MergeDocumentArchive`, `status.ahead`,
  `DocumentArchiveLoadHost::merging` and `PluginApp::begin_document_archive_merge`.
- S5-STORE's API as announced (wave PM): `merge_persisted_history(pack, history: HistoryLog) -> Result<PairMerge, VcsError>`
  (the plugin decodes the `.spr` stepped, then one merge call; the fold of the foreign log inside is not stepped yet — a
  follow-up behind the same signature) and `SpaceMember::merge_persisted_envelope` with a default refusal.
  **Recorded limit:** until `space_members!` implements it (S5-NESTED), an archive whose member envelopes differ from the
  live members' is refused `plugin.document-load.members-differ` and the host LOADS it — a composed document on a shared
  folder keeps the replacement semantics whenever a peer changed a child lane.

**Wave `f16` (staged, table rows landed by S5-GATES 04:59):** one helper + ten codes beside `MEDIA_SCHEMA_MISMATCH_CODE`;
71 sites of the load and archive surface (selected by their own message text, every site reviewed by region: trait
defaults ≈15205, member ingress ≈23698–23762, `archive_member_entries` ≈24426/24461, the stepped machine ≈26300–26760,
admission/poll/cancel/acknowledge/export ≈35480–35640) → unavailable 6, too-many-members 2 (`{count}{maximum}`), too-large 2
(`{bytes}{maximum}`), busy 1, incomplete 1, member-invalid 1 (`{ordinal}`), history-invalid 3, operation-unknown 5,
changed 1, failed 48. Law `every_refused_whole_document_load_is_named_and_told_in_every_locale` (in the route law file): six
refusals → their code and an en + de framework notice that names the counts and bounds, a cancel that leaves the document.
`python3 T/🧪️s5-load-waves.py f16 check` → 2 files, anchors hold. Verify: `cargo check -p semio-framework-plugin --lib`, then
`cargo test -p semio-framework-plugin --lib --features artifact-app-testing folder_reload_route`.

### S5.10 Landing 2 — wave `attach` under `serve` (05:28:05 → 05:29:25) and what is staged behind wave B

**Landed (9 files, copies under `🗑️generated/s5-load/pre-attach/`):** `🔌️plugin/📡️backbone/🔗️binding/🟦️.ts`
(`splitDocumentBackboneControlTurnV1`); `📺️renderer/…/🔌️PluginRuntime/🟦️.tsx` (`exchange` + import); `📺️renderer/…/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`
(`documentBackbone` + import); `📺️renderer/…/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts` (`replaceAttachedDocumentV1`);
`📺️renderer/…/🏛️ShellHost/🟦️.tsx` (import, `loadDocumentArchive`, the loud catch); new laws `💻️os/🧪️tests/🧪️attached-document-replacement/🟦️.ts`
and `💻️os/🧪️tests/🧪️document-port-control-turn/🟦️.ts`; os `📦️packages/🟦️typescript/{📜️script.ts,📋️project.json}` (`test-channel-oracles`).

| Command (run in the hold, live tree) | Result |
|---|---|
| `bun ./📜️script.ts test-channel-oracles` (cwd `💻️os/📦️packages/🟦️typescript`; `test-channel-oracles-1.txt`) | **9 pass / 0 fail / 1519 expects** (3 files) |
| renderer-react `bunx tsc --noEmit -p tsconfig.json` (`tsc-react-attach-1.txt`) | **exit 0, 0 errors** (covers ShellHost, PluginRuntime, the document and binding modules) |
| `bunx tsc -p T/🧪️s3-w2c-typecheck-wgpu-host.tsconfig.json` (`tsc-wgpu-attach-1.txt`) | **exit 0, 0 errors** (covers the wgpu plugin bridge) |
| renderer-react `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ../../../../🧪️tests/📡️actor-backbone` | **8 passed** (duplicate receipts are still refused) |
| `curl http://127.0.0.1:6012/` | 200 |
NOT proven live: batch H on this TS (S5-E2E re-runs it). The expectation of S5.9 stands unproven until that run.

**Live fault F6 (batch I: after "Cancel replay" the body's "Loading document: 0 of 1" stayed for 180 s; one step for a
two-example archive) — read in source, fix staged as wave `f6`.**
- The row is `reprojection_status()` → `live_document_load()` (`⏪️time-travel/🦀️.rs` ≈2439), which lists every load that is
  not `terminal()`. A cancel only sets the machine's `terminal_target`; `state` reaches `Cancelled` after the retirement
  steps of `drive_document_archive_terminal`, which run in maintenance steps. `document_loading_refusal` (`PLG` ≈30841)
  uses the same predicate, so verbs kept waiting too.
- "0 of 1": before the fold begins the machine counted nothing (`completed 0`, `total = members + 1`); fold progress is
  added from the replacement job only once the store initializer runs.
- Wave `f6` (3 files, `python3 T/🧪️s5-load-waves.py f6 check` clean): `ActiveDocumentArchiveLoad::loading()` =
  `!terminal() && terminal_target.is_none()` — a cancelled, faulted or committed load that only retires what it held does
  not read as loading; both call sites ask it (one line in S5-RUNTIME's `live_document_load`). The decode phase adds the
  history records it decoded to `completed` and `total` (work discovered so far; `completed <= total`, both monotone), the
  parent unit is added, not assigned. Laws (route law file): `a_whole_document_load_reports_the_work_it_did_and_never_goes_back`
  (24-edit history: counts never go back, never exceed the total, more than one value, a finished load did all of it) and
  `a_cancelled_load_stops_reading_as_loading_at_once_and_retires_without_a_host_poll` (guest cancel through
  `historyEditCancelReplay`: no loading row, no verb waits, terminal within maintenance turns with zero polls, the host's
  next poll reads `Cancelled`, document unchanged).
- Not in this layer: whether the runtime's maintenance pump gets turns while the page is idle. The law drives
  `PluginApp::maintenance_step` directly; if the live row still stayed after this wave it would be the pump's arming.

**Wave `merge-guest` (staged; needs f16, f6, wave B):** `PluginApp::begin_document_archive_merge(operation, archive)` = the
load's own admission, flagged a merge; machine phase `MergeParent` after the stepped `.spr` decode: member envelopes must
equal the live members' (else `plugin.document-load.members-differ`), then ONE `store.merge_persisted_history(&pack, history)`
(`ValidationFailed` → `plugin.document-load.other-document`), `completed/total += merged`, `ahead` into the status,
`deliver_base_moved` when the generation moved, `follow_derivable_children`, terminal `Ready` through the machine's stepped
retirement. A merge never reads as loading. `python3 T/🧪️s5-load-waves.py merge-guest check` refuses until its
preconditions are on disk (by design).
**Still to write after wave B:** the channel arm (replaces S5-CHANNEL's `app.command.unsupported` arm), kernel
`DocumentArchiveLoadHost::merging` + corpus rows + TS twin + `AppChannelClient.mergeDocumentArchive` + plugin handle
`mergeAppDocumentArchive`, the ShellHost hunks (`FolderReadBackRouteV1` + `FolderArchivePersistenceV1` + STORE's
`documentArchiveAbsent`), then the route-law v2.
**Order agreed with the coordinator (05:41):** my `landing` ticket lapsed on request; RUNTIME D → WGPU 2+3 → UI s1r → STORE
RB go first, then wave B, then f16 → f6 → merge-guest → merge-host.

### S5.11 Live proof of the `attach` wave, landing 3 (`checkin`, O4), and the queue behind wave B (06:00)

- **LIVE (coordinator relay, S5-E2E batch H on the served `attach` wave): 27 PASS / 0 FAIL.** All four F4 verdicts pass: A lists
  B's edit "Not applied while editing" 12 s after B's drag, Accept replays it, A's finalize is PUT, B shows it 11 s later, both
  peers converge and B's drag survives; no restore alert, no `timeTravel.stale`, 0 console errors. Build: guest B0 + served TS.
- **Live finding O4 (same run): an AUTOMATIC check-in was dispatched into the loading document** — console
  `input #17 commitCheckpoint refused: dispatch-failed … document.loading`, and the person, who pressed nothing, was shown
  "The document is still loading — wait for it or cancel it first."; earlier one fired into a retired port after a detach.
  Read in source: `AutoCheckinScheduler` (`🛠️ShellHelpers/🟦️.tsx`) latched `pending` when it fired and `dispatchCheckpoint`
  (`🏛️ShellHost/🟦️.tsx` ≈11145) knew one reason to wait (an open history edit) — a waited check-in was never asked for again
  (the latch stayed set until a checkpoint landed), and a loading or unbound document was not a reason at all.
- **Wave `checkin` LANDED under `serve` (05:56 → 05:58, 5 files, copies `🗑️generated/s5-load/pre-checkin/`):**
  `AutoCheckinScheduler.defer()` (the latch is released and the idle period starts again), `automaticCheckinWaitsV1({loading,
  attached, bound})` (loading, or attached with an unbound port), `dispatchCheckpoint` answers whether it dispatched and an
  automatic one waits (guest-reported load `reprojection.kind === "load"`, a host replacement in flight, or a port that is
  null or closing), the scheduler's callback defers when nothing was dispatched — also for the open-history-edit wait, which
  used to lose the check-in. Corpus `🛠️ShellHelpers/🧫️fixtures/🧫️automatic-checkin/🔣️.json` + schema
  `🛠️ShellHelpers/🧬️schema/🔣️automatic-checkin/🔣️.json` (5 states, 4 timelines; agreed with an independent model:
  `bun 🗑️generated/s5-load/wave-checkin/model.ts`); law appended to the scheduler's suite in
  `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` (fake timers over the corpus).
  | Command | Result |
  |---|---|
  | renderer-react `bunx tsc --noEmit -p tsconfig.json` (`tsc-react-checkin-1.txt`) | exit 0, 0 errors |
  | `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ../../../../🧪️tests/🔬️engine-contract -t "AutoCheckinScheduler"` (`vitest-checkin-2.txt`) | **5 passed** (697 skipped); the first run failed 1 — my test imported the predicate through the package barrel, which lists its exports; fixed to the ShellHelpers import |
  | `curl 6012` | 200 |
  Not covered: a load that begins between the check and the dispatch in the same tick can still be refused by the guest; the
  wgpu shell's automatic check-in (`handle_checkin_action`) is S5-WGPU's twin — the corpus is language-neutral for it.
  NOT proven live yet (S5-E2E's next batch H: no `commitCheckpoint refused … document.loading` line, no loading notice).
- **f16 gained the notice row** `plugin.document-load.members-differ` (kernel Rust table 82 → 83, TS twin, fixture; S5-GATES
  ended its turn): `python3 T/🧪️s5-load-waves.py f16 check` → 5 files. It needs `landing` + `serve`; verify with
  `cargo check -p semio-framework -p semio-framework-plugin --lib`, `bun test ./🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧪️framework-notices/🟦️.ts`,
  `bun test ./🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️fault-notices/🟦️.ts`, `bun test ./…/🧪️test/🧪️tests/🧪️fault-notices-gate/🟦️.ts`,
  then `cargo test -p semio-framework --lib -- framework_notices` and the plugin `folder_reload_route` filter.
- **`merge-guest` extended (staged):** the channel arm (S5-CHANNEL's `app.command.unsupported` arm becomes the real handler)
  and S5-CHANNEL's interim law in `🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs` becomes
  `a_merge_archive_command_is_admitted_under_its_own_sequence_on_both_routes`.
- **Queue:** wave B (S5-CHANNEL, channel 22) is not on disk at 06:00 (`MergeDocumentArchive` absent from the kernel channel).
  After it: f16 → f6 → merge-guest → merge-host (kernel `DocumentArchiveLoadHost::merging` + corpus rows + TS twin +
  `AppChannelClient.mergeDocumentArchive`) → the ShellHost read-back route (`persist` + `wave-merge/`) → route-law v2.

### S5.12 Staged for the minutes after "WAVE B LANDED" (state at 06:05) — exact order and commands

All anchors below were verified against the live tree or against the exact text S5-CHANNEL's rider produces
(`T/🧪️s5-channel-merge-archive.py`); `check` of a wave that needs wave B refuses by design until it is on disk.
| # | Wave (`python3 T/🧪️s5-load-waves.py <wave> check|land|restore`) | Locks | Verify |
|---|---|---|---|
| 1 | `f16` — 71 load-surface sites → `plugin.document-load.*`, the `members-differ` notice row (Rust table 82 → 83, TS twin, fixture), law | `landing` + `serve` | `cargo check -p semio-framework -p semio-framework-plugin --lib`; `bun test ./🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧪️framework-notices/🟦️.ts`; `bun test ./🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️fault-notices/🟦️.ts`; `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️fault-notices-gate/🟦️.ts` |
| 2 | `f6` — `loading()` predicate (2 call sites), decode records counted, 2 laws | `landing` | same plugin check |
| 3 | `merge-guest` — `begin_document_archive_merge`, `MergeParent`, channel arm, S5-CHANNEL's interim law converted | `landing` | `cargo check -p semio-framework-os-kernel -p semio-framework-plugin --lib` |
| 4 | `merge-host` — kernel `DocumentArchiveLoadHost::merging` + `ahead()`, corpus law; 3 merge cases + schema fields | `landing` + `serve` | `cargo test -p semio-framework-os-kernel --lib -- the_document_archive_load_host`; `python3 T/🧪️s3-load-archive-host.py` (model to extend) |
| 5 | test build of the plugin: `cargo test -p semio-framework-plugin --lib --features artifact-app-testing -- folder_reload_route a_merge_archive_command` | — | 5 route laws + the dispatch law |
Then, still to WRITE (TS, `serve`): the TS twin `DocumentArchiveLoadHost.merging` / `ahead`, `AppChannelClient.mergeDocumentArchive`
(result `merged {merged, ahead}` or `unmergeable {code}` for `other-document` / `members-differ`), plugin handle
`mergeAppDocumentArchive`, the load-host TS law rows; the ShellHost read-back route (`persist` + `wave-merge/route.ts`,
needs S5-STORE's `documentArchiveAbsent` event, not on disk at 06:02); route-law v2 (`wave-merge/route-law/`, fixture
`twoPeers` + twin + the Rust law `two_programs_on_one_folder_converge_through_an_open_history_edit`).
Design detail fixed while staging: a merge's status counts merged events only (`completed == total == merged` at `Ready`;
the decode records of `f6` are not added for a merge), so a host can tell "took nothing" from the status alone.

### S5.13 Stand-back for wave B (06:12 →): the whole merge route is staged, one incident of mine (06:23–06:25), repaired

**Incident, stated plainly.** At 06:23 a scratch script of mine (`T/🧪️s5-load-mirror.py`, first version) wrote five staged
files INTO THE TREE without a lock, during S5-CHANNEL's wave-B hold. It was meant to build a scratch copy of the repository
(real directories down to each staged file, symlinks elsewhere) so a staged TypeScript wave can be proven with bun before
its `serve` hold. Its bottom-up `exists()` test is true *through* a directory symlink, so for every staged file lying two
or more levels below an already mirrored directory it took the real directory for a mirror directory, unlinked the real
file and wrote the staged text there. Files: kernel `🧵️channel/🧪️tests/🔬️unit/🦀️.rs`, the load-host corpus and schema
(`🧫️document-archive-load-host`, `🔣️document-archive-load-host`), `🔌️PluginRuntime/🟦️.tsx` (served: two hot reloads) and
`💻️os/🧪️tests/🧪️document-archive-load-host/🟦️.ts`. Noticed from the bun stack trace (a tree path where a mirror path was
expected). Restored at 06:25:21: for each file the pre-image was rebuilt by inverse replacement, the wave re-applied to it in
memory and byte equality with the tree required before writing (nobody else had edited them); `git diff HEAD` afterwards
shows only S5-CHANNEL's rider and my landed `attach` hunks in those files. No cargo was started. Reported to the
coordinator at once. What I cannot know: whether a check, a test or a probe of someone else ran inside those two minutes.
The script now proves, before every mutation, that the parent's REAL path lies inside the mirror, builds directories
top-down, creates files with `O_EXCL | O_NOFOLLOW` and compares the bytes of every touched tree file before and after;
every later use below was bracketed by an identical `git diff … | md5` of `🧰️framework/🛍️products/💻️os`.

**Staged since S5.12** (all in `T/🧪️s5-load-waves.py`; nothing of it is on disk):
- `merge-host` now carries the TypeScript half too (one hold, because the corpus rows and the twin that replays them must
  change together): `DocumentArchiveLoadHost.merging` / `ahead`, `AppChannelClient.mergeDocumentArchive` (answers
  `merged {merged, ahead}` or `unmergeable {code}` for the guest's `other-document` / `members-differ`; drops the cached root
  pair when it took events), one private drive loop for load and merge, plugin handle `mergeAppDocumentArchive`, and the
  bun law's merging rows (host replay, client replay with pack-encoded faults, the cache rule). 7 files.
- `merge-shell` (needs `merge-host` and S5-STORE's wave AA): `FolderArchivePersistenceV1` + `FolderReadBackRouteV1` beside
  `restoreDocumentArchiveV1`, corpora, two bun laws (joined to `test-channel-oracles`), and the ShellHost hunks — every open
  document owns a policy and a route; `send:` → `published()`, a folded foreign batch → `ingested()`, `documentArchiveAbsent`
  → `absent()`, `documentArchiveReplaced` → the route (first archive loads, later ones merge with the port bound). The
  route answers `unmergeable` (was `otherDocument`) and now drives ONE read-back at a time, keeping only the newest waiting
  one — a second merge admitted while one runs would be refused `busy` by the guest and tell the person "restore failed".
- `merge-law`: the two-programs law as in-place edits of the route law file (f16 and f6 append to the same file), fixture +
  schema + twin replaced whole, guarded by the hashes of the files it was written against. NOT compiled.
- `T/🧪️s3-load-archive-host.py` (python model) knows `merging` cases and a ready outcome's `ahead`.

**What I ran (all without cargo, nothing landed):**
| Run | Result |
|---|---|
| `python3 T/🧪️s3-load-archive-host.py` on the tree's corpus / `--negative` | 15 cases, 0 failures / 2 failures (as it must) |
| the same model on the staged corpus (3 merge rows), schema-validated in memory | 18 cases, 0 failures |
| the five os oracle laws inside the guarded mirror (`merge-host` + `part:merge_shell_module`) | 18 pass, 0 fail, 2120 expects |
| mutants of the staged TS twin (admit command, `ahead`, cache rule ×2, unmergeable ×2) | 6 of 6 killed |
| mutants of the staged read-back route (first-is-load, adoption ×2, re-read ×2, write ×2, stale, unmergeable, serial) | 10 of 10 killed |
| in-memory chain `f16 → f6 → merge-guest → merge-host → merge-law`, then `merge-shell` with wave AA's event member simulated | every anchor holds |
Not run, and so not claimed: any Rust of the staged waves (no cargo since the stand-back order), `tsc` over the staged
ShellHost hunks (a mirror cannot type-check: files reached through symlinks resolve to the tree's module instances).

**Guest merge restaged (06:40), two faults of my own staging found by reading the machine again — neither was on disk:**
1. `begin_document_archive_load` keeps the archive's members REVERSED (the machine pops them from the end); a plain
   `live != archive.members` would have refused every archive with two or more members as `members-differ`. The staged
   test is now `archive_member_entries()` (the roster `document_archive()` itself writes) against `archive.members.iter().rev()`.
2. The merge no longer runs inside the synchronous maintenance step through `resolve_ready` (which PANICS on a future that
   suspends, and `deliver_base_moved` dispatches a typed app command). The step parks at `MergeParent` reporting no progress;
   `poll_document_archive_load` — async, the host's own turn — calls `merge_document_archive`, which awaits the store's
   `merge_persisted_history`, the base-move delivery and `follow_derivable_children`, then requests `Ready` or the fault. A
   close while it waits cancels it like any load.
No law covers the member comparison yet: the route law's toy document has no members, and the composed fixture is red
(`register_child`, S5-NESTED). OWED with the composed laws: identical members merge, differing members answer
`plugin.document-load.members-differ`.

**Order after "LOCKS OPEN"** (replaces the "still to WRITE" paragraph of S5.12): `f16` (`landing` + `serve`) → `f6`
(`landing`) → `merge-guest` (`landing`) → `merge-host` (`landing` + `serve`; kernel test, python model, `bun
./📜️script.ts test-channel-oracles`, renderer `tsc`) → `merge-law` (`landing` + `serve`; the plugin test build) →
`merge-shell` (`serve`; after wave AA; `test-channel-oracles`, renderer `tsc`, wgpu-host `tsc`).

### S5.14 After the 07:45 cut (resumed 09:36): repair-first, then the B1 live fault `receipt-count` — host half served 09:53

**Repair-first.** Nothing of mine was half-applied: every staged marker (`document_load_fault`, `loading()`,
`begin_document_archive_merge`, `merging(`, `mergeAppDocumentArchive`, `FolderReadBackRouteV1`, the `members-differ` row, the
two-programs law) occurs 0 times in the tree; `check` of the landed waves writes 0 files. One staged copy was stale: the
store's `description` deletion (wave B era, 192 files) had edited the landed route law, so `p1 check` wanted to write it
back — the staged copy is now the tree's file.

**What run 6 really shows (read from E2E's files, not from the verdict table).**
- Both batch folders, `run6/folder-2026-10-05T05-27-21-en` and `…05-38-44-en`, are EMPTY. On build B1 the folder attach
  never bound the document port; batch A's step 5 passes only because it does not look at the folder (batch B's reload
  half does, and fails 6 verdicts).
- The stack's `PluginRuntime 2207:7` is, in the served transform (fetched read-only), `await binding.port.retire()` in the
  CATCH of `bindDocumentPort`. So the bind had already failed with an error nobody saw; the cleanup retirement's control
  turn then carried no single receipt, threw `actor-document-control.receipt-count`, and that replaced the bind's error.
- Three more host defects on the same path: the failed binding stayed in `documentBindings` with a kept (memoized)
  rejection, so EVERY later bind of that program rethrew it; ShellHost's `bindDocumentBackbone` catch and the attachment
  lane's detach awaited the same rejected retirement and replaced the failure again; and the sync card called
  `void openSyncTarget(target)` bare — hence the unhandled rejection, and the person was told nothing.
- My own attach wave made it harder to read: `splitDocumentBackboneControlTurnV1` treated a receipt that does not decode
  as "not a receipt" and routed it on as a frame of the program's, so codec drift would also surface as `receipt-count`.

**Landed (wave `receipt`, `serve` 09:53:07 → 09:54:47, 8 files, copies under `🗑️generated/s5-load/pre-receipt/`).**
`🔌️plugin/📡️backbone/🔗️binding/🟦️.ts`: a frame is a receipt when it NAMES the receipt schema (a damaged one then fails under
its own decode code); `soleDocumentBackboneReceiptV1` says the count (`receipt-count:0` / `:2`); `ActorDocumentBindingV1.retire()`
is single-flight, askable again, and takes the program's `stale-generation` refusal as a retirement; `bindActorDocumentV1`
settles a failed bind as itself and hands the retirement's failure to its caller; `releaseActorDocumentBindingV1` releases a
predecessor the program did not let go of. `🔌️PluginRuntime/🟦️.tsx` binds through both and logs a retirement that failed.
`🏛️ShellHost/🟦️.tsx`: lane detach and the bind catch never replace the failure they follow; attach and reconnect failures
are a notice (`ui.sync.attachFailed`, en + de in `🖱️ui/…/🌐️i18n/🟦️.ts`, type in `📚️I18n/🟦️.tsx`) plus
`console.error("[os-shell] sync attach failed", …)`. Law `💻️os/🧪️tests/🧪️document-port-control-turn/` rewritten with a
corpus (10 cases) + schema; its first case is the coordinator's "fresh folder attached to a program that holds a bound port".

| Run (in the hold unless noted) | Result |
|---|---|
| control-turn law in the guarded mirror, before the hold | 6 pass / 0 fail; 11 of 11 mutants killed (masking, untold, no re-ask, kept rejection, stale ×2, damaged routed on, count, re-ask of a retired one, refused retired, a deliberately unhandled rejection) |
| `bun ./📜️script.ts test-channel-oracles` | 12 pass / 0 fail / 1572 expects |
| vitest `📡️actor-backbone` | 8 passed |
| `bunx tsc -p T/🧪️s3-w2c-typecheck-wgpu-host.tsconfig.json` | 0 errors |
| renderer-react `bunx tsc --noEmit -p tsconfig.json` | 1 error, NOT in this wave: `🎛️UtilityTree/🧪️tests/🎛️picker-explicit-press/🟦️.tsx(52,61)` TS2769 (`ShellScopeProvider` wants `children`; file saved 09:28 by a peer) |
| served modules fetched from 6012 | the three edited modules carry the new code |

**NOT fixed, and not known: why a control turn of the B1 guest carries no single receipt.** The host's control code is
byte-identical to what batch H ran green on B0; the reactor turn, the guest's control handler, the binding's Rust module and
`wire-turn` are unchanged in this session; the pack encoder's uncommitted edits date from 01:32–03:06, before B0's batch H.
I could not prove a cause by reading and did not guess one into the code. The next React batch prints it in step 1 as
`[os-shell] sync attach failed <error>`: `receipt-count:0` (the guest answered no receipt), `receipt-count:2` (twice),
`actor-document-control.<x>` (the receipt's bytes no longer fit the TypeScript grammar), anything else (the bind's own
refusal). Staged as wave `golden` to close the gap that let this stay invisible: the binding fixture gains `codec.control`
(six control maps with their wire bytes), a Rust unit law and a TypeScript law pin both directions — TS half proven in the
mirror (7 pass); the Rust half is OWED until the first plugin test build.

### S5.15 F9 — cause, fix, and the merge route on disk (10:00 → 10:53)

**F9 cause (found by S5-CHANNEL, confirmed by S5-E2E's console line `actor-document-control.noncanonical`).** A peer's
deliberate, uncommitted edit of the pack encoder (`🎒️pack/🌱️value/🦀️.rs`, saved between builds B0 and B1) stopped ordering
an object's members. The document-port control codec encoded `…WireV1{…}.to_value()` and so wrote its receipts in struct
order; the TypeScript closed grammar re-encodes in key-byte order and refuses anything else. On B1 no document port could
be bound, so nothing was ever written to an attached folder. My reading in S5.14 ("pack edits predate B0") was wrong: the
os-side encoder was saved 05:52.

**Fix on disk (wave `f9`, train line 10:18:03, apply-only; train FRAMEWORK GREEN through it).**
`🔌️plugin/📡️backbone/🔗️binding/🦀️.rs`: `canonical_control_bytes` orders the control map's members by key bytes; both
encoders use it; both readers (`binding-noncanonical`, `receipt-noncanonical`) compare against it — their old
`encode(decode(x)) != x` had become vacuous for member order. The peer's encoder and the TypeScript grammar are untouched.
Reaches the browser only with a guest rebuild (B2).

**Laws.** Binding fixture `codec.control`: six control maps with their wire bytes (wave `golden`, served 10:01). TypeScript:
`💻️os/🧪️tests/🧪️document-port-control-turn` pins encode + decode of every row (in `test-channel-oracles`). Rust:
`document_backbone_control_maps_are_the_golden_wire_bytes` and
`document_backbone_controls_are_ordered_by_key_bytes_whatever_the_pack_encoder_writes` in the binding's unit tests, plus a
standalone crate `T/🧪️s5-load-control-wire/` reading the same rows through the plugin's public codec — WRITTEN, NOT RUN
(gate closed at every attempt so far). Not yet done: S5-CHANNEL's hostile TS row (a struct-order receipt in the fixture).

**What the person sees (wave `attach-told`, served 10:17).** After the 09:53 wave E2E still saw no notice and an unhandled
rejection, because the sync card's Attach button does not use the attach action: ShellHost passed
`onAttach={openSyncTarget}` and the card drops the promise. The card, the attach action and the reconnect offer now share
`attachSyncTarget`, which logs `[os-shell] sync attach failed <bind's own error>` and shows `ui.sync.attachFailed`.
renderer-react tsc 0 errors at 10:17; served module verified. No automated law covers this wiring — E2E's re-probe is its
proof, and at the time of writing I have not seen that re-probe.

**My own frames with the same exposure (a reader that re-encodes and compares, or hashes, pack bytes whose member order now
follows the author):**
| Frame | Exposed? |
|---|---|
| document-port control command / receipt | YES — fixed above |
| `DocumentArchiveLoadStatus.fault`, `AppFrame::Error.fault` (pack-encoded `Fault`) | no — TS reads by key (`decodeFaultFromWire`), never re-encodes; `mergeDocumentArchive` reads `.code` the same way |
| document archive bytes, `.spr`, load / merge commands and statuses | no — positional binary framing, no maps |
| `merge_persisted_history`'s "is this the same document" test (digest of the archive's initial pack against the store's) | NOT across encoder versions: an archive written by a guest that ordered members is read by one that does not as ANOTHER document, and the route then LOADS it instead of merging. Same-build peers are unaffected. Not fixed (the digest is S5-STORE's); listed for the coordinator |
| folder archive "write only when the bytes differ" (STORE's AA) | no — one guest's archive bytes are deterministic |

**The merge route on disk.** Rust chain `f16 → f6 → merge-guest → merge-host → merge-law` applied 10:42:02–10:42:47 in one
apply-only hold (train line 10:43:02, 17 files); `merge-shell` under `serve` 10:49:30 (10 files). Re-based in the hold: the
kernel notice table had grown to 85 rows, so f16's anchor now reads the declared length (85 → 86).
Changed while staging: the guest merge no longer removes the operation from its registry across awaits
(`merge_decoded_document_archive(operation)` takes the pack and the decoded log out and answers `(taken, ahead)`).

| Run | Result |
|---|---|
| train `--lib` (kernel, plugin, wgpu renderer, ui) | FRAMEWORK GREEN 10:50:43, through a line after mine |
| `bun ./📜️script.ts test-channel-oracles` after `merge-shell` | 22 pass / 0 fail / 2185 expects (load host, attached replacement, control turn, folder persistence, folder read-back) |
| `python3 T/🧪️s3-load-archive-host.py` on the landed corpus | 18 cases, 0 failures |
| `bunx tsc -p T/🧪️s3-w2c-typecheck-wgpu-host.tsconfig.json` | 0 errors |
| renderer-react tsc after `merge-shell` | 13 errors, none in this WP's files: all `🧱️elements/🛠️ShellHelpers/🟦️.tsx` 5757–5801 (`iconId`) |
| `bun test …/🎠️kernel/🧪️tests/🧪️framework-notices/🟦️.ts` + manifest `fault-notices` | 8 pass / 1 fail — row 10 `toolGesture.slot-poisoned` does not fit the fixture schema's code pattern (a peer's row; my `members-differ` row fits) |
| served modules (6012) | ShellHost carries `entry.readBack.archive(`, PluginRuntime `mergeAppDocumentArchive` |

**NOT RUN — the merge route is unproven until these pass (gate closed 10:53–10:57, retried after):**
`cargo test -p semio-framework-plugin --lib --features artifact-app-testing -- folder_reload_route a_merge_archive_command document_backbone`
(5 route laws incl. the two-programs law, the dispatch law, the binding golden laws) and
`cargo test -p semio-framework-os-kernel --lib -- the_document_archive_load_host`.

**A detour that cost about twenty minutes and proved nothing.** To type-check the never-compiled chain before the train I
built an isolated copy-on-write clone outside the repository (`T/🧪️s5-load-clone.py`, 300 117 files, 5 min) and ran a cold
private `cargo check`; after 8.7 min it had reached 81 of the closure's units, far from the plugin crate, so I removed it and
landed through the train instead. The script stays as an input; it is not worth using under fleet load.

### S5.16 State at 11:01 — what is proven, what is owed, what is open

**Proven live on B1 (S5-E2E batch G, `🗑️generated/s5-e2e/run6-react-en-G.txt`, after the 10:17 host save):** the notice
`sync.attach.failed` ("The document could not be attached") is shown; `no-uncaught-page-errors` PASS (count 0); the console
carries `[os-shell] sync attach failed Error: actor-document-control.noncanonical` (the bind's own failure) beside
`[plugin-runtime] a document port that failed to bind could not be retired …`. The attach itself still fails on B1 — the
guest half of F9 is in the tree, not in the served wasm.

**Added after S5.15:** the control turn law gained "a receipt whose members stand in struct order names the receipt schema
and fails as noncanonical" (test directory, no lock; `test-channel-oracles` 23 pass / 0 fail / 2167 expects) — S5-CHANNEL's
hostile row, built in the law from a golden receipt instead of stored in the fixture.

**OWED — nothing below was run; each attempt met a closed gate (3 shared cargos, then the 25 GiB floor: 23 GiB free at 11:01)
and since 10:56 the plugin `--lib` is red on a peer's half-applied member-lane change (`send_member_mutations` with four
arguments, `ChildPackEntry.owner`; not this WP's lines — every landed wave of mine still checks `0 file(s) to write`):**
1. `zsh T/🚦️gate.sh 3 25 && CARGO_BUILD_JOBS=3 CARGO_INCREMENTAL=0 RUST_MIN_STACK=268435456 cargo test -p semio-framework-plugin --lib --features artifact-app-testing -- folder_reload_route a_merge_archive_command document_backbone`
   — the reload law, F16's notice law, F6's two laws, the two-programs merge law, the dispatch law, the two binding laws.
   The merge route and the F9 golden bytes are UNPROVEN in Rust until this passes.
2. `zsh T/🚦️gate.sh 3 25 && CARGO_BUILD_JOBS=3 CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-kernel --lib -- the_document_archive_load_host` — the kernel host corpus law with the three merging rows.
3. `zsh T/🚦️gate.sh 3 25 && CARGO_BUILD_JOBS=3 cargo test --manifest-path T/🧪️s5-load-control-wire/Cargo.toml --offline -- --nocapture` — the golden control bytes through the plugin's public codec.
4. Carried over: the two composed load laws (after S5-NESTED's fixture), `cargo test -p semio-framework --lib -- framework_notices a_foreign_document_schema`, MCP `long::a_long_history_document_loads_through_polls` and `--test binding_cancellation_law`.
5. No law yet: the guest merge's member comparison (identical members merge; differing members answer `plugin.document-load.members-differ`) — the toy document of the route law has no members.

**Open, not started in code:** F17 (`child:<slot>` in `depends_on`, MCP `infer_real` reads declared slots only,
`inference.children-stale`); §21.6 stepped media-export job (design in S5.3); F11a.

**Limits that ship with the merge route:** a composed document's read-back falls back to LOAD (`members-differ`) until
members merge (S5-NESTED); the merge's log fold is one store call, not stepped (S5-STORE); an archive written by a guest
whose pack encoder ordered members reads as another document to one that does not (S5.15 table) and is then loaded, not
merged; the wgpu shell has no read-back route (S5-WGPU); the ShellHost wiring of the read-back route and of the automatic
check-in's deferral has bun laws for its rule classes but no component-level law — live probes are their proof.

**Scripts of this WP (inputs, kept):** `T/🧪️s5-load-waves.py` (every wave: `check | land | restore`), `T/🧪️s5-load-mirror.py`
(guarded off-tree mirror for bun laws), `T/🧪️s5-load-clone.py` (isolated clone; not worth using under load),
`T/🧪️s5-load-control-wire/` (standalone golden-bytes crate), `T/🧪️s3-load-archive-host.py` (host model, knows merges).

### S5.17 After the reboot (resumed 16:46, build B2 live) — the one red law, F13, F5 remainder

**Live on B2 (S5-E2E run 7, relayed by the coordinator):** F9 closed (attach writes the archive, reconnect after reload), F4
closed through the merge route (two peers converge through an open history edit), F6 closed (stepped load status, Cancel
keeps the previous document). Law runs on the shared binaries before this turn: kernel `the_document_archive_load_host` 1/0,
plugin 10/1.

**(1) The one red — fixed, verified.** `document_backbone_binding_reducer_preserves_generation_and_live_owner` wrote its
instance-zero command through the pack encoder in struct order, which the control codec refuses since `f9`. It now uses the
codec's own encoder (wave `f9-test`, test-only, train line 16:46:58). RAN:
| Run | Result |
|---|---|
| `cargo test -p semio-framework-plugin --lib --features artifact-app-testing -- document_backbone folder_reload_route a_merge_archive_command` (gate `2 18`, 94 s) | **11 passed, 0 failed** (5 binding laws, the dispatch law, 5 route laws incl. the two-programs merge law) |
| `cargo test --manifest-path T/🧪️s5-load-control-wire/Cargo.toml --offline -- --nocapture` | **1 passed**; all six golden control maps: Rust writes exactly the fixture's bytes and reads them back (the crate's path depth was one `..` short and is fixed) |
With this the OWED list of S5.16 items 1–3 is paid.

**(2) F13 — a load the person cancelled was announced as a failure.** Cause: the read-back handler treated every rejection
of the load alike (alert "Document restore failed: <raw error text>" + `console.error`). Landed (wave `f13`, `serve`
17:01:58 → 17:04:34, 7 files, restore `python3 T/🧪️s5-load-waves.py f13 restore`):
- `FolderReadBackRouteV1.archive` answers how a read-back ended — `held`, `superseded`, `cancelled` — and only a real
  failure rejects; only the LOAD's own rejection is read as a cancel. Corpus `🧫️folder-read-back` gained `outcomes` for every
  case and the cases `a-cancelled-load-is-no-failure-and-leaves-the-next-archive-a-load` and
  `a-failed-load-fails-its-read-back-and-adopts-nothing` (11 cases).
- ShellHost: a cancelled read-back raises the polite `shell.documentTransfer.load-cancelled` notice the file-open path
  already has (en / de: "Loading “{file}” was cancelled; the previous document is unchanged."), no alert, no console line.
  A failed one keeps its alert, told by the program's own fault notice (`appFaultNoticeV1`, placeholders filled from the
  fault) or else the `load-failed` label — never the failure's engineering text, which stays in `console.error`.
- `AppChannelClient` rejects a refused or faulted load / merge with `DocumentArchiveFaultError` carrying the decoded fault.
| Run | Result |
|---|---|
| read-back + load-host laws in the guarded mirror, before the hold | 13 pass / 0 fail; 6 of 6 mutants killed (cancel is a failure, cancel adopts, cancel reads superseded, every failure a cancel, a cancel-shaped re-read failure a cancel, fault loses its structure) |
| `bun ./📜️script.ts test-channel-oracles` in the hold | 24 pass / 0 fail / 2134 expects |
| renderer-react `bunx tsc --noEmit -p tsconfig.json` in the hold | 0 errors |
NOT proven: the ShellHost wiring itself (notice instead of alert) has no component-level law — E2E's batch I is its proof and
has not run on it yet. The bootstrap frame still prints the WORKER's own failure messages raw (`${failed}: ${message}` for
`artifact-bootstrap-failed` events from the store worker) — not this route, not changed.

**(3) F5 remainder — decided, and the reading was wrong.** The shell status frame did not show because a folder read-back
load fed it NOTHING, not because 190 ms is under a refresh floor: only the file-open path passed its polled statuses into the
history projection (`programHistoryProjectionsWithLoadV1`). The restore ports now do the same for every archive restore, so
the frame shows from the first status the host polls until the load ends. Decision, written into the corpus note: a load that
ends within its first poll may show no frame; any longer load shows it. Not proven live.

**Rule 68 violation, mine (17:01:58–17:04:34).** `T/🗑️generated/coord/activation.flag` was created 16:56; my landing command
printed "ACTIVATION FLAG SET" and went on to save, because the check did not gate the commands after it. Seven files were
saved in the window, five of them served (`🏛️ShellHost/🟦️.tsx`, the document admission module, `💻️os/🟦️.ts`, the read-back
corpus and its schema), no `.rs`. No cargo was started after 16:51. Reported to the coordinator at once; nothing restored
(a restore is two more served saves inside the same window).

**A hazard found while fixing F13 — not changed, needs a decision.** After a cancelled (or failed) FIRST load the document
stays attached with the program's own document, and the next published batch writes that document over the folder's archive
(run 7 batch I: `105.1s HTTP 200 PUT` right after the cancel) — the folder's history is replaced by a page that declined to
load it. Two coherent rules: (a) a declined load detaches the folder (keeps the remembered binding, so the reconnect offer
returns); (b) the folder is not written until a read-back is adopted, and the status says so. I recommend (a).

### S5.18 The coordinator's decision on the overwrite hazard — a first archive that is not loaded detaches the folder (17:11)

**Decision (a), landed** (wave `detach`, `serve` 17:11:09 → 17:11:31, 6 files; activation flag absent, and the landing helper
now refuses `land` / `restore` while `🗑️generated/coord/activation.flag` exists; restore `python3 T/🧪️s5-load-waves.py detach restore`):
- Route: when the FIRST archive of an attachment is not loaded — the person cancelled it, or it failed — the route asks its
  `detach` port and the read-back ends `detached`; later events of that attachment drive nothing. Once the folder is adopted,
  a cancelled load of a later read-back ends `cancelled`, a failed one rejects, and the document stays attached and written.
- Corpus `🧫️folder-read-back` (14 cases): a cancelled first load and a failed first load detach and keep the folder
  remembered (the route has no port that forgets); a reconnect after a declined first load loads again; a cancelled later
  load keeps the adopted folder attached and written; a failed later load fails its read-back and keeps it attached.
- ShellHost `detach`: closes the document's attachment without forgetting the folder (the reconnect offer returns), clears
  the sync card when it names that document, and raises ONE polite notice, `shell.documentTransfer.folder-detached` (en / de:
  "The folder was detached because its document was not loaded; nothing is saved to it. Reconnect the folder to load its
  document."); a failed first load is told first — the program's own fault notice, else the `load-failed` label — and logged.
| Run | Result |
|---|---|
| read-back law in the guarded mirror, before the hold | 8 pass / 0 fail (with the persistence law); 6 of 6 mutants killed (stays attached, a later cancel detaches, a detached attachment still drives, a failure told as a cancel, a later failure swallowed, a declined load adopts) |
| `bun ./📜️script.ts test-channel-oracles` in the hold | 24 pass / 0 fail / 2184 expects |
| renderer-react `bunx tsc --noEmit -p tsconfig.json` in the hold | 0 errors |
| `bunx tsc -p T/🧪️s3-w2c-typecheck-wgpu-host.tsconfig.json` after the hold | 0 errors |
NOT proven: the ShellHost port itself (close without forgetting, the sync card, the notice) has no component-level law; E2E's
batch I must show after a cancelled first load: no PUT, card detached, reconnect band offered, notice `folder-detached`.
**Limits.** (1) A published batch in the few milliseconds between the bind and the first answered folder read is still
written before anything is adopted (the persistence policy does not wait for adoption). (2) A load declined before the
attach committed is detached but was never remembered, and its attach gesture then reports "could not be attached".
**Open, untouched:** F17, §21.6 stepped media export, the wgpu read-back route twin (S5-WGPU).
