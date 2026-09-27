# Slider Readout Editing Parity

## Authority

The mounted React `Slider` owns a persistent controlled draft and a temporary numeric readout editor. The static readout is a `role="button"` activated only by double-click. Editing replaces the readout with an auto-focused `type="number"` spinbutton. Enter accepts only finite values already inside the raw range, then snaps to the authored step, publishes change and commit for a changed value, and closes. Escape and blur close without publication. A controlled draft stays visible across stale declared values and retires when the declaration catches up. Thumb Arrow/Page/Home/End handling continues to use the live draft.

The neutral contract is:

- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/⌨️slider-readout-editing/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/⌨️slider-readout-editing/🔣️.json`

## Implementation

- `WidgetState` retains `slider_draft_value` and the bounded last readout click time.
- Reconciliation keeps the draft across stale controlled declarations and clears it only when the normalized declaration catches up within React's step-aware epsilon.
- The event router opens the numeric editor only after two primary releases in `SliderPresentation.value_cell` within 500 ms. Track presses stay owned by the shared track-cell geometry.
- Text, selection, clipboard, paste and IME reuse the retained `EditState`; Slider buffers never publish per keystroke.
- Enter validates the raw range, snaps once, publishes one `Change`, stores local echo and blurs. Escape and pointer blur discard the edit without action.
- Pointer, keyboard and accessibility value changes update the same local controlled draft.
- The accepted accessibility projection continues to publish the live thumb as `slider` and adds a focused editable `spinbutton` while the readout editor exists. Its bounded virtual address retires with the editor.
- Slider paint reads the local draft for track/thumb/readout and the live edit string while editing. Static and editing typography remain separately tokenized.

## Laws

Actual React and neutral schema:

```text
NX_DAEMON=false bun nx run @semio-tech/framework-renderer-react:test -- long '../../../../🧪️tests/⚙️settings-general-layout/🟦️.ts' --run --silent=false --reporter=verbose -t 'language-neutral Slider|actual React Slider'
```

Result: exit 0; 2 passed, 12 skipped; Vitest duration 19.82 s.

Native laws queued to the root runner:

- `slider_readout_editing_matches_the_neutral_react_contract`
- `an_editing_slider_projects_its_live_thumb_and_numeric_spinbutton`

All edited Rust sources parsed successfully with Rust 2021 `rustfmt --emit stdout`. A parent-owned native run remains the authority for compilation and execution because shared Cargo sessions are serialized.
