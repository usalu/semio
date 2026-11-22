# F1: tooling seal refused (module set exceeds 2000 files)

## Cause
- Refusal text: `seal/🟦️.ts:33` (`captureV1`); limit is already schema-declared: `unit-cache/🧬️schema/🔣️.json:96` `seal.maximumFiles: 2000`.
- The wrapper closure (`📜️script.ts`/`🏁️entry.ts`) was 2212 files, accidental: `offload/🟦️.ts:24` re-exported `detectWorkerCommandsV1` from `worker-tools`, whose lazy `import("…/🛠️tools/🕸️wasm/🟦️.ts")` (wasm-opt finder; the sealer follows quoted file literals) -> `budget/opening` -> `headroom` -> lazy `🏗️build/📥️inputs/💾️storage/🟨️.mjs` -> `cache-gc/launch` -> `🏗️build/📜️script.ts` = the whole repo CLI (691 build + 496 caching + 112 native-build storage files ...).
- Consumers of `detectWorkerCommandsV1` (sidecar agent, remote-execution command) import worker-tools directly; nobody used the barrel export.

## Fix (shrink, not raise the limit)
- Dropped the barrel re-export. Closure 2212 -> 340 (script) / 341 (entry); edge-cut simulation showed this single edge was responsible (headroom edge alone: 2212).
- Law test in seal tests: real wrapper seal succeeds, files <= maximumFiles/2 (1000), sealed tree has no `📚️library/🏗️build/📜️script.ts`.
- Suite `cargo-unit-cache-processes` declares tools [python]: the seal test file (routed there by tree) failed at import without the pinned binding.

## Live wrapper sealing
`.cache/b/r/m/u/sealed/`: earlier snapshots 331-335 files; the snapshot sealed 22:00:10 (after the edit) has 341 files, contains no worker-tools, sealed fine. Limit headroom is now ~5.9x. Background collector (`startUnitCacheCollectorV1`) test passes.

## Tests (repo test-file)
- seal 10/0 (incl. new), gc 14/0 (the background-collector test now passes), unit-cache top 26/2 (the 2 failures: args.metadata key component and sccache comparison, owned by the unit-key-metadata lane, unrelated).

## Open
- Lazy-literal edges are followed by design; a new lazy import into the build CLI from the wrapper closure would be caught by the new law test.
