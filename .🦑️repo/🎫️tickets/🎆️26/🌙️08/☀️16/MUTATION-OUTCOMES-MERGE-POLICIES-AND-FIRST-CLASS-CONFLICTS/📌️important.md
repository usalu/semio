# NOTICE TO OTHER LIVE SESSIONS — breaking change in flight

> Ticket `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`.
> This file is cleared by the coordinator immediately before `ticket_close`.

## What is changing, atomically

`Mutation::diff` and `MutationKind::diff` **no longer return the diff directly**. They now return
`protocol::MutationOutcome<Diff>` — the diff plus a `Vec<MutationMessage>` at levels
`Info | Warning | Error | Fatal`. There is deliberately **no compatibility layer and no staging
method** (CLAUDE.md forbids compat layers), so the change lands in one piece.

Also landing in the same window:

- **`validate` is deleted** from `Mutation`, `MutationKind` and `CompositeMutationKind`, and every
  leaf override of it is removed. Its checks move into the `🔺️diff` leaf as `Error`/`Fatal` messages.
- **The CRDT layer is deleted**: `📡️spr/🔀️crdt/**`, `MergeStrategyKind`, `ConflictRule`,
  `merge_concurrent_diffs`, `db_conflict::ResolutionPlan`, `assert_crdt_commutative/idempotent`.
  It was unreachable from the store path and contradicted CLAUDE.md ("MUST NOT use CRDTs").
- **`Severity::Hint` becomes `Severity::Info`** repo-wide (Rust, TS mirrors, WIT enum, golden
  fixtures), and the variant order is reversed so `derive(Ord)` is the level order.
- `Mutation::{merge_strategy, conflict_rule, reconcile}`, `ReconcileReport`, `ReconcileSeverity`
  and `SpaceConflict` are deleted. `MutationDescriptor` loses `conflict_rule`.
- `CHANNEL_VERSION` goes **10 → 11** (once, by our lane 1-C). New `AppCommand` tags 30/31/32, new
  `AppFrame` tags 23/24, new `ArtifactCommand` ordinals 15/16. All appended — no existing tag moves.

## The one-line adaptation, if a `diff` leaf of yours goes red

Wrap what you already return:

```rust
// before
pub fn diff(payload: &Payload, base: &Artifact) -> ArtifactDiff { ... }
// after
pub fn diff(payload: &Payload, base: &Artifact) -> protocol::MutationOutcome<ArtifactDiff> {
    protocol::MutationOutcome::new(/* the diff you already computed */)
}
```

That is enough to compile. Messages are optional at that point — our W3 fan-out lanes add the proper
`Error`/`Warning`/`Fatal` codes per verb family afterwards, so **you do not need to add messages**;
just get the type right, or leave the leaf to us.

If you had a `fn validate` override on a mutation, **delete it** — do not try to keep it working.

## Expected red window

Plugin crates under `✏️s/🔌️plugins/` are **expected to fail to compile** between our W0 barrier and
the moment their W3 fan-out lane lands. That is by design, not a regression. `🧰️framework/` is kept
green at every barrier. If you see a plugin crate red, check `📓️w3-*-report.md` in this ticket
folder before investigating.

Before blaming us for any failure: `git log --date=iso -- <file>`. Commit *message* dates in this
repo are a frozen fake template and must never be parsed.

## Regions we hold

See `📋️ownership-and-handoffs.md`. Short version: `📡️spr/{🎮️command,⚔️conflict,📜️history,🧵️channel,🧪️testkit,🧾️wire/🔖️Policies}`,
`🏪️store` (`🔖️ArtifactStore`/`🔖️Authority`/`🔖️Schemas`/`🔖️Backbone`/`🔖️Composition`),
`🗣️dsl/{⚠️diagnostic,✨️derive}`, `🌿️vcs`, `🔌️plugin` (Emit/Exchange/plugin_runtime, additive
`preview` on `VcsArtifactApp` — **not** `🔖️Surfaces`), `🖥️host`, `🏃️run`, `💻️os/🟦️component.ts`,
`🎠️kernel/🟦️component.ts`, the React shell elements listed there, `🛢️db`, `🌎️hub`, and — coordinator
only — root `📜️script.ts`, `.vscode/launch.json`, `📋️project.json`.
