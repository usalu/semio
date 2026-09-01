# Whole-Field Grammar Owner: Initial Source-Based Proposal

Status: ticket-only design and read-only source census. No production, frozen R204 JSON, canonical declaration, API, price, caller, launch, parser or native-build change. R210 is the separate provenance proposal. None of the desired cases below ran against a new decoder.

## Decision

Use the existing exact `OwnedUiResidentPayload` as the parent and evolve its existing semantic destination, `RetainedUiTypedCursor`, into its privately admitted whole-field child. Continue using `RetainedUiWireValueCursor` for the existing UI PACK bridge/DSL-value grammar and the existing typed rules for normalization. Do not add a second opaque-transfer consumer, a second PACK codec or a whole-field byte buffer.

The payload already owns the original private source Field association, the original builder, reader/page rosters and one seven-word pending child slot. Source Field metadata is usable only after the original private Field↔Payload and lifetime/activation ownership checks; copied `{opcode,name,receipt}` is not admission authority. A new whole-field child needs its own declared record, strong parent registration and child-admission slot. Current payload312, the scalar proposal992, and the prospective payload+8/reader+16 do not fund that graph by implication.

The scalar proposal is an algorithm/receipt source oracle, not sufficient whole-field provenance. Its five profiles remain unchanged. Its “trusted exact parent” must become this concrete grammar owner, with profile selection determined by the original field and current grammar phase. The global mandatory-reader cutover must wait for this ownership join, variable-child admission and actual transport-law migration. No optional old `advance` overload or test-only positive consumer is proposed.

## Exact Existing Grammar Authority

Source files below were inspected, not executed as new runtime features:

- `kernel/📤️return/📦️content/🧬️wire.json`: canonical outer record/field dialect. Each UI packed field is one independent `store::pack_rt::encode_wire_value` document; length is not permission to allocate; no whole packed field allocation; page ACK and UI validation/publication remain distinct.
- `kernel/.../content/🟦️component.ts:186–233`: original header chooses opcode, node and field name; opcode4 finishes after node and leaves the list count inside the field, while packed opcodes check the exact byte length.
- `kernel/.../input/🟦️component.ts:102–186`: original Field has private source, builder, payload and two-way detachment associations; `matchesOwner`, `matchesResidentPayload`, `matchesBuilder` are direct private checks. `value` is original frozen field metadata, not a caller profile.
- OS store `🏪️store/🦀️component.rs:4573–4651`: `encode_wire_value` makes a synthetic record with field1 of `Shape::Value`, then invokes `os_pack::encode_record_body`.
- OS PACK `🎒️pack/🔢️value/🦀️component.rs:2148–2196`: container-less format is symbol count, length/UTF8 entries, then record fields. This is NOT the SPK file header/segments/footer inspected in framework PACK format. No SPK container parser belongs in the UI field path.
- Same PACK source: tags21–44, interning160–205, `encode_dsl_value`531–565. DSL values use null12, bool1/2, f64 5, symbol6, inline string7, list0C and map10. Object keys are forced inline and byte-lexically sorted. Strings are independently interned; each field gets its own table.
- Existing native `RetainedValueCursor`655–1194 has explicit Field-versus-DSL context, one-byte pending input, canonical u64/UTF8 primitives and a typed token stream. It begins at record fields, not at the preceding symbol table, and it is not a ready UI TS consumer. Its wider field tags and limits must not become additional UI DSL tags. Its preallocated stack, native close/accounting and output interpretation are separate native work; no native execution or admission proof is claimed here.

Exact source-selected dispatch:

| Original opcode/name | Semantic dialect / existing typed profile |
|---|---|
| 0/node | UI wire-value bridge; node |
| 1/component | UI wire-value bridge; component |
| 2/layout | UI wire-value bridge; layout |
| 3/activity | UI wire-value bridge; activity wrapper |
| 4/children | Canonical list<u64>, NOT the PACK bridge |
| 5/style | UI wire-value bridge; style |
| 6/accessibility | UI wire-value bridge; accessibility |
| 7/bindings | UI wire-value bridge; bindings |
| 8/menu | UI wire-value bridge; menu option |
| 9/remove, 10/setRoot | No field decoder; their original scalar header remains source-owned |

