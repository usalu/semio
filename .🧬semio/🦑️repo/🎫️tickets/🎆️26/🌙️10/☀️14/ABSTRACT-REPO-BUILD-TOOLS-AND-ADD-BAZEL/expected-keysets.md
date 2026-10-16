# Owned Expected Keysets

The 90-second SDK inspector recording identified `requireExactKeys` at 10.209349 seconds self weight and 18.965566 seconds inclusive weight. These are sampling weights, not an operating-system CPU/IO partition or an end-to-end speedup measurement.

## Contract and Implementation

`compileExactKeysV1` copies, sorts and freezes an explicitly supplied expected-key image. Each invocation still calls `Object.keys` on the current actual object and checks exact length plus membership. The expected length is retained, so duplicate expected strings remain rejecting. Generic `requireExactKeys` remains unchanged and observes mutable expected arrays on each call. Symbols, inherited and nonenumerable keys remain ignored; getters are not read by shape admission; proxy enumeration and thrown values remain observable every call.

Only the two schema-owned expected arrays for Script invocation and control records use the compiled plan. All policy, capabilities, callable ports, current signals, current remaining time and Original issuance checks remain in place. There is no actual-object, authority, time, Source or filesystem cache. Peer process cleanup implementation and schema were not modified.

## Actual Evidence

- Before production changes, the new actual-checker regression failed: two calls performed 8 sorts against the required 0. `expected-keysets-red.log`: terminal 1, 0 pass, 1 fail, 1 assertion, 403 ms.
- Focused final laws: terminal 0, 3 pass, 48 assertions, 898 ms. The language-neutral nine-case key corpus agrees with independent SQLite; schema-owned arrays are independently Ajv-validated. Tests cover detached expected-array mutation, duplicate keys, unicode/numeric/prototype key strings, getters, symbols, inherited/nonenumerable fields, proxy reenumeration and exact thrown undefined.
- Full routing file: terminal 0, 47 pass, 308 assertions, 3.37 seconds. This includes genuine Original handoff, actual late AbortSignal completion refusal, ordinary no-deadline controls, fractional remaining authority and independent SQLite agreement for private derived-signal acknowledgement.
- Full existing validator proof: terminal 0, 67 portable vectors, independent Ajv and esbuild-bundled actual Node agreement.

The gates ran under the existing ticket harness with a genuine standalone finite 60-second Original and no SDK use, storage image or quota change. All owned runtime scopes have closed. These results do not claim public SDK setup performance or acceptance.

## Permanent Registration

The three new tests are in `PROCESS/🧭️routing/🧪️tests/🟦️.ts`, already selected by `PROCESS/📜️script.ts test routing`, project target `@semio-tech/framework-process:test-routing`, and its existing launch row. No new permanent executable command was introduced. The generic validator proof remains registered by the existing schema-validator Rust package test router; its full pure proof was invoked directly by the ticket harness for this change.

## Source Inventory

- `🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts`: explicit pure snapshot compiler.
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🧬️schema/🔣️.json`: owned expected-key arrays.
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts`: exactly two compiled keyset callers.
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/🧪️tests/🟦️.ts`: actual-checker regression and semantic laws.
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🧪️tests/🧫️fixtures/🔑️keys.json`: language-neutral corpus.
- Ticket `capacity-fixtures/📜️script.ts`: focused/full routing and full pure validator selection, unchanged Original ceiling.

Production and test Source are held after these final gates. Root owns the expanded strict check, actual closure recapture and coordinated public SDK refresh.
