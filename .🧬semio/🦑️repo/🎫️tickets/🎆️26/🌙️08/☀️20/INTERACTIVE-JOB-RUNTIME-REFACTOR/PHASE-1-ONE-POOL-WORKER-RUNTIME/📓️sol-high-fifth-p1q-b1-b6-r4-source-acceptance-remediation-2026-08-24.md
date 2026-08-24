# Fifth P1q B1–B6 and R4 Source-Acceptance Remediation

Date: 2026-08-24  
Implementer: Sol High  
Disposition: source-audit ready; isolated P1q gate green.

## Counterexample-to-fix map

| Fifth Terra counterexample | Bounded production repair | Exact source evidence |
| --- | --- | --- |
| `DbIoPageWriter::finish` validated and transitioned every page in two terminal loops, and Memory/FS/SQLite/Neo/state/WAL/index callers bypassed the retained seal. | Page-writer seal phase, page, visible-page count and source phase now persist inside the exact writer. `seal_retained_step` performs one unused-page return, validation, transition, phase change or publication opportunity. The owning future delegates to that step. Bulk `finish`/`seal` exist only under `cfg(test)`. | `🗄️storage/🦀️component.rs:615-745`; generic copies at `1131-1180`; Memory/FS completion at `5614`, `5734`, `5805`, `5845`, `5891`, `6462`; SQLite at `🪶️sqlite/🦀️component.rs:125`; async callers use `seal_retained().await` in state/WAL/index and engine. |
| Neo `BoltBytes`, state/WAL/index `Vec<u8>`, and Ready-only pseudo-yields had no durable external-owner interruption. | `DbIoExternalBytes` is the shared exact dynamic-allocation authority. Neo ranged reads move `BoltBytes.value` into `db_io_write_observed_bytes_range`; Neo append/truncate move their current value into the same authority and yield after each close step. State/WAL/index admission wrap the source immediately after preflight, copy one fragment between real `yield_once` turns, close one external opportunity per turn, then use retained page sealing. Rejected-source Drop converts any unclaimed raw source into the retained external authority. Observed driver strings now follow the same retained close protocol. | Shared authority/ranged transfer at `🗄️storage/🦀️component.rs:953-1080`; retained text at `1787`; Neo transfer at `🌐️neo4j/🦀️component.rs:71-73`, append/truncate at `420-440` and `502-521`; state at `🔘️state/🦀️component.rs:212-242`, Drop at `279`; WAL at `📝️wal/🦀️component.rs:238-278`, Drop at `408`; index at `🔢️index/🦀️component.rs:177-207`, generated rejection close at `291-314`, Drop at `282`. |
| PostgreSQL and Neo macros reconstructed `ArtifactId(owner.as_str().to_string())` after fixed admission. | `DbIoArtifactId` is now the exact bounded conversion authority: fixed `DbIoText`, ledgered driver reservation, retained `ArtifactId`, external-allocation close owner, terminal witness and lossless Drop enrollment. Both macros borrow `owner.as_artifact()`, await the actual backend call, and yield through every close opportunity. No backend macro reconstructs an uncensused identifier. | Authority at `🗄️storage/🦀️component.rs:874-950`; PostgreSQL macro at `🐘️postgres/🦀️component.rs:88-99`; Neo macro at `🌐️neo4j/🦀️component.rs:49-60`. |
| The verifier checked the retained primitive but not live callers, missed the macro spelling, and did not kill raw external/pseudo-yield regressions. | The P1q verifier now strips test-only source and scans storage, SQLite, PostgreSQL, Neo, state, WAL and index for direct writer `finish`/`seal`, raw admitted-source drops, Ready-only pseudo-yields, missing external-owner Drop, missing retained text/byte close, and dynamic identifier reconstruction. Its synthetic baseline contains the repaired structures, and new mutations restore every exact counterexample and mutilate every hostile-law body. | `📜️script.ts`, exact P1q B1–B6 and R4 regions only. |

## Hostile-law body evidence

- `db_io_page_writer_seal_memory_sqlite_neo_state_wal_index_max_cancel_fault_drop_is_one_opportunity` polls the real retained seal, proves `Pending`, drops an interrupted exact owner, drains mounted recovery to the prior ledger, counts more than `DB_IO_OPERATION_PAGES` distinct opportunities for a maximum reservation, rejects MAX+1, injects an invalid queued phase, recovers the writer from the typed fault, and returns to the exact prior ledger.
- `retained_state_exact_backing_cancel_capacity_and_close_are_hostile`, `wal_bytes_exact_backing_handback_cancel_and_close_are_one_owner`, and `exact_backing_handback_cancel_close_and_fragment_order_are_deterministic` assert pointer/capacity identity for preflight refusal, cancellation handback and explicit deadline handback. Their production admission bodies now retain `DbIoExternalBytes`, use real yields and retained sealing.
- The existing worker-lane, primary/overflow/quarantine identity, Ready/Pending close-interruption and backend-no-service laws remain live and are still required by the strengthened verifier.
- Mutations now kill shared seal delegation, Memory and SQLite caller bypass, observed-byte final loops, raw state source/drop, WAL Ready-only yield, index bulk finish, rejected-source Drop removal, artifact macro allocation/close/yield removal, and exact hostile-law evidence removal.

## Scoped validation

| Gate | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | Pass: `live-source and hostile mutations clean.` |
| `rustfmt --edition 2021 --check` over the scoped P1q Rust files | Pass after scoped formatting. |
| Scoped unstaged and cached `git diff --check` | Pass. |
| Production source census for direct writer `finish`/`seal`, admitted `ArtifactId(...to_string())`, raw Neo/state/WAL/index external drop and Ready-only pseudo-yield | No production match; remaining writer `finish` hits are test-only fixtures. |

No Cargo, Nx, Wasm, browser, database, network or broad build command was run.
