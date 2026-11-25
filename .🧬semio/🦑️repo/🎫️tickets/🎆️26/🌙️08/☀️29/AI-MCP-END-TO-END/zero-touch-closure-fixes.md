# Zero-touch closure fixes

Test: `📥️source/🛗️zero-touch/🧪️tests/🟦️.ts` (`repo test-file`): 34 pass, 3 fail before; 35 pass, 2 fail after.

## Failures and root causes
1. Setup script without closure: the micromamba setup script of `🧰️tool-setup` (`🎓️teaching/.../🐍️python/🧱️native/🟦️.ts`) had no `full` closure in the roster. Added closure `micromamba-setup` (only node builtins reached).
2. and 3. (real checkout and command laws): about 870 findings.
   - Workspace specifiers (`@semio-tech/...`) in the publication module and its setup, funding, intent, journal, contract and bindings modules, and in os dev activation, plugin store, local hub, renderer, vite artifacts, assets and kernel modules. Rewritten to the relative path of the export target (`workspaceSpecifierTargetV1`) with a script (`🗑️generated/zero-touch/rewrite.ts`), run to a fixed point.
   - Seam-static: the project farm worker service imports the post-install inference seam statically, so it is itself post-install. Declared as roster seam.
   - External packages (vite, playwright, graphql, ajv, typescript) in `native-repo-script`: all reached only through the root playground, which `dev playground` loads post-install inside the opened dependency store. Declared `🛠️dev/🎮️playground/🟦️.ts` as roster seam.

## Open
Eight workspace specifiers remain in `🌉️mcp/🟦️.ts` (`@semio-tech/repo-lib/artifacts`, `@semio-tech/framework-plugin/model/reading`) and `🌉️mcp/📡️stdio/📦️component-models/🟦️.ts` (schema-validator-ts, repo-lib artifacts/storage, process/routing, framework-plugin model contract/reading). The mcp owners must apply the same relative rewrite; `rewrite.ts` does it when its `🌉️mcp/` skip is removed.

## Other tests
Publication tests: 19 files pass; 3 fail for environment reasons unrelated to imports (case-sensitive filesystem, setup-all purpose, pending-publication source scan). Local hub session test: one source-text assertion on `viteRuntimeEnvironmentV1(runtimeChannels)` fails, not an import matter.
