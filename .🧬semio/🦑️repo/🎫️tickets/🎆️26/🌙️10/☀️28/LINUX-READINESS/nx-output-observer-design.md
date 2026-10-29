# Owned Output Observer Design Review

Read-only review of proposed interface before implementation, 2026-10-29. No Source edits or tests launched.

The proposed domain-owned watch/observe ports and actual ResourceLeaseControl preserve external tool independence. Registration before baseline scan is the correct ordering, but requires a post-registration identity/reconciliation barrier rather than a single pre-watch observation.

Concrete correctness obligations:

1. Missing output: watch the nearest existing ancestor nonrecursively; on each structural event descend/rebind through newly created components. After each new subscription is active, rescan and revalidate the requested output and report invalidation. Creating a directory and immediately writing its child before the watcher attaches must not create an unobserved trusted baseline.
2. Existing directory: retain parent membership coverage alongside recursive content coverage. Rename/delete/recreate changes physical identity even if an old-inode watcher remains alive. Reobserve, attach replacement coverage, invalidate and only then retire old coverage.
3. Symlink alias: retain logical endpoint/link-parent membership and authenticated physical target content/parent coverage. Link replacement and physical target replacement are independent events. A single initial realpath is insufficient. Watch-start/observe/reobserve must reject or retry changed ancestry/target identity; physical target must remain within allowed actual workspace/custody.
4. Glob: watch the static prefix directory recursively plus parent membership, preserving literal native matching semantics for actual outputs. If the prefix is missing, use the same staged ancestor reconciliation. Do not substitute only current expansions. An empty static prefix must not silently recurse the entire workspace; use shared raw Source coverage plus explicit bounded managed-root channels or refuse unsupported scope visibly.
5. Flush: drain native pending events and asynchronous callback/reconciliation work to a stable observer epoch before output freshness responses. A single forceFlushPending call can enqueue additional reconciliation and is not a completion barrier. Serialize registration without deadlocking callbacks that request reconciliation. Stop cancels admission, drains accepted callbacks and closes all handles even after partial registration failure.
6. Cancellation/error: check actual control before and after watch startup, observation, reconciliation and baseline handoff. Preserve native callback errors and stop errors; callbacks after stop cannot republish trusted state. Registration failure rolls back newly acquired handles without releasing preexisting coverage needed by earlier records.
7. Coordinates: accept the already interpolated Nx workspace-relative expressions and normalize their parent traversal against workspaceRoot before containment checks. Windows drive/UNC and separator behavior require explicit platform vectors. Distinguish literals from glob syntax using a closed parser; unexpanded tokens or unsupported expressions must refuse rather than be partially watched. Count full absolute UTF16 path budget, not just expression length.
8. Extent/lifetime: coalesce physical roots while retaining logical alias dependencies. A recursive ancestor can replace narrower watches only after it is active and reconciled. Directory-only watches cannot stand in for recursive child coverage. Finite subscription/event limits must refuse and preserve error identity instead of dropping channels. All ordinary authored outputs, receipts and generated outputs stay covered regardless of ignore classification.

Suggested adversarial vectors: missing a/b created and written in one turn; directory inode replacement; symlink retarget with old target still alive; target replacement under unchanged link; glob member creation/removal; parent rename; same content different identity; callback error during flush; cancellation while watch startup waits; stop during reconciliation; registration overlap while an existing record is queried; clone under a generated-looking ancestor; Windows drive/case/UNC boundaries. Use neutral expected channel/transition vectors with independent Python/Ajv validation and actual filesystem/native watcher proofs. No passing execution is claimed by this review.


## Initial Core Implementation Review

Reviewed actual observation/🟦️.ts after first implementation. Staged subscriptions are active before observation identity recheck; failed/retried plans stop only newly created handles. Old handles are replaced only after new coverage passes recheck. Missing ancestry remains directory-only. Logical and target ancestry are both traversed for links, with depth and workspace-containment refusal. Mutable state is instance-owned; accepted operations serialize; callback admission/settlement reentry is explicitly rejected. Failure keeps first error identity, drains asynchronous notification, and retries previously failed handle stops on final settlement. No concrete stop/drain regression found statically.

One concrete validation gap sent to Root: root admission accepts lexical noncanonical roots such as /repo/../other or /repo//x because it checks prefix/trailing slash but not normalized equality. Require canonical root normalization before lexical containment and scope keys; include Windows drive/UNC cases. The backend root adapter may already provide canonical values, but the public planner’s own accepted input contract should not silently allow ambiguous anchors.

Only three initial tests existed at this checkpoint (neutral scope vectors, accepted-stop drain and terminal error drain); Root is adding adversarial lifecycle coverage before freeze. Consequently missing-path transition, alias target replacement, stable flush and watch-start race correctness remain required real execution evidence, not presumed from static code. No tests or Source edits by reviewer.


