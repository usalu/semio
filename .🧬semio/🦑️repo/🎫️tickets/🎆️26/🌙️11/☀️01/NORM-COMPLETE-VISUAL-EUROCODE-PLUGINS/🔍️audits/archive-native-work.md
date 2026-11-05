# Archive Native Work Receiving

The document archive consumer now supplies the actual initiating StepContext to a private closing-aware History turn/work recipient. The context contains Cell<ClockStride>, so the recipient holds an inline Mutex over the exact mutable context; try_lock excludes overlapping access without heap allocation or unsafe shared access. Its admits method only checks real cancellation/yield and checked observed plus additional work against the real remaining fuel. It does not debit work or construct another wallet.

DecodeParent observes the driver native counter after its call, takes its original physical receipt, and passes the independent reported and observed work to OriginalNativeDecodeSource.receive_observed_work. Terminal cleanup follows the same path. The canonical reconciler posts max(reported, observed) once and retains any independent receiving refusal. A simultaneous producer ValueError and reconciler primary ValueError are retained in the existing OriginalReceivingPrimary envelope, alongside outer receipt and context failures, without formatting/cloning the producer diagnostic.

PendingDocumentArchiveMember and ActiveDocumentArchiveLoad now return RetainedWorkOutcome. Nested erased retirement, hydration, and owned-retirement birth delegates propagate their original work even on failure. Direct page and header primitives report their actual work at their operation sites. Active close separately exposes the actual History native delta; it does not infer it from grants or physical progress.

MemberOpenRequest and OwnedDocumentMemberIngress now reset and report actual close primitives per call. The erased request retirement delegates that reporter instead of inferring work from a nonempty receipt. Public ingress literals initialize the scalar reporter.

The archive maintenance ladder now uses original_lifecycle_step, so nested already-posted physical receipts are adopted rather than billed twice. Terminal archive owner removal occurs on its own next one-item turn; a cleanup failure cannot be erased by dropping the now-terminal owner.

The existing schema-neutral archive physical lifecycle fixture remains the source of grant/capacity/member cases and serde_json remains its independent oracle. Its native law is being adapted to an actual OriginalJobFixtureCaller and the actual context work recipient. No native acceptance is claimed at this source-current checkpoint; shared lock updates and focused native gates remain outstanding.

The initial focused check74872 stopped before rustc because framework/Cargo.toml excludes the OS instance-docs package. That gate establishes no source result. Its framework workspace update5527 was terminal0 with no package version changes but did not cover the new OS edges. Corrected update15798 uses the exact instance-docs manifest, whose authored workspace resolves to repository root. The original logs remain under generated.

## Changed source owners

- pack/json/retained/schema/decode-pages: actual directory-close reporter and reset.
- os/store/composition/open: actual request-close reporter, erased delegate propagation, release admission before closing-state mutation.
- plugin/child/document-member-ingress: actual request and identity reporter, initialized public owner field.
- plugin/document/document-archive-load: Pending and Active canonical work outcomes and actual History observed counter.
- plugin/document root: explicit native work recipient reexport.
- plugin/instance/docs/staging/maintenance-ladder: real guarded original context payer, canonical receiving and complete refusal envelope.
- plugin/instance/docs package: explicit job-native-decode and value-native-owned dependencies.
- plugin/maintenance-ladder: original lifecycle accounting and separate terminal owner-removal turn.
- plugin app-child-member-registry native laws: actual original fixture caller and denied-work assertion.
- plugin runtime-close-budget TypeScript law and shared archive-close neutral fixture: updated canonical source signature and work contract.

Corrected exact-manifest workspace lock update15798 is terminal0 with zero package version changes. Focused check37377 is active against the actual instance-docs Cargo manifest; no compiler or behavioral acceptance is claimed yet.

## Native check 37377

The corrected exact instance-docs manifest gate reached Rust compilation and returned repository101 before compiling the archive owner. The shared NativeSnapshot API had become canonical RetainedWorkOutcome, but presence/snapshot/json still supplied two receive generic arguments instead of T/O/E and used Result-style ? inside its now-outcome callback. The sqlite-snapshot receiving macro still matched the canonical outcome as Result (16 compiler errors). The original diagnostics are retained in generated/archive-work-check-2.log.

This establishes no archive or page custody acceptance. The next archive/forms successor is held until those shared consumer owners are source-current. Root and native recovery were notified with exact first diagnostics. The existing archive physical law has not yet run.

## Neutral fixture gate

The exact archive fixture law is terminal0, 1 test and28 assertions, through ordinary registered Bun capacity (archive-work-fixture-bun-4.log). Ajv validates the shared work contract and SQLite independently validates all four max(reported,observed) posting vectors. The same law checks the original custody/source owners. This remains a neutral fixture/source law, not native lifecycle acceptance.

The full containing runtime-close-budget file previously returned19 pass,1 skip,11 fail and374 assertions. Its archive law passed. The other failures were in concurrently changed mounted worker/constructor/publication/runtime/media/typed and actor fixtures; the actor case specifically lacked sqlVector. The original full diagnostics remain archive-work-fixture-bun-2.log and have not been represented as an all-green gate. An initial Bun operand omitted ./ and was refused before testing; a subsequent authored schema syntax error was corrected and retained in archive-work-fixture-bun-3.log.

The source census reports instance-docs15files205574bytes/max2176lines and document-media12files191600bytes/max2610lines, within unchanged262144/32/4000 caps. Full details and conservative cfg limitations are in archive-consumer-source-census.md.

A follow-up source audit preserves whole failure effects explicitly from the exact selected producer progress on every branch, including successful value plus receiving refusal and producer error without receiving refusal. The reconciler may author a literal work refusal whose own primary effects are zero while already posting the producer physical receipt. The enclosing PluginLifecycleFailure must carry the complete operation span, or the outer original lifecycle subtraction would invent a child/whole mismatch. Producer diagnostics remain unchanged. Root was notified to verify the same aggregation in Member and codec.
