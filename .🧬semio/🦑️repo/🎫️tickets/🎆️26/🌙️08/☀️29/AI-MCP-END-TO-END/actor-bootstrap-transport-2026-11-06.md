# Actor Bootstrap Transport

Status: approved architecture and304-byte wire contract; no native qualification. This protocol replaces the actual pre-Source admit_owned transport. It does not represent Source closure or create a Source receipt.

## Namespace And Authentication

SAB1 is accepted only by the actual NativeCore AdmitActorContext startup frontier. Host state binds it to that exact compiled component, native instance/actor identity and bootstrap incarnation. A receipt borrowed from another Core or ordinary semantic frontier is rejected. The wire echoes all nineteen authenticated OriginalJobBudget scalars exactly; operation/generation/epoch alone are insufficient qualification. Guest storage has a monotonic bootstrap sequence, and host state pairs that sequence with its actual Core/incarnation. No new export or public authority constructor is implied.

Approved fixed extent: 304 bytes. Actor/Core identity is authenticated by the actual borrowed-memory transport and expected startup owner, not guessed guest data. The little-endian layout is prefix8 at0, exact B19 at8, actual current-call progress32/work8 at160, observed prepaid progress32/work8 at200, held-return progress32/work8 at240, sequence8 at280, prior physically returned sequence8 at288, status/phase/refusal word8 at296. The prefix is SAB1 magic/u16 version1/u16 extent304. Status and phase occupy one byte each, two reserved bytes are zero, and the final u32 is the refusal-kind code (zero means none); Failed requires a retained full cause, never only that code. B19 uses the canonical scalar order and 0/1 cancellation. Root approved this layout and owns host settlement/handback.

Statuses are SourceAbsent (no admitted Source), Pending (actual bootstrap partials, including a Source that is not yet ready for normal publication), and Failed (a complete original retained cause). No SAB1 Ready, SourceClosed, SourceID, SRE or zero escrow is permitted. The first Source-ready result uses normal SAC3 plus a genuine SRE.

## Actual Owners And Handback

The preborn Entry bootstrap owner retains its exact fixed header backing, physical loan state, current sequence/budget, any incomplete admission owner and any full original failure. Before body effects, the actual bootstrap budget reserves header write work/copy and the exact no-context handback target. A real returned bit is written only by the exact resident-pointer/length handback port. Wrong extent, duplicate return or different namespace cannot reach Box deallocation. Future calls observe only the actual written bit, never infer completion from a flag supplied by the host.

The return reservation must identify exact target bytes, items and work. Header fresh effects, held return credit, and actual observed prior prepaid-return effects must remain distinct. Host settlement requires its actual successful Deallocate witness for the same Core/pointer/extent/sequence and validates observed prepaid effects against that exact earlier hold. Remaining hold cannot be refunded or silently treated as current fresh spend. Header production uses an exact success plan and compares actual effects before exposure; failure retains all partial header/admission owners.

## Finite Admission Debt

Every pre-Source admission step reports its actual effects/work. Earlier bootstrap costs remain consumed from the single finite original grant/work header; a new epoch cannot refill them. Source creation must initialize its treasury from the original authenticated total while retaining all preceding admission debt, then borrow current scopes only below the current monotonic header and scoped budget. In particular, the current code passes a scoped policy to OriginalCallerSourceParent::admit but later borrows using initial_grant; if policy is narrower than the unified header this can reject every real borrow. Source birth must separate its authenticated total from the per-crossing physical birth preflight, without minting a larger grant.

The actual pending-epoch borrow ports can connect bootstrap completion and normal header publication within one call after Source is ready. They do not authorize replaying the external epoch or a new Source. UI registration and bundle/semantic effects remain after actual Source readiness.

## Failure Custody

Failed bootstrap retains the original typed error and all partial context/cancel/payload/Source owners. A scalar WorkLimit or synthetic zero receipt must never replace post-effect failure. A Source-absent failure must either reach a genuine bounded bootstrap failure recipient or remain a terminal retained native failure; it cannot be fed to Source-only SAF1 publication or erased during an ordinary retry. Reusing the existing failure export selectors for AdmitActorContext is possible only with an explicit SAB1 bootstrap branch, actual finite budget and independent decoder/handback contract. No new guessed export is allowed.

## Required Laws

