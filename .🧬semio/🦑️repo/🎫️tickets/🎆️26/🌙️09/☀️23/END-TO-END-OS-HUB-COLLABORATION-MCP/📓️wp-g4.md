# WP-G4 — AI Over The Semio MCP (Session 10)

Slice G4 · session 10 · 2026-09-23. Ports: hubs 7830–7839, serves 6330–6339.
Continues GJ3 (inference_run hang), GJ2 (zero-touch staging), M10b (hub agent participant).

## 0. Headline

- **Green, measured:** stdio probe **17/17**, including in-flight cancellation (CANCELLED 3 ms after
  `notifications/cancelled`). `live-agent-loop-check` **20/20** on a note `s` serve (:6330). `hub-agent-participant`
  **17/17** on hub :7830 (H2 binary). os-mcp rust tests: fundamental 39/39 and quick 434 + fixes. Kernel sync 61/61
  under both cargo test and nextest isolation. Plugin-host relay 26/26. wfc-bitmap lib 151/151.
- **Agent edit seen by a human:** the human's live document socket receives the agent's `Commands` relay (the agent's own
  actor, after the invoke). The hub ledger status does **not** advance (`head_seq` 1→1), handed to H2 with evidence.
- **Root-fixed:** MCP cancellation never reached a running inference (3 defects, §3.2). Also fixed: stale os-mcp test,
  stale schema mirror, a gate input bug, two kernel sync tests that never registered their codec, and GJ3's wrong wfc
  admission rule plus its genesis misdiagnosis.
- **client-e2e 38/38** after W1's wfc rebuild (`e36b9d88…`, my admission fix + G5's infer Pump). The earlier 34/37 note, kept for history: The `inference_run` row on the plugin's genesis artifact (16×16, N=3, sym 8,
  24×24) does not finish within 240 s (it also runs past 540 s). Natively the same pair solves in 16.6 s (regression test
  added). A host `sample` shows guest code in ~10% of samples while the host main thread is parked 90%, so the
  pooled cold-relay per-crossing overhead dominates. `workUnits=6e6` did not help. See §8.

## 1. Inherited State

| source | state at session-10 start (measured from captures) |
|---|---|
| GJ3 | pool-pump root fix in source; standalone wfc solve PASS 42.6 s on a manually uplifted guest; client-e2e 34/37 stopped at `inference_run` 240 s |
| M10b | hub 7651 (dead after reboot): `hub-agent-participant` **17/17**, ledger proof `head_seq=1 commit_seq=1`, `live-agent-loop` 20/20 on 6080 — all with `[DEBUG] m10b` file-logging left in product code |
| GJ2 | zero-touch `ensureMcpBinary`; 28 tools / 7 resources / 5 prompts |

## 2. Wfc Guest Rebuild (via W1)

- Request `wp-w1/requests/g4.txt` (wfc + note, product verbs only). W1 rebuilt both through `nx run <p>-plugin:describe`
  on the shared target: staged wasm-dev sha256 == committed `hashes.wasmSha256` (wfc `ce48bc9f…78ad`, note `4084a258…fe96`).
- The staged wfc (19:43, `ce48bc9f`) already carried the GJ3 genesis/admission/while-Submitted fixes; W1's product rebuild
  reproduced the same bytes, so no manual uplift is in play any more.

## 3. Gates

