# Concrete Builder Ledger Proposal Review

Coordinator read the complete proposed builder/page/reader report and current neutral Owner/Page/Reader implementation. Proposed domain fields and callables are source design, not a runtime release. Reusing the existing payload transition slot, removing the extra PageCell, retaining private neutral resources, and replacing all boolean/counter admission with real mandatory phased calls follow the shared ownership contract.

## Exact Current Intrinsic Prices

Current neutral constants are owner200/2/2, page520/3/2, reader136/2/2; page backing is256 bytes and is separately allocated under a256-byte work grant. Every resource has its own296/6/6 admission cell. The proposed page domain wrapper160/3/3 plus domainrecord264/3/3, domaincell296/6/6, neutralpage520/3/2 and neutralcell296/6/6 totals **1536 bytes /21 slots /20 owners per256-byte destination page**. This is declared logical capacity, not measured JavaScript heap size.

## Whole-Field Capacity Consequence

Simple exact arithmetic: an8MiB byte field needs32768 such pages; page charges alone would be50331648 bytes (48MiB), exceeding the explicitly shared32MiB composition ceiling before the payload, source, parser, builder or UI document. This is a model inference, not an executed app-fit test or a request to raise the quota. Current builder `beginRead` also waits for `copyPhase === ready`, retaining the whole destination extent before exposing a reader. Small scalar/page fixtures cannot establish that all currently admitted application fields fit this composition.

The live cutover therefore must either prove a stricter real schema-derived maximum for these particular fields or provide retained incremental consumption/release so completed input pages need not coexist for the whole field. The natural pipeline is source fragment copy, admitted sequential semantic consumption, and exact last-reader/page retirement under backpressure; the same original source/evidence and final publication rules still apply. This does not authorize an unbounded queue, early native InputAck, whole-buffer fallback or a second pool.

This capacity law is being routed to the UI owner before the child declaration is frozen. Test-first local child admission can proceed while this large-field readiness requirement stays explicit; no all-app fit or completed streaming claim follows from the proposed API.

## Other Required Boundaries

Actual constructor/source/finalizer failures must retain the exact first unknown root before rejection; the old string-only catch paths are not sufficient. Evidence storage/refund needs the separate exact two-party source release. A public caller cannot supply page links or reader source identity; all traversal derives from the original admitted roster. Domain completion, intrinsic resource completion, admission-result detachment, cell completion and parent-slot unlink remain separate granted phases.

