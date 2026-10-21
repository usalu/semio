# Result pets-fix (executor pets-fix, wave 1021a): GREEN

Crate `semio-framework-pets-staging-draft` (395 -> 0 errors) and its direct dependents `semio-framework-pets-staging` (697 errors on the first dependent check, same defect) and the facade `semio-framework-pets`. Lane `petsfix`, `bun repo cargo --store=shared`, `nice -n 10`, load 5 to 23.

## Peer check
No peer edit in the crate or its closure in the last 30 min. Only `🧗️climbing/🦀️.rs` (traversal crate, a one-line `WALL_LIP` import, 07:11) was touched by a peer; it is a dependency, compiles, and was left alone.

## Root cause
Not a visibility-only carve. Commit 696 (`bd9e4e02bd0`) grew `🚶️locomotion/🦀️.rs` from 332 to 1,446 lines without ever compiling it: the header imported only the 332-line version's names, so `extent_at`, `claim_of`, `released`, `feel`, `MARGIN`, `GRIP_HANG`, `LADDER_*` ... (about 200 names) were never imported. Commit 697 carved it into 9 children (`use super::*` + type imports only) and 698 made the carve `pub`. No function was lost: every missing name exists in `pets-clearance`, `pets-traversal` (`climbing`, `swing`), `pets-character` (`feeling`, `behavior`), `pets-kinematics`, `pets-interaction`, or the draft crate itself. The 21 E0364/E0365 errors were `pub use` of `pub(crate)` items in the locomotion root.

## Fix (root, no compat re-exports)
1. Explicit `use` lines in 13 files of the draft crate (draft, spacing, schedule, attention, locomotion root + 9 children), generated from a symbol index and resolved by the compiler (`pets-fix-tools/addimports.py`, `resolve.py`).
2. Public API widened where `pets-staging` (and its unit tests) call it: all locomotion root re-exports are `pub use` over `pub` definitions (`Plan`, `Outing`, `guarded` included, `#[cfg(test)]` import dropped); `Draft`, `Room`, `Launch` and their fields `pub`; `Post`, `Prospect` `pub`; `WARMUP`, `SHY_OPACITY`, `clips_of` `pub`; 18 further fns (`astir`, `stand`, `enter`, `carve`, `beat_of`, `facing_to`, `trick_of`, schedule and attention helpers) via `widen.py`.
3. Same import fix in the dependent `pets-staging` parts (sociability, choice, population, clock, projection, stage) and in the 5 unit-test modules living there; draft-crate paths are `semio_framework_pets_staging_draft::...`.
4. One test fix: `ticked(ticks as u64)` in the sociability unit test (`Ticks` is `i64`).
5. Removed 12 unused imports; stale doc line "everything is `pub(crate)` at most" updated in the 5 draft-crate modules. Zero warnings remain in both crates.

## Evidence
- `check -p semio-framework-pets-staging-draft -p semio-framework-pets-staging --lib --bins --tests`: rc 0, 0 errors, 0 warnings.
- `check -p semio-framework-pets --lib --bins --tests`: rc 0.
- `test --no-fail-fast` for `-staging-draft` + `-staging`: 36 passed, 0 failed (all in `-staging`; the draft crate has no own tests). `test -p semio-framework-pets`: ok (0 tests).
- Source boundary: draft crate 2,838 lines / 165 KB / 16 files (budget 4,000 / 256 KB / 32); staging parts 2,098 lines.

## Notes for others
- The root `Cargo.lock` was stale (`semio-framework-value-derive-resources` missing, created by a value-family peer); one `update --workspace --offline` added only that package. If `cargo fetch --locked` fails on the root workspace again, that is the lock group, not this crate.
- `pets-staging` modules still say "pub(crate) at most" (true there). Open: the stage-trace subject adapters (`🧪️tests/🎪️stage-trace`) were not compiled here.
- Tools: `C/pets-fix-tools/` (`chk.sh` lane wrapper, `addimports.py`, `widen.py`). Logs deleted.
