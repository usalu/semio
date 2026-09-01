# Streaming Semantic Owner Inventory

This is a read-only design boundary after the authored R178/R179 release. It does not add an API, change a price, or mount a semantic decoder.

## Actual Existing Source

The entire retained `📦️wire/🟦️component.ts` was read. It owns a whole transferred Uint8Array, eagerly allocates NumericIndex.empty, retains previous symbol/map-key Uint8Array views, uses subarray/TextDecoder for text and DataView over the source buffer for numbers, and produces arbitrary map/array containers. Its surface path instantiates UiSurfaceByteBuilder; that builder allocates an uncharged page-pointer array and byte arrays. Its string error field substitutes for arbitrary caught roots. These paths cannot become live same-ledger semantics by changing only byte access.

The typed module's ownership region and UiPayloadRetirement were read. Its Root has value/references/owned/bytes/children/fields/kind, but no neutral registration or admitted parent handoff. UiPayloadRetirement forces byte/child results to pending and detaches some wrapper pointers in the same step. The shared ledger adoption must preserve exact child refusal and grants and must not imply that these old roots are already resident-accounted.

## Required Ownership Split

1. The exact operation payload must retain one semantic decoder shell before construction/finalization and before the already admitted reader is exposed to it. Existing Payload already has separately inventoried builder/reader/evidence/page roots; none is an unused decoder slot. Any new decoder reference is an explicit payload catalogue change requiring peer-coordinated price release, not an implicit reuse of the current312-byte domain.
2. Decoder fixed state contains one pending scalar/read status, exact profile, phase, numeric/UTF8 accumulators, original reader/builder/payload pairing, one exact child-admission slot, and strong roots for variable semantic storage. Each field, facade and original phaseful witness must appear in its own16+8n catalogue before allocation. No price is asserted until that concrete declaration is complete.
3. Text, symbol records, parse frames, typed container records and Surface backing are separate admitted variable descendants. The declaration must account for both the semantic data and every neutral admission/record/control pointer, including simultaneous source window and destination output. A NumericIndex wrapping uncharged values is insufficient.
4. Input reader advancement and parser application are distinct grants. A reader child may consume a whole grant while progressing alias/page retirement; the decoder forwards that exact result and performs its own latch/parse bookkeeping later. Varints, UTF8, floats and field headers may cross windows without borrowed subarrays or an entire reconstructed operation buffer.
5. Typed output must be a genuine retained, resident-accounted owner with exact publication/close handoff. Wrapping an arbitrary decoded map in the old OwnedUiPayload would retain an uncharged parallel graph. The final decoder witness requires input-reader/builder pairing settled and every parser/transient child retired; published typed output remains charged under its actual recipient.

## First Proposed Executable Boundary

The smallest next schema packet is exact scalar streaming plus original reader binding, not whole UI decoding. It should declare a concrete decoder record and parent slot; accept only the genuine field-owned payload and its private reader; and prove constructor loss, foreign reader, one-byte latch, full-grant child forwarding, UTF8/varint/f64 window splits, every cancellation phase, and exact first-fault custody. A third-party native/Buffer/TextDecoder oracle can validate bytes and scalar values in tests, but does not become a runtime interface.

The next packet then adds admitted symbols/frames and typed output storage. Before all supported native profiles and complete field ownership are covered, it remains off the live UiNodeView/WGPU-web entry. Whole-field streaming capacity, source-page continuation, publication/notification/ACK and per-instance final retirement remain separate exit gates. Neither R178 seventeen authored tests nor the old whole-buffer typed tests certify this new boundary.
