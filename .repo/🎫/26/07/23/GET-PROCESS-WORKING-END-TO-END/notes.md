# Get Process Working End to End

## Root cause
Corrupted match arm in `toggle_sun_round_trips_through_runtime_and_defaults_off` (`process/plugin/rs/lib.rs`): stray `value: None, min: None, … on_change: None,` field lines were pasted inside a `match measure` block, producing 16 compile errors (`expected one of @ or |, found :`).

## Fix
Restored the match to:

```rust
WindowMeasure::Group { id, children, .. } if id == "process3d-measure-sun" => Some(children.clone()),
_ => None,
```

## Verification
- `cargo build -p process-plugin --target wasm32-wasip2 --release` — ok
- `cargo test -p process-plugin --lib` — **34 passed**, 0 failed (isolated `CARGO_TARGET_DIR` under this ticket)
- `SEMIO_RENDERER=react bun run dev:process:3d` — Vite on http://127.0.0.1:6022/
- Playwright e2e (`verify-process-e2e.ts`): 1 canvas, Process 3D chrome (`semio · process · 3d`, Timber Beam Joinery, Workpiece), no render/page errors

## Note
Default `dev:process:3d` uses wgpu (port 6122). An earlier wgpu trunk attempt timed out waiting on a shared cargo lock held by concurrent builds; React path confirmed end-to-end without that contention.
