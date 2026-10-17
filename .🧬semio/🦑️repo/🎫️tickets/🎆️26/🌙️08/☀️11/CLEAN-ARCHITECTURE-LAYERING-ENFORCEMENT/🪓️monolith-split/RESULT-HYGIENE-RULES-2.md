# Result: Hygiene Rules 2 (slice HYGIENE-RULES-2, prefix `hr2`)

Status: COMPLETE for what is unblocked (2026-10-16 16:20 to 18:30). Rename applied: only for the five folders listed in section 3.1 (stdio `base` reverted).
Folder F = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/hr2-tools/`. Logs: `F/🗑️generated/hr2-*`.

## 0. Verdict

- `not-to-unlisted`: DONE. Production edges 146 to 0 (all 889 to 405; the rest are test edges). Neither `no-circular` nor the cross-package family rose.
- `no-core-path`: PARTIAL. Production edges 625 to 536 (all 992 to 883). Fixed: gltf `geometry-core` (29), block `shared` (19), procedural `core` (18), cad `core` (1), vscode `path-helpers` (3), and 26 `@/lib/*` alias imports. The stdio `🧱️base`/`✉️base` renames (493 edges, the bulk) are BLOCKED and fully reverted, see section 4. The rest are policy false positives or other slices (section 5).
- Incident: at about 17:50 my half-applied stdio rename made `loadTaxonomy` throw (the coordinator raised it). I reverted the taxonomy rows at once, chmod 644, then reverted both renames; `loadTaxonomy` loads again and the dangling-reference scan over 11,086 stdio files is identical to the baseline (312 before, 312 after, 0 added).

## 1. Rule semantics (read-only policy)

- `no-core-path` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs`, `noCorePathRule`): an edge whose RESOLVED target path has a segment that ENDS in one of `bannedNameStems` of `📚️library/🔣️taxonomy.json` (`core common util utils helper helpers misc shared base lib impl`). The regex is `(^|/)([^/]*?)(stems)(/|$)`, so it also hits `score`, `zlib`, `database`, and Node built-ins (`node:util`, `node:zlib` resolve to `util`, `zlib`). It applies from `✏️s/`, `🧰️framework/`, `🌎️hub/`, `♻️mit-bestand/`. There is no sanctioned entry alias: the only way out is a folder name without a banned stem.
- `not-to-unlisted`: dependency-cruiser's stock rule. The importer's NEAREST ancestor `package.json` must list the package. For importers outside a `📦️packages/<lang>` directory this is an area manifest (`✏️s/package.json`, `♻️mit-bestand/package.json`) or the repository root `package.json`.

## 2. Baseline (exact cruiser command, tree of 16:22 to 16:59)

`node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type err` (`F/hr2-tools/../🗑️generated/hr2-cruise.sh before`): exit 239 (13,551 violations, 17,686 modules, 49,689 dependencies). Raw output `hr2-before-err.txt`. Counts by `F/hr2-tools/counts.py` (production = no `/🧪️tests/` in source or target):

| Rule | all | production |
| --- | ---: | ---: |
| `no-core-path` | 992 | 625 |
| `not-to-unlisted` | 889 | 146 |
| `no-circular` | 584 | 499 |
| `no-cross-package-relative-*` | 8,592 | 4,739 |

Baseline of the other checks: bun tests of 66 files (`hr2-tests-before.tsv`; 12 files fail before, listed there), taxonomy inventories per plugin (`hr2-inv-*-before.json`), dangling-reference scan of 11,086 stdio files (312 dangling before, `hr2-dangling-stdio-before.json`).

## 3. What changed

### 3.1 Folder renames (tool `hr2-tools/rename.ts`, context-aware token rewrite, then directory rename in the same step)

| Old | New | Files rewritten | Check |
| --- | --- | ---: | --- |
| `✏️s/🔌️plugins/🧱️block/🧬️schema/🧱️shared` and `…/🧪️tests/🧫️fixtures/🧱️shared` | `🧱️document-fields` | 19 | 0 dangling refs |
| `✏️s/🔌️plugins/🌀️procedural/🫀️core` | `🫀️foundation` | 21 | " |
| `✏️s/🔌️plugins/📐️cad/⚙️engine/🫀️core` | `🫀️kernel` | 2 | " |
| gltf `…/💡️inferences/🔨️geometry-core` | `🔨️geometry-kernel` | 43 (incl. the `members-of-inferences` row in `📚️library/🔣️taxonomy.json`, one name) | " |
| vscode client `🧩️vscode/🎵️path-helpers` | `🎵️path-navigation` | 3 | " |

Foreign folders with the same names (space and flow `🫀️core`, db engine `🧱️base`, writer `…/🔖️1.4/🧱️base`, sequence editor `🧱️base`) were classified by path context and left alone (listed in `hr2-plan-plug2.json`/`hr2-plan-stdio.json` under `skipped`). Digest transplant (`transplant.ts`): the `schema-catalog.json` hash rows of rewritten files were recomputed; fresh-hash count 6,526 before and after.

### 3.2 `@/lib/*` aliases

26 imports in 10 files of `🖥️server/🎛️coordinator/**` now point at `🖥️server/📚️library/<module>/🟦️.ts` by relative path (no new cycle: `🔌️ports` imports only `next` and `pg-boss`). The `tsconfig` aliases stay (library fixtures describe them).

### 3.3 `not-to-unlisted`

Declared in the nearest manifest, versions from `🔒️dependencies.json`: root `package.json` (17 packages, production-runtime ones under `dependencies`), `✏️s/package.json` (6), `♻️mit-bestand/package.json` (1), `♻️mit-bestand/📋️bericht/…/package.json` (2), gismap, replication, os rust, os ts, print inferences (1 each). Six packages were not in the freeze baseline (oracle libraries `jszip`, `fast-xml-parser`, `fflate`, `mdast-util-to-markdown`, `crc-32`, `fast-deep-equal`); `bun freeze.ts` (the repo's own `dependencyFreezeCheck`) listed exactly those six as new, and I added their entries to `🔒️dependencies.json` (150 lines). All six lockfiles (root, `✏️s`, `🌎️hub`, `♻️mit-bestand`, `🏢️semio-tech`, `🎓️teaching`) were updated with the psc method (bun derives the lock in a scratch copy, `psc-tools/lock.ts` transplants it). `bun install --frozen-lockfile --dry-run` passes in all six scopes. The lock derive of 18:15 differs from the checked-in locks only by `@semio-tech/framework` rows that the psc2 codemod added to manifests (not mine).

## 4. Blocked: stdio `🧱️base` and `✉️base` (493 production edges)

The rename was applied (12 + 1 directories, 777 files, same-length names `🧱️full`, `✉️wrap`) and reverted. Reason: `validateTaxonomy` (`📚️library/🔍️discovery/🟦️.ts:5382`) requires each `subsetDirectoryOverrides` value to be `<one semantic emoji><subset id>`, so a subset directory can only be renamed together with its logical subset id. The id `base` is used about 8,900 times in the stdio subtrees (Rust module paths `subsets::base::` about 3,000, schema ids `stdio/<artifact>/<standard>/base/…`, `"subset": "base"` rows, oracle ids, crate names). Renaming ids needs the Rust compile that the launcher currently refuses. Options for the parent: (a) a campaign that renames the id `base` to `full` (and `semio` `base` to e.g. `wrap`) in manifests, rosters, Rust and TS, verified by a real `cargo check`; or (b) relax the validator so the directory name may differ from the id (the roster already maps ids to directories). Prepared and reusable: `hr2-tools/rename.ts` plus `spec-stdio.json` (apply) and `spec-stdio-revert.json`. Name decision: `full` (SVG already calls its unrestricted profile "Full"), `✉️wrap` for semio (4 letters keep the longest path at 240 bytes). Also found: 2 stale references in the semio `🌊️flow` and `🏛️model` oracle JSONs pointed at a non-existent `🧰️framework/…/🧬️semio/✉️base/` and now point at the stdio subset (kept). Pre-existing: one tracked orphan file under the corrupted folder `🗟️artifacts/🧰️xml/…` (not referenced).

## 5. Hand-offs (not fixable by me)

| Edges (prod) | Cause | Owner and suggested fix |
| ---: | --- | --- |
| 36 | Node built-ins `node:util`, `node:zlib` resolve to `util`, `zlib` | policy: add `dependencyTypesNot: ["core"]` (or a word-boundary stem match) to `noCorePathRule` |
| 11 | renderer `🕸️NodeGraph/🧰️graph-core` | rhu2 (renderer TS) |
| 7 | library `🔬️interactivity-audit/🗄️database` ("database" ends in "base") | policy false positive (stem needs a word boundary) |
| 6 | os flow `🫀️core` (generated wasm bindings, workspace entries in 6 lockfiles, project.json, caching) | os flow owner |
| 2 | `…-symmetry-score` ("score" ends in "core") | policy false positive |
| 493 | stdio `base` | section 4 |
| psc2 | its facade plan mirrors module paths including `🧱️base`, creating new `no-core-path` edges (facade path is the resolved target) | name the facade directories without the banned stem |

## 6. After measurement (same cruiser command, 18:03 to 18:25, exit 97)

`hr2-after-err.txt`: 10,593 violations, 17,896 modules, 50,509 dependencies (before 13,551; the other drift is psc2 and peers).

| Rule | all before | prod before | all after | prod after |
| --- | ---: | ---: | ---: | ---: |
| `no-core-path` | 992 | 625 | 883 | 536 |
| `not-to-unlisted` | 889 | 146 | 405 | 0 |
| `no-circular` | 584 | 499 | 574 | 491 |
| `no-cross-package-relative-*` | 8,592 | 4,739 | 6,430 | 3,580 |

`ui-no-framework-packages` 262 to 270, `framework-no-products` 154 to 166, `plugins-framework-sdk-only` 619 to 626: none of the added edges involves a file or folder I touched (one `framework-no-products` edge comes from a facade file under `📦️packages/🟦️typescript/🛍️products/…` that is not mine).

## 7. Verification

| Check | Result |
| --- | --- |
| bun tests, 18 files incl. the library dependency-policy tests, before and after the renames | identical exit, pass and fail for all 18 (`hr2-tests-before2.tsv`, `hr2-tests-after2.tsv`) |
| bun tests, 66 stdio-related files before (baseline, 12 files failing already) | stdio untouched at the end; not re-run |
| `tsc --noEmit` on 80 touched TS files of the renamed families (tools `mktsc.ts`, `tscdiff.ts`) | before 2 errors, after 0, 0 added |
| `tsc` on 668 stdio TS files | baseline 58 lines (`hr2-tsc-s1-before.txt`); stdio reverted, not re-run |
| Taxonomy inventory (`inv.ts`, real `inventoryTaxonomy`) for stdio, procedural, cad, block | violation counts identical before and after, 0 new `directory-kind-unresolved`/`ambiguous` (the one existing unresolved is the orphan `🗟️artifacts`) |
| `loadTaxonomy` (`loadtax.ts`) | ok after the revert; file mode 644 |
| Dangling references (`dangling.ts`): stdio 11,086 files; renamed families 88 files | stdio 312 = 312, 0 added; families: 0 dangling reference contains an old or new name |
| `bun install --frozen-lockfile --dry-run`, six scopes | pass |
| `taxonomy checker F/tx3-tools/` | does not exist in F (also no `tx2-tools`); I used the real `inventoryTaxonomy` instead |
| Real cargo check | not run (no Rust file was moved or edited by the final state except path strings in the five families; the launcher refuses compiles) |

## 8. Files and tools

Edited outside the renamed folders: `📚️library/🔣️taxonomy.json` (one memberNames row, parent area, hash-guarded in effect: before/after diff is that row plus a peer's `session-playthrough`), `📚️library/🔣️schema-catalog.json` (paths and hashes), `📓️schema-catalog.md`, `🗂️schema-document-index.json`, `📋️build-owners.json`, `🔒️dependencies.json`, root and area `package.json` and lock files, `.vscode/launch.json` and seed, Cargo manifests carrying the paths. Backups of the manifests and locks before my edits: `🗑️generated/hr2-backup/`. Tools in `hr2-tools/`: `rename.ts`, `transplant.ts`, `deps.ts`, `lockderive.sh`, `wsmanifests.ts`, `freeze.ts`, `unlisted.ts`, `counts.py`, `dangling.ts`, `inv.ts`, `invall.sh`, `loadtax.ts`, `mktsc.ts`, `tscdiff.ts`, `tsc.sh`, `runtests.sh`, `catalog-hash.ts`, specs `spec-*.json`. Notice for psc2: `HR2-NOTICE-FOR-PSC2.md`.
