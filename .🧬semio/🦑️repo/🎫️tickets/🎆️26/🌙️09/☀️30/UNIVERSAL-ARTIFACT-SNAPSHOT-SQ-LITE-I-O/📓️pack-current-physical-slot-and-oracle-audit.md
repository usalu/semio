# Pack Current Physical Slot And Oracle Audit

Read-only current source audit. No Cargo, Source execution or production edits. Parent's authentic kernel baseline fd83de53-6f50-4074-ada5-07e61012a813 measured3 pass/1 fail,1278 outside, with decoder admitted1665/requested1049. This report does not infer unchanged replay results.

The two facet roots, relative to `/Users/ueli/Documents/semio`, are `🧰️framework/🔨️modules/🎒️pack/🌱️value` (framework) and `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value` (actual kernel).

## Mounted Pair

Framework `🦀️.rs:2027` and kernel2037 now charge `count * size_of::<T>()`, checked, before Vec::try_reserve_exact(count). Inline symbols framework3200/kernel3210 charge `count * size_of::<String>()` before reserving their exact slot count. This removes the fixed32 symbol credit and minimum64 generic slot credit, and distinguishes Array<DslValue> from Object<(String,DslValue)> physical storage. No target ABI size is assumed here.

Shared DecCtx is used by ordinary and controlled paths. Ordinary ValueMaterialization framework1977–1987 starts a fresh per-call counter. ControlledMaterialization2006–2015 routes the same charges to the exact supplied NativeDecodeControl; its maximum check uses that controller's cumulative owned_bytes, not a fresh child ledger. Literal symbol copies still charge actual UTF-8 bytes separately. Thus these storage changes affect ordinary decode_value_record_body_exact and generic record/document decoding as well as direct intrinsic controlled decode. The borrowed encoder's wire grammar and public Value occurrences are not altered by this slot repair.

Generic record_slots framework2028/kernel2038 already charges size_of<(u16,FieldValue)> for actual slot storage. This should not be changed back to a blanket64 credit. Ordinary generic schema factories, chunk/expression paths and retained cursor ownership have separate scopes; selected intrinsic helpers do not prove every generic branch.

## Actual Allocator Law

Framework `🧪️tests/🎞️intrinsic-media/🦀️.rs:4–37` is mounted into the actual kernel selection by Root. It constructs the full neutral specimen plus accepted deep arrays outside measured intervals. Actual encoder17/decoder27 are observed through crate::test_allocation; ordinary neutral comparison occurs afterward. It asserts controller debit equals requests, exact-request replay succeeds, request-minus-one refuses typed OwnershipLimit, and repeated same-controller calls exhaust a2*requests-1 allowance. The unchanged depth and complete occurrence/word laws remain present.

Actual kernel allocator authority is `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs:32–38`, mounting `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs`. This observer records each alloc/alloc_zeroed layout request and full realloc request, includes nested current-thread scopes, restores state on unwind, and uses existing HeapWitness. No second allocator or ABI oracle is necessary. It measures cumulative requests, not live occupancy or deallocation work. The short/cumulative refusals are not themselves observed for admission-before-allocation, and their first results are temporary; this law is not a retained lifetime/drop proof.

## Concrete Stale Neutral/Source Assumptions

OS facet `📜️script.ts:79` still charges every collection count*64n, and92 charges symbol count*32n in proveWireValueMaterializationFixture. Both facet `🧫️fixtures/🧮️wire-materialization/🔣️.json` still declare:

| Cases | Former exact accepted/refused ceiling |
|---|---|
| interned-repeat-at-limit / over-limit, lines13–22 |352 /351|
| array-slots-at-limit / over-limit, lines37 onward |128 /127|
| map-slots-at-limit / over-limit |131 /130|

These boundaries encode former logical credits, not platform-authored physical backing. The independent Source script validates its own fixed-credit outcome102. For accepted cases105 it compares TS decoded output, without invoking actual Rust bounded materialization. Rejected cases do not execute the actual Native decoder. It can therefore remain internally consistent while contradicting current Native physical admission.

Preserve platform-neutral full wire/literal/order/depth/framing expectations. Separate neutral logical byte/count limits from target-specific physical storage. Derive Native exact/subexact physical ceilings from actual existing allocator observation or authored native size_of arithmetic outside measurement; do not put one machine's slot byte sizes into the language-neutral fixture, restore overcharging to satisfy stale fixtures, or claim JSON/TS accounting proves native backing. Keep independent AJV/BigInt/UTF-8 readback as independent semantic evidence.

Search of current Pack Rust/TS/JSON found no other explicit count*32/count*64 slot-credit oracle. Unit test occurrences of64 include depth limits, chunk/frame sizes and fixed allowed budget ceilings; they are not automatically old slot-credit assertions. Retained unit laws request exact bytes dynamically using next_allocation_bytes and reserve_allocation, and should retain their own ownership contract. Generic refusal law record(...65,64...) tests the unchanged generic record depth ceiling, not intrinsic frontier or physical slots.

Whole-module Native, independent Source alignment, refused-operation allocation observation and terminal retirement remain separate required evidence. No new runtime success is asserted.
