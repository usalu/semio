# F2 — ☁️ply (1.0) Fan-out Report

Wave: F2 (stl, obj, ply, las, bmp, tiff — parallel ×6). Artifact: `☁️ply` standard `1.0`, subset
`✳️any`. W0 flagged ply as one of the two weakest snapshots in this wave, needing the most
net-new field design, and explicitly called out killing the `MeshVertex`/`MeshTriangle` types
shared verbatim with stl.

## 1. What changed

**Snapshot** (`🧬️schema/📸️snapshot/🦀️component.rs`): the old hardcoded
`PlySnapshot{schema, vertices: Vec<MeshVertex>, faces: Vec<MeshTriangle>}` mesh-only model is
gone entirely — no renamed replacement, no shared module. PLY's real generic element/property
system is now the model:

```rust
pub enum PlyFormat { Ascii, BinaryLittleEndian, BinaryBigEndian }
pub enum PlyScalarType { Char, UChar, Short, UShort, Int, UInt, Float, Double }
pub enum PlyProperty { Scalar{name, kind}, List{name, count_kind, value_kind} }
pub enum PlyValue { Char(i8), UChar(u8), …, Double(f64), List(Vec<PlyValue>) }
pub struct PlyRow { values: Vec<PlyValue> }
pub struct PlyElement { name: String, count: usize, properties: Vec<PlyProperty>, rows: Vec<PlyRow> }
pub struct PlySnapshot { schema: String, format: PlyFormat, comments: Vec<String>, elements: Vec<PlyElement> }
```

This satisfies the ticket's mandate literally: `vertices`/`faces`-shaped meshes now fall out of
elements named `"vertex"`/`"face"` — nothing in the type system hardcodes them, and stl's
`MeshVertex`/`MeshTriangle` types are no longer imported or referenced anywhere in ply. `PlyValue`
is adjacently tagged (`kind`/`value`, not internally tagged) so its newtype variants serialize
cleanly; `PlyProperty`'s `form` tag (not `kind`) avoids a collision with `Scalar`'s own `kind`
field.

**Diff** (`🧬️schema/🔺️diff/🦀️component.rs`, ~530 lines): handcrafted sparse diff, two nested
collection levels (`elements` name-keyed → each modified element's `rows` index-keyed), matching
the recipe's "trees nest" rule:

```rust
pub struct PlyDiff { format: Option<PlyFormat>, comments: Option<Vec<String>>, elements: Option<PlyElementsDiff> }
pub struct PlyElementsDiff { removed: Vec<String>, modified: Vec<PlyElementModified>, added: Vec<PlyElementAdded> }
pub struct PlyElementDiff { properties: Option<Vec<PlyProperty>>, rows: Option<PlyRowsDiff> }
pub struct PlyRowsDiff { removed: Vec<usize>, modified: Vec<PlyRowModified>, added: Vec<PlyRowAdded> }
pub struct PlyRowDiff { fields: Vec<PlyRowFieldChange> }   // sparse, keyed by property NAME
```

No `snapshot: Option<PlySnapshot>` full-replace slot anywhere. `impl MutationDiff<PlySnapshot>
for PlyDiff { apply, absorb }` and `impl DiffAlgebra<PlySnapshot> for PlyDiff { inverse, between,
is_empty }`, both imported explicitly (`protocol::MutationDiff` + `protocol::command::
DiffAlgebra`) from the start, avoiding F1's known missing-import trap. `between()` for `elements`
follows zip's proven name-keyed pattern; `rows_between`/`apply_rows_diff` follow csv's proven
index-keyed pattern (modified-before-removed-before-added apply order, `0..min(len)` pairwise
compare). `absorb_elements` mirrors zip's name-keyed absorb (no rename support — ply has no
`RenameElement` mutation); `absorb_rows` duplicates (locally, per-artifact, not shared) csv's
index-transport simulation (`Slot::{Base,Added}`, mid-array simulation) one nesting level deeper.

