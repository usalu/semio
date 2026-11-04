# Assets Source Admission — 2026-11-03

## Cause

The focused Flow Cargo invocation exited 1 in native Source admission before Cargo metadata. Its captured import chain reached `@semio-tech/assets/resolver/delivery` through styling projection and Storybook preparation. Root workspace membership already includes the exact assets TypeScript package, and the package already declares the delivery export. The manifest omitted `private: true`; the Source workspace index correctly preserves nonprivate members without authorizing them. The authorizer guard was correct and remains unchanged.

The assets README describes shared repository assets served through the asset port. Its owner targets have no publication route. This is a private first-party workspace package, so its manifest now explicitly declares that fact. No dependency, export, import, compatibility alias, resolver fallback or policy was added.

## Regression and Validation

Added the language-neutral `authorizers/📦️workspace/🧪️tests/assets.json` contract and one authored-manifest regression alongside the existing workspace export laws. It validates actual root membership, actual private metadata and exact delivery export using the native captured resolver, independent SQLite JSON admission and Ajv.

Before the manifest edit, the new regression failed: SQLite found no eligible private assets owner and returned undefined instead of the expected delivery source. After adding only `private: true` to the assets package manifest, the new regression plus the existing portable private export corpus pass: 2 pass, 0 fail, 38 expectations; one process-invocation test was filtered out.

Command: `repo tools exec --light bun -- test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️workspace/🧪️tests/🟦️.ts -t 'authored assets delivery|portable private export decisions'`.

## Files and Handoff

Updated the assets TypeScript package.json and workspace authorizer test module; added its assets.json fixture and this report. Source edits are complete. Dependency lock/sync and a native Source/Cargo retry are delegated to the root after the coordinated freeze; neither ran during this fix. The supplemental focused laws prove the authored package contract; a native Source gate pass remains pending.
