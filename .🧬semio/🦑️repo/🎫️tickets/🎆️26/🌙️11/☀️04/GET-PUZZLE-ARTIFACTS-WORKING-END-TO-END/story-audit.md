# Storybook Story Identity Audit

Read-only inspection of installed Storybook CSF and authored metadata.

## Authoritative ID Behavior

Installed `node_modules/storybook/dist/_browser-chunks/chunk-NULBREPZ.js` defines sanitize using a punctuation blacklist rather than an ASCII whitelist. It preserves emoji and variation selectors. `toId(title, name)` sanitizes the title and name separately, joined with `--`. The installed core-server index builder (`dist/core-server/index.js`, around line 16885) produces IDs from `input.__id ?? toId(input.metaId ?? title, storyNameFromExport(input.exportName))`; entry display name is `input.name ?? storyNameFromExport(input.exportName)`. Therefore title/name lookup is appropriate for current stories, whose export names are automatically display formatted and have no custom name/id override in the relevant metadata.

`.storybook/🧪️tests/🧪️catalog-browser/🟦️.ts` already reads `index.json`, filters entry type/title, and navigates by actual entry key. Reuse that principle. Assert exactly one matching story entry for the authored title and display name, then URL-encode its actual ID. This avoids reproducing third-party ID rules and honors explicit Storybook ID overrides.

## Authored Title and Display Name Pairs

| Title | Display Name | Expected Current ID |
| --- | --- | --- |
| 🧩️puzzle🩻️2d | Overview Select | 🧩️puzzle🩻️2d--overview-select |
| 🧩️puzzle🩻️2d/Fixtures | Nakagin Capsule Tower | 🧩️puzzle🩻️2d-fixtures--nakagin-capsule-tower |
| 🧩️puzzle🧊️3d | Concrete Forest | 🧩️puzzle🧊️3d--concrete-forest |
| 🧩️puzzle🕐️5d | Nakagin Capsule Tower | 🧩️puzzle🕐️5d--nakagin-capsule-tower |
| ♾️infinite/WasmCanvas | Mock Session | ♾️infinite-wasmcanvas--mock-session |
| ♾️infinite/WorldR3f | Chunked Field | ♾️infinite-worldr3f--chunked-field |
| ♾️infinite/ReferenceMedia | All Fixtures | ♾️infinite-referencemedia--all-fixtures |

2d board additionally exports LassoSelect, BrushUtility, BrushPlacement and ForcedLodPane, with display names Lasso Select, Brush Utility, Brush Placement and Forced Lod Pane. Both fixture/3d stories export ConcreteForest and NakaginCapsuleTower. 5d adds CapsuleDream. Their titles and exports reside under `✏️s/🧑‍💻dev/🧩️puzzle/📖️stories`. Infinite fixtures reside under `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📖️stories`.

## Existing Test Faults

The 2d test uses `puzzle-2d--...`, which disagrees with title `🧩️puzzle🩻️2d` and cannot identify these authored stories. Spatial puzzle IDs retain emoji and appear consistent with installed sanitizer. Spatial infinite IDs are stale: WasmCanvas title is `♾️infinite/WasmCanvas`, not a GraphWasmCanvas title; the actual separator remains a hyphen in every infinite story prefix, e.g. `♾️infinite-worldr3f--chunked-field`, while the checked-in spatial test currently uses this latter correct value for WorldR3f and ReferenceMedia. Its WasmCanvas ID includes stale `graphwasmcanvas` and is wrong.

No running or built Storybook index was present in `.storybook` during inspection. Expected IDs above derive from installed implementation and current authored source; runtime index lookup remains definitive.

## Review of New Portable Story Index Fixture

Reviewed `🧪️tests/📚️storybook-index/🧪️tests/🟦️.ts` and `🧪️tests/🧫️fixtures/📚️storybook-index/🔣️.json` while implementation was not yet present.

The test import path correctly resolves to the shared tests fixture directory. Cases cover emoji IDs, explicit ID override, ignoring docs entries, missing matches, rejecting ambiguity, and distinguishing same display name under another title. The lodash-es filter supplies the requested third-party comparison without introducing runtime dependencies in the helper.

