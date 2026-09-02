# 🪤️ `#[value(…)]` is INVALID on `Dsl*`-derived types — a mechanical-rewrite landmine

## What happens
The bulk `#[serde(…)]` → `#[value(…)]` rewrite is correct ONLY for types deriving
`ToValue`/`FromValue`. It silently breaks two other shapes:

**1. Types deriving `dsl::DslRecord` / `DslArtifact` / `DslEnum` / `DslOps`.**
That macro registers `attributes(dsl)` ONLY —
`🗣️dsl/✨️derive/📦️packages/🦀️rust/🦀️.rs:14  #[proc_macro_derive(DslRecord, attributes(dsl))]`
— so `#[value(…)]` on such a type is an unregistered attribute:
    error: cannot find attribute `value` in this scope

**2. Types with HAND-WRITTEN `impl ToValue` / `impl FromValue`.**
Here `#[value(…)]` is not merely unregistered but MEANINGLESS — a manual impl never reads the
attribute — and "fixing" the error by adding `ToValue, FromValue` to the derive list produces
    error[E0119]: conflicting implementations
`PlaybookBlockOption` (📖️playbook/🦀️.rs:116) is exactly this: manual impls at :128 and :133.

## The wrong fix, and why it is tempting
Seeing `cannot find attribute 'value'`, the obvious move is to add `ToValue, FromValue` to the
derive. That resolves the attribute error and CREATES E0119 on any type with manual impls. I did
this, and it took the file 18 → 10 errors while introducing 2 new ones. The correct fix for shape 2
is to DELETE the stray `#[value(…)]`, leaving the manual impls to do their job.

## Correct triage before rewriting any `#[serde(…)]`
1. Does the container derive `ToValue`/`FromValue`? → `#[value(…)]` is correct.
2. Does it derive a `Dsl*` macro instead? → the attribute is `#[dsl(…)]`, NOT `value`.
3. Does it have a hand-written `impl ToValue`/`FromValue`? → NO container attribute at all; remove it.

## Scale
Measured repo-wide: only **8 files** have a `Dsl*`-derived container carrying `#[value(…)]` without
`ToValue`. Contained, not systemic — but each is a hard compile error, invisible to rustfmt,
bracket-balance and attribute-parity checks, which is exactly what agents fell back on while the
build lock was held.

## Verified progress on 📖️playbook (crate `semio-framework-os-flow`, NOT os-kernel)
18 → 10 → **5** errors. Remaining 5: four `serde_json::Value: ToValue/FromValue` bounds at
:229/:235/:979 and 🧬️generation/🦀️.rs:97 (the FormGeneration consumer wave, still owed), plus one
unrelated `E0502` borrow error in 🌿️vcs/🦀️.rs:2771.

⚠️ Verify this file with `cargo check -p semio-framework-os-flow`. It is mounted by `#[path]` at
🌊️flow/📦️packages/🦀️rust/🦀️.rs:35 and is NOT part of os-kernel — an os-kernel check reads green
regardless of what you did to it, and I briefed an agent with that false baseline earlier.
