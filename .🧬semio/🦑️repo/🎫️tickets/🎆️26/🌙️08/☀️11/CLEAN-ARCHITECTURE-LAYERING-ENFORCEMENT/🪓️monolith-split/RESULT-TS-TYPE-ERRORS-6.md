# Result: TS-TYPE-ERRORS-6 (prefix `tt6`)

Executor: Sonnet, started 2026-10-17 20:10. Foreground, no sub-agents, no modifying git command, no ticket tool. Logs `🗑️generated/tt6-*`, tools `tt6-tools/`. Continues `RESULT-TS-TYPE-ERRORS-5.md`.

## 0. Verdict

**PARTIAL.** Rulings 1 and 3 are done, the renderer regression is explained (a transient race, no code defect), work orders (b) and (c) are done except 4 positions (mesh and bcf diff parsers), (a) is done except 5 positions and (d) except 3 tests, all of which need an owner seam.

| Measure | TT5 end (`tt5-w6`) | TT6 end (`tt6-w3`, 23:05, 14,082 roots) |
| --- | --- | --- |
| Whole-workspace diagnostics | 363 | **309** |
| Skipped by order (TS2307 / TS7016 / `mit-bestand`) | 266 | 265 (TS2307 102, TS7016 15, of which 160 `mit-bestand`) |
| Real | 97 | **44** (TT5 final effective 93) |
| Swapped `expect(actual).matcher(expected)` sites | 163 files swapped by TT5 | **0** (`tt6_swaps.mjs`: `swapped=0 matcherChanged=0`) |
| Generated-parser refusals (`schema parsers report`) | 1,295 | **888** (protobuf 446 -> 281, graphql 424 -> 182, parser 294, typescript 131) |
| `schema parsers check` | green | green (`pending=0 dangling=0 helperless=0 drift=0 orphans=0 unowned=186`) |
| Note document oracle mutation fixtures | 0 / 0 found | 66 / 33 found |

The 44 real diagnostics (list `tt6-tools/tt6-w3-real.txt`) are in section 8: 21 belong to peer slices in flight (14 capacity tests whose owner move has not landed, 7 crate-tree / testkit / unmirror from the peer carve), 13 are follow-ons of packages missing from `node_modules` or unbuilt wasm (`@xmldom/xmldom`, `rxjs`, `tree-kill`, `d3-regression`), 4 are the mesh / bcf diff parsers (work order b), 5 are custody tests that need an owner seam (work order a), 1 was fixed after the run (GIF 89a unit test).

## 1. Coordinator addition: whole-renderer tsc 7 -> 73

Cause: not a code defect, a race. The `c4pp` type-check `c4pp-tsc-after.txt` ended 19:44:58; the peer carve of `🧰️framework/🔨️modules/📇️directory/🧬️schema/🟦️.ts` created the 11 child folders (`🌊️stream`, `🎫️command`, `🎯️execution-target`, `👀️views`, `📂️document-open`, `📄️event-page`, `📋️admin-page`, `📜️event`, `🛠️admin`, `🧊️artifact-checkpoints`, `🪢️checkpoint-pair`) at 19:43:15 and their `🟦️.ts` at 19:43:26, so tsc loaded the importers (directory root, `🏘️spaces`, `🚪️io/📝️text`, `📞️directory-client`, `🛰️backbone`, the `📦️packages` directory facade) before the children existed (TS2307 x45 and the implicit-any TS7006 that follow from unresolved types). Re-run now (20:25, `tt6-ren.json`, the same tsconfig as `c4pp-tsconfig.json`): 4 diagnostics, all TS7016 (jsdom 2, lodash-es 1, graphlib 1 because that program does not include `🌿️ambient`), the 59 `📇️directory` rows and the 9 `SpaceAdministration` / `directory-client` / `catalog` rows are gone. An import-resolution scan over every relative import in the directory module also finds 0 unresolved paths.

## 2. Ruling 1: swapped matcher sites (restored, typed from the expected side)

