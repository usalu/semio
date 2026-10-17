# Result: DRAW-RUNTIME-TESTS (prefix `drt`)

Executor: Sonnet, 2026-10-16 19:08 to about 19:50. No sub-agents, no modifying git command, no ticket tool, no `bun install`, no download. Logs are in `🗑️generated/drt-*`, tools in `drt-tools/`, pre-edit snapshots of the files I edited by hand in `🗑️generated/drt-snap/`.

## Verdict

DONE. The draw test route (`drt-tools/drt-run-draw-tests.sh`, the same 69 files and the same per-file `bun test ./<file>` as `tt2-tools/tt2-run-draw-tests.sh`) went from **581 pass / 46 fail (tt2 baseline run)** to **663 pass / 7 fail**. The 7 left are the `layout-drawing` sqlite test, which I skipped on purpose, and 6 files that do not load because `@xmldom/xmldom` is not installed. The set of failing test names went from 32 distinct to 1, and nothing is newly failing (`drt-draw-tests-after2.log` against `tt2-draw-tests-before.log`: 0 new, 31 fixed). `tsc` on the touched files shows no new diagnostic.

The 44 failing entries, by cause:

| Entries | Cause | Class | Fix |
| --- | --- | --- | --- |
| 16 | `native-owner` test reads Rust text | 8 pure moves (the host was carved into the `🏟️arena` and `🔏️digest` crates) and 8 stale since the pre-split code change `03c902fa488` | re-pointed and rewritten, see 1 |
| 3 | sqlite snapshot test, `Draw row identity or fields` | real defect in the production reader | fixed in the reader, see 2 |
| 14 | `ENOENT …/📦️data/🔢️properties.bin` | the data file was carved into two files; the TS loader still named the old one | loader and every consumer updated, see 3 |
| 2 | `prepare` test: `no schema with key or ref draft-07` | stale test (Ajv 2020 without the draft-07 meta schema) | fixed |
| 6 | `🔤️text` test: expected provider reads 17, got 18 | stale expectation (the Unicode record is now two reads) | fixed |
| 1 | `edit-selection` nudge shortcuts | the manifest was carved out of the editor root | re-pointed |
| 1 | `export-document` PNG refusal | fixture moved under `🧪️tests` | re-pointed |
| 1 | `interaction` Select All | same manifest carve | re-pointed |
| 1 | `kinds-catalog` (count 0 of 26) | fixtures moved to `<subset>/🧪️tests/🧫️fixtures`, the glob did not follow | glob fixed |
| 1 | `layout-drawing/🪶️sqlite` | skipped, see "Not done" | none |
| 6 files | `Cannot find module '@xmldom/xmldom'` | environment | reported, not touched |

The rows with 2 (`prepare`) and 6 (`🔤️text`) entries were not among the 44: they were hidden behind the missing Unicode file and only showed once it loaded. The table therefore sums to more than 44; the distinct-name comparison in the verdict is the verified number.

## 1. native-owner: source-text readers

