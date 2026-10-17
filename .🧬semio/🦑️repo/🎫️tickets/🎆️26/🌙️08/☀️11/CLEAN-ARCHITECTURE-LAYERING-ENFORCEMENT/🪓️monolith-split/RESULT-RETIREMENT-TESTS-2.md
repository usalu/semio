# Result: RETIREMENT-TESTS-2 (prefix `rt2`)

Executor: Sonnet, 2026-10-16 about 20:00 to 20:25. Foreground, no sub-agents, no modifying git command, no ticket tool. Logs are in `🗑️generated/rt2-*` (nothing deleted), tools in `rt2-tools/` (`rt2-run-tests.sh`, `rt2-sum.sh`, `rt2-list.txt`, `rt2-tree-list.txt`). Every file was re-read right before its edit; scripted edits asserted a unique anchor (`count == 1`).

## Verdict

DONE. All 6 handed-off failures are fixed, none was a production defect, no test title changed.

- The 6 failures are 4 distinct tests. The `🔐️unique` test file imports the `🔒️sealed` test file (`import "../../🔒️sealed/🧪️tests/🟦️.ts"`), so the two sealed tests ran and failed twice (once under each file): protocol 1 + shared factory 1 + sealed 2 + the same sealed 2 again under `🔐️unique` = 6.
- Whole retirement TS tree (18 test files): **44 pass / 6 fail before (rot after-run) to 50 pass / 0 fail**. Nothing new fails.
- Controlled-retirement test: 15 pass / 0 fail before and after (unchanged result; its local `between` moved to the shared module).
- Production code is untouched. Rust was not compiled (compiler ingress); every assertion is a source-text law checked against the current files.

## 1. Baseline and classification

Command per file: `perl -e 'alarm 900; exec @ARGV' -- bun test ./<file>` from the repo root (`rt2-tools/rt2-run-tests.sh`). Before: `🗑️generated/rt2-before.{log,tsv}`.

| Test file | Before | After |
| --- | --- | --- |
| `♻️retirement/🏭️factory/🧪️tests/📦️protocol/🟦️.ts` | 0 pass / 1 fail | 1 / 0 |
| `♻️retirement/🔗️shared/🏭️factory/🧪️tests/🟦️.ts` | 1 / 1 | 2 / 0 |
| `♻️retirement/🔗️shared/🔒️sealed/🧪️tests/🟦️.ts` | 2 / 2 | 4 / 0 |
| `♻️retirement/🔗️shared/🔐️unique/🧪️tests/🟦️.ts` (imports sealed) | 6 / 2 (rot run) | 8 / 0 |
| `♻️retirement/🎮️controlled/🧪️tests/🟦️.ts` | 15 / 0 | 15 / 0 |
| all 18 retirement test files | 44 / 6 (rot run) | 50 / 0 (`rt2-tree-after.{log,tsv}`) |

| # | Test (title unchanged) | Class | Cause and fix |
| --- | --- | --- | --- |
| 1 | `original protocol ticket keeps physical backing separate from its box shell` | stale Rust source anchor | `factory_ticket_demands` / `close_factory_ticket` are no longer defined in `🏭️factory/🦀️.rs`; they live in `♻️retirement/🎟️tickets/🦀️.rs` (mounted by the crate root `📦️packages/🦀️rust/🦀️.rs`), and `🏭️factory/🦀️.rs` only re-exports them. The test now reads the `🎟️tickets` file for the two `pub fn` definitions and asserts the factory root re-export list with `between(…, "pub use semio_framework_value_retirement::{", "};")`. The native-test anchors after it (`born += heap.0`, `backing + shell + born`, …) were still present in `🏭️factory/🧪️tests/🦀️.rs`; I also replaced the silent `slice(indexOf(...))` there with an asserted start index (a missing marker used to slice the last character and fail only on a later `toContain`). |
| 2 | `🔐️ shared factory metadata never enlarges a narrow payload copy grant` | stale Rust source anchor | `pub use factory::FactorySharedRetirement;` is gone from `🔗️shared/🦀️.rs`; the type is defined in `🔗️shared/🏭️factory/🦀️.rs` and exported by the factory root as `#[path="../🔗️shared/🏭️factory/🦀️.rs"] pub mod shared; pub use shared::FactorySharedRetirement;`. The test now asserts that mount, the re-export in `🏭️factory/🦀️.rs` and `pub struct FactorySharedRetirement<` in the shared factory file. |
| 3 | `sealed original alias custody agrees with independent patch and schema oracles` | stale oracle (deliberate production addition) | The forbidden list contained `Arc::strong_count`, but commit `7c8af563c74` (`…🚩️687`; the test was green at `7af1259dba8`) added `SealedShared::original_aliases_are_returned(&self) -> bool`, which reads `Arc::strong_count(source)==1` and is used by `🧵️job/⏱️context/🎟️source/🦀️.rs` (`original_loans_are_returned`). The invariant the struct documents is "no raw, Weak, mutable or consuming escape hatch"; a `bool` observer exposes none of these. The test now isolates that single observer (`between(source, "pub fn original_aliases_are_returned(", "\n")`), asserts it returns `bool` and holds exactly one `Arc::strong_count`, and applies the full forbidden list (`downgrade`, `weak_count`, `strong_count`, `pub source:`, `into_arc`, `as_arc`) to the rest of the file. A second `strong_count` or any escape hatch still fails. |
| 4 | `borrowed original strong parent witnesses one directly paid nonlast alias return` | stale end marker | The body was sliced from `pub fn return_borrowed_alias(` to `pub fn return_slot(`, but the same change inserted `terminal_is_empty` and `original_aliases_are_returned` between them, so the slice swallowed the new `Arc::strong_count` and failed the "body has no strong_count" law. End marker is now `pub fn terminal_is_empty(` (the real end of the function), via `between`, which throws on a missing marker instead of slicing to `""`. |

