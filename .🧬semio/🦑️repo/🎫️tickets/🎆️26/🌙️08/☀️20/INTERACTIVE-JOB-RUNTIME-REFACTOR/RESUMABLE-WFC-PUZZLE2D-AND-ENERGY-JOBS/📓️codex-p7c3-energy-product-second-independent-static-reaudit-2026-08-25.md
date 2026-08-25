# Codex P7c3 Energy Product Second Independent Static Re-Audit

Date: 2026-08-25  
Scope: current workspace source, schemas, fixtures, and aggregate Energy diff state; no Cargo, Nx, Wasm, browser, or runtime command was run.

## Verdict

**RED — P7c3 remains source/static unacceptable.** The first Terra RED remediation did remove the old cache route and added materially better retained ownership machinery, but three P0 product-path failures remain: a completed final can leave the worker permanently attached, model admission is not revalidated at the transfer boundary, and adopted output can be returned to a different application instance.

## P0 Findings

### 1. Normal final terminal cannot reach the required Drop/recovery path

The mounted wrapper consumes a commit lease before asking `EnergyJob` to emit its `Complete` terminal. `MountedState::collect_channels_one` takes the commit at `🧵️simulation-session/🦀️component.rs:1064-1074`. On the next worker grant, `worker_step` returns `JobStep::Running` whenever `self.commit.is_some()` (`:1140-1142`). The engine's actual `EnergyJobStage::Complete` requires that same packet still be in `publication.commits` (`🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️component.rs:3645-3647`); after the lease has been taken it instead yields forever.

Therefore normal final completion neither returns `Done` nor drops `EnergyMountedBoundedJob`, so the common Drop publisher and fixed recovery/retirement cursor are not reached. Adoption only ACKs the lease; it does not restore the packet needed by `take_terminal`. This violates the exact normal-terminal / worker-owner / recovery requirement.

### 2. Snapshot freshness is checked during capture, not immediately before irreversible model admission

`capture_one` checks the generation-qualified lease before copying an incremental unit (`🧵️simulation-session/🦀️component.rs:919-925`). Once capture returns complete, `reconcile` can inspect or transfer a checkpoint and then calls `state.admit_job(checkpoint)` (`:2001-2025`). `admit_job` immediately moves `capture` into a `Model` and calls `EnergyJob::admit` or `EnergyRestoreJob::admit` (`:928-963`) without a second `snapshot_is_fresh`, cancellation, document-render, or identity validation.

A revision/generation change in that boundary admits a stale fully captured model. The earlier per-unit guard is not a transfer-boundary guard, and the packet contract explicitly requires freshness immediately before numerical admission.

### 3. Adopted projections have a cross-application dangling/read leak

`with_adopted_projection` scans every active slot and accepts only `base_revision`, document `generation`, and `canonical_base_revision` (`🧵️simulation-session/🦀️component.rs:2082-2091`). It does not require `authority.app_instance_id == render.app_instance_id`, operation, request, or configuration identity.

Two application instances with matching document provenance can therefore receive the other instance's retained adopted projection. This breaks the product identity boundary and the no-dangling-adopted-projection requirement.

## Confirmed Improvements / Non-Blocking Green Evidence

- The mounted session source contains no `ENERGY_SCRATCH`, `HashMap`, `BTreeMap`, `serde_json`, `Engine::run`, `Engine::job`, `model.clone()`, `snapshot.model.clone()`, `block_on`, `thread::spawn`, or `capture.take()` token. This census is scoped to the mounted session; it does not claim those facilities are absent from unrelated artifact/editor import/export routes.
- The artifact has an authoritative typed `EnergyModelSnapshot.model`; `EnergyModelReadLease` is backed by `store::SnapshotRead`, revalidates `commit_authority_matches`, and retains a `SnapshotReadReturn` witness (`🗿️artifacts/🔋️model/🦀️component.rs:169-202`).
- Model capture reserves and copies dynamic string/vector backing incrementally, and partial capture enters `EnergyModelCloseCursor` before fieldwise close (`🧵️simulation-session/🦀️component.rs:380-837`, `:1386-1405`).
- New shell retirement and recovery reservations occur before `take_snapshot_read` / `MountedState::new` in the ordinary admission path (`:1898-1939`). The registry remains fixed-size and registration is a single `register_bounded_job_kind` call (`:1743-1748`).
- Cancel/retry/discard/adopt are typed with request/operation/generation/config digest and their current-session paths call `matches_request` (`:1770-1849`). Retry uses retained rejected owners rather than a default configuration (`:965-997`).
- P7c3 schema and event fixture parse. The fixture contains the requested operation identity fields.

These do not cure the three P0 defects above.

## Mutation and Fixture Assessment

The mounted fixture's `hostile` booleans are declarative JSON, not an executable behavioral mutation harness. Several new Rust “laws” inspect source text (for example cache, retry, reservation, and terminal-loss checks at `🧵️simulation-session/🦀️component.rs:2477-2488`, `:2531-2536`, `:2561-2575`, and `:2600-2607`). The only Drop law drops a manually constructed wrapper with an empty shell (`:2579-2596`); it does not execute a real final packet, ACK, process completion, recovery, and one-unit close sequence.

Accordingly the required hostile behavioral mutations do **not** demonstrate killing the three failures above, nor do they demonstrate normal-final, lost-handle, panic, application/document/window close, or adopted-projection isolation end-to-end. No source mutation was made during this audit.

## Static Gate Record

| Gate | Result | Evidence |
| --- | --- | --- |
| Scoped `rustfmt --edition 2021 --check` | GREEN | Session, artifact, editor, editor-window, and viewer-window Rust sources. |
| P7c3 schema and fixture `jq -e .` | GREEN | `✏️editor/🧵️simulation-session/🔣️component.json` and `🧪️fixtures/🔣️events.json`. |
| `git diff --check -- ✏️s/🔌️plugins/🔋️energy` | GREEN | No whitespace errors in the current scoped diff. |
| Mounted-path forbidden-authority census | GREEN, scoped | No old cache/whole-serde/whole-model-clone admission token in the mounted session source. |
| Product ownership/freshness/terminal/adoption caller audit | RED | The three P0 traces above. |
| Behavioral mutation execution | NOT RUN / unavailable | Prohibited by this static-only assignment; the checked-in fixtures/laws are not sufficient substitutes. |

## Deliberately Deferred Gates

No Cargo, Nx, Wasm, browser, WorkerPool, or runtime execution was run. After repairing the P0 findings, a behavioral matrix must prove: exact final terminal-to-Drop recovery; a revision change between final capture and admission; cross-app adopted-projection refusal; 1/2/4/default worker parity; queue-full and each cancel stage; and normal/lost/panic/document/window/application close with one-owner/page/ACK accounting.
