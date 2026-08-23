# P8 Child Snapshot Retirement Domain Bindings

Date: 2026-08-22

Verdict: **RED — domain cursor implementation present; shared ownership and construction hooks remain acceptance blockers.**

## Scope

Production changes are confined to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️component.rs`.
The implementation supplies explicit factory impls and install arms for all 18 `SemioMembers`
variants. The bounded cursor structurally dismantles every reachable owned field: recursive
`DocBlock`, `DrawNode`, and `SemioValue`; every nested `Vec`, `String`, media byte buffer,
kit child/link handle, artifact reference, and schema root. Empty vectors release their allocation
only after their items have been cursor-retired; strings transfer to byte buffers and truncate
under the byte grant. No thread, unsafe code, `mem::forget`, `ManuallyDrop`, background
deferral, or quarantine is used in this production cohort.

`close_step` uses `maximum_items` as exact transition fuel and never reports item/byte progress
above the grants. A zero item or byte grant remains resumable. `Complete` is returned only when
both the captured Arc slot and cursor stack are empty; `terminal_is_empty` witnesses the same
state. A non-unique Arc returns truthful `Blocked` while retaining its exact Arc, preventing a
cloned reader from becoming the last owner after this disposer reports terminal.

## Tests Added

- large nested text snapshot retires over multiple bounded turns and reaches terminal empty;
- zero-grant cancellation/app-close pump preserves resumable ownership;
- cloned public snapshot read reports `Blocked`, then resumes only after the clone releases.

The existing create/open envelope test now exercises installation through both intended public
wrappers. The match table and factory list statically bind all 18 production variants.

## Commands and Results

`rustfmt --edition 2024 <component.rs>`

- PASS.

`rustfmt --edition 2024 --check <component.rs>`

- PASS.

`bun nx show projects | rg 'stdio|plugin'`

- PASS; resolved `@semio-tech/stdio-plugin`.

`bun nx show project @semio-tech/stdio-plugin`

- PASS; resolved `test-quick`.

`bun nx run @semio-tech/stdio-plugin:test-quick`

- BLOCKED before compiling the stdio cohort by concurrent shared-source failures.
- `semio-framework-ui-contract` fails with E0382 at `text_edit.rs:88` and `:120`.
- `semio-framework-os-kernel` fails with concurrent incomplete shared factory-field work:
  E0560/E0609/E0063 around `snapshot_retirement_factory`, plus unrelated generic Debug bounds.
- No stdio test result is claimed.

Source audit:

- cohort JSON has 18 production variants plus one test variant;
- production factory list: 18 snapshot types;
- production install match: 18 `SemioMembers` arms;
- `rg 'mem::forget|thread::spawn|ManuallyDrop|unsafe' <component.rs>`: no matches.

## Required Shared Repairs

### Generated construction bypass

`dsl::space_members!` generates public
`<SemioMembers as dsl::MemberFactory>::create/open`. Those methods call shared
`create_member_store/open_member_store` directly, bypassing
`install_semio_snapshot_retirement`. The intended
`create_semio_member/open_semio_member` wrappers install correctly, but the generated UFCS path
cannot be sealed from this cohort without duplicating every `SpaceMember` delegation.

Recommended minimal shared macro hook:

```rust
space_members! {
    retirement_hook = install_semio_snapshot_retirement;
    pub enum SemioMembers { /* variants */ }
}
```

Both generated factory methods should construct the enum, call
`retirement_hook(&mut member) -> Result<(), VcsError>`, then return it. No default hook or blanket
factory should exist; omission must preserve fail-closed behavior.

### Public cloneable Arc read leases

`SnapshotRead<T>` and `ErasedSnapshotRead` are public cloneable Arc capabilities. Retaining the
disposer Arc and reporting `Blocked` prevents a surviving clone from becoming the last owner, but
it can also permanently block app close while the live store itself retains the same Arc. Dropping
the disposer Arc instead would merely move the unbounded last-drop hazard to another reader/store.
Therefore global bounded retirement and app-close compatibility cannot both be proven with the
current contract.

Acceptance requires either non-clone scoped/registered read leases whose final release is routed
through the retirement authority, or a paged snapshot representation whose last-owner destructor
is definitionally bounded. The current cohort must remain RED until that shared ownership repair
and the macro hook land and the focused Nx test reaches this crate.

The activation gate was not changed and Phase 8 acceptance is not claimed.
