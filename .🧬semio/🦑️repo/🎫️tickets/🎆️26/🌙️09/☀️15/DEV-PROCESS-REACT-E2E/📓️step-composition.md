# Process3d Step Composition

The mounted editor dispatches add, edit, and reorder operations against the persisted step timeline. Inspector edits now address the selected step’s ID, update the label or the measure’s pose and primitive dimensions, and leave repeated values and invalid dimensions out of document history. Full step updates use the existing semantic mutations for fields that changed.

Stock dimension edits publish the authored primitive and its composed B-Rep child together. The timeline stays in place. Stock and machine inspector controls accept finite numeric strings for numeric properties and keep identical edits out of document history.

The snapshot stores the edited timeline and regenerates its content-addressed flow and tool-solid references. Replacing a Process3d artifact snapshot now restores both the stock and step payloads used to reconstruct the working scene.

The neutral interaction fixture adds a cut, a drill, and an attachment, edits the first step and the drill, disables the first step, moves the attachment before the drill, and compares the reopened timeline with JSON expectations independently calculated by the [SQLite oracle](step-composition-oracle.py). The Rust test also checks the associated flow edges and child references.

The extension entry points define fabrication simulation catalogues. This change edits simulated artifacts only.

## Verification

- SQLite matched the fixture’s timeline and edited fields and independently calculated the resized timber stock volume.
- Repository diff whitespace check passed.
- Targeted strict TypeScript schema check passed with installed declaration checking skipped.
- Mutation roster check found 16 aligned schema, binary-tag, manifest, and reference-vector entries.
- The source parser measured 3,802 active lines and 257,357 bytes for the editor test closure. The pane integration closure measured 746 lines and 63,792 bytes including the STEP corpus fixture.
- Source admission passed for the initial `composition::` pane integration run. Its Cargo graph predated an engine manifest change, so it was canceled before reaching the tests. The fresh combined editor run includes the pane, inspection, workpiece and catalogue integration targets.

## B-Rep Planar Measurement

Moved incremental planar area and volume integration into the B-Rep analysis package to keep the engine under its source limit. The engine resolves the live handle and enforces owner identity, shape revision, and retained turn grants. The engine wrapper fell from 177 to 47 lines. The same analysis package owns the pure analytic curve and surface calculations. Engine wrappers validate parameters and resolve handles before delegating those calculations. The repository’s active-source parser measured 3,494 test-configured engine lines against the 4,000-line limit and 1,816 lines in the analysis test closure. Analysis regressions compare incremental area and volume to the mass-properties query and check analytic line, circle, and plane measurements.

The focused native planar area and volume regression passed on the first extraction. The one-core follow-up exited before tests with unresolved tessellation externs for the axial face preview and spline trim packages. The live framework manifest grew by 1,290 bytes during the run. Both dependencies are declared in the current tessellation and workspace manifests.

## Certified Boolean Boundaries

The B-Rep Boolean package now computes the certified orthogonal, bore and coaxial-cylinder results. The native registry continues to bind live operand identities and publish the resulting topology. After the coordinated split, the source parser counted 3,929 Boolean lines and 3,786 native-registry lines against their 4,000-line limits. The combined editor run will exercise this path as well as Process3d composition.

## Imported Stock

The importer saves an exact Semio B-Rep pack for closed STEP, OBJ, STL and GLB geometry. The composed stock child and replay/export paths consume that same stored geometry. Valid open GLB files stay visible as display-only meshes. Solid-edit commands return a typed refusal for that stock. A malformed solid cannot create substitute preview geometry.

The file picker admits model files up to 24,576 bytes. Three ordinary AP214 boxes in the existing corpus measure 15,447 bytes each. A dedicated retained import contract admits the corresponding data URL up to 32,832 bytes and its command envelope up to 33,856 bytes, including space for Unicode file names and the retained transport framing. Gesture commands retain their own 8,192-byte envelope. The encoded B-Rep may occupy up to 65,536 bytes of the separate Process3d document contract. The typed-operation outbox hands the owned `LoadDocument` effect to the reactor for its own wire encoding. Command-result chunk limits apply to the result lane.

Tests cover a shared 8 m³ OBJ cube, native STEP and STL cubes through the retained import action, text and binary reopening, export, display-only GLB, malformed geometry, and both the picker and encoded geometry limits. The STEP case pads legal source to the raw cap and sends a 255-code-unit name using multibyte characters. The native import regressions have not yet completed against this source revision, including measurement against the existing 7,500 µs job step.

Process3d uses the STEP owner's declared metric length scale before storing exact geometry. The shared geometry reader continues to use the source coordinates for its other callers. Process3d exports declare metres, so reopening a 2 m STEP cube preserves its 8 m³ volume. The independent TypeScript Part21 parser accepted the padded OpenCASCADE-produced fixture and read its millimetre declaration. The mounted Rust regression expects this 20 mm cube to reopen at 8 × 10⁻⁶ m³.
