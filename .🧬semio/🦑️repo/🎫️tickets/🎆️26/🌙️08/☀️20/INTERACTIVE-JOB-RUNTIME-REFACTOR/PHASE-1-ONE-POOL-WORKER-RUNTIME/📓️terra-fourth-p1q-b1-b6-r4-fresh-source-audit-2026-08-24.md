# Fourth Independent P1q B1–B6 and R4 Source Audit

Date: 2026-08-24  
Auditor: Terra (read-only acceptance audit)  
Disposition: **RED — P1q remains unaccepted; do not advance P1w/P1x/P1y/P1z.**

## Scope And Method

Read in full: root `AGENTS.md`; the P1q actual database-I/O ownership repair contract; the
three earlier Terra RED reports (plus the initial Terra audit); and
`📓️sol-high-third-p1q-b1-b6-r4-source-audit-remediation-2026-08-24.md`.

This independently inspected the current storage core, memory/filesystem/SQLite/PostgreSQL/Neo4j
backend paths, snapshot, index, WAL, state, artifact, query, engine, compaction, CLI, pack, and
the isolated verifier. This report is the only edit. No Cargo/Nx/build/database/network command
was run.

## Result Matrix

| Gate | Result | Evidence |
| --- | --- | --- |
| Shared typed `Lane::Io` task and registered/rejected backend-close submission | Source-only pass | The registered and rejected close request paths submit `Lane::Io` jobs, and only their lane jobs call `close_backend_step(context)` (`🗄️storage/🦀️component.rs:2240-2335`, `:2562-2745`). No production `Waker::noop()` or `mem::forget` was found in the inspected core/backends. |
| Page/list/platform retained cursors | Partial only | The former Ready-only helpers now retain state and return `Pending`; however writer publication still invokes a hidden multi-page phase loop, and dynamic driver/artifact owner destruction remains ordinary drop/replace (`🗄️storage/🦀️component.rs:580-624`, `:832`, `:854-889`). |
| Primary → overflow → quarantine identity/recovery | **RED** | The stated max+1/max+2 law stores *both* named candidates in overflow and never fills/promotes/quarantines an owner (`🗄️storage/🦀️component.rs:7235-7265`). All three rings are implemented, but the required three-tier boundary is not proven; an all-tier refusal then falls into destructive “recover” helpers. |
| Exact terminal owner/credit retirement | **RED** | Storage uses `drop(std::mem::replace(...))` for `DbIoArtifactId`, and the observed driver cursor takes/drops an arbitrary `Vec<u8>` in one close phase. This is precisely the replace/ordinary-drop retirement forbidden by the contract (`🗄️storage/🦀️component.rs:808-838`, `:842-889`). |
| R4 streaming/no contiguous owner/no serde decode | **RED** | Actual WAL replay reconstructs dependencies and each diff/inverse field into preallocated `Vec`s (up to 65,536 dependency entries and 256 MiB per field) in synchronous loops; it returns a `protocol::MutationEnvelope` containing those raw vectors. This is production, not a fixture (`📄️artifact/🦀️component.rs:366-404`). |
| Ready/Pending/fault and public close robustness | **RED** | Four production mounted CLI close futures use `Option::expect`; a legal repeat poll after an error/completion panics instead of producing a deterministic stale/closed terminal result (`⌨️cli/🦀️component.rs:351`, `:397`, `:443`, `:489`). |
| Hostile-law and verifier sufficiency | **RED** | The verifier accepts all above counterexamples. Its R4 mutations contain 30 cases (`📜️script.ts:8348-8378`), but none restores the artifact replay `Vec`/serde materialization, `DbIoArtifactId` replacement close, CLI `expect`, or overflow→quarantine transition. The B1–B6 predicate merely requires a quarantine token and checks the first two candidates in overflow (`📜️script.ts:7923-8156`). |

## Blocking Findings

### R1 — The claimed max+1/max+2 proof never reaches quarantine

`DB_IO_LOST_OWNER_OVERFLOW` and `DB_IO_LOST_OWNER_QUARANTINE` each contain 64 slots
(`🗄️storage/🦀️component.rs:3368-3409`). The hostile law fills only the primary ledger, parks
`exact-plus-one-candidate`, parks `exact-plus-two-candidate`, and asserts that **both** names are
in overflow (`:7235-7257`). It never fills overflow, installs an exact quarantine candidate, then
proves ordered promotion and terminal credit recovery from quarantine.

The same shallow shape exists in the artifact, query, engine, and compaction laws: their two
named candidates are asserted in overflow, not in quarantine (`📄️artifact/🦀️component.rs:3689-3693`,
`🔍️query/🦀️component.rs:1396-1401`, `⚙️engine/🦀️component.rs:5633-5637`,
`🗜️compact/🦀️component.rs:775-780`). This does not establish the required
primary → overflow → quarantine max+1/max+2 recovery law.

Worse, the nominal all-tier recovery deliberately destructures the cursor then drops its leaves:
artifact `artifact_state_return_to_leaf_authorities` (`📄️artifact/🦀️component.rs:500-511`),
compaction `retire_compaction_pages_or_recover` (`🗜️compact/🦀️component.rs:334-341`), and query
`retire_query_rows_or_recover` (`🔍️query/🦀️component.rs:826-836`). That cannot return the same
outer authority, cursor, or aggregate credit witness after every durable ring is full.

