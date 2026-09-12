# Router Fallback Runtime-Binding Probe

Root ran four minimal current source cases on 2026-09-12 while the scoped command-router correction was in progress. This is a checkpoint of two public classifiers, not a final audit of the executor's unfinished implementation.

| Case | Source | Command Disposition | Generic Source Role |
| --- | --- | --- | --- |
| Runtime delegate | import { execute } from "./dep.ts"; await execute(); | unresolved | thin-delegation |
| Type-only delegate | import type { execute } from "./dep.ts"; await execute(); | unresolved | thin-delegation |
| Runtime terminal | import { runBundleScriptMain } from "./dep.ts"; await runBundleScriptMain(); | tool-metadata | thin-delegation |
| Type-only terminal | import type { runBundleScriptMain } from "./dep.ts"; await runBundleScriptMain(); | unresolved | thin-delegation |

The generic classifier's evidence for all four was "top-level delegation to imported owners". The installed TypeScript compiler's TS1361 behavior for type-only runtime calls is independently established in 📓️terra-script-policy-audit-2026-09-12.md. Raw probe source/results are retained at generated/coordinator/router-fallback-binding-probe.json.

Discovery collectPackageRoles currently uses the specialized disposition only when it is tool-metadata; otherwise it falls back to classifyPackageSource and accepts allowed generic roles. Thus a now-correct rejection by the specialized command parser alone does not establish end-to-end runtime-binding enforcement for these bare imported-call forms. This is not a claim that the unfinished parser report is final.

The active parser lane must cover actual caller fallback behavior in its regression evidence. Preserve valid direct imported delegation and the many minimal runArtifactRustPackageMain routers from the 455-script scope. Reject the corresponding type-only forms through the actual package/discovery admission path. The forthcoming fixed-script normalization gate also needs this shared semantic result, rather than recreating permissive fallback logic.

Prefer one owned scope/value-binding authority and precise classifier integration. Do not fix this by admitting all import text, banning legitimate opaque imported results, or turning every unresolved specialized disposition into an automatic failure when generic delegation has actually been proven. Portable paired fixtures should distinguish valid runtime values from type-only syntax and exercise the ordinary discovery call path as well as the direct parser.
