# 🧪️ Handcrafted mutation fixtures — `📖️playbook` (9), `🎞️animate` (9), `🎬️sequence` (8)

26 cases, one per mutation leaf. Every case is handcrafted: its own `before`/`after`/`mutation`/
`diff`(or absent)/`outcome` JSON plus a `🦀️component.rs` with **seven** assertions worded for that
mutation's own guard. No shared harness, no macro, no loop.

## 📍 Scope confirmed by discovery
```
find ✏️s/🔌️plugins/📖️playbook ✏️s/🔌️plugins/🎞️animate ✏️s/🔌️plugins/🎬️sequence -type d -name 🧬️mutations
```
reports five directories, but only three contain mutation LEAVES (`🦠️mutation/🦀️component.rs`):
- `📖️playbook/…/🧬️schema/🧬️mutations` — 9 leaves
- `🎞️animate/🗿️artifacts/🎬️present/…/🧬️schema/🧬️mutations` — 9 leaves
- `🎬️sequence/…/🧬️schema/🧬️mutations` — 8 leaves

The two `🚪️io/🧬️mutations` directories (animate, sequence) hold only `📝️text`/`💾️binary` codec
facets — the lint's own `NON_MUTATION_DIRS` set — and no leaves. Playbook's `🧬️schema/🧬️mutations`
carries the same two codec dirs alongside its nine real leaves. Nothing is uncovered.

## ⚠️ The structural constraint that shaped every case
All three artifacts are **composed**: their real content lives in an opaque `store::ArtifactChild`
handle, not in the snapshot.

| plugin | composed slot(s) | content-handle minted by |
| --- | --- | --- |
| playbook | `document` (`s.stdio.semio.document`) + `flow` (`s.stdio.semio.flow`) | `diff_replace_content` |
| animate | `presentation` (`s.stdio.semio.presentation`) + `animation` | `diff_set_presentation` |
| sequence | `content` (`s.stdio.semio.flow`) | `diff_replace_content` |

Every one of those builders derives the child's `child_id` from
`std::collections::hash_map::DefaultHasher` over the child content JSON. `DefaultHasher`'s output is
explicitly unspecified by `std`, so an `➡️after` for any content-changing branch could only be
produced by *running* the code and pasting the digest back — i.e. by forging a value, not by
handcrafting one. That is the same wall `🕸️dag`'s already-landed fixtures hit and documented
(`🌱create-node/🧪️tests/rejects-a-duplicate-node-id/🦀️component.rs`).

**Consequence, applied consistently across all three trees:** each case pins a branch that mints no
handle at all — a rejection (`🔺️diff/🚫️component.absent`) or an applied Warning no-op (empty diff,
committed in full). Each fixture's before-snapshot carries a **unique** `childId` and the test seeds
the plugin's own working-scene cache for exactly that id, so no two fixtures can contaminate each
other through the shared `thread_local!` scratch.

The single exception is **playbook `✏️change-title`**, whose diff builder never reads or writes the
composed children — it sets the root `title` scalar and nothing else. That is the one case in this
slice with a genuinely non-empty committed `🔺️diff` and a genuinely different `➡️after`.

## 🔣 serde shapes — verified per plugin, not assumed
- `PlaybookMutation`: `#[serde(tag = "mutation", rename_all = "camelCase")]` → `{"mutation":"addStep", …}`
- `SequenceMutation`: `#[serde(tag = "mutation", rename_all = "camelCase")]` → `{"mutation":"createStep", …}`
- `PresentMutation`: **no serde attribute at all** → serde's externally tagged default,
  `{"CreateTile": { … }}`, PascalCase variant name. Pinned by an assertion in the create-tile case.

Diff containers all carry `#[serde(rename_all = "camelCase", default)]` with **no**
`skip_serializing_if`, so every field is emitted, `null` when unset — the committed empty diffs list
all 10 (playbook) / 6 (present) / 7 (sequence) fields explicitly.

⚠️ `PlaybookDiff::title` is the slice's only double-`Option` (`Option<Option<String>>`): a committed
`null` decodes back as `Some(None)` — an explicit "clear the title" — so every playbook fixture's
snapshots carry `"title": null` and the `committed_diff_applies_to_after` round trip stays a fixed
point. Documented inline in each playbook test.

