# Typed Completion Authority for Source Holds

Read-only source checkpoint2026-10-21. No new protocol is automatically required merely because browser completion lacks generation. Native execution remains unverified.

## Actual authority chain

Native typed ingress/docs staging obtains operation IDs from allocate_operation_id_in_slot at instance/docs/staging/ingress:82,102 and typed-operations:76. A free output slot is selected,then a globally advancing ID with that residue is minted; slot reuse does not ordinarily reuse the ID. Native jobs also retain generation/base revision/cancellation authority internally. Runtime typed-operations:97 exports completion operation+revision+scope/history. Generation is absent from the actual OperationCompleted frame,not merely discarded by renderer. App-channel unsolicited-feed:87 binds decoded completion to its actual client instance; renderer plugin handle/channel lifecycle supplies the rest of publication provenance.

Framework trace/operations-and-trace:27 allocate_operation_id uses AtomicU64.fetch_add. Slot allocator:36 uses unchecked advance(next)+1. Contrary to its never0/processunique comments,there is no checked exhaustion refusal: fetch_add wraps and slot arithmetic can panic debug/wrap release at u64 exhaustion. This is an extreme finite-domain boundary,not a practical duplicate reproduction. Strict unbounded never-reuse cannot be proven from current source. A narrow allocator exhaustion fix is cleaner than adding generation to an otherwise sufficient completion protocol solely on speculation.

## Sound narrow transfer

Within an authentic module/instance lifetime with nonreused allocated operation IDs,exact plugin handle+LocalDocumentOwner object+instance+operation identifies the admitted terminal operation. Retain a source hold before command; on admission attach its opaque release token to that exact completion subscription/owner. On matching actual completion release once. Owner retirement closes remaining tokens; watchdog/duplicate waiter/capacity null never releases a known started operation. Generation can remain attached locally to the admission record for internal cancellation,without pretending the completion frame independently proves it.

Protect completion-before-admission registration by subscribing/recording under the command owner before submitting command,or bounded terminal evidence scoped to exact handle/instance lifetime. Do not reuse a global string-key completion ring across plugin reload/document owner replacement. Reject duplicate live admission keys and keep source unresolved on capacity/refusal. A late completion from a previous owner must not release a new owner's hold even if numeric IDs coincide. Never infer completion from UI/history/cache hashes.

For formal all-u64 correctness,make allocator overflow refusal/nonwrap explicit and test boundary allocation independently. Until then document finite-lifetime nonexhaustion assumption rather than claim source proves never-reused monotonic IDs. No need to change protocol if exhaustion is correctly refused and exact channel/owner lifetimes are preserved. Actual host integration tests should cover slot reuse with fresh IDs,late completion/reload,completion-before-registration,duplicate callback,watchdog and overlapping holds.
