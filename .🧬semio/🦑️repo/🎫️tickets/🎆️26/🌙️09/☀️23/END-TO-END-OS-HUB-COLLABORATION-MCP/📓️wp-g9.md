# WP-G9 — Generic Inference Quartet (Session 10)

Slice G9 · session 10 · 2026-09-24. Ports: hubs 7920–7929, serves 6420–6429. Captures: `wp-g9/generated/`.
Continues peer-audit-g10 Outcome 4 row "`inference_submit/…` hardcoded to gis" (NOBODY) and audit-semio-mcp gap 7 (M6).

## Status

| Item | Status |
|---|---|
| 1. Read inherited state (G4–G8, audit, hub inference runtime, MCP quartet) | DONE |
| 2. Design: one service model | DONE (§1) |
| 3. Schema-first job/event/proposal wire + language-agnostic law + TS/AJV oracle | DONE in source; Rust law 4/5 green now, the 5th (commit action from the descriptor) needs the frozen hunk (§4) |
| 4. MCP quartet generic (guest + hub sites), `inference_run` on the same engine | DONE in source, `cargo check` green, targeted tests 178/179 (§3) |
| 5. Hub publishes its executed services (`features.inferenceServices`) | DONE in source, `cargo check -p semio-hub --bins --tests` green |
| 6. wfc `pin-solution` commit action | DONE natively (wfc-bitmap `--features component-app-assembly` targeted tests green) |
| 7. Frozen-crate hunk (framework plugin + manifest + projection + wfc contracts + MCP resolver) | PREPARED `wp-g9/g9-apply-commit-binding.py`; waiting for the coordinator's freeze lift |
| 8. Guest rebuild + catalog request to W1 | PENDING (filed after the hunk lands) |
| 9. Live proof over stdio MCP + hub | SCRIPT READY `wp-g9/g9-quartet-live.ts`; PENDING hub/catalog |
| 10. client-e2e + hub-agent-participant rerun | PENDING |

## 1. Design: one service model

- **Declared service.** The only source of services is the plugin descriptor roster, the same one `inference_list` reads.
  `select_inference_service(declared, hubServices, artifactKind, inferenceSchema?, pluginId?)` picks exactly one service
  and gives named refusals: `NOT_FOUND`, and `INPUT_INVALID` with field `inferenceSchema` or `pluginId`.
- **Execution site.**
  - `hub`: only for a service the bound hub publishes in `/readyz` `features.inferenceServices[] = {serviceId, route}`.
    Today that is gis, whose hub runtime executes natively and commits through its own composition transaction. The
    MCP builds every hub path from the published `route`, so it holds no table of its own.
  - `guest`: every other service. The plugin's guest runs it in the gateway, on the same engine as `inference_run`
    (`run_guest_inference`, shared).
- **One job model.** A `JobRegistry` job now has an append-only event journal: `accepted`, `progress`,
  `cancel-requested`, `awaiting-approval`, `approved`, `committed`, and one terminal event. It also has an
  `AWAITING_APPROVAL` status. Both sites answer the same page, `InferenceJobPageV1`:
  - state: accepted / running / succeeded / failed / cancelled
  - proposalState: none / offered / approved / stale / cancelled
  - events, progress fractions, cursor, proposal, result, commit
- **Proposal and approval.**
  - The guest proposal is the declared commit action (`InferencePayloadContract.commit.action`) plus exactly the result
    fields that action declares as arguments. It is digested with SHA-256 over key-sorted canonical JSON of
    `{domain, serviceId, documentId, capabilityId, input}`.
  - `inference_approve(hash)` consumes the approval exactly once (`resume_approved`). It then prepares and invokes the
    action through `ActionAdapter`, the normal edit path: the hub-bound document relays to the hub ledger, the action's
    own approval policy and undo token apply, and a refusal leaves the proposal `stale`.
  - `inference_cancel` stops a running job or withdraws an offered proposal.
- **wfc commit action.** A solve's pixels are inferred, never persisted, so the durable, shared commit of a solve is to
  pin every output cell (`pin-solution`, destructive, one `pin-pixel` per differing cell). Re-committing the same solve
  writes nothing.

## 2. Files Changed (so far)

