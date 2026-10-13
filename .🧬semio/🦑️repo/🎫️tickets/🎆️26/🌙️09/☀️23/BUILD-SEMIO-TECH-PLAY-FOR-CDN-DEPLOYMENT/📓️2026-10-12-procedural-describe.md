# 2026-10-12 Procedural Describe Refusal And Hub Assembly Sweep

Executor: `proc-describe` (Sonnet). Trigger: cold release build #7, `@semio-tech/procedural-plugin:describe` refused with `plugin assembly failed — no declared inference capability owns the runtime claims`.

## Root cause (procedural)

Not today's migration: `HEAD` already carries the same wiring. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` validates every `ArtifactDeclaration` in two layers, and `generation3d` failed both:

1. `require_declared_capability_or_record` (`.inference_services(...)`): the runtime claim of a service is `schema:<metadata.inference_schema>` and it must equal the claim set of one declared `inference` capability. The geometry service (`🔨️modules/🏠️host/📐️geometry-service`) published `inference_schema = "s.procedural.generation3d.geometry"`; the only declared inference capability claims `s.procedural.generation3d.inference`. This is the refusal text from the build.
2. Behind it, `ArtifactDeclaration::preflight` (`plugin-assembly.inference-owner`, then `plugin-assembly.inference-descriptor`): a service's `owner` must equal the plugin id (`"procedural"`, the service said `"procedural-generation3d"`), and a descriptor registered through `.inferences([...])` must have `id == inference_schema`. The only descriptor is `generation3d_artifact_inference_descriptor()` with id `s.procedural.generation3d.inference`.

The established convention (`🌍️gis/gismap`, `🧰️framework/🛍️products/📓️print`) is: the artifact's service IS the service of its one declared inference, so `inference_schema == descriptor id == declared capability claim`, and `owner == plugin id`. `generation3d` deviated, so the declaration is not stale; the service metadata drifted from the declaration. (Alternative considered and rejected: a second `inference.geometry` capability plus a second facet-leaf descriptor, the `gltf` per-leaf shape. It needs five authored facet files for a payload contract that already has its own request/result JSON, and `Generation3dInference` already lists `geometry` as one of its two fields.)

## Fix (procedural, source of truth in the artifact, schema-first)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🛰️service/🦀️.rs`: `GEOMETRY_INFERENCE_SCHEMA = "s.procedural.generation3d.inference"` (was `...geometry`). The payload schema stays `s.procedural.generation3d.geometry.payload`.
- `…/🧊️generation3d/🔨️modules/🏠️host/📐️geometry-service/🦀️.rs`: `owner: "procedural"` (was `"procedural-generation3d"`).
- `…/geometry-service/🧪️tests/🔬️unit/🦀️.rs`: the metadata test asserts `(owner, artifact_kind, inference_schema)`, the registry lookup uses the new schema id.
- `…/✳️any/✏️editor/🦀️.rs`: two docstrings name the new schema id.
- No generated artifact definition exists for procedural (the `definition()` in `…/🧊️generation3d/🦀️.rs` is hand-authored; only `stdio` artifacts such as `gltf` have `📜️artifact-definition.json`), so nothing to regenerate. `🦀️.rs` of generation3d is unchanged at the end (a temporary extra capability was added and removed).

## Proof

- Native: `cargo test -p semio-hub-procedural --lib` (private target, slot-gated): 7 of 8 pass, including `plugin_manifest_builds_synchronously`, `generation3d_manifest_examples_are_registered_on_the_dialect_for_both_surfaces`, `list_flow_extensions_*`. The 8th, `descriptor_is_fresh`, failed only because the committed `🛂️.descriptor.semio` was the stale assembly-failed stub, i.e. what the real describe then rewrote.
- Real: `NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/procedural-plugin:describe --skip-nx-cache` (wrapped in `🚦️cargo-slot.sh`, `SLOTS=2`) exit 0, `described procedural (plugin semio:procedural@0.1.0)`, wasm sha256 `98a311fa…`, descriptor sha256 `53d443d5…`, fuel 1.62e9 of the 8e9 cap. It rewrote `🌎️hub/🧩️compositions/🌀️procedural/🛂️.descriptor.semio` + `🔣️.json`; `pluginId` is `procedural`, no `assembly-failed`, and `inferenceServices[0].inferenceSchema` is `s.procedural.generation3d.inference`.
- NOT run: the geometry-service unit tests. `semio-s-artifact-procedural-generation3d --lib` does not compile as a test target because of an unrelated missing file, `…/🧬️mutations/❌delete-widget/🧪️tests/🔓️removes-an-unselected-middle-row/🦀️.rs` (declared by `mod tests_removes_an_unselected_middle_row;` at `…/🧬️mutations/🦀️.rs:221`). The edits to that test are one assertion tuple and one string.

## Sweep of the other hubs

Method: native lib-test assembly is not available for most hubs (hub lib test targets do not compile: `plugin_exports!`'s `descriptor_is_fresh` needs `::semio_framework_async` in hubs that do not depend on it, stale `assert_viewer_never_mutates` arities, wfc tests naming `schema::inferences::*_inference_metadata` that now live in `host::inferences`; also a transient `os-kernel` break in `store/🧩️composition/🗄️durable-group` from a peer's `prepare_one_item` signature change when several hubs are tested in one invocation). So the sweep used the real describe emitter, two ways:

1. Run `semio-framework-plugin-describe describe` directly over the existing `dist/component-dev/*.wasm` of every hub (separate core file, scratch `--out`, nothing written into the tree). These wasms are from build #7 or older, so a pass is weaker than a rebuild: hubs whose wasm predates the `opt-level = 1` profile change (10-11 12:30) are listed as "old wasm".
2. For the hubs that failed or were never described this build, the real `nx run @semio-tech/<hub>-plugin:describe --skip-nx-cache` (rebuilds the wasm from current sources).

