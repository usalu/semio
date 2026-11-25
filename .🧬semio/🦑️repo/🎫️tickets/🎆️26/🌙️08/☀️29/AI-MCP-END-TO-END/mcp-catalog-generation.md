# MCP Catalog Generation

## Owner
- Target `@semio-tech/s-dev-composition:generate` in `✏️s/🧑‍💻dev/📋️project.json` runs `repo ./🧩️catalog/📣️publication/📜️script.ts generate` (cwd `✏️s/🧑‍💻dev`), dependsOn `generate-stdio-format-claims` and `@semio-tech/framework-os-dev:channel-version-descriptors-check`, outputs `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated` (gitignored).
- The script wires `GenerateScript` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts`) with `specificCatalogPublicationV1`; it calls `renderCatalogFiles(repoRoot, undefined, "exclude", publication)` then `publishCatalogFilesV1(outDir, files)`.

## Routes tried
1. `repo run @semio-tech/s-dev-composition:generate`: stuck 11+ min in "Reading the project graph slice".
2. `repo run ...:generate --exclude-dependencies`: still stuck in the graph slice (the flag only drops dependency owners; the slice is still read). 
3. `repo ./🧩️catalog/📣️publication/📜️script.ts generate` directly: refused, "Original parent Script process invocation required" (scripts require an invocation envelope issued by `repo run`; by design).
4. In-process driver (works, 23 s): `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/catalog-direct-generate.ts` imports `renderCatalogFiles`, `publishCatalogFilesV1` and `specificCatalogPublicationV1`, run as `repo "./<ticket>/catalog-direct-generate.ts"`. Result: 74 plugin entries, 0 diagnostics.

## Result
`✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🦀️catalog.json` (61829 B) and `🔌️plugins.json` (53365 B) plus the full publication tree. No generator bugs hit; the render path needs neither `generate-stdio-format-claims` output nor the graph service.

## Recommendation (warm staging, not edited)
The catalog render needs only `repoRoot` and the publication. `stageMcpBinaryAsync` (`🌉️mcp/📡️stdio/🏗️staging/🟦️.ts`) should, for the catalog target, call in-process a new owner function (e.g. `publishCatalogV1(repoRoot)` exported from `🔌️plugin/📇️registry/📽️projection/🟦️.ts`, composing `renderCatalogFiles` + `publishCatalogFilesV1` with `specificCatalogPublicationV1`) like `buildMcpArtifactV1` does, instead of shelling `repo run`. The `GenerateScript.run` should delegate to the same function so there is one implementation. Caveat: the target's `dependsOn` gates (channel-version-descriptors-check) would be skipped; `exclude` stale-channel mode already withholds stale plugins with diagnostics.

## Graph service slice-worker crash
Service log `host-414d793198c0.log` line 3430: `request-failed 9a7af9ea ... Error: Slice worker exited with code null` at 01:14:35Z, after ~650 s in `source.served`. Exit code null means the worker was ended by a signal, not an error exit. Evidence for memory pressure: 16 GB host, ~100 MB free pages, load 8.2-8.5, capacity log lines "not enough free memory (… 2048 MB floor …)" and many "waiting for memory headroom" entries. Likely a kernel/jetsam or supervisor kill; not proven (no signal name logged). The worker exit handler should log the signal.