## Initial System Port Review

Read actual system port before runtime integration. Directory-only membership uses Node fs.watch with recursive:false; recursive coverage is an injected ready backend behind owned types. No unbounded recursive fallback found. Observation uses no-follow lstat, dev/inode directory identity, file revision identity and link target plus ctime; each path is constrained through the platform path resolver to the canonical workspace. Unexpected close/error is delivered, intentional stop waits for close and is idempotent.

Concrete small cancellation gap sent to Root: link observation awaits readlink after its last control check, then returns without a post-readlink check. The integrated core checks again after port reads, so this is not an established integrated authority escape; make the standalone port’s cancellation behavior exact too.

Windows junction readlink namespace forms and UNC roots need actual adapter vectors rather than assuming equivalent lexical roots. An initial concern about setImmediate-only Node membership flush was qualified after tracing core: every flush replans and reobserves exact file/missing/link/ancestor identities, while recursive content requires the injected backend’s explicit flush. No stale-result bug from membership yield alone was established. Keep immediate mutation-before-flush actual FS tests to demonstrate those roles and ensure alias-coordinate invalidation reaches recorded output keys. No native proof or Source edits by reviewer.


## Quiet-Window Alias Review

During the Source/report freeze, reviewed current physical-to-logical translation read-only. Content aliases preserve physical descendant suffixes at the logical recorded prefix. Link membership events invalidate the complete logical output; ancestor membership events project to that prefix. Workspace-root ambiguity projects every recorded output prefix. Synthetic reobserve transitions retain link membership coverage. No recursive alias breadth-first expansion was added. New subscription rollback, retired-stop retry and finite pending-event accounting remain present.

Concrete issue sent directly to Root during freeze: feed captured the caller-owned events array and objects for deferred queue work while using events.length both for admission accounting and eventual decrement. Backend batch reuse/mutation could retarget events or corrupt queue accounting. Required repair is a bounded owned snapshot of event rows and captured count at ingress, including receiveSource, with a neutral mutable-batch adversarial test. Root acknowledged ownership of this fix after quiet-window release00:23:57UTC. No writes occurred during freeze.

Native installed proof98298 completed cold and warm search0 with late cancellation130/4ms according to Root; progress-throttle assertion failed and physical snapshot/index reuse remained under Go owner investigation. This observer report does not claim those separate native acceptance gates passed.


## Updated Core and System Review

Reviewed current delta after Root’s readiness notice. Ingress now validates and freezes copied event rows, retains a captured batch count, and subtracts that exact count after work. This resolves the queued caller-owned array mutation finding. Notification AsyncLocalStorage stores a frame whose live flag retires in finally, so later scheduled work does not remain falsely classified as an active callback. Physical/logical mappings and finite queue/expansion bounds remain present. Post-readlink cancellation and disappearance handling are implemented. No new material lifecycle or alias blocker found statically.

Shared Windows path projection is applied before System containment. It accepts closed ordinary DOS/UNC forms, projects verbatim and NT prefix spellings, rejects device namespace/alternate-stream forms and checks projected256-unit extent. Small input validation issue sent to Root: ordinary-path invalid-character regex omits double quote; add a refusal vector if the helper promises ordinary Win32 syntax. This is not an established observer escape because containment and actual I/O still apply.

Root reports combined18tests131assertions0 including real cancellation, retargeting and inode replacement. No tests run by this reviewer. Service registration/SDK integration remains subsequent work and is not approved by this core review.


## Final Core Cancellation and Interface Checkpoint

Read latest current Source after the10k-event cancellation regression. Delivery accumulates an owned frozen outgoing batch privately, checks the actual control per event/alias and publishes/yields at the schema-owned256-work-unit interval. Cancellation during a yield throws before notify, so no partially expanded batch is externally delivered. Queued batch count is released in finally. Yielding does not mutate alias plans concurrently because reconciliation/delivery remain serialized. The callback frame still retires in finally and does not permanently poison future scheduled admission.

Within each delivery, seen(type:path) deduplicates overlapping physical/logical alias projections while preserving distinct create/delete transitions. Physical==logical aliases are omitted during planning, avoiding a second identical logical event. No new duplicate-alias lifecycle blocker found. Public scope, observation, event, port and handle interfaces expose owned shapes and built-in primitives; no Native Watcher or external test library type escapes.

Owner reports19tests136assertions green, genuine SDK kernel3tests13assertions and source-discovery2tests22assertions green, roster errors0. Exact final semantic check41299 was queued when requested; this report does not claim its terminal result. No compiler, tests or Source edits by reviewer. Core review finds no material blocker, but service/SDK integration remains separately owned and is not accepted by this checkpoint.
