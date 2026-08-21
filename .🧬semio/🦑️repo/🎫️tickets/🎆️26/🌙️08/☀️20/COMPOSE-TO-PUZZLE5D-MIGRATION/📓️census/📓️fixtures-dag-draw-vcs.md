# 🧪️ Handcrafted mutation fixtures — `🕸️dag` (14) · `🖍️draw` (14) · `🌿️vcs` (6)

Slice: the three mutation trees below, plus each plugin's `📦️packages/🦀️rust/📦️glue.rs`.
34 leaves, 34 cases, one case per leaf. `🚪️io/🧬️mutations` is EMPTY in all three plugins — every
leaf lives under `🧬️schema/🧬️mutations`.

Verification run (no `cargo` — workspace broken by the peer de-async sweep, contract note in
`📓️fixture-recipe.md`):

| check | result |
|---|---|
| `fixtures lint --by-tree` | all three trees gone from the uncovered list; `205 covered` (was 28); zero error findings naming dag/draw/vcs |
| `include_str!` targets | 136/136 resolve |
| glue `#[path]` targets | 6/6 vcs · 14/14 draw · 14/14 dag resolve |
| `rustfmt --edition 2021 --emit stdout` | 34 test files + 3 `📦️glue.rs` + 2 mutation roots all parse |

---

## 🔧️ Two shared-file edits (both inside the slice)

`draw` and `vcs` had **no** artifact-level apply/inverse entry point, unlike dag
(`apply_dag_mutation`) and puzzle5d (`apply_puzzle5d_mutation`). Rather than inline 20 copies of the
trait dance into the test files, one `🔖️Apply` region was added to each mutations root, verbatim in
dag's shape:

- `…/🖍️draw/…/🧬️schema/🧬️mutations/🦀️component.rs` → `apply_draw_mutation` / `inverse_draw_mutation`
- `…/🌿️vcs/…/🧬️schema/🧬️mutations/🦀️component.rs` → `apply_vcs_mutation` / `inverse_vcs_mutation`

⚠️ Note for every consumer: like dag's own helper, these apply the outcome's **diff**, so a
*rejecting* mutation still returns `Ok(())` with the snapshot untouched. Rejection is read from
`MutationOutcome::messages`, never from `is_ok()`. Every rejected case in this slice asserts through
the messages.

---

## 🌿️ vcs — 6 leaves, 6 applied cases

Base snapshot (one small `VcsSnapshot` per case, same shape):

```json
{ "schema": "vcs.vcs", "title": "Fixture Base", "counter": 3,
  "notes": "Initial notes.", "status": "draft", "tags": ["review"] }
```

`tags` carries `#[serde(default)]` with no `skip_serializing_if`, so it is always on the wire.
Enum shape: `#[serde(tag = "mutation", rename_all = "camelCase")]` → `{"mutation":"renameVcs", …}`.

| leaf | case | after |
|---|---|---|
| `✏️rename-vcs` | `retitles-the-document` | `title` → `"Retitled Fixture"` |
| `🔢change-counter` | `sets-counter-to-seven` | `counter` → `7` (absolute set, asserted **not** an increment) |
| `📝change-notes` | `rewrites-the-notes` | `notes` replaced wholesale |
| `🚦change-status` | `draft-to-review` | `status` → `"review"` |
| `🏷️add-tag` | `appends-urgent-tag` | `tags` → `["review","urgent"]` (delta appends at the END) |
| `🗑️remove-tag` | `detaches-the-review-tag` | from `["review","urgent"]` → `["urgent"]` |

### Rejection / no-op codes found in vcs
- `🏷️add-tag` — `Warning mutation.no-op` when BASE already carries the tag (inverse then empty).
- `🚦change-status`, `🔢change-counter`, `📝change-notes`, `✏️rename-vcs` — `Warning mutation.no-op`
  when the requested value already equals BASE's. **No Error, no Fatal path at all.**
- `🗑️remove-tag` — the facet's **only** `Error mutation.target-missing`, at `[tag]`.
- Apply-level (in `🚪️io/🔺️diff/📝️text::apply_tags_delta`, reached only via a hand-built diff, not
  via these mutations): `mutation.apply.missing-target`, `mutation.apply.duplicate-target`,
  `mutation.apply.conflicting-target`.

⚠️ `remove-tag`'s inverse (`add-tag`) restores **membership, not position** — the delta appends. The
fixture asserts set membership and count, deliberately not snapshot equality; documented in the test.

---

## 🖍️draw — 14 leaves, 13 applied + 1 rejected

