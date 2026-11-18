# 📓 Catalog Audit: semio-tech play release and pane boot references

Date: 2026-11-17. Read-only audit. No source files were edited and no git command was run.

## Result

No stale reference was found that would break the release build or a pane boot. All four checks passed.

## 1. Catalog panes (151) against the playground registry

- Catalog: `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json` (151 panes, 20 groups).
- Registry: `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json` (158 entries, 44 distinct `pluginId`s). It is generated, and its timestamp is 2026-11-17 18:09.
- Every pane `variant` exists as a registry `variant`.
- Every pane `example` exists in the `examples` list of a registry entry for that variant. Example names match after stripping the emoji prefix, for example `🌲️concrete-forest` matches `concrete-forest`.
- Every registry `cratePath`, `compositionConfigPath` and `devContribution` path exists on disk.

## 2. The 35 prepare/activate lanes in play's project.json

- `📋️project.json` `prepare-prototype` lists 35 `@semio-tech/framework-os-dev:prepare-<lane>-react-prototype` dependencies. `activate-prototype` lists the same 35 lanes plus `prepare-prototype`. Both lists are identical, with no duplicates.
- All 35 lane variants exist in the registry. Their paths exist on disk.
- The lane targets are not declared in the os-dev `📋️project.json`. They are inferred by the build graph from the generated registry. This is not stale, but nothing in os-dev declares them.
- The lane set is the greedy set cover from `🔨️modules/🧩️runtime/♻️activation/🟦️.ts` (`playActivationLanes`). `🧪️tests/🧪️playactivation/🟦️.ts` asserts it equals the project.json list. I did not run that test.
- The 116 panes whose variant is not a lane are covered transitively by closures. This is by design, not stale.

## 3. Release job roots (`targets.build.metadata.semio.release.jobs`)

All 13 jobs resolve. Each root is a directory containing `📜️script.ts`:

- os/framework roots: `describe-tool`, `font-assets`, `fonts`, `support`, `canvas-runtime-assets`, `flow-wasm`, `surface-wasm`, `editor-wasm`, `graph-wasm`, `puzzle-wasm`.
- play-owned roots: `map-tiles`, `bundle`, `pages` (all `🏢️semio-tech/🎡️play/🔨️modules/📦️site`). `📦️site/📜️script.ts` registers `map-tiles`, `bundle` and `pages`.
- `stageAfter` job ids (`job:describe-tool`, `job:support`) exist.

## 4. Prerequisites

- `workspace:deps-wasm-opt`: target exists in the root `📋️project.json`.
- `@semio-tech/plugin-registry:generate`: project `📇️registry/📋️project.json` has a `generate` target.
- `@semio-tech/assets:build`: project `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📋️project.json` has a `build` target.

## Other checks

- All play targets have a `cwd` that exists. All play `dependsOn` entries are either local targets or `@semio-tech/framework-os-dev:` lanes.
- A scan of relative imports in play and os-dev TypeScript sources found 0 broken imports. The os-dev `📜️script.ts` lazy-imports `../../♻️activation/🧰️preparation/🟦️.ts` from the dev project root, and that file exists.
- Play `package.json` `semio.browserSessionFactories.engine` points to `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust`, which exists.

## Low-priority observations (not breaking)

1. Duplicate generated registry. `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json` and `🎠️playgrounds.json` are byte-identical (172,905 bytes). `📖️catalog-view/🟦️.ts` reads the `🎠️` file. I did not find a consumer of the `🚀️` file. Since the repo is greenfield, the generator should emit one file.
2. `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` `cold-boot-check-s` depends on `activate-s-react-prototype`. This resolves because the `s` variant exists in the registry, but the target is inferred, not declared.

## Not verified

- I did not run `🧪️tests/🧪️playactivation`, `🧩️release-job-order` or any build target.
- I did not check that each job's sub-command (for example `wasm`, `stage-runtime-assets`) is registered in its root's `📜️script.ts`. I only checked that the script exists.
- I did not check example directories on disk. The registry is generated from disk today, so I treated it as the source of truth.

## Corrections during the audit

- An early suspicion that `♻️activation/🧰️preparation` was missing was wrong. The directory exists at `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts`.
- Some `*project.json` files are named with an emoji prefix (`📋️project.json`). Name lookups must match `*project.json`, not only `project.json`.
