# Shared Image Payload Tests

Tests were authored against the announced RasterImagePayload API before the production module was created. The neutral fixture fixes transparent image wire, original native receiving, shared immutable aliases, lease closure, finite stock, two exact small images and clone sizes of four bytes and one MiB.

The model tests check refused owned construction returns the same pixel allocation, refused shared construction returns the same Arc, successful owned/shared construction preserves pixel storage, public owner aliases share an Arc, and shared admission has zero retained capacity. A live external child keeps its pixels while payload retirement completes; closing a live lease takes finitely few turns and final ownership closes the image allocation under funded grants.

Native receiving runs through OriginalWorkFixture and its same original decoder. Transparent ToValue and retained canonical JSON output must match the Image snapshot and independent Serde JSON bytes. Retained clone tests require the original pixel pointer and a constant alias capacity across four-byte and one-MiB frames. Positive source, projection, canonical cursor, alias, clone and closure effects all execute within genuine original test turns.

The model already declared the artifact-app-testing dependency. Tests additionally require semio-framework-job with its existing testkit feature; this dev dependency is authored. The production owner subsequently created the module and mounted these tests; its public API matches the authored test signatures. The retirement imports were checked against their actual controlled and arc_custody modules.

The exact image equality helper was reviewed read only. Each comparison charges both input bytes; odd remaining byte grants yield. Empty fields still require paid length prefixes, metadata is accessed by ordinal, both immutable original root addresses are bound and mismatch completion is sticky. No concrete type or boundedness fault was found in this review.

Validation is static only. The payload fixture parses as JSON and its path has 144 characters. No compiler, native execution or runtime test pass is claimed while source admission remains blocked.
