# Continuation TypeScript Execution

Execution in progress on 2026-09-30 under the reopened parent ticket. No Git mutations, worktrees, AGENTS edits or new script files.

## Owned Work

The presentation project names one authored package but imports a separate pseudo-package for its spec source in five slide consumers and repeats the alias in Vite and Vitest. Its package exports only the root. The portable direction schema, fixture and test now specify three public export cases: canonical `/spec` must resolve; private subpath and pseudo-package must fail. The independent resolver materializes the actual authored manifest and export filenames in a ticket-owned package tree; Ajv validates the portable contract.

## Checks

Uncached red lint and regression checks are running through existing Nx targets. Actual results will replace this section after execution. No passing result is claimed yet.

## Executed Checks and Additional Ownership

- Uncached baseline `@semio-tech/repo-lib:lint-dependency-direction`: failed after 120s owned timeout before verdict, 9,004 sources inventoried. No passing direction claim.
- Uncached `@semio-tech/repo-lib:test-dependency-direction` before the spec export: 8 passed / 1 failed, 516 assertions; canonical public spec subpath unresolved independently.
- After export repair, the same uncached regression target: 9 passed / 0 failed, 523 assertions, 14.39s test time.
- Uncached `@semio-tech/presentation:test`: passed (owned core suite); exact test counts to be preserved below.
- Uncached live lint after spec owner repair: completed in 1m23s and failed on nonexistent WFC grid3d snapshot binary facade export. Timeout increase was unnecessary and no script budget change was made.

Remaining authored spec consumers in Animate were found by an independent local source inventory. They now use the same public `/spec` subpath; its test resolver now resolves the actual spec file. Animate presentation consumers and resolver entries now use the existing neutral `@semio-tech/presentation` owner. The duplicate old model differs from the canonical owner in obsolete slide-path parsing and in-source test registration; removal/preservation analysis remains in progress. The Puzzle story's parser export exists in `@semio-tech/puzzle-wasm` generated bindings and now imports that actual authored package.

WFC root TypeScript barrel exports 55 artifact facades, including nonexistent text/binary files under grid3d. No authored TypeScript consumer imports this barrel. WFC's registered TypeScript package contributes a real Vitest test route over artifact examples and schema oracles. The runtime barrel violates plugin→artifact ownership independently of its missing targets; ownership repair must retain the real test route without a substitute facade API.

## Complete Live Resolver Diagnostics Before Unused Barrel Retirement

- Dependency direction graph has an unresolved workspace dependency: ✏️s/🔌️plugins/🎞️animate/📖️stories/🎭️presentation-deck/🧪️.story.tsx → @semio-tech/animate-js/globals.css
- Dependency direction graph has an unresolved workspace dependency: ✏️s/🔌️plugins/🎬️sequence/🧪️tests/🌐️browser-consumer/🟨️.js → @semio-tech/sequence-sequence
- Dependency direction graph has an unresolved workspace dependency: ✏️s/🔌️plugins/🎬️sequence/🧪️tests/🔮️protocol-oracle/🟨️.js → @semio-tech/sequence-sequence
- Dependency direction graph has an unresolved workspace dependency: ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🟦️.ts → ../../../../../../../../../../../../../🧬️schema/🌳️ast/🟦️
- Dependency direction graph has an unresolved workspace dependency: ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/🟦️.ts → ../../🟦️
- Dependency direction graph has an unresolved workspace dependency: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/🧬️schema/🟦️.ts → ../../✳️base/🧬️schema/🟦️
- Dependency direction graph has an unresolved workspace dependency: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/🧬️schema/🟦️.ts → ../../✳️base/🧬️schema/🟦️
- Dependency direction graph has an unresolved workspace dependency: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/🧬️schema/🟦️.ts → ../../✳️base/🧬️schema/🟦️
- Dependency direction graph has an unresolved workspace dependency: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts → ./✏️set-attribute/🟦️.ts
- Dependency direction graph has an unresolved workspace dependency: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts → ./✏️set-declaration/🟦️.ts
- Dependency direction graph has an unresolved workspace dependency: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts → ./✏️set-doctype/🟦️.ts
- Dependency direction graph has an unresolved workspace dependency: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts → ./✏️set-text/🟦️.ts
- Dependency direction graph has an unresolved workspace dependency: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts → ../../🟦️.ts
- Dependency direction graph has an unresolved workspace dependency: ✏️s/🧪️tests/🎭️storybook-end-to-end/🟦️.ts → ../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins.ts
- Dependency direction graph has an unresolved workspace dependency: 🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🟦️.tsx → @semio-tech/ui-react/chrome
- Dependency direction graph has an unresolved workspace dependency: 🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts → @semio-tech/ui-react/i18n
- Dependency direction graph has an unresolved workspace dependency: 🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/👥️presence/🟦️.tsx → @semio-tech/ui-react/chrome
- Dependency direction graph has an unresolved workspace dependency: 🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🪟️chrome/🟦️.tsx → @semio-tech/ui-react/chrome
- Dependency direction graph has an unresolved workspace dependency: 🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/▶️run/🟦️.tsx → @semio-tech/ui-react/chrome
- Dependency direction graph has an unresolved workspace dependency: 🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🏆️leaderboard/🟦️.tsx → @semio-tech/ui-react/chrome
- Dependency direction graph has an unresolved workspace dependency: 🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🏠️home/🟦️.tsx → @semio-tech/ui-react/chrome
- Dependency direction graph has an unresolved workspace dependency: 🧰️framework/🛍️products/❓️quiz/🧪️tests/🏠️home-grid/🟦️.tsx → @semio-tech/ui-react/chrome
- Dependency direction graph has an unresolved workspace dependency: 🧰️framework/🛍️products/❓️quiz/🧪️tests/🗣️translation-completeness/🟦️.tsx → @semio-tech/ui-react/i18n

