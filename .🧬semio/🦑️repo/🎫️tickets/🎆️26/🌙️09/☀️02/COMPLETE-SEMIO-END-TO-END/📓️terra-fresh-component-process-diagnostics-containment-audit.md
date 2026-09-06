# Fresh Component Failure Diagnostics and Process Containment

## Current finding

The observed GIS `cargo` exit 101 currently loses its useful compiler transcript through a concrete path, not an inferred compiler diagnosis:

1. `freshRun` spawns every child with `stdio: "inherit"` at `plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:257-278`.
2. A nonzero close becomes only `"<command> <args> exited with <status>"`; no stdout/stderr owner is retained.
3. `produceFreshComponentV1` removes its private work root in its `finally` (`:426-468`), and `materializeTrustedStdioGisBundle` removes its build root in its own `finally` (`hub/📜️script.ts:6677-6845`).

Thus a terminal that scrolled, was truncated, or belongs to a now-ended build is the only place the compiler rendered error survives. This does not establish the cause of Cargo 101. It establishes why the current source cannot provide an evidence-backed cause.

Replacing `freshRun` with the existing `runExactCargoLawProcess` is the correct bounded subprocess owner. It captures stdout/stderr through exclusive 0600 files, checks cancellation every 100 ms, applies one aggregate output ceiling, and tree-kills timeout/cancellation/output-limit children cross-platform. See `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1908-1943`.

## P0: retain traces outside every producer-cleanup root

The shared runner accepts explicit paths; it does not choose an evidence directory. That is intentional. The current producer has no such parameter in `FreshBuildControlV1` (`describe script:43-47`), while its `freshTargetRoot`, `workRoot`, package stage root, Hub `buildRoot`, and Hub `stageRoot` are all either deleted on terminal failure or renamed/used as trusted-generation inputs. A diagnostic transcript in any of them is either lost or risks becoming an accidentally trusted catalog input.

The active neutral process fixture already exposes the needed contract at `plugin/🖨️describe/🧪️fixtures/🧵️fresh-process/`: every post-preflight case expects exactly one retained `stdout.jsonl`, `stderr.txt`, and `outcome.json`; only pre-cancellation creates nothing. The test implementation is intentionally RED until the runner replacement lands.

Use a private, caller-owned evidence root, not an arbitrary producer path or an ambient fallback:

```ts
type FreshFailureEvidenceV1 = Readonly<{
  // prevalidated absolute, regular ticket-generated directory
  root: string;
}>;

type FreshBuildControlV1 = Readonly<{
  cancelled(): boolean;
  remainingMs(): number;
  checkpoint(stage: string, completed: number, total: number): void;
  failureEvidence?: FreshFailureEvidenceV1;
}>;
```

The qualification/native caller obtains and validates `root` from its ticket-owned `SEMIO_TEST_ARTIFACT_DIR` before constructing the control. The shared describe producer receives only this opaque prevalidated root. It must not read `SEMIO_TEST_ARTIFACT_DIR` itself, and must not make the trusted catalog data root, `CARGO_TARGET_DIR`, stage root, component path, or a plan-selected path into a diagnostic destination.

For a command that passes preflight, create a single `mkdtempSync(join(root, "fresh-component-process-"))` trace; validate it is still inside `root`, regular/no-link, and use `wx` paths:

```
stdout.jsonl
stderr.txt
outcome.json
```

`outcome.json` needs a schema-first fixed record, e.g. `semio.plugin.fresh-process-outcome/v1`, with only `stage`, fixed command class (`cargo-json`, `jco`, `emitter`), `status`, `signal`, `reason`, `maxOutputBytes`, and **relative** trace filenames. It must not contain component bytes, credentials, catalog bytes, package/stage absolute paths, or become part of `FreshComponentReceiptV1`, profile framing, generation id, or a Hub response.

If a production bootstrap has no ticket evidence authority, it may still return a bounded terminal failure summary but must not fall back to the trusted catalog tree. The process/native qualification caller is the one that can promise retained ticket evidence. This cleanly separates server-owned catalog state from developer failure evidence.

## Exact wrapper requirements

Replace the implementation of `freshRun` with a small wrapper around `runExactCargoLawProcess`; do not duplicate its spawning/kill tree. The wrapper needs these properties:

