# Retained Wire Range Identity Audit — 2026-10-28

Read-only source support for actual17308 E0063; backend owns repair. No tests, builds, source edits or probes. Paths below are beneath 🧰️framework/🔨️modules/📡️replication/⚙️codec/🛬️retained/ unless stated otherwise.

## Actual Whole-Owner Base

OriginalWireSourceIdentity at 🦀️.rs:10 contains address, extent, range_start and authority and derives full Eq. Plain slice adapter16 uses range_start0 with address=self.as_ptr; its pointer already identifies that slice start. Vec delegates its slice at20. The failing PagedBytes adapter206 borrows its whole retained_bytes from index0, extent=self.len and byte_at(index) uses the same logical origin. value/📦️paged/🦀️.rs:17,56,85 establishes that PagedBytes owns its complete PagedList and retained_bytes returns that backing. Therefore range_start0 is appropriate for this whole-owner adapter; this is not a rule to use0 for every framed view.

Existing fixture 🧪️tests/🪪️range.json and Rust test82 use rangeStart2/extent8, assert a moved descriptor remains equal, and assert rangeStart10 with identical backing/extent/authority differs. Foreign backing and authority generation also differ. This is the declared distinction between a stable underlying owner and an actual framed subrange.

## Concrete Remaining Decoder Binding Defect

At this read RetainedWireFieldDecode struct47 stores original address, extent and authority but no range_start. Constructor new50 discards identity.range_start. Its original_source_identity92 reconstructs range_start0, and validate_source108 compares only address/extent/authority. Thus a custom framed view with nonzero range base loses that identity on decoder construction; a same-address/same-extent/same-authority source with a different range_start is not rejected by validation. That contradicts the cross-range distinction the identity fixture explicitly encodes.

The existing test82 checks identity struct Eq only; it does not construct/advance a decoder against switched framed sources. The precise test seam is a genuine RetainedWireByteSource whose stable backing/authority stays fixed while byte_at reads a nonzero frame offset: initialize decoder at one frame, verify original_source_identity retains it, then ask demands/advance with another equal-width frame and expect the existing identity refusal before mutation/allocation. Backend received this concrete finding. A full-identity retained field or equivalent storage/roundtrip/validation can address it; do not substitute a fresh authority or silently force0.

## Other Initializers And Coverage

Bounded Rust initializer inventory also found test-source constructors still omitting range_start in tool-run tick receiving native/progress/step/entities/identity test modules. Those whole fixture byte owners can appropriately use0 if their byte_at truly begins at their descriptor origin. Pack value source and dictionary source tests also construct identities and must follow their actual view origin. These are compile-time test migration sites, not proof of another runtime defect. No compatibility API or runtime dependency is needed.

All ranges used by new_range/new_json_string/prefixed UUID/timestamp decoding remain relative to the source byte_at coordinate system. The additional identity range_start distinguishes that source window against another view; it should not be blindly added to each byte_at index and double-offset physical slices. Existing range/funding laws remain useful but passing them has not been observed by this audit.

## Final Semantic Patch Review While88804 Compiles

Current retained decoder47 stores one complete OriginalWireSourceIdentity. Constructor50 retains it, full identity accessor92 returns it verbatim and validate_source108 compares full Eq. The previous loss/hardcoded0/partial-comparison defect is removed in current source. byte_at addressing and relative field-range coordinates are unchanged, with no new range_start addition to reads, so no source-level double-offset application was found. PagedBytes206 remains whole-ownerbase0; reference/Vec adapters preserve the source identity rather than reconstructing a framed view.

The additional usize changes decoder layout, but retirement wrapper190/194 quotes size_of the actual type rather than a stale literal. No direct old decoder.original/extent/authority field access was found in the inspected consumers/tests; similar search hits were identity accessor methods. Funding/cancel algorithms retain their previous grant/receipt paths. These are source checks, not a runtime layout/allocator pass.

Outside-wire reviewed constructors for tool-run pages, pack original pages and dictionary pages now use0 where byte_at begins at their full descriptor origin. Delegating methods return their existing identity unchanged. Pack source test18 now explicitly perturbs range_start using struct update over the existing identity, preserving other authority fields and asserting zero-allocation refusal. This audit did not independently certify a claimed total21constructors by count; it reviewed the relevant literal/delegating sites found in the existing module inventory and their byte origins.

New retained test85–89 is genuinely decoder-level, not only identity Eq: Frame.byte_at indexes backing at range_start+index; test constructs decoder with nonzero base, asserts full identity roundtrip, compares demands for moved/equivalent versus switched frame, and on inequality checks advance refusal, zero heap, no receipt and unchanged original demand. It then cancels/closes the decoder. Limit: it cancels before successful field decoding, so it does not itself assert successful nonzero frame output bytes; no double-offset source change was found, but that success path is not newly executed by this test alone.

No concrete remaining source defect found in the reviewed patch. Compilation/run88804 remains pending and outside-wire initializer tests remain subject to their owning test binaries; no build/test/probe or source edit was performed by this audit. Backend received the exact consistency and test-limit findings.
