# Lane projection result

## Root cause
The derive `✨️derive/📽️projection` emits `drive_close` in an inherent impl on the generated cursor. It called `close_demands`/`close_work_demand`, which are methods of the trait `SnapshotProjectionCursor` (📽️projection/🦀️.rs:63-64), but the trait was not in scope in the user crate. The quote-key change introduced those calls.

## Fix
One place: in `drive_close`, `use #value_crate::__private::snapshot_projection::SnapshotProjectionCursor as _;` inside the quote-key block. No change to the 6 crates.

## Verification
- W-projection-1 wasm (hub, 6 crates) rc0
- W-projection-2 native (hub, 6 crates + semio-framework-value-snapshot-projection) rc0
Rerun: `"$C/1105b-tools/chk.sh" W-projection-3 "🌎️hub/Cargo.toml" native -p semio-framework-value-snapshot-projection ...`

Not verified: tests; array/captured/field/root projection files beyond what compiled.

STATUS: FIXED-verified W-projection-2
