# EN 1995 SQLite snapshot tests (2026-11-24)

## Verdict
en1995-specific. The framework (`sqlite-snapshot`) and the norm registry admission helpers are correct; they refused stale en1995 inputs. No framework file changed.

## Root causes (all en1995)
1. "duplicate authored IEEE scalar column": `MEMBER_FLOATS` in `en1995/.../🪶️sqlite/📸️snapshot/🦀️.rs` listed column 29 twice (21 entries for the 20 authored member floats, `b_m` .. `bridge_crowd_per_m2`). Removed the duplicate.
2. "norm native text required": the handwritten admission (`📸️snapshot/🛂️admission/🦀️.rs`) read schema enum slots with `Text(n)`. Native owners carry enums as `F::Enum(index)`, so `text()` refuses them. Now `Enumeration(n, LIST)` for: document `ulsEquation`; member `strengthClass`; member action `kind`, `category`, `loadDuration`; connection `fastenerType`, `strengthClass`; connection action `kind`, `loadDuration`; panel `strengthClass`. Lists are schema-ordered consts (`ULS`, `CLASSES`, `KINDS`, `CATEGORIES`, `DURATIONS`, `FASTENERS`).
3. Test constants stale vs the committed semantic contract (`🎛️semantic.json`: 20 tables, widest `en1995_member` = 70 columns): `max_tables:5/4` -> `20/19`, `max_columns:73/72` -> `70/69` (exact and one short; strictly tighter, nothing weakened).
4. Test plugin identity: builder used plugin `norm` for artifact kind `s.norm-eurocode-materials.en1995` ("artifact identity is not owned by the declaring plugin"). Now `norm-eurocode-materials` / `semio:norm-eurocode-materials`, as en1999 does.

## Independent cross-check (Python sqlite3)
`en1995-sqlite-oracle.py` (this folder; run `python3 -I en1995-sqlite-oracle.py <snapshot dir>`): executes the committed `🗄️.sql` in Python `sqlite3`, reads `PRAGMA table_info` for every table and asserts equality with `tableWidths` of `🎛️semantic.json` (20 tables, max width 70); asserts each admission enum const equals the JSON schema enum and every SQL `CHECK ... IN (...)` domain. Output: `tables 20 max width 70` / `admission enum lists match the JSON schema enums and every SQL CHECK domain`. The Rust suite additionally cross-reads exported bytes with bun:sqlite (`independent_sqlite_joins_and_domain_edits`, passes).

## Results
`semio-s-artifact-norm-en1995` (bare `repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1995 --offline`):
- `test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.64s`
- doc tests: `test result: ok. 0 passed; 0 failed; ...`

## Changed files (under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/`)
- `🦀️.rs` (MEMBER_FLOATS duplicate)
- `🛂️admission/🦀️.rs` (enum slots, consts)
- `🧪️tests/🦀️.rs` (plugin identity, table/column limits)
- ticket: `en1995-sqlite-oracle.py`; `🗑️generated/` holds logs only.

## Sibling comparison
en1998 and en1999 sqlite tests were not re-run: nothing shared changed, and their sources show the correct patterns en1995 now follows (en1999 uses plugin `norm-eurocode-materials`, `max_tables` 25 / `max_columns` 34 equal to its semantic contract; en1998 declares enums via `Enumeration`). A queued sibling run was cancelled because compile headroom queueing exceeded 15 min.
