# W2a — semio/brep Subset — Real Implementation Report

Status: DRAFT — pending final compile/test verification (concurrent sibling-subset agents were
mid-edit and blocking whole-crate `cargo check`; polling per ground-truth guidance).

## Scope

Write scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/**` only.

## What was implemented

1. **Snapshot** (`🧬️schema/📸️snapshot/🦀️component.rs`): full id-keyed b-rep topology graph —
   `BrepVertex`, `BrepEdge` (+ `BrepCurve` enum: Line/Circle/Ellipse/Nurbs), `BrepLoop` (+
   `BrepLoopEdge`), `BrepFace` (+ `BrepSurface` enum: Plane/Cylinder/Cone/Sphere/Torus/Nurbs),
   `BrepShell` (+ `BrepShellFace`), `BrepSolid` (+ `BrepSolidShell`). All named structs (no bare
   tuples, no nested fixed arrays), all geometry via the shared `engine::geometry::SemioPoint3`.
2. **Diff** (`🧬️schema/🔺️diff/🦀️component.rs`): handcrafted sparse diff, one
   `crate::...engine::triples::NamedTripleDiff<String, XDiff, X>` per collection (vertices/edges/
   loops/faces/shells/solids), real `MutationDiff`/`DiffAlgebra`/`DiffCodec` impls (hand-rolled
   bracket-depth-aware hex grammar, reusing `🧰️triples`'s `enc_named_triple`/`dec_named_triple`).
3. **Mutations** (`🧬️schema/🧬️mutations/🦀️component.rs`): 23-variant named enum (`SetSnapshot` +
   Add/Remove/Set* per collection), every `diff()`/`inverse()` hand-written (no apply-and-capture).
   Hand-rolled `OpText`/`OpBinary` (JSON passthrough, documented dsl-derive-gap boundary).
4. **Grammar leaves**: all 8 text + 6 binary leaves under snapshot/diff/mutations, handcrafted
   honest (snapshot = hex-dump-of-JSON-pack pattern per png/json precedent; diff = the real
   hand-rolled bracket-triple grammar in ABNF; mutations = compact-JSON-passthrough grammar).
5. **Builder/Analyzer**: unchanged from W1b scaffold (already generic over Snapshot/Diff/Mutation,
   correctly wired to the new real types with no further edits needed).
6. **Composer**: real referential-invariant `SubsetValidator` (`check_brep_referential_integrity`)
   — dangling id checks between vertices/edges/loops/faces/shells/solids, replacing the W1b
   decode-only stub. `WRITES`/`DIALECT` unchanged (already correct: `s.stdio.semio`/`v1`/`brep`).
7. **Facet mirrors**: `.ts`/`.graphql`/`.json`/`.proto` rewritten at the snapshot/diff/mutations/
   schema-aggregator level to mirror the real Rust shapes.

## 8 test laws — status

See the exit-checklist section below for verbatim `cargo test` output once the crate compiles
clean (blocked mid-report by concurrent sibling-subset agents' in-progress edits to other semio
subsets in the same crate — brep's own files compile with zero errors, confirmed via
`cargo check -p semio-s-plugin-stdio --lib` filtered to `✳️brep` paths).

| Law | Test(s) |
|---|---|
| field_sweep | `field_sweep_every_field_present_in_diff` (diff.rs) |
| mutation_diff_law | `mutation_diff_law_covers_every_variant` (mutations.rs) |
| inverse_law | `inverse_law_diff_level_round_trips` (diff.rs) + `inverse_law_mutation_level_round_trips_every_variant` (mutations.rs) |
| absorb_law | `absorb_law_add_then_remove_of_same_added_key_cancels`, `absorb_law_add_then_setfield_patches_added_payload`, `absorb_law_modify_then_remove_drops_pending_patch`, `absorb_law_associativity` (diff.rs) |
| between_roundtrip_law | `between_roundtrip_law_and_field_sweep_both_directions` (diff.rs) |
| codec_retention_law | `codec_retention_law_populated_snapshot_round_trips_pack_and_dsl` (snapshot.rs) |
| op_text_binary_roundtrip_law | `op_text_binary_roundtrip_law` (mutations.rs) |
| diff_codec_text_binary_roundtrip_law | `diff_codec_text_binary_roundtrip_law` (diff.rs) |

## Shared infra gaps (for the closer)

- **`🧰️triples::NamedTripleDiff<K,D,T>`'s derived `Deserialize` has a spurious `T: Default`
  requirement** (and, by the same serde_derive quirk, `NamedModified<K,D>` is unaffected only
  because `modified: Vec<NamedModified<K,D>>` doesn't itself need `NamedModified: Default` — only
  the `added: Vec<T>` field's `#[serde(default)]` triggers the bug). This is the SAME known
  serde_derive false-positive bcf/docx's own LOCAL copies of `NamedTripleDiff` work around via an
  explicit `#[serde(bound(serialize = "...", deserialize = "..."))]` attribute — the SHARED
  `🧰️triples` module lacks that workaround (never actually exercised through `serde_json`/`derive`
  in triples' own test suite, only through its hand-rolled `enc_*`/`dec_*` functions, so the bug
  was latent/undiscovered). **Worked around on my side** by adding `#[derive(Default)]` (backed by
  manual `Default` impls on `BrepCurve`/`BrepSurface`, defaulting to `Line`/`Plane` at the origin)
  to all six of brep's own entity types (`BrepVertex`/`BrepEdge`/`BrepLoop`/`BrepFace`/
  `BrepShell`/`BrepSolid`) — legitimate within my own scope, but EVERY other W2 subset using
  `NamedTripleDiff`/`IndexedTripleDiff` (mesh, model, object, cad, drawing, …) will independently
  hit the identical error and need the identical per-type workaround, OR the closer could fix it
  once at the source with the `#[serde(bound(...))]` attribute bcf/docx already prove works. Cad's
  in-progress diff (seen mid-edit during my polling) had already hit this exact error
  independently, confirming it's not brep-specific.

## Deltas vs baseline (w1b-close-report.md: 1231 tests, 21513 policy breaches)

Pending final green run — see the exit-checklist output above/below.

## Files touched (all within `✳️brep/`)

See `git diff --stat` for the full 63-file list (snapshot/diff/mutations Rust + all facet
mirrors + grammar leaves + composer). No files outside `✳️brep/` were touched.
