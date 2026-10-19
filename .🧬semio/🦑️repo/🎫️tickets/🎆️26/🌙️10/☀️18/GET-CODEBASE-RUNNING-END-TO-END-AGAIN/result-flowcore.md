# Result flowcore: OS flow `🫀️core` rename and vendored jco shim

Executor `flowcore`. No `package.json`/`bun.lock` edit by me (the tool `flowcore-tools/manifest-batch.py` was written but never run), no cargo, no git-modifying command.

## 1. Rename `🌊️flow/🫀️core` to `🌊️flow/🎁️browser-package`

Under `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/`. The directory is not a Rust crate. It holds the Flow browser package: the wasm-bindgen output `🕸️bindings` plus the build/test owner `📦️packages/🦀️rust` (Nx project, `📜️script.ts wasm|test|...`), the name already used by the generator contract `flow-browser-package` and `publishFlowBrowserPackage`. Same depth, so every `../` in `📜️script.ts` stays valid.

Renamed (no alias, no symlink): directory; Nx project `semio-framework-os-flow-core` to `semio-framework-os-flow-browser-package` (project.json, build-owners, root `📋️project.json`, play `📋️project.json`, repo-lib `📋️project.json`, rebuild/catalog fixtures, selected-runtime fixtures, nx-contract fixtures, 39 launch.json and 29 launch.seed configurations (commands only; order and groups untouched, both parse, no new duplicate names)); wasm build `logPrefix` `os/flow/browser-package`; bindings.json owners; taxonomy (`members-of-members-of-modules` row, generator contract `flow-browser-package` ownerPath/inputPatterns/outputRoots re-sorted lexically, fixedFilename pathPatterns, bootstrap source `flow-core-package` output); hub `🧩️compositions/🌊️flow/🧩️component` Cargo.toml `engines`; storybook coordination; verify command; playground-targets; vite engines catalog; tests/fixtures that name the path (40 files, list in `flowcore-tools/files-all.txt`). The resolved-boundary fixture case `resolved-flow-core-directory` became the synthetic `resolved-nested-core-directory` (`…/🎁️browser-package/🧱️core/…`) because the real path is no longer banned.

Deliberately kept (package.json frozen): npm names `@semio-tech/flow-core` and `@semio-tech/flow-core-build`, wasm basename `flow_core`, taxonomy contract ids `flow-core-*`. Recommended later rename for the manifest batch: `@semio-tech/flow-browser-package` and `@semio-tech/flow-browser-package-build`, wasm basename `flow_browser_package` (touches 7 files that name the package; list via `git grep flow-core`).

Not touched (not `flow/🫀️core`): `🖥️host/🧱️core` (crate `semio-framework-os-flow-host-core`, 271 references), `members-of-engine` row `🫀️core` (stale, no such dir exists any more), `POLICY_DIFF_COMPLETENESS_ALLOWLIST` entry `"flow/core"`, `.cursor` plans, ticket history.

### Manifest state at the end (recorded, not edited by me)
- All six `package.json` workspaces lists already name `🎁️browser-package` (edited by someone else at 22:31).
- All six `bun.lock` still name `🫀️core` (4 occurrences each: two workspace keys, two `@semio-tech/flow-core*@workspace:` specs). The deps executor must relock; `flowcore-tools/manifest-batch.py <root>` replaces `🌊️flow/🫀️core` by `🌊️flow/🎁️browser-package` in all 12 files if a text patch is wanted.
- `node_modules/@semio-tech/flow-core` and `flow-core-build` are symlinks into the old path and dangle until the relock re-links them: this is why tsc now reports TS2307 for `@semio-tech/flow-core` in `renderer/…/hidden-tab-surfaces`, `node-graph-hosts` and `dev/host-ports/wasm-bindings`.
- Last attempt `bun repo run semio-framework-os-flow-browser-package:test-browser-ownership` stopped in dependency inspection with "Locked workspace package lacks original admitted Source", the same stale-lock cause.
- The bootstrap source generator already re-materialized `🎁️browser-package/🕸️bindings/package.json` once (race with my move); I kept the tracked content from the old directory, the generator rewrites it with identical content.

## 2. jco preview2 shim: vendored, now a declared taxonomy mechanism

Evidence: `🧪️testing/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/*` is byte-identical to `@bytecodealliance/preview2-shim@0.30.0` `dist/browser` for common, in-memory-http, in-memory-filesystem, in-memory-sockets, opfs-filesystem, index, io, clocks, config, environment, filesystem, random, sockets (http.js and cli.js carry small local differences). The `http.js` to `in-memory-http.js` cycle and `common.js` are upstream, so it is vendored, not ours.