A mismatch of private original opcode/name, field/parent, receipt, activation or lifetime rejects before admission. The original node id stays exact u64 in source metadata. Existing TS safe53 identity admission remains an explicit fail-closed host boundary; this proposal does not silently round or claim full-u64 UI parity.

The generic PACK encoder preserves signed zero and normalizes NaN (value source110–119). Existing UI wire admission rejects negative zero/nonfinite. Keep that distinction; do not impose a codec-wide ban or infer Scene geometry policy from the UI primitive profile.

## Actual Current Census and Missing Ownership

The complete TypeScript AST census, all field names/line numbers and ten inspected source hashes are in `🧪️renderer-whole-field-source-census-r211-2026-08-28.json`. Six TS files were hashed again after the inspection; all six matched. This is not a full-import-tree or runtime gate.

| Existing object | Own words / relevant variable roots |
|---|---|
| RetainedUiWireValueCursor | 31 instance fields; input backing, symbols/edit/reader/old root/retirement, previousSymbol input slice, text, frame, owned list, Surface byte child |
| Wire Frame | 8 words: owner,count,index,key,previousKey,array,bytes,parent |
| Wire Owned link | 2 words: value,next |
| RetainedUiTypedCursor | 9 fields: decoder,builder,program,payload,retirement,closing,failure,phase,profile |
| Typed Builder | 6 fields: owned,bytes,json,children,fields,active |
| Typed JsonFrame | 6 distinct words: index,count,parent,input,output,keys |
| Typed payload Root | 7 words: value,references,owned,bytes,children,fields,kind |
| Typed Owned / Bytes / PayloadLink | 2 words each |
| UiSurfaceByteBuilder / byte Root / ByteView | 6 / 3 / 1 words, plus page-array slots/backings |
| Payload | 21 words, including current builder/reader/evidence/page roots and pending slot |
| Payload reader | 11 words, plus facade/witness and separately admitted intrinsic alias |
| Operation payload builder | 24 instance fields |
| Original source Field / Fragment / Release / InputOwner | 12 / 6 / 4 / 14 words; source-owned, not charged by a future decoder record |

The Typed Builder contains 22 generator-method syntax sites and 19 nested arrow/function-expression syntax sites. These are NOT 22 or19 guaranteed simultaneous allocations. They demonstrate why counting six Builder words is insufficient: generator activation records, captured callbacks, Object.keys arrays, whole value clones and nested builder/link records need their real owners or replacement with explicit frames. Static-block closure syntax elsewhere is likewise not a per-instance count.

Current concrete obstacles:

1. Wire constructor transfers a whole buffer; `#byte` increments its offset directly. Text decoding and key/symbol comparison retain `Uint8Array.subarray` aliases into that backing. This cannot outlive a retired input page.
2. Wire builds a complete generic value graph; typed normalization subsequently clones it into another graph using generator frames and callbacks. A byte-array adapter would preserve the double graph and hidden allocations.
3. Wire fixed-object/map/array finalization and typed `Object.keys`/lists/freeze are not automatically charged by a one-byte input grant.
4. Current Surface detection is a component/node path check; only later typed validation proves actual component kind/schema. An early `doc.bytes` allocation must remain owned even if type validation later fails.
5. Existing catches often convert an arbitrary error into a message/string; this is not fault-root retirement. A live grammar join must retain the exact first raw root in already charged ownership.
6. Existing lower child joins include forced-pending forwarding. Reusing grammar does not certify their refusal/failure or near-grant bookkeeping. The new parent must forward exact child work and observe completion on another grant.
7. Current typed node has seven independent owned fields. A streamed replacement must preserve these captures and unchanged-field identity rather than creating one copied parallel content tree.

