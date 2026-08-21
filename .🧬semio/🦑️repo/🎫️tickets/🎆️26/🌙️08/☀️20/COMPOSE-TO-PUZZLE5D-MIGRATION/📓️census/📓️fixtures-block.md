# 🧱️ Handcrafted mutation fixtures — the whole `🧱️block` plugin

104 cases, one per mutation leaf, across all three artifacts of
`✏️s/🔌️plugins/🧱️block`: `🖐️5d` (41), `🧊️3d` (37), `◻2d` (26).
Every case follows puzzle5d's committed reference shape
(`📍move-part2d/🧪️tests/moves-part-a/`, `🗑delete-part/🧪️tests/removes-part-a-and-severs-fastener/`):
five hand-authored source-of-truth files plus a per-mutation `🦀️component.rs` with the seven
assertions, each worded for that mutation.

## 🔖️ Coverage

| tree | mutations | cases | uncovered |
| --- | --- | --- | --- |
| `🗿️artifacts/🖐️5d/…/🧬️mutations` | 41 | 41 | 0 |
| `🗿️artifacts/🧊️3d/…/🧬️mutations` | 37 | 37 | 0 |
| `🗿️artifacts/◻2d/…/🧬️mutations` | 26 | 26 | 0 |

`bun ./📜️script.ts fixtures lint --by-tree` (run from `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust`)
went from `205 covered · 1353 uncovered` to `555 covered · 1003 uncovered`, and **no `🧱️block` row
appears in the `--by-tree` uncovered list any more**. A scoped re-run of the linter's own rules over
just the three block trees (`block-fixtures/blocklint.ts` in this ticket folder — the shared linter
truncates its printed error list at 40 repo-wide) reports `0 errors · 832 derived-encoding warnings`,
which are the expected contract-D1 `fixtures generate` gaps.

## 🔖️ How each case was derived

1. `🦠️mutation/🔣️component.json` — the enum's serde shape read off
   `#[serde(tag = "mutation", rename_all = "camelCase")]` on each `Block*Mutation`, with the payload
   struct's own `camelCase` field renames. `Option` payload fields carry no `skip_serializing_if`,
   so they are emitted as `null` (`updatePart2d`/`updatePresentation` carry `newRadius: null`).
   `BlockAuthor::email` and `BlockAttribute::definition` DO skip when `None`, so they are omitted.
2. `🔺️diff/🔣️component.json` — transcribed field-for-field from that leaf's own
   `🔺️diff/🦀️component.rs`. `Block5dDiff`/`Block3dDiff`/`Block2dDiff` are `#[serde(default)]`
   containers with no per-field skips, so every one of their 16 / 22 / 13 fields is present, `null`
   for everything the mutation does not touch; every identified-collection delta carries all four of
   `added`/`removed`/`patched`/`reordered`.
3. `📸️snapshot/➡️after/🔣️component.json` — produced by replaying the committed diff through a port
   of the artifact's own `MutationDiff::apply` (`apply_identified_delta`: removed → added (pushed
   last) → patched in place → reordered), so `after` cannot drift from the diff it is paired with.
