# Image Platform Cleanup Independent Review

Read-only comparison, 2026-10-15. No tests, Originals, production edits, mount operations, or Linux/Windows runtime claims on this macOS host. All paths below are relative to `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/`.

## Persistent ownership after successful open

Linux and Windows use `🧱️fixed/🟦️.ts` 34–48. The successful provider transfers its actual shared image lease to the returned object's close closure at 46. Close directly releases that held lease without an Original recheck or fresh command admission. As with persistent macOS open, staying mounted after successful cache-user close is intentional persistent ownership, not by itself abandoned temporary state.

The platform port at fixed 25–29 exposes only prepare and observe: there is no detach/unmount/discard/failed-provision cleanup port. Neither platform exposes the separate temporary owned-image lifecycle that macOS `provisionNativeImageStorageV1` returns. Therefore the precise macOS defect (temporary close attempts detach with canceled control) must not be asserted to be the Linux/Windows close implementation.

## P1: Failed fresh preparation has no physical reconciliation/cleanup owner

Linux `🐧️image/🟦️.ts` 11–12 runs fallocate, mkfs.ext4 and mount under the body control after creating directories. Windows `🪟️image/🟦️.ts` 13–14 creates a fixed VHD, mounts it, initializes/formats it and adds a mount path within one controlled PowerShell command. Cancellation or command failure can occur after some external effects. Neither prepare has a catch/finally unmount/dismount, loop/device reconciliation, backing deletion, or retained cleanup transaction.

The enclosing fixed opener 41 releases image EX in finally and then outer catch 47 releases its shared handle (possibly an already released predecessor); it does not undo physical prepare effects. Receipt creation occurs only after prepare succeeds and fresh actual mapping observation succeeds at 41. Thus partial/complete fresh physical material can remain without a published receipt when prepare or observation fails. No Source path establishes an explicit ownership transfer of such failure state to a recoverable retained owner. A later fresh prepare refuses an existing directory (Linux 10, Windows 13), so simple retry does not itself recover this state.

This is a concrete missing cleanup/recovery mechanism; it is not evidence that any partial mount exists now. A successfully receipt-published persistent cache failing only later reader admission is different: its durable owned image remains a persistent artifact rather than automatically needing deletion.

## Cancellation and guard behavior

All Linux preparation commands at 11–12 and Windows commands at 10–14 use the supplied Original control. Windows actual mapping observation at 18 additionally starts a new controlled command; Linux observation at 14–16 reads kernel facts directly after checking that same control. No current cleanup command reuses a canceled control because these paths have no cleanup commands at all. Adding unmount/dismount through the existing capture closures would recreate the temporary macOS canceled-admission problem unless cleanup was accepted/funded before body cancellation.

Fixed 40 catches any observe failure, including authority cancellation, as a reason to prepare. Its subsequent EX acquisition checks the same control, so an aborted Original refuses before new prepare, but error classification is broad and no physical failure reconciliation is performed. EX always releases after prepare attempt (41); returned shared close and error shared release do settle their already-issued lease/claim handles without minting fresh authority. That settles database guards, not proof of external mount cleanup.

## Platform-neutral contract boundary

A future accepted-physical-operation cleanup contract should distinguish fresh exclusive creation from existing persistent reattachment, and successful persistent transfer from failed unpublished ownership. It must retain exact image plus kernel device/mount identity and actual guard through cleanup/recovery settlement, account for command-side effects even when command completion fails, and allow only finite preaccepted cleanup actions after body authority stops. Linux loop/mount and Windows VHD/disk/partition/access-path details belong behind platform ports; a neutral contract must not assume macOS detach syntax, a boolean mounted flag, or receipt text alone proves custody. Releasing guards or terminating a child command is not equivalent to undoing the command's already committed system effects.

No implementation recommendation here authorizes physical cleanup, adoption of unknown material, or platform qualification.
