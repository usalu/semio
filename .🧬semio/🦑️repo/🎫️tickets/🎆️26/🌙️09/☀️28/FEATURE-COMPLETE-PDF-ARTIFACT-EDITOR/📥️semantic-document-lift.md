# Progressive semantic document lift

DocumentLiftCursor owns the existing Lifter and its unpublished results across turns. Its traversal grant drives the bounded page tree and visited-index retirement. A separate semantic permission admits one named action. Header and document information, optional content, embedded files, branch resources, page recording, each page projection, catalog processing, and finalization have distinct actions. Catalog processing starts after the page projection turns.

Cancellation or error closes any retained traversal index under the caller's grants. The stopped cursor transfers its partial Lifter, remaining semantic plan, catalog, and any page rejected by semantic lifting. The synchronous lift_document_with entry point drains the cursor and preserves its existing return path.

The standalone harness passed all 48 tests, including the production staging state machine with stand-in semantic methods, one-work page traversal, and cancellation ownership. The existing package test exercises the real resource/text path with a nested PDF fixture. A separate package test checks the named semantic turns and partial results after cancellation or malformed page content. The dedicated Cargo attempt stopped before PDF lift in semio-plugin-retained-command because ArtifactApp lacked completion_retirement_demand. A refreshed shared Cargo invocation is pending.

Whole-page projection remains an opaque synchronous action that can read or decode nested resources and content. Mounted import has no bounded-work claim for that action yet and full publication remains separate.