| hub | result |
| --- | --- |
| procedural | fixed, nx describe OK |
| bim | **second real defect, fixed**, nx describe OK (see below) |
| energy | nx describe OK, fuel 3.80e9 (the 8e9 exhaustion seen on its old pre-`opt-level=1` wasm is gone) |
| fem | nx describe OK |
| reasoning | nx describe OK |
| wfc | nx describe OK, fuel 1.37e9 (the "exceeds 268435456 bytes" on the old wasm is gone, same cause) |
| architect | **third defect, fixed**: describe fuel cap too small, see below; nx describe OK after the fix (fuel 22.66e9) |
| writer, mathematical, animate, sequence, shooting, forms, lowpoly, vcs, imperative, remodel, trinity, dag, layout, draw, raster, note, space, sourcing, puzzle, block | direct describe OK on the existing wasm (shooting, lowpoly, imperative, remodel, layout, draw, raster, block on old wasm) |
| flow, cad, demonstrator, gis, playbook, process | described OK inside build #7 itself |
| norm, stdio | NOT verified. No wasm in `dist/component-dev`; a norm nx describe was started, then stopped after 26 min with no cargo progress while the cargo build lease was contended, stdio never started. Both embed the same `stdio-semio` member set that makes architect cost 22.7e9 fuel, so expect them to need the new cap. |

### bim: duplicate panel tab ids (assembly panic, not a refusal)

`describe` trapped with `app s.bim.model@1/*#editor duplicate panel tab id framework.panel.catalogue` (`validate_panel_tab_spec`, plugin `🦀️.rs:918`). The BIM editor registered 10 top-level `PanelTabDefinition`s that reused the 3 framework ids: Catalogue ×3 (`library`, `families`, `classification`), Inspection ×6 (`properties`, `diagnostics`, `options`, `clashes`, `rules`, `issues`), Artifact ×1. Same at `HEAD`; the bim pane is new in this lane.

Fix, the shape `📸️remodel` already uses for nested tabs: every panel now owns a unique id (`bim.panel.<name>`, constant `PANEL_TAB_ID`, `clashes/rules/issues` via `*_TAB_ID`), and `create_bim_app` groups them under exactly one framework tab each through `framework_panel_tab`: Catalogue (Workbench) with library, families, classification; Inspection (Details) with properties, diagnostics, options, clashes, rules, issues. Outliner stays the single Artifact tab. Body keys, labels and renderers are untouched.

Files: `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` and `…/📌️panels/{🛍️library,🪑️families,🗂️classification,🔍️properties,🚨️diagnostics,🧭️options,🤝️coordination}/🦀️.rs`. A static scan of every `PanelTabKind::App(...)` under `✏️s/🔌️plugins/**/🗿️artifacts` found no other artifact with colliding ids. Proof: lib compiles (`semio-s-artifact-bim-model --lib`; its `--lib` test target does not, unrelated `Point`/`new_app_with_registry` test debt) and `nx run @semio-tech/bim-plugin:describe` exit 0 (`described bim (plugin semio:bim@0.1.0)`, fuel 2.24e9).

### architect: describe fuel cap

On the current `opt-level = 1` build `architect` (one artifact, `s.architect.program`, hosted over the whole `stdio-semio` member set) exhausted `DESCRIBE_FUEL_BUDGET = 8_000_000_000` at 212 s. Measured with the cap raised: it completes at **22_659_514_423** fuel (≈ 490 s). The emitter's own doc says to re-measure and take the next measured step, so `DESCRIBE_FUEL_BUDGET = 32_000_000_000` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs`, with the measurement recorded in its docstring. The deadline is the no-fuel-progress stall bound, not total wall time, so it is unaffected. Verified twice with the emitter built from that source: direct describe of the nx-built `semio_hub_architect.wasm` OK, then `nx run @semio-tech/architect-plugin:describe --skip-nx-cache` exit 0 (`described architect (plugin semio:architect@0.1.0)`, fuel 22_657_753_969, ≈ 13 min incl. the emitter rebuild).

## Open items for the coordinator

1. norm and stdio hub describes are unverified (above). The first thing to look at if build #8 fails there is `fuel exhausted`, then the 1 M function ceiling.
2. Regenerated tracked descriptors (written by the real describe, expected): `🌎️hub/🧩️compositions/{🌀️procedural,🏙️bim,🔋️energy,🏗️fem,💡️reasoning,🀄️wfc,🏛️architect}/{🛂️.descriptor.semio,🔣️.json}`.
3. Hub lib test targets do not compile (list above). The `descriptor_is_fresh` generated test wants `::semio_framework_async` in every hub that uses `plugin_exports!` (procedural has it as a dependency; writer, flow, … do not). This blocks a fast native sweep for next time; the clean fix is for `__semio_plugin_descriptor_fresh_test` to reach `resolve_ready` through `$crate`.
4. `generation3d` lib test target is missing `…/🧬️mutations/❌delete-widget/🧪️tests/🔓️removes-an-unselected-middle-row/🦀️.rs`.

## What was run

- `cargo test --manifest-path 🌎️hub/Cargo.toml -p semio-hub-procedural --lib` (×3, private target `.🧬semio/🦑️repo/⚡️cache/play-fleet/proc-describe/target`, deleted at the end).
- `semio-framework-plugin-describe describe` directly on 24 hub wasms; `bun nx run @semio-tech/{procedural,bim,energy,fem,reasoning,wfc,architect}-plugin:describe --skip-nx-cache` one at a time, each wrapped in `SLOTS=2 🚦️cargo-slot.sh`.