## Proposed Initial Source/API Boundary — Not Released

Keep source roles in the existing wire/typed/resident domains; no new grammar directory or runtime dependency is proposed in this packet.

The resident facade would expose a phased `payload.beginTyped(grant)` returning an admission result containing only the genuine `RetainedUiTypedCursor|null`, not a caller-provided profile, field reader callback, record or ledger capability. It resolves the exact original field from private payload state. Repeated calls recover the same admitted pending/root instance; a foreign parent, closing/faulted parent or consumed field cannot allocate a replacement.

The existing `RetainedUiTypedCursor` constructor becomes private exact-parent construction, with parent-only construction/install matchers. Its public work surface remains bounded `advance(grant)`, `beginClose()`, `closeStep(grant)` and read-only progress; no raw byte accessor and no whole-object compatibility converter. A result must transfer into the exact owned operation destination through a preadmitted private handoff; an arbitrary public `takeResult` with caller-loss exposure is not the final live contract.

`RetainedUiWireValueCursor` remains the grammar machine, but its whole-buffer constructor/input representation changes in the future in place. It must not coexist with a newly duplicated “streamed pack” implementation as a legacy mode. Its symbol/stack/text/value events are private owned children of the typed owner; the same typed profile rules consume them. Do not expose structural event callbacks carrying unknown graphs.

For the scalar link there are two separate questions, not a hidden API substitution:

- Algorithm reuse: the five frozen scalar algorithms apply at grammar-determined states: u64 lengths/counts/references before domain narrowing, UTF8 codepoint only within an exact remaining byte extent, UI value tag only in DSL-value context, UI f64 only after actual tag5. Scalar results have their original serial and remain held until the original grammar parent applies and consumes them.
- Structural bytes: the bridge's outer tag11 is deliberately outside the frozen `ui-value-tag` profile. Preserve its existing exact single-byte structural check in the grammar. Do NOT relabel it as an accepted scalar value, add a raw profile, or pretend arbitrary bytes are f64. Consequently the concrete runtime reader binding must be declared as a whole-field grammar/receipt transaction before global cutover; the frozen scalar-only callable spelling alone cannot honestly process every grammar byte. The same one-byte receipt/latch/serial/settlement law can be reused under this one real typed field consumer, with private scalar algorithm state as its child. This is a required separately reviewed semantic API delta, not part of the15 provenance pointers.

No optional scalar-or-raw union or test-only positive consumer is proposed. Exact class/receipt type spellings and replacement of the frozen proposed binding are deliberately not released as production callables here; the receipt still needs one original mandatory consumer and no caller-selected parser. Root review of this real-consumer decision precedes any source edit.

## Initial Fixed-Root Declaration Candidate

This candidate makes the original field owner explicit before detailing variable children. It is NOT an approved allocation price, new source schema or complete decoder census.

Proposed FieldDecodeState words (19): `payload, facade, cell, record, witness, wire, scalar, typed, output, phase, profile, closing, failure, progress, nextChild, expectedResultSerial, fieldLength, parsedOffset, pending`. Field identity is resolved through the original payload; no duplicate source Field pointer/registry. `profile` is selected privately once from original opcode/name. `fieldLength/parsedOffset` are checked u64 values. `nextChild` is a closed discriminator, not a callback.

Proposed fixed companion records: facade(state)24; progress(state)24; witness(original,phase)32; child pending(requestOwner,cell,record,entry,phase,failure,witness)72. Under the existing logical16+8n model the state is168 and this five-record fixed domain subset totals320. Its own neutral record/admission would add their actual imported prices (currently264+296), but that880 subset excludes every wire/scalar/typed/output child, variable storage, closure and reader. It is not a total-fit assertion or permission to reserve880 and construct everything.

The payload requires one strongly owned grammar-root word and a closed pending-slot union extension. Whether that word replaces the prospective unmounted scalar word or adds another word must be resolved in the future semantic declaration; the frozen +8 is not silently reassigned. The mandatory reader's actual consumer/receipt associations must similarly be inventoried. A scalar private child needs its exact original grammar association and admission under the same ledger; no default ledger and no free subdecoder.

