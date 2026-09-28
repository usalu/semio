# Lossless Mask Link/Unlink Requirements

The initial Raster model and shared RasterStackTransform exposed translation, rotation and separate X/Y scale. The compositor already carries six-coefficient affine matrices internally. Linking uses the layer matrix; unlinking uses the parent frame. The common ancestor transform is applied afterward. Inspector initially had no link/unlink action.

A Boolean-only toggle would move the mask. Re-expressing the placement with the existing five fields is also insufficient: a layer scaled `(2,1)` composed with a mask rotated 45° yields columns `(√2,1/√2)` and `(-√2,1/√2)`. Their dot product is `-1.5`; rotation with diagonal scale always has orthogonal columns. The composed placement therefore requires shear.

The implementation must preserve the full affine map through schema, Rust/TypeScript parsers and models, mutation wire/inverse, editable output/load, Raster stack preparation, both renderers and pointer mapping. Existing `CompositeAffine` multiplication/inversion provides the internal six-coefficient representation; do not add a competing matrix library. The persisted representation still needs a deliberate schema choice: a complete matrix or a mathematically complete decomposed form. A fallback to the old five fields or a silent approximate decomposition would not satisfy this requirement.

Test the same neutral matrices in Rust and TypeScript, with Three.js Matrix3 as an independent oracle. Required cases include nested nonuniform scale plus rotation, reflections, translation, near-singular refusal consistent with the compositor, and toggling twice without displacement. Native tests must assert unchanged composite pixels, exact semantic undo and editable round-trip. Mounted tests must verify paint/selection pointer mapping after the frame change; live acceptance remains separate.

This investigation does not implement or claim completion of mask linking. The active export/archive/selection native run remains the immediate verification priority.

## Shared Frame Conversion Verified

Added `pixels/compositing/frames` (emoji paths) with a neutral JSON schema, five placement vectors and unsafe geometry cases. Rust and TypeScript `reframe` reuse the compositor matrix operations and enforce finite invertibility for inputs and output. The vectors cover nonuniform scale with rotation producing shear, a nested rotated frame into a sheared parent, reflections, and fractional translation. Three.js Matrix3 independently verifies the same output; world points and reverse conversion are checked. No runtime dependency was added.

Existing Nx targets and script now include this domain. Red run: 120 existing TypeScript tests passed and the new suite failed for its missing module. Green run: strict source type check plus **126 TypeScript tests passed**, and **36 native pixel tests passed, zero skipped**. Handles 25002 (red), 69797 (green TS), 99330 (native) are terminal. Generated logs: mask-frames-ts-red-1.log, mask-frames-ts-green-1.log, mask-frames-native-1.log.

This establishes exact matrix coordinate-frame conversion only. Persisted Raster transform representation, link/unlink commands, undo/archive/render integration and mounted/live acceptance remain open.

## Persisted Representation Decision and Control Conversion

Persist complete affine matrices for exact link/unlink frame changes; do not round-trip placement through decomposed controls when merely changing mask linkage. The editor's rotation/scale/shear controls require an explicit lossless-capable decomposition, but those controls are a presentation and edit interface rather than the authoritative stored transform. This replaces the earlier unresolved matrix-versus-components choice. At this checkpoint the production Raster model had not yet been changed; the integration below supersedes that status.

Added the Affine Transform Controls schema and six shared vectors: identity, horizontal shear, rotated shear, vertical reflection, horizontal reflection and double reflection. Rust/TypeScript compose as T × R × H × S, and canonical decomposition puts reflection in signed scaleY with positive scaleX. Finite/invertible checks reuse the compositor. Three.js Matrix3 independently composes every neutral case. Additional tests cover every frame-change matrix, missing TypeScript fields, singular geometry and nonfinite components.

Red: missing compose export, 120 preexisting TS tests passed and new module failed (session 33606 terminal exit 1). Green: **134 TS tests passed**, strict source type check passed (51941 terminal exit 0), and **38 native pixel tests passed, zero skipped** (4103 terminal exit 0). Logs are mask-controls-ts-red-1.log, mask-controls-ts-green-1.log and mask-controls-native-1.log under generated.

Integration inventory: RasterTransform currently lives at the artifact root Rust module; schema leaves include JSON/TypeScript/GraphQL/Proto. Retained mutation copies and byte accounting explicitly list the five current fields. Shared stack preparation has Rust/TypeScript RasterStackTransform; native surface paint TransformJson and compositing bridge consume it, as does Paint2dHost editing pointer math. All of these plus fixtures, masks, semantic edits and Inspector controls must move together. No matrix migration or compatibility layer is to remain.

