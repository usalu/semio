# Store Fixture Codec Review 41

## Scope

Reviewed the frozen test-only Store fixture packet rooted at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store`, its fifteen direct leaves, the four test-fixture aggregate codecs in the Store test module, and the real `os_pack` facade. No Store production/history/lifecycle/timestamp source was changed.

The fifteen leaf inputs are discovered from the existing neutral fixture manifest and span Demo (4), Timestamped (2), Severity (5), Validated (2), Lossy (1), and Presence retirement (1). The controller reads and hashes each direct leaf, schema, descriptor, and aggregate under no-follow and no-`compose` guards.

## API and codec result

The actual facade declaration is:

```rust
decode_record_body(bytes: &[u8], spec: &RecordSpec, options: &DecodeOptions)
```

The four retained Store fixture aggregate codecs—`DemoMutation`, `TimestampedMutation`, `SeverityMutation`, and `ValidatedMutation`—currently call it as:

```rust
decode_record_body(body, &spec, &PackDecodeOptions::default())
```

They do not contain the rejected `decode_record_body(&spec, body, ...)` order. Therefore no Store source correction was made: changing already-correct code would risk a concurrent test-fixture packet without resolving a real error.

`🧪️store-fixtures-39/📜️script.ts` now parses the actual `os_pack` function argument list and the braced `OpBinary` implementation for each of those four fixture aggregates, then compares the extracted call arguments. This is a source-structural API-boundary regression rather than a generic `includes` check. It also retains the full existing neutral schema/descriptor/inverse/absence checks for all fifteen leaves.

## Executed controller result

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-fixtures-39/📜️script.ts
```

Passed: **378 assertions, 0 failures**. Retained receipt: `🧪️store-fixtures-39/🧫️run-fu6H5h/🔣️result.json`.

The controller’s first invocation after adding the regression stopped before assertions because its own temporary strict-mode identifier was named `arguments`; that controller-only syntax error was corrected immediately, then the retained green run above completed. No pre-change Store source red is claimed.

## Stable source inputs from the retained receipt

| Input | SHA-256 |
| --- | --- |
| Store component | `1158a4701f4387f45aa1081b3037a9f516621273a4200d9edae9267abbfe8102` |
| Presence retirement component | `e3ae8ceb8e4c0035e94006848f1a323e8b5d268ede5a6366a5be2695cff5fe71` |
| `os_pack` facade | `fbcbf7d9fa0f8f7e148f0a66631808a13e914b74c47677cecea70b8fe5062547` |
| Store fixture controller | `f0dbffc1d986d83f0327f1e296dc28e595fccf82a29e13d4c1e154e15cfdbf23` |

The receipt contains the complete per-leaf and schema hash roster. The controller re-read every input after checks, so it would fail on detected source drift.

## Native boundary

No Cargo or Rust compiler command was run. The 378 assertions are Bun/Ajv/jsonc/source-structure evidence only. Existing native Store fixture tests, including optional absence and inverse behavior, remain pending the root-controlled compiler slot.

## Native Red And Canonical Text Repair

Root's OS-kernel selection compiled successfully in 50.50 seconds. Of94 selected tests,89passed, one failed, and four were not run after the failure. The failure was `direct_store_fixture_absence_inverse_boundary` at Store component line19616: the aggregate rejected `bump-n delta=4` after the direct `AddN` leaf had printed it. The retained native evidence is `🧪️os-kernel-fixtures-41/🧫️run-wKD0uM/🧪️test-089.log` with its accompanying compile and JSON receipts. The four unexecuted tests have no result.

Cause: `AddN`'s semantic/aggregate identity was canonical `add-n`, while its descriptor and `DslRecord` keyword still used the retired `bump-n` spelling. The aggregate dispatch therefore could not parse the leaf's own printed text.

The fixture-only repair changes both authoritative leaf facets to `add-n`:

- `🧪️fixtures/🧮️demo/🧬️mutations/➕️add-n/🦀️.rs`: `#[dsl(keyword = "add-n")]`.
- `🧪️fixtures/🧮️demo/🧬️mutations/➕️add-n/🔣️.json`: `textOpcode: "add-n"`.
- `🧪️fixtures/🔣️mutations.json`: canonical `AddN` opcode plus a schema-first text-codec vector with canonical `add-n delta=4`, rejected `bump-n delta=4`, and binary tag 2.
- Store's cfg(test) fixture test: every aggregate case now asserts printed text against the actual descriptor text opcode and encoded binary byte 1 against its descriptor binary tag. The new AddN vector asserts canonical parse/print and old-spelling rejection.

No compatibility spelling, history change, lifecycle change, or timestamp behavior was added.

The ticket controller now checks all descriptor-advertised text opcodes against the direct leaf `DslRecord` keyword; checks every non-null binary tag against its aggregate index and family uniqueness; and checks the explicit old-text rejection vector. The first expanded neutral run correctly exposed two no-text leaves as an overbroad controller condition; the condition was narrowed to descriptor-advertised text surfaces only. This was a controller test expectation correction, not an additional Store source defect.

Final neutral execution:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-fixtures-39/📜️script.ts
```

Passed: **428 assertions, 0 failures**. Retained receipt: `🧪️store-fixtures-39/🧫️run-Ijj7XN/🔣️result.json`; native rerun is pending root's next compiler slot.

Current key source hashes are Store component `15d1779465873bf6fba1ce9c37f484a134bbd2cd2ae0d651ea12b3e9f2c59f49`, AddN leaf `2ab82e1572bca658cc6b4a20498b9ec39f84f690e3aa37a7c06dc021ace4d0d9`, AddN descriptor `f38717b513242de76ff88f3736954f571a5336eceb7beff13dad2c6ede0898b3`, neutral fixture `13e246d1f24887c597b25a271c5818f014e3f5974ad612837a1076968ea54f87`, and controller `f15f79b51192ebe77cdf8498092076b7045ccc8fd4b4e71b901af78668cd899c`. The receipt retains the complete source roster.

Root subsequently corrected the two newly added native binary-tag assertions to compare the actual descriptor's u32 tag, converting byte1 with `u32::from` and the neutral JSON tag with checked `u32::try_from`. This is a test-source correction before the next compiler gate; the preceding hash receipt is retained as its actual earlier snapshot, not current-source readiness.
