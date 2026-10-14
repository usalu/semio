# Current Inputs and Codec Audit

Read-only audit of current Source, prerequisite, Next projection and codec owners. Read all four requested authority reports. No build, setup, gate promotion or ticket lifecycle operation was performed; findings below are static code evidence, not runtime qualification.

## Actionable Findings

### P1: Non-JavaScript Next route assets are absent from the projected hash

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/📥️inputs/🟦️.ts:10` enumerates directories but admits only `[cm]?[jt]sx?` files as entries. Next file convention metadata routes such as `app/icon.png`, `app/favicon.ico`, `app/opengraph-image.png`, and `app/robots.txt` can affect produced artifacts without a literal JavaScript import. Existing asset content changes do not change the directory membership observation; no configured file capture or Nx Source input covers those files. The Nx projector at `.../📥️inputs/🔌️nx/🟦️.ts:19` replaces target inputs with app files plus command/config/manifest/lock Sources, so there is no broad fallback input. Adding a metadata asset triggers a different directory observation during the callback, but its path is never added to the projected Nx inputs or dependency identity; a fresh invocation can therefore still reuse the prior hash.

Before enabling Next admission, admit all supported physical Next route conventions and their contents, and hash membership for added/removed relevant route assets. Add an ordinary changed-existing-metadata-asset and added-metadata-route cache invalidation oracle. Production admission currently remains false, so this is a qualification blocker rather than a claim of an enabled production cache defect.

### P2: Configured Source budgets do not bound compiler configuration discovery

`.../🕸️dependencies/🟦️typescript/⚙️configured/🟦️.ts:14,16` hands an unbounded observed TypeScript host to `parseJsonConfigFileContent`. The host delegates configuration reads to `compiler.sys` before the asynchronous `add` admission check; inherited JSON Sources are only charged afterwards. TypeScript configuration include scanning also uses spread `sys.readDirectory`, which is neither bounded nor checkpointed. A large inherited configuration or expansive include tree can therefore allocate/read beyond maximumBytes/maximumFiles and occupy a synchronous phase before rejection/cancellation checks.

Bound configuration read sizes and discovery work before compiler consumption, and capture include-directory discovery through owned observed host operations. Add a neutral oversized inherited configuration and excessive include-discovery refusal law. The current 32MiB/4096 schema ceiling is a final admitted-file ceiling, not a bound on every operation performed to discover that set.

### P2: Qualified Source resolution and actual runtime package export resolution need a dedicated parity case

`.../⚙️configured/🟦️.ts:17` accepts a nonexternal result of `typescript.resolveModuleName` as the first-party executable Source. TypeScript can select a declaration/type conditional export while the bundler selects a runtime/import conditional export. `.../🕸️dependencies/🟦️typescript/🟨️.mjs:72` intentionally stops at declarations. A first-party package exposing separate types and import entrypoints can therefore terminate admission at its declaration instead of traversing the executable implementation.

The current application request uses `packages:{}` (`.../📥️inputs/🟦️.ts:10`), so the explicit package override cannot repair that case. Add independent esbuild parity with a workspace package using separate `types` and `import` exports, then use the observed runtime export authority or refuse unsupported first-party conditions. This is a supported-resolution gap; no current repository package trigger was established during this static audit.

## Checked Without a New Finding

The shared prerequisite owner walks configuration-effective dependsOn, caret dependency targets, named projects, wildcard targets, project/tag selectors, and targetless dependency bridges. Its queue/bridge limits and cancellation checkpoints are present (`.../🏗️build/🧭️selection/📥️closure/🟦️.ts:14–31`). Exact missing requested tasks refuse. I did not independently run the Nx scheduler oracle, and do not claim parity beyond the cited code and existing report.

Configured Source captures SHA256 contents, rejects symlink ancestry and hardlinks, checks before/after file identity, and rechecks observed resolution probes. The Next callback has a live guard and retains dependencies storage custody through its body (`.../📥️inputs/🟦️.ts:10`). Its outer Nx wrapper rechecks before and after consumer execution (`.../📥️inputs/🔌️nx/🟦️.ts:19`).

The codec production manifest depends only on first-party Artifact Reference and IO Schema (`🧰️framework/🔨️modules/🚪️io/📦️codec/📦️packages/🦀️rust/Cargo.toml:25–27`). Current whole-framework symbol census found the concrete SourceSpan, CodecBudget, DecodeContext and ResourceResolver declarations only in the new codec owner (lines 9,268,356,519). Confidence is imported from IO Schema at line 3. No Store/Kernel production dependency or duplicate moved definition was found. Native compilation remains Root-owned and was not run here.