| gate | target | measured | capture (`wp-g4/generated/`) |
|---|---|---|---|
| os-mcp rust `test` (fundamental) | green | **39/39** | `g4-osmcp-rust-test.txt` |
| os-mcp rust `test quick` | green | 434/435 → the 1 red (stale test, see §3.3) fixed + re-run targeted **3/3**; notify **27/27** | `g4-osmcp-rust-test-quick.txt`, `g4-osmcp-targeted.txt` |
| plugin-host guest-cold-relay | green | **26/26** (2 new) | `g4-test-host-relay.txt` |
| kernel sync (`--features sync,ureq`, cargo test + nextest `test(sync)`) | green | **61/61** both (was 60/61 cargo, 2 red nextest; see §3.4) | `g4-kernel-sync.txt`, `g4-kernel-nextest-sync.txt` |
| wfc-bitmap lib (native) | green | **151/151** | `g4-wfc-bitmap-lib.txt` |
| stdio probe transcript | full chain | **17/17** (`g4-probe4`) | `g4-probe4.txt`, `g4-probe4-transcript.json` |
| live-agent-loop-check (note serve :6330) | green | **20/20** | `g4-live-agent-loop.txt` |
| hub-agent-participant (hub :7830, H2 binary) | 17/17 | **17/17** | `g4-hub-agent-participant.txt` |
| client-e2e | real total | **38/38** on W1's rebuilt wfc `e36b9d88…` (my admission fix + G5's infer Pump); the genesis `inference_run` now SUCCEEDS inside 240 s (was 34/37) | `g4-client-e2e-4.txt` (earlier `-3`) |

### 3.1 Probe transcript (stdio, `wp-g4/g4-probe.ts`)

initialize (semio@0.1.0, 2025-06-18) → tools/list 28 → resources/list 7 → prompts/list 5 → context_resolve (headless) →
artifact_create `s.note.note` → artifact_open → action_prepare `addBlock` → action_invoke SUCCEEDED → artifact_snapshot
(spr 223→718) → history_undo → history_redo → transaction begin→commit (snapshot moved) → begin→rollback (unchanged) →
inference_run `s.wfc.bitmap.solve` 6×4 SUCCEEDED with progress `[0.05,0.15,0.25,0.35,1]` → second inference_run 96×96,
`notifications/cancelled` sent after its 0.35 row → **CANCELLED 3 ms after the cancel** → ping answers.

### 3.2 Root cause fixed: in-flight cancellation never reached a running inference

Measured before the fix (`g4-probe2`/`g4-probe3`): the 96×96 solve ignored `notifications/cancelled` and ran to SUCCEEDED
79–108 s after the cancel. There were three independent defects:

1. **stdio serve loop is sequential.** `StdioTransport::serve` dispatches one line at a time, so a `notifications/cancelled`
   sat in the reader channel until the call it names had already returned. Fix: `protocol::intercept_cancellation` runs on
   the stdio reader thread and applies the cancel out of band (`🚚️transport`, `🧭️protocol`).
2. **Request→job binding only existed with a progress token.** `ProgressBinding.request_id` tied jobs to the request only
   when `_meta.progressToken` was present. Fix: separate `notify::RequestScope`. Every `tools/call` binds its request id.
   A cancel that overtakes its request is remembered (bounded to 256) and cancels the job the moment it is minted
   (`notify::cancel_request`).
3. **The job cancel had no path into the guest.** `JobRegistry::request_cancel` only flipped a flag. The host relay ran
   under `root_cancel_token()`, which nobody could cancel. Fix: `InferCommand.cancel: InferenceCancel` (a CancelToken).
   `inference_run` binds it with the new `JobRegistry::bind_cancel` hook, and `PluginArtifactChannel::infer_real` →
   `ArtifactInferenceRouter::infer(request, &cancel)` → `PluginInstanceHandle::infer(request, &cancel)` →
   `run_job_on_worker(kind, input, cancel.child_now())`. Other cold kinds pass their own root token. The relay runs under
   a CHILD, so a finished route never cancels the caller's token (a dependency chain shares it).
   The stale comment claiming the guest polled `cancellationId` was deleted.

Laws: `🌉️mcp/🖥️ui/🧫️fixtures/🛑️job-cancel-hook-law.json` (language-agnostic, 5 cases) + Rust replay; notify tests for the
no-token cancel, overtaking cancel, out-of-band interception; host tests `a_callers_cancel_stops_a_mounted_infer_blocked_in_a_pending_guest_step`,
`a_finished_mounted_infer_leaves_the_callers_token_live`. The live stdio probe is the black-box JSON-RPC check.

### 3.3 Stale tests/gates fixed

