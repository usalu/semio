# Audit verify-1019h: read-only verification of six result claims

Auditor `aud-verify8`. Read-only: no source, package.json, bun.lock, taxonomy or git state changed. Only this report and scratch outputs under `🗑️generated/aud-verify-1019h/` were written (scripts `vr_check2.py`, `vr_check3.py`, `closure.py`, `removed_dirs.py`; outputs `*.out`, `sample40.json`). No cargo, no bun repo, no sub-agent. One `bun test` run (see 6).

## 0. Verdicts

| result | claim checked | verdict |
| --- | --- | --- |
| result-value-repoint | 1,025 files; rewrites resolve; facade lines removed | Mostly confirmed. Resolution holds (2,417 references, 40/40 sample). Facade re-exports removed. Doc section 1 exception is stale. "49 workspace deps" NOT reproduced. |
| result-tsc5 | demonstrator shell boot API fix | Confirmed statically (signatures match). Not booted. |
| result-tax3 | path policy; 820 empty dirs; 182 orphans; 18 catalogs | Policy correct, but one consumer still uses removed `maxPathBytes` (silent check loss). 820 dirs: all verified clean. 74 moves verified; 182 vs 74 and 18 vs 21 unreconciled. |
| result-lay14 | renderer policy narrowness | Confirmed: allow-list is exactly the 102 declared `elements` export targets. |
| result-walkperf | fence coverage; binding endpoints | NOT sound as staged: `implementationFiles` covers 10 files of a ~156-file walk closure. Binding-endpoint claim holds for recorded endpoints. |
| result-tco2 | private-item tests; crate split | Partly verified (presence crate exists). Compile and gate ratios not checked. |

## 1. result-value-repoint

Method: `🗑️generated/value-repoint/backup/ledger.json` lists 1,025 files, all present on disk. For each file I diffed the backup against the current file and extracted every new-side reference to `semio_framework_value_{edit,persistent,registry,native_owned,schema}` (brace groups included). Each reference was checked against the owner crate's closure: declared or `pub use`-re-exported names in the value family, following `pub use semio_framework_value_*` re-exports across crates (34 value crates indexed).

- Scope: 2,417 owner-path references in the 1,025 files.
- Name-level closure check: 2,070 resolve directly. The other 347 are all of these kinds:
  - `semio_framework_value_persistent::list::*` (the bulk). `persistent/📦️packages/🦀️rust/🦀️.rs` declares `#[path="../../../🌳️roots/🦀️.rs"] pub mod list`, and `🌳️roots/🦀️.rs` has `pub use semio_framework_value_paged_list::list::*`. I confirmed every distinct `list::` item used in tracked Rust (PagedList, PagedListRefusalKind, PagedListReturnProgress, PagedListError, PagedListProgress, PagedListEditCursor, PagedIter, PagedListEditStep, PagedListAllocationError, `list::persistent`) is declared in the `📋️list` tree.
  - Enum-variant paths (`RequestTerminalKind::{Validated,Faulted,Cancelled}`, `RequestLookupOutcome::{Searching,Matched,Missing}`, `NativeRecordKey::Ordinal`, `PagedListRefusalKind::*`). The enums exist (`📋️registry/🎫️requests/🦀️.rs:56,64`, `🛬️decode/🧳️owned/🧩️record/🦀️.rs:6`).
  - One `persistent::retained_equality::equality_box`, defined at `🌱️value/🌳️tree/🟰️equal/🦀️.rs:85`.
- Sample of 40 (random, seed 1019; `🗑️generated/aud-verify-1019h/sample40.json`): 40/40 resolve after adjudication (36 direct, 2 via the list glob, 2 enum variants).
- Limits: name-level, not module-level. `#[cfg]` and `#[path]` mounts are not parsed and there is no rustc run. Treat this as "every referenced name exists in the owner closure", not "every path compiles".
- Facade: neither `🧰️framework/🔨️modules/🌱️value/🦀️.rs` nor `…/📦️packages/🦀️rust/🦀️.rs` contains any `semio_framework_value_{edit,persistent,registry,native_owned,schema}` re-export now. HEAD's `register_value_schema_exports` is gone. This part of the claim holds.
- Stale statement: result section 1 "Exception kept on purpose" says the facade keeps `native_decoding`, `retained_equality`, `RetainedEqualityCursor` and others for derive output. Current state differs. The receive derive emits `#native::native_decoding::…` with the native-owned root (`✨️derive/🛬️receive/🦀️.rs`), and the equality derive emits `#persistent::retained_equality` (`✨️derive/🟰️equality/🦀️.rs`). No derive output references the removed facade names, so the open "derive decision" is resolved in the tree (not compiled).
- Not reproduced: "49 workspace deps added in 8 workspace roots". A whole-repo `find` for Cargo manifests timed out, and `git grep` sees only tracked files (1 workspace declaration). Count it with an explicit manifest list.
- Not verified: compile (`cargo check` was never completed per the result itself).