Order: reserve root domain and neutral admission; install the same state into original pending.entry and payload's grammar-root BEFORE any fallible shell construction; install shell before record.install/freeze; install progress/witness before their finalizers. Release the parent's pending slot only after original root captures all resource authorities. The root's separately charged pending slot then admits grammar/scalar/typed children and the genuine early reader serially, installing every shell before finalizers. No construction retry may allocate a second root. At every outer after-return fault, the old parent/slot still retains the original state and exact first fault.

The19-word list is an initial root proposal, not a claim that it absorbs existing31+9+6 fields. Full wire/typed state partition, exact child shells/closures, all construction and close grants must be declared and AST-checked before runtime admission. Initial source work must reject missing child admission, not instantiate existing constructors eagerly.

## Variable Ownership and One-Window Progression

The long-term streamed typed path should build one owned semantic representation, not a whole generic JS graph followed by a copied typed graph.

- Symbols: preserve independent per-field table and canonical UTF8-byte ordering. Each symbol text root/chunk/index registration is separately admitted and strongly held. Previous-symbol ordering retains owned text identity, not an input slice. The numeric index's edit, prior root, lookup reader and retirements remain separate exact children with their own grants. A symbol reference captures the actual owned symbol, not an unpriced copied string.
- Text/map keys: incremental UTF8 state produces codepoints; append them to an admitted text owner. Compare key/symbol byte ordering incrementally using owned codepoints/UTF8 projections and exact cursors. A UTF8 chunk is not a free JS string concatenation. Any eventual bounded JS string materialization must be separately charged and retained. Empty string still has real owner/metadata rules; no empty-result success placeholder.
- Containers: each active frame, child slot/edge, member-name/value reference and previous-key owner must be charged before construction. Preserve the current explicit parent-chain grammar but replace opaque generator/callback activation storage with declared closed frame records. A frame remains owned through child-finalization failure. Map canonical byte order, duplicate rejection, depth and item constraints remain distinct from resident capacity.
- Typed contexts: retain schema-selected field destinations and exact defaults/unknown-field policy. Since canonical key order may place `doc` or other fields before `type`, earlier values cannot assume their eventual component kind; they remain in admitted pending semantic slots until the discriminant validates. This is bounded semantic ownership, not permission for an uncharged temporary whole object.
- Surface: only the original component/node path and subsequently validated typed Surface contract can route the decoded array's numeric values into a byte owner. Each value must pass actual UI f64 and0..255 integer checks. This is a semantic destination, not an opaque PACK-byte sink. The source SceneDoc bytes retain their own dialect; parsing all15 host schemas and nested JSON/generic-pack fields remains the separate prepared-scene path.
- Output: the actual typed payload/field roots and Surface backing leases must retain their admitted resources after the input grammar closes. Moving output to an owned operation needs an exact retained destination and private receipt. Closing the decoder cannot refund a record that still funds live output or merely clear its local output pointer.

One256-byte transport window progresses as follows:

1. Original builder copies source bytes into its admitted destination page, using its existing distinct source-read/destination-write grants.
2. Genuine reader is admitted before source EOF. The grammar's original receipt acquires one byte; original cursor commits once; the private latch parses on its own grant.
3. The grammar applies a primitive or stores an admitted variable semantic piece before settling that receipt. If a destination reservation is refused, no source cursor/profile/result advancement is invented.
4. At page end, outstanding receipt/latch/reference obligations settle before reader alias close. Original child close/retirement work forwards unchanged; the following observation gets a new grant.
5. Original page/builder binding retirement releases that window, allowing the next256 bytes while grammar frames/symbols/partial text persist in separately owned semantic storage.
6. The reader reports EOF only from original builder/source completion with exact consumed field length; no page or incomplete source fragment means backpressure, not EOF. Grammar completion additionally requires bridge/stack/type validation and no trailing bytes.

