# Result loc: no default language (row 59) and env on every generated launch route

Executor `loc`, foreground, no cargo, no sub-agents, no modifying git command, no `package.json` / `bun.lock` edit, nothing under the frozen partitions. `D` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`, `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `U` = `🧰️framework/🔨️modules/🖱️ui`.

## 1. `Locale` / `Terminology` have no `Default` (row 59: 57/1 -> 58/0)

* Generator fix (schema-first): `U/🎚️axes/📽️projection/🟦️.ts` no longer emits `#[default]` or `Default` in the derive of the two generated enums. The generated `U/🎚️axes/🤖️generated/🦀️.rs` (gitignored) was regenerated through the generator's own `uiAxesTargets` + `publishUiAxes` (stale before, fresh after); the TypeScript projection is unchanged.
* Census of consumers (static, 34,798 tracked and untracked `.rs` files; scripts in the scratchpad, not kept): `Locale::default()`/`Terminology::default()` occur once (`D/⚙️preferences` `cli_locale`); `Locale::parse(..).unwrap_or_default()` once (`Preferences::locale`); no `#[derive(Default)]` struct or enum holds a `Locale`/`Terminology` (the only derive-Default hit, `LocalizedLabel`, holds `[[Cow; Locale::COUNT]; Terminology::COUNT]`, which needs no `Locale: Default`); no `#[serde(default)]` field, no `let l: Locale = Default::default()`, no type alias or rename of either type.
* Dashboard consumer: the language is now data with an explicit chain. New module `D/⚙️preferences/🌐️language` (`🔣️.json` declaration `systemVariables: [LC_ALL, LC_MESSAGES, LANG]`, `fallback: [en, de]`; schema `D/🧬️schema/⚙️preferences/🌐️language/🔣️.json`; `🦀️.rs`; unit tests). Resolution order, lowest first: system variables (`de_DE.UTF-8`, `de-AT`, `de@euro` name `de`; `C`/`POSIX`/unsupported name none), first declared fallback, then shared journal, local journal, `SEMIO_LOCALE`, `--language`. `Preferences::default()` takes its language from the declared chain, `Preferences::baseline(lookup)` adds the system layer, `load`/`load_lenient` delegate to the new `load_in`/`load_lenient_in` (explicit environment, so tests do not depend on the process), `cli_locale` falls back to system then declared chain, `Preferences::locale()` falls back to the declared chain instead of `unwrap_or_default`. Journal compaction (`snapshot`) no longer freezes a language the journal never chose.
* Language-agnostic test `D/🧪️tests/🌐️language/🟦️.ts` (5 pass): Ajv validates the shipped declaration and refuses 9 invalid ones, `Intl.Locale` independently negotiates the shared system values, an independent resolver agrees with the shared environments, the dashboard source holds no default language and `emitUiAxesRust` emits no `Default`, every scenario is proved. Vectors in `D/🧪️tests/🧫️fixtures/🌐️language/🔣️.json`; feature scenario added to `D/🧪️tests/⚙️preferences/🥒️.feature` (proof list in the Rust unit test updated, order preserved).

## 2. Launch routes: the generator now exists and always emits `env`

Finding: there was no generator of per-crate `.vscode/launch.json` routes. `new crates` registered workspace members and owner rows only (`result-cg2.md` item 6), and the 4,772 existing `🦀️ <crate> 🏗️build|🔎️check|🧪️test 🛠️prototype|🚀️release` entries came from throw-away ticket scripts, which is why `env` regressed three times. New permanent module `L/🏗️authoring/📦️crate-tree/🚀️launch-routes`:

