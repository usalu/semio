# Fifth Independent P1q B1–B6 and R4 Source Audit

Date: 2026-08-24  
Auditor: Terra — read-only, source-only  
Disposition: **RED — P1q remains unaccepted.**

## Scope And Method

Read completely: `AGENTS.md`, the P1q repair contract, the fourth Terra RED audit, and
`📓️sol-high-fourth-p1q-b1-b6-r4-source-audit-remediation-2026-08-24.md`. I then inspected live
storage, SQLite/PostgreSQL/Neo4j, artifact, state, WAL, index, snapshot, query, engine,
compaction, CLI, and verifier source. No implementation source was edited and no Cargo, Nx,
build, runtime, database, or network command was run.

## Result Matrix

| Gate | Result | Live-source conclusion |
| --- | --- | --- |
| B1 typed shared `Lane::Io` facade and backend-close polling | Source pass | Memory receives its pool; PostgreSQL/Neo facades submit then start the async-native task on `Lane::Io`; backend maintenance schedules `db_io_poll_backend_close_on_lane_io` instead of directly polling the driver. |
| B2 exact owned working set / B3 one-owner finalization | **RED** | `DbIoPageWriter::finish` still validates and transitions *every* retained page in two loops. It is live in Memory, SQLite, Neo4j, state, WAL, and index paths. |
| B4 lossless cancellation/drop handback | **RED** | Neo4j, state, WAL, and index retain ordinary dynamic driver/input buffers through an entire loop and then directly `drop` them while releasing the reservation; no durable external-owner close cursor represents an interruption. |
| B5 terminal backend close reachability | Source pass | The typed close dispatcher and terminal witness are present; this does not cure the page/external-owner failures above. |
| B6 hostile laws and killing mutations | **RED** | The focused verifier passes but omits the real caller-side finalization and dynamic-buffer regressions below. |
| R4 retained streaming / close | **RED** | The artifact WAL decoder/adapter itself is materially improved, but live state/WAL/index and Neo4j paths retain Ready-only or whole-loop work plus multi-page `finish` handoff. |

## Blocking Findings

### 1. Live finalization still bulk-transitions every admitted page

`DbIoPageWriter::finish` is public and performs a full validation loop and a full page-phase
transition loop before moving the page array (`🗄️storage/🦀️component.rs:604-628`). That is exactly
the hidden all-page final handoff the fourth remediation was required to eliminate; the existence of
`DbIoPageWriterSeal` does not repair callers that bypass it.

This is production-reachable, not a fixture:

- Memory WAL/snapshot/payload/catalog/index completion calls `output.finish()` after its retained
  read cursor completes (`🗄️storage/🦀️component.rs:5530-5533`, `5646-5648`, `5712-5714`,
  `5746-5749`, `5788-5790`).
- SQLite completes a stage read with `output.finish()` (`🪶️sqlite/🦀️component.rs:120-125`).
- Neo4j `write_driver_bytes` loops through `BoltBytes`, then calls `output.finish()`
  (`🌐️neo4j/🦀️component.rs:71-89`).
- State and index admission use `poll_fn` that returns `Ready` immediately inside their fragment
  loops, then call `writer.finish()` (`🔘️state/🦀️component.rs:215-234`,
  `🔢️index/🦀️component.rs:180-199`).
- `WalBytes::try_admit` and `copy_for_operation` copy whole input in their async invocation and
  call `writer.finish()` (`📝️wal/🦀️component.rs:241-265`).

`control.grant()` is not a pending turn. In particular, the state/index `poll_fn` returns
`Poll::Ready(())`, so it cannot interrupt the surrounding async poll. The affected completion
paths can therefore perform all page transition work after the last ordinary copy opportunity.
This fails B2/B3 and R4's page-seal, one-opportunity, cancellation, and close requirements.

### 2. Neo4j and semantic equivalents still have an ordinary dynamic external-owner drop

`write_driver_bytes` owns a live `BoltBytes`, directly indexes it across the loop, then executes
`drop(bytes)` and returns the driver reservation in the same terminal invocation
(`🌐️neo4j/🦀️component.rs:71-89`). It is not `DbIoExternalBytes`, has no terminal-empty witness,
and cannot be durably re-enrolled on interruption. State/index/WAL have the equivalent raw
`Vec<u8>` owner and direct `drop(source)` (`🔘️state/🦀️component.rs:212-234`,
`🔢️index/🦀️component.rs:165-199`, `📝️wal/🦀️component.rs:235-265`).

This is distinct from the corrected PostgreSQL route: `DbIoObservedBytesWrite` owns
`Option<DbIoExternalBytes>`, advances its close phases, and only takes it after its terminal-empty
phase. The Neo/state/WAL/index routes have no corresponding retained authority.

