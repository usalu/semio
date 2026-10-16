# Private Expected-Key Membership

## Measured Input

The actual SDK diagnostic `19472` expired its unchanged 120-second Original. Its 90-second inspector recording is `🗑️generated/cpu-sdk/sdk-inspector-58633.cpuprofile`, 819,168 bytes, SHA256 `18a1bfe16dda4d8adadf118a424794c96f878166e29ac2c8dbaa1b008235842c`. The captured process was PID 58642. The sidecar records 90,002.849167 ms wall and 31.100665 seconds process CPU at capture; profiler cleanup ends at 90,487.932959 ms and 31.531422 seconds process CPU. This includes profiler work and excludes child-process CPU. The difference between wall and process CPU is not an exact IO-wait measurement.

The profile contains 2,366 nodes and 11,368 samples, with time deltas totaling 89,995,621 microseconds. The bounded existing summary harness completed in 0.716 seconds and independently checked self/inclusive aggregation using SQLite. Its output is `🗑️generated/sdk-current-cpu-whole.json`; fixed relative 0–30, 30–60 and 60–90-second windows are in `sdk-current-cpu-windows.json`. Window boundaries do not assert SDK phase alignment.

Relevant self weights are `lstatSync` 14.221 seconds, the compiled key-plan closure 13.076, `some` 5.740 and `includes` 2.943. The key-plan closure has 21.632 seconds inclusive weight. These are sampled elapsed weights, not frame CPU durations. Inclusive entries overlap. No multiplication by the OS CPU ratio was performed.

The completed SDK log records source capture at 22.688 seconds, derivative projection/clone at 61.054 seconds, 37 patches totaling 622,059 bytes, and source-index verification at 11.216 seconds. The diagnostic is not package-alias acceptance or a successful SDK setup. No clone/IO cause is inferred solely from the weights.

## Contract and Change

Only `compileExactKeysV1` changed in production. It retains the detached sorted/frozen expected array, expected length and exact error text. A separate Set is closure-private, never exposed and never mutated after compilation. Every call freshly enumerates `Object.keys(actual)` and uses a direct loop with Set membership. The generic mutable-expected `requireExactKeys` remains unchanged.

Expected duplicates still reject because the original length is checked independently of Set membership. Actual symbols, nonenumerable and inherited fields remain ignored; getters remain unread; proxies are enumerated on each call and preserve thrown values. No actual object, value, Original issuance, policy, signal, deadline, Source or filesystem fact is memoized. No authority, budget, quota or peer cleanup implementation changed.

## Actual RED and GREEN

The language-neutral existing key corpus adds an explicit work vector: two rounds over invocation/control keysets require four actual enumerations, zero array callbacks, zero array membership scans and fourteen direct membership checks. SQLite independently computes the enumeration and membership cardinalities from the schema-owned arrays.

The regression extracts and transpiles the actual production function AST into an isolated Node VM. Only that VM's method instrumentation counts the selected function. Host test, schema and Original prototypes are untouched. Genuinely issued Original/control inputs are authenticated by the actual receiver outside that instrumentation.

- RED `expected-key-membership-red.log`: terminal 1; 3 pass, 1 fail, 50 assertions. Actual enumeration 4, array callbacks 4, array membership 14, Set membership 0. The intended work assertion failed.
- Focused GREEN `60918`, `expected-key-membership-green.log`: terminal 0; 4 pass, 50 assertions, 681 ms. Actual counts 4/0/0/14 agree with SQLite.
- Full routing `83566`, `expected-key-membership-routing-whole.log`: terminal 0; 48 pass, 310 assertions, 3.34 seconds. This includes current-value changes, callable ports, genuine Original handoff, cancellation, late completion and immutable/mutable/proxy/getter/symbol/nonenumerable/duplicate/undefined-primary semantics.
- Full validator `26374`, `expected-key-membership-validator-whole.log`: terminal 0; all 67 portable vectors agree with independent Ajv and esbuild-bundled actual Node.

All gates used the existing standalone finite 60-second ticket owner, without SDK use, stores or image leases. Low independently reviewed the final Source and isolated instrumentation. These operation counts and law results do not establish end-to-end SDK speedup.

## Permanent Registration and Held Inventory

The new law is in the existing full process routing test entry, invoked by `PROCESS/📜️script.ts test routing`, project target `@semio-tech/framework-process:test-routing` and its existing launch route. No executable command was added. Root owns strict TypeScript and fresh actual Source closure capture after this hold.

Current held SHA256 values, recorded after the final gates:

| File | SHA256 |
| --- | --- |
| `🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts` | `77cb184b64bc63e0faf4ea382b64a3dcb2556e0f99c9470cf091d5578f9306fe` |
| `🧰️framework/🔨️modules/🏃️process/🧭️routing/🧪️tests/🟦️.ts` | `2ce80e5614d3c2b606420de2225c4fe6c1ff4a6f1843003246a01821d4fce018` |
| `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🧪️tests/🧫️fixtures/🔑️keys.json` | `4fb0ecb07c6287f9524443d5ed73eaea72ac17740c0cd42a7507316d2ebb55b7` |

These are post-change bytes, not reconstructed hashes of the earlier profiled Source. Production and test Source are held. No SDK retry was performed by this lane.

## Strict TypeScript Follow-Up

Root's actual expanded strict gate `76031` found TS2775 on the new test's dynamically imported assertion namespace. The test now gives `api` an explicit `typeof import("../📥️invocation/🏃️process/🟦️.ts")` annotation. The actual `requireIssuedScriptInvocationV1(original)` assertion remains intact. This is a test-only typing correction; no diagnostic suppression, production code or runtime authority changed.

Full routing rerun `41511`, `expected-key-membership-routing-typed.log`, completed terminal 0: 48 pass, 310 assertions, 2.28 seconds under the same genuine 60-second owner without SDK/store leases. The final test entry SHA256 is `989a7cd983ab775bbbbf50c0c3ea9b5137cd0d56e9f70d10e44c3bab475778b9`, superseding its earlier inventory hash above. Root owns the new expanded strict TypeScript result; this runtime pass is not labeled a compiler pass. Source is held again.