## 🧪️ The cases

### 📖️ playbook (9)
| leaf | case | branch pinned | outcome |
| --- | --- | --- | --- |
| `➕add-step` | `no-ops-on-a-duplicate-step-id` | duplicate id → **Warning** (never Fatal, unlike other artifacts) | applied + `mutation.no-op` |
| `➖remove-step` | `rejects-removing-a-missing-step` | sole target guard, one-segment path | rejected `mutation.target-missing` |
| `↔️move-step` | `no-ops-when-the-step-is-already-at-that-index` | landing slot computed on the list with the step already removed | applied + `mutation.no-op` |
| `🧱add-block` | `rejects-adding-a-block-to-a-missing-step` | OUTER (step) guard → path truncates to one segment | rejected `mutation.target-missing` |
| `🗑️remove-block` | `rejects-removing-a-block-missing-from-its-step` | INNER (block) guard → two-segment path | rejected `mutation.target-missing` |
| `🔀move-block` | `rejects-moving-a-block-into-a-missing-step` | third guard, unique to this verb: destination step | rejected `mutation.target-missing` |
| `🔄replace-block` | `no-ops-when-the-block-is-already-identical` | whole-`PlaybookBlock` value equality | applied + `mutation.no-op` |
| `🩹update-step` | `no-ops-when-the-header-is-already-current` | conjunction of `title` AND `description` | applied + `mutation.no-op` |
| `✏️change-title` | `changes-the-playbook-title` | **real applied edit**, real diff | applied, no messages |

### 🎞️ animate / present (9)
| leaf | case | branch pinned | outcome |
| --- | --- | --- | --- |
| `🆕create-tile` | `rejects-a-duplicate-tile-id` | the vocabulary's only Fatal-by-identity | rejected `mutation.duplicate-id` |
| `🗑delete-tile` | `rejects-deleting-a-missing-tile` | singular delete target guard | rejected `mutation.target-missing` |
| `🧹delete-tiles` | `rejects-when-every-addressed-tile-is-missing` | the `missing.len() == ids.len()` THRESHOLD (a partial miss is instead an applied `mutation.partial`) | rejected `mutation.target-missing`, 3-segment path |
| `✏rename-tile` | `no-ops-when-the-tile-already-has-that-name` | name-scalar equality | applied + `mutation.no-op` |
| `✂resize-tile-crop` | `rejects-a-zero-width-crop` | positive-extent invariant (finiteness guard cleared first) | rejected `mutation.invariant` (Fatal) |
| `🔀reorder-tiles` | `no-ops-when-the-tile-is-already-at-that-index` | order arithmetic; asserts the order-bearing slot stays unset | applied + `mutation.no-op` |
| `🔁replace-tiles` | `no-ops-when-the-collection-is-already-empty` | whole-collection equality; the "clear" gesture | applied + `mutation.no-op` |
| `🖼replace-source` | `no-ops-when-the-source-is-already-identical` | whole-`FigureTileSource` equality (source and tiles share ONE handle) | applied + `mutation.no-op` |
| `🔲resize-source-frame` | `no-ops-when-the-frame-is-already-identical` | third guard, after both geometry invariants | applied + `mutation.no-op` |

### 🎬️ sequence (8)
| leaf | case | branch pinned | outcome |
| --- | --- | --- | --- |
| `🌱create-step` | `rejects-a-duplicate-step-id` | sole guard, Fatal | rejected `mutation.duplicate-id` |
| `🗑️delete-step` | `rejects-deleting-a-missing-step` | target guard BEFORE the edge cascade — asserts no `mutation.cascade` note | rejected `mutation.target-missing` |
| `📍move-step` | `no-ops-when-the-step-is-already-at-that-position` | coordinate identity (dyadic, exact `f64`) | applied + `mutation.no-op` |
| `🔧edit-step-params` | `no-ops-when-the-params-are-already-identical` | whole-`StepParams` dictionary equality | applied + `mutation.no-op` |
| `🗂️change-step-collapsed` | `no-ops-when-the-step-is-already-collapsed` | SETTER semantics, not toggle | applied + `mutation.no-op` |
| `🔗connect-steps` | `rejects-connecting-a-step-to-itself` | 4th of 5 guards: self-loop, addressed at the EDGE id | rejected `mutation.invariant` (Fatal) |
| `✂️disconnect-steps` | `rejects-disconnecting-a-missing-edge` | the only edge-addressed verb; searches `edges`, never `steps` | rejected `mutation.target-missing` |
| `🧬duplicate-step` | `rejects-when-the-new-id-already-exists` | source found, NEW id collides; diagnostic targets the destination | rejected `mutation.duplicate-id` |

