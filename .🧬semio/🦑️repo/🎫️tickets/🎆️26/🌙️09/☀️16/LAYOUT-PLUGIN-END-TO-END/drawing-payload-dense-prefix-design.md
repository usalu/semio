# Dense Payload Page Prefix Design

Root requested a genuine ownership representation correction for the existing Worker success law whose entire original lifetime stock is256 items. Current RetainedPayloadBuilder initializes256 None bindings plus one scalar-header turn before any page, Source or close. The dense prefix removes that unconditional work; it does not increase the original stock, quotas, deadlines or turn guards.

## Defining representation

RetainedJobPayload owns fixed inline storage [MaybeUninit<JobPayloadPage>;256] and an exact initialized/live page_count. Only indices below page_count contain valid Page values. The suffix is never read, iterated, dropped, copied as payload or treated as Option. The page limit remains256 and operation bytes remain4MiB.

All invariant-bearing payload fields become crate-private: safe external mutation of page_count or pages would otherwise permit reading uninitialized values. Existing public page/page_count/reader/capacity/identity/typed-transfer ports carry actual observations and ownership. Add a readonly stream accessor only if an actual external caller needs it. The payload crate's existing page-transfer implementation is the defining owner and may use its internal prefix operations; no external unqualified constructor or compatibility layer is needed.

Allocation-free cold Builder construction leaves its original MaybeUninit header untouched. One paid header turn initializes scalar fields, empty ledger and valid uninitialized array storage, then ready=true. No empty Page/Option rows are born or written. This has actual CPU1/items1, copy0, heap0 and depth1; cached/denied0. The old257 turn bound stays an upper bound in the regression, while its implementation-specific exact expectation becomes1.

Remove Builder.initialized and its test-only initialized_bindings observation; initialized live pages are the real payload page_count. Hand-migrate all assertions to actual public page_count/header readiness rather than keep a misleading alias.

## Existing owner operations

Staged commit writes exactly one Page at pages[page_count] only after the existing funded reservation, allocation/retained incoming and publication guards. It then increments page_count/length. This is an ownership move, not backing clone or allocation; existing CPU/item/depth quote remains actual.

Borrowing checks index<page_count before assume_init_ref/mut. Whole-prefix iteration constructs a borrowed slice over exactly that prefix. Last-page close and typed transfer use assume_init_read only at page_count-1, update count/length, then retain/move/release that exact Page under the existing grant. A two-turn transfer may retain its ledger alias before popping; pending ordinal/frontier validation and original backing/ledger identity remain unchanged. A reused suffix slot is written once before returning to the live prefix.

Cancellation closes only real incoming/reservation/staged/committed custody, then the ledger and separate header controls. Empty suffix rows perform no work. Existing ManuallyDrop ledger and explicit Drop emptiness guards remain. MaybeUninit page slots have no implicit Page drop; every live Page must already have been moved/released before empty terminal Drop.

## Bounded current caller map

The conservative Job closure census is generated/drawing-payload-dense-prefix-census.json. Reached real bodies needing handwritten changes are payload-pages-and-ledger; Builder root and staged commit; payload-owner close; original-byte accessor; typed page-transfer. Context only reexports identities/types. No external page-field mutation was found in that bounded map; broader reverse callers and struct literals must be checked before privacy is sealed.

Builder work fixture currently declares bindings256/initializationTurns257. Correct schema to maximumPages256, empty live prefix0 and actual header turns1, retaining257 as maximum initialization guard. Preserve actual null/reservation/source-error/cached/CPU/heap/paid close cases. Other fixed 512/1024 guards remain unchanged upper bounds.

## Neutral regression authored first

Source case/schema are at Builder/📄️prefix/🧪️tests/case.json and case.schema.json, currently unmounted. Cases pin empty header after one real funded turn with no heap/copy, page counts0/1/2/255/256, refusal at257 with exact source retained, dense last-page ordinals, ledger/pointer identity, staged plus incoming custody, actual positive-axis denials and cancellation cuts. The original Worker256-item single-page success law remains mandatory unchanged; no hidden cleanup bank is introduced.

A real public same-package payload test will exercise these ports with existing finite issuer/actual allocator and serde_json reader parity; the actual Worker/BatchHost law remains the genuine Source/host proof. Mount and author the actual owning law before implementation. Root's final Context gate is currently using the existing bodies, so no production or existing fixture has been changed for this design yet.
