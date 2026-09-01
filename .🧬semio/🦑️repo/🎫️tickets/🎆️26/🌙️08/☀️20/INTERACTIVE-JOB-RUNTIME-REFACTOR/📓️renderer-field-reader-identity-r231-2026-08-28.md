# Whole-Field Reader and Scalar-Child Identity Declaration R231

Ticket-only proposed ownership refinement after the released strict scalar source-oracle packet. No runtime source, schema, grant, price, caller, canonical scalar file or native code is changed. This does not extend the R204 arithmetic oracle into a live consumer certificate.

## Source-Based Decision

The mandatory original resident-reader consumer must be the **original field grammar owner**, not a standalone scalar profile and not an opaque-transfer consumer. The actual typed path already determines whole-field semantics; preserve that ownership and grammar instead of adding a second codec.

- [RetainedUiTypedCursor](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts:417) currently takes a public input/profile, owns a whole-buffer WireValue decoder, and only then runs typed normalization. Its9 cursor fields and6 Builder fields are not a streaming/admitted parent yet.
- [RetainedUiChildIdsCursor](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts:465) is8 fields over an intrinsic whole BigUint64Array up to1024 bytes. It is not a canonical list<u64> byte parser.
- [Resident reader admission](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts:689) already retains reader state in both the original payload and original pending slot before shell construction. Its current raw byte advance is the exact113-call replacement seam, not a fallback to keep.
- [Private field](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️component.ts:107) binds the real source, operation, opcode, node, field name, byte length and producer receipt. None of these may be supplied by a semantic-decoder caller.

The direct scalar-reader binding in the frozen proposed ScalarReadReceipt contract cannot by itself consume PACK's structural outer0x11 tag or whole list/container grammar. The five arithmetic profiles must not be expanded into an opaque escape hatch. R204's runtime-unmounted declaration remains byte-preserved. The change below is a future **semantic** declaration amendment, not one of the fifteen provenance pointers or an implied authority to implement.

## Original Parent and Mandatory Callables — Proposal Only

Use the existing typed cursor as the actual field owner. Do not create a twin PACK parser or a scalar/raw consumer union.

```ts
payload.beginTypedField(grant): { step: RetainedUiWireStep; cursor: RetainedUiTypedCursor | null }
payload.closeTypedField(cursor, grant): RetainedUiWireStep

payload.beginReader(builder, consumer: RetainedUiTypedCursor,
  receipt: OwnedUiFieldReadReceipt, grant): OwnedUiResidentReaderAdmission

reader.prepareField(consumer, receipt, grant): RetainedUiWireStep
reader.advanceField(consumer, receipt, grant): RetainedUiWireStep
reader.settleField(consumer, receipt, grant): RetainedUiWireStep
reader.cancelField(consumer, receipt, exactRetirement, grant): RetainedUiWireStep

RetainedUiTypedCursor.matchesReaderConstruction(consumer, payload, reader, receipt): boolean
RetainedUiTypedCursor.matchesReceiptApplied(consumer, reader, receipt): boolean
RetainedUiTypedCursor.matchesReceiptDiscarded(consumer, reader, receipt, exactRetirement): boolean
```

These are names for review, not source-present exports. No optional consumer, old advance overload, profile parameter, public record/cell, structural callback, caller-selected raw source, or second positive test consumer is proposed.

The original payload chooses the dialect from its privately associated original Field before allocating. The existing typed cursor's public input/profile constructor must be replaced at the live cutover, not left as a second admissible route. Its private state holds the original payload and derives the original source field from that association; equal metadata, same node/ordinal or copied producer receipt does not substitute for identity.

The scalar child is private to that exact grammar owner. A field grammar phase—not a public selector—chooses natural-u64, natural-safe53, ui-value-tag, utf8-codepoint or ui-f64. Structural PACK bytes such as0x11 are checked by the same field grammar; they are not recast as f64 or accepted by a raw scalar profile. The existing WireValue and TypedBuilder algorithms supply the grammar/normalization semantics, but their whole arrays, generators and object ownership require the separately enumerated refactor before live use.

## One Receipt, One Latch, Original Serial

There is one original reader receipt for the entire field, installed before reader finalization. Rename the currently proposed scalar-specific receipt at the eventual semantic schema cutover; do not allocate a second receipt when the scalar child becomes active.

