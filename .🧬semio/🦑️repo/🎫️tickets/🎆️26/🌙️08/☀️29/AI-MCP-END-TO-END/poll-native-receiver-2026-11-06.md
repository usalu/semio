# Source-Owned Poll Receiver and Failure Boundary — 2026-11-06

The end-to-end semio MCP goal is still active and incomplete. The actual MCP channel still has its raw GuestInstance/Arc driver consumers; these changes establish the real retained receiver composition needed to replace them.

GuestPoll<T,R> and GuestPollSession<T,R> now require an explicit R: OriginalNativeRetirement. R physically resides beside decoded T inside the same actual NativeInstanceHome; the Driver remains outside that native root. No additional Source, clock, budget, hidden Arc or context-free fallback was introduced. Constructors append R; drive/close receivers now receive (&T, &mut R, &mut StepContext). Normal Poll completion/reset preserves R between turns. Actual native retirement and Drop require both the Poll payload and R to have closed.

A receiver error now retains captured T and Phase::Deliver, records the full actual transport failure into the exact preissued destination, and returns false. Another receiver cannot run until that full failure was received. Only an actual accepted receiver starts T retirement. The native callback law was migrated to this stronger contract; the old law's no-retry-before-cause requirement remains, and explicit continuation after cause reception now accepts the still-retained input. Transport/scalar/input failure-taking callbacks also receive the actual R so their destinations can live inside the native root.

Portable verification: all five Poll test files pass, 6 tests /77 assertions. This includes real bootstrap/cleanup ordering model, independent SQLite ownership traces and full callback/cause ordering. Native tests migrated; additional actual OriginalJobFixtureCaller zero-copy denial and consumer-native-retirement test authored. Real rustfmt parses the new composition and native tests; formatting exit1 is syntax evidence only. No native/type qualification yet.

## Kind7 and Causal Identity

ActorFixedKind InputFailure is strict144 with real input_returned proof. Runtime unit acknowledgment uses the shared input_disposition helper and records whether the input failed; a failed frontier cannot enter an ordinary repeat. Raw donor release still depends only on the real returned flag. Detailed-failure controller/cleanup must handle the remaining failed input without inventing cancellation or returned proof.

The opaque Host causal factory now uses shared is_failure_for(actual_call_kind), so kind7 is accepted only for the actual AdmitActorInput frontier. Frozen SAF1 identity now reads the genuine nested SAC3 epoch at offset32, like SJH2.

A new origin-scoped native helper captures actor failure only from the real frozen producer and exact retained destination, using the immutable cache resolver rather than a cloned runtime. Poll stores this sealed reference, Session stage27 captures it before fixed/header acknowledgment and deallocation, and semantic continuation/terminal payload closure remains blocked while it is unreceived. Normal InputAdmission and InputFailure journal/resume dispositions are both recognized. The shared failure-fetch controller and sealed final acknowledgment proof are still being authored by Server; no unsafe boolean clear or fake actor cause acknowledgment was added.

## Remaining Consumer/Transport Work

Server owns the shared Host failure owner and frontier. The agreed full-cause recipient uses a real OriginalActorReturnSlot with an empty preborn recipient Home, actual capsule NativeLease, two qualified transfer claims, actual-written settlement and same-source sealed reference. Only a real recipient acceptance proof plus GuestAck and physical header handback may yield ActorFailureAcknowledged. Borrow6 has no synthetic184-byte header/SRE or new guest loan. Root mounts actual verified History/journal and deallocator proofs.

Guest owns schema-first SAB1 bootstrap304 protocol because five real context births span epochs before a Guest Source exists. No fake SourceID/SRE/zero escrow/SourceClosed is allowed. Real current progress, observed prepaid, held return currency, physical sequence return and finite admission debt remain distinct. Root owns subsequent Host SAB1 qualification/settlement/handback integration. The first real Source-ready response uses ordinary SAC3+genuine SRE.

Launch owns archive/config/media actual borrowed admission and retained constructors. Config will use existing OriginalArtifactPairReceiving plus RetainedConfigStoreHydration; media uses actual reserved media/import and controlled base64/archive ports. Guest retains cold-pair/checkpoint Source restore gaps and Presence cancellation aliases rather than rawdropping them.

Root owns shared full NativeSnapshotFailure/OriginalArtifactPairFailure projections needed by those constructors, and actual MCP session/receiver wiring after these ports are coherent. Native Source birth/return-cell metadata audit remains open in native-admission-follow-up-2026-11-06.md.

## Reusable Commands

A completed Poll already clears its per-command binding on the next actual advance; the earlier research statement that Complete was unreachable was incorrect. A permanent opaque origin binding/frame address now survives that command cleanup, separate from the active command binding. begin_next_step validates that actual origin, requires a fully settled successful round and live nonretirement turn, and pays the exact empty-unit/decoder/control reset before beginning Encode on the same guest context. No guest or decoder Source birth occurs during the reset. Native component laws and actual MCP adoption remain to be qualified.

The Session next_input_step uses a preborn original Result<bool,OriginalTurnFailure> slot and exact sized Source return place. It always captures the actual reset result, including Pendingfalse or the complete cause; a later funded turn settles the witness and promotes the result before another request may drive. Cleanup drains that same frontier. A reset error returns through the existing complete Driver carrier instead of being formatted or replacing another error. This is authored and awaiting native qualification.

## Current Component Evidence

Poll round fixture passes1test/7assertions using strict AJV and SQLite; required native work-view fixture passes1test/4assertions using retained SQL reservations. Actual Rust original-work-view law is registered standalone in NativeDecode owning manifest, authored/uncompiled. Poll native round component law covers all eight denied frontiers, exact same original binding/frame/receiver, actual Source return-witness settlement, zero reset allocation, and real scoped CPU delta across two resets. It arranges a settled Poll component directly and does not execute a VM or assert an end-to-end artifact journey. Selected Rustfmt check passed parsing (formatting-only status1, no parse errors). Owning Pollcfgtest measurement before last held-witness law was9files/164231bytes/1008lines; fresh final measurement remains to be taken.
