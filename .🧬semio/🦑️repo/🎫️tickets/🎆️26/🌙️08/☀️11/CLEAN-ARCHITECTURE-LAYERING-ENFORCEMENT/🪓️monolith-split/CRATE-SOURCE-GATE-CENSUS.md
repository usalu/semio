# 📏️ Crate Source Gate Census

Read-only audit of the compiler-ingress source boundary over the semio Rust crates. No source edited, no git writes, no cargo builds. Snapshot taken 2026-10-15 ~11:00 local time while peer fleets were still carving; the numbers drift (see "Snapshot drift").

Machine output: `🗑️generated/crate-census.csv` (210 nodes), `🗑️generated/census-summary.json`, `🗑️generated/analysis.txt`, `🗑️generated/peer-activity.tsv` (all under this ticket folder).

## Headline

- 210 first-party Rust nodes (90 root workspace members + 120 in-repo path-only deps, many under `[workspace] exclude`).
- 125 of 210 are closure-compilable today (their whole in-repo dependency closure is within limits). Among the 90 members, 33 are compilable.
- 31 crates are oversized. 21 are members, 10 are path-only. 2 are refused outright by the closure scanner, independent of size.
- Six carvable leaves unlock first (tier 0): `compiler`, `pets`, `quiz`, `repo-graphql`, `replication`, `ui-contract`. Two of them (`quiz`, `repo-graphql`) are over by 19 and 12 active lines.
- Asset-driven overage is large: `canvas`, `print`, `compiler`, `ui-render` are over mainly because of embedded `include_bytes!` payloads (about 5.9 MB combined). `compiler` becomes compliant after its five embedded fonts move out. `print` also fits after its 11 embedded assets move out.
- Per-crate `rust-source-boundary` declarations cannot loosen the gate. Admission takes `min(policy, declared)`. Three looser declarations (`os-kernel`, `plugin`, `ui`) are inert.

## Counting rules (what the gate actually counts)

Canonical implementation (verified by reading, not by running the gate):

1. Closure counter: `readRustSourceClosureV1(entries, root, cfg, read, owner?)` in `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts`. The caching library imports it through a relative path. Its admission sibling is `admitCargoSourceBoundariesV1` in `.../📥️source/🎟️admission/🟦️.ts`. Descriptor and unit-script callers use the same closure with the same limits.
2. Limits: policy `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json` → `storage.sourceBoundary` = `{maxSourceFiles 32, maxSourceBytes 262144, maxActiveLines 4000}`. Per unit, effective limit = `min(policy, unit.boundary)` where `unit.boundary` = `package.metadata.semio["rust-source-boundary"]` from the crate's `Cargo.toml`.
3. Unit scope: the gate runs per cargo compilation unit (lib, bin, build-script). Units whose manifest lies inside the repo root have provenance `workspace`, so both root members and in-repo path-only deps are gated. Out-of-repo path deps are `external-path` and not gated (none in this census). The closure of a unit starts at its target entry (`[lib] path`, `src/lib.rs`, bin roots, `build.rs`). Dependencies are their own units, so a crate compiles only if every reachable unit passes. The census computes that transitive condition directly.
4. Which files are followed: `mod x;` resolves to `x.rs` or `x/mod.rs`. `#[path = "..."]` resolves relative to the containing file's directory at file scope, and to the module directory otherwise. Inline `mod x { }` bodies recurse. `include!` counts as source. `include_str!` and `include_bytes!` count as embedded (see below). Generated inputs (macro expansion, `OUT_DIR`, generated includes) are refused unless the compiler owns them. Those refusals are the two crates marked REFUSED.
5. cfg selection happens before following mods. `#[cfg(test)]` and other cfg-false items are removed from the scanned text, so their `mod` targets are never read. Test directories (`tests/`, `benches/`, `examples/`) are not in the build graph and are not counted.
6. Counting:
   - `files` = distinct closure files, source and embedded together.
   - `bytes` = on-disk size of every closure file, including lines removed by cfg-selection and all embedded assets.
   - `activeLines` = for each source file, `selected.split("\n").length` of the cfg-selected text when it is not blank. Blank lines, comments and doc comments are counted, not stripped. Embedded files contribute 0 lines.
   - `overLimit` if files, bytes or activeLines exceeds the effective limit. Refusals count as over.
7. cfg snapshot: the gate uses the unit's `rustc --print cfg` plus `feature="..."` flags. The census approximates it with host cfg, `debug_assertions`, and the transitive default feature set. Workspace-wide feature unification can enable more modules, so exact per-unit numbers can differ. Rows where features are gated should be re-checked with the real graph.

Entry points:

