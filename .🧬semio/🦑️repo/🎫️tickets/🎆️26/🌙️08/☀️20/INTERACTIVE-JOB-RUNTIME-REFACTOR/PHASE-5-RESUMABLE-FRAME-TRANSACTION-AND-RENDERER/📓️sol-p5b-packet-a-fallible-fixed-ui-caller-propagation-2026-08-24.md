# P5b Packet A Fallible Fixed UI Caller Propagation

Date: 2026-08-24

## Outcome

Packet A's authoritative 107-file producer census is source-audit-ready for the admitted P5b UI boundary. Every live `ActionFactory::action` caller in the census now supplies `Option<UiValue>` and propagates `UiAssemblyResult`; explicit and inferred UI child/action/section staging uses fixed owners with fallible admission; scene encode boundaries return assembly failure; and the four Space home TableWindowKit callers use the frozen consuming, fixed/fallible API.

No core contract/runtime/reconcile/reactor file, renderer Packet B file, root `📜️script.ts`, P2 construction/job region, or stdio/oracle region was intentionally changed by this packet.

## Exact Working-Tree Inventory

The shared working tree contains Packet A changes in 72 of the authoritative 107 files. The other 35 census files required no source edit after their direct boundary was checked.

- Writer: `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️component.rs`.
- Procedural: `✏️s/🔌️plugins/🌀️procedural/🦀️component.rs`; and the `🌀️procedural2d` and `🧪️procedural3d` editor panel files `📄️artifact`, `🔍️inspection`, and `🛍️catalogue` under their canonical artifact roots.
- Flow: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/{📄️artifact,🛍️catalogue}/🦀️component.rs`.
- GIS: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs` and `📌️panels/{📄️artifact,🛍️catalogue}/🦀️component.rs`.
- VCS: `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️component.rs`.
- Animate Present: `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️component.rs`.
- Shooting: the canonical editor root plus `📌️panels/{📄️artifact,🛍️catalogue}/🦀️component.rs` under `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.
- Sequence: `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/{📄️artifact,🛍️catalogue}/🦀️component.rs`.
- FEM: `✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/🦀️component.rs`.
- Architect: the canonical editor `🎨️chrome`, document and catalogue panels, and edit windows `↔️adjacency`, `📄️report`, and `🧭️trace` under `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.
- Process3d: the canonical editor root plus `📌️panels/{📄️artifact,🛍️catalogue,🛠️workshop}/🦀️component.rs` under `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧪️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.
- Lowpoly: the canonical editor root plus document, layers, and catalogue panels under `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.
- Wires: document and catalogue panels under `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.
- Forms: document and catalogue panels under the canonical Forms editor root.
- Layout: the document, preflight, and catalogue panels under the canonical Layout editor root.
- CAD: the canonical editor root and its document and catalogue panels.
- Imperative: document, inspection, and catalogue panels under the canonical Imperative editor root.
- Trinity: Rewrite document/catalogue and Jack document/catalogue panels under their canonical editor roots.
- DAG: document and catalogue panels under the canonical DAG editor root.
- Draw: layers and catalogue panels under the canonical Draw editor root.
- Raster: document and masks panels under the canonical Raster editor root.
- Note: the canonical Note document panel.
- Puzzle: Puzzle2d edit mode plus document/catalogue panels; Puzzle5d editor root plus document/catalogue panels; Puzzle3d document/catalogue panels.
- Block: Block2d, Block5d, and Block3d document panels.
- Space: engine catalogue, Home editor main window, and Space members panel.

## Ownership and Close Table

| Boundary owner | Populated authority | Refusal, stale, cancel, or fault close |
|---|---|---|
| Tree and panel render helpers | `UiFixedList<BuiltNode>` admitted with `try_push` | The enclosing `UiAssemblyResult` returns the exact admission error; no empty/default replacement is constructed. |
| Action bindings | `(ActionId, Option<UiValue>)` from `ActionFactory` | Text/list/map admission or factory refusal propagates through the current render owner. |
| Catalogue drag payloads | `UiFixedMap<UiText>` | MIME key, payload, and map admission failure returns from the populated catalogue owner. |
| Scene render boundaries | Encoded fixed scene props | Encode refusal returns `PluginAssemblyError`; no empty scene is substituted. |
| TableWindowKit callers | Consumed `TableRowsView` with fixed rows/cells/actions | Each `try_push_*` and `render_rows` failure returns from the exact Space window owner. |

## Laws Preserved

- No compatibility adapter converts `serde_json::Value` into action arguments at an `ActionFactory` boundary.
- No `unwrap`/`expect`, silent default, truncation, or empty fallback was introduced in a migrated producer definition.
- Populated child, section, row, cell, action, and drag-data authorities remain fixed and fallible.
- Recursive catalogue/document construction has one bounded traversal opportunity and returns the first exact admission failure.
- Scene encode and wrapper/render boundaries preserve `UiAssemblyResult` instead of erasing refusal.
- Packet A does not broaden mutation ownership into core reconciliation, renderer Packet B, P2 job/session, stdio, or oracle regions.

## Exact Residual Census

The authoritative predicate resolves 107 files.

- Explicit `Vec<UiTreeItemNode>`, `Vec<UiTreeSectionNode>`, `Vec<BuiltNode>`, or `Vec<semio_framework_plugin::BuiltNode>` UI staging: 0.
- Inferred named UI `Vec::new()` staging for items, sections, children, actions, glyph rows, or pair sections: 0.
- `ActionFactory::new(...).action(... json!(...))` or `serde_json::json!(...)`: 0.
- Unmapped `semio_framework_ui_scene::encode` call in the census: 0.
- Old TableWindowKit struct-literal/borrowed consumer pattern in the four migrated Space caller boundaries: 0.

The remaining JSON/`ActionDescriptor` occurrences in broad plugin roots are pre-existing non-`ActionFactory` WindowMeasure/engagement and command/test bridges, not the admitted Packet A factory boundary; this packet did not add an adapter for them.

## Validation and Deferred Gates

- `rustfmt --edition 2021 --check` was run over every changed Rust file. The parser-error filter returned no output.
- `git diff --check` completed without output.
- The exact 107-file residual census above was rerun after the final edits.
- Cargo, Nx, Wasm, and browser gates were deliberately not run while overlapping source packets remain active, as required by the packet coordination contract. They remain the integration gate after packet convergence.
