# Sol P7c3 Mounted Energy Product Session Implementation

Date: 2026-08-25  
Packet: P7c3 only  
Disposition: source-static audit candidate; runtime execution remains deferred by packet instruction.

## Outcome

The Energy model artifact now owns a schema-first, event-sourced mounted simulation product session. Start, cancel, retry, discard, and adopt are session events rather than document CRUD. The editor reconciles admitted snapshot capture into exactly one registered process job kind, numerical stepping occurs only through the process `BoundedJob`, and editor/viewer windows consume immutable try-only four-tier projections.

## Retained ownership architecture

- A fixed 16-entry active registry, 32 retained shells, 32 retirement entries, and 64-entry event log provide generation-tagged admission and replacement without dynamic map authority.
- Start preflight performs retained top-level and nested String/Vec capacity census before a snapshot owner or process slot moves. Every census turn observes one model record/backing; checked item and byte bounds reject MAX+1 before mount.
- `ModelCapture` pre-reserves each destination top-level and nested backing. Dynamic records are reconstructed field-wise: one record shell, Unicode character, scalar record, or collection item per grant. There is no whole `Model`, semantic-record, serde, or JSON clone on the production mounted route.
- Snapshot freshness, document revision, document generation, canonical revision, numerical config digest, operation, generation, and seed are checked before capture, checkpoint transfer, packet install, and adopt ACK.
- Replacement cannot outrun the fixed retirement arena. Saturation retains the existing owner and reports backpressure.

## Process and P7c1/P7c2 integration

- `semio.energy.mounted-simulation.v1` is the sole registered Energy process kind.
- Process input is a fixed 95-byte request/generation/provenance token; factory lookup requires the exact fixed shell and numerical or restore authority.
- Fresh jobs use `EnergyJob::admit`. Checkpoint starts validate the prior-session token before shell allocation, move the exact saved page only after a second freshness check, and use the retained `EnergyRestoreJob` replay cursor one fuel unit per worker grant.
- Preview consumption is latest-wins and monotonic across the four accepted tiers. Checkpoint, commit, and fault packets use P7c2 generation leases. Queue advancement occurs only through explicit ACK; checkpoint-to-restore transfer uses the exact owned packet.
- Commit adoption drains an already-prepared packet page-wise and then performs explicit queue ACK. No terminal grant computes or encodes output.
- Normal terminal, cancel, lost-handle and panic all use one unconditional fixed Drop publisher. It records the exact shell and complete request/operation/generation/document/config identity; maintenance validates ABA identity, consumes the shell's pre-reserved retirement slot, and closes one owner/page/control per grant.
- Energy numerical admission rejection and checkpoint rejection remain exact retryable owners. Retry invokes their retained retry APIs rather than rebuilding from a cloned input.

## Product surfaces

- Editor commands and the JSON/TypeScript event schema require explicit `en` or `de` locale for Start and expose start/cancel/retry/discard/adopt actions.
- The customizable editor window exposes busy, cancelled, faulted, final-ready, and adopted states, a polite live region, keyboard guidance, operation/generation identity, and provisional/final four-tier labels in English and German.
- The read-only viewer imports only the artifact-neutral session interface. It exposes the adopted exact result for the matching document revision/generation and otherwise renders a bilingual no-result state; it has no actions or mutation channel.
- The numerical digest excludes UI locale and checkpoint selection while retaining every numerical configuration field.

## Narrow accepted-interface repairs

P7c3 exposed concrete ownership gaps in the accepted P7c1/P7c2 public boundary. The Energy engine received only the following narrow interfaces: wire identity read, exact rejected-owner close/terminal checks, restore close/terminal checks, queue transfer ACK, and checkpoint lease-to-restore packet transfer. The accepted numerical substages and wire protocol were not redesigned.

## Laws and hostile mutations

Local executable/source laws cover:

