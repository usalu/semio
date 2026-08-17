# Worker brief — resume waves (read this before touching anything)

Ticket folder (`$T`):
`/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`

Read `$T/📋️contract-freeze.md` (frozen contract C1–C10, the 7 message codes, the verb-family table) and
`$T/📋️master-plan.md` §"Fan-out recipe" before editing. `/Users/ueli/Documents/semio/CLAUDE.md` is binding.

## The landed API (verified in the tree — code against THIS, not against prose)

`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`, in-crate alias `protocol`:

```rust
pub trait MutationDiff<P> { fn apply(&self, base: &P) -> MutationApplyResult<P>; fn absorb(&mut self, other: Self); }
pub trait Mutation<P>    { type Diff; fn diff(&self, base: &P) -> MutationOutcome<Self::Diff>; fn inverse(&self, base: &P) -> Vec<Self>; }
pub trait MutationKind<P, Op> { fn diff(&self, base: &P) -> MutationOutcome<<Op as Mutation<P>>::Diff>; fn inverse(&self, base: &P) -> Vec<Op>; fn label(&self) -> String; fn target(&self) -> Vec<String>; }

MutationOutcome::<D>::new(diff)                                  // silent success
MutationOutcome::<D>::empty()                                    // D::default(), no messages
MutationOutcome::<D>::error(code, message, target_iter)          // EMPTY diff + Error message
MutationOutcome::<D>::fatal(code, message, target_iter)          // EMPTY diff + Fatal message
outcome.info(code, message) / .warn(code, message)               // chainable, keeps the diff
outcome.absorb_messages(msgs) / .stamp_op_index(i) / .worst_level() / .is_applicable(policy)
outcome.into_parts() -> (D, Vec<MutationMessage>)   outcome.diff() -> &D
MutationMessage::{info,warn,error,fatal}(code, message).at(targets).at_op(i)
```

`validate` no longer exists on `Mutation`/`MutationKind`/`CompositeMutationKind` — delete every override.

**The only 7 legal codes** (a gate will fail on anything else):
`mutation.target-missing` (Error, empty diff) · `mutation.no-op` (Warning, empty diff) ·
`mutation.partial` (Warning, survivors only) · `mutation.clamped` (Warning, non-empty) ·
`mutation.duplicate-id` (Fatal, empty) · `mutation.invariant` (Fatal, empty) ·
`mutation.cascade` (Info, non-empty).

## Reference implementation (already converted — copy this shape)

`✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/` —
look at `🌱create-node/🔺️diff`, `🗑️delete-node/🔺️diff`, `🔗️connect-nodes/🔺️diff` and the facet's
`🧪️Tests` region. Also `✏️s/🔌️plugins/📐️cad` and `💠️lowpoly` are fully converted.

## Per-leaf recipe

1. `🔺️diff/🦀️component.rs`: signature becomes `-> protocol::MutationOutcome<XDiff>`; wrap the success
   path in `MutationOutcome::new(..)`; add the real detection per the verb-family table (target
   missing ⇒ `::error`, idempotent ⇒ `.warn("mutation.no-op", ..)` with an EMPTY diff, cascade ⇒
   `.info("mutation.cascade", ..)`, duplicate id / invariant ⇒ `::fatal`).
2. `🦠️mutation/🦀️component.rs`: only the `fn diff` return type changes (it delegates); delete any
   `fn validate` override, moving its check into the diff leaf as Error/Fatal.
3. `↩️inverse` leaves: unchanged.
4. Hand-written `impl Mutation<P> for XMutation` (config/presence/legacy dispatch enums): change the
   `diff` return type and wrap; if the match arms delegate, fold their outcomes.
5. Call sites: `.diff(base)` now yields an outcome — use `.into_parts().0` or `.diff()`; `apply`
   returns `MutationApplyResult<P>`, so `?`/`expect` it.
6. Facet `🧪️Tests` region: add `assert_missing_target_is_error` and `assert_fatal_never_applies`
   (from `protocol::testkit` / the spr testkit) for one representative kind per verb family.

**Never invent a new code. Never leave a bare `MutationOutcome::new(..)` where the verb-family table
says a message is required. Never widen a diff's meaning to make a test pass.**

## Rules (binding on every lane)

- Write ONLY inside your lease. If you must touch a file outside it, STOP and report instead.
- Shared live tree: re-read a region immediately before editing; edit region-locally with `Edit`;
  never whole-file `Write` over an existing file; never revert/reformat foreign changes. Attribute a
  suspicious failure with `git log --date=iso -- <file>` before blaming yourself (commit *message*
  dates are a frozen fake template — only `--date=iso` is real).
- NEVER run a modifying git command (commit/stash/checkout/restore/reset/add). NEVER use worktrees.
- NEVER call `ticket_close` / `ticket_reopen` / `ticket_open`. The coordinator closes the ticket.
- All scratch/logs go inside `$T` as `.txt` (never `.log` — repo-wide gitignored).
- Validate every claim: never say a check/test passed unless you ran it; paste the real counts into
  your report. If a crate is blocked by an error outside your lease, say so and name the file:line.
- `cargo` may block on the workspace build lock while a sibling lane builds — wait and retry, do not
  kill the tree.
