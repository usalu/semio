# Audit Granularity 1020e: Can an edit rebuild little, and can two agents on different crates avoid collisions?

Auditor `aud-granularity-1020e`, read-only on the repo. No builds, no `bun repo`, no cargo, no git. Scratch scripts and JSON are under `/private/tmp/aud-granularity-1020e/` (`graph.py`, `analyse.py`, `analyse2.py`, `budget.py`; outputs `crates.json`, `graph2.json`, `budget.json`). The system Python is 3.9 without `tomllib`, so manifests were read with a line-based parser. Edges and fan-in are therefore INFERRED from manifest text, not from `cargo metadata`.

Tags: **VERIFIED-READ** = I read the file or the cited text. **INFERRED** = computed or estimated by me.

## 0. Answer

No. The partitioning is by directory, but the dependency graph is one tall, wide tower. A typical crate edit touches about 261 crates on average (8 % of 3,213). Editing the value tower touches about 2,880 crates, and the chain above it is 122 crates deep. Two agents on different crates collide less at the source level (no shared lib root files) than at the shared manifests, lockfiles and nx keys (section 7).

## 1. Cargo workspaces and member counts

- VERIFIED-READ `/Users/ueli/Documents/semio/Cargo.toml` (122,740 bytes): `[workspace]` with 564 `members` and 236 `exclude` entries. The excludes are 232 nested workspace roots (each has its own `[workspace]` table), 3 glob patterns, and 1 entry without a `Cargo.toml`.
- Total: **233 Cargo workspaces** (1 root + 232 nested). Each nested root is a separate Cargo workspace; the root excludes them from its membership.
- Literal members of the nested workspaces: 2,667 (INFERRED from their own `members` arrays). 93 of them have no `members` key, so they are single-package workspaces (INFERRED).
- Crate manifests found: **3,213** (564 root, 408 in `/Users/ueli/Documents/semio/🧰️framework/Cargo.toml`, about 2,241 in the other 231 nested workspaces). Membership claims are disjoint (0 conflicts, INFERRED).
- Largest workspaces by members: root 564; `🧰️framework` 408; `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model` 237; `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio` 178; `🌎️hub` 152; `…/🌀️procedural/🗿️artifacts/🧊️generation3d` 90.
- 231 of 232 nested workspaces have their own `Cargo.lock`. The root lock is VERIFIED 707,310 bytes with 1,562 `[[package]]` entries.

## 2. Source-boundary budget (4,000 lines / 32 files / 262,144 bytes per crate closure)

- Canonical rule: VERIFIED-READ memory note `project-source-boundary-gate-and-static-rust-oracle` and `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/audit-census10-1020.md` §1–2 (run 10-20 02:15 with the canonical `crate-gate8-disk.ts`): **prod cfg: 0 crates over the gate, 0 refused; test cfg: 1 over (`semio-framework-actor`, bytes)**.
- My own estimate (all cfgs, module walk incl. `#[path]` modules and `include_*!`, raw line count): **30 crates over 4,000 lines** and 0 over files or bytes (INFERRED). These are an upper bound because they include test modules. The 30 are: `semio-framework-schema` (4,513 lines, 11 files), `semio-framework-geometry-skeleton` (6,525), `semio-framework-ui-scene-views` (5,497), `semio-framework-ui-scene-modelling` (4,360), `semio-framework-ui-render-shader-contract` (4,410), `semio-framework-course-presence` (4,492), and 24 plugin/hub artifact crates (`semio-s-artifact-*`, `semio-hub-artifact-creation`, 4,052 to 8,985 lines), all with fan-in 0.
- Closure impact, all-cfg upper bound (INFERRED): 2,254 of 3,213 crates have one of the 30 in their normal+build closure. The five framework crates alone drive 2,235 of them (union of transitive dependents). The canonical prod-cfg census shows 0. The gap is most likely test-module counting (INFERRED), so the framework crates are still the place to cut headroom.
- Near the line on the active-line measure: `semio-framework-schema` (about 3,972 non-blank non-`//` lines, INFERRED).

