# Patch Fixture Finance Audit — 2026-10-14

Read-only source audit requested by the root task. No builds, native tests, production edits, Git mutations, or cache changes performed. Concurrent edits mean line numbers describe observed reads and require refreshing before editing. All paths below are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`.

## Critical API inventory

- `⚛️reactor/🩹️patches/🦀️.rs:137`: `commit_source(&mut self, &mut Option<ComponentTree>, &mut StepContext) -> Result<bool, OriginalTurnFailure>`. A successful call consumes the source; a denied call leaves it owned. At line 147 an observed intermediate branch still says `return Err(tree)` with no defined `tree`; this is an independent production compile blocker during migration.
- Unit old by-value call sites: `⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs:259,294,344,402,567,601,642,691,732,772,1656,1721`. Line 402 expects `Err(root)`, which is no longer the refusal contract. It must assert `Ok(false)` and exact retained `Some(root)` ownership, retaining the grant until its lifecycle is explicitly settled. Ordinary `.unwrap()` on `Result<bool>` is insufficient: assert success, not merely lack of failure.
- `⚛️reactor/🔄️turn/🦀️.rs:1986`: `drive_reconcile_within(patches, opportunities, cx)` now requires the actual original context, not an Instant. Legacy deadline arguments remain throughout three fixture suites: reconcile-spin lines 96,158,200,206,242,259,269,318; more-work-drive lines 445,489,577 (and earlier cases); turn-patch-batch line 102. Deadline-denial case reconcile-spin:242 needs an original context with a narrowing work/deadline loan, preserving identity and remaining fuel, rather than constructing replacement authority.

## Original caller and alias lifetime

`🧪️testing/🎟️caller/🦀️.rs:15-37` owns one source parent and cumulative receipt. `context()` removes its budget and can only be issued once. `retire(fuel)` closes the original source roots under remaining authored stock. This rules out per-loop `context()` or a caller freshly admitted per reconcile opportunity. The unit suite and reconcile-budget now retain one caller/context across a body; reactor-driver `patches_diff` also accepts caller context. That signature change alone does not finance the whole helper graph.

Every returned published/ack/ready/patch child and every retained tracker must reach paid terminal empty before `drop(cx); caller.retire(fuel)`. Unit helpers `published` return a live child; `finish` and `acknowledge_one_publication` return a live patch. Inspect their callers for stored vectors/options, rejected sources and callback roots before final source retirement. Unit's body closure helps normal lexical teardown but an ordinary Drop of a live child is not evidence of paid closure. Reactor-driver's thread-local tracker persists across `patches_diff` calls, so root retirement must follow final settlement of all source-parent aliases, not merely return from the last render helper. No native alias-lifetime success is established by this audit.

## Helper graph still requiring one genuine context

- Unit: `finish:85 -> publish_test:43 -> close_published:51`; `acknowledge_one_publication:120 -> publish_test -> acknowledge_into:127 -> close_ack:62`; `published:146 -> publish_test -> close_test_patch:73`. `drain_released_output:104` and `close_instance_to_empty:172` already take cx. The four raw publish/retirement helpers do not, so their bodies still bypass original debit even when their parents take cx.
- Reconcile-budget: `publish_nakagin_world_patch:76` and `mount_and_publish_nakagin_world:165` take cx for tracker semantic work, but raw publication/ready/published closure remains at 101-105 and 184-188. `retirement_turns:118` takes no cx. Whole ladder case:198 charges tracker close but raw world close:220 and global handback close:228 remain independently bounded rather than posted to original cumulative stock.
- Reactor-driver `patches_diff:131` charges drive/close but publish:145, ready close:148, acknowledge:152, ack close:157 and published close:160 are raw grants.
- More-work-drive: `settle_one_opportunity_per_turn:544` must thread cx through drive:577, `drain_pending_authority:528`, `retire_host_patch:517`, pending acknowledgement:560 and pending close:572. Byte-cut case:437 also uses raw drive:445/489, ack:478 and close:484.
- Reconcile-spin: `settle_surfaces:51 -> settle_tracker:62 -> drain_pending_authority:24 / retire_host_patch:219 / drive_reconcile_within:96`. All must share the initiating context, including cancellation and deadline tests.
- Turn-patch-batch: `settle_batched:82 -> retire_pending_authority:27 / drain_page:52 -> retire_host_patch:40 / drive_reconcile_within:102`, plus raw issued ack:97. Page transfers and handbacks need receipt ownership retained over refused/cut pages; closing only the returned host patch does not close guest publication authority.

## Receipt/API requirements

Raw `publish_into(... required_publish_bytes())`, `acknowledge_into(... required_acknowledge_bytes())`, `close_step_with_grant(items,bytes)` and pending-patch `close_step(items,bytes)` describe grants or partial page receipts; they do not prove the initiating source accepted CPU/copy/birth/release totals. Existing use of descriptive required-byte methods is not authority. Defining UI-runtime and UI-contract owners must expose demanded bounded work plus original-context transaction/close APIs that preserve partial progress on failure, debit before ownership mutation/allocation, post exact physical release after it occurs, and return denied owners unchanged. Pending publication/issued-ack/global handback must likewise propagate receipts and original context. Do not paper over these seams with larger constants, newly admitted contexts, demand-derived positive stock, or fixture-only accounting adapters.

## Minimal coherent migration sequence

1. Finish defining commit_source and original-context owner APIs; clear its intermediate undefined-tree branch. Preserve deny/false source ownership semantics.
2. Convert all old commit_source fixture expressions to explicit mutable grant + Option source + same cx, assert true on admission and false with exact owner retained on rejection; settle refused trees and grants before root retirement.
3. Thread one cx from each whole fixture body through the complete helper graphs above. Model deadline/item/copy/release denials as loans narrowing that original stock, retaining recipient identity and fuel.
4. Replace raw publication, ack and retirement calls with their defining original-context APIs; preserve refused/cut-page custody and partial receipts. Close every child/tracker/global handback owner before original caller roots.
5. Perform repository-authorized compile and native physical laws after concurrent API edits settle. Source signature alignment is compile evidence only. Native proof must compare real heap births/releases with accepted original progress, show zero/short grants do not change owners, exact loans release original allocations, failure retains partial receipts, and terminal Drop allocates/releases zero. Also demonstrate repeated render same-recipient fuel and all aliases retired before parent root release.

No fixture, feature, compilation or native finance pass is claimed. Existing authored fixtures provide source coverage, not observed physical success.