Thus an8MiB transport field does not require8MiB of transport page records at once. This does NOT prove the eventual semantic result/index/text graph fits the shared32MiB ledger. Large legitimate semantic graphs can still exhaust admitted capacity; no cap widening, borrowed UI limit, discarded tail or fake EOF is allowed. The next full grammar declaration must include exact capacity/liveness cases and clean bounded cancellation for this state.

## Desired Language-Neutral Cases — Not Executed

Keep all43 frozen scalar values and113 existing caller inventory separately. New field cases must name actual opcode/name, canonical bytes, expected typed result or precise fault frontier, source/page split schedule, declared budget schedule and original identity trace.

1. Actual setMenu field8 with bridge bytes `0001011112` yields null; same bytes under component are a typed fault, not a profile selected by the test.
2. setStyle field5 with `000101111000` (empty map) yields existing declared defaults. Wrong bridge field count/id/outer tag rejects.
3. Original Field↔Payload foreign/replay/revoked-operation cases; revocation blocks new construction but leaves original close authority.
4. Valid node/component/layout/activity/style/accessibility/bindings/menu from the existing native-owned field grammar; children separately has canonical list<u64>, including safe53 boundary rejection with original u64 retained.
5. Every supported18 component fixture and its existing defaults/unknown-field negatives, plus all7 independent node-field owners. No successful unknown kind or empty fallback.
6. Independently encoded component text long enough to cross256/512 windows; tune a legal field's real serialized text so UTF8 or a length varint crosses byte255. No arbitrary prefix, seek, opaque-byte reinterpretation or parser restart.
7. Per-field symbols: empty table, repeated reference, index boundary, multibyte sorted symbols, duplicated/unsorted entries, truncated/invalid UTF8; second field must not reuse first table.
8. Maps: forced-inline keys, duplicate/descending keys, a UTF8 ordering case differing from JS UTF16 order, discriminant arriving after earlier owned values, nested frames crossing pages.
9. f64 tag/payload split at every byte; generic signed-zero/NaN source bit behavior is tested separately from UI finite/nonnegative-zero admission.
10. Surface0/1/255/256/257/32768 decoded bytes and max+1 rejection, actual type/doc validation, source PACK number tag/value checks, all existing Scene host kinds retained as opaque child bytes until their own prepared owner.
11. Reader held/closing page, no next raw page, no available resident credit, and source fragment incomplete: all preserve grammar/receipt state and do not report EOF or publication.
12. Before/after original reader read/commit, parser mutation, text append, frame install, symbol index publish and output handoff faults. Exact raw first fault is retained; no rewind/retry through a mutation or local string replacement.
13. Cancel every construction/byte/variable/normalization prefix, with either original parent alias still held and near-grant child terminal work. Refund refuses held output/reader/receipt/frame/text/symbol aliases; completion and wrapper observation are separate grants.
14. Existing opaque copy goldens stay transport laws with original writes and genuine intrinsic sealed-page readback from R205. They do not become accepted semantic packets. Existing UI reader fault/window laws move only to genuine grammar-valid cases, preserving original effects/cursor assertions.

Independent oracle plan: exact native `encode_wire_value/decode_wire_value` plus typed native serde fixtures establishes wire/typed parity, while existing third-party `@webassemblyjs/leb128`, strict Ajv, Buffer/DataView/TextDecoder and Immer validate primitive encodings, schemas, expected values and closed ownership traces. No new runtime dependency. Native full typed examples need actual future serialized execution, not a copied source claim. The first missing-method TDD should use the real captured setMenu field, phased parent admission and original receipt binding, then add legal cross-window text before any global reader caller cutover.

## Release Boundary

Only the MD and census JSON were added. No new runtime method/class/decoder/consumer, source fixture, capacity or native output publication was mounted. No whole-field grammar test, current broad renderer test, strict gate, source ACK, final output retirement or heap bound is claimed. The initial root/callable/receipt decision and full variable allocation census require review before schema or implementation.

