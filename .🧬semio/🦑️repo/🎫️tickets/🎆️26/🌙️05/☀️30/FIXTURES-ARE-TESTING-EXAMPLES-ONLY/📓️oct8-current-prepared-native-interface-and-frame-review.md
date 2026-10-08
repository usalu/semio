# Current Prepared Native Interface and Frame Review

Observed prepared Rust SHA `38e8c6ffbc1457c0198db96d2263a7908f24d2186d7850201a40fd4f5b7d43bc`; peer grant additions are current observations, not authored work or runtime positives. No native invocation was run here.

Narrow interface removal belongs to the actual retained owners: AtlasPages, RasterRejected/Producer, RenderUpload, InputRejected/Input, RenderJobRejected and CommandPages. Keep unrelated scalar Gate and engine document/router booleans outside this scope. Existing prepared native unit tests directly call producer276, rejected288–289, pages322–326 and input/admitted304. Replace their owner-driving interface with explicit demand queries and caller grants; preserve original generation, reservation credit and mailbox identity assertions. Local JobRejected2647 is a genuine Input boolean delegation, unlike unresolved engine calls sharing the same spelling.

Physical decomposition currently visible:

- Atlas boxed byte pages: PREPARED_ATLAS_PAGE_BYTES per actual page, then sizeof fixed Option<Page> directory backing, then permit phase transitions. Permit release_step changes original process counters in four atomic phases; these counter changes are not additional physical frees.
- Raster source/retained-source Vecs: logical truncate phases do not release backing; original capacity*sizeof(element) is due separately even for empty retained vectors. Key String capacity is separate; raster slots Vec backing and metadata/credit phases remain separate.
- Command page and directory Boxes: remove scalar entries without pretending physical page release; whole page sizeof and directory sizeof separately before final terminal.
- Abandoned generic frame (~70) and Job frame2800: claim atomic slot, recover exact original Box from pointer; denied/nonterminal step restores Box::into_raw and queued state. Terminal requires actual child emptiness, then separate sizeof<T>/Self shell release. This preserves pointer custody; do not replace original owner with a fresh frame or discard generation state.

Concrete concurrency qualification: both abandonment close routines return Complete when no queued slot CAS succeeds. Another actor may already hold state3 with a live pointer; no selectable work does not prove aggregate terminal emptiness. Distinguish genuine empty from contention/claimed-owner Blocked, using actual slot state and original pointer ownership.

Reuse genuine Value RetainedCloneGrant/Progress/Step and fallible demand axes through existing first-party interfaces. Demand is a quote, not authorization: do not synthesize a grant inside public close_step. Existing prepared_vec_bytes checked capacity and pointer-preserving Box recovery are reusable; wrappers calling bool then synthesizing receipts still need canonical replacement. Preserve original allocation extent, denied grant identity, terminal child joins and exact physical shell receipt; no scalar/error fallback.
