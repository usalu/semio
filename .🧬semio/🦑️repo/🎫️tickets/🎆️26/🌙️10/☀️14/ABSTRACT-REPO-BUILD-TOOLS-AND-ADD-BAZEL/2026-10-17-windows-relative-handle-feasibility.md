# Windows Relative Handle Feasibility

Read-only Source design for Root's descriptor-relative filesystem kernel. No Windows DLL, Native/protected root, cache path or existing257-character leaf was opened. Selected Tools current-plan fifteen-file Source and independent hold stay frozen. Existing Windows end-to-end remains unqualified.

## Documented Primitive And ABI

`NtCreateFile` supports child names relative to a genuine directory HANDLE in OBJECT_ATTRIBUTES.RootDirectory. Use a single validated basename and FILE_OPEN, never creating/overwriting. Desired access is role-specific; read and delete handles are distinct. Open the leaf itself without reparse processing, then reject wrong type/reparse/volume/file identity before use. SYNCHRONIZE is required with synchronous I/O options. Status and IO_STATUS_BLOCK must be complete; pending/alternate status never implies success. These are feasibility constraints, not evidence that each proposed flag combination has run on Windows. [Microsoft NtCreateFile](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntcreatefile).

OBJECT_ATTRIBUTES declares ULONG, HANDLE, name pointer, ULONG and two further pointers. On x64 the proposed48-byte buffer offsets are0/8/16/24/32/40; null security pointers and no inheritance. This layout follows the documented C declaration plus x64 alignment. [Microsoft OBJECT_ATTRIBUTES](https://learn.microsoft.com/en-us/windows/win32/api/ntdef/ns-ntdef-_object_attributes).

UNICODE_STRING lengths are byte counts, excluding the trailing null from Length. Proposed x64 buffer16 bytes, offsets0/2/8; actual UTF16 basename bytes and both buffers remain retained until the operation is proved complete. [Microsoft UNICODE_STRING](https://learn.microsoft.com/en-us/windows/win32/api/ntdef/ns-ntdef-_unicode_string).

The ticket windows-relative JSON/schema fixes those Source layout expectations. An independent Python ctypes oracle with explicit32bit Windows ULONG and64bit pointers matched all3 structures/11 offsets. IO_STATUS_BLOCK declares a status/pointer union plus ULONG_PTR Information; its16-byte x64 layout has Information at8. Microsoft states that a nonpending return status governs and pending requires actual completion before interpreting the status block. Windows runtime binding is still unqualified. [Microsoft IO_STATUS_BLOCK](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/ns-wdm-_io_status_block). Generated/windows-relative-layout-oracle.json is Source DATA only. No numeric HANDLE is accepted from a caller or returned publicly.

## Public First-party Interface And Ownership

Keep Root's neutral first-party rooted API: rootStat, complete bounded list, probeChild/kind, withDirectory, withFile, readlink, unlink(expected seven-field identity,kind), sync. Constructor creates and privately owns the actual anchor; every receiver is exact/live and every child operation uses one validated basename plus its already-held parent handle. Child scopes and DLL/buffers stay within the same existing32-owner custody registry; copied/escaped/closed scopes refuse before calls. Actual domain ADMIN owner retains genuine Core token/gates/root/backing/policy/raw-row checks. The filesystem capability never mints or restores Native authority.

A label or numeric descriptor must not upgrade read-only Core/Windows observation context. Existing Windows publication read mechanics depend on the filesystem descriptor registry, so importing that high-level module back into filesystem would create a dependency inversion. Preferred division: Root owns neutral interface/schema/shared32-owner and POSIX region; interfaces owns an additive Windows region in the same existing descriptor module, with primitive ABI constants/Source vectors folded into its existing schema/tests after names are agreed. Existing read-only Windows publication methods stay read-only. No provider-registration/effect callback or fake image is introduced.

Windows facts must privately retain actual volume serial and128bit file ID, reparse/type, change/write/security/stream information; Node-compatible seven fields alone are insufficient to assert strong Windows object identity. Root must distinguish a genuine mapped Native root anchor from an ordinary physical ancestor; intentional volume mount-point binding cannot be mistaken for permission to follow arbitrary child reparse points.

Names reject slash/backslash, colon/ADS, null/control, dot/dotdot, DOS devices, trailing dot/space and malformed UTF16. Existing deep leaf traversal builds no full child pathname; data may retain relative component vectors. No new over256-character filesystem path is created. Full existing Native GUID observation must not be forced through a256-character absolute child-path decoder.

## Deletion And Sharing

Existing readonly Windows handles allow read sharing and deny delete. A second DELETE open while an outer withFile handle remains open can therefore fail even in the same process. Do not broaden all read handles to share delete. Prefer read callback fully drained/closed, then parent.unlink opens a fresh delete-capable exact-CAS child handle, marks disposition, closes, and probes absence relative to the still-held parent. Alternatively an explicitly owned same-handle delete method needs its own contract. This is a required neutral caller ordering decision before implementation.

SetFileInformationByHandle FileDispositionInfo requires DELETE access. Classic deletion completes on handle closure, so marking alone cannot report removal. [Microsoft SetFileInformationByHandle](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfileinformationbyhandle), [FILE_DISPOSITION_INFO](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_disposition_info).

The extended disposition has explicit DELETE/POSIX/force-image-check/on-close/ignore-readonly flags. POSIX behavior removes the visible link after closing the delete handle while other allowed handles can retain streams. Source must distinguish file/directory/link and verify actual absence and remaining roster; hardlink semantics cannot be inferred from a boolean result. Readonly-attribute override is a separately authored fixed selected-role decision after exact expected facts. No fallback to raw DeleteFile by reconstructed full path. [Microsoft FILE_DISPOSITION_INFORMATION_EX](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_file_disposition_information_ex).

## Durability Is Separate

FlushFileBuffers requires GENERIC_WRITE. Microsoft documents whole-volume flushing with the exact volume GUID handle without a trailing slash and administrative privileges. The documentation does not establish that the existing readonly directory handle supplies metadata durability. Therefore sync cannot silently return success or alias the readonly observer. [Microsoft FlushFileBuffers](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-flushfilebuffers).

Existing canonical fixed-publication metadata owner already authenticates its specific NTFS containing volume then uses volume Flush. Its publication-only request cannot be fabricated for maintenance. For a generic rooted Windows directory kernel, directory durability needs a verified writable handle/NTFS operation; otherwise a distinct fixed actual Native maintenance owner must flush its genuine dedicated volume. No arbitrary caller-provided volume string/check callback grants that scope. Keep unsupported Windows refusal until a real mechanism, Source failure/drain laws and actual Windows fixture acceptance exist.

Required Source cases: nonzero/alternate/pending status, sharing conflict, copied/foreign/closed root, malformed basename/UTF16, reparse/ADS/security/file-ID drift, same-ID-looking metadata replacement, live child during unlink, close refusal with arbitrary primary, cancellation/expiry, partial disposition/late roster, flush failure and receiver reuse. Independent ctypes/SQLite/Node oracles must agree with actual production bodies; Windows ABI Source models are explicitly not actual Windows runtime acceptance.


## SDK Constants Confirmed For Schema-first Work

The published Microsoft SDK header confirms the following fixed values. They are declarations for Source-model packet tests; compatibility of the combined options still requires the real receiver guard and Windows tests. [Microsoft win32metadata winternl.h](https://raw.githubusercontent.com/microsoft/win32metadata/main/generation/WinSDK/RecompiledIdlHeaders/um/winternl.h).

| Constant | Hex |
| --- | --- |
| FILE_OPEN | 0x00000001 |
| FILE_OPENED | 0x00000001 |
| FILE_DIRECTORY_FILE | 0x00000001 |
| FILE_NON_DIRECTORY_FILE | 0x00000040 |
| FILE_SYNCHRONOUS_IO_NONALERT | 0x00000020 |
| FILE_OPEN_REPARSE_POINT | 0x00200000 |
| FILE_COMPLETE_IF_OPLOCKED | 0x00000100 |
| OBJ_DONT_REPARSE | 0x00001000 |

No complete-if-oplocked alternate success is accepted as a complete open; no requiring-oplock flag is proposed because that introduces a mandatory later oplock protocol. Ordinary traversal refuses reparse; an explicit readlink/unlink-link role needs the open-reparse leaf branch and exact supported tag decoding without following its target. Root anchor mapping is a separate role.
