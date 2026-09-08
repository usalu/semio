# History Future Completion Handoff

## Source Audit

The DB engine's HistoryFuture::poll reads and removes completion, then registers its waker after dropping the completion mutex, and returns Pending without a second completion check. ArtifactHistoryState::complete publishes the result and then wakes the current waiter. A result published in that registration gap can leave Pending with no wake. This is the same missed-wake mechanism as the capability-opening future, but a separate owner and result type.

The Ready path also does not clear its retained waker. Unlike capability opening, history terminal-root checks do not include the waker, so source inspection alone does not establish the same admission leak. Do not conflate the two retirement predicates.

## Required Follow-on

After the queued capability-opening native TDD slot, add a deterministic history-publication interleaving law using the actual HistoryView owner (success) and actual error path, before poll / in registration gap / after Pending. Observe first-poll readiness and wake count before any manual cleanup poll. Close every actual HistoryView and history admission/reservation and check the exact generation's registry entry is gone. Reuse the existing neutral publication vocabulary, with a dedicated history schema and fixture.

The production repair should centralize completion consumption, clear the retained waiter before retiring terminal state, and recheck completion after registration. It must not replace the retained history actor, reservation, or terminal close protocol.

No HistoryFuture production change or native test was made by this audit. The shared native build cache remains owned by the running GIS gate; this follow-on must not displace the already queued capability-opening verification.

## Test-First Handoff Regression

Added a closed six-row language-neutral history-completion fixture: actual two-operation replay success or pre-handoff cancellation, published before poll, in the empty-check/waker-registration gap, or after Pending. The registered source gate passed (session50421): AJV1 and independent SQLite publication6. Its oracle compares readiness, wake count, exact result, waiter retirement, admission release and registry release. Launch seed entries411.0765/411.0766 register source and native gates.

The new native law captures the actual actor-produced HistoryView or the actual cancellation result using a test-only completion hook, then publishes the retained exact result through the production complete method at each boundary. Success checks replay entries allocation identity, original admission slot/generation, terminal-state Arc, and both operation IDs. First readiness and wake count are captured before any second poll; bounded cleanup closes every actual replay/reservation owner before reporting the exact generation's registry/admission release. Test-only publication hooks and behavior-preserving prepare/submit factoring are applied; the missing recheck and waiter retirement are not repaired yet. Native RED is queued after GIS; no native test result is claimed.

The independent CatalogRead audit found no additional P0 in its qualified19-case lease boundary. Home reported retained-admin GREEN2 at8sPN0G and released the native target to GIS. GIS is waiting for unrelated active compiler owners to clear severe memory pressure; no other process is modified. Ticket and overarching goal remain active.

## Native RED and Minimal Repair

The fixture additionally requires the waiter mutex to be released before calling the real wake callback; its bounded waker uses try_lock rather than risking a blocking test. The expanded source gate passed (session69756). Registry generation and subsequent freshness check passed (98678/85598); an earlier freshness run79594 correctly detected intermediate source-driven plugin digest changes and was not called green.

Initial native capture `ek9UHW/00` (session69567) failed at compile because the new test's database binding needed mut for shutdown; no runtime claim follows from that attempt. After that test-only correction, `2pQ91I/00` (session38793) built/listed and produced the intended native RED. All six DEBUG rows were read. Both registration-gap rows returned first-ready=false with zero wakes and a retained waiter; both after-Pending rows woke once while the waiter lock was still held. Every case preserved its exact result and completed bounded admission/registry cleanup, so teardown does not mask the failed first-poll observation.

Production now centralizes completion consumption, clears its transient waiter before Ready bookkeeping, and performs check/register/recheck. The publisher first moves its waker into a local owner and releases the mutex before invoking it. No replay, reservation, admission, or terminal-close protocol is replaced. Rustfmt2021 passed. The corrected five-law native cohort is running; no final GREEN is claimed yet.

The first repair rerun `9oPwN2/00` (session67748) failed at compile: an ambiguous patch context placed the new helper/waker change in sibling ArtifactSubmit code instead of History. The misplaced changes were removed, and both History edits were reapplied with explicit impl/type anchors. This is a patch-placement failure, not a runtime result or a new sibling feature. Scope was re-read and Rustfmt2021 passed. Fresh five-law rerun is session21682.

## Qualified Native Result

Fresh `wmx4f5/00` (session21682) passed all five native laws and six neutral rows. All six actual history DEBUG outcomes and all five success lines were read. Both registration-gap outcomes now return Ready immediately with no stale waiter; after-Pending outcomes wake once with the waiter mutex released. Replay entries, operation IDs, exact admission/state identity and complete bounded retirement remain correct. The other four actual replay/admission/reservation/public-close laws pass unchanged. Executable SHA256: `8eaee2ac873078bc834452f560295b07078267798f4c137ffb9a5669c4bb350e`. Scoped whitespace checks passed. The shared native target was explicitly released to GIS; no root native process remains. This closes the recorded HistoryFuture handoff defect only, not every retained History actor interleaving or the broader end-to-end goal.
