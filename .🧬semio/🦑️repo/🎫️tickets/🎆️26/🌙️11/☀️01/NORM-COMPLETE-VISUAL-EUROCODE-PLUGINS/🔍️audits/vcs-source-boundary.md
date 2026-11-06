# VCS source boundary

The semio-os-vcs manifest resolves its lib path to vcs/🦀️.rs. The existing readRustSourceClosureV1 collector returns Linux production 31 files, 264460 bytes, 3271 active lines; Linux, Windows and macOS test cfg each return 31 files, 264460 bytes, 3310 active lines. Against 32 files, 262144 bytes and 4000 active lines this is 2316 bytes over, with one file and 690 test lines remaining. Collector census is not a compiler gate.

Production compaction candidates preserve concise meaningful public docstrings: vcs-errors/🦀️.rs has 5779 bytes of doc lines including historical migration and pass narratives; checkpoint-records/🦀️.rs has 1553 doc bytes including obsolete report narrative; artifact-vcs/🦀️.rs has 697 doc bytes including historical serializer discussion. These are candidates, not measured proposed savings. No files are excluded or tests moved.

## Current census after compaction

Linux production: 31 files, 261329 bytes, 3241 active lines; remaining headroom 1 file, 815 bytes, 759 lines. Linux, Windows and macOS test cfg: 31 files, 261329 bytes, 3280 active lines; headroom 1 file, 815 bytes, 720 lines. Net change since prior census is minus 3131 bytes, reflecting current shared source. This is collector evidence, not a build.
