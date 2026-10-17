# Windows Directory-Relative Capability — Source Checkpoint

The existing first-party filesystem descriptor module now owns the Windows branch of the neutral directory-relative API. The implementation and all tests are Source-qualified only. No agent performed Native image, protected-cache, host Windows DLL, or NTFS operations. This is not Windows runtime or durability acceptance.

## Implementation

The private Windows x64/Bun branch opens a drive root and each ordinary root component without following reparse points, checks NTFS and the actual volume, and anchors every subsequent child operation to `NtCreateFile.RootDirectory` plus one validated UTF-16 basename. It requests synchronous access with `SYNCHRONIZE`, keeps read-only sharing without delete sharing, and serializes enumeration on the original writable directory handle. No numeric HANDLE, pointer, writable Native image, reservation, store, or administrative authority escapes.

Immutable entry proofs are privately bound to their exact parent frame, name, seven public facts and current Windows 128-bit file ID, volume identity, security bytes digest, unnamed-stream digest and link target. Copies, escaped receivers, wrong parents/names and drift refuse before selection effects. Registry capacity is reserved before every library or handle acquisition; accepted descendants drain before release. Arbitrary `undefined` and frozen primary failures remain first, with distinct closure failures retained. Unknown acquisitions, pending I/O and ambiguous close retain custody and prevent unload or further destructive work.

Removal takes a fresh actual delete handle, compares the full private proof, applies fixed handle-bound disposition, then checks a separate held-handle pending transition before actual close. That transition requires DeletePending, the same private identity/security/streams/type/size/write time, nondecreasing change time, and a link count of n or n−1. This preclose allowance does not prove a final decrement or zero charge. Actual close, selected-name absence and a permitted parent transition are still required. One selected hard link can be removed while another remains.

Directory synchronization opens with actual write access and calls `FlushFileBuffers`; false results or metadata drift refuse. It never substitutes read-only flushing or a no-op. Microsoft documents POSIX deletion visibility at delete-handle close in [FILE_DISPOSITION_INFORMATION_EX](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_file_disposition_information_ex), and the pending/link fields in [FILE_STANDARD_INFORMATION](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/ns-wdm-_file_standard_information). Directory-flush behavior and these constrained pending transitions still need actual NTFS qualification; a future authenticated Native mapped-volume anchor is separately required for reparse mount roots.

## TDD Evidence

Schema-owned `RelativeWindowsProtocol`, `RelativeWindowsCase`, `RelativeWindowsCases` and `RelativeWindowsFacts` precede their implementation/checks. The final corpus has 60 cases, including independent SQLite seven-field derivation, basename/ABI packets, exact receiver/proof failures, ID/security/stream drift, real issued-control cancellation and expiry, post-acquire cancellation, registry exhaustion, accepted unawaited children, arbitrary primary plus distinct close, hard links, pending deletion transition faults, malformed NT records, flush failure/drift, and three-page 127/127/46 enumeration progress. UTF-16 BOM target preservation was already implemented and its added positive case passed; the log named `windows-relative-bom-red.log` is GREEN and is not RED evidence.

The exact authored Windows functions are selected by the TypeScript AST and executed with an explicit NT syscall Source port. This port supplies native buffers/handles and observes access masks, sharing, disposition, closure and unload ordering. It does not replace the owner logic. Ajv validates the portable cases; SQLite independently derives the seven facts and compares accepted/effect/closure results. Node UTF-16/crypto and separately recorded Python ctypes validate byte encoding, digest and x64 layout. The ctypes oracle covers three structures and 11 field offsets. None of these model ports proves the real Windows ABI, filesystem or durability.

Semantic RED evidence retained under Generated:

- `windows-relative-owner-model-red-details.log`: the authored wire contract required an explicit `SYNCHRONIZE` bit, expected1048576/received0. This is a Source mask-policy RED, not proof that Windows rejects the earlier generic-read access: Microsoft documents that [GENERIC_READ already maps to SYNCHRONIZE](https://learn.microsoft.com/en-us/windows/win32/fileio/file-security-and-access-rights). The added explicit bit is redundant and harmless. No real Windows failure is attributed to its prior omission.
- `windows-relative-sharing-red.log`: the old duplicate writable enumeration handle violates mutual read-only sharing.
- `windows-relative-acquire-cancel-red.log`: cancellation after actual Source child acquisition happened before wrapper registration, losing the exact primary and retaining the orphan.
- `windows-relative-enoent-once-red.log`: an arbitrary single ENOENT checkpoint was swallowed; expected refusal/received success. Only an exact private sentinel issued by the native probe-open not-found status now becomes absence.
- `windows-relative-yield-semantic-red.log`: 300 names in 127/127/46 pages yielded0 rather than at least2. The fixed threshold counts all processed records, including across page boundaries.
- `windows-relative-post-disposition-semantic-red.log`: altered private ID after disposition previously returned success; now it refuses after the actual partial selected-name effect.

Earlier six-up source path/initial harness errors, wrong golden payload size26 (actual27), a close-primary assertion that incorrectly disallowed a required AggregateError, and the first hard-link model not clearing pending on the remaining link were harness failures, not production RED evidence. `windows-relative-enoent-red.log`, `windows-relative-cadence-red.log` and `windows-relative-post-disposition-red.log` retain those exact failures. One malformed shell invocation did not execute a gate.

## Final Executed Gate

`SEMIO_WORKSPACE_ROOT="$PWD" SEMIO_TEST_ARTIFACT_DIR="$PWD/<ticket>/🗑️generated" bun '<ticket>/stdio-phase/📜️script.ts' --relative-strict`

Final session4344, log `🗑️generated/windows-relative-sixty-final.log`: strict0, Root POSIX1test222assertions0fail, separate Windows3tests2961assertions0fail, terminal0. Both subprocesses receive their own genuine carried Original within the unchanged60s helper. The Windows body tests use explicit Source syscall buffers only. The Root POSIX test includes actual Generated filesystem operations and its existing native Darwin SDK layout check; Linux and Windows runtime acceptance remain absent.

The root ancestry currently retains every opened ancestor until callback completion and therefore honestly refuses if the shared32 capacity is exhausted. Iterative bounded path descent for depth64 is the next distinct feature; no capacity is raised here. Existing Native mapped mount roots are still rejected by this ordinary-root constructor.

## Point-in-Time Manifest

Shared files include Root-owned POSIX/schema/test prefixes; the Interfaces edits are the appended Windows region, Windows definitions and Windows tests only. The unchanged helper provides separate Root/Windows Bun processes and existing launch coverage.

| File | Bytes | SHA256 |
| --- | ---: | --- |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts` | 51886 | `6810d9c16cbda33432d9cac0086ae8d004670e7aee406c34381b7b8fc74d5abc` |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json` | 19759 | `e4b3a140ec1400a12503b4b20af7b534b31bfe8687c4efc032b68e632d4a86da` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts` | 90172 | `7f779f8a02a22f10efe5930edd7fa52e69d57ea50cb6273f3ac5b8be21e3d6b0` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/stdio-phase/📜️script.ts` | 8439 | `e85b3d5d00739b85f2ad22d8e1c0ad3409a3cd986ea27f17366d9a1fb16ac829` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/windows-relative/🔣️.json` | 620 | `fa0e9fe02fc0488a236a59f3ab7aa255d78d7ce67b559cbe0e59a89da6da52d1` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/windows-relative/🧬️schema.json` | 1043 | `369d62ad95643b75029442ef7e81fb5657b4c1211e55af32ef01ac9c96a19fc9` |
