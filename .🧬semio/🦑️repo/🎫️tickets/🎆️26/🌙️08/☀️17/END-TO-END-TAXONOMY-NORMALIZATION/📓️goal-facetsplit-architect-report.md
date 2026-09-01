# Architect Facetsplit — Report

Scope: `✏️s/🔌️plugins/🏛️architect` only, 266 inlined mutations.

## Predicate
Before: 266/268 inlined (behavioral predicate: bare `pub fn diff`/`inverse` with no sibling facet dir).
After: **0/268 inlined**. Coordinator's independent repo-wide sweep confirms 🏛️architect at 0.

## Method
For each of 266 mutation dirs: restored `🔺️diff/🦀️.rs` and `↩️inverse/🦀️.rs` from pinned commit
`bb06c41f73`, stripping the obsolete `mutation::` qualifier (struct now lives directly in the
leaf, not a `🦠️mutation` submodule — verified byte-identical to current inline bodies modulo
that qualifier, 266/266). Leaf `🦀️.rs` pruned of the free fns; `MutationKind` impl delegates via
`super::diff::diff` / `super::inverse::inverse` (266/266, zero stragglers). Unused leaf imports
pruned. `glue.rs` rewired with 266 `pub mod diff;` / `pub mod inverse;` `#[path]` mounts,
matching the vcs `add-tag` exemplar's order.

## Near-miss
Manually retyping an emoji path segment during one glue.rs edit produced `🏅️标准` (Chinese
"biāozhǔn") instead of `🏅️standards` in 3 mount blocks. Caught immediately via grep before
verification, fixed via scripted string replace. Automate this next time — never hand-retype
emoji path segments; always derive them from an existing matched string.

## Build — NOT verified end-to-end
`cargo check -p semio-s-plugin-architect` did not complete. Two synchronous attempts (9 min,
then 4m40s) blocked on `Blocking waiting for file lock on build directory` (shared target dir,
concurrent sibling sessions). One earlier background run did finish and produced real output:
it fails downstream in `semio-s-plugin-stdio` (a dependency of architect) — "error: could not
compile `semio-s-plugin-stdio` (lib) due to 65 previous errors" (codes E0046, E0425, E0599).
All warnings/errors in that output point at `✏️s/🔌️plugins/🗄️stdio/...` (bmp/svg/xml/gltf
mutations) — zero hits on `🏛️architect` paths. Per coordinator, stdio is another worker's slice
(repo-wide sweep: 16 remaining, all stdio). Not claiming green — this is unverified by a full
build.

## Evidence that does exist
`rustfmt --edition 2021` clean (exit 0, no diff) across all 798 touched files + `glue.rs`.
Brace balance in `glue.rs`: 346/346. Cross-checks: 266/266 delegate calls correct, 266/266
struct-name matches between leaf and facets, 0 leftover inline `diff`/`inverse` fns, 0 triple-
blank-line artifacts.

## Files
Scripts: `📜️goal-facetsplit-architect-restore.py`, `-glue.py`, `-predicate.py` (ticket root).
Temp lists: `🗑️temp/goal-facetsplit-architect-*.txt`.
Touched: 266 leaf `🦀️.rs` + 532 new facet `🦀️.rs` under
`✏️s/🔌️plugins/🏛️architect/🗿️artifacts/...`, plus
`✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📦️glue.rs`.
