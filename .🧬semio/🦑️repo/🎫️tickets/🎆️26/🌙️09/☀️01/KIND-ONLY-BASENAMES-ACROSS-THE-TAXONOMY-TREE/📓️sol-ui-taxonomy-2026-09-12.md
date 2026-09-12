# UI Rust Target Taxonomy Repair

## Scope

This execution packet repaired the UI crate's WGPU and TUI target topology. It did not expand into the UI render core or operating-system renderer backend source packets.

The canonical shape is owner → target → domain → implementation leaf:

```text
🧰️framework/🔨️modules/🖱️ui/
└── 🎯️targets/
    ├── ⌨️tui/🦀️.rs
    └── 🧊️wgpu/
        ├── 🦀️.rs
        ├── 🔀️reconcile/🦀️.rs
        └── …
```

The Rust package remains the single UI crate. Its `🦀️.rs` is minimal package glue that mounts the owner-first targets. No target package or compatibility layer was added. The empty former `📦️packages/🦀️rust/🎯️targets` chain was removed with `rmdir` after its final TUI leaf moved.

## Test-First Evidence

Before the move, a focused language-agnostic census found 31 WGPU Rust implementation leaves with semantic basenames. The initial schema membership check also failed because `📨️scene_slots` was absent from the WGPU target membership list. These gave the failing expectations before the repair.

The repaired tree is described by `🔣️taxonomy.json` rather than by Rust naming rules:

- `semanticDirectoryKinds.input.parentKindIds` includes `wgpu-target`.
- `semanticDirectoryMemberKinds["members-of-wgpu-target"].memberNames` includes all 30 WGPU domain directories in this packet.
- The existing language-agnostic leading-grapheme test independently compares the repository grapheme parser with Lodash. Its schema and Lodash checks passed after these additions; the full target retained one separate launch-registration assertion described under verification.

## Move Map

| Previous WGPU leaf | Canonical WGPU leaf |
| --- | --- |
| `⚙️engine.rs` | `⚙️engine/🦀️.rs` |
| `🎯️events.rs` | `⚡️events/🦀️.rs` |
| `🌐️locale_terminology_value.rs` | `🌐️locale-terminology/🧾️value/🦀️.rs` |
| `🌲️tree.rs` | `🌳️tree/🦀️.rs` |
| `📦️prepared.rs` | `🎟️prepared/🦀️.rs` |
| `🧊️shaders.rs` | `🎨️shaders/🦀️.rs` |
| `🎨️theme.rs` | `🎨️theme/🦀️.rs` |
| `🧾️action.rs` | `🎬️action/🦀️.rs` |
| `🪟️host.rs` | `🏃️host/🦀️.rs` |
| `🕳️arena.rs` | `🏟️arena/🦀️.rs` |
| `🎗️label.rs` | `🏷️label/🦀️.rs` |
| `🐚️shell.rs` | `🐚️shell/🦀️.rs` |
| `🖱️cursor.rs` | `👆️cursor/🦀️.rs` |
| `🧵️mounted_layout.rs` | `📌️mounted_layout/🦀️.rs` |
| `📏️flex.rs` | `📐️flex/🦀️.rs` |
| `📐️geometry.rs` | `📐️geometry/🦀️.rs` |
| `🖋️text.rs` | `📝️text/🦀️.rs` |
| `⌨️input.rs` | `📥️input/🦀️.rs` |
| `🎬️scene_slots.rs` | `📨️scene_slots/🦀️.rs` |
| `🔁️reconcile.rs` | `🔀️reconcile/🦀️.rs` |
| `🔣️icon_name_value.rs` | `🔣️icon-name/🧾️value/🦀️.rs` |
| `🖌️paint.rs` | `🖌️paint/🦀️.rs` |
| `📋️draw_types.rs` | `🖍️draw/🏷️types/🦀️.rs` |
| `✍️draw.rs` | `🖍️draw/🦀️.rs` |
| `🎛️chrome.rs` | `🖥️chrome/🦀️.rs` |
| `🗺️minimap.rs` | `🗺️minimap/🦀️.rs` |
| `🤖️generated.rs` | `🤖️generated/🦀️.rs` |
| `🦀️.rs` | `🦀️.rs` target glue |
| `🖥️gpu.rs` | `🧊️gpu/🦀️.rs` |
| `🧩️component.rs` | `🧩️component/🦀️.rs` |
| `🧮️layout.rs` | `🧮️layout/🦀️.rs` |
| `🎚️widgets.rs` | `🪀️widgets/🦀️.rs` |

The TUI target glue moved from `📦️packages/🦀️rust/🎯️targets/⌨️tui/🦀️.rs` to `🎯️targets/⌨️tui/🦀️.rs`.

## Wiring and Consumer Repairs

The WGPU and TUI target glue now mounts domains, elements UI components, shared icons, and element target modules from the owner-first location. The UI package root mounts both targets through `../../../../`-free owner-relative paths. Unit-test mounts and the WGPU text font embeds were rebased to the same correct owner-relative depth.

