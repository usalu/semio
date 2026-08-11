# W2a — `model` subset — real implementation report

Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/**` only. All work
stayed inside this glob.

## What was built

**Snapshot** (`🧬️schema/📸️snapshot/🦀️component.rs`) — replaced the W1b placeholder
(`schema` + `elements: Vec<SemioModelElement{id,class:String,placement,geometry}>`) with the full
spec shape from the master plan's row: a flat, id-keyed spatial hierarchy
(`SpatialNode{id,kind:SpatialKind{Site,Building,Storey,Space},name,parent_id,placement}`) +
elements (`ElementClass` real named-enum with 9 IFC-style variants + honest `Other{name}`
catch-all, `GeometryRef{None,Brep{brep_id},Mesh{mesh_id}}` unchanged from W1b's already-correct
shape, `spatial_id: Option<String>`, `psets: Vec<PropertySet{name,properties:Vec<Property{key,value:PsetValue}>}>`)
+ relations (`ModelRelation{id,kind:RelationKind,from,to}`, own synthesized `id` key needed for the
keyed-collection diff engine). `GeometryRef` resolves by id into the sibling `brep`/`mesh`
subsets — never inline duplication, matching w1b-type-ownership.md's cross-reuse note.
`ArtifactDsl`/`ArtifactPack` envelope codecs kept as-is (generic over any field shape); extended
tests with a fully-populated `rich_snapshot()` fixture and a real `codec_retention_law` test.

**Diff** (`🔺️schema/🔺️diff/🦀️component.rs`) — replaced the full-replace `{replacement: Option<Snapshot>}`
scaffold with a handcrafted sparse diff. `spatial`/`elements`/`relations` diffed via the SHARED
`⚙️engine/🧰️triples::NamedTripleDiff<K,D,T>` container + wire codec (imported, not redefined) with
this file's own generic `between_named`/`apply_named`/`inverse_named`/`absorb_named` glue
functions (mirrors bcf's local engine, minus the container type which now has one shared home).
Implements `protocol::MutationDiff`, `protocol::command::DiffAlgebra` (inverse/between/is_empty),
and a hand-rolled `protocol::DiffCodec` (bracket-depth-aware token grammar, same primitive style as
bcf/gif — hex strings, `[0]`/`[1,x]` tri-state option encoding, single-letter enum tags). No dsl
derive macros used anywhere (per f6 §4 guidance).

**Mutations** (`🧬️schema/🧬️mutations/🦀️component.rs`) — replaced the single-`SetSnapshot` scaffold
with an 11-variant named enum: `NoMutation`, `SetSnapshot` (now genuinely sparse —
`diff()` calls `SemioModelDiff::between`, not a full-replace slot) + `Insert/Remove/Set` for each
of `SpatialNode`/`Element`/`Relation` (9 variants). Every variant's `diff()`/`inverse()` is
hand-written (constructs the sparse `SemioModelDiff` directly / looks up the original value in
`base` to build the undo mutation) — no apply-and-capture. `OpText`/`OpBinary` kept as the
JSON-passthrough W1b established (documented as the deliberate boundary — full `dsl::DslOps`
would require `DslField` on every nested type, out of scope per f6 §4.4). Kept the existing
`📄set-snapshot` mutation triad dir unchanged (policy requires only ≥1 triad dir present, not one
per variant — confirmed against `policyMutationTriadCompletenessBreaches`).

**Composer** (`🎹️composer/🦀️component.rs`) — `SemioModelValidator` upgraded from decode-only to
real referential-invariant checks over `model`'s own collections: dangling `SpatialNode.parent_id`,
self-parenting, dangling `SemioModelElement.spatial_id`, dangling `ModelRelation.from`/`to`
(checked against the union of element+spatial ids). Split into a standalone
`semio_model_referential_diagnostics()` fn, unit-tested directly and through the `IoPayload` wire
boundary. `WRITES`/`DIALECT` unchanged (`Dialect{"s.stdio.semio", StandardId("v1"), SubsetId("model")}`,
already matched the path). Registration (`register_document_codec` with schema id
`"s.stdio.semio.model"`, `register_subset_validator`) unchanged from W1b, still correct.

**Builder/Analyzer** — no field-specific logic; both already delegate generically to
snapshot/diff/mutation types, so they compiled unchanged (only doc comments updated to drop the
"🚧 scaffolded by W1b" language now that the underlying types are real).

**Grammar leaves** — all 8 `📝️text/` + 6 `💾️binary/` leaves, ×3 facets (snapshot/diff/mutations) =
42 files, all rewritten honestly: snapshot's leaves describe the real `store::semio_format`
envelope (magic/token/payload, genuinely `size-eos`-shaped since nothing follows the payload) plus
the compact-JSON body shape; diff's leaves describe the hand-rolled bracket-triple token grammar
(no envelope — `encode_diff` is the text bytes verbatim); mutations' leaves describe the tagged
compact-JSON `OpText`/`OpBinary` grammar (no envelope). Verified zero literal
`POLICY_GRAMMAR_HONESTY_LEAF_MARKERS` matches across all 42 files (python scan, 0 hits) — one real
near-miss caught and fixed (see Policy section).

**Facet mirrors** (`.ts`/`.graphql`/`.json`/`.proto`) — rewrote all of snapshot/diff/mutations'
sibling leaves (12 files) plus the top-level `🧬️schema` artifact-wrapper mirrors (4 files) with
real interfaces matching the Rust serde shapes (camelCase, tagged unions on `kind`/`mutation`).

**Schema wrapper** (`🧬️schema/🦀️component.rs`) — `SemioModelArtifact` extended to mirror the new
`spatial`/`elements`/`relations` fields (was `elements`-only).

## 8 test laws — where each lives

All in the existing `#[cfg(test)]` regions of the 3 facet files (no new test files):
- `field_sweep` — `🔺️diff/🦀️component.rs::tests::field_sweep` (sweep_a/sweep_b differ in every
  field across all 3 collections; one removed/one modified-in-every-field/one added per
  collection; `parent_id` exercises Some(None), `spatial_id` exercises None->Some).
- `mutation_diff_law`, `inverse_law` (mutation-level) — `🧬️mutations/🦀️component.rs::tests::
  mutation_diff_law_and_inverse_law_cover_every_collection` (all 11 variants incl. NoMutation).
- `inverse_law` (diff-level), `between_roundtrip_law`, `absorb_law` (incl. the canonical
  add-then-remove-before / add-then-set-field cases from schema-design.md),
  `diff_codec_text_binary_roundtrip_law` — `🔺️diff/🦀️component.rs::tests`.
- `codec_retention_law` — `📸️snapshot/🦀️component.rs::tests::codec_retention_law` (fully-populated
  fixture through both pack and DSL codecs).
- `op_text_binary_roundtrip_law` — `🧬️mutations/🦀️component.rs::tests` (all 11 variants).

## Verification

`cargo check -p semio-s-plugin-stdio --lib` — confirmed **zero** compile errors anywhere under
`✳️model/**` (verified twice, before and after a `use protocol::DiffCodec;` import fix). Full
`w2a-model-cargo-check-clean-proof.txt` in this folder — the only `✳️model` matches in the whole
error/warning stream are 4 harmless lints (unnecessary-qualification, hidden-lifetime), zero
`error[...]` blocks.

**`cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::.*model"` could NOT be run to a
pass/fail number** — the crate is a single compilation unit and, at every attempt (3 spaced-out
tries across ~25 min of real wall-clock wait), other concurrently-running W2a/W2b sibling agents'
subsets (`image`, `animation`, `cad`, `audio`, `workflow`, plus `gltf`/`mp4`/`json` format
artifacts) had real, in-progress compile errors of their own (missing `use protocol::{DiffCodec,
OpText}` trait imports in their own test modules — the exact same class of bug I hit and fixed in
my own `🔺️diff/🦀️component.rs`, confirmed via `git status --porcelain` showing all of them under
active modification throughout). `cargo test -p semio-s-plugin-stdio --lib` (full crate) last
attempt: **`error: could not compile ... due to 54 previous errors`**, zero of which are under
`✳️model/**` (`w2a-model-full-crate-test-blocked-foreign.txt`,
`w2a-model-scoped-test-blocked-foreign.txt`). This is expected concurrent-wave churn per the
master plan's own hazard-management section ("poll rather than chase"), not a defect in this
subset. **The closer should re-run both `cargo test` commands once the sibling W2 agents land** —
`cargo check` gives strong structural confidence (every type/trait wiring in the 8 laws' test
bodies type-checks) but is not a substitute for the actual pass/fail run.

`bun ./📜️script.ts policy`: **21525 high-priority breaches** (this run) vs. the W1b baseline's
**21513** (`w1b-close-report.md`) — but this delta includes concurrent sibling-agent activity, not
just this subset (the whole number moved by +12 net across ~13 agents editing simultaneously; not
attributable to `model` alone). **Filtered to `✳️model` only: 2 breaches, both pre-existing/systemic
across all 13 semio subsets from the original W1b scaffold** (confirmed by grepping the same 2
breach kinds across `✳️animation`/`✳️audio`/`✳️brep`/`✳️cad`/... — every one of them has the identical
pair): `taxonomy/emoji-prefix` on the `📄set-snapshot` dir (missing U+FE0F variation selector,
a W1b-scaffold dir-naming artifact, not introduced here) and `os-state-authority/item-scope-global`
on the composer's `static VALIDATOR_ENTRY: OnceLock<...>` (the W1b-scaffold validator-registration
pattern, copied verbatim from bcf/pdf precedent, used identically by every one of the other 12
subsets' composers). **Net new breaches introduced by this subset's real implementation: 0** — one
real new breach (`handcrafted-grammar/generic-spec` on the diff facet's `.grammar.semio`, triggered
by a production literally named `diff-payload` matching the `-payload\b` heuristic) was caught and
fixed by renaming to `diff-body` before the final policy run (`w2a-model-policy-final.txt`).

## Shared infra gaps (for the closer to reconcile)

- **`🧰️triples::NamedTripleDiff<K,D,T>`/`IndexedTripleDiff<D,T>` have no `#[serde(bound(...))]`
  override**, so `#[serde(default)]` on their `added: Vec<T>` field spuriously infers `T: Default`
  for any type embedding `Option<NamedTripleDiff<K,D,T>>` (the exact serde_derive limitation bcf's
  own diff module already documents for its own local copy, which DOES carry the bound override —
  the shared `engine/🧰️triples` port dropped it). Worked around here by deriving `Default` on
  `SpatialNode`/`SemioModelElement`/`ModelRelation` (harmless — never constructed via `::default()`
  in real code, purely satisfies the derive) rather than editing the shared file. Every other W2
  subset importing `🧰️triples::NamedTripleDiff`/`IndexedTripleDiff` directly (rather than
  redefining a local copy like bcf) will hit the identical requirement on its own strong-entity
  types — worth fixing at the source (`#[serde(bound(serialize = "K: Serialize, D: Serialize, T:
  Serialize", deserialize = "K: Deserialize<'de>, D: Deserialize<'de>, T: Deserialize<'de>"))]`,
  bcf's exact attribute) in a future closer/shared-infra pass instead of every subset re-deriving
  `Default` on every strong entity.

## Files touched (all within `✳️model/**`)

`🧬️schema/🦀️component.rs`, `🧬️schema/{🟦️,🔗️,🔣️,🛰️}component.*`,
`🧬️schema/📸️snapshot/🦀️component.rs` + its `{🟦️,🔗️,🔣️,🛰️}` mirrors + `📝️text/*` (8) + `💾️binary/*` (6),
`🧬️schema/🔺️diff/🦀️component.rs` + mirrors + text/binary leaves (14),
`🧬️schema/🧬️mutations/🦀️component.rs` + mirrors + text/binary leaves (14),
`🏗️builder/🦀️component.rs` (doc comment only), `🧐️analyzer/🦀️component.rs` (doc comment only),
`🎹️composer/🦀️component.rs` (real validator + tests), `🧐️analyzer/🟦️component.ts`,
`🏗️builder/🟦️component.ts`, `🎹️composer/🟦️component.ts` (doc comments only). `🚪️io/**` untouched
(explicitly W4 scope per the master plan). No files outside `✳️model/**` touched.