Required repair: use a preflighted, exact typed refusal before losing a Drop-only owner, or add a
fourth durable authority whose capacity itself is guaranteed by the admission calculation. Add
separate tests that fill primary and overflow, prove exact quarantine identity, interrupt at every
promotion, resume the same cursor, and return every relevant ledger/credit counter exactly once.

### R2 — Production terminal paths still use replace/take-and-drop for dynamic owners

`DbIoArtifactId::close_step` retires its heap `ArtifactId(String)` by replacing it with an empty
string and immediately dropping the previous value (`🗄️storage/🦀️component.rs:830-834`).
`DbIoObservedBytesWrite` then calls `owner.source.take()` in phase 2 (`:875-878`), which drops the
entire external `Vec<u8>` after its page copy. Neither operation retains a closeable external
allocation cursor or demonstrates its exact terminal backing before returning the reservation.

The contract expressly disallows ordinary Drop and vector/map replacement as terminal retirement.
One outer Future poll being `Pending` does not make the hidden destruction resumable; both owners
are destroyed in the grant that changes phase. `DbIoPageWriter::finish` also validates and phase
transitions every retained page through two loops during the final `DbIoPageWriterSeal` Ready
return (`:580-624`, called from `:673-696`), so the “one page/owner opportunity per poll” property
is not source-proven for publication.

Required repair: eliminate dynamic `ArtifactId(String)` at this boundary in favour of fixed
`DbIoText`, and make any unavoidable driver allocation an explicit retained external owner with a
durably resumable close/rejection protocol. Split publication into a retained page-transition
cursor, or prove the final all-page handoff is an indivisible fixed arena operation allowed by the
contract and add a law that interrupts it at every page.

### R3 — The live WAL replay route is a full-buffer, serde-dependent synchronous decoder

`ArtifactEngine::open` invokes `decode_retained_envelope` for each replayed WAL command
(`📄️artifact/🦀️component.rs:832-850`). It allocates a dependency `Vec` sized from wire input,
then calls `decode_protocol_field` twice.
`decode_protocol_field` uses `Vec::with_capacity(remaining)` for a field allowed up to 256 MiB,
copies every 4 KiB fragment in a synchronous `while`, and returns the contiguous `Vec`
(`📄️artifact/🦀️component.rs:366-404`). The same production module publicly exposes
`serde`/`serde_json` value conversions (`:146-165`, `:215-260`, `:1274-1280`).

Fuel decrements in `WalCursorControl` do not yield a WorkerPool turn in this synchronous function;
the loop has no retained Future/cursor state. Thus replay can perform a full allocation and all
copy work in one operation, with no cancellation/close interruption point and no operation-credit
owner. This violates the R4 no-contiguous-buffer/no-full-preallocation/no-hidden-loop condition.

Required repair: represent replay diff/inverse bytes and dependencies with fixed admitted page/list
owners; expose a Future/cursor that processes one bounded fragment/entry per poll and survives
cancel, malformed input, and Drop. Keep `serde` outside this database I/O/replay boundary or behind
a schema-owned incremental adapter that owns and closes its pages. Add page MAX/MAX+1,
interrupted/replayed/corrupt/deadline tests for the real decoder.

### R4 — CLI mounted close futures can panic

The record, batch, replay, and snapshot close futures each use `self.owner.as_mut().expect(...)`
in `poll` (`⌨️cli/🦀️component.rs:351`, `:397`, `:443`, `:489`). Each error and terminal branch sets
`self.owner = None`; a later poll therefore panics. The required public fault/stale/close contract
is a typed terminal outcome, not an `expect` path.

Required repair: retain a terminal exit state in each mounted close future and return its same
typed witness/error for any repeat poll, or make the future non-public and prove it cannot be
polled after its terminal transition. Add explicit repeat-poll-after-Ready and
repeat-poll-after-fault laws.

## Verifier And Law Assessment

`bun ./📜️script.ts verify interactivity p1q-b1-b6` passes, but it is not an acceptance oracle:
it scans selected markers and mutations. `interactivityP1qR4Failures` does not inspect the
artifact replay decoder for `Vec::with_capacity`, `serde`, its synchronous decode loop, or the
post-admission storage replacement. The R4 self-test list has 30 mutations, and its structural
mutations only target selected close-loop/dynamic-page regressions. It has no counterexample for
the four live failures above. A passing string/mutation gate therefore cannot supersede this
source evidence.

The existing max+1 and Ready/Pending fixture names are preserved, but their current bodies are not
the required proof of a quarantine transition or panic-free public close.

## Scoped Validation

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | Pass — current verifier only; insufficient for the findings above. |
| `rustfmt --edition 2021 --check` over storage/backends, snapshot/index/WAL/state/artifact/query/engine/compact/CLI/pack | Pass. |
| Scoped uncached and cached `git diff --check` | Pass. |
| Scoped current/cached `git diff --name-status` | Modifications only; no `D` entries in the audited P1q files. |

No broad build, Cargo/Nx, runtime, database, browser, or network verification was run.

## Acceptance Conclusion

The third remediation materially improves lane placement, drops the prior `mem::forget` and noop
waker paths, and introduces retained cursor forms. It does **not** meet the P1q contract while
max+1/max+2 never demonstrates quarantine recovery, dynamic owners still close through
replace/take-and-drop, actual WAL replay fully materializes vectors synchronously, and public
mounted close states contain `expect` panics. Keep P1q RED until those paths and verifier/law
mutations are repaired and independently re-audited.