`DrawSnapshot` is fully self-describing JSON (`schema`/`id`/`title`/`layers`/`assets`/`artboard`), so
every after-state here is derived by hand from the diff builder. Layers are internally tagged
(`#[serde(tag="kind")]`) with a `#[serde(flatten)] base`. Bases are kept minimal per case: a single
shape (`shape-a`) for the base-field verbs, plus a boolean, a trace, a group-with-child and a
two-shape root where the mutation needs them.

| leaf | case | outcome / after |
|---|---|---|
| `👁️set-layer-visible` | `hides-shape-a` | applied · `visible` → false, locked/opacity/blend untouched |
| `🔒️set-layer-locked` | `locks-shape-a` | applied · `locked` → true, `visible` stays true |
| `🌫️set-layer-opacity` | `dims-shape-a-to-half` | applied · `opacity` → 0.5, fill alpha untouched |
| `🖌️set-layer-blend-mode` | `normal-to-multiply` | applied · `blendMode` → `"multiply"` |
| `✏️rename-layer` | `renames-shape-a-without-touching-its-id` | applied · `name` changes, `id` does not |
| `🔄️update-layer-transform` | `translates-and-scales-shape-a` | applied · whole transform facet swapped, shape `rect` untouched |
| `🔁replace-layer-fill` | `solid-to-linear-gradient` | applied · fill VARIANT swapped, stroke stays `None` |
| `♻️replace-layer-stroke` | `adds-a-dashed-stroke` | applied · `None → Some(..)` incl. optional `dash` |
| `🔀set-layer-boolean-operation` | `union-to-subtract` | applied · `operation` on the Boolean variant, `children` untouched |
| `🔧update-layer-trace-params` | `sharpens-the-trace` | applied · both params move together, `sourceKey` untouched |
| `🌱create-layer` | `appends-shape-b-at-the-root` | applied · omitted `parentId`/`index` resolve to root-append at BASE's root length |
| `🧬️duplicate-layer` | `rejects-a-missing-source-layer` | **rejected** `mutation.target-missing` `["shape-missing"]` |
| `🗑️delete-layer` | `removes-group-a-with-its-child` | applied · group subtree removed, child not reparented |
| `🔃reorder-layer` | `moves-shape-a-above-shape-b` | applied · remove+insert at FINAL index 1, `reordered` stays `None` |

### Why `duplicate-layer` is the one rejected draw case
`clone_draw_layer_node` mints the copy's id through `create_draw_id` → `draw_id_hex` →
`std::collections::hash_map::DefaultHasher`. A hand-authored `➡️after` would have to embed that
digest, i.e. hand-forge a value from `std`'s deliberately unspecified default hasher — the same class
of forbidden hand-reimplementation the recipe bans for the binary codecs. The `target-missing` branch
reaches no hash, so that is the branch the fixture pins. **A successful-duplicate case should be
added once the fixture generator can produce derived encodings from real code.**

### Rejection / no-op codes found in draw
- `Error mutation.target-missing` at `[layer_id]` — every one of the 14 verbs except `create-layer`.
  `reorder-layer` has a **second** target-missing at `[parent_id]` for a missing new parent.
- `Fatal mutation.invariant` — `set-layer-opacity` (non-finite), `update-layer-transform` (non-finite
  **or** non-positive scale), `create-layer` (parent absent or not a group).
- `Fatal mutation.duplicate-id` — `create-layer` (id collision), `duplicate-layer` (the minted copy
  id collides).
- `Warning mutation.no-op` — `set-layer-visible`, `set-layer-locked`, `set-layer-opacity`,
  `set-layer-blend-mode`, `rename-layer`, `update-layer-transform`, `replace-layer-fill`,
  `replace-layer-stroke`, `reorder-layer`; plus `set-layer-boolean-operation` and
  `update-layer-trace-params`, which fire it **only for the matching variant** (a Boolean/Trace layer
  already holding the requested value). `create-layer`, `duplicate-layer` and `delete-layer` have no
  no-op branch.
- Apply-level (`DrawDiff::apply` → `apply_layers_delta` / `apply_assets_delta` / `apply_layer_patch`):
  `mutation.apply.missing-target`, `mutation.apply.duplicate-target`,
  `mutation.apply.conflicting-target`, `mutation.apply.invalid-index`,
  `mutation.apply.invalid-order`, `mutation.apply.invalid-value`, `mutation.apply.invalid-target`.

