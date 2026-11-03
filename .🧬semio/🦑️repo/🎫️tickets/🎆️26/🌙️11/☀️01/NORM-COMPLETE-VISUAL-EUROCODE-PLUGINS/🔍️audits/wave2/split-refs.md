# split-refs (wave 2): `crate::<name>` references that cross a -more split

No cargo/rustc, no git writes. `freeze.txt` named only build-en1993 (untouched). Tools: `🧰️scripts/split-refs/scan.py` (read-only; mounted-module closure per crate through `#[path]`/`mod`, resolves `pub use <sibling-crate>::*` and local `pub use <mod>::*`), `move.py` (moves named mounts from base root into the -more root, appends them to the base `pub use ..._more::{..}`).

## Findings and fixes (mutation crates)
- vdi3805 `mut-geometry-more`: `remove_curve`, `remove_geometry`, `remove_geometry_connection` used `crate::add_curve|add_geometry|add_geometry_connection` (left in base). Moved the three mounts into `-more`, re-exported from the base `📐️geometry`.
- en1999 `mut-members-more`: `remove_member` used `crate::insert_member`. Moved `insert_member` into `-more`, re-exported from `📏️members`.
- en1997 `mut-investigation` and `mut-nested-rows-more` (not a split issue, stale rename): inverse files of insert/remove-ground-investigation(-reading) referenced the non-existent modules/types `insert_investigation`, `remove_investigation`, `insert_investigation_reading`, `remove_investigation_reading` (`InsertInvestigation`, ...). Renamed to the mounted `*_ground_investigation[_reading]` / `*GroundInvestigation[Reading]` in 8 files (inverse + mutation kind bounds). Field names match the structs.
- en1995 (`entity-rows` / `-more`): no remaining findings (insert_fastener fix already in place).

## Scan result after fixes
0 mutation crates with missing `crate::` modules. Remaining 25 crates are false positives outside split scope: `*-io` (macro-generated `En19xxAnalyzer/Composer`), crates with opaque globs, and macro/string-generated paths (din18599 `crate::document`, iso16757 `crate::standards`, en1997 oracle-wiring doc text, app-surface `crate::protocol` extern aliases/macros). Not changed.

## Gates
- Source-budget test: moved crates fine (en1997-mut-investigation 97%, 31 files; unchanged by the rename). 2 OVER are `en1997-model` (131%) and `en1992-model` (191%), unrelated to this task (owners' in-progress model growth); report to coordinator.
- `check-cargo-authority.py`: 31 crates, 0 errors.

## Unverified by compiler
Moved insert/add mounts now compile in -more (same dependency set as base; no `cfg(feature)` in moved files). Visibility and ambiguity of re-exports unchecked.

## Ready
`- ready for rebuild: split-refs` appended to `build-en1997/latest.md`, `build-en1999/latest.md` (vdi3805 has no runner build dir; en1993 untouched).
