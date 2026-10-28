# Original Question Dictionary Equality

The first-party dictionary owner exports QuestionDictionaryEqualityCursor for original optional dictionaries and QuestionValueEqualityCursor for original intrinsic DslValue projections. Both expose new, demand/next_demand, advance returning RetainedEqualityStep { equal: Option<bool>, progress }, result, verify, verify_original, begin_close, close_demand, close_step and terminal_is_empty. Both preserve complete Number variants and f64.to_bits, ordered keys/members, repeated intrinsic object keys, bytes, all recursive variants and absent versus empty ownership. They never call cold PartialEq or build a substitute graph.

Original issued left/right bindings precede all borrowed pair loans. Exact native paged frames hold immutable original references; one ordinal child or at most 256 source bytes compares per funded turn. Fixed own copy/depth overhead is subtracted before delegated work and error receipts preserve it. Normal close drains pending/frame slots, releases actual page allocations, then right/left source aliases. Rust lifetime extension is protected by those original aliases and child-before-alias closure. No source can be replaced while either pair binding is live.

Reusable settled-pair lifecycle is begin_rebind, rebind_demand, rebind_step, ready_to_bind. It drains prior borrowed slots and closes both old aliases under actual grants while retaining empty page backing. A separate funded final state reset precedes new binding. Normal final close releases that backing once. Cancellation can interrupt rebind and immediately enter normal closure. No new actor or page allocation occurs per equal-depth row.

TypeScript follows the same algorithm and exposes camelCase methods with explicit logical page accounting; its original source wrappers model identity/projection/value qualification. Native uses the genuine RetainedCloneRef issuer and exact physical PagedList allocations. The TS ledger is not a physical JS heap claim.

The durable question-equality-check.mjs harness returned strictDiagnostics 0; 22 shared lossless witnesses agree with existing fast-deep-equal on independently owned variant words and ordered member arrays. Main cases used 289 turns and 973 denials. Eight cancellation cuts plus eight cancellation-during-rebind cuts paired all capacity and release. Long Unicode keys/text used 396 turns. A depth-1024 intrinsic tree used 4,645 turns; 256 to 512 dictionary rows used 1,541 to 3,077 turns. Reuse across 256 alternating equal/false pairs used one 3,072 logical byte workspace and 4,481 turns. Test-only deep snapshots use iterative witnesses because Node structuredClone has a lower recursive stack limit; the production comparator is iterative.

Native real-issuer, exact-byte lifecycle, source substitution, special float words, deep stack, and rebind reuse/cancellation laws are authored. All three changed native dictionary sources (root, equality, equality tests) were syntax inspected; no native compilation or execution was possible because the required repo runner is unavailable. Syntax is not compile evidence.

Exact agent-owned changed paths for this slice:

- `🧰️framework/🔨️modules/🌱️value/🧾️dictionary/🟰️equal/🔣️.json`
- `🧰️framework/🔨️modules/🌱️value/🧾️dictionary/🟰️equal/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/🧾️dictionary/🟰️equal/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🧾️dictionary/🟰️equal/🧪️.json`
- `🧰️framework/🔨️modules/🌱️value/🧾️dictionary/🟰️equal/🧪️tests.rs`
- `🧰️framework/🔨️modules/🌱️value/🧾️dictionary/🟰️equal/🧪️tests.ts`
- `🧰️framework/🔨️modules/🌱️value/🧾️dictionary/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🧾️dictionary/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/🧾️dictionary/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🌱️value/📦️packages/🟦️typescript/🧾️dictionary/🟦️.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/LAYOUT-FEATURE-COMPLETE/question-equality-check.mjs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/LAYOUT-FEATURE-COMPLETE/QUESTION-EQUALITY.md`
