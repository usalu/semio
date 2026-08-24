# Fourth P1q B1–B6 and R4 Source-Audit Remediation

Date: 2026-08-24
Implementer: Sol High
Input audit: `📓️terra-fourth-p1q-b1-b6-r4-fresh-source-audit-2026-08-24.md`
Disposition: source-audit-ready

## Counterexample-to-Fix Map

| Audit counterexample | Bounded production repair | Hostile-law evidence |
| --- | --- | --- |
| R1 max+1/max+2 laws stopped in overflow and all-tier fallback destructured leaf owners | Artifact staging, query rows, engine query streams, and compaction pages now acquire an atomic exact tier/index reservation before the first leaf owner moves. Query projection/refresh/projection-source and artifact scanning preflight the destination before extracting or constructing a page-backed row (`🔍️query/🦀️component.rs:729,1198,1417`, `📄️artifact/🦀️component.rs:1765`). Snapshot-chain compaction preflights before reading a source page (`🗜️compact/🦀️component.rs:271,506`). The reserved empty slot is excluded from ordinary installation and promotion. Drop installs the unchanged aggregate into that slot. Storage and aggregate maintenance retain an overflow/quarantine owner in place until a primary target exists, otherwise advancing one close opportunity in its current tier (`🗄️storage/🦀️component.rs:3549`). The former artifact/compact/query/engine `*_or_recover` destructure/drop helpers are deleted. | Storage fills primary and overflow, installs two named owners into quarantine, proves both identities through promotion, then fills all three tiers and proves exact typed refusal plus re-parking (`🗄️storage/🦀️component.rs:7389`). Artifact, query, engine, and compaction laws assert both candidates' reservation tier is exactly quarantine (`2`), inspect exact pointer/row/path/operation identity after promotion, fill all three tiers, and prove exact preflight refusal without losing the candidate (`📄️artifact/🦀️component.rs:4074`, `🔍️query/🦀️component.rs:1488`, `⚙️engine/🦀️component.rs:5676`, `🗜️compact/🦀️component.rs:826`). |
| R2 heap `DbIoArtifactId`, observed `Vec` take/drop, and writer-seal all-page publication loops | `DbIoArtifactId` owns fixed `DbIoText` (`🗄️storage/🦀️component.rs:859`). `DbIoExternalBytes` owns the external allocation and closes one admitted page-content or backing phase per poll (`:881`). `DbIoObservedBytesWrite` persists source close plus per-page publication validation/transition (`:948`). `DbIoPageWriterSeal` persists unused-page return, validation, and transition indices and never calls `finish` (`:674`). | The storage law covers exact artifact text and lease handback, observed external bytes, Ready/Pending interruption, max+1 refusal, and terminal ledger equality. The verifier mutates the fixed artifact owner back to `ArtifactId`, the external owner back to `Vec`, both writer paths back to `finish`, and requires each mutation to fail. |
| R3 live artifact replay synchronously preallocated dependency/diff/inverse vectors and decoded whole fields | `ArtifactWalEnvelopeDecode` owns a fixed 64-entry `DbIoText` dependency list plus retained `DbIoPageWriter`/`DbIoPageWriterSeal` fields and advances one text fragment, dependency, payload fragment, page validation, or page transition per poll (`📄️artifact/🦀️component.rs:483`, `:523`). `ArtifactWalEnvelopeAdapter` is the schema-owned conversion boundary and advances one text, dependency, page copy, or retained page close per poll (`:625`, `:667`). `ArtifactEngine::open_retained` awaits both cursors; the old `decode_protocol_field`, dependency preallocation, and synchronous field loops are deleted. | `retained_wal_decoder_covers_pending_cancel_deadline_corrupt_max_and_max_plus_one` manually observes `Pending`, resumes the same decoder, interrupts a live diff writer with cancellation and drains it through storage maintenance, verifies deadline and trailing-byte corruption, accepts dependency/page MAX, and rejects both MAX+1 boundaries (`:4279`). |
| R4 mounted CLI close futures panicked after Ready/fault | All four mounted close futures retain `CliCommandCloseTerminal::{Witness, Fault}`. Closed repeats return the same witness and fault repeats return a clone of the same typed `DbError`; no owner `expect` remains (`⌨️cli/🦀️component.rs:335-530`). | The CLI hostile law polls record and batch again after Ready and polls record, batch, replay, and snapshot faults twice, asserting the exact error each time (`:1586`). |
| Verifier accepted every fourth-audit regression | B1–B6 now inspects fixed artifact/external owners, both persisted publication cursors, non-destructive in-tier reclaim, real overflow fill/quarantine identity/all-tier refusal, and Lane::Io backend-close placement. R4 now inspects durable reservation authorities, every production query/compaction preflight ordering, prohibits destructive recovery equivalents, inspects the real decoder/adapter/open path, and checks each mounted CLI Future body for repeat-terminal state and absence of `expect`. | Added killing mutations cover dynamic artifact restoration, raw external allocation, observed/seal `finish`, take-before-target reclaim, shallow overflow/quarantine/all-tier laws, missing or reordered reservation preflight, decoder `Vec`/loop restoration, adapter page-drop restoration, replay bypass, CLI `expect`, missing repeat-fault evidence, and every new hostile-law body token (`📜️script.ts:8260-8535`). |

## Exact Durable Reservation Authorities

- Artifact: `reserve_artifact_state_retirement` and `install_reserved_artifact_state_owner` (`📄️artifact/🦀️component.rs:829`, `:871`). `DocumentState::apply_entries` preflights every mutation slot before admitting a state leaf; a later staging refusal consolidates all exact staged leaves into one reserved cursor.
- Query: `reserve_query_rows_retirement` and `install_reserved_query_rows` (`🔍️query/🦀️component.rs:829`, `:865`). Row filtering, projection, and refresh reserve before removing a row from its source aggregate.
- Engine: `reserve_engine_query_retirement` and `install_reserved_engine_query_stream` (`⚙️engine/🦀️component.rs:2355`, `:2391`). Query result construction reserves before backend values or path owners are requested.
- Compaction: `reserve_compaction_retirement` and `install_reserved_compaction_pages` (`🗜️compact/🦀️component.rs:342`, `:378`). Snapshot-chain collection reserves before reading its first page; public fixed-owner admission returns the unchanged page on refusal.

## Validation

| Gate | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | Pass: `live-source and hostile mutations clean.` |
| `rustfmt --edition 2021 --check` on storage, PostgreSQL, Neo4j, artifact, query, engine, compaction, and CLI | Pass |
| Scoped current `git diff --check` | Pass |
| Scoped cached `git diff --check` | Pass |
| Scoped current/cached name-status | Modifications only; no deletion |

No Cargo, Nx, Wasm, browser, database, runtime, or network gate was run, as required by the audit scope.