1. Run `freshCheckpoint` before allocating a trace. Pre-cancel/pre-expired calls create no files, matching the neutral corpus.
2. Set `budgetMs` from a checked snapshot of `control.remainingMs()` after that checkpoint, and set `cancelled` to `control.cancelled() || control.remainingMs() <= 0`. After await, checkpoint again before success publication, so cancellation that races an otherwise-zero exit cannot advance the eight-stage producer.
3. Use the fixture's `67_108_864` aggregate output maximum. It applies across stdout and stderr, before any further summary parsing. The runner records its terminal reason and kills the entire process group on timeout, cancellation, or output limit.
4. For the two Cargo stages, append Cargo's `--message-format=json` and classify stdout as `cargo-json`; keep JCO and the descriptor emitter as bounded textual stages. Do not try to parse a non-Cargo tool's stdout as Cargo JSON.
5. On any non-success terminal (`status !== 0`, signal, or reason other than `exit`), synchronously write the outcome record before throwing. Its thrown message is a bounded summary: stage/reason/status/signal, trace directory, then at most a fixed diagnostic excerpt. The raw transcript remains the evidence; do not concatenate up to 64 MiB into an error, Hub readiness diagnostic, catalog receipt, or client route.
6. Reuse `capturedTestFailureDiagnostics` only after adding a hard result cap to its selected compiler/rendered and stderr lines. It currently joins every decoded rendered error (`library:1308-1327`), so directly calling it can recreate an unbounded terminal string even when raw capture is bounded at 64 MiB. A small fresh-specific extractor is also fine: at most 3 error messages and 6 KiB total, with raw text fallback only from the final 6 KiB of each stream.
7. On success, the fixture currently expects the trace to remain; retain it exactly as the test corpus specifies. On failure, never remove it in producer, materializer, or outer candidate cleanup. Ticket cleanup owns removal only after the ticket is complete.
8. Preserve `windowsHide: true` when routing through the library owner (or extend its process options narrowly); current `freshRun` explicitly has it, while `runExactCargoLawProcess` currently does not pass that spawn option. This prevents a Windows console-window regression in the setup/bootstrap flow.

The process trace holds developer compiler/JCO/emitter output, not authority. It is diagnostic-only and must never alter whether descriptor snapshots, browser actors, codecs, or a generation are admitted. Existing raw/core/descriptor wipe and stage cleanup remain mandatory after a failed stage.

## Cancellation and ownership audit

The existing control is sound as a caller-scoped signal listener: `trustedBootstrapBuildControl` installs SIGINT/SIGTERM state and removes both listeners in `close()` (`hub/📜️script.ts:6653-6674`); `materializeTrustedStdioGisBundle` calls that close in its terminal `finally`. `runExactCargoLawProcess` can use the boolean predicate without taking a signal listener or creating a second cancellation owner.

The current fresh loop's direct `child.kill` is weaker than the shared runner: it signals only the immediate process and uses no detached process group. Reuse of the shared owner therefore improves cancellation containment. Still preserve these distinctions in `outcome.reason`:

| Terminal condition | Required producer result |
| --- | --- |
| control cancelled before start | no trace, `fresh component cancelled at <stage>` |
| control cancelled while child runs | retained trace, `reason: "cancelled"`; no next-stage checkpoint or stage bytes |
| deadline while child runs | retained trace, `reason: "timeout"`; no next-stage checkpoint or stage bytes |
| output ceiling exceeded | retained trace, `reason: "output-limit"`; process tree killed, no parser work outside the capped content |
| spawn failure | retained empty streams and `reason: "spawn-error"`; fixed summary names the trace |
| nonzero Cargo/JCO/emitter | retained actual streams and `reason: "exit"`; bounded stage diagnostic |
| zero exit | trace records `reason: "exit"`; only then the completion checkpoint may run |

The runner test suite covers exit, timeout, cancellation, and output-limit, but the fresh fixture must continue to exercise `spawn-error`; its expected trace is the guard against a failed child bypassing the evidence record. A separate regression should assert that the runner returns/settles on spawn error rather than relying on an implicit child-close ordering.

## Minimal schema-first laws

Keep the current eight fixture rows and add only these assertions where not already explicit:

1. **Compiler JSON selection:** warning plus error JSON yields a summary containing the error sentinel, not the warning; the full JSONL remains byte-exact in `stdout.jsonl`.
2. **Text tool fallback:** JCO/emitter plain stderr yields its bounded fallback excerpt; it is not silently discarded because no Cargo JSON parsed.
3. **No transcript authority:** force a child failure after component snapshot/stage setup; assert no generation/current pointer/browser actor/profile receipt is published and the outcome's fields never occur in bundle JSON.
4. **Cancellation race:** request cancellation after process launch but before close; require `reason: cancelled`, no `completed + 1` checkpoint, no stage output/lease handoff, and a retained trace.
5. **Trace containment:** reject an evidence root outside ticket `🗑️generated`, a symlink/reparse root, and a trace filename collision; pre-cancel must leave that root empty.
6. **Bounded public error:** flood either stream, require raw combined size `<= 67_108_864`, summary `<= 6 KiB + fixed trace metadata`, and no transcript propagated into `FreshComponentReceiptV1` or trusted bootstrap logs.

The existing neutral fixture can remain the source oracle. Its `maxOutputBytes` and `diagnosticChars` fields already give exact initial bounds. Register `testFreshComponentProcessV1` next to `testFreshComponentStagingV1` in `proveTrustedStdioGisBootstrapFixture`; it is an actual host-process law without launching Cargo. The first native GIS materialization then supplies the real Cargo transcript only if it fails, under the selected ticket's generated directory.

## Scope

This fixes diagnostic observability and subprocess containment. It does not explain Cargo 101, change selected package dependencies, weaken fresh raw/core/descriptor ownership, make a browser actor trusted, or claim real GIS execution. The fresh component producer still must fail closed before catalog publication on any stage terminal.
