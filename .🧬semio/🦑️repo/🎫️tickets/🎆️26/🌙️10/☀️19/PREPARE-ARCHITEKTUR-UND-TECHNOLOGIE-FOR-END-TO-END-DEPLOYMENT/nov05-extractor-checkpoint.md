# Native Extractor Review Checkpoint

This is a narrow read-only review of the in-progress production Go ZIP and MSI-path helpers, not a completed provider review or readiness decision.

The Windows input helper opens a genuine regular file with read sharing only, compares the pre-open and held-file identities, and hashes and reads ZIP data through that same held file. ZIP extraction bounds archive size, per-member size, total size, file count, path depth and UTF-16 output length. It rejects traversal, reserved names, alternate separators, links and case-folded duplicate output paths; files use exclusive creation, with identity-aware cleanup of outputs created by this operation.

Two boundary cases were sent to the extraction owner for explicit tests and resolution. First, cancellation delivered by the final progress callback currently bypasses the next-iteration cancellation check and can return success; a final context check plus a one-member cancellation case should cover it. Second, the MSI destination helper can finish at an arbitrary empty parent without reaching the canonical `TARGETDIR` row; the intended disconnected-root policy needs a neutral case.

The owner reproduced both findings with failing tests before changing production code. The final context check now observes cancellation after the last progress callback and triggers owned-output cleanup; MSI paths must reach explicit canonical `TARGETDIR`/`SourceDir`. The focused native Go suite subsequently passed with normal `go.work` enabled. This closes those two observed findings; it does not establish the unfinished MSI/CAB, publication or canonical-execution paths.

MSI/CAB production extraction, CLI/source registration, fixed pins, managed-store publication, held-custody canonical execution and independent full review remain pending. No arbitrary output path or metadata inspector gains readiness authority from these checks.

The subsequent MSI reader uses bare-name `syscall.NewLazyDLL("msi.dll")`. The owner was asked to bind MSI and SetupAPI loading to the trusted system directory or the existing repository system-library interface. Go's [Windows DLL implementation](https://go.dev/src/syscall/dll_windows.go) distinguishes its known-system-DLL path from general name-based loading; the latter searches multiple locations. This review does not establish an exploit or a substituted DLL on this host. A focused system-loader check and resolution remain pending.

The reserved-name regexp also omitted the six COM/LPT names using superscript digits ¹, ² and ³, including names with extensions. Microsoft's [file naming contract](https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file#naming-conventions) explicitly reserves them. The owner was asked for neutral rejection cases before extraction writes. No such member was observed in the selected publisher archives.

Both subsequent findings are resolved in the focused implementation. The system-library port obtains the actual system directory through Go's trusted kernel32 path, allows only MSI and SetupAPI, and loads their absolute system32 paths; its native loader check passed. Superscript device-name cases reproduced red before the regexp was corrected, and the full current focused Go suite passed in session 5422. Full CAB/CLI/publication and canonical runtime verification remain pending.

A metadata-only check of the 4,947-file prototype manifest measured its compact provider-relative maximum as 107 UTF-16 units; the raw MSI plan's earlier 119-unit maximum describes a different pre-compaction layout. The ticket root is 115 units. This check does not substitute for the production extractor's per-file physical-path guard or establish final managed-store paths.
