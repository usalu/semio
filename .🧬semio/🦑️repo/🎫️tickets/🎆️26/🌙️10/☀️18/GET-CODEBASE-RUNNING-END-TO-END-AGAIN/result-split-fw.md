# Result split-fw (framework and teaching monolith files)

Executor `split-fw`, foreground, no sub-agents, no modifying git command. `C` = this ticket folder. Tools: `C/split-fw-tools/` (`tssplit.py`, `tssplit2.py` TS splitters with line-multiset oracle, `rssplit.py` Rust `include!` fragments, `repoint.py` importer rewriter, `rs/` copy of the split-s `modsplit.py` for module splits, `vt.sh`/`vtt.sh` vitest runners, `orig/` originals, `*.json` specs). Logs: `C/🗑️generated/split-fw/`.

## Verdict per file (census 8, `🧰️framework/**` and `🎓️teaching/**`)

| # | File (before) | Lines | After | Children |
| ---: | --- | ---: | --- | --- |
| 2 | `📚️course/🧪️tests/🚶️learner-journey/🟦️.tsx` | 2,167 | deleted | already split by the codex course ticket into `🚶️journey-and-tabs`, `🧩️deputy-and-contract`, `🏔️challenge-session`, `🧷️run-truth`, `🧰️learner-journey-kit` (not named in the vitest config or the tsconfig; all 47 test titles of the old file exist in the new suites, which hold 49) |
| 11 | `📚️course/🧪️tests/📡️presence-client/🟦️.tsx` | 1,051 | deleted | `🧰️presence-client-kit` 180, `🚪️presence-room` 213, `🧭️presence-places-and-rooms` 210, `🖱️pointer-and-focus` 124, `🪟️presence-screens` 335 |
| 8 | `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🟦️.ts` | 1,163 | 306 | `🌐️site` 135, `🐳️docker-image` 90, `🩺️docker-image-check` 330, `🧱️docker-stack` 178, `🎁️stack-bundle` 129, `⚖️readiness` 67 |
| 14 | `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` | 1,016 | 284 | `🔖️turn-result` 377, `🩹️retained-ui-patch` 70, `🚚️more-work-drive` 145, `📥️inbound-request` 80, `🧵️spawned-job` 98 |
| 15 | `…/💻️os/…/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts` | 1,010 | 330 | `🧑‍🎨engine/🔮️oracles/`: `📇️directory-and-hub-contracts` 235, `🖱️world3d-gestures-and-selection` 321, `🪪️surface-host-retention` 119, `🧬️owned-schema-exports` 25 |
| 4 | `🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🧪️tests/🔬️unit/🦀️.rs` | 1,495 | 126 | `include!` fragments `🔗️open-ready` 50, `🧲️docking` 477, `🎹️keymap-and-launcher` 283, `🖼️window-focus-and-sessions` 267, `🩺️runtime-audit` 143, `🧭️scenario-proofs-and-batching` 185 |
| 9 | `💻️os/🔨️modules/📡️spr/📜️history/🧪️tests/🔬️unit/🦀️.rs` | 1,077 | 126 | `include!` fragments `🧩️composition-and-retained-decode` 178, `⚔️conflicts-and-messages` 211, `📜️text-grammar-and-payloads` 230, `🔐️codec-and-append` 119, `🧮️fold-scan-frontier-and-index` 174, `⏳️native-fold-law` 57 |
| 16 | `💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🦀️.rs` | 1,006 | 107 | modules `🔭️artifact-view` 180, `🪪️operation-contexts` 104, `🔍️lane-views` 152, `🕳️empty-lanes` 408, `🫂️peer-presence` 104 (explicit `pub use` lists, 8 fields/members widened to `pub(super)`) |
| 5 | `🐾️pets/🔨️modules/🚶️locomotion/🦀️.rs` | 1,442 | 70 | modules next to the existing TS twin folders: `⚓️footing` 76, `🛬️courses` 116, `🛫️departure` 128, `🏃️motion` 223, `✋️hand` 108, `🪜️outings` 485, `🧳️trips` 173, `🪄️mischief` 104, `🚪️leaving` 34 (`pub(crate) use` lists, 33 fields/members widened to `pub(super)`) |

