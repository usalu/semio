# 🧪️ Handcrafted mutation fixtures — `🧩️puzzle` `◻2d` + `🧊️3d`

Slice: the two remaining puzzle artifacts. One test case per mutation leaf, 61 cases total
(26 for `◻2d`, 35 for `🧊️3d`), each with the full hand-authored quintet
(`⬅️before` / `➡️after` / `🦠️mutation` / `🔺️diff` / `🎯️outcome` / `🦀️component.rs`) and its own
seven assertions.

## 📊️ Result

| tree | leaves | cases | lint uncovered | lint errors |
|---|---|---|---|---|
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations` | 26 | 26 | **0/26** | **0** |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations` | 35 | 35 | **0/35** | **0** |

`bun ./📜️script.ts fixtures lint --by-tree` no longer lists either tree in its uncovered-by-tree
roll-call and raises no finding against either (the repo-wide error/warning totals it still prints
belong entirely to other plugins' trees — `🏛️architect`, `🗄️stdio`, `📕️norm`, `🪐️space`, …).
Derived-encoding warnings (`.op.semio` / `.spr.semio` / `.patch.semio` / `.dsl.semio` /
`.pack.semio`) are expected and correct: contract D1 says those are produced by
`fixtures generate` once the workspace compiles, never hand-forged.

## 🧱️ The two base snapshots

Every case in a tree starts from one small hand-authored base, so a reader can diff any `➡️after`
against it by eye.

**`◻2d`** — `schema: puzzle.2d.fixture`, a camera at the origin, two nodes and one edge:
`node-a` (circle, kind `node-kind-a`, handles `handle-1` + `handle-spare`), `node-b` (rectangle,
kind `node-kind-b`, handle `handle-2`), `edge-1` joining `handle-1 → handle-2` with tips
`none`/`arrow`, and `meta` carrying `manifestId: manifest-alpha` plus one
`handle-kind-a → handle-kind-b` compatibility row. No `kindCatalogs`.

**`🧊️3d`** — `schema: puzzle.3d`, `domain: architecture`, `object-a` (posed, meshed, vortices
`vortex-1` + `vortex-spare`) and `object-b` (vortex `vortex-2`), `attraction-1` joining
`object-a:vortex-1 → object-b:vortex-2`, one `volume-1` target volume (per-axis scale
`[2, 2, 2]`), one `reference-1` plane, and `meta` with one
`vortex-kind-a → vortex-kind-b` compatibility row. No `kindCatalogs`.

The bases deliberately carry both `Puzzle3dScale` shapes (`object-a` uses the scalar form,
`volume-1` the `[x, y, z]` triple) so the scale mutations exercise the union's serialization in
both directions.

## 🔺️ Diff derivation

Every `🔺️diff/🔣️component.json` was transcribed field by field from that leaf's own
`🔺️diff/🦀️component.rs`, never from its name or docstring:

* Both diff structs are `#[serde(rename_all = "camelCase", default)]` with **no**
  `skip_serializing_if` on any field, so serde emits *every* field — 28 for `Puzzle2dDiff`, 40 for
  `Puzzle3dDiff` — with `null` for the untouched ones. Each committed diff carries the full field
  list in declaration order.
* Per-field edits (`move-node`, `change-object-hidden`, …) produce a **whole-item replacement**
  inside `<collection>.patched[{id, patch: {replacement}}]` — never a sparse per-field patch —
  because that is what the builders construct.
* Cascades are real removals in a *second* collection, in the same diff: `delete-node` +
  `remove-node-handle` sever `edges`; `delete-object` + `remove-object-vortex` sever `attractions`
  (matched on the `object:vortex` full-id form). `delete-target-volume` / `delete-reference` have
  no cascade at all and touch exactly one collection.
* `meta`-carried edits (`change-manifest-id`, `connect`/`disconnect-kind-compatibility`,
  `replace-kind-catalogs`) republish the **whole `meta` block** — neither diff type has a meta
  delta — while `change-domain` is the one genuinely scalar document edit, landing on the diff's
  own `domain` field.
* `create-*` / `connect-*` with a `null` index emit `added` only and leave `reordered` unset; the
  `reordered` branch is only reachable from an explicit index, which is what the *inverses* of
  `delete-node` / `delete-object` / `delete-reference` use.

## 🎯️ Outcomes

59 of 61 cases are `{"status": "applied"}` with no messages. The two exceptions are
`replace-node-handle` and `replace-object-vortex` — see the surprise below — which are
`applied` with a `warn` / `mutation.no-op` message and an all-`null` diff, per the recipe's rule
that a warned no-op is *applied with an empty diff*, not rejected.

No case is `rejected`. That is deliberate: `apply_puzzle2d_mutation` / `apply_puzzle3d_mutation`
go through `vcs::apply_mutation`, which applies the outcome's diff and only surfaces a
`MutationApplyError` from `MutationDiff::apply` itself. A `MutationOutcome::error` /
`::fatal` rejection carries `D::default()`, and applying a default diff always succeeds — so
`apply_*` returns `Ok` even for a rejected op, and the recipe's `declared_outcome_holds`
assertion (`rejected` ⇒ `apply` is `Err`) could not hold for any target-missing or duplicate-id
case. The committed puzzle5d reference set makes the same choice (28 cases, none rejected, no
`🚫️component.absent` anywhere).

## 🧪️ The seven assertions

Each `🦀️component.rs` carries the recipe's seven, worded for its own mutation and case, plus a
handcrafted targeted assertion block:

1. `applies_to_committed_after` — equality against `➡️after`, **plus 2–4 case-specific field
   assertions** (e.g. `move-node` checks `(node.x, node.y) == (5.0, 7.0)` and that the radius did
   not change; `replace-node-geometry` checks the circle's `radius` was *cleared*, not retained
   alongside the rectangle extent).
2. `inverse_restores_before` — forward then every inverse step.
3. `committed_json_is_canonical` — both snapshots and the mutation payload.
4. `declared_outcome_holds` — plus, on the two no-op cases, an assertion that the declared message
   really is `mutation.no-op`.
5. `produces_committed_diff` — equality against the committed diff, **plus case-specific
   structural assertions** naming which collections this mutation may and may not touch
   (e.g. `change-domain` asserts `objects` and `meta` are both `null`; `scale-object` asserts the
   per-axis scale serialized as an *array*; `scale-target-volume` asserts the uniform scale
   serialized as a *bare number*).
6. `committed_diff_is_canonical`.
7. `committed_diff_applies_to_after`.

Calls are written in the de-async style (no `.await`), matching the committed puzzle5d fixtures.

## 🔌️ Wiring

61 mounts added to `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs`, each immediately after
that leaf's existing `pub mod inverse;` at the same indentation:

```rust
#[cfg(test)]
#[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-node/🧪️tests/moves-node-a/🦀️component.rs"]
mod tests_moves_node_a;
```

`git diff --stat` on `📦️glue.rs`: **183 insertions, 0 deletions** (61 × 3 lines). The puzzle5d
mounts are untouched.

## ✅️ Verification performed

`cargo` was **not** run — a peer's de-async sweep has the workspace broken, and no claim is made
here that any test passes. Structural validation instead:

* `fixtures lint --by-tree`: both trees absent from the uncovered roll-call, zero findings.
* All 700 `#[path]` targets in `📦️glue.rs` resolve on disk (0 missing).
* All 305 `include_str!` targets across the plugin's fixture tests resolve (0 missing).
* `rustfmt --edition 2021 --emit stdout` parses all 61 new test files and `📦️glue.rs` (0 failures).
* A serde-shape validator (written from the record structs' own attributes) confirms every
  committed snapshot emits exactly the always-emitted fields, omits every
  `skip_serializing_if = "Option::is_none"` field rather than writing `null`, and writes every
  `f64` as a JSON float — so the `committed_json_is_canonical` assertions cannot trip on shape.
* A payload validator derived from each leaf's own `pub struct` confirms every
  `🦠️mutation/🔣️component.json` carries the `"mutation"` tag with the correct camelCased variant
  name plus exactly the payload's camelCased fields (no `skip_serializing_if` exists on any
  payload, so every field, including `null` ones, must be present).
* A delta-apply simulator (mirroring `apply_identified_delta`) confirms, for all 61 cases, that
  applying the committed `🔺️diff` to `⬅️before` yields `➡️after` exactly — the same property
  assertion 7 checks at runtime.

## ⚠️ Surprises

1. **`replace-node-handle` and `replace-object-vortex` are dead — every invocation is a no-op.**
   Both builders do
   ```rust
   let mut next = object.clone();
   if next == *object { return …warn("mutation.no-op"…); }
   for vortex in next.vortices.iter_mut() { … }
   ```
   The clone-versus-original comparison sits **before** the write loop, so it is unconditionally
   true and the new handle/vortex is never applied. Per the binding instruction to derive the
   fixture from the diff builder rather than from the name, both cases are committed as warned
   no-ops (`rekind-handle-1-is-noop`, `rekind-vortex-1-is-noop`) with an all-`null` diff and
   `after == before`. Fixing the guard (move it after the loop, compare `next` against `*object`
   there) will require re-authoring exactly these two cases.
2. **The same bug is in puzzle5d's `🔌replace-part-grip`, and its committed fixture contradicts
   it.** `🖐️5d/…/🔌replace-part-grip/🧪️tests/rekinds-grip-1/` declares `status: applied` with a
   populated `parts.patched` diff and a differing `after` — the intended behaviour, not the
   actual one. That fixture will fail until the guard is fixed. Flagged, not touched: it is
   outside this slice.
3. **`create-*` uses `MutationOutcome::fatal("mutation.duplicate-id", …)` while every other
   rejection uses `::error`.** Immaterial to these fixtures (no duplicate-id case is authored,
   for the reason given under Outcomes) but worth knowing when rejected cases become
   expressible.
4. **`Puzzle3dScale` is a hand-written `Serialize`/`Deserialize` union** (bare number *or*
   `[x, y, z]`). Any base or diff written with an integer literal (`1` rather than `1.0`) would
   round-trip to a different `serde_json::Number` variant and fail the canonicality assertion;
   the float-typing validator above exists specifically to catch that.
5. The `fixtures lint` repo-wide totals moved a lot during this run (209 → 552 covered) because
   peer sessions are landing fixtures in other plugins concurrently. Only the two rows above are
   this slice's.

## 📁️ Files

* 61 case directories × 6 files under
  `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/…/🧬️mutations/<leaf>/🧪️tests/<case>/` and
  `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/🧬️mutations/<leaf>/🧪️tests/<case>/`
* `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs` (61 test mounts)
* Ticket-local transcribers (temporary, kept for audit):
  `📓️census/📜️writer-puzzle-2d.py`, `📓️census/📜️writer-puzzle-3d.py`
