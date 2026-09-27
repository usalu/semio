# NumberStepper Editing and Hold Parity

## Scope

The generic retained `NumberStepper` now carries optional finite `min`/`max` bounds through the shared Rust contract, typed visitor, generated TypeScript metadata, decoder, validation, accessibility projection, React retained adapter, WGPU node, and reconcile boundary.

WGPU center editing uses retained focus/edit state. It supports focus seeding, selection, caret navigation, deletion, copy/cut/paste, committed IME input, Escape restore, Enter blur, bounded absolute edits, and local displayed values. Pointer and keyboard increments preserve React's binding distinction: a declared `Delta` binding receives the raw signed step, while the absolute binding receives the clamped value. A side at its bound is pointer-disabled, while an arrow key at the same bound still emits the React action.

Held side input emits one immediate action, waits 500 ms, then changes only the local displayed value every 100 ms. Pointer release, cancellation, leaving that exact side segment, and node retirement cancel the timer. The clock comparison uses a one-microsecond tolerance so repeated `f32` interval addition does not defer an exact 700 ms tick to another frame.

The retained painter shows the live edit/composition text and limits hover fill to the active side segment. The center remains transparent and borderless; the chrome budget reserves one optional hover quad in addition to the seven base quads.

## Neutral Contract and React Oracle

- Schema: `🧰️framework/🔨️modules/🖱️ui/🧬️schema/⌨️number-stepper-editing/🔣️.json`
- Fixture: `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/⌨️number-stepper-editing/🔣️.json`
- Actual React oracle: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚙️settings-general-layout/🟦️.ts`
- Native law: `number_stepper_editing_and_hold_repeat_match_the_neutral_react_contract`

The mounted React oracle established one detail that differed from the initial source audit: a mixed Stepper displays its placeholder before focus, then focus marks it edited and displays its default/current numeric value (`0`). The neutral fixture and WGPU buffer follow that observed result.

## Verification

`NX_DAEMON=false bun nx run @semio-tech/framework-renderer-react:test -- long '../../../../🧪️tests/⚙️settings-general-layout/🟦️.ts' --run --silent=false --reporter=verbose -t 'actual React Stepper'`

- PASS: 1 test, 11 skipped.
- Vitest duration: 19.13 s.
- Covers bounded edit/Escape, mixed focus, absolute and delta boundary keys, disabled bound pointer side, Enter, and held timing/cancellation.

`NX_DAEMON=false bun nx run @semio-tech/framework-renderer-react:test -- long '../../../../🧪️tests/⚙️settings-general-layout/🟦️.ts' --run --silent=false --reporter=verbose -t 'language-neutral NumberStepper'`

- PASS: 1 test, 11 skipped.
- Vitest duration: 23.97 s.

Parent native UI packet `80369` compiled and executed the new native law. Its only NumberStepper failure was the exact 700 ms repeat tick (`actual 4`, `expected 5`), which exposed `f32` deadline drift. The production clock tolerance above repairs that observed failure; a focused rerun is pending.
