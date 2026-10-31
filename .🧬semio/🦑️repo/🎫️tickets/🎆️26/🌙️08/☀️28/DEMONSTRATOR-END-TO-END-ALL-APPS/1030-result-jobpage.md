STATUS: GREEN. TS job-funds 11/11, wit-lowering 5/5, retained-turn 2/2; Rust retained-turn 11/11, host-guest-runtime 11/11, guest-contract 8/8 at default parallelism (incl. paged-output law with copy 128), actor 14/14 (`1030-tools/jobpage/test4.sh`, outputs jobpage-t4/t5.txt). Details of the final round: (1) CLOCK_CALLS race: the output tests counted clock calls in a shared static; now a thread_local counter (`clock_calls()`), no --test-threads crutch. (2) failure_publication red = STALE TEST: the Entry publishes the faithful keyed projection of OriginalTurnFailure (`project_original_turn_failure`: primary{kind,original}, receiptRefusal, contextRefusal, effects; consumers such as TS `faultFromOriginalTurnFailure` flatten secondaries into causes), the test still expected the flattened Fault with merged causes; expectation rewritten to the envelope. (3) actor diagnostics red = STALE TEST: `TypedOperationResultPage::close_step` re-quotes its demand from the grant (copy hint = grant copy), so a grant of quote-1 legitimately re-quotes lower and makes progress (copied 3135); the test now skips a denial whose own re-quote still fits (impl unchanged since before the wave; last change a doc comment). (4) host-guest-runtime: missing cfg(test) HeapWitness allocator added.

## Finding
The 64 was a hard `.min(64)` in the guest page size (`OS/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🦀️.rs` `read_job_output_step_with`) on top of `min(grant.maximum_copy_bytes, grant.maximum_capacity_bytes)`. The grant already decided the page in the existing tests (copy ceilings 1, 3, 64); but both hosts passed the whole loan (64 MiB), so removing the cap alone would page the entire output in one allocation. Intended design: the page = what the host grants one read crossing.

## Design (one source)
- Schema: `🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/💼️job/📄️output-page/🧬️schema/🔣️.json` (`outputPageBytes` const 16384, same as `JOB_PAYLOAD_PAGE_BYTES`).
- Rust: `OriginalJobBudget::OUTPUT_PAGE_BYTES` + `for_output_page()` (`retained-turn/💼️job/📥️source/🦀️.rs`) narrows only `maximum_copy_bytes` of the read crossing; the funding loan still settles the real spend. Test pins const == schema.
- TS: `🎟️retained-turn/🟦️.ts` imports the schema JSON (`ORIGINAL_JOB_OUTPUT_PAGE_BYTES`, embedded into the worker source too); `OriginalJobFunds.outputCrossing()` used by `readJobOutput`.
- Guest: `.min(64)` removed. Page = min(remaining copy, remaining capacity, host page).
- Native host (`OS/🔌️plugin/🖥️host/🎭️guest-runtime/💼️job-crossing/🦀️.rs`): `JobFunds::output_crossing()` for reads; a pageless unclosed page is now a retry bounded by 1024 consecutive pending pages (was an immediate trap), matching the TS host.

## Laws / tests
- N bytes complete in ceil(N/page) crossings, each copy grant <= page: TS `retained-turn/🧪️tests/💼️job-funds/🟦️.ts` (3 pages + 17 bytes -> 4 reads). Rust: `OS/.../🌉️call/📤️output/🧪️tests/🦀️.rs` now also runs copy ceiling 128 and asserts `pages == ceil(83/copy)` (was capped at 64 -> 2 pages).
- Pending = retry: TS existing tests (retry, stall bound, wall stall, resume); native loop mirrors it.
- Rust source parity test: `retained-turn/💼️job/📥️source/🧪️tests/🦀️.rs` (`original_job_output_page_bound_matches_its_schema_and_narrows_only_the_copy_axis`).

## Not changed (noted)
- Other 64-byte pagers: `⚛️reactor/💼️jobs/🏛️reserved/🦀️.rs:134` (reserved job body scan copy per step) and `🔄️turn/📥️receiving/📦️completed/🦀️.rs:37` (completed-input wire copy). Same stall shape for large payloads, different mechanism (per-step copy, not a crossing page); out of this lane.

## Verification
- `bun test ./<…>/retained-turn/🧪️tests/💼️job-funds/🟦️.ts` 11/11; `…/browser-bundle/…/🧬️wit-lowering/🟦️.ts` 5/5; `…/retained-turn/🧪️tests/🟦️.ts` 2/2.
- Rust: `C/1030-tools/jobpage/test.sh` (root `Cargo.toml`, `-p semio-framework-actor -p semio-framework-plugin-reactor-context -p semio-framework-plugin-host-guest-runtime`, `--lib`), output `C/🗑️generated/1030/jobpage-t1.txt/.rc`.
