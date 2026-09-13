# Test Selection And Source Consumer Repair

Status: bounded selector and source-consumer repair complete; current selection controls, registered admin tests and focused MCP unit tests pass. Remodel now reaches its full oracle and reports current protocol parity failures below. This does not finish conventional configuration filename ownership or claim a passing Remodel suite.

## Portable Red And Green Controls

The retained schema and language-neutral fixture are `📋️tool-config-controls/🧭️test-selection/🧬️schema/🔣️.json` and `📋️tool-config-controls/🧭️test-selection/🔣️.json`. Before product changes, installed Ajv validated the fixture, installed Vite7.3.6 loaded each actual config with its native loader under Bun, and installed fast-glob projected exact test/source/setup identities with symlink following disabled. All three rows failed: admin selected no component test and a nonexistent setup, MCP selected no direct test, and Remodel omitted its schema suite.

Current projection is green: admin selects two direct tests, its existing in-source i18n test and setup; MCP selects six direct suites and its existing in-source unit owner; Remodel selects four suites. The new admin command case had an additional observed red selection phase before registration. Final expected identities are explicit fixture outputs, not copies of include patterns. Raw before/after/final projections are ticket-generated `coordinator/tool-config-controls/test-selection/*.json`.

## Changes

Admin's test, coverage and setup selectors now resolve from its package directory to the existing admin concern. Its existing two command-routing tests moved from the jsdom component test to anonymous `🧪️tests/🧪️command-routing/🟦️.ts`, with native Vitest's per-file Node environment directive. This keeps the native repository command API out of the browser component import graph. The actual taxonomy resolves the new directory as the existing `test-case` under `tests`; its anonymous implementation leaf has no basename finding. No new global taxonomy exception was added.

The shared setup now installs DOM polyfills and component cleanup only when a DOM `Element` exists. The Node command case therefore has no browser setup dependency, while the actual component and in-source i18n cases retain their jsdom setup. An intermediate native run exposed the unguarded `Element` access before this final correction.

MCP's direct suite glob now resolves to the MCP concern. The exact imported-only `registerTests1` helper is excluded as a standalone suite; the existing `includeSource` owner still imports it and executes its eight unit cases. The actual inference-process command selector now names its current anonymous inference suite. No former named test alias is retained.

Remodel's schema include now reaches `🧬️schema/🧪️tests/🧩️suite/🟦️.ts`. That previously omitted oracle still read fixtures beside native test sources and an example asset at its former example-local path. Its discovery and byte-reader consumers now read `🧫️fixtures/🧬️mutations/<operation>/<case>` and the real subset-owned `🖼️assets/🎬️demo` asset. The fixture contract binds every vector to its existing native case and checks one fixture owner per normative mutation tag, plus unique identities and the existing full tag-coverage check. The former directory-count check incorrectly counted the binary representation's tests as a mutation operation. No mutation, snapshot, diff, expected outcome or asset byte was changed.

## Executed Runtime Evidence

- Native installed Vitest, selecting the actual MCP in-source owner through the corrected config: one file, eight tests passed,1.04seconds. This runs binary-path admission/unit fixtures only; it does not start MCP, build Rust, run the six process suites or validate the inference-process selector at runtime.
- First actual registered Remodel target reached all four selected files and failed five of13tests because of the stale fixture/asset consumers above; three example files passed. The subsequent run reached the restored vector tests and exposed two additional stale byte-reader paths, now rebased as well. Neither failed invocation is described as green.
- First actual registered admin target reached its component and i18n files. The three i18n tests passed; the component file failed import because its command test pulled `node:sqlite` into the browser transform. The Node command-case separation addresses that boundary; final registered verification is in progress.
- Final actual registered admin target: three files and18tests pass; Vitest18.39seconds, Nx20.7seconds, cache skipped. The Node command case, browser component case and in-source i18n case all execute in their intended environments.
- Final actual registered Remodel target: four files execute,1,023tests pass and330fail out of1,353; Vitest33.74seconds, Nx36.2seconds, cache skipped. The retained output has no missing-path/module errors. Its116 grouped assertion reports include an extra `locale:null` diff field and integer-versus-decimal byte formatting such as `0` versus `0.0`. These are current protocol value/serialization discrepancies exposed by restored coverage. No expected document, diff, locale value, number format or assertion was altered to conceal them, and no full Remodel pass is claimed.
- All eight current product files pass scoped whitespace checks. Installed Prettier accepts seven; its only proposed changes in the shared admin component file are three formatting differences outside the moved command-test/import hunks. Those unrelated hunks were inspected and left alone. No product/configuration build or browser process is claimed.

The actual registered targets are `os-hub-admin:test` and `@semio-tech/remodel-js:test`, run with `long --configLoader=native --no-cache --maxWorkers=2`, skipped Nx cache, disabled Nx daemon/plugin isolation, and per-target ticket-owned Nx workspace/cache, TMP and test-artifact directories. These are native-loader runs, not default bundled-loader acceptance. Their logs are under `🗑️generated/coordinator/tool-config-controls/test-selection/{admin,remodel}`.

## Exact Product Attribution

1. `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/vitest.config.ts`
2. `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🛡️admin/🟦️.tsx`
3. `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🧪️command-routing/🟦️.ts`
4. `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/vitest.config.ts`
5. `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📜️script.ts`
6. `✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/vitest.config.ts`
7. `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🧩️suite/🟦️.ts`
8. `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🧹️environment/🟦️.ts`

Attribution is limited to the described selectors, test move and fixture-reader consumers. All other shared changes remain with their authors. The two authored ticket controls and this report are retained; generated native outputs are disposable. No modifying Git, AGENTS edit, fixture fabrication, native dependency installation, server startup or ticket/goal lifecycle operation was used.

Terra's independent `📓️terra-test-selection-consumer-review-2026-09-13.md` accepts the inspected owner/consumer boundaries and separately counted134 fixture vectors across35 mutation tags, with zero missing native case files. Its read-only review is not a duplicate native run. All coordinator processes for this slice have finished.
