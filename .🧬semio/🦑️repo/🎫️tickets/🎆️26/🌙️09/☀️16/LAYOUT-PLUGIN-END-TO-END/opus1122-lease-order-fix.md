# Descriptor emitter lease order fix

## Cause
`ensureFreshDescriptorEmitterWithV1` took the exclusive tool singleton `tools:descriptor-emitter:<mode>:<platform>` (rank 2) and then ran the whole emitter build under it. The build's cargo dependency fetch requests `cargo:registry-fetch:...` (rank 8, exclusive). Another build held that fetch for 176 s, so after the 15 s grace the enforced `holdAndWait` rule threw `SEMIO_LEASE_ORDER`.

## Fix
The singleton now serializes only the final publication. `buildDescriptorEmitterV1` takes a `publishing` wrapper (default passes through, so the `:build` target and playground are unchanged). Fetch and compile run before it, and the store publication runs inside it. `ensureFreshDescriptorEmitterWithV1` inspects, reports, builds, and then publishes under `serialize` after re-inspecting. A concurrent caller that finds the emitter fresh skips its publication and returns "fresh". The policy is unchanged. I tried a declared `tool-singleton` to `registry-fetch` edge first and discarded it, because it would have weakened the enforced rule.

## Files
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🟦️.ts
- .../🛠️emitter/🥾️prerequisite/🟦️.ts
- .../🛠️emitter/🥾️prerequisite/🧪️tests/🟦️.ts

## Tests
Run with `bun test` on the prerequisite tests file: 12 pass.
- The existing event-order tests now expect `inspect, report, build, lock, inspect, publish, unlock`.
- New: the registry fetch happens before the lock is taken.
- New: the enforced policy still refuses a held tool singleton waiting for the fetch lease (hold-and-wait) and refuses the reverse order.
- Not run: cargo builds, and the policy module's own tests (the order module's policy.json is modified by others).
