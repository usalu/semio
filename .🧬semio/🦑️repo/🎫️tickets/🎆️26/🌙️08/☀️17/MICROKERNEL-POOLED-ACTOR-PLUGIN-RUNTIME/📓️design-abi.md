# 📓️ Design — ABI / SDK / manifests

Source of truth for packets `A2-abi-sdk`, `A3-kernel-types`, `E1-describe`, and every `M*` plugin migration.

## 0. Measured baseline (rg, not assumed)

- Exactly **one** plugin calls a host wrapper directly: `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️component.rs` (`host_now_ms` ×3). Every other sync host import is reached only through the SDK (`🔌️plugin/🦀️component.rs` `component::host_*` L164–216, `host_port` L19323–19400) and `store::set_host_backbone_channel`.
- `read-artifact`/`write-artifact`/`open-window`/`invoke-action`/`network-fetch`/`write-blob`/`read-blob`/`engine-derive`/`engine-read` have **zero guest callers** today — host impls exist but are dead weight. They still get typed effect variants (the capability surface is real), but no plugin migration work follows from them.
- Plugins reach the OS through `HostEffect` (22 variants, `🎠️kernel/🦀️component.rs` L247–387), ~330 usages. Heaviest: 🪐️space 81, 🧩️puzzle 21, 📏️layout 20, 🏗️fem 19, 🎞️animate 18, 🏭️process 16. Dominant variants: `LoadDocument` 112, `DownloadMediaExport` 36, `ReplayShellCommand` 37, `DispatchAction` 27.
- **26** extension crates: 9 flow + 5 imperative (all with `.handler(...)`), 4 cad, 4 process, 3 sourcing, 1 playbook — **12 of these have zero handlers** (topic contributions only) and therefore need no wasm at all.

## 1. WIT layout

Directory `🔌️plugin/📦️packages/🦀️rust/📜️wit/`, one file per interface, all `package semio:framework@1.0.0`.

| file | interface | content |
|---|---|---|
| `📜️types.wit` | `types` | `plugin-error`, `request-id = u64`, `instance-id = u32`, `pack = list<u8>`, `revision = u64`, inference/mutation records moved verbatim from today's `types` |
| `📜️pure.wit` | `pure` | **the only host imports**: `log(level,message)`, `now-ms() -> s64`, `trace-span(name)` |
| `📜️capabilities.wit` | `capabilities` | `resource capability-token`, `capability-id`, `capability-grant`, `capability-change {granted, revoked, narrowed}` |
| `📜️effects.wit` | `effects` | the `effect` variant (§2) |
| `📜️events.wit` | `events` | the `event` variant (§2) |
| `📜️ui.wit` | `ui` | `ui-patch`, `patch-op`, `surface-ref`, `resource surface` |
| `📜️documents.wit` | `documents` | `resource document` (id, lane, revision), `resource transaction`, `resource blob` (`size`, `read(offset,len)`) |
| `📜️jobs.wit` | `jobs` | `start-job` / `step-job` / `cancel-job` |
| `📜️checkpoint.wit` | `checkpoint` | `checkpoint() -> result<list<u8>>`, `restore(state)` |
| `📜️reactor.wit` | `reactor` | `poll` |
| `📜️describe.wit` | `describe` | `describe() -> list<u8>` (pack `PackageDescriptor`) — **build-time only, never called at runtime** |
| `📜️world.wit` | worlds | `world actor { import pure; export reactor; export jobs; export checkpoint; export describe; }` — used by plugins **and** extensions |

Deleted: `plugin-world`, `extension-world`, interfaces `contributor` and `host`; exports `manifest`, `instantiate-app`, `exchange`, `clear-instance-guard`, `migrate-artifact`, `list-*`, `artifact-compose`, `activate`/`deactivate`/`invoke`, `io-run`/`io-sniff`.

