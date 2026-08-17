# UI Mobile Panel One-Consumer Audit

## Baseline

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- MobilePanel component: `9030ca9ecc24ac257a39b73e3c51779b74854f2030db88aa41068d27ff5d27b1`, clean.
- MobilePanel story: `09ec4401427a80fdbf7480da2f543a286bf590644d45756c730c9e520cabbdbd`, clean.
- Layout component: `28b23cd4dc78b57c6fa856a06e973a3e168431c3205c1b7a710f8ded5a699132`, clean.
- Shared React index: `fa8dbb145f3c31af948dc7f18bc51a931cc7cb981fcdac3bd26086e273b99f0b`, accepted serialized UI changes only.

## Consumer Closure

`Layout` is the only active production component that imports and renders `MobilePanel`, using it for the mobile branch while keeping the canvas mounted. The exclusive story and Storybook smoke comment are test/example evidence. Package barrel exports and documentation links are glue/referrers, not independent production terminals. No runtime registry or second direct production consumer exists.

## Decision

Inline the implementation and its contract into the `Layout` semantic component as private `LayoutMobilePanel` behavior and `LayoutMobilePanelProps`. Delete the separate component and exclusive story. Update direct documentation referrers, remove the separate barrel surface, and export the contract with `Layout` because it is part of `LayoutProps`. Do not introduce a module, wrapper, alias, compatibility export, or replacement story. This closure stays within clean framework UI paths and does not cross protected renderer or moving plugin owners.
