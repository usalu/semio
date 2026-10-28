# Shared Live Drawing Composition

The Layout artifact now owns one retained Drawing composition at scene/drawing/composition. Export, Blueprint and Preview consume this owner. Source addresses distinguish document and individual page children; document content fitting and authored page Canvas placement remain distinct policies. Drawing placed in image frames retains exact binary64 frame bounds. Deferred display-list preparation no longer eagerly lowers a complete document Drawing.

Canvas paths and glyphs retain source-local geometry, full six-coefficient affine transforms, nonzero fill, RGBA channels and unrounded stroke width. Images publish an original encoded proxy once through a unit-square affine. Inline formatting uses a fixed 4096-byte stack buffer. Publication slices are bounded to 512 bytes; original grants fund retained fragment strings and paged Chunk rows before source progress. Image proxy ownership moves into the existing Chunk owner without duplicating its bytes. Cancellation and original snapshot identity are checked before Drawing work.

The public TypeScript package explicitly exports Page Drawing validation, independent child identity helpers, the shared composition, and Canvas publication helpers. Package forwarding files refer to canonical sources; no source copies were introduced.

## Evidence

After the source move and initial portable font grant integration, actual composed tests passed ten cases/twenty runs, sixteen paths, twenty-four glyphs, six images and fifty cancellation cuts. Page Canvas witnesses passed six runs, twenty-four glyphs and twenty-four cancellation cuts against independent OpenType/Three/Skia/Node byte oracles. Canvas publication helpers passed two path cases and one image case through independent Three matrices and Ajv. Public Layout/composition/PDF strict checks covered 699 sources with zero diagnostics at that point.

The settled portable font retirement consumer now forwards the original remaining grant through FontRetirementCursor and retires returned glyph/segment/line buffers through the same ledger. The actual composed suite passed again with 38,940 positive-axis unchanged-state undergrant probes, retaining all twenty content runs, six Page Canvas runs and seventy-four cancellation cuts. Matching native quote/refusal laws are authored. The latest strict graph covers 700 sources with zero diagnostics.

Native actual RenderWork laws now cover document plus Page Canvas paths/glyphs/images and four cancellation cuts through OriginalJobFixtureCaller, together with the existing real text-overlay law. Native syntax and fixture mounts are checked, but Rust compilation and GUI execution remain unrun because the repository runner is unavailable.

## Remaining Work

The original prepare() still builds the frame/chrome display list synchronously, and non-Drawing fragment preparation and final scene_surface assembly retain older eager operations. Cold display-list scene-query routes still need convergence with the retained Page Drawing composition. Drawing ancestry/source visits need enough separately granted source items; RetirementDemand does not contain a minimum item field. Portable VM object/header and immutable String allocation are not claimed as native physical-capacity accounting. The central retained mutation/diff/issuer work and complete embedded Drawing controls remain active.
