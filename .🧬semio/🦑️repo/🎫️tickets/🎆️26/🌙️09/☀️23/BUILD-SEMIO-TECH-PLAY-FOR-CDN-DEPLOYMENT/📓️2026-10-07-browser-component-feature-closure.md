# Browser Component Feature Closure

The source and independent Cargo feature audit passed on 2026-10-07: **43 composition owners covering all 148 authored Play panes; every Cargo oracle exited 0; zero app feature omissions; zero feature mismatches**. No component source repair is warranted by this audit. This proves build inputs and feature resolution; fresh descriptor publication and browser execution remain separate gates.

The materializer in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📦️materialization/🟦️.ts` reads the selected owner package and calls `pluginCargoArgs(packageName, profile, join(repoRoot, target.cratePath, "Cargo.toml"))`. The plan in adjacent `📋️plan/🟦️.ts` emits `cargo rustc --manifest-path <owner> -p <owner-package> --target wasm32-wasip2 --profile wasm-release`, preserving default features. It neither compiles the standalone artifact test manifest nor disables owner defaults.

The focused WFC concern is resolved at the correct owner: `🌎️hub/🧩️compositions/🀄️wfc/📦️packages/🦀️rust/Cargo.toml` explicitly enables `component-app-assembly` on bitmap, grid2d, 2d, grid3d, and 3d artifact dependencies. Its root app enum directly names all five editor/viewer pairs, and its default `plugin-entry` exports that enum. Independent Cargo tree output confirms `component-app-assembly,default` on bitmap and the same assembly gate on the other four. The standalone bitmap package's empty default feature list therefore does not remove apps from the browser component.

Stdio follows the same ownership rule: its main default `plugin-root` reaches `component-app-assembly`, which activates seven artifact package app features for nine text/data panes; its nine family composition owners enable their own artifact app features directly. Codec and conversion dependencies do not require their own UI app assembly merely because their artifact package offers that feature. The audit distinguishes actual editor/viewer references from those dependencies.

The independent oracle uses Cargo's actual feature resolver with target `wasm32-wasip2`, normal dependency edges, and the workspace's resolver version. Each generated observer manifest depends on the actual owner path with its defaults intact; this isolates that owner's closure from unrelated workspace members and keeps Cargo lock updates entirely under the ticket's generated directory. It does not alter source manifests or shared lock files and compiles no Rust code. The initial shared-lock attempt was blocked by `--locked`; a subsequent alternate-lock experiment failed because this Cargo tree does not support `--lockfile-path`. Neither failed experiment is counted as proof.

Reproducible input: `feature-audit/📜️script.ts`, run using `NX_TUI=false NX_DAEMON=false bun nx exec --projects=@semio-tech/semio-tech-play --excludeTaskDependencies -- bun <absolute-ticket>/feature-audit/📜️script.ts`. Successful output: `🗑️generated/browser-feature-closure-complete.log`; details: `🗑️generated/browser-feature-closure.json` and `🗑️generated/feature-oracles/*.txt`. The prior all-pane contract catalog ties every pane to the audited owner.

The retained probe was subsequently made independent of that generated catalog: it now rebuilds pane ownership from the source Play runtime JSON and hub workspace member manifests' actual playground metadata, rejecting duplicate or missing owners. This zero-touch version also passed all 43 Cargo oracles covering 148 panes with no omissions or mismatches, log `🗑️generated/browser-feature-closure-zero-touch.log`. Its launch can rerun after the ticket's generated output has been deleted.

“Assembly dependencies” below counts directly declared artifact dependencies whose resolved app assembly feature is enabled; zero means the owner's apps do not use that separate feature gate. It does not mean the owner has no apps.

| Composition Package | Play Panes | Assembly Dependencies | Cargo Exit |
| --- | ---: | ---: | ---: |
| semio-hub-cad | 1 | 0 | 0 |
| semio-hub-procedural | 2 | 2 | 0 |
| semio-hub-flow | 1 | 0 | 0 |
| semio-hub-lowpoly | 1 | 0 | 0 |
| semio-hub-remodel | 1 | 0 | 0 |
| semio-hub-draw | 1 | 0 | 0 |
| semio-hub-raster | 1 | 0 | 0 |
| semio-hub-layout | 1 | 0 | 0 |
| semio-hub-shooting | 1 | 0 | 0 |
| semio-hub-puzzle | 3 | 3 | 0 |
| semio-hub-block | 3 | 3 | 0 |
| semio-hub-wfc | 5 | 5 | 0 |
| semio-hub-fem | 2 | 2 | 0 |
| semio-hub-energy | 1 | 0 | 0 |
| semio-hub-process | 1 | 0 | 0 |
| semio-hub-sourcing | 1 | 0 | 0 |
| semio-hub-gis | 2 | 2 | 0 |
| semio-hub-architect | 1 | 0 | 0 |
| semio-hub-norm | 15 | 0 | 0 |
| semio-hub-note | 1 | 0 | 0 |
| semio-hub-writer | 1 | 0 | 0 |
| semio-hub-forms | 1 | 0 | 0 |
| semio-hub-mathematical | 1 | 0 | 0 |
| semio-hub-reasoning | 1 | 0 | 0 |
| semio-hub-dag | 1 | 0 | 0 |
| semio-hub-imperative | 1 | 0 | 0 |
| semio-hub-playbook | 1 | 0 | 0 |
| semio-hub-trinity | 2 | 2 | 0 |
| semio-hub-stdio | 9 | 7 | 0 |
| semio-hub-stdio-image | 11 | 6 | 0 |
| semio-hub-stdio-media | 4 | 4 | 0 |
| semio-hub-stdio-cad | 10 | 3 | 0 |
| semio-hub-stdio-bim | 6 | 2 | 0 |
| semio-hub-stdio-mesh | 5 | 5 | 0 |
| semio-hub-stdio-pdf | 10 | 1 | 0 |
| semio-hub-stdio-office | 9 | 3 | 0 |
| semio-hub-stdio-semio | 19 | 1 | 0 |
| semio-hub-stdio-binary | 5 | 4 | 0 |
| semio-hub-animate | 1 | 0 | 0 |
| semio-hub-sequence | 1 | 0 | 0 |
| semio-hub-vcs | 1 | 0 | 0 |
| semio-hub-demonstrator | 1 | 3 | 0 |
| semio-hub-space | 2 | 2 | 0 |