Schema/corpus and independent binary/SQLite oracle must cover every phase, exact B19 echo, current-call versus prepaid effects, denied complete header reserve before effects, partial allocation failure, cancellation, stale/foreign incarnation, wrong resident length, duplicate handback, missing return observation, finite debt through Source birth, and the first genuine SAC3/SRE transition. Native qualification additionally requires real heap/drop witnesses and actual dynamic component execution.

## Host Frozen Bootstrap Custody and Transition

Runtime now exposes with_original_scalar_header only for its actual Source-validated Ready scalar result; it borrows original exported memory without taking the result. Host with_original_bootstrap_parts requires the actual native producer, real ActorContext export and closed SAB1 namespace. Bootstrap has distinct GuestBootstrapHeaderIdentity / GuestFrozenBootstrapBorrow / GuestBootstrapHeaderDeallocateReturned types; no escrow fields or synthetic SRE identity are used. Physical handback proof comes from the real Core Deallocate return and exact original recipient/address/extent.

Normal scalar escrow now accepts actual AdmitActorContext and CloseActorContext results (previous whitelist omitted both). Exact confirmed cleanup exports use a shared runtime classifier: Deallocate, CloseActorContext, CloseActorOutput and failure publication/read/borrow/acks. Fresh Admit/Input/Start/Step/Poll/Restore remain semantic. Language-neutral schema + independent SQLite membership check pass1/4; native classifier law authored, not compiled. Job cleanup ports require Server audit before expansion.

SBT1 is the agreed explicit first-Source transition:200bytes = prefix8 + genuine inner SAC3/SRE144 + actual previous bootstrap returned_sequence8 + observed prepaid progress/work40. Guest private enter_source must prove the preceding real handback; Host must match stored Core/sequence/Deallocate proof and settle the last hold exactly once before journaling the inner genuine SRE. Same AdmitActorContext19 export, same pending epoch; no fake escrow row or credit. Guest owns wire/physical Source transition; Root owns Host ledger/history/Poll mount. Current physical owner and data codecs are authored; callers and native qualification remain incomplete.

Actual reserve arithmetic corrected: completion item count is independent of header words. Extent184 to200 adds2work and16copy bytes, not items. After Server pending_input changes baseline93items/3052work, SBT1 becomes93items/3054work; with32producer items minimum125 fits unchanged128. Recompute from actual function after both changes land; do not inflate grant.

Host ledger plan: retain each issued Bootstrap origin in the real Native History owner; register it before VM just like normal scopes. Fresh = actual minus exactly proved previous prepaid. Record fresh/aggregate and settled state for the exact issuer descriptor. New held1item/2bytes/1work remains separately reserved across actual Deallocate, with previous origin index/sequence/identity and private returned proof. On next SAB or SBT, redeem precisely that old held amount into observed usage and the actual old IssuedOrigin redeemed fields; never let a later SRE origin reuse that scope currency. Normal SRE book holds remain separate from Bootstrap holds, and physical/logical remaining includes both. No SRE book is invented while Source is absent. Terminal close must retain all unfinished holds/causes until actual Core/native witness.

Source cap ownership changes preserve every existing law: six public mock-runtime tests moved to default-discovered mock-guest-runtime executable; actual scalar Source/Core/VM/foreign/retirement law moved from inline Call into existing actual external law file and registered original-scalar-call; three public Wake treasury/Source laws moved to original-wake-public executable while both private lock/actor laws stay inline. Self dev-dependencies activate existing testkit fixtures for standalone integration, without skipping tests or adding runtime libraries. Latest measurements: HostGuestRuntime19files257610B2703lines; InterpreterRuntime31files256583B2406lines; scalar test2files9224B20lines; Wake17files251005B1086lines; public Wake test3files12965B38lines. All under fixed limits, no cap changes. Native parse/type/test qualification pending.


## SBT1 First Source Transition: Authored Codec and Pending Orchestration

SBT1 has exact extent 200: prefix `SBT1`, little-endian version1/extent200; genuine144-byte SAC3 at offset8; physically returned prior bootstrap sequence at152; actual prepaid items/copy/capacity/release/work at160..200. It only wraps Admission or Source Failure, and requires a nonzero genuine SRE extent. Parsed DATA never mints producer authority. `OriginalActorBootstrapTransitionProof` has private fields and is minted only by the existing stable bootstrap backing after its real exact handback. With no earlier SAB, the proof is absent.

