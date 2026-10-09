# Draw PNG Export Execution Plan

## Verified Current Gap

The app command export-document admits PDF and SVG only. Its registered PNG artifact serializer uses drawing_document_to_semio_drawing and deliberately loses gradient, blend and fill-rule semantics. The existing document raster producer already preserves these facets for supported resolved path/image content and resolves actual Boolean and trace geometry. It explicitly refuses unresolved text rather than producing faithful shaped text. Therefore adding a synchronous png branch to the current command would not complete the requested experience.

## Owned Route

Give exportDocument its own resumable command factory, with explicit publication contract/proof rows and PNG in the argument options. Keep all format handlers under this owned route; expensive SVG/PDF must also stop blocking the first command step. The export is a download effect, not an authored mutation or history entry.

Acquire the genuine immutable SnapshotRead and retain its source authority through rendering and publication. DocumentVectorJob::from_snapshot_read already supports this without a self-referential borrowed snapshot. Factor the document raster job's construction from that owned vector source and return the actual read lease through its retirement rather than dropping the vector at the stage boundary.

Advance preparation, trace, Boolean resolution, raster conversion, raster composition, PNG encoding, output sealing and source-close through bounded grants. Expose their real progress, checking cancellation at every transition and immediately before publication. Use the first-party pixels PNG encoder and ArtifactOutputChunks/SealedArtifactDownload patterns already used by Raster; avoid a second encoder, base64 string duplication or transient file export.

## Retirement Prerequisite

DocumentRasterJob and RasterSceneJob currently clear owned children/records at cancellation and completion. Add actual composed retirement before using them in a retained command. Adopt the real PathRaster, affine sampler and compositor retirement children, remaining resolved records/assets, and unpublished output. Keep typed scene owners shallow and drain collection entries under grants. PNG encoding must hand off its real bytes/pixel/deflate owners through the same physical retained-close API used by current framework jobs. No phase may substitute invented counts for actual owned payload or backing release.

## Contract and Fixtures

Author schema-first export options: format, output framing, pixel dimensions, transparency, source/pixel/work/encoded-byte limits. Preserve authored artboard framing by default; selection export must explicitly use selected world geometry. Refuse invalid/nonfinite/empty requested extents and unsupported visible content before downloading.

Use language-neutral documents for transparent solid paint, gradient alpha, fill-rule holes, isolated group opacity, every blend operation, nested affine images, actual Boolean/trace results and exact output dimensions. Run Rust and TypeScript against the same decoded pixels; independently decode bytes and compare authored SVG pixels using existing test-only Sharp or renderer oracles. Include max-work/source/pixel/output limits and abort before, during and after encoding. Source must remain unchanged, undo history must remain unchanged, and no partial download may be published.

Text needs a genuine shared shaped glyph producer for native paint, picking and raster export. A rectangle or midpoint-gradient proxy is insufficient for full acceptance. Keep this remaining requirement explicit until actual Unicode, multiline, rotation, shear and gradient text workflows are demonstrated.

## Current State

This is an execution design tied to current source evidence, not a product implementation or passing test claim. The coordinator is finishing current integration evidence; the next available execution lane can adopt this route without duplicating the existing raster or PNG infrastructure.