* `🧬️schema/🔣️.json`: the protocol (name prefix and icons, command template, target groups and order bases, `lib`/`tests` argument shapes) and the `Subject`/`Route` shapes. It names no environment variable: `🟦️.ts` takes `env` from `process/🏗️build/🎛️environment/🧬️schema` (`Protocol.const.environment`) and `crateLaunchRouteProblems` validates every route and its `env` against that schema.
* `🟦️.ts`: `crateLaunchRoutes`, `nextLaunchRouteOrders`, `crateLaunchRouteProblems`, `withCrateLaunchRoutes` (appends to the `configurations` array by text insertion, so the 8.9 MB catalog keeps every other byte; repeatable), `registerCrateLaunchRoutes`. `new crates` (`L/🏗️authoring/🎮️command/🟦️.ts`) now registers the routes of the carved crates right after the owner rows (not on `--dry-run`) and prints `launch routes: N added`; a restore list `<backup>-launch-routes.json` is written beside the backup.
* Test `L/🧪️tests/📦️crate-tree/🚀️launch-routes/🟦️.ts` (7 pass, imported by the `crate-tree` aggregator; the aggregator is 126 pass): env via Ajv + a SQLite oracle, refused subjects, a differential that regenerates the standard routes of >500 crates of the shipped catalog byte-equal modulo `order`, family conformance of `.vscode/launch.json` and the seed, an env-less route is reported, order and text-insertion laws. Negative control: emitting `env: {}` fails 3 tests (restored).
* The 4,772 routes of `launch.json` and the seed's family already satisfy the protocol (law `process/build/environment` 2/0). The seed is not written by the generator (same practice as uicarve; only `.vscode/launch.json`, the catalog `Protocol.const.catalog`).

## Verification

| command | result |
| --- | --- |
| `LAW3_ART=ticket LAW3_MAX_MS=3600000 LAW3_CAPS=<native grant> python3 C/law3-tools/run1.py "M/🖱️ui/🌐️locale/🧪️tests/🟦️.ts"` | 58 pass / 0 fail (was 57/1) |
| same runner, `M/🏃️process/🏗️build/🎛️environment/🧪️tests/🟦️.ts` | 2/0 |
| same runner, `M/🖱️ui/🧪️tests/🎚️axes/🟦️.ts` | 5/0 |
| `bun test ./D/🧪️tests/🌐️language/🟦️.ts` | 5/0 |
| `bun test ./L/🧪️tests/📦️crate-tree/🚀️launch-routes/🟦️.ts` | 7/0; aggregator `bun test ./L/🧪️tests/📦️crate-tree/🟦️.ts` 126/0 |
| `bun x tsc` (strict, repo options) over the new module and its test | 0 errors |
| taxonomy engine (`semanticDirectoryKindId`) | `🚀️launch-routes` = `crate-module-children`, its schema = `schema`, test folder = `members-of-tests` (same as siblings) |
| `bun test` dashboard `🚀️launch`, `⌨️controls` | 5/0 each; `🌀️control-plane` 8/3 (Nx namespace and missing `strip-ansi`, environment, unrelated) |

## Open

* Rust is verified statically only (no cargo): the dashboard `⚙️preferences` changes (`language` module, `load_in`, `snapshot`, tests) and their unit tests need one `bun repo cargo ... test -p semio-framework-repo-dashboard-preferences` and a check of dependents (`semio-framework-repo-dashboard-{terminal,daemon,cli}`) when the compiler is available. Census residual: only generic `T: Default` instantiations with `Locale` would escape grep (none found).
* The seed catalog is not extended by the generator.
* `lib-wiring-budget` is red for 7 unrelated plugin roots (puzzle jobs, generation3d, process3d, energy, layout, norm), peers' files.

## Files

Production: `U/🎚️axes/📽️projection/🟦️.ts`; `D/⚙️preferences/🦀️.rs`; `D/⚙️preferences/🌐️language/{🔣️.json,🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`; `D/🧬️schema/⚙️preferences/🌐️language/🔣️.json`; `D/README.md`; `L/🏗️authoring/📦️crate-tree/🚀️launch-routes/{🟦️.ts,🧬️schema/🔣️.json}`; `L/🏗️authoring/🎮️command/🟦️.ts`.
Tests: `D/⚙️preferences/🧪️tests/🔬️unit/🦀️.rs`; `D/🧪️tests/🌐️language/🟦️.ts`; `D/🧪️tests/🧫️fixtures/🌐️language/🔣️.json`; `D/🧪️tests/⚙️preferences/🥒️.feature`; `L/🧪️tests/📦️crate-tree/🚀️launch-routes/🟦️.ts`; `L/🧪️tests/📦️crate-tree/🟦️.ts`.
Regenerated, gitignored: `U/🎚️axes/🤖️generated/🦀️.rs`.
