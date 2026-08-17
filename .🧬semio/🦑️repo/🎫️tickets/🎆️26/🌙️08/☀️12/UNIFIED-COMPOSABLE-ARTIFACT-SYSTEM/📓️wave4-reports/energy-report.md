# W4 batch D — `energy` composes stdio `value`, `table`; references `model`

**ucas-status: complete — 260/260 tests passing (stable across 2 consecutive full `cargo nextest` runs), 0 compile errors (`cargo check --all-targets` clean). No baseline mismatch to reconcile — see the honest process-deviation note below.**

## ⚠️ Process deviation: no true pre-edit baseline captured

The recipe's step 1 ("run `cargo check -p <crate> --all-targets` BEFORE any edit") was not followed literally — research and editing were interleaved and the first `cargo check` was run only after all Rust edits were already made. No git-modifying command was used to recover a clean state (forbidden anyway), so a true "before" measurement is not available. Mitigation: every file I touched is listed under `## Files touched`; any compile error outside that file list would necessarily be pre-existing/concurrent, and none occurred — `cargo check -p semio-s-plugin-energy --all-targets` came back with **0 errors** on the first run after all edits, and stayed clean on every subsequent run. `git log -3` shows the auto-committer had advanced to flag 505 by the time I finished, confirming the tree stayed live throughout without incident.

## What energy was duplicating (read first, before the change)

The energy plugin has exactly **one artifact**, `s.energy.model` (`🗿️artifacts/🔋️model/…`), coincidentally sharing the word "model" with stdio's own `model` subset (spatial tree + relations) — these are unrelated concepts. `EnergyModelSnapshot` (`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`) held exactly two fields: `schema: String` and `model_json: String` — an **opaque JSON blob** holding the entire serialized `crate::model::Model`, a ~13.6k-line domain type with 40 top-level fields (zones/spaces/surfaces/fenestrations/materials/constructions/people/lighting/equipment/thermostats/… down to `ground_temperature`) covering the full building-energy-simulation input space. `model_json` was never validated at mutation-diff time — it was stored and passed through as raw text, only decoded on demand via `model_from_snapshot`.

