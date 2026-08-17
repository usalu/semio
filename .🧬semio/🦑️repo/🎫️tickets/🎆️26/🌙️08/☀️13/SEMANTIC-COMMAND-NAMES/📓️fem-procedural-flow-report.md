# Fem / Procedural / Flow — semantic command folders

Ticket: `26/08/13/SEMANTIC-COMMAND-NAMES`. Scope: `🏗️fem`, `🌀️procedural`, `🌊️flow` only. Writer was not touched.

## Result

Every `🎮️commands/<folder>/` under these plugins is one command. Folder slugs are verb-noun kebab (or a single verb) matching the inner payload module. Payload structs sit at file top level. Noun buckets (`🗂️selection`, `🗣️locale`, `🎥️camera`, `🕸️graph`, `🕸️node-graph`, `🧬️generation`, `🏋️loads`, `🧱️model`, …) are gone.

Glue `pub mod` names match the slug (`set_locale`, `add_node`, `node_graph_edit`). Struct names, `#[dsl(keyword)]`, and `app_commands!` action ids are unchanged.

## Counts

| Plugin | Old grouped files | New command folders |
| --- | ---: | ---: |
| 🏗️fem | 15 | 37 |
| 🌀️procedural | 17 | 65 |
| 🌊️flow | 13 | 42 |
| **Total** | **45** | **144** |

Flow includes five generate-mode commands under `🎭️modes/🧬️generate/🎮️commands/`.

Mapping (paths only): `scratch-fem-procedural-flow-mapping.json`.

## Representative splits

- 🏗️fem `🧱️model` → `add-node`, `add-bar`, `add-beam`/`add-frame`, `add-material`, `add-section`, `add-support`, `add-region`/`add-solid`
- 🏗️fem `🏋️loads` → `add-nodal-load`, `add-member-udl`, `add-area-load`, `add-load-case`, `add-combination`, `set-self-weight`
- 🏗️fem `🗣️locale` → `set-locale`; `🎥️camera` → `set-camera`; `🗂️selection` → `remove-selection`
- 🌀️procedural `🕸️graph` → `node-graph-edit`, `move-media-node`, `reorganize`, `node-graph-viewport`, `node-graph-select`, `node-graph-hover`, (`connect-media-ports` 2d / `graph-pointer-down` 3d)
- 🌊️flow `🕸️node-graph` → `node-graph-edit`, `spotlight-commit`; `🗣️locale` → `set-locale`; `🧮️eval` → `evaluate`, `flow-eval-tick`, `flow-eval-resolve`

## Glue and imports

Updated `📦️glue.rs` in each plugin crate. App `use crate::apps::<app>::commands::{…}` lists the flat command modules (writer pattern). Flatten `use selection::set_selection` / `use eval::{…}` blocks removed.

Cross-file helpers kept as sibling-module imports (not duplicated):

- `evaluate::evaluate_result`, `reorganize::reorganize_operations` (extension commands)
- `run_extension_action::FLOW_AUTOMATIONS` (catalogue panel)

## Follow-up fixes after the mechanical split

1. A global folder-name replace corrupted two glue paths (`🕸️graph` prefix → `🕸️node-graph-edit-pointer-down`; `🧮️eval` prefix of `evaluate` → `set-eval-outputsuate`). Both restored; all `#[path]` targets under these plugins exist.
2. The splitter dropped the tail of multi-line `use foo::{ … };`. Reconstructed in 3 gumball commands (procedural3d) and 5 generate-mode commands (flow).

## cargo check

`--offline`, workspace packages:

| Crate | Log | Result |
| --- | --- | --- |
| `semio-s-plugin-fem` | `scratch-fem-cargo-check.txt` | **ok** (warnings only; unused copied load helpers) |
| `semio-s-plugin-procedural` | `scratch-procedural-cargo-check.txt` | **ok** |
| `semio-s-plugin-flow` | `scratch-flow-cargo-check.txt` | **ok** |

No git modifying commands were used.