- `workspace::quick::authenticated_hub_workspace_resources_…` expected `semio://artifact/{id}/schema` on a hub document
  to be `PluginUnavailable`. M8 made it answer the descriptor's schema/kind. The test now asserts the real answer.
- Schema mirror (`🌉️mcp/🧬️schema/🔣️.json` + `🟦️.ts`) was stale after M10b's `sessionDocument`. Regenerated with the
  product verb `schema-mirror`.
- `hub-agent-participant` step 11 sent `input: {}`, which only worked for a gis verb. It now uses
  `capabilities_describe` → `minimalInputForSchema` (now exported from `🌉️mcp/🟦️.ts`), the same as client-e2e.
  15/17 → 17/17.

### 3.4 Kernel sync test (reported by H2)

`detach_drains_pending_outbound_operations` failed 3/3 when run in isolation. The test never registered the `demo/v1` codec,
so `start_connect_hub` returned before admission on every retry. Measured with temporary instrumentation (removed):
`no codec demo/v1` ×4. The test only passed when an earlier test had registered the codec. Fix:
`spawn_mock_hub_with_session_gate` calls `ensure_demo_codec_registered()`. R1 then reported that under nextest process
isolation `hostile_hub_binding_cannot_receive_a_credential_bound_document_grant` failed for the same reason
(admissions 0); it now registers the codec too. `nextest -E 'test(sync)'` 61/61.

### 3.6 wfc: GJ3's admission rule and genesis premise

- GJ3's rule `pattern > min(w,h)` refused valid periodic samples. It broke
  `a_sample_that_cannot_tile_the_output_reports_a_contradiction` (a 3×1 periodic input). The rule is now
  `!periodic_input && pattern > min(w,h)`, which matches the extractor's own window enumeration.
- GJ3 changed `BitmapInput`/`BitmapOutputSpec` defaults to 2×2 because it believed the client-e2e artifact was the 1×1
  default. It is not. The genesis pair `artifact_create` answers (captured live) decodes to a **16×16 3-colour, N=3,
  sym 8, 24×24 periodic** demo. The defaults are reverted to HEAD, along with GJ3's in-definition comment.
- New regression `the_artifact_bound_genesis_document_resolves_and_solves` (the live-captured pair) proves that the
  artifact-bound carrier decodes and solves natively. It flaked once as `job-session.terminal-fault` under a
  parallel run at load ~30 (4 ms headless step budget), then passed 2/2.
- These are wfc guest source changes, so a rebuild is re-requested from W1 (`wp-w1/requests/g4.txt`).

### 3.5 Cold-compile budget (observation)

The first `inference_run` after a new wfc guest pays a wasmtime compile of the 129 MB component. That took 258–395 s at
load ~35, silent after progress 0.35. The result is cached as a `.cwasm` in `~/.semio/cache/wasmtime` (warm solve 8–15 s).
client-e2e's 240 s inference budget is therefore only met warm.

## 4. Hub Agent Participant Live

Hub **:7830** runs a local copy of H2's 00:16 binary (`wp-g4/bin/os-hub`) on an APFS clone of the HC1 catalog root
(`.🧬semio/🌐hub/g4-boot`). It is ready with `mcpWorkspace: true` and CatalogResolved 13/13. `hub-agent-participant`
17/17 on the first binary and again on this one (`g4-hub-agent-participant-2.txt`).

`wp-g4/g4-agent-edit-human-sees.ts`, run 4 (`g4-agent-edit-human-sees-4.txt`), 6/7:
- The human signs in, delegates an agent, and holds a live document socket on the note. The sequence is H2's contract:
  open-plan 200 → socket-grants 200 → `/scopes/{s}%2F{d}/document/ws?surface=…`, `semio.session.v1`, SocketHelloV1 →
  Welcome/Session.
- The agent, over the semio MCP `--hub` delegated session, opens the same document (`writePath=open`) and commits
  `addBlock` → SUCCEEDED, `relayedBatches=1`.
- **The human's socket receives the agent's live `Commands` frame after the invoke**: 1 envelope, `document_id`
  `artifact-4445…`, actor `hub.v1.51fb…`, which is the agent's own socket actor, not the human's.