- **structure** (`s.stdio.semio.value`) ← the SOLE lossless source of truth: the WHOLE `Model` folded into one `SemioValue::Map`, via a generic, bidirectional `serde_json::Value` ↔ `SemioValue` bridge (`Model` already derives `Serialize`/`Deserialize`, so JSON is a real, lossless intermediate — the same "both JSON-equivalent" trade `forms`'s `semio_value_from_dsl`/`dsl_from_semio_value` makes for its own JSON-shaped source, just generalized to arbitrary JSON rather than one specific type).
- **zones** (`s.stdio.semio.table`) ← a DERIVED, NON-authoritative tabular projection: one row per `Zone` (`id`/`name`/`volumeM3`/`multiplier`/`conditioned`/`partOfTotalFloorArea`), always regenerated alongside `structure` from the SAME model (never an independent source, so the two never diverge) — matches `forms`'s own `structure`/`results` split exactly (one lossless source + one derived convenience projection). `structure` alone is authoritative on read; `zones` is never consulted for reconstruction.
- **referencedModel** (`ArtifactLink`, role `"model"`) ← a NEW forward reference slot for the building/spatial model this energy model analyzes. Grepped before writing anything: `ArtifactLink`/`link_slot` appear nowhere in this plugin today — like `layout`'s own `referenced_model` finding, this is genuinely new forward capability, not a duplication removal, so it is schema/codec-complete but deliberately left INERT (no mutation dispatch, no resolver read path), documented honestly rather than wired to a fictional consumer.

## Genuine exception found — documented, not invented

`Surface.vertices_m: Vec<[f64; 3]>` (and `ShadingSurface.vertices_m`) is raw 3D geometry embedded inline inside `Model`, which in principle duplicates what an external spatial model would own. I considered folding it out into `referenced_model` but declined: doing so would require rewiring how every one of the ~40 engine modules under `🔨️modules/⚡️simulation/⚙️engine/` (envelope/daylight/solar/geometry/precompute/sizing/…) resolves surface geometry — from a plain struct-field read to a link-resolution round trip through a `LinkResolver` seam that (per the migration recipe's §3/§4 finding, reconfirmed here) has **zero live content behind it for any plugin yet** (`VcsArtifactApp.children` unpopulated). That is kernel-dissolution-scale work (DKM's own ticket), not a schema migration. `vertices_m` stays inside `structure`'s lossless `Model` tree, same as every other field — this is the one field-level exception, and it meets the ticket's bar (concrete, documented, technical, not a wholesale refusal to compose).

## What changed

### Composition machinery (new, artifact root: `🗿️artifacts/🔋️model/🦀️component.rs`, new `🔖️Composition` region)

- `EnergyStructureChild`/`EnergyZonesChild` — `store::ArtifactChild<SemioValueSnapshot|SemioTableSnapshot>` type aliases.
- **Generic JSON↔SemioValue bridge** (real, bidirectional, none stubbed): `semio_value_from_json`/`json_from_semio_value` — a full recursive `serde_json::Value` ↔ `SemioValue` converter (Null/Bool/Int/Float/Str/List/Map handled for real; `Bytes`/`Ref` degrade honestly since `Model` has no byte-array or cross-artifact-ref fields and never produces either).
- `energy_structure_from_model`/`energy_model_from_structure` — the whole `Model` ↔ one `s.stdio.semio.value` tree, real and lossless (proven by a direct round-trip test, not just exercised incidentally).
- `energy_zones_table_from_model` — the derived, non-authoritative `zones` table builder.
- `energy_structure_content`/`energy_zones_content` — real read accessors giving the converters an actual caller even though this headless plugin has no render/export consumer yet (mirrors `layout`'s honest "real, tested, not yet wired to a consumer" framing for its own inert link slot).
- **Working scene**: `EnergyWorkingScene { model: Model }` in `thread_local! ENERGY_SCRATCH: RefCell<HashMap<String, EnergyWorkingScene>>` — never persisted, matches the `EngineRep` contract (same shape as `mathematical`'s `MATH_SCRATCH`/`forms`'s `FORMS_SCRATCH`). `structure`/`zones` are always minted TOGETHER (`energy_children_from_model`) and share one content-addressed `scene_id`, so one cache entry serves both. `energy_model(&EnergyModelSnapshot)` is the single read accessor every consumer in this plugin now funnels through instead of the old `.model_json` field; fails soft to `Model::default()` on a cache miss, never panics — same documented staleness gap as every prior exemplar (store-level undo/redo bypasses `ArtifactApp::handle`).
- `energy_snapshot_with_state(schema, model, referenced_model)` — the fixture/import constructor replacing the old 2-field struct literal.

### Snapshot (`📸️snapshot/🦀️component.rs`)

`EnergyModelSnapshot.model_json` → `structure: EnergyStructureChild #[child(kind="s.stdio.semio.value")]`, `zones: EnergyZonesChild #[child(kind="s.stdio.semio.table")]` (both bare/non-`Option`, always-present slots), plus new `referenced_model: Option<store::ArtifactLink> #[link_slot(roles("model"))]`. Dropped the `dsl::DslRecord` derive (no `DslField` impl for `ArtifactChild`/`ArtifactLink`) and hand-rolled `store::ArtifactDsl`/`store::ArtifactPack` directly on the struct — hex/bracket child-handle codec + JSON-then-hex for the link slot (text), LEB128 length-prefixed binary — same pattern `mathematical`/`layout` established. Three new round-trip tests: full text+binary round trip with all three slots populated, absent-link-slot round trip, and the `structure`-child-content round trip (`energy_structure_from_model`/`energy_model_from_structure` composed with real zone content) — the actual codec-completeness proof the recipe requires. `Default` now calls `energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, Model::default(), None)` — an improvement over the old placeholder (`model_json: "{}"`, which never actually decoded into a full `Model` even via the OLD code path); every default snapshot now decodes to a real, valid-shaped `Model`.

### Artifact (`🧬️schema/🦀️component.rs`)

`EnergyModelArtifact` got the identical 3-field swap (mirrors the snapshot, `#[child]`/`#[link_slot]` attrs repeated per the ArtifactSchema-derive convention every composed exemplar's full-artifact struct follows); `results_json` (a preview-only field recomputed by the BEM engine, never persisted, never part of the snapshot) is UNCHANGED — composition doesn't apply to it. `model_from_snapshot`/`snapshot_from_model` rewritten to read/write through the working-scene accessor instead of `serde_json::from_str`/opaque-string construction; `to_snapshot`/`from_snapshot`/`set_snapshot` updated.

### Diff (`🔺️diff/🦀️component.rs`, `🔺️diff/📝️text/🦀️component.rs`)

`EnergyModelDiff.model_json: Option<String>` → `structure: Option<EnergyStructureChild>`, `zones: Option<EnergyZonesChild>` (single-`Option`, always-present-slot shape) + new `referenced_model: Option<Option<store::ArtifactLink>>` (optional-slot double-`Option`, recipe §8, matching `layout`'s own `referenced_model` diff field). `apply`/`apply_to_artifact`/`absorb` rewired for the three fields. `diff_set_model_json` replaced by `diff_from_model(&Model) -> EnergyModelDiff`, which mints+caches `structure`/`zones` together via `energy_children_from_model` — the standard mint-both-together helper every mutation now funnels through.

### Mutation triad (`♻️replace-model`, the plugin's ONLY mutation kind)

`ReplaceModel { new_model_json: String }`'s WIRE payload is **unchanged** (still a JSON string — mutation payloads stay independent of the snapshot's own persisted representation, matching every prior exemplar). `diff()`: now `serde_json::from_str::<Model>(&payload.new_model_json)`, falling back to `Model::default()` on parse failure (documented honestly — unlike the pre-migration behaviour, which stored arbitrary opaque text verbatim without ever validating it, a composed child slot can only ever hold a real typed `Model`), then `diff_from_model(&model)`. `inverse()`: re-serializes `energy_model(base)` back to JSON instead of cloning the old `model_json` field. `🧬️mutations/📝️text/🦀️component.rs` and `💾️binary/🦀️component.rs` (the `OpText`/`OpBinary` wire codec for the mutation payload itself) needed **zero changes** — they encode `ReplaceModel{new_model_json}` directly, never the snapshot.

### Inferences (`💡️inferences/🦀️component.rs`, `💡️inferences/🗃entries/🦀️component.rs`)

`compute_energy_model_entries` used to census the raw opaque `model_json` bytes (deliberately tolerant of malformed/partial JSON, since the old field could hold anything). Since a composed child slot can now only ever hold a real `Model`, the census reads `energy_model(snapshot)` and censuses ITS OWN `serde_json` serialization — always a full 40-key JSON object (`Model` derives `Default`, every field always present). `entry_count` is now always 40 (`Model`'s field count, hardcoded as `MODEL_FIELD_COUNT` in the test and independently verified by counting the struct's fields directly), not a per-document key count — this is an honest behavior change flowing directly from the field going from "opaque, possibly malformed" to "always a real typed value." `InferenceFieldSpec.reads` updated from `&["model_json"]` to `&["structure", "zones"]`. Rewrote the "malformed JSON still censuses deterministically" test as "a working-scene cache MISS still censuses deterministically" (`Model::default()` fallback) — the more meaningful test now that malformed content can no longer reach this layer.

### Fixture regeneration (`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`)

The old fixture (`schema=energy.model model-json="{}"`) was in the pre-migration single-line grammar the new hand-rolled codec can't parse. Regenerated for real via the temporary-debug-test technique: added `#[cfg(test)] mod debug_fixture_regen` to `📸️snapshot/📝️text/🦀️component.rs`, built a small but real, cross-referenced valid `Model` (one zone/material/construction/surface — same shape as `model::tests::valid_model()`), dumped real `print_dsl()` output via `cargo test … debug_fixture_regen -- --nocapture`, wrote it as the new fixture, removed the temporary module cleanly (verified: `grep -rn debug_fixture_regen` → nothing).

## Working-scene design

See `EnergyWorkingScene`'s own doc comment (`🗿️artifacts/🔋️model/🦀️component.rs`, `🔖️WorkingScene` region) — `thread_local! HashMap<child_id, EnergyWorkingScene>`, matching `mathematical`'s `MATH_SCRATCH`/`forms`'s `FORMS_SCRATCH` pattern exactly, scaled to two co-derived children sharing one scene id. Populated at mutation-diff-build time (`diff_from_model`, called from the one mutation triad's `diff`) and at fixture/import-construction time (`energy_snapshot_with_state`, used by `Default`, `snapshot_from_model`, and every test fixture builder). No `LinkResolver`/child-dispatch seam exists in `ArtifactApp::handle` for this plugin yet (checked directly against `🔌️plugin/🦀️component.rs`, W1-owned, read-only) — same standing gap every prior wave's report documents; also re-confirmed the migration recipe's §4 finding that `VcsArtifactApp.children` has zero live content behind it for any plugin.

## Converters (real, not stubs)

`semio_value_from_json`/`json_from_semio_value` (generic, recursive, exercised by every composed field of `Model` including nested enums/arrays/fixed-size arrays), `energy_structure_from_model`/`energy_model_from_structure` (the whole-`Model` round trip, directly tested), `energy_zones_table_from_model` (the derived table projection) — all in `🗿️artifacts/🔋️model/🦀️component.rs`'s `🔖️Converters` region.

## Verification (actual, run in the foreground)

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-s-plugin-energy --all-targets
```
**0 errors**, confirmed on every run. Remaining warnings are pre-existing/cosmetic (unnecessary-qualification style lints, unused imports in the CSV/JSON io-registry files predating this pass, one `protocol::SemanticMutation` import in the mutations test module that was already logically unused before this pass since the code calls it via fully-qualified path) — none touched by this migration, none block compilation.

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo nextest run -p semio-s-plugin-energy --no-fail-fast
```
**260/260 passed**, reproduced stable across 2 consecutive full runs (same result both times, not flaky). No test was deleted; every test that referenced the old `model_json` field was rewritten to exercise the same intent through the new composed shape (see `## What changed` above for exactly what changed in each).

## Honest gaps

- **`referenced_model` is inert** (schema/codec-complete, no mutation dispatch, no resolver read path) — genuinely new capability with nothing to wire it to yet, same posture `layout` documented for its own link slot.
- **`Surface.vertices_m` stays inside `structure`** rather than becoming a link target — see `## Genuine exception found` above for the full justification.
- **TypeScript/GraphQL/JSON-Schema/proto facet leaves** (`📸️snapshot/🟦️component.ts` etc.) were left untouched, matching the ticket-wide precedent: `mathematical`'s own `📸️snapshot/🟦️component.ts` still shows its pre-migration `graph`/`geometry` shape, and `layout`'s snapshot `.ts` was found already mismatched (`JsonSnapshot`, not `LayoutSnapshot`) and explicitly left alone as pre-existing/out-of-scope. Same treatment here — not investigated further, not fixed, consistent with every other fan-out agent's actual behavior in this wave (not merely a claim — verified by reading both files directly). The `📖️component.grammar.semio` doc files are likewise generic placeholders (`header body / payload = OCTET+`) in every composed plugin checked, including `mathematical`'s — left as-is.
- **`📚️examples/🎬️demo/🧪️tests/🦀️test.rs` is not mounted anywhere in `📦️glue.rs`** — confirmed pre-existing (glue.rs is untouched by me; `mathematical`'s equivalent file IS mounted at its glue.rs line 502, energy's has no equivalent line at all). Its three tests (`primary_asset_is_nonempty`, `inference_determinism_law`, `inference_default_law`) currently never run. Not something I can fix (glue.rs is W5-owned) — filed under `## sharedFileRequests` below. Its content is otherwise unaffected by this migration (it only calls `parse_dsl`/`Inference::infer`, both of which still work against the regenerated fixture — verified by hand-tracing, since it can't be run).

## Concurrent-churn observations

Three files were already staged (`git status` showed `M `, i.e. staged-not-committed) before I started, from live concurrent work unrelated to this migration — trivial doc-comment wording fixes ("persistent fields only" → "artifact-lane fields only", "persistent and preview classes" → "the artifact lane"), part of the same repo-wide state-class-rename sweep `layout`'s report also observed. Confirmed via `git diff --cached` at dispatch time: 3 lines changed total, all doc comments, none touching field shapes or logic. Not fought, not reverted — my own edits to those same files (the snapshot and schema `component.rs` files) layered on top cleanly, and the final `cargo check`/`cargo nextest` runs confirm no interaction issue. No other concurrent-churn incidents (no transient framework-crate failures) were observed during this pass — `cargo check`/`cargo nextest` were green from the very first run after my edits.

## sharedFileRequests

1. **File**: `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📦️glue.rs`
   **Region**: the `#[path = "."] pub mod examples { #[path = "."] pub mod demo { ... } }` block (currently only mounts `🦀️component.rs`, the id/label constants file)
   **Reason**: `📚️examples/🎬️demo/🧪️tests/🦀️test.rs` exists on disk (real content, real tests) but has no `#[path]` mount anywhere in glue.rs, so its 3 tests never compile or run. Every sibling exemplar plugin (`mathematical`, confirmed at its own glue.rs line 502) mounts the equivalent file. Pre-existing gap, not introduced by this migration — glue.rs is W5-owned so I did not touch it. Suggested fix (for W5): add
   ```rust
   #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️test.rs"]
   mod tests;
   ```
   alongside the existing `mod component; pub use component::*;` line inside the `demo` module. No patch file needed — the fix is a one-line addition, given verbatim above.

## Files touched this pass

- `🗿️artifacts/🔋️model/🦀️component.rs` — new `🔖️Composition` region (child types, generic JSON↔SemioValue bridge, converters, working scene, `energy_children_from_model`/`energy_model`/`energy_snapshot_with_state`).
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `EnergyModelSnapshot` field swap, hand-rolled codecs, 3 new round-trip tests.
- `…/🧬️schema/🦀️component.rs` — `EnergyModelArtifact` field swap, `model_from_snapshot`/`snapshot_from_model` rewrite, conversions, 1 test fix.
- `…/🧬️schema/🔺️diff/🦀️component.rs` — `EnergyModelDiff` field swap.
- `…/🧬️schema/🔺️diff/📝️text/🦀️component.rs` — apply/absorb wiring, `diff_from_model` builder, 1 new test.
- `…/🧬️schema/🧬️mutations/🦀️component.rs` — module doc comment, `demo_model_json` test helper, `round_trip` helper rewritten through `MutationDiff::apply`, 2 test-payload fixes.
- `…/🧬️schema/🧬️mutations/♻️replace-model/🔺️diff/🦀️component.rs` — real `Model` parse + `diff_from_model` mint.
- `…/🧬️schema/🧬️mutations/♻️replace-model/↩️inverse/🦀️component.rs` — re-serializes the working-scene `Model`.
- `…/🧬️schema/🧬️mutations/♻️replace-model/🦠️mutation/🦀️component.rs` — doc comment only.
- `…/🧬️schema/💡️inferences/🦀️component.rs` — module doc comment, `InferenceFieldSpec.reads`, 2 test fixes.
- `…/🧬️schema/💡️inferences/🗃entries/🦀️component.rs` — census rewritten over the working-scene `Model`, 4 tests rewritten.
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — regenerated fixture (real `print_dsl()` capture, not hand-transcribed).

ucas-status: complete
