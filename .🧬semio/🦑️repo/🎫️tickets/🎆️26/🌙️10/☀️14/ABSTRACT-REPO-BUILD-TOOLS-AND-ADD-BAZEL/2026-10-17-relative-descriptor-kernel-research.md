# Bounded descriptor-relative filesystem kernel

Source/header/document research only; no FFI execution, Native operation or protected path read. The framework seam is existing `🧰️framework/📦️packages/🟦️typescript/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts`. Keep backend FFI types/private pointers behind a first-party closed API. The domain caller retains actual Core token, parent/gates, full candidate provenance and all-current-binding exclusion; a filesystem handle never grants administrative authority.

## Darwin primary local ABI

Local Xcode SDK `.../MacOSX.sdk/usr/include/sys/fcntl.h` declares openat(int,const char*,int,...), O_NONBLOCK0x4, O_NOFOLLOW0x100, O_SYMLINK0x00200000, AT_SYMLINK_NOFOLLOW0x20 and AT_REMOVEDIR0x80. sys/unistd.h declares ssize_t readlinkat(int,const char*,char*,size_t) and int unlinkat(int,const char*,int). sys/errno.h declares int*__error(void). dirent.h supplies fdopendir/closedir/readdir and warns against readdir_r.

The 64-bit sys/dirent.h record has uint64 inode/seekoffset, uint16 recordlength/namebytes, uint8 type, then name buffer. Therefore offsets are inode0, seek8, reclen16, namlen18, type20, name21. Validate record length/name length/NUL within the returned record; never blindly read the1024-byte declared name array. Copy bytes before advancing the cursor. Resolve the actual readdir symbol ABI: sys/cdefs.h maps to `$INODE64` where needed and omits suffix on64-bit-only platforms. A single assumed symbol/layout is insufficient for every Darwin architecture. Use actual ABI qualification and fail closed if unsupported.

## Linux

Kernel asm-generic defines O_PATH as1<<21, O_DIRECTORY1<<16, O_NOFOLLOW1<<17, O_CLOEXEC1<<19 and O_NONBLOCK1<<11, with architecture overrides possible. Do not transplant Darwin constants. [Kernel UAPI header](https://raw.githubusercontent.com/torvalds/linux/master/include/uapi/asm-generic/fcntl.h).

O_PATH is an identity/path handle, not a readable/syncable directory or file: most other flags are ignored. With nofollow it can hold the symlink itself. Use separate readonly directory/file handles for enumeration, content and durability. [Linux open manual](https://man7.org/linux/man-pages/man2/open.2.html).

On supported64-bit glibc ABI, dirent64 has inode0, offset8, reclen16, type18, name19, with no namlen. Layout is libc/architecture-specific; do not claim musl/32-bit without its header/proof. Glibc header declares fields but not a portable fixed record size. [glibc primary header](https://raw.githubusercontent.com/bminor/glibc/master/sysdeps/unix/sysv/linux/bits/dirent.h).

For readdir, clear thread-local errno before the call, inspect it immediately when null, and distinguish EOF from failure; d_type can be unknown, so use real descriptor facts. fdopendir owns its descriptor after success; failure leaves the caller responsible. Prefer an independently opened `openat(parentfd,".",readonly|directory|cloexec)` cursor rather than dup (shared directory offset), while retaining the original identity/durability descriptor. [readdir](https://man7.org/linux/man-pages/man3/readdir.3.html), [fdopendir](https://man7.org/linux/man-pages/man3/fdopendir.3.html).

## Lifetime and effect contract

Validate each basename as exact bounded bytes: nonempty, no slash/NUL/dot/dotdot; UTF8 must roundtrip or refuse instead of silently replacing invalid bytes. Use only actual retained parentFD+basename in openat/readlinkat/unlinkat; do not construct, pass to Node, or log an overlength absolute child. A short relative DATA path is still bounded separately. Full stat can use first-party Node fstatSync(fd,{bigint:true}), avoiding native struct-stat ABI copying.

Include CLOEXEC/nonblocking/nofollow from initial opens; reject unsupported special files, cross-device objects, directory aliases and unexpected metadata. Cursor buffers/pointers are borrowed; never retain across readdir/close or asynchronous callbacks. Bun FFI supports explicit signatures and pointer buffers, but does not make wrong ABI declarations safe. Capture errno synchronously before any other foreign/JS operation, and never leave native pointers as public API. [Bun FFI documentation](https://bun.com/docs/runtime/ffi).

Before unlink, reobserve named child via parent descriptor and compare its held FD identity; after unlink verify expected inode/link transition plus named absence, parent roster and sync the actual directory FD. unlinkat is name-based, not compare-and-unlink-by-inode: same-directory external replacement between check and effect is not prevented by this syscall. Genuine exclusive domain/catalog custody and complete stable parent proof remain essential; post-check detects uncertainty but cannot retroactively prevent removal. Never describe this as a generic atomic identity-delete primitive. [unlink manual](https://man7.org/linux/man-pages/man2/unlink.2.html).

Schema-first tests should cover EOF versus errno, failed fdopendir ownership, exactly-once close and distinct undefined/frozen body+close failures, max record/name lengths, invalid UTF8, unknown type, symlink nofollow, foreign device, replacement, hardlinks, cancellation before/after effect, fsync failure and a deliberately overlength logical subtree addressed only through short basenames. Compare actual Generated outcomes with independent Python dir_fd operations; no actual protected fixture required for Source proof. Unsupported ABI refuses before effect.

## Windows

Existing Windows witness handle owner uses absolute CreateFileW and NtQueryDirectoryFile/NtQueryInformationFile for reads; it does not already provide rooted relative deletion. NtCreateFile OBJECT_ATTRIBUTES.RootDirectory can resolve a relative object name from an existing directory handle; full path omission is feasible but needs its own first-party Windows owner, exact FILE_OPEN_REPARSE_POINT/object-attribute nofollow rules, owned handle close/cancellation, file-ID/volume checks, directory enumeration and genuine delete/durability semantics. [Microsoft NtCreateFile contract](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntcreatefile).

Current readonly Windows witness is not effect authority. A POSIX-only repair must refuse Windows before effects and keep cross-platform acceptance explicitly incomplete until a real rooted-handle mutation backend and actual tests exist. No Windows ABI implementation is proposed as already qualified.