1. The field owner prepares its original receipt in a separately granted64-byte phase, with checked u64 serial.
2. The resident reader preflights65 bytes, reads one byte into the same preadmitted receipt, and leaves source cursor unchanged. Child maintenance forwards exact kind/items/bytes and performs no appended receipt mutation.
3. A separate64-byte commit advances source cursor once. The field owner observes that exact committed serial separately.
4. The field grammar either consumes the byte as structure or delegates it to its exact active scalar child. Only one arithmetic latch owns that byte. No second reader, copied input page or arbitrary-byte API appears.
5. Scalar apply is65 bytes; field structural work requires its own declared fixed transition grant. No numeric grant is certified for structural parsing in this report.
6. The field owner records the exact scalar-child/phase/result serial before the original receipt can settle. A child close/replacement is blocked until that receipt's applied or cancelled frontier is owned.
7. Reader settlement and the owner's settlement observation each use separate64-byte turns. The next byte, next scalar profile and result publication wait for the original observation.

The scalar's result is not a public free-floating primitive taken before parent adoption. The grammar retains the exact child and result slot until its own consumption observation; a wrapper throw after child mutation preserves that same slot/serial and first raw fault. Scalar arithmetic profiles still distinguish UI finite/-zero rejection from generic PACK IEEE behavior; no codec-wide float ban is introduced.

## Construction, Aliases and Close Ordering

The same actual neutral ledger/admission cell funds the typed field owner. No allocation is funded by the old payload312 or by R211's320 fixed subset.

Before a fallible typed-field shell constructor, both `payload.typedField` and the original pending `entry` must hold the exact admitted state. The original neutral result cell must retain record/admission results across outer wrapper throws. A field-state shell, reader receipt, witness and child slot each install into that state before finalization/exposure. Foreign payloads, records, readers, receipts and retired/rebound fields reject before spending.

Close is original-parent driven after operation revocation. First stop source consumption; cancel/discard the same held receipt/latch without claiming copied/parsed/EOF progress. Then detach the original reader consumer binding, close the reader's page alias and observe its terminal ownership, retire active scalar/frame/symbol/output children through separate child and wrapper turns, establish genuine field-body emptiness, and clear both payload and pending state aliases in a separately granted unlink before record refund. Preserve the R204 missing-unlink refusal and before/after exact-fault laws in the later grammar-owned model.

If an old source/body fault remains, retain its exact arbitrary root in the charged original state/cell/parent. No Error/String conversion, empty placeholder, guessed rollback or blanket refunded-domain proof is proposed.

## Fixed Inventory Delta — Not an Admission Certificate

R211's19-word FieldDecodeState omitted a direct reader and receipt reference. A field-owned mandatory transaction needs those two real words:

`payload, facade, cell, record, witness, wire, scalar, typed, output, phase, profile, closing, failure, progress, nextChild, expectedResultSerial, fieldLength, parsedOffset, pending, reader, receipt`.

Under the current logical16+8n model:

| Candidate fixed record | Bytes |
|---|---:|
| FieldDecodeState21 words |184|
| Field facade1 word |24|
| Field progress1 word |24|
| Field witness2 words |32|
| Original seven-word pending slot |72|
| Original field receipt8 words |80|
| Fixed domain subset6 records |416|
| Neutral record264 + admission296 |560|
| Fixed intrinsic+domain subset |976 /15 slots/15 owners|

The8 receipt words remain reader,consumer,phase,serial,kind,items,bytes,value. A scalar child must not independently charge or mint the same receipt again. Moving receipt ownership would require an explicit scalar-catalogue semantic amendment and original dependency-close proof; no subtraction from the current proposed992 or automatic quota approval is made here.

This976 subset does **not** fund the resident reader/page/storage aliases, scalar state, existing wire31/typed Builder6 state, explicit container frames, symbol/text/index storage, Surface output, final typed result, unknown faults, or concurrent old/new persistent roots. Every such owner still needs its exact declared record and strong parent before construction. The current32MiB composition cap is unchanged. No whole8MiB input accumulation or1536-byte-per-page retained full field is proposed.

## All Nine Source Dialects