The stack mask preparation was re-read while tracing integration: an unlinked mask is parent-relative, not absolute world-relative. Its placement is `parent × mask`, while a linked mask is `parent × layer × mask`. Therefore linkage changes use reframe with the layer matrix and identity within the common parent; nested ancestors cancel algebraically. The initial wording in this note was corrected accordingly.

## Full Affine Integration Checkpoint

The authoritative RasterTransform and shared stack transform now persist required `{x,y,a,b,c,d}` fields, mapped directly to compositor `[a,b,c,d,x,y]`. JSON, TypeScript, GraphQL, Proto, grammars, embedded demo assets, retained mutation copies/digests, snapshot codecs, source normalization/crop placement, native surface parsing, React pointer mapping and their fixtures were updated together. No compatibility parser or migration remains. Mutation byte buffers grew from 55 to 63 for mask transforms and from 49 to 57 for pixel transforms.

Inspector now exposes localized mask linkage and horizontal shear. Linkage uses `inverse(target) × source × mask` directly in the common parent frame; scale, rotation and shear controls use the shared canonical decomposition. Link changes remain the existing guarded ChangeLayerMask semantic mutation. A new native law renders nested pixel/group masks before and after unlink/relink, checks archive reconstruction and exact inverse restoration. The existing neutral control sequence now exercises shear and both linkage directions; Inspector vectors require both controls and their localized binding/value/step contracts. These native integration assertions are not yet verified.

Executed evidence: initial Raster red run 109 passed/18 failed; first green attempt 118 passed/9 failed exposed signed-zero fixture values. After normalizing JSON fixture zeroes, **127 Raster TypeScript tests passed** (`raster-affine-ts-green-2.log`). Shared pointer/compositing first run 133 passed/1 failed exposed a partial identity fixture. After replacing it with the full six coefficients, strict source checking and **138 shared TypeScript tests passed** (`raster-affine-pixels-ts-2.log`), including sheared/reflected pointer vectors and SVG/Three.js reference output. Shared native affine run 2 passed **38 tests, zero skipped** (`raster-affine-pixels-native-2.log`, session 61087 terminal exit 0).

Full Raster affine native run 2 (40235) and focused native paint run 1 (59459) are live. The added Inspector assertions may require another run if compilation already consumed their source. Native document/mask integration and live controls remain unverified. Browser policy rejection remains in force; no bypass was attempted.

## Mounted Fixture Audit

The old mounted selection suite and native surface test literals still supplied empty transform objects. Complete matrices are now required for any supplied transform; an omitted whole optional transform remains identity where the host contract permits omission. The mounted red run (4223, raster-affine-mounted-red-1.log) executed 31 tests: 23 passed and 8 failed. Those test inputs and the Paint2dHost session fixture now explicitly carry six identity coefficients. Native paint's empty transform literals were corrected as well.

Four added mounted cases use neutral linked/unlinked sheared-mask placements on pixel and group owners, asserting the emitted stroke coordinates and expected-mask descriptor. The independent matrix/pointer oracle remains covered by the shared suites. Mounted green run 1 (78690) is live; no passing result is claimed.

Mounted affine green run 1 (78690) completed successfully: **35 passed, zero failed** (`raster-affine-mounted-green-1.log`, exit 0). This includes all 31 existing selection/protection/cancellation cases plus four linked/unlinked sheared-mask pointer cases. These are mounted React tests, not live browser acceptance. No new production pointer changes were needed after the original six-coefficient integration; the observed eight regressions came from incomplete test transforms.

The follow-up full-host compositor suite passed two selected completion/cancellation cases (40038, 703 filtered). Native integration remains pending: Raster's Nx process exited 137 but its child build was still alive; native paint run 1 stopped in shared retained-clone compilation. See native-verification for exact process ownership and run 2 status.

Native surface paint run 2 (75160) is terminal exit 0: **66 tests passed, 196 filtered** (`raster-affine-surface-native-2.log`). This verifies the native paint suite against explicit affine fixtures, including the shared shear/reflection pointer vectors; it does not establish the full Raster mask-link/undo/archive workflow or live end-user acceptance. Full Raster run 4 (63457) remains live.
