# Scalar Construction Alias Correction and Reader Callers R200–R204

## Outcome

R200 reproduces seven desired-law failures in the frozen R199 construction close plan: prefixes7..13 refund688 while payload.scalar and pending.entry still refer to the original state. This is an executed declaration-model RED, not an executed runtime bug.

R202 passes the corrected exact-identity construction model. R204 executes all four frozen controllers through Nx and exits0 at 2026-08-28T04:40:29.964Z–2026-08-28T04:40:30.497Z. All seven complete source identity/hash tuples match before/after. The exact command, programs, complete output and captures are retained in [R204 evidence](🧪️renderer-scalar-alias-r204-2026-08-28.json), [controller manifest](🧪️renderer-scalar-alias-r204-controllers-2026-08-28.json), and the ticket-only [replay controller](📄️scalar-decoder-declaration/📜️script.ts).

R201 is a separate oracle harness failure: comparing an Immer draft proxy with an original plain-object identity inside the recipe failed. R202 uses immutable non-draftable identity objects; exact identity is checked before/after transitions, not replaced by metadata equality. The R201 raw output remains retained.

No production source, canonical schema, API, price, caller, launch registration or native target was changed. The new ticket replay controller has a30-second bound per frozen child program and uses the existing Nx project; it is not a collected renderer test or a new runtime dependency.

## Exact Alias and Ownership Correction

The existing32-word state must initialize its original payload/cell/record references before installation. The same state is then strongly linked in BOTH original pending.entry and payload.scalar before any fallible shell/finalizer. Prefix13 has released the pending slot; cancellation first recovers the same original state and its original resource references from payload.scalar.

The64-byte close capture establishes original pending cell/record/witness authority before the272-byte body-clear turn. Body clearing does not implicitly remove external state aliases.

One new separately granted phase, `unlink-original-state-aliases`, is inserted immediately before record-begin for prefixes7..13. It costs1 item/64 work bytes and refunds0. It verifies exact original payload.scalar, pending.entry, cell and record, body emptiness, and the existing body proof when a facade exists. Only then does that one bounded private turn clear both state aliases and set the existing pending phase to state-unlinked. Zero/63-byte grants and foreign/missing/one-sided identities leave every pointer and counter unchanged.

After this turn, original pending cell/record/witness ownership remains. Prefix7 uses the recorded never-installed-facade frontier, not a negative matcher. Prefixes8..13 use the same preadmitted witness and exact shell detachment/observation. Record refund refuses either held state alias, absent unlink frontier, live body, wrong cell/record or still-linked record shell. Later resource-result alias, cell and parent-observation phases remain separate.

No new field, pointer, slot, witness, record or resident credit is added. Proposed decoder992/14/14, payload+8 and reader+16 are unchanged. Construction prefixes7..13 each add one64-byte work turn. The existing full-close64-byte unlink phase is clarified to clear both aliases; its pending controller remains through cell/record/witness, not pending.entry. Full-close22/2048 and reader-ready37/3440 are unchanged.

| Completed construction prefix | Close turns | Work bytes |
| --- | ---: | ---: |
|0|1|64|
|1|5|552|
|2|5|552|
|3|4|488|
|4|4|488|
|5|7|880|
|6|7|880|
|7|9|1216|
|8|13|1440|
|9|13|1440|
|10|14|1504|
|11|13|1472|
|12|13|1472|
|13|13|1472|

The executable exact model tracks payloadScalar, pendingEntry, pendingCell, pendingRecord, stateCell, stateRecord, witnessState, recordShell and cellResult independently. All use original identity objects; foreign objects are different identities. The two state aliases and variable body are empty before record refund; resource-result and body-empty witness identities retire in their later separate phases. Every clean terminal empties all tracked aliases. The existing witness/neutral observation may temporarily retain only an already body-empty identity facade; this is not physical-GC proof.

Before/after every phase, the actual first fault object remains exact. After-unlink faults retain the original pending cell/record and observed unlinked phase. After-child-refund faults remain under the original proposed880-byte payload charge. The model rejects forward retries after fault and makes no arbitrary-root disposal or whole-parent terminal claim.

## Actual Controller Census

-43 original values remain byte-for-byte unchanged:25 accepted/18 rejected; leb128, Buffer and TextDecoder oracles.
-8 fixture semantic/shape negatives plus2 serial-domain negatives.
-91 closed receipt traces/2238 rows and matching Immer transitions.
-40 unchanged original child-close suffix checks.
-14 construction prefixes/121 declared close steps;242 zero/short and242 before/after fault checks.
-Exact alias model:121 transitions,139 hostile/refusal cases,242 zero/short cases,242 original-fault cases,7 unlink replay refusals and7 omitted-unlink refusal probes. These additional counts are distinct from the earlier arithmetic checks, not extra runtime tests.
-7 maintenance forwarding cases and16 explicit reader construction/binding phases.

## Complete Resolved Caller Inventory

