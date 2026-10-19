# Result fnsplit2 (function-length census and decomposition of functions over 600 lines)

Executor `fnsplit2` (Sonnet 5.5), foreground, no sub-agents, no modifying git command, no cargo build, no `bun repo` command (a `bun repo dependencies sync --all` plus other agents' cargo runs were active the whole time). `C` = this ticket folder.

STATUS: done for everything that is a pure extraction. 41 of 54 outermost functions over 600 lines are decomposed; 13 remain over 600 (4 inside the freeze zone, 9 that need a restructuring rather than an extraction, see Open items). After the split no root exceeds 246 lines and no new helper exceeds 439 lines (`check_uls_actions`); most helpers are 100-250 lines.

## Census

- `C/fnsplit2-census.md`: every outermost function or method body over 300 lines (Rust and TS) after this run, 297 rows (13 over 600, 284 between 301 and 600). Columns: lines, kind, name, generated, age in minutes, file:start.
- Tool: `C/fnsplit2-tools/census.ts` (`bun C/fnsplit2-tools/census.ts --out <md> --json <json>`; Rust: own lexer over `git ls-files -co` with strings, raw strings, lifetimes and nested comments; TS: TypeScript compiler API, every function-like node with a body). Excluded roots: node_modules, target, 🗑️generated, 🎫️tickets, .cache, dist.
- Before/after per function: `C/fnsplit2-stats.md` (same data as the table below), tool `C/fnsplit2-tools/stats.ts`.
- Before this run: 66 function bodies over 600 lines (54 outermost). After: 13 outermost.

## Results (function, before lines, after root, largest helper, verification)

Line numbers come from `census.ts` (function start to end line). "after root" = the original function in its file, "largest helper" = the longest function among the new files below the same folder.

| function (file tail) | before | after root | largest helper | state | verification |
| --- | ---: | ---: | ---: | --- | --- |
| `check_special_elements` (`⚖️checks/🔗️special-element-checks/🦀️.rs`) | 980 | 7 | 246 | split | Rust: rustfmt parse, inline-capture audit (rscap), helper-name collision grep, shape check; cargo not run |
| `check_full_seismic` (`💡️inferences/🌍️full-seismic-check/🦀️.rs`) | 972 | 136 | 271 | split | Rust: rustfmt parse, inline-capture audit (rscap), helper-name collision grep, shape check; cargo not run; oracle5: only the 15 extern-crate path uses (`semio_s_artifact_norm_en1998_model::…`) are new |
| `registerGisInferencePortTests` (`🧪️tests/💡️inference-port/🟦️.ts`) | 953 | 7 | 193 | split | registration trace identical (27 events), tsdiff clean |
| `registerGisDocumentOpenTests` (`🧪️tests/🗺️document-open/🟦️.ts`) | 941 | 941 | - | **remaining** | see Open items |
| `registerTests1` (`🧪️tests/🧪️repluserfacingsuggestiondetail/🟦️.tsx`) | 940 | 7 | 202 | split | registration trace identical (87 events), tsdiff clean |
| `new` (`🏭️factory/🏗️pipeline-resources-construction/🦀️.rs`) | 938 | 66 | 191 | split | Rust: rustfmt parse, inline-capture audit (rscap), helper-name collision grep, shape check; cargo not run; oracle5 clean; phase struct named `UiPipelineSet` because `UiPipelines` already exists in `execution` |
| `(callback of describe.if(testLevelAtLeast("long")))` (`🔬️workspace-contract/🧱️taxonomy-normalization/🟦️.ts`) | 933 | 9 | 163 | split | lossless token expansion, tsdiff clean, 38 tests register before and after |
| `check_catalog_integrity` (`📚️document-checks/🗃️catalog-integrity/🦀️.rs`) | 908 | 48 | 204 | split | Rust: rustfmt parse, inline-capture audit (rscap), helper-name collision grep, shape check; cargo not run; oracle5: only extern-crate path uses new (8x) |
| `planArtifactCratesOnce` (`🗺️plan/🧮️pack/🟦️.ts`) | 899 | 899 | - | **remaining** | see Open items |
| `run` (`📦️packages/🟦️typescript/📜️script.ts`) | 890 | 890 | - | **remaining** | see Open items |
| `applyTaxonomyPlan` (`🧹️normalization/🚀️apply/🟦️.ts`) | 870 | 870 | - | **remaining** | see Open items |
| `evaluate_member` (`⚖️checks/📋️member-evaluation/🦀️.rs`) | 856 | 70 | 237 | split | Rust: rustfmt parse, inline-capture audit (rscap), helper-name collision grep, shape check; cargo not run; oracle5: only extern-crate path uses new (4x), orig-only E0689 inference errors vanish |
| `paint_node_step_with_driver` (`🖌️paint/🎨️node-painting-with-driver/🦀️.rs`) | 834 | 86 | 164 | split | Rust: rustfmt parse, inline-capture audit (rscap), helper-name collision grep, shape check; cargo not run; oracle5 only new extern paths |
| `expand` (`🔣️json/📖️expand-document/🦀️.rs`) | 811 | 246 | 56 | split | Rust: rustfmt parse, inline-capture audit (rscap), helper-name collision grep, shape check; cargo not run; oracle5 clean; 26 arms extracted, short arms stay inline |
| `evaluate_wall` (`⚖️masonry/🧱️evaluate-wall/🦀️.rs`) | 802 | 33 | 340 | split | Rust: rustfmt parse, inline-capture audit (rscap), helper-name collision grep, shape check; cargo not run; oracle5 clean |
| `(callback of describe)` (`🧪️tests/🧶️node-graph-hosts/🟦️.ts`) | 780 | 7 | 249 | split | lossless token expansion, tsdiff clean |
| `useShellActions` (`🏛️ShellHost/🖱️shell-actions/🟦️.ts`) | 773 | 773 | - | **remaining** | see Open items |
| `(callback of describe)` (`🧪️tests/📞️app-channel-client/🟦️.ts`) | 765 | 7 | 224 | split | lossless token expansion, tsdiff clean |
| `registerRegistryTests` (`🧪️semio-tech-cad-js-core-interactions/🗂️registries/🟦️.ts`) | 760 | 4 | 233 | split | registration trace identical (40 events), tsdiff clean |
| `plugin_exchange` (`🔄️exchange/🔀️plugin-exchange/🦀️.rs`) | 759 | 759 | - | **remaining** | see Open items |
| `parseTaxonomy` (`🧹️normalization/🔣️taxonomy/🟦️.ts`) | 757 | 21 | 166 | split | function-level old-vs-new differential of `loadTaxonomy` with 150 mutations, tsdiff clean |
| `registerResidentPoolAndMetadataTests` (`🧪️tests/🧠️resident-pool-and-metadata/🟦️.tsx`) | 756 | 6 | 150 | split | registration trace identical (80 events), tsdiff clean |
| `usePluginLifecycle` (`🏛️ShellHost/🚀️plugin-lifecycle/🟦️.ts`) | 751 | 13 | 204 | split | psplit2 phases, tsdiff clean, comment-check clean |
| `(callback of describe)` (`🧪️tests/🔒️shell-option-locks/🟦️.ts`) | 746 | 8 | 189 | split | lossless token expansion, tsdiff clean |
| `check_member` (`⚖️checks/🧱️member-checks/🦀️.rs`) | 735 | 73 | 114 | split | Rust: rustfmt parse, inline-capture audit (rscap), helper-name collision grep, shape check; cargo not run; oracle5 clean |
| `evaluate_document` (`⚖️checks/🩺️document-evaluation/🦀️.rs`) | 714 | 35 | 101 | split | Rust: rustfmt parse, inline-capture audit (rscap), helper-name collision grep, shape check; cargo not run; oracle5 clean |
| `createGisMapInferenceWorkerV1` (`💡️inference/👷️worker/🟦️.ts`) | 710 | 710 | - | **remaining** | see Open items |
| `useDockAssembly` (`🏛️ShellHost/🏗️dock-assembly/🟦️.ts`) | 710 | 16 | 260 | split | psplit2 phases, tsdiff clean, comment-check clean |
| `step` (`🪜️solve-step/🍓️staged-step/🦀️.rs`) | 709 | 182 | 62 | split | Rust: rustfmt parse, inline-capture audit (rscap), helper-name collision grep, shape check; cargo not run |
| `useFocusAndInput` (`🏛️ShellHost/🎮️focus-and-input/🟦️.tsx`) | 689 | 19 | 245 | split | psplit2 hook phases (types from producers), tsdiff clean, comment-check clean |
| `loadPluginModule` (`🐚️plugin-bridge/🔌️plugin-module-loader/🟦️.ts`) | 688 | 688 | - | **remaining** | see Open items |
| `(callback of describe)` (`🧪️tests/🫥️decorative-layer/🟦️.tsx`) | 679 | 8 | 137 | split | lossless token expansion, tsdiff clean |
| `(callback of describe)` (`🧪️tests/🤝️integration/🟦️.ts`) | 675 | 93 | 255 | split | lossless token expansion, tsdiff clean |
| `createBrowserBundleTests` (`🧪️tests/🌐️browser-bundle/🟦️.ts`) | 669 | 669 | - | **remaining** | see Open items |
| `useApplyHostEffects` (`🏛️ShellHost/⚡️host-effects/🟦️.ts`) | 663 | 663 | - | **remaining** | see Open items |
| `registerExtensionInvocationWireTests` (`🧪️tests/📬️extension-invocation-wire/🟦️.tsx`) | 657 | 6 | 214 | split | registration trace identical (33 events), tsdiff clean |
| `step` (`💧️hydration/🚶️hydration-step/🦀️.rs`) | 650 | 50 | 40 | split | Rust: rustfmt parse, inline-capture audit (rscap), helper-name collision grep, shape check; cargo not run; oracle5: only extern-crate path uses new (6x) |
| `run` (`🎛️commands/✅️verify/🟦️.ts`) | 647 | 656 | - | **remaining** | see Open items |
| `Mode` (`🧱️elements/🎨️Canvas/🟦️.tsx`) | 646 | 77 | 151 | split | psplit2 hook phases (arrow component), tsdiff clean, comment-check clean; no runnable owning test (`@dnd-kit/core` absent) |
| `shardWorkerSource` (`🏗️materialization/🧵️shard-worker-source/🟦️.ts`) | 642 | 3 | 254 | split | exact string equality old vs new, tsdiff clean; three dependent tests re-pointed |
| `interactivityMountedFrameTransactionFailures` (`🖥️mounted/🎞️frame-transaction/🟦️.ts`) | 642 | 14 | 191 | split | function-level old-vs-new differential, tsdiff clean |
| `(callback of describe.skipIf(!HUB_E2E))` (`🧪️tests/🤝️two-client-document/🟦️.ts`) | 637 | 637 | - | **remaining** | see Open items |
| `useBackboneWorker` (`🏛️ShellHost/🧵️backbone-worker/🟦️.ts`) | 635 | 635 | - | **remaining** | see Open items |
| `(callback of describe)` (`🧪️tests/🛣️workflow-flow-routing/🟦️.ts`) | 634 | 56 | 182 | split | lossless token expansion, tsdiff clean |
| `(callback of describe)` (`🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts`) | 632 | 7 | 168 | split | lossless token expansion, tsdiff clean |
| `useIdentityAndDocumentPorts` (`🏛️ShellHost/🔒️identity-document-ports/🟦️.ts`) | 626 | 37 | 287 | split | psplit2 hook phases, tsdiff clean, comment-check clean; 5 exported types moved to `identity-port-types`, 5 importers re-pointed |
| `dispatch_action` (`🎬️shell-actions/🎯️dispatch-action/🦀️.rs`) | 621 | 166 | 22 | split | Rust: rustfmt parse, inline-capture audit (rscap), helper-name collision grep, shape check; cargo not run |
| `useThemeDriverSettings` (`🏛️ShellHost/🎨️theme-driver-settings/🟦️.tsx`) | 620 | 11 | 191 | split | psplit2 hook phases (types from producers), tsdiff clean, comment-check clean |
| `registerProjektetageDeckTests` (`🧪️tests/🧪️projektetage-deck/🟦️.ts`) | 612 | 4 | 127 | split | registration trace identical (30 events), tsdiff clean, comment-check clean |
| `check_full_steel_structure` (`⚖️checks/🧱️member-checks/🦀️.rs`) | 612 | 97 | 439 | split | Rust: rustfmt parse, inline-capture audit (rscap), helper-name collision grep, shape check; cargo not run; oracle5 clean |
| `registerTests1` (`🧪️tests/🪟️tree-windows/🟦️.tsx`) | 611 | 7 | 162 | split | lossless text expansion (62 statements), tsdiff clean |
| `useHubConnection` (`🧱️elements/🔗️HubConnection/🟦️.tsx`) | 609 | 44 | 189 | split | psplit2 hook phases (types from producers), tsdiff clean, comment-check clean |
| `registerMachineTests` (`🧪️docklayoutstore/🎰️machines/🟦️.ts`) | 607 | 7 | 383 | split | registration trace identical (46 events), tsdiff clean |
| `shell_chrome_string` (`🧊️wgpu/📖️shell-vocab/🦀️.rs`) | 603 | 8 | 191 | split | all 598 (key, language) arms identical and in order (script), rustfmt parse; cargo not run |

`expand` (root 246) and `dispatch_action` (root 166) stay larger than the other roots because their short match arms intentionally stay inline.

## Verification

- TS test registrations (`register*Tests`, describe callbacks): recording vitest proxy (`run-reg.sh`, `cmp-reg.py`, `reg-trace.test.ts`) compares test names, nesting and a hash of every callback source before and after: identical for gis-inference (27 events), cad repl (87), cad registries (40), extension-invocation-wire (33), resident-pool (80), docklayout machines (46), projektetage deck (30). Module-level describe callbacks use `verify-expand.cjs` (lossless token expansion of the new tree back to the original text) or `verify-text2.cjs` (tree-windows, 62 statements).
- TS hooks and functions: `tsdiff.cjs` compares the distinct compiler diagnostics of the original snapshot against the new tree (React types via `TSDIFF_REACT=1`): clean for every TS split. `comment-check.cjs` proves every original comment line is still present (header blocks and region markers of 9 roots and the `Mode` root were restored by hand after the tool dropped them).
- TS function-level differentials: `loadTaxonomy` of `parseTaxonomy` (150 mutations), frame-transaction old vs new, `shardWorkerSource` exact string equality.
- Rust: `rustfmt --check`-style parse of all 73 new or changed files in the Rust splits (`rustfmt --edition 2021 --emit stdout`), `rscap.py` (inline format capture audit: `{name}` in format strings is invisible to a token lexer; 0 findings in 73 files after the lexer fix), `collide.py` (helper name collision grep), `shape.sh` (path length <= 255 bytes, folder emoji equals module doc emoji, taxonomy resolution, unique doc emoji), `ptrarg.py` (63 `&String`/`&Vec<T>` parameters rewritten to `&str`/`&[T]` for `clippy::ptr_arg`, `.clone()` on a former `&String` became `.to_owned()`).
- Rust name resolution oracle (`rsora.py`, rustc `--emit=metadata` on the original and on the new tree, extern crates absent so only resolution is compared): CLEAN for en1993 member checks, en1996 evaluate-wall, en1999 member checks, din18599 document evaluation, layout export `expand`, wgpu factory. Differences limited to extern-crate path uses that the sandbox cannot resolve (which exist identically in the real build): en1992 member evaluation (4x), vdi3805 catalog integrity (8x), en1998 full seismic (15x), hydration (6x), wgpu paint. The oracle found and I fixed: missing type paths in new signatures (`ConcreteGrade`, `AlloyProps`, `SectionClass`, `GoverningEffects`, `SecurityLimits`, `BucklingCurve`) and a struct name clash (`UiPipelines`, renamed `UiPipelineSet`).
- Rust `shell_chrome_string`: a script compared the 598 `(key, is_de) => text` arms before and after: identical in content and order (keys are unique, first match order is preserved by the chained `or_else`).
- `cargo` was NOT run (queue busy with other agents' checks and the dependency sync). No Rust test was run. Every Rust claim above is static.
- Owning tests: `bun test ./<file>` was only possible where dependencies exist; `Canvas/🧪️tests/🎯️stack-drop-destination` passes (14), the Window component test and the `hub-session-capability` test (gated, 0 tests ran) could not run here (`@dnd-kit/core` absent). I did not claim runtime behaviour for them.

## Changes (files)

Roots that lost their long body (children are the new folders next to each root, one `🟦️.ts(x)` or `🦀️.rs` per folder):
- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🧪️projektetage-deck/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🎞️visual-job/🪜️solve-step/🍓️staged-step/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🧪️tests/🧪️repluserfacingsuggestiondetail/🟦️.tsx`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/🗂️registries/🟦️.ts`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🩺️document-evaluation/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📋️member-evaluation/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📚️document-checks/🗃️catalog-integrity/🗃️product-and-index-checks/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📚️document-checks/🗃️catalog-integrity/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🔗️special-element-checks/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧱️member-checks/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚖️masonry/🧱️evaluate-wall/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧱️member-checks/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌍️full-seismic-check/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌍️full-seismic-check/🧱️structural-system-checks/🦀️.rs`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/💡️inference-port/🟦️.ts`
- `🌎️hub/🧪️tests/🤝️integration/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🎨️node-painting-with-driver/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🔧️pipeline/🏭️factory/🏗️pipeline-resources-construction/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx`
- `🧰️framework/🛍️products/🐾️pets/🧪️tests/🫥️decorative-layer/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🚶️hydration-step/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📖️shell-vocab/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📬️extension-invocation-wire/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔒️shell-option-locks/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🛣️workflow-flow-routing/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧶️node-graph-hosts/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎨️theme-driver-settings/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎮️focus-and-input/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🏗️dock-assembly/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔒️identity-document-ports/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🚀️plugin-lifecycle/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🧠️resident-pool-and-metadata/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️HubConnection/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🪟️tree-windows/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🧪️tests/📞️app-channel-client/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🖥️mounted/🎞️frame-transaction/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🧱️taxonomy-normalization/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔣️taxonomy/🟦️.ts`
- `🧰️framework/🧪️tests/🧪️docklayoutstore/🎰️machines/🟦️.ts`

Other edited files:
- ShellHost importers of the five types that moved to `🔒️identity-document-ports/🧰️identity-port-types`: `📬️artifact-opening`, `🖱️shell-actions`, `✅️check-in-sync`, `🗄️shell-uri-documents`, `🎮️focus-and-input/⌨️registers-and-context-menu` (all `🟦️.ts(x)`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🎟️hub-session-capability/🟦️.ts`: the ShellHost source reader now uses `readModuleTreeSource` per listed file so the children are part of the text.
- Earlier in the run (before the freeze): the `🎯️tool-job-coverage/⚙️fixed-operation` import, the interactivity-audit arm scopes in `paintNodeBoundary`, `more-work-drive`, `runtime-admission` and `bootstrap` tests re-pointed after the shard-worker-source split, `os-dev-composition-ownership` fixture and the dev `📋️project.json` input glob, the draw `🧪️tests/🔣️.json` and `🧬️schema/🔣️.json`.
- Tools and inputs only under `C/fnsplit2-tools/` (new this stretch: `rscap.py`, `collide.py`, `ptrarg.py`, `stats.ts`, `unused_ins.py`, `shell-vocab-split.py`, specs `en1992-member`, `en1993-steel`, `en1996-wall`, `en1998-seismic`, `en1998-systems`, `en1999-member`, `din18599-doc`, `vdi3805-catalog`, `vdi3805-products`, `layout-expand`, `hydration`, `canvas-mode`, `projektetage-deck`, plus the `rsfn.py`/`psplit2.cjs` extensions listed below). Snapshots of the originals: `C/fnsplit2-tools/orig/`.
- Freeze: nothing under `📚️library`, the root `📜️script.ts`, any `🔣️taxonomy.json`, `package.json`, `bun.lock` was edited after the freeze message (the three library files listed above were split before it).

## Tool defects found and fixed (worth knowing for the next split)

- `rsfn.py` ignored inline format captures (`{name}`): a phase could silently lose a parameter. `used_in` now reads captures of format macros; `rscap.py` audits finished files.
- `psplit2.cjs` parsed import-pruning text as `.ts` even for `.tsx`, which dropped imports used only inside JSX. Fixed (`ScriptKind` follows the file).
- New: `psplit2.cjs` splits `const X: React.FC = (…) => {…}` (`arrowVar`), may move exported types to the shared file (`allowExportedMoves`; importers must be re-pointed by hand); `rsfn.py` supports `drop_ins` (shadowed names look like uses), `ensure_semicolon`, a `where` clause on method impl headers and a `prelude_if` line.
- Lifetime trap: a returned `&str` from a two-reference helper needs a lifetime; keep such locals in the root (done for `article` in the vdi3805 product loop).
- Path length: three folder families exceeded 255 bytes and were renamed (`dispatch-action` children, vdi3805 product children).

## Open items (13 functions still over 600)

Freeze zone (not touched, same tool chain applies once lifted): `planArtifactCratesOnce` (899), `run` of `📦️packages/🟦️typescript/📜️script.ts` (890), `applyTaxonomyPlan` (870), `run` of `🎛️commands/✅️verify` (656).

Not a pure extraction, each needs a design decision:
- `registerGisDocumentOpenTests` (941): one 839-line `it(...)` with six mutable `let` locals crossing a `try`; needs a state holder object per phase.
- `useShellActions` (773): the 669-line `onAction` callback is a `try` whose body is a ladder of `if (action.action === X) { … return …; }` blocks over ~40 closure variables. Method: extract each always-returning block as `function handler(ctx, …)` with the context typed from the checker and `return await handler(ctx)` inside the `try`.
- `useApplyHostEffects` (663): a `for` over effects with `continue` ladders and a mutated `nextViewState`; `useBackboneWorker` (635): a 500-line `worker.onmessage` handler; same method as `onAction` or a per-kind handler table.
- `createGisMapInferenceWorkerV1` (710) and `loadPluginModule` (688): closure factories with mutually calling inner functions over shared state; need an explicit state record and late-bound handler maps.
- `createBrowserBundleTests` (669): mutually recursive inner functions (an attempt was reverted).
- Hub `two-client-document` e2e describe (637): one 600-line `it`.
- `plugin_exchange` (Rust, 759): async, `?` and `return Ok(PluginExchangeOutput { … })` inside a 540-line `if let`; needs a `Flow::Done(out) | Flow::Continue(state)` return type per phase, which is not a pure extraction.
- Still large helpers worth a second pass: `check_full_steel_structure` `check_uls_actions` (408 lines), `check_load_cases` of en1996 (321), `check_member_ductility` of en1998 (257).

Next steps: run `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- test -p <crate> --lib` for `semio-s-artifact-norm-en1992-checks`, `-en1993-checks`, `-en1996-schema`, `-en1998-inferences`, `-en1999-checks`, `-din18599-checks`, `-vdi3805-inferences`, `semio-s-artifact-layout-layout-engine-json`, `semio-framework-os-store-document`, `semio-framework-os-renderer-wgpu` once the queue is free (not run here).