Method (`tt6-tools/tt6_swaps.mjs`, TypeScript AST): every `expect(A).matcher(B)` call of the working tree is compared with the same file at HEAD (2026-10-16, before TT4/TT5). A call is a swap when HEAD has `expect(B).matcher(A)`, a matcher change when HEAD has the same A and B with another matcher (`toEqual` -> `toMatchObject`). Scanned: the 163 files TT5 listed plus every test file that differs from HEAD (2,141 files; 258 have no HEAD version because a split created them).

- Found 71 swapped sites in 45 files (all restored to the HEAD order by `--apply`) and 3 matcher changes. A second population TT5 hid behind spreads or helper variables (`expect(row.expected).toEqual([...actual])`, `expect(fixture.lanes).toEqual([...LANES])`) was found with a token-similarity pass and restored by hand (about 20 sites).
- The 16 mutual `expect(A).toMatchObject(B); expect(B).toMatchObject(A)` pairs (a weaker, direction-free stand-in for `toEqual`) in 13 files became `expect<typeof B>(A).toEqual(B)` again (`tt6-tools/tt6_pairs.py`).
- Typing instead of swapping: the 74 type errors the restored order produces were fixed from the expected side by `tt6-tools/tt6_widen.mjs`: `expect<typeof expected>(actual).toEqual(expected)` (the actual value must be assignable to the type of the expected value, which the compiler checks; a non-name expected expression is hoisted into a `const` before its statement). Where the actual type is not assignable (a JSON fixture type with `string` against a literal-union domain type, `never[]` empty arrays, `readonly` tuples, `null` rows from `bun:sqlite`), the second `bun:test` overload is used: `expect(actual).toEqual<typeof expected>(expected)`; no `unknown`, no `any`, no cast. Two sites got explicit element unions (`expect<(string | undefined)[]>`).
- Order restored in all of them; a failure now prints received as the produced value and expected as the fixture again.


## 3. Work order (c): note document oracle mutation-fixture discovery (done)

Cause: the oracle looked for the mutation fixtures under a folder name that the carve renamed. Fixes: `mutationRoots` in `✏️s/🔌️plugins/🗒️note/…/✳️any/🧬️schema/🧪️tests/🪪️document/🟦️.ts` now points at `🧪️tests/🧫️fixtures/🧬️mutations`; the text snapshot binder in `…/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts` binds a scalar field called `value` under its field name (`bind(member, scalarFields.has(field) && key === "value" ? field : key)`), and the contract fixture `…/🧪️tests/🧫️fixtures/🔬️contract/🪪️document/🔣️.json` has the three dense `validDiffs` the oracle now needs. Result `tt6_run_note.ts`: 66 mutation vectors / 33 diff vectors found (before 0 / 0), all agree with Ajv (re-run 23:30: `Note exact document contracts matched 66 native snapshots, 33 committed diffs`, exit 0).

## 4. Work order (d): `production-cache-input-boundary` fixture sample paths (done, one semantic note)

The fixture `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/⚡️production-cache-input-boundary/🔣️.json` named sample paths from before the carve; they are replaced by the real paths of the moved sources and fixtures, so `select("production")` / `select("test")` agree with the Nx glob oracle for every sample. The test `…/🧪️tests/⚡️production-cache-input-boundary/🟦️.ts` first test now passes in the repository process envelope (`tt6_envrun.ts ... -t "production cache inputs exclude"`: 1 pass, 0 fail).

**Semantic change that needs the owner's ruling:** HEAD asserted `nativeSelected("test") === true`, i.e. the native project's `nativeTestSources` named input selects the native fixture path by glob. The owner has since replaced the glob list by a source-closure digest (`cacheInternals.projectInputs` gives `nativeTestSources = [two digest files, {env: SEMIO_SOURCE_CLOSURE_…}]`), so a glob can no longer select that file. I kept the oracle agreement (`nativeSelected` still compares Nx to the independent matcher for both targets) and replaced the obsolete boolean with `expect(closure(nativeTestSources)).toBeDefined()` and `…not.toBe(closure(nativeSources))` (the test closure exists and differs from the production closure). If the owner wants the fixture to be a glob input again, that is a production decision, not a test fix.

