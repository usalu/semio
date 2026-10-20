# Windows Native Maintenance Custody

## Confirmed Current Gap

The actual current owner prepares a POSIX descriptor plan before preview, captures ps, and for retirement requires physical compiler flock plus complete lsof custody. Its physical lock backend explicitly accepts Darwin/Linux only. Thus Windows preview and retirement are not yet usable; cross-platform Source vectors cannot establish a Windows runtime pass. No Windows native command has been executed.

## Primary System Contracts

- [CreateFileW](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew): share mode0 excludes read/write/delete opens and rejects incompatible existing access. Attribute-only access is not excluded. Excluding FILE_SHARE_WRITE also rejects writable file mappings. Therefore an exclusive open is a kernel exclusion capability for ordinary I/O, not a complete inventory of all handles or read-only mapped views.
- [LockFileEx](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfileex): byte-range exclusion does not cover memory-mapped views. Replacing flock with this call alone cannot replace complete foreign-reader custody.
- [RmGetList](https://learn.microsoft.com/en-us/windows/win32/api/restartmanager/nf-restartmanager-rmgetlist): directory resources return access denied; it describes registered application/service resources. No documented complete recursive handle census follows. Restart Manager is not accepted as the authority proof.
- [FILE_INFORMATION_CLASS](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/ne-wdm-_file_information_class): FileProcessIdsUsingFileInformation47 is reserved for system use. It is not a documented public complete-census replacement.
- [FILE_DISPOSITION_INFORMATION_EX](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_file_disposition_information_ex): non-POSIX disposition waits for final handles; FORCE_IMAGE_SECTION_CHECK protects image sections. A failure can indicate an existing mapped view. The documented image check does not alone prove absence of every read-only data mapping.
- [Section synchronization and oplocks](https://learn.microsoft.com/en-us/windows-hardware/drivers/ifs/fs-filter-acquire-for-section-synchronization2): writable section creation breaks modern oplocks immediately; this does not establish equivalent protection for all read-only views.
- [Requesting and granting oplocks](https://learn.microsoft.com/en-us/windows-hardware/drivers/ifs/granting-oplocks): exclusive oplock admission is asynchronous and must retain its pending request until break. Dropping or treating a pending request as a permanent census would be invalid.

## Boundary for a Sound Provider

A Windows provider must either prove complete foreign custody through a documented system facility, or expose a distinct kernel conditional retirement operation with precisely documented exclusions and observable settlement. A successful exclusive file open must not be labeled a complete process/descriptor census. It must retain physical nofollow ancestors, exact file identity and single-link checks through publication/deletion, preserve finite Original cancellation, reject every access-denied/unsupported/incomplete result, and derive physical headroom only from a fresh statfs observation after settlement. No such provider has been installed or claimed complete at this research checkpoint. Existing strict refusals remain intact.

## Concurrent Test Port Alignment

The actual maintenance owner regression now supplies the added directory witness port, calling the provided guard before/body/final settlement and exposing the exact directories. It also preserves the new descriptors.result.json write separately from the retained plan evidence. This is Source authorship only; scheduled Source26 owns execution.

## Authorized Portable Read-Only Preview Slice

The owner now separates preview from destructive custody selection. A schema-authored local process plan uses ps on Darwin/Linux and tasklist.exe /FO CSV /NH on Windows ([Microsoft tasklist contract](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/tasklist)); this is read-only context, never a handle census or retirement capability. Existing capture exit/status/signal/stderr completeness checks remain. Available bytes use the existing nativeDirectoryHostAvailableBytesV1 interface, which already selects POSIX statfs or Windows GetDiskFreeSpaceExW. The unchanged issued control is checked before and after this single host observation; invalid/unsafe numeric observations are refused. Windows destructive retirement still selects the unsupported descriptor provider before owner effects.

Four neutral platform rows have a SQLite lookup oracle. Actual owner extraction covers preview dispatch on all three modeled platforms with no descriptor/physical/delete port, and Windows denied/truncated/headroom-failure cases refuse before plan evidence or deletion. A real issued-control helper law covers cancellation after the host observation and unsafe sizes. These are Source changes only: no Windows host execution or Source suite pass is claimed.

The next sequential native plan is retained as hand-authored input e2e-native-runtime-plan.json. It starts with the exact terminal owner manifest and three editor controller laws, then the canonical build/install and advertised installed authority/execution/journeys/smoke/windows routes. It never calls the human dashboard entry.

## Exact Additional Task-Owned Paths

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/👁️observation/🟦️.ts` (created semantic library)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/e2e-native-runtime-plan.json` (created hand-authored runtime input; retain during output cleanup)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/oct20-windows-native-maintenance.md` (created research report)

The existing budget execution/schema/fixture/tests/feature paths were updated in place and remain in the prior exact source inventory.

## Separate Windows Conditional Retirement Design (Review Only)

A separately declared Windows capability can aim to retire inactive immutable compiler artifacts safely without claiming a complete foreign-handle or mapped-section census. Its schema must name a different custody kind from the existing POSIX complete-observation protocol. The present POSIX guards stay unchanged; Windows retirement remains unsupported until this contract and native port are implemented and tested. Portable read-only preview already has a separate source-reviewed path.

### Primary API Boundary

[CreateFileW](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew) documents that share mode zero conflicts with other read/write/delete access, and omitting share-write also conflicts with a writable file mapping. Attribute-only access is outside that sharing rule. Successful exclusive open is therefore an exclusion of conflicting mutable access, not a complete census. Use OPEN_EXISTING and no truncating or write-content operation.

[SetFileInformationByHandle](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfileinformationbyhandle) operates on an existing appropriately authorized handle. The [extended disposition specification](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-fscc/2e860264-018a-47b3-8555-565a13b35a45) provides DELETE and FORCE_IMAGE_SECTION_CHECK, with separate POSIX and ignore-readonly options. The proposed capability excludes the latter options. The [driver reference](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_file_disposition_information_ex) describes deferred non-POSIX deletion and refusal for an existing mapped view; filesystem and OS support must be qualified on an actual Windows host, rather than inferred from a mocked successful call.

[FILE_ID_INFO](https://learn.microsoft.com/en-us/windows/desktop/api/winbase/ns-winbase-file_id_info) combines volume serial and128-bit file ID to compare pinned objects. A later implementation should reuse the repository stable-volume binding and domain-neutral system port rather than export Win32 handles or third-party types. The Microsoft [STL implementation](https://github.com/microsoft/STL/blob/main/stl/src/filesystem.cpp) confirms the public FileDispositionInfoEx call spelling, but its POSIX deletion fallback is intentionally not the proposed contract.

### Proposed Finite Operation

1. Authenticate the existing Original, a finite requested maintenance phase, current canonical compiler writer and exact physical backing root. Snapshot a closed bounded inactive-unit plan; keep current/in-use/source files out.
2. Pin all ancestry and candidate entries without following reparse points. Verify final volume/file IDs, ordinary-file types, bounds, exact metadata, hard-link count and unit membership against the plan. Keep directory identities pinned while processing. A changed, inaccessible or unsupported object is a typed refusal, never an assumed absence.
3. For each exact file, request DELETE plus metadata access with share mode zero and OPEN_EXISTING. Retain the exact handle through disposition and close. Refuse conflicting opens, writable mappings, unsupported disposition flags and image-section checks. Never change file contents, lengths, attributes, ACLs or existing mappings to make retirement succeed.
4. Request non-POSIX deletion with image-section checking. Close owned handles and settle the kernel result. If a directory gained a child, nonempty deletion must fail rather than recurse into the new entry. A partial result records exactly which accepted objects settled; it must not claim atomic retirement of the whole unit. Cancellation stops before the next irreversible step and reports already settled effects.
5. Measure actual physical availability through the existing first-party headroom port after handle settlement. Report observed availability and remaining headroom deficit separately from planned logical bytes and accepted deletion bytes. Concurrent allocation prevents attributing every availability delta to this command. Deferred mappings or other kernel retention can yield unreclaimed bytes; success must not mean that all planned logical bytes became free.

### Proof and Runtime Gates Before Implementation

The final contract must choose an honest outcome for a surviving immutable read-only mapping: refusal or preserved-readable pending reclamation, with no absence-of-readers claim. Required Windows integration laws include open-reader, writable-map, readonly-map, executable-image, hard-link, reparse substitution, foreign child insertion, unsupported filesystem, permission refusal, cancellation between files, kernel refusal after a partial unit, and physical availability that does not rise. Unit handles need once-only cleanup on every path. The native adapter must expose exact typed receipts/errors behind an owned interface; schema vectors plus an independent SQL/state-machine oracle cannot replace these Windows kernel tests.

No conditional-retirement production code was added in this research pass. The design is ready for architectural review; the important qualification is preservation of immutable foreign data, not equivalence to the existing complete POSIX census.

The final primary API cross-check found FileDispositionInfoEx in the public [FILE_INFO_BY_HANDLE_CLASS](https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ne-minwinbase-file_info_by_handle_class) enumeration. Public [DeleteFileW](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-deletefilew) documentation independently distinguishes deferred deletion from POSIX semantics and describes open-I/O/mapping sharing constraints. These pages support choosing explicit non-POSIX refusal semantics; they do not establish a global inventory. Low accepted this design as research only, with the readonly-map outcome and actual Windows filesystem qualification still required before implementation.