- **Red:** `GET /spaces/{s}/documents/{d}` stays `head_seq=1 commit_seq=1`, and a fresh human's Welcome catch-up replays
  only 1 Commands frame after 4+ accepted agent edits. So the hub relays without the commit becoming visible to
  status/catch-up. The evidence and a repro command went to H2, which owns hub routing.
- The agent's native actor sent no presence beat, so the human saw an empty presence roster.

## 5. `[DEBUG]` Cleanup

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs`: removed `m10b_debug` (file logger into the ticket folder)
  and all 40 call sites, restored the original `?`/let-else shapes, dropped M10b's in-definition comments.
  `cargo check -p semio-framework-os-kernel --features sync` green (4 pre-existing warnings).
- `🌎️hub/📄️documents/🦀️.rs`: removed the three `[DEBUG] m10b submit` `writeln!` blocks (before H2 announced it is
  deleting this module); `cargo check -p semio-hub` green. Script: `wp-g4/strip-m10b-*.py`.
- No GJ3 `[DEBUG]` left in `🌉️mcp`/`🔌️plugin` reactor/infer (remaining `[DEBUG]` lines there are peers' test output).

## 6. Pids (left running for peers/coordinator)

| pid | what |
|---|---|
| 4067 / 4072 | hub hold (bun) / os-hub on :7830 (`g4-boot`, `wp-g4/bin/os-hub`) |
| 42187 | note react dev serve on :6330 (`wp-g4/g4-serve-note.sh`, local-only) |

## 7. Files Changed

| path | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs` | M10b `[DEBUG]` logger + 40 call sites removed |
| `…/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs` | mock hub + hostile-binding test register the `demo/v1` codec |
| `🌎️hub/📄️documents/🦀️.rs` | three M10b `[DEBUG]` writeln blocks removed (H2 is deleting the module) |
| `…/🔌️plugin/🖥️host/🦀️.rs` | `run_job_on_worker(…, cancel)`, `PluginInstanceHandle::infer(request, &cancel)` (child token), `ArtifactInferenceRouter::infer/infer_with_visited(…, &cancel)` |
| `…/🔌️plugin/🖥️host/🧪️tests/🔬️guest-cold-relay/🦀️.rs` | call sites + 2 cancel laws |
| `…/🌉️mcp/🔀️dispatch/🦀️.rs` (+ quick test) | `InferCommand.cancel: InferenceCancel` |
| `…/🌉️mcp/💡️inference/🦀️.rs` | job cancel hook bound to the inference token; cancelled-on-error arm; stale comment removed |
| `…/🌉️mcp/🖥️ui/🦀️.rs` (+ quick test, `🧫️fixtures/🛑️job-cancel-hook-law.json`) | `JobRegistry::bind_cancel`, hook fired by `request_cancel` |
| `…/🌉️mcp/📣️notify/🦀️.rs` (+ quick tests) | `RequestScope`, `cancel_request`, overtaking-cancel memory; `ProgressBinding.request_id` removed |
| `…/🌉️mcp/🧭️protocol/🦀️.rs` | `intercept_cancellation`, request scope per `tools/call` |
| `…/🌉️mcp/🚚️transport/🦀️.rs` | stdio reader applies `notifications/cancelled` out of band |
| `…/🌉️mcp/🏠️workspace/🦀️.rs` (+ quick test) | router call passes the command's token; stale hub-schema test updated |
| `…/🌉️mcp/🧬️schema/🔣️.json`, `🟦️.ts` | regenerated via `schema-mirror` |
| `…/🌉️mcp/🟦️.ts`, `🧪️tests/🤖️hub-agent-participant/🟦️.ts` | `minimalInputForSchema` exported; gate derives input from `capabilities_describe` |
| `✏️s/🔌️plugins/🀄️wfc/…/🖼️bitmap/…/💡️inferences/🦀️.rs` (+ unit test) | periodic-aware admission; genesis regression test |
| `✏️s/🔌️plugins/🀄️wfc/…/🖼️bitmap/…/📸️snapshot/🦀️.rs` | GJ3 default change reverted (now equals HEAD) |
| ticket scripts | `wp-g4/g4-probe.ts`, `g4-agent-edit-human-sees.ts`, `g4-hub-hold.ts`, `g4-serve-note.sh`, `strip-m10b-*.py`, `thread-inference-cancel-*.py`, `out-of-band-cancel*.py` |