Remaining in (d): three further tests of the file publish into a temporary workspace through `publishNxCollectionBoundary`, which needs an owner seam (an issued ScriptInvocation bound to a workspace other than the repository). Not faked.

## 5. Work order (b): generated parsers match their Rust-faithful schemas (14 of 18 done)

Tool: `bun ./📜️script.ts schema parsers report|apply|check` (no new script). Method per diagnostic: compare the schema (`🔣️.json`), the Rust-faithful type and the emitted parser; correct the schema first (`required`, nullability, bounded integers, closed objects), then `apply`.

- Fixed at the schema (and regenerated: json schema, graphql, proto, TS interface and parser): html artifact and xml artifact; ifc binary snapshot and diff and the ifc sqlite test; pdf diff; equation diff; graph diff; model text snapshot; wav mutations test; gif 87a and 89a diff plus the gif snapshot graphql / proto, with new vector tests and fixtures (`🧪️tests/🔬️unit` and `🧫️fixtures`, independent Ajv oracle; 89a unit test: 1 pass, 28 expect calls).
- Owner fix in `📐️type-render/🟦️.ts`: the GraphQL and protobuf renderers now unwrap nullable fields (`nullableOf`) and render a free-form object as a scalar. This is what reduced the refusals from 1,295 to 888 (protobuf -165, graphql -242). New test `🪞️parser-projection/🧪️tests/🫥️nullable-fields/{🟦️.ts,🔣️.json}` (graphql and protobufjs as independent renderer oracles).
- Unowned registry `…/🪞️parser-projection/🧱️unowned/🔣️.json`: 186 rows (+9 net: html artifact, 8 gif); each row names its cause (`unrenderable`, `in-memory-model`, `schema-disagrees-with-type`).
- **Remaining 4 (TS2322): `🧿️semio/…/🔺️mesh/…/🔺️diff` 3 and `💬️bcf/…/🖊️markup/…/🔺️diff` 1.** Cause: the diff parsers need snapshot parsers and an in-memory model that do not exist yet (the diff schema `$ref`s a snapshot that has no parser). Declaring them unowned would hide a real missing artifact, so they stay as findings for the mesh and bcf owners.
- Remaining refusal classes (888, `tt6-tools/tt6-refused-final.txt`): 294 parser refusals (schemas with `oneOf` of open objects, recursive refs and integer carriers wider than 53 bits, same cause per artifact family), 131 typescript, 281 protobuf and 182 graphql (unions of objects without a discriminator, `additionalProperties` maps with non-string keys). Left for the schema owners; none is a regression.

## 6. Work order (a): custody / storage-admission tests (partially done)

Rule applied: the test obtains its issued invocation, storage reservation and destination exactly as the production caller does (`receiveScriptProcessInvocation` / `withScriptProcessEnvelope`, `withLawStorageV1`, `withNativeImageStorageV1`, `acquireNativeStorageV1`, `withRepositoryCompilerStoreV1`, `withRepositoryCacheStoreV1`, `withRepositoryArtifactDestinationV1`, `withBuildTestInvocationV1`), no fake authority, no cast.