- Canonical counter, runnable without cargo: `bun` importing `readRustSourceClosureV1` with the host cfg and a file reader. `🗑️generated/dump-closure.ts <entry> <pkgDir>` lists the closure members for one entry.
- Full census: `bun 🗑️generated/census.ts` (about 60 s; `ONLY=name1,name2` prints per-crate rows to stdout without writing outputs). `🗑️generated/analyze.ts` derives carving tiers from the summary.
- The gate itself (`admitCargoSelectionV1`) first captures the cargo unit graph and needs `rustc`, so it is not a standalone read-only report. I did not run it.

## Validation

- Ran the canonical closure on all 210 nodes (both lib and bin/build targets). No nodes were missing manifests.
- `semio-framework-pets`: canonical closure 28 files / 556,044 bytes / 11,324 lines. An independent regex resolver (no tokenizer, no cfg) found 52 files. The difference is exactly 24 `🧪️tests/🔬️unit/🦀️.rs` modules gated by `#[cfg(test)]`, so 52 - 24 = 28. A disk sum of the 28 canonical files is 556,044 bytes, an exact match.
- `semio-framework-compiler`: canonical 16 files = 11 source + 5 embedded fonts. Embedded fonts total 1,469,864 bytes and source totals 110,892 bytes, which sum to 1,580,756 exactly. The regex resolver found 19 files, of which 8 are test modules (19 - 8 = 11 sources). It omits `include_bytes!` fonts, which explains its lower byte count.
- These cross-checks confirm the file and byte semantics. They do not confirm the gate's cfg features. Treat counts near a limit as approximate until checked against the real unit graph.

## Snapshot drift

Two census runs about 30 minutes apart disagreed, while peer fleets were editing:

| Snapshot | Nodes | Members | Oversized | Closure-compilable | Tier 0 |
|---|---|---|---|---|---|
| A (first run, policy limits) | 191 | 87 | 34 | 99 | compiler, replication, pets, job, repo-graphql, quiz |
| B (final, effective limits) | 210 | 90 | 31 | 125 | compiler, replication, ui-contract, pets, quiz, repo-graphql |

Snapshot B is the final census. Between A and B, `semio-framework-job` was carved below the limits (A: 31 files, 5,581 lines; B: 23 files, 2,367 lines), and `ui-contract` moved from tier 1 to tier 0 as its dependencies were reduced. The 19 extra nodes and the drop from 34 to 31 oversized are consistent with active carving. Applying effective per-crate limits to snapshot A's scan left the oversized count at 34, because no declared tighter limit was exceeded.

Manifest mtimes in snapshot A: 121 of 191 `Cargo.toml` files changed on 2026-10-14, consistent with a bulk metadata edit (77 manifests now declare `rust-source-boundary`).

## Census summary

- Nodes: 210 (90 members, 120 path-only). Missing manifests: 0.
- Closure-compilable: 125 of 210 nodes; 33 of 90 members.
- Oversized: 31 (21 members, 10 path-only). Refused by the closure scanner: 2 (`os-kernel-db`, `os-renderer-wgpu`).
- Declarations: 77 crates declare `rust-source-boundary`.
  - Looser than policy and therefore inert: `os-kernel`, `plugin`, `ui` (128 files / 1,048,576 bytes / 12,000 lines). `os-kernel` and `plugin` are oversized only because the gate caps them at policy.
  - Tighter than policy and currently not exceeded: `artifact-reference-borrow`, `composition-admission`, `io-base64`, `os-identity`, `os-inference`, `os-store-blob-identity`, `os-store-link-schema`, `pack-source`, `plugin-component-receipt`, `replication-crypto`, `replication-error`.

## Top oversized crates (all 31, sorted by worst ratio)

Ratio is the worst of files/limit, bytes/limit, lines/limit. Binding is the dimension that exceeds. Emb = embedded files (bytes). Deps = oversized transitive first-party deps. Tier is the bottom-up carving tier (see next section).

