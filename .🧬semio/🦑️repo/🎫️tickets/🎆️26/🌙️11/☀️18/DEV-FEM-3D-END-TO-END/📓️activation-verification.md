# FEM 3D activation verification

## Baseline on November 20

At 02:30 local time the FEM core was still the 01:53:50 publication. The server listened on port 6087. In a new tab the shell selected Demo, stalled on the initial setActiveExample, reported a dispatch refusal for input 1, and showed original-turn.source-not-ready.

The generated shared worker predated the source changes. Its timestamp was November 18 at 13:04:45, and its catch block returned String(payload) when JSON.stringify failed. Rebuilding the framework-plugin-web support target published a 43,087-byte worker at 02:38:09 on November 20. A new tab against the same FEM core then showed a structured failure with tag semantic, 14 copied items, 4,565 copied bytes, 24 work units and zero retained or released bytes. The other three requested console filters had no entries on this baseline.

The ticket fallback planner now resolves an authored target with an explicit empty dependency list when the selected graph omits it. The support target failed its lookup before this change and emitted its correct command afterward. The chain accepts an optional last step, so `bash native-chain.sh 04 04` rebuilds browser support in isolation. A failed plan now stops the chain. Git Bash syntax validation passed.

## Attempt 3

The 177,314,662-byte FEM core published at 02:56:38. The first preparation stopped while importing the Windows compiler code because the shared tree lacked its new Go tooling module. The setup module also referenced absent Linux libc and Rustup host-triple modules. Preparation and activation completed at 03:00 after resolving the imports. The Rustup host-triple test passed its neutral corpus and independent SQLite oracle. The complete Rustup suite also recorded a Windows lease-shard expectation mismatch and a private test workspace cleanup failure.

In a fresh tab on port 6087 the initial Demo dispatch refused with `original-turn.native-return-refused: semantic`. Explicit switches to Concrete Forest and House produced the same refusal as inputs 2 and 3. The selector changed its label after each rejected switch. The three narrower console filters were empty. Model and Results did not render.

The next build stopped during hub-wide Cargo.lock resolution. Two PDF package manifests were missing in a concurrently edited editor. An isolated native FEM build references the same component entry point and production crates directly and omits the unrelated hub workspace members.

At 03:18 the shell included the picker change. With the same 02:56 guest, both the boot dispatch and a manual Concrete Forest switch were refused. The combobox continued to show its placeholder after both refusals. The published guest exposes only the semantic tag through the WIT poll result. The later guest source includes a temporary diagnostic at that boundary.

The shared PDF package directories arrived with real implementations, and the full hub Cargo metadata resolved offline with its lock. The normal FEM build resumed alongside the isolated build.
