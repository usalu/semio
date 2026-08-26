# Shared Action Runtime and Discovery Status

## Outcome

The 12 shared framework routes are production-runtime accepted. The external harness joins the exact registered route, exact concrete factory type, and typed dispatch for every route, then proves exact maximum admission, maximum-plus-one owner handback, cancel-before, cancel-after, same-owner ABA refusal, preview/checkpoint interruption and resume, `applied_progress = 1`, completion, revision/generation freshness, bounded close, and repeated terminal close. All 36 dispatches passed.

The descriptor source-of-truth is also implemented. `Plugin::register_app_factory` applies one exact schema-driven disposition table to both the manifest `app.definition` and the definition retained by the runtime factory. The join visits only `window_kinds[].actions`; app commands, mode commands, near IDs, and unrelated local actions cannot inherit migration. The language-neutral fixture covers every accepted ID in two distinct owner placements and hostile local/near-ID owners, and the Ajv 2020 oracle validates the fixture schema.

Generated descriptors remain unchanged because the required 33-target `bun nx ... describe` batch reached a fresh unrelated FEM compile blocker after the repaired stdio dependency compiled. `@semio-tech/fem-plugin:describe` reports 141 errors and 137 warnings across E0026/E0053/E0277/E0308/E0425/E0599/E0608/E0609. No descriptor output was emitted before the batch was stopped, so this packet does not claim generated-disposition acceptance.

The apparent 934-to-1,488 count shift is a denominator change, not topology growth. The current descriptor topology has 1,488 total placements of the 12 IDs: 934 missing and 554 already migrated by earlier plugin-local cohorts. The 554 consists of 56 migrated placements for each of `copy`, `cut`, `paste`, `noteShellCommand`, and `setHistoryCommandFilter`, 58 for `recordTutorial`, and 36 for each of the six selection/interaction IDs. The earlier 934 census counted only missing placements; the later 1,488 inventory counted every owner placement.

## Infrastructure repairs

- Taxonomy discovery now discriminates exact descendant bundles from catalog descendants, so `exclusiveAlternatives` is required and read only for the exact variant. The shipped taxonomy, a hostile missing-exact-alternatives mutation, and a hostile wrong-catalog-contract mutation pass their exact regression.
- Nx isolated plugin-worker IPC was the source of the malformed `📦��packages` root. Repository Nx launchers now force in-process plugin discovery even against a hostile caller setting. The real package and workspace membership remain canonical; no path exception, rename, or duplicate project was introduced.
- Interactivity runtime discovery now scans only authored product roots (`✏️s`, `🧰️framework`, `🌎️hub`). The repository workspace scanner already defines `temp` as scratch. Hostile `temp`, `compose`, archive, and ticket paths are rejected without touching or allowlisting the scratch files.

## Exact verification

- `bun test …/🧪️index.test.ts --test-name-pattern 'Nx Unicode project transport|discriminates catalog descendants'`: exit 0, 3 pass, 226 filtered, 0 fail.
- `bun nx reset && bun nx show projects --with-target describe`: exit 0, 34 targets.
- external Cargo runtime harness: exit 0, 12 routes, 36 accepted dispatches.
- `cargo check -p semio-framework-plugin --lib`: exit 0.
- Ajv 2020 fixture validation: exit 0, 12 routes, 6 descriptor laws, 12 hostile runtime laws.
- `bun ./📜️script.ts verify interactivity tool-jobs --shared-action-fixture-only`: exit 0 after the final source join.
- `bun nx run-many -t describe --all --exclude=@semio-tech/os-plugin-describe-rs --skip-nx-cache`: blocked at `@semio-tech/fem-plugin:describe`; 141 errors, 137 warnings; zero descriptor outputs.
- final `bun ./📜️script.ts verify interactivity apps --actions`: exit 1, 2,781 disposition failures plus 1,011 production-join failures; 138 accepted commands and 20 accepted shared routes. The owned descriptor subset remains 934 missing plus 554 migrated because regeneration emitted nothing.
- final `bun ./📜️script.ts verify interactivity --self-test`: exit 0, deny clean, no thread-pool finding.

## Evidence

- `🧪️sol-shared-action-runtime-harness-resume-2026-08-26.txt`
- `🧪️sol-shared-action-infrastructure-regressions-2026-08-26.txt`
- `🧪️sol-shared-action-all-app-gate-before-describe-2026-08-26.txt`
- `🧪️sol-shared-action-describe-regeneration-blocked-fem-2026-08-26.txt`
- `🧪️sol-shared-action-all-app-gate-after-source-join-2026-08-26.txt`
- `🧪️sol-shared-action-full-interactivity-final-2026-08-26.txt`
- `🧪️sol-shared-framework-action-routes-owned-2026-08-26.txt`
- `🧪️sol-shared-framework-action-routes-ajv-oracle-2026-08-26.txt`
