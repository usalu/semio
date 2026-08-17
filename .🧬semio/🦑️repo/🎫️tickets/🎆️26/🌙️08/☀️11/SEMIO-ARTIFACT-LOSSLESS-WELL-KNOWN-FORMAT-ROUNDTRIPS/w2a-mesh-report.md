# W2a — semio/mesh Subset — Real Implementation Report

Status: DRAFT — pending final whole-crate compile/test verification. Concurrent sibling-subset W2
agents (`✳️presentation`, `✳️image`, `✳️animation`, `✳️workflow`, `✳️document` at various points
during polling) are mid-edit and intermittently blocking whole-crate `cargo test`/`cargo check`
(system under heavy multi-agent load — `uptime` load average peaked at 40 on a 10-core machine).
**Mesh's own files compile with zero errors across every one of 5+ independent full-crate
`cargo check`/`cargo test` runs taken at different points as sibling files churned** — every
single failure observed across all runs was in a foreign subset path (never `✳️mesh`), confirmed
by `grep -c "✳️mesh"` against each run's raw output returning 0 every time, and cross-checked via
`git status` showing the failing foreign files as `M`odified/mid-edit by other sessions. Polling
per the ticket's hazard-management guidance rather than chasing foreign in-progress files. Most
foreign failures observed are the identical `#[cfg(test)] use protocol::{OpText, OpBinary}` import
scoping bug this subset's own mutations.rs already avoids (moved `OpText` import out of
`#[cfg(test)]` since non-test `OpCodecs` code calls `self.print_op()`/`Self::parse_op()`) — those
sibling subsets (`image`, `document`, `workflow`) simply haven't landed that one-line fix yet.

## Scope

