# Result: RETIREMENT-ORACLE-TESTS (prefix `rot`)

Executor: Sonnet, 2026-10-16 about 19:30 to 19:50. No sub-agents, no modifying git command, no ticket tool. Logs are in `🗑️generated/rot-*`, tools in `rot-tools/`, pre-edit snapshots of the two test files in `🗑️generated/rot-snap/`. Nothing was deleted in any `🗑️generated` folder.

## Verdict

DONE.

- Framework controlled-retirement test: **8 pass / 7 fail to 15 pass / 0 fail**. The 15 test titles are identical before and after (checked by diffing the `test("…")` lines of the pre-edit snapshot against the file).
- Draw route (the same 69 files as `drt-tools`): **663 pass / 7 fail before, 663 pass / 7 fail after**. Per-file counts are identical (diff of the two result tables is empty), no new failing name. The 7 are the known ones (the `layout-drawing` sqlite test plus 6 files that do not load because `@xmldom/xmldom` is not installed). The draw `native-owner` file itself is 22 pass / 0 fail before and after.
- Whole retirement TS tree (18 test files under `🌱️value/♻️retirement`): 37 pass / 13 fail to 44 pass / 6 fail. The 7 fixed are exactly the 7 controlled failures. Nothing new fails.
- The copy-demand oracle now has a single owner, a shared test-support module that both the framework test and the draw test import.
- Exactly one test title changed: the draw `native-owner` test whose title overstated the code (section 3). Production code is untouched.

## 1. Baseline

All commands from `/Users/ueli/Documents/semio`. Runner: `rot-tools/rot-run-tests.sh <list> <tag>` (per-file `perl -e 'alarm 900; exec @ARGV' -- bun test ./<file>`), totals by `rot-tools/rot-sum.sh`.

| Route | Files | Before | After | Logs |
| --- | --- | --- | --- | --- |
| controlled test alone | 1 | 8 pass / 7 fail | 15 pass / 0 fail | `rot-controlled-before.log`, `rot-controlled-after1.log` |
| retirement tree | 18 | 37 pass / 13 fail | 44 pass / 6 fail | `rot-ret-before.{log,tsv}`, `rot-ret-after.{log,tsv}` |
| draw route | 69 | 663 pass / 7 fail, 6 load errors | 663 pass / 7 fail, 6 load errors | `rot-draw-before.{log,tsv}`, `rot-draw-after.{log,tsv}` |
| draw `native-owner` alone | 1 | 22 / 0 | 22 / 0 | `rot-native-owner-after1.log` |

Failing names, before and after (`rot-ret-fail-*.txt`, `rot-draw-fail-*.txt`): new failing names 0 on both routes; fixed on the retirement tree: the 7 controlled names below; draw: none changed.

The 7 controlled failures were **not all the stale oracle**. The draw result said so for "at least two"; I diagnosed all of them:

| # | Test (unchanged title) | Cause | Fix |
| --- | --- | --- | --- |
| 1 | `controlled retirement keeps logical UTF8 and ordered scalar work distinct from physical releases` | stale oracle: `TextEncoder`/`Uint32Array.of` byte length against fixture `processedBytes: 0` | shared oracle |
| 2 | `controlled paged retirement exposes the next independent payload work demand` | stale oracle, same, against `totalCopyBytes: 0` | shared oracle |
| 3 | `controlled paged retirement closes appended UTF8 without unadmitted scaffold births` | stale source-text anchor: `pub fn close_granted(` was renamed `close_step` (the same rename the draw slice found, commit `03c902fa488`) | anchor re-stated |
| 4 | `controlled paged retirement exposes typed child metadata and exact scaffold demand` | stale marker: `trait ControlledCloneRetirement` no longer exists, so `slice(indexOf(a), indexOf(-1))` silently became `""`; the trait now lives in `🧬️retained-clone/🧬️contract/🦀️.rs` (not the crate root) | read the contract file, end marker `pub struct RetainedCloneClose` |
| 5 | `controlled paged retirement observes retained scalar and UTF8 leaf birth demand` | same: `ScalarCursor`/`StringCursor` impls moved into `🧬️contract/🦀️.rs`; the leaf slice in `♻️retirement/🦀️.rs` still exists | read the contract file |
| 6 | `controlled paged retirement preserves copy demand through native clone close frontiers` | the first of six read paths pointed at the retained-clone crate root, the default `next_close_copy_byte_demand` is in the contract file | path re-pointed |
| 7 | `cold shared retirement keeps logical work separate from exact physical backing` | `SharedRetirement<T>` is now `SharedControlledRetirement<T>`, in `♻️retirement/🔗️shared/🦀️.rs`; the method sits in its `RetirementCursor` impl | read the shared file, slice that impl up to `RetireOwned for Arc<T>` |

