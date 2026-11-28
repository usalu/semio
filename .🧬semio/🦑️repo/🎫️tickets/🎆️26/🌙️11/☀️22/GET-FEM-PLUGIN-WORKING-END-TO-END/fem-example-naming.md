# FEM example naming: one declaration for the combobox and the palette

## Defect
The same bundled examples had two names and two option sets.
- Artifact panel combobox (3D): `No example, Demo, Concrete Forest, House`. The rows are the host's own `No example` row plus the plugin's registered `ExampleSource`s (`root 🦀️.rs examples()`).
- Palette action `setActiveExample` (3D): `Concrete Forest, Default, House`, with `demo` hand-labelled `Default` / `Standard`, hand-ordered, no empty option. In 2D: only `Demo`, no empty option.

## Single source of identity and labels
`examples::sources()` in each artifact's io crate (`🚪️io/📦️packages/🦀️rust/🦀️.rs`, `pub mod examples`) is the one ordered declaration: `demo, concrete-forest, house` in 3D, `demo` in 2D. Ids and the localized (EN first, DE second, no default language) labels live in each example leaf (`📚️examples/*/🦀️.rs`): `Demo/Demo`, `Concrete Forest/Betonwald`, `House/Haus`. The canonical name of `demo` is `Demo` everywhere.

| Surface | Before | After |
|---|---|---|
| Host registry / combobox rows | root `examples()` listed the leaves by hand | `examples::sources()` |
| Palette `setActiveExample.exampleId` options | hand-written options per manifest | `semio_s_fem_app_surface::set_active_example_arg(&examples::sources(), default_id)` |
| Empty option | combobox only (host i18n `ui.common.noExample`) | `semio_s_fem_app_surface::EMPTY_EXAMPLE_ID` + `no_example_label()` ("No example" / "Kein Beispiel") as first palette option in 2D and 3D |

The command handlers already treat any unknown id, including `""`, as the empty document, so the new empty option works without a handler change.

Defaults are unchanged: 3D `concrete-forest` (the boot example), 2D `demo`.

## Files
- `✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/🦀️.rs`: `EMPTY_EXAMPLE_ID`, `no_example_label`, `example_options`, `set_active_example_arg`.
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{🧊️3d,◻️2d}/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📦️packages/🦀️rust/🦀️.rs`: `examples::sources()`.
- `.../🌐️any/🦀️.rs` (both): `examples()` uses `sources()`.
- 3D `✏️editor/📋️manifest/🦀️.rs`, 2D `✏️editor/🚀️ui-helpers-and-app-assembly/🦀️.rs`: the hand-written select replaced by `set_active_example_arg`.
- Law (both apps) `✏️editor/🧪️tests/🔬️unit/🦀️.rs`: `set_active_example_options_equal_the_combobox_options`. It asserts the declared options (id, EN, DE, order, default) equal the language-agnostic vector `✏️editor/🧪️tests/🧫️fixtures/📚️example-options/🔣️.json` and equal `[empty] + examples::sources()` (the combobox source).
- Harness: `matrix-v3/checks-model.mjs` `act.setActiveExample` now restores `Demo` (was `Default`) in 3D and lists the rows `No example, Demo, Concrete Forest, House` (2D `No example, Demo`).

## Notes
- The host combobox's `No example` label is the UI i18n key `ui.common.noExample` (EN "No example", DE "Kein Beispiel"); the plugin's `no_example_label()` carries the same two strings. They are two declarations of one label across languages; unifying them needs a host change and was left out.
- Rust changes ship with the coordinator's next wasm publish; until then the browser still shows `Default` in the palette.
