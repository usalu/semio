# Norm component: generated inputs for a native wasm build

Read-only check on 2026-11-17. Repo root: /Users/ueli/Documents/semio.

## Build path
- Crate: `🌎️hub/🧩️compositions/📕️norm/🧩️component/📦️packages/🦀️rust/Cargo.toml` (cdylib, `[lib] path = "../../🦀️.rs"`). The folder has no `📋️project.json`.
- The `component` target is not in any project.json. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧩️component-targets/🟨️.mjs` (`componentBuildTargets`) builds it: `command: repo "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts" component --manifest <crate>/Cargo.toml`, cwd `.`.
- Its dependsOn: `@semio-tech/os-plugin-describe-rs:build`, `@semio-tech/framework-plugin-web:support`, `workspace:deps-wasm-opt`. Its owner inputs `🧩️component/🛂️.descriptor.semio` and `🧩️component/🔣️.json` both exist.
- Hub norm `📦️packages/🦀️rust/📋️project.json` has no `component` target. Its only generate-style target is `mutation-leaf-taxonomy-generate`.

## Closure
- Cargo closure (path and `workspace = true` deps resolved through `🌎️hub/Cargo.toml`): 746 manifests, 264 of them norm crates.
- No `build.rs` and no `OUT_DIR` use anywhere in that closure.
- Source closure walked from each `[lib]` root through `mod`, `#[path]` and `include*!`: about 8,950 files. No norm-owned include target is missing.
- No path under `📕️norm` or `✏️s/🔌️plugins/📕️norm` contains `🤖️generated`.

## Generated inputs the component closure reads (all exist)
| Generated path | Read by | On disk | Producer (cwd) | Exact command |
|---|---|---|---|---|
| `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🪪️icon-name/🦀️.rs` | `🖼️assets/🔣️icons/🧬️schema/🦀️.rs` (`#[path]`) | yes, Nov 14 13:16 | `@semio-tech/assets:build` (`🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript`) | `repo ./📜️script.ts build` |
| `…/🖼️assets/🔣️icons/🤖️generated/🔤️shortcodes/🦀️.rs` and `…/🔤️shortcodes/🔏️digest/🦀️.rs` | `🖼️canvas/🔌️runtime-assets/🦀️.rs` | yes, Nov 14 13:16 | same assets target | same |
| `🧰️framework/🔨️modules/🧬️schema/🤖️generated/🏷️entity-kinds/🦀️.rs` | `🧬️schema/⚛️component/🦀️.rs` | yes, Nov 14 13:16 | schema rust `generate` (`🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust`) | `repo ./📜️script.ts generate` |
| `🧰️framework/🔨️modules/🖱️ui/🎚️axes/🤖️generated/🦀️.rs` | `🖱️ui/🌐️locale/🦀️.rs` | yes, Nov 14 11:38 dir | ui rust `generate` (`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust`); bootstrap fallback in `🚀️bootstrap/🎚️ui-axes/🟦️.ts` | `repo ./📜️script.ts generate` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️generated-test-contracts/🦀️.rs` | plugin `🦀️.rs` | yes | checked-in test, no generator | none |

Not needed by the closure: the other icon outputs (`🖼️icons`, `🖼️icon_svgs`), `🌱️metabolism/🔣️icons/🤖️generated/*` (exist), and `🛂️manifest/🤖️generated/🎚️ui-axes/🟦️.ts` (TypeScript, exists).

## Norm-specific generated input
- `✏️s/🔌️plugins/📕️norm/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json`: exists (Nov 13 20:40). Producer: hub norm project target `mutation-leaf-taxonomy-generate` (cwd `🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust`), command `repo ./📜️script.ts mutation-leaf-taxonomy-generate`. Read only by the TypeScript test `🌎️hub/🧩️compositions/📕️norm/🧪️tests/🔬️rust-package-commands/🟦️.ts`, not by the Rust build.

## Missing generated paths
- In the component closure: none.
- Repo-wide `🤖️generated` includes that are missing (`normcomp-missing-generated.py`, 22 rows, none under norm): writer, reasoning, trinity, draw, puzzle, hub stdio/flow.
- Test-only and outside the closure: `🧰️framework/🔨️modules/🎭️actor/🤖️generated/🎭️actor/🟦️.ts` (actor typegen test) and `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` (ui projection test).

## Ticket notes
- `normcomp.md` Finding 1 (missing icon-name file) is resolved: the file exists.
- `normcomp-generate-icons.ts` (icon outputs) and `normcomp-generate-more.ts` (entity-kinds, metabolism icons) produced outputs that are still present. Timestamps are Nov 14 13:16, later than the ticket's Nov 13 notes. I did not byte-compare them with a fresh run.
- `normcomp-check.sh` still points at `repo cargo`. The `component` driver is the path the graph uses.

## Caveats
- Playground catalog `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json` exists. It is read only by the plan's variant lookup, not the component cargo pass. Its generator was not traced.
- The source walker treats `🦀️.rs` as a mod-rs file. Its "missing" entries were checked: the 10 norm entries are `#[cfg(test)]` modules with `#[path]`, and the framework entries are not generated.
