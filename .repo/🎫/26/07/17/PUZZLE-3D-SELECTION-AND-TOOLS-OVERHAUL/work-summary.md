# Work summary

- Removed `select` utility from Puzzle 3D toolbar; default utility is `move`.
- Selection chrome lives in window measures (`puzzle3d_select_measures_group`) including merge modes.
- Fixed `worldPick` null-id clear (no re-select index 0).
- Cross-entity replace clears other selection bags.
- Gumball gated to `move`/`rotate`/`scale` only.
- Engagement `session_active` only for `brush`/`fill`/`worldRelocate`.
- World3dHost vortex click-vs-drag: click selects, drag connects.
- Puzzle 5D 3D window parity (select removed from 3D utilities; 2D keeps select).

Rust tests added in `puzzle/plugin/rs/lib.rs` d3 tests module.
TS tests updated in `framework/renderer/react/index.test.ts`.

Local `cargo test` blocked by unrelated `semio-framework-os` compile error (`terminology_documents` missing in `AppDefinition`).
