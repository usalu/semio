# Current I/O Endpoint Authority

The neutral I/O vocabulary now compiles independently of OS/S/Hub. Its compiler and ownership laws prove selected source/manifest separation, literal reference preservation and controlled typed error projection. General routing still models framework-owned byte/text carriers using specific artifact-kind identifiers, so package compilation alone does not prove semantic independence from specific registrations.

Current literal carrier declarations from the actual neutral vocabulary:

- Line 195: /// two carrier dialects (`CARRIER_BINARY`, `CARRIER_TEXT`), whose native encoding IS the raw
- Line 206: pub const CARRIER_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

The canonical next interface should distinguish a generic payload endpoint (binary or text) from an artifact endpoint carrying an exact ArtifactDialect. A payload endpoint has no plugin/artifact owner and survives deletion of every specific provider. An artifact endpoint retains literal identity and is admitted by owner-supplied runtime registration. Carrier routes and artifact routes must be different closed schema alternatives; no prefix classifier, renamed specific identifier, implicit fallback or compatibility conversion is justified.

This proposed interface requires schema-first endpoint/routing fixtures and coordinated direct caller updates. I/O execution must also supply the caller's finite admission/progress/cancellation context rather than allocate a hidden unlimited NativeEncodeControl for TextError decoration. Native async Send requirements and callback ownership need explicit validation before designing that context. The current schema and production are unchanged by this report; this is a queued architecture design, not a completed extraction or arbitrary deletion proof.