The read-only TypeScript checker loaded the actual React tsconfig and406 non-declaration source files. It resolved113 direct call sites and113 corresponding property references for the three exact declared methods. There were zero resolved extracted method references and zero matching spyOn-method indirections. Five selected source/config hashes match before/after the inventory. This is source reachability, not an execution count.

A repository-wide authored TS/TSX symbol scan, excluding dependencies/tickets, found only the same resident, pages, UiDocumentStore, ShardClient and Kernel input files. Kernel input contains imported/binding types but no calls to these three methods. No extra caller outside the selected graph was found. The full program, every call's exact path/line/column/enclosing law/text, references and hashes are in [R203 caller inventory](🧪️renderer-scalar-reader-callers-r203-2026-08-28.json).

| File | beginReader | builder.beginRead | reader.advance |
| --- | ---: | ---: | ---: |
|Paged builder|1|0|0|
|ShardClient|4|2|10|
|UiDocumentStore|14|13|69|
|Total|19|15|79|

The only production forwarder is pages:196 `builder.beginRead(grant) -> payload.beginReader(builder,grant)`. Resident:489 implements the latter; resident:754 implements reader.advance. Both old unowned entry points must disappear in a future atomic mandatory-consumer/receipt cutover, with no optional arguments or compatibility overload. The scalar-only proposed callable is not permission to cut those callers over now.

### Law-by-Law Sites

Each entry below lists method and line:column; exact full paths/text are in the JSON.

-Paged builder / beginRead: beginReader 196:227.

-ShardClient / ShardClient captured return authority / OwnedKernelReturnInput consumes only privately copied bytes and retains the containing raw page: beginReader 3485:25, beginReader 3486:87, beginRead 3488:16, advance 3488:132, advance 3490:66, advance 3498:73, advance 3501:87, advance 3520:58, advance 3525:131.

-ShardClient / ShardClient captured return authority / OwnedKernelReturnInput stops at the exact copied page boundary without fabricating a next range: beginReader 3555:23, beginReader 3556:85, advance 3559:64, advance 3568:73, advance 3571:85, beginRead 3583:128, advance 3583:208.

-UiDocumentStore / TypedWire / OwnedPagedCopy copies the genuine first fragment into admitted fixed pages and strongly owns the sequential reader: beginRead 664:107, advance 664:202, advance 665:690.

-UiDocumentStore / TypedWire / OwnedPagedFault preserves child refusal and raw over-grant accounting without advancing the producer: beginRead 674:214.

-UiDocumentStore / TypedWire / OwnedPagedFault retains a registered reader after an exact page read throws without skipping the failed byte: beginRead 679:61, advance 679:610, advance 679:698, advance 679:755, advance 680:213, advance 680:364.

-UiDocumentStore / TypedWire / OwnedPagedCancel retains every first-fragment cancellation prefix and distinguishes copied from detached input: beginRead 701:182, advance 701:341.

-UiDocumentStore / TypedWire / OwnedPagedContinuation completes only after the original field consumes its released range: advance 722:357, advance 723:505.

-UiDocumentStore / TypedWire / OwnedResidentReaderConstructor keeps the preinstalled reader and exact finalization fault charged: beginRead 734:55, beginRead 736:20, beginRead 737:14.

-UiDocumentStore / TypedWire / OwnedResidentPool reserves shared pages before allocation and returns credit only after final alias retirement: beginRead 757:217, advance 757:383, advance 757:471, advance 758:85, advance 758:310, advance 758:425, advance 758:618.

-UiDocumentStore / TypedWire / OwnedResidentPool roots abandoned reservations and delays the exact instance witness for live read aliases: beginRead 772:217, advance 772:345, advance 772:433, advance 772:517, advance 772:731, advance 772:850.

-UiDocumentStore / TypedWire / OwnedResidentPageProducer admits its original page through the shared parent before allocation: beginReader 961:197, advance 961:262.

-UiDocumentStore / TypedWire / OwnedResidentPageProducerWindow copies two original windows with separate scalar grants before source EOF: advance 971:9, advance 971:104, advance 971:260, advance 971:408, advance 971:508, advance 971:649, advance 971:749, advance 971:825.

-UiDocumentStore / TypedWire / OwnedResidentReaderPageFence invalidates the exact sealed page when the reader captures retirement: advance 1115:314, advance 1115:402, advance 1115:452, advance 1115:545, advance 1116:14.

-UiDocumentStore / TypedWire / OwnedResidentReaderBindingFault keeps exact before and after detachment failures charged: advance 1131:697.

-UiDocumentStore / TypedWire / OwnedResidentReaderRevocation preserves the original read alias and parent close after operations revoke: advance 1145:305, advance 1145:393, beginRead 1146:108, advance 1146:169.

-UiDocumentStore / TypedWire / OwnedResidentReaderQuarantine preserves the original parent failure across reads and binding close: advance 1152:195.

-UiDocumentStore / TypedWire / OwnedResidentReaderAdmission installs the exact reader before EOF without charging another page or owner: beginReader 1158:159, beginReader 1160:157, beginReader 1160:345, beginReader 1161:224, beginRead 1161:330, advance 1161:425.