## 2. The oracle and its single owner

What the production rule is, read from the source (not from the fixture): `Vec<T>` retires through `Collection<T>` (`♻️retirement/🦀️.rs`), whose `next_work_byte_demand` is `T::retirement_element_copy_bytes()` while the vector is non-empty and `T` has no `Drop`; for the scalar leaf macro that value is `size_of::<T>()`, so a `Vec<u32>` of 4 elements drains as 4 turns of 4 bytes, 16 in total. A lone scalar (`Leaf<T>::next_work_byte_demand` is `Ok(0)`) and a `String` (`Bytes::next_work_byte_demand` is `Ok(0)`) move to their cursor without payload work and only release their physical backing. `ControlledRetirement::next_copy_byte_demand` is 0 while the root value is still held. This agrees with the fixtures `📏️copy-demand` and `⚖️physical-work` and with the Rust unit test that asserts `copied == totalCopyBytes`. The stale TS rule `TextEncoder` byte length for text and `Uint32Array.of` for a scalar contradicted all of that.

Where the oracle lived: inline in draw `native-owner` (`row.kind==="u32-list"?…:0`, left by the previous slice) and twice in the framework test (the physical-work test and the copy-demand test; the frontier test only reads `row.totalCopyBytes` and now takes the typed law from the shared reader). Now one module:

- `🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🧫️fixtures/🟦️.ts` (new). Exports `retirementCopyDemand(row)` returning `{minimumCopyBytes, totalCopyBytes}` from the rule above, and throws on a kind it has no rule for instead of defaulting to 0. Also exports `readRetirementCopyDemandLaw()` and `readRetirementPhysicalWorkLaw()` (typed readers of the two corpora, via the existing `fixtureUrl`) and the types. Docstring starts with a new emoji.
- `🧰️framework/📦️packages/🟦️typescript/🌱️value/♻️retirement/🧪️testing/🧫️fixtures/🟦️.ts` (new SDK entry, same pattern as the peer entries `identity/testing/fixtures` and `interaction/testing/fixtures`) plus one export line in `🧰️framework/📦️packages/🟦️typescript/package.json`: `"./value/retirement/testing/fixtures"`. Re-read right before the edit because a peer is editing the same file; the file still parses as JSON.
- Framework test imports the module by relative path (`../../🧪️tests/🧫️fixtures/🟦️.ts`). Draw imports `@semio-tech/framework/value/retirement/testing/fixtures`. Both now assert `retirementCopyDemand(row)` against the corpus numbers, so the draw copy-demand test also checks `minimumCopyBytes` now (it only used it for the SQL grant lookup before).

Hardening in the framework test: a local `between(text, from, to)` replaces the five `slice(indexOf(a), indexOf(b))` sites. A missing or misordered marker now throws with the marker text instead of slicing to `""`, which is what made failures 4, 5 and 7 hide as an empty `Received: ""`.

## 3. The overstated title (draw `native-owner`)

Before: `Drawing initializer transfers original active owners into inline typed retirement`.
After: `Drawing initializer holds original owners in typed pending retirement and hands them to the active retirement through artifact_retirement_admit_owned`.

What the test asserts (kept unchanged): `retire_pending: ManuallyDrop<Option<DrawingRetirementOwner>>`, one `*self.retire_pending = Some(DrawingRetirementOwner::<kind>(` per kind in the fixture, the hand-over `artifact_retirement_admit_owned(&mut self.retire_pending, &mut self.active, child)`, and the absence of `Box::new(DrawingOwnedRetirement` and `ArtifactOwnedValueRetirementFactory::retire_owned`. Production reality, read in `🌱️value/📸️snapshot/👁️read/🦀️.rs:728`: `artifact_retirement_admit_owned(pending: &mut Option<T>, active: &mut Option<Box<dyn ErasedSnapshotRetirement>>, …)` calls `admit_owned_retirement` and stores the result as `Some(owner)`, a boxed trait object; the draw initializer's `retirement` field is `ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>`. So "inline typed" is true of `retire_pending` and false of `active`. The test body needed no change. The adjacent `[DEBUG]` console line claimed the exact originals sit "under one typed active owner"; I corrected that wording to match (it is a pre-existing temporary log, not added by me).

