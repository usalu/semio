# Result lw2b: infinite-world-r3f promotion, link/blob Rust crates, schema catalog, orphan os keys

Executor `lw2b` (follow-up of `result-lw2.md`). No `package.json`/`bun.lock` edit that survives (see 7.1), nothing under the frozen storage/revision paths, no cargo, no git-modifying command, `AGENTS.md` not edited, `.vscode/launch.json` not edited. Tools (inputs, kept): `lw2b-tools/`; generated output removed.

## 1. infinite-world-r3f promoted into the framework

- Moved (plain `mv`, history untouched) `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f` to `🧰️framework/🔨️modules/♾️infinite/🌍️world/🎨️r3f` (next to the existing framework `♾️infinite/🎲️board`). Package name `@semio-tech/infinite-world-r3f`, nx project name and `node_modules` link target keep working; the package's `node_modules` is an absolute symlink and moved with it.
- Inside the package (`lw2b-tools/fix-r3f-internal.ts`): `📜️script.ts` (framework process/vitest imports, 8 to 6 ups, repo library through `🛍️products/🦑️repo`), `🧪️tests/🎚️config/🟦️.ts` (ui builder imports, repoRoot 9 to 7 ups), `📋️project.json` (`$schema`, namedInputs, cwd), `package.json` script cwd.
- Path re-pointing of every consumer (`lw2b-tools/repoint-path.ts`, 15 files; project.json, `🎨️.css` and the ui test by hand): `.storybook` coordination (2), demonstrator and play vite builders, os dev builder vite alias, os dev browser-graph fixture, os renderer React vitest alias, `📋️build-owners.json` root and manifest, fem (2) and stdio (3 standards x 5) oracle `🔣️.json` source lists (74 paths), the vitest-configuration-ownership fixture (3 paths, **and `projectionSha256` recomputed**: the projection hashes `root`; `lw2b-tools/ownership-hash.ts` replicates the gate's projection and reproduces the fixture hash of two untouched owners exactly, r3f: `4619c6d5...` to `3e73e22c...`), the remaining-package-purity fixture (6 paths, non-authoritative snapshot), ui `checked-values` test (relative URL), renderer `🎨️.css` `@source`.
- cad (the only plugin importer, 6 files, 7 specifiers) no longer imports the os-owned name. New SDK facade (explicit lists, 182 values and 62 types generated from the compiler's export table with `lw2b-tools/gen-sdk-glue.ts`): `🧰️framework/📦️packages/🟦️typescript/♾️infinite/🌍️world/🎨️r3f/🟦️.ts`. The cad files import it by relative path (`lw2b-tools/repoint-cad.ts`) until the batch adds the key; the specifier flips to `@semio-tech/framework/infinite/world/r3f` then. os renderer importers keep the bare package name (os to framework, legal).
- `node_modules/@semio-tech/infinite-world-r3f` symlinks of the root and of ✏️s, ♻️mit-bestand, 🌎️hub, 🎓️teaching, 🏢️semio-tech relinked to the new directory (untracked state; `.cache/b/d/m/j/*` install caches and `.🧬semio/🌐hub/*-overlay` still point at the old path and go away on the next `bun install`).
- Root `📜️script.ts` does not import any moved module.

## 2. link and blob Rust crates moved next to their schema (and the schema made a real catalog scope)

First placement (crates beside the four schema docs in `artifact-reference/<x>/`) regenerated into a worse catalog (see 3), so the promotion was completed the way the old os layout was: the whole `🧬️schema` module moved as a unit.

New layout under `🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/` (each is a catalogued `🧬️schema` module):
- `🪆️child/🧬️schema/{🔣️.json,🔗️.graphql,🛰️.proto,🟦️.ts}`
- `🔗️link/🧬️schema/{🔣️.json,🔗️.graphql,🛰️.proto,🟦️.ts,🦀️.rs,📦️codec,📦️packages/🦀️rust,🧪️tests}`
- `📦️blob/🧬️schema/{...same, no codec}`

Document ids (scope id is derived from `$id`): `framework/schema/artifact-child.json` to `framework/schema/artifact-reference/child/schema.json` (scope `framework.schema.artifact-reference.child`), `artifact-link.json` to `.../link/schema.json`, `blob-ref.json` to `.../blob/schema.json` (`lw2b-tools/reid.ts`, 63 files: plugin schemas, hub/os Rust tests, derive index, Rust registry, the three docs). 153 files got the new directory level in their paths (`lw2b-tools/insert-schema-dir.ts`: graphql/proto imports, `include_str!` paths, schema-ts facade, build-owners, Cargo members).

Rust/cargo side (no cargo run):
- Crate names, `id` metadata and nx project names are unchanged (`semio-framework-os-store-link-schema`, `semio-framework-os-store-blob-identity`, `@semio-tech/os-store-{link-schema,blob-identity}-rs`), so none of the ~190 `Cargo.lock` files and no launch entry changes. The `os-store` naming is now misleading for framework crates; renaming needs a cargo-capable executor because native builds run `cargo build --locked` (re-sorting ~190 locks by hand is not safe).
- Workspace membership: removed from the root `Cargo.toml` members (2 lines), added to `🧰️framework/Cargo.toml` members (sorted beside `🔗️borrow`), and `🧰️framework/Cargo.lock` got the two package blocks (alphabetical, dependencies sorted; the root and plugin locks are unchanged because the packages stay reachable as path dependencies). `workspace` key now points at `🧰️framework` (7 ups), dependency paths 7 ups, link to blob `../../../../📦️blob/🧬️schema/📦️packages/🦀️rust`.
- Dependents re-pointed (`lw2b-tools/repoint-crate-deps.py`): `semio-framework-os-kernel` (os `📦️packages/🦀️rust`), `-os-store-sync-native`, `-os-store-space-member`, `-os-store-testing-fixtures`, `-os-store-child`; the `include_str!` of both crate sources and four scope names in `STORE_SCHEMA_EXPORTS` (`os.store.{child,link,blob}` to `framework.schema.artifact-reference.{child,link,blob}`; `os.store.child.owner` stays os) in `🧰️test-support/📥️lifted/🏪️store/🦀️.rs`.
- Scripts and tests: the two `📜️script.ts` now use the framework-owned admitted cargo runner (`runAdmittedCargoTestsV1`, same as the 66 other framework crates and `🔗️borrow`) instead of the repo product runner; `📋️project.json` gained `cwd` like `🔗️borrow`. The contract tests no longer read os source (framework to os reverse edge): their three store-reexport assertions moved to the new os test `🏪️store/🧪️tests/🪞️artifact-reference-reexports/🟦️.ts`.
- Repairs of lw2 leftovers found on the way: stdio sqlite `🫙️prefix` test still read the old os schema paths (failed, now 48/48).
- Launch seed: no change. Entries are path-agnostic (`bun repo run @semio-tech/os-store-*-rs:...`) and the names did not change; `workspace:generate-launch` has nothing to regenerate for this move. Taxonomy: no hand edit; directory names are flat vocabulary and the new `🧬️schema` modules are admitted by the `framework-module` level (`🧰️framework/🔨️modules/**`).

## 3. Schema catalog regenerated through its generator

`bun ./📜️script.ts schema generate` then `schema docs` (both foreground-equivalent, about 2.5 min each; `🔣️schema-catalog.json` and `📓️schema-catalog.md`).
- Scopes 3566 to 3567: `os.store.{child,link,blob}` (stale paths) replaced by `framework.schema.artifact-reference.{child,link,blob}` (level `framework-module`, paths real); `framework.replication.mutation.canonical.authority` is a peer's addition.
- Regenerating with lw2's layout (docs directly in `artifact-reference/<x>/`) dropped the three scopes and added 106 `schema-ref-unresolved` diagnostics (docs outside a `🧬️schema` dir are not catalogued: the generator only walks `🧬️schema` modules). With the layout in 2, `schema-ref-unresolved` is 958 (was 1,035 on that first run), the three scopes resolve and 27 scopes list them in `dependsOn` again (52 documents `$ref` the child schema).
- Remaining related finding, pre-existing: 12 refs to `framework/schema/artifact-reference.json#/$defs/ArtifactRef` do not resolve (that sibling doc sits in `🗿️artifact-reference/🔣️.json` outside a `🧬️schema` module and its id `framework/schema/artifact-reference.json` would clash with scope `framework.schema`). Fix is the same move for the parent doc (`artifact-reference/🧬️schema`, id `.../artifact-reference/schema.json`) and a rename of the id in every mutation schema; not done (hundreds of files, unrelated to this move).
- The catalog check still reports about 18.3k findings repo-wide (`schema-export-incomplete` 13k dominates); that is the standing baseline, not new.
- `🗂️schema-document-index.json`: only my 3 entries re-keyed and re-ordered by the projection's own sort (`lw2b-tools/derive-index-projection.ts`); the projection also wants nine peer entries, which I left to their owners. 4,411 entries, 0 missing, 0 id mismatches (`lw2b-tools/check-derive-index.ts`).

## 4. Batch spec (orphan os keys, new SDK key, workspaces)

`lw2b-tools/exports-needed.json` (regenerable with `lw2b-tools/exports-needed.ts`):
- Add to `@semio-tech/framework` (`🧰️framework/📦️packages/🟦️typescript/package.json`): `"./infinite/world/r3f": "./♾️infinite/🌍️world/🎨️r3f/🟦️.ts"` (glue exists, covered by the existing SDK surface law once listed); then rewrite the six cad importers to `@semio-tech/framework/infinite/world/r3f`.
- Remove from `@semio-tech/framework-os` (`🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/package.json`): `./modules/store/blob/schema`, `./modules/store/child/schema`, `./modules/store/link/schema` and delete the three glue files `📦️packages/🟦️typescript/🔨️modules/🏪️store/{📦️blob,🪆️child,🔗️link}/🧬️schema/🟦️.ts` (importers left: 0; replacement `@semio-tech/framework/schema/artifact-reference`).
- 22 plugin `package.json` declare `@semio-tech/infinite-world-r3f` and import it nowhere (cad imports through the facade): drop the dependency from all 22; cad gains nothing new (it already depends on `@semio-tech/framework`).
- Workspace entries of the root, ♻️mit-bestand, ✏️s, 🌎️hub, 🎓️teaching, 🏢️semio-tech `package.json`: the new path is already present in all six (see 7.1); `bun.lock` files regenerate with the next `bun install`; `🔒️dependencies.json` (`users` of `@react-three/drei`, `@react-three/fiber` and the two Rust crates) still lists the old `package.json`/`Cargo.toml` paths until its generator runs.

## 5. Verification (exact commands, outcomes)

All from the repo root.
- Type-check (`lw2-tools/tc.ts`, strict TypeScript program, fails on unresolved modules): SDK glue + 6 cad files + all 27 r3f sources: 34 files, 0 diagnostics. The 31 os files importing the bare name: 12 diagnostics, all `TS2488 NodeListOf` lib flags of the checker on untouched test files (same class lw2 reported).
- `bun test` link, blob, artifact-reference neutral suite and the new os re-export suite: 18 pass, 0 fail (71 expects; workspace key, manifest dependency list, Ajv and TOML oracles, source ownership). `🧪️tests/📤️module-package-surface` 6/6. stdio sqlite prefix 48/48.
- `lw2b-tools/check-refs.ts`: 5,247 schemas with `$id`; unresolved semio refs 3 (the same 3 as before lw2b: stdio gltf x2, a reactor job schema); 52 refs to the child schema; 0 stale old ids.
- Rust static (`lw2b-tools/rust-static.ts`, no compiler): every path dependency of the 2 crates and the 5 dependents resolves to a manifest of the declared package; `workspace` keys resolve to a `[workspace]` that lists the crate; lib and test paths exist; root workspace no longer lists the crates, framework workspace does; framework `Cargo.lock` blocks sorted, new dependency lists sorted and complete; 37 `include_str!/include_bytes!` literals in the 10 touched `.rs` files all resolve; source closure (`readRustSourceClosureV1`): link 2 files / 409 active lines / 23,510 bytes, blob 1 / 21 / 1,154, test-support 21 / 907 / 78,297, all inside the global gate (4000/32/262144) and the declared per-crate boundaries. The moved Rust sources are byte-identical to `HEAD` (diff of 5 files).
- vitest r3f owner (`lw2b-tools/runvt.py`, config of the package): reaches test collection through the moved relative imports, then stops on `@dnd-kit/sortable` missing in `node_modules` (same gap lay6 reported; a peer is adding it to the root `package.json`). `bun build --target=bun 📜️script.ts`: one error, a peer's `toolJobOwnerSourceText` export in `⚖️laws/🎯️tool-job-coverage`; nothing about moved modules.
- `bun ./📜️script.ts schema generate --check` right after regeneration was not repeatable: a peer's taxonomy edit (`generatorContracts["ui-axes"].inputPatterns` unordered) makes `loadCatalogTaxonomy` throw. Rerun it once that is fixed; expect stale only from peer schema edits.
- Not run: any `bun repo run`, cargo, the vitest-configuration-ownership gate end to end (a peer's taxonomy edit made `loadCatalogTaxonomy` throw at that moment: `generatorContracts["ui-axes"].inputPatterns` not ordered, not mine), the full naming-law walk (see 6).

## 6. For the compile executor (crates to check, in dependency order)

1. `semio-framework-os-store-blob-identity` (framework workspace, `-p`; tests `blob-identity-neutral`)
2. `semio-framework-os-store-link-schema` (framework workspace; test `link-schema-neutral`)
3. `semio-framework-os-store-test-support` (compiles the moved `.rs` via `include_str!` and the registry scope names)
4. `semio-framework-os-store-child`, `semio-framework-os-store-testing-fixtures`, `semio-framework-os-store-space-member`, `semio-framework-os-store-sync-native` (root workspace)
5. `semio-framework-os-kernel` (re-exports link/blob, root workspace), then the existing cargo/wasm consumers of the store.
Expect cargo to rewrite nothing in the root lock; if the framework lock gets touched it is only the two inserted blocks. Native builds use `--locked`, so a lock diff is a finding.

## 7. Notes and still open

1. Incident: while re-pointing path strings I briefly rewrote the old r3f workspace entry in the six `package.json` files (my file list filter missed them) and restored them in the same step; the files currently carry the new path in sorted position and a regenerated list, i.e. a concurrent actor owns that content. Nothing of mine remains in them.
2. Crate rename `os-store-*` to `artifact-reference-*` (cargo-capable executor, regenerates ~190 locks).
3. Parent artifact-reference doc not catalogued (12 diagnostics), see 3.
4. Naming laws (`policyEmojiPrefixBreaches`/`policyBannedNameStemBreaches`, `lw2b-tools/law-shape.ts`, repo-wide, 25 min): banned name stems 0. Emoji uniqueness: 4,415 repo-wide (standing class); three entries touch the moved area: `artifact-reference/🔗️link` and `🔗️borrow` share 🔗 with the sibling file `🔗️.graphql` (`🔗️link` came with lw2, `🔗️borrow` pre-exists), and `🔗️link/🧬️schema/📦️packages` shares 📦 with `📦️codec` (moved as it was in the os). No new class; renaming `🔗️link` would re-touch the 153 path files again.
5. Host-contract vitest suites of lw2 item 4.2 and the manifest batch entries of lw2 are unchanged by this work.