-UiDocumentStore / TypedWire / residentReaderFixture: beginReader 1166:67.

-UiDocumentStore / TypedWire / OwnedResidentReaderRetirement uses its exact original binding witness and rejects ambiguous replay: beginReader 1171:200, beginReader 1171:310.

-UiDocumentStore / TypedWire / OwnedResidentReaderWindow consumes two original byte pages before source EOF without retaining the whole extent: advance 1177:16, advance 1177:158, advance 1178:72, advance 1179:16, advance 1179:163, advance 1180:82, advance 1180:234, advance 1180:335.

-UiDocumentStore / TypedWire / OwnedResidentReaderHeld preserves original handle identity throughout its admitted read phases: advance 1188:164, beginReader 1189:14, beginRead 1189:120.

-UiDocumentStore / TypedWire / OwnedResidentReaderCancellation retires every admitted prefix under the exact original parent: beginReader 1194:180.

-UiDocumentStore / TypedWire / OwnedResidentReaderFault retains exact shell and binding failures without replacing the original admission: beginReader 1204:125, beginReader 1207:22, beginReader 1208:16.

-UiDocumentStore / TypedWire / OwnedResidentReaderBodyFault preserves original alias and byte offset after read or detach wrapper failure: advance 1215:499, advance 1215:587, advance 1216:52, advance 1216:126, advance 1219:22, advance 1219:75, beginReader 1219:262.

-UiDocumentStore / TypedWire / residentCopiedRangeFixture: advance 1274:706, advance 1274:794, advance 1274:907, advance 1274:1061, advance 1274:1194.

-UiDocumentStore / TypedWire / OwnedResidentCopiedRange admits one original copied token and observes every real source byte before EOF: advance 1278:1480, advance 1281:639, advance 1282:571.

-UiDocumentStore / TypedWire / OwnedResidentCopiedEmpty obtains actual zero-byte source completion without allocating a destination page: advance 1299:329, advance 1299:743.

-UiDocumentStore / TypedWire / OwnedResidentActiveInputWindows closes latched and written bytes behind the exact reader and page aliases: beginRead 1332:375, advance 1334:417, advance 1334:505, advance 1334:562.

## Opaque Byte Laws: Required Semantic Decision

Fresh source inspection confirms ShardClient:2996 generates `(index * 37 + 11) % 256`. The two laws beginning at the reader calls3485 and3555 assert the exact opaque payload bytes over windows, including the8193-byte source whose next range is unavailable. Those bytes are not accepted by the five scalar profiles; the first0x0b is not even an admitted ui-value tag. Retaining the original goldens and all79 raw-byte call semantics cannot be accomplished by merely adding scalar consumer/receipt arguments.

Decision for the next proposal: keep these two existing transport-copy boundary laws and their exact opaque byte goldens unchanged. Do not add a raw scalar profile, weaken scalar validation, substitute valid scalar bytes, mint a test-only consumer or leave an optional unowned read path. The current scalar-only mandatory signature is consequently **not ready for global caller replacement**.

The preferred production design to evaluate separately is a genuinely owned, schema-neutral byte-transfer consumer for retaining opaque wire-field bytes into their actual destination owner. It is a transport operation, not scalar acceptance or a diagnostic no-op. Its real original source-reader/receipt transaction would be exercised by those copy tests, with independent Buffer assertions against the actual destination or test observation of issued production receipts. A subsequent scalar/schema decoder remains a separate consumer and cannot claim opaque transfer as successful parsing. The transfer owner must use bounded source-read/receipt and destination-write/settlement turns, one bounded live window, exact original destination/parent registration before exposure, and private two-sided close. It requires its own schema, field/witness census, resident charge and runtime fault/refusal tests before approval; decoder992 cannot fund it.

This proposal entails reviewing a semantic-neutral reader transaction receipt bound to a closed set of genuine production consumer classes (scalar and field transfer), not a public structural reader callback/interface. No names, union, new consumer, record, price or positive constructor are released here. If that separately inventoried production transfer owner is not approved, the honest outcome is to defer the global reader API replacement; it is not safe to keep the existing copy tests green using a fake scalar consumer. The current production reader path remains unchanged and testable while that decision is pending.

The other96 UI sites also include byte-preservation, active-input cancellation, late alias reads, saturation, constructor faults and foreign/replay laws. Their exact goldens and effects must be mapped to the genuine production consumer used by each case; this inventory authorizes no broad textual replacement. Scalar arithmetic43 remains a separate acceptance cohort.

## Reproduction and Boundary

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=@semio-tech/framework-renderer-react -- bun '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📄️scalar-decoder-declaration/📜️script.ts'
```

The frozen controller manifest supplies the exact four programs and seven expected input hashes. The ticket controller verifies full file identity/hash before and after. Canonical taxonomy and the eventual registered renderer selector remain pending with the coordinator. No semantic parser, variable symbol/container/Surface owner, source InputAck, native publication, renderer mount, current full renderer or strict gate is claimed by this declaration-only packet.
