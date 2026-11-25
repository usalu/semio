# Node-kind trigger tests

Test: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🧭️node-kind-trigger/🟦️.ts` (project `@semio-tech/plugin-registry`, a `bun:test` file).

## Result
`bun test ./🧪️tests/🧭️node-kind-trigger/🟦️.ts` in the registry project: 40 pass, 0 fail (before: 18 pass, 22 fail).
`repo test @semio-tech/plugin-registry` returned exit 0 with empty output, so it is not a usable confirmation here. `repo run @semio-tech/plugin-registry:generate` never completed; the graph service did not answer within 120 s while the host was saturated.

## Root causes
1. Stale generated catalog. The tests read `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json` (git-ignored, dated Nov 22). Every `flow-extension-*` entry had `activationEvents: []`, although each extension `Cargo.toml` declares `activates-on = ["on-node-kind:<ns>"]`. With no deferred extensions `documentExtensionOwners` returned `[]` and 21 tests failed downstream.
2. Why it stayed stale: the `generate` target had been failing for a while, so no refresh ever succeeded.
   - The generator input closure (`registryCatalogInputPaths`) walks every import from `✏️s/🧑‍💻dev/🧩️catalog/📣️publication/📜️script.ts`. Every non-relative import must have a `workspaceImports` binding in the `plugin-registry` contract of `taxonomy.json`.
   - The playground session started to import the repository cache-store barrel, which pulled the storage, enforcement and bootstrap modules into the closure. No bindings existed for them, so `repo:generator-inputs` threw "not schema-owned". Nothing in `test` gated on that.
   - Separately, a workspace-bound `.json` entry was visited as an implementation module and rejected ("implementation language is not supported").
3. Test wiring gap: `test` depends on `generate`, but `generate` has `cache: false` and no declared outputs, so `dependentTasksOutputFiles` is empty. `test` is cached and reads `✏️s/.../🤖️generated/🔌️plugins.json` outside the project, which was in none of its inputs. A regenerated or stale catalog therefore never changed the test's cache key.
4. One stale source-text assertion. The React scope now has `handoff: async () => { ... await onAction(buildActiveExampleAction(...)) }` (notices and refusal handling), so the literal `handoff: () => onAction(...)` no longer matched. The assertion was rewritten to the same intent (handoff inside `openExampleAfterActivation`, the single `onAction(buildActiveExampleAction(` after `handoff:`), not weakened.

## Fixes
- Added 21 schema-owned (29 total now) `workspaceImports` bindings (ui-layout, process/* storage, leases, execution, routing, framework-filesystem, nx-bootstrap-tools retention/sdk) to the `plugin-registry` contract. The closure now resolves (149174 input paths). Keys in that block are sorted.
- Generator: workspace-bound `.json` entries are visited as `static-import` (JSON data), not as an implementation entry.
- `test` target: added `{workspaceRoot}/✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/**/*` to its inputs so the catalog content keys the test cache.
- Regenerated the catalog by calling the same `renderCatalogFiles` and `publishCatalogFilesV1` the `generate` script uses, with `specificCatalogPublicationV1` (`direct-generate.ts`), because the graph service was unreachable. Result: 74 plugins, with the extensions' `on-node-kind:*` events present.
- Test: updated the handoff assertion (see 4).

## Not done
- `repo run ...:generate` end to end and `repo test @semio-tech/plugin-registry` were not confirmed green (see Result).
- Declared Cargo.toml inputs for `generate`: the closure input list comes from `scanRepo` (manifests are included there). I did not change that.

## Files changed
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧲️registry-catalog-projection/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🧭️node-kind-trigger/🟦️.ts`
- Regenerated (git-ignored): `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/*`

## Ticket scripts kept
`closure-probe.ts`, `closure-fix.ts` (iterates missing bindings), `direct-generate.ts`. Logs under `🗑️generated` deleted.
