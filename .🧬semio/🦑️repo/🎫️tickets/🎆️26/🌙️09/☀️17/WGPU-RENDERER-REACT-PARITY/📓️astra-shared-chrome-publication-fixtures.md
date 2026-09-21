# Shared Chrome Publication Fixtures

## Finding

The failing Actions, palette, Find, and ToolRun tests mixed candidate paint state with accepted input state. A completed `render_chrome_step` or retained-document paint only fills staging. Production makes those rows interactive after the exact presented-input candidate is sealed and acknowledged.

## Repair

- `publish_dense_actions_chrome` completes the normal chrome walk and then calls the test-only `ShellState::publish_retained_hit_registry`.
- `publish_palette_chrome` starts from the default `ShellChromeFrameCursor`, so `FrameSetup` clears the candidate registries and begins the accessibility visibility pass. It acknowledges the completed walk before its callers inspect rows or route input.
- ToolRun begins the accessibility visibility pass, paints its retained document, registers the body through `ShellState::register_retained_body_hits`, and acknowledges the candidate through `publish_retained_hit_registry`.
- ToolRun pointer laws now select from `InputState::hits()` and use `ShellState::handle_pointer_button`, the same published retained routing path used by the host. They no longer inspect `staged_hits` or dispatch directly into a candidate tree.
- ToolRun teardown requests the mounted UI document close and advances the bounded close owner before releasing the document lease.

## Validation

All four changed Rust files parse with `rustfmt --edition 2021 --emit stdout`.

The Native134 source-law failure in `the_present_watchdog_signature_carries_within_item_upload_progress` was a stale tuple-string assertion. It now checks GPU cursor progress and the independent upload, raster-retirement, and presented-input progress terms.

The prepared-input A→B→C dependency law is:

`NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/ui-rs:test-wgpu-engine -- accepted_a_stale_b_abort_and_accepted_c_preserve_exact_presenter_owners --exact`

Root executed it after Native134: 1 passed, 663 filtered, 0.015 s test time, Nx 31.5 s.

The corrected Shell fixture laws still require the next renderer native gate. No Cargo command was run in this workstream.
