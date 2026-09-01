# Fix: `semio-framework-plugin-host` native build blocker

## Root cause

Not a peer regression in `🎮️mutation/🦀️.rs` itself — that file's `Mutation<P>` trait shape
(`DESCRIPTORS`/`descriptor()`) is exactly what the OS config-schema module's own
`#[derive(dsl::Mutations)]` aggregate enums (`OpeningConfigMutation`, `MergePolicyConfigMutation`,
`IdentityConfigMutation` in `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`)
were already coded to satisfy. `dsl::Mutations`' generated impl reads
`<PayloadTy as ::semio_framework_os_kernel::MutationLeaf>::DESCRIPTOR` for every variant's payload
type. The five direct mutation leaves under `🧬️mutations/` — `SetDefaultApp`, `ClearDefaultApp`,
`ChangeMergePolicy`, `SignIn`, `SignOut` — each hand-implemented `protocol::MutationKind<P, Op>`
(which itself requires `MutationLeaf` as a supertrait) but never implemented `MutationLeaf`. That
made every `MutationKind` impl unsatisfied (E0277) and left `Mutation::diff`/`inverse` unresolved
at every call site through the aggregate (E0599), which is what the ticket's 38 errors trace back to.

This was a half-finished migration, not fresh peer breakage: each leaf directory already had a
freshly-authored `🔣️.json` mutation-leaf descriptor sitting next to the old `🦀️component.rs`
(committed, not a working-tree edit) — someone had staged the descriptor but never wired the
`#[derive(dsl::MutationLeaf)]` that consumes it, and never renamed the source file to match.

## Migrated example followed

`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/✏️rename-node/🦀️.rs`
(and its sibling leaves in the same directory) — the fully-working `dsl::MutationLeaf` +
`protocol::MutationKind` pairing, plus the aggregate
`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/🦀️.rs` using
`#[derive(..., dsl::Mutations)]`.

Key convention this exposed: `dsl::MutationLeaf`'s derive macro resolves the mutation leaf's
canonical source filename from taxonomy (`mutationComponentFileKindId` → `rust-source` fileKind →
emoji `🦀️` + extension `.rs` = **`🦀️.rs`**, never `🦀️component.rs`) — confirmed against the
repo-wide taxonomy comment at `_mutationOwnershipComment` in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: "Every concrete
`🧬️mutations/<emoji><verb>-<noun>/` directory directly owns one `🦀️.rs`...". Every other
migrated mutation leaf/aggregate in the repo already follows this; config-schema's leaves were the
one holdout still on the pre-taxonomy `🦀️component.rs` naming.

## Files changed

- Renamed (leaf sources, taxonomy-canonical filename):
  - `🧬️schema/🧬️mutations/📌️set-default-app/🦀️component.rs` → `🦀️.rs`
  - `🧬️schema/🧬️mutations/🧹clear-default-app/🦀️component.rs` → `🦀️.rs`
  - `🧬️schema/🧬️mutations/🛡️change-merge-policy/🦀️component.rs` → `🦀️.rs`
  - `🧬️schema/🧬️mutations/🪪️sign-in/🦀️component.rs` → `🦀️.rs`
  - `🧬️schema/🧬️mutations/🚪️sign-out/🦀️component.rs` → `🦀️.rs`
  - `🧬️schema/🧬️mutations/🦀️component.rs` (the aggregate dispatch enums) → `🦀️.rs`
  (all under `🧰️framework/🛍️products/💻️os/🎚️config/`)
- Edited — added `dsl::MutationLeaf` derive + `#[mutation_leaf(contract = ::protocol)]` to each
  payload struct (`SetDefaultApp`, `ClearDefaultApp`, `ChangeMergePolicy`, `SignIn`, `SignOut`) in
  the five renamed leaf files above.
- Edited — fixed two stale `🔣️.json` `aggregateVariant` fields that did not match their enum
  variant name (required by `dsl::Mutations`' compile-time assertion
  `str_eq(leaf::DESCRIPTOR.aggregate_variant, expected_variant)`):
  - `🛡️change-merge-policy/🔣️.json`: `"MergePolicySetting"` → `"ChangeMergePolicy"`
  - `🪪️sign-in/🔣️.json`: `"Identity"` → `"SignIn"`
