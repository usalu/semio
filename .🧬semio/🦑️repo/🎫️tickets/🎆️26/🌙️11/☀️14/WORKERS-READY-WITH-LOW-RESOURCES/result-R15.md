# Result R15: g1-m06 "Cannot find package '@semio-tech/pack-json-decode-ts'"

## Cause
`schema/source/closure/🟦️.ts` (reached by the build implementation that `repo` loads after source admission) imported two workspace packages by bare name, `@semio-tech/pack-json-decode-ts` and `@semio-tech/framework-pack-json-ts/members`. Those resolve only through the machine-local root `node_modules` link, which does not exist before `workspace:deps-js-all`.

## Scan (import closure of door script and CLI script, Bun scanner, static and dynamic)
1776 files; bare specifiers found: 4, in 4 files (all fixed):
- `schema/source/closure/🟦️.ts`: `@semio-tech/pack-json-decode-ts`, `@semio-tech/framework-pack-json-ts/members`
- `pack/json/decode/🟦️.ts` (second hop, found after the first fix): `@semio-tech/framework-pack-json-ts/members`
- `ui/styling/projection/🟦️.ts`: `@semio-tech/assets/resolver/delivery`
- `assets/icons/builder/projection/🟦️.ts`: `@semio-tech/framework/identity/grapheme`
All now relative imports of the package public entry file (the target `workspaceSpecifierTargetV1` computes); Bun dedups by real path, so there is still one implementation.

## Law
The law already exists: `build/input/source/zero-touch` (closure roster `🗂️closures/🔣️.json`, covers repo-cli, repo-door, dependencies-js/native, wasm-tools, root-setup, ... 27 closures). It was red since the closure file was added. After the fix the roster has no finding for repo-cli, repo-door, dependencies-*, root-*, wasm-tools, bootstrap closures.
Added to its test file: "the front door and the schema source closure load from a copy of their relative closure that has no node_modules anywhere" (copies the closure files, 1779 files / 17 MB, to a tmp dir, imports the schema closure and runs `door repo --help` with a child bun). Pass, 2.2 s.

## Proof without node_modules (scratch copy, no symlink)
- fixed copy: `import(closure/🟦️.ts)` loads, exports `schemaReferenceClosure`; `door repo --help` prints usage.
- same copy with the old import line: `error: Cannot find module '@semio-tech/pack-json-decode-ts'`.
Touched-module tests: schema closure 10 pass, pack json decode 5 pass, zero-touch 35 of 37 pass.

## Second line "The stamps moved again after the restart"
Consequence of the first, not independent. `format-stamps/🔄️reexec/🟨️.mjs` line 29: `staleImplementationOfV1` (`format-stamps/🟨️.mjs` line 102) classifies a resolve failure as a half-edited module load (`isHalfEditedModuleLoadV1`, line 99); the outermost command restarts (announcement "edited by another process", backoff), and after `maximumReexecutions` restarts line 29 prints the original message plus `final` and returns `protocol.exitCode`, which is 75 (the `tools_rc=75` in the log). So it is already bounded and typed; no code change. Caveat: a permanent missing package is retried the whole bound with sleeps before it ends (about a minute), by design.

## Not fixed / not verified
- Zero-touch tests still red for reasons outside this package: roster `native-repo-script` (35 findings: `commands/🛠️dev` imports `os/dev/tests/time-travel`, which imports bare `@semio-tech/framework-time-travel-ts`, `process/execution`, `repo-lib/process`); commands law red for `test`, `bench`, `dev` (test directories), and `components/publish/release` (`io/failure-owner` bare `value-ts/*`), `schema/verify` (`verification/wasm` bare `@semio-tech/schema-validator-ts`). None is on door, setup or JS-install path.
- `door/🧪️tests/🚦️startup`: "fleet-entry modules 110 > 100" red; not touched by this change (fleet-entry does not load these modules), not investigated.
- No real `repo` build command or worker run (no graph commands allowed); the real m06 rerun is unverified.
- Landed: edits were made on the live tree directly (anchored one-line edits); `land.py` verified all as same and ran tests under lock, recorded in `T/landings.md`. N/LAND-OK created and removed.
Script: `T/🧰️scripts/R15/land.py`.
