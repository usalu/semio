# Resumed Current Artifact IO Audit

Read-only bounded audit on 2026-10-09. No production edits, builds, or runtime tests. Source observations below supersede the older Draw foreign-control report only where explicitly stated. Existing report receipts were read; they are not new executed receipts.

## Priority 1: Complete Original Draw And Shared Route Receiver

`🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🦀️.rs:2015,2022,2138,2611,2620,2787–2814` now carries explicit `IoRunControl` through Deserializer, serializer, IoEntry, route execution and controlled native snapshot output. The old report asserting a payload-only entry pointer is stale. `🏪️store/📦️codec/📸️native-snapshot/🦀️.rs:15–17,21–45` establishes a separate explicit native factory interface; Draw binary snapshot implements it at line 56. This is an actual source advance, not runtime acceptance.

The actual registered SVG implementation still defeats that control. `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs:18` receives original control, but line 21 shadows it with a new 72-byte encoder and an always-true callback. Lines 23–25 instantiate/run SvgImportJob without passing original admission/cancellation or retirement. Signature conversion alone therefore leaves the actual foreign codec uncontrolled.

Shared native SQLite still has a split authority chain: io root line 2629 receives native and SQLite controls, but line 2647 calls `run_snapshot_hop` with SQLite alone. `run_snapshot_hop:2571,2585,2604` delegates native export/import to providers without receiving original native controls. Close provider/factory propagation rather than synthesizing local controls. Direct original Plugin WIT receivers at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:42719,42737` still call the old `(route,payload,limits,always_true)` signature. These are concrete stale consumers of the new shared API, not merely an untested possibility.

Execute the full original Draw source/native/whole receiving rosters after porting those consumers. Preserve original source snapshots and all fixtures; do not broaden all ArtifactPack implementations blindly merely to satisfy the smaller actual snapshot registry. Existing reports establish scoped Draw source/native/TypeScript acceptance only; no original whole native Kernel/Draw host acceptance follows.

## Priority 2: Implement Missing Native Forwarded Allocation Producer

`🧰️framework/🔨️modules/🌱️value/🛫️encode/🛂️allocation/🧪️tests/🦀️.rs:2,11–14,23–24` already demands NativeEncodeAllocation, NativeForwardedEncodeControl and NativeEncodeControl::new_forwarded. Current producer `…/🛫️encode/🦀️.rs:12,25–27,49` has only callback/ceiling state, ordinary new/resume, and charge. No forwarded API or original preallocation recipient is present in that source. Small charges can bypass callback after started; progress cannot implement independent preallocation admission. Complete consuming forwarded continuation that retains the actual recipient across pause/resume, finite cumulative ledger, original cancellation, and Send-capable allocation port. Run the registered neutral/Serde law; no acceptance inferred from startup logs or absent processes. Process inspection found no actual process matching the cited 98744/10615 names; session IDs are not OS PIDs.

## Priority 3: Retained Native Factory Retirement And Error Authority

New native factory `🏪️store/📦️codec/📸️native-snapshot/🦀️.rs:23–30,38–45` constructs spec/record/body temporaries and returns through ordinary local lifetime. It exposes no explicit retirement recipient in this factory interface. Audit actual RecordValue/spec retirement on success and every refusal against original decoder/encoder recipient; do not declare closure merely because byte charges pass. This is a concrete missing-proof frontier, not a measured runtime allocation failure.

Shared io root `:2105–2108` retains `text_refusal` with synthesized unlimited encoder and always-true callback. Original child archive path `:2063–2067` decodes/folds without caller controls, and `:2038,2043` clones archive members/slots into ordinary collections. These remain actual original composed-content entry points requiring source-linked admission/retirement investigation.

## Current Foundation Floor And Scope

Latest ResumedAuthoritativeState/root receipt reports retain whole Flow/Kernel RED 512 consumer errors; no later whole native Kernel pass was found in the read reports. Full Scene receiving current4 session73973 executed all195 laws:192 pass/3 fail/0skip. Its three genuine failures remain constructor refusal64 unreceipted bytes, component birth8256 versus8192, and zero-index nonempty wire/point Schema refusal. Do not treat earlier Scene235/Value197 scoped receipts as superseding this later full receiving failure.

Root latest full React33469/current6 is reported716/716 GREEN, preserving714 original laws and2 fixture-owner laws. Root native27471/current17 is8/8 GREEN; strict7-source typed receiving74097/current1 is deadline RED with no diagnostics before deadline. New ordered-map retirement change needs a new relevant runtime receipt. These are report-derived facts, not tests executed by this audit.

## Coverage Blind Spots

Source ownership census establishes declaration ownership, not caller control, temporary retirement, whole native compilation, or pure semantic typed facts. Draw original SVG body above proves a signature census can pass while controls are shadowed. Shared SQLite branch and original Plugin WIT calls need dedicated direct boundary witnesses. New native factory scoped laws must observe physical allocation/release and refusal/cancel preservation, including every temporary and diagnostic. Older MP3 raw metadata audit is historical; current typed metadata receipts must be checked before reclassifying its former findings as current defects. This audit intentionally did not rescan all230 ArtifactPack owners or ignored outputs.