The UI axes generator in `🖱️ui/📦️packages/🦀️rust/📜️script.ts` now derives the WGPU target root from the UI owner and emits Rust to `🎯️targets/🧊️wgpu/🤖️generated/🦀️.rs`. The generated-source contract in `🔣️taxonomy.json` names that output. `🤖️generated/🦀️.rs` remains generated and is not treated as authored source.

Active source consumers were updated in:

- the root `📜️script.ts` interactivity audit;
- the UI styling suite, render documentation, interactivity and action tests, package glue, contract documentation, and contract schema documentation;
- the manifest and value documentation;
- the mesh and space-plugin documentation;
- the OS renderer Winit documentation, mounted-frame transaction test, and async-boundary Rust test.

An active-source search excluding `.🧬semio`, `.cursor`, build outputs, dependencies, and `♻️mit-bestand` found no remaining reference to the former UI package-owned WGPU or TUI target paths. The nested-Cargo authority fixture still contains one old UI WGPU token under its explicit `preservedNonReferences` evidence; it is a historical frozen fixture, not a source consumer, and was left unchanged.

## Source and Referent Identity

The concurrent live bytes of the three edited implementation leaves were recorded before moving:

| Previous leaf | Pre-move live SHA-256 |
| --- | --- |
| `✍️draw.rs` | `3cfd27df7941cd63f114e80efa073b394d6827792572db94d2937742b5cfaa81` |
| `🔁️reconcile.rs` | `40eeb6fd8dc9022f4fa4613dc2d7777a378b0deaa160cd10ccaebdb7584dd897` |
| `🖥️gpu.rs` | `d0ee35d0f1129cfc1a1f3a7f6f65f99b114a48a37531ad08d1884191f17c7dae` |

A post-move audit reversed only the required test/font/document path rebases. It reproduced 27 tracked WGPU leaves from `HEAD`, all three pre-move live hashes above, and the tracked TUI glue byte-for-byte. `🤖️generated.rs` was absent from `HEAD`; the registered UI axes freshness check validates its current canonical bytes against the generator instead.

A separate path-identity audit normalized the previous and current source directories and compared 61 changed external `#[path]`/embed referents. Every rebased WGPU test, font, icon, element, TUI component, and TUI element reference resolves to the same canonical repository path as before. Moved internal source referents resolve through the move map above and inherit its byte-identity proof.

## Verification

| Check | Result |
| --- | --- |
| Focused post-repair Bun structural/schema/referent audit | Passed: 32 anonymous WGPU Rust leaves, owner-first TUI glue, 30 registered WGPU domains, 99 existing source referents, and no package-owned `🎯️targets` directory. |
| Source identity audit | Passed: 27 `HEAD` WGPU leaves, three pre-move live WGPU leaves, and TUI glue reproduced after reversing path-only rebases. |
| External referent identity audit | Passed: 61 rebased test/font/icon/component/element referents resolve to the same canonical paths. |
| `NX_DAEMON=false NX_ISOLATE_PLUGINS=false … bun nx run @semio-tech/ui-rs:check` | Passed: `ui axes are fresh (2 locales, 2 terminologies).` |
| `bun ./📜️script.ts test-wgpu-engine quick reconcile` | Passed: 12 passed, 378 skipped. |
| `bun ./📜️script.ts test-wgpu-engine quick mounted_layout` | Passed: 15 passed, 375 skipped. |
| Full direct `test-wgpu-engine quick` attempt | Compiled and ran 138 tests before fail-fast: 137 passed; the hostile-fixtures test failed at its existing source fixture assertion with `hostile builder: (ArenaFull, SurfaceId("hostile.surface"))`; 252 were canceled. The engine source differs from `HEAD` only by its test mount paths, and the hostile fixture has no working-tree diff, so this assertion was not changed by the taxonomy move. |
| `bun nx run @semio-tech/repo-lib:test-taxonomy-leading-grapheme` | Eight checks passed, including Lodash parity and schema member prefixes over 5,597 occurrences. One launch-registration assertion failed because it expected ` --skip-nx-cache` and received the command without that suffix. |
| `bun nx run @semio-tech/ui-rs:test-wgpu-engine` | Did not start the target. Nx reported: `NX Failed to process project graph. An error occurred while processing files for the @repo/test-cases plugin (Defined at nx.json#plugins[3]). - The "path" argument must be of type string. Received undefined`. A verbose graph retry produced no stack before it was canceled. Concurrent test-taxonomy work was changing `🔮️oracles` and the plugin at the same time, so the observation is recorded without assigning sole cause. |
| Registered focused TUI test attempt | `bun ./📜️script.ts test quick window_control_clicks_resolve_to_close_and_maximize_signals` produced no output for 90 seconds through the shared Cargo workspace and was canceled with exit 130. Structural, referent, and source-identity checks completed for the TUI move. |
| Full `bun 📜️script.ts verify taxonomy enforce` | Produced no output for about 90 seconds under concurrent project-graph work and was canceled with exit 130. The ticket scanner similarly did not finish within 85 seconds. Focused checks above are the bounded evidence for this lane. |

No generated diagnostic or cache output from this lane is retained.
