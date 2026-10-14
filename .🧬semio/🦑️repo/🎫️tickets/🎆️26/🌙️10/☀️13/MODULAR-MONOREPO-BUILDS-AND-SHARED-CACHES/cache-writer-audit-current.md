# Current Cache Writer Audit

Read-only source audit on 2026-10-14. No builds, tests, shared generated output changes, or runtime claims. Source references below are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## Highest Priorities

1. **P1: Dependency installation bypasses the dependencies partition.** `⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts` `prepareJavascriptDependencies` launches `bun install --frozen-lockfile` directly in each authored workspace with inherited `process.env`; no `withRepositoryCacheStoreV1(...,"dependencies",...)`, finite install destination, or bounded Bun cache environment surrounds it. `🏗️build/🚀️bootstrap/📜️script.ts:24-36` explicitly validates workspace `node_modules` and writes `.🧬semio/🦑️repo/⚡️cache/build/bootstrap/dependencies.json` outside any store. Provide dependency custody and actual finite install/cache roots before invoking Bun, and move the bootstrap receipt under that custody. Preserve authored lockfile behavior separately.

2. **P1: Global lease files bypass the leases partition.** Policy declares `leases` backing `.cache/b/l` with 64 MiB capacity, but `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts:18`, `📥️opening/🏪️store/🟦️.ts:19`, `🕸️graph/🏁️startup/🟦️.ts:14`, `📦️bundle/🟦️.ts:35,92`, `🚀️bootstrap/🛠️tools/📥️use/🟦️.ts:20`, and graph journal worker `📜️script.ts:50` still write/acquire under workspace `.cache/framework/leases`. Decide a finite bootstrap lock/control surface for opening the leases image itself, then place ordinary resource leases inside the enforcing lease store; simple path replacement would create a provisioning cycle.

3. **P1: Public graph bypass and inference fallback remain.** `🏗️build/🕸️graph/🟦️.ts:182-187` publicly exports `pinBuildGraphV1`, writes `.🧬semio/🦑️repo/⚡️cache/build/graphs/<digest>.json`, and requires no invocation/store custody, retention budget, source verification, or consumer lifetime. Remove it and use `withRepositoryBuildGraphArtifactV1`. `🏗️build/🕸️graph/📥️inference/🟨️.mjs:1634-1640,1815` accepts arbitrary `SEMIO_BUILD_INFERENCE_DIRECTORY` and otherwise writes old `cache/build/inference` (`np`, `mr`, inference receipts). Require original graph-store authority and finite paths rather than trusting an environment string or fallback.

## Other Confirmed Writers

- `⚡️caching/🚀️bootstrap/📜️script.ts:65-83,210-219` admission snapshots and ignore candidates live in `.🧬semio/🦑️repo/⚡️cache/nx-admission`, outside graph custody. Move them into the graph store with finite retention and recover abandoned candidates.
- `🏗️build/📜️script.ts:145,169` accepts arbitrary output paths and writes graph/plan JSON directly. If these are generated diagnostic reports covered by the budget, publish through diagnostics custody and hand out validated destinations.
- `⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts` contract-check defaults to old `cache/tests/bootstrap-dependencies`; test diagnostics remain outside diagnostics backing.
- `⚡️caching/📦️bundle/🟦️.ts:28` copies compiled artifacts to caller-selected `options.output`. The compilation is distinct from final output publication; callers must prove the output belongs to products custody. This audit does not establish every caller does so.

## Already Improved

`🏗️build/🕸️graph/🟦️.ts:62,227-241` maps mutable graph and authority receipts into graph partition and encloses producer attempts in `withRepositoryCacheStoreV1`. `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts:35-43` retains immutable snapshots inside the graph store, validates bounded single-owner original files, prunes only unheld content, and keeps consumer custody alive. Their resource leases still share the global lease gap above.

Tool/Bazel physical partition work and Nx execution/unit pipeline are owned by concurrent execution agents. This audit intentionally leaves those implementations untouched.
