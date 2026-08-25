# P9-A6 OS Host Codec-Only Browser ABI Cleanup

## Verdict

**GREEN for the bounded P9-A6 source packet.** The OS host no longer exposes the four
`wasm_bindgen`/`serde_wasm_bindgen` codec functions and its crate no longer declares either direct
dependency. Their replacement is one schema-first A1 request/page/event/reply service whose public
surface contains only owned A1 types, primitive values, and OS-host-owned enums/records.

The public interactive route now has no constructor, field, trait, or call edge to the prior batch
`WorkflowFixture` codecs. It uses a session-owned `WorkflowStructuralCursor`: canonical DSL is
validated while each byte is admitted, and the pack operation consumes the schema-owned
`WFP1/version/canonical-length/canonical-DSL` framing the same way. Both operations therefore emit
the same canonical DSL bytes without first reassembling an input buffer. The existing real `.spk`
and DSL fixture law remains the offline semantic source of the canonical bytes. Format
normalization now uses a fixed 1,024-byte structural cursor and resolves the existing descriptor's
`short_id` on the final admitted byte. Accept-filter construction uses only fixed three-byte
header, two-byte length, and 1,024-byte current-kind state; it resolves each completed kind during
admission and appends descriptor extensions to independent bounded output state in declaration
order. Neither route retains a whole raw request or performs a post-seal slice parse.

## Scope

Changed production paths:

- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🟦️component.ts`, only the obsolete test-only generated-wasm consumer
- `🧰️framework/🛍️products/💻️os/🖥️host/🧬️schema/🔣️codec-abi.json`
- `🧰️framework/🛍️products/💻️os/🖥️host/🧪️fixtures/📒️codec-abi.tsv`
- `🧰️framework/🛍️products/💻️os/🖥️host/🧪️fixtures/🛡️hostile-batch-edge.rs`
- this report

No root Cargo manifest, Cargo lock, other crate manifest, shared script, Nx project, launch
configuration, Wasm output, browser surface, or unrelated product source was edited. The repo MCP
goal/ticket tools were not exposed in this agent session; work stayed inside the already-open master
ticket and its Phase 9 report directory.

## Direct Dependency And Source Census

| Boundary | Before | After |
| --- | ---: | ---: |
| OS-host direct `wasm-bindgen` rows | 1 | 0 |
| OS-host direct `serde-wasm-bindgen` rows | 1 | 0 |
| OS-host active bindgen exports | 4 | 0 |
| OS-host `JsValue` type occurrences in the 4 public signatures | 7 | 0 |
| Hand-written TS calls to the two fixture wasm exports | 2 | 0 |

The hand-written TS `normalizeStdioFormatKind` and `mediaAcceptFilterKinds` functions remain
independent native TypeScript twins; they were never callers of the removed Rust exports. The
checked-in generated renderer `🟨️frame-worker.js` still contains two stale strings copied from an
older build. It is not a source owner and was intentionally not regenerated because this packet
forbids Nx/Wasm/browser execution; its regeneration is a deferred derived-artifact gate.

## Schema And Wire Contract

Operation codes are fixed and bounded below A1's `4095` ceiling:

| Operation | Code |
| --- | ---: |
| Decode workflow fixture pack | 1537 |
| Parse workflow fixture DSL | 1538 |
| Media accept filter kinds | 1539 |
| Normalize stdio format kind | 1540 |

Nine owned domain errors distinguish malformed request, malformed pack, malformed DSL, missing kind
array, unknown kind, invalid UTF-8, input limit, output limit, and invalid state. Diagnostics are
fixed deterministic UTF-8 byte strings and remain inside A1's 1,024-byte bound.

An `AbiRequest` body carries only version `1` plus declared input length. Input is admitted as
ordered `AbiPage`s. The DSL operation carries canonical UTF-8 directly. The pack operation carries
magic `WFP1`, version `1`, canonical DSL byte length, and those canonical UTF-8 bytes. This is the
new owned interactive contract, not a compatibility route to the legacy whole `.spk` decoder.

Every input step consumes and structurally validates exactly one byte. The workflow cursor
incrementally validates UTF-8, the `name` prefix, required graph/dirty/delivery sections,
quote/escape state, balanced braces/brackets, control bytes, and the terminal newline. It writes
that byte directly into the already-framed retained output, so no complete input `Vec<u8>` exists
and seal only checks scalar cursor state and moves the owned output into A1 paging.

The filter cursor admits version/count/length fields one byte at a time, rejects count 257 and
length 1,025 before current-kind copying, incrementally validates UTF-8, and resolves exactly one
kind on its completing admission opportunity. The normalizer rejects declared length 1,025 in
`begin`, validates UTF-8 into a fixed current-kind array, and resolves only on the final admitted
byte. Seal for either route performs scalar completion checks and moves already-resolved output
into A1 paging; it never receives a raw input slice. Output copies one byte per A1 reader grant.
The public service exposes no input slice, raw input vector, string, fixture, registry, serde,
browser, or external runtime type.

`OsHostCodecService::new` constructs only `RegisteredOsHostFormatResolver`. There is no workflow
backend capability in the service, and the codec production region contains no `UiForbidden`,
`ArtifactPack`, `ArtifactDsl`, `decode_pack`, or `parse_dsl` token/call edge. It also contains
no `Bytes(Vec<u8>)`, `Self::Bytes`, `input.bytes()`, or `execute_filter` whole-input edge.

Output payloads are version `1`, reply-kind byte, little-endian byte length, then canonical DSL or
UTF-8 result bytes. The final `AbiReply` carries a six-byte version/kind/output-length summary.
Progress is an ordered A1 event with phase, completed units, and total units. Output pages remain
retained until exact handle-generation-index ACK.

## Lifecycle Laws

- request IDs use direct modulo-256 slots; collisions are `Busy` rather than scans;
- handle reuse, loss, stale generations, and ABA classification are delegated to accepted A1;
- max input/output are 1,048,576 bytes, pages are 65,536 bytes, transfers are capped at 256 pages,
  kind count is 256, and kind bytes are 1,024;
- max-plus-one request admission returns the exact unconsumed request; page rejection returns the
  exact allocation;
- cancellation after a partial page returns the exact original page plus admitted/copied counts and
  makes later progress terminal;
- pack, DSL, filter, and normalize cancellation can occur between any two structural bytes;
  no route has an uninterruptible post-seal parse;
- zero credit, interruption, and expired deadline do not advance input or progress;
- duplicate output ACK is rejected; no next page or final reply is published before ACK;
- close retires pending page, copied input, decoded items/errors, and output one retained unit per
  admitted close step; interrupted close does not advance;
- handle loss clears the direct request slot and rejects late work; no browser handle is retained.

## Executed Gates

| Gate | Result |
| --- | --- |
| direct dependency-free debug `rustc -D warnings --test` against accepted A1 source | GREEN — 30 passed, 0 failed |
| direct dependency-free optimized `rustc -D warnings -O --test` | GREEN — 30 passed, 0 failed |
| feature-enabled public-wrapper retained structural-cursor suite with dependency-free owned stubs | GREEN — 32 passed, 0 failed |
| feature rlib plus external public-wrapper link/run | GREEN |
| hostile source fixture over the real component | GREEN — normal compile/run |
| hostile raw-`Bytes(Vec<u8>)` and whole-slice injections | GREEN — both rejected by the source-law assertion |
| hostile compile fixture trying to name/inject the removed batch backend | GREEN — expected `E0425` |
| `rustfmt --edition 2021` plus focused `--check` | GREEN |
| Bun standard-library schema/ledger/fixture-pair parser | GREEN — 4 operations, 9 errors, 10 ledger rows, 5 DSL/SPK pairs |
| host source/manifest deny-list for browser ABI/value tokens | GREEN — zero matches |
| host direct manifest row census | GREEN — zero rows |
| focused `git diff --check` | GREEN |

The 30-test total contains sixteen P9-A6 laws and the accepted A1 module's fourteen laws. P9-A6
covers every byte/header-field split for pack and DSL; exact canonical equivalence and deterministic
reply; malformed/truncated pack and DSL; invalid UTF-8; missing and unknown kinds; exact max/+1
input/output; max-page-count rejection before sequence classification with exact page return;
mid-operation cancel; no-credit/interruption/deadline non-advance; handle loss; stale generation;
duplicate ACK; interrupted close; source-edge quarantine; schema and language-neutral ledger
reconstruction. Filter and normalize coverage additionally exercises every byte/field split;
zero/256/257 kinds; zero/1,024/1,025 current-kind bytes; malformed/truncated framing and UTF-8;
missing/unknown kinds; mid-item cancellation; non-advancing budgets; deterministic result order;
exact page ACK; and interrupted close.

The feature-enabled run adds two P9-A6 laws that construct the actual public
`OsHostCodecService`: one streams canonical DSL through the workflow cursor, and the other
exercises the filter and normalize structural cursors. They ACK retained pages and receive owned
replies without any workflow batch capability or raw-input service variant.

## Deferred Runtime Gates

- No Cargo workspace/package, Nx, Wasm, or browser command was run, per packet instruction.
- The existing real-crate `workflow_fixture_dsl_and_spk_pairs_are_canonical_and_equivalent` law is
  preserved unchanged but was not re-executed without Cargo. It remains the offline producer law
  proving real `.spk` and DSL inputs yield the canonical DSL carried by the interactive structural
  pack. Runtime invocation through the full crate remains a coordinator integration gate.
- The generated renderer worker bundle must be regenerated only in the later authorized renderer or
  build-artifact packet; its two stale removed-export strings are not claimed clean here.
- No JS linear-memory shim was added: the only hand-written caller was test-only and is gone. A
  future actual browser consumer must use the shared A1 generated low-level page shim, not invent a
  second OS-host ABI.

These deferred gates prevent claiming P9-A or Phase 9 completion; they do not block the bounded
P9-A6 source/declaration cleanup.