| path | change |
|---|---|
| `🌎️hub/🏗️bootstrap/🦀️.rs` | readiness `features.inference: bool` replaced by `features.inferenceServices[{serviceId, route}]` |
| `🌎️hub/💡️inference/🧬️schema/🦀️.rs` | `GIS_SERVICE_ROUTE` |
| `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` | readiness law: published route is a registered route; unbound hub publishes `[]` |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | two readiness consumers moved to `inferenceServices` |
| `🌉️mcp/💡️inference/🦀️.rs` | GIS-named hub client renamed service-neutral (`HubInference*`, `InferenceJobStateV1`, …); paths and client functions take the published route; `read_hub_inference_services`; `HUB_INFERENCE_ROUTES` table removed; generic quartet handlers (guest + hub); `inference_run` on the shared engine (hub-executed service: submit + follow); `hub_inference_read` generic; module doc rewritten |
| `🌉️mcp/💡️inference/💼️jobs/🦀️.rs` (new) | service selection, proposal digest, guest-job record, page folds for both sites |
| `🌉️mcp/💡️inference/💼️jobs/🟦️.ts` (new) | independent Bun/AJV oracle for the law (wired into `inference-bridge-check --source`) |
| `🌉️mcp/💡️inference/🧬️schema/🔣️.json` | `InferenceJobPageV1`, `InferenceProposalV1`, `HubInferenceServiceV1`, `InferenceServiceLawV1`, … |
| `🌉️mcp/💡️inference/🧫️fixtures/💼️inference-service-law.json` (new) | language-agnostic law: 7 selection, 3 proposal, 5 lifecycle, 1 hub-page case |
| `🌉️mcp/💡️inference/🧪️tests/💼️inference-service-law/🦀️.rs` (new) | Rust replay + owned-validator check of every produced page |
| `🌉️mcp/🖥️ui/🦀️.rs` | `JobRegistry` event journal, `AWAITING_APPROVAL`, `await_approval`/`resume_approved`/`record_event`/`events` |
| `🌉️mcp/🏠️workspace/🦀️.rs`, `🔗️remote/🦀️.rs` | GIS kind gate removed; route-parameterised hub calls; `hub_inference_services` |
| `🌉️mcp/🔀️dispatch/🦀️.rs` | `HubInferenceApprovalUndoMemberV1` (+route), `retain_hub_inference_approval_undo` |
| `🌉️mcp/🧬️schema/🦀️.rs` | export `HubInferenceApprovalRequestV1` (mirror regen pending the binary) |
| `🌉️mcp/💡️inference-bridge/🟦️.ts` | reads the renamed export |
| `🌉️mcp/📦️packages/🦀️rust/📜️script.ts`, `🏠️workspace/🧫️fixtures/🔐️canonical-checkpoint-resource/🔣️.json`, `🏠️workspace/🧬️schema/🔣️.json` | removed the obsolete "GIS selector" from the checkpoint-resource gate |
| MCP tests (`💡️inference` quick + inference-jobs, `📣️notify`, `🔀️dispatch`, `🏠️workspace` quick) | follow the rename and the route parameter; the GIS-only route-table tests are replaced by id and route shape laws |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/…/✏️editor/🎮️commands/📌️pin-solution/` (new) + editor + crate root | `pin-solution` command, action, publication route, bounded-first-step proof, tests |
| `…/🖼️bitmap/…/💡️inferences/🦀️.rs` | `BITMAP_INFERENCE_COMMIT_ACTION` |

## 3. Measured

| gate | result | capture |
|---|---|---|
| `cargo check -p semio-hub --bins --tests` | green | — |
| `cargo check -p semio-framework-os-mcp --lib --tests` | green | — |
| os-mcp lib `inference ui:: notify:: dispatch:: workspace::` | 178/179; the one red is the expected pre-hunk commit-action row | `mcp-targeted-2.txt` |
| wfc-bitmap lib `--features component-app-assembly` (`pin_solution`, `editor::`) | new laws 2/2, roster law green; `the_solve_command_publishes_a_real_collapse…`, `every_example_solve…`, `a_whole_solve_grant…` fail with `job-session.terminal-fault` under load ~25–30 (same solver flake G4 §3.6 recorded), none of them touch my change | `test-wfc-bitmap-3.txt`, `-4.txt` |
| `inference-bridge-check --source` | blocked: the schema mirror needs the rebuilt binary (`HubInferenceApprovalRequestV1`) | `bridge-source-1.txt` |

## 4. Frozen-crate hunk (prepared, not applied)

`wp-g9/g9-apply-commit-binding.py` is one compile-atomic set:
- `🔌️plugin/🦀️.rs`: `ArtifactInferenceCommitBinding`, `ArtifactInferencePayloadContract.commit`, `WireInferenceCommitBinding`,
  the three conversions, and the re-exports.
- `🛂️manifest/🦀️.rs`: `InferenceCommitBinding`, `InferencePayloadContract.commit`.
- `📽️projection` TS type.
- The five wfc contracts: bitmap declares `commit: pin-solution`; the other four declare `None`.
- MCP `declared_commit_action` real body (the interim body answers `None`), plus two test literals.

After it is applied: `cargo check` plugin + wfc-bitmap (native + wasm32) + os-mcp, then the law 5/5, then the W1 request.

## Pids

| pid | what |
|---|---|

## Honest Gaps

- The live proof, client-e2e and hub-agent-participant wait on the freeze lift, W1's wfc describe/rebuild and a
  catalog that carries stdio, gis and wfc.
- The cost side of "approval for destructive/costly runs" is not modelled as a separate start gate. A commit is gated
  twice: by the owner's exact-hash approval, and by the commit action's own declared approval policy
  (`pin-solution` is destructive, so `WhenDestructive` applies; the live probe runs with `--auto-approve all`).
