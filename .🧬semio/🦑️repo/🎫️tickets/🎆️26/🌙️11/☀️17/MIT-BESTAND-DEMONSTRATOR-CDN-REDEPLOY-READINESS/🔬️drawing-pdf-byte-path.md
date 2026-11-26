# Drawing PDF byte writer

The Drawing PDF leaf starts from a default PDF snapshot and fills the page, extended graphics states and optional standard text font. Its current painter leaves the retained object graph, trailer, encryption, document ID and images empty. The generic PDF encoder therefore takes its fresh lowering branch with the same starting object number, default lowering and writer options, and empty extra trailer entries. Its PNG-source filter cannot remove any object for an empty image table. Both paths use the same byte serializer.

The Drawing exporter validates the same declared-version character set and uses the same `1.7` fallback. Its malformed-version error string is constructed at the Drawing boundary. The broad PDF encoder constructs its typed malformed error before the caller turns it into text.

The existing portable Drawing fixture exercises text, geometry, transforms, opacity and stroke. `the_page_keeps_its_text_through_the_pdf_codec` now compares the exact bytes from both entry points before decoding them. `hayro_paints_the_same_geometry_the_rasterizer_paints` checks the output independently with the Hayro rasterizer.

This source audit establishes the equivalent call path for current Drawing snapshots. The compiler-image reclaim removed the coherent native PDF and Drawing externs needed for a standalone rustc replay, and the shared compiler lane already has the Energie native verification and component rebuilds. The compiled Drawing test is pending.
