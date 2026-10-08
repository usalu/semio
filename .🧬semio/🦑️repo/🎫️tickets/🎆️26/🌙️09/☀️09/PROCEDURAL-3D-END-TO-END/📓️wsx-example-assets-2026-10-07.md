# generation3d Example Assets, Registration, Kind Ids and Tests (2026-10-07)

Read-only audit for the AD1 widget-set rewrite (see `📓️brep-mesh-widget-set-2026-10-07.md`). No source, fixture or test was edited, no build or test was run, and no git-modifying command was used. Command lines below marked "inferred" are derived from source and were not executed.

Path abbreviations used throughout:

- `G3D` = `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any`
- `EX` = `G3D/📚️examples` (the example root; the task's `GEN/✏️editor/📚️examples` is `G3D/✏️editor/📚️examples`, which holds only the non-picker `🎬️demo-session` leaf, see section 3.9)
- `COMP` = `✏️s/🧑‍💻dev/🧩️composition/🧪️tests`

## 0. Summary

1. The `🗣️.dsl.semio` text file is the only source of truth. Every code path, test and oracle reads it. It is the only example asset that carries document data.
2. The `🎒️.pack.semio`, `📡️*.spr.semio` and `🔧️*.op.semio` files are placeholders. The pack and spr files are byte-identical across all eight examples that have them and contain no payload. The op files are a single header line plus one `edit reuse` line that matches no mutation keyword. No code reads them.
3. No tool derives the encodings. The `fixtures generate` named in the mutation-test doc comments only processes oracle entries of class `third-party-generated`, and the generation3d oracle registry has none.
4. Nine examples exist. Registration is repeated in at least eight places (five confirmed by the pipeline audit, plus the crate module list, the io round-trip list and the composition manifests). mesh-workbench is offered in the picker but has no fixture, no TS root file, no Rust test module and no entry in any example-switch or budget list.
5. Nineteen distinct neuron kind ids are used. Only three examples need renames under AD1: `🍩️sphere-cut-with-torus` (1 kind), `📦️rectangle-extrude-volume` (1 kind) and `🥽️mesh-workbench` (5 kinds). All 19 ids resolve to the brep and math extension catalogues, and all synapse port names match the operator port signatures.
6. Tests tied to examples: per-example Rust primary-asset tests (10 functions), per-example TypeScript tests (32 `it` blocks plus one mesh-workbench test), a shared TS helper, a Python `parry3d`/`scipy` oracle with 8 Gherkin scenarios, and a 1230-line Rust composition lane (8 analytic, 8 delivery, 1 scene-bridge, 1 calibration test). The lane checks 8 examples; mesh-workbench is excluded everywhere except its DSL string-check test.
7. Several stale or contradicting statements exist (section 8): an 8-vs-9 comment, a TS `de` label equal to English, a missing `🧪️tests/🧩️geometry/🦀️.rs` reference, a stale cached preview value in the sphere-cut DSL, a mislabelled slider, and a magic-number mismatch between the binary protocol file and the on-disk files.

## 1. Asset file formats

### 1.1 Per-example layout

Each example has `🖼️assets/` (authored), `🦀️.rs` (registration constants), `🟦️.ts` (registration constants for the TS side), `🧪️tests/🧩️example/` (per-example tests) and `🧫️fixtures/🧩️example/🔣️.json` (committed expected-geometry statement). The DSL lives under a subdirectory of `🖼️assets/` whose name is the example's own emoji-prefixed directory name, except for the hex column, where the subdirectory is `🍄️hexagonal-mushroom/` while the id is `hexagonal-mushroom-column`. The `.pack`, `.spr` and `.op` files sit directly in `🖼️assets/` and are named after the id.

| Example (dir) | Id | Asset files | Fixture | TS root | Rust root |
|---|---|---|---|---|---|
| `🍄️hexagonal-mushroom-column` | hexagonal-mushroom-column | pack, spr, op, `🍄️hexagonal-mushroom/🗣️.dsl.semio` | yes | yes | yes |
| `🍩️sphere-cut-with-torus` | sphere-cut-with-torus | pack, spr, op, dsl | yes | yes | yes |
| `🐚️box-shell-preview` | box-shell-preview | pack, spr, op, dsl | yes | yes | yes |
| `📐️box-fillet-preview` | box-fillet-preview | pack, spr, op, dsl | yes | yes | yes |
| `📦️rectangle-extrude-volume` | rectangle-extrude-volume | pack, spr, op, dsl | yes | yes | yes |
| `🥽️mesh-workbench` | mesh-workbench | dsl only | no | no | yes |
| `🧲️sphere-box-fuse` | sphere-box-fuse | pack, spr, op, dsl | yes | yes | yes |
| `🧹️face-sweep-extrude` | face-sweep-extrude | pack, spr, op, dsl | yes | yes | yes |
| `🪢️rectangle-wire-preview` | rectangle-wire-preview | pack, spr, op, dsl | yes | yes | yes |

Shared fixture trees: `EX/🧫️fixtures/⏱️budget-calibration/🔣️.json` (one row for all examples). Shared TS test helper: `EX/🧪️tests/🧩️geometry/🟦️.ts`. Shared Python oracle: `G3D/🧪️tests/📐️example-geometry-3d-1/🐍️.py` with `🥒️.feature`.

### 1.2 `🗣️.dsl.semio` (text, source of truth)

Header, then blocks. Confirmed by the hexagonal column file (`EX/🍄️hexagonal-mushroom-column/🖼️assets/🍄️hexagonal-mushroom/🗣️.dsl.semio`, 1503 bytes):

```
semio procedural.generation3d.dsl v1
schema="flow.host_snapshot"
camera { x=94.75581571737445 y=-97.50833134679668 zoom=1.7844325616011099 }
layout={ column-preview=x=237.4 y=-103.1 extrude=x=34.8 y=-154.1 ... }
synapses [id:TEXT wire:WIRE] {
  e1 height@number->extrusion-axis@z
  e2 radius@number->profile@radius
  e3 sides@number->profile@sides
  e4 profile@wire->extrude@wire
  e5 extrusion-axis@vectorOut->extrude@vector
  e6 extrude@solid->column-preview
}
generations [id:TEXT name:TEXT values:VAL] { }
widgets {
  input-slider id="height" label="Column Height" value=6 min=0 max=10 step=0.5
  input-slider id="radius" label="Profile Radius" value=0.5 min=0.1 max=2 step=0.05
  input-slider id="sides" label="Side Count" value=6 min=3 max=12 step=1
  neuron id="profile" neuron-kind=brep.curve.polygon preview=true input-ports=[ "radius" "sides" ] output-ports=[ ] params=[ ]
  neuron id="extrusion-axis" neuron-kind=math.vector preview=true input-ports=[ "x" "y" "z" ] output-ports=[ ] params=[ ]
  neuron id="extrude" neuron-kind=brep.solid.extrude preview=true input-ports=[ "wire" "vector" ] output-ports=[ ] params=[ ]
  output-preview id="column-preview" preview=[ ] expanded=[ ]
}
```

Widget keywords seen across the nine files: `input-slider`, `neuron`, `output-preview`. Every `generations` block is empty. `output-ports=[ ]` and `params=[ ]` are empty in all neurons. `layout` holds x/y per widget id and `camera` holds the view.

Parse and print:

- Parse: `parse_dsl` in `G3D/🚪️io/📝️text/📸️snapshot/🦀️.rs:435` delegates to `store::ArtifactDsl::parse_dsl`.
- Print: `print_dsl` in the same file at line 440. It is used by the text exporter `G3D/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs:18` and by the text importer `G3D/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs:17`.
- The header line is written by the generic `store::ArtifactDsl` implementation, which is outside this subset and was not traced further.

### 1.3 `🎒️.pack.semio` (binary whole-document container, placeholder)

- Magic `89 53 45 4d 0d 0a 1a 0a` (`\x89SEM\r\n\x1a\n`), a little-endian length `1f 00 00 00` (31), then the 31-byte kind string `procedural.generation3d.pack v1`, then zero bytes. Total 171 bytes.
- All eight pack files are byte-identical (one SHA-256 across eight files) and have no non-zero byte after the kind string. They carry no widget, synapse or layout data.
- Producer: `encode` in `G3D/🚪️io/💾️binary/📸️snapshot/🦀️.rs:12-14` calls `store::ArtifactPack::encode_pack`. The file doc comment at line 39 calls these files "exactly like every committed `🎒️.pack.semio` asset", which is consistent with the placeholder reading.
- Readers: none. The only references to the file name in the repo are doc comments.

### 1.4 `📡️<id>.spr.semio` (binary state-patch record, placeholder)

- Magic as above, length `1e 00 00 00` (30), kind `procedural.generation3d.spr v1`, zero bytes. Total 170 bytes. All eight are byte-identical with no payload.
- Intended codec: the state-patch wire codec in `G3D/🚪️io/💾️binary/🧬️mutations/🦀️.rs` (line 1 says "state-patch-representation wire codec"; `encode` at line 73 says "Encodes a `Generation3dMutation` to its binary state-patch form"). Its protocol is `G3D/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio`.
- Readers: none.

### 1.5 `🔧️<id>.op.semio` (text mutation log, placeholder)

Exact content (73 bytes, identical in all eight files):

```
semio procedural.generation3d.op v1
edit reuse started="0" actor=example
```

- Intended codec: the OpText codec in `G3D/🚪️io/📝️text/🧬️mutations/🦀️.rs`. `parse_op` (line 111) matches a line by a variant keyword followed by a space, and returns `unknown operation` otherwise. `print_op` (line 122) writes one mutation per line.
- The line `edit reuse started=... actor=...` does not match any mutation keyword. The keyword list in the generation3d tree contains `set-widget-input`, `add-widget`, `translate-selection` and similar, but no `edit`, and no keyword `reuse`. So the file would be refused by its own codec.
- The text grammar `G3D/🚪️io/📝️text/🧬️mutations/📖️mutations.grammar.semio` describes a mesh-op vocabulary (`add-vertex`, `set-face`, `transform-mesh`, `merge-solid`, `revolve-profile`) that no current mutation uses. Treat it as stale.
- Readers: none.

### 1.6 Verdict

- Source of truth: `🗣️.dsl.semio`. Everything else is either a placeholder or a stale codec target.
- For the rewrite: author the DSL by hand. Do not copy the placeholder trio into new examples unless a decision is made to create a real encoder (see section 9).
- The `.dsl.semio` writer exists (`print_dsl`), so the editor's `export-document` command (`G3D/✏️editor/🎮️commands/📤️export-document/🦀️.rs`) can regenerate a DSL from a live document if the lane wants a round trip.

## 2. Derivation tool

Short answer: none exists for these assets.

- The doc comments in roughly 30 mutation test files under `G3D/🧬️schema/🧬️mutations/*/🧪️tests/` say the `.op/.spr/.dsl/.pack/.patch` encodings "are derived from it by `fixtures generate` and are asserted by the shared codec-matrix harness". The codec-matrix harness was not located in the framework or the procedural tree. The `fixtures generate` claim is not supported by code.
- The only `fixtures generate` implementation is `FixtureScript` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/📋️orchestration/🟦️.ts` (class at line 10, `case "generate"` in `run`). It iterates oracle `testEvidence` entries with `class === "third-party-generated"` and a `generator` record, runs the recorded command with `SEMIO_FIXTURE_OUT` set, and copies the output into the content-addressed store.
- `G3D/🔮️oracles/🔣️.json` has zero `testEvidence` and zero `third-party-generated` entries. Its `_comment` says "no oracle is registered" for `Generation3dMutation`. So `fixture generate` processes nothing for generation3d.
- No `🏭️generator` directory exists under the generation3d artifact. Other artifacts (for example `➗️equation`) have one.
- Router registration: `.register("fixture", FixtureScript)` at `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts:84`, run through `runRepoScriptMain`. Inferred command line (not executed): `bun <repo>/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts fixture generate`. The same router form is used by the package scripts, for example `bun ./📜️script.ts test quick --test generation3d-example-geometry` in the composition package.
- The DSL/pack equivalence that does run is in-memory only: `dsl_round_trip_every_bundled_example` and `assert_dsl_pack_equivalence_cold` in `G3D/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:13-30`. They never compare against the committed files.

## 3. Registration and enumeration sites

### 3.1 Per-example registration constants

Each `🦀️.rs` declares `ID`, `label()`, `ICON` and `PRIMARY_TEXT`. Line numbers are identical in the eight files that follow the template:

- `EX/<dir>/🦀️.rs:6` `pub const ID: &str = "<id>";`
- `:8` `LocalizedLabel::native("<EN>", "<DE>")`
- `:10` `pub const ICON: &str = "file";`
- `:11` `pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/<dir>/🗣️.dsl.semio");`

`mesh-workbench` differs: its `🦀️.rs` is compact (ID at line 4, label at line 5, ICON at 6, PRIMARY_TEXT at 7).

Each `🟦️.ts` (eight files; none for mesh-workbench) declares `export const id`, `export const label = { en, de }` (line 3) and `export const icon = "file"`. The `de` value equals the English text in all eight files, so the TS side has no German labels.

### 3.2 Crate module list

`G3D/🦀️.rs`:

- Lines 552-554: `#[cfg(feature = "component-app-assembly")] #[path = "."] pub mod examples {`.
- Lines 555-572: nine `#[path] pub mod art_generation3d_<x>;` declarations (mesh-workbench at 555-556, box-fillet at 557-558, box-shell 559-560, face-sweep 561-562, hex 563-564, rect-extrude 565-566, rect-wire 567-568, sphere-box-fuse 569-570, sphere-cut 571-572).
- Lines 573-595: `#[cfg(test)]` test modules for eight examples. mesh-workbench is absent (it has no Rust test file).

### 3.3 Schema constants and membership

`G3D/🧬️schema/🦀️.rs`:

- Lines 125-133: nine `pub const PROCEDURAL_EXAMPLE_<X>: &str = "<id>";`. Hex is 125, rect-extrude 126, sphere-cut 127, box-fillet 128, sphere-box-fuse 129, face-sweep 130, rect-wire 131, mesh-workbench 132, box-shell 133 (picker order is in section 3.10).
- Lines 156-170: `pub fn is_generation3d_example_id` (match of all nine ids plus the alias `"demo"` at line 160, which is also generation2d's example id).

### 3.4 Text constants and lookup

`G3D/🚪️io/📝️text/📸️snapshot/🦀️.rs`:

- Lines 19-27: nine `GENERATION3D_EXAMPLE_<X>_TEXT` `include_str!` constants of the DSL files.
- Line 470: `default_snapshot()` parses the hex column text.
- Lines 473-487: `example_snapshot(example_id)` match arms (line 475 is `PROCEDURAL_EXAMPLE_HEX_COLUMN | "demo"`, lines 476-483 the other eight).
- Line 489: `example_document_json` serializes the snapshot.

### 3.5 Editor catalogue and picker

`G3D/✏️editor/🦀️.rs`:

- Line 1926: `fn examples()` in the `Generation3dPlayApp` impl delegates to the free function at 3145.
- Lines 2930-2938: `ActionArgOption` list for the `exampleId` select argument of the `setActiveExample` action (args declared from line ~2922; nine entries, labels EN/DE).
- Lines 3135-3144: doc comment for `pub fn examples()`. It says "The eight bundled ..." and "in `is_generation3d_example_id`'s order". Both statements are stale (nine entries; different order, see 4.3).
- Lines 3145-3156: `pub fn examples() -> Vec<ExampleSource>` returns `crate::examples::art_generation3d_<x>::source()` for the nine modules.
- Lines 617-624: `pub mod examples` for `✏️editor/📚️examples/🎬️demo-session` (command-replay leaf, intentionally excluded from the picker, per the doc comment at 3139-3144).

### 3.6 Viewer picker

`G3D/👁️viewer/🦀️.rs`:

- Lines 1644-1657: `fn generation3d_view_example_options()`. Its doc comment at line 1643 says "the SAME eight ids". The function lists nine ids at lines 1648-1656, with the same EN/DE labels as the editor list. Keep the three label copies in sync (examples `🦀️.rs`, editor list, viewer list).
- Test: `G3D/👁️viewer/🧪️tests/🔬️unit/🦀️.rs:70` iterates two ids (hex and rect-extrude) only.

### 3.7 Round-trip, fold and switch lists

- `G3D/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:13-30`: `dsl_round_trip_every_bundled_example` lists eight texts and omits `GENERATION3D_EXAMPLE_MESH_WORKBENCH_TEXT`. The same file at line ~49 includes mesh-workbench in a different test.
- `G3D/✏️editor/🧪️tests/🔬️fold-contract/🦀️.rs:8`: `const BUNDLED_EXAMPLES: [&str; 8]` (no mesh-workbench).
- `COMP/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️example-switch/🦀️.rs:10`: `const EXAMPLE_SWITCH_ORDER: [&str; 8]` (no mesh-workbench).

### 3.8 Fixture data that enumerates examples

- `G3D/🧫️fixtures/⏱️evaluate-budget.json`: rows reference the eight non-mesh ids.
- `G3D/🧫️fixtures/🎨️example-switch.json`: `examples` maps each of the eight ids to its authored widget ids, plus an empty-string key with `[]`. Mesh-workbench is absent.
- `G3D/🧫️fixtures/🔁️incremental-eval.json`, `🎚️slider-values.json`, `🧭️graph-keyboard-navigation.json`: hex column only.

### 3.9 Outside the subset

- `.vscode/🧩️launch.seed.jsonc:5211` and `:5232`, and the generated `.vscode/launch.json:7134` and `:7155`: `SEMIO_DEFAULT_EXAMPLE: "hexagonal-mushroom-column"`.
- `♻️mit-bestand/🧺️demonstrator/🪧️brand.ts:529`: `defaults: { exampleId: "hexagonal-mushroom-column" }`.
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs` (lines 94, 218, 261, 546, 589, 700): a learning program for the hex column only (`_ => None` for the others).
- `G3D/✏️editor/🗣️terminology`, the mesh and brep extensions and the io import/export modules reference the renamed kind ids (section 5.5). They are not example enumerations.

### 3.10 Ordering

Picker order (`examples()`, editor 3146-3154, and both option lists): hex column, mesh-workbench, rect-extrude, sphere-cut, box-fillet, sphere-box-fuse, face-sweep, rect-wire, box-shell.

Membership order (`is_generation3d_example_id`, schema 157-169): hex, "demo", rect-extrude, sphere-cut, box-fillet, sphere-box-fuse, face-sweep, rect-wire, mesh-workbench, box-shell.

Test-list order (fold-contract and example-switch): hex, rect-extrude, sphere-cut, box-fillet, sphere-box-fuse, face-sweep, rect-wire, box-shell (no mesh-workbench).

There is no single ordering source. AD7 asks for catalogue-driven ordering; until then, the picker order is the one the user sees.

### 3.11 Labels (EN and DE)

| Id | EN (native label) | DE (native label) | TS `de` |
|---|---|---|---|
| hexagonal-mushroom-column | Hexagonal Mushroom Column | Sechseckige Pilzsäule | Hexagonal Mushroom Column |
| sphere-cut-with-torus | Sphere Cut With Torus | Kugel mit Torus geschnitten | Sphere Cut With Torus |
| box-shell-preview | Box Shell Preview | Hohlkörper Vorschau | Box Shell Preview |
| box-fillet-preview | Box Fillet Preview | Kantenrundung Vorschau | Box Fillet Preview |
| rectangle-extrude-volume | Rectangle Extrude Volume | Rechteck-Extrusionsvolumen | Rectangle Extrude Volume |
| mesh-workbench | Mesh Workbench | Netzwerkstatt | (no TS file) |
| sphere-box-fuse | Sphere Box Fuse | Kugel und Quader vereinen | Sphere Box Fuse |
| face-sweep-extrude | Face Sweep Extrude | Fläche extrudieren | Face Sweep Extrude |
| rectangle-wire-preview | Rectangle Wire Preview | Rechteck-Draht Vorschau | Rectangle Wire Preview |

The same EN/DE pair is repeated in the example `🦀️.rs`, the editor option list and the viewer option list. The TS German column is untranslated.

## 4. Example inventory

### 4.1 Asset and structure summary

| Id | Sliders (widget id: value, range) | Neurons | Synapses | output-preview |
|---|---|---|---|---|
| hexagonal-mushroom-column | height 6 (0-10), radius 0.5 (0.1-2), sides 6 (3-12) | profile, extrusion-axis, extrude | 6 | column-preview |
| sphere-cut-with-torus | slider_2 2.2 (0-10), labelled "Torus Major Radius" | brep_prim3d_sphere_3, brep_prim3d_torus_4, brep_bool_cut_5, brep_measure_volume_2 | 5 (e100, e101, e102, e103, e108) | preview_3 (cached value, see 8.6) |
| box-shell-preview | size 2 (0.5-5), thickness 0.2 (0.05-0.8) | box, shell | 5 | none |
| box-fillet-preview | size 2 (0.5-5), radius 0.15 (0.01-0.5) | box, fillet | 6 | preview |
| rectangle-extrude-volume | width 2, height 2, distance 3 (all 0.1-10) | rect, vector, extrude, volume | 6 | none |
| mesh-workbench | inset 0.1 (0.01-0.3), extrusion 0.5 (0.01-2) | box, inset, extrude, analysis, preview | 6 | none (neuron `preview` is brep.mesh.toBrep) |
| sphere-box-fuse | radius 1.2 (0.2-3), size 1.5 (0.5-4) | sphere, box, fuse | 7 | preview |
| face-sweep-extrude | width 2, height 1.5, distance 4 | rect, face, vector, extrude | 6 | none |
| rectangle-wire-preview | width 2, height 1.5 | rect | 2 | none |

Preview flags (`preview=true` on neurons): hex profile, extrusion-axis, extrude; sphere-cut cut and measure; shell; fillet; rect-extrude extrude; mesh extrude; fuse; face-sweep extrude; rect-wire rect.

### 4.2 Per-example expected-geometry and budget rows

Taken from `🧫️fixtures/🧩️example/🔣️.json` of each example (all `kernelStatus` green).

| Id | Preview node@channel (kind) | Expected volume | closed | Meshes (roles) | minTriangles / minEdgeSegments | maxRoundTrips | Budget µs (eval / tess / preview) | Kernel volume node |
|---|---|---|---|---|---|---|---|---|
| hexagonal-mushroom-column | extrude@solid (solid) | 3.897114 | yes | 3 (solid 1, vector 1, wire 1) | 20 / 18 | 1 | 20000 / 20000 / 20000 | none |
| sphere-cut-with-torus | brep_bool_cut_5@solid | 37.841143 | yes | 1 (solid) | 1114 / 45 | 1 | 1545000 / 3171000 / 279000 | brep_measure_volume_2 |
| box-shell-preview | shell@solid | 3.904 | yes | 1 (solid) | 24 / 24 | 1 | 20000 / 20000 / 20000 | none |
| box-fillet-preview | fillet@solid | 7.888635 | yes | 1 (solid) | 92 / 72 | 2 | 20000 / 408000 / 73000 | none |
| rectangle-extrude-volume | extrude@solid | 12.0 | yes | 1 (solid) | 12 / 12 | 1 | 20000 / 20000 / 20000 | volume |
| sphere-box-fuse | fuse@solid | 9.708451 | yes | 1 (solid) | 398 / 25 | 2 | 87000 / 845000 / 44000 | none |
| face-sweep-extrude | extrude@solid | 12.0 | yes | 1 (solid) | 12 / 12 | 1 | 20000 / 20000 / 20000 | none |
| rectangle-wire-preview | rect@wire (wire) | none (edge perimeter) | no | 1 (wire) | 0 / 4 | 1 | 20000 / 20000 / 20000 | none |

The rows with 20000 µs on every phase look like a shared floor for trivial examples (inferred from the values, not verified against the Rust lane). The large values sit on the boolean, fillet and mesh-heavy phases.

### 4.3 Calibration row

`EX/🧫️fixtures/⏱️budget-calibration/🔣️.json`:

```
schema: s.procedural.generation3d.example-budget-calibration/v1
rounds: 3584
checksum: 0x24d974393bfa7c15
referenceMicros: 101180
maximumLoadFactor: 16.0
```

Ceilings are scaled by a measured load factor clamped to `[1, maximumLoadFactor]` (Rust composition test, section 6.3).

## 5. Neuron kind ids per example

### 5.1 Decoded from the DSL

| Example | `neuron-kind` values (widget id) |
|---|---|
| hexagonal-mushroom-column | brep.curve.polygon (profile); math.vector (extrusion-axis); brep.solid.extrude (extrude) |
| sphere-cut-with-torus | brep.prim3d.sphere (brep_prim3d_sphere_3); brep.prim3d.torus (brep_prim3d_torus_4); brep.bool.cut (brep_bool_cut_5); brep.measure.volume (brep_measure_volume_2, line 23) |
| box-shell-preview | brep.prim3d.box (box); brep.solid.shell (shell) |
| box-fillet-preview | brep.prim3d.box (box); brep.solid.fillet (fillet) |
| rectangle-extrude-volume | brep.curve.rectangle (rect); math.vector (vector); brep.solid.extrude (extrude); brep.measure.volume (volume, line 26) |
| mesh-workbench | brep.mesh.box (box, line 20); brep.mesh.inset (inset, line 21); brep.mesh.extrude (extrude, line 22); brep.mesh.analyze (analysis, line 23); brep.mesh.toBrep (preview, line 24) |
| sphere-box-fuse | brep.prim3d.sphere (sphere); brep.prim3d.box (box); brep.bool.fuse (fuse) |
| face-sweep-extrude | brep.curve.rectangle (rect); brep.surf.planarFaceWire (face); math.vector (vector); brep.sweep.extrude (extrude) |
| rectangle-wire-preview | brep.curve.rectangle (rect) |

Distinct ids: 19 (brep.curve.polygon, brep.curve.rectangle, brep.solid.extrude, brep.solid.shell, brep.solid.fillet, brep.prim3d.box, brep.prim3d.sphere, brep.prim3d.torus, brep.bool.cut, brep.bool.fuse, brep.measure.volume, brep.surf.planarFaceWire, brep.sweep.extrude, brep.mesh.box, brep.mesh.inset, brep.mesh.extrude, brep.mesh.analyze, brep.mesh.toBrep, math.vector).

### 5.2 Verified against the operator catalogues

Parsed from the `manifestJson` of `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🔣️.json` (142 operators) and `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/🔣️.json` (25 operators). All ids exist. Port names used by the synapses match the operator inputs:

- brep.curve.polygon: in `radius`, `sides`; out `wire`.
- brep.solid.extrude: in `wire` (geometry), `vector` (vector/point); out `solid`.
- brep.prim3d.sphere: in `radius`; out `solid`. brep.prim3d.torus: in `major`, `minor`. brep.prim3d.box: in `width`, `depth`, `height`.
- brep.bool.cut and brep.bool.fuse: in `a`, `b` (geometry); out `solid`.
- brep.measure.volume: in `geometry`; out `volume`.
- brep.solid.shell: in `geometry`, `thickness`, `openFaces` (list, unwired in the example). brep.solid.fillet: in `geometry`, `radius`.
- brep.curve.rectangle: in `width`, `height`; out `wire`. brep.surf.planarFaceWire: in `wire`; out `face`. brep.sweep.extrude: in `face`, `vector`; out `solid`.
- brep.mesh.box: in `width`, `height`, `depth`; out `meshOut`. brep.mesh.inset: in `mesh`, `faces` (text, unwired), `amount`. brep.mesh.extrude: in `mesh`, `faces` (text, unwired), `distance`. brep.mesh.analyze: in `mesh`; out a list of 12 measurement ports. brep.mesh.toBrep: in `mesh`, `tolerance`; out `geometry`.
- math.vector: in `vector`, `x`, `y`, `z`; out `vectorOut`, `xOut`, `yOut`, `zOut`, `errors`.

Unwired inputs (`openFaces`, `faces`, torus `major`/`minor`, mesh `box` inputs) fall back to channel defaults. The Python oracle reads those defaults from the brep catalogue.

### 5.3 Other notes on kinds

- `brep.mesh.analyze` and `brep.mesh.toBrep` are ambiguous under AD1 (see 5.4).
- The kind ids are declared in the catalogue, not in the example. Renaming a kind in the example alone breaks `channel_defaults()` in the Python oracle and the composition test.

### 5.4 Rename map under AD1

| Old id | AD1 rule | New id | Examples affected |
|---|---|---|---|
| brep.measure.volume | brep.measure.* becomes analysis.* | analysis.volume | sphere-cut (line 23), rect-extrude (line 26) |
| brep.mesh.box | brep.mesh.* becomes mesh.* | mesh.box | mesh-workbench (line 20) |
| brep.mesh.inset | same | mesh.inset | mesh-workbench (line 21) |
| brep.mesh.extrude | same | mesh.extrude | mesh-workbench (line 22) |
| brep.mesh.analyze | same, or analysis.* under AD4 | mesh.analyze (or analysis.mesh) | mesh-workbench (line 23) |
| brep.mesh.toBrep | same, or stays brep.* as a converter | mesh.toBrep (or brep.mesh.toBrep) | mesh-workbench (line 24) |
| brep.curve.*, brep.prim3d.*, brep.bool.*, brep.solid.*, brep.surf.*, brep.sweep.* | B-Rep groups stay brep.<group>.<name> | unchanged | none |
| math.vector | math.* core | unchanged | none |

Decision needed for `brep.mesh.analyze` and `brep.mesh.toBrep` (section 9).

### 5.5 Files that must change for the rename

Example-specific (the only files the example rewrite touches):

- `EX/🍩️sphere-cut-with-torus/🖼️assets/🍩️sphere-cut-with-torus/🗣️.dsl.semio:23`
- `EX/🍩️sphere-cut-with-torus/🧫️fixtures/🧩️example/🔣️.json:8` (`opChain`)
- `EX/📦️rectangle-extrude-volume/🖼️assets/📦️rectangle-extrude-volume/🗣️.dsl.semio:26`
- `EX/📦️rectangle-extrude-volume/🧫️fixtures/🧩️example/🔣️.json:8` (`opChain`)
- `EX/🥽️mesh-workbench/🖼️assets/🥽️mesh-workbench/🗣️.dsl.semio:20-24`
- `EX/🥽️mesh-workbench/🧪️tests/🧩️example/🟦️.ts:5,7,8` (string-contains checks)
- `G3D/🧪️tests/📐️example-geometry-3d-1/🥒️.feature:101,103,164,166` (chain text and kernel-volume wording)
- `COMP/🧩️generation3d-example-geometry/🦀️.rs:599` (assertion message only)

Shared (outside the examples, required by the rename of the kind ids):

- Operator registrations and catalogues: `brep` and `mesh` extension catalogues, `🥽️mesh` tests and the `math` catalogue.
- Editor, inspection, catalogue and terminology modules: `G3D/✏️editor/🎮️commands/*`, `G3D/✏️editor/🎯️selection/🦀️.rs`, `G3D/✏️editor/📌️panels/🔍️inspection/🦀️.rs`, `G3D/✏️editor/📌️panels/🛍️catalogue/🧫️fixtures/🗣️.json`, `G3D/✏️editor/🗣️terminology/🦀️.rs`.
- Io: `G3D/🚪️io/📝️text/📸️snapshot/🦀️.rs`, the obj, gltf and ply importers, and the io test fixtures.
- Mutation schema: `G3D/🧬️schema/🧬️mutations/🦀️.rs` and `G3D/🧬️schema/🧬️mutations/🧪️tests/🧪️gesture-leaves/🦀️.rs`.
- Composition: `COMP/🧊️generation3d/.../✏️editor/🎮️commands/🎚️set-widget-input/🧪️tests/🔬️unit/🦀️.rs`, `COMP/🧊️generation3d/.../✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs`, `COMP/🧊️generation3d/.../✏️editor/🧪️tests/🔬️unit/🦀️.rs`.

A `git grep` for `brep\.mesh\.|brep\.measure\.` over the generation3d and composition trees returns 34 files. The full list is what the rename must cover.

## 6. Tests tied to examples

### 6.1 Per-example Rust primary-asset tests (`EX/<dir>/🧪️tests/🧩️example/🦀️.rs`)

Eight files, each `#[test] fn primary_asset_is_nonempty()` asserting the DSL text is longer than 8 bytes. Mesh-workbench has no Rust test file. The box-fillet file adds two laws:

- `inference_determinism_law`: `Generation3dInference::infer` on the parsed DSL is deterministic.
- `inference_of_the_example_is_a_non_trivial_acyclic_topology`: `topology.node_count` equals the widget count, `edge_count` equals the synapse count, `cycle_free` is true, and `topo_order.len()` equals the widget count.

These files are compiled by the `#[cfg(test)]` modules at `G3D/🦀️.rs:573-595`.

### 6.2 Per-example TypeScript tests (`EX/<dir>/🧪️tests/🧩️example/🟦️.ts`)

Eight files, four `it` blocks each, using the shared helper:

1. `ships primary asset`: DSL length is greater than 8.
2. `commits a well-formed expected-geometry fixture`: `assertFixtureContract`, which checks schema, example id, opChain non-empty, preview kind is `solid` or `wire`, bbox ordering, volume tolerance, kernel status, delivery contract and budget contract (including the calibration row).
3. `declares exactly the op chain its dsl wires`: every fixture opChain kind occurs in the DSL, and every DSL `neuron-kind` occurs in the fixture opChain.
4. `recomputes the committed ... from the dsl sliders`: an independent closed form (per example) against the fixture volume and bounding box.
   - hex: regular-polygon area times height; bbox and inradius checks.
   - sphere-cut: ball minus torus, Simpson quadrature with the torus radii hard-coded to 2.0 and 0.5; kernel volume node is `brep_measure_volume_2`.
   - box-shell: size cubed minus inner cube.
   - box-fillet: Minkowski rounded-box formula.
   - rect-extrude: width times height times distance; kernel volume node `volume`.
   - sphere-box-fuse: union by Simpson quadrature of the clipped octant.
   - face-sweep: prism volume.
   - rect-wire: perimeter and open-wire check.

Mesh-workbench's TS test (`EX/🥽️mesh-workbench/🧪️tests/🧩️example/🟦️.ts`) is a single `test` that reads the DSL and checks the five `brep.mesh.*` kinds, four wires, and the `preview=true` and `preview=false` flags on two neurons. It does not evaluate anything.

### 6.3 Shared TypeScript helper (`EX/🧪️tests/🧩️geometry/🟦️.ts`, 330 lines)

Exports:

- `EXAMPLE_GEOMETRY_FIXTURE_SCHEMA = "s.procedural.generation3d.example-geometry/v1"`.
- `EXAMPLE_ASSETS_DIR = "🖼️assets"`, `EXAMPLE_FIXTURE_PATH = 🧫️fixtures/🧩️example/🔣️.json`.
- `EXAMPLE_GEOMETRY_KERNEL_STATUSES = ["green"]`.
- `EXAMPLE_PREVIEW_MESH_ROLES = ["solid", "wire", "point", "vector"]`.
- `EXAMPLE_DELIVERY_ROUND_TRIP_CEILING = 2` (one round trip is a whole `flowEvalTick`).
- `EXAMPLE_BUDGET_INTERACTIVE_CEILING_MICROS = 2_000_000` (evaluate and preview tessellation).
- `EXAMPLE_BUDGET_FIDELITY_CEILING_MICROS = 8_000_000` (full tessellation).
- `loadExample(here, assetDirName, assetName)`, `assertFixtureContract`, `assertDeliveryContract`, `assertBudgetContract`, `assertBudgetCalibrationContract`, `assertOpChainDeclared`, `sliders`, `assertExpectedVolume`, `assertExpectedPerimeter`, `assertExpectedBoundingBox`, `simpson`.

The file header says the Rust half lives at `EX/🧪️tests/🧩️geometry/🦀️.rs`. That file does not exist (see 8.3).

### 6.4 Rust composition lane (`COMP/🧩️generation3d-example-geometry/🦀️.rs`, 1230 lines)

Crate target: `✏️s/🧑‍💻dev/🧩️composition/📦️packages/🦀️rust/Cargo.toml:63-64` (`name = "generation3d-example-geometry"`). Nx target `test-generation3d-geometry` in that package's `📋️project.json` (line 73), command `bun ./📜️script.ts test quick --test generation3d-example-geometry` (cwd `✏️s/🧑‍💻dev/🧩️composition/📦️packages/🦀️rust`).

Tests (18 functions):

- `rectangle_wire_preview_evaluates_to_an_open_wire`, `rectangle_extrude_volume_evaluates_to_the_analytic_box`, `face_sweep_extrude_evaluates_to_a_closed_solid`, `hexagonal_mushroom_column_evaluates_to_the_analytic_prism`, `box_shell_preview_evaluates_to_a_hollow_solid`, `box_fillet_preview_evaluates_to_a_rounded_solid`, `sphere_box_fuse_evaluates_to_the_union_volume`, `sphere_cut_with_torus_evaluates_to_the_difference_volume` (lines 649-684). Each calls `assert_example(dsl, fixture_json)`, which:
  - checks the fixture schema and the budget contract;
  - checks each opChain kind occurs in the DSL;
  - evaluates the DSL through `FlowHost` with the brep and math extensions installed (`install_flow_extension`, `FlowExtensionSpec`);
  - tessellates and checks triangle count, closed surface, zero orientation defects on closed surfaces, tessellated volume, `parry3d` volume, and the kernel `brep.measure.volume` value within tolerance;
  - checks the bounding box.
- `hexagonal_mushroom_column_preview_payload_matches_the_scene_bridge_fixture` (line 731): preview payload against `G3D`-external `🧰️framework/.../♾️infinite/🌍️world/🧫️fixtures/🌉️scene-bridge/🔣️.json`.
- `delivery_*` (lines 1187-1223), one per example: `assert_delivery` runs the preview LOD path, checks round trips against `maxRoundTrips`, chunks, mesh count and per-role counts, payload bounds, a selection round trip, and phase budgets with best-of-N timing scaled by the calibration factor.
- `the_budget_calibration_is_deterministic_and_never_narrows_a_ceiling` (line 315): runs the fixed workload twice, checks the checksum, and checks the load factor is in `[1, maximumLoadFactor]`.

Caveat for the rewrite: `assert_example` and `run_example` depend on `install_flow_extension`, `FlowHost` and the `flow-extension-brep` path, which the plan removes (AD2, AD5). This lane needs to be rebuilt on the geometry inference service.

### 6.5 Composition manifest (`COMP/🧫️fixtures/🔣️.json`)

A manifest entry at lines 38-63 lists the lane's `laws` (the test names above), a `sha256` of `🧪️tests/🧩️generation3d-example-geometry/🦀️.rs`, and a `previousPath` pointing at the old location `G3D/📚️examples/🧪️tests/🧩️geometry/🦀️.rs`. Editing the Rust test file requires updating this entry. The verifier that checks the sha256 was not located.

### 6.6 Python oracle (`G3D/🧪️tests/📐️example-geometry-3d-1/`)

- `🐍️.py` (277 lines): discovers examples with `EXAMPLES.glob("*/🧫️fixtures/🧩️example/🔣️.json")`, requires exactly 8 (line ~212 fails otherwise), reads sliders and `neuron-kind` values from the DSL, takes channel defaults from the brep catalogue (`BREP_DESCRIPTOR`, line 47), and computes expected volume, perimeter and bounding box per example id through `expected()` (lines 141-168, one `if example == ...` branch per example). It uses numpy and scipy for the two solids without a closed form. The test function is `test_committed_example_geometry_fixtures_match_the_oracle` (line 225).
- `🥒️.feature`: eight scenarios tagged `@id-<example>` (lines 86-160), each with a `When the flow host evaluates its "<kind> -> ... " chain` step. Mesh-workbench is absent.
- Run (inferred from the docstring, not executed): from the `📐️example-geometry-3d-1` directory `.venv/bin/python3 "🐍️.py"`, or `uv run pytest` from the repo root.

### 6.7 Budget and calibration fixtures

- `EX/🧫️fixtures/⏱️budget-calibration/🔣️.json` (section 4.3).
- Per-example `budget` rows in each example fixture (section 4.2).
- `G3D/🧫️fixtures/⏱️evaluate-budget.json`: a separate `evaluate` capability budget law (`rows` with envelope and phase fields); it references the eight non-mesh example ids.

### 6.8 Other tests that name examples

- `COMP/🧊️generation3d/.../✏️editor/🧪️tests/🔬️fold-contract/🦀️.rs:8` (BUNDLED_EXAMPLES, 8 entries).
- `COMP/🧊️generation3d/.../✏️editor/🧪️tests/🔬️example-switch/🦀️.rs:10` (EXAMPLE_SWITCH_ORDER, 8 entries) and the fixture `G3D/🧫️fixtures/🎨️example-switch.json`.
- `COMP/🔁️generation3d-incremental-eval/🦀️.rs:103,153`: hex column fixture and DSL.
- `G3D/👁️viewer/🧪️tests/🔬️unit/🦀️.rs:70` (two ids).
- `G3D/✏️editor/🎮️commands/🎨️set-active-example`-related tests (`COMP/🧊️generation3d/.../👁️viewer/🎮️commands/🎨️set-active-example/🧪️tests/🔬️unit/🦀️.rs`).

## 7. Recipes

### 7.1 Add a new example (`<slug>`, `<emoji>`, `<X>` = upper snake of slug)

Author the example, then register it. Do not hand-compute fixture numbers; the Rust lane and the Python oracle are the two independent checks.

1. Create `EX/<emoji>-<slug>/`.
2. `🖼️assets/<emoji>-<slug>/🗣️.dsl.semio`: write the DSL by hand following section 1.2. Requirements: unique widget ids; every `neuron-kind` from the brep or math catalogue (or the renamed `analysis.*`/`mesh.*`); every synapse port name matching section 5.2; one `layout` entry per widget; `generations` empty; `output-preview` only where a preview is wanted; `preview=true` on the neuron(s) that feed the preview.
3. Do not add `🎒️.pack.semio`, `📡️*.spr.semio` or `🔧️*.op.semio` unless a real encoder is decided (section 9). The existing placeholders carry no data.
4. `🦀️.rs`: copy `EX/🍄️hexagonal-mushroom-column/🦀️.rs`, change ID, label EN/DE and PRIMARY_TEXT path. Keep ICON `"file"`.
5. `🟦️.ts`: copy the hex file; set `id`, `label` with a real German string (not the English), `icon`.
6. `🧪️tests/🧩️example/🦀️.rs`: copy the hex test (`primary_asset_is_nonempty`), change the include path.
7. `🧪️tests/🧩️example/🟦️.ts`: copy the hex file. Change the example id and asset dir name in `loadExample`, and write the closed-form expected volume or bbox from the sliders in the fourth `it`.
8. `🧫️fixtures/🧩️example/🔣️.json`: schema, example id, opChain (must equal the DSL kinds), preview node/channel/kind, tessellationTolerance, expect (volume and tolerance, closed, bbox, kernel node if any), delivery (meshes, meshRoles, minTriangles, minEdgeSegments, bbox, maxRoundTrips at most 2, maxChunks 1), budget (ceilings within the TS and Rust ceilings), kernelStatus `green`. Values come from running the lane, then checking them against the closed form.
9. Register (every site in section 3):
   - `G3D/🧬️schema/🦀️.rs`: add `PROCEDURAL_EXAMPLE_<X>` after line 133 and add it to `is_generation3d_example_id` (lines 157-169).
   - `G3D/🚪️io/📝️text/📸️snapshot/🦀️.rs`: add `GENERATION3D_EXAMPLE_<X>_TEXT` after line 27 and one arm in `example_snapshot` (lines 475-483).
   - `G3D/🦀️.rs`: add `#[path] pub mod art_generation3d_<slug>;` in `pub mod examples` (lines 555-572) and the `#[cfg(test)]` test module (lines 573-595).
   - `G3D/✏️editor/🦀️.rs`: add an `ActionArgOption` at lines 2930-2938 and `crate::examples::art_generation3d_<slug>::source()` in `examples()` (lines 3146-3154). Update the doc comment at 3135-3144.
   - `G3D/👁️viewer/🦀️.rs`: add an `ActionArgOption` at lines 1648-1656 and update the "eight" comment at 1643.
   - `G3D/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs`: add to the list at lines 14-22.
   - `G3D/✏️editor/🧪️tests/🔬️fold-contract/🦀️.rs:8`: extend the array and its length.
   - `COMP/.../✏️editor/🧪️tests/🔬️example-switch/🦀️.rs:10`: extend `EXAMPLE_SWITCH_ORDER` and its length. Add the entry to `G3D/🧫️fixtures/🎨️example-switch.json` (`examples`, with the authored widget ids).
   - `G3D/🧫️fixtures/⏱️evaluate-budget.json`: add the id where the example is listed.
   - `COMP/🧩️generation3d-example-geometry/🦀️.rs`: add one analytic test and one `delivery_<slug>` test (following the existing pattern at lines 649-684 and 1187-1223). Update the manifest entry in `COMP/🧫️fixtures/🔣️.json` (laws list and sha256).
   - Python oracle: add an `if example == "<slug>"` branch in `expected()` (lines 141-168), change the expected count at line ~212 from 8 to 9, and add a `@id-<slug>` scenario to `🥒️.feature`.
   - Labels: keep EN/DE identical in the example `🦀️.rs`, the editor option list and the viewer option list.
10. Verify (inferred commands, not executed):
    - Rust: `bun ./📜️script.ts test quick --test generation3d-example-geometry` in `✏️s/🧑‍💻dev/🧩️composition/📦️packages/🦀️rust` (or the nx target `test-generation3d-geometry`).
    - TypeScript: `bun test "<example>/🧪️tests/🧩️example/🟦️.ts"` and the geometry helper.
    - Python: `uv run pytest` from the repo root, or `.venv/bin/python3 "🐍️.py"` in the `📐️example-geometry-3d-1` directory.
    - The fixture manifest: the sha256 check in `COMP/🧫️fixtures/🔣️.json` must pass.

### 7.2 Rename kind ids in an existing example

1. Settle the mapping in section 5.4 (decisions in section 9).
2. Change the DSL `neuron-kind=` values at the lines listed in 5.5. Do not change widget ids unless needed: `brep_measure_volume_2` and `kernelVolumeNode` depend on the widget id, not the kind id.
3. Change the fixture `opChain` entries (sphere-cut line 8, rect-extrude line 8). The TS op-chain test will then pass only if the DSL and fixture agree.
4. Change the TS string checks in mesh-workbench's test (lines 5, 7, 8).
5. Change the Gherkin chain text in `🥒️.feature` lines 101, 103, 164 and 166.
6. Change the Rust composition assertion message at line 599 (cosmetic).
7. Rename the operators in the brep, mesh and analysis extension catalogues and registrations. The Python oracle's `channel_defaults()` reads these ids, so it fails until the catalogue matches.
8. Update the other files in 5.5 (editor, inspection, terminology, io, mutation schema, composition tests). These are not example files but are needed for the rename to compile and pass.
9. Re-run the three verification commands from 7.1.

## 8. Discrepancies and risks found

1. Placeholder encodings (section 1.3 to 1.5). The pack and spr files are identical zero-payload stubs in all eight examples. The op files are header-only. None are read by any code. Every "derived by `fixtures generate`" doc comment is unsupported.
2. Missing reader for codec-matrix (section 2). The doc comments reference a shared "codec-matrix harness" that was not located.
3. Round-trip list omits mesh-workbench. `dsl_round_trip_every_bundled_example` (io unit test, line 13) has eight texts.
4. Hard-coded eight-entry lists: `fold-contract` `BUNDLED_EXAMPLES` (line 8), `example-switch` `EXAMPLE_SWITCH_ORDER` (line 10), the Python oracle count (line ~212), and the example-switch and evaluate-budget fixtures. Mesh-workbench is never checked by example-switch, budgets, delivery or geometry.
5. Stale comments: `G3D/✏️editor/🦀️.rs:3135-3144` ("eight bundled", "in `is_generation3d_example_id`'s order"); `G3D/👁️viewer/🦀️.rs:1643` ("the SAME eight ids" for a nine-entry list). The picker order also differs from `is_generation3d_example_id`'s order.
6. TS `de` labels equal the English text in all eight TS files (section 3.11). Native German labels exist only in Rust.
7. Missing reference: the TS helper header and the Python oracle docstring point at `EX/🧪️tests/🧩️geometry/🦀️.rs`, which does not exist. The file moved to the composition lane (manifest `previousPath`, section 6.5).
8. Sphere-cut DSL cache: the `output-preview preview_3` block stores `decimal=37.73359174067349`. The committed fixture volume is 37.841142613316. The cached value does not match the committed expectation, so it is stale or from a different state. The preview value block is a stored result inside the source file.
9. Sphere-cut slider label: `slider_2` is labelled "Torus Major Radius", but edge `e100` drives the sphere radius. The torus `major` and `minor` inputs are unwired, so the TS test hard-codes 2.0 and 0.5 (`🍩️…/🧪️tests/🧩️example/🟦️.ts`, sphere-cut block).
10. Asset directory names differ from ids in one case: hex uses `🍄️hexagonal-mushroom/` for the DSL subdirectory while the id and other file names use `hexagonal-mushroom-column`.
11. Binary protocol magic mismatch: `G3D/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio` declares `magic 0x8953f83f7d340d0a`. The on-disk files start with `89 53 45 4d 0d 0a 1a 0a`, which differs from byte 2 onward. Either the protocol file or the placeholder files are stale.
12. Mutation text grammar is stale (section 1.5): `📖️mutations.grammar.semio` describes a mesh-op vocabulary, not the current mutations.
13. Composition lane depends on the flow extension path (`install_flow_extension`, `FlowHost`), which AD2 and AD5 remove.
14. Composition manifest pins a sha256 of the Rust test file (section 6.5). Edits to that file need a manifest update.
15. Mesh-workbench is in the picker (`examples()`, the option lists and the schema membership) but has no fixture, no TS root file, no Rust test module, no oracle scenario and no switch-list entry. It is effectively untested beyond a string check.
16. Demo alias: `is_generation3d_example_id` and `example_snapshot` accept `"demo"` as an alias of the hex column. The same string is also a generation2d example id.
17. Default example is hard-coded in launch seed, generated launch and the mit-bestand brand (section 3.9). Renaming the hex id would break those.

## 9. Decisions the lane must make

1. `brep.mesh.analyze` and `brep.mesh.toBrep` under AD1: `mesh.analyze` and `mesh.toBrep`, or `analysis.*` for the analyze kind (AD4) and `brep.mesh.toBrep` for the converter.
2. Derived encodings: keep the placeholders (and document them as such), delete them, or implement a real encoder. If a real encoder is chosen, it must be a Rust function that writes the three files and a test that reads them back, and the fixture command needs a generator record. Do not copy the placeholders into new examples until this is decided.
3. Mesh-workbench coverage: add a fixture, a TS root file, a Rust test module and a switch-list entry, or remove it from the picker.
4. The sphere-cut cached preview value: regenerate it or drop the `output-preview` block.
5. The `Torus Major Radius` label versus the sphere radius wiring, and whether to wire the torus `major`/`minor` inputs explicitly.
6. German labels for the TS side (`de` values), one source of truth for EN/DE labels (AD7 catalogue).
7. Single ordering source for the picker, membership and test lists.
8. Whether widget ids such as `brep_measure_volume_2` are renamed along with the kind ids.