Every new file is below 500 lines; every `// #region` has its `// #endregion` in the same file (checked on all new files).

## Method

- TS: `tssplit.py` / `tssplit2.py` take line ranges of the recorded original, regenerate imports per part (only used specifiers, relative specifiers re-based, `type` modifiers on type-only cross-part imports), add `export` to cross-part names and compute a line-multiset oracle (`unassigned 0`, `lines assigned twice 0`, `OUTPUT vs ORIG diff 0 0`, no import cycle). Two relative `import("…")` expressions inside moved code were re-based by one level (`🩺️docker-image-check`).
- Rust test roots: `rssplit.py` keeps every non-moved line in the root in original order, appends one `include!("<child>/🦀️.rs");` per fragment, rebases `include_str!("../…")` by one `../`, and compares the line multiset (`MISSING 0 EXTRA 0`) and the `#[test]` count (terminal 74 to 74, history 1 to 1 plus unchanged `async_test` multiset).
- Rust lib modules: the split-s `modsplit.py` (copied to `C/split-fw-tools/rs/`, its own scratch `/Users/ueli/.cache/splitfw`): whole items move into `#[path] mod` children with `use super::*;`, explicit re-export lists, cross-child private fields and members widened; `modsplit.py lossless` reports `LOSSLESS OK` for `declview` and `locomotion`.

## Importers re-pointed and text readers fixed

- Deploy: `📦️packages/🟦️typescript/📜️script.ts`, `🎭️e2e/🟦️.ts`, `🧪️tests/🧪️deploy/🟦️.ts`, `🧪️tests/🪢️presence/🟦️.ts` (symbol to module map, no barrel). Source reader `🧪️tests/🩺️preflight/🟦️.ts` now also reads `🚀️deploy/🐳️docker-image/🟦️.ts` (the image guard moved there).
- Wire-turn: 26 importers by `repoint.py` (`🧊️wgpu/🐚️plugin-bridge/*`, `🏪️store/👷️worker`, `📺️renderer/🧑‍🎨engine/🧪️tests/*`, `🧱️elements/🔌️PluginRuntime/*`), the two public entries `🧰️framework/📦️packages/🟦️typescript/🎭️actor/🖼️wire-turn/🟦️.ts` and `🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn/🟦️.ts` (explicit re-exports now name the owning child), the `Pick<typeof import(…)>` type of `🧪️tests/📨️effect-wire-routes`, and the actor vitest config coverage include (`🖼️wire-turn/*/🟦️.ts`). `🖼️wire-turn/🟦️.ts` keeps the effect wire and the in-source test host (`includeSource` unchanged).
- Presence: `🎚️config/🟦️.ts` and `tsconfig.json` of `@semio-tech/course-react` list the four new suites instead of `📡️presence-client`; `🔮️oracles/🔣️.json` names the new suites in the lodash throttle rationale.
- Renderer-react oracles: doc reference in `🧪️tests/🖱️world3d-interaction/🟦️.tsx`; the eight `*Oracle` functions now live in `🔮️oracles/*`, the script keeps the verbs.
- Terminal tests: `every_scenario_of_the_window_view_features_is_proved_by_a_test` read its own file as text; it now reads the root and all six fragments (`let sources = [include_str!(…), …]`, `&[source]` to `&sources`).

## Verification