```wit
interface reactor {
  record budget { fuel: u64, deadline-ms: u32, max-effects: u32, max-patch-bytes: u32, max-frames: u32 }
  variant turn-status { idle, more-work, checkpoint-ready, faulted(list<u8>) }
  record turn-result { ui-patches: list<ui-patch>, effects: list<effect>,
                       next-wake: option<u64>, status: turn-status, fuel-used: u64 }
  poll: func(events: list<event>, budget: budget) -> result<turn-result, plugin-error>;
}
interface jobs {
  record job-budget { fuel: u64, deadline-ms: u32 }
  variant job-step { running(option<list<u8>>), done(list<u8>), failed(list<u8>) }
  start-job: func(job: u64, kind: string, input: list<u8>) -> result<_, plugin-error>;
  step-job: func(job: u64, budget: job-budget) -> result<job-step, plugin-error>;
  cancel-job: func(job: u64);
}
```

Deliberately **not** dependent on stackful-async component-model features (still incomplete in wasmtime); the reactor/effect shape avoids needing them.

## 2. Events and effects

**Events into the guest.** Lifecycle: `instance-open{instance, app-id, actor, config: pack, assets: list<(string, blob)>, capabilities, quotas}`, `instance-close`, `activate{reason: activation-event}`, `suspend-request`, `capability-changed`, `quota-changed`. Channel: `app-command{instance, seq, command: pack(AppCommand)}`. UI: `surface-visible/hidden/resized`, `patch-ack{surface, revision}`, `patch-rejected{surface, revision, reason}` (guest resends a full body). Completions: **one generic** `completed{req, result<pack, fault-bytes>}` (the SDK decodes by originating request kind; the host never interprets it), plus streaming `http-chunk{req, bytes, done}`, `job-progress`, `job-completed`. Messaging: `message{source: message-endpoint{shell(instance)|backbone(uri)|plugin-instance(id)|extension(id)|topic(name)}, payload}` — this single variant replaces `backbone-poll`, the `DocumentChanged` push, `InvokeExtension` replies and topic subscriptions. Timers: `timer{id}`, `wake`. Inbound requests: `request{req, from, capability, payload}` — the former `extension.invoke`, `artifact-compose`, `io-run`, `io-sniff`, `artifact-infer`, `artifact-mutation-plan`, `migrate-artifact`; answered with the `respond{req, result}` effect within a bounded number of turns or by spawning a job.

**Activation events**: `on-command:<id>`, `on-view-visible:<id>`, `on-file-type:<ext>`, `on-artifact-kind:<kind>`, `on-extension-request:<point>`, `on-startup-finished`.

**Effects out of the guest** (each carries `req: request-id` when a completion exists):

| today | effect |
|---|---|
| every `AppFrame::*` except UiSection/Effects/Events | `send-message{target: shell(instance), payload: pack(AppFrame)}` |
| `AppFrame::UiSection` | `ui-patch` — returned in `turn-result.ui-patches`, not as an effect |
| `AppFrame::Events` | `publish-event{topic, payload}` |
| `backbone-send` | `send-message{target: backbone(uri), payload}` |
| `backbone-poll` / `backbone-status` | deleted → `event.message` / `subscribe{topic}` |
| `read-asset` | preloaded `blob` in `instance-open.assets`; lazy `blob-load{req, hash}` |
| `write-blob` / `read-blob` | `blob-write{req, media-type, bytes}` / `blob-load{req, hash}` |
| `network-fetch` | `http-request{req, method, url, headers, body, stream}` |
| `read-artifact` / `write-artifact` | `document-read{req, doc, lane}` / `document-write{req, doc, lane, ops}` |
| `resolve-artifact-link` | `link-resolve{req, link}` |
| `io-dialects` | routing table injected in `instance-open`; on-demand `registry-query{req, kind, filter}` |
| `io-compose` | `io-compose{req, key, sources}` (host `IoRouter` routes it to the owner as `event.request`; one hop, no re-entrancy) |
| `engine-derive` / `engine-read` | `cache-derive{req, engine-id, input}` / `cache-read{req, engine-id, key}` |
| `open-window`, `invoke-action` | `open-window{req, kind, params}`, `dispatch-action{req, target, invocation}` |
| `HostEffect::InvokeExtension{response_action}` | `invoke-extension{req, extension-id, capability, payload}` — **`response_action` disappears**; the SDK resumes the awaiting future |
| remaining `HostEffect` variants (CloseWindow, Notify, ClipboardWrite, Navigate, OpenExternalUrl, SetPanel, SetActiveUtility, SetActiveTool, PatchWorld3dChrome, ReplayShellCommand, SpawnPluginInstance, OpenPluginInstance, OpenDialog, IconRenderExport, DownloadMediaExport, LoadDocument, RequestSync) | one typed variant each, **same field names** so plugin call sites are renames; `RequestFileOpen`, `RequestMediaFrames`, `OpenDialog`, `SpawnPluginInstance` gain `req` + completion instead of a follow-up action id |
| self-tick loops, `pending_effects()` polling | `set-timer{id, after-ms, repeat}` + `next-wake` |
| new | `spawn-job{job, kind, input, placement: inline\|isolated\|exclusive}`, `cancel-job`, `respond{req, result}`, `storage-read/write/delete`, `request-capability`, `release-capability`, `subscribe`/`unsubscribe` |

