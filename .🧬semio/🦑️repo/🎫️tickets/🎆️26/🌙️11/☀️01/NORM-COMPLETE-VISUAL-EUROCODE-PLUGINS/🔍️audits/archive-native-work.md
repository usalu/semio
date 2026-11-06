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

## Current Gate Blocker

The current repository cargo authorizer fails before compilation at 1,831 files and 16,788,269 bytes against its unchanged 16,777,216-byte source limit. Its captured entry is the general build command runtime, whose imports include graph, benchmark, conformance, resident, and other command paths in addition to Cargo. A dedicated Cargo runtime entry with a complete independent captured closure is a possible architectural direction; simply hiding reachable sources or raising the limit would not establish source custody. This is a source inspection, not an implemented fix or acceptance result. Native Terra reports SQLite source updates and remaining fixture compilation errors; the archive gate remains pending shared source readiness.

## Cargo Source Boundary Repair

The actual kernel-selection neutral fixture now specifies Cargo, Bazel-selected Cargo, opaque child Cargo, and help. Before implementation the SQLite oracle and actual selection disagreed on Cargo; after the repair the focused test passed, 1 test and 79 assertions. Ajv validates the fixture. Cargo now selects its own executable kernel, using the existing authenticated captured command, original invocation and implementation pin, diagnostics custody, resolved command environment, genuine Cargo launcher and issued cancellation/work control. The sealed kernel table includes the same entry. No source, size or authority limit was changed.

The direct actual compiler-source capture regression passed: it captures the selected Cargo entry and real launcher, excludes the unreachable general runtime, and remains within existing compiler source-file and byte bounds. The actual focused ownership-package Cargo check has passed source admission and is still pending. Native Terra was notified that source is ready.

The independent sealed trampoline suite also completed 8/8 with 104 assertions, covering the exact kernel table, saved snapshot, promotion, restart, exit status and stop-signal behavior. The focused Cargo check remains pending.

The focused ownership-package Cargo check reached the real Cargo launcher and workspace/lock admission, then returned 1 before Rust compilation because concurrent plugin-result-page requests package semio-framework-os-store-one-item while its path manifest declares semio-os-store-one-item. This is not a Cargo compile pass. Root was notified. The exact instance-docs archive successor is running independently as execution 64190 with output under generated/archive-work-check-3.log.

Archive successor 64190 returned 1 before Rust compilation: the current interpreter imports workspace member inherits semio-framework-os-kernel from a workspace dependency that does not exist. The archive has no new native acceptance. Root was notified; the complete generated diagnostic is archive-work-check-3.log.

Archive successor 1921 admitted capacity but returned 1 before Rust: the live one-item-owners Cargo manifest feature durable-group-testing still names semio-framework-os-store-one-item-publication after its dependency key was renamed. Exact lock error is in archive-work-check-4.log. Root owns the ongoing coherent identity repair and was notified; no manifest change or archive acceptance is claimed here. Full forms native 40199 remains pending capacity.

The only referenced feature in one-item-owners is now corrected to the actual dependency key and actual target feature semio-os-store-one-item-publication/durable-group-testing; test-support remains empty. No feature membership was removed or aliased. Full forms native 40199 returned 1 before compilation because its norm workspace now declares semio-plugin while inspector-forms still inherits semio-framework-plugin; root was notified to coordinate the coherent crate identity repair.

Archive43068 returned 1 before compilation on root guest-contract package identity, concurrently corrected by its owner. Forms26406 returned 1 before compilation on outliner inheriting obsolete semio-framework-plugin. A scoped repair updated nine inherited plugin/kernel keys across outliner, entity-editing, shell-hooks and interaction-topology plus exact actual Rust identifiers; all feature attributes were retained. A fresh recursive path/workspace closure census over all explicit root and norm workspace members and their normal, development, build and conditional path dependencies visited 1,295 manifests and found zero missing workspace keys, mismatched target package names or missing feature dependency references. This is static manifest evidence, not compilation or import proof. Successor archive24211 and full forms61386 are in normal capacity admission.

History close now returns the original rich retirement failure through ActiveDocumentArchiveLoad instead of converting it to ValueError. The archive lifecycle merger preserves the child primary, child work refusal and independent outer quote/receipt/context refusals with the exact producer progress. Source-only independent audit reports no reachable cause or progress loss. An actual caller law is authored with owned prose pointer checks, the real original caller, a positive 73-byte HeapWitness, zero-allocation fold and funded controlled owner/boxed retirement; exact native gate94603 is queued. These latest sources and law are not yet accepted.

Archive rich-cause native gate 94603 returned 1 before the archive law. Rust compilation stopped in shared semio-ui-contract-presentation because SurfaceProps has value attributes without an owning derive at surface.rs:154 and :166. The actual compiler diagnostic is in generated/archive-rich-cause-1.log. No archive law acceptance is claimed.

SurfaceProps now declares its documented NativeReceive and RetireOwned derives, preserving the authored camelCase and optional bindings wire attributes. ActionBinding already has the same native derives and UiNodeBindings is its fixed list. Archive law successor 45539 is pending; this is source repair, not compiled acceptance.

Native gate 94829 identified five shared component types selecting the nonexistent default native-owned crate. RowTarget, TreeItemProps, TableRowProps, ExtensionProps and Component now explicitly select their existing semio_framework_value facade, preserving their wire metadata. SurfaceProps also explicitly selects that facade. Compilation remains pending.

