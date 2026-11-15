# show projects: "Bounded canonical UTF-8 auxiliary DATA required"

Root cause: the owned auxiliary channel bound (24 MiB, `owned-execution/auxiliary/schema Policy.maximumBytes = 25165824`) was smaller than the bound of the data the build ships through it. `executeCapturedBuildSourceV1` and `runAdoptedCapturedBuildSourceV1` (`build/source/closure/command`) ship the captured source-closure snapshot JSON (base64 file bodies) as auxiliary DATA and admit up to `maximumSnapshotBytes = 33554432` (32 MiB; also the `StreamTransport` descriptor-3 bound). Any snapshot between 24 and 32 MiB passed the caller's check and was refused at `auxiliary/🟦️.ts:9`. The closure of the `show projects` / `tools setup` entries grew past 24 MiB; the `repo run` entry apparently stayed below.

Fix (schema first): auxiliary `maximumBytes` / `Case.count` bound raised to 33554432 to equal the snapshot bound (the 24 MiB value was the wrong bound, not the caller's data). Fixture boundaries moved (32 MiB exact accepted, +1 refused, 8388609 emoji refused). New pin test in the closure command tests asserts auxiliary max >= both snapshot bounds.

## Verification

- `repo show projects --lang=en`: passed `build:source-admission` (no auxiliary DATA error). The full run was stopped on purpose by the coordinator at owners 235 of 4628 (memory pressure); it did not complete, so `repo doctor --only=graph` was not run and the graph generation is still stale.
- `repo tools exec --light bun -- test` of the auxiliary tests: 2 pass, 9 fail. The 8 "actual owned auxiliary pipe" tests and one unnamed hook fail with "Original parent Script process invocation required" (they need an issued Original, which `tools exec` does not provide), so they are not exercised. The UTF-8 boundary test against the 32 MiB bound passed.
- Closure command tests, new pin test ("owned auxiliary channel carries every snapshot the closure admits"): 1 pass, 0 fail.
