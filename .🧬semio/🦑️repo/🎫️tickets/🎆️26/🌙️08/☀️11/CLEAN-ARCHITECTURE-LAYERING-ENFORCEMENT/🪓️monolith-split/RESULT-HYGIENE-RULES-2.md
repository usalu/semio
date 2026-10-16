# Result: Hygiene Rules 2 (slice HYGIENE-RULES-2, prefix `hr2`)

Status: IN PROGRESS (started 2026-10-16 16:20). This file is rewritten after each milestone.
Folder F = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/hr2-tools/`. Logs: `F/🗑️generated/hr2-*`.

## 0. Verdict

Baseline measured, design done, no source edit applied yet (the apply waits for the psc2 baseline cruiser run to finish, so that the rename does not corrupt its measurement).

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

## 3. Design

### 3.1 `no-core-path`, 625 production edges

| Target folder | Edges | Action |
| --- | ---: | --- |
| stdio `🪆️subsets/🧱️base` (12 directories) | 466 | rename to `🧱️full` |
| stdio semio `🪆️subsets/✉️base` | 27 | rename to `✉️wrap` (4 letters, so the longest path stays at 240 bytes) |
| gltf `💡️inferences/🔨️geometry-core` | 29 | rename to `🔨️geometry-kernel` |
| block `🧬️schema/🧱️shared` | 19 | rename to `🧱️document-fields` (+ the fixtures twin) |
| procedural `🫀️core` | 18 | rename to `🫀️foundation` |
| cad `⚙️engine/🫀️core` | 1 | rename to `🫀️kernel` |
| vscode client `🎵️path-helpers` | 3 | rename to `🎵️path-navigation` |
| `@/lib/*` alias in `🖥️server/🎛️coordinator` | 26 | import the library modules by relative path |
| Node built-ins `util`, `zlib` | 37 | not fixable in code: policy false positive (hand-off) |
| renderer `🕸️NodeGraph/🧰️graph-core` | 11 | rhu2 area (hand-off) |
| library `🔬️interactivity-audit/🗄️database` ("database" ends in "base") | 7 | policy false positive, parent area (hand-off) |
| flow `🫀️core` | 6 | os flow crate with generated wasm bindings and lock entries in 6 lockfiles: left (hand-off) |
| `…-symmetry-score` ("score" ends in "core") | 2 | inference ids: policy false positive (hand-off) |

The subset id stays `base` (JSON `subsets.base`, Rust namespaces, crate names such as `…-xlsx-base-model`): the taxonomy separates the logical id from the physical directory (`subsetDirectoryOverrides`), and the rule is about paths.

### 3.2 `not-to-unlisted`, 146 production edges

Declare in the nearest manifest, with the version of the existing declaration in `🔒️dependencies.json`. Seven packages are not in that baseline and are oracle libraries: `jszip`, `fast-xml-parser`, `fflate`, `mdast-util-to-markdown` (stdio fixture generators and probes), `crc-32`, `fast-deep-equal` (verification scripts). They get declared as devDependencies and added to the freeze baseline.