Fix (doc): rewrite section 1's exception paragraph to the current derive state. Re-measure the 49 declarations with an explicit file list.

## 2. result-tsc5 (demonstrator shell boot)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🟦️.ts:17`: `bootFrameworkOsDev(options:{catalogRows; variant: string; admission: PluginCatalogAdmissionV1; brands; surfaceSessionFactories?; …})`. The `admission` type (`🧰️framework/🛍️products/🦑️repo/…/🗂️plugin-catalog/🟦️.ts:14`) has maxBytes, maxRows, maxEdges, maxWork, deadlineMs, now, cancelled, progress. The shell's literal matches all of them.
- `♻️mit-bestand/🧺️demonstrator/🐚️shell/🟦️.ts` passes `variant`, `admission`, `catalogRows`, `brands`, `surfaceSessionFactories` (from `@semio-tech/puzzle-2d` `PUZZLE_BOARD_SESSION_FACTORIES`, exported in the puzzle source). Verdict: the claimed required-argument fix is present.
- `createFrameworkOsDevConfig` requires `browserSessionFactories` (`…/🏗️builder/🌐️vite/🟦️.ts:32`). The demonstrator vite config passes `contribution.browserSessionFactories` from `🐚️shell/🧬️schema/🔣️.json`. Verified.
- Not verified: the boot itself (no Vite run). Result says so.
- Open, as the result says: `♻️mit-bestand/🧺️demonstrator/package.json` still carries `semio.browserSessionFactories` (duplicate of the schema JSON).

## 3. result-tax3

### 3.1 Path policy (live)
- `🔣️taxonomy.json` `collisionPolicy`: `maxPathUtf16Units: 256`, `maxComponentUtf8Bytes: 255`. `maxPathBytes` is gone from the schema. Verified.
- `🔍️discovery/📏️path-budget/🟦️.ts` is correct: `pathUtf16Units` = `path.length` (UTF-16 units), components via `TextEncoder` bytes.
- DEFECT: `🔍️discovery/🎁️package-projection/🟦️.ts:69` and `:85` still compare against `taxonomy.collisionPolicy.maxPathBytes`. That field no longer exists, so `Buffer.byteLength(x) > undefined` evaluates to false. The nested-Cargo destination check and the wgpu leaf check are silently disabled at runtime (tsc flags TS2339 only; tsc5 reported it). The tax3 claim that every consumer was renamed is false for this file.
  Fix (library is frozen, queue it): replace both comparisons with the budget module, e.g. `pathBudgetCodes(path, taxonomy.collisionPolicy).length > 0` (import from `../📏️path-budget/🟦️.ts`). Add a test that a 257-unit nested Cargo destination is rejected.

### 3.2 820 empty directories
- `🗑️generated/tax3-run/empty-dirs-removed.json` lists 820 removed paths (kept: 0). I checked all 820, not a sample:
  - 0 still exist on disk.
  - 0 had any tracked file under them at HEAD (`git ls-tree -r HEAD`, 183,881 files).
  - 0 had any tracked file under them in the index (`git ls-files`, 185,909 files).
- Verified: the removals deleted no tracked content.

### 3.3 182 puzzle orphans
- `tax3-tools/rehome-plan.json` lists 74 directory moves. All 74 destinations exist and all 74 sources are gone.
- The report's "182 orphans" and "74 directories" are not reconciled. Either state the unit (orphan fixture entries vs directories) or list the remaining 108 entries. Not verified: the 206 rewritten references (static ref-check was not re-run).

### 3.4 18 catalogs realigned
- `git diff --name-only HEAD -- ✏️s` shows 21 `🔮️oracles/🔣️.json` files changed, not 18. Other fleets may have touched 3. Name the 18 in the report, or list the 3 extra.

## 4. result-lay14 (renderer boundary policy)