- New owner kit helpers (no test logic, reusable): `withIssuedLawStorageV1`, `lawBuildOptionsV1` in `…/🏗️native-build/🧪️tests/🦀️cargo/🟦️.ts`; new `…/🧪️testing/🦀️cargo/🧪️tests/🗄️admission/🟦️.ts` with `withCargoAdmissionV1` (production admission shape `{workspaceRoot, control, storage, sourceBoundary, planningPort?, sourcePort?, withCargoInputs?}`) and `writeOwnerPackageV1`.
- Rewritten tests (list `tt6-tools/tt6-files-custody.txt`): native-build (`🧪️tests`, `📦️publication`, `🔒️lease`, `🎟️admission`, `🎯️recipe`, `🧹️cleanup`, `🍎️image checkpoint`), `process/🏗️build/…/🏗️native`, `testing/🦀️cargo` and its `🔒️lease`, `library/🏗️build/…/🏗️native`, caching artifacts, bootstrap tools, server coordinator storage / inputs, dev catalog, discovery, bootstrap-tools.
- Type result: the 48 custody diagnostics of TT5 are down to 5 in `tt6-w3` (scoped runs `tt6-a0..a3`: 78 -> 37 diagnostics over the edited set; the 37 are 15 TS2307 and 13 TS7006 of the peer capacity tests, the open ones below, and the GIF line).
- **Still open (5 diagnostics, need an owner seam, not faked):**
  1. `…/🏗️native-build/🎯️recipe/🧪️tests/🟦️.ts` (3 x TS2345): the production recipe test observes lease queueing by passing its own `control`; the owner exposes no seam to observe that through an issued control.
  2. `…/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🚚️vendor-transport/🟦️.ts` (1 x TS2339): `pluginOutRoot` was removed from the catalog builder; the test needs the new destination accessor.
  3. `…/📚️library/🔍️discovery/🕸️runtime/🧪️tests/🟦️.ts` (1 x TS2345 at line 496): the runtime helper builds paths outside the compiler store.

## 7. Tests run (real results)

Harnesses: plain `bun test` (files that do not need a process envelope), and `tt6-tools/tt6_envrun.ts` (runs `bun test` inside the original process envelope the repository Script entrypoint creates; the envelope allows one Original birth per bun process, so `tt6_pertest.py` runs every test of a file in its own process with `-t`). Baseline for comparison: the HEAD version of each file, run by the same harness (`tt6-pt-*.tsv`, copies removed afterwards).

| Family | Now | HEAD (same harness) | Verdict |
| --- | --- | --- | --- |
| Custody set A (native-build, build/native, cargo, admission, recipe, cleanup, checkpoint, lease, publication; 44 tests) | 26 pass / 18 fail | 19 pass / 25 fail | 0 worse, 7 fixed |
| Custody set B (dev catalog, cargo lease, profile-and-catalog build, caching artifacts, bootstrap tools, library build native, discovery, coordinator storage / inputs; 62 tests) | 8 pass / 47 fail / 7 none | 8 / 47 / 7 | identical per test |
| Swap / pair files, plain `bun test`: draw prepare (`🎨️paint/📋️prepare`) | 14 pass, 0 fail | load error | better |
| `layout-drawing/🪶️sqlite` | 18 pass, 1 fail | 18 pass, 1 fail (`$.dataFields: unknown field`) | identical, pre-existing |
| `retention/📥️physical`, `selection/📥️scope` (fail names) | 34 and 12 | 34 and 12 | identical sets |
| `platform/📋️contract/…/📥️parent` | 29 fail | 26 fail | +3 are three of 8 tests a peer added at 22:23 (same envelope error), none of mine |
| `production-cache-input-boundary` first test (envelope) | 1 pass | n/a | pass |
| GIF 89a diff unit test (Ajv oracle) | 1 pass | new | pass |
| `schema parsers check` | green | green | identical |
| Note oracle (`tt6_run_note.ts`) | 66 / 33 vectors, all pass | 0 / 0 | fixed |

The failures of set A / B in this harness are environment failures of the harness itself, not defects of the migrated tests: `Live Original resource control issuance required`, `Host storage cannot admit fixed image envelope`, `Original process admission closed`, `Native storage path escapes enforcing boundary`. They fail identically at HEAD. The authoritative lane for these tests is the repository test route through nx, which I did not run (it is workspace-wide and its budget exceeds the 20-minute slice budget under the present fleet load; load average 65-140 throughout).

## 8. The 44 real diagnostics of `tt6-w3` and the skipped class

Real (list `tt6-tools/tt6-w3-real.txt`):

