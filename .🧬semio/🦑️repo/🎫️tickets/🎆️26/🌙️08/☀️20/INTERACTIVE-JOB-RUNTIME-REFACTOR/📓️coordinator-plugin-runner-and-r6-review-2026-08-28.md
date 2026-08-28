# Coordinator Plugin Runner And R6 Review

## Independently Executed Runner Selection

The coordinator executed the actual exported selector tests via Bun/Nx, without invoking Cargo. All six neutral cases passed, checked by strict Ajv and Node parseArgs; four selected script/project/schema/fixture hashes were equal before and after.

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false SEMIO_COVERAGE=0 bun x nx exec --projects=workspace -- bun -e 'import {pluginTestRunnerSelfTests} from '\''./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts'\''; console.log('\''[DEBUG] coordinator plugin runner cases='\''+pluginTestRunnerSelfTests());'
```

```text
[DEBUG] coordinator plugin runner cases=6
```

```text
c98da5ce13ef320d2bc14da17cea5550096a92066fd2ec0185311e528f7a0ac6  🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts
2cca2f0540a54a25e7d7b64d2b7d977c7dce1170e04dc701bc583544bf19c35f  🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📋️project.json
490db8d8cbaf920f7c3f6e55ae8cb59dbc3fe9c16960a74ec19339b15beb866f  🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🏃️runner/🧬️schema.json
f1adc09942b859300038975c785b7d9ea433d2a625a32ecd6fef3a6c9e6183b7  🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🏃️runner/🧪️fixture.json
```

Normal Plugin tests now use the existing shared level resolver and budgeted nextest runner. Explicit `--no-run` before the libtest separator remains compile inventory. Source review confirms no production synchronization, central runner, stack or timeout modification. The current target executes its selector self-test before selecting the real runner. Six-case proof does not establish all possible CLI inputs or production runtime behavior.

## Native R6 Actual-Output Review

The sole compiler's exhaustive full PatchTracker selection ran30 tests:29PASS,1FAIL,492unselected,0.646s,Nx1. Root read the actual raw tail before the file became absent; its surviving `📓️plugin-patchtracker-exhaustive-r6-native-2026-08-27.md` retains the copied output. All seven mounted-output-admission cases passed. Root also read the actual two-thread test and copied output:63 preoccupied shared slots, exactly one admitted contender, refusal of overadmission, and all64 slots reusable after explicit close. This is one concrete concurrent admission law, not an 8ms proof or whole-Plugin acceptance.

The sole failure occurs before the old test's intended overflow assertion. It computes32MiB/8MiB=4 without deducting the already-charged fixed contract/runtime backing. Root read the ledger's initialized CONTRACT_BACKING_BYTES, fixed_backing_bytes and exact reservation checks. The executor owns a test-first fixture correction using the actual checked remaining capacity, per-admission bytes/items/slots, exact overflow pointer and full bounded retirement; no capacity or accounting increase is authorized.

The earlier combined R5 remains2PASS/4FAIL. Its six individually passing runs used the same immutable binary; R6's canonical process-isolated suite and explicit same-process concurrency law are the newer evidence, not a relabeling of R5.

## Awaited Fixture Boundary

Generated validator registration is actual-output reviewed1PASS,521unselected,0.016s. The executor reports direct idempotent registration1PASS and checkpointSIGABRT with primary missing applyCountFromTask factory then secondary Store Drop. Their raw files were absent before root review; the report has missing-file placeholders, so those two outcomes are explicitly executor-reported here. Dag owns the exact retained fixture factory/disposer repair. See `📓️coordinator-native-evidence-absence-2026-08-28.md`; no raw output is reconstructed.

## Next Native Work

The newly registered neutral resident package reached a genuine missing-API compilerRED: one E0432 for four missing symbols; four mounted tests did not execute. Root read the actual retained error report and package metadata. Dag implements the sole native source after this RED; the sole compiler will rerun native4 and both declared Wasm targets. This is not a composition permit or live host/guest bootstrap result.

