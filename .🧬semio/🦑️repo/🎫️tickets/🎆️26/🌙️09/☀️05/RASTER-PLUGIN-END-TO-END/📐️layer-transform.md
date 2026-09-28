# Layer Transform Controls

The Inspector offers layer translation and pixel dimensions, while scale/rotation/shear currently exist only for masks. Complete layer transforms require a semantic replacement valid for pixel and group layers, exact prior-transform conflict checks, structural protection, complete history/codecs and native/React-neutral controls.

The ChangeLayerTransform contract carries layerId, expected and transform. Reuse the complete six-field RasterTransform and existing shared affine controls. Generalize the diff field pixelTransform to transform, with no compatibility alias, so groups use the same field. Existing partial move patches remain valid translation deltas; coalescing must preserve sequential full-transform/translation order and reject ambiguous mixed fields in one input patch.

New schema and neutral shear/reflection/rotation cases are authored first. The TypeScript test compares a complete parsed layer patch with existing fast-json-patch replacement and inverse. Native mutation, retained publication, exact undo and both-language Inspector verification are required before claiming this feature complete.

## Implementation and Verification Checkpoint

Implemented ChangeLayerTransform across JSON, TypeScript, Rust, DSL, binary retirement/digest/application, GraphQL, Proto and the semantic mutation catalog. Its exact expected matrix prevents stale replacement, and singular matrices/adjustment targets are refused. Generalized the layer diff field to transform and removed pixelTransform without an alias. Sequential full-transform and translation coalescing preserves order; a single ambiguous full-plus-partial patch is refused.

The Inspector now exposes horizontal/vertical scale, rotation and horizontal shear for pixel/group layers with English/German labels and commit bindings. Existing structural protection applies. Controls reuse shared affine decomposition/composition, while history carries exact matrices. Added neutral matrix vectors, independent Three.js composition, fast-json-patch replacement/inversion, native mutation/coalescing laws, property/Inspector tests and retained app tests that undo/redo each of four edits for both layer kinds.

Raster TypeScript green run 3 passed 139 tests (41757, terminal exit 0). A subsequent explicit ambiguous-patch rejection/null-partial acceptance test is being verified in green run 4. Native layer-transform run 1 (18490) remains active; expected census is 321 if all latest test source is included. Native publication, exact retained history and live Inspector behavior are not claimed verified.

TypeScript green run 4 (29252) is terminal, exit 0: **140 passed, zero failed**. Full/partial ambiguity and null partial-field acceptance are now explicitly covered. Source review also fixed the new mutation's missing entry in the ordered KINDS vocabulary. Protocol inspection confirmed wire tags are looked up by keyword in the normative protocol file, not inferred from Rust enum position; the new metadata tag 16 matches its protocol record. No tag reorder is needed.

The additional singular-patch regression failed as intended in TS red run 2 (19836, exit 1: 139 pass/1 fail). The diff parser now calls the shared inverse validator, matching native rejection. TS green run 5 (48286, exit 0) passed all 140 tests, 582 assertions. This is the latest TypeScript source checkpoint. The oracle catalog's supported kinds list also includes change-layer-transform.

## Native Integration Result

Native run 1 (18490) completed: **321 tests, 317 passed, 4 failed, zero skipped**. All four new transform tests passed, covering pixel/group semantic changes, ordered diff coalescing, localized Inspector bindings and retained four-edit undo/redo. Mask link/unlink exact composite preservation and full-capacity pixel/mask replacement also passed. One failing exhaustive retirement fixture omitted the new mutation; added the real ChangeLayerTransform payload instead of weakening its census assertion. Its follow-up verification is running.

Remaining failures are editable archive hydration and two retained PNG export cases. Archive fixture construction omitted the dialect that normal app construction sets; corrected the fixture to carry RASTER_DIALECT. Export command and media export both reported worker terminal faults. Native run 2 (75362, raster-layer-transform-native-2.log) contains temporary test-only [DEBUG] stage timings/error output to distinguish expensive work from operation failures. Remove the two cfg(test) timing/log statements in editor/export/🦀️.rs after diagnosis. The overall goal remains open; native transform evidence does not prove live UI acceptance.

Full native run 2 (75362) passed all **321 tests**, zero skipped. The corrected archive dialect and complete retirement fixture are verified, together with exact transform undo/redo. Live end-user acceptance remains separate and open.