File: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/📋️native-owner/🟦️.ts`. Before: 6 pass / 16 fail (`drt-native-owner-before.log`). After: **22 pass / 0 fail** (`drt-native-owner-3.log`).

The owner is no longer one `🦀️.rs`: `🦀️.rs` (host, 2163 lines), the arena crate `🏟️arena` (mounts `📐️footprint`, `♻️retirement` and `🏗️initialization/📚️catalog`) and the digest crate `🔏️digest`. The test now has two helpers (docstrings with an emoji):

- `ownerSource(owner)`: `readModuleTreeSource` over the three crate roots, concatenated.
- `ownerSlice(owner, from, to?)`: `sliceModuleTreeSource` in the crate that holds `from` (the old `host.slice(host.indexOf(a), host.indexOf(b))`; 12 sites rewritten with one regex plus one hand edit).

This alone took the failures from 16 to 8 (`drt-native-owner-1.log`). The other 8 were stale before the split already: the code changed in `03c902fa488` (positional `close_initialization_catalog(.., 1, maximum_bytes)` became a `child` grant with `maximum_items: 1`, `PluginCloseStep` became `RetainedCloneStep`, `close_granted` became `close_step(grant)`, the decoded-field retirement became the derived `DrawingRetirementOwner` enum plus `artifact_retirement_*` demands, `crate::PathSegment` became the model crate path). I re-stated each assertion against the current code, keeping the property it names:

- initializer closes the catalog one page at a time: `let child = RetainedCloneGrant { maximum_items: 1,`, `close_initialization_catalog(&mut self.owner_catalog, child)`, `catalog.close_step(grant)`, terminal check kept.
- initializer transfers owners into typed retirement: `retire_pending: ManuallyDrop<Option<DrawingRetirementOwner>>`, one `*self.retire_pending = Some(DrawingRetirementOwner::<kind>(` per kind of the fixture's `initializerOwners`, and the hand-over `artifact_retirement_admit_owned(&mut self.retire_pending, &mut self.active, child)`.
- native retirement variants: the derive and the enum, every `law.variants` entry present, the three byte-demand axes.
- decoded field close: `next_grant`, `maximum_copy_bytes: demand.copy_bytes`, `artifact_retirement_owned_birth_demands(&self.value)` and `artifact_retirement_box_demands`.
- duplicate cancellation: `close_step(&mut self, grant: RetainedCloneGrant)` on the digest authority, `retire_displaced_granted(grant)`.
- sparse path patches and layer clone: the new type path and `close_step`.
- The oracle line `bytes = TextEncoder / Uint32Array byteLength` in "payload work separately from physical backing" was wrong against the fixture `🧰️framework/…/♻️retirement/🧪️tests/🧫️fixtures/📏️copy-demand/🔣️.json` (u32 and text `totalCopyBytes: 0`, the list 16). The fixture is unchanged since `03c902fa488` and the Rust test `🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs` asserts `copied == totalCopyBytes`, so the fixture is the truth (scalars and text move without a copy). The oracle is now `u32-list ? Uint32Array byteLength : 0`.

Design note for the owner of the Rust (not changed): the test title "inline typed retirement" now holds for `retire_pending` only. `active` is `Option<Box<dyn store::ErasedSnapshotRetirement>>` since `03c902fa488`, so the typed owner is boxed when it is admitted. I asserted what the code does, not what the title says.

## 2. sqlite snapshot: real reader bug

`🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts`. `drawSnapshotFromSqliteDatabase` checks every row against a hand-written width table (`widths=[…]`, one entry per table). The schema `🗄️.sql` got `font_family` in `draw_text` (commit `c44e964518f`, 6 columns) and the writer and the text case of the reader use it (`string(v,5)`), but the width entry for `text` stayed 5, so the first text row was refused with `Draw row identity or fields`. I proved it by comparing `PRAGMA table_info` of an in-memory database built from `🗄️.sql` against the table (`🗑️generated/drt-widths.ts`): the only mismatch was `text`, schema 6, reader 5. Fix: `5` to `6` at index 20, one character. The three failing tests were the red tests; now 7 pass / 0 fail (`drt-sqlite-1.log`). The Rust reader `🦀️.rs` next to it was not looked at.

## 3. Unicode data: carved into two files

`📦️data/🔢️properties.bin` (282392 bytes) no longer exists. `git ls-tree` shows the carve (commit `36fbe9da786`) split it at the decompositions offset into `📦️data/🏷️properties/🔢️properties.bin` (136336 bytes, header plus ranges) and `📦️data/🔁️mappings/🔢️mappings.bin` (146056 bytes, the rest); the Rust crate joins them at const time (`PROPERTIES` + `MAPPINGS`). Proof the split is lossless: `git cat-file blob 0884ccf3…` and `cat properties.bin mappings.bin` both hash to `492d9d8880e3ee31d3770876371787b059a5b98189ce444c99c4790bcfe290c1`. Nothing to restore, nothing to download.

The TS side still had `unicodeSourceUrl` (one URL). Changed, with every caller updated at once and no alias:

- `🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/🟦️.ts`: `unicodeSourceUrl` is now `unicodeSourceUrls` (the two parts in byte order); `loadBuiltinUnicodeSource` fetches both through the same provider (ids `16.0.0/0` and `16.0.0/1`), keeps the 16 MiB limit on the total and the cancellation checks, and concatenates. The admission job already checks the header's total length, so a wrong order or a missing part is refused.
- The two re-exports (`🧰️framework/📦️packages/🟦️typescript/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/🟦️.ts` and the untracked in-flight copy `🧰️framework/🔨️modules/◻️2d/📦️packages/🟦️typescript/📝️text/…/🌐️unicode/🟦️.ts`: one token each).
- 8 framework test files (`🌐️unicode`, `↔️bidi`, `🧵️cluster`, `🧵️normalize`, `📇️face`, `🧵️layout`, `🧵️run`, and `🔤️font/🧪️tests`) used `new Uint8Array(await Bun.file(unicodeSourceUrl).arrayBuffer())`; they now call `await loadBuiltinUnicodeSource()` (Bun's `fetch` reads `file:` URLs; checked in `drt-fetchfile.ts`). All 8 pass: 9, 3, 2, 2, 1, 1, 1 and 8 tests (`drt-fw-unicode-*.log`). These were failing before for the same ENOENT, which is outside the draw route.
- Draw: the shared fixture module `✳️any/🧪️tests/🧫️fixtures/🟦️.ts` got `unicodeSourceBytes()` (sync, joins the two parts); the four scene tests that read the file use it. The `🔤️text` test counted provider reads as `1 + fonts`; it is now `unicodeSourceUrls.length + fonts`.
- The generator `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts` wrote the single `🔢️properties.bin`. It now writes the two halves, split at the u32 at byte 20 (the decompositions offset). I checked the split expression against the files on disk (`drt-split-check.ts`: offset 136336, both halves byte-equal); **I did not run the generator** (it may download).

## 4. The rest

- `prepare` unit test: two `new Ajv({strict:true})` on a draft-07 schema, but `Ajv` comes from `ajv/dist/2020.js`; the file's other instance already did `.addMetaSchema(draft7)`. Added to both. 109 pass.
- `edit-selection` and `interaction`: they read `✏️editor/🦀️.rs`; the manifest (`.keybinding`, `.action_audience`, `hierarchy: HierarchyProvider::Topology`) lives in `✏️editor/🛂️manifest/🦀️.rs`, mounted from the draw root `🖍️drawing/🦀️.rs`, not from the editor. Re-pointed to the manifest file; `fn interaction_topology(` is still in the editor root. 9 and 9 pass.
- `export-document`: `frameworkSourcePath("…/✍️encode") + "🧫️fixtures/🔣️.json"` is now `…/✍️encode/🧪️tests/🧫️fixtures/🔣️.json`. 8 pass.
- `kinds-catalog`: glob `*/🧫️fixtures/🧬️mutations/…` to `*/🧪️tests/🧫️fixtures/🧬️mutations/…`; it finds the 26 fixtures that match the aggregate `oneOf` length. 2 pass.

## Verification

All from `/Users/ueli/Documents/semio`.

1. Baseline: `tt2-draw-tests-before.log` (581 pass / 46 fail entries, 32 distinct failing names).
2. `drt-tools/drt-run-draw-tests.sh after1` (mid-way, while I was still fixing the last four): 659 pass / 11 fail.
3. `drt-tools/drt-run-draw-tests.sh after2` (final): **663 pass / 7 fail**. Failing: `Layout dictionary changes distinguish unchanged, clear, present empty and member null` (layout-drawing) and 6 load errors `@xmldom/xmldom`. New failing names against the baseline: none (python set comparison).
4. Targeted runs: native-owner 22/0; sqlite snapshot 7/0; framework Unicode consumers 8 files all green; the five scene files (8, 14, 29, 109, 44 pass).
5. `node --max-old-space-size=12288 node_modules/typescript/bin/tsc -p 🗑️generated/drt-tsconfig.json --noEmit --pretty false` (`drt-tsconfig.json` = the 227 `tt2-tsconfig-all.json` roots plus the new fixture module, the touched tests, the unicode module and its re-exports, the draw script; 239 files). Output `drt-tsc-after.txt` (4530 lines; exit 2 because of the repo-wide pre-existing diagnostics). In the touched files the diagnostics are identical to `tt2-all3.txt` (103 against 103, 0 new, 0 gone; only column numbers moved by 30 where `await loadBuiltinUnicodeSource()` replaced the longer expression). No diagnostic mentions `unicodeSource*`, `ownerSlice`, `ownerSource` or `readModuleTreeSource`. The remaining ones are third-party typings (`hinting`, `bidi-js` declaration) and missing packages.

## Not done, and hand-offs

1. **`layout-drawing/🪶️sqlite/🟦️.ts`** (1 failing test): the file was modified at 18:43 (41 minutes before this check), so it is not abandoned; skipped as instructed. Given the draw bug above, the first thing to compare is its width table against its `.sql`.
2. **`@xmldom/xmldom` missing** (6 test files that do not load: SVG export unit/write, SVG import document/path/transform, fill rule). Install decision of the user.
3. **Framework `🌱️value/♻️retirement/🎮️controlled/🧪️tests`**: 7 of 15 tests fail (`drt-fw-controlled.log`). At least the two "copy demand" ones use the same stale oracle (u32/text byte length against `totalCopyBytes: 0`); the others I did not diagnose. Not draw, not touched.
4. **Generator not run**: the draw `📜️script.ts` change is verified by the split-point check and `tsc`, not by regeneration.
5. Rust was not compiled (compiler ingress); the native-owner assertions are text laws over the current sources, not a compile result.

## Files changed

Draw, under `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/` unless stated: `🔨️modules/🏠️host/🧰️owned/🧪️tests/📋️native-owner/🟦️.ts`; `🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts` (reader width); `…/✳️any/🧪️tests/🧫️fixtures/🟦️.ts` (`unicodeSourceBytes`); `…/🧬️schema/🎬️scene/🎨️paint/📋️prepare/🎯️query/🧪️tests/🔬️unit/🟦️.ts`, `…/🎨️paint/📋️prepare/🧪️tests/🔬️unit/🟦️.ts`, `…/📋️prepare/🧪️tests/🔬️unit/🟦️.ts`, `…/🔤️text/🧪️tests/🟦️.ts`; `…/✏️editor/🎮️commands/🎛️edit-selection/🧪️tests/🔬️unit/🟦️.ts`, `…/📤️export-document/🧪️tests/🔬️unit/🟦️.ts`, `…/🕹️interaction/🧪️tests/🔬️unit/🟦️.ts`; `…/🧬️schema/🧬️mutations/🧪️tests/🔬️kinds-catalog/🟦️.ts`; `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts`.

Framework: the unicode module, its two re-exports and the eight test files named in section 3, all under `🧰️framework/`.

Tools: `drt-tools/` (`drt-needles.py`, `drt-native-owner-{1,2,3}.py`, `drt-unicode-source.py`, `drt-unicode-consumers.py`, `drt-unicode-framework.py`, `drt-run-draw-tests.sh`). No `[DEBUG]` log added. Nothing deleted in any `🗑️generated` folder.
