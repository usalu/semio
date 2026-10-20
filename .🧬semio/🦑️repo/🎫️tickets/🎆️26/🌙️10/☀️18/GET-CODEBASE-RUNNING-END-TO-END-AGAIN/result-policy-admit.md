# Result policy-admit (executor policy-admit, 2026-10-20)

Option 1 of `result-manifest-batch2.md` 5.1, staged only. Nothing outside C was written. Apply after the sync / when the freeze lifts.

## 1. Design (schema-first, derived from data)

`taxonomy.dependencyDirections.publicApi` gains `sdkFoundationPackages`: the declared list of neutral packages that carry the keys once published by `@semio-tech/framework`. The construction (`SDK_FOUNDATION_TARGETS`) turns the authored export targets of those workspace packages into allow-patterns and adds them to
- `plugins-framework-sdk-only` `to.pathNot` (next to the SDK package targets),
- `renderer-hosts-only-ui` allowlist (`RENDERER_HOST_ALLOWED_RESOLVED_PATHS`).

Only authored public targets are admitted (private files of the same packages stay forbidden); both resolved-path and `node_modules/<name>` alias forms are covered by the existing `exportTargetPatterns`. No glue, no manifest change, no rule renamed.

Declared list (8): `framework-hash`, `framework-interaction`, `framework-manifest`, `framework-trace`, `ui`, `ui-contract`, `ui-react`, `ui-styling` (all `@semio-tech/`). Chosen from the 58 violating edges (plugins: ui-react 20, hash 6, manifest 5, ui/scene 2, ui-styling, ui-react/build-tooling, ui-contract/testing/color-input, interaction; hosts: trace 9, manifest 9, interaction 3) plus the sibling carved foundation owners named in the brief.

Validation (closed, fail-fast): the `publicApi` key set is now `rulePrefix|sdkFoundationPackages|sdkPackage|toolingRoles|version`; the list must be a non-empty array of unique `@scope/name` strings that are not the SDK package. Names are not required to exist in the `workspacePackages` input (many tests build with partial package sets, same as `sdkPackage`); existence is enforced by the new test against the real workspace.

## 2. Staged files (`C/policy-admit-tools/`)

Flat names because the repo paths exceed the ticket path budget; `plan.json` maps each to its target (target paths are at most 145 chars).