- event log MAX/MAX+1 chronology, collision-free active-app slot MAX+1, and exact fixed shell MAX+1;
- retirement MAX/MAX+1 retention;
- independent capture item and byte MAX+1 rejection;
- nested dynamic surface record capture with at most one record/character/item mutation per grant;
- removal of whole-record clone backdoors;
- explicit invalid-config rejection and checkpoint-token/locale digest invariance;
- fixed process token stale-generation rejection;
- tier chronology vocabulary for 1/2/4/default fuel;
- localized action registration, live-region/busy vocabulary, and viewer structural read-only behavior.

The JSON schema and hostile event fixture parse with `jq`.

## Terra RED remediation

- Deleted `ENERGY_SCRATCH`, `EnergyWorkingScene`, its dynamic `HashMap`, the serde-derived scene key, and `with_energy_model_ref`. `EnergyModelSnapshot` now owns the authoritative typed `Model`; an artifact-owned `EnergyModelReadLease` wraps the store's immutable generation-qualified `SnapshotRead`, exposes only a borrow, checks store generation/revision before every capture turn, and returns the exact store retirement witness.
- Start now carries a nonzero, strictly increasing per-app request id. Cancel/retry/discard/adopt carry request, operation, generation, and config digest in Rust, TypeScript, JSON Schema, and fixtures. Every action checks those four fields plus document render provenance immediately before mutation. Retry has no default fallback: it retries an exact rejected owner or reuses the matching shell's retained config and checkpoint token.
- Every shell reserves its exclusive fixed retirement entry and fixed recovery entry immediately after shell allocation and before `take_snapshot_read`, `MountedState::new`, capture, checkpoint transfer, or numerical admission. Reservation failure releases the empty shell and leaves the preflight/snapshot/config owner retryable.
- Partial capture no longer disappears through `capture.take()`. It moves into the P7c1 `EnergyModelCloseCursor`, which retires nested strings/items/records under the same one-unit close grant.
- The process wrapper has a single checked-out owner and always publishes an exact fixed recovery record from Drop, including clean terminal. A mounted state cannot report terminal-empty while a process owner remains attached; recovery marks the witness returned before bounded shell close, preventing late-drop ABA reuse.
- Added live laws for cache removal/read-lease presence, every stale request identity dimension, exact retained retry config, nested capture close granularity, reservation-before-owner-move ordering and MAX+1, and unconditional normal/lost/panic/cancel Drop recovery identity.

## Source-static gates

- Scoped `rustfmt --edition 2021` completed for every P7c3 Rust source and the narrow Energy engine interface file.
- Energy registration census reports exactly one `register_bounded_job_kind` call for the mounted session.
- Mounted-session production census reports no `HashMap`, `BTreeMap`, `collect`, `sort`, `serde_json`, `Engine::run`, `Engine::job`, thread spawn, `block_on`, process-pool construction, whole-record clone, airflow clone, ground-temperature clone, cache accessor, default retry fallback, or whole partial-capture drop.
- Focused reachability census reports one Energy job-kind registration, one transferable snapshot-read take, explicit retirement and recovery reservation before that take, ten request-provenance match sites, one unconditional fixed Drop publisher route, and the bounded partial-model close interface.
- Rust/TypeScript/GraphQL/JSON/Proto artifact snapshot and diff facets now carry the authoritative typed `model` field; mutation application updates it together with the derived structure/zones handles, preventing event-store edits from leaving the mounted read lease stale.
- Scoped diff/status census was reviewed against the shared dirty tree. Puzzle2d, shared `📜️script.ts`, actor, renderer, and unrelated files were not touched by this packet.
- Cargo, Nx, Wasm, browser, and runtime/build commands were intentionally not executed.

## Audit boundary

This report claims P7c3 source-static implementation only. It does not claim runtime execution, reopen P7c1 numerical behavior, reopen P7c2 wire semantics, or alter shared scheduler/renderer/actor code.
