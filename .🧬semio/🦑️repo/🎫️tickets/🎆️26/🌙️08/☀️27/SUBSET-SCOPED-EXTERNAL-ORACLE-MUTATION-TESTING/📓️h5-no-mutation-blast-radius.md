# 📓️ H5 — `no-mutation` retirement: blast radius

**Question:** Protocol v2 rules that there is no *no-op mutation KIND* — a no-op is an OUTCOME CLASS
that a real mutation reaches. The framework already says so
(`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`: `Vec::new()` is "the semantic
replacement for the old `NoMutation` sentinel variant — there is no 'no-op mutation'"). The v2 contract
gate reports `test-only-mutation: no-mutation` for `s.stdio.step@ap214/cc6`. What does retiring it cost?

**Answer: 151 sites across 7 subsets, and it cannot be compile-verified today.**

## Scope

| Surface | Count | Where |
| --- | --- | --- |
| `NoMutation` enum references (Rust) | 60 | 7 enums: `StepMutation` (✳️any) + `StepCc1Mutation` … `StepCc6Mutation` |
| `"no-mutation"` string references | 91 | `KINDS` arrays, `kind()` match arms, oracle catalogs, test adapters, feature files |
| Physical mutation vector bundles | **0** | none exist — an identity operation has no before/after evidence to store |

Enum files: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️{any,cc1,…,cc6}/🧬️schema/🧬️mutations/🦀️component.rs`
Adapters: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🧪️tests/mutate-step-ap214-cc{1..6}/🦀️component.rs`
Catalogs: `…/🪆️subsets/✳️{any,cc1,…,cc6}/🧪️oracle/🔣️.json`

## The one decision that is not mechanical

All seven enums declare `#[default]` on `NoMutation` and derive `Default`:

```rust
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum StepCc6Mutation {
    #[default]
    NoMutation,
    …
```

Removing the variant forces a choice: **designate another variant as the default, or stop deriving
`Default` altogether.** The v2-correct answer is the second — "no mutation" is precisely the concept
v2 abolishes, so a mutation enum with a default value is a mutation enum with a sentinel by another
name. But taking `Default` away means finding and repairing every `::default()` call site, and that is
a compiler-driven refactor.

## Why it is not executed in this ticket

`cargo check -p semio-s-plugin-stdio --lib --offline` **fails**, and not because of anything here:
`semio-framework` reports errors in `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs`
(missing `DESCRIPTORS`/`descriptor` on two `protocol::Mutation` impls, plus missing `os_spr` imports) —
a concurrent session's in-progress mutation-descriptor refactor. The error count grew from 2 to 75
during this session, which is a live refactor, not a static defect.

So a 151-site edit across seven subsets, one of which is a `Default`-derive removal requiring
compiler-driven call-site repair, could not be verified at all today. Doing it blind would mean
reporting work as complete that nothing has checked — and `📓️w0-baseline.md`'s whole point is that an
unverified claim is the failure mode this protocol exists to remove.

## The mechanical remainder, once the workspace compiles

1. Drop `#[default]` and the `NoMutation` variant from all 7 enums; remove the `Default` derive and
   repair `::default()` call sites.
2. Remove `"no-mutation"` from all 7 `KINDS` arrays and all 7 `kind()` match arms. The framework's own
   `kinds_const_matches_enum_variants_in_declaration_order` test forces these to move together.
3. Remove `"no-mutation"` from the `kinds` array of all 7 oracle catalogs.
4. Delete the now-dead adapter guards `if kind != "no-mutation" && projection == before { … }` and the
   docstrings that explain them (each adapter carries the comment "`no-mutation` is not
   short-circuited: the trivial case is evidence" — that evidence becomes the identity round-trip).
5. Remove the `| no-mutation | {} |` `Examples` rows from the 7 feature files.
6. Re-declare the no-op OUTCOME on the mutations that can actually reach it. In the cc6 manifest
   `set-product-identity` already declares `no-op`, and `set-shape-representation` declares
   `no-op`/`empty`/`disjoint` — so the behaviour `NoMutation` stood for is already expressible, and
   nothing is lost by deleting the kind.

**Gate status meanwhile:** `mutationInventoryBreaches` reports `test-only-mutation` for it on every
contract run. The debt is visible and blocking, not forgotten.