| Source opcode | Original field | Existing semantic route | Proposed distinction |
|---:|---|---|---|
|0|node|Typed node + strict PACK bridge|Whole-node typed authority|
|1|component|Typed component + strict PACK bridge|Includes Surface semantic bytes, not raw PACK-as-Surface|
|2|layout|Typed layout + strict PACK bridge|Exact declared variants/defaults|
|3|activity|Typed activity + strict PACK bridge|Activity and disabled together|
|4|children|Current RetainedUiChildIdsCursor8 fields|Canonical list count/u64 items requires its own byte grammar; not PACK bridge|
|5|style|Typed style + strict PACK bridge|Exact declared defaults|
|6|accessibility|Typed accessibility + strict PACK bridge|Exact optional fields|
|7|bindings|Typed bindings + strict PACK bridge|Owned list and typed entries|
|8|menu|Typed menu + strict PACK bridge|Owned optional/list nested values|

Opcode4's canonical content declaration says list<u64>, not a whole BigUint64Array. Its canonical count and item ULEBs, truncation/overflow/extent, and the renderer's existing safe53 rejection require explicit bytewise state. The current128-slot constructor limit is an implementation/admission fact, not a new wire-format truncation rule. A list count must be checked against the actual owning host/domain limit before storage admission; never silently clamp or infer a new128/4096 wire bound.

Node-internal `children` in the existing TypedBuilder is an already decoded JSON/PACK array route. It must not be confused with source opcode4 framing. Opcodes9/10 have no variable semantic field and must not acquire a fake decoder.

## Required Language-Neutral Cases — Desired, Not Executed Here

The first real grammar transaction declaration must preserve the existing43 scalar values and113-call inventory while adding:

- Genuine original Field→payload→typed owner→reader→receipt positive chain; equal-metadata foreign Field, same-lifetime foreign payload, reflected/proxy facade and stale settled field refusal before allocation.
- Exact grammar-selected profile, wrong-profile scalar child and stale child result serial; no public request can choose a profile.
- Structural outer0x11 split from following value tag; all16 restricted current strict-bridge cases from R216 stay independent of generic Store decoder acceptance.
- Symbol-table count/text split across the single256-byte window; current page retirement lets the next page arrive before full-field EOF.
- Object keys, discriminant-after-payload and nested container frames with separately admitted variable owners; no Object.keys/whole-object compatibility copy.
- Source EOF versus awaiting page versus awaiting seal; structural/truncated scalar faults keep exact source offset and original receipt.
- Prepare/read/commit/observe/apply/settle/observe prefixes with before/after actual throws, zero/short grants and no wrapper work appended to a full-grant child.
- Cancellation at uncommitted byte, committed byte, scalar latch, active page alias and source backpressure; exact two-alias unlink refusal before refund.
- Opcode4 count/item boundary splits, safe53 maximum/overflow and canonical/noncanonical ULEB, separate from PACK bridge cases.
- Typed Surface numeric-byte output and prepared scene ownership, keeping its backing distinct from the incoming PACK transport page.
- All19 authored fixture laws and the two actor opaque copy goldens retain their own actual producer/page/receipt observations; no fake scalar consumer substitutes for arbitrary octets.

Ajv and Immer remain the closed contract/model oracles; Buffer/TextDecoder and existing @webassemblyjs/leb128 independently check scalar encodings. Native Store's generic fallback Null is not used as a strict UI acceptance oracle. These are proposed test requirements, not a newly executed parser suite.

## Fresh Source Readback

Single capture only, not a pre/post execution certificate:

- typed48495B `af27ecc7b6c7f6f5bd676edf86be7e2d73ee38aacba4a196e285403555ed98e9`
- wire17634B `3ac49d7eb43a5db72acfea50b58709769f753767f0cd3cce92e0621cde51e30d`
- resident140630B `fe78bb744ff06b7927258afc90d9055dc7471ab9d0b9fffce88cd9e9e0fa2b27`
- source input30567B `5edeb104796ee6c8231bc87648a447cb34fc13e5849a768fad8e78f02165cd51`

The initial combined source display was truncated; narrower ChildIds/reader admission/private Field reads followed. No source was edited. The next approval needed is the semantic reader/grammar child contract and its complete allocation catalogue, not an accessor swap or permission inferred from scalar source-oracle3.
