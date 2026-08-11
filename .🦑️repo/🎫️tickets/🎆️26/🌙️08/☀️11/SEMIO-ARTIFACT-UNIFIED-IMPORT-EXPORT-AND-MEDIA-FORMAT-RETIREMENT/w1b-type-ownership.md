# W1b Cross-Subset Type Ownership

Feeds W2a/W2b prompts directly. Source: `w1b-scaffold-manifest.md` §4, cleaned up and
cross-checked against the actual scaffolded files on disk by the W1b closer.

All subset-owned types live under
`crate::artifacts::semio::standards::v1::subsets::<slug>::schema::snapshot`.

## Shared infrastructure (do NOT redefine per-subset)

- **🧮️geometry** — `crate::artifacts::semio::standards::v1::engine::geometry`
  (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/⚙️engine/🧮️geometry/🦀️component.rs`).
  Real, complete, tested (4 tests): `SemioPoint3`, `SemioPoint2`, `SemioUv`, `SemioRgba`,
  `SemioQuaternion`, `SemioTransform`. Rotation is a named 4-field struct
  (`SemioQuaternion{x,y,z,w}`), never a bare tuple/array, per the f6 §4.3 `DslField`-for-tuples
  gap. W2 subset agents `use` this instead of re-deriving.
- **🧰️triples** — `crate::artifacts::semio::standards::v1::engine::triples`
  (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/⚙️engine/🧰️triples/🦀️component.rs`).
  Real, complete, tested (5 tests incl. a nested-bracket-payload depth-awareness proof):
  `IndexedTripleDiff<D,T>` + `IndexModified`/`IndexAdded`, `NamedTripleDiff<K,D,T>` +
  `NamedModified`, `split_top_level`, `strip_brackets`, `enc_indexed_triple`/`dec_indexed_triple`,
  `enc_named_triple`/`dec_named_triple`. Ported from bcf's/docx's hand-rolled versions — this is
  what every W2 subset's real sparse diff (replacing today's full-replace scaffold) should be
  built on.

Both are mounted once at `standards::v1::engine::{geometry,triples}` in stdio's `📦️glue.rs`
(sibling submodules of the engine's own `🦀️component.rs`, matching gif's own
snapshot/text+binary sibling-mount convention).

## Per-subset owned types

| Subset | Owned types | Notes |
|---|---|---|
| brep | `SemioBrepSnapshot`, `BrepSolid`, `BrepSurface` | `BrepSurface` has `Plane`/`Cylinder` only today — `BrepCurve` name is RESERVED for W2, not yet defined. Informing source: step `⚙️engine/🧱️brep` + `StepSnapshot`. |
| mesh | `SemioMeshSnapshot`, `SemioMesh`, `SemioMaterial` | `SemioPrimitive` name RESERVED for W2 (positions currently flat on `SemioMesh`, not yet split into primitives). Informing source: gltf. |
| model | `SemioModelSnapshot`, `SemioModelElement`, `GeometryRef` | `GeometryRef{None,Brep{brep_id},Mesh{mesh_id}}` — named variants, no tuple. **Spec-mandated cross-reuse: model embeds brep/mesh snapshots** (via `GeometryRef`, resolved by id — not inline duplication). Informing source: ifc/4. |
| object | `SemioObjectSnapshot`, `SemioValue`, `SemioObjectEntry` | `SemioValue::Ref` variant RESERVED for W2 (object-graph refs). Informing source: json. |
| document | `SemioDocumentSnapshot`, `DocBlock`, `DocRun`, `DocStyle` | **Reused by `presentation`** (see below) — do not redefine block types in presentation. Informing source: docx/md; replaces PageDoc/TextDoc. |
| cad | `SemioCadSnapshot`, `CadEntity` | `Line`/`Circle` only today — `Arc`/`Ellipse`/`Polyline`/`Text`/`Insert`/`Solid`/`Dimension` variants RESERVED for W2. Informing source: dxf/dwg + 📐️cad plugin. |
| drawing | `SemioDrawingSnapshot`, `DrawNode` | Recursive `Group`/`Path`/`Text` — matches svg's `SvgNodeDiff` recursive-diff template per the master plan. Replaces DwgDrawing-as-neutral. |
| image | `SemioImageSnapshot`, `SemioImageFrame` | icc/metadata fields RESERVED for W2. Informing source: png/gif; replaces RasterImage. |
| video | `SemioVideoSnapshot`, `SemioVideoStream`, `SemioVideoSample` | Payload-opaque by design (honest boundary — container-typed, sample bytes opaque — matches the master plan explicitly). |
| audio | `SemioAudioSnapshot`, `SemioAudioChannel` | `tags` field RESERVED for W2. Informing source: wav-shaped. |
| animation | `SemioAnimationSnapshot`, `AnimTimeline`, `AnimChannel`, `AnimKeyframe`, `AnimValue` | `AnimValue::Rotation` variant RESERVED for W2 (only `Scalar`/`Vector` today). Informing source: gltf animations. |
| presentation | `SemioPresentationSnapshot`, `Slide`, `SlideShape` | **`SlideShape::TextBox` explicitly reuses `document`'s `DocBlock`** — spec-mandated cross-reuse per the master plan ("presentation mirrors document's block shape with own types"). `masters`/`layouts`/notes RESERVED for W2. |
| workflow | `SemioWorkflowSnapshot`, `WorkflowNode`, `WorkflowEdge` | **DISTINCT crate** from the OS kernel's own `semio_framework::WorkflowSnapshot`/`WorkflowNode` (that lives in `semio-framework`, mounted via W1's Task 5 workflow-mount fix; this lives in `semio-s-plugin-stdio`). Same names, zero collision risk (different crates), but do not conflate the two when reading code or writing W2 briefs. |
| *(envelope, `✳️any`)* | `SemioSnapshot` (struct, `{schema, subset}`), `SemioSubsetSnapshot` (tagged union enum) | `SemioSnapshot` wraps the union in a struct rather than deriving `ArtifactSchema` directly on the enum — de-risks against an unverified macro-on-enum capability. Owned by the closer/W2's envelope pass, not any one domain subset agent. |

