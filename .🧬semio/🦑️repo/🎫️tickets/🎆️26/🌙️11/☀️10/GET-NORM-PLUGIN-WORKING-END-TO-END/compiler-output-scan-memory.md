# Compiler output scanning on the 16 GiB Mac

The fleet cache and build-script offloader inspect binaries for embedded machine paths before sharing outputs. Their common scanner read each complete compiler output into a single buffer. A large module therefore added an allocation proportional to its size while other outputs could still await garbage collection. This was a concrete source of avoidable memory demand. The incident diagnostics do not establish whether it was active at the instant macOS paused Codex.

The scanner now reuses a 64 KiB buffer plus enough bytes for the longest UTF-8 path across read boundaries. It skips dependency text files as before and returns an unshareable result when it cannot read an output. Three tests passed with 14 assertions covering root hints, spellings, a path split inside a multibyte character and an unreadable file. The old scanner accepted the missing file as neutral.

Against a real 224,950,465-byte Eurocode component, a full negative scan raised Bun process peak RSS from 54,688 KiB to 55,440 KiB. An independent Python byte search agreed on the result. The measurement covers this scan and runtime on macOS.
