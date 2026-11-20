# Native memory module restored (R222 item 3 never arrived)

Missing `host-capacity/🪟️native-memory/🟦️.ts` (importers: sampling, unmanaged, file-budgets, tests windows/footprint) was written from the importers' contract.

- `🧬️schema/🔣️.json`: Layout constants (MEMORYSTATUSEX 64, PROCESSENTRY32W 568, PROCESS_MEMORY_COUNTERS_EX 80 with offsets), Reason enum (`platform`, `host-call`), Texts (en, de), Unknown, Facts, Row, Table, Answer.
- `🟦️.ts`: `systemWindowsMemoryPortV1(platform?)`, `WINDOWS_MEMORY_LAYOUT_V1`, `windowsMemoryUnknownV1`, `readWindowsMemoryV1`, `readWindowsProcessesV1`, types `WindowsMemoryPortV1/FactsV1/UnknownV1`. Non-win32 returns the typed unknown (reason `platform`) and never loads `bun:ffi`; win32 loads kernel32 lazily (GlobalMemoryStatusEx, toolhelp snapshot, OpenProcess with PROCESS_QUERY_LIMITED_INFORMATION, K32GetProcessMemoryInfo, CloseHandle), counts unreadable processes, closes every handle.
- `🧪️tests/🟦️.ts`: non-Windows path, Ajv validation, layout constants, fake-kernel decode (4 tests).
- Verified on macOS only: native-memory 4, windows 6, footprint 11, sampling 11, unmanaged 4 pass. The real kernel32 path is untested on Windows (struct offsets from the headers, x64 only).
- If the original R222 module lands later, it supersedes this one; compare exports first.
