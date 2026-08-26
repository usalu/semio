# Second Independent Typed-Command Full-Operation Audit

Date: 2026-08-26

## Verdict: RED

The narrow legacy tail is still fail-closed, and the two former `fault.message.as_bytes().to_vec()` copies are gone. That is real progress. It is nevertheless **RED** for the requested shared full-operation foundation: two reachable entry paths perform generic command identity/serialization before the gate; the claimed fault cap does not cap the retained fault owner; no live freshness grant, publication consumer, or ACK exists; and the fixture's new hostile rows are mostly declarations rather than exercised implementation/serde laws.

No production file was changed. No Cargo, Nx, Wasm, browser, cache, or Git-mutating command was run. This is source/static evidence only.

## Narrow Green Evidence

| Claim | Result | Source evidence |
| --- | --- | --- |
| Legacy root work remains behind the inner gate | GREEN | `dispatch_typed_command_inner` gates at `plugin/component.rs:19500`, before `A::command_id`, operation allocation, `refresh_cache`, snapshots, and worker dispatch. |
| Action and manifest adapters gate before app conversion | GREEN | `dispatch_action` gates before `A::command_from_action` at `19429-19431`; `dispatch_command` does likewise at `19458-19463`; intent conversion itself remains after the gate at `20755-20757`. |
| Old whole fault-message *copies* are removed | GREEN | The prior `fault.message.as_bytes().to_vec()` spelling is absent. `FaultEncoding` transfers the existing `String` and copies at most one scalar into a `[u8; 256]` buffer per step (`15232-15255`). |
| Step guard and scalar close mechanics exist | GREEN, but not complete law proof | Cancellation precedes deadline/fuel and one fuel unit at `15276-15285`; fault/description/coalesce close removes one Unicode scalar after exact `len_utf8` credit checks (`15442-15472`). |

## Blocking Findings

### RED-1 — The gate does not precede all generic identity/parsing work

`VcsArtifactApp::dispatch_typed` is public and reachable by in-process callers. It calls app-controlled `A::command_id(command)` and fully serializes the arbitrary command with `OpBinary::encode_op(command)` at `19648-19649`; only then does it calculate admission and reach the inner gate at `19501`. Thus a rejected typed command can execute generic identity and allocate/encode a whole command before fail closure.

`handle_intent_frame` also materializes a full `serde_json::Value` and `Vec<u8>` at `20751-20753` before its gate at `20755`. The action-name lookup is an acceptable fixed-address selection, but the generic JSON traversal/serialization is not a retained or pre-admitted operation.

The source regression test (`15770-15779`) checks only three app converter spellings. It omits `dispatch_typed`, `A::command_id`, `OpBinary::encode_op`, `serde_json::to_value`, and `serde_json::to_vec` in the route prefix, so it cannot prove the stated ordering.

Required correction: remove/replace the bare-value `dispatch_typed` entry with an owner-qualified, pre-admitted raw-wire entry. For UI intent input, admit a fixed manifest address and raw bounded page before any JSON materialization, then let the exact app-owned factory perform resumable decoding. Do not treat a post-serialization admission as a pre-decode gate.

### RED-2 — Fault detail is capped, but the retained fault owner is not

`begin_fault_encoding` moves an arbitrary `Fault.message: String` into `self.fault_message` (`15232-15237`). The 256-byte array limits only emitted detail; the complete source string remains retained until encoded or individually discarded by close. An app-owned future reducer can therefore transfer an unbounded message into this supposedly bounded operation without a copy. This closes the exact former-copy finding but not the bounded-resource requirement.

No fixture case or source test constructs an over-cap fault, verifies scalar-boundary truncation, verifies the retained owner is bounded, or proves close handback for that path. A fixed diagnostic code/detail, or an input-side bounded fault-page owner, is required before live reducer admission.

### RED-3 — No producer-to-mounted publication, live freshness grant, or ACK exists

`admit_exposure_freshness` is defined at `15264-15270`; no caller exists. `Expose` therefore checkpoints forever unless an external caller mutates the private job, and it never reads the live document revision/generation itself. `validate_commit` at `15404` consequently has no live admission source.

`MountedTypedCommandFullOperation::drive_worker_step` changes completed/rejected jobs to `Retiring`, then returns `Blocked { reason: "typed command awaits its bounded revision-validated publication turn" }` (`15068-15112`). `maintenance_step` only calls that method (`20316-20329`); there is no Retiring publication branch, no `ArtifactToolCompletion::take_emit`, no store/event/UI transfer, and no removal/close protocol for the completed typed operation.

`ArtifactToolCompletion` is a single-assignment/take-once cell (`13155-13204`), not a result-page protocol. There is no result-page token, acknowledge call, retry state, or receiver identity. The fixture's `publicationLaws` are only length-checked at `15726`; their `ack`, `retry`, and `retry-exhausted` rows execute no runtime behavior.

### RED-4 — Nonempty lanes and their cursors are intentionally non-runnable