The actual actor receipt exports, immutable scalar escrow borrowing (200/inneroffset8), Source header validators, publisher200-byte backing, 27-write completion plan and genuine extra work reservation are mounted. Snapshot publication actual work remains unchanged; the two extra transition writes are charged as header effects. This costs 16 additional copy bytes and2 work versus184; item count stays fixed, so existing93-item/3052-work quote becomes93/3054 (producer32 gives125 items, within128).

Neutral JSON/schema plus strict AJV and independent SQLite little-endian oracle: SAB/SBT2 tests passed63 assertions. Same-corpus native codec/borrowed-range and stable exact handback/no-refill laws are authored but have not compiled. Actual repo rustfmt check exited1 for formatting differences, with no parser diagnostics; generated/sbt1-parser.txt retains the observation. Current measured ActorReceipt7files64642B293lines; GuestContract32files257266B1051lines before shared accepted-cause helper; ActorContext13files101044B684lines before pending bootstrap ports.

Actor Context now has explicit phase observation, bootstrap physical/work debt recording and same-issued-epoch dispatch. These ports retain the real admitted context and never mint a Source. Actual admit_owned orchestration, metered immutable clock initialization and private transition proof lifetime through actual200 header acceptance remain incomplete. The old admit_owned/return_fixed path is still present and is not accepted runtime evidence.

Shared accepted full-cause cleanup is mounted: gates accepted actual entry/Source origins, closes publication and complete value/cleanup owners under Source, keeps associated input failure identity/origin until actual wire return. UI/jobs close remain separate.

Owned native failure33676 ended precompiler exit1: generated/mcp-24032.json reports stale failure manifest expected1819 observed1914 after the newly authored dependency. It produced no native test/type result. No duplicate native invocation was launched.


## Actual Export Mount and Qualification Run

The actual `admit_owned` now delegates to the public Entry header port and actor-receipt bootstrap state machine. Partial context/cancellation/payload/Source births publish304-byte SAB1. The readiness turn continues the exact already-issued epoch through200-byte SBT1, then later UI/bundle admission uses genuine144-byte Source headers. The obsolete136-byte static return function and runtime handback fallback were removed. Bootstrap header handback uses only the exact stable backing and prepaid target. Pointer-width and cancellation checks precede effects.

The actual prior SAB handback enters the ActorContext cumulative physical counter before the new epoch baseline. Its work is recorded as reserved observation, not current fresh work. The private once-only observation marker is itself paid current metadata. This avoids double-debiting stock already withheld by the Host. Admission, physical-post and work-post full causes have independent retained slots; both posts are attempted, failures expose no header and fence further Entry semantic work. The private transition proof remains retained in Entry, while subsequent headers use ordinary SAC3; it is not reissued as another transition.

Shared200-byte Source header completion includes actual readiness/transition metadata in the self-inclusive inner receipt, uses27 actual writes, preserves full failure through the200 failure branch, and uses explicit pending dispatch rather than reopening an epoch. Accepted full-cause cleanup was shared with actual canceled ActorInput and lifecycle close; associated failed input identities remain until actual wire return.

Current measured GuestContract is32files261938B1062lines (206B margin), ActorReceipt7files74174B319lines; hard limits unchanged. Portable SAB/SBT/treasury3 tests passed67 assertions. Actual repo rustfmt parser session64076 ended with formatting differences only and no syntax diagnostics, although the most recent fullcause/pointer changes were authored afterward and need fresh type qualification.

After fresh owned doctor census1793945853113 showed569 processes and no actor-receipt target, one native owning-manifest actor-receipt --lib test was launched via repo MCP with4h delivery and1job. Current owned session68062, repo MCP PID14804. Inputs: bootstrap-native-test.json and bootstrap-native-census.ts; source hashes are in generated/bootstrap-native-authored-snapshot.json. It is live and has no native result yet. No Wasm producer/artifact/action/inference/UI/history end-to-end pass is claimed.


## Public Native Law Source Ownership

To reduce actual production/Wasm source closure without changing limits or deleting cases, public bootstrap codec/transition/debt tests moved to default-discovered Cargo integration executable actor-receipt/📦️packages/🦀️rust/tests/actor-bootstrap.rs. Public Source/failure/input receipt laws moved to adjacent tests/actor-receipt.rs. Relative neutral corpus imports were corrected by hand. Private owner flags, stable backing, exact preissued handback and refusal tests remain inline unchanged. No manifest/runtime dependency changed. The inline production source reduction is10599bytes; actual cfgtest ActorReceipt closure reduced from7files74174B to4files58190B. Integration closures independently measure3files9990B and2files6597B, all below unchanged limits. GuestContract remains32files261938B1062lines,206B headroom.

