# WFC TypeScript Test Audit

The artifact package test scripts explicitly select the following suites:

| Target | Suites |
| --- | --- |
| `@semio-tech/wfc-bitmap:test` | SQLite snapshot, text snapshot |
| `@semio-tech/wfc-grid2d:test` | SQLite snapshot, schema |
| `@semio-tech/wfc-2d:test` | SQLite snapshot, schema, four examples |
| `@semio-tech/wfc-grid3d:test` | SQLite snapshot |
| `@semio-tech/wfc-3d:test` | SQLite snapshot |
| `@semio-tech/wfc-js:test` | Globbed schema suites and example suites across the WFC artifacts |

The shared target's schema glob finds bitmap, grid2d, wfc2d and grid3d suites; no wfc3d schema suite exists at that path. Its example glob includes wfc2d and wfc3d examples. Neither shared nor artifact target scripts explicitly include the bitmap and wfc3d mount-contract tests. These are coverage observations, not claims that the editor UI or solve behavior is tested.

All six targets were started with `SEMIO_OFFLOAD=never`; the five artifact targets run sequentially. At audit time the shared run waited for project-graph host capacity and the bitmap run waited for admission. See `[DEBUG] wfc*-typescript-test.log` in `🗑️generated` for current results.

Bitmap target exited 1 before executing tests with `Internal error: Canonical build task identity required (reference 8af55145)`. The message originates at `🏗️build/🕸️graph/🚪️admission/🟦️.ts:222`, where a `run` invocation is rejected if its second argument is not a colon-delimited task identity. The log does not expose the actual arguments passed into that function, so the cause is not yet established. The other targets are still in progress.

## Results via ticket's native-step envelope

With `SEMIO_BUILD_MODE=prototype SEMIO_OFFLOAD=never`, shared `wfc-js` passed 183 tests across 11 files; grid2d passed 27 tests across two files; wfc2d passed 117 tests across six files. Bitmap, grid3d and wfc3d each failed their pre-test TypeScript check, before suite execution, with missing generated manifest/actor modules and related implicit-any diagnostics; see respective `[DEBUG] wfc-*-native2-typescript.log` files. Native-step bypasses graph prerequisites, so missing generated modules may reflect the bypass and should not alone be attributed to the package. A direct run of those four explicitly listed test files is queued through `repo tools exec bun` to distinguish suite outcomes from the check.

The graph-driven bitmap command failed with the task-identity error above. The other graph runs were stopped while queued and verified closed before starting the envelope-based runs. The shared graph run was stopped while checking the file inventory, verified closed, and replaced by the passing native-step shared run.

Correction: the graph-driven bitmap command's malformed task identity came from the audit loop: zsh parsed `$variant:test` as a parameter modifier. Use `${variant}:test`; the earlier result does not demonstrate a repository defect.

The direct suite run returned 37 pass and one failure across four files. Grid3d SQLite passed 5/5, wfc3d SQLite passed 3/3, and bitmap text passed; bitmap SQLite failed the independent public-package subprocess assertion, which reports child exit 1 without its stderr. See `[DEBUG] wfc-direct-suites.log`. A separate probe is pending.

The bitmap package's export map points at `dist/🟦️.js`, but `dist` is absent. The TS artifact provider builds and runs suites inside `withCompilation`, and calls `bindTypeScriptPackageArtifactV1` only after its callback returns; therefore the self-reference test may run before the new output is bound. This is a strong candidate explanation for the failure, pending the subprocess stderr probe and a full package rerun.

Binding concurrency: the package-binding lease precedes artifact/scratch leases in `🏃️process/🔒️leases/🧭️order/🔣️policy.json`. Moving binding into the current live scratch callback would invert the required order. A fix needs build/publish, close scratch, bind, then acquire a shared binding plus artifact reader before suite work; public `dist` consumers additionally need the active mode to match, since the current reader permits a different active mode. No provider source edits were made. A separate stderr probe was cancelled after waiting over four minutes for host capacity and its log was verified closed. Generated manifest and actor files remained empty at last check, so the three package reruns are not yet actionable.

### Verified after regenerating the bindings and correcting package publication

All five package test targets finished with exit code 0 using the ticket's native package script and the prototype provider. Bitmap passed 30 (8 SQLite and 22 text), grid2d passed 27, wfc2d passed 117, grid3d passed 5 and wfc3d passed 3. The bitmap public-export test ran in its own Bun process and passed. The separate shared WFC package passed 183. Per-editor logs are `[DEBUG] final-ts-bitmap.log`, `[DEBUG] provider-grid2d-test.log`, `[DEBUG] provider-wfc2d-final.log`, `[DEBUG] final-ts-grid3d.log` and `[DEBUG] final-ts-3d.log`.

The actor, manifest and UI contract exports were regenerated by their Rust source exporters. The shared provider now imports the bound entry by file URL and uses a forward-slash module specifier in the generated TypeScript declaration probe.

The independent physical binding test now passes with 13 assertions across a mode switch and lease retirement. All five editor package suites passed again with the final active-mode retry behavior (`[DEBUG] ts-crossmode-*.log`).

The wfc3d mount-contract and example-outcome Bun suites passed 12 additional tests with 45 assertions. The four bitmap mount-contract TypeScript adapter scenarios also passed when invoked directly against the shared fixture and the IDs from its feature file. The logs are `[DEBUG] wfc3d-typescript-additional.log` and `[DEBUG] bitmap-typescript-mount.log`.