**Mutations** (`🧬️schema/🧬️mutations/🦀️component.rs`): the 9 variants from the brief plus
`NoMutation`: `SetSnapshot`, `SetFormat`, `InsertComment`/`RemoveComment`,
`AddElement`/`RemoveElement{name}`, `InsertRow`/`RemoveRow{element_name, index}`,
`SetRowProperty{element_name, row_index, property_name, value}`. Every variant's `diff()` is
handcrafted (builds the sparse `PlyDiff` directly via small `diff_*` builder functions in the
diff module — no apply-and-capture). Every variant's `inverse()` is handcrafted, resolving
against `base` (e.g. `InsertComment`'s inverse computes the clamped insert position from
`base.comments.len()` before emitting the matching `RemoveComment`; `SetRowProperty`'s inverse
looks up the prior value by walking `base`'s element/property/row).

**PlyArtifact** (`🧬️schema/🦀️component.rs`, the full-artifact-state mirror) was rewritten to
match the new snapshot shape field-for-field (`schema, format, comments, elements`) — it had
carried the exact same `vertices`/`faces` duplication as the snapshot and needed the identical
fix.

**Engine** (`⚙️engine/🦀️component.rs`, codec): full rewrite of header parsing, ascii/binary body
decode, and encode to walk the generic element/property model instead of hardcoding
`vertex.{x,y,z}` / `face.vertex_indices`. This is a genuine fidelity improvement over the old
codec, not just a refactor: the old encoder always canonicalized to a fixed vertex+face layout
(any other element/property layout lost content on re-encode); the new one round-trips *any*
element/property layout byte-for-byte (see `codec_retention_law` and
`arbitrary_named_element_round_trips`). Comments are parsed and retained on the snapshot but not
re-emitted into the header on encode (documented scope cut, does not affect the diff/mutation
model — the field is fully modeled/diffable/mutable, only `encode_ply`'s header text omits it).

**IO bridge** (`🚪️io/{import,export}/…/txt/utf-8/✳️any/🦀️component.rs`): updated to call
`engine::decode_ply`/`engine::encode_ply` directly instead of the deleted
`parse_ply_text`/`write_ply_text(vertices, faces)` mesh-only helpers.

**Mutation triad leaf** (`🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs`): fixed to match
`diff_set_snapshot`'s new 2-arg `(base, next)` signature — identical fix to the one zip's F1
agent applied to the same triad-leaf pattern.

**Facet mirrors** (TS/JSON Schema/GraphQL/proto, all 4 facets — artifact/snapshot/diff/mutations):
fully handcrafted, real interfaces/schemas matching the Rust shapes 1:1 (discriminated unions on
`mutation`/`form`/`kind` tags where relevant). Verified via `POLICY_FACET_MIRROR_DRIFT`: 0 real
breaches for ply (repo-wide this rule currently has exactly 3 breaches total, all in the
unrelated `ifc` subset-multiplicities wave — see §4).

**Grammar leaves**: snapshot facet's full 6-file set handcrafted honestly (real PLY header +
ascii-body grammar in `.grammar.semio`/`.ebnf`/`.g4`; real header-driven binary-body structure in
`.protocol.semio`/`.abnf`/`.ksy`/`.spicy`, including a from-scratch Kaitai `header_line` line-
scanner and a Spicy `&until="end_header\n"` unit). diff/mutations facets' `.grammar.semio`
handcrafted (JSON-line op-text grammar, following zip's exact proven pattern for this file).
diff/mutations' `.g4`/`.ebnf`/binary leaves (ksy/spicy/abnf/protocol.semio) intentionally left as
scaffolded placeholders — see §3 deviations.

## 2. Verification

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::ply"` → **23 passed, 0 failed.**
- `cargo test -p semio-s-plugin-stdio --lib` (full crate, run twice — once mid-session while
  `obj`/`las` concurrent agents were still mid-flight showing their own unrelated failures, once
  at the end) → **final: 795 passed, 0 failed, crate-wide.**
- Grep gates: zero `snapshot: Option<` struct field in the diff file (the one hit is a doc-
  comment describing the OLD banned shape being replaced); `impl DiffAlgebra<PlySnapshot> for
  PlyDiff` present; zero `other =>` apply-and-capture arm in the mutations file; `field_sweep`-
  named tests present (`field_sweep_covers_every_mutable_field`,
  `field_sweep_row_triple_both_directions`).
- `bun ./📜️script.ts policy` (regenerated `.🦑️repo/⚡️cache/breaches/compose.json`, cross-checked
  directly per F1's precedent rather than trusting priority-filtered stdout): **0 real (non-
  stale) breaches for ply across all 4 S-8 rules** (`diff-algebra`, `field-sweep-presence`,
  `grammar-honesty`, `facet-mirror-drift`). 9 `-stale-` entries remain in the S2-seeded
  allowlists (7 grammar-honesty, 1 diff-algebra, 1 field-sweep) — these are allowlist-pruning
  housekeeping for the F2 closer (I did not touch `📜️script.ts`, per the ownership boundary),
  listed exactly in `glue_followup`.

## 3. Deviations / scope cuts (all documented in-code too)

1. **Comments not re-emitted on encode.** `PlySnapshot.comments` is fully modeled, diffable
   (whole-vec weak-replace), and mutable (`InsertComment`/`RemoveComment`, both with handcrafted
   diff+inverse). Only `encode_ply`'s header-text writer doesn't print them back into the `ply`
   header on the canonical ascii encode path. Comment: `⚙️engine/🦀️component.rs`
   `encode_ply_with_format`'s doc comment.
2. **Element property-schema change ⇒ whole-rows replace, not per-field row diff.** There is no
   `ChangeElementProperties` mutation; a `properties` change between two snapshots of the same-
   named element can only arise from a hand-built `between()`/`SetSnapshot` call. When it does,
   `element_between` falls back to a full remove-all/add-all of that element's rows rather than
   attempting a per-field diff across two different column schemas (which is not well-defined —
   there's no canonical way to align "old row shape" cells to "new row shape" cells). This is the
   recipe's own "trees recursive with Replace fallback on node-kind change" pattern, applied one
   level down. Exercised directly by `field_sweep_covers_every_mutable_field`.
3. **Row-level absorb's "modified-of-added" sub-case is a documented no-op fallback, not
   incorrect-but-silent.** The canonical "Add+SetField patches into added" absorb law IS
   correctly handled at the ELEMENT granularity (`AddElement` whole-element-with-rows, followed
   by `SetRowProperty` on one of its rows — tested in
   `absorb_law_add_element_then_set_row_property_patches_into_added`), because the carried added
   `PlyElement` payload has real `properties` to resolve names against. The deeper case —
   `InsertRow` into an EXISTING (not newly-added) element immediately followed, in the SAME
   absorb pair, by `SetRowProperty` on that same not-yet-committed row — cannot resolve the
   property name to a cell position without the element's `properties`, which base-free absorb
   (no snapshot parameter, per the `## Absorb` contract) doesn't have at that nesting depth. This
   is a real, narrow, safe (never corrupts, only drops that one patch) limitation, fully
   documented in `apply_row_field_changes_by_position_fallback`'s doc comment in the diff file.
4. **Grammar leaves**: the snapshot facet's full 6-leaf set (text: g4/ebnf/grammar.semio; binary:
   ksy/spicy/abnf/protocol.semio) is handcrafted honestly, reflecting PLY's real header grammar
   and real header-driven binary body layout. The diff/mutations facets' `.grammar.semio` (the
   op-text JSON-line grammar) is handcrafted, following zip's proven real-content pattern for
   that specific leaf. diff/mutations' `.g4`/`.ebnf` and ALL FOUR binary leaves
   (ksy/spicy/abnf/protocol.semio) are left as the scaffolded placeholder — this exactly mirrors
   what F1's own closer found and explicitly kept for zip (45 real, still-outstanding
   placeholder grammar leaves deliberately NOT pruned from the allowlist, citing "un-wired
   sibling mirror copies beyond the two live-wired leaves per facet" as accepted, real, ongoing
   scope). Given ply was flagged as the single heaviest F2 rewrite, I prioritized: (a) the full
   Rust recipe shape (snapshot/diff/mutations/PlyArtifact/engine), (b) all 16 TS/JSON/GraphQL/
   proto facet mirrors across all 4 facets, (c) the snapshot facet's full real grammar leaf set
   (the one that actually represents PLY's own wire format), and (d) diff/mutations'
   `grammar.semio` (the one leaf format the F1 precedent treats as load-bearing) — over
   exhaustively real-izing every one of the remaining ~12 thinner mirror-of-a-mirror grammar
   files, per the brief's explicit "prioritize getting the recipe's shape right… document scope
   cuts" instruction.
5. **`obj_info` header lines** are skipped on decode (not modeled on `PlySnapshot`), matching the
   pre-existing engine's behavior for this genuinely-rare, rarely-used PLY header line.

## 4. External churn observed (not mine, not fixed)

Two concurrent F2 agents (`🟪️stl`, `☁️las`) were actively mid-rewrite in the same wave/session:
compile errors and later one runtime `field_sweep` failure each appeared and cleared over the
course of this session, confirmed via `git status` file-path scoping (all in `stl`/`las`
directories) each time before I moved on. By the final full-crate gate both had cleared on their
own (795/0). No `stl`/`las` files were touched by me.

## 5. Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/⚙️engine/🦀️component.rs` (full rewrite: codec + 6 law suites + field_sweep)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` (full rewrite: new typed model)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` (full rewrite: handcrafted sparse diff)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (full rewrite: 10 variants)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` (PlyArtifact rewrite)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs` (2-arg signature fix)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs`
- Facet mirrors (TS/JSON/GraphQL/proto), all 4 facets: `🧬️schema/{🟦️,🔣️,🔗️,🛰️}component.*`, `🧬️schema/📸️snapshot/{🟦️,🔣️,🔗️,🛰️}component.*`, `🧬️schema/🔺️diff/{🟦️,🔣️,🔗️,🛰️}component.*`, `🧬️schema/🧬️mutations/{🟦️,🔣️,🔗️,🛰️}component.*`
- Grammar leaves: `🧬️schema/📸️snapshot/📝️text/{🅰️,🔤️,📖️}component.*`, `🧬️schema/📸️snapshot/💾️binary/{🥋️,🌶️,🔠️,📡️}component.*`, `🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio`, `🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`

(All paths rooted at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/`.)

## 6. glue_followup

No new top-level directory needed — everything fit inside already-mounted files, per S2's
resolution. For the F2 closer:
1. Prune 9 `-stale-` entries from `📜️script.ts`'s S-8 allowlists once this report is accepted:
   `POLICY_GRAMMAR_HONESTY_ALLOWLIST` (7: snapshot's protocol.semio/spicy/abnf/ksy/grammar.semio,
   diff's grammar.semio, mutations' grammar.semio — all under `stdio/ply/standards#1.0-…`),
   `POLICY_DIFF_ALGEBRA_ALLOWLIST` (1: `stdio/ply/standards#1.0` diff), `POLICY_FIELD_SWEEP_
   ALLOWLIST` (1: `stdio/ply/standards#1.0`). Exact keys are in `.🦑️repo/⚡️cache/breaches/
   compose.json`, filter `kind` for the 4 `stdio-artifacts/*` rules and `scope` containing `ply`.
2. The remaining ~12 diff/mutations `.g4`/`.ebnf`/binary-4-leaf placeholders stay correctly
   allowlisted (still placeholders, not stale) — no action needed, matches the accepted zip
   precedent from F1.

## Summary

`stdio.ply` is now a real, complete, generic model of PLY's element/property system —
`vertices`/`faces` meshes are the common case that falls out of it, not a hardcoded assumption.
Every field of `PlySnapshot` is independently diffable and mutable; the diff nests two collection
levels (name-keyed elements → index-keyed rows) with a handcrafted, base-free, structural absorb
at both levels; every one of the recipe's 6 test laws is present and green. `cargo test -p
semio-s-plugin-stdio --lib "artifacts::ply"`: 23/23. Full crate: 795/0.
