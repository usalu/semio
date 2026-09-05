# Terra P0 — Server-Owned GIS Map AI Provider Execution

## Decision

**RED: no server-owned AI/MAP provider execution slice exists.** The smallest honest
packet is one server module which owns one `s.gis.gismap` job kind, receives provider
credentials only through a server-side lease, journals its own job state durably, and
exposes an authenticated receipt/status/cancel/replay stream. It must not claim that
the existing generic server authority is durable: its `Embedded { data_dir }` profile
currently constructs `MemoryAuthorityStore`.

This packet deliberately does **not** touch the frozen GIS Map binding, React ShellHost,
WGPU directory bootstrap, hub directory-event work, typed GIS group work, or Stdio
taxonomy. It consumes the existing canonical map artifact wire shape only.

## Validated Current Boundary

| Concern | Evidence | Result |
| --- | --- | --- |
| Map inference contract | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`: `GisMapInference` declares static `protocol::Inference`; `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs`: `gis_map_inference_service`, `infer_gis_map_controlled` | **GREEN** typed, bounded local inference metadata exists. It is deterministic local map analysis, not an external model/provider. |
| Bounded cancellation in the map service | `gismap/🦀️.rs`: `infer_gis_map_controlled` checks non-empty cancellation identity, allocation/work/recursion budgets, and invokes its checkpoint before and after bounded work | **GREEN** useful input/cancellation semantics to preserve at the server boundary; no provider call, resume token, job receipt, or recovery exists here. |
| OS-local inference router | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs`: `ArtifactInferenceRouter::{infer,infer_with_visited,set_live_revision_generation}`; `.../🔌️plugin/🦀️.rs`: `ArtifactInferenceServiceRegistry`, `wire_artifact_infer` | **GREEN, local only.** Router keeps routes/runtimes/live commits in mutex/process memory and validates revision/generation; it is neither server-owned nor durable/provider-credential aware. |
| Local bounded job machinery | `.../🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs`: `BoundedJob`, `start_job`, `cancel`, `checkpoint_jobs`; `.../💼️jobs/💡️infer/🦀️.rs`: `InferenceBridge`, `run_interactive_inference` | **GREEN, local only.** Worker slices, cooperative cancellation, and capped preview/lossless buffers exist; no server journal/restart ownership is supplied. |
| MCP inference execution | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs`: `execution_not_wired_error`, `InferenceJobPayload` | **RED.** Discovery is static and the execution path deliberately returns `channel.not-wired`; it must not be repurposed as a provider executor. |
| MCP job records | `.../🌉️mcp/🖥️ui/🦀️.rs`: `JobRegistry::{begin,report,succeed,fail,request_cancel}` | **RED for recovery.** This is a process-global in-memory `Mutex<HashMap<...>>`; producer cancellation is cooperative and no provider producer or restart journal exists. |
| Caller authentication | `🧰️framework/🛍️products/🖥️server/🔨️modules/🛡️policy/🦀️.rs`: `Credential`, `Resolved`, `BearerTokenResolver`, `AdminGate`; `.../📡️gateway/🦀️.rs`: `ServerState::{identify,authorize}` | **GREEN** for authenticating/authorizing callers. **RED** for provider secrets: bearer/capability credentials identify a caller; they are not an upstream credential vault or lease. |
| Generic authority receipts/leases | `.../🎭️authority/🦀️.rs`: `CommandBus::submit`, `AuthorityDirectory`, `Activation`, `Decision::Defer`; `.../🗄️storage/🦀️.rs`: `AuthorityStore`, `Lease` | **GREEN** semantic precedent: authorization, idempotency receipts, lease epochs, append/outbox order. `Decider` is pure and `Defer` only produces `Pending`; neither runs nor recovers provider work. |
| Generic persistence | `.../🗄️storage/🦀️.rs`: `AuthorityStores::Memory`; `.../📡️gateway/🦀️.rs`: `ServerBuilder::build` destructures `StorageProfile::Embedded { data_dir }` then instantiates `MemoryAuthorityStore::new()` | **RED.** The existing server data directory does not make authority/event state durable. A provider job cannot honestly rely on it for restart recovery. |
| Event replay/socket | `.../📡️gateway/🦀️.rs`: `get_event_stream_ws`, `pump_events`, `Fanout`; existing tests `the_replay_live_seam_has_no_gap_and_no_duplicate`, `a_resuming_subscriber_skips...` | **GREEN** replay/live seam model, but it reads the generic in-memory authority store. It supplies no persistent provider-job route or provider-job event stream. |
| Server integration seam | `.../📡️gateway/🦀️.rs`: `ServerModule`, `ServerModules`, `ServerBuilder`, router mounts; `ServerModules` currently contains only `Counting` | **GREEN** module/mount seam. **RED** actual provider module, provider state, routes, websocket, or injected executor/vault. |
| Existing HTTP surface | `.../📡️gateway/🦀️.rs`: base routes include `/commands`, `/queries`, actor event HTTP/WS, blobs, scopes/documents | **RED** no AI/MAP provider HTTP or socket route exists. |
| Provider/credential implementation | targeted repository search in server/MCP/GIS sources for `openai`, `anthropic`, `ollama`, `bedrock`, `gemini`, `model provider`, `api_key`, `credential vault` found no provider or vault authority; `🌉️mcp/📒️audit/🦀️.rs` only redacts sensitive audit keys | **RED** no model vendor interface, upstream secret custody, provider lease, persistence, or retry/reconciliation implementation exists. |

`🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/Cargo.toml` contains no model-provider,
secret-vault, SQLite, or database runtime dependency. The proposed core therefore uses
existing Rust/serde facilities and an injected port; it does not add a vendor SDK.

## Required Ownership and State Machine

The server owns the job and its provider lease; browsers, WGPU, MCP, and plugin hosts
may only consume typed receipts/events later. A request carries a configured
`credentialRef`, never a provider token. `ServerState::identify` and `authorize` derive
the caller principal and scope; no client-provided principal/scope becomes authority.

```text
New --durable Admit--> Admitted --lease(epoch)--> Running --progress*--> Succeeded
                                  |                    |                 
                                  |                    +--request cancel--> CancelRequested --> Cancelled
                                  |                    +--provider/budget error--> Failed
                                  +--restart--> reconcile(provider idempotency key) --> Running/Succeeded/Failed