Rust SSOT for all of it: `semio_framework::kernel::{Effect, Event, UiPatch, Budget, TurnResult}` in `🎠️kernel/🦀️component.rs`, replacing the `HostEffect` region, with the ts-rs mirror. WIT variants mirror the Rust field-for-field; the SDK glue converts (same pattern as the existing `ArtifactInferenceRequest` conversion). `payload: pack` is always `store::pack_rt::encode_wire_value`.

**`contributor` resolution.** `list-artifact-inferences` / `list-artifact-mutations` / `list-artifact-dialects` / `list-io-entries` become **static descriptor data**. `artifact-infer`, `artifact-mutation-plan`, `artifact-compose`, `io-run`, `io-sniff`, `migrate-artifact` become **well-known cold job kinds** (`semio.infer`, `semio.mutation-plan`, `semio.compose`, `semio.io-run`, `semio.io-sniff`, `semio.migrate`) driven by `start-job` + `step-job` on a pooled instance — bounded, cancellable, budgeted, so a WFC/SfM-class inference can no longer trap on a whole-store fuel budget.

**`exchange` collapse.** `exchange(id, cmds)` ⇒ `poll([app-command{id,seq,cmd}…], budget)`. The `exchange(id, [])` drain disappears (guests are woken by events/timers/`next-wake`). Channel **v12** (`📡️spr/🧵️channel/🦀️component.rs` + the hand-written TS twin in `💻️os/🟦️component.ts` + the wgpu `ProgramBridge` decoder): remove `Hello`/`Bye`/`AttachBackbone`/`DetachBackbone`/`RefreshUi` from `AppCommand` and `Welcome`/`UiSection`/`Effects`/`Events` from `AppFrame`; add revisioned `ui-patch{surface, kind, revision, base_revision, ops: pack}` with `PatchOp::{Replace(path,node), InsertChild(path,idx,node), RemoveChild(path,idx), SetProps(path,pack)}`; `Ephemeral` keeps its generations. `Welcome.manifest` is unnecessary — the shell already holds the static descriptor.

## 3. Static descriptor emission (packet E1)

`PackageDescriptor { descriptor_version, role: Plugin|Extension, manifest, activation_events, capability_requests, extension_points, execution: ExecutionMode, quotas, contributions{commands, menus, file_types, panels, themes, topic_contributions, artifact_contributions, inference_services, mutation_services, io_entries, composer_entries}, assets, hashes{wasm_sha256, core_wasm_sha256, descriptor_sha256} }` in `🛂️manifest/🦀️component.rs` + ts-rs mirror. Builder additions: `.activation(..)`, `.extension_point(..)`, `.requests(cap)`, `.quota(..)`, `.execution(mode)`, `ExtensionBundle::mode(..)`.