## Retired Unused Runtime Barrels

All entries below were unused by authored package imports and relative source imports. Registered private owner test routes remain; manifests no longer advertise runtime exports. No empty facade was added. Animate consumers now call the actual neutral presentation renderer, including its public CSS path.

- `✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/➗️mathematical/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/➗️mathematical/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🌿️vcs/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🌿️vcs/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🎥️shooting/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🎥️shooting/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🏭️process/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🏭️process/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/📋️forms/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/📋️forms/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/📖️playbook/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/📖️playbook/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/📜️imperative/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/📜️imperative/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🔋️energy/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🔋️energy/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🔱️trinity/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🕸️dag/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🕸️dag/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🗒️note/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🗒️note/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🪐️space/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🪐️space/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/📜️script.ts`
- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📦️index.ts`
- `✏️s/🔌️plugins/🎞️animate/🧪️tests/🎚️config/🟦️.ts`
- `✏️s/🔌️plugins/🎞️animate/📖️stories/🎭️presentation-deck/🧪️.story.tsx`
- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🏗️builder/🌐️vite/🟦️.ts`

## Concrete Owner and Stale Authored Path Repairs

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts`
- `✏️s/🧪️tests/🎭️storybook-end-to-end/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🪟️chrome.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🌐️i18n.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🟦️typescript/package.json`
- `package.json`
- `🏢️semio-tech/🎡️play/🟦️.tsx`
- `♻️mit-bestand/🧺️demonstrator/🟦️.tsx`
- `♻️mit-bestand/🧺️demonstrator/🐚️shell/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🎚️config/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🧑‍💻dev/🚀️entry/🟦️.ts`
- `✏️s/🧑‍💻dev/🚀️entry/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🌳️ast/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/🟦️.ts`

- Updated concrete composer metadata/dependencies: `🏢️semio-tech/🎡️play/package.json`, `♻️mit-bestand/🧺️demonstrator/package.json`.
- Portable public-export matrix extends to actual UI chrome/i18n and artifact-owned Puzzle session manifests.
- Before removing facade entry files, package-import inventory and relative import/reexport inventory both confirmed zero consumers except the explicitly repaired Animate/Puzzle clients.
- Removed Animate duplicate core and its wrapper test registration only after comparing all55 named tests with the canonical owner suite; none were missing. The canonical owner suite ran56 tests successfully.

## Concrete Puzzle Development Composition Owner

Moved its actual contribution, Vite config and entry together into the S development variant owner. The contribution retains its narrowed ownerRoot and concrete session factories; the plugin no longer imports its artifact. Parent updates Cargo metadata strings referencing the contribution.

- `✏️s/🔌️plugins/🧩️puzzle/🧑‍💻dev/🚀️entry/🟦️.ts`
- `✏️s/🧑‍💻dev/🎭️variants/🧩️puzzle/🚀️entry/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts`
- `✏️s/🧑‍💻dev/🎭️variants/🧩️puzzle/🏗️builder/🌐️vite/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🧑‍💻dev/🧬️schema/🔣️.json`
- `✏️s/🧑‍💻dev/🎭️variants/🧩️puzzle/🧬️schema/🔣️.json`

## Artifact-Owned Sequence Consumer Oracles

Moved the public browser consumer and independent protocol oracle to their actual artifact owner and updated the existing registered script route; plugin runtime no longer imports the artifact for these tests.

- `✏️s/🔌️plugins/🎬️sequence/🧪️tests/🌐️browser-consumer/🟨️.js`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🧪️tests/🌐️browser-consumer/🟨️.js`
- `✏️s/🔌️plugins/🎬️sequence/🧪️tests/🔮️protocol-oracle/🟨️.js`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🧪️tests/🔮️protocol-oracle/🟨️.js`
- `✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/📜️script.ts`

## Source Resolution Contract

The portable public export fixture now selects `semio-source` before normal import conditions and expects the package-contained authored Sequence entry. Published `import`/`types` entries keep their declared generated build output. The existing Sequence artifact build passed uncached before this change (three outputs); the final architecture run must resolve its source condition after removing only that invocation's generated `dist`.

The existing wire schema and Rust QueryResult already defined table/graph output. The missing TypeScript declaration now sits next to that schema, and three portable JSON cases ask the independent TypeScript compiler to accept a valid table and reject invalid kind/rows.

- Final intermediate live TS check inventoried8,978 sources but hit the existing120s command budget before an edge verdict.
- A full registered regression rerun hit its existing45s total budget under concurrent load after the independent neutral resolver test alone took33s; this is an execution limitation, not a verdict.
- Retained private test manifests use the existing `repo` bundle category; there is no new bundle taxonomy or graph exception.

The focused source-condition RED ran through `bun nx exec --projects=@semio-tech/repo-lib --excludeTaskDependencies -- bun test <existing-source> -t 'authored owner exports'`: 0 passed, 1 failed, 21 assertions,6.48s. It received `dist/🟦️.js` instead of portable expected `🟦️.ts`. The canonical resolver now requests the explicit source condition, and the actual package exports that condition to a package-contained entry forwarding its artifact API. Its publication build conditions remain intact. The successful build's temporary `dist` was removed before the final live architecture run.

- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs`
