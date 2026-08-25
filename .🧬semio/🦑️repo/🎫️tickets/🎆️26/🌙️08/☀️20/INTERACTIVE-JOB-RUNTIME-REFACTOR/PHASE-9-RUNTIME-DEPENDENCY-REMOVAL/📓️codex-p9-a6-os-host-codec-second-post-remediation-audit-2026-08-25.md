# P9-A6 OS-Host Codec Second Post-Remediation Audit

## Verdict

**RED — do not accept P9-A6.** The prior P0 is fixed for the two workflow routes:
the live public `OsHostCodecService::new` constructs only
`RegisteredOsHostFormatResolver`; the workflow routes retain a
`WorkflowStructuralCursor`; and the codec-production region contains no
`UiForbidden`, `ArtifactPack`, `ArtifactDsl`, `decode_pack`, or `parse_dsl`
edge. The required rule is nevertheless universal: *every* public interactive
operation must not reassemble or feed a whole input buffer. The live filter and
normalization operations still violate it.

`OsHostCodecInput::new` selects `Bytes(Vec<u8>)` for both
`MediaAcceptFilterKinds` and `NormalizeStdioFormatKind`. Every admitted byte is
then appended by `OsHostCodecInput::feed` (live
`🖥️host/🦀️component.rs:4959-4989`), and, only after `seal`, `execute` borrows
the complete slice for `str::from_utf8` or `execute_filter`
(`:5192-5254`). Filter parsing consequently retains the entire array before it
processes even its header; normalization retains the entire kind before it
validates UTF-8 and resolves it. This is a reachable public path through
`OsHostCodecService::{begin,offer,seal,step}` (`:5524-5560`), not a test-only
or private batch helper. It fails the stated no-whole-buffer / one
admitted-byte-or-item-grant P0 even though the byte cap is bounded.

## Remediated Workflow Boundary

The former RED trace through `UiForbiddenOsHostWorkflowBatchBackend` is absent.
For operation codes 1537 and 1538, `OsHostCodecInput` holds
`WorkflowStructuralCursor`, never `Bytes`. That cursor independently handles
WFP1/version/u32-length framing, UTF-8 continuation state, quote and escape
state, balanced braces/brackets, required `name`, `graph`,
`dirty-node-ids`, and `expected-deliveries` markers, terminal newline, and a
single retained canonical output payload. `finish()` takes the output vector
into A1 paging; it does not reconstruct an input vector or call an offline
codec. Seal is scalar validation plus the owned handoff.

The source-level deny census of the codec production subsection (from
`pub mod codec_abi` to its test region) is zero for all seven forbidden terms:
`UiForbidden`, `ArtifactPack`, `ArtifactDsl`, `decode_pack(`,
`parse_dsl(`, `decode_workflow_fixture_pack`, and
`parse_workflow_fixture_dsl`. The 28 matching tokens in the whole host source
are historical/offline workflow code outside this public codec region.

## Gate Results

| Gate | Result |
| --- | --- |
| Ticket-local debug retained ABI binary | GREEN — executed, 26 passed / 0 failed |
| Ticket-local optimized retained ABI binary | GREEN — executed, 26 passed / 0 failed |
| Ticket-local feature/public-service binary | GREEN — executed, 27 passed / 0 failed |
| Fresh wrapper compilation against the feature ABI rlib | GREEN — compiled and executed |
| Fresh hostile source injection | GREEN — rejected with required `E0425` for `UiForbiddenOsHostWorkflowBatchBackend` |
| `rustfmt --edition 2021 --check` on host component | GREEN |
| Focused `git diff --check` | GREEN |
| Host source external-browser ABI census | GREEN — 0 `wasm_bindgen`, `serde_wasm_bindgen`, `JsValue`, `web_sys`, or `js_sys` matches |
| Direct host manifest dependency rows | GREEN — 0 `wasm-bindgen` / `serde-wasm-bindgen` rows |
| Bun schema/ledger/static-pair harness | GREEN — 4 operations, 9 errors, 10 ledger rows, 5 DSL/SPK pairs |
| Public all-operation no-whole-input contract | **RED** — filter and normalize retain `Bytes(Vec<u8>)` |

The 26/27 binaries are ticket-local test artifacts and were executed as the
authorized no-Cargo harness. The wrapper and hostile probes were freshly
compiled with `rustc`; no Cargo workspace/package, Nx, Wasm, or browser command
was run.

## Canonical And Derived-Artifact Checks

The static harness verifies five one-to-one DSL/SPK fixture names, every DSL's
terminal newline and required workflow sections, and every SPK's PNG-style SPK
magic (`89 53 50 4b 0d 0a 1a 0a`). It cannot decode the real SPK grammar without
the prohibited Cargo/Wasm route. The source retains the real
`workflow_fixture_dsl_and_spk_pairs_are_canonical_and_equivalent` law, so this
is correctly classified as static fixture-pair evidence rather than a fresh
runtime equivalence claim.

Filter semantics are structurally correct after data is buffered: each item
resolves through the registered resolver and declaration-order extensions are
joined; normalize returns the registered `short_id`. This does not cure their
admission violation. The checked-in generated frame worker has exactly two
removed-export references, both inside `if (import.meta.vitest)`: one
`parseWorkflowFixtureDsl` and one `decodeWorkflowFixturePack`. They are stale
Vitest-only derived calls, not a production route; regeneration remains outside
this no-Nx/no-Wasm packet.

## Required Closure

Replace `Bytes(Vec<u8>)` for codes 1539 and 1540 with bounded incremental
cursor states. A filter cursor must admit and validate its version/count/length
fields and UTF-8 kind bytes as they arrive, resolve exactly one completed kind
under one admitted item opportunity, and retain only its bounded current-item
state plus result. A normalizer cursor must validate and resolve its bounded
kind incrementally without reconstructing a `Vec` or `String`. Add public
service laws for page/field splits, cancellation, interruption and deadline at
those boundaries. Reaudit only after this removes the sole remaining P0 path.
