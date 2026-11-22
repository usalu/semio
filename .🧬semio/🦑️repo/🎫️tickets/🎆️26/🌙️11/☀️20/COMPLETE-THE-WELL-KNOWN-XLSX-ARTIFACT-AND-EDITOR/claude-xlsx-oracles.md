# 🔭️ XLSX Oracle Gaps Closed

Base = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base`. Oracle crate tests were run through `repo cargo --store=shared --configuration=prototype -- test --manifest-path .../📕️xlsx/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`. Result: 26 passed, 2 failed. The 2 failures are the strict and transitional `every_declared_kind_is_observable_and_its_inverse_restores_the_package` ("no test parameters for kind set-relationship-base"). They were not touched by this work and were already failing.

## 1. insert-cell
- Added: oracle arm `insert-cell` plus its inverse (`🔮️oracles/🧮️mutation-dispatch/🦀️.rs`), a vacancy resolver `cell_address::addressed_vacancy` (`🔮️oracles/🔢️grid-and-cell-addressing/🦀️.rs`; row 1-based, column 0-based, refuses an occupied coordinate).
- Fixture: `🧪️tests/🧫️fixtures/➕️insert-cell-applied/{⬅️before,➡️after}.xlsx` (class `handcrafted`: the before is the committed openpyxl package of set-cell-applied, the after adds row 2 and widens the dimension), evidence row `insert-cell-applied` in `🔮️oracles/🔣️.json`.
- Feature rows: `insert-cell` in the mutate (differential) and inverse Examples of `🧪️tests/🔀️mutate-xlsx-ecma-376/🥒️.feature`, on the real workbook (sheetData revision `ecd99b57eb5d4205`, A52 = 42). The revision came from a Python port of the subject hash that reproduces the two known cell revisions.
- Tests (all green): `insert_cell_fills_a_vacancy_of_the_real_workbook_and_is_undone` (new row and existing row, read through calamine, inverse), `insert_cell_refuses_what_is_not_a_vacancy`, pair `insert-cell-applied`.
- Also fixed: `read_workbook_grid` used calamine's `used_cells` relative positions as absolute; it now adds the range origin.
- NOT run: the feature rows on the subject side (`repo test` on the feature file printed nothing, and the project graph is unpublished).

## 2. Shared strings
- Added: `calamine_shared_string_law` (`🔮️oracles/🔭️third-party-witness/🦀️.rs`). calamine resolves cells through the pool it parses itself. `set-shared-string` must change exactly the resolved cells that carried the old text; insert and remove of an unreferenced entry must change none. Wired in-role into the oracle `mutate` and `inverse` handlers and into the subject `mutate` handler (`🧪️tests/🔀️mutate-xlsx-ecma-376/🦀️.rs`). The `set-shared-string-applied` pair is also compared through calamine.
- Remaining: an unreferenced pool entry, the thing insert and remove move, is invisible to calamine (private table). Only the zip + quick-xml reader sees it. openpyxl would, but is not installable offline.
- Subject-side law and handler wiring compile only under the `sut` feature and were not executed.

## 3. Relationship and content-type
- Added: table `🧪️tests/🧫️fixtures/🔭️third-party-pairs/🔣️.json` (language-agnostic) mapping all 20 committed third-party pairs (13 plumbing, 7 grid or shared-string) to their wire mutation, and `the_oracle_reproduces_every_third_party_pair` (`🔮️oracles/🧪️tests/🔭️third-party-pairs/🦀️.rs`). The oracle's plumbing result must equal the jszip + fast-xml-parser written `after`, and rejected pairs must be refused. Green for all 20.
- Caveat: the projection reading both sides is still the oracle's own zip + quick-xml reader. The independence is in the producer of the `after`, not the reader.

## 4. calamine-oracle suite, openpyxl, jszip
- Chosen: implement, not remove. The declarations were not false: openpyxl and jszip wrote the committed pairs. But neither was replayed (openpyxl is not installable offline, the jszip script cannot resolve jszip from its ticket folder, verified by running it). The oracle rationales now say so, and the Rust pair differential is what runs.
- Suite: `🔮️oracles/🧪️tests/🔭️calamine-oracle-suite/🦀️.rs`, test `the_calamine_oracle_suite_covers_every_declared_fixture`. It asserts the artifact definition's fixture list equals the covered list, then reads each of the five fixtures through calamine: reuse-marketplaces (sheets, 612 and 18 cells), mutations (the pair table), canonical-xml-save (calamine cases, before and after values and kinds), native-document (implicit coordinates), viewer-cell-window (window). Green.
- `📜️artifact-definition.json`: suite status `unimplemented` to `implemented`.

## Files
Edited: `🔮️oracles/🦀️.rs`, `🔮️oracles/🧮️mutation-dispatch/🦀️.rs`, `🔮️oracles/🔢️grid-and-cell-addressing/🦀️.rs`, `🔮️oracles/🔣️.json`, `🧪️tests/🔀️mutate-xlsx-ecma-376/🥒️.feature`, `🧪️tests/🔀️mutate-xlsx-ecma-376/🦀️.rs`, `📕️xlsx/📜️artifact-definition.json`.
Created: `🔮️oracles/🔭️third-party-witness/🦀️.rs`, `🔮️oracles/🧪️tests/🔭️third-party-pairs/🦀️.rs`, `🔮️oracles/🧪️tests/🔭️calamine-oracle-suite/🦀️.rs`, `🧪️tests/🧫️fixtures/🔭️third-party-pairs/🔣️.json`, `🧪️tests/🧫️fixtures/➕️insert-cell-applied/*`; in the ticket: `claude-xlsx-address-revision.py`, `claude-gen-xlsx-oracle-evidence.py`.