⚠️ Non-obvious: `set-layer-boolean-operation` / `update-layer-trace-params` applied to a
*non-matching* layer kind produce a **successful diff** whose patch then fails at apply time with
`mutation.apply.invalid-target`. Their `inverse` returns `Vec::new()` for a non-matching kind. That
mismatch is real behaviour, not covered by these fixtures, and is worth a follow-up case.

---

## 🕸️dag — 14 leaves, 14 rejected cases

### ⚠️ The blocking constraint (read this before extending the tree)
`DagSnapshot` has exactly two persisted fields: `schema` and `content: ArtifactChild<SemioGraphSnapshot>`.
Nodes and edges live in the composed `s.stdio.semio.graph` **child**, and `ArtifactChild` serializes
as `{childId, target}` only. Consequences:

1. A committed `DagSnapshot` JSON **cannot express a graph**. Decoding one standalone yields an
   unresolved handle; `dag_working_scene` fails soft to an EMPTY scene (documented in the plugin's
   own `🔖️WorkingScene` region — no `LinkResolver`/child-dispatch seam exists in any WASM-guest
   plugin yet).
2. Every **applied** dag diff goes through `diff_replace_content` →
   `dag_content_child_handle_and_cache`, which mints `dag-content-{DefaultHasher(json):016x}`. A
   hand-authored `➡️after` for any applied case would have to embed that digest — hand-forging a
   value from `std`'s unspecified default hasher, which is unverifiable here and is the forbidden
   parallel-implementation pattern.

Therefore **every dag case pins a rejection branch**, where the diff is `DagDiff::default()`, the
`content` slot is never touched, and `➡️after` is byte-identical to `⬅️before`. Each case carries
`🔺️diff/🚫️component.absent` per contract D6.

Base snapshot (13 cases), with a deliberately non-hash-shaped child id so nothing reads as a forged
digest:

```json
{ "schema": "dag.dag",
  "content": { "childId": "dag-content-unresolved-fixture",
    "target": { "artifactId": "dag-content",
      "dialect": { "artifactKind": "s.stdio.semio", "standard": "v1", "subset": "graph" } } } }
```

| leaf | case | code · target |
|---|---|---|
| `🌱create-node` | `rejects-a-duplicate-node-id` | **Fatal** `mutation.duplicate-id` · `["node-a"]` |
| `🗑️delete-node` | `rejects-deleting-a-missing-node` | Error `mutation.target-missing` · `["node-a"]` |
| `🏷️rename-node` | `rejects-renaming-a-missing-node` | Error `mutation.target-missing` · `["node-a"]` |
| `🔤change-node-name` | `rejects-renaming-the-label-of-a-missing-node` | Error `mutation.target-missing` · `["node-a"]` |
| `↔️move-node` | `rejects-moving-a-missing-node` | Error `mutation.target-missing` · `["node-a"]` |
| `📐resize-node` | `rejects-resizing-a-missing-node` | Error `mutation.target-missing` · `["node-a"]` |
| `🖼️change-node-icon` | `rejects-reiconing-a-missing-node` | Error `mutation.target-missing` · `["node-a"]` |
| `🔡change-node-abbreviation` | `rejects-reabbreviating-a-missing-node` | Error `mutation.target-missing` · `["node-a"]` |
| `🧮change-node-operator-kind` | `rejects-rebinding-the-operator-of-a-missing-node` | Error `mutation.target-missing` · `["node-a"]` |
| `🔁replace-node-kind` | `rejects-rekinding-a-missing-node` | Error `mutation.target-missing` · `["node-a"]` |
| `🗃️replace-node-properties` | `rejects-repropertying-a-missing-node` | Error `mutation.target-missing` · `["node-a"]` |
| `🔀reorder-nodes` | `rejects-a-duplicate-id-in-the-order` | **Fatal** `mutation.invariant` · `["node-a"]` |
| `🔗connect-nodes` | `rejects-a-missing-source-node` | Error `mutation.target-missing` · `["node-a"]` (the SPLIT source node, not `node-a@out`, not `edge-1`) |
| `✂️disconnect-nodes` | `rejects-disconnecting-a-missing-edge` | Error `mutation.target-missing` · `["edge-1"]` (edge id) |

`🌱create-node` is the only dag verb with no rejection against an empty scene. Its case seeds the
working-scene cache for the committed handle (`dag-content-seeded-fixture`) with **the very node the
committed mutation JSON carries** — nothing invented — producing the `duplicate-id` collision. That
seeding is the one deviation from "JSON is the whole source of truth" in this slice, and it is
documented in the test's module docstring.

