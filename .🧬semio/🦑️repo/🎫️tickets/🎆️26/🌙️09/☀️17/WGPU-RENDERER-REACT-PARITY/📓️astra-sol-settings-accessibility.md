# Settings Accessibility Visibility and Select Semantics

## Scope

This packet closes the fourth Settings follow-up from `📓️astra-terra-settings-visual-audit.md`:

- the unnamed production accessibility dump must announce the retained documents painted by the last complete Shell walk, rather than every live document;
- a named diagnostic dump must still inspect an inactive retained document;
- accessibility events must use the same visible-document authority;
- an open retained Select must expose a combobox, sibling listbox, and option nodes, while closing retires the virtual listbox without retiring the document;
- option activation must commit one Change action and retain focus on the Select trigger.

No retained document is retired merely because it is inactive. No pointer coordinates or synthesized pointer event enter the accessibility path.

## Neutral contracts

`semio.ui.retained-select-accessibility.v1` records the stable Select owner address, closed/open role sequences, option identities and selection, and the exactly-once Dark activation result. The existing React Select is the independent implementation oracle: its actual trigger mounts the same role matrix, commits once, removes the listbox, and returns focus to the trigger.

`semio.renderer.wgpu.accessibility-visibility.v1` records three live documents, the two visible documents, the unnamed publication including `shell.chrome`, an explicit inactive diagnostic request, and a rejected hidden-document event. An Ajv 2020 law validates the grammar in the existing renderer Vitest suite.

## Product ownership

The Shell starts a bounded visible-document staging roster with each frame setup. Every retained body registers its window id before publishing any hits, including a visible document that has no actionable hit. The roster is promoted only with the completed hit-registry/chrome publication. An abandoned chrome walk therefore leaves both pointer and accessibility ownership at the prior complete generation.

The Interpreter filters only unnamed production dumps through that promoted roster and applies the same roster before retained accessibility dispatch. Its complete live-window census remains available, and a caller that explicitly names a live inactive window still receives its retained accessibility tree for diagnostics.

The retained UI projects virtual Select semantics only while the arena Select is open:

- the real record remains the focusable combobox;
- `<key>::listbox` is its sibling;
- `<key>::option::<value>` nodes are children of the listbox and reuse the record's generation-owned node id;
- option dispatch validates the owner record, exact item value, current surface generation, and current open state before firing Change;
- popup close retires the virtual address and preserves focus on the real trigger.

The browser mirror now renders a combobox as a read-only `button[role=combobox]`, so it does not create an editable text authority. Options remain non-focusable listbox children and actionable options emit one node-addressed accessibility activation. Root's keyboard owner separately forwards combobox Arrow/Enter/Escape/typeahead while suppressing native edit behavior and duplicate button activation.

## Tree paint correction discovered by the runtime comparison

Checkpoint 13 showed correct lower-band hit geometry with duplicated paint: Tree labels were drawn from the full panel viewport's top, while retained inline controls painted at their accepted lower layout. The retained Tree walker now starts reversed rows at `viewport.bottom - visible_content_height`, using the current disclosure state, and no longer manually repaints inline controls already owned by retained child records. A normal mounted General-shape frame law requires the Select trigger and row glyphs in the same lower band and exactly one selected-value glyph run.

## Validation receipts

- focused React Select oracle through Bun+Nx: **1 file, 13 tests passed**;
- focused browser mirror and visibility-schema suite through Bun+Nx: **1 file, 9 tests passed**;
- root keyboard ownership suite: **49/49 passed** across three suites, reported by root after the mirror/root focus repair;
- root UI31: **621/621 passed**, zero skips. This includes actual retained Select accessibility open/option/close dispatch, the normal mounted Up-Tree paint law, and all P2/P3 laws;
- `rustfmt --emit stdout` parsed every changed Rust source and law.

The shared paired browser probe now adds four General steps before the Driver disclosure journey:

- open Appearance and require a physical `dark` option;
- activate that physical option and require popup retirement;
- open Language and require a physical `de` option;
- activate that physical option and require popup retirement.

The React driver resolves the actual `role=option` and `data-value` node; the WGPU driver resolves the retained Button hit for the same value. Each open receipt records trigger and option rectangles, each activation waits for the option authority to disappear, and the ordinary per-step action/structure/AX diagnostics remain intact. Values can be selected only through `SEMIO_PROBE_APPEARANCE` and `SEMIO_PROBE_LANGUAGE`; renderer targets and step filtering remain the existing environment-controlled interface. `bun build --target=bun --packages=external` parsed the modified probe without launching a browser.

The full native renderer and fresh paired browser run remain owned by root. This report does not claim those gates before their receipts.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/♿️retained-select-accessibility/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/♿️retained-select-accessibility/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🧪️tests/🧩️component/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔽️retained-select-origin/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/♿️wgpu-accessibility-visibility/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/♿️wgpu-accessibility-visibility/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/♿️wgpu-accessibility-interaction/🟦️.tsx`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🐍️parity-interact-probe.mjs`
