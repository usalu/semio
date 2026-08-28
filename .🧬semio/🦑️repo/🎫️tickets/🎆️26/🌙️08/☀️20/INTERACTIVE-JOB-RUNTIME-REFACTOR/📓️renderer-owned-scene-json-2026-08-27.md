# Owned Scene JSON Syntax

## Implementation Boundary

`OwnedUiSceneJsonCursor` captures an exact `OwnedUiPreparedScene` and checked field index. It opens the existing retained text-byte reader on its first admitted step. It retains at most one 256-byte input chunk, consumes at most one byte per lexical step, and emits fixed-size provisional token spans. It never builds a JSON object, concatenates the field, parses a complete number/string, or recursively visits nested values.

The explicit linked grammar frames preserve array/object state, colon/separator requirements, literal and number grammar, escaped UTF-16 code units, and raw UTF-8 scalar validity. JSON syntax permits `1e400` and escaped lone surrogates; schema-specific finite geometry admission is a separate obligation. Tokens are provisional until the complete input validates; they confer no ownership or publication authority.

Cancellation retires one frame per step, then the bounded chunk, exact byte-reader retirement, and captured prepared-root retirement. A shared parser can cancel without invalidating its sibling or source pages. Close propagates rejected/blocked child results. Final terminal state requires every frame, chunk, reader, and root owner to be empty.

## Accounting and Finite Test Bound

Production admission remains one item and 4,096 bytes. Lexical transitions charge 16–128 bytes; a frame pop charges 64; chunk release charges at most 288; reader/source transitions charge 128. Byte-reader work retains its actual existing accounting (at most a 256-byte copy per page). No grant credits accumulate and no step performs a whole-field conversion.

The fixture has exactly three source ScenePack records: the outer map, `buffer` key, and text value. Its test bound is `4 * B + 512` for input byte count `B`: at most `B` consuming lexical transitions; at most `B` non-consuming number-completion token transitions; at most two chunk transitions per input byte (a conservative bound on fetch/release, actually two per 256 bytes); and 512 reserved transitions for the fixed three-record lookup/start/EOF path. The 4,096-frame case now executes the same byte-derived bound and asserts monotonic offsets, at most one consumed byte, and fewer than 512 consecutive non-token/non-byte transitions. The 512 startup allowance is specific to this fixture, not a certificate for arbitrary source indexes.

## Schema and Third-Party Oracles

Language-neutral `scene-json.json` plus strict Ajv schema covers 12 valid documents, 18 malformed documents, a 24-KiB Unicode key/value document, 4,096 nested arrays, eight cancellation prefixes, and independently captured concurrent parsers. Node Buffer reconstructs raw token spans; `JSON.parse` supplies the syntax/value oracle, including duplicate escaped keys, `__proto__` data, negative zero, Unicode escapes and numeric conversion. Reconstructed objects exist only in tests.

## Executed Results

- R1: one actual constructor-missing failure, 623 skipped, 624 total; 5.69 seconds.
- R2: one passed, 623 skipped, 624 total; 9.65 seconds.
- R3: one passed, 625 skipped, 626 total; 11.56 seconds, including the derived deep-frame bound.
- R4: one passed, 625 skipped, 626 total; 14.63 seconds, including independent captured parser cancellation.
- Strict R1: 11 diagnostics: seven tutorial, one peer PluginRuntime fixture, one owned nullable fixture closure, two discovery. The owned fixture was corrected by capturing the checked non-null lifetime without an assertion.
- Strict R2: exactly nine diagnostics: seven tutorial and two discovery; no owned JSON/receipt/fixture errors. Full outputs remain in the corresponding `renderer-owned-scene-json-*` ticket files.
- Targeted `git diff --check`: exit zero.

All tests use the existing canonical React Nx `test-long --args='--run -t OwnedSceneJson'`; strict uses its canonical `typecheck` target. No new runner/dependency or native build was introduced.

## Exact Source Admission Follow-Up

A schema-first forged-source test reproduced a constructor admission defect: `source.capture()` invoked a structurally supplied callback and accepted its null result. The actual source-mint R1 run failed one test, 626 skipped, 627 total, 7.94 seconds, because the constructor did not throw. Capture now invokes the native `OwnedUiPreparedScene.prototype.capture` method on the supplied receiver, so its private field brand is checked before any caller-provided method can execute. The fixture asserts zero forged callback invocations. This is an admission fix, not a claim of a forged publication or source mutation. Combined scene rerun is pending at this report update.

Combined R3 completed: canonical `test-long --args='--run -t OwnedScene'` passed six tests, 621 skipped, 627 total, exit zero, 43.80 seconds. This includes the unchanged raw ScenePack laws, JSON source-mint repair, and packed-field byte preparation. Strict R3 contains exactly seven known tutorial diagnostics; no owned scene/base64/fixture diagnostics. Full outputs are `🧪️renderer-owned-scene-json-pack-r3-2026-08-27.txt` and `🧪️renderer-owned-scene-json-pack-strict-r3-2026-08-27.txt`.

## Remaining Live Work

This is syntax preparation, not a live host cutover. The existing host field decoder also accepts `pk:` plus generic pack base64. That path requires retained base64 and generic-pack traversal, preserving symbol references and arbitrary admitted text/collection content; the UI wire decoder's native 512-byte scalar/256-item profile cannot be reused as a silent domain restriction. Typed host views must consume prepared fields without rebuilding an entire compatibility object. All 15 surface schemas, exact per-instance intake/publication, React and TypeScript WGPU-web adoption remain in scope. Native finite-geometry/default parity and the native-u64/TS-safe53 identity boundary remain explicit separate obligations.
