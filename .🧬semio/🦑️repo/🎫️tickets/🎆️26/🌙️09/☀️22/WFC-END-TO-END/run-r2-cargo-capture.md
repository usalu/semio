# Cargo Whole-Capture Parser

The owned Cargo capture contract is one complete JSON object. JSON whitespace and newlines are valid. The old linewise parser rejected valid pretty objects and could admit an array or discard junk lines. A pure domain-neutral parser now parses the entire capture once and accepts only a non-null, non-array object. Empty, truncated, JSONL, concatenated, scalar and junk captures refuse with the existing generic caller errors; the new parser refusal does not print machine stdout. Existing planning failure wrappers may retain bounded raw stderr as standard diagnostic context.

This fixes a concrete parser contract defect. It does not establish that pretty JSON caused native session56485: that session did not preserve the failing stdout.

## Test-Driven Validation

The same isolated registered Bun suite was red with the old extracted parser:19 passed,7 failed. With whole-capture parsing it passes26 tests,0 failures,51 assertions,479ms. The portable schema describes25 cases; each expected result is independently checked with the existing strict jsonc-parser implementation. Existing Ajv validates fixture schema. No runtime external dependency or package/lock change was introduced.

The requested existing Ajv JTD compileParser({values:{}}) oracle runs under registered Node and matches24 cases. It has an explicit observed limitation on the valid pretty empty object (opening brace, newline, closing brace), which is independently checked by jsonc-parser and the native JSON parser. No implementation test skips that case. Bun JTD also mishandles arbitrary nested values; no such Bun JTD result is counted as validation.

Five edited TypeScript files parse without diagnostics; maximum checked absolute file path169 characters. Scoped diff HEAD --check passes. Actual native compilation remains outstanding.

## Worked Global Paths

- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/⚙️configuration/📥️capture/🟦️.ts
- Same capture directory: 🧬️schema/🔣️.json; 🧪️tests/🔣️.json; 🧪️tests/🟦️.test.ts; 🧪️tests/👓️oracle.cjs.
- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🟦️.ts
- Same product configuration directory: 🧪️tests/📥️runtime/🟦️.ts.

Production compiler configuration capture and generic Cargo metadata/unit-graph capture share the pure parser. The configuration runtime law uses the same parser. All existing source, storage, environment, profile and admission gates remain in place.

## Retained Inputs and Temporary Evidence

Ticket cargo-capture.bunfig.toml keeps the isolated test free of the full native import graph. Red/green logs remain under generated until final ticket cleanup. The native retry uses the unchanged owned repo cargo ingress, with no raw compiler or guard override.

## Separated Machine Channels

Native75927 still refused configuration capture after whole-object parsing and terminated exit1 before WFC source admission/compiler. Read-only implementation inspection then proved captureOwnedCommandV1 returns a human diagnostic tail formed from both stdout and stderr. This is unsuitable for a machine JSON contract. Whether warnings caused75927 remains unproven: its actual capture was not preserved.

Only machine JSON configuration and planning consumers now use the existing captureOwnedCommandResultV1 separated API with unchanged original control and byte/line budgets. That API refuses overflow and incomplete/invalid encoding before returning; successful status0 and null signal are required before parsing complete stdout. General human tail semantics remain unchanged. The configuration runtime law also uses separated capture; expected Cargo refusal still must have nonzero terminal status. The new configuration-capture refusal copies no stdout/stderr values. Existing CargoPlanningFailureV1 may append bounded raw stderr as standard diagnostic context, and existing owned streaming stderr forwarding remains enabled; this change guarantees machine stdout is not logged. Invalid configuration reports only numeric stdout/stderr byte counts, terminal status and a signaled boolean as safe shape evidence.

Eight schema-first response cases cover warning stderr, pretty JSON, interleaved diagnostic rows, stderr-only valid JSON, malformed stdout, failed exit, signal and missing terminal status. Existing independent strict jsonc-parser validates all accepted response outputs. The transport-selection red law had29pass/5fail; final isolated registered Bun suite35pass/0fail/68assertions in290ms. Five edited TS files parse and all45 relative imports exist; max path169. Scoped diffHEAD--check passes. New global fixture/schema paths: capture/🧪️tests/📬️responses.json and capture/🧬️schema/📬️responses.json.

Bitmap6900 terminal exit1 before compilation: existing separated capture API has hard16MiB maximum, while actual generic Cargo planning declares128MiB. Its refusal message mentions finite Original control but the proven failing predicate is maximumBytes>16777216; config budget is4MiB. No current native job. Original planning envelope must remain unchanged; no API limit raise or admission bypass is authorized. Evaluating existing controlled streaming outputConsumer interface for the large machine response.

## Original-Budget Streaming Planning Port

The first separated retry6900 exposed a real API-bound mismatch: generic Cargo planning declares128MiB while the existing separated-result API permits16MiB. No ceiling or original budget was changed. The generic planning port now uses existing runOwnedCommand outputConsumer under withIssuedResourceOperationV1, with original control, exact control signal, remaining timeout,128MiB/65536-line envelope, stdout ignored for logging, and stdout-only strict streaming UTF-8 decoding. String chunks are joined once after the successful settled outcome and control recheck. Existing stderr forwarding remains unchanged. Four-MiB configuration capture retains the existing separated-result API: inspection confirms independent fatal TextDecoder per stream with stream:true and final decode flush.

Pure injected-runner tests do not forge resource authority. They exercise stderr interleave, every-byte emoji/Unicode split, nonzero/cancel/output-limit propagation, invalid UTF-8, and truncated final flush; actual budget enforcement remains in the existing owned runner. Response and byte-stream refusal fixtures are schema-first portable JSON. Existing independent jsonc-parser verifies parsed successful response values. Streaming red44pass/5fail; final complete focused suite51pass/0fail/85assertions in368ms. Five TypeScript files parse,49 relative imports exist, maximum checked path169; scoped diffHEADcheck passes. Additional global inputs: capture/🧪️tests/🧵️streams.json and capture/🧬️schema/🧵️streams.json.

Bitmap6887 terminal exit1 at streaming adapter: Original remainingMilliseconds returned a fractional value while existing timeout requires a safe integer. Adapter corrected to floor the remaining value after positive deadline check, never extending authority; null retains0 with Original cancellation signal. This was a new adapter integration error, before WFC compilation. Narrow TS parse passes. Sole retry follows with exact same original budgets.

Preserved six portable timeout regression fixtures/schema under capture/tests and capture/schema ⏱️deadlines.json, including fractional119876.234567->119876 and positive<1 refusal. Existing Ajv validates the schema. The actual integration red6887 exposed the safe-integer requirement; actual retry77440 passed that boundary. No runtime or current gate source changed while compiling, and no new fake-authority test claims are made.