| Bucket | Count | Owner |
| --- | --- | --- |
| Peer capacity tests (`bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests`, TS7006 / TS18048, imports `../🟦️.ts` not landed) | 14 | peer split in flight |
| Peer crate-tree carve (`🏗️authoring/📦️crate-tree` TS2305 / TS7006 x2, `🧰️testkit` TS2304, `🪞️unmirror` TS2741, `🧪️tests/📦️crate-tree` TS2300 x2) | 7 | peer c4pp / bd11 in flight |
| Follow-on of missing packages: svg export test (`@xmldom/xmldom`) 7, poll (`rxjs`) 1, termination (`tree-kill`) 1, print charts (`d3-regression`) 2 | 11 | `bun install` (user decision) |
| Unbuilt wasm outputs (counted in the 13 of the verdict): trinity jack lsp (`trinity_jack_lsp.js` has no `JackLspSession`), flow core rust (`semioSetRuntimeAssets`) | 2 | build artifacts |
| mesh / bcf diff parsers (b) | 4 | mesh / bcf owners |
| Custody seams (a) | 5 | process / dev / discovery owners |
| GIF 89a unit test (fixed after the run: `toEqual<typeof row.value>`) | 1 | done |

Skipped by order (ruling 3), unchanged class: TS2307 102 (relative `../🟦️.ts` 16 of the peer capacity move; `@semio-tech/mit-bestand-praesentation-projektetage` 12; `@xmldom/xmldom` 7; `brepjs` 4 + `brepjs-opencascade` 1; `@semio-tech/stdio-*` and `cad-js-module-*` workspace packages; `tree-kill` 2; unbuilt wasm outputs: `puzzle-wasm` 3, `semio_puzzle.js` 2, `framework_surface_*` 4, `flow_core.js` 1), TS7016 15 (`jsdom` 4, `yargs-parser` 4, `d3-regression` 2, `bidi-js`, `check-error`, `color-convert`, `suncalc`, one `jcoprobe` `.js`), `mit-bestand` follow-ons 160 (139 TS7006). Nothing was added to `node_modules`.

## 9. Commands run

- `bun ./📜️script.ts verify typescript check [--files-from LIST] --out PREFIX --heap-mb 16384 --max-diagnostics 100000` (`tt6-s0..s7`, `tt6-a0..a3`, `tt6-ren`, `tt6-w1` OOM at 8 GB, `tt6-w2` killed after 26 min by the fleet load, `tt6-w3` final)
- `bun ./📜️script.ts schema parsers report|apply|check`
- `node tt6-tools/tt6_swaps.mjs` (scan / `--apply`), `tt6_widen.mjs`, `tt6_pairs.py`, `tt6_pertest.py`, `tt6-envbatch.sh`, `tt6-ptboth.sh`, `tt6_bun_cmp.py`, `tt6-bun.sh`
- No git command that modifies anything; no ticket tool; no sub-agent.

## 10. Files changed

- Matcher order restoration and typing: 67 files, list `tt6-tools/tt6-files-matchers.txt`; originals of the 45 swap files in `tt6-tools/orig-swap/`.
- Custody tests and kit: list `tt6-tools/tt6-files-custody.txt` (46 files) plus the new `…/🧪️testing/🦀️cargo/🧪️tests/🗄️admission/🟦️.ts`.
- Parsers / schemas: sections 3 and 5 (html, xml, ifc, pdf, equation, graph, model text snapshot, wav, gif 87a / 89a; `📐️type-render/🟦️.ts`; the unowned registry; `🪞️parser-projection/🧪️tests/🫥️nullable-fields`).
- Note oracle and (d): sections 3 and 4.
- Tools: `tt6-tools/` (`tt6_swaps.mjs`, `tt6_widen.mjs`, `tt6_pairs.py`, `tt6_edit.py`, `tt6_unowned.py`, `tt6_gif.py`, `tt6_refused.ts`, `tt6_envrun.ts`, `tt6_pertest.py`, `tt6-envbatch.sh`, `tt6-ptboth.sh`, `tt6_run_note.ts`, `tt6_note_probe.ts`, `tt6_show.py`, `tt6-bun.sh`, `tt6-bun-head.sh`, `tt6_bun_cmp.py`, the file lists, `tt6-w3-real.txt`, `tt6-refused-final.txt`).
- Logs stay in `🗑️generated/tt6-*` for the coordinator. `.cache/tt6-art` could not be removed (an orphan from a killed run holds `i/m`); it is outside git.
