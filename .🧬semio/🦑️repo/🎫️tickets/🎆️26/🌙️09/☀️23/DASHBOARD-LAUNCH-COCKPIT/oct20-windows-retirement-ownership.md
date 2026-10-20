# Windows Inactive Compiler Retirement Ownership Audit

## Current Production Boundary

Windows preview is supported through the owned tasklist/headroom observation port. Windows inactive deletion is still unsupported, and is not counted as cross-platform completion. The deletion path resolves authenticated compiler backing and exact build/target roots, then calls planNativeCompilerDescriptorObservationV1 before creating its output turn or admitting a deletion plan. That selector refuses Windows. Removing this refusal would not complete the feature: acquirePhysicalFileLocksV1 independently accepts only qualified Bun darwin/linux flock backends.

The exact current owners are:

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📂️descriptors/🟦️.ts`: closed descriptor selector, strict complete observation, exact allowed own descriptors, mounted-scope proof and root witnesses.
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/📁️physical/🟦️.ts`: actual process-held physical Cargo build locks, unchanged identity fencing and all-close lifetime.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/⚙️execution/🟦️.ts`: authentic compiler storage/writer custody, plan, physical locks, descriptor census, recensus, bounded per-unit deletion and truthful result.
- Existing pruning deletion owner: no-follow same-device census/deletion under the accepted Original.

## Primary System Contracts

The Windows physical-lock counterpart is LockFileEx, not an emulation using directory sentinels. Rust documents its Windows try_lock mapping to exclusive immediate LockFileEx; the system API takes a real file handle and preserves lock ownership until actual unlock/handle closure. This can be implemented behind the existing firstparty physical-lock interface, but requires a qualified Windows handle/file-ID and cleanup port rather than guessing how Bun numeric descriptors map to a CRT handle. [Rust File](https://dev-doc.rust-lang.org/stable/std/fs/struct.File.html), [Microsoft LockFileEx](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfileex).

Restart Manager is insufficient as a drop-in descriptor inventory. RmGetList reports affected applications and explicitly returns access denied for a registered directory. It cannot replace the current directory-witness and per-descriptor custody relation merely by treating successful process output as a complete inventory. [Microsoft RmGetList](https://learn.microsoft.com/en-us/windows/win32/api/restartmanager/nf-restartmanager-rmgetlist).

The documented Process Snapshotting API captures a specified process and returns explicit failure codes; it is a possible system port for handle names, but one successful snapshot does not prove complete cross-process coverage beneath both roots. A complete adapter must account for inaccessible processes, process incarnation changes, handle enumeration bounds and all result errors. Skipping inaccessible processes cannot establish inactivity. [Microsoft PssCaptureSnapshot](https://learn.microsoft.com/en-us/windows/win32/api/processsnapshot/nf-processsnapshot-psscapturesnapshot).

CreateFile sharing exclusion is another possible kernel custody strategy, but is not the same observation contract. Its sharing modes constrain read/write/delete opens while attribute and extended-attribute access is explicitly outside that rule. A directory handle alone also does not establish exclusive ownership of every descendant. Replacing the current complete descriptor proof with a quick exclusive-open attempt would therefore require a separately authored, complete closure-custody contract and actual Windows kernel laws. [Microsoft CreateFileW](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew).

## Required Qualified Implementation

A sound native Windows port must combine exact Windows file IDs/reparse refusal and retained actual Cargo-compatible LockFileEx locks with a complete bounded system observation or an explicitly equivalent stronger per-file closure-custody operation. It must retain the same real Original signal/deadline/progress, exact own process incarnation/handle witnesses, refusal of unknown relevant holders, before/after identity fences, recensus and per-unit cap. Every acquired handle must be transferred immediately into cleanup custody and drained after any failure, including partial acquisition and cancellation. The public DTO must stay firstparty and closed; OS structures cannot escape the port.

The actual host acceptance must cover an open regular file and directory in each root, foreign and same-process unknown handles, mapped/in-use artifacts, exact own lock/root witnesses, path replacement/reparse refusal, inaccessible or truncated observation, lock contention, cancellation and all-handle cleanup. A process-list or mocked Windows plan does not satisfy these laws. No Windows host or complete qualified Windows adapter is available in this agent session. No speculative partial production path was enabled and no unsupported Windows success is claimed. This remains an implementation/host-verification gap for the parent to carry through end-to-end acceptance.