### What makes each dag test mutation-specific (they are rejections, not clones)
- its own `SemanticDescriptor` tuple (`verb`/`entity`/`kind`/`record`) is asserted;
- its own inverse contract is asserted, and these genuinely differ:
  `create-node` → always `delete-node(payload.id)`; `connect-nodes` → always
  `disconnect-nodes(payload.id)`; `reorder-nodes` → always a counter-reorder to BASE's order (here
  the EMPTY order); all eleven others → `Vec::new()`;
- guard-ORDER assertions where a second guard exists: `move-node` (NaN payload still reports
  target-missing, so the lookup precedes the finite invariant), `resize-node` (zero extent, same),
  `change-node-icon` (empty icon), `change-node-operator-kind` (`None` unbind),
  `replace-node-properties` (empty bag);
- `delete-node` additionally asserts that **no** `mutation.cascade` Info accompanies a miss;
- `reorder-nodes` asserts `target()` is empty — it is the vocabulary's only collection-scoped verb.

### Full dag rejection / no-op / info code census
- `Error mutation.target-missing` — `delete-node`, `rename-node`, `change-node-name`, `move-node`,
  `resize-node`, `change-node-icon`, `change-node-abbreviation`, `change-node-operator-kind`,
  `replace-node-kind`, `replace-node-properties`, `disconnect-nodes` (edge id),
  `connect-nodes` (source node, then target node), and — uniquely — `reorder-nodes` emits it as a
  **non-blocking** absorbed message listing unknown ids **alongside a real diff**.
- `Fatal mutation.duplicate-id` — `create-node` (node id), `rename-node` (new id already taken),
  `connect-nodes` (edge id already taken).
- `Fatal mutation.invariant` — `move-node` (non-finite x/y), `resize-node` (non-finite or
  non-positive extent), `reorder-nodes` (duplicate ids in the order), `connect-nodes` (self-loop;
  would-create-cycle).
- `Warning mutation.no-op` — `rename-node` (`new_id == id`), `change-node-name`, `change-node-icon`,
  `change-node-abbreviation`, `change-node-operator-kind`, `replace-node-kind`,
  `replace-node-properties`, `move-node`, `resize-node`, `reorder-nodes` (resulting order equals the
  current one), `connect-nodes` (a parallel edge between the same node pair). `create-node`,
  `delete-node` and `disconnect-nodes` have no no-op branch.
- `Info mutation.cascade` — `delete-node` only, listing the edge ids it severed.

---

## ❓️ Could not determine / deliberately out of scope

1. **No test is claimed to pass.** `cargo` is unusable at this pin (peer de-async sweep). Validation
   was structural only: lint coverage, `include_str!` resolution, glue `#[path]` resolution, and
   `rustfmt` parsing. In particular the serde round-trip assertions (`committed_json_is_canonical`)
   are reasoned from the `#[serde(...)]` attributes on each type, not executed. The two places most
   worth re-checking the moment the workspace builds: draw's internally-tagged
   `DrawLayerNode` + `#[serde(flatten)] base` pairing, and dag's `DagNodeSpec` flattened
   `DagNodeKind`.
2. **Applied dag coverage is blocked, not skipped.** It needs either child-document resolution (so a
   snapshot can carry its graph) or a fixture generator that derives `➡️after` from real code. Until
   then, hand-authoring a dag applied case means forging a `DefaultHasher` digest.
3. **`duplicate-layer`'s success path is likewise blocked** on the same hasher problem
   (`create_draw_id`).
4. The `🚪️io/🧬️mutations` roots in all three plugins are empty directories — no leaves, nothing to
   cover; the lint agrees.
5. `dag`'s `📦️glue.rs` already carried one unrelated `🧪️tests` `#[path]` before this pass (an
   examples test); the 14 added ones bring its total to 15.

---

# 🔺️ Follow-up pass — the serialized diff (`🔺️diff/🔣️component.json`)

The dev ruled the serialized diff the highest-value file in a case, and the lint moved it from
`DERIVED_CASE_FILES` into `CORE_CASE_FILES` (still skipped for `rejected`, which keeps needing
`🔺️diff/🚫️component.absent`). This pass closed that gap across the slice.

| tree | applied cases → diff JSON authored | rejected cases → `.absent` (already present) |
|---|---|---|
| `🌿️vcs` | 6 | 0 |
| `🖍️draw` | 13 | 1 (`duplicate-layer`) |
| `🕸️dag` | 0 | 14 |
| **total** | **19** | **15** |