## 8. Honest Gaps

1. **(Resolved by the rebuild; client-e2e 38/38.) Earlier genesis inference throughput.** client-e2e runs `inference_run` on the artifact that
   `artifact_create s.wfc.bitmap` mints: the 16×16, N=3, sym 8, 24×24 demo.
   - Measured: native debug `solve_with_job` takes 16.6 s. Over MCP (warm `.cwasm`) it exceeds 540 s
     (`g4-artsolve.txt`), and `workUnits=6000000` makes no difference (`g4-artsolve-wu.txt`). A payload solve of a
     4×4 → 6×4 problem takes 8–15 s.
   - `sample` of the MCP process mid-solve (`g4-artsolve-sample.txt`): guest wasm fiber in 62/614 samples, the main
     thread parked in `block_on(ArtifactInferenceRouter::infer)` 555/614, and pool workers idle or contending on the
     maintenance mutex.
   - Conclusion: each `step-job` crossing (RELAY_JOB_BUDGET = 2 ms wall) pays roughly 10× its compute in relay
     overhead. Next step: profile one crossing on the host relay (publication paging, pool wakes, the 1 ms
     cancellation poll timer in `wait_for_guest_relay_cancellation`, epoch ticks), and do not raise the 2 ms slice
     (the framework's 8 ms ceiling). The rows that follow from it are red for the same reason: client-e2e's post-hoc
     cancel sends `requestId: 0`, because its timed-out envelope has `id: null`, and ping waits behind the still-running
     call.
2. **Hub ledger does not advance** after an agent commit that is relayed live (§4). This is H2's area; the evidence has
   been sent.
3. **Browser human view** was not driven. The human observer is a second live document connection, not a React shell.
   C7 owns browser collaboration.
4. **Cold compile:** the first inference after a new wfc guest pays a 258–444 s wasmtime compile at load ~35, with no
   progress rows during it.
5. **Agent edit identities are deterministic across processes (H2 root cause of the frozen ledger).** H2 measured the
   agent's relayed envelope `mutation_id` = `edit-2bb425fa800a3b7f`, the document's FIRST committed edit. The hub
   dedupes batches by command id, so every agent edit is an idempotent replay: Accepted, nothing written. My own
   captures show the same thing locally. The transaction member `editId` is `edit-e73113e9c5251c7e` in all 5 probe runs
   (`g4-probe*.txt`), hours apart, in fresh processes and folders.
   - The Store's own mint (`🏪️store/🦀️.rs` `edit_id()`) hashes actor + sequence + HLC, so a constant id means a constant
     actor, sequence 1, and a clock that is not advancing inside the guest.
   - The owned-interpreter runtime writes zeroes for `wasi:clocks/wall-clock.now` (`🔌️plugin/🖥️host/🦀️.rs` ~1704). I did
     not confirm whether the wasmtime path hands the guest a real clock.
   - Fix direction: a hub-bound session must mint under a per-session identity. Either the guest store's actor becomes the
     hub socket actor, or the minted HLC takes a host-provided time or nonce. That is a guest-framework change and needs
     every guest rebuilt through W1. It is not landed.
   - H2 has landed hub relay suppression in source, with a law: a replayed batch is acked Accepted, not relayed, and
     commit_seq stays 1. It is not staged: `wp-h2/bin/os-hub` (00:16) still relays replays, and the next binary comes
     from H3, who now owns semio-hub. On that binary the §4 "human sees the relay" row will go red until this is fixed.
6. wfc guest source changed (admission) → W1 rebuild re-requested; client-e2e must be re-run after it.