Mechanism (schema-first): new `dependencyDirections.vendoredPaths` in `🔣️taxonomy.json` (value: the shim directory), required in `🧬️schema/🧱️dependency-direction/🔣️.json`, typed in `taxonomy-document` and `DependencyPolicyTaxonomy`, audited in `audit-dependency-directions`, applied in `🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs` (validated, then added to `IGNORED_GRAPH_PATHS`, i.e. cruiser `exclude` and `doNotFollow`). New language-agnostic test `…/🏗️construction/🧪️tests/🏺️vendored-exclusion/🟦️.ts` plus `🧫️fixtures/🔣️.json`: portable 3-case corpus (declared, undeclared control, non-segment-prefix control) checked against dependency-cruiser and against the exclude patterns evaluated directly, plus a check on the real taxonomy.

Overlap to know: a peer added `bannedNameStemExemptions.vendoredFiles` (only `common.js`) in the same `.cjs`/taxonomy while I worked. It serves the filesystem naming law; mine serves the graph. They coexist (all construction tests green). Both hold the old `🧪️testing/🧩️jcoprobe` path: if the pending jcoprobe relocation to `🧪️tests/🧫️fixtures/🧩️jcoprobe` happens, update both entries together with the `jco-vendor-*` contracts.
Open (pre-existing, outside this task): the implementation-leaf census still reports `kind-only-basename` for the 5 shim files without a `jco-vendor-*` contract (`common`, `in-memory-{filesystem,http,sockets}`, `opfs-filesystem`); adding contracts would break the pinned count 48 in `generated-outputs`.

## 3. Verification

- dependency-cruiser, OS scope (`--include-only ^<os>/`, lay0 config, 3,508 modules): before (lay6/lw8 numbers) no-core-path 12, no-circular 219; after no-core-path **0**, no-circular **1** (only the gitignored generated bundle `🧑‍💻dev/📤️distribution/🧶️bundles/🖥️runtime-*.js`; the policy owner can add `**/📤️distribution` to `implementationLeafPolicy.ignoredPathPatterns` and the pinned list in `audit-leaf-and-reference-policies`), no-cross-package-relative 689, renderer-hosts-only-ui 54.
- `bun test` construction + bootstrap + new test: 25 pass, 0 fail (`resolved-boundary-classification`, `🚀️bootstrap/🧪️tests`, `🏺️vendored-exclusion`). `host-activation` 1 pass. `generated-outputs` "Flow compiler output boundaries" pass (re-reads the renamed `bindings.json`; the other 4 failures there are the pending jcoprobe fixture relocation and an ignored `Cargo.lock`). `dependency-direction` 17/18 (failure: path capacity of `✏️s/🔌️plugins/🧪️example`), `path-emoji-statutes` 36/37 (one 30 s timeout under load). `package-discovery`, vite `artifacts` and `build/🕸️graph` failures are unrelated environment or peer issues (writer role `hub`, ticket sink, "Original parent Script process invocation required").
- `validateTaxonomy` via `loadCatalogTaxonomy` passes (it caught the lexical-order rule for `flow-browser-package`, fixed).
- tsc OS (`os/tsconfig.json`): 81 errors; none new from the rename except the 4 dangling-symlink TS2307 above; `semioSetRuntimeAssets` missing in the stale built `flow_core.d.ts` is pre-existing.
- Path resolution oracle `flowcore-tools/resolve-check.py`: all 58 literal references resolve (the 8 reported are published-name or fixture strings, not paths). Final grep: 0 occurrences of `🌊️flow/🫀️core`, `semio-framework-os-flow-core`, `🫀️core/(🕸️bindings|📦️packages|🧪️bindings)` outside tickets, `.cursor`, and the six `bun.lock`.
- Rust (static): no crate source or `Cargo.toml` of a workspace crate changed except the string path in `🕸️wasm/🧪️tests/🔬️component-domain-laws/🦀️.rs` (resolves: `../../../🎁️browser-package/📦️packages/🦀️rust/📜️script.ts` from the crate's `fixture_path!` base, checked by file existence) and hub component `Cargo.toml` metadata `engines`. For the compile executor: `semio-framework-os-flow-wasm` (test `component-domain-laws`) and `semio-framework-os-flow-component` (metadata only). No crate was renamed; no workspace-member change.

## 4. Open
- Relock (`bun.lock` x6) and dependency sync, then re-run `bun repo run semio-framework-os-flow-browser-package:test-browser-ownership` and `:test-browser`.
- Rebuild the wasm bindings (stale `flow_core.d.ts`).
- Rename `🖥️host/🧱️core` and the npm package names (section 1) when the freeze lifts.

Tools kept in `flowcore-tools/` (`rename.py`, `manifest-batch.py`, `resolve-check.py`, `census.ts`, file lists). Generated output deleted.