Result: `fixtures lint --by-tree` → all three trees absent from the uncovered list, **zero** error
findings naming them; repo-wide covered 205 → 209. A slice-scoped audit re-implementing `lintCase`'s
own rules over the 34 cases reports `applied=19 rejected=15 errors=0`. `include_str!` targets now
155/155 (136 + the 19 new `DIFF` consts). `rustfmt --edition 2021` parses all 34 test files.

## Assertions added

**Applied cases (19)** got recipe items 5–7, each with its own docstring and its own extra structural
assertion — no shared helper, and every message names its mutation:
- `produces_committed_diff` — `Mutation::diff(...).diff()` re-encodes to exactly the committed JSON,
  plus a per-mutation negative pin (e.g. `set-layer-opacity` asserts `fillJson` stays `null`;
  `reorder-layer` asserts `reordered` stays `None`; `create-layer`/`delete-layer`/`reorder-layer`
  assert the untouched sibling id appears **nowhere** in the committed diff text).
- `committed_diff_is_canonical` — the file decodes to `VcsDiff`/`DrawDiff` and re-encodes identically.
- `committed_diff_applies_to_after` — `MutationDiff::apply(&committed, &before) == after`, so the diff
  is a complete description of the change rather than a summary of it.

**Rejected cases (15)** carry the equivalent pin instead, and already did before this pass — verified
mechanically across all 15: each asserts `produced.diff() == &<Artifact>Diff::default()`, the exact
`code.0`, the exact `Severity`, and the exact `target`. That is the reason `.absent` is committed
rather than an invented empty patch: there is no diff to serialize, only a diagnostic.

## 🔍️ What is surprising about the three diff types

1. **`VcsDiff` and `DrawDiff` both serialize EVERY field.** Container-level `#[serde(default)]` with
   no `skip_serializing_if` on any field means untouched lanes are committed as explicit `null` — 9
   keys for `VcsDiff`, 15 for `DrawDiff`. This is what gives the file its teeth: the `"artifact": null`
   lane is the whole-document-replace escape hatch, so a mutation that reached for it could not hide.
2. **`DrawLayerAddition.parent_id` is the single exception in the family** — it carries
   `skip_serializing_if = "Option::is_none"`, so a root insert **omits the key entirely** while every
   sibling lane writes `null`. `create-layer` and `reorder-layer` are the two cases that show it.
3. **Draw's structured values travel as JSON-blob STRINGS, not nested objects.** `DrawLayerPatch`
   carries `transform_json` / `fill_json` / `stroke_json` / `trace_params_json` / `layer_json` as
   `Option<String>` produced by `serde_json::to_string`. The committed diffs therefore embed escaped
   compact JSON, e.g.
   `"transformJson": "{\"x\":24.0,\"y\":-8.0,\"scaleX\":2.0,\"scaleY\":1.5,\"rotation\":0.0}"`.
   Two consequences worth flagging: floats must carry `.0` (serde_json/ryu always emits it, which is
   why the blobs were hand-written rather than generated through `JSON.stringify` — JS cannot
   represent the distinction); and `stroke_json`/`fill_json` hold a serialized **`Option`**, so
   *clearing* a stroke is the literal four-character string `"null"`, not an absent lane. Each blob in
   this slice was cross-checked to parse and to deep-equal both the mutation payload's own sub-object
   and the corresponding value in `➡️after`.
4. **`DagDiff` has no per-collection delta at all** — just `content: Option<DagContentChild>`, because
   the composed child is opaque and every dag mutation replaces the whole handle. So dag has nothing
   diff-shaped to commit even in principle for an applied case: its diff would be one content-addressed
   `child_id`, i.e. the `DefaultHasher` digest this slice cannot forge. All 14 dag cases stay on
   `.absent`, and their pin is `DagDiff::default()` plus the code/severity/target triple.
5. **`DrawDiff`'s double options are a latent round-trip trap.** `title: Option<Option<String>>` and
   `artboard: Option<Option<DrawArtboard>>` use the outer `None` for "untouched" and `Some(None)` for
   "cleared" — but serde serializes BOTH to a bare `null`, and deserializing `null` yields the outer
   `None`. A future *clear-the-title* or *clear-the-artboard* diff is therefore **not**
   JSON-round-trippable and would fail `committed_diff_is_canonical` through no fault of the fixture.
   No mutation in this slice writes either lane (all 19 commit a plain `null` meaning untouched), so
   nothing here is affected; whoever adds such a case needs a `serde_with`-style double-option
   representation on those two fields first.