File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs`, lines 29-34 and 446-447.

- `RENDERER_HOST_UI_CONTRACT_TARGETS` selects, from the authored `@semio-tech/framework-os` package (`🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/package.json`), exports whose subpath starts with `./modules/renderer/engine/elements/`.
- The manifest has 184 exports: 102 under that prefix, all with one string target, and no wildcard keys anywhere. `exportTargetPatterns` (line 358) therefore emits only literal, escaped, anchored patterns, two per target (owner path and node_modules form). The allow-list is exactly the 102 declared UI-contract targets.
- The other 82 `@semio-tech/framework-os` exports are not allowed by any new entry. `RENDERER_HOST_ROOT` (pre-existing, `^…/📺️renderer/🧑‍🎨engine/(🎯️targets/⚛️react|🧱️elements)/`) is unchanged.
- Verdict: narrow, as claimed.
- Not reproduced: `bun test ./…/🚫️resolved-boundary-classification/🟦️.ts` did not finish within 110 s (no summary). The result reports 5 pass.

## 5. result-walkperf (staged receipt, read-only)

### 5.1 Fence coverage: NOT sound as staged
The verifier accepts a tree when only the 10 files in `protocol.implementationFiles` (`staged/schema.json`) are unchanged. A static relative-import closure from the storage module, the `🗂️workspaces/🟦️bun` files and the generation files reaches 156 files, 127 of them outside the library (`🗑️generated/aud-verify-1019h/closure.py`; type-only imports are included, so this over-approximates).

Concrete fences that run inside the walk and are not hashed:
- `requireNativeStoragePathV1` (symlink, escape and reserved-entry refusal): `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts`. Called by `observeJavascriptDependencyBindingsReadV1` (storage line 82) and by the bootstrap (line 63). Changing its refusal rules leaves receipts valid, although the complete walk would now refuse.
- `withArtifactRelativeDirectoryV1` / `withArtifactDescriptorV1` (file and directory read observation): `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts`.
- Schema validation (`compileJsonSchemaSubset`, `validateJsonSchemaSubset`): `🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts`. The receipt parser and the install schema both use it.
- Store opening and partition layout: `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts`, `…/📂️paths/🟦️.ts`, `…/📐️layout/🟦️.ts`.
- Native caller authority: `⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts`.
- Library files not in the list but on the walk path: `⚡️caching/🔣️policy.json`, `⚡️caching/🧬️schema/🔣️.json`, `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/🔣️.json`.
- Walkperf patches two files that are not in the list: `🏗️build/🚀️bootstrap/🟦️.ts` (the receipt-first decision and the `receipt-required` branch, lines 47-64) and `🗂️workspaces/🟦️bun/🟦️.ts` (the new `readBunWorkspaceListingProjectionV1` export). The second is a forwarder only, low risk.

Fix:
1. Make `implementationFiles` the computed runtime import closure of the roots (storage module, generation core and glue, bun workspaces, bootstrap caller). Hash that closure in capture and verify. Add a law test that walks imports and fails when a reachable non-type module is missing from the closure. Include the two patched files above.
2. Add fixtures that change a fence in an unlisted module (e.g. `reservedApplicationEntries`) and expect `implementation-changed`.

### 5.2 Producer-side inputs not sealed
The producer recipe reads `~/.bunfig.toml`, `$XDG_CONFIG_HOME/.bunfig.toml`, `~/.npmrc` and environment values (`NODE_ENV`, `BUN_INSTALL_CACHE_DIR`, `BUN_INSTALL_GLOBAL_DIR`, `BUN_INSTALL_BIN`, `TMPDIR`, …; storage module, around `javascriptDependencyProducerArgumentsV1` and the "Fixed installer environment anchor" checks). The receipt seals workspace-level bunfig and lock only. The result's claim that a generation is "a pure function of these bytes + runtime constants" is therefore overstated. The complete reader walk does not read them either, so the receipt is not weaker than the current fallback. Fix: fold home and config bytes and the recipe environment into `recipeSha256` (or into `inputs`), or state the limit.

### 5.3 Stamp trust on non-POSIX or bind-mounted trees
Settled directories are trusted by `dev/ino/mode/mtimeNs/ctimeNs` (frontier tier). Devcontainer bind mounts and some Windows volumes can report unstable or zero inode values and stale ctime. Fix: refuse to trust a stamp when `ino` is 0 or the platform is unverified; add a Windows and devcontainer run before enabling the fast path.

### 5.4 Binding endpoints: claim holds for recorded endpoints
- `observeJavascriptDependencyBindingsReadV1` (storage line 82) reads, for each recorded pair, the target chain (path, dev, ino, mode for each segment) and the source link's (dev, ino, mode) plus `realpathSync(source) === target`. It is called with the receipt's pairs and compared to `observationSha256`, both before and after consumption.
- So a retargeted link, a replaced link inode, a removed binding, or a target chain change makes verify return `binding-changed`. Fixtures "binding retargeted" and "binding removed" cover this.
- Residual: the pair set is not re-derived from the live tree. A new or swapped binding is caught only if the containing directory is in the frontier and its stamp or listing changes. Add a fixture "binding added under a listed directory" to make this explicit.

### 5.5 Verification status
Nothing ran from the staged tests (the result says so). Per the result, the nine-test suite passed in a scratch mirror; not re-run here.

## 6. result-tco2 (partial)

- Crate `semio-framework-server-presence` exists (`🖥️server/🔨️modules/📡️gateway/👥️presence/📦️packages/🦀️rust/Cargo.toml`), and the companion `semio-framework-server-tests` exists (`🖥️server/🧪️tests/🥣️companion/…`). Consistent with the claim.
- Not verified: removal of the `testkit` feature in `dsl-derive-expand` (my path search did not find the manifest), the source-gate ratios, and any compile. The result itself says the value carve was not compiling at the time.
- Open per the result: the `🔒️dependencies.json` lock still needs regeneration.

## 7. Not verified or not run in this audit
- No cargo, no `bun repo`, no product run.
- `bun test` for `resolved-boundary-classification` did not finish in 110 s.
- The 49 workspace declarations, the 18-vs-21 catalog count, the 182-vs-74 orphan count, the tco2 `testkit` removal and the gate ratios.
