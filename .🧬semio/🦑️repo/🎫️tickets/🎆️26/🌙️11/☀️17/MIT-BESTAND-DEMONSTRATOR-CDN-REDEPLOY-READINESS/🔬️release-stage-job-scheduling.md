# Release Stage Job Scheduling

The native component driver started all jobs but waited for every job to finish before admitting the component passes. That postponed the WASI guest whenever the Darwin descriptor emitter needed a rebuild. The authored release plans already identify the stages that consume the emitter, native transpiler and browser support.

The pass now waits only for jobs without a declared stage consumer, profile-lane jobs and the transitive prerequisites of either group. The descriptor emitter, native transpiler and browser support can run alongside guest input capture and compilation under the existing broker, compiler backing and jobserver. The font producer chain, Stdio catalog preparation, canvas runtime assets and profile-lane modules retain their pre-Cargo barrier. Both release plans attach the native transpiler to `transpile`, where the binary is first read. Description waits on the emitter and browser assembly waits on support.

A failed required job stops new work, retires running children and blocks the dependent stages. Failure cleanup rejects stage completions with no producer after the lane owners have settled. An aggregate completion waits for all its members, and a finalizer waits for active prerequisite work to settle before removing shared stage files. An expected process exit is carried as a result across the child resource scope so the live Original can still retire its work before the driver rejects. Pre-Cargo failures retire the timeline and jobserver within the still-live capacity grant.

The runner fixture uses real driver scheduling, a funded native storage image, the actual Cargo planning route, a deterministic guest link child and an independent descriptor-tool process. The child waits for the guest marker with two slots, creating a causal overlap check. The failure vector leaves work running past the tool failure and verifies that discard follows that work's settlement. A one-token vector verifies serial completion without a deadlock. SQLite independently reconstructs the expected order from the neutral JSON fixture.

The first baseline invocation showed tool settlement before the pass began and then hit a fixture planning refusal. The shared stub fixture was inside the repository, where it inherited the production Rust compiler ingress. The fixture now explicitly uses the pinned ordinary Rust version probe for Cargo planning. Guest compilation remains the stub child and the release compiler configuration remains in effect in production.

The targeted runner verified six cases. The two-token overlap and failed tool vectors, one-token serial completion, reserved worker isolation, the declared lane barrier and fail-fast retirement of sibling jobs all passed. The full `native-component-jobs` suite passed 12 tests with 54 assertions. The transcripts are `🗑️generated/stage-job-targeted.log` and `🗑️generated/stage-job-suite-verified.log`.

Native executable refresh and browser acceptance are coordinated separately in the release queue.

Affected repository paths:

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/📋️contract/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/📋️prerequisites/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🎭️stub/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🏃️jobs/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🧫️fixtures/🔣️.json`