## 🧾 Assertion set (seven per case, worded per mutation)
**Applied cases:** `applies_to_committed_after` (incl. "child handle not re-minted") ·
`produces_committed_diff` · `committed_diff_is_canonical` · `committed_diff_applies_to_after` ·
`committed_json_is_canonical` · `declared_outcome_holds` · a mutation-specific inverse assertion.

**Rejected cases:** `rejection_leaves_the_document_at_the_committed_after` · a named guard assertion
(code + `Severity` + exact target) · `the_committed_diff_is_declared_absent` (binds the empty
`🚫️component.absent` marker via `include_str!` AND re-checks the identity diff) ·
`committed_json_is_canonical` · `declared_outcome_holds` · a mutation-specific inverse assertion ·
`semantics_bind_this_fixture_to_<kind>` (descriptor + address).

The recipe's `inverse-restores-before` is kept verbatim only where it is actually true (the value-
identity no-ops, where a verb is its own inverse — playbook `move-step`/`replace-block`/`update-step`/
`change-title`, animate `rename-tile`/`reorder-tiles`/`replace-tiles`/`replace-source`/
`resize-source-frame`, sequence `move-step`/`edit-step-params`/`change-step-collapsed`). Where the
inverse is payload-derived it would *change* the document (e.g. `add-step`'s inverse removes a step
the refused add never created), so the slot instead carries a shape-level assertion on the inverse
plan — the same substitution `🕸️dag` made, and the more informative test either way: it pins the
BASE-derived vs PAYLOAD-derived split, which is where these vocabularies actually differ.

## 🔌 Wiring
Each case is mounted in its plugin's own `📦️packages/🦀️rust/📦️glue.rs`, immediately after that
leaf's `pub mod inverse;` line, at the same indentation:
```rust
#[cfg(test)]
#[path = "…/<leaf>/🧪️tests/<case>/🦀️component.rs"]
mod tests_<case_with_underscores>;
```
9 + 9 + 8 = 26 mounts added. No other file in any of the three plugins was touched.

## ✅️ Verification
- `bun ./📜️script.ts fixtures lint --by-tree` (from `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust`):
  `🧬️ 115 artifact mutation trees · 1558 mutations · 623 covered · 935 uncovered`. **None of the
  three trees in this slice appears in the uncovered list, and none appears in any error line** —
  every remaining error belongs to `🗄️stdio` / `🧰️framework` trees owned by other agents.
- Independent structural re-check of all 26 cases (file set per contract D6, outcome status/code
  well-formedness, `➡️after` byte-identical to `⬅️before` on every rejected case, zero-byte
  `🚫️component.absent`, JSON parses): **0 problems**.
- Every `include_str!` target in all 26 test files exists: **0 missing**.
- Every `#[path]` in all three `📦️glue.rs` files resolves: **0 missing** (checked over the whole
  file, not just the added lines).
- `rustfmt --edition 2021 --emit stdout` parses all 26 test files and all three `📦️glue.rs` files.
- `cargo` was **not** run — the workspace is broken by a peer's in-flight de-async sweep. No test in
  this slice is claimed to pass. Calls are written in the target de-async style (no `.await`),
  matching the committed puzzle5d reference.

## 🔭 Follow-ups for whoever owns child resolution
Once a real `LinkResolver`/`ChildContentView` seam reaches `MutationKind::diff`, every leaf in these
three trees becomes eligible for a second, content-changing case with a real committed `🔺️diff`.
Until then the `DefaultHasher` handle is unforgeable by hand and these guard-branch cases are the
honest maximum.
