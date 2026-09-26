# Retained Toggle Semantics

The actual React Interpreter has two retained Toggle presentations: checkbox appearance renders `TreeCheckbox checked={on}`; button appearance renders the native `Toggle pressed={on}`, whose element is a button with `aria-pressed`. The shared accessibility projection previously collapsed both into role switch plus checked state.

The Rust and TypeScript projection twins now publish checkbox appearance as role checkbox with checked state only, and button appearance as role button with pressed state only. The WGPU arena stamp updates the same appearance-specific channel from live retained state. The React Interpreter's button path also forwards disabled state and the authored accessibility label, which keeps the actual element's accessible name aligned with the projection.

The strict neutral `retained-toggle-semantics-v1` schema and fixture contain one case for each appearance. Rust, TypeScript and WGPU consume the same cases. The independent jsdom oracle mounts the actual Interpreter and uses `dom-accessibility-api` to read its native role and accessible name, then checks the WGPU accessibility mirror exposes the same exclusive state attribute.

Focused receipts on 2026-09-26:

- TypeScript projection twin: 263 checks passed;
- actual React Interpreter plus WGPU mirror: 2 of 2 tests passed;
- EventFeed door rerun after the shared graph recovered: 4 of 4 tests passed.

Native laws supplied to the coordinated runner are `every_toggle_appearance_projects_only_its_native_state_channel` and `mounted_toggle_appearances_stamp_only_their_native_live_state_channel`. Their Cargo result remains owned by the coordinated root build.