The existing native68062 --lib run remains owned and live, last observation621s in build-entry, waiting cargo configuration publication exclusive position4/4 behind PID71840, holder resource-retirement-inventory. It has no compiler result. Because public laws are now separate executables, the next coherent qualification must include --test actor-bootstrap and --test actor-receipt (or --lib --tests) after the current run is terminal. No duplicate was started.

Launch separately authored the genuine Note mutation DslReceiving derivation and original ordinal operation receiving adapter at the real Note artifact owners. The known prior note producer40551 is terminal from the16MiB authorizer refusal; no current accepted guest/component/native end-to-end proof exists. A new coherent note producer must capture all final26 Source and receiving changes.


## Shared Native Dependency Diagnostic Check

Parent recovered real compiler output from Root11269/59471, preceding later stale root-manifest extents: Value retained-clone had29 argument propagation errors after maximum_work_units was added. Read-only current tuple-cursors still has old two-argument advance_demands and three-argument advance child calls, while the canonical trait requires source/body/work and source/body/work/grant. No edit made pending owner coordination. ActorReceipt68062 remains precompiler at801s, configuration-publication position4/4 behind71840; it has not independently confirmed compiler errors. Its exact progress is generated/mcp-36756.jsonl.


## Native68062 Terminal Result

Actual structured generated/mcp-36756.json reports exit1/okfalse after1648488ms (~27m28s). This run ended PRECOMPILER while Framework Cargo.lock refresh parsed host/⚠failure/Cargo.toml: inherited semio-plugin-interpreter-runtime was absent from Framework workspace.dependencies. It produced no compiler/type/test result. Root was notified to fix its host-failure manifest; no retry started. Next coherent owner invocation must include public actor-bootstrap and actor-receipt standalone executable laws.

## Scoped Admission And Distinct Bootstrap Preflight

OriginalActorContext/Slot admission now takes a genuine StepBudget and propagates complete OriginalTurnFailure. SourceParent birth receives its stable actual allocation marker and actual birth work; scoped preflight reserves marker clearing, and failures preserve the allocation marker and independent body/settlement causes. Initial inline Context birth now charges its real typed Option extent. SourceAbsent bootstrap retains debt in its preborn SAB owner without inventing a Source.

OriginalActorBootstrapReceipt::first_bootstrap_response_reserve exposes a separate46item bootstrap quote (38header writes+3header bookkeeping+held return+clock+fault+observer+onebirth); work also covers the larger actual Source birth, and capacity/copy derive from actual typed birth demands. It does not alter the normal32item producer cap. The schema+SQLite reserve vectors32/45 denial and46/128 acceptance pass: two bootstrap tests,45assertions. The new native preflight/no-allocation-denial law is authored but uncompiled. Direct owned job/trace dev-dependencies are explicit; current native3395 captured before these edits and remains a freshness-qualified run.

Fresh cfg(test) owner measurements: GuestContract32files260974bytes1057activeLines; ActorReceipt4files59167bytes316activeLines; ActorContext13files111515bytes718activeLines. Five current bootstrap/context/Entry/lifecycle/derive files parsed through real rustfmt with formatting-only exit1, no parser diagnostics.

## Latest Owned Native Census

At 1793953449867, actual repo doctor census had541processes. ActorReceipt test launcher24856, parent49700, remains alive age3752s; descendants24856,31984,72656. The immutable loader is source.served/preparation; no compiler/test result is established. Full process metadata is retained in generated/bootstrap-native-census.json. No restart or duplicate was issued.

## Mandatory Scoped Closure And Complete Cause Retirement

Context, ContextSlot and GuestEntry close_step now require StepBudget; no retained-grant overload remains. Scope fuel/deadline cannot exceed the original envelope, selected Source/payload/cancellation work is quoted before custody changes, and decoder closure runs under the actual scoped Source context. The existing expired/cancelled retirement rule is preserved: genuine retirement may finish despite the semantic deadline, within the finite retained work authority.

