# Three-Pillar Current Residual Execution Graph

**Scope.** Read-only current-tree reconciliation on 2026-09-05. I inspected source and registered-gate definitions only; I ran no Cargo, Nx, browser, or process command. “Recorded native/source result” below means a result reported by the owning lane in this ticket, not a rerun by this audit.

## Decisive verdict

The repository now has several real, fail-closed building blocks, but **none of the three user journeys is yet end-to-end executable**:

1. The hub can statically preview an exact `stdio + gis` native codec closure, and a server-owned two-package materializer is source-gated. It has not been qualified by the registered candidate/process run, it exposes only one GIS *WASM viewer* target, and the browser's present open-authority code rejects that target because it requires `react`.
2. The hub's document socket is substantially more real than older audits said: it has periodic authorization revalidation, 4401 revocation, command durability-before-fanout, bootstrap/tail recovery, and focused socket tests. The ordinary Home/Space client still cannot ingest a bounded ordered directory page, command retries have no idempotency key, invite consumption is not durably marked, and presence only disappears on handler exit.
3. GIS has exact local codec receipts and a deterministic map calculation. Hub inference has a private SQLite job ledger, authorization helper, canonical command decoder, and committed-WAL witness; none is constructed in `HubState` or routed from a Map action. No AI/model request, progress stream, approval route, or collaborative typed apply exists.

The shortest honest runnable two-user milestone is therefore **not “all plugins” or “AI”.** It is: install one fresh server-owned GIS Map profile, bind it to an immutable client execution-target lease, open the read-only Map surface for two authenticated members, and drive an ordered, privacy-safe directory page plus revocation. Authoring through Flow and AI approval come later.

## Reconciled status of previous findings

