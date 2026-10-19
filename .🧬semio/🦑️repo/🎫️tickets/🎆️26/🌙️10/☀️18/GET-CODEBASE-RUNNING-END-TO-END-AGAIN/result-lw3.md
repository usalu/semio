# Result lw3: WP3 plugin to plugin and s-dev to plugin violations

Executor `lw3`. Freeze respected: no `package.json` / `bun.lock` edit, no cargo, no git-modifying command, no ticket tool. Tools (inputs, kept): `lw3-tools/{cruise.sh,agg.ts,scan-literals.ts,strip-bad-export-key.cjs,exports-needed.json}`; tool output deleted.

## 1. Verdict

The WP3 premise is thin: **there is no plugin to plugin import left** (TS or Rust source). `no-cross-plugin-<a>-to-<b>` (1,260 rules) has 0 hits, `s-modules-no-plugins` 0. `plugins-framework-sdk-only` is plugin to framework by definition, so it has no "between plugins" hits (those belong to the WP1/WP2 executors). What was left inside `✏️s`:

| rule (scoped run `✏️s --include-only ^✏️s/`, 8,751 modules) | before | after |
|---|---:|---:|
| no-circular | 1 | **0** |
| no-cross-package-relative | 23 | **9** |
| plugin-no-extension-or-artifact | 15 | **5** |
| total | **39** | **14** |

The 14 left all need a `package.json` / workspace change (frozen); they are specified in `lw3-tools/exports-needed.json`.

## 2. Changes (file list)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/🧿️semio-frames/🟦️.test.ts` and `…/🧪️tests/🧫️fixtures/🧿️semio-frames/🔣️.json`: moved from the stdio plugin `🧪️tests` (a plugin-level test reaching into one artifact: 12 ncpr + 6 plugin-no-extension-or-artifact edges). Imports now relative inside the artifact owner, fixture `subsetsRoot` re-based to `🏅️standards/🔖️v1/🪆️subsets`.
- `✏️s/🧪️tests/🏘️plugin-composition/🚢️stdio-shipped-fleet/🟦️.ts` plus `🧫️fixtures/{🧠️owners,🧩️dependencies}/🔣️.json`: moved from `stdio/🧪️tests/🚢️shipped-fleet` + `stdio/🧪️tests/🧫️fixtures/🚢️shipped-fleet`. It spans four stdio artifacts through their public package entries, so it lives where plugins may meet (`🏘️plugin-composition`, same home as `🖨️raster-decoded-image`). Fixture and artifact-definition URLs re-based; case added to the `test` list in `✏️s/🧪️tests/🏘️plugin-composition/📜️script.ts`. The hub's own Rust `🚢️shipped-fleet` test and fixtures are untouched (they never read the moved files).
- `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧪️compose5d-preparetopologymodel/🟦️.tsx`: dropped the deep relative import of the 5d-react compose-model leaf. Importing the package entry instead created a cycle (module hook to test to entry), so the suite is now self-contained: test-local `FlatRecord` and a minimal `ComposedModel`, `registerTests1<Model extends ComposedModel>`; the module hook call type-checks unchanged.
- `✏️s/🧪️tests/🎭️storybook-end-to-end/🟦️.ts`: `../../🧑‍💻dev/🧩️catalog/🤖️generated/🧩️plugins/🟦️.ts` replaced by `@semio-tech/s-dev-catalog-ts/generated/plugins` (export already exists).
- Already gone when I started: the architect `registers/🗂️catalog` <-> `schema` cycle from lay1 (the catalog was regenerated at 00:05 and imports its sibling schemas directly).

## 3. Left for the frozen batch (`lw3-tools/exports-needed.json`)