### 3. The actual PostgreSQL/Neo4j task path recreates an uncensused `ArtifactId(String)`

Both `with_admitted_artifact!` macros first create a fixed `DbIoArtifactId`, then immediately
create `ArtifactId(owner.as_str().to_string())` for each live task call
(`🐘️postgres/🦀️component.rs:88-96`, `🌐️neo4j/🦀️component.rs:49-57`). This is a driver-facing
working owner, not the permitted terminal frozen wire-schema result. It is neither reserved as an
exact external owner nor closed incrementally; the fixed `DbIoText` is closed only after the
dynamic `ArtifactId` is ordinarily dropped. This falsifies the claimed fixed-identity boundary in
actual async-native execution.

## What Is Actually Fixed

- The three-tier storage registry now retains primary/overflow/quarantine owners in place and the
  law fills primary plus overflow, proves both named quarantine owners, promotion, and all-tier
  exact refusal (`🗄️storage/🦀️component.rs:3526-3628`, `7389-7464`).
- Artifact/query/engine/compaction use reservation authorities before their source extraction;
  their current quarantine/promotion laws are substantive rather than mere names.
- The artifact WAL decoder uses fixed dependency slots and retained page fields. Its adapter makes
  one dependency/page/close step per poll under `WalCursorControl` (`📄️artifact/🦀️component.rs:483-724`),
  and the cancel/corrupt/MAX/MAX+1 law exercises that route (`4282-4365`). Its terminal
  `MutationEnvelope` `Vec`/`String` fields are frozen protocol wire-schema ownership, so I do
  **not** count those adapter result fields as the prohibited intermediate database working set.
- All four mounted CLI close futures retain `CliCommandCloseTerminal` and repeat the same typed
  witness/fault rather than using `expect` (`⌨️cli/🦀️component.rs:335-530`, `1586-1626`).

Those improvements cannot compensate for the live finalization and external-owner bypasses.

## Verifier Assessment

`bun ./📜️script.ts verify interactivity p1q-b1-b6` prints
`live-source and hostile mutations clean.`, but that result is false-green for these paths:

- B1–B6 checks only the `DbIoPageWriterSeal` body for `writer.finish()`, not every production
  `DbIoPageWriter::finish`/`seal` caller (`📜️script.ts:8366-8371`).
- Its memory shape census does not prohibit `output.finish()` (`8373-8389`).
- Its external `ArtifactId` predicate looks only for
  `ArtifactId(document.as_str().to_string())`, missing the macro's `owner.as_str()` form
  (`8430-8437`).
- The R4 adapter check validates decoder/adapter markers but does not reject the state/WAL/index
  raw input loops or their `finish` calls (`8669-8678`).

Thus the mutations are insufficient to kill the demonstrated production regressions, so B6 is not
accepted.

## Bounded Repair Packets

1. Remove production `DbIoPageWriter::finish`/`seal` completion. Hold `DbIoPageWriterSeal` (or an
   equivalent typed retained seal) in every Memory/SQLite/Neo4j/state/WAL/index cursor and return
   `Pending` after each validation, transition, unused-page, and shell opportunity. Make direct
   finish unavailable to production callers.
2. Give Neo4j `BoltBytes` and state/WAL/index `Vec<u8>` the same exact retained external-owner
   protocol as PostgreSQL: admitted capacity, one fragment/close opportunity per poll, durable
   handback on cancellation/drop/fault, and reservation return only after terminal-empty.
3. Remove the `with_admitted_artifact!` allocation bridge. Make the backend task layer consume the
   admitted `DbIoText` directly, or introduce a bounded repository-owned conversion authority
   whose dynamic representation is admitted and terminally closed.
4. Extend the verifier and mutations to scan all production callers (including macro expansions)
   for `finish`/`seal`, `Poll::Ready` pseudo-yields, raw external `Vec`/`BoltBytes`, and post-
   admission `ArtifactId(String)`. Add Memory/SQLite/Neo4j/state/WAL/index MAX/MAX+1,
   cancellation/fault/drop, interrupted-seal, and exact-ledger laws.

## Scoped Validation

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | Pass — insufficient predicate, as above. |
| `rustfmt --edition 2021 --check` over scoped P1q Rust sources | Pass. |
| Scoped unstaged and staged `git diff --check` | Pass. |
| Scoped current/cached name-status | Modifications only; no deleted audited source/test entry. |

No runtime claim is made. P1q remains RED until the caller-side finalization and raw external-owner
paths are repaired and independently re-audited.
