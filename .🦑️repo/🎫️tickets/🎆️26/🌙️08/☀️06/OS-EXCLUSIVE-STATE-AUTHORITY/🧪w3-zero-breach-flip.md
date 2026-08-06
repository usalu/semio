# Wave 3 zero-breach flip

- Rust `os-state-authority/*` + document-app-shape: **0** (ungated).
- Authority-struct-map cleared by renaming non-OS-authority suffixes (`CadProjection`, `ViewModel`, …).
- TS module ephemeral state routed through `ephemeralBox` / `ephemeralMap` / `ephemeralSet` / `ephemeralWeakMap` in framework-core.
- ESLint OS-authority rules: **0** hits after migration.
- Verify gate runs OS policy functions only (does not fail on the ~490 unrelated policy kinds).
