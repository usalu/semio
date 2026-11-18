# Graph slice loop on a missing generated module

## Symptom
Every `repo run` stalled in "Reading the project graph slice": the slice worker ended with exit code 75 and `Cannot find module '...📇️registry\🤖️generated\🧩️plugins\🟦️.ts'`, restarted, and the inventory began again at 0 of 600, for about 2 hours.

## Root cause
1. Importer: `.storybook/🧪️tests/🧪️browser-runner/📚️suites/🟦️.ts:8` imports the spec `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/📚️storybook-plugins/🟦️.ts`, whose line 10 imports `../../🤖️generated/🧩️plugins/🟦️.ts`. That is the neutral registry's gitignored output, which only `@semio-tech/plugin-registry:generate` writes. No other non-test file imports the path (the s-dev catalog has its own copy under `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated`).
2. Taxonomy gap: commit 7af1259dba83 moved the generator contract `plugin-registry` to the s-dev catalog output root. The framework registry output stayed without an owner contract, so the graph walk could not resolve that import to a producer (`readBuildGeneratedCompileOutputV1` in `🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs`) and fell through to the physical file, which does not exist on a fresh replica.
3. Endless retry: `🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs` line 106 reset the same-cause counter (`sliceRestartSameCauseMaximum` = 3) whenever the worker ran at least `sliceRestartProgressMilliseconds` (60 s), and the churn clock reset too. A worker that dies after a 4 minute inventory on the same missing module therefore counted 1 every time and was never stopped.

Evidence it was the file: someone ran generate at 06:45:09 (the directory now exists) and the inventory then passed 600 s without a restart.

## Fix
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: new owned contract `framework-plugin-registry` (target `@semio-tech/plugin-registry:generate`, preview and check targets, ignored output root `.../📇️registry/🤖️generated`). Graph walks now resolve imports of that output to its producer and never read the file. `validateGeneratorContractsAgainstWorkspace` reports 0 problems.
- `.../🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs`: the same-cause count now ignores how long a worker ran (`sameCause` and `lastCause` follow the cause alone), so three identical deaths stop the plan with `slice-worker-failing` and the cause text, however long each run was.
- `.../🏡️resident/🧪️tests/🗣️restart-cause/🟦️.ts`: new test with `progressMilliseconds: 0` (every run counts as long); the file passes 6 of 6 (needs `--timeout 60000` on this loaded machine).

## Not verified / left
- The exact runtime frame that executes the spec's import inside the worker was not captured (the worker's stderr keeps only the cause line). The generated-output resolution itself was not run end to end through a fresh graph read; the machine's graph read is slow.
- Other specs/tests that import `🤖️generated` of an undeclared output root would behave the same; the new contract covers the neutral registry only.

## Follow-up: "Generated task cannot consume its own output" (wasm, NodeGraph)
- Cause: the `wasm` target of `@semio-tech/framework-os-node-graph-rs` has no `inputs`, so it uses `default` = `{projectRoot}/**/*`. The package directory holds React entry wrappers (`🟦️.ts`, `🟦️.tsx`, `🧭️wasm-graph-surface/🟦️.tsx`); their `@semio-tech/framework`/`framework-os` imports are followed by the compile-phase closure (`owner-discovery/🟨️.mjs`, role `compile`, because the target declares no `sourceInputObservation`). That walk reaches `🧑‍💻dev/🪝️host-ports/🕸️wasm-bindings/🟦️.ts`, which imports the wasm bindings this very target produces, so `generated()` refuses it (line 219, `producer.target===json.name+":"+name`).
- A Rust to wasm compile has no TypeScript compile inputs; those files are freshness inputs only. Fix: `"sourceInputObservation": "freshness"` in the NodeGraph `wasm` target's `metadata.semio` (same mechanism as `actor` `typegen`). `buildInputSourceRoleV1` now yields `freshness` instead of `compile` for it (probed directly).
- Siblings checked: the other wasm package owners (editor, actor, surface, flow, host, lsp, session, cad) hold only their `📜️script.ts` or command scripts (host) as TypeScript, so their compile walk cannot reach consumers of their own output; not changed.
- Not verified: a full graph read (my own BFS of imports did not reproduce the chain because Bun's resolver differs from the graph's workspace resolver; the exact intermediate file was not named).
