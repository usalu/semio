# Concurrent Persistent Source Review

Read-only Source review on 2026-10-16, after the frozen whole-operation checkpoint. No image/system command or test of these new persistent functions ran in this lane. The latest peer Source is preserved. This report does not supersede native-image-cleanup-current.md defining hashes or turn later Source changes into tested evidence.

The platform owner now again exports prepareNativePlatformStorageV1 and imports prepareNativeLinuxImageStorageV1/prepareNativeWindowsImageStorageV1, which did not exist at CPU strict39580. The new macOS prepareNativeImageStorageV1 calls acceptNativeImagePersistentReceiverV1, performs effects, commits, then cleanup settlement releases EX and returns void. Caller subsequently opens a separate SH.

## Unresolved Actual Receiver Responsibility

In cleanup/command commitNativeImagePersistentReceiverV1, the private receiver record contains only scope and owner. commit sets scope.persistent.committed=true and deletes the receiver WeakMap entry. In settleNativeImageCleanupV1 the committed branch sets scope.settled=true and deletes the strong accepted owner. prepareNativeImageStorageV1 then releases its actual lease and returns void. No concrete NativeImageStorage object, retained receiver guard, receiver opening promise, or responsibility for the later separate SH acquisition is retained by that record.

This is a Source-level repetition of the previously named receipt/boolean handoff frontier: private construction and real EX are useful, but deleting the receiver before actual reader/port ownership transfers does not establish the approved receiver lifecycle. The plan requires the concrete receiver to retain the unchanged EX/control, acknowledge responsibility, and itself own the EX-to-SH acquisition/final-verification gap. No runtime leak or escape is claimed here.

## Exact Own Partial Receipt

Fresh commit writes image.json using writeFileSync and only then stamps/adds it to the created inventory. If the synchronous write opens/partially writes and throws, cleanup retains the unknown entry rather than removing it. Retention is safe and bounded, but known-own partial cleanup is unqualified. Capturing the actual creator descriptor/inode immediately after wx/nofollow open before write would let the accepted owner retire only that exact partial receipt, preserving the primary failure.

## Scope Coordination

Storage lane has asked Root and native_cleanup_execution who owns this concurrent persistent implementation before editing its private authority. The newly staged unimported receiver schema/corpus in cleanup/receiver is planning only and has not been executed or wired. No duplicate permanent receiver/authority API should remain after coordination.

## Root Review Of The Concrete One-Opener Successor

2026-10-15 22:28 UTC. The earlier prepare/boolean frontier above is superseded by the concrete one-opener Source epoch recorded in native-image-cleanup-current.md. Storage's frozen successor passed receiver BOTH3/106 and Core/provider strictBOTH0. Root read the actual command owner, borrower, image delegation and receiver laws. No image/system/physical command ran. Composite repository caller typing remains separate while the mandatory store phase migration is active.

The actual receiver now retains its real guard and issuance control, concrete image/port, exact backing/root/receipt/kernel facts and pre-reserved bounded unresolved slot. Receipt descriptor identity is captured before write and partial extent is retained for exact rollback. Creator settlement authenticates retained EX before acknowledging durable receiver responsibility; the same receiver owns subsequent EX release, genuinely separate original-phase SH acquisition and final verification. This resolves the specific earlier boolean/deleted-owner finding at Source level. Actual gap/replacement behavior remains unexecuted.

Root found a new failed-open cleanup defect. In the mounted fast branch, stage becomes ready and image.recheck is awaited before return. Rejection changes stage to unresolved while retaining its SH/guardOwner without returning the image. Post-gap inspection/recheck rejection similarly retains newly acquired SH. That read-only guard can prevent later maintenance until process termination. Existing closeNativeImageBorrowerV1 already drains accepted operations and releases its exact lease even if verification rejects; failed open can invoke that cleanup while preserving the primary, without detach or publication deletion. Storage confirmed the finding and is adding genuine ordinary-SQLite expiry/release plus exact actual catch-path Source regression. Uncertain creator EX/cleanup responsibility must remain retained; this finding does not authorize releasing it.

Root separately reviewed the successor store outer/inner ordering. Actual image.open/close now lie inside a live outer requested phase; reservation/body run in a genuine inner requested phase whose canonical settlement joins direct Core children before image close. Private consumer frames drain separately. The staged direct-Core/throw-undefined image laws remain unexecuted; protocol/AST/negative-control tests alone do not establish actual store runtime.