TypeScript JSON inference may infer the entries field as a union of differently keyed object shapes with optional absent keys, making `row.entries` incompatible with `Readonly<Record<string, StoryIndexEntry>>` and causing possibly-undefined entries in the lodash predicate. Give the imported fixture an explicit owned fixture type once; keep the helper index contract strict rather than admitting undefined entries to accommodate inference. A fixture type can use a cases array of records containing name/title/story, entries as the owned record, and expected as string or null. This is a type-inference concern to confirm with the relevant TypeScript check; Vitest transpilation alone does not prove the types pass.

The helper should expose only owned structural types and return the actual record key after exactly one matching story entry. Error messages should identify title, story name and match count for browser test diagnostics. Existing fixture cases suffice for the behavior change; an additional wrong-name/same-title candidate would independently catch accidental omission of the name predicate, since currently the missing-name case has an empty index. The existing same-name/other-title case already catches omission of the title predicate. A docs-only case would catch accidentally ignoring type filtering independently of the explicit-id case, although that current case already catches the common missing-type-filter regression by producing ambiguity.

Browser suites should obtain the index through Playwright request, assert HTTP success, resolve one entry with the pure helper, then URL-encode its key. The actual index is authoritative; avoid re-implementing Storybook sanitization.

## Final Source Review

Reviewed implemented pure helper, typed six-case fixture tests, normal Vitest inclusion, and both browser suites. Relative imports and fixture paths resolve correctly; each browser title/display name matches authored metadata. Both suites assert index HTTP success and URL-encode actual index keys. The helper correctly rejects zero/multiple exact story matches and includes coordinates/count in errors. Explicit fixture case typing addresses heterogeneous JSON inference. The additional other-story/same-title refusal case closes the previously identified predicate coverage gap. No source-level behavior defect found in this change. Existing browser checks still require runtime execution for renderer and interaction validation; this review performed no builds or browser runs. Root package supplies lodash-es as existing test infrastructure.

Type follow-up: installed lodash-es has no types/typings field and node_modules/@types/lodash-es is absent. Root already supplies lodash and @types/lodash; using its filter in the test avoids a likely strict TypeScript TS7016 declaration error without adding dependencies.

## Keyboard and Readiness Review

The readiness gate now requires both parsed WASM snapshot and exactly two engine nodes before interacting with each board story, matching all current board fixtures. Scoped keyboard wrapper refuses editable targets, composition, consumed events and modified Delete; canvas pointer-down capture acquires focus without scroll. Neutral keyboard corpus and its new browser journey exercise select-all, preservation of input editing, and board deletion. One precise remaining test defect: the older Ctrl/Cmd+A test only moves the mouse over the canvas, so it never focuses the wrapper that now owns keyboard events; it must click the canvas or explicitly focus the interactive board before issuing the chord. The new keyboard-scoping test already clicks correctly.

Timeline story historyColumnsFromParts reverses assembly order (latest first); last checkpoint is therefore oldest. Scoped `.semio-graph-timeline-host` button selection matches the current semantic renderer and avoids the removed table test-id. No additional source-level event/focus/selector defects established in this review. Runtime browser validation remains necessary.

Recheck after parent clarification: latest Ctrl/Cmd+A test uses page.mouse.click at canvas center before issuing the chord. The previously reported mouse-move focus issue does not apply to current source; no remaining precise issue identified.

## Bounded Browser Collector and Root Owner Review

The browser runner now scans only its dedicated suites directory and imports an explicit 13-suite registry. All 13 import paths resolve to existing source files. Registry includes catalog, visual layout, presentation deck, CAD renderer, renderer hosts without/with WASM, plugin registry, both Puzzle suites, s end-to-end, styling, new UI stories, and uncovered UI components. The shared Playwright port reexports the same test/expect/config/devices identity and Page/Locator owned import surface, avoiding test-registration identity divergence when CLI executes under Bun. This review did not identify a dropped intended suite.

The root test owner forwards arguments after storybook directly into Playwright; build and static serve happen first, readiness waits for a successful index.json response, and Playwright receives its configured base URL and tool cache. The static server is killed through finally. The Bun CLI invocation uses process.execPath with x --bun playwright, consistent with the repository scripts Bun runtime assumption; invoking this implementation directly under Node would not work, but repository routing owns the runtime. Narrow collector testMatch and testDir prevent broad workspace traversal while imported suites remain registered. Actual parent discovery reportedly confirms 142 tests across 13 source files; this review verified source/path structure without starting another build.
