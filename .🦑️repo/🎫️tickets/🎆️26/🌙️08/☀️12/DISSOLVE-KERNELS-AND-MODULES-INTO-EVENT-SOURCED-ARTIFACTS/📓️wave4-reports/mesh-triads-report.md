# Wave 4 report — ✳️mesh mutation vocabulary + inference facet

Author: this session (mutation vocabulary + inference facet for `✳️mesh`, the last of three stdio
subsets DKM owns; `✳️brep` and `✳️drawing` are done and were used as the working reference).
Boundary: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/**`
except `📸️snapshot/` (untouched, another session's composition slots), plus the `✳️mesh` mount
blocks in stdio's `📦️glue.rs`, plus two mechanical-fallout sites: one line in `✳️mesh/🚪️io/🦀️component.rs`
(outside the stated `🧬️schema/` boundary, fixed anyway — see "Mechanical fallout" below, same class
of necessary out-of-boundary fix brep's own wave made in `✳️any`) and `✳️any/🧬️schema/🧬️mutations/🦀️component.rs`.

## ⚠️ Provenance — read this before the vocabulary section

**`replace-primitive-geometry`** (renamed from the old `set-primitive-geometry`) is **SMO's
rename, not DKM's**. Per `📌️important.md`'s verb-rulings table: *"mesh `set-primitive-geometry`→
`replace-primitive-geometry` | ✅ reasoning approved, ❌ but it is SMO's to do, not ours."* SMO
reviewed and approved the reasoning — a positions/normals/uvs/colors/indices vertex-buffer blob is
a structured sub-payload, so `set` (a single-field-only verb) is the wrong choice — and explicitly
**reserved the edit for themselves**, since `✳️mesh` was in their lane at the time. **SMO has since
wound down without doing it.** DKM completes it here under the user's explicit instruction to
finish the stdio mutation vocabulary end to end. Status: **SMO-approved in reasoning, SMO-reserved
in execution, completed by DKM after SMO ended.** This is reported as SMO's rename executed by
DKM, not as DKM's own design decision — the same discipline `📌️important.md`'s binding-ruling rule
asks for ("not re-litigated").

## What changed

### The vocabulary — 17 verbs derived from the real snapshot fields, checked against both rule docs
`create-mesh` / `delete-mesh` · `create-primitive` / `delete-primitive` · `set-primitive-topology`
· `replace-primitive-geometry{mesh_id,primitive_id,positions,normals,uvs,colors,indices}` (SMO's
rename) · `set-primitive-material` · `create-material` / `delete-material` ·
`change-material-base-color` · `change-material-metallic` · `change-material-roughness` ·
`create-texture` / `delete-texture` · `change-texture-mime` · `replace-texture-bytes` ·
`move-vertex{mesh_id,primitive_id,vertex_index,new_point}`.

**A real, pre-existing, non-conforming facet was found and replaced, not a blank slate.** The
subset already had a fully hand-rolled `SemioMeshMutation` with 16 variants
(`AddMesh`/`RemoveMesh`/`SetPrimitiveGeometry`/`SetMaterialPbr`/`SetTextureBytes`/… plus
`NoMutation`/`SetSnapshot`) and a working, hand-rolled `Mutation`/`OpText`/`OpBinary` impl — but
ZERO of those 16 variants had a matching triad directory (only `📄set-snapshot`'s triad existed on
disk), and it used the two globally-banned identifiers `NoMutation`/`SetSnapshot`. The dispatch
enum was replaced wholesale with a `#[derive(dsl::Mutations)]`-based one (matching brep's own
established convention, and derivation-rules.md's "Per-mutation implementation shape"); the
underlying `SemioMeshDiff` (`🔺️diff/🦀️component.rs` — `NamedTripleDiff`-based, with a full
`between`/`apply`/`inverse`/`absorb` algebra and its own 10 tests) was **kept and reused**, since it
was already correct and exactly the right shape — extended with presence/duplicate checks (see
"Presence-check fix" below) and four small new helpers, not rewritten.

### Deliberate departures from the ticket brief's "expect roughly" vocabulary list, with reasoning
The brief's own "expect roughly" list is explicitly hedged; two places were checked against
`📓️taxonomy.md`/`📓️derivation-rules.md` and diverged from that rough list, on rule-grounded reasoning:

1. **`add-mesh`/`remove-mesh`/… renamed to `create-mesh`/`delete-mesh`/…** (all four collections:
   mesh, primitive, material, texture). Derivation-rules.md rule 2 is explicit and unconditional:
   *"Per id-keyed collection: `create-<singular>`, `delete-<singular>`."* Cross-checked against
   real precedent already in this codebase: `✳️graph` uses `create-node`/`delete-node` for the full
   NODE entity but `add-node-port`/`add-node-property` for genuinely set-like MEMBERS attached to a
   node (taxonomy's own distinction: `add` = "attach a set-like member (attribute, tag,
   connector)"); `✳️table` uses `create-column`/`delete-column`. None of mesh's four collections are
   set-like members — each is a full structured entity with its own field set — so rule 2 governs.
   Also matches brep's own precedent exactly (its pre-existing `AddVertex`/`RemoveVertex` renamed to
   `CreateVertex`/`DeleteVertex` for the identical reason). Documented in the dispatch enum's own
   module doc comment, not silently done.
2. **`set-material-pbr{metallic,roughness}` decomposed into `change-material-metallic` +
   `change-material-roughness`; `set-texture-bytes{mime,bytes}` decomposed into
   `change-texture-mime` + `replace-texture-bytes`.** Both bundle TWO independent top-level scalar
   fields (`SemioMaterial.metallic`/`.roughness`, `SemioTexture.mime`/`.bytes` are separate fields,
   not grouped into one value-type struct the way `base_color` is a `SemioRgba`). SMO's own
   `StrokeStyle` ruling in `📌️important.md` draws exactly this line: *"`StrokeStyle` is a 5-field
   struct whose fields are independently set by the editor... decompose"* — same shape here
   (metallic/roughness are independently-set PBR sliders in every real 3D editor API). `bytes`
   alone (after separating out `mime`) is then the "large" swapped payload, matching
   `replace-primitive-geometry`'s own rename rationale exactly → `replace`, not `change`/`set`.
   `change-material-base-color` was KEPT bundled (not decomposed) because `base_color` genuinely IS
   one grouped `SemioRgba` field on `SemioMaterial` — the opposite shape, correctly matching
   `change-stroke-color`'s own precedent for a color VALUE type.

### `move-vertex` — authored, not omitted, with explicit reasoning either way was considered
Mesh's `positions: Vec<SemioPoint3>` has no per-element id (a raw parallel buffer, unlike brep's
vertices which carry a `PersistentLabel`). Inserting/removing ONE position element without
touching `normals`/`uvs`/`colors`/`indices` in lockstep (and renumbering `indices`) has no honest
single-verb expression — so no `insert-vertex`/`remove-vertex` is authored, matching the same
"no valid stable address" reasoning brep used to exclude `create-loop`/`delete-loop`. But a
same-length REPOSITION (`move-vertex{mesh_id,primitive_id,vertex_index,new_point}`, addressed by
BASE-state index per derivation-rules.md rule 3) doesn't need insert/remove semantics and IS
authored — a real address (three-part) plus one field, matching taxonomy's `move` verb exactly and
mirroring brep's own approved `move-vertex`.

### `create-loop`-shaped/insert-remove-vertex verbs are NOT authored
Stated in the dispatch module's own doc comment (`🧬️mutations/🦀️component.rs:1-40`) — not an
oversight; see above.

### Presence-check fix, applied proactively (not found reactively via failing tests this time)
Brep's wave found, AFTER law-testing, that its `delete-*` diff constructors unconditionally
included the target id even when absent from `base`, making `is_empty()` lie. Forewarned by that
exact finding (`📌️important.md`'s own "four gates are necessary, not sufficient" section), this
wave added presence/duplicate checks to every reused/new `diff_*` helper in `🔺️diff/🦀️component.rs`
**before** running the laws, rather than discovering the bug via a failing test. The laws below
were still run in full and would have caught it had the proactive fix been wrong or incomplete —
they didn't need to, but that is confirmed by the round-trip/no-op tests below, not assumed.

### 💡️inference facet — one honest field, two honestly omitted (same shape as brep)
`💡️inferences/📦aabb/` — a real `InferredField<SemioMeshSnapshot>` with a genuine **per-primitive**
`DepHash` chain (key = `"{mesh_id}:{primitive_id}"`, NO parents — each primitive's AABB depends
only on its own `positions`), the stronger/more idiomatic shape the proven puzzle3d
`🎛flat-position` pilot establishes, not the whole-document single-key fallback brep's own
`validationReport` used (brep's field genuinely needed whole-document reads; mine doesn't, so it
gets the richer per-entity chain). Proven via cache-transparency, and TWO separate incrementality
tests: touching one primitive's positions misses only that primitive's cache entry (not the
sibling's), and touching an unrelated field (`material_id`) on the SAME primitive does not miss at
all — a stronger proof than a single generic incrementality test.

**`computed-normals`/`tessellation-preview` are deliberately NOT authored.** Full reasoning lives in
`💡️inferences/📦aabb/🦀️component.rs`'s module doc comment; summary:
- `computed-normals` would infer a SECOND, competing definition of a field the mutation vocabulary
  already owns as tier-(b) authored state — `SemioPrimitive.normals` is itself a persisted,
  independently-authorable field (stylized/sculpted normals are a legitimate authored value this
  format explicitly supports). Inferring a shadow value for it would blur the tier-(b)/(c) boundary
  this entire ticket exists to keep sharp — a DIFFERENT and, on reflection, sharper reason than
  brep's "no honest math home" reasoning, discovered by actually reading this subset's own schema
  rather than copying brep's justification verbatim.
- `tessellation-preview` is not a genuine derivation for mesh at all: `positions`/`indices`/
  `topology` already ARE the tessellated render buffers (unlike brep's B-rep, which needs real
  curve/surface evaluation to produce a renderable mesh). A "preview" that merely copies
  already-authoritative snapshot data is not an honest inference.

Both omissions are the sanctioned outcome per `📌️important.md` ("if a real dependency chain cannot
be authored honestly for a field, omit that field and say why"), not silently dropped fields.

### Mount blocks added to stdio's `📦️glue.rs`
Deleting `📄set-snapshot`'s triad dir without removing its `#[path]` mount would have been a hard
compile error for the whole workspace. The stale mount was removed and 17 new triad mount blocks
(`create_mesh`/`delete_mesh`/…/`move_vertex`, each `{inverse, diff, mutation}`) plus the
`inferences`/`aabb` mount were added, generating every `#[path]` string from a real directory
listing (`os.listdir()`/`find`) of the on-disk triad names — never hand-typed — per the
unicode-normalization-trap warning.

**⚠️ Concurrent-churn incident, self-corrected, documented here in full because it is instructive:**
after this wave's mount edit was applied and independently verified (a script walking every
`#[path]` under the mesh region and `stat`-ing its target: 76/76 resolved), a LATER pass of the
same verification script found the `inferences`/`aabb` mount **missing** — 74/74 resolved instead
of 76/76, with `git log` showing three more auto-commits (`🚩️497`–`🚩️499`) had landed on `glue.rs`
in between from other concurrent sessions (this file is shared by all five sessions in this tree).
The mutation-triad mounts were untouched; only the two inference mounts were casualties. Re-applied
immediately, re-verified (76/76), and reconfirmed by a full recompile + test run afterward. Also
caught (twice, both times before landing) via the same verification script: a self-inflicted
unicode-normalization typo (`🏅️标准`, corrupted CJK, instead of `🏅️standards`) that would have
produced a silently-empty/dangling mount — exactly the trap `📌️important.md` warns about — never
shipped, per the "generate paths, verify by resolving every path" discipline.

### Existing files edited (not authored fresh)
- `🧬️schema/🦀️component.rs` (top-level artifact): `derived_construction::mutate()` no longer calls
  a free `apply_semio_mesh_mutation` fn (removed along with the old dispatch); inlined to
  `<Mutation>::diff` + `<Diff as MutationDiff>::apply`, matching brep's own builder convention
  exactly (and `✳️text`'s, which both established this pattern first).
- `🔺️diff/🦀️component.rs`: kept the diff algebra itself untouched; added presence/duplicate checks
  to every reused `diff_*` helper (now takes `base`), renamed `diff_set_material_base_color`→
  `diff_change_material_base_color` and `diff_set_primitive_geometry`→
  `diff_replace_primitive_geometry`, removed `diff_set_snapshot` and `diff_set_material_pbr`/
  `diff_set_texture_bytes` (superseded by the decomposed pairs), added four new small helpers
  (`diff_change_material_metallic`, `diff_change_material_roughness`, `diff_change_texture_mime`,
  `diff_replace_texture_bytes`) and one new one requiring `base` (`diff_move_vertex`), and promoted
  `mesh_at`/`primitive_at`/`material_at`/`texture_at` from the (now-deleted) dispatch file into this
  file as `pub(crate)` so all 17 triad leaves share one copy instead of re-deriving four one-line
  finders seventeen times over.
- `🧬️mutations/{🟦️component.ts,🔣️component.json,🔗️component.graphql,🛰️component.proto}` (facet
  mirrors, rewritten for the new 17-verb vocabulary).
- `🧬️mutations/📝️text/📖️component.grammar.semio` (new 17-keyword alternation, `snapshot-lit`
  production removed since `set-snapshot` has no replacement).
- `🧬️mutations/💾️binary/{🌶️component.spicy,🔠️component.abnf,🥋️component.ksy}` (stale
  `"empty for no-mutation"` prose fixed — `no-mutation` no longer exists in this vocabulary; the
  binary frame SHAPE itself needed no change, matching brep's own finding that an opaque
  format+tag+payload frame doesn't enumerate keywords).
- `🧬️mutations/💾️binary/📡️component.protocol.semio` (comment updated: tag now ranges 0-16 for 17
  triads; frame shape unchanged).

### Mechanical fallout — two files outside the stated boundary, fixed because leaving them broken
would be a workspace-wide compile break, not a redesign of either file's own logic
- `✳️mesh/🚪️io/🦀️component.rs` (this subset's own composer/validator file, but outside the stated
  `🧬️schema/**` boundary — the same relationship brep's own `🚪️io/🦀️component.rs` had to brep's
  boundary): `sample_mutations()` filtered out `SemioMeshMutation::NoMutation`, which no longer
  exists. Fixed by removing the now-impossible filter (every `demo_mutation_cases()` entry is a
  genuine mutation now, so `.take(1)` alone suffices) — a one-line, purely mechanical fix, not a
  logic change.
- `✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (mechanical fallout, not a boundary violation — same
  class brep's own wave hit and fixed in the identical file): one `demo_mutation_cases()`
  construction site (`SemioMeshMutation::NoMutation`) and one `all_twelve_wrapped_kinds_…` sweep
  test's `bases` list/match arm no longer compiled once `NoMutation` was removed from mesh's own
  vocabulary. Fixed by: (a) replacing the `demo_mutation_cases()` entry with a real absent-target
  op (`DeleteMesh{id:"mesh-absent"}`, mirroring brep's own `DeleteVertex{id:"v-absent"}` pattern
  exactly); (b) excluding `mesh` from the generic sweep test's `bases` list (matching the
  already-established `text`/`table`/`graph`/`brep` precedent in the very same file, whose own doc
  comment names this exact situation) and renaming the test from `all_twelve_wrapped_kinds_…` to
  `all_eleven_wrapped_kinds_…` (grepped repo-wide first — zero other references to the old name);
  (c) adding a dedicated `wrapped_mesh_kind_diff_and_inverse_route_correctly` test using a real
  `CreateMesh`, mirroring `wrapped_brep_kind_diff_and_inverse_route_correctly` verbatim in shape.

## Files touched

**Deleted** (6): `🧬️mutations/📄set-snapshot/{🦠️mutation,🔺️diff,↩️inverse}/{🦀️component.rs,🟦️component.ts}`

**Created** (17 triads × 6 files = 102, plus inference = 7 (`💡️inferences/{🦀️component.rs,
🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}` + `📦aabb/{🦀️component.rs,
🟦️component.ts}`), plus this report = 1 → 110 new files):
- `🧬️mutations/{🕸️create-mesh,🗑️delete-mesh,🔺create-primitive,✂️delete-primitive,
  🔀set-primitive-topology,📐replace-primitive-geometry,🔗set-primitive-material,🎨create-material,
  🚮delete-material,🌈change-material-base-color,⚙️change-material-metallic,
  🧱change-material-roughness,🖼️create-texture,🕳️delete-texture,🏷️change-texture-mime,
  📀replace-texture-bytes,📍move-vertex}/{🦠️mutation,🔺️diff,↩️inverse}/{🦀️component.rs,🟦️component.ts}`
- `💡️inferences/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`
- `💡️inferences/📦aabb/{🦀️component.rs,🟦️component.ts}`

**Updated**: `🧬️mutations/🦀️component.rs` (full rewrite: derive-based dispatch enum, hand-rolled
`OpText`/`OpBinary`, demo fixtures, law tests) · `🧬️mutations/{🟦️component.ts,🔣️component.json,
🔗️component.graphql,🛰️component.proto}` (facet mirrors) · `🧬️mutations/📝️text/📖️component.grammar.semio`
· `🧬️mutations/💾️binary/{🌶️component.spicy,🔠️component.abnf,🥋️component.ksy,📡️component.protocol.semio}`
(stale-comment fixes only) · `🔺️diff/🦀️component.rs` (presence checks + renames + new helpers +
`mesh_at`/`primitive_at`/`material_at`/`texture_at` promoted here) · `🦀️component.rs` (top-level
artifact schema; `mutate()` simplified) · `✳️mesh/🚪️io/🦀️component.rs` (1-line mechanical fallout,
outside stated boundary) · `✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (mechanical fallout: 1
construction site + 1 test restructure + 1 new test) · `📦️glue.rs` (17 new triad mounts + 1
inference/aabb mount, re-applied once after concurrent-churn casualty, see above).

## Verification commands run, with real output pasted

Baseline given by the ticket (brep's wave): 2245 passed, 3 failed (2 `fixture_honesty_law` on
dwg/ifc matching an earlier baseline, 1 unrelated `drawing` failure from a concurrently-authoring
session). This wave's own first-touch baseline, measured before any mesh edits landed, showed the
same dwg/ifc failures plus three more (`binary::extent`, `dxf::bounds`, `zip::entries`
`inference_default_law`) — all pre-existing, unrelated, concurrent-session churn, confirmed absent
from every mesh/any file this wave touched.

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS/🎯️target" \
  cargo check -p semio-s-plugin-stdio --all-targets
```
Forced recheck (`touch`'d a mesh file first each time), final result: **zero errors**, only
pre-existing warnings in files this wave never touched.

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS/🎯️target" \
  cargo test -p semio-s-plugin-stdio --lib
```
Final real result (re-run three times across the wave for stability — once right after the mount
was first verified complete, once after discovering and fixing the concurrent-churn mount loss,
once as the final check — identical failure set every time):
```
failures:
    artifacts::binary::standards::v_raw::subsets::any::schema::inferences::extent::component::tests::inference_default_law
    artifacts::dwg::standards::v_ac1018::engine::tests::conformance_laws::fixture_honesty_law
    artifacts::dxf::standards::v_r12::subsets::any::schema::inferences::bounds::component::tests::bounds_matches_hand_built_entity_extent
    artifacts::ifc::standards::v2x3::engine::tests::conformance_laws::fixture_honesty_law
    artifacts::zip::standards::v2_0::subsets::any::schema::inferences::entries::component::tests::inference_default_law

test result: FAILED. 2415 passed; 5 failed; 5 ignored; 0 measured; 0 filtered out; finished in 21.11s
```
**Zero failures anywhere in `✳️mesh`, in the `✳️any` fallout fix, or in `✳️mesh/🚪️io`**, confirmed by
name in the failure list above (none match) and by grepping the full test list for `mesh` (all
`ok`, including the new `wrapped_mesh_kind_diff_and_inverse_route_correctly` in `✳️any`, all 17
triad-covering tests in `mesh::schema::mutations`, and all 9 new inference tests in
`mesh::schema::inferences`). None of the 5 failures touch a file this wave edited — independently
confirmed by grepping each failing file's path against this wave's file list.

### Laws actually run (this wave's own, not gate-passed and stopped there)
- `inverse_round_trip_law_covers_every_variant` — every one of the 17 demo cases: forward `diff`
  then `apply`, then every inverse mutation applied in sequence, restores `base` EXACTLY (plain
  equality, not set equality — both `create-*`'s append-diff AND every `delete-*`'s
  position-preserving inverse are exercised together).
- `diff_consistency_law_matches_independent_between` — every variant's hand-built `diff(base)`
  matches an independently-derived `SemioMeshDiff::between(base, diff.apply(base))`.
- `determinism_law_diff_and_inverse_are_pure_functions_of_payload_and_base` — every variant's
  `diff`/`inverse` called twice yields identical results.
- `set_change_replace_move_of_an_absent_target_have_empty_inverse_and_are_no_ops` — explicitly
  covers `set-primitive-topology`, `replace-primitive-geometry`, `change-material-base-color`, and
  `move-vertex` (out-of-bounds index) against absent/invalid targets: `inverse().is_empty()` AND
  `diff(base).apply(base) == base`.
- `delete_of_an_absent_id_has_an_empty_inverse_and_is_a_diff_level_no_op` — explicitly asserts
  `diff(base).is_empty()` (not merely "harmless to apply"), the exact assertion shape that caught
  brep's own bug; this wave's presence checks were added proactively so this test passed on first
  run, not after a fix.
- `op_text_binary_roundtrip_law` — all 17 variants round-trip through both the hand-rolled text
  (`print_op`/`parse_op`) and binary (`encode_op`/`decode_op`) codecs.
- `semantic_kinds_cover_every_variant` — `SemioMeshMutation::kinds().len() == 17`, and
  `delete-mesh`'s `SemanticDescriptor` (`kind`, `record`) and `target()` checked explicitly.
- Inference laws (`📦aabb`): cache-transparency (disabled cache matches pure recompute), TWO
  incrementality tests (only the touched primitive's own entry misses; an unrelated field on the
  SAME primitive does not miss at all), plus two honesty tests (a populated primitive's AABB is the
  real componentwise extent; an empty primitive's AABB is the honest zero default, not a faked one).
- `mesh::schema::diff`, `mesh::schema::snapshot`, and `mesh::io::derived_composition`'s own existing
  test suites (unchanged logic, all still pass) — including
  `conformance_laws::{grammar_conformance_law,ops_grammar_conformance_law,protocol_walk_law,
  diff_grammar_conformance_law,committed_facet_files_parse}`, which independently validate that
  this wave's grammar/facet-mirror rewrites (`📖️component.grammar.semio`, `🟦️component.ts`,
  `🔣️component.json`, `🔗️component.graphql`, `🛰️component.proto`) stay consistent with the real
  `🦀️component.rs` source of truth.

### Harness independence from `din4108`
This wave's own tests thread `mutation.diff(&current); current = diff.apply(&current)` against the
CURRENT evolving state at every step, forward and backward (see
`inverse_round_trip_law_covers_every_variant` above) — written from scratch against
`(payload, base)` semantics, never derived from `din4108`'s reference (which the ticket flags as
diffing each inverse against the stale pre-operation `base`, silently discarding the forward
mutation's effect).

## Four gates — checked mechanically, pasted, not just claimed
- Triad dirs ↔ dispatch enum variants: **17 ↔ 17**, both directions (`find … -maxdepth 1 -type d |
  wc -l` on the 19 total mutations-dir entries minus the 2 non-triad facet-mirror siblings
  `💾️binary`/`📝️text` = 17; `grep -c '^    [A-Z][a-zA-Z]*('` on the enum = 17).
- Unique emoji per sibling triad dir: `🕸️🗑️🔺✂️🔀📐🔗🎨🚮🌈⚙️🧱🖼️🕳️🏷️📀📍` — 17 distinct glyphs, checked
  by listing directory basenames programmatically.
- Real leaves: every triad's `🦠️mutation/🦀️component.rs` has a genuine `impl protocol::MutationKind<…>
  for X`; every `🔺️diff/🦀️component.rs` has a real `pub fn diff(payload, base)` (delegating to a
  presence/duplicate-checked helper in the shared `🔺️diff/🦀️component.rs`, itself built directly
  from `(payload, base)` — never apply-then-capture, independently confirmed by
  `diff_consistency_law_matches_independent_between`); every `↩️inverse/🦀️component.rs` has a real
  `pub fn inverse(payload, base)` reconstructed from `base`, returning `Vec::new()` when the target
  is absent (all 5 `delete-*`, `set-primitive-topology`, `replace-primitive-geometry`,
  `set-primitive-material`, `change-material-base-color`/`-metallic`/`-roughness`,
  `change-texture-mime`, `replace-texture-bytes`, and `move-vertex` — verified by
  `set_change_replace_move_of_an_absent_target_have_empty_inverse_and_are_no_ops` and
  `delete_of_an_absent_id_has_an_empty_inverse_and_is_a_diff_level_no_op`).
- Non-stub `🟦️component.ts` beside every triad `🦀️component.rs`: 51 pairs checked programmatically
  (17 triads × 3 leaves), every `.ts` file present and well over 20 bytes (real `export interface`,
  not a stub).

## sharedFileRequests

1. **File**: `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs` (SHARED across all 14 `s.stdio.semio.*`
   subsets — explicitly out of `✳️mesh/`-only scope, same file brep's own wave flagged for the
   identical reason). **Region**: wherever the other subsets' `register_artifact_inferences()`-equivalent
   calls live. **Reason**: `semio_mesh_artifact_inference_descriptor()` (new, this wave,
   `💡️inferences/🦀️component.rs`) is authored and ready but not registered into the OS-wide
   inference catalog — needs one `::schema::register_artifact_inference_descriptor(…)` call added,
   matching the pattern already used for json/csv/xml/etc. and requested (still open, per brep's
   own still-open item 1) for brep's own descriptor. **Patch**: not prepared (out of edit
   boundary); the descriptor fn signature and id (`"s.stdio.semio.mesh.inference"`) are stable and
   ready to wire.
2. No open vocabulary questions remain for `✳️mesh` — unlike brep's `Loop`/`Coedge` addressability
   question left open for SMO, every field in this subset's snapshot has a clear, closed vocabulary
   home (the two deliberate exclusions — insert/remove-vertex-shaped verbs and
   computed-normals/tessellation-preview — are both closed decisions with stated reasoning, not
   open questions).

## Concurrent-churn observations
`📦️glue.rs` (shared by all five sessions in this tree) received three more auto-commits
(`🚩️497`–`🚩️499`) between this wave's own inference-mount edit landing and a later re-verification
pass, and the re-verification caught that the `inferences`/`aabb` mount had been dropped in that
window (the 17 mutation-triad mounts were untouched). Documented in full under "Mount blocks added"
above, per the ticket's own instruction that this class of incident is instructive, not merely an
inconvenience — re-applied and re-verified before this report was written, and reconfirmed by the
final full recompile + test run pasted above. `✳️mesh/🚪️io/🦀️component.rs` was also touched
externally once (flagged by the harness as "modified, either by the user or a linter... intentional")
during this wave; verified afterward that this wave's own 1-line mechanical-fallout fix in that
file was still present and the crate still compiled clean. Retried the scoped check with real
`touch`-forced rechecks after both incidents per protocol, never trusted a zero-diagnostic run that
wasn't forced.

## Honest pass/fail
**Pass.** All four mechanical gates satisfied and independently re-verified (not merely claimed).
Laws actually executed, with presence/duplicate checks added PROACTIVELY (informed by brep's own
law-testing finding) rather than discovered reactively via a failing test this time — still
independently confirmed correct by the same law shapes brep's wave used. Final state is a clean
law-test pass for every `✳️mesh` and `✳️any` test, diffed against the pre-wave baseline with zero
attributable new failures (all 5 residual failures are pre-existing, unrelated, and independently
confirmed absent from every file this wave touched). `computed-normals`/`tessellation-preview` are
honestly omitted with reasoning specific to this subset's own schema (not copied verbatim from
brep's different reasoning), not silently dropped. `insert-vertex`/`remove-vertex`-shaped verbs are
honestly unauthored per the sanctioned "no valid stable address" outcome. One `sharedFileRequests`
item remains open (inference registration wiring, out of boundary, same class as brep's own still-open
item). One concurrent-churn incident (a dropped glue.rs mount from another session's overlapping
edits) was caught, self-corrected, and is documented above rather than silently absorbed.