| staged | target (under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`) | op |
| --- | --- | --- |
| `staged/10-taxonomy.json` | `🔣️taxonomy.json` (+`sdkFoundationPackages`, 10 lines) | edit |
| `staged/20-schema.json` | `🧬️schema/🧱️dependency-direction/🔣️.json` (`publicApi` required + property) | edit |
| `staged/30-construction.cjs` | `🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs` (key set, validation line, `SDK_FOUNDATION_TARGETS`, two allowlists, comments) | edit |
| `staged/40-bootstrap.ts` | `🕸️dependencies/🧭️direction/🚀️bootstrap/🟦️.ts` (taxonomy type) | edit |
| `staged/60-sdk-foundation-admission.fixture.json` | `🕸️dependencies/🧭️direction/🏗️construction/🧪️tests/🔑️sdk-foundation-admission/🧫️fixtures/🔣️.json` | create |
| `staged/61-sdk-foundation-admission.test.ts` | `…/🧪️tests/🔑️sdk-foundation-admission/🟦️.ts` | create |

Tools: `build.ts` (re-derives staged copies + `plan.json` from the live tree via anchored edits), `apply.sh` / `apply.ts` (anchored apply, `--dry-run`), `dump-policy.ts`, `verify-staged.ts`, `verify-ruleset.ts`.

`apply.sh [--dry-run]`: verifies every target first, writes nothing unless all pass. Live hash equals the staged base: replace with the staged copy. Hash drifted: patch in place only if every edit anchor still occurs exactly once; otherwise `REFUSED` (exit 1, nothing written). Create targets refuse when present with different content; already-applied targets are skipped (idempotent). `SEMIO_ROOT` overrides the root (used for the overlay checks).

The language-agnostic test: portable corpus (17 cases: synthetic packages + `{from,to,forbidden}` rows for both rules, each outcome per rule) evaluated directly and against dependency-cruiser's `validateDependency` (third-party oracle); a bare declaration naming no corpus package forbids every foundation case (proves the admission comes from data); closedness errors; real taxonomy validates against the schema (Ajv 2020) and names only existing workspace packages.

## 3. Verification (all from `/Users/ueli/Documents/semio`)

1. Anchors on the live tree: `C/policy-admit-tools/apply.sh --dry-run` -> `dry run clean: 6 writes` (4 `replace (hash equals base)`, 2 `create`).
2. Apply mechanics on an overlay COPY (cp of the 4 live files into `C/🗑️generated/policy-admit-overlay`, no symlinks): `SEMIO_ROOT=<overlay> apply.sh` -> `applied 6 writes`; second run -> 6 skips, `applied 0 writes`; applied files `cmp`-identical to staged. Drift: appended newline to taxonomy -> `would patch (drifted, anchors intact)`; broken anchor in the type file -> `REFUSED bootstrap-type: drift, anchor missing or ambiguous`, rc=1.
3. Policy dumps: `bun policy-admit-tools/dump-policy.ts <out> [--overlay <dir>]` (live loader for workspace authority; overlay rebuilds with the overlay taxonomy + construction). Only `plugins-framework-sdk-only` (pathNot 530 -> 1216) and `renderer-hosts-only-ui` (236 -> 922) differ among 1652 rules; options identical.
4. Scan: `bun manifest-batch2-tools/scan.ts <policy> <out.tsv> [--refresh]` (16,682 files, live tree, 14 unresolved `@semio-tech`, all generated outputs, as before).

| rule | live policy | overlay policy |
| --- | ---: | ---: |
| `plugins-framework-sdk-only` | 37 | **0** |
| `renderer-hosts-only-ui` | 21 | **0** |
| `no-core-path` | 51 | 51 |
| `no-cross-technology-✏️s-to-🧰️framework` / `-🌎️hub-to-🧰️framework` | 16 / 3 | 16 / 3 |
| `framework-no-products` | 8 | 8 |
| `plugin-no-extension-or-artifact` (🖨️raster / 🧩️puzzle) | 3 / 1 | 3 / 1 |

   Row-level diff of the two TSVs: 58 lines removed (exactly the two rules), 0 added.
5. `bun policy-admit-tools/verify-staged.ts <overlay>` (stand-alone mirror of the staged test, includes the dependency-cruiser oracle, closedness, Ajv schema validity, schema rejection of empty/unscoped/missing list, workspace existence of the 8 names against the live `loadDependencyDirectionPolicy`): `cases 17, failures 0`.
6. `bun policy-admit-tools/verify-ruleset.ts <policy.json>` (dependency-cruiser's own rule-set validation incl. regex safety) on live and overlay dumps: both `rule set valid: 1652 rules` (7.1 s / 11.4 s).
7. `node --check` on the staged `.cjs`; `bun build --no-bundle` on the staged test: transpiles.

Not run (per brief, wait for "PAUSE LIFTED"): any bun test suite, including the new `🔑️sdk-foundation-admission` test, `🏗️construction/🧪️tests/*`, `🧪️tests/🧱️dependency-direction`, `🚀️bootstrap/🧪️tests`, the `🧾️selection` test; the dependency-cruiser cruise; tsc on the staged test (its relative imports only resolve in place).

## 4. Open items

- Apply with `C/policy-admit-tools/apply.sh` once library edits are allowed (edits `📚️library/**` and `🔣️taxonomy.json`, which the dependency sync loads: apply before the next sync start or accept one restart). Run `build.ts` first if the live files changed (the plan is hash-pinned; apply also tolerates drift when anchors hold).
- After applying: run the suites listed under "not run" (the new test also needs `ajv`, `dependency-cruiser` from root node_modules, both present today); then the real cruise.
- New test directory name `🔑️sdk-foundation-admission` must pass the taxonomy filename/emoji admission laws and artifact-name registry gate (not run); rename the emoji if the registry rejects it (single directory, referenced only by its own files).
- Any hash-keyed artifact of the taxonomy or schema (nx input hashes, generated catalogues) refreshes after apply.
- Admitting `ui` and `ui-contract` gives plugins their full authored export tables (168 and 109 keys) not just the keys observed in use; narrow by trimming `sdkFoundationPackages` (data only) if that is too wide.
- `🗑️generated/policy-admit-*` (overlay copy, dumps, scan TSVs, about 5 MB) can be deleted with the folder.