| # | Crate | Kind | Files | Bytes | Active lines | Ratio | Binding | Emb (bytes) | Deps | Tier |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | semio-framework-plugin | member | 184 | 5,986,475 | 83,069 | 22.84 | bytes | 1 (1,694) | 7 | T4 |
| 2 | semio-framework-os-kernel | member | 180 | 3,676,554 | 54,848 | 14.03 | bytes | 33 (50,290) | 2 | T2 |
| 3 | semio-framework-os-infinite | member | 68 | 2,229,641 | 42,203 | 10.55 | lines | 0 | 9 | T4 |
| 4 | semio-framework-canvas | path | 318 | 1,992,623 | 5,619 | 9.94 | files | 311 (1,653,610) | 1 | T1 |
| 5 | semio-framework-os-mcp | member | 51 | 1,822,586 | 29,297 | 7.32 | lines | 5 (117,819) | 9 | T5 |
| 6 | semio-framework-print | member | 31 | 1,847,153 | 1,173 | 7.05 | bytes | 11 (1,719,946) | 8 | T5 |
| 7 | semio-framework-compiler | path | 16 | 1,580,756 | 2,411 | 6.03 | bytes | 5 (1,469,864) | 0 | T0 |
| 8 | semio-framework-ui-render | path | 28 | 1,461,562 | 7,544 | 5.58 | bytes | 15 (1,083,548) | 1 | T1 |
| 9 | semio-framework-os-flow | member | 37 | 1,328,222 | 19,306 | 5.07 | bytes | 1 (17,795) | 14 | T5 |
| 10 | semio-framework-replication | path | 57 | 1,039,702 | 16,470 | 4.12 | lines | 0 | 0 | T0 |
| 11 | semio-framework-plugin-host | member | 31 | 1,023,124 | 14,447 | 3.90 | bytes | 0 | 7 | T4 |
| 12 | semio-framework | path | 30 | 861,339 | 15,071 | 3.77 | lines | 6 (58,714) | 4 | T3 |
| 13 | semio-framework-ui-contract | path | 37 | 756,785 | 14,968 | 3.74 | lines | 3 (3,979) | 0 | T0 |
| 14 | semio-framework-pack | path | 33 | 804,976 | 11,174 | 3.07 | bytes | 0 | 1 | T1 |
| 15 | semio-framework-pets | member | 28 | 556,044 | 11,324 | 2.83 | lines | 0 | 0 | T0 |
| 16 | semio-framework-ui-scene | path | 14 | 481,386 | 10,181 | 2.55 | lines | 0 | 3 | T2 |
| 17 | semio-framework-repo-dashboard | member | 35 | 619,524 | 10,096 | 2.52 | lines | 4 (10,634) | 4 | T3 |
| 18 | semio-framework-ui-runtime | path | 16 | 366,728 | 6,304 | 1.58 | lines | 0 | 4 | T3 |
| 19 | semio-framework-surface | path | 19 | 291,172 | 5,977 | 1.49 | lines | 0 | 4 | T2 |
| 20 | semio-framework-server | member | 8 | 280,385 | 5,752 | 1.44 | lines | 0 | 1 | T1 |
| 21 | semio-framework-os | member | 14 | 306,233 | 5,486 | 1.37 | lines | 1 (2,551) | 10 | T5 |
| 22 | semio-framework-os-kernel-neural-engine | member | 18 | 350,084 | 4,420 | 1.34 | bytes | 0 | 2 | T2 |
| 23 | semio-framework-plugin-host-test-component | member | 41 | 52,308 | 626 | 1.28 | files | 20 (2,742) | 8 | T5 |
| 24 | semio-framework-artifact-flow-flow | member | 39 | 321,804 | 4,971 | 1.24 | lines | 1 (7,611) | 9 | T4 |
| 25 | semio-framework-artifact-infinite-dag | member | 36 | 150,611 | 2,417 | 1.13 | files | 2 (9,979) | 3 | T3 |
| 26 | semio-framework-repo-cli | member | 9 | 218,701 | 4,362 | 1.09 | lines | 0 | 1 | T1 |
| 27 | semio-framework-artifact-workflow-workflow | member | 34 | 224,869 | 3,051 | 1.06 | files | 1 (6,857) | 6 | T4 |
| 28 | semio-framework-quiz | member | 12 | 190,352 | 4,019 | 1.005 | lines | 0 | 0 | T0 |
| 29 | semio-framework-repo-graphql | member | 7 | 179,150 | 4,012 | 1.003 | lines | 0 | 0 | T0 |
| 30 | semio-framework-os-kernel-db | member | 0 | refused | refused | - | refused | - | 3 | T3 |
| 31 | semio-framework-os-renderer-wgpu | member | 0 | refused | refused | - | refused | - | 17 | T6 |

Refusal reasons: `os-kernel-db` reports `Unsupported Rust compile module body`. `os-renderer-wgpu` reports `Generated Rust input requires compiler ownership` (a generated `🦀️.rs` input via include).

Embedded assets dominate four crates. `canvas` is 311 of 318 files embedded (1.65 MB), and still 5,619 lines over the 4,000-line cap after assets move. `print` is 1.72 MB embedded; without them it is 20 files, 127,207 bytes and 1,173 lines, which fits. `compiler` without fonts is 11 files, 110,892 bytes and 2,411 lines, which fits. `ui-render` without assets is 13 files, 378,014 bytes and 7,544 lines, still over on lines.

## Carvable leaves, bottom-up

Tier 0 is unlocked now. Each crate has no oversized transitive first-party dep, so it can be carved independently:

