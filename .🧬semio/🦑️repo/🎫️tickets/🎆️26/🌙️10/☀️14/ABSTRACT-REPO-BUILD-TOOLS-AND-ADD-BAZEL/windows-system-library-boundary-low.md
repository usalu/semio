# Windows System Library Boundary Review

Read-only bounded Source census found the existing Bun Win32 FFI boundary at `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎰️jobserver/🟦️.ts:89–109`. Its private `semaphoreChannel` dynamically imports bun:ffi, opens literal kernel32.dll, defines explicit pointer/i32/u32 signatures, calls UTF-16 CreateSemaphoreW, and closes handle/library through its returned private TokenChannelV1. Public jobserver APIs do not expose Bun FFI types. This is a reusable internal pattern, not an existing trusted system-directory resolver or a proof of loader provenance. It does not validate CloseHandle success and does not clean up the library if CreateSemaphoreW fails; a new resolver should not inherit those lifetime gaps.

Filesystem observation and its descriptor helper contain no Win32 FFI/system-library resolver. `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:1–5` supplies node:fs nofollow flags and a boxed arbitrary-primary asynchronous descriptor closer only. PROCESS leases physical owner has a POSIX Bun FFI interface, not Windows.

The proposed resolver can keep Bun imports/signatures private and return only a validated string through a repository-owned interface, with fixed bounded UTF-16 buffer/length checks, no environment/CWD fallback, and a primary-safe library close. The explicitly accepted trusted-process Kernel32 import assumption remains an assumption to document; this census establishes no stronger trust primitive. Existing pinned-Bun ARM64 FFI limitation remains the separate previously recorded host inconsistency in windows-system-bootstrap-boundary.md; the current jobserver has no architecture guard or alternative implementation. No Windows execution, system-directory lookup, library load, allocator, or physical qualification occurred in this audit.


### Resolver Source review

Current fixed/system owner uses two private pointer/u32→u32 Kernel32 signatures, reads at most 256 UTF-16 code units into 257-unit storage, checks exact terminator and absence of embedded NUL, rejects truncation and invalid surrogate decoding, snapshots own primitive facts, and closes its library while preserving arbitrary body failure. No concrete buffer/cleanup blocker found. These signatures concern the intended x64 ABI, not Windows/ARM64 runtime acceptance.

Reported narrow hardening candidate: win32.normalize does not remove trailing-dot/space component aliases, so the pure facts projector currently admits those while describing local paths as canonical. Actual system API outputs are the production provenance; the concern is completeness of the pure canonical-path contract, not demonstrated system-directory spoofing. Program/environment integration was not part of this first owner read.


### Preparation receipt follow-up

Current receipt reader compares closed before/after facts semantically, independent of JSON key ordering. Generated PowerShell now uses actual single-backslash suffix/separator checks and exact 49-character volume GUID syntax with Guid.TryParseExact. The earlier empty separator/vacuous suffix and regex ambiguity were reported before this correction. Actual handle facts, no-delete sharing, root-to-receipt final-handle ancestry, exact volume UniqueId/NTFS/fixed classification, numeric Flush zero, postbarrier facts and handle closure remain required. The initial-host caller is fresh/nonparent/noncleanup only, carries actual issued preparation control and bounded owned output, and the publisher rechecks exact receipt/directory facts. No concrete new blocker found in inspected corrections. No Windows provider/ABI/Flush physical qualification is inferred.


### Confirmed Node authorizer boundary gap

The authorizer compiler emits target node (`tools/authorizers/script.ts`), and its runtime exports `withOriginalNxGraphNativeStorageV1`. The patched Nx entry/run-executor calls that wrapper. Its actual Source receives Original, enters repository graph storage, opens fixed platform images, and calls fixedPersistentState. Already-mounted Windows images still execute Windows observe → captureFixedReadV1 → fixedCommandSpec → nativeFixedWindowsSystemV1 → createRequire("bun:ffi"). Thus lazy loading avoids initial import failure but does not make this reachable Node reader path compatible: Node does not implement the Bun built-in. This is a concrete Source runtime-boundary gap, not a Windows execution result.

A correction requires actual trusted system-binding execution available to Node, or an authenticated dedicated Bun receiver carrying genuinely obtained OS directory facts under accepted process custody. A freely claimed environment directory is insufficient. The existing Bun jobserver does not prove all Core storage callers run in Bun. No fallback/loader weakening, native execution or Source changes were performed by this audit.


### Issued-capability Node correction and terminal caveat

Current routing getter reads only the actual control WeakMap invocation capabilities and enforces issuance/liveness. System resolution consumes closed capability facts, rechecks actual control, and refuses Node missing facts before bun:ffi. This addresses the reader runtime boundary once genuine prebirth OS-fact production is integrated; that producer remains a separate gate.

Concrete terminal interaction reported to Root/Lease: fixedCommandSpec now unconditionally resolves through options.control. Accepted terminal cleanup can remain funded after that body control expires or aborts, so the live getter can reject before cleanup command birth. Retain previously authenticated immutable OS facts privately in the actual receiver/preparation while live, then consume those facts under accepted cleanup enrollment plus physical identity checks. Do not revive the body control or accept environment paths. Pending correction; no physical behavior is claimed.


### Producer split and retained terminal correction

Fresh createNativeWindowsSystemGrantV1 obtains actual Bun OS facts and compares authored facts; non-Windows authored facts refuse. Build/Nx fresh authoring routes use that authenticator. The inherited neutral child projector expands only declarations from actual readOwnedBuildInvocation and preserves exact native grants, without reissuing a fresh OS grant. NxScript incoming path retains incoming capabilities and checks exact equality; only fresh authored input calls the authenticator. Pure Nx launch expansion supplies DATA only. No new blocker in inspected split. Standalone fresh Node factory without issued facts remains fail-closed, not zero-touch qualified.

Fixed terminal command specs now receive privately retained system facts captured on the receiver before its first lease await and copied into preparation. Inspected metadata, disposable observe/inactive/deactivate and preparation cleanup calls pass these facts under accepted terminal owner checks, rather than querying a dead body control. This resolves the reported terminal interaction in those paths. No physical Windows, loader or ARM64 qualification follows.