Not found in any of the six: a missing fixture (all JSON corpora and schemas load) or a real production defect. I checked rather than assumed for #3: the observer is read-only, returns a `bool`, hands out no `Arc`, and the Rust code around it (`Cursor::step`, `try_unwrap`, `return_borrowed_alias`) is unchanged in behaviour. Because it is a design decision rather than a defect, it is listed in section 4.

## 2. Shared helper

`between(text, from, to)` moved from a local helper in the controlled test into the shared test-support module `♻️retirement/🧪️tests/🧫️fixtures/🟦️.ts` (exported, same code, still throws `Source marker missing: …` / `Source end marker missing after …`). The controlled test now imports it. The protocol, shared factory and sealed tests import it from there (`../../../🧪️tests/🧫️fixtures/🟦️.ts`). Verified by `bun -e`: missing start marker throws, misordered end marker throws, valid markers return the slice. `readModuleTreeSource` was evaluated but does not help here: the retirement crate root mounts these files from outside its directory (`#[path="../../🎟️tickets/🦀️.rs"]`), which the tree walker does not follow (`listModuleTree` of the crate root returns only the root), so each test reads the file that owns the item directly.

## 3. Type-check

`node --max-old-space-size=12288 node_modules/typescript/bin/tsc -p 🗑️generated/rt2-tsconfig-after.json --noEmit --pretty false` over the five touched files (the rot tsconfig with `files` replaced; it still inherits the root `include`, so the whole repo's diagnostics appear: `rt2-tsc-after.txt`, 4505 lines, exit 2, same as rot's 4513 minus the draw file).

- Controlled test: 16 diagnostics, identical to rot's count (all pre-existing `TS2769 No overload matches this call` on `applyPatch`/`readFileSync` typings), none involves `between`.
- Shared factory test: 1 diagnostic, `TS2769` at (13,272), the same `applyPatch` call that reported at (12,272) in `rot-tsc-after.txt` (one line shift from my added import). Pre-existing.
- Protocol test, sealed test, shared fixtures module: 0 diagnostics.

## 4. Not verified and hand-offs

1. Rust not compiled or run (compiler ingress). The native tests these laws mirror (`🏭️factory/🧪️tests/🦀️.rs`, `🔗️shared/🔒️sealed/🧪️tests/🦀️.rs`) were not executed.
2. Design note for the Rust owner (no change made): `SealedShared::original_aliases_are_returned` uses `Arc::strong_count`, which is racy by nature if another thread still holds an alias. Its only consumer is the job context source (`original_loans_are_returned`). If the intended invariant is "sealed values never read a strong count", the observer should be replaced by the alias-return witness (`SharedAliasReturn::terminal_is_empty`), and test #3 can then drop its exception.
3. Test #3 now encodes the exception as "exactly one `Arc::strong_count`, inside a `-> bool` function named `original_aliases_are_returned`". Renaming that function requires touching the test, which is intended.
4. The remaining name-keyed source-text reads in these files (for example `source.includes("Arc::into_inner(source)")` over a whole file, `self.issuance!=slot.issuance`) pass today but have no region guard; they are the same shape as the failures fixed here.
5. No `.vscode/launch.json` change: no test entry point was added.

## Files changed

- `🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🧫️fixtures/🟦️.ts` (new in the previous slice, edited: exported `between`)
- `🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🧪️tests/🟦️.ts` (local `between` removed, import added)
- `🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🧪️tests/📦️protocol/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️shared/🏭️factory/🧪️tests/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️shared/🔒️sealed/🧪️tests/🟦️.ts`

No title changed (the four titles above are as in `RESULT-RETIREMENT-ORACLE-TESTS.md` section 5). No `[DEBUG]` log added; the pre-existing ones in these tests were left.
