# Graph loader cap: declared memory cap for every process that reads the graph

## Finding about the plan-stage process (answer to the coordinator)
- The plan-stage guard (`createPlanStageGuardV1`, used by `discoverBuildGraphPartV1` and the prefetch) measures the managed bytes (`heapUsed + external`) of the process that runs the inference. For `repo run` that process is the slice worker forked by the resident graph service (`repo service host run`), not the front-door loader. "Reading the project graph slice" is the client waiting for it.
- That worker already carried `BUN_JSC_forceRAMSize=536870912` (declared `childHeapMegabytes`). Process-environment samples of the rerun (`🗑️generated/cap-samples.txt`) show worker pid 32402 with `BUN_JSC_forceRAMSize=536870912 MIMALLOC_PURGE_DELAY=0` and its service parent with `1073741824` (declared `serviceHeapMegabytes`). So the 8290 MB refusal of the earlier capped run was not caused by a missing cap: the cap bounds how much freed memory stays resident, not the live managed growth the guard counts. The earlier run was most likely refused because the guard counts managed bytes, which the cap does not change (see `graph-read-memory.md`).

## Changes
- `🧠️memory/🧬️schema/🔣️.json` (+ `🧠️memory/🟨️.d.mts`, `🧠️memory/🧪️tests/🔣️.json`): new declared policy value `heapEnvironment` (`BUN_JSC_forceRAMSize`), next to `childHeapMegabytes`.
- `🧠️memory/🐏️child-environment/🟨️.mjs` (+ `.d.mts`): `graphChildEnvironmentV1()` returns the heap cap (`childHeapMegabytes` in bytes) and the allocator purge of the host policy (`memoryPurge`); dependency free so the sealed front door can load it.
- `🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs`: the worker environment now uses `graphChildEnvironmentV1()` instead of its own `BUN_JSC_forceRAMSize` literal (adds the declared purge delay too).
- `⌨️command/🧭️selection/🧬️schema/🔣️.json`, `🔣️.json`, `🟦️.ts`: declared `graph` policy (`kinds` runtime and task, `freeVerbs` tools, reset, remote-execution, leases, doctor, capacity, fleet, scheduler) and `repositoryCommandReadsGraphV1`.
- `🏛️sealed/🚦️trampoline/🟦️.ts`: the loader (`bun --eval`) is spawned with `graphChildEnvironmentV1()` only when `repositoryCommandReadsGraphV1` holds; help, preflight, cache, cargo, dependencies, utility and light door commands are not capped.
- Tests: `🏛️sealed/🧪️tests/🚦️trampoline/🟦️.ts` (new test: spawned door environment carries the cap for run/show/exec only, with an SQLite model of the gate; fixture door `🧫️fixtures/🟦️.ts` prints the two variables for `<verb> env`), `🧠️memory/🧪️tests/🟦️.ts` (child environment equals the declared policy).

## Test results (bun through `repo tools exec`)
- sealed trampoline: 9 pass, 0 fail.
- memory: 28 pass, 1 fail (`process tree reader ...`, a `ps` based test, not touched by this change).
- resident host: 35 pass, 1 fail (`a service started by a client ... declared collector target` only waits 5 s for a spawned process to write a file and times out on this saturated host; the service-side env code is unchanged).
- command kernels selection test: pass. Two other tests of that file fail with "Closed authorizer relative source path required" (environment, unrelated).

## Activation rerun (`repo run @semio-tech/framework-os-dev:activate-shooting-react-prototype --lang=en --output=stream`, no timeout, no manual env)
- Log: `🗑️generated/activate-cap-rerun.txt` (the last line `exit=N` is written when it ends).
- Result: exit code 1 after 2131 s (no exit 75 from the plan ceiling any more; the earlier run was refused at 222 s).
- Plan stage inference finished in 2094 s: managed 4053 MB (3852 MB more than at its start), resident 6114 MB, under the 6553 MB ceiling.
- Peak resident memory sampled: 6080 MB (slice worker pid 32402, cap `BUN_JSC_forceRAMSize=536870912 MIMALLOC_PURGE_DELAY=0` present).
- Why exit 1: after inference the graph service refused one project, `@semio-tech/s-forms-composition-tests` ("needs 2221 MB but one project is granted 1756 MB here and 0 MB are free after 75 s", `BUILD_GRAPH_MEMORY_REFUSED`, reference 20c9039d, service log `host-6505bcf747a1.log`), surfaced as "The resident service met an unexpected failure". This is a per-project admission refusal on a saturated host (rustc jobs ran in parallel), not the plan ceiling. Activation did not complete; a rerun on a warm generation or a less loaded host is the next step.
