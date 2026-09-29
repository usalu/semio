# Delete Committed Binaries Over 100MB

The pack held three Mach-O binaries larger than 100MB. None of them are in the current branch tip.

| Size | Blob | Path | Where it lived |
| --- | --- | --- | --- |
| 394.8MB | `a4150cef50592f6b983ce523d248e1c3ae419f31` | `END-TO-END-OS-HUB-COLLABORATION-MCP/wp-h8/semio_hub_lib_test` | Pre-amend commit `fe0033d12a0436f81f8bc62e5467fe82aa0d4e1b` (`🚩️649`), reflog only |
| 167.4MB | `196f150ca35600eb8ffd6a609ee8f9340e7f8b10` | no path left | Unreachable pack blob |
| 111.7MB | `7ab36ee27c20dea4485d6ae557d80d1fa205b844` | `semio-os-mcp` | Pre-amend commit `e5878235ee0e9c34983c76812b126523ee23f119` (`🚩️658`), reflog only |

Both commits had already been amended. The amended tips do not contain the binaries, and neither commit was an ancestor of `HEAD`. The only remaining pointers were reflog lines on `HEAD` and `🐙ueli/⛳wip`.

Those reflog lines were removed. `git gc --prune=now` then deleted the three blobs. `git cat-file` can no longer see them. `HEAD` has no file larger than 100MB.

Other pack blobs over 100MB are text (compiler logs, JSON, a grep dump), not binaries, so they were left in place.