```

Every transition is append-only and sequence-numbered. A worker validates its journal
lease epoch before emitting progress or a terminal record; an old owner cannot complete
after retirement. A durable `CancelRequested` record precedes upstream cancellation.
After a crash while `Running`, recovery first calls provider `reconcile(idempotencyKey)`:
it may continue only on `NotStarted`, returns the persisted terminal result on
`Completed`, and waits/retries boundedly on `StillRunning`. It never submits a second
external effect merely because local memory was lost.

An upstream which cannot confirm cooperative cancellation by the deadline ends as a
typed `Failed/TimedOut`, not falsely as `Cancelled`. Policy is checked before vault
resolution and each dispatch. Secret material is retired with the provider lease and is
never written to the journal, response, event, websocket frame, diagnostic, or audit.

## Smallest Schema-First Implementation Packet

Create a new server-owned module; do not alter any excluded active file.

1. Add `🧰️framework/🛍️products/🖥️server/🔨️modules/💡️inference/🧬️schema/🔣️.json` as the canonical v1 schema and `.../💡️inference/🧫️fixtures/🔣️.json` as cross-language vectors. It defines:
   - `MapProviderJobRequestV1`: idempotency key, map artifact kind exactly `s.gis.gismap`, document schema/version, revision/generation, canonical bounded snapshot, configured provider ID, `credentialRef`, and `ProviderWorkBudgetV1`.
   - `MapProviderJobReceiptV1`, monotonic `MapProviderJobEventV1`, `MapProviderJobSnapshotV1`, and typed failure codes. Fields named `token`, `secret`, `apiKey`, `authorization`, or raw provider credentials are invalid.
   - explicit byte/work/deadline/diagnostic/output limits. The schema accepts the frozen map artifact wire contract; it does not declare a map mutation, artifact group, or client execution protocol.

2. Add `.../💡️inference/🦀️.rs` with private `MapProviderJob`, append-only transition reducer, and these server ports:
   - `ProviderCredentialVault::lease(ProviderCredentialRef) -> ProviderCredentialLease`; lease contents are private and non-serializable.
   - `MapProviderExecutor::{execute_slice,cancel,reconcile}` keyed by job idempotency key and supplied a cancellation/deadline/budget context.
   - `MapProviderJobJournal::{admit,append,events_after,snapshot,acquire_lease,recover}`. Its embedded implementation appends and fsyncs frames beneath the configured server `data_dir`; a faulting in-memory implementation supports transition-failure laws. It persists the reference, not a secret.

3. Integrate only at framework seams: add `PolicyPoint::ProviderExecute` in `🧰️framework/🛍️products/🖥️server/🔨️modules/🧬️contract/🦀️.rs`; add a `MapProviderModule` alternative to `ServerModules` and mount it in `.../📡️gateway/🦀️.rs`. Inject the executor, vault, journal, clock, and limits through the module/builder. Do not route this through pure `Decider::decide`, local `ArtifactInferenceRouter`, or MCP `JobRegistry`.

4. Mount only the minimum authenticated interface:
   - `POST /map-ai/jobs` → durable admission receipt;
   - `GET /map-ai/jobs/{jobId}` → authorized snapshot;
   - `POST /map-ai/jobs/{jobId}/cancel` → durable cancel receipt;
   - `GET /map-ai/jobs/{jobId}/events/ws?since=<sequence>` → journal replay then live fanout, with one strictly-increasing event sequence.

   Each endpoint identifies the caller via `ServerState::identify`, authorizes the new
   policy point and scope, and filters events before writing a frame. It does not expose
   generic current authority events as a substitute for this journal.

5. Add no vendor integration yet. A deterministic `RecordingMapProvider` and test vault
   make the lifecycle executable without network, external credentials, or a new runtime
   dependency. The later real adapter must implement reconciliation and provider-side
   idempotency before it is enabled.

## Acceptance Laws

Native Rust laws in the new module:

1. **Schema parity.** Valid and invalid fixture vectors have the same result in Rust and a Bun/Ajv validation run (Ajv `8.20.0` is locked); unknown fields, foreign artifact/schema/version, raw credential fields, and oversize data fail before vault/provider access.
2. **No secret disclosure.** Inject a sentinel provider secret; provider receives it only through `ProviderCredentialLease`, while journal bytes, receipts, status, event JSON, websocket frames, and audit payloads do not contain the sentinel.
3. **Receipt idempotency.** Same authenticated principal/scope/key and digest returns the original receipt and causes one provider submission; a different digest with that key conflicts without a second provider call.
4. **Bounds and cancellation.** Over-limit snapshot/output/diagnostic and expired deadline call no provider. Cancel-before-dispatch calls no provider; in-flight cancel records `CancelRequested` before `cancel`, has no later success, and only reports `Cancelled` after acknowledgement.
5. **Lease retirement.** A previous journal holder/epoch cannot append progress or terminal success after a newer epoch is acquired.
6. **Crash recovery.** Inject failure after every journal frame. Reopen the same data directory and recover the exact committed prefix; a pre-terminal running job reconciles by idempotency key and produces one terminal result, never a duplicate external submission. Torn WAL input fails closed according to the journal framing law.
7. **Authorization.** Denial or revocation before dispatch resolves no vault lease and makes no provider call; unauthorized status/cancel/event requests disclose no job metadata.
8. **Frozen map binding.** Exactly `s.gis.gismap` and its declared schema/version are accepted. The provider result creates no GIS artifact/group mutation; existing local `gis_map_inference_service` behavior remains independent.

One new process journey is required because none currently exercises server restart for a
provider job: launch a server with a temporary embedded data directory, deterministic
recording provider, and test vault; admit with bearer auth; disconnect during progress;
reconnect the job websocket at `since=N` and assert strictly increasing, gap-free,
duplicate-free events. Kill/restart after the provider observes its idempotency key but
before the local terminal append. Recovery must reconcile, emit exactly one terminal
event, and show one provider submission. A competing worker must lose its stale lease.

## Strict Nonclaims

- No external vendor, LLM, geocoder, routing, map API, production vault, encryption/
  rotation system, billing, catalog, or broad multi-provider scheduler exists today.
- Existing caller `Credential`/`BearerTokenResolver` is not provider-secret custody, and
  current `StorageProfile::Embedded` is not durable server authority storage.
- Existing plugin-host inference, reactor jobs, and MCP UI jobs do not provide server
  persistence, socket exposure, or safe upstream exactly-once effects.
- Exactly-once external submission is conditional on the future provider adapter's
  idempotency/reconciliation contract; without it, recovery must fail closed rather
  than retrying an ambiguous execution.
- This packet adds no browser, ShellHost, WGPU, GIS typed-group, hub event-page, or
  Stdio integration, and it does not make provider output mutate a GIS Map.
