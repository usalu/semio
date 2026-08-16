# Store Group Fatal Preview Fix Acceptance

## Scope

Updated only `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`.

- `SpaceMember::validate_wire` now keeps the dry-run state local, inspects every returned message list, and returns the first fatal message before accepting its proposed next snapshot or allowing phase-two dispatch.
- `ValidatedMutation::SetN` now produces `MutationOutcome::fatal("mutation.invariant", "n must be non-negative", ["n"])` for every negative value. The fatal constructor supplies `DemoDiff::default()`; non-negative values retain the successful diff.
- The group-atomicity history assertions were not changed. Only directly stale validation prose was updated.

## Preflight

- Expected and observed HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- Expected and observed pre-edit store SHA-256: `24db6c9cd31c40e80dcc2a649c7f53a7aaebef4eb117b346ac2f71d01b8f6015`.
- The pre-existing source-only ordinary diff was `74` additions and `98` deletions; no source hunk was staged.

## Verification

The owner `test` script has no focused-test argument surface: its `TestScript::run()` ignores segments and invokes `cargo test --manifest-path Cargo.toml --lib`. The smallest command consistent with that contract was used first:

```text
cargo test --manifest-path Cargo.toml --lib dispatch_group_validate_all_atomicity_one_bad_member_applies_nothing
```

Result: passed — `1 passed; 0 failed; 879 filtered out`.

```text
bun nx run @semio-tech/framework-os-kernel:check --skip-nx-cache
```

Result: passed.

```text
bun nx run @semio-tech/framework-os-kernel:test-quick --skip-nx-cache
```

Result: passed — `880 passed; 0 failed`.

All commands emitted existing compiler warnings but no error. The focused test confirms a fatal child preview rejects the group while the unchanged assertions keep both parent and child edit histories empty.

## Final Scope Audit

- HEAD remained `0727b80aa6a802cac1760f90fb7a148f74035413`.
- Post-edit store SHA-256: `dff257c4c8b1169c32f61ec8f1b487319aa7ce558a4afd5bd4215f1f3565c915`.
- Scoped ordinary diff: `76` additions and `104` deletions, limited to the pre-existing store migration surface plus this fatal-preview correction.
- Scoped cached diff: empty.
- `git diff --check` and `git diff --cached --check` for the store source: clean.
- The existing MutationOutcome migration hunks remain present; no SPR, VCS, Cargo, Pack, DAG, renderer, stdio, or conflict/merge-policy path was changed.
