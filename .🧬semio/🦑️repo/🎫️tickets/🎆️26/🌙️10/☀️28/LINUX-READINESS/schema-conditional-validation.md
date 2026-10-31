# Conditional JSON Schema Validation

The TypeScript owned subset silently ignored if/then/else and not. This allowed three real domain-invalid inputs through IFC machine validation, OS binding validation and Rust participation projection. The existing Rust implementation already enforces these predicates.

Added 28 language-neutral cases to the existing shared validator corpus, covering selected branches, ignored orphan branches, nonasserting conditions, boolean schemas, local references, nested conditionals and negation. The original implementation produced 100 passing and 12 failing cases; Ajv agreed with every expected verdict. The fix prepares branch checks once and applies only the selected branch, and refuses values accepted by a not schema.

Focused current proof: 112 cases and 336 assertions pass against independent Ajv, prepared validation and one-shot validation. Three additional exact-SHA real Source counterexamples pass their rejection regressions (115 total cases). The separate complete dialect cohort passed 121 of 121 nonvacuous contracts and 415,879 triple-validator observations, including the three formerly invalid acceptances.

The same neutral corpus is already read by Rust serialized and intrinsic validator tests. Actual canonical funded Rust package test33162 is running; no Rust result is claimed yet. Full framework/monorepo gates remain required.

Production files changed:

- 🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts
- 🧰️framework/🔨️modules/🧬️schema/✅️validator/🧪️tests/🧫️fixtures/🔣️.json

Retained proof input: schema-conditional.test.ts. Generated red/green logs are under generated and will be removed when the full ticket is finished.

Malformed reference follow-up: added12 neutral compilation refusals; unresolved references and non-schema values now throw during preparation, so not/if/anyOf cannot hide schema faults. Two initially constant oracle branches were refined because Ajv optimized unused references away; all12 final nonvacuous refusal cases independently throw in Ajv and owned compilation. Full focused suite127tests381assertions0. Existing Node/Bun oracle now uses the registered runToolV1 interface and reports129shared vectors0.

Canonical funded Rust49342 finished0:6of6 owning package tests passed, including serialized and intrinsic consumption of the112validity cases and the new compilation-refusal test. The initial33162 attempt failed before tests on an exact duplicate RetainedWireByteSource import in pack/value/decode/driver/Rust; removed only its duplicate, preserving its canonical import. Actual optimized crate build25.57seconds. No full-monorepo green claim.

Additional production files changed:
- 🧰️framework/🔨️modules/🧬️schema/✅️validator/🧪️tests/🟦️.ts
- 🧰️framework/🔨️modules/🧬️schema/✅️validator/🧪️tests/🔬️unit/🦀️.rs
- 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode/🎮️driver/🦀️.rs

Read-only review found malformed orphan branches skipped preparation although valid orphan branches must remain nonasserting. Added two neutral compilation refusals ({then:42}, {else:[]}); actual red127pass2fail proved Ajv rejected both while owned preparation accepted them. Removed only the condition gate around branch preparation, keeping runtime application under if. Actual green40831:129tests387assertions0. Both new refusals are in the existing shared Rust corpus; fresh actual Rust execution remains required for this addition. First unprefixed ticket Bun operand was refused by the operand guard before tests; corrected explicit ./ operand produced the causal red above.

Updated actual existing Node/Bun contract proof62848 finished0 with131shared vectors after both orphan-schema refusals. Registered Node interface was used; no fake Original or raw subprocess bypass. Fresh six-test Rust unit acceptance submitted normal after Lowpoly final terminal0.

Actual normal-admitted Rust50391 TERMINAL0: all6owning unit tests pass with current112validitycases and14compilationrefusals, including both malformed orphan branches. Prepared TypeScript/Bun/registeredNode/serializedRust/intrinsicRust use the same durable corpus. No full monorepo acceptance inferred.
