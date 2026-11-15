# native-grammar: hull assertion and the failures behind it

## 1. Hull assertion (fixed, verified)
Root cause: wrong test expectation. `compileNativePrimitiveHullEffects` compared the stock cases (`stock-convex`, `stock-concave`, no `data=`) with rings computed from the authored `hull-contract` points. Stock cases draw the built-in `demo` table `(t,val)` = (1,4),(2,7),(3,3),(4,8),(5,5); the TeX ring (5 vertices; the concave one is identical because no interior point exists) was correct. Authored cases already passed.
Oracle: `d3-polygon` `polygonHull`/`polygonArea` (independent third-party), now fed the demo points scaled with `d3-scale` extents.
Verification: in the later full run (r7) the hull test no longer failed; the run then failed at later stages (below).

Files:
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🔣️.json` (`nativePrimitiveHullEffects.stock.points`)
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🏭️domain-effects/🟦️.ts` (per-case `ring`/`points`, `extent` import)

## 2. `Plain acyclic PDF inspection DATA required`
Fixed by another agent in `🧰️framework/🛍️products/📓️print/📦️artifacts/📖️pdf/🟦️.ts` (undefined operator args treated like null). I only added a diagnostic in `📦️artifacts/📖️pdf/📜️script.ts` (operator name, index, page and arg types in the refusal).

## 3. Native emission (next failure in the full run, r11)
`Owned data capture output limit exceeded ... stderr 8193/8192 lines`. Causes found with the narrow target `@semio-tech/print-rs:test-native-emission`:
1. Cargo replays ~4,500 workspace warnings per invocation (Rust-wide lint campaign, `unused_qualifications` etc.) through the emission child, far over the 8192-line / 2 MiB capture; `--message-format=short` is ignored by the launcher.
2. The emission command filtered `-p semio-framework-print`, but the tests live in `semio-framework-print-tests` (`mod tests` of the companion crate): cargo ran **0 tests**, so no wire rows could ever be produced.
3. The fixed 120 s deadline expired on a cold `-j1` build; the repo convention is `buildBudgetMs()`.

Fix: `🧰️framework/🛍️products/📓️print/🧪️tests/🔬️rust-package-commands/🟦️.ts` (`runNativeGrammarEmissionScript`): package `semio-framework-print-tests`, `--quiet`, `buildBudgetMs()`, cargo runs with `emit=false` (diagnostics captured and discarded under the 16 MiB / 65536 line limits) and the wire rows go through a temp file (`SEMIO_PRINT_NATIVE_EMISSION_FILE`), which the script reads back and prints to stdout unchanged (same `SEMIO_PRINT_NATIVE_V1` protocol toward the emitter).
`🧰️framework/🛍️products/📓️print/🧪️tests/🧬️chart-mutations/🦀️.rs` (`emit_native_grammar_source`): appends the row to that file when the variable is set, else `println!` as before.
Caveat: each cargo invocation still produces ~52k diagnostic lines, below the 65536 cap but not by a wide margin; it shrinks as the lint campaign lands.

## Update after disk was freed
- Capture margin: the emission cargo invocations now also pass `--message-format=json`, so the discarded diagnostics are one JSON line per warning (about 4.5k lines instead of about 52k) and the line cap no longer depends on the warning count; wire rows travel via the file, never via the captured streams. Bytes stay well under the 16 MiB cap.
- Not verified: r20 and r21 (`repo run @semio-tech/print-rs:test-native-emission --output=stream`) were refused at planning by the per-host graph service: `request-failed 494e1576 ENOENT lstat .cache/b/g/m/build-input-closures/91c289...617.json`. That closure file was removed (the directory is now empty), most likely by the `repo cache cas gc` I ran while the disk gate was refusing runs. The resident service (pid 12553, shared by all agents) still holds the reference.
- Needs one action I did not take because it affects every agent: `repo service host stop` (the service restarts on the next request and recomputes the closure), or another owner-approved refresh of that memo. After that: the emission run, then one full `repo run @semio-tech/print:test-native-grammar --lang=en --output=stream --skip-remote-cache`.

Final exit codes: hull assertion fixed (r7 passed it); emission r15 exit 0 but ran 0 tests (invalid), r16/r17 failed for infrastructure and the 0-test filter, r18/r19 disk gate, r20/r21 graph service ENOENT (exit 1). `test-native-grammar` has no passing run yet.
