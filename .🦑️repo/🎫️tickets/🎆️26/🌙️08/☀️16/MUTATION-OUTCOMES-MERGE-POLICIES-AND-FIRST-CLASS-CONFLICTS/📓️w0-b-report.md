# W0-B Derive — Report

## Lease
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs`

Both edited identically (three edits each). Mirror check: `diff <fileA> <fileB>` → **empty** (exit 0), confirmed both before and after edits.

## Changes (region `🔖️Mutations` / `🔖️CompositeMutation`)

### 1. `#[derive(Mutations)]` — `impl Mutation<Snapshot>::diff`
Before:
```rust
fn diff(&self, base: &#snapshot_ty) -> Self::Diff {
    match self { #(#diff_arms),* }
}
```
After:
```rust
fn diff(&self, base: &#snapshot_ty) -> ::protocol::MutationOutcome<Self::Diff> {
    match self { #(#diff_arms),* }
}
```
`diff_arms` unchanged (`<#payload_ty as MutationKind<Snapshot,#name>>::diff(payload, base)`), so this is a pure forward — no `MutationOutcome::map` needed: `MutationKind<Snapshot, Op>::diff` returns `MutationOutcome<Op::Diff>`, and here `Op = #name` so `Op::Diff == Self::Diff` by construction (the enum is its own `Op`). Map is only required "where the enum's Diff differs from the leaf's" per the brief — that never happens in this shape, so no map call was introduced. No `validate` was ever generated for `Mutation`/`MutationKind` here — nothing to delete on that side.

### 2. `register_calls` / `MutationDescriptor::new` — dropped `conflict_rule`
Before:
```rust
::protocol::MutationDescriptor::new(
    ::protocol::SchemaId(...), ::protocol::SchemaVersion(1), ::protocol::StateClass::Artifact,
    ::protocol::ConflictRule::Merge(::protocol::MergeStrategyKind::LwwRegister),
)
```
After: the `ConflictRule::Merge(MergeStrategyKind::LwwRegister)` 4th argument is deleted; `MutationDescriptor::new` is now called with exactly 3 args (`SchemaId`, `SchemaVersion`, `StateClass`).

### 3. `#[derive(CompositeMutation)]` — `impl MutationKind<Snapshot,Op>::diff`, `validate` deleted
Before:
```rust
fn diff(&self, base: &#snapshot_ty) -> <#op_ty as ::protocol::Mutation<#snapshot_ty>>::Diff {
    ::protocol::fold_plan_diff(self, base)
}
...
fn validate(&self, base: &#snapshot_ty) -> Result<(), String> {
    ::protocol::CompositeMutationKind::validate(self, base)
}
```
After:
```rust
fn diff(&self, base: &#snapshot_ty) -> ::protocol::MutationOutcome<<#op_ty as ::protocol::Mutation<#snapshot_ty>>::Diff> {
    ::protocol::fold_plan_diff(self, base)
}
```
(the `fn validate` block removed entirely). `fold_plan_diff` under C4 already returns `MutationOutcome<Op::Diff>`, matching the new return type directly — again no wrap/rewrap, no map (types coincide by trait definition).

`inverse`, `label`, `target`, `foreign_steps` — untouched in both derives, as specified.

## Mirror verification
```
$ diff <componentA> <glueB>
(empty)
```

## Acceptance

**`cargo check -p semio-framework-os-kernel-dsl-derive`** — green. `Finished dev profile [unoptimized] target(s) in 50.72s`, 0 errors, 0 warnings. (Proc-macro crate emits token streams only; it does not itself reference `protocol::MutationOutcome`/`MutationDescriptor`, so it type-checks independently of 0-A's landing state.)

**`cargo test -p semio-framework-os-kernel --lib -- os_spr::command`** — **did not run to completion; kernel crate is mid-flight from lane 0-A**, exactly as anticipated by the coordination note. 3 compile errors, all attributable to 0-A's in-progress files, none touching my lease:
- `error[E0425]: cannot find type MutationOutcome in module ::protocol` (×2) — `protocol::MutationOutcome` (C2) not yet landed.
- `error[E0061]: this function takes 4 arguments but 3 arguments were supplied` at `📡️spr/🎮️command/🦀️component.rs:1390` inside the `#[derive(Mutations)]` expansion — `MutationDescriptor::new` at `📡️spr/🎮️command/🦀️component.rs:611` still requires the old 4th `conflict_rule: crate::os_spr::ConflictRule` parameter; my derive now emits the 3-arg call per frozen C4 ("`MutationDescriptor` loses `conflict_rule`"), so this error resolves itself once 0-A drops that parameter from `MutationDescriptor::new`'s signature. No edit made to `📡️spr/🎮️command` (outside my lease).

Full compiler output tee'd to `🧪️w0-b-cargo.txt` in this ticket folder.

## Contract contradictions hit
None. C2/C4 were followed as frozen; the two kernel-crate errors are the expected W0-A/W0-B landing race, not a contract conflict.
