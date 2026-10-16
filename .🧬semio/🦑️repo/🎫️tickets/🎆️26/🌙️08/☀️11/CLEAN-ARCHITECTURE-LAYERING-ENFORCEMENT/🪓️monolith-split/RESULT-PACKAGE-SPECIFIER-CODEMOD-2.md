# Result: Package Specifier Codemod 2 (slice PACKAGE-SPECIFIER-CODEMOD-2, prefix `psc2`)

Status: IN PROGRESS (updated 17:50). Tools: `psc2-tools/` (this folder). Logs and raw outputs: `🗑️generated/psc2-*`.

## 1. Verdict (current)

- Baseline measured with the exact cruiser command (section 3): `no-cross-package-relative-*` family 8,429 error lines (4,849 outside `🧪️tests`), 6,795 distinct edges (4,106 outside tests). `not-to-unlisted` 890/142, `no-circular` 574/472, total 13,375.
- Batch 1 (norm contract) applied: 691 importers, 2 facades, 15 manifests (details in section 5).
- Remaining batches, skips and the final before/after table follow as they complete.

## 2. Method and tools

Generalised from `RESULT-PACKAGE-SPECIFIER-CODEMOD.md`. The three psc tools were fixed to three owners; the psc2 tools take every workspace package from the live policy snapshot.

- `dump-policy.cjs`: loads the dependency policy snapshot (`loadDependencyDirectionPolicy`) and dumps the workspace packages and all 272 `no-cross-package-relative-*` rules (their exact regexes).
- `model.ts` / `scan.ts`: TypeScript-AST scan of 12.9k source files; an edge is flagged when the importer is outside the owner's owned region (`from.pathNot`) and the resolved target matches the rule's `to.path` and not `to.pathNot`. The scan finds 4,183 production edges against 4,106 distinct cruiser edges (the cruiser also folds import lines of one statement and mojibake aliases).
- `plan.ts`: per target file decides the public entry. (1) reuse an existing exported facade that already covers the names (checked with the TypeScript checker by symbol identity, never the package root barrel); (2) otherwise add the missing names to the closest existing facade; (3) a real module inside the package directory is exported directly; (4) otherwise a new facade at the mirrored path inside the TypeScript package directory (paths over 236 bytes drop leading segments; a key already used by a built `dist` entry gets a `📥️source` prefix). Names are the union of what importers use, value versus type from the checker; default imports and JSON default imports are supported.
- `apply.ts`: writes facades (merging into existing files), the explicit `exports` entries, and new package manifests; snapshots every file it changes.
- `rewrite.ts`: re-scans each importer right before editing and patches only the module-specifier literal (JSON default imports become named imports); bootstrap entries (`📜️script.ts`, Vite builders, vitest configs) get a relative path to the facade; adds `workspace:*` to the importer's owning package (`dependencies` for production, `devDependencies` for test-only).
- `link.ts`: symlinks planned packages into the root `node_modules`, as `bun install` would.
- Verification tools: `mktsc.ts`, `tsc.sh`, `tscdiff.ts`, `picktests.ts`, `runtests.ts`, `testdiff.ts`, `facade-load.ts`, `vite-check.mjs`, `spec-check.ts`, `ruletab.ts`, `errdiff.ts`, `cruise.sh`.

Skipped by rule (importer or target in these areas, see section 7): `rhu2` renderer TS, `🌎️hub/📇️directory`, `pi1` (`🔌️plugin/**`), `sf3` (`🌀️procedural`), `📚️library` (parent-held).

## 3. Baseline (exact cruiser command)

Command (audit 2 section 1, `psc2-tools/cruise.sh before full`): `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type err`

- First attempt (16:22) ran at nice +5 (zsh `BG_NICE`) and was starved; a restart at 16:48 was killed with SIGTERM at 17:19 (by something outside this slice) after 30 minutes. The third run (17:19 to 17:37, 1,073 s, nice 0) completed: exit 63 (13,375 errors mod 256), 17,804 modules, 50,168 dependencies. Raw: `🗑️generated/psc2-before-err.txt`.

| Rule | all / outside `🧪️tests` |
| --- | ---: |
| `no-cross-package-relative-*` (all 272 rules) lines | 8,429 / 4,849 |
| `no-cross-package-relative-*` distinct edges | 6,795 / 4,106 |
| `not-to-unlisted` | 890 / 142 |
| `no-circular` | 574 / 472 |
| `ui-no-framework-packages` | 271 / 216 |
| total error lines | 13,375 |

Largest target packages by production lines: `repo-lib` 1,020 (library, parent), `process` 695, `norm-js` 691, `framework-rs` 377 and `framework` 376 (the SDK, sibling rules), `framework-os` and `framework-os-kernel` 187, `semio-s-workspace` 135, `schema-validator` 102, `plugin-registry` 102.

## 4. Batches

(filled in per batch)