Write scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/**` only.
Verified clean via `git status --porcelain` — 65 files touched, all under `✳️mesh/`.

## What was implemented

1. **Snapshot** (`🧬️schema/📸️snapshot/🦀️component.rs`): full field coverage per the master plan's
   row — `SemioMeshSnapshot{schema, meshes, materials, textures}`, `SemioMesh{id, primitives}`,
   `SemioPrimitive{id, topology, positions, normals, uvs, colors, indices, materialId}` (the
   `SemioPrimitive` type RESERVED at W1b — this file is where it lands, splitting positions off
   `SemioMesh` into real id-keyed primitives), `SemioTopology` enum (Points/Lines/LineStrip/
   Triangles/TriangleStrip/TriangleFan — gltf 2.0's primitive mode), `SemioMaterial{id, baseColor,
   metallic, roughness}` (PBR), `SemioTexture{id, mime, bytes}`. All named structs (no bare tuples,
   no nested fixed arrays), all geometry via the shared `engine::geometry::{SemioPoint3, SemioUv,
   SemioRgba}`. No `serde_json::Value` anywhere.
2. **Diff** (`🧬️schema/🔺️diff/🦀️component.rs`): handcrafted sparse diff — `SemioMeshDiff{meshes,
   materials, textures}`, each an `engine::triples::NamedTripleDiff<String, XDiff, X>` (imports the
   SHARED struct rather than redefining it, per `w1b-type-ownership.md`); `primitives` nested one
   level inside a modified mesh's own `SemioMeshItemDiff`. Real `MutationDiff`/`DiffAlgebra`/
   `DiffCodec` impls: hand-rolled `between`/`apply`/`inverse`/`absorb` generic helper functions
   (ported from bcf/docx's own copies, instantiated over the shared triples struct) plus a
   hand-rolled bracket-depth-aware hex `DiffCodec` grammar reusing `🧰️triples::enc_named_triple`/
   `dec_named_triple` directly (not re-derived).
3. **Mutations** (`🧬️schema/🧬️mutations/🦀️component.rs`): 16-variant named enum (`NoMutation`,
   `SetSnapshot` + `Add/Remove/Set*` per collection: `AddMesh`/`RemoveMesh`, `AddPrimitive`/
   `RemovePrimitive`/`SetPrimitiveTopology`/`SetPrimitiveGeometry`/`SetPrimitiveMaterial`,
   `AddMaterial`/`RemoveMaterial`/`SetMaterialBaseColor`/`SetMaterialPbr`, `AddTexture`/
   `RemoveTexture`/`SetTextureBytes`). Every `diff()` calls exactly one hand-written `diff_*`
   constructor in the diff module (never apply-and-capture); every `inverse()` is hand-written,
   id-aware (looks up the pre-mutation value in `base`). Hand-rolled `OpText`/`OpBinary`
   (`keyword arg=value ...` grammar, same shape docx's own hand-rolled convention uses).
4. **Grammar leaves**: all 8 `📝️text/` + 6 `💾️binary/` leaves, handcrafted honest, under each of
   the 3 facets (snapshot/diff/mutations — 42 files total): snapshot facet = hex-dump-of-JSON-pack
   pattern (this subset's snapshot is a neutral semio type, not an on-disk format — same honesty
   boundary as bcf/docx's own `ArtifactPack` envelope); diff/mutations facets = the REAL hand-rolled
   `DiffCodec`/`OpText` grammars written out formally (ABNF/EBNF/ANTLR/ksy/spicy), noting the
   binary representation is honestly the text bytes verbatim (no separate framing, matching
   `encode_diff`/`encode_op`'s actual implementation). No `*OCTET`/size-eos catch-alls.
5. **Builder/Analyzer** (`🏗️builder/🦀️component.rs`, `🧐️analyzer/🦀️component.rs`): unchanged logic
   from the W1b scaffold (already generic over `Snapshot`/`Diff`/`Mutation` — `mutate()` delegates
   to `apply_semio_mesh_mutation`, so it automatically carries the real 16-variant vocabulary with
   zero builder-level code changes needed); doc comments updated to remove the stale "🚧 scaffolded"
   markers now that the underlying types are real.
6. **Composer** (`🎹️composer/🦀️component.rs`): real referential-invariant `SubsetValidator`
   (`check_mesh_referential_invariants`) — dangling `primitive.materialId` references into
   `materials`, plus duplicate-id checks within each of the 4 id-keyed collections (meshes,
   primitives-per-mesh, materials, textures), replacing the W1b decode-only stub. 4 new unit tests.
   `WRITES`/`DIALECT` unchanged (already correct: `s.stdio.semio`/`v1`/`mesh`); schema id
   `s.stdio.semio.mesh` confirmed unique repo-wide (only 2 occurrences, both in this subset's own
   files: the `#[artifact_schema(id = ...)]` attribute and the descriptor's `id:` field).
7. **Facet mirrors**: `.ts`/`.graphql`/`.json`/`.proto` rewritten at the snapshot/diff/mutations
   facet level AND the schema-aggregator (`SemioMeshArtifact`) level to mirror the real Rust field
   shapes (every camelCased field from the Rust leaves appears in all 4 sibling leaves).

## 8 test laws — status

| Law | Test(s) | Home |
|---|---|---|
| field_sweep | `field_sweep` | mutations.rs |
| mutation_diff_law | `mutation_diff_law` | mutations.rs |
| inverse_law | `inverse_law` (mutation- and diff-level) | mutations.rs |
| absorb_law | `absorb_law` (Add+Remove annihilate, Add+Add both survive, Add+SetField patch-into-added, Modify+Remove annihilate, associativity) | mutations.rs |
| between_roundtrip_law | `between_roundtrip_law` | mutations.rs |
| codec_retention_law | `codec_retention_law` | mutations.rs |
| op_text_binary_roundtrip_law | `op_text_binary_roundtrip_law` | mutations.rs |
| diff_codec_text_binary_roundtrip_law | `diff_codec_text_binary_roundtrip_law` | diff.rs |

Plus supporting tests: snapshot.rs (`json_pack_round_trips`, `dsl_text_round_trips`,
`default_snapshot_has_no_meshes_materials_or_textures`), diff.rs
(`between_apply_and_inverse_round_trip`, `absorb_composes_two_sequential_diffs`), mutations.rs
(`add_then_remove_mesh_apply_and_inverse`, `remove_mesh_inverse_restores_removed_mesh`,
`primitive_mutations_apply_and_inverse`, `material_and_texture_mutations_apply_and_inverse`),
composer.rs (4 validator tests) — **20 test functions total** in this subset's own tree.

## Own-scope compile proof

`cargo check -p semio-s-plugin-stdio --lib 2>&1 | grep -B3 -A15 "✳️mesh"` shows **zero errors**
under the mesh subset path, reconfirmed across **7 independent full-crate compile/test attempts**
taken at different points over ~30 minutes as sibling agents' files churned under heavy
multi-agent system load (`uptime` load average observed as high as 40 on a 10-core machine) — the
only two "✳️mesh" hits in the LAST run's full output are pre-existing-pattern warnings, both
non-blocking: (1) a hidden-lifetime-parameter warning on `ComposeSource` shared identically with
every other subset's composer; (2) I fixed the one genuinely-mine cosmetic warning
(`impl protocol::OpText for` → `impl OpText for`, since `OpText` was already unconditionally
imported) mid-wave — confirmed by the warning count dropping 232→231 between the two runs that
bracket the fix. Full-crate `cargo check`/`cargo test` remained blocked across every attempt by
FOREIGN, in-progress errors — at different points: `✳️presentation` (`SlideShapeDiff`/`SlideShape`
missing `Serialize`/`Deserialize`), then (as that got fixed) `✳️workflow`/`✳️image`/`✳️document`
(`SemioWorkflowMutation`/`SemioImageMutation`/`SemioDocumentMutation` missing `print_op`/`parse_op`
in scope — the identical `#[cfg(test)] use protocol::OpText` scoping bug this subset's own
mutations.rs already avoided) — all confirmed foreign via `git status` showing those files
`M`odified/mid-edit by other sessions, never by me. Polling, not chasing, per the ticket's
hazard-management guidance; the closer's `cargo test -p semio-s-plugin-stdio --lib` re-run should
go green once those 3 sibling subsets land the same one-line `OpText` import fix.

## Shared infra gaps (for the closer)

- **`🧰️triples::NamedTripleDiff<K,D,T>`'s derived `Deserialize` has a spurious `T: Default`
  requirement** (confirmed independently by the sibling `w2a-brep-report.md`, same root-cause
  serde_derive quirk bcf/docx's own LOCAL copies work around via an explicit
  `#[serde(bound(...))]` attribute the SHARED `🧰️triples` module lacks). **I did not hit this as a
  build failure** because all 4 of this subset's entity types used as the triple's `T`
  (`SemioMesh`/`SemioPrimitive`/`SemioMaterial`/`SemioTexture`) already derive `Default` (needed
  independently for their own test-fixture ergonomics), which happens to satisfy the spurious
  bound — but this is incidental, not a deliberate workaround, and the underlying bug is real:
  confirming a THIRD independent sighting (after brep, cad) of the same shared-file gap. Not fixed
  here (shared framework file, out of this subset's write scope) — the closer should consider
  adding brep's own `#[serde(bound(serialize = "K: Serialize, D: Serialize, T: Serialize",
  deserialize = "K: Deserialize<'de>, D: Deserialize<'de>, T: Deserialize<'de>"))]` fix once at
  `engine::triples::NamedTripleDiff`'s definition (mirrors bcf's/docx's own local attribute) rather
  than every W2 subset re-deriving `Default` on every entity type as an incidental workaround.
- No other genuine ambiguity or bug found in `engine::geometry`/`engine::triples`.

## Deltas vs baseline (w1b-close-report.md: 1231 tests, 21513 policy breaches)

**Policy**: `bun ./📜️script.ts policy` → 21528 high-priority breaches total (repo-wide, all W2
sibling agents' concurrent work included — not attributable to this subset alone). Filtering the
full breach list to paths containing `subsets/✳️mesh` or `subsets-mesh` yields exactly **2**
breaches, BOTH pre-existing, unchanged-by-me scaffold patterns shared identically across every
other subset in the program (confirmed by reading the exact flagged lines):
  1. `taxonomy/emoji-prefix` on the `📄set-snapshot` mutation-triad directory name (missing
     U+FE0F on `📄`) — the same directory-naming convention every other artifact's `📄set-snapshot`
     triad dir uses (bcf, docx, gif, ...), not something this wave renamed.
  2. `os-state-authority/item-scope-global` on `composer.rs:131`'s `static VALIDATOR_ENTRY:
     OnceLock<...>` — the identical pattern every other subset's composer registers its validator
     entry with (pdf's `✳️a` composer template, copied verbatim per the task brief); this line was
     not touched by this wave.

Both pre-date this wave's edits (present in the W1b scaffold before any snapshot/diff/mutations
work landed) — **zero new policy breaches attributable to this subset's real implementation**.
Confirmed zero `facet-mirror-drift`/`grammar-honesty`/`diff-algebra` breaches anywhere under
`✳️mesh` (these 3 S-8 rules, the ones this wave's real work specifically targets, are fully clean).

**Tests**: whole-crate `cargo test -p semio-s-plugin-stdio --lib` could not be brought to a green
run within this session's window — every attempt (9 total, spanning ~35 minutes under heavy
multi-agent system load, `uptime` load average peaking at 40 on a 10-core machine) failed on a
FOREIGN error (never `✳️mesh`, always outside this subset's write scope). The specific foreign
subset(s) blocking varied run to run as sibling W2 agents landed fixes: `✳️presentation` (missing
`Serialize`/`Deserialize`) → `✳️workflow`/`✳️image`/`✳️document` (missing-`OpText`-import, the same
one-line fix class this subset's own mutations.rs already carries) → most recently also
`✳️animation` (missing `MutationDiff` in scope, same import-scoping family). This subset's own 20
test functions are real, complete, and logically verified by hand against the proven bcf/docx
patterns they're built on (identical `between`/`apply`/`inverse`/`absorb` algorithm shape,
identical hand-rolled `DiffCodec`/`OpText` grammar style) — they have not yet been RUN end-to-end
because the whole crate must compile first. The last `cargo check -p semio-s-plugin-stdio --lib`
(non-test profile) DID confirm this subset compiles with zero errors and only one pre-existing
shared-pattern warning (plus one unavoidable test-only "unnecessary qualification" lint on the
`impl protocol::OpBinary for SemioMeshMutation` header, which must stay fully-qualified since
`OpBinary` is only imported under `#[cfg(test)]` — importing it unconditionally would instead
trigger an "unused import" warning in non-test builds, the same trade-off every hand-rolled
`OpBinary` impl in the repo accepts). **The closer must re-run `cargo test -p semio-s-plugin-stdio
--lib "artifacts::semio::standards::v1::subsets::mesh"` once the remaining foreign subsets
compile** and treat this report's test-count claims as unverified until that real output is
captured (per the ticket's own "Agent report without verbatim test output ≠ verification" rule).

## Files touched (all within `✳️mesh/`)

65 files modified under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/`:
snapshot/diff/mutations Rust (3 files, full rewrites) + schema-aggregator Rust (1 file) + composer
Rust (1 file) + builder/analyzer doc-comment touch-ups (2 files) + all facet-mirror `.ts`/
`.graphql`/`.json`/`.proto` leaves at the snapshot/diff/mutations/schema-root level (16 files) +
all 8 text + 6 binary grammar leaves × 3 facets (42 files). No files outside `✳️mesh/` were
touched (verified via `git status --porcelain`).
