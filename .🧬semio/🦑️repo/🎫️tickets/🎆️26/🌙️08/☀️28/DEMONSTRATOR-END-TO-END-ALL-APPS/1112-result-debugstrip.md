# 1112 debugstrip result

## Removed (by hand, exact anchors)
- 3d-brep-boolean differential tests: four `if KB1_TRACE {eprintln!("[DEBUG] ...")}` statements (whole `if` removed).
- value-retirement arc-custody `original-loan.rs:116`: eprintln, `dbg_calls` counter; `dbg_state` renamed `state`.
- time-travel `ref-reads`: eprintln of residue (the `if` keeps its `return Err`).
- plugin-operation `cancellation`: assert message suffix `\n[DEBUG] ledger-lease-drop {backtrace}`.
- decl-registry `birth` test: `Err(e)=>{eprintln;Ok(None)}` -> `Err(_)=>Ok(None)`; whole debug probe block (`take_failure_into_step`, eprintln, `panic!("[DEBUG] stop")`) removed. Side effect: `caller.receipt()` now reads the admission turn (the debug turn used to overwrite it).
- plugin-instance-initialization `registration` test: `if index<512{eprintln!}`.
- mcp probe-document: commented `// [DEBUG]` block deleted. Re-enabled it fails to compile (~70 errors in the three sqlite test files, retained-work API drift, W-debugstrip-2). Test debt recorded in `1028-coordination.md`.
- Nx graph test TS (`graph-root`), cargo custody TS (two `console.error("[DEBUG] ...")` lines).

## Prefix dropped, text kept (the message is an assertion/panic reason, not a trace)
- os-flow-host publication work test (2 assert_eq messages), store-initialization-replay supersession tests (3 assert messages incl. line 10, missed by the audit), instance-initialization registration + ephemeral assert messages, value-native-decoding `retained-outcome.rs:85` panic message.

## Renamed (functional marker protocol, test asserts on the literal)
- mcp process-arena: child stderr marker `process-arena-child-ready` (emitter + test `contains`).
- Nx graph test `prefix="graph-root "` (child log lines parsed by the parent; 4 emitters + prefix).
- cargo custody output: `custody stdout backpressure=` (child script + test `toContain`).

## Kept
- `nx/📦️graph/🟦️.ts:336`: parses the recorded producer log lines `[DEBUG] Fixed UDRW image ` (SHA-pinned historical log format, not an emitted trace).
- `docstring-and-debug-census/🟦️.ts` (census checker), `teaching/.../🌐️site/🟦️.ts:107` (`leftover` literal), `…/📥️consumption/🟦️.ts:41` and `teaching/.../sealed-publication-and-bundle/🟦️.ts:64` (test fixture strings asserting detection of `console.log('[DEBUG] x')`).

## Skipped (other lanes)
- `plugin/🧪️testing/🧪️artifact-app-laws/.../registered-app-construction-and-typed-operation-settlement/🦀️.rs:303` (semio-plugin-testing-laws, lane `doubles`).
- New hit `value/🛬️decode/🧳️owned/🧩️record/🔎️schema/🎛️drive.rs:47` (`eprintln!("[DEBUG] ack work ...")`, native-owned, lane custody3), not in the audit.

## Verification
- W-debugstrip-3: `--tests -p semio-os-mcp-probe-document` rc 0.
- W-debugstrip-2 (10 crates, `--tests`) rc 101 from pre-existing upstream/test API drift only (value-retirement, value-native-decoding, instance-initialization ephemeral/windows tests, supersession line 14, process-arena line 10 `terminal_is_empty`, probe sqlite tests before removal); no error or warning at any edited line.
- `semio-framework-3d-brep-boolean` is not a root-workspace member (W-debugstrip-1: package ID not found); the edit is a plain statement removal, checked by reading.
- TS: `bun build --no-bundle` OK on the 3 edited files.
- Remaining `[DEBUG]` literal count in rust/ts: only the kept and skipped entries above.