The shared job faults on every nonempty task, artifact/config/draft, presence/transient, effect/event, and child lane (`15337-15365`). The retained item cursors are never used to transfer an item. `Ephemeral` and `Emit` merely increment lane counters (`15381-15397`); neither invokes a store, event sink, child coordinator, nor UI publication path.

This is safe fail closure, not one-item pagination or publication. It must remain classified as such.

### RED-5 — The fixture and serde oracle do not establish the advertised hostile laws

The seven output cases do prove byte arithmetic against the local serde-based census oracle (`15680-15718`), including the four-byte scalar. The remaining fixture sections do not prove implementation behavior:

- grant laws are checked only by array lengths, and cancellation only by equality of names (`15720-15741`), not by stepping every phase with cancellation, zero fuel, or expired deadline;
- freshness is equality of fixture JSON numbers (`15729-15732`), not `validate_commit` against a live revision/generation grant;
- saturation/missing-root are arithmetic comparisons (`15733-15739`), not registry/root admission;
- publication/ACK/retry rows are never interpreted;
- close tests exercise only the two string-credit rows (`15743-15755`), not the 13-owner handback or second-call terminal idempotence rows;
- no serde oracle covers fault truncation, all grant laws, live freshness, admission saturation, ACK/retry, or owner handback.

The `parent_document_id` close path additionally charges `String::capacity()` rather than its logical UTF-8 byte length (`15434-15440`). Allocation capacity is not an exact wire/owner byte measure and can exceed the caller credit unrelated to the document id's contents. This breaks the requested exact close-credit law even before a live route is enabled.

## Exact Work Required for a Runnable Route

This cannot be completed only by changing `TypedCommandFullOperationJob`. The exact architectural surfaces are:

1. `🧰️framework/🔨️modules/🎯️action-bus/🦀️component.rs` — use `ActionBus::admit_exact_wire` (`430-446`) on owner/controller/tool/schema plus a bounded raw page *before* app decoding; extend the factory-side wire construction path (`dispatch_wire`, `460-483`) so it has retained input-page ownership and a close contract. `dispatch_typed` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:19644-19653` must no longer infer the key through `A::command_id` or serialize before admission.
2. `plugin/component.rs` — replace the batch `ArtifactOwnedToolJobRequest`/`ArtifactToolCompletion` seam (`13155-13230`) with app-owned input and output page/item authorities: exact typed decoder/reducer, lane-specific page codecs, completion page ids, receiver/attempt state, acknowledgement, retry exhaustion, and bounded close/handback. The existing `ArtifactApp::{register_tool_job_factories, build_tool_job}` surfaces (`11646-11656`) and `ArtifactToolFactoryRegistry::register` (`12897-12958`) are the registration boundary. The source census found no domain file outside this component implementing these hooks, so actual app-owned command factories still have to be introduced rather than enabled by the shared default.
3. `plugin/component.rs` — add a real publisher state to `MountedTypedCommandFullOperation` and its `maintenance_step` case 0 (`15052-15112`, `20287-20329`): capture `ArtifactStore::content_revision` and `generation` exactly once at exposure, call the job freshness admission, validate immediately before each publication/terminal result, retain each unacknowledged page, and move the operation into bounded retirement only after all ACKs and owner handbacks.
4. `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` plus the plugin publisher above — expose operation-scoped one-item/page apply-and-receipt surfaces. Current `PresenceStore::apply` (`3523-3533`) and `TransientStore::apply` (`3621-3632`) take whole slices and clone/apply a complete candidate; `ArtifactStore::dispatch` (`13400`) is likewise a whole command. Artifact/config/draft, presence, transient, and child lanes need resumable item receipts, generation bumps, and cancellation/stale rejection that the publisher can ACK/retry.
5. `plugin/component.rs:18072-18470` — split `dispatch_emit`/`dispatch_emit_group` batch vectors into the same per-lane publication interface. This is the only existing route to artifact/config/draft stores, child `CompositionCoordinator`, `effects`, `events`, and `UiDirtyScope`; calling it after accumulating `Emit` would reintroduce the forbidden whole-operation commit.
6. `plugin/component.rs:12149-12205` and the plugin exchange implementation in the same file — add an object-safe operation-result page/ACK frame alongside `PluginApp`'s invocation/effect/event/presence drains. Then wire that protocol through `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`, whose `ExchangeOutcome`/`exchange` path currently consumes ordinary vectors, so the live UI can acknowledge a retained page and observe stale/cancel/retry state rather than receive an unacknowledged final `InvocationResult`.
7. The language-neutral fixture and its owned + serde oracle in `plugin/component.rs:15650-15780` — turn every listed hostile law into a real state-machine trace, including exact UTF-8 fault truncation, zero fuel/deadline/cancellation in all six phases, live freshness, registry saturation, missing root, ACK/retry exhaustion, 13 root handbacks, terminal second-call idempotence, and the capacity-vs-logical-byte close regression.

Until those surfaces are implemented and compiled in native/Wasm tests, the honest status is: the generic typed-command route is deliberately fail-closed before legacy root work on the main adapters, but the shared foundation is not runnable and does not universally gate generic parsing/identity.
