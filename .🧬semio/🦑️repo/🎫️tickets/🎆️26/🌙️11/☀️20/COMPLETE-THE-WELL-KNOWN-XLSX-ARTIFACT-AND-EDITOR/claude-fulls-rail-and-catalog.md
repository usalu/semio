# Full S rail row and required catalog root

## Task 1: no `openArtifactFile` rail row in the full S host

Cause: not a reduced-session or `s` dev-contribution defect. `openArtifactFile` is declared per editor app (`mount_natural_file_actions` in `framework/.../natural-file-and-media-artifacts`, called from the plugin builder for every app with a natural-file codec; the stdio-office component declares it on the three XLSX editors and on no viewer). The rail lists the actions of the focused window's app (`resolveWindowActions`), so with only Space/Home focused there is no Open File row. The standalone pane boots the editor directly, so its rail has the row. The shell's own `openNaturalFile` needs no focused editor (it resolves the owner through the format catalog), which is why the OS drop entry already worked; the repo's artifact-acceptance browser gate handles the picker the same way (`pickNatural`: spawn the owner editor from the command palette when the rail lacks the row).

Decision: the contract (Open File belongs to the owner editor, asserted by `natural-file-lifecycle` and the lifecycle fixtures) is intentional, so the journey driver was wrong for a document-less S shell, not the host.

Fix (driver): `claude-fulls-journey.ts` `openViaPicker` now spawns the dialect's XLSX editor through the command palette (`spawn.<plugin>.<app>` from the catalog probe) when the rail has no row, clicks Open File there, and closes the bootstrap window after the opened file settles (otherwise its empty A1 input would shadow the opened workbook's). Refusal cases use the transitional editor. The drop entry is unchanged.

Test: `claude-fulls-rail.test.ts` (2 pass): every XLSX editor declares paired Open and Save, no XLSX viewer declares Open; the driver bootstraps an editor when the rail has no row.

Not run: the live journey against 6070 (needs the rebuilt wasm components and the host; out of scope here).

## Task 2: required catalog root

Decision: split the two behaviours instead of changing the neutral reader. `readGeneratedCatalogProjection()` keeps returning the empty neutral catalog when `SEMIO_PLUGIN_CATALOG` is unset (the publication test's `catalogRoot: null` case and the build plan's playground read need it). Operations that cannot work without a catalog now refuse.

- `catalog-view/🟦️.ts`: new `requiredRuntimeCatalogProjectionRootV1()` (throws an English plus German error naming `SEMIO_PLUGIN_CATALOG`) and `readRequiredCatalogProjectionV1()`; `runtimeCatalogBuildInventoryV1()` (deployment inventory for size, driver stages, distribution, parity, activation module directories) now uses it.
- Activation: `♻️activation/🔍️freshness` reads the required projection; playground `session` script reads the required projection.
- Unchanged: build plan, publication refresh, explicit-directory callers.

Test: `🔌️plugin/📇️registry/🧪️tests/📖️generated-projection/🟦️.ts` gained 3 tests (9 pass total): unset root refuses on all three entry points with both languages; catalog-free reader still returns the empty catalog; a supplied root reads identically.

## Files

- `.../COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/claude-fulls-journey.ts` (driver)
- `.../claude-fulls-rail.test.ts` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🔍️freshness/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/📖️generated-projection/🟦️.ts`

## Caller regression run (strict catalog root)

Command: one `repo test-file` over 15 files covering the callers made strict (size/driver stages/distribution/parity/activation/freshness/session) and their neighbours, `SEMIO_PLUGIN_CATALOG` unset in the shell. Result: 10 passed, 5 failed, 76 s. No failure mentions `SEMIO_PLUGIN_CATALOG`, so nothing in the runnable set relied on the silent empty catalog.

Real flow supplies the root already: the dev package script takes `--catalog <root>` and sets `SEMIO_PLUGIN_CATALOG`; the distribution script already refused without it; the dev `config` test lane (`🎚️config`) injects it into the suites whenever the catalog exists. No flow needed a change and no silence was re-added.

| File | Result | Cause |
| --- | --- | --- |
| registry composition (43), parity-verdict (15), inventory refusal (1), stage-keys (2), generated-projection (9) | pass | none |
| registry discovery | 12 pass, 1 fail ("source deployment names remain available...") | unrelated: bundles generated TypeScript from the absent `registry/🤖️generated/🧩️plugins` and cannot resolve its `deployment` import; explicit roots, no env |
| dev staging-root | 56 pass, 1 fail | unrelated: named test inputs lack `🏗️builder/🌐️vite/📥️entry/🟦️.ts` (another agent's in-flight vite entry) |
| stage-remote | 5 pass, 1 fail | unrelated: `comp.d.ts` hash differs from the local run |
| hot-update, dev `🧹️config`, `🎚️config`, source-freshness, playground default, vendor-transport, routes-and-bridge, production-browser-artifacts | 0 tests ran | these use the vitest API (`vi.mock`, `it.runIf`) and are not owned by a `test-file` suite, so bare bun cannot run them (hot-update and `🧹️config` fail at load for that reason, before and after my change); the project `test` target runs them with the catalog env |

Not exercised: vendor-transport reads `runtimeCatalogBuildInventoryV1()` at line 397 and now needs the catalog root; its lane supplies it, but no generated catalog (`🎠️playgrounds.json`) exists on this host to run it against. A run through the project `test` target once the catalog is generated is the remaining confirmation.
