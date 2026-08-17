# F2 — obj 3.0 — Schema Overhaul Report

Ticket: `26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION`. Artifact:
`🧊️obj` standard `3.0` (path `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/`).

## 1. Snapshot — before/after

**Before** ("medium" tier per W0 recon): `vertices: Vec<{x,y,z:f32}>`, `texcoords: Vec<{u,v:f32}>`,
`normals: Vec<{x,y,z:f32}>`, `faces: Vec<{vertices, object, group, material, smoothing_group}>`
(state tagged per-face, no separate collections, no mtllib, no unknown-statement retention).

**After** — complete per the Wavefront OBJ 3.0 spec's real, commonly-implemented grammar:
- `vertices: Vec<ObjVertex{x,y,z:f64, w:Option<f64>}>` — upgraded to `f64`, added the optional
  homogeneous `w` component.
- `texcoords: Vec<ObjTexCoord{u,v:f64, w:Option<f64>}>` — added the optional 3rd component.
- `normals: Vec<ObjNormal{x,y,z:f64}>` — upgraded to `f64`.
- `faces: Vec<ObjFace{vertices: Vec<ObjFaceVertex{vertex,texcoord,normal}>}>` — pure geometry now;
  `o`/`g`/`usemtl`/`s` state moved OUT to their own collections (matches the recipe's index-keyed
  collection shape; a face no longer duplicates state that belongs to a name-keyed/range-tagged
  collection).
- `groups: Vec<ObjGroup{name, faces: Vec<usize>}>` — name-keyed, face-index-LIST membership (a
  face can be in several simultaneous groups, matching real `g a b c` semantics — my call,
  documented in the snapshot's doc comments).
- `objects: Vec<ObjObject{name, faces: Vec<usize>}>` — name-keyed, face-index-LIST membership
  (exactly one object active at a time in practice, but modeled the same shape as groups for
  consistency).
- `mtllib: Option<String>` — new.
- `usemtl: Vec<ObjUsemtlRange{face_index_from, material}>` — new, range-tagged (my call).
- `smoothing_groups: Vec<ObjSmoothingRange{face_index_from, group: Option<u32>}>` — new,
  range-tagged (`None` = `s off`).
- `unknown_statements: Vec<ObjUnknownStatement{line_index, raw}>` — new; comments AND any
  unrecognized keyword line are now retained (previously silently dropped).

`ObjArtifact` (the top-level artifact aggregate at `🧬️schema/🦀️component.rs`) was updated to
mirror `ObjSnapshot` field-for-field (it's `ObjEngine`'s internal `artifact_state`, not exported
elsewhere) — needed to keep `ObjArtifact::from_snapshot`/`to_snapshot`/`set_snapshot` compiling
against the enriched snapshot.

## 2. Diff

Handcrafted sparse `ObjDiff` — no `snapshot: Option<ObjSnapshot>` full-replace slot anywhere
(confirmed via grep). Ten fields:
- `vertices`/`texcoords`/`normals`/`faces`: index-keyed `removed: Vec<usize>` /
  `modified: Vec<{index, diff}>` / `added: Vec<{index, item}>` recursive triples, each item's
  `diff` a real per-field sparse patch (`ObjVertexDiff{x,y,z: Option<f64>, w: Option<Option<f64>>}`
  etc.) — not a whole-item replace.
- `groups`/`objects`: name-keyed triples (`removed: Vec<String>`, same recursive shape), mirroring
  `stdio.zip`'s `ZipEntriesDiff` pattern (no rename tracking needed — `g`/`o` never rename a group
  in place).
- `mtllib: Option<Option<String>>` — tri-state scalar.
- `usemtl`/`smoothing_groups`/`unknown_statements: Option<Vec<T>>` — whole-vec-replace scalars
  (weak value lists, per the recipe's weak-entity rule).

**Code-reuse note (documented in the file's module doc comment)**: the four index-keyed
collections share IDENTICAL position algebra (apply order: modified on BASE positions → removed
descending → added ascending clamped; label-simulation absorb). Rather than hand-duplicate that
~150-line algorithm four times, it's written ONCE via a small intra-file `ObjIndexElem` trait +
generic `generic_apply`/`generic_between`/`generic_absorb_pair` functions — never exported, never
shared with another artifact. Every PUBLIC diff type (`ObjVerticesDiff`, `ObjTexCoordsDiff`, …)
stays fully concrete with its own field names, matching the recipe's "specific code, not generic"
mandate at the type-shape level (the mandate is about killing CROSS-ARTIFACT shared types like the
old `MeshVertex`, not about intra-file DRY). The label-simulation algorithm itself mirrors
`stdio.txt`'s already-proven `Lbl`/`simulate_labels`/`absorb_pair` shape (same author precedent
cited in the module docs).

`impl protocol::MutationDiff<ObjSnapshot> for ObjDiff` (`apply`/`absorb`) and
`impl protocol::command::DiffAlgebra<ObjSnapshot> for ObjDiff` (`inverse`/`between`/`is_empty`)
both present — confirmed via grep (`impl DiffAlgebra<ObjSnapshot> for ObjDiff` at line 817).

## 3. Mutations

22 variants (`NoMutation`, `SetSnapshot` + 9 vertex/texcoord/normal Insert/Remove/Set + 3
face Insert/Remove/Set + 4 group/object Set/Remove + `SetMtllib`/`SetUsemtl`/
`SetSmoothingGroups`/`SetUnknownStatements`) — matches the brief's "SetVertex ×3 for v/vt/vn"
instruction literally, plus went beyond the brief's explicit list (which didn't call out
`SetSmoothingGroups`/`SetUnknownStatements`) to keep every snapshot field genuinely mutable, per
CLAUDE.md's thoroughness mandate. Every variant's `diff()` is handcrafted (constructs the sparse
`ObjDiff` directly via the diff-builder functions in `🔺️diff`, or a `*_diff_between` field
comparison for the three "whole-item Set" variants) — apply-and-capture never appears. `inverse()`
is handcrafted per variant, reading pre-state from `base` (index/name lookups, `NoMutation`
fallback on out-of-range).

