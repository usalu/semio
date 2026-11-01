STATUS: PARTIAL. generator-inputs fix GREEN by probe (rc 0, 132205 inputs, `🗑️generated/1101/geninputs-probe.txt`). Native opening-binding fix edited, funding test GREEN, the selected-state-recovery test UNVERIFIED (needs a script-process invocation). Manifest regeneration NOT run: the owning target is cargo-backed and the framework gate is closed.

## 1. generator-inputs: "Registry implementation import is not schema-owned"
Root cause: the registry-catalog gate bound only whole-package specifiers (`manifest.exports["."]`), so subpath specifiers could not be declared. Fix is schema-first (taxonomy authority + gate), the validator is untouched.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` `generatorContracts.plugin-registry.inputDiscovery.workspaceImports`: added `@semio-tech/framework-pack-json-ts/members` (manifest `🔤️json/📦️packages/🟦️typescript/package.json`, entry `.../🧩️members/🟦️.ts`) and `@semio-tech/pack-json-decode-ts` (`🔤️json/📥️decode/...`), byte-ordered.
- `🔍️discovery/🧲️registry-catalog-projection/🟦️.ts`: binding check now splits the specifier into package name + optional subpath and compares `manifest.exports["."]` or `exports["./<subpath>"]` to the entry.
- `🔍️discovery/⚗️audit-generator-contracts/🟦️.ts`: the "exact package name" regex now admits subpaths (`@scope/name(/segment)*`).
- Test `🧪️tests/🔬️workspace-contract/🎨️artifact-path-projection/🔸️authored-draw-residue-recovery/🟦️.ts`: package-name check uses the package part of the specifier.
- Proof: `1101-tools/tsdemo/probe.ts` -> `132205 inputs`, rc 0 (`geninputs-probe.txt/.rc`). `bun test` registry-import-language + residue-recovery: 16 pass, 0 fail (`geninputs-tests.txt`). The full Bazel `generator-inputs:prototype` task was not run (probe is the same `registryCatalogInputPaths`).

## 2. Nx fallback "Exact canonical Native image opening binding required"
Root cause: `observeExistingNativeStorageStateV1` passed the image's MEASURED `filesystemBytes` as the "declared" value of the opening binding. On a mounted macOS image the measured value (`.cache/b/n/m`: statfs 392950 x 4096 = 1609523200) differs from the declared one (1609564160, from `repositoryCacheBudgetV1().partitions.nx.filesystemBytes`), so the binding always refused for the real nx partition (temp-image tests never differ).
- `🏗️native-build/💾️storage/🌐️platform/🟦️.ts` `requireNativePlatformImageBindingV1`: `expected.filesystemBytes` is optional; compared only when stated.
- `.../💾️storage/👁️existing/🟦️.ts`: input gains optional `filesystemBytes` (declared); binds it only when given instead of echoing the measured value.
- `...⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts` `nativeFundingSnapshotV1`: passes `filesystemBytes: policy.maximumWorkspaceBytes` (= declared nx `filesystemBytes` by construction, same file).
- Test: `...📦️budget/🕸️graph/📏️headroom/🧪️tests/🔀️selected-state-recovery/🟦️.ts` line 29 asserts the unbound form is accepted and a wrong declared `filesystemBytes` still throws. UNVERIFIED: this test requires a Script process invocation ("Original parent Script process invocation required" when run via bare bun, same pre-existing fault, `geninputs-t2.txt`). Nx funding test: 3 pass (`geninputs-fund.txt`, `geninputs-t2.txt`). The nx fallback itself (`repo --tool=nx run ...:test`) was not re-run.

## 3. Stale generated framework manifest (34 TS2322 in `🪧️brand.ts`)
Owner: `generatorContracts.framework-manifest`, target `@semio-tech/framework-rs:generate`, output `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` (gitignored, dated Oct 28). The target runs `repo ./📜️script.ts generate` through the admitted cargo command in `🧰️framework/📦️packages/🦀️rust` (cargo work). `🗑️generated/1101/framework-green` does not exist, so it was NOT run. Next: after the marker appears (or by the coordinator), `repo run @semio-tech/framework-rs:generate`, then re-run the scoped tsc.

## Files
Edited: the 7 files named above. Tools: `1101-tools/geninputs/lease.ts` (lease-dir probe). Logs: `🗑️generated/1101/geninputs-*.{txt,rc}`.