| Earlier report/finding | Current source status | Evidence and correction |
| --- | --- | --- |
| Hub linked no GIS/native codec bindings | **Fixed at source.** | `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:9-74` now fixes `stdio+gis/native-codecs/v1`, 26 stdio plus two GIS receipts, checks exact package/version/hash/schema/factory identity, and does not publish by preview alone. Older matrix/AI reports saying “stdio-only” are stale. |
| Hub starts with an empty/untrusted catalog | **Partially fixed.** | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5524-5559` creates the provider set, reads only `OS_HUB_TRUSTED_CATALOG_BUNDLE/PROFILE`, and keeps `openable_catalog` absent unless `configured_artifact_authority` succeeds. This is fail-closed. The fresh producer/candidate is source-gated in `🌎️hub/📦️packages/🦀️rust/📜️script.ts:3889-3970,4575-4658`, but no audit-run native materialization/candidate/process receipt exists. |
| Descriptor limit ambiguity (64 KiB vs 4 MiB) | **Fixed for the trusted profile path at source.** | The trusted-bundle gate explicitly requires `CATALOG_DESCRIPTOR_MAX_BYTES = 4 * 1024 * 1024` at `📜️script.ts:4581-4588`; the materializer carries descriptor byte lengths and SHA-256 at `:3956-3965`. Do not revive the historical 64 KiB policy without a live conflicting code path. |
| Document plan lacked parent dialect / catalog revalidation | **Fixed at source; runtime still unqualified.** | Private plan authority contains `parent_dialect` and validates it with descriptor/package/artifact/grant identity at `🚀️bin.rs:1047-1124`. Issuance binds it from selected catalog data at `:2042-2133`; exchange repeats catalog generation, target, dialect, surface, grant, descriptor, and revision checks at `:2200-2246`. |
| Browser can execute a trusted GIS Map target | **Still RED, now more precise.** | Materialization selects exactly `s.gis.gismap`, viewer, `rendererTarget: "wasm"`, read-only at `📜️script.ts:3937-3948`. The browser validates the returned plan against caller-held `installedTarget`, then unconditionally requires `plan.surface.rendererTarget === "react"` at `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:482-510`. Thus the first advertised Map target is rejected even before a fetched-component integrity or render claim. Its `installedTarget` is configuration, not a server-minted immutable execution lease. |
| All-plugin catalog activation | **Still RED.** | The provider inventory has only the two entries at `native-openable-provider/🦀️.rs:25-34`; the loader correctly rejects a selected artifact with no exact binding and any extra returned binding at `trusted-catalog/🦀️.rs:421-455`. The historical 59-component census needs a fresh generated census, but it cannot be described as activated: Flow is not in the linked provider set and no profile selects it. |
| Generic child retained publication/ACK absent | **Foundation changed; Flow binding remains RED.** | The shared child/ACK and child-close work is now outside this audit packet. Flow still advertises `addWidget` as `BatchOnlyPendingRewrite` at `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1922-1976`, although the local child mutation law exists in `…/🎮️commands/➕️add-widget/🦀️.rs:22-89`. Do not duplicate framework work; bind that one app factory after public member open. |
| Public `MemberFactory::Open` was missing | **Partially fixed, native acceptance blocked.** | Kernel exports `MemberOpenOperation`/`MemberSnapshotOpenOperation` at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:28,18008-18012`; generated factories implement `begin_open` at `:18373-18375`. The provider reports source oracle success and a corrected Flow replay law, but the last exact integration run was stopped by unrelated AVI compilation before the acceptance law. Treat this as a prerequisite, not an accepted public opener. |
| Document socket/revocation has no live authorization | **Fixed at source and focused test level; no full two-client process proof.** | `handle_ws` revalidates every second (`🚀️bin.rs:3296-3311`), before ingress (`:3325-3347`), before fanout (`:3360-3405`), and listens to the live invalidation notify (`:3410-3415`). It closes 4401 on revocation. Scoped directory socket routes intentionally hide member events, with focused source tests around `:7819-8000`; that privacy decision is not a directory-page implementation. |
| Directory append could release the writer before broadcast | **Fixed at source.** | `DirectoryService::append_and_publish_locked` appends and synchronously publishes while it retains the one writer guard at `🌎️hub/📇️directory/🦀️.rs:1596-1610`; the registered exact law is named in `🌎️hub/📦️packages/🦀️rust/📜️script.ts:4727-4745`. |
| Invite redemption is exactly once | **Still RED, but not because the writer lock is released.** | `redeem_invite` holds `self.write` across authentication and append at `directory/🦀️.rs:1716-1735`. `InviteRecord` and each backend authenticate only when `accepted_at.is_none()` (`directory/🦀️.rs:222-237`, `🪶️sqlite/🦀️.rs:1209-1216`), yet the `InviteRedeemed` projection only upserts membership and never updates `hub_space_invite.accepted_at` (`🪶️sqlite/🦀️.rs:654-661`; no PostgreSQL equivalent update exists). Sequential and concurrent retry can therefore redeem the same still-valid invite repeatedly. |
| Presence can be reliably evicted after client loss | **Still RED.** | `PresenceSession` contains only surface/user/color/opaque peer (`🚀️bin.rs:410-420`), is inserted after session and removed only after WS loop exit (`:3263,3420-3428`). The one-second tick revalidates authorization, not liveness. There is no last-seen/lease expiry or deterministic ghost eviction. |
| SQLite/admin foundations are absent | **Partially fixed / separately qualified.** | A registered `admin-live-journey-check` exists and its script says it runs SQLite browser proof at `📜️script.ts:4707-4724`; that is admin SPA evidence, not the normal Home/Space two-user journey. It must not stand in for directory-event page ingestion, capability-less test identities, or client revocation. |
| GIS only has a descriptor, no native codec | **Fixed at source.** | GIS owns closed Map/Terrain receipts and validates declarations before typed codec creation at `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🦀️.rs:31-101`; hub preview consumes those receipts at `native-openable-provider/🦀️.rs:53-74`. |
| GIS inference is an end-to-end AI feature | **Still RED.** | The local Map service is a bounded deterministic calculation (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:195-279`). Hub private modules offer a ledger/control/WAL primitives (`🌎️hub/💡️inference/🦀️.rs:1-83`, `🪶️sqlite/🦀️.rs:121-310`), but `rg` finds no `InferenceJobLedgerV1` construction or inference route in `🚀️bin.rs`. No UI action, provider/model transport, result stream, approval handler, or typed Map publication is wired. |

## Current executable boundaries

### A. OS frontend and plugins/artifacts

The trusted loader has the right server boundary. It canonicalizes every selected path, bounds component/descriptor bytes and their closure totals, verifies SHA-256 and BLAKE3, validates descriptor ownership, preflights exact provider closure, then rejects missing/extra bindings before registration (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:363-455`). It also requires exactly one open target and verifies that the target has an exact native codec (`:456-525`).