- Edited `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📦️glue.rs`:
  updated the six `#[path = "…/🦀️component.rs"]` mount points (the mutations-aggregate module plus
  the five leaf modules) to `…/🦀️.rs`, matching the renames. The schema-level
  `🎚️config/🧬️schema/🦀️component.rs` mount (line 32, `OpeningPreferences` itself — not a mutation
  leaf) was left untouched; it isn't governed by the mutation-leaf taxonomy rule.
- Edited `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` — updated a stale doc-comment file:line reference
  (`…/🧧mutations/🦀️component.rs:15` → `…/🧬️mutations/🦀️.rs:16`) that pointed at the renamed
  aggregate file.

## Not touched

- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs` — the peer's trait file, left untouched
  per instructions. Its shape is correct and is what every other migrated mutation leaf in the repo
  already satisfies.
- The leftover `🔣️component.json` files sitting beside each leaf's new `🦀️.rs` (the pre-existing,
  now-superseded descriptor format) — not read by any Rust code path once `dsl::MutationLeaf` reads
  `🔣️.json` instead (confirmed no repo-wide references to
  `<leaf>/🔣️component.json` outside each leaf's own now-dead reads). Left in place: deleting them
  is a cleanup, not required to unblock the build, and out of this ticket's stated lane. Flagging
  for the config-schema owner.

## Verification

### `cargo check -p semio-framework-plugin-host --message-format short` — real output, completed

Ran to completion (full workspace-through-plugin-host build, ~26 min under heavy shared-lock
contention — see below). Tail of the real output:

```
    Checking semio-framework-plugin-host v0.1.0 (.../🔌️plugin/🖥️host/📦️packages/🦀️rust)
🧰️framework/.../🖥️host/📦️packages/🦀️rust/../../🧵️shard/🏃️executor.rs:339:75: warning: unnecessary qualification
🧰️framework/.../🖥️host/📦️packages/🦀️rust/../../🧵️shard/🏃️executor.rs:564:100: warning: unnecessary qualification
🧰️framework/.../🖥️host/📦️packages/🦀️rust/../../⏳️runtime.rs:279:8: error[E0063]: missing field `ui_patch_receipt` in initializer of `semio_framework::kernel::TurnResult`: missing `ui_patch_receipt`
🧰️framework/.../🖥️host/📦️packages/🦀️rust/../../🧵️shard/🦀️component.rs:195:8: error[E0063]: missing field `ui_patch_receipt` in initializer of `semio_framework_actor::TurnResult`: missing `ui_patch_receipt`
🧰️framework/.../🖥️host/📦️packages/🦀️rust/../../🧵️shard/🦀️component.rs:1730:30: error[E0063]: missing field `ui_patch_receipt` in initializer of `semio_framework::kernel::TurnResult`: missing `ui_patch_receipt`
🧰️framework/.../🖥️host/📦️packages/🦀️rust/../../⏳️runtime.rs:144:69: error[E0277]: the trait bound `AsyncActorHostState: component::actor_bindings::semio::framework::byte_page::Host` is not satisfied
🧰️framework/.../🖥️host/📦️packages/🦀️rust/../../⏳️runtime.rs:144:69: error[E0277]: the trait bound `AsyncActorHostState: component::actor_bindings::semio::framework::instance_lifetime::Host` is not satisfied
🧰️framework/.../🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:1921:64: error[E0277]: the trait bound `ActorHostState: component::actor_bindings::semio::framework::byte_page::Host` is not satisfied
🧰️framework/.../🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:1921:64: error[E0277]: the trait bound `ActorHostState: component::actor_bindings::semio::framework::instance_lifetime::Host` is not satisfied
🧰️framework/.../🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:2037:12: error[E0063]: missing field `ui_patch_receipt` in initializer of `semio_framework::kernel::TurnResult`: missing `ui_patch_receipt`
warning: `semio-framework-plugin-host` (lib) generated 6 warnings
error: could not compile `semio-framework-plugin-host` (lib) due to 8 previous errors; 6 warnings emitted
```

**Zero errors trace to config-schema or the mutation trait wave.** Every one of the 38 originally
reported errors that belonged to config-schema/`🎚️config` is gone. The 8 that remain are a
**different, unrelated, already-landed-but-unfinished wave**, entirely inside
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/` (not config-schema, not the mutation
trait):

