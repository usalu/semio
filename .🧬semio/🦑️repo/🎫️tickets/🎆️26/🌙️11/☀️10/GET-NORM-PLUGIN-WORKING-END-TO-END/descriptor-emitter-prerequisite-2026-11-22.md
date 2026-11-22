# Descriptor Emitter as a Build Prerequisite

## Policy change
The freshness preflight refused every component build once any of the ~2055 emitter sources changed. Now a missing, stale or unusable
emitter is rebuilt and republished first, then the component build continues. The refusal is gone; the only error is a failed rebuild.

## Where it runs
`runRepositoryComponentDriverV1` (driver-stages) is the one owner of every component build (`buildComponentClosureV1`,
`buildReleasePipelineV1`, `describeComponentsV1`). Before it takes the dependencies store, compiler store or any emitter reader it awaits
`ensureFreshDescriptorEmitterV1` (emitter module), unless
- the plan itself builds the emitter as a job its describe stage waits for (`planBuildsEmitterV1`, the old `buildsEmitter` test, extracted), or
- the caller already holds the emitter reader (`emitterHeld:true`; only `produceFreshComponentV1`, which rebuilds the emitter itself and runs the
  closure inside `withDescriptorEmitterV1`, sets it).
`ensureDescriptorEmitterV1` (absent-only, `describeComponentsV1` only) is replaced by it (greenfield, no compat). The in-store preflight
`checkDescriptorEmitterCapabilitiesV1` keeps only the `capabilities` probe.

## Custody
`ensureFreshDescriptorEmitterV1` opens one dependencies store scope (for the lease directory), then:
1. `inspect`: absent files -> missing; else `withDescriptorEmitterV1` + `withDescriptorEmitterTaskV1` reads the compiler receipt and runs
   `staleDescriptorEmitterInputsV1`. The reader is released when that scope returns (unreadable publication = rebuild; cancellation propagates).
2. If not fresh: `serialize` takes the exclusive lease `tools:descriptor-emitter:<mode>:<platform>` (class tool-singleton, rank 2, the policy
   already allows tool-singleton -> toolchain), re-inspects (another build may have republished while we waited), and only then calls
   `buildDescriptorEmitterV1`, the very function `@semio-tech/os-plugin-describe-rs:build` runs: same source, same publication, same receipt.
   The writer is thus taken with no reader of ours held. Nested store/control use goes through `requestedCompilerAdmissionControlV1(actual,0)`.
3. The logic lives in the pure `🥾️prerequisite/🟦️.ts` (`ensureFreshDescriptorEmitterWithV1` with inspect/publish/serialize/report ports), so it is
   testable without the repo infrastructure. A failed rebuild becomes an English/German error naming `repo run ...:build` with the cause kept;
   aborts rethrow unwrapped. Reasons are reported bilingually via the driver's `log` (console by default).

## Concurrency and the live chain
Concurrent component builds: one republishes under the lease, the others wake, re-inspect, find it fresh and build nothing. The coordinator
chain that rebuilds the emitter explicitly before each component keeps working under either policy: after its own rebuild the emitter is fresh
(or at worst stale by a few minutes of edits, which now triggers one more serialized rebuild rather than a refusal). No cache, `modules` or
`tools` directory was touched and no component build was run.

## Tests (`repo test-file`, all green)
- `🥾️prerequisite/🧪️tests/🟦️.ts`: 10 pass. Fresh -> skipped, no lock; stale/missing/unusable -> republished in order inspect, lock, inspect,
  publish, unlock with bilingual reasons; waiter finds it republished -> builds nothing; 4 concurrent callers -> exactly one publication;
  failed rebuild -> bilingual error with target and cause; cancellation unwrapped; lease name classified by the repo's lease-order policy
  (rank 2, before artifact and compiler-writer ranks); independent oracle: hard-coded SHA-256("abc") known answer plus an SQLite
  comparison decide fresh/stale, compared with the real `staleDescriptorEmitterInputsV1`.
- `🌡️freshness/🧪️tests/🟦️.ts`: 4 pass (the refusal test and `requireFreshDescriptorEmitterV1` were removed).
- `🧪️tests/📏️request/🟦️.ts`: 6 pass; the absent-publication test now exercises `descriptorEmitterAbsentV1` over the same publication cases
  and asserts the driver awaits the prerequisite before its dependencies store, gated by `emitterHeld`/`planBuildsEmitterV1`.
- Module load check of the emitter and driver-stages through bun (temporary file removed).

## Not verified
No real rebuild through the new path was run (instructed not to disturb the chain). The nested dependencies-store + `withDescriptorEmitterV1`
+ `buildDescriptorEmitterV1` custody and the tools lease under a live lease registry are covered by design reading and structural tests only;
first live run: any component build after the emitter sources change. If the nested store refuses, the fallback is to run `inspect` and
`publish` as sibling scopes with the lease held by a standalone acquisition.

## Changed files
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🥾️prerequisite/🟦️.ts (new)
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🥾️prerequisite/🧪️tests/🟦️.ts (new)
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🌡️freshness/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🌡️freshness/🧪️tests/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🧪️tests/📏️request/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts
