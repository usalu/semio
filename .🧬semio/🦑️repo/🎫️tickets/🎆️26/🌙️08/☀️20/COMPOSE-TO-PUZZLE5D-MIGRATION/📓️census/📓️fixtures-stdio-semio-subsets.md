# 🧿️ Handcrafted mutation fixtures — the nine multi-mutation `🧿️semio` subsets in `🗄️stdio`

Slice: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/<subset>/🧬️schema/🧬️mutations`
for `✳️drawing` · `✳️mesh` · `✳️kit` · `✳️brep` · `✳️image` · `✳️graph` · `✳️object` · `✳️table` · `✳️text`.
**109 leaves · 109 cases · 545 committed JSON files · 109 handcrafted `🦀️component.rs` test files.**
One case per leaf, every case `applied`. No shared harness, no macro, no loop: each test file names
its own mutation, its own guard branches, its own diff shape and its own inverse.

## 🧹️ Scoped lint (same rules as `fixtures lint`, no 40-row truncation)

```
✳️drawing: 17/17 leaves covered · 4 error(s) · 136 derived-encoding warning(s)
   ❌️ ✳️drawing:Group: enum variant has no mutation directory
   ❌️ ✳️drawing:Ungroup: enum variant has no mutation directory
   ❌️ ✳️drawing:Flatten: enum variant has no mutation directory
   ❌️ ✳️drawing:Unflatten: enum variant has no mutation directory
✳️mesh:    17/17 leaves covered · 0 error(s) · 136 derived-encoding warning(s)
✳️kit:     15/15 leaves covered · 0 error(s) · 120 derived-encoding warning(s)
✳️brep:    13/13 leaves covered · 0 error(s) · 104 derived-encoding warning(s)
✳️image:   12/12 leaves covered · 0 error(s) ·  96 derived-encoding warning(s)
✳️graph:   11/11 leaves covered · 0 error(s) ·  88 derived-encoding warning(s)
✳️object:   9/9  leaves covered · 0 error(s) ·  72 derived-encoding warning(s)
✳️table:    8/8  leaves covered · 0 error(s) ·  64 derived-encoding warning(s)
✳️text:     7/7  leaves covered · 0 error(s) ·  56 derived-encoding warning(s)
TOTAL: 4 error(s), 872 derived-encoding warning(s)
```

Runner: `📓️census/📜️scoped-lint-semio-subsets.ts` (rules transcribed verbatim from
`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts`). Derived-encoding warnings are the expected
`fixtures generate` gap (contract D1/D11) — `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/
`.patch.semio` are NOT hand-forged.

Repo-wide `bun ./📜️script.ts fixtures lint --by-tree` now lists **no `🧿️semio` subset in the
uncovered-tree list** — the only remaining `--by-tree` row is `🧊️gltf` (41/120), another slice.

## ⛔️ The four residual `✳️drawing` errors are NOT fixture defects — and are NOT mine to fix

`SemioDrawingMutation` declares four variants whose names differ from the payload struct in the
corresponding leaf, and the lint pairs variants to leaves by `^pub struct <VariantName>`:

| enum variant | leaf | leaf's `pub struct` |
| --- | --- | --- |
| `Group(...)` | `🧷group` | `GroupNodes` |
| `Ungroup(...)` | `💫ungroup` | `UngroupNode` |
| `Flatten(...)` | `🫓flatten` | `FlattenNode` |
| `Unflatten(...)` | `🎈unflatten` | `UnflattenNode` |

All four leaves ARE present, complete and now covered by a fixture; the error is a pre-existing
schema-naming mismatch. Closing it means renaming either the four enum variants or the four payload
structs — which changes the externally-tagged serde wire name, the hand-rolled `OpText`/`OpBinary`
codecs, and the `🟦️component.ts`/`🔗️component.graphql`/`🛰️component.proto` mirrors. That is a
production schema decision outside fixture authoring, and stdio is under concurrent edit by the
de-async sweep, so it is reported rather than performed.

## ✅️ Structural verification (cargo NOT run; no test is claimed to pass)

- `include_str!` targets: **545/545 resolve** (109 cases × before/after/mutation/diff/outcome).
- `#[path]` targets: **109/109 wired test modules resolve**, plus every in-file leaf mount
  (`✳️image` mounts its own `🔺️diff` oracle per case, see below) — 0 missing.
- Per-subset wiring counts equal the leaf counts (17/17/15/13/12/11/9/8/7); no duplicate `mod` names.
- `rustfmt --edition 2021 --emit stdout`: **118/118 files parse** (109 test files + the 9
  mutations-root `🦀️component.rs`).
- All 545 committed JSON files parse.
- Static cross-checks over all 109 cases: `before != after`, diff never `{}`, outcome always
  `applied`, mutation JSON in the right serde shape, and for the four whole-list subsets
  (`text`/`table`/`graph`/`kit`) `diff.<slot>.values == after.<slot>`.
- Format-string audit: no unescaped `{`/`}` in any `assert!` message across the 109 files.

## 🧬️ Serde conventions actually verified per subset (nine subsets, several conventions)

- **Mutation enums** — `text`/`table`/`graph`/`object`/`brep`/`kit`/`mesh`/`drawing` derive
  `Serialize`/`Deserialize` with NO `#[serde]` attribute, so they are EXTERNALLY tagged:
  `{"InsertRun":{"index":1,…}}`. Payload structs carry no `rename_all`, so payload fields stay
  snake_case (`new_content`, `child_id`, `start_vertex`, `to_index`, `new_base_color`).
- **`✳️image` is the exception**: `#[serde(tag = "mutation", rename_all = "camelCase")]` — internally
  tagged with camelCase VARIANTS. It does NOT declare `rename_all_fields`, so struct-variant FIELDS
  stay snake_case: `{"mutation":"setBitDepth","bit_depth":16}`, `{"mutation":"setFrameDelay",
  "index":1,"delay_ms":200}`. See the drift note below.
- **Snapshot/diff structs** are `rename_all = "camelCase"` throughout (`startVertex`, `outerLoop`,
  `isVoid`, `typeId`, `materialId`, `baseColor`, `bitDepth`, `delayMs`, `strokeWidth`, `childId`).
- **Internally-tagged value enums** (`SemioValue` `kind`, `BrepCurve`/`BrepSurface` `kind`,
  `PathSegment`/`DrawNode`/`DrawNodeDiff` `kind`, `LinkPin` `kind`) — camelCase variants,
  snake_case fields (`radius_major`, `control_points`, `x_rotation`, `half_angle`).
- Every float in every fixture is dyadic (0.5 / 0.25 / 2.0 / 4.0 / -0.5 / 2.5 / 12.0 …) so the
  canonical-JSON assertion holds exactly. Quaternion rotations use the exactly-representable half
  turn `(0, 0, 1, 0)` rather than a quarter turn.

## ⚠️ `Option<Option<T>>` — pinned, never papered over

`Some(None)` encodes as an explicit `null` but JSON `null` decodes back into the OUTER `None`, so
such a diff is NOT a decode→encode fixed point. Four committed cases produce one:

- `✳️object/💥delete-brep`, `✳️object/🧨delete-mesh`, `✳️object/🚫delete-properties`
- `✳️kit/🚫delete-properties`

Their assertions 6 and 7 are rewritten rather than asserted falsely:
`committed_diff_json_pins_the_option_option_collapse` states the collapse exactly (decode yields
outer `None`; re-encode drops the key; the in-memory `Some(None)` diff DOES encode to the committed
JSON), and `authored_diff_applies_to_after_while_the_decoded_one_is_inert` exercises the apply law
against the in-memory diff while asserting the decoded one is inert. If serde's behaviour ever
changes, those tests fail and the fixtures get revisited.

Every OTHER tri-state slot in the slice deliberately takes the SET arm, whose inner value is a real
object/string and therefore round-trips intact: `✳️object`/`✳️kit` `create-*` children,
`✳️image/🎨️set-icc`, `✳️mesh/🔗set-primitive-material`, `✳️drawing` `🪣replace-fill` /
`🖌️change-stroke-color` / `📐change-stroke-width`.

## 🔗️ `✳️image` mounts its own leaf diff oracle

`📦️glue.rs` mounts only `📸️set-snapshot`'s triad for `✳️image`, and the enum-level
`Mutation::diff` arms carry NO guard branches — every `mutation.no-op`/`mutation.clamped`/
`mutation.target-missing`/`mutation.invariant` decision lives in the leaf's `🔺️diff/🦀️component.rs`.
Each `✳️image` case therefore mounts its own leaf with
`#[path = "../../🔺️diff/🦀️component.rs"] mod leaf_diff;`, destructures the committed mutation to
recover the payload fields, and asserts against `leaf_diff::diff(...)` — the real oracle — while
also cross-checking `apply_semio_image_mutation` reaches the same state. `📦️glue.rs` was NOT edited.

## 🧾️ Cases with a declared diagnostic

Three cascades are declared `applied` WITH an INFO `mutation.cascade` message (an INFO never turns
an applied mutation into a rejected one), and each test asserts the code AND the `Severity::Info`
level:

- `✳️table/🗑️delete-column` — drops the column and its cell in every row.
- `✳️graph/🗑️delete-node` — severs the edge that referenced the node.
- `✳️brep/🗑️delete-vertex` — removes both incident edges.

Every other case declares `{"status":"applied"}` with `messages().is_empty()` asserted, and its
docstring names the specific guard branches that must NOT fire for that base.

## 🧪️ Wiring

Each subset's own mutations-root `🦀️component.rs` gained a single appended
`//#region 🧪️FixtureTests` block:

```rust
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "<leaf-dir>/🧪️tests/<case>/🦀️component.rs"]
    mod tests_<leaf>_<case>;
    …
}
```

Same idiom as the already-landed `📕️din4108` precedent. Edits are strictly additive (appended at
end of file); no existing content was reformatted or reordered. `📦️glue.rs` was not touched.

## 🐞️ Drift found in passing (not fixed — outside this slice)

`✳️image`'s `🧬️mutations/🟦️component.ts` and `🧬️mutations/🔣️component.json` declare `bitDepth` and
`delayMs` for the `setBitDepth`/`setFrameDelay` payload fields, but the Rust enum emits `bit_depth`
and `delay_ms` (serde 1.0.228: `rename_all` on an enum renames VARIANTS only; `rename_all_fields`
would be required for the fields, and is used elsewhere in the framework — e.g.
`🧰️framework/🔨️modules/🎭️actor/🦀️component.rs`). The committed fixtures follow the RUST encoding,
which is what the canonical-JSON assertion pins. Reconciling the mirrors (or adding
`rename_all_fields = "camelCase"` to `SemioImageMutation`) is a separate change.
