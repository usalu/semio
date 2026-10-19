# Result law5: laws 34 and 46 redesigned, durable bounded journal fixtures fixed

Executor `law5` (Sonnet 5.5). Foreground only, no sub-agents, no cargo, no modifying git command, no `package.json` / `bun.lock` edit, nothing under the frozen storage / revision paths. No storage change is needed for the journal fix (all defects were in the law). Abbreviations: `P/` = `🧰️framework/🔨️modules/🏃️process/`, `C` = this ticket folder.

## Result (law3 harness, `LAW3_CAPS` native grant, `LAW3_ART=ticket`, `run1.py`, whole file)

| suite | before (pass/fail) | after |
|---|---|---|
| row 34 `P📦️artifacts/🏗️native-build/📥️inputs/🧪️tests/📥️consumers` | 0 / 2 | 2 / 0 |
| row 46 `P🧪️testing/🦀️cargo/🏃️execution/🧪️tests` | 15 / 2 | 25 / 0 |
| `P📦️artifacts/💾️cache/🧹️retirement/🧪️tests` (`durable bounded journal`) | 56 / 1 | 57 / 0 (8 consecutive full-file runs at load average 38, all green) |
| neighbours `inputs/🧪️tests`, `inputs/🧪️tests/⚙️operation` | 3/0, 1/0 | 3/0, 1/0 |

Type check: the three test files were checked with a temporary `tsc --noEmit` project (extends the framework tsconfig, `files` = the three tests): zero errors in them after two cast fixes (temporary project deleted).

## Row 46: what was wrong, and the new design

Evidence (probed, not assumed):

* `executeCargoRequestV1` ends with `scriptInvocationEnvironment(invocation, ...)` which requires an *issued* invocation (`requireIssuedScriptInvocationV1`). The law built a literal `ScriptInvocation`, so the positive rows `original` and `cancel-during-execution` could never reach the port (failures 1 and 2), and no row could prove anything about the port path. Refusal tests passed without the asserted reason ever being checked (`catch{refused=true}`).
* The `command delegation` test used `try { ...; expect(plan...) } catch { accepted = false }`: any exception, including a failed `expect` in the positive checks or a missing fixture file, counted as "refused". Real instance: `foreign-manifest` pointed at `other/Cargo.toml`, which the fixture never created, so it was refused with `ENOENT` (verified), not for the foreign selection. Now `other/Cargo.toml` exists, so the refusal is the production one.
* Issued invocations are deeply frozen (`readScriptProcessJsonV1` freezes the capability graph; the invocation and control are frozen), so the old "after publication" tampering rows (`Object.assign(invocation, ...)`) can only run against a forged mutable object. Re-expressed accordingly.

New design (all expectations are DATA in `P🧪️testing/🦀️cargo/🏃️execution/🧪️tests/🧫️fixtures/🔣️.json`, which now carries `custody`, `reason`, `settlement`, `policyMilliseconds`, `progress` per control row and `reason` per refused command row):

* Command delegation (10 rows): positive `original` asserts the resolved plan outside any `try`; every negative row asserts it threw an `Error` whose message contains the recorded reason (`Original delegated Cargo command selection differs` x6, `... policy differs` x2, `Explicit Cargo test policy required`).
* Control rows (19 + 1 test). `custody: "issued"` rows run in a real Original (`withScriptProcessEnvelope(createScriptProcessEnvelope(...))` + `withScriptInvocationSignalV1`, one fresh process birth per row, so a refused row cannot poison another). The fake port asserts command, arguments, cwd, budget, input and the carried wire (policy, capabilities, deadline) on every positive path.
  * positive: `original`, `original-progress` (port drives `onProgress` through `retain` + `advanceScriptInvocation`).
  * refused with asserted reason, port never reached: `foreign-manifest`, `foreign-action` (now a *valid* exact-laws request, so it hits the "differs" check, not the schema), `foreign-package` (`Cargo execution request differs from original selected command`), `malformed-request` (`Cargo execution request refused`, the old foreign-action shape), `missing-port` (`Cargo execution backend refused`), `missing-transport` (new, `Original Cargo execution transport required`), `cancel-before` (abort reason), `deadline-exhausted` (real 40 ms process deadline, `Original Script process deadline exhausted`, and the Original itself settles rejected with that reason).
  * refused after the port ran (commands = 1): `cancel-during-execution` (abort reason, acknowledged through `acknowledgeScriptInvocationSignalStopV1`), `request-during-execution`, `request-during-progress` (`Original Cargo execution authority changed`).
  * `custody: "forged"` rows (structurally valid but never issued, mutable): `forged-original` is refused with `Live Original process issuance required` (the reason the old suite hit everywhere), and the six `forged-*-after-publication` rows prove the retention guard fires before the issuance check with `Original Cargo execution authority changed` (`Original Script deadline refused` for the clock row).
* New test `issued invocations cannot be replaced by the caller`: invocation, control, policy, capabilities, backend and its argument array are frozen; replacing the policy / signal or pushing an argument throws `TypeError`.
* Negative control: temporarily removing the final `retain()` from `executeCargoRequestV1` (restored byte-identically from a backup, `git diff` empty) turned `cancel-during-execution` and `request-during-execution` red; the other rows stayed green.

## Row 34: what was wrong, and the new design