## 4. Test laws (in the existing `🧬️mutations/🦀️component.rs` and `⚙️engine/🦀️component.rs`
test regions)

All 6 present and green:
1. `mutation_diff_law` — all 22 variants.
2. `inverse_law` — mutation-level + diff-level, all 22 variants.
3. `absorb_law` — Insert+Remove-before, Insert+Insert-same-index (both survive), Add+SetField
   (patches into the added payload), Modify+Remove (collapses), a name-keyed Add+Remove
   (annihilation), and associativity over a triple.
4. `between_roundtrip_law` — on `sweep_a()`/`sweep_b()`, both directions + `between(a,a).is_empty()`.
5. `codec_retention_law` (in `⚙️engine`) — decode→encode retains every field including
   mtllib/w-components/comments/unknown-statement content+order; documented normal form (see §5)
   verified stable from the 2nd generation onward.
6. `field_sweep_every_mutable_field_changes` — **avoided the exact F1/txt trap** flagged in the
   brief: `sweep_a`/`sweep_b`'s four index-keyed collections use ASYMMETRIC lengths (2 vs 3 items)
   so `between(a,b)` proves `modified`+`added` (b is longer) and `between(b,a)` proves
   `modified`+`removed` (b is longer as base too) — assertions split across both directions, never
   claiming all three from one call on a flat collection. `groups`/`objects` (name-keyed) DO show
   `removed`+`modified`+`added` from a single `between(a,b)` call, since name-keyed collections
   aren't subject to that limitation (documented inline). Every diff field asserted `is_some()`;
   `texcoords[1].w` exercises `Some(None)` and `mtllib` exercises the same tri-state.

## 5. Engine (codec)

`decode_obj`/`encode_obj` fully rewritten for the new snapshot. Real changes: `f64` throughout,
optional `w` on `v`/`vt`, multi-name `g` (a face joins ALL active group names, not a single
joined-string hack like before), `mtllib` parsing (last-occurrence-wins, multi-name join), and
comment/unrecognized-line retention into `unknown_statements` (previously silently dropped).
Documented normal form (module doc comment + `codec_retention_law`): encode always emits
`mtllib` → `v`/`vt`/`vn` blocks → one `f` line per face preceded by whichever of `o`/`g`/`usemtl`/`s`
changed → a trailer of retained `unknown_statements` in original relative order. Content and
relative order are fully retained across one decode→encode cycle; `unknown_statements[].line_index`
is renumbered on re-encode (documented, tested); decode/encode is a true fixed point from the
**second** generation onward (asserted in `codec_retention_law`). A second, narrower, documented
limitation: `groups`/`objects`/`usemtl`/`smoothing_groups` reconstruction assumes the "sticky
range" shape real parsing always produces — a hand-built snapshot with a genuinely disjoint/
non-contiguous membership pattern for the same name is a synthetic case this TEXT codec doesn't
attempt to round-trip (does not affect diff/mutation semantics, which never go through the text
codec).

## 6. Facet mirrors & grammar leaves

All handcrafted (no `payload = *OCTET`-as-the-whole-grammar placeholders remain):
- **Snapshot text**: real Wavefront OBJ statement grammar (`.grammar.semio`, `.g4`, `.ebnf`) —
  `v`/`vt`/`vn`/`f`/`o`/`g`/`usemtl`/`mtllib`/`s`/comment productions, `unknown_stmt` as the one
  legitimate any-other-line catch-all (not a stand-in for the whole grammar).
- **Snapshot/diff/mutations binary**: the shared `.semio` envelope (magic/token-len/token/payload)
  wrapping UTF-8 text — same pattern F1's csv/zip already established; `payload = *OCTET` here is
  legitimate (the binary envelope defers structure to the sibling text grammar, exactly like csv's
  approved binary leaves).