1. **raster** (3 edges): `@semio-tech/raster-js` is a plugin-level package whose glue files under `📦️packages/🟦️typescript/🗿️artifacts/…` import the artifact. Move the package to `🗿️artifacts/🖨️raster/📦️packages/🟦️typescript` (stdio/puzzle-2d pattern); names and subpaths unchanged, so the one importer stays valid.
2. **puzzle 5d** (1 edge): new artifact package `@semio-tech/puzzle-5d` for the schema-diff glue; remove `./artifacts/5d/…` from `@semio-tech/puzzle-js`; rewrite one import in `✏️s/🧑‍💻dev/🧩️puzzle/🧪️tests/🔺️diff-parsers/🟦️.ts`.
3. **energy epw** (1 edge): remove the dead export `./extensions/epw/testing/unit` (no importer anywhere, `git grep`) and its glue file.
4. **puzzle stories** (9 ncpr edges, `✏️s/🧑‍💻dev/🧩️puzzle/📖️stories/{2d-fixtures,3d-world,5d-timeline}` to example `.dsl.semio` assets): already specified in `lw2-tools/exports-needed.json` (puzzle-2d, puzzle-3d-ts/rs, puzzle-js); the 5d ones should land on `@semio-tech/puzzle-5d` once item 2 exists.
5. **Phantom manifest edges** (not seen by depcruise): 20 plugin `-js` packages (writer, procedural, gis, vcs, shooting, demonstrator, architect, process, reasoning, forms, layout, norm, playbook, imperative, trinity, dag, note, puzzle, space, sourcing) declare `dependencies` on the four `@semio-tech/cad-js-module-*` extension packages; no source in those plugins imports them. Also unused: `block-js` -> `s-dev-block-ts`, `fem-js` -> `s-fem-composition-tests`. Remove in the batch.

## 4. Observations (no change made)

- Rust: `block`, `space`, `sourcing`, `note`, `puzzle` crates depend on stdio artifact crates by Cargo path (`📦️packages/🦀️rust`, the crate public entry, not a deep import); the only other cross-plugin path strings in `.rs` are doc comments. `✏️s/🔌️plugins/🧪️tests/📁fixture-placement` is a law fixture that lists plugin paths as data.
- Reverse direction without a rule: `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🚀️renderer-boot-variants/🟦️.ts` imports `@semio-tech/s-dev-catalog-ts/generated/playgrounds` (plugin test to s-dev catalog). It is wired through `🎚️renderer-contract` (OS renderer config, lw2's area), so I left it; the clean fix is to move it to `✏️s/🧑‍💻dev/🧩️puzzle/🧪️tests` and include it from there like `🔺️diff-parsers`.
- **A peer broke the cruiser policy loader during this run**: `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/package.json` got a prose key in `exports` ("(declare the built wasm package …)", staged at 00:23), and `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/package.json` + `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/package.json` have `".//…"` export targets. Every `dependency-cruise` run aborts with "Invalid authored package export key/target" until those are fixed. I did not edit them (freeze, not mine); my runs used `lw3-tools/strip-bad-export-key.cjs` (`LW3_PRELOAD`, in-memory only). The owner (probably the lw2 exports applier) should fix the three manifests.

## 5. Verification (exact commands, outcomes)

- Cruise, before and after: `lw3-tools/cruise.sh <out.json>` = `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs ✏️s --include-only '^✏️s/' --config <policy .cjs> --output-type json`; `bun lw3-tools/agg.ts <out.json>`. Before 39 (23 ncpr, 15 plugin-no-extension-or-artifact, 1 circular), after 14 (9, 5, 0); a first re-run caught a cycle I had introduced (5d-react entry <-> suite) which the suite rewrite removed.
- `bun test ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/🧿️semio-frames/🟦️.test.ts`: 47 pass, 0 fail (107 expects).
- `bun test ./✏️s/🧪️tests/🏘️plugin-composition/🚢️stdio-shipped-fleet/🟦️.ts`: 5 pass, 0 fail (45 expects).
- `tsc --noEmit` over the compose5d suite + 5d-react module: only the expected `ImportMeta.vitest`/`dir` environment errors (vitest/bun types not configured in the scratch tsconfig); no error on the new `registerTests1` signature or its call.
- `bun -e 'await import("@semio-tech/s-dev-catalog-ts/generated/plugins")'` from `✏️s/🧪️tests/🎭️storybook-end-to-end`: resolves, `PLUGIN_HOST_CONFIGS` present.
- Taxonomy admission of the 6 new directories (`lay1-tools/tax.ts`): all admitted (`members-of-tests` / `fixtures`).
- Literal scan (`lw3-tools/scan-literals.ts ✏️s`, 72,957 files, every relative / `✏️s/🔌️plugins/…` string literal resolved): no TS/JS cross-plugin reference.
- Not run: vitest for the compose5d suite (brepjs/vitest environment, see lay1 open item 1), cargo.