No other draw test title changed (diff of the `test("…")` lines, snapshot against current: only line 2).

## 4. Type-check

`node --max-old-space-size=12288 node_modules/typescript/bin/tsc -p 🗑️generated/rot-tsconfig-{before,after}.json --noEmit --pretty false` (the `drt-tsconfig.json` with `files` replaced; it still inherits the root `include`, so the whole repo's diagnostics appear, outputs `rot-tsc-before.txt` 4617 lines and `rot-tsc-after.txt` 4513 lines, exit 2 both times). Before = the pre-edit snapshots of the two test files, after = the real touched files: the draw test, the controlled test, the new module and the new SDK entry.

- Controlled test: 16 diagnostics before, 16 after, identical columns, lines shifted by exactly 10 (one import line plus the nine-line `between` helper). All 16 are the pre-existing `TS2769 No overload matches this call` on `applyPatch`/`readFileSync` typings. None is new, none mentions the new module.
- Draw `native-owner`: 0 diagnostics before and after.
- New module and SDK entry: 0 diagnostics; the import `@semio-tech/framework/value/retirement/testing/fixtures` resolves.

The 4.5k other lines are unrelated repo-wide diagnostics, unchanged in kind.

## 5. Not verified, and hand-offs

1. **Design hand-off for the Rust owner (not changed):** if the intent really is inline typed retirement for `active` as well, then `artifact_retirement_admit_owned`/`admit_owned_retirement` would have to keep the typed owner in an enum slot instead of returning `Box<dyn ErasedSnapshotRetirement>`. Today only `retire_pending` is inline. The title now says what the code does.
2. **Rust not compiled** (compiler ingress). The oracle is checked against the production source by reading and against the committed fixtures; the Rust unit test `🧪️tests/🔬️unit/🦀️.rs` that asserts `copied == totalCopyBytes` was not run. The source-text assertions are text laws over the current files, not a compile result.
3. **Six other retirement tests fail, same before and after, not copy-demand and not touched:** `🏭️factory/🧪️tests/📦️protocol` (`original protocol ticket keeps physical backing separate from its box shell`, line 17), `🔗️shared/🏭️factory/🧪️tests` (`shared factory metadata never enlarges a narrow payload copy grant`, line 15), and `🔗️shared/🔒️sealed/🧪️tests` (`sealed original alias custody …` and `borrowed original strong parent …`, each twice, lines 25 and 35, `Expected false, Received true`). All are Rust source-text assertions in other files; likely the same carve drift. A next slice should apply the `between` helper there too.
4. Draw leftovers from the previous result are unchanged: `layout-drawing/🪶️sqlite` (1 test) and `@xmldom/xmldom` missing (6 files).
5. Two sibling stale-anchor risks of the same shape remain in the framework test, passing today but name-keyed: paths such as `🧬️retained-clone/📦️paged/🦀️.rs` that assert `includes("fn …(")` over a whole file.
6. No launch.json change: no new test entry point was added (the new module is not a test file).

## Files changed

- `🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🧪️tests/🟦️.ts` (edited)
- `🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🧫️fixtures/🟦️.ts` (new, shared oracle)
- `🧰️framework/📦️packages/🟦️typescript/🌱️value/♻️retirement/🧪️testing/🧫️fixtures/🟦️.ts` (new SDK entry)
- `🧰️framework/📦️packages/🟦️typescript/package.json` (one export line)
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/📋️native-owner/🟦️.ts` (edited: import, copy-demand test, title, one debug line)

Tools: `rot-tools/` (`rot-run-tests.sh`, `rot-sum.sh`, `rot-names.sh`, `rot-controlled-edit.py`, `rot-draw-edit.py`). Scripted edits used asserted-unique anchors. No `[DEBUG]` log added; the pre-existing ones in the controlled test were left.
