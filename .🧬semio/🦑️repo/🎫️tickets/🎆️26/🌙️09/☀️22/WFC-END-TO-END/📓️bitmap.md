# 🖼️ Bitmap fill slice

## Cargo

```
export RUST_MIN_STACK=33554432; cargo test -p semio-s-artifact-wfc-bitmap --features component-app-assembly --lib -j 4 -- --test-threads=4
```

`test result: ok. 203 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 40.34s`

## Done

- Fill tool (`fill`, non-mutating) with tick payload `{pixels, decided, width, height, contradiction, done}`; live paint via output window + decided mask; finished cache stays `SetSolve` / `BitmapTransient`; abort does not dispatch commit.
- Editor root wires `.tool(fill)` + `.mode_tools(edit, [fill])`; Solve starts the fill run.
- Contract fill tests: partial progress, final payload vs `solve_with_job`, abort without SetSolve, partial≠empty≠finished render, Python oracle vector.
- Audit MISSING rows: contradiction overlay render, direct txt/json serialize/deserialize round-trips, inference checkpoint admission.

## Ownership

Only `🗿️artifacts/🖼️bitmap` and this ticket folder. Fill job path left as-is (`pending_tick` / child Yield).
