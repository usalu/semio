# Fix: `semio-framework-os-kernel` 🏪️store mutation-trait blocker

## Root cause

`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs` was rewritten (today, 2026-09-01,
ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`, see its
`🔍️research/📓️serde-replacement-surface.md` and `📓️serde-fanout-playbook.md`) to swap
`Mutation<P>`'s and `MutationDiff<P>`'s supertrait bounds from `serde::Serialize +
serde::de::DeserializeOwned` to the new first-party `crate::value::ToValue +
crate::value::FromValue` (a `DslValue`-based analog, added so plugins implementing a mutation
are no longer forced onto `serde`). This is a real, framework-wide, already-landed trait-bound
change — not a race with a still-running peer. `🏪️store`'s own mutation aggregate
(`SpaceHistoryMutation`) and diff type (`SpaceHistoryDiff`) were never updated to satisfy it,
which is Family A (missing `FromValue`/`ToValue`, 16 of the 75 errors, all E0277, cascading
through every leaf's `impl MutationKind<SpaceHistorySnapshot, SpaceHistoryMutation>` — that impl's
own `where Op: Mutation<P>` bound fails transitively). Family B (the remaining errors, `Mutation:
serde::Serialize`/`DeserializeOwned` not satisfied at two generic call sites, one of them a
downstream E0599 on `.dispatch(...)`) is unrelated pre-existing bound-propagation debt: two
functions generic over a `Mutation` type parameter call into `ArtifactStore<P, Mutation>`
(whose own struct/impl bounds require `Mutation: Serialize + DeserializeOwned`) without
declaring those bounds themselves.

`🏪️store`'s individual mutation LEAVES (`CommitSpaceCheckpoint`, `CreateSpaceAlternative`, etc.)
were **not** part of this breakage — they already carry `#[derive(..., dsl::MutationLeaf)]` +
`#[mutation_leaf(contract = ::protocol)]` and a hand-written `impl
crate::os_spr::MutationKind<SpaceHistorySnapshot, SpaceHistoryMutation> for <Leaf>`, matching the
already-migrated pattern the ticket pointed at (`🔁️workflow/🧬️schema/🧬️mutations/`). Only the
**aggregate** (`SpaceHistoryMutation`) and the **diff** (`SpaceHistoryDiff`) needed the new
`ToValue`/`FromValue` impls — no leaf file needed touching.

## Migrated example followed

Not the workflow module in the end (see "A note on the stated reference" below) — the actual
precedent used is `📖️playbook`'s `PlaybookMutation`/`PlaybookDiff`
(`✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/{🧬️mutations/🦀️component.rs,🔺️diff/🦀️component.rs}`),
which is the pilot the fan-out playbook doc documents as actually exercised against a real
`Mutation`/`MutationDiff` pair, and `📖️playbook`'s own framework-side domain structs
(`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️component.rs`) for the
`#[derive(..., ToValue, FromValue, ...)]` + `#[value(...)]` attribute mechanics.

Key facts pulled from that precedent and the two research docs:
- `#[derive(ToValue, FromValue)]` (crate `semio-framework-value-derive`, already a dependency of
  `semio-framework-os-kernel`'s `Cargo.toml`) is the preferred way to satisfy the new bound,
  **except** it does not support adjacently-tagged enums (`#[serde(tag = "...", content =
  "...")]`) — the single biggest documented gap in `📓️serde-replacement-surface.md`.
  `SpaceHistoryMutation` uses exactly that shape
  (`#[serde(tag = "operation", content = "payload", ...)]`), so it cannot use the derive.
- For framework code (not a `✏️s/` plugin), `📓️serde-replacement-surface.md` §(b) explicitly says
  the sanctioned interim answer for a type that can't derive is to keep it on the existing
  `serde`-based bridge (`crate::os_dsl::to_dsl_value`/`from_dsl_value`, `T: Serialize +
  DeserializeOwned -> DslValue`) rather than hand-rolling a tag-matching `to_value`/`from_value` —
  framework crates are exempt from the zero-third-party-serde rule (only `✏️s/` plugin manifests
  must reach it). `🏪️store`'s own `OpBinary::encode_op`/`decode_op` for `SpaceHistoryMutation`
  already routes through this exact bridge, so wiring `ToValue`/`FromValue` through it too keeps
  the trait's `DslValue` shape byte-identical to what's already on the wire, for free.

## A note on the stated reference (`🔁️workflow/🧬️schema/🧬️mutations/`)

Investigated first, since the task named it as "the fully-working reference." It is **not**
currently working against this trait file: `WorkflowMutation`/`WorkflowDiff` (and, one crate
mount away, `RunMutation`/`RunDiff` in `🔁️workflow/🗿️artifacts/🏃️run/`) also lack
`ToValue`/`FromValue` and fail the identical way — confirmed for real by building
`semio-framework` (see "Downstream: `semio-framework-os-mcp`" below), not assumed. It was correct
and useful as the precedent for the **`dsl::MutationLeaf` + `protocol::MutationKind`** pairing on
individual leaves (which `🏪️store`'s leaves already had, so nothing to copy there), but it is not
itself past this specific `ToValue`/`FromValue` migration — likely why the earlier
`fix-plugin-host-blocker` report never needed to touch it: that wave was about `MutationLeaf`
wiring, landed before today's separate serde→`ToValue`/`FromValue` supertrait swap.

## Files changed

All edits in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`:

1. **Import line** — added `FromValue, ToValue, ValueError` to the existing
   `use crate::os_dsl::{from_dsl_value, to_dsl_value, DslOps, DslRecord, DslValue, ...};` line.

2. **`SpaceHistoryDiff`** (plain struct, all-optional fields, no tag issue) — added, right after
   its existing `impl MutationDiff<SpaceHistorySnapshot> for SpaceHistoryDiff` block:
   ```rust
   impl ToValue for SpaceHistoryDiff {
       fn to_value(&self) -> DslValue {
           to_dsl_value(self).expect("SpaceHistoryDiff converts to DslValue infallibly")
       }
   }
   impl FromValue for SpaceHistoryDiff {
       fn from_value(value: DslValue) -> Result<Self, ValueError> {
           from_dsl_value(value).map_err(ValueError::new)
       }
   }
   ```

3. **`SpaceHistoryMutation`** (adjacently-tagged enum — derive unsupported) — added the same
   shape, right before its existing `impl OpText for SpaceHistoryMutation` block:
   ```rust
   impl ToValue for SpaceHistoryMutation {
       fn to_value(&self) -> DslValue {
           to_dsl_value(self).expect("SpaceHistoryMutation converts to DslValue infallibly")
       }
   }
   impl FromValue for SpaceHistoryMutation {
       fn from_value(value: DslValue) -> Result<Self, ValueError> {
           from_dsl_value(value).map_err(ValueError::new)
       }
   }
   ```

4. **Family B, site 1** — `apply_ops_binary_impl<'a, P, Mutation>`'s `where` clause (inside the
   `#[cfg(...)] mod` block around line 9167) called `ArtifactStore::new(envelope)` and
   `store.dispatch(...)`, both of which require `Mutation: Serialize + DeserializeOwned` per
   `ArtifactStore<P, Mutation>`'s own declared bounds — but this function's generic `Mutation`
   parameter never declared them. Changed:
   ```diff
   - Mutation: OpText + OpBinary + self::Mutation<P> + Send + 'static,
   + Mutation: Serialize + DeserializeOwned + OpText + OpBinary + self::Mutation<P> + Send + 'static,
   ```

5. **Family B, site 2** — `pub trait SubsetRoundtripSpec { type Mutation: ...; }`'s associated
   type bound (inside `test_support`, around line 19550) was used by `assert_subset_roundtrip`
   to call `assert_store_roundtrip::<S::Snapshot, S::Mutation>(...)`, which requires the same
   `Serialize + DeserializeOwned` bound — added at the associated type's own declaration (per the
   ticket's instruction to fix the declaration, not the call site):
   ```diff
   - type Mutation: Clone + PartialEq + std::fmt::Debug + Mutation<Self::Snapshot> + OpText + OpBinary;
   + type Mutation: Clone + PartialEq + std::fmt::Debug + Serialize + DeserializeOwned + Mutation<Self::Snapshot> + OpText + OpBinary;
   ```

No leaf file, no `🔣️.json` descriptor, and no other module was touched. `dsl::Mutations`'s own
codegen, `MutationKind`/`MutationLeaf` on every leaf, and the existing `OpText`/`OpBinary`/
`ArtifactDsl`/`ArtifactPack` impls for `SpaceHistoryMutation`/`SpaceHistorySnapshot` were already
correct and untouched.

## Not touched (per instructions)

- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs` — the peer/today's-wave trait file. Its
  `ToValue`/`FromValue` supertrait shape is correct; store now conforms to it.
- `🔁️workflow/🧬️schema/🧬️mutations/` and `🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/` —
  same disease (`WorkflowMutation`/`WorkflowDiff`/`RunMutation`/`RunDiff` missing
  `ToValue`/`FromValue`), but these live in the `semio-framework` crate, not
  `semio-framework-os-kernel`, and are out of this ticket's stated scope (`🏪️store`/os-kernel
  only, "do not opportunistically refactor"). See below — this is what blocks
  `semio-framework-os-mcp`.

## Verification — real command output

### `cargo build -p semio-framework-os-kernel --message-format short` — zero errors

```
export CARGO_TARGET_DIR=".../AI-MCP-END-TO-END/🗑️generated/target"
export RUSTC_WRAPPER=""
cargo build -p semio-framework-os-kernel --message-format short
```

Tail (full run, ~1m32s, from-scratch against the isolated target dir):

```
🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:13609:13: warning: the `redo_edit_ids:` in this pattern is redundant: help: use shorthand field pattern: `mut redo_edit_ids`
🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:13610:13: warning: the `cursor_applied_edit_ids:` in this pattern is redundant: help: use shorthand field pattern: `mut cursor_applied_edit_ids`
🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:13611:13: warning: the `cursor_redo_edit_ids:` in this pattern is redundant: help: use shorthand field pattern: `mut cursor_redo_edit_ids`
warning: `semio-framework-os-kernel` (lib) generated 33 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 33 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 1m 32s
```

Shell exit code: `0`. 33 warnings, all pre-existing (`unnecessary qualification`, redundant
pattern-field shorthand) — none introduced by this fix, zero errors. Down from 75 errors.

### `cargo build -p semio-framework-os-mcp --bin semio-os-mcp --message-format short` — cannot complete, NOT this ticket's bug

```
cargo build -p semio-framework-os-mcp --bin semio-os-mcp --message-format short
```

Does not reach `semio-framework-os-mcp` at all — its direct dependency `semio-framework`
(`Cargo.toml:39`, `semio-framework = { workspace = true }`) fails first, with 166 errors, all of
the identical missing-`ToValue`/`FromValue` shape, confined to `🔁️workflow` (verbatim, deduped by
type):

```
error[E0277]: the trait bound `WorkflowMutation: FromValue` is not satisfied: unsatisfied trait bound
error[E0277]: the trait bound `WorkflowMutation: ToValue` is not satisfied: unsatisfied trait bound
error[E0277]: the trait bound `WorkflowDiff: FromValue` is not satisfied: unsatisfied trait bound
error[E0277]: the trait bound `WorkflowDiff: ToValue` is not satisfied: unsatisfied trait bound
error[E0277]: the trait bound `RunMutation: FromValue` is not satisfied: unsatisfied trait bound
error[E0277]: the trait bound `RunMutation: ToValue` is not satisfied: unsatisfied trait bound
error[E0277]: the trait bound `RunDiff: FromValue` is not satisfied: unsatisfied trait bound
error[E0277]: the trait bound `RunDiff: ToValue` is not satisfied: unsatisfied trait bound
```

Locations (every file that errors, from the real run):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs` (`WorkflowDiff` impl at
  `:1350`; `RunMutation`/`RunDiff` usage at `:2015`, `:2020`, `:2072`, `:2076`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/🦀️.rs` (the
  `WorkflowMutation` aggregate itself, plus all 17 leaves: `➕️add-node`, `🗑️remove-node`,
  `🔗connect-ports`, `✂️disconnect-edge`, `↔️move-node`, `✏️rename-node`, `🧩add-parameter`,
  `🧹remove-parameter`, `🩹change-parameter`, `🔒bind-parameter-field`,
  `🔓unbind-parameter-field`, `🔄update-node-ports`, `📥add-input`, `🚮remove-input`,
  `🔌bind-input`, `🚪unbind-input`, `📤bind-output`, `⛔️unbind-output`, each at `🦀️.rs:13`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/🦀️.rs`
  (the `RunMutation` aggregate) and its leaves `🚀️start-run`, `✅️finish-run-node`,
  `▶️start-run-node`, `🔏️seal-run`, `🪵️append-run-log`

Final line: `error: could not compile `semio-framework` (lib) due to 166 previous errors; 8
warnings emitted`. Cargo never attempts `semio-framework-os-mcp` itself, so its own
`📇️registry`/`🗿️artifact`/`💡️inference`/`🖥️ui`/`💬️prompts` facet code is **not exercised at all**
by this build — there is no "os-mcp's own" error list to hand back yet; the report above (68
distinct E0277 lines in `--message-format short`'s output, though rustc's own summary counts 166
— short format appears to fold/drop some duplicate-shape diagnostics) is everything blocking it.

**This is out of scope for this ticket** (`🏪️store`/`semio-framework-os-kernel` only, "do not
opportunistically refactor") and is a different crate (`semio-framework`, which does not depend
on `semio-framework-os-kernel`'s `store` module at all) — not touched. It is the same root-cause
*class* as the fix above (today's `Mutation`/`MutationDiff` supertrait swap in
`📡️replication/🎮️mutation/🦀️.rs`), just in the still-unmigrated `🔁️workflow`/`🏃️run` facets, which
would need the identical `ToValue`/`FromValue` treatment (most of `🔁️workflow`'s leaves are
internally-tagged, so unlike `🏪️store` they likely CAN use `#[derive(ToValue, FromValue)]`
directly — see `📖️playbook`'s `PlaybookMutation` for the exact attribute pattern) before
`semio-framework`, and therefore `semio-framework-os-mcp`, can build.

## Summary for the ticket owner

- `semio-framework-os-kernel`: **fixed, verified, zero errors** (was 75).
- `semio-framework-os-mcp`: **still blocked**, but not by `🏪️store` and not by mcp's own code —
  blocked by `semio-framework`'s `🔁️workflow`/`🏃️run` mutation facets needing the same
  `ToValue`/`FromValue` migration `🏪️store` just got. Needs its own ticket/scope; flagging rather
  than fixing, per this ticket's boundaries.
