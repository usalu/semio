# ✅️ `step@ap214/✳️cc1` migrated — the pattern is proven against the compiler

**60 → 59 `E0046`, and `✳️cc1` produces zero diagnostics.** The leaf migration is no longer a plan; one
aggregate has been through it end to end and the compiler agrees.

## What the migration actually required

| step | detail |
|---|---|
| 4 leaf directories | `📋set-snapshot`, `🏷set-file-schema`, `🪪set-product-identity`, `🗑remove-shape-representation`, each with `🦀️.rs` + `🔣️.json` |
| leaf payloads | `#[derive(.., dsl::MutationLeaf)] #[mutation_leaf(contract = ::protocol)]` + `impl protocol::MutationKind<StepSnapshot, StepCc1Mutation>` |
| aggregate | newtype variants + `#[derive(dsl::Mutations)] #[mutations(snapshot = StepSnapshot, diff = StepDiff, schema = "s.stdio.step.cc1")]`; the hand-written `impl Mutation` deleted (the derive generates it) |
| shared logic | `class_diff`/`class_inverse` `pub(crate)` in the aggregate — one implementation, four leaf callers, no duplication |
| module wiring | `#[path = "📋set-snapshot/🦀️.rs"] pub mod set_snapshot;` **inside the aggregate**. No `📦️glue.rs` change was needed: `mutations` is itself declared by `#[path]` from the schema component, so child modules resolve relative to `🧬️mutations/` |
| call sites | the aggregate's own tests, the external harness `🧪️tests/mutate-step-ap214-cc1/🦀️.rs`, and the catalog's `kinds` list |

## The two corrections, confirmed in practice

**`SetSnapshot` survived.** It migrated as `{ verb: "set", entity: "snapshot", kind: "set-snapshot" }`
and the derive accepted it — `set` is an approved verb, and only `SEMANTICS.verb` is checked against
the table. CC1's documented escape hatch is therefore intact: `class_inverse` still degrades to
`SetSnapshot(base.clone())` for a representation removal, exactly as the module header describes. The
earlier claim that migration would strip cc1's undo was a consequence of my mistake, not of the work.

**`NoMutation` was the only casualty**, and it cost nothing: it is a unit variant where the derive
requires exactly one payload, and no approved verb means "do nothing". Its sole role was `inverse()`'s
"nothing to undo" arm, which is now the empty vector — the same statement without a vocabulary entry
behind it. `#[derive(Default)]` went with it, as it did in `✳️drawing`.

## What this tells the remaining 59

* The wire format claim holds in practice: three of four variants kept their names and shapes.
* Per-artifact cost is roughly *(leaves × ~40 lines)* + one `NoMutation` decision + call-site syntax.
* `#[path]` inside the aggregate works when `mutations` is `#[path]`-declared by its schema component,
  which is the common shape. `✳️drawing`'s glue-based wiring is the alternative where it is not.
* The `🔣️.json` fields that looked like they needed rulings did not, for this artifact: `verb`/`entity`
  came off the variant name, and `invertibility: explicit-mutation` off the existing `inverse()` arms
  returning real values.

## Verification

```
cargo build -p semio-s-plugin-stdio --offline   →  59 error[E0046]   (was 60)
                                                    ✳️cc1 diagnostics: 0
harness                                          →  121/121 checks passed
```