## Cross-reuse summary (spec-mandated, from the master plan's Architecture section)

- **model embeds brep/mesh** — `SemioModelElement.geometry: GeometryRef` resolves by id into the
  brep/mesh subsets' own snapshots; model does not duplicate geometry inline.
- **presentation mirrors document's block shape with its own type names** —
  `SlideShape::TextBox` reuses `document::DocBlock` directly (not a presentation-local copy).

## Diff/Mutation shape (all 21 schema-owning units, uniform today)

Every subset's `<Prefix>Diff`/`<Prefix>Mutation` (13 semio subsets + the `✳️any` envelope + the 7
new format artifacts = 21 total) is currently a full-replace `<Prefix>Diff{replacement:
Option<Snapshot>}` + a single `SetSnapshot` mutation variant — genuinely law-tested (all 8 laws
pass, `field_sweep_full_replace_round_trip` present in every diff file, zero
`field-sweep-presence`/`grammar-honesty` policy breaches) but intentionally coarse. **This is
W2's primary job per subset**: replace the full-replace scaffold with a real `🧰️triples`-backed
sparse diff, at which point that subset's entry should come off
`POLICY_DIFF_COMPLETENESS_ALLOWLIST` in `📜️script.ts` (seeded by this wave, keyed by
`policyNormalizeRelPath`).

## What's genuinely real vs scaffolded placeholder (don't re-derive, don't assume broken)

See `w1b-scaffold-manifest.md` §6 for the full list with policy-impact numbers. Summary:

- **Real, tested, not placeholders**: 🧮️geometry, 🧰️triples, all 8 `🔣️component.json` vocabulary
  manifests, the 7 format artifacts' `sniff`/`parse_minimal` engine logic (real magic-byte
  detection + minimal structural parsers, 21 tests), tsv's engine specifically (fully complete,
  not partial — IANA TSV has no quoting so a full parser was genuinely correct to write day one),
  all 8 `📚️examples/🎬️demo` dirs (real non-empty assets, format examples are byte-identical copies
  of the real W0 fixtures).
- **Scaffolded placeholders** (W2/W3/W4 complete these): the full-replace Diff/Mutation shape
  above; grammar leaves (honest one-line 🚧-marked stubs, do not trip
  `POLICY_GRAMMAR_HONESTY_ALLOWLIST`'s banned-marker check so no allowlist entry needed there);
  the JSON-pack passthrough `ArtifactPack` impl for the 7 formats (W3 rewires onto each format's
  own real encoder — this is what's keyed in `POLICY_ROUND_TRIP_TEST_ALLOWLIST`, seeded by this
  wave); SubsetValidator (decode-only today, 13 real semio subsets need real referential-invariant
  checks from W2); io leaves (structure-only, no import/export deserializer/serializer leaf dirs
  exist yet anywhere under semio or the 7 formats — that's W4's explicit job, tracked via the
  catalog's `owners` row for `s.stdio.semio` having empty `import`/`export` lists on purpose until
  then, even though its `stdio_artifacts` field already documents the full 28-format bridged
  target list as a forward-looking capability statement).