Archive successor 45539 returned 1 during planning because its workspace-manifest -p selection did not resolve the docs package. It did not compile. Exact package-manifest successor 35604 is queued; log archive-rich-cause-3.log.

Exact manifest archive successor 35604 completed production admission and passed the earlier SurfaceProps attribute errors, but returned 1 before the archive law in semio-os-channel archive/decode.rs:18: NativeSnapshotDecodeOwner::receive now takes three generic arguments and returns a richer result. The E0107 and E0308 diagnostics are in generated/archive-rich-cause-3.log. Root owns this SPR boundary and was informed.

Channel archive decoder now returns its full NativeSnapshotFailure and RetainedWorkOutcome, supplies the actual third generic, and records performed parsing primitives through native_snapshot_turn. Existing refusal tests were updated to consume the rich outcome and actual retirement result; no other direct decoder callers were found in the selected workspace. Focused channel gate 24009 is running. These are authored changes, not compiled acceptance.

Focused channel gate 24009 returned 1 before Rust compilation: job-context exceeded its source extent at owner/🦀️.rs (3,702 bytes requested with 3,089 remaining and nine files unread). The diagnostic is in generated/channel-native-snapshot-1.log; the native job owner was notified. Channel test acceptance remains pending.

Channel successor 27763 compiled successfully, but ran zero tests: its archive decoder test file existed physically but was unmounted. The original test module is now mounted in the decoder; focused actual law successor 68653 is queued. The zero-test run is not test acceptance.

Unified erased retirement migration has begun after the target API red: OwnedOperationBytes, JsonBackingOwner and CheckpointOutputRetirement now return OriginalRetirementOutcome and preserve their original ValueError as the primary cause. Their work is counted at the actual close/release action, not inferred from a requested budget or receipt progress. Nested failures are being handled separately; no migrated cohort acceptance is claimed.

Mounted channel law 68653 returned 1 before Rust during lock refresh: the concurrently added job clock-identity-and-budget package reported wrong workspace (expected root, actual framework). The source owner was informed. Full diagnostic: generated/channel-native-snapshot-3.log. No test acceptance.

The IO snapshot owner now implements the unified erased outcome: original child failures remain whole, a child-settled refusal cancels only the unused enclosing reservation, and charge/commit failures settle together. NativeSnapshotFailure moves a retirement primary and work refusal into independent existing owned fields without a schema variant; its fixture law checks both original pointers, retained progress, zero-allocation folding, a positive 73-byte witness and controlled retirement. The IO test port now explicitly tracks and cancels only its pending reservation. These changes remain uncompiled.

Additional unified leaf migrations cover zero-payload draft, interaction state and capture, presence root, protocol conflict and the artifact-message composer. Original errors remain owned; explicit work is recorded at the invoked close, release or selected move, and the protocol conflict wrapper forwards its controlled child last-turn work and rich failure. The shared controlled cursor migration and selected-child original-control propagation are still pending; no compilation acceptance is claimed.

IO selected-child original control now follows pending, intermediate, nonterminal unwind failure, then output. The controlled and original loan paths delegate the selected child without a second settlement. Independent review verified this priority after catching an earlier output bypass; protocol conflict forwarding also passed focused source review. The mounted IO test adds a zero-fuel pending-child control denial, with actual caller and preflight-error coverage still outstanding. No runtime acceptance.

Plugin child-group and archive-ladder registry closers now use actual registry turn.work and retain failure effects through original receipt debit. The archive no-target/no-backing path returns without artificial work. Following independent review, paid private-group retirement uses retirement_should_yield so an expired semantic deadline does not halt funded cleanup. Source re-review and genuine caller tests remain pending.

The earlier two-field retirement conversion described above was superseded when the original failure gained receipt and prepaid nested child owners. Current source retains the entire OriginalRetirementFailure as NativeSnapshotCause::Retirement; an encoder arm is present, while paid recursive decoder/schema integration remains in progress. No nested owner is flattened.

Nested retirement transport now uses an iterative native-depth-preflight encoder with individually guarded fields, and an iterative decoder retaining partial node vector and child in the original failure frame. Child nodes use the original retirement loan quote, actual fallible storage admission, and physical post; failed admission posts zero and leaves originals in the frame. Independent focused source review found no further concrete field or node-custody defect. Neutral source-depth cases and serde JSON projection law are authored; no runtime/allocator-failure acceptance is claimed.

Terminal Box release-only fixture now requires zero copied bytes and the independent SQLite/Ajv terminal oracle passed within execution 7944. The containing Bun file had 6 passes and 1 failure: a separate existing publication-header source law conflicts with concurrently changed publication_progress reporting zero copy bytes. Root was notified; the file is not all-green and publication semantics remain unresolved. An actual original-caller frame-size release-only law is authored but unrun.

After restoring the actual Option<O> output-slot transfer quote and receipt, snapshot neutral SQLite/Ajv successor 68542 passed 7 tests and 124 assertions, including terminal Box release-only and separate publication header law. This is JS/source oracle evidence; the actual original-caller native law remains pending.

The channel decoder was further aligned to the current caller-owned receive slot and quote-backed intermediate publication; parser primitives request original work before effects. A real OriginalJobFixtureCaller/OriginalJobDecodeOwner denial-and-paid-receive law is authored, including a 73-byte positive witness, direct typed fixture checks and same-caller retirement. Existing forwarded-allocation/cancellation cases need migration to original work authority; shared fallible allocation quote/post is still under development. No channel native pass is claimed.