Freshness without wasm: `plugin_exports!`/`extension_exports!` additionally expand `#[test] fn descriptor_is_fresh()`, byte-comparing the natively-assembled `describe()` against `<crate>/🤖️generated/🛂️descriptor.semio` — so `verify gate` enforces it through ordinary `nx run-many -t test`.

Emitter: new bin crate `💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust` (`semio-framework-plugin-describe`, wasmtime) — `describe <component.wasm> --out <dir>` runs the built component's `describe` export once (fuel-limited, `pure` imports only) and writes `🛂️descriptor.semio` + `🔣️descriptor.json`. Called from the dev `📜️script.ts` right after the wasip2 build and by each plugin crate's `📜️script.ts describe`.

Registry `📇️registry/📜️script.ts`: `parsePluginCargo` stops inferring `capabilities`/`contributes`/`consumes` from Cargo metadata and reads `🔣️descriptor.json` instead (Cargo `[package.metadata.semio]` keeps `role`, `extends`, `mode`, playground rows). `check` gains: descriptor exists per crate, `pluginId` matches the component package, `extends` matches the first dependency, referenced extension points exist on the host plugin, and the built wasm's sha256 matches `hashes.wasm_sha256`. Web packaging (jco lowering) moves to `🤖️generated/🌐web/` at publish time — **no jco at runtime, no runtime-generated JS**.

## 4. Guest SDK (packet A2)

Split `🔌️plugin/🦀️component.rs` along its regions via `#[path]` into `🔌️plugin/⚛️reactor/🦀️component.rs` (turn loop) + `⚛️reactor/{🧵️executor,📮️requests,🩹️patches,💼️jobs,📸️checkpoint}/🦀️component.rs` + `🔌️plugin/🌐host/🦀️component.rs` (async host API replacing `host_port`). `app`, `world3d_host`, `engagement` stay.

- **Executor**: single-threaded `LocalExecutor` (thread-local task list, index-based wakers, no `Send` bounds — wasm32 has one thread). `poll()` = install budget → route events (`app-command` → the existing `PluginApp` dispatch **unchanged**; `completed` → `RequestRegistry::resolve` wakes the future; `timer`/`wake` → scheduled closures; `message` → store backbone ingest; `request` → handler or job) → run the executor until idle or budget exhausted (`more-work`) → drain the `EffectSink` (capped by `max-effects`, overflow carries over) → emit patches for dirty **visible** surfaces → `next-wake` = earliest pending timer.
- **Request registry**: `host::request(effect) -> impl Future<Output = Result<T, Fault>>` allocates a `request-id`, pushes the effect, parks. Ergonomic surface: `host::{blob, http, documents, links, io, cache, extensions, messaging, timers, jobs, ui, shell, storage}`. `host::now_ms()`/`log` stay synchronous. `store`'s `HostBackboneChannel` becomes a per-instance `EffectBackbone` (the process-global `set_host_backbone_channel` is deleted — it cannot survive pooled multi-instance actors).
- **Handlers**: `ArtifactApp::handle` keeps its synchronous signature; `Emit` gains `tasks: Vec<AsyncTask>` so a command may await host results and then emit follow-up mutations under the same `ActionMeta`. This is what replaces `InvokeExtension{response_action}`, the `RequestFileOpen`+follow-up-action pattern, and `pending_effects()`.
- **Surfaces** render lazily: `surface-visible`/`hidden` replace the `RefreshUi` section-probe protocol; `🩹️patches` keeps `last: HashMap<(instance,surface), (revision, UiNode)>` and diffs by node identity path, falling back to a full body when the patch exceeds ~60 % of the body or on `base_revision` mismatch.
- **Checkpoint** = pack of `{instances: [{id, app_id, actor, document/config/draft packs, view_state, ephemeral}], timers, pending_requests}`; async tasks are not serialised but marked re-run-on-restore (they are host reads; mutations only apply on completion).
- **Deleted**: `InstanceGuard`/`INSTANCE_GUARD`/`clear-instance-guard` (a trap now kills the actor and the kernel restores from checkpoint — which makes checkpoint cadence correctness-critical), `install_io_fallback_dispatcher`, `host_port`, `component::host_*`, `plugin_exchange`, all `SECTION_KIND_*` probe code, `extension_component` (one `actor` world for both roles).
- **Actor granularity**: `panic = "abort"` means a panicking task kills the whole instance — so **one actor per app instance** is the default; multi-instance pooling is opt-in for first-party packages only (`🎪️demonstrator` bundles six panes and must declare its mode and raised quotas explicitly).