4. `🎯️outcome/🔣️component.json` — `{"status":"applied"}` for all 104. No rejected/no-op cases were
   authored: `vcs::apply_mutation` returns `Ok` even for an `Error`/`Fatal` outcome (a rejected
   outcome's diff is `D::default()`, so applying it is a no-op), which means the reference test's
   `"rejected"` branch (`assert!(!applied)`) cannot hold for these artifacts. Every case is a real
   state change that clears its leaf's own no-op guard.
5. `🦀️component.rs` — the reference's seven assertions
   (`applies_to_committed_after`, `inverse_restores_before`, `committed_json_is_canonical`,
   `declared_outcome_holds`, `produces_committed_diff`, `committed_diff_is_canonical`,
   `committed_diff_applies_to_after`), every message naming `<leaf>/<case>`, plus one extra
   mutation-specific state assertion inside `applies_to_committed_after` that no sibling mutation
   would satisfy (e.g. `move-grip-2d` asserts the 3D `position` did NOT move; `change-vortex-kind-label`
   asserts the composed `catalog` handle did NOT move). Written de-async — no `.await`.

## 🔖️ Base snapshots

One small `⬅️before` per artifact, reused by that artifact's cases (the reference does the same):
a single kind identity, one representation/presentation, two kind-catalog rows, one rim template,
one compatibility rule, one attribute, one author, cameras and meta. Collection membership is
ordered so every delete's inverse (`create-*`, which appends) restores the original order exactly —
deletes always target the LAST row of their collection, and `remove-representation-tag` targets the
last tag.

## 🔖️ `🧊️3d`'s composed vortex-kind catalogue — the one real hazard

`Block3dSnapshot` does not store its vortex kinds. The `id`/`name` half lives in a composed
`s.stdio.semio@v1/kit` CHILD addressed by the content-hashed `catalog` handle
(`catalog-{:016x}` of `DefaultHasher` over `serde_json::to_string(&types)`), and only the
`label`/`color`/`defaultCableKind` overflow is persisted in `vortexKindExtra`. Two consequences the
fixtures had to honour:

- **Reads.** `vortex_kinds_of` resolves through a thread-local working-scene cache that a fresh test
  process starts empty, so all six vortex-kind diff builders would return `mutation.target-missing`.
  Every `🧊️3d` case's `before()` therefore calls
  `crate::artifacts::block3d::seed_vortex_kind_catalog_scratch(&[…])` with the two kinds the
  committed `before` snapshot's handle addresses — exactly what a real loader does, since the DSL
  never embeds child content.
- **Writes.** Applying a `vortexKinds` delta re-mints the handle through `set_vortex_kinds`, so the
  `after` snapshot embeds a hash that cannot be guessed. It was computed with a SipHash-1-3
  (zero-key) implementation in `block-fixtures/siphash.ts`, **validated against two independently
  committed handles** before use:
  `catalog-9dc5de0f33c9568d` (nakagin-capsule, one `door` kind) and `catalog-3b18d1b44d9af6de`
  (hexagonal-cut-concrete-forest-left, six kinds) — both reproduced exactly. As a third check,
  `delete-vortex-kind`'s computed `after` handle lands on `catalog-9dc5de0f33c9568d`, the committed
  nakagin value for the surviving single-`door` catalogue.
  `change-vortex-kind-{label,color,default-cable-kind}` touch only the overflow half, so their
  fixtures pin the handle as UNCHANGED; `create`/`delete`/`rename-vortex-kind` move it.

## 🔖️ Wiring

`✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs` gained 104 blocks, each inserted immediately
after that mutation's own `pub mod inverse;` line at the same indentation:

```rust
#[cfg(test)]
#[path = "../../🗿️artifacts/<a>/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧪️tests/<case>/🦀️component.rs"]
mod tests_<case_with_underscores>;
```

## 🔖️ Verification (no `cargo` — the workspace is broken by the in-flight de-async sweep)

Run `block-fixtures/verify.py` from this ticket folder:

- `📦️glue.rs` `#[path]` entries: **914 · 0 dangling**.
- `include_str!` targets across the 104 test files: **0 dangling**.
- Committed JSON files: **520 · all reparse**.
- `rustfmt --edition 2021 --emit stdout`: **105/105 parsed** (104 test files + `📦️glue.rs`).

No test is claimed to pass: nothing was compiled or executed.

## 🔖️ Authoring tooling (kept, per the ticket-folder rule)

`block-fixtures/` holds the per-artifact case tables whose `before`/`mutation`/`diff` payloads are
the hand-authored data (`block5d.ts`, `block3d.ts`, `block2d.ts`), the diff-replay + canonical-JSON
writer (`emit.ts`), the test renderer (`rust.ts`), the validated hash (`siphash.ts`), the glue
patcher (`wire.py`), and the two checkers (`verify.py`, `blocklint.ts`). The committed fixtures carry
no shared harness, macro or loop — each case is its own directory, its own JSON and its own test.
