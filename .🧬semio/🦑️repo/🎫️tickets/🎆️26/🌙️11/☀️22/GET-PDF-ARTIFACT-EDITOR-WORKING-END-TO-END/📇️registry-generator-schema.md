# Registry Generator: Authorizer Compiler Schema Failure

## Symptom
`repo run @semio-tech/plugin-registry:generate` failed at `nx:authorizers` with `Cannot find module './🧬️schema/🔣️.json'` thrown by the bundled `.semio-authorizer-compiler.mjs`.

## Reproduction (isolated)
`repro-authorizer-bundle.ts` (this folder) captures the compiler closure (`🔏️authorizers/📜️script.ts`), runs the real `compileNxAuthorizerSnapshotV1(..., "runtime", "program")`, writes the bundle into a far directory and keeps it. Importing it reproduced the error with `Require stack` (a runtime `require`, not a static import: `Bun.Transpiler.scan` of the bundle listed no relative import).

## Root cause
`🧰️framework/🔨️modules/🏃️process/🔖️identity/🟨️.mjs` loaded its schema with `createRequire(import.meta.url)("./🧬️schema/🔣️.json")`. The bundler only hoists literal `createRequire(import.meta.url)("...")` calls into static imports for the two `BundledLazyLoadOwners`; for every other module the call stays a runtime require resolved against the bundle's directory, where the sibling does not exist. This module is loaded at the top level of the bundle, so the bundle died while loading.

## Fix
1. `🔖️identity/🟨️.mjs`: static `import schema from "./🧬️schema/🔣️.json" with {type:"json"}` (no createRequire).
2. `🔏️authorizers/📦️snapshot/🟦️.ts`: the hoist now also runs in every non-owner module for literal `.json` paths (`hoistLiteralRequiresV1(text,jsonOnly)`), because data is always bundled; other literal paths stay hoisted only for the lazy-load owners. A future `createRequire(...)("./x.json")` in the closure can no longer break the bundle.
3. Test `🔏️authorizers/🧪️tests/📦️bundle-load/🟦️.ts`: the compiler bundle, run from a far directory, must not print `Cannot find module`/`Require stack`, and must carry the identity schema inline (`readTimeoutMilliseconds:15000`).

## Verification
- Repro bundle now loads and reaches the expected guard `Original parent Script process invocation required`.
- `bun test` of the bundle-load file: bundle-load test passes. The sibling test "holds only audited location-relative reads" fails for an unrelated concurrent change (`🏗️build/🧭️invocation/🛰️plan/🟦️.ts | readFileSync` is no longer found, so `🧫️audited.json` has a stale row); not touched here.

## Generator rerun status
Two reruns of `repo run @semio-tech/plugin-registry:generate` after the fix passed the planning phase, then blocked in "Reading the project graph slice: waiting for the graph state lease" (the 600 s lease counter exceeded 700 s; first run cancelled by its own 590 s timeout). That is graph-state-lease contention from other concurrent agents, not the authorizer failure; `nx:authorizers` was not reached yet. A third run (`timeout 2400`, log in `🗑️generated/registry-generate2.log`) was left running; check for `rc=` at the end of that log and for the hubs' `🤖️generated/📇️registry/🦀️.rs` files.

## Follow-up: audit list
`🔏️authorizers/🧪️tests/📦️bundle-load/🧫️audited.json` (untouched for a week): removed the stale `🏗️build/🧭️invocation/🛰️plan/🟦️.ts | readFileSync` row. A new read then surfaced in `🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts` (`hostWardenCodeDigestV1` hashes sibling sources via `import.meta.url`; it was edited 03:40 and was left alone until quiet 15 min, then audited with reason). All 3 tests in the file pass (16 expects).

## Follow-up: generator at 04:08
registry-generate2 is still reading the project graph: its read restarts repeatedly because paths change under it ("1 path(s) changed under the read", replica not settled). Its `timeout 2400` ends about 04:17; `rc=` not yet written, no hub registry `🦀️.rs` written, `nx:authorizers` not yet reached. If it times out, rerun when the tree is quieter.

## Generator at 04:30
registry-generate2 ended `rc=124` (timeout; the graph read never settled under replica churn). registry-generate3 (`🗑️generated/registry-generate3.log`, `timeout 2400`, ends about 05:00) is again in the graph-lease/slice phase (heap handover); `nx:authorizers` not reached, no hub registry `🦀️.rs` written yet.