## 5. Extension modes, capability broker, quotas

`ExecutionMode { Declarative, Linked, Isolated, Exclusive, Cold }` on both manifests, default `Isolated`. `ExtensionPointDeclaration { id, publisher_scope, allowed_modes, capability_allowance, quota_ceiling, payload_schema, activation }` on the host plugin replaces the Cargo `consumes` tag.

Effective permissions = extension requests **∩** the host plugin's extension-point allowance **∩** user approvals **∩** the host plugin's own effective set (a host can never delegate more than it holds). Enforced at install (mode + request check), at link (`Linked` requires same publisher **and** `Linked ∈ allowed_modes`; feature-gated to avoid the `semio-framework-os-flow` ↔ extension-crate cycle), and at runtime (broker denies → `completed{Err(capability-denied)}` + audit).

Mapping of the 26 extension crates: **Declarative** (12, zero handlers, never instantiated) — process {metal, robotic, concrete, wood}, sourcing {slabs, windows, beams}, cad {aec-building-structure, spatial-shape, aec-building-energy}, playbook/procedural, and cad/aec-building's topic contribution (its four artifact contributions become **Cold** job kinds). **Linked** (14, same publisher, pure evaluation) — 9 flow + 5 imperative operators; flow/brep tessellation runs as an **Exclusive** job. Third-party operators arriving later default to **Isolated**.

Broker (`🎠️kernel` `//#region 🔖️Broker`): `CapabilityId` (`storage.read|write`, `http:<origin>`, `timers`, `messaging.backbone:<uri>`, `messaging.plugin:<id>`, `documents.read|write`, `blobs.read|write`, `jobs.spawn|exclusive`, `ui.window|dialog`, `shell.navigate|clipboard`, `extension-registry.query`, `extension.invoke:<id>`), `CapabilityRequest{id, scope, reason, optional}` (replaces `CapabilityRequirement`), `CapabilityGrant{token, id, scope, expires_ms}`, `CapabilityChange{Granted|Revoked|Narrowed}` — revocation invalidates the guest handle table so the next await returns `Fault(capability-revoked)`.

`QuotaSchema { memory_bytes, fuel_per_turn, turn_deadline_ms, tables, mailbox_len, message_bytes, outstanding_requests, timers, storage_bytes, network_bytes_per_min, ui_nodes, patch_bytes, patch_hz, blob_resident_bytes, gpu_ms_per_frame, background_ms_per_min, log_bytes_per_min }` — all `Option` = inherit; `QuotaTree` resolves min-down os → plugin → extension → instance. A plugin can sit inside its memory limit and still exhaust the host through timers, UI nodes, requests or GPU allocations, which is why the schema is this wide. `BrokerHooks { admit_effect, on_turn_finished, on_breach -> FailureAction, on_capability_change }` are what the scheduler calls.

## 6. Migration ordering (W3)

SDK first, then `🗄️stdio` alone (every plugin depends on it), then the fan-out batches, then `🎪️demonstrator` last. Per crate the work is: rename `HostEffect::X` → `Effect::X`, replace `pending_effects`/self-ticks with timers or async tasks, declare activation events / extension points / capability requests / quotas / execution mode in the builder, move long computations (WFC, FEM solve, SfM, brep tessellation) to jobs, run `describe`, commit `🤖️generated`.