1. **E0063 × 4** — `TurnResult { .. }` struct-literal initializers missing the `ui_patch_receipt`
   field, at `⏳️runtime.rs:279`, `🧵️shard/🦀️component.rs:195` and `:1730`, `🦀️component.rs:2037`.
   `ui_patch_receipt: Option<ActorUiPatchReceipt>` was added to `TurnResult` in
   `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:1696` (and mirrored in
   `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs:2937`) — a UI-patch-receipt feature, already
   fully committed there and in most call sites (`🚪️lifetime/🦀️component.rs:322` already passes
   `ui_patch_receipt: None`), just not yet propagated to these four plugin-host construction sites.
2. **E0277 × 4** — `AsyncActorHostState`/`ActorHostState` don't satisfy
   `component::actor_bindings::semio::framework::byte_page::Host` /
   `…instance_lifetime::Host`, at `⏳️runtime.rs:144` and `🦀️component.rs:1921` (both in the
   `actor_bindings::Actor::add_to_linker::<_, HasSelf<_>>(...)` call). This is a WIT
   component-model host-binding gap — `🦀️component.rs:1730` already has
   `impl wit_types::Host for ActorHostState {}`(3634 in `🧵️shard/component.rs` context — Actor
   bindings gained `byte_page`/`instance_lifetime` interfaces that `ActorHostState`/
   `AsyncActorHostState` haven't implemented yet.

Both are self-contained, unrelated feature waves in the actor/kernel/WIT-bindings layer, landed by
someone else, mid-migration — not touched, per the "stay in your lane" instruction; fixing them
would mean guessing at `ui_patch_receipt` semantics and the new WIT host interfaces, which isn't
this ticket's job.

### `cargo check -p semio-framework-os-mcp --message-format short` — could not complete

`os-mcp`'s own `Cargo.toml`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/Cargo.toml:82`) declares
`semio-framework-plugin-host` as a direct path dependency. Cargo cannot compile a crate whose
dependency fails to compile, so **`os-mcp` is structurally guaranteed to fail with the same 8
errors above** until they're fixed — that is a certainty of Cargo's build-graph semantics, not
a guess.

I launched the real command (`cargo check -p semio-framework-os-mcp --message-format short`,
background PID still running as of this writing) to get literal output confirming this, but this
build machine is under extreme concurrent load from other sessions right now — `ps` showed 50-60+
concurrent `cargo check`/`test`/`build` processes for over an hour, including three whole-workspace
`cargo check --workspace --all-targets --keep-going` jobs that have been alive 45-55+ minutes while
accumulating well under 1 second of CPU time each, i.e. everyone (including my own process) is
mostly just waiting on the shared `target/` build-directory lock, not actively compiling. After
waiting ~80 minutes real time the os-mcp check still had not acquired the lock. I am not reporting
this as a pass — it genuinely has not run to completion. If you want the literal transcript, the
background command is still alive; check
`/private/tmp/claude-501/-Users-ueli-Documents-semio/18531112-3c0c-4107-80d3-9756853453f8/scratchpad/check-os-mcp.txt`
(the last line will be `EXIT:0` on success) or re-run it once the machine quiets down.

### `cargo test -p semio-framework-plugin-host --lib`

Also launched; also stuck behind the same lock contention, and — since the crate doesn't compile —
guaranteed to report the identical 8 errors rather than run any tests. Not independently useful
beyond what's already shown above. config-schema's own mutation-leaf unit tests
(`label_names_role_and_dialect`, `unpinned_coordinate_inverts_to_clear`,
`already_pinned_app_is_a_warned_no_op`, the `🧪️tests/*` fixture suites, etc.) will only actually
run once the unrelated 8 errors are fixed elsewhere; I did not touch or need to touch them — their
code is unchanged and was already correct.
