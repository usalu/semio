# Codex P7c3 Energy Product Third Independent Source/Static Re-Audit

Date: 2026-08-25  
Scope: current working-tree P7c3 source, schema, fixture, and relevant engine/artifact boundary. This was a source/static acceptance audit: no Cargo, Nx, Wasm, browser, WorkerPool/runtime, shared script, or production-source command was run; no production source was changed.

## Verdict

**GREEN — P7c3 passes this third source/static acceptance re-audit.** The three prior P0 paths now have real retained-owner code paths and executable Rust-law coverage, rather than declarative fixture/count evidence. This is not a runtime acceptance claim; the contract's deferred native/WorkerPool/Wasm matrix remains required.

## Reproduced Prior P0 Traces

### Final commit lease → ACK witness → terminal `Done` → Drop recovery

1. `MountedState::collect_channels_one` retains the exact final commit lease in `self.commit`; it does not ACK it while collecting (`🧵️simulation-session/🦀️component.rs:1100-1108`). A worker grant therefore yields while the lease remains mounted (`:1170-1176`).
2. Explicit adoption drains the retained packet pagewise, then calls `EnergyJob::ack_commit_packet(lease)` only after the payload is empty (`:1235-1258`). The engine ACKs the exact P7c2 lease and records `commit_acknowledged` (`🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️component.rs:2050-2057`).
3. On the following numerical grant, engine `Complete` consumes that witness and returns an empty `StepOutcome::Complete` (`engine:3652-3661`). Mounted `worker_step` retains that outcome and returns `JobStep::Done` (`session:1177-1186`, `:235-243`); it no longer remains permanently `Running` merely because the final consumer existed.
4. Scheduler destruction of the bounded wrapper unconditionally writes the exact fixed `RecoveryRecord` (`session:1748-1756`). `maintenance_step` validates the full identity, marks `worker_attached = false`, begins close, removes the current/pending reference, and installs the pre-reserved retirement entry (`:2207-2241`). `close_step` then advances one owned page/control at a time and refuses a false terminal while the worker remains attached (`:1273-1453`).

This is backed by actual executable laws, not JSON booleans: the engine law creates a real commit queue, proves unacknowledged final ownership yields, ACKs the exact lease once, then proves the next grant is terminal (`engine:4947-4979`). The mounted law runs `retain_worker_outcome` and an actual wrapper `Drop`, then observes the fixed recovery witness (`session:2708-2733`). These laws were inspected but deliberately not executed under this static-only assignment.

### Immediately-before-admission full revalidation and bounded stale close

`take_captured_model_for_admission` is the sole capture-to-`Model` transfer point. Before `mem::replace` transfers a completed `ModelCapture`, it requires exact mounted/expected identity, full render authority (application, base revision, document generation, canonical revision), live request, retained configuration digest, generation-qualified snapshot freshness, and non-cancellation (`session:852-866`). `MountedState::admit_job` invokes that guard immediately before `EnergyJob::admit` or `EnergyRestoreJob::admit` (`:959-997`).

The snapshot guard is authoritative rather than session-local: `EnergyModelReadLease::model` verifies the store's generation/canonical revision through `SnapshotRead::commit_authority_matches` immediately before every read (`🗿️artifacts/🔋️model/🦀️component.rs:168-203`; store `🦀️component.rs:259-285`). A stale transfer returns the exact capture, retains a transferred checkpoint packet in the same shell, begins close, and enters the pre-reserved retirement entry (`session:2095-2106`). During close, lane 11 moves the model only into `EnergyModelCloseCursor`; lane 12 releases it under the cursor's one-grant budget (`:1418-1438`).

The hostile law calls the production transfer guard for changed expected generation, application, revision, document generation, canonical revision, request, configuration digest, snapshot freshness, and cancellation. Each failed mutation proves that the capture remains intact and drains it via the real `EnergyModelCloseCursor` with at most one released item and four bytes per turn (`session:2588-2644`). It is a behavioral law, not a source-text search; it was not executed here.

### Application-partitioned adopted projection and complete ABA check

Adoption is fixed-slot storage, not a scan over all active apps. `Registry::adopted_projection` first resolves only the requesting app's slot, then requires the slot's live request and `AdoptedProjectionAuthority::matches_render` (`session:1678-1682`). The authority stores application-qualified render provenance plus the full mounted identity (`:1484-1495`). Its check requires adopted state and exact request/operation/generation/configuration identity, then validates every retained tier's application, document revision/generation, canonical revision, operation, generation, and configuration digest (`:1498-1515`). Thus matching document provenance in a different app cannot reach another app's adopted result.

The direct registry law uses two apps with identical document provenance and rejects the second app. It then mutates request, operation, generation, configuration digest, tier application, and a newer-request ABA condition; every lookup is refused (`session:2647-2705`). This invokes the real adopted lookup and authority predicate, not a fixture declaration. The fieldwise source predicate additionally covers the tier operation/generation/configuration fields.

## Preserved P7c3 Gates

| Gate | Result | Current evidence |
| --- | --- | --- |
| Artifact-owned, generation/revision-qualified model lease | GREEN | `EnergyModelReadLease` owns `SnapshotRead`, borrows only after store authority revalidation, and retains an exact return witness (`🗿️model/🦀️component.rs:168-203`). |
| No mounted scratch/cache/whole-serde/whole-model-clone route | GREEN, production slice | An `awk` production-only census of session source found zero `ENERGY_SCRATCH`, map, serde, `Engine::run`/`Engine::job`, whole-model clone, `capture.take`, blocking, or thread-spawn matches. The only raw forbidden token is in a test's negative-census list (`session:2553`). |
| Request identity and retained retry | GREEN | `matches_request` compares request/operation/generation/configuration (`:221-223`); event actions require it (`:1853-1920`), and retry retains state configuration instead of defaulting (`:1866-1869`). |
| Pre-reserved retirement and recovery; bounded fieldwise close | GREEN | Both reservations occur before snapshot take or shell ownership (`:1967-2016`); close lanes are explicit, including model capture and store witness (`:1273-1453`). |
| Normal/lost/panic/Drop/cancel/app/doc/window recovery | GREEN, source-static | One unconditional wrapper Drop publisher (`:1748-1756`), generation-checked recovery consumer (`:2207-2241`), terminal false-owner guard (`:1457-1474`), and product `close_step` (`:2266-2305`). Runtime permutations are deferred. |
| Fixed arenas, UI/channels, and one registration | GREEN | Fixed registry arrays (`:1532-1578`) and exactly one `register_bounded_job_kind(ENERGY_SIMULATION_JOB_KIND, ...)` call (`:1813-1818`). |
| Scoped format / schema / diff hygiene | GREEN | `rustfmt --edition 2021 --check` passed for session, artifact, editor, editor window, viewer window, and engine; both P7c3 JSON files passed `jq -e .`; `git diff --check -- ✏️s/🔌️plugins/🔋️energy` passed. |

## Audit Limits and Deferred Blockers

No source/static P0 blocker remains. The following are deferred executable acceptance gates, not passed by this report: actual mounted WorkerPool 1/2/4/default replay; real final/adopt/ACK/Drop/recovery execution; injected revision change exactly between final capture and admission; cross-app adoption execution; queue MAX/MAX+1 and every cancel/close/panic/lost-handle matrix; native/Wasm builds; accessibility rendering; performance and allocation budgets. No declarative fixture or string/count result was used as proof for the three repaired P0 traces.
