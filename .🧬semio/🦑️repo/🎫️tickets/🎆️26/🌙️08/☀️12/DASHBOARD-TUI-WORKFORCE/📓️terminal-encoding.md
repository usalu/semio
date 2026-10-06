# Native Windows Terminal Encoding

The current runtime capture shows UTF-8 box drawing decoded as OEM glyphs. The native backend enables VT modes but leaves the console input/output code pages inherited. This also weakens German labels and Unicode command search. The startup trace contains ANSI styling between `elapsed_us=` and its digits; that is a harness framing issue, separate from startup latency.

The Windows backend must acquire UTF-8 input/output code pages, preserve their previous values and restore them on setup failure, detach and retry. This uses the existing private first-party Kernel32 ABI. Microsoft documents these console-wide input/output transformations and nonzero success returns in [SetConsoleCP](https://learn.microsoft.com/en-us/windows/console/setconsolecp) and [SetConsoleOutputCP](https://learn.microsoft.com/en-us/windows/console/setconsoleoutputcp).

Verification must check actual PTY glyphs, Unicode input vectors against Node readline, and retryable ownership restoration. The runtime regression must project ANSI patches to a terminal screen instead of matching raw repaint bytes.
