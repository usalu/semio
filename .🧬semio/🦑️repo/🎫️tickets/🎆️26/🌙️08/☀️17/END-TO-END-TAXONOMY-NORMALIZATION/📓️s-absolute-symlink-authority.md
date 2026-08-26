# S-Absolute Symlink Authority

## Outcome

The stale inventory's 13 `symlink-absolute-target` findings are one bounded ticket-evidence class, not production dependencies:

- 11 links are in `26/08/03/PRINT-SOLID-HEADING-CHIP-ROW-PARITY`;
- 2 links are in `26/08/05/STALE-CONFIG-FIXES-AND-CAPABILITY-LINT-REVIVAL`;
- every stored target starts with the current repository's absolute root;
- every lexical target is therefore provably repository-local after stripping that exact root;
- all 13 target nodes are currently absent, so the links are already historical broken-link evidence rather than runnable inputs.

The deterministic normalization is to preserve each link and its logical repository-relative target while replacing the machine-specific absolute target text with the relative path computed from the link's final parent to the target's final logical path. A target need not exist for this evidence-preserving rewrite; escaping the repository, targeting an opaque prefix, or failing current-target preimage validation must block the plan.

## Transaction requirement

Symlink target changes must be first-class frozen plan operations with:

- source link path and final link path;
- exact old target text;
- exact new relative target text;
- source-target digest and operation identity;
- final-path move-map resolution for both link and target;
- preflight rejection on target-text drift;
- staging, rollback, cancellation, resume, and affected-state digest coverage;
- an empty second plan after apply.

They must not be implemented as textual file edits, must not dereference the links, and must not be silently allowlisted. Absolute targets outside the repository remain errors. A relative target whose lexical resolution enters an opaque prefix remains an error.

## Evidence

A read-only `readlink`/`lstat` census inspected link identity only and never followed a link. It returned 13/13 repository-local lexical targets, 0/13 existing target nodes, and 0/13 opaque targets. Compose and `temp/compose` were not read. No Git or workspace state was modified.