## 3. Fan-in and dependency depth

Measured over 45,013 normal+build edges (dev-deps excluded; no cycles). Mean transitive fan-in 260; median 1; 1,429 crates have zero dependents; 484 have more than 100 transitive dependents; 352 have more than 500 (INFERRED). A uniformly random single-crate edit rebuilds about 261 crates (8.1 %).

Top hubs (transitive dependents / direct dependents), INFERRED:

| Crate | Transitive | Direct | Note |
|---|---:|---:|---|
| `semio-framework-value-control-contract` (`🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract`) | 2,882 | 24 | leaf, sits at the bottom of the value tower |
| `semio-framework-value-allocation-return` | 2,881 | 3 | value retirement chain |
| `semio-framework-value-paged-list` | 2,880 | 8 | |
| `semio-framework-value-retirement` | 2,879 | 24 | |
| `semio-framework-schema-registry` | 2,875 | 1,188 | build-free, high direct fan-in |
| `semio-framework-hash` | 2,873 | 442 | |
| `semio-framework-io-base64` | 2,869 | 327 | stable leaf, low churn |
| `semio-framework-value-derive` (proc-macro) | 2,866 | 1,909 | see section 4 |
| `semio-framework-schema` (over budget) | 1,986 | 1,481 | |
| `semio-framework-geometry-skeleton` (over budget) | 1,923 | 1 | |
| `semio-framework-ui-scene-modelling` (over budget) | 1,827 | 2 | |
| `semio-framework-ui-scene-views` (over budget) | 1,826 | 7 | |

Depth: the longest normal+build chain is 122 crates, from `semio-framework-value-control-contract` up through the `os-renderer-wgpu` shell tests (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/…/🧊️wgpu/…`). 2,713 crates have a height of at least 20 (INFERRED). An edit to the value contract therefore forces a 122-crate sequential compile chain, regardless of the number of cores.

## 4. Proc-macro and build.rs on hot paths

- Proc-macro crates (VERIFIED by `[lib] proc-macro = true` in each manifest; direct / transitive dependents, INFERRED):
  - `semio-framework-value-derive` (`🧰️framework/🔨️modules/🌱️value/✨️derive`): 1,909 / 2,866
  - `semio-framework-dsl-record-derive` (`🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive`): 844 / 2,440
  - `semio-framework-os-kernel-dsl-derive` (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive`): 18 / 2,443
  - `semio-framework-async-macros` (`🧰️framework/🔨️modules/⏳️async/✨️macros`): 40 / 2,396
  - `semio-framework-dispatch-macros` (`🧰️framework/🔨️modules/🔀️dispatch`): 350 / 1,756
  - `semio-framework-schema-derive` (`🧰️framework/🔨️modules/🧬️schema/✨️derive`): 1 / 1,987
  - `semio-framework-machine-derive` (`🧰️framework/🔨️modules/🔄️machine/✨️derive`): 1 / 1,690
- build.rs: only one crate has a real build script, `semio-framework-os-renderer-wgpu` (`…/📺️renderer/…/🧊️wgpu/📦️packages/🦀️rust`, fan-in 2). Its `build.rs` has no `rerun-if-changed` (VERIFIED-READ by grep). Nineteen other crates set `build = false` explicitly (VERIFIED by manifest parse). My earlier 20-crate build-script count was wrong; this is the corrected one.

## 5. Cross-workspace coupling

- 33,533 of 45,013 normal+build edges (74.5 %) cross a workspace boundary (INFERRED). 29,418 of those point into the `🧰️framework` workspace. Each nested workspace has its own `Cargo.lock`, so the plugin, hub and course workspaces depend on framework source through path deps, not through a published artifact.

## 6. Source layout that breaks directory partitioning

