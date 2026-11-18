# Configuration publication under compiler CPU custody

## Symptom
After 148 min of unit compilation the driver died at 11:50:03 with `Refusing to wait: cargo:configuration-publication (exclusive) ... while this process holds semio.compiler.cpu/v1/0` (log `🗑️generated/native/site-build-115030.log`, line 1261 ff.). The run also warned `cargo:configuration-view:N (rank 1) requested while cargo:registry (rank 2) held`.

## Evidence
- `.cache/b/r/m/configuration/*/view.json`: the 13 wasip2 pass views of this run were all (re)published between 11:49:48 and 11:50:02, at the end of the compile phase, not at start. Earlier runs (11-17 19:52) left views for the same manifests with different digests.
- Diff of old vs new view for one manifest: only `compilerFlags` and `configuration.*.rustflags` (`-Z threads=0`, `-C link-arg=...`), i.e. `.cargo/config.toml` of the shared live worktree changed during the 2.5 h run.
- `nativeBuildPassInputRequestsV1` + `withRepositoryCargoPassInputsV1` DO hold a view per declared pass up front, but the per-pass port (`withCargoInputs` -> `repositoryCargoCompilerInputsV1` -> `withRepositoryCargoInputsV1`) ignored them and re-captured `cargo config get` / rustc probes from the live tree for every pass/unit-graph request inside the compiler writer and CPU token. Any drift gives a new digest, so the pass publishes lazily (rank 2 under rank 3/7); with many passes contending the 15 s refusal fires. The same re-entry requested the rank-1 view lease under the rank-2 registry reader (the warning).

## Change
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts`
- `withRepositoryCargoPassInputsV1` registers every declared selection (keyed by exact manifest, cwd, port-rendered args, consumption and environment, per storage reservation) for the lifetime of its body.
- `withRepositoryCargoInputsV1` adopts the held view for an identical request: no capture, no publication lease, no rank-1 lease; drift is caught by the view `recheck` (loud error) instead of republishing under custody. Undeclared requests (unit graph, jobs) keep the previous path.
- `adoptedCargoSelectionKeyV1` exported for the test.

## Tests
- New: `.../📥️dependencies/🧪️tests/🟦️.ts` "an adopted declared input is named by its exact request" (node:util isDeepStrictEqual oracle) passes.
- Passing: `⚙️configuration/🧪️tests/📮️order.ts`. Pre-existing unrelated failure in the same file: scheduler cold bootstrap (`shareSchedulerBuildV1` undefined).
- Not runnable directly (need script envelope): `🧩️component-pass/🧪️tests/🟦️.ts` (19 fail with "parent Script process invocation required") and the adoption path end-to-end; verify by the next play build.

## Open
Unit-graph and cargo-job requests are still not declared up front (different args / runtime environment); if drift hits them the lazy publication remains possible. Not verified by a real build.
