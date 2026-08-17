# UI Class Name Presentation Registrar Integration

## Baseline

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- React barrel pre-edit SHA-256: `0f8def42b5703b2ab00bd31f6e7b242e334ea9f60fdd9a5d35c1a88fdf8fa401`
- React barrel final SHA-256: `9e24d693c415feaf14804482df1f24c76e33fbcec13d958630496453fb419838`

## Registrar Change

Replaced the single ClassNames umbrella import/export region with explicit imports and exports from the specific UI-owner modules:

- class-name composition
- status-border presentation
- interaction presentation
- form-control presentation
- border presentation
- surface presentation
- menu-item presentation
- shell-floor presentation
- chrome-control presentation

The coordinator deliberately removed Slider and Table presentation symbols from the public barrel because the execution lease moved them into their sole production components.

## Static Result

- `🏷️ClassNames/🟦️component` active UI scan: zero.
- Slider-private and Table-private presentation symbol scan in the barrel: zero.
- Scoped barrel `git diff --check`: passed.
- No wildcard or compatibility export was added.
- The first typecheck exposed duplicate status-element imports left in the chrome-control group. The coordinator removed those two duplicate imports and exports; `loadingBorderElementClass` and `waitingBorderElementClass` now come only from status-border presentation.
