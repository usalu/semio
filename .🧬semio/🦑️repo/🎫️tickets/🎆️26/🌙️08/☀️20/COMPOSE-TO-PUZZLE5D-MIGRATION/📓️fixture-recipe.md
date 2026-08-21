# 🧪️ Handcrafted mutation-fixture recipe

The reference implementation is puzzle5d's 28 cases under
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/*/🧪️tests/*/`.
Read `📍move-part2d/🧪️tests/moves-part-a/` and `🗑delete-part/🧪️tests/removes-part-a-and-severs-fastener/`
before starting. Copy the SHAPE; never copy the CONTENT.

## Hard rule: every test is handcrafted
There is **no shared harness, no generic variant sweep, no macro, no loop over mutations**. Each
mutation gets its own directory, its own JSON, and its own `🦀️component.rs` whose assertions name
that mutation's own behavior. A test that would pass unchanged for a different mutation is wrong.

## Layout (one case per mutation, minimum)
```
<mutation-leaf>/🧪️tests/<kebab-case-name-describing-the-change>/
  📸️snapshot/⬅️before/🔣️component.json
  📸️snapshot/➡️after/🔣️component.json
  🦠️mutation/🔣️component.json
  🔺️diff/🔣️component.json      ← THE MOST IMPORTANT FILE. See below.
  🎯️outcome/🔣️component.json
  🦀️component.rs
```

## 🔺️ `🔺️diff/🔣️component.json` — the serialized diff, and the point of the whole fixture
This is **mandatory** and is the highest-value file in the case. `before`+`after` only prove the end
state; the diff pins **which collections and fields the mutation is allowed to touch**. A mutation
that reaches the right end state by rewriting the whole snapshot is a bug that only the diff catches.

It is the plain serde serialization of the artifact's own diff type (`<Artifact>Diff`), which is a
**sparse per-collection delta** — the same shape compose used
(`compose/fixture/nakagin-capsule-tower.deleted.design.diff.compose.json`: `pieces.removed[]`,
`pieces.updated[{piece,diff}]`). puzzle5d's equivalent is
`parts: {added[], removed[], patched[{id,patch:{replacement}}], reordered}` plus the same for
`fasteners`, and scalar `Option` fields for document-level edits.

Author it by transcribing **exactly** what that mutation's `🔺️diff/🦀️component.rs` constructs —
every field it sets, and nothing else. Check the diff struct's serde attributes: if the container has
`#[serde(default)]` but the fields carry no `skip_serializing_if`, serde emits **every** field,
`null` for the untouched ones — the committed JSON must match that exactly or the canonical-JSON
assertion fails.

A **rejected** case has no diff: it carries `🔺️diff/🚫️component.absent` (empty file) instead of
`🔺️diff/🔣️component.json`.
Do NOT author `.dsl.semio` / `.pack.semio` / `.op.semio` / `.spr.semio` / `.patch.semio` — those are
generated from the codecs later (contract D12). Hand-forging a binary would fake the codec test.

## Procedure per mutation — no shortcuts
1. Read `🦠️mutation/🦀️component.rs` — the payload struct's exact fields and types.
2. Read `🔺️diff/🦀️component.rs` — **this is the oracle**. It tells you precisely which fields change,
   which collections are touched, every cascade, every rejection code, and every no-op guard.
3. Read `↩️inverse/🦀️component.rs` — confirms the inverse restores prior values.
4. Author `⬅️before` as a SMALL snapshot of the artifact's own snapshot type that actually contains
   the entity the mutation targets.
5. Author `➡️after` by applying, **by hand, exactly what the diff builder does** — no more, no less.
6. Author `🦠️mutation/🔣️component.json` in the enum's serde shape.
7. Author `🎯️outcome/🔣️component.json`.
8. Write `🦀️component.rs` with assertions specific to this mutation.

## serde shapes — verify, do not assume
Read the mutation enum's `#[serde(...)]` attributes in the tree's `🧬️mutations/🦀️component.rs`.
For puzzle5d it is `#[serde(tag = "mutation", rename_all = "camelCase")]`, so a payload encodes as
`{"mutation":"movePart2d","id":"part-a","newX":5.0,"newY":7.0}`. **Other artifacts may differ** —
a different tag key, `rename_all = "snake_case"`, or an untagged/adjacent representation. Check.
Serde's `camelCase` on a *variant* lowercases only the first character (`MovePart2d` → `movePart2d`);
on a *field* it converts snake_case → camelCase (`new_x` → `newX`).

Only emit fields serde actually emits: `skip_serializing_if = "Option::is_none"` fields are omitted
when null; plain `#[serde(default)]` fields are always emitted. Mutation payload Option fields
usually have NO skip attribute, so they must be present, as `null` when empty.

## Outcome
```json
{ "status": "applied" }
{ "status": "applied", "messages": [{ "level": "warn", "code": "mutation.no-op" }] }
{ "status": "rejected", "code": "mutation.target-missing", "path": ["the-missing-id"] }
```
A `warn` no-op is **applied with an empty diff**, not rejected. A rejected case additionally needs
`🔺️diff/🚫️component.absent` (empty file) and `➡️after` identical to `⬅️before`.

## The test file
At least these assertions, each worded for this mutation:
1. applies-to-committed-after
2. inverse-restores-before
3. committed-json-is-canonical (both snapshots and the mutation)
4. declared-outcome-holds
5. **produces-committed-diff** — the mutation's own diff equals `🔺️diff/🔣️component.json`
6. **committed-diff-is-canonical**
7. **committed-diff-applies-to-after** — applying the committed diff to `before` yields `after`

See puzzle5d's `📍move-part2d/🧪️tests/moves-part-a/🦀️component.rs` for all seven. Use the artifact's own apply/inverse entry points —
find them in the tree's `🧬️mutations/🦀️component.rs` (puzzle5d exposes
`apply_puzzle5d_mutation` / `inverse_puzzle5d_mutation`; other artifacts name theirs differently).

Write calls in the **de-async style — no `.await`** — matching the committed example tests under
`📚️examples/*/🧪️tests/🦀️test.rs`. A repo-wide de-async sweep is in flight; the target state is
plain `fn`.

## Wiring
Add to the plugin's `📦️packages/🦀️rust/📦️glue.rs`, immediately after that mutation's existing
`pub mod inverse;` line, at the same indentation:
```rust
#[cfg(test)]
#[path = "<same relative prefix the sibling mod lines use>/<leaf>/🧪️tests/<case>/🦀️component.rs"]
mod tests_<case_with_underscores>;
```

## Checking your work
```
cd ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust && bun ./📜️script.ts fixtures lint --by-tree
```
Your tree must reach `0/N`. Derived-encoding warnings are expected and correct.

`cargo` is NOT usable: a peer session's de-async sweep has the workspace broken
(`semio-framework-os-infinite`, `semio-s-plugin-stdio`). Do not run it, do not try to fix it, and do
not claim any test passes. Validate structurally instead: every `include_str!` target exists, every
glue `#[path]` resolves, and `rustfmt --edition 2021 --emit stdout <file>` parses each test file.
