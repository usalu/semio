# Carve notes: semio-s-artifact-norm-en1993-checks

## Cause
Real closure (`readRustSourceClosureV1`, cfg `--test`): `lib[cfg(test)]` was 21 files / 312208 B / 5329 active lines (limits 32 / 262144 / 4000). Bytes and lines were over; the build unit (10 files / 131 KB / 2159 lines) was fine.

## Split
New tests-only crate `semio-s-artifact-norm-en1993-checks-laws` at `⚖️checks/🧪️tests/⚖️laws/` (root `🦀️.rs`, `#![cfg(test)]`, `pub use semio_s_artifact_norm_en1993_checks::*;` so `crate::clauses`, `crate::check_full_steel_structure` etc. keep resolving). It `#[path]`-mounts the two largest suites without moving them: `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` and `⚖️checks/🧪️tests/🔭️special-wiring/🦀️.rs`. The checks lib no longer mounts them. `fixture_path!("../../../../..")` in the compliance suite became seven levels (`../../../../../../..`) because the manifest directory is two levels deeper; it still resolves to the family `✳️any` root.

## Files
- changed: `⚖️checks/🦀️.rs` (two mounts removed), `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` (3 fixture_path depths), family `Cargo.toml` (member + workspace dependency), family `📦️packages/🦀️rust/📜️script.ts` (crate list), `Cargo.lock` (cargo)
- new: `⚖️checks/🧪️tests/⚖️laws/🦀️.rs`, `⚖️checks/🧪️tests/⚖️laws/📦️packages/🦀️rust/Cargo.toml`

## Static after (real reader)
- checks lib[cfg(test)]: 18 files / 233706 B / 3795 lines; lib 10 / 131084 / 2155
- laws lib[cfg(test)]: 4 files / 78707 B / 1541 lines

## Test counts
Before 147 test fns (names-before.txt); after 62 + 85 = 147, sorted name multisets identical (checked). No test macros in this crate.