- **Diff/mutations text**: "wire text IS the JSON serialization" pattern (real field/tag names
  named explicitly, not RFC 8259 restated) — same pattern as csv's diff/mutations text leaves.
- **TS/GraphQL/JSON-Schema/proto** for artifact/snapshot/diff/mutations: real, field-complete,
  self-contained (no cross-file imports, matching the zip/csv precedent since these are embedded
  via `include_str!`, not compiled as a real module graph).

## 7. Verification

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::obj"` → **17 passed, 0 failed** (all 6 law
  suites + pre-existing codec/analyzer/demo tests).
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate, no filter) → **795 passed, 0 failed**.
  No regressions anywhere else in the crate.
- Grep gates: zero `snapshot: Option<` inside the diff struct (only appears in a doc comment
  describing the OLD pattern being replaced); zero `serde_json::Value` in obj's
  snapshot/diff/mutations files; `impl DiffAlgebra<ObjSnapshot> for ObjDiff` present.
- `bun ./📜️script.ts policy` (S2's noted correct entrypoint): zero hits for obj on any of the 4
  new S-8 rules (`facet-mirror-drift`, `grammar-honesty`, `diff-algebra`, `field-sweep`) — full
  pass. The handful of other `🧊️obj`-tagged lines in the policy report (triad-completeness,
  facet-completeness, composer, emoji-prefix taxonomy) are pre-existing structural/naming-pattern
  breaches confirmed present IDENTICALLY on `🎒️zip` (an already-closed, verified-clean F1
  artifact) — not introduced by this wave, not mine to fix. The one substantive obj hit,
  `dsl-migration/diff-completeness` ("implements MutationDiff but never gives that diff type a
  DiffCodec impl"), is explicitly out of scope for this wave (`OpText`/`OpBinary`/`DiffCodec` are
  the plan's own final-wave item).

## 8. Concurrent-session note (external churn, resolved by the time of the final gate)

Mid-session, `cargo check` failed with exactly one error: las's (a sibling F2 artifact, NOT mine)
`🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs` still called the pre-migration 1-arg
`diff_set_snapshot(snapshot)` even though las's OWN `diff_set_snapshot` had already been
independently upgraded to the 2-arg `(base, next)` shape by las's own concurrent agent (confirmed
via `git status` showing las's files mid-edit this session — not caused by anything I touched;
obj and las's diff modules are fully independent). I fixed my own equivalent leaf
(`🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs` under `🧊️obj/`, mine to own) to the new
2-arg signature. I verified my own logic independently via a standalone scratch crate (this
ticket folder's `f2-obj-scratch/obj-scratch/`, ported snapshot+diff+mutations with local trait
stand-ins, zero framework deps) while las's blocker was outstanding — all 5 law tests green there
too — then re-ran the real crate check periodically; by the time facet-mirror/grammar-leaf work
finished, las's own fix had landed and the full crate compiled and tested clean (795/0). Cargo
logs saved as `f2-obj-cargo-check.txt` / `f2-obj-cargo-test-full.txt` in this ticket folder.

## Files touched

- `🏅️standards/🔖️3.0/⚙️engine/🦀️component.rs` — decode/encode rewrite + tests.
- `🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` — `ObjArtifact` aggregate, field parity.
- `🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — new snapshot model.
- `🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — new handcrafted diff.
- `🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — new mutation enum + laws.
- `🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs` — 2-arg fix.
- Facet mirrors (TS/GraphQL/JSON-Schema/proto) under `🧬️schema/`, `🧬️schema/📸️snapshot/`,
  `🧬️schema/🔺️diff/`, `🧬️schema/🧬️mutations/`.
- Grammar leaves (`.g4`/`.ebnf`/`.grammar.semio` under each `📝️text/`; `.protocol.semio`/`.abnf`/
  `.ksy`/`.spicy` under each `💾️binary/`) under `📸️snapshot/`, `🔺️diff/`, `🧬️mutations/`.
- Scratch crate (not part of the real tree, this ticket folder): `f2-obj-scratch/obj-scratch/`.

## Deviations from the brief

- Went beyond the brief's explicit mutation list to add `SetSmoothingGroups`/
  `SetUnknownStatements` (not individually named in the brief) so every snapshot field has a
  dedicated mutation, not just SetSnapshot-reachability.
- `texcoords`/`vertices` gained the optional `w` component (brief said "or 3-component if your
  codec should support the optional w — your call"); I chose to support it for full spec honesty.
- Precision upgraded `f32` → `f64` for vertices/texcoords/normals (brief's completeness table
  specifies `[f64;3]`/`[f64;2]`), a real (small) behavior change from the pre-existing codec.
- `groups`/`usemtl` chosen shapes ("face-index list" for groups/objects, "range" for usemtl/
  smoothing) are exactly the brief's offered "your call, document it" latitude — documented in
  the snapshot's own doc comments.

## glue_followup

None — no new top-level directory was needed; all real work landed in the already-mounted
snapshot/diff/mutations/engine files per S2's confirmed "zero glue.rs edits needed" finding.
