# Energy w2q3K5F optimization

At inspection the current run's pinned Binaryen 130 `wasm-opt -Oz` process was active on the transpiled core, elapsed 26 minutes 37 seconds and CPU time 38 minutes 5 seconds. The linked and transpiled outputs appeared at 10:17–10:18; the run has not yet issued its final stage report, so no completed per-pass timing or final cache verdict is claimed.

The exact input core is 385,335,638 bytes, SHA-256 `40e19a7d80dd02bd1b3f05f837bf3c5c172e320d8865ba22e4b5cfe8ed15893c`. Its code section is 365,474,094 bytes across 753,610 functions; its entire data section is 17,787,913 bytes. Raw embedded data cannot explain the code volume, and no particular asset was proved accidentally retained. All 11 currently retained optimize entries were checked: none has this core digest. They are not eligible exact-input hits for this run. This does not prove an entry was never present or that a different stage was reusable.

The previous native-throughput audit already evaluated optimization scheduling and source/sidecar cache custody. These facts do not establish a safe optimization bypass, a wrong cache key, or a permissible build-profile change. Preserve the active canonical stage and its output. No process or configuration was changed.
