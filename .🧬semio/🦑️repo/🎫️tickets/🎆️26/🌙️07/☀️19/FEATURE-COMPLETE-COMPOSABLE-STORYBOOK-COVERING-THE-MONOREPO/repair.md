# Storybook Repair

Follow-up component spacing, connector and two-sided-grid repairs are complete; see `visual-repair.md` for their detailed acceptance and the refreshed live inspection server.

## Implemented

- Queried the npm registry's stable tags for Storybook, React/Vite, docs, Vitest and the lint plugin; all were **10.6.5**. Aligned all three declaring workspace manifests and the eight resolved Storybook packages to that exact version. Preserved unrelated lock resolutions/layout; `bun install --frozen-lockfile` succeeds without changes.
- Corrected canvas-renderer scope aliases/watch roots to the current source owner; removed the undeclared `golden-layout` optimizer inclusion.
- Added a shared generated-asset preparation function using the existing real styling/icon generators and freshness checks. Both direct dev/build routing and every canonical dev/build/test target prepare these prerequisites. No fabricated asset fallback or WASM substitute was added.
- Added catalog integrity tests for source aliases, story discovery, relative imports, version alignment, prerequisite edges, inactive host-WASM exclusion and cross-platform runtime chunk identity.
- Restored the missing Mode interaction helpers with assertions for four panes, maximize/restore and tab switching. Fixed Engagement's stale interaction expectation.
- Fixed the Tree drag/drop play function to arm its actual handle and use browser-native DragEvent/DataTransfer objects. Testing Library's convenience constructor clones DataTransfer and loses its payload between events; React Testing Library also requires unavailable production `React.act`. The browser story now uses the DOM test interface, verifies the actual published payload and confirms the target receives the item.
- Fixed Input's actual controlled/uncontrolled/defaultValue semantics rather than masking the Label story warning; controlled/lazy behavior and native form reset are covered by 18 focused tests against a React native-input oracle.
- Added an explicit empty favicon for isolated previews, preventing a default `/favicon.ico` 404 rather than filtering console errors.
- Removed the old change-detection disablement; current Storybook's normal behavior works in browser validation.
- Isolated Storybook's global-initializing runtime in its own bundle chunk. This fixes the production-only `__STORYBOOK_MODULE_CLIENT_LOGGER__ is not defined` failure that a successful build alone did not detect.
- UI-only builds now compile out unrelated framework-host WASM loaders/bindings. Framework scopes retain their real loaders; no test doubles or ignored WASM failures are substituted.
- Static builds copy only referenced shared assets through the existing asset interface instead of copying the entire asset source tree.
- Fixed cold dev startup: workspace exclusions hide transitive external ports from Vite's dependency scan. Explicitly prebundling the actual React/Three/drag/panel/PDF entrypoints prevents first-preview `504 Outdated Optimize Dep` failures. Workspace source still stays out of prebundling, avoiding unrelated unbuilt WASM. Removed the global `.ts`→TSX optimizer override, which misparses valid generic TypeScript. The dev probe now clears only its own optimizer cache before each run; failures are not retried or filtered.
- Added an accessible group label and expanded state to WindowMeasureTreeGroup using instance-unique label IDs. Its default story verifies the accessible button name, collapse/expand state, child visibility and retained input value.

## Component Stories

Added **17 explicit subjects and 45 states** for status surfaces, titlebar, tutorial UI, control/refusal hints, Aside, NotFound, Catalogue, Pane and WindowMeasuresTree. See `story-coverage.md` and `remaining-story-coverage.md` for exact subjects, API decisions and files. Pane uses the real PaneHost. Labels use existing locale machinery. A language-neutral fixture and both TypeScript/Storybook-CSF parsing guard the known subject set; this is not an exhaustive claim about every possible present/future export.

## Final Validation

| Check | Result |
| --- | --- |
| Catalog/scope/preparation unit suite | 28 passed |
| New subject contract/CSF suite | 19 passed, 254 assertions |
| Focused Input/native oracle suite | 18 passed |
| Cold-cache dev browser catalog, English desktop/default appearance | 240 UI/styling variants, zero failed |
| Actual static production bundling | 240 UI/styling variants built |
| Chromium against the built static artifact | 240 UI/styling variants, zero failed |
| Geometry, terminal connector and below-floor pixel guards | Pass in dev and static |
| Button autodocs, dev and static | Pass in both |
| Dependency lock frozen installation | Pass, no changes |
| Diff whitespace check | Pass |

Final dev and static browser runs both passed in the local validation environment with Chromium 156.0.8078.79. Temporary audit builds, browser artifacts and audit-server logs were removed, and audit servers terminated. The separate refreshed read-only inspection server remains intentionally running on6010; its current state and retained runtime files are documented in `visual-repair.md`.

The browser guard waits for each story's actual finished phase, runs play interactions, and rejects preview exceptions, console errors and failed resources. It checks the current index rather than a stale hand-maintained list. Hidden/no-cue stories are valid; a nonempty root alone is not considered success.

Canonical targets added: `prepare-storybook`, `test-storybook-subjects`, `test-storybook-catalog`. The existing `test-storybook` includes these checks and the catalog browser test is registered in the existing browser runner. An explicit Chromium executable path is supported without breaking Playwright's normal installed-browser selection elsewhere.

Reproducible dev/static probe scripts are retained as ticket inputs. They place outputs/logs under the ticket's generated folder, support cancellation and remove generated outputs after validation. The normal runtime entry remains `repo run workspace:dev-storybook-ui` after dependency-store setup.

## Limits and Infrastructure

- Full native MCP/canonical task publication did not finish within earlier bounded attempts. See `dependency-staging.md`: repeated zero-elapsed staging events indicate work checkpoints, **not proof of deadlock**. No speculative store reset or custody bypass was made. The installed `repo tools` runner executed all checks above.
- Full typecheck is not certified: existing missing generated manifest/UI-contract declarations cause unrelated diagnostics (exact paths in `input-default-value.md`). A broader configuration typecheck was cancelled after its bounded attempt; it is not reported as passing.
- Full OS/plugin/WASM catalog production builds and every locale/device/theme combination are not certified by the UI/styling runs. Those scopes still require real producer outputs; no fake fallbacks were introduced to make them appear healthy.
- Goals/ticket lifecycle operations remain unavailable without full native MCP; no ticket state was fabricated.

## External Evidence

Stable versions were checked from `https://registry.npmjs.org/<package>` dist-tags for the five declared Storybook toolchain packages. The upstream discussion of Storybook's global-initialization ordering is background evidence, not a claim that this Rollup build is the identical Rolldown defect: https://github.com/vitejs/rolldown-vite/issues/182 and https://github.com/vitejs/rolldown-vite/issues/562. The local runtime chunk decision is validated against the real built artifact.