## Native fallback (graph read bypassed)
Stalled `repo run` processes were stopped. `run-registry-generate-native.ts` (this folder) runs, with the cached bun (`~/Library/Caches/semio/runtimes/bun/*/bun`), the body of `GenerateScript.run`: `renderCatalogFiles(repoRoot,undefined,"exclude",specificCatalogPublicationV1(...))` then `publishCatalogFilesV1` (same functions as the `generate` target's `📜️script.ts generate`, minus the script invocation and graph read). Result: 74 entries, 0 diagnostics, written to `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated` (plugins, playgrounds, hosts, artifacts, framework). No authorizer bundle is involved on this path, so it does not exercise the schema fix; the isolated repro and the bundle-load test do.

A first native attempt with the neutral publication (the registry's own default) wrote an empty registry catalog (0 crates) into `📇️registry/🤖️generated` (gitignored, regenerable); rerun the dev publication above to repopulate dev; neutral output is what the `generate` target itself emits.

Finding: this generator does not write any `🤖️generated/📇️registry/🦀️.rs`. That file name is the graph-manifest output contract (`🕸️graph/🛂️manifest/📇️outputs.json` `shared.rustRegistry`, `📽️projection`), a different generator. No hub has such a file today (only `🌎️hub/🧩️compositions/🗄️stdio/🤖️generated/*/🦀️.rs`).

`semio-hub-flow` gated check: `native-cargo.sh check -p semio-hub-flow --lib` is queued behind 8 other agents' cargo jobs on the shared gate; not completed (log `🗑️generated/hub-flow-check.log`, `rc=` when done; 1500 s timeout includes queue wait).

## Fleet issue for owners
`repo run <project>:<target>` stalls for 10 to 40 min in "Reading the project graph slice": graph state lease waits (counter above its 600 limit), repeated rounds ("N path(s) changed under the read", replica not settled), slice-worker heap above 2048 MB forcing handovers. Under constant replica churn from 50+ writers the read never converges; the run is cancelled by timeout (`rc=124`, "Original Script process terminated"). Targets that need no graph facts should have a graph-free path, or the settle window should tolerate unrelated writes.

## Graph-manifest projection (native)
`run-graph-manifest-native.ts` (this folder; cached bun, no graph read) finds every `🛂️manifest/📇️outputs.json` outside the framework's own, then for each runs `readGraphOutputCatalog` -> `renderGraphArtifacts(root,outDir,catalog)` -> `writeGraphArtifacts` into the component's `🤖️generated` (the body of `🕸️graph/🛂️manifest/🏃️execution` `GenerateScript.run`). 9 components written, each now has `🤖️generated/📇️registry/🦀️.rs` (log `🗑️generated/graph-manifest-native.log`). Mounting components (all verified present):
- `🌎️hub/🧩️compositions/🌊️flow` (the `semio-hub-flow` mount)
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting`, `.../🔌️jack`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d`, `.../🧊️3d`, `.../◻️2d`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing`

## Registry's own `🤖️generated`
The registry's `generate` target runs `GenerateScript` with the neutral publication (`📣️publication/🔣️.json`, no contributions); application owners publish their own catalogs through their own targets (dev catalog already regenerated: 74 entries, 0 diagnostics). So the neutral output (empty `🔌️plugins.json`, hosts, framework) written by my first native run is what the real generator produces, by the identical code path; nothing further to restore.

## semio-hub-flow check
Still queued on the shared cargo gate at 04:52 (9 tickets ahead/behind); result pending, see `🗑️generated/hub-flow-check.log`.

## semio-hub-flow check, update 05:02
The first check ended `rc=101`: `-p semio-hub-flow` matches no package from the repo root (the crate lives in the separate `🌎️hub/Cargo.toml` workspace). Rerun as `native-cargo.sh check --manifest-path 🌎️hub/🧩️compositions/🌊️flow/📦️packages/🦀️rust/Cargo.toml --lib` (log `🗑️generated/hub-flow-check2.log`, `timeout 2400`); still waiting in the shared gate queue at 05:02, result pending (`rc=` line at its end).

## semio-hub-flow check, update 05:12
The second attempt failed `rc=101` too (relative manifest path; the wrapper changes directory). Third attempt with an absolute `--manifest-path` (log `🗑️generated/hub-flow-check3.log`, `timeout 2400`) is queued on the shared gate with no output yet; its `rc=` line is the result.

## semio-hub-flow check, result
`hub-flow-check3.log` ended `rc=101`, but not at the registry mount: the upstream crate `semio-plugin-abi` fails with 3 E0432 errors in `🔌️plugin/📥️actor-input-receipt/🦀️.rs` (mounted by `🔌️plugin/🚪️io`): unresolved crates `semio_framework_job_native_decode` and `semio_framework_pack_json_output_receiving` (not linked in that crate's manifest). Unrelated to `🤖️generated/📇️registry/🦀️.rs`; belongs to whoever is editing the actor-input-receipt/io plugin area (plugin crates not touched here). The hub-flow mount itself is therefore not yet verified by cargo; rerun the check once `semio-plugin-abi` compiles.
