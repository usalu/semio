# Current Prepared Implicit-Grant Review

Read-only source SHA-256: `38e8c6ffbc1457c0198db96d2263a7908f24d2186d7850201a40fd4f5b7d43bc`. Current source differs from the historical six-error compiler epoch; peer-added grant helpers are observations, not Root-authored work or actual runtime credit.

Exact unsupported authority paths in prepared Rust:

- PreparedRasterProducer1395, PreparedRenderUpload1581 and PreparedRenderInput2309 manufacture authority from their own demand; caller supplies no grant, and query/refusal becomes boolean false. Preserve typed refusal and original custody through a caller-supplied full grant.
- PreparedRenderJobRejected2645 calls input.close_step() at2647 and removes input only on boolean completion. This is a direct local live delegation into the implicit Input path.
- PreparedAtlasPages864 drops a scalar page, directory backing and permit through an ungranted boolean method. PreparedRasterRejected1086 first truncates logical length, then replaces original Vec to release physical capacity without caller admission. Logical page truncation is not physical release credit.
- PreparedAtlasAbandonment746, AtlasPages782 and RenderCommandPages1699 grant wrappers still call old boolean mutation routines then synthesize progress. Pages/Commands use demand.unwrap_or(0); commands lacks a before/after no-progress check. Preserve typed demand failure explicitly and derive physical receipts from the actual accepted release rather than wrapper shape.
- PreparedRenderInputRejected2082 calls draw/overlay retire_step and permit release without a grant. Preserve originals and mailbox/permit generation identities until actual admission and terminal child ownership.
- PreparedRenderGate3744 only changes scalar gate phases and waits for pending/last_valid owners; this alone is not evidence of a physical backing defect.

Prepared unit tests directly call these booleans: producer276; rejected288–289; admitted304; pages322–326; owners346/382/410; rejected432/438/456/461. These are existing behavioral tests needing genuine full-grant propagation, not independent authority for self-created grants. Do not attribute unrelated engine/reconcile close_step calls to prepared types solely by method spelling. The local rejected-input call and internal wrapper calls above are resolved source edges; broader production call resolution remains bounded.

Canonical repair should gate item/depth/capacity/release axes before original mutation, propagate real child receipts/refusals, preserve denied pointer/permit/mailbox identity, pay original empty retained backing and final abandoned Box frames separately, and require terminal emptiness. No compiler, test, production job or source edit was performed by this review.
