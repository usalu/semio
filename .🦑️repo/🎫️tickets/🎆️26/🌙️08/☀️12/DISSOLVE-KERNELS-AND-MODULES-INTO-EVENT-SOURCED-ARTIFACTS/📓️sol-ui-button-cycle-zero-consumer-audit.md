# UI Button Cycle Zero-Consumer Audit

## Baseline

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Button component SHA-256 before the active ClassNames split: `ed5087bf46db0f7e1a988c566e564a11e656b658a3bff4c01f538739aba52918`

## Finding

The Button component currently combines the actively used single Button interaction with a separate `ButtonCycle` interaction and its `ButtonCycleItem`/`ButtonCycleProps` contracts.

`ButtonCycle` has zero independent active production consumers. Its only external references are the framework React assembly barrel and the Button Storybook story. Package glue and example/test provenance do not qualify as production consumers.

The ordinary `Button` remains independently consumed by Tree, UIDialog, IconSelector, authored React application surfaces, and the paired Rust UI implementation.

## Disposition

Delete `ButtonCycle`, its private item contract, its exported props contract, and only its exclusive story region. Remove the React-barrel import/export and corresponding Storybook smoke identifiers. Retain Button and its story. Do not create a module, compatibility export, or replacement interaction.

Execution must wait until the ClassNames umbrella lease releases Button and the shared React barrel.