1. `semio-framework-compiler` (path) - 16 files, 1.58 MB, 2,411 lines. Move the five embedded fonts out (1,469,864 bytes), which makes it compliant with no code split. Quiet: no source change since 2026-10-08, no recent notes.
2. `semio-framework-pets` (member) - 28 files, 556,044 bytes, 11,324 lines, no first-party deps. Its 28 files come through `#[path = "../../🔨️modules/..."]` into a shared modules tree. Needs a line split. Quiet since 2026-10-14.
3. `semio-framework-quiz` (member) - 12 files, 4,019 lines. Trim 19 active lines (blank and comment lines count). Quiet since 2026-10-14.
4. `semio-framework-repo-graphql` (member) - 7 files, 4,012 lines. Trim 12 active lines. Active: 8 source files changed, newest 2026-10-15 04:08.
5. `semio-framework-replication` (path) - 57 files, 16,470 lines. Active: source changed 2026-10-15 09:31 today.
6. `semio-framework-ui-contract` (path) - 37 files, 14,968 lines. Active: source changed 2026-10-15 05:21 today.

Next tiers (each unlocks when everything below it is compliant):

- T1 (5): `canvas` (assets, then lines), `ui-render` (assets, then lines), `pack`, `server`, `repo-cli`.
- T2 (4): `os-kernel` (180 files, 54,848 lines), `ui-scene`, `surface`, `os-kernel-neural-engine`.
- T3 (5): `semio-framework` (15,071 lines), `repo-dashboard`, `ui-runtime`, `artifact-infinite-dag` (36 files), `os-kernel-db` (refused).
- T4 (5): `plugin`, `os-infinite`, `plugin-host`, `artifact-flow-flow`, `artifact-workflow-workflow`.
- T5 (5): `os-mcp`, `print`, `os-flow`, `os`, `plugin-host-test-component`.
- T6 (1): `os-renderer-wgpu` (refused).

Tier order means "safe to carve now" for T0 only. Higher tiers depend on lower ones via transitive closure.

## Peer-activity flags

Notes = recent ticket markdown (mtime within 3 days) naming the crate across `MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES`, `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT` and `🪓️monolith-split`. Changed = Cargo.toml or `.rs` files modified within 2 days. Newest = latest source mtime.

Actively carved (touched today, or many recent notes):

- `os-kernel` - 9 notes (e.g. `rust-writer-ingress-current.md`, `rust-crate-cache-hard-ceiling-audit.md`, `DEPENDENCY-DIRECTION-AUDIT.md`, `PRODUCER-CRATE-RECIPE.md`); sources changed 08:02 today.
- `plugin` (7 notes, 08:02 today), `plugin-host` (6 notes, 08:02), `os-infinite` (4 notes, 08:02). Notes include `RESULT-PLUGIN-INSTANCE-HOST-SPLIT.md` and `SEAM-MAP-PLUGIN-MANIFEST-HUB.md`.
- `os-renderer-wgpu` (7 notes; `RESULT-WGPU-SCENES-SPLIT.md`, `RESULT-WGPU-RENDERER-SPLIT.md`, `RESULT-WGPU-SHELL-SPLIT.md`), sources 08:02 today. Refused, so the fleet is likely mid-split.
- `replication` (tier 0, 3 notes: `rust-writer-ingress-current.md`, `actor-growth-audit-current.md`, `ui-value-owner-current.md`), sources changed 09:31 today.
- `repo-graphql` (tier 0, `RESULT-REPO-CLI-GRAPHQL.md`, `SEAM-MAP-HUB-REPO-SURFACE.md`), 8 source changes, newest 04:08 today.
- `ui-contract` (tier 0, `ui-value-owner-current.md`), sources 05:21 today.
- `pack`, `repo-cli`, `os-flow`, `surface`, `os-kernel-db` and `semio-framework` show source or note activity today or within the last 2 days.

Quiet (no recent notes; last source change older than 2 days): `compiler` (2026-10-08), `ui-render` (2026-10-08), `ui-scene` (2026-10-09), `print` (10-13), `server` (10-13), `pets` (10-14), `quiz` (10-14), `ui-runtime` (10-14), `repo-dashboard` (10-14).

Interpretation: the fleets are carving the core (`os-kernel`, `plugin*`, `wgpu`, `replication`) right now. The quick wins (`quiz`, `repo-graphql`) are in the 19-line and 12-line range, and nobody has recently touched `compiler` or `ui-render`, so they are unclaimed. `compiler` and `print` are asset moves, not code splits.

## Caveats

- Snapshot drift during the run (see above). Counts for members that are being edited may already differ.
- cfg features are approximated (default features, transitive). Crates whose closure differs under workspace feature unification need the real unit graph to confirm.
- Refusals (`os-kernel-db`, `os-renderer-wgpu`) follow the scanner's current rules; I did not run the gate to confirm the refusal path.
- Byte and file semantics were cross-checked on two crates only (pets, compiler). Lines were not cross-checked against an independent counter.
- Peer activity is inferred from note mentions and mtimes, not from fleet claims or locks.