JobPayloadAuthority delegates its canonical selected ledger work; CancelTokenRetirement delegates the real ordinary and alias-return frontiers. RetainedFaultPublication delegates actual wire/builder work. GuestEntry removed the old guessed-work/grant-only close_failure_step; its Source callback drains accepted full causes and controlled retirement under the scoped context. Unaccepted origins and active jobs remain held. Its pre-funded typed result slot is written on both successful and failed body outcomes, and complete cleanup failures remain in Entry custody.

CancellationReceipt{blocked,receipt:OriginalTurnFailure} now preserves a blocked cancellation together with an independent physical receipt failure. OriginalActorContextError and CancelTokenRetirementError derive controlled RetireOwned; the exact blocked reason is a leaf owner. The native five-case law checks genuine Source capture, no-effect initial denial, whole-owner pointer retention and exact heap birth/release through final retirement; it is authored, not yet compiled. Neutral schema/TreeSitter/SQLite closure inventory and six-cause accounting passed three tests,31assertions. Existing paired admission neutral laws passed two tests,13assertions; context/epoch/conservation/decoder laws passed eight tests,92assertions; treasury+transition two tests,33assertions.

Typed cause handoff now uses one retained item for the atomic ownership transition, its full donor+recipient copy extent, and two separate actual control work units; a selected item-one Source no longer stalls this handoff. All affected actor/Entry fixture close callers carry explicit finite CPU/deadline scope.

Owned native3395 remains live, latest output~5166s waiting cargo registry-fetch exclusive behind equality process52072; no native compiler/test result, no duplicate or restart. The fixed caps remain unchanged.

## Authoritative Lease Status

Owned read-only repo MCP leases_status completed successfully at1793955141639; fullstructured evidence generated/mcp-70956.json. Registry-fetch/GC selection:

{"at":1793955141639,"holders":[{"resource":"cargo:registry-fetch:C:\\git\\semio\\.cache\\b\\r\\m\\cargo\\home:-","hash":"f3472c3030f1a20e30066a993ebb16f056f0914615e672e76d354f641770e27e","mode":"exclusive","pid":72656,"task":372,"sinceMs":1793955022300,"heldSeconds":119,"class":"registry-fetch","rank":8,"source":"registry","command":"44cd64c1d50b3326b463760dd8a74bc2d00a78fcb9b59cdd8ad0609e16416ffd cargo --configuration=prototype --store=shared --parallelism=1 -- test --manifest-path 🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/📤️receipt/📦️packages/🦀️rust/Cargo.t","session":"5cea31605a9b.ai0fwranfc","standing":{"state":"silent","priority":"agent","phase":"cargo:registry-fetch:C:\\git\\semio\\.cache\\b\\r\\m\\cargo\\home:-","silentSeconds":119,"etaOverdue":true,"implementationDigest":"8c274a8966893a8d","implementationAgeSeconds":5440,"parentPid":31984}}],"waiters":[],"cycles":[],"violations":[{"version":1,"kind":"hold-and-wait","held":"cargo:configuration-publication","heldMode":"exclusive","heldRank":2,"heldClass":"cargo-coordination","requested":"native-storage-mapping:76db27e3761010dffb33d53028e1ed1f88a0c6fe0279d9c679f55783c00192c6","requestedMode":"shared","requestedRank":0,"requestedClass":"unclassified","atMs":1793948971007,"pid":45508,"waitedMilliseconds":15173,"enforced":true},{"version":1,"kind":"hold-and-wait","held":"40d145001c8a","heldMode":"exclusive","heldRank":0,"heldClass":"unclassified","requested":"cargo:configuration-publication","requestedMode":"exclusive","requestedRank":2,"requestedClass":"cargo-coordination","atMs":1793951173327,"pid":52588,"waitedMilliseconds":16022,"blockerPid":36236,"blockerHeldSeconds":19,"enforced":true}]}

This status records no wait-for cycles. It records two historical enforced hold-and-wait violations at PIDs45508/52588, distinct from current ActorReceipt72656. ActorReceipt has now acquired its registry-fetch lease (119s held at the snapshot), which is genuine preparation progress; it is not compiler-qualified. No holder was terminated.

## Host Reserve Alternatives

Both Poll and the actual Wake guest-loan gate now require header200 to cover two alternative envelopes: genuine Source SBT1 and the actual SAB1 first-bootstrap reserve. Each is checked independently against the same original issued physical and work budget. Their items/copy/capacity/work are not added together, so the standard128-item first-response policy remains possible while both actual paths are funded. Authored source, not yet native qualified.
