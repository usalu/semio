# 🧭 Wrong-Direction Relocation Brief (follow-up 3, 2026-10-15)

Shared rules for the executors that relocate the wrong-direction code the monolith-split fleet grouped but did not move (`📓️SUMMARY.md`, follow-up 3). These come on top of `/Users/ueli/Documents/semio/AGENTS.md`, `📓️EXECUTOR-RULES.md`, `📓️RESOLUTION-ORACLE.md` and rule 7 of `📓️CARVING-RULES.md`.

## Slices and ownership

| Slice | Owner prefix | Deliverable | Owns |
| --- | --- | --- | --- |
| A manifest product vocabulary | `relo-manifest` | `RESULT-RELOCATE-MANIFEST-VOCABULARY.md` | `🧰️framework/🔨️modules/🛂️manifest/**` (Rust + TS twin + schema), `🏗️builder/🧩️app` injection sites in the plugin SDK, the mcp catalog injection site, callers of moved names |
| B tiled-map GIS policy | `relo-tiledmap` | `RESULT-RELOCATE-TILED-MAP-GIS.md` | `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/**`, the GIS plugin map host and its consumers |
| C hub GIS map binding | `relo-hubgis` | `RESULT-RELOCATE-HUB-GIS-BINDING.md` | `🌎️hub/💡️inference/**`, the hub bootstrap GIS call sites, the GIS-owned destination |
| D puzzle2d naming in the infinite board | `relo-board` | `RESULT-RELOCATE-BOARD-PUZZLE2D.md` | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/**`, the puzzle 2d plugin consumers |
| E plugin SDK layering | `relo-pluginsdk` | `RESULT-RELOCATE-PLUGIN-SDK.md` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/**`, `🔌️plugin/🖥️host/**`, `⏪️time-travel`, `⏯️tool-run`, plugin and plugin-host `Cargo.toml`/lib roots |

Cross-slice seams:
- History-edit ids, dialog and definitions, and `tool_run_action_definitions`/`app_declares_tool_run`, belong to slice A. Slice E must not move them; it only re-reads and follows their new import path.
- Slice A does not edit `🧵️instance`/`🖥️host` beyond the import lines its own moves break. Slice E does not edit `🏗️builder`.
- Re-read every file right before each edit; other slices and peers edit concurrently.

## Design principles

1. The framework is domain-neutral. A domain feature lives in the module that implements it and reaches the framework through a framework registration point (an extension trait, a contribution row such as `TopicContribution`, an injected policy object or a parameter supplied by the host), never by the framework naming the domain.
2. Choose the owner by evidence: who implements the behaviour, who consumes it, and which layer each consumer sits in. Layers from low to high: `🧰️framework/📦️packages` and `🧰️framework/🔨️modules` (framework), `🧰️framework/🛍️products/💻️os` (os product, including the plugin SDK), `✏️s/🔌️plugins/**` (plugins), top-level products (`🌎️hub`, `♻️mit-bestand`, `🏢️semio-tech`). Dependencies point downward only. An item a framework module itself needs is not product vocabulary; move it to its semantic framework owner instead and say why.
3. Greenfield: no aliases, compatibility re-exports, deprecations or shims. Update every caller, TS twin, schema file (`🧬️schema/🔣️.json`, `.graphql`, `.proto`, `.ts`), fixture, test, story and source-text reader in the same pass.
4. The four recurring splitter defects: keep `//#region` markers balanced in every file; give moved free structs, enums and methods the visibility their users need; re-point source-text readers (`readModuleTreeSource`, `POLICY_SPLIT_SOURCE_ROOTS`, `?raw` imports, `include_str!` readers, `📜️script.ts` gates); never leave a compatibility re-export.
5. New crates follow `PRODUCER-CRATE-RECIPE.md` exactly. New folders are `<RGI emoji>️<kebab>` (emoji + U+FE0F, reused elsewhere as a folder prefix, unique among siblings), paths at most 240 UTF-8 bytes, new files start with a docstring whose first character is a unique emoji, no comments inside definitions.
6. Test-driven: every moved behaviour keeps or gains a test at its new home, and every new extension point gets a test proving a non-default implementation is honoured.

## Process

- Foreground only, no sub-agents. Never run a modifying git command. Use `/usr/bin/grep` or `git --no-optional-locks grep` (the shell `grep` alias misses emoji paths). Use `perl -e 'alarm N; exec @ARGV' -- <cmd>` instead of `timeout`.
- Logs and scratch go to `🪓️monolith-split/🗑️generated/<prefix>-*`; tools you want to keep go to `🪓️monolith-split/<prefix>-tools/`. Never delete any `🗑️generated` folder. Do not open, close or reopen tickets.
- Save a pristine copy of every file you will edit before the first edit (sandbox under the session scratchpad or `🗑️generated/<prefix>-baseline/`), because the oracle baseline needs it.

## Verification (record every command and its real result)

1. Patched full-resolution differential (`📓️RESOLUTION-ORACLE.md`) for every Rust crate you touched, in every configuration it builds for (native, `--test`, wasm targets where applicable), baseline sandbox against the current tree. No new E0412/E0425/E0422/E0423/E0432/E0603/E0616/E0624 errors. Expected differences from the move (for example an unresolved new crate name in a standalone run) must be listed and explained.
2. Definition scan for field and method privacy over the moved code, with one seeded regression proving it is sensitive.
3. `rustfmt +nightly-2026-07-20 --edition 2021 --emit stdout` parse of every touched Rust file; region balance; path length; emoji admission.
4. Direction checks:
   - dependency-cruiser on the touched TS paths before and after: `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs <paths> --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type err` (no new violation; list removed ones);
   - static Cargo direction scan: no `path =` dependency from a lower layer into a higher one in any `Cargo.toml` you touched;
   - static Rust source direction scan: no `#[path]`, `include!`, `include_str!` or `include_bytes!` from a lower layer into a higher one, and no `use` of a higher-layer crate name from lower-layer code you touched.
5. Scoped `tsc --noEmit` and the relevant vitest files wherever TS changed (failure sets identical before and after, apart from documented flakes).
6. Try `cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --message-format=short` once per touched crate. The compiler-ingress gate will most likely refuse it; report the exact outcome. Never compile around the gate.

## Deliverable

`🪓️monolith-split/RESULT-RELOCATE-<SLICE>.md` with: verdict; design decision and rationale per item (owner evidence); moved items (from → to); the registration point and its test; every caller, fixture and reader updated; verification commands with real results; residual risks and follow-ups. Confirm it exists with `ls -l` before you finish.
