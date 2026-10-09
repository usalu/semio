# Native Nextest Policy Recovery

The current PNG, BMP, JPEG and GIS owner checks reached preparation after Cargo discovery was corrected, then failed before compilation because policy assumed each isolated artifact Cargo workspace had its own `.config/nextest.toml`. Native profiles are authored at physical product and repository ancestors.

The new language-neutral fixture exercises root, product, artifact, sibling, absent and escaping configuration ownership. The implementation selects the nearest configured physical ancestor and validates the retained native path using the existing symlink and repository-boundary checks. It never borrows a sibling configuration or invents a profile. Current package policy also uses this selection directly.

Observed RED: registered Nx target ran one test and failed on the missing artifact-local profile. Observed GREEN: the repaired selection ran one test with 22 assertions, independently parsed with `@iarna/toml`, emitted its runtime `[DEBUG]` witness, and completed Nx in 662 ms. The fixture now includes current PNG, JPEG and GIS package policy paths; their added assertions are pending verification.

The expanded current package policy test also passed: one test, 28 assertions, runtime `[DEBUG]` witness, Nx 1.0 seconds. Current PNG, JPEG and GIS policies all selected the independently parsed `✏️s/.config/nextest.toml`. Artifact native checks may now resume; this policy result does not certify their compilation or runtime.

The initial stale `@semio-tech/repo-lib-js` command terminated before testing because the current project is `@semio-tech/repo-lib`; it is not a semantic test result. Both `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json` register the current verification command.

## Files

## Actual Nextest Scope Correction

The ancestor-selection TOML proof was incomplete: BMP and PNG runtime Nextest admission failed because `✏️s` overrides name packages belonging to that product workspace, while an isolated artifact workspace contains different members. This new failure was also reproduced in the registered test using a real isolated Cargo fixture and Nextest; one selection test passed and the native admission law failed before compilation on thirteen package filters.

The final configuration policy now binds explicit local profiles to the selected Cargo workspace. When that owner has no profile, the authored repository-wide common profiles apply. Product-specific package overrides stay within their actual product workspace. Intermediate product and sibling configurations do not author an isolated child workspace's package membership. Updated neutral fixtures and actual Nextest compile/runtime verification are pending.

The official [Nextest filter compiler](https://raw.githubusercontent.com/nextest-rs/nextest/main/nextest-filtering/src/compile.rs) explicitly refuses empty workspace package matches; changing equality to a regex would retain the failure. [Per-test settings](https://nexte.st/docs/configuration/per-test-overrides/) are scoped to the selected package graph. No no-test success flag, skipped verification or copied artifact-local configuration is used.

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧫️fixtures/🏎️nextest-configuration/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🏎️nextest-configuration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`

## Current Terminal Proof

Final selection and real native admission are GREEN: Bun/Nx session 67076 exited 0, two tests with 35 assertions, and an actual isolated Rust package compiled and executed its one Nextest law. Independent TOML selection and actual Nextest execution emitted `[DEBUG]` witnesses. The preceding corrected-policy run compiled and ran that law but its enclosing count assertion failed on ANSI colour separators; the assertion now strips terminal colour. No artifact compilation/runtime success follows from this infrastructure result.