The residual is client execution, not weakening loader checks:

- `DocumentOpenPlanV1` carries catalog generation, component SHA-256/BLAKE3, descriptor SHA-256, artifact/pack hash, parent dialect, surface, grants, checkpoint, and revalidation (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:432-580`).
- Browser code compares those fields to `PersistenceBinding.installedTarget` before exchange (`backbone-worker.ts:482-546`), but it does **not** obtain an immutable, server-authenticated byte lease or fetch and hash the selected component. The static target has the further `wasm`/`react` contradiction described above.
- There is no Flow native provider or profile. The first provider expansion should remain explicit and generated from real package-owned receipts; no descriptor/matrix row may impersonate a factory.

### B. Collaboration, admin, presence, and DB

The document socket performs actual commit-then-fanout on its command lane (`🚀️bin.rs:2985-3001`), holds a live grant, follows rebootstrap on lag, and periodically verifies the same membership/session grant. That makes command authorization and close semantics credible source-level candidates for a two-user test. It does not give Home/Space a safe consumer for directory changes.

`FoldDirectoryEvents` still assumes a complete history and folds from default; old Home/Space wire/config code cannot safely resume from a partial page. The current Page P0 remains the separate owner packet: sealed scan cursor versus visible cursor, authenticated scope/generation, page ACK, duplicate/gap handling, bounded retirement, reconnect and 4401 terminal state. It must not be folded into scoped-socket revocation—scoped sockets deliberately omit membership changes to prevent leakage.

The first normal UI-admin write also needs a durable caller request id. Browser and native command retry lanes resubmit naked `DirectoryCommand` objects; a response-loss retry can repeat `CreateSpace` even though append order is correct. Direct `upsert-member`/`remove-member` can demonstrate the initial P0 without inviting the still-broken redemption path; invite one-use and presence lease form a separate P1 reliability packet.

### C. AI over GIS Map

There is meaningful substrate, but it is disconnected on both sides:

- GIS Map declares a deterministic bounded service and the source holds exact Map/Terrain codec receipt checks.
- Inference control carries deadline, cancellation, bounded work, and progress counter (`hub/💡️inference/🦀️.rs:27-63`); SQLite binds a job to user/session/space/document and has idempotent request creation, private result/proposal, cancellation, prepared approval outbox, and WAL-witness reconciliation (`hub/💡️inference/🪶️sqlite/🦀️.rs:121-310`).
- The hub binary owns none of those values and its routes do not expose an inference endpoint. A local `ArtifactInferenceService` cannot authenticate a caller, pin current descriptor/catalog/frontier, stream a job, or publish a collaborative event.

Therefore the first AI packet is a **single deterministic Map proposal job**, not an external-model feature. It must be installed only after the Map execution-target lease exists and must maintain requester-private output until explicit author approval.

## Dependency-ordered P0/P1 graph

```text
P0-A  public MemberFactory Open (owned external prerequisite; no duplicate work)
       │
P0-B  fresh trusted stdio+GIS materialize → candidate readiness → atomic current publication
       │                                                  │
       │                                                  └─ no client execution claim
       ▼
P0-C  immutable execution-target lease + byte verification (browser and native)
       │
       ├─ P0-D  two-user GIS Map read-only open/reconnect/revocation journey
       │          │
       │          ├─ P0-E  retained Home/Space directory event page (separate active packet)
       │          └─ P0-F  request-id DirectoryCommand + Home/Space admin outcome UI
       │
       └─ P1-A  first Flow provider/open target + Flow addWidget retained child factory
                  │
                  └─ P1-B  atomic parent/child publication + global composition history routing

P0-D + P0-C ───► P1-C  deterministic, private Map proposal job
                           │
                           └─ P1-D  explicit approval → atomic typed GIS publication → B observes

Parallel reliability P1: invite consume transaction + presence lease/expiry
```

### P0-A — public member opening (do not overlap)

**Owner boundary:** existing provider lane. The public `MemberFactory::Open` program has mounted source and source-oracle coverage but no accepted native integration run. It must finish before a real Flow child or member-backed artifact load is claimed. Required gate is the dedicated public-member-open integration target after its current external compilation obstruction clears. This audit does not prescribe new code there.

### P0-B — first server-owned trusted profile

**Small independent Sol packet:** qualify, rather than redesign, the existing producer/candidate machinery.

- Keep the isolated producer at `hub/📦️packages/🦀️rust/📜️script.ts:3889-3970`; it builds into private fresh roots, produces 26+2 exact closure, includes both GIS Map and Terrain codecs, computes full generation identity (including zero-target stdio), and selects only the Map viewer.
- Run its registered source/native/process modes through `bun nx run os-hub:trusted-stdio-gis-bundle-check --skip-nx-cache -- --source|--native|--process`. The process mode already specifies failed-candidate retention, restart-only server-owned descriptor rotation, old receipt denial, then fresh GIS plan (`📜️script.ts:4628-4658`).
- Acceptance is only candidate hub readiness plus persistent current pointer after restart. Missing profile, swapped component/descriptor/hash, stale plan, cancelled build, partial closure, Terrain omission, failed candidate, and path escape must leave old current untouched. Do not count browser/native rendering.

### P0-C — execution-target lease (new, narrow shared packet)

**Exact seam:** the browser's `installedTarget` comparison in `backbone-worker.ts:482-546` and the native directory/WGPU loader. Define one sealed cross-language `ExecutionTargetLeaseV1` minted only from an exchanged server plan. It owns: scope; grant/read-write-observe; authorization/session generation; catalog generation; package id/version; component SHA-256+BLAKE3; descriptor SHA-256; artifact kind/schema/pack hash; full parent dialect; surface; expiry; and an invalidation generation.

The client obtains component bytes only through a server-selected endpoint, checks both component hashes and descriptor hash before module/runtime creation, and invalidates the lease on abort, expiry, receipt exchange failure, generation/descriptor/revocation change, or reconnect. It cannot be supplied in client `PersistenceBinding`. First implementation must accept the Map target's declared `wasm` renderer target rather than silently rewriting it to React; native WGPU remains a separately honest RED until a matching WGPU target/renderer exists.

**Law packet:** language-neutral canonical lease fixtures plus Rust/TS parser comparison; browser test verifies `wasm` target selection, both hashes, descriptor substitution, catalog/parent-dialect/surface/grant/scope mismatch, expiry/abort/reconnect; native test repeats byte and generation verification. A process test opens the one generated GIS plan only after P0-B. Accessible EN/DE “unavailable / stale / cancelled” terminal UI is required, but no WGPU rendering claim is permitted.

### P0-D/E/F — runnable two-user directory journey (three non-overlapping slices)

1. **P0-D harness only.** Seed two authenticated test identities and a document descriptor through real SQLite directory/bootstrap APIs; start the candidate hub; A and B exchange their own open plans/grants, connect document sockets, and verify B cannot use A's scope. Cover command broadcast, revocation close 4401, reconnect/fresh grant, lag rebootstrap, and restart. It consumes P0-B/C; it does not implement event paging or admin UI.
2. **P0-E retained event page.** Keep the current Home/Space retained event-page owner packet separate. It replaces full-history folding with a bounded scan/page/ACK/close lifecycle and only publishes events visible to the authenticated member. Required hostile cases: duplicate, sequence gap, stale generation, cancelled page, response-after-close, reconnect exact cursor, invisible foreign scope, and 4401 without raw removal event.
3. **P0-F UI-admin idempotency.** Add a schema-first `DirectoryCommandRequestV1` request id/digest owned by the authenticated principal and persisted with the append result. Route Home/Space create/upsert/remove through it; surface terminal outcome accessibly in EN and DE. Test create + member role change/removal from actual Home/Space, duplicate response-loss retry, cross-space attempt, viewer write, and B's ordered authorized observation. The first direct-member path excludes invite redemption.

Current reusable server proof points are `DirectoryService::append_and_publish_locked` and its `directory-ordered-publication-check`; current scoped revocation tests at `🚀️bin.rs:7819-8000` remain a dependency rather than a duplicate packet. Existing admin SPA check is a regression gate only.

### P1-A/B — Flow after the first read-only journey

1. Generate/link an actual Flow provider receipt and profile only when Flow owns real codec factories and an editor/open target. If unavailable, emit Flow explicitly unavailable—never add a dummy provider entry.
2. Bind the existing local typed child `addWidget` mutation to a Flow app-owned retained child group factory. It must use captured member authority, a synchronous bounded `FlowEvalSession` borrow, existing child publication/ACK, cancellation/freshness checks, child-first close, and `dispatch_emit_group`. Remove `BatchOnlyPendingRewrite` only when the new factory runs through a non-vacuous lifecycle law.
3. Only then add atomic parent+existing-child visibility and global composition history. This remains separate from simple document socket delivery: group acceptance must stage/abort all owners and update redo/history in a single publish boundary.

### P1-C/D — first AI Map proposal then typed approval

1. Construct the ledger in `HubState` and expose exactly four authenticated scoped routes: submit, read/page progress, cancel, and approve. The server—not request bytes—selects the one receipt-admitted Map service and derives subject/session generation, scope, descriptor/catalog identity, Map lease, and base frontier. It persists queued/running/proposal terminal states and requester-private bounded result/progress.
2. Cancellation must be capability- and generation-bound, checked before work and every terminal side effect; restart recovery replays the durable ledger/outbox only after descriptor/lease/frontier revalidation. No provider/model network call in this P1.
3. Explicit author approval consumes immutable `{job, proposalHash, commandHash, baseFrontier}` and emits one finite typed `GisMapMutation`/inverse through the same atomic composition publication/history route as Flow. Reject duplicate approval, other principal/space, stale session/catalog/frontier, cancelled job, payload substitution, and WAL proof mismatch. Only that typed event is sent to B.

Required registered gate: a new exact `os-hub:gis-map-proposal-check` should run language-neutral fixtures, exact native hub/ledger/WAL laws, then two authenticated SQLite clients. It must prove private progress/result, cancellation/deadline/reconnect/restart, no mutation before approval, one post-approval event, and B cannot read A's private job. It must explicitly say no external model provider and no WGPU rendering.

### Parallel P1 reliability — invite and presence

**Invite.** In each backend's atomic append/projection transaction, claim `accepted_at IS NULL` for the exact invite before emitting `InviteRedeemed`/member projection; reject a second claim and recover correctly after injected append failure/restart. The current service mutex is insufficient because the durable field is never set. Native and SQLite process laws need two simultaneous redeemers, sequential retry, revocation/expiry, cross-space capability, write failure, restart, and append-before-broadcast order.

**Presence.** Add server-stamped lease/last-seen and a bounded sweep owned by the document session manager. Refresh only authenticated frames/heartbeat; eviction needs generation comparison so an old timer cannot remove a reconnected session. Emit the roster removal once after local removal, free color after final connection, and never persist peer bytes. Laws: silent loss, timely heartbeat, stale timer after reconnect, two sockets same actor, revoked session, restart, cross-space peers, and delayed broadcast. Keep authorization tick and 4401 logic unchanged.

## Exact gates and evidence policy

- Existing source/neutral gate: `bun nx run os-hub:trusted-stdio-gis-bundle-check --skip-nx-cache -- --source`.
- Existing qualification modes, not yet credited by this audit: same command with `--native` and `--process`.
- Existing ordered writer gate: `bun nx run os-hub:directory-ordered-publication-check --skip-nx-cache -- --native`.
- Existing admin regression/process gate: `bun nx run os-hub:admin-live-journey-check --skip-nx-cache`—it is not a Home/Space two-user acceptance gate.
- Existing plan/browser gates are registered as `open-plan-check`, `open-plan-server-check`, `browser-document-open-check`, and `native-document-open-check` in `hub/📦️packages/🦀️rust/📜️script.ts:4759-4770`. They must be extended or superseded by the lease test; no plan-parser fixture proves executed bytes.
- Needed new exact gates: `gis-map-proposal-check`, a two-user `document-socket-journey-check`, and a Home/Space `directory-event-page-check`. Each needs explicit native/browser/process stages; no package-wide quick test can substitute.

## Explicit nonclaims

This graph does not claim all 59 registered components are runnable, a browser has executed a signed/verified module, a WGPU Map renderer works, public member opening is native-accepted, a two-user server was started, admin SPA evidence covers Home/Space, invite redemption is one-time, presence survives packet loss correctly, or AI/model inference has reached a user. It intentionally leaves scoped-directory revocation, public member-open, generic child publication, directory event-page ownership, and Flow factory work as their own packets rather than duplicating them.