- 1,910 of 3,213 crates set `[lib] path` to a file outside their own package directory, for example `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/📦️packages/🦀️rust/Cargo.toml` with `path = "../../🦀️.rs"` (VERIFIED-READ). No two crates share a lib root file (0, INFERRED).
- Consequence: the nx `cargo` namedInput is `{projectRoot}/**/*.rs`, which does not cover those out-of-directory roots (VERIFIED nx.json; INFERRED gap). The staged input-precision set (`/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/result-input-precision.md`) is the planned fix.

## 7. Two-agent collision points (shared files and keys)

- Root `Cargo.toml` (122,740 bytes; 123 `workspace.dependencies` path entries) and root `Cargo.lock` (707,310 bytes): one edit by any agent in the root workspace rewrites the shared file.
- `/Users/ueli/Documents/semio/.cargo/config.toml` (VERIFIED-READ): one shared `target-dir` and `build-dir` for all 233 workspaces; `rustc` is a bun wrapper script; `fine-grain-locking = true`. Parallel builds serialize on per-unit locks in one directory.
- `/Users/ueli/Documents/semio/nx.json` (VERIFIED-READ): the `cargo` namedInput includes `{workspaceRoot}/Cargo.toml`, `{workspaceRoot}/.cargo/config.toml` and `{workspaceRoot}/rust-toolchain.toml`. An edit to any of them keys all cargo projects anew.
- `schemaSources` in `nx.json` includes `{workspaceRoot}/**/🧬️schema/**/*`, `{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` and the discovery `*.ts`. Any schema file edit in any agent's crate invalidates every project that uses `schemaSources` (VERIFIED nx.json; the project count is not measured).
- TS: `✏️s/tsconfig.json` and `🌎️hub/📦️packages/🟦️typescript/tsconfig.json` (VERIFIED-READ, below).

## 8. TypeScript program roots

- Root `package.json`: 134 literal `workspaces` patterns, no globs (VERIFIED). No root script mentions `tsc` or `tsconfig` (VERIFIED).
- 16 of the 134 have a `tsconfig.json` (VERIFIED). Five use a broad include that leaves their own package:
  - `…/🛍️products/🦑️repo` (`**/*.ts`, `**/*.tsx`; about 2,264 `.ts` files under it, INFERRED)
  - `…/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript` (`../../**/*.ts`)
  - `…/renderer/…/⚛️react/📦️packages/🟦️typescript` (`../../**/*.ts`)
  - `…/📚️library/📦️packages/🟦️typescript` (`**/*.ts`, `../../**/*.ts`)
  - `…/🔌️plugin/🪟️window-kits` (`**/*.ts`)
- `✏️s/tsconfig.json` (VERIFIED-READ): `include: ["**/*.ts", "**/*.tsx"]` over the whole `✏️s` tree (about 7,877 `.ts` files by my count; `result-input-precision.md` reports 12,959 program files including libs, tsc-verified, 0 missing). Its `paths` map points into `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle.d.ts`, a generated wasm-bindgen file.
- `🌎️hub/📦️packages/🟦️typescript/tsconfig.json` (VERIFIED-READ): `include: ["../../**/*.ts"]`, which is the whole hub tree (about 279 `.ts` by my count; `result-input-precision.md` reports 2,220 program files including libs).
- Root `tsconfig.json` (VERIFIED-READ): `include: ["**/*.ts", "**/*.tsx", …]` for the whole repo. Not used by any root script.

## 9. Top 20 restructuring opportunities (ranked by expected incremental-rebuild savings)

Savings are the rebuild set removed per edit that becomes local, weighted by blast radius. Edit frequency is not measured, so the order is INFERRED.

