# Browser Staging and Declaration Boundary Audit

Read-only Source inspection on 2026-10-16. No build, test, compiler, Native, SDK, browser, network, lease or maintenance operation was executed. Only this report was written. The frozen lane records strict prototype12/release11 at 94 explicit roots and Source prototype5/release6 success; those are prior recorded results, not executions by this audit. Physical runtime remains HOLD.

## Concrete Finding

**P2: Shell birth can escape its descendant phase before close/join is installed.** In Hub Script lines 12266–12280, `spawn` occurs before the second `checkResourceLeaseControl(control)` used by initial timer setup (12276). If the Original deadline crosses between the initial check and that check, timer setup throws after child birth, before the `close` promise/listener exists. There is no enclosing `finally` that kills and joins this child. The startup catch (12281) only awaits the already rejected lifetime and stops the relay; it never kills `daemon`. The phase can therefore retire while the actual Shell survives. This is a Source control-flow defect; no runtime reproduction was attempted. Install the terminal observer immediately after birth and cover all subsequent setup in a kill-and-join cleanup scope.

## Traced Boundaries

- The only staging consumer is Hub `proveGisMapTwoAuthorShellProcess` (12438 onward). It returns `withTestBrowserHostV1` and passes its genuine invocation and issued control. Both Shell starts consume returned `browserPhase`; normal peer shutdown awaits the actual `lifetime` before relay shutdown. The outer browser callback awaits peer/browser/Hub cleanup before returning.
- Staging checks the supplied control/invocation relationship, carries dependency/store phases into scratch, uses sink control and environment for both jco calls, and keeps the capacity lease in try/finally. Space bytes come from `withComponentModelV1`, with bounded reads and model recheck, rather than a guessed compiler output. The final opaque destination publication completes before `withRepositoryArtifactV1`/`withRepositoryArtifactTaskV1` holds the reader around the entire browser consumer. Source hashes are rechecked before staging, before publication and after successful consumer completion. Callback failure propagates; post-success rechecks do not execute on failure.
- Shell output counts aggregate stdout/stderr bytes and lines against the actual authority transport limits, terminates on excess, and keeps a 65,536-character diagnostic tail. A boolean failure flag preserves a thrown undefined reason. Normal terminal handling clears the timer, joins `progressWork`, removes abort handling, and only then settles its descendant phase. The birth/setup hole above remains.
- Geometry admits only objects exposing all eleven current methods, including all four demand methods; it refuses stale generated bindings rather than adapting the two-argument retirement API. Actual checked-in generated runtime regeneration and retirement behavior remain explicitly unqualified. Method existence admission does not itself prove Rust behavior.
- Browser Flow declarations reexport the actual owned generated ABI; host declarations describe the adjacent authored receiver. No ambient runtime stub, old-runtime fallback or new authority factory is introduced by these declarations. Actual Wasm execution is unqualified.
- The moved descriptor observation implementation is `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🟦️.ts`. Current Hub, actor, provenance, Native/Wasm artifact and filesystem-current consumers import the Core owner directly. Its bounded chunk/hash/stat checks consume supplied observation callbacks. The move does not issue an Original or renew a clock.

## Frozen Twelve-File Comparison

| Path | Current SHA256 | Frozen Match |
| --- | --- | --- |
| `tsconfig.json` | `9314568d174cb801648d1e3558c2b24aff7d020d0248e320298c626fa1f1bffa` | yes |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | `71de4d1c83bf23c4d13ddb69ab24123441a5fc7ed3d83077dd54e93dfc00455f` | yes |
| `✏️s/🧑‍💻dev/🧩️catalog/🌐️browser-host/🏗️staging/🟦️.ts` | `b530e652b3dc047ce6c55eb8ef37111bea1fa6f9215cc9f15f7ff53fad3d010c` | yes |
| `✏️s/🧑‍💻dev/🧩️catalog/🌐️browser-host/🟦️.ts` | `14a4541dba9096c72035541d3c9b1bc2adab6dd18a591148b37e1b414ca903e5` | yes |
| `✏️s/🧑‍💻dev/🧩️catalog/🌐️browser-host/🧪️tests/🟦️.ts` | `9c9e486cd1351a03fbe58464cd2f5feb867ccdf379770b3d8dcc68a141c614cd` | yes |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟦️typescript/📚️libraries/🟦️.ts` | `183f8169a48eecbbd8af1bf38339222bac350529b4554a9542098c198b998d63` | yes |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟦️typescript/📚️libraries/🧬️schema/🔣️.json` | `468932a29729089d4d506098c3f3d8725364aff00973d0d2c01ebd2c4690a125` | yes |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟦️typescript/📚️libraries/🧪️tests/🟦️.ts` | `d9208998781e8eb34aab36faf0280031e5943dbb816c61138fb7faa509d8a5e1` | yes |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` | `f384457d246599fae982880b29b5243056d66b1968e224678c7f0b67f1e153f4` | yes |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📥️checked-inputs/🟦️.ts` | `79e6168521478fb55617f20969c7dca96c37128cb740feab93864e7569be4095` | yes |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` | `4b29aab6fab919a6ddd91aeeb81f2491bd773971ebc5b671d531536c8749c72d` | yes |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/sdk-derivative/📜️script.ts` | `05c3d7dad7701017b87c471f33178260d872214aef305dc51ebc70e9e1664b7a` | yes |

Compared 12 frozen entries. No whole-workspace immutable snapshot is claimed.

## Additional Inspected Source Hashes

| Path | SHA256 |
| --- | --- |
| `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🟦️.ts` | `82d77f58766ac9768dfa5693282fb2c7a0b1e7bebd696cf0f5ff415301495972` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏃️runtime/🟨️.d.ts` | `aba418d72e07a1cfe78bff41f13c7e261a30c5ff65b9fb7f3f954bb54683d7d7` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🖥️host/🏃️runtime/🟨️.d.ts` | `a25ea1f28c83bf8a762d6cdfcee89074302e7de49495d329721ad06c475ca15f` |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🟦️.ts` | `a596f25cdba8c0a311b2120d6b70aa519ef31d6159ee192d6b3389c0c93bec70` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts` | `c16b44825775aad06081c3ec33ee541c90c0f887f28aa3d2aef3f22225da8460` |
