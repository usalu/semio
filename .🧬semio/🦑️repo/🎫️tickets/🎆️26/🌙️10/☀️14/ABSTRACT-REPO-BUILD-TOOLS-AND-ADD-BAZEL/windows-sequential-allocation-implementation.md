# Windows Sequential Allocation Implementation

The selected action is a single existing owned Windows PowerShell process. It owns one CreateNew FileStream from the first allocation effect through actual sequential zero writes, the first-party fixed VHD footer, Flush(true), handle observations, and confirmed disposal. This supersedes the earlier plan to keep unrelated Node and PowerShell handles. No Windows allocation, Native image, SDK or canonical store was executed for this slice.

## Boundaries

The private fixed preparation constructs the action under its actual body control and existing accepted cleanup/image writer. The existing command counter, output bounds, progress, cancellation, command drain, and cleanup reserve remain. There is no new timeout, quota, native grant, generic command-channel exception or fallback to New-VHD. Parent body Original0 remains valid; capture handles its existing nullable budget. The action uses bounded 1MiB writes and the already-held TypeScript VHD encoder's exact512 bytes. It does not SetLength, create a sparse placeholder, or depend on optional HyperV modules.

`fixed/🎛️kernel/🟦️.ts` supplies a closed Reflection.Emit API roster. AssemblyBuilderAccess.Run creates an executable in-memory assembly without saving it; the action uses the actual OS Environment.SystemDirectory kernel32 path. There is no Add-Type, compiler invocation, source file, temporary DLL, package or native FFI dependency in this allocator. The provider remains the existing OS-resolved WindowsPowerShell executable and its .NET Framework/FullLanguage environment. The resolver's separate Bun kernel-directory prerequisite is unchanged by this slice.

`fixed/📥️allocation/🟦️.ts` captures primitive data once, rejects accessors, validates bounds and local Windows path shape, embeds the real footer, and emits the fixed action. Before/after GetFileInformationByHandle, FileStandardInfo, FileIdInfo and FileAttributeTagInfo observations bind volume32/index64, full volume64/fileID128, EOF, allocation, nlink and attributes. Another read-only named FileStream is opened while the creator denies other writers/deletion. Its facts must match the creator. Unknown attributes and sparse, compressed, encrypted, reparse, directory and offline semantics refuse. EOF plus full allocation must fit the existing maximum.

The action emits one small closed receipt only after creator Dispose returns and the captured SafeFileHandle is closed. Body failure is retained if close also fails. A missing ABI/result, cancelled process, partial output or unknown close does not become a creator proof merely because the child exited.

The parent opens the named backing nofollow, binds native volume/index to actual runtime dev/ino, verifies link/type/size/allocation, reads the actual footer bytes, compares the encoded SHA, repeats descriptor/named stamps and confirms close, then repeats named identity after close. `creatorPending` is cleared only after these proofs; failed or uncertain preparation retains its writer and scope. Later phase checks retain the proven creator identity. Missing or different Windows runtime stat↔native identity/allocation correspondence refuses; this ABI remains physically unqualified on this Mac host.

## TDD Evidence

The genuine60s existing Source harness first invoked the actual fixedCommandSpec AST and failed because it emitted New-VHD: `G/windows-allocation-red.log`, terminal1, 0pass/1fail/1assertion. Two initial model runs also failed on a malformed fixture footer request and then on copied-control acceptance. They remain retained; the production entry now requires actual issued control identity.

The closed language-neutral corpus contains20 physical/identity/closed receipt rows and10 capture-owner fault rows. Ajv validates its closed shape; independent SQLite evaluates the receipt and fault outcomes. The actual capture-owner AST runs with explicit memory IO/process ports while the control is genuinely issued. It covers lost/partial receipt, short actual footer read, undefined/frozen primary, close faults, combined primary/close faults and inode replacement after close. It proves Source semantics, not Windows system calls or Native image ownership.

The permanent fixed storage test entry imports these laws; no new command or launch target was added. Exact final model/strict receipts and Source hashes are appended after final gates. Windows physical allocation/mounting/formatting and zero-touch provisioning remain separate acceptance work.

## Primary API References

- [GetFileInformationByHandleEx](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getfileinformationbyhandleex) provides handle-bound information classes; failure is refused.
- [FILE_STANDARD_INFO](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_standard_info) distinguishes allocated bytes, EOF, links, deletion and directory state.
- [BY_HANDLE_FILE_INFORMATION](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/ns-fileapi-by_handle_file_information) gives the volume and file index used by the parent ABI comparison. Its64-bit identifier has filesystem limitations; it is supplemented by [FILE_ID_INFO](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_id_info)'s128-bit identity while the handles are live.
- [DefinePInvokeMethod](https://learn.microsoft.com/en-us/dotnet/api/system.reflection.emit.typebuilder.definepinvokemethod?view=netframework-4.8.1) defines the selected signatures; PreserveSig retains the actual BOOL return. [AssemblyBuilderAccess.Run](https://learn.microsoft.com/en-us/dotnet/api/system.reflection.emit.assemblybuilderaccess) cannot save the assembly.
- [Environment.SystemDirectory](https://learn.microsoft.com/en-us/dotnet/api/system.environment.systemdirectory?view=netframework-4.8.1) supplies the OS directory rather than caller environment authority.
- [FileStream.Flush(Boolean)](https://learn.microsoft.com/en-us/dotnet/api/system.io.filestream.flush?view=netframework-4.8.1) is used before physical observations. File and directory publication durability beyond this allocation action belongs to the separate metadata owner.

## Final Held Gates

Final current Source model49694 terminal0:2pass/0fail/114assertions/1.94s, `G/windows-allocation-model-held-final.log`. Integrated strict63680 terminal0 with no diagnostics across four explicit roots: allocation owner/test and fixed owner/permanent entry. Kernel/VHD/shared dependencies are transitive. `G/windows-allocation-strict-held-final.log` retains its output. Earlier strict63790 and76037 also returned0 before the last closed request checks.

`G/windows-allocation-source-held.json` retains bounded nofollow FD Source hashes, bytes and before/after stable per-file stamps, plus absolute UTF16 path lengths. This is a point-in-time Source receipt; it is not a global future hold or installed SDK freshness assertion. Shared fixed/kernel bytes include the separately coordinated Root/Lease changes.

Permanent registration: Process TestScript `native-storage-platform` selects the existing fixed test entry; that entry imports the allocator laws. Existing launch configurations call `bun repo run @semio-tech/framework-process:test-native-storage-platform --exclude-dependencies --skip-cache`. This registration was inspected, not physically executed. An attempted process-local project.json lookup found no such file and was not used as registration evidence.

All allocator Source/test tool sessions are closed. No Windows, Native image, canonical store, SDK or physical allocation effects ran. The private creator action and actual parent Windows ABI still require the target-host execution acceptance described above.