1. **nx `cargo` key**: drop root `Cargo.toml`, `.cargo/config.toml` and `rust-toolchain.toml` from every project key; key on the nearest workspace manifest and lock. Now: any root-manifest edit re-keys all 3,213 crates.
2. **nx `schemaSources`**: replace the repo-wide `**/🧬️schema/**/*` glob with per-project schema closures. Now: any schema edit re-keys every consumer.
3. **Value tower core**: split `value-control-contract`, `value-allocation-return`, `value-paged-list`, `value-retirement`, `value-shared-issuance` into stable API crates plus implementation crates. Now: about 2,880 crates per edit; it also shortens the 122-crate critical path.
4. **`value-derive` proc-macro**: move runtime support out of the proc-macro crate so the macro only emits calls. Now: 1,909 direct / 2,866 transitive.
5. **`schema-registry`** (2,875 transitive, 1,188 direct): separate registry data types from its runtime.
6. **`hash`** (2,873 transitive, 442 direct): separate algorithm from type definitions.
7. **`semio-framework-schema`** (1,986 / 1,481 direct; over budget at 4,513 raw lines): split derive-facing and registry-facing parts; also restores gate headroom.
8. **`semio-framework-geometry-skeleton`** (1,923; 6,525 raw lines in 3 files): split the largest file set out of the skeleton API.
9. **`ui-scene-views` and `ui-scene-modelling`** (1,826 / 1,827; over budget): split views from modelling types.
10. **Framework workspace boundary**: 29,418 cross-workspace edges into `🧰️framework` (INFERRED). Publish framework as a versioned artifact (or a stable cached `rlib` set) that plugin workspaces consume, instead of path deps on live source.
11. **`dsl-record-derive`** (844 direct, 2,440 transitive): keep derive output stable so only schema-facing edits propagate.
12. **`async-macros` and `os-kernel-dsl-derive`** (2,396 / 2,443 transitive): split emission from parsing so macro edits do not rebuild the derive dependents.
13. **`dispatch-macros`** (350 direct, 1,756 transitive): same split as 12.
14. **`ui-render-shader-contract`** (370 dependents; 4,410 raw lines): split contract from shader data (over budget).
15. **`io-base64`** (2,869 transitive, 327 direct): a stable leaf, so low churn; move it under a std-only leaf crate to keep it out of the value chain.
16. **Lib roots outside package dirs** (1,910 crates): colocate lib roots with `src/` or make the cargo key use the crate closure, which closes the nx under-keying gap (section 6).
17. **Root `Cargo.lock`** (707 KB, 1,562 packages): one lock per product, to remove the shared-lock collision when two agents add dependencies.
18. **`✏️s/tsconfig.json`** (`**/*.ts` over 7,877 `.ts` files): split into per-plugin tsconfigs with project references, so one plugin's edit does not re-derive the whole program.
19. **Generated hub bindings**: publish `semio_puzzle.d.ts` as its own package or cache key instead of a `paths` mapping into hub `pkg/`.
20. **Broad tsconfig includes**: replace `../../**/*.ts` (ui, renderer, library) and the repo-product `**/*.ts` with explicit per-package includes; drop the unused root `tsconfig.json` include.

## 10. Verification and limits

- VERIFIED-READ: root `Cargo.toml` structure, `nx.json` namedInputs and schemaSources, `.cargo/config.toml`, `✏️s/tsconfig.json`, `🌎️hub/📦️packages/🟦️typescript/tsconfig.json`, root `tsconfig.json`, root `package.json` workspaces, one member manifest with out-of-dir `[lib] path`, `audit-census10-1020.md` §1–2, `result-input-precision.md`.
- INFERRED: crate graph (line-based manifest parser; `workspace = true` path deps resolved through the owning workspace), fan-in and depths, source sizes (`mod`/`#[path]`/`include_*!` walk, raw lines, all cfgs), TS file counts (`.ts` only).
- Not done: `cargo metadata`, `bun repo` census (the canonical counter), nx project-count for `schemaSources`, measured edit frequencies.
- Ticket hygiene: no files were written outside this report; scratch lives in `/private/tmp/aud-granularity-1020e/`.
