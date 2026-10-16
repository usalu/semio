# Windows State Metadata Durability Port Proposal

Proposed implementation, pending review: retain the current no-replace link transaction and POSIX directory-descriptor sync; add an authenticated Native-image metadata barrier that calls the documented Windows Storage `MSFT_Volume.Flush` method on exactly the retained image's NTFS volume. This avoids relying on directory `FlushFileBuffers`, a new runtime compiler, an external dependency or a caller-supplied volume string. Production Source is unchanged by this investigation. No Windows/native/storage runtime or platform spoof ran.

## Verified Microsoft Contracts

All links below were opened directly at Microsoft Learn; no search tool was used.

| API | Verified contract and design consequence |
| --- | --- |
| [MSFT_Volume.Flush](https://learn.microsoft.com/en-us/windows-hardware/drivers/storage/msft-volume-flush) | Flushes cached filesystem data for that volume to disk. It has no arguments and returns 0 for success; 1 unsupported, 2 unspecified error, 3 timeout, 4 failed, 5 invalid parameter. Minimum client is Windows 8.1. Use the exact instance already resolved from the owned image and accept only return 0. |
| [Write-VolumeCache](https://learn.microsoft.com/en-us/powershell/module/storage/write-volumecache?view=windowsserver2025-ps) | The existing Storage module exposes a volume-cache flush through a CIM input object. Its documentation includes cluster-wide behavior on failover clusters. The lower-level documented Flush method is preferred for its explicit result code; never select volumes by label, drive letter or an unfiltered collection. Reject cluster/remote storage rather than infer local fixed-volume behavior. |
| [Get-Volume](https://learn.microsoft.com/en-us/powershell/module/storage/get-volume?view=windowsserver2025-ps) | Supports selection from an exact partition/disk image and from a file path. Use the actual Get-DiskImage → one disk → one partition with the exact retained access path → one NTFS volume chain, then compare the directory's actual volume to that same volume. |
| [Invoke-CimMethod](https://learn.microsoft.com/en-us/powershell/module/cimcmdlets/invoke-cimmethod?view=powershell-5.1) | Invokes an instance method. An input instance can retain a remote session; therefore construct the instance locally in the system child and forbid CimSession/ComputerName/remote inputs. No CIM type crosses the repository interface. |
| [FlushFileBuffers](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-flushfilebuffers) | Its file handle requires GENERIC_WRITE. Microsoft separately documents flushing all open files via a volume handle and requires administrative privileges for that route. This does not establish directory-handle flushing. Direct volume-handle flushing is a documented alternative but would require an additional first-party FFI/helper; it is not the minimal proposed route. |
| [Directory Handles](https://learn.microsoft.com/en-us/windows/win32/fileio/obtaining-a-handle-to-a-directory) | A directory handle uses FILE_FLAG_BACKUP_SEMANTICS. The documented directory-handle function list does not establish a directory FlushFileBuffers contract. Do not append win32 to the existing POSIX allowlist and call Node FileHandle.sync on a directory. |
| [CreateFileW](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew) | Unicode paths are supported; directory handles require backup semantics. Write-through can flush NTFS metadata changes caused by that request, but this is not a documented substitute for the later independent hard-link/unlink namespace transaction. Avoid a runtime Add-Type/C# or new FFI implementation merely to obtain this flag. |
| [File Caching](https://learn.microsoft.com/en-us/windows/win32/fileio/file-caching) | Metadata is cached; an explicit flush/write-through barrier is required. A successful in-memory readback alone is not a durability receipt. |
| [CreateHardLinkW](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createhardlinkw) | NTFS file links must remain on the same volume, and directories cannot be hard-linked. This matches the existing file-record transaction; directory creation needs its own metadata barrier. |
| [MoveFileExW](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefileexw) | WRITE_THROUGH explicitly describes flushing a copy/delete move. Switching the transaction to rename would introduce a different recovery protocol and would not establish mkdir durability. Keep the current transaction. |

The selected CIM filesystem-cache barrier is a documented OS request, not proof of arbitrary hardware/power-loss behavior or of nested VHD cache propagation. Native qualification must exercise the actual fixed VHD stack, including the persistent child image inside Products. Do not invent or rely on an undocumented FlushVirtualDisk function; the attempted direct documentation URL did not establish such an API.

## Existing Integration to Reuse

`PROCESS/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts` already resolves Windows system PowerShell, uses `-NoLogo -NoProfile -NonInteractive`, sanitizes the environment, and constructs Get-DiskImage/Get-Disk/Get-Partition/Get-Volume commands. Its retained receiver keeps the backing inode, receipt, actual mount mapping, root identity, actual control and live shared/maintenance borrower in private state. The observed Windows mapping contains image/device/disk/unique/partition/volume/bytes. Reuse those actual private facts; copying JSON is never a volume capability.

The preparation mutation command path requires a private EX writer proof and is restricted to allocate/format/activate/information/inactive/deactivate. A normal persistent-State metadata flush occurs under a live image borrower and a State write operation, so it must have its own closed authenticated operation. Do not label it information, route it through the raw read helper, add an EX-proof bypass, or enlarge the preparation command quota.

## Concrete First-Party Interface

Proposed owner: `PROCESS/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🧾️metadata/` with `🟦️.ts`, schema and platform implementations behind it. State records retain their current owner and consume this capability explicitly.

```ts
declare const nativeMetadataBrand: unique symbol;
export interface NativeImageMetadataV1 {
  readonly [nativeMetadataBrand]: true;
}
export function withNativeImageMetadataV1<T>(
  image: NativeImageStorageV1,
  control: ResourceLeaseControl,
  body: (metadata: NativeImageMetadataV1, actual: ResourceLeaseControl) => Promise<T>
): Promise<T>;
export function syncNativeImageMetadataV1(
  metadata: NativeImageMetadataV1,
  directory: string,
  control: ResourceLeaseControl
): Promise<void>;
```

The issuer verifies the actual platform-image WeakMap binding and actual phase refinement; it enters the existing retained image operation. It privately captures the real receiver/kernel mapping and creates a noncopyable live token for the callback. Each sync verifies the token, same-image path custody and phase, retains actual directory identity, performs the platform barrier, and rechecks the descriptor/name and image/kernel identities. The token closes births and drains all accepted syncs before returning. No callback supplied by a public caller can impersonate a successful barrier. JSON result data never issues the token. Node Stats/FileHandle, CIM objects and OS handles remain private.

State record write and directory-preparation functions take this capability as a mandatory argument. `openBound` uses a capability for the Products parent to publish its intent/binding. `openData` and all accepted data rechecks use one for the actual persistent child image. Greenfield changes update every call site at once, with no optional compatibility parameter. Read-only record decoding does not need to issue durability authority.

Important close order: wrap each relevant record/data lifetime in the metadata callback, drain State operations, return from that callback, then call `image.close()`. Calling image.close while its enclosing metadata/image operation is still pending would create a self-drain cycle. The parent image remains held by the existing caller throughout. Tokens must not escape to the public directory-data task.

## Windows System Operation

One owned PowerShell child per accepted barrier:

1. Authenticate the live actual Native image and supplied descendant before child birth; capture directory and backing identities.
2. Use the same canonical system-program resolver and clean local environment pattern. Resolve the exact VHD through the existing image/disk/partition/access-path chain, require one local NTFS volume, and compare every mapping field to the private retained receiver. Resolve the target directory's volume and require that exact same ID. Reject host boot/foreign/UNC/cluster volumes, nested foreign mount redirection and unknown ancestry.
3. Invoke `Invoke-CimMethod -InputObject $actualVolume -MethodName Flush -ErrorAction Stop`, using only the locally constructed exact object. Accept numeric ReturnValue 0 only. No AsJob, remote session, labels, wildcards, background process or raw user-supplied PowerShell body.
4. Re-observe the image/disk/partition/volume mapping in that same child, return a closed bounded receipt, then await the child's full process/stream closure.
5. Recheck the actual image/root/directory identities and control in the parent before accepting the barrier. A changed or unproved mapping, malformed receipt, nonzero method result, transport error, stopped child or failed close refuses successful completion and Native commit; any linked transaction remains for exact recovery.

All path text is passed through the existing literal escaping as DATA, without shell execution. Validate canonical absolute paths and both Unicode/UTF16 bounds at 256 before dispatch; use the retained mount's volume ID rather than drive-letter or C: inference. Use one pending child, the existing record-output size bound or smaller closed receipt, and the current phase's remaining budget/signal/progress. No new Original, deadline, cap, quota or independent timer is issued.

A PowerShell system tool is not a Semio receiver and must not mint an Original. Its owned capture is authenticated and bounded by the same actual issued control; the system environment can remain clean instead of leaking the application envelope into PowerShell. Every accepted process and publication/progress callback must drain before the Native borrower/State scope closes. On cancellation, preserve an arbitrary caught primary (including undefined/frozen objects) and retain the incomplete transaction for exact recovery; a durable-success receipt is never inferred from attempted work.

## Privilege and Zero-Touch Limits

The metadata operation introduces no package/runtime library and no C#/Rust compiler bootstrap. It uses Windows Storage/CIM system facilities already adjacent to the existing Windows image route. Capability/method/provider availability and actual permissions must be checked on the live admitted image before State reservation where possible, with actual system failures surfaced and no elevation prompt or fallback filesystem directory. The Microsoft Flush method page does not state a privilege exemption, so this proposal does not claim that it works for unprivileged users. The documented direct volume-handle alternative explicitly requires administrative privileges.

Existing image attachment already has a privilege requirement: [AttachVirtualDisk](https://learn.microsoft.com/en-us/windows/win32/api/virtdisk/nf-virtdisk-attachvirtualdisk) documents SE_MANAGE_VOLUME_PRIVILEGE. That is not proof that an account also has permission for every proposed CIM flush; validate the actual operation. Do not introduce automatic UAC, a service or a broader grant as an unreviewed fix.

Two existing zero-touch concerns remain independent of this metadata port. The current resolver hardcodes `C:\Windows` even though [GetWindowsDirectoryW](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getwindowsdirectoryw) documents retrieving the actual Windows directory; do not duplicate that assumption in a new helper. Current allocation calls the [Hyper-V New-VHD](https://learn.microsoft.com/en-us/powershell/module/hyper-v/new-vhd?view=windowsserver2025-ps) cmdlet. Microsoft documents Hyper-V as an optional feature with elevated setup and no Hyper-V role on Windows Home in its [installation requirements](https://learn.microsoft.com/en-us/windows-server/virtualization/hyper-v/get-started/install-hyper-v). The existing Source therefore does not establish zero-touch support for every Windows edition/privilege configuration. A metadata-only change cannot honestly claim to solve those backend provisioning constraints.

## Schema-First Fault Laws

The new closed schema defines operation `sync-metadata`, platform, same-image request DATA, canonical bounded directory, and a result with exact before/after mapping plus ReturnValue 0. It defines one pending command, bounded receipt bytes/lines and no free-form program/action/volume selector. DATA validation is independent of private capability authentication. Existing record transaction states and recovery semantics stay unchanged.

Before implementation, extend the permanent State corpus with:

- exact macOS/Linux descriptor success and exact Windows owned-volume success;
- unknown/copied/closed token, copied/foreign/expired/stopped controls, all with zero dispatch;
- wrong image, changed backing/receipt/root/directory identity, junction escape, foreign/nested volume, multiple disks/partitions/volumes, non-NTFS and remote/cluster instances;
- missing method/provider, permission failure, CIM transport failure and each documented nonzero Flush return;
- stop before dispatch, during the system call, after Flush but before receipt, during progress, and before parent recheck;
- malformed/oversized/extra-field receipt, child exit 0 without a valid Flush result, and changed mapping after successful Flush;
- directory creation followed by barrier; synced prefix/link/unlink followed by barrier; crash/restart recovery at each transition; unknown existing bytes survive;
- concurrent metadata calls remain serialized and scope close awaits child/stream/progress drain; undefined/frozen primary survives any secondary cleanup error;
- Native image close occurs strictly after metadata scope close, preventing the self-drain cycle.

Run actual function Source models with explicit physical/system ports and genuine Originals/issued phases, independent Ajv plus SQLite trace/identity oracles, then focused strict checks. Do not count a PowerShell text or Windows platform-string model as Windows runtime proof. Existing low-level filesystem transaction tests need honest separation between bounded Source laws and genuine Native-image physical qualification; their host temporary path cannot authorize flushing the host's entire Windows volume. Any necessary physical task must be registered through existing 📜️script.ts/project/editor routing and use an actually issued disposable image, not a fake Native image or raw volume path.

Finally qualify the actual Windows NTFS fixed-image stack, including nested State backing and process cancellation/drain, and independently rerun macOS/Linux transaction tests. An ordinary reopen validates recovery/readback, not power-loss persistence; any crash-durability claim needs a suitably controlled crash/power-loss test. No allowlist widening, Native physical run or Source implementation is authorized by this proposal itself.


## Native Journal Integration Addendum (Awaiting Review)

Root identified the same raw directory `fsyncSync` assumption in Native storage checkpoint and first journal creation, plus the new derived Products receipt. A State-only capability is insufficient for native Windows storage tasks. The proposed contained shared contract is a mandatory first-party method on the existing neutral `NativeStoragePortV1`:

```ts
syncMetadata(backing: NativeStorageBackingV1, directory: string, control: ResourceLeaseControl): Promise<void>;
```

Both real image ports implement it under their actual retained borrower. Each method checks the captured exact backing identity/root and actual descendant control before effects. Fixed Windows delegates to the new private retained-receiver CIM barrier; POSIX ports use an adjacent neutral descriptor/ancestry helper. Native storage imports no platform dispatcher at runtime: it calls its supplied enforcing port, preserving the existing storage → injected port boundary. The public metadata capability authenticates the real platform image and obtains its exact backing through the existing port.observe inside its retained operation; each sync then uses this same port method. No caller volume string, callback, copied backing DATA or Repository type issues the capability.

The Native journal's current synchronous append/checkpoint callbacks can remain synchronous. Within its already serialized transaction, mark metadata dirty after any accepted append or checkpoint rename, invalidate cached projection authority on mutation, then await `port.syncMetadata(backing, root, actualControl)` once after the callback and before publishing success/returning/unlocking. Multiple synchronous writes in one transaction are covered by the final barrier. File fsync and exact replay/readback remain. If the callback, append or barrier fails, do not claim rollback: authoritative bytes/linked transaction remain for exact recovery and no durable-success receipt is returned. Cleanup still releases the accepted journal gate and drains the system child before image/maintenance closure.

The new derived Products receipt writer should receive the actual maintenance port/backing and await that same method after its file publication, within the existing EX callback. `withNativeImageMetadataV1(image, custody.control, ...)` can also nest under genuine maintenance because fixed `port.withOperation` closes over the current private receiver.owner, replaced by its actual maintenance owner. No `image.close()` is called inside that operation. A direct actual-port method avoids needing a platform object in the generic journal and avoids NativeStorage ↔ platform runtime cycles.

Update every typed port fixture to the mandatory method with an explicitly labeled metadata effect port; no optional POSIX compatibility fallback. Extend laws with one final barrier after multiappend/checkpoint, no success or cached projection after failed barrier, immutable replay after failure, exact backing/phase before dispatch and close/drain order. Existing host temporary filesystem transaction tests use explicit metadata Source ports and remain distinct from future genuine Native-image Windows/macOS/Linux qualification. Root has not authorized this addendum's additional production edits yet.
