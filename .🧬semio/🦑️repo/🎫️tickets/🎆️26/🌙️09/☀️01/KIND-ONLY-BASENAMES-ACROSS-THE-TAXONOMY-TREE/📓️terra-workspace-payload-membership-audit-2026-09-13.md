# Workspace Payload Membership — Independent Audit

## Status

Read-only current-source review. **Accepted for generic payload membership and observed-change protection.** Root separately executed the registered workspace write; this audit did not run it. The completed controls are not an atomic filesystem compare-and-swap guarantee.

## Current Decision

`📚️library/🗂️workspaces/🟦️.ts` now derives nested-payload suppression only from a nearest ancestor package with the same declared name and a concrete physical `exports` target below that ancestor. It no longer special-cases `pkg` or tests for Cargo metadata.

The decision is bounded correctly:

- `exportTargets` traverses nested condition and subpath values, so direct, `types`, `import`, and explicit subpath exports can each establish authority.
- `ownsPayload` admits only `./` targets with no path separators, traversal, wildcard, or URI/special characters. It checks every target path segment using the discovery operation and accepts only physical directories ending in a regular file.
- discovery stops at the nearest manifest ancestor. A differently named or independently declared intermediate package therefore prevents a farther ancestor from suppressing the nested package.
- `main` without `exports` intentionally has no payload authority. A directory literally named `pkg` is ordinary domain structure.

The 20 schema-first cases cover direct, anonymous output, conditional, subpath, independent-name, duplicate, prefix/traversal/wildcard, nearest-boundary, missing final entry, final-entry symlink, nested-independent, `main`-only, `pkg`, intermediate-directory symlink, default-before-later-condition, mixed subpath/condition map, and numeric-condition cases. The portable test cross-checks declared conditional/subpath targets through installed `resolve.exports`, inventories payload files through `fast-glob` with `followSymbolicLinks: false`, then compares its expected package list with `computeWorkspaces`.

## Historical Finding — Ancestor Symlink Admission, Resolved

The earlier control covered only a final-file symlink. It did not prove the no-follow boundary for an intermediate target segment.

The completed hostile uses a physical same-name candidate payload whose intermediate exported directory links outside the private root. Discovery admits the physical candidate, and `ownsPayload` reaches the linked segment and rejects authority through `lstat`. A payload directory that is itself a link is intentionally not the control: discovery excludes it before duplicate-identity admission.

## Acceptance Boundary

Conditional export authority, physical regular-file final targets, no `main` fallback, no `pkg`/Cargo heuristic, and the full intermediate no-follow boundary are accepted for the reviewed generic source. The source route is current. This audit makes no live-root membership or publication claim.


## Final Current Evidence and Resolution

The earlier ancestor-link finding is resolved with the correct physical shape: the candidate payload directory remains real, while an intermediate exported directory under it links outside the private root. Discovery therefore admits the candidate and `ownsPayload` reaches that linked segment, whose `lstat` state rejects authority. A payload directory that is itself a link would be excluded by discovery and could not exercise duplicate-identity admission. The fixture now has **20** payload cases, adding this outside-root intermediate-link vector, default-before-later-condition rejection, mixed subpath/condition-map rejection, and numeric-condition rejection.

The parser now stops condition evaluation at `default`, rejects ambiguous mixed subpath/condition objects and numeric condition keys, retains explicit concrete regular-file targets only, and uses segment-by-segment no-follow checks. Conditional `types`/`import` and subpath vectors continue to compare through installed `resolve.exports`; payload file inventories continue to use `fast-glob` with symbolic-link following disabled.

Current executor evidence is green: direct source **9 tests / 170 assertions / 1.221 s**, then isolated uncached Nx source **9 / 170**, **1.4 s critical / 1.6 s target**, cache skipped. The existing physical `computeWorkspaces` group is green at **8 / 146**, including a live read-only discovery. That live comparison observes **123 expected versus 88 current entries, 35 missing and 0 stale**. Two Actor/Puzzle compiler-only identities still need stable authored wrappers before a reviewed root-manifest publication. This acceptance therefore covers the generic membership decision and its controls, not a live root `package.json` update.

## Observed-Change Protection Refresh

The publisher now repeats both root package document read and physical-kind inspection immediately after discovery, before either check reporting or a write. Fixture controls mutate the document or replace it with a non-file during discovery and prove the exact changed-during-discovery error with zero writes. That protects the observed read/discovery interval only.

The current isolated Nx source route passed **10 tests / 176 assertions / 0 failures**: Bun 826 ms, target 1.2 s, critical path 967 ms, cache skipped. Root then executed the registered workspaces-write path: 88 entries became 123, adding 35 and removing none, while an independently parsed before/after comparison found every unrelated root package field unchanged. Nx took 9.2 s with a 9.0 s critical path. The registered uncached workspaces-check then confirmed 123 fresh packages in 3.8 s (3.7 s critical, cache skipped). There is no lock, atomic rename, or filesystem CAS claim.