| Check | Command | Outcome |
| --- | --- | --- |
| Presence suites (4 files) | `C/split-fw-tools/vt.sh presence2 presence-room presence-places pointer-and-focus presence-screens` (vitest, `@semio-tech/course-react` config) | 4 files, 61 tests passed (the old file had 61) |
| Deploy and presence tests of the teaching course | `C/split-fw-tools/vtt.sh deploy4 <teaching config> deploy presence` | 2 files, 29 tests passed |
| Wire-turn in-source and actor suites | `C/split-fw-tools/vtt.sh actor1 <actor config>` | 105 of 107 passed; the 2 failures (`🗞️typed-operation-page` lane maximum 14 versus `TYPED_OPERATION_RESULT_LANE_MAX` 15) read the relocated fixture `🖼️wire-turn/**/🧪️tests/🧫️fixtures` that another fleet is moving (untracked in `git status`); the constant is 15 at HEAD too |
| Renderer-react suites importing wire-turn | `vtt.sh rr3 <react config> more-work-drive inbound-request package-integration` with `SEMIO_TEST_LEVEL=long` | `🚚️more-work-drive` passes; `📥️inbound-request` cannot load (`@dnd-kit/sortable` missing in this tree) and 6 `package-integration` wgpu tests fail on `bun@1.4.2` versus installed 1.3.14; none touch the moved code |
| Renderer oracles | `bun run` of the original script copy against the new modules, all 8 oracle functions on the real repo | identical check counts: 33, 15, 21, 29, 75, 17, 89, 26 |
| TS syntax and resolution | `bun build --no-bundle` on every new and touched file; `bun build --target=bun` on the course script, the renderer script, the wgpu plugin-module-loader and 26 wire-turn importers | all parse; only the pre-existing `@semio-tech/course` and `chromium-bidi` resolutions fail |
| Rust parse | `rustfmt --edition 2024 --check` over every new fragment and child | no parse error |
| Rust line oracle | `rssplit.py terminal.json`, `history.json`; `modsplit.py lossless declview`, `locomotion` | MISSING 0, EXTRA 0 (only wiring, docs, re-exports) |
| Course package typecheck | `tsc -p …/📦️packages/🟦️typescript/tsconfig.json` | see "Open items" |
| Cargo | `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p <crate> --lib --tests` | see "Open items" |

## Open items

- Cargo: not run to completion. `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-repo-dashboard-terminal -p semio-framework-os-history -p semio-framework-plugin-decl-views -p semio-framework-pets-staging-draft -p semio-framework-pets-staging --lib --tests` was tried 8 times between 15:20 and 16:25 (logs `C/🗑️generated/split-fw/cargo1..6.log`, retry driver `C/split-fw-tools/cargo-retry.sh`). Outcomes: `Bounded unique parent kernel roster required` / `System image plist required` (native image roster), `cargo fetch --locked` refusing a workspace in churn (`semio-framework-os-kernel-dsl-derive-expand` feature `testkit`, `semio-framework-value-persistent`, `cannot update the lock file`), and two 590 s SIGALRM expiries while queued behind the fleet. The coordinator should run that command once the workspace resolves. Static evidence meanwhile: line oracles, `defscan.py` (0 cross-file private leftovers for `declview` and `locomotion`), rustfmt parse of every new file.
- `🐾️pets/…/🚶️locomotion/🦀️.rs` sits in `semio-framework-pets-staging-draft`, which `rg-core` created at 15:03 and which does not compile yet by itself (the monolith names `Trip`, `Foothold`, `Rope`, `Pitch`, `LadderStand` and `Course` without importing them). After the split every missing import belongs in the root `use` block (the children take it through `use super::*;`), and widening a function to `pub` belongs in the child that now holds it (`outings` for `set_out`, `departure` for `stroll`, and so on). `🎪️stage/🧪️tests/🔬️unit/🦀️.rs` `sources()` now also reads the nine children (11 to 20 entries), otherwise the forbidden-token law would only scan the 70-line root.
- The `include!` form keeps one module per test root, so test names are unchanged (terminal 74, history 1 `#[test]` plus the `async_test` set).
- `🚶️learner-journey/🟦️.tsx` was removed rather than split: it is the pre-split ancestor of the six codex suites (title sets compared, all 47 old titles present). Restore it from `git show HEAD:…` if a title is ever found missing.
- The taxonomy registry `members-of-tests` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`) already lacks most course test directories (including the codex course suites) and still lists `learner-journey` and `presence-client`; not edited, the drift predates this slice.
- `🗞️typed-operation-page` fixture drift (2 failures in the actor run) and the `@dnd-kit/*` and `bun@1.4.2` environment gaps belong to other slices.
