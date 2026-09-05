# UI Hand-Reviewed Repair

Status: in progress; no whole-tree cleanliness claim. The elements subtree is now hand-repaired: all 262 physical entries were traversed, with zero naming findings.

The first 12 manual moves are applied (six root identities and six module identities). Nine independently checked empty duplicate shells were removed; recreating an empty directory restores them, and no files were deleted. The module traversal now has 51 entries and zero naming findings. React's long Nx suite passes all 692 tests; Admin's stylesheet and entry tests pass all 18 tests.

The UI root was inspected, including stylesheet contents, the package exports, the oracle contribution, and the rendering package manifests. The handpicked root choices are:

| Current identity | Chosen identity | Meaning |
| --- | --- | --- |
| `🎨️.css` | `🧵️.css` | Composes the shared stylesheet imports |
| `🎨️globals-ui.css` | `🌐️globals-ui.css` | Global UI interaction styles |
| `🎨️theme.css` | `🌓️theme.css` | Theme/palette stylesheet entry |
| `🔣️components.json` | `components.json` | Reserved shadcn configuration basename |
| `🧪️oracle` | `🔮️oracle` | Independent comparison authority, distinct from the story type file |
| `🖼️render` | `🖌️render` | Rendering operations and backends, distinct from image assets |

The `components.json` exception follows the actual schema in the file and the [official shadcn configuration contract](https://ui.shadcn.com/docs/components-json). It is registered only at this exact owner path. The existing `🔣️ui-axes.json` becomes unique once this fixed-name correction is applied and is left intact. The existing story type file is likewise left intact once the oracle receives its own handpicked identity.

The theme entry also contains a broken `../styling/🎨️palette.css` reference; its actual sibling styling owner is `./🎨️styling/🎨️palette.css`. Source reference fixes are exact edits. No automatic emoji picker, broad text replacement, Git mutation, or overwrite from history is used.

All six root moves are applied. The four changed React package exports resolve to existing files. The first Admin stylesheet test caught its remaining old export key; that exact fixture reference is corrected without changing the test assertions.

The modules traversal found nine empty unprefixed directories, each duplicating an existing populated semantic owner: border presentation, chrome controls, class-name composition, form controls, interaction, menu items, shell floor, status borders, and surfaces. They contain no files and will be removed with individual empty-directory-only operations; all populated owners remain.

After reading the implementations, the keybinding context will use `🕹️control-keybinding-context` (dispatch), tooltip composition `💡️control-tooltip-presentation` (hints), shortcut text `🔤️keybinding-text-interpretation` (glyph interpretation), and style choices `🧬️style-variants` (variants). `🏷️class-name-composition` and `⌨️control-hotkey-presentation` already fit their actual contents and remain. The single-child React slot will use `🪆️slot.tsx`, with its composition test `🧩️slot.test.tsx`, distinct from the executable-test directory.

## Element Choices

The physical element census has 262 entries and 30 collisions. Reviewed directory choices are `↔️Resizable` (splitter size), `🔽️Select` (dropdown options), `🔀️Toggle` (pressed state), `🔳️ButtonGroup` (grouped buttons), `🪙️Chip` (terminal pill), `🎗️UiLabel` (branded localized text), `📨️UIDialog` (argument submission), `🕸️Diagram` (directed node graph), `🧭️PanelTabBar` (panel navigation), `🕰️HistoryTable` (checkpoint chronology), `🎴️IconSelector` (pictogram selection), `🔲️WindowSilhouette` (outline geometry), and `🌳️Tree` (hierarchical tree). Their existing siblings remain unchanged.

Diagram's non-React implementation is directed layout, so `🟦️.ts` becomes `📐️layout.ts`; Ports' non-React implementation is its interactive job contract, so `🟦️.ts` becomes `📡️interactive-jobs.ts`. The React implementations keep their existing single-emoji names.

Each colliding story was inspected for its subject and exported examples. Story names will be `🪗️.story.tsx` for Collapsible, `🎮️.story.tsx` for Command, `✅️.story.tsx` for Checkbox, `📋️.story.tsx` for Select, `🎚️.story.tsx` for Slider, `🔀️.story.tsx` for Toggle, `🎛️.story.tsx` for ToggleGroup, `💬️.story.tsx` for Dialog, `🕸️.story.tsx` for Diagram, `🍽️.story.tsx` for MenuItem, `📑️.story.tsx` for Tabs, `👤️.story.tsx` for TableAvatar, `💭️.story.tsx` for Popover, `📨️.story.tsx` for Form, and `🌳️.story.tsx` for Tree. Storybook's old literal `🧪️.story.tsx` discovery must be corrected and tested before these names are applied.

All 30 element moves are applied: 15 stories, 13 directories, and the two non-React implementation leaves. Storybook now selects the semantic `.story.tsx` suffix, with a neutral six-case fixture checked against both the repository matcher and picomatch. The regression was observed failing before the matcher change and passing afterward; all 42 physical UI stories remain discoverable. React imports, the executable test catalog, Rust mounts, and exact taxonomy member records have been patched individually. The renderer agent owns the corresponding WGPU input declarations and regenerated browser output.

The first post-element React run executed all 692 tests: 681 passed and 11 encountered the styling agent's in-flight stylesheet rename. These are recorded as a failed run, not a pass; a rerun follows its exact reference completion. Output: `🗑️generated/ui-react-elements.txt`. Naming audit: `🗑️generated/ui-elements-after.json`.

After styling references settled, the React rerun passed **692/692** tests across 20 files (`ui-react-elements-final.txt`). Native UI element verification passed **235/235** tests via the existing `@semio-tech/ui-rs:test-long -- --lib` Nx target (`ui-native-elements.txt`).

The React package review found three existing fixed-name contracts that its decorated files violated: `tailwind.config.ts`, `postcss.config.ts`, and `eslint.config.ts`. They are restored to the exact reserved names. Four implementation boundaries receive individual identities: `🏗️build-tooling.ts` for build integration, `⚛️runtime.ts` for the React boundary, `🧰️vitest.setup.ts` for DOM test setup, and `🖌️render.ts` for the test renderer. Public export keys stay unchanged; source targets and configuration readers are patched explicitly. Verification of this later seven-file change is pending.

## Scene, Host, and Runtime Review

Each Rust module header and mount was reviewed before selecting the following identities. Scene: `📐️math.rs` for spatial geometry, `📦️pack.rs` for binary encoding, `🎬️scenes.rs` for scene payloads, `🖼️canvas2d_snapshot.rs` for immutable canvas packets, `🌉️surface.rs` for the typed/opaque surface bridge, and `🌍️world3d_snapshot.rs` for world packets. Host: `🔌️backend_alias.rs` for platform backend selection, `📥️enqueue.rs` for the event queue, `📡️event.rs` for event normalization, and `🪟️window.rs` for window hosting.

Runtime: `🎛️context.rs` for effect context, `🎯️dispatch.rs` for intent routing, `🪪️entity.rs` for generational identity, `📮️gateway.rs` for command submission, `📥️inbox.rs` for coalesced projection intake, `👥️presence.rs` for peer presence, `🎭️present.rs` for presentation, `♻️reconcile.rs` for keyed reconciliation, `🕸️tracking.rs` for read-dependency edges, and `🔄️transaction.rs` for the frame transaction. Each is distinct from every sibling including the unchanged Rust entry point. Five neutral runtime specimen directories use `🧫️fixture`, each distinct from its executable `🧪️tests` sibling. These are selected names, not generated assignments; moves and reference edits follow individually.

All 25 scene/host/runtime moves are applied. Physical audits report zero naming findings and zero missing literal Rust mounts across scene (12 entries), host (58), and runtime (51). All five renamed fixture owners resolve through the actual full taxonomy ancestry. Runtime's pre-move test run exposed a corrupted `🧪️test/🦀️s.rs` reference; the actual existing test is `🧪️tests/🦀️.rs`, and the exact reference was corrected. Runtime now passes 121 native tests; scene passes 108. Host executes all 79 tests, with 70 passed and nine input-admission/root behavior assertions failed; no assertions were weakened. The latest React package run also passes 692 tests after its seven-file repair.

## WGPU Package Review

Each of the 30 colliding Rust files was inspected for its module contract and main declarations. Chosen identities are: `🧾️action.rs`, `🕳️arena.rs`, `🎛️chrome.rs`, `🧩️component.rs`, `🖱️cursor.rs`, `✍️draw.rs`, `📋️draw_types.rs`, `⚙️engine.rs`, `🎯️events.rs`, `📏️flex.rs`, `📐️geometry.rs`, `🖥️gpu.rs`, `🪟️host.rs`, `🔣️icon_name_value.rs`, `⌨️input.rs`, `🎗️label.rs`, `🧮️layout.rs`, `🌐️locale_terminology_value.rs`, `🗺️minimap.rs`, `🧵️mounted_layout.rs`, `🖌️paint.rs`, `📦️prepared.rs`, `🔁️reconcile.rs`, `🎬️scene_slots.rs`, `🧊️shaders.rs`, `🐚️shell.rs`, `🖋️text.rs`, `🎨️theme.rs`, `🌲️tree.rs`, and `🎚️widgets.rs`. These distinguish actual responsibilities rather than repeating the Rust language icon. Their existing root entry and generated axes file remain unchanged.

The 30 moves and all current module mounts/root policy readers are applied. The first native rerun caught an action-source test reading its old own basename; that exact `include_str!` now points to `🧾️action.rs`. The package build script also watched a nonexistent target-local axes file instead of the actual owner-level `../../🔣️ui-axes.json`; its watcher is corrected without changing generated output.

The nested WGPU `build.rs` contains only `fn main() {}` and has no current reader or Cargo manifest at its owner. It is moved intact to this ticket as `🧱️unused-ui-target-build.rs`, preserving its bytes outside the production tree. The real package-root `build.rs` remains literal and active. The existing empty Objective-C comparison owner is preserved as `🔮️oracle/🧠️objc2-runtime` and registered narrowly under the oracle role; no contents are removed.

The final WGPU package native run passes **235/235** tests after the action self-reference fix. Evidence: `🗑️generated/ui-wgpu-package-final.txt`.

## Duplicate Asset Owner

The UI-local `🖼️assets` tree duplicates the active framework-level `🖼️assets` owner. A read-only comparison found **531 byte-identical files, zero differing same-path files**, and two source files whose declarations already live under the active owner's `🎯️concepts` and `🔍️resolver`. TypeScript's independent parser/printer confirms every non-import declaration in those two files matches the active implementation exactly; the active resolver additionally owns the metabolism functions. The old copies have broken imports and no executable consumer. The repository's existing styling-owner plan also describes this completed consolidation, but current bytes and consumers—not that plan—are the authority for repair.

Storybook's one remaining source-root reference now uses the active asset owner, and an unused presentation-test variable referencing the obsolete tree is removed. Storybook's existing global-CSS alias is corrected to the actual renamed stylesheet. The entire redundant 533-file tree is moved intact to `🗑️generated/🖼️retired-ui-assets` for recovery during the incident. No source bytes are discarded, no Git operation is used, and the active asset owner is untouched. The active owner still requires its own handpicked naming review; duplicate removal is not a claim that those assets are already clean.

Comparison evidence: `🗑️generated/ui-asset-ownership-comparison.json`.

Loading the actual Storybook scope confirms every UI source root resolves after the consolidation (`ui-source-roots.txt`). The final physical UI audit, excluding only the separately assigned contract tree and actual compiler/dependency caches, reports **649 entries, 601 governed, zero naming findings**. Python's literal `pyproject.toml` and `uv.lock` are resolved using their actual package context, not waived. The contract agent is still working; there is no whole-UI or whole-workspace completion claim.