Evidence: the Core consumers run model ports that call `expect(...)` inside issued operations. Modeled failures (`throw primary`, `throw undefined`, release failure) and the substitution mutant are failures of issued operations, so the Original rejects at settlement (`Original.settle` rethrows `owner.failures[0]`), which left the callback and failed the file with `error: undefined` (the `undefined` outcome row) or the matcher error (mutant row), regardless of the `failed` capture inside.

New design (`P📦️artifacts/🏗️native-build/📥️inputs/🧪️tests/📥️consumers/{🟦️.ts,🔣️.json,🧬️schema.json}`):

* The Original settlement is captured *outside* the callback and judged after settlement (`judge`): `success` must resolve; every failing outcome must have failed with exactly the modeled primary (or release failure for `cleanup`), identity-compared, and any other thrown value (a model or matcher mismatch) is rethrown unchanged so it is reported. Per-row `settlement` (`settles` / `rejects`) is corpus DATA and schema-validated; measured: all failing rows reject with the identical error object except `wasm-cleanup`, where the wasm path returns the release failure to its caller without registering it as an operation failure (recorded as `settles`; noted below as an asymmetry).
* The substitution mutant is no longer `replaceAll` with a vacuous `expect`: the model port `exact()` throws a typed `ConsumptionRefusal(port, received, expected)` when arguments differ from the consumed view. Each declared substitution site (`substitutions` in the corpus: `tests`#0, `lint`#0, `wasm`#0, all port `lease`) is mutated individually; the law asserts the site count equals the declared count, the positive twin of the same row passes first, the refusal is that class at the declared port with `received == ["unconsumed"]`, `expected` non-empty and different, and the Original settles rejected with that same error. Verified non-vacuous by editing the corpus (wrong port, wrong settlement): both fail with the asserted values.

## Durable bounded journal (`cache/🧹️retirement`)

All defects are in the law; no production or storage change. Evidence by instrumenting `Promise.allSettled` of the nine fixtures and running the mapping child by hand (`SEMIO_MAPPING_*` env, one row at a time):

1. Fixtures 0, 2 (and the cascade 1,3-7): `Original process admission closed` only. This is the poison of fixture 8's rejected issued operation; once fixture 8 is green all nine are green.
2. Fixture 8 `proveMappingTransactions`, row `mapping-upgrade-refused` (child process): the child expected the refusal `Parent mapping SH cannot upgrade while its journal is retained`, but the call used the shared-read `binding.control` with the enrollment of `withAcceptedPhysicalCleanupV1`; production checks `requireAcceptedPhysicalCleanupDescendantV1` first and threw `Actual accepted physical body descendant required`, so `upgradeRefused` was false (right outcome, wrong reason). The callback now takes the cleanup control (`async(enrollment,cleanup)=>acquireNativeStorageMappingWriterV1({...binding,control:cleanup},enrollment)`), exactly as the exclusive branch does, so the refusal is the upgrade rule.
3. Born-red slice in `proveConcurrentProducers`: `JSON.parse(line.slice(22))` for the 21 character prefix `"[DATA] producer-wire "` (JSON parse error, masked until fixture 8 passed). Now `line.slice("[DATA] producer-wire ".length)`.
4. Intermittent `Expected: 0 Received: null` (second child status null, about 1 run in 5 under fleet load, reproduced and instrumented): `withOwnedCommandChannelV1` terminates the child tree when the body returns (`channel.close()` kills a child that has not closed), and the law returned right after reading the child's `finished` line, racing the child's own exit. Both conversation fixtures (mapping and concurrent producers) now drain the child's stdout to end (`exit(channel)`, bounded to 32 reads) after `finished`, before the body returns. 8 consecutive green full-file runs afterwards.
5. The test timeout was a hard 30 s; the test takes about 20 s alone and exceeded 30 s at load average 49 (observed as a timeout failure). Raised to 120 s, the budget class of the file's heavy fixtures.

## Files changed

Laws and fixtures (all under `P`):
* `🧪️testing/🦀️cargo/🏃️execution/🧪️tests/🟦️.ts`, `.../🧪️tests/🧫️fixtures/🔣️.json`
* `📦️artifacts/🏗️native-build/📥️inputs/🧪️tests/📥️consumers/🟦️.ts`, `.../📥️consumers/🔣️.json`, `.../📥️consumers/🧬️schema.json`
* `📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts` (four anchored edits in lines 36, 170, 202, 204)

No production file was changed (the single production probe was reverted byte-identically). Temporary `[DEBUG]` edits were removed (4 pre-existing `[DEBUG]` lines in the retirement law belong to earlier work and are untouched). Output under `C/🗑️generated/{law3-out,ta,law5}` deleted.

## Still open

1. `wasm-cleanup` row: the wasm builder returns a release failure to its caller but does not register it on the Original (`tests-cleanup`, `lint-cleanup` do). Pinned as `settles` in the corpus; if the asymmetry is unintended, flip the row to `rejects` after fixing `buildWasmWebV1`.
2. `P📦️artifacts/💾️cache/🧪️tests/🟦️.ts` also uses `withOwnedCommandChannelV1`; it may have the same child-exit race (not run here).
3. 21 hdiutil images leaked by other runs remain mounted under the shared scratchpad `ta/` (`ex-*`, `case-candidates-*`); not mine, not detached.
4. The delegation test still creates its `delegation-*` temp directory under `SEMIO_TEST_ARTIFACT_DIR` without removing it (pre-existing).
