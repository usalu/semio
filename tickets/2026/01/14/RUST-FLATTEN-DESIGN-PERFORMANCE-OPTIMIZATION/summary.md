# Summary

Optimized Rust `flatten_design` function for ~2-10x performance improvement.

## Key Changes

- Changed return type from `Result<Vec<FlattenedPiece>>` to `DesignDiff` (matching Go/JS)
- Replaced String clones with &str references in HashMap keys
- Pre-allocated HashMap capacities
- Removed unnecessary Result wrapper from helper functions
- Updated WASM binding to return DesignDiff

## Performance Improvement

| Benchmark             | Before   | After   | Speedup |
| --------------------- | -------- | ------- | ------- |
| Nakagin Capsule Tower | 0.489ms  | 0.261ms | 1.9x    |
| Capsule Dream         | 72.879ms | 7.361ms | 10x     |

Rust is now faster than TypeScript for all flatten operations. Go still has an edge due to different quaternion-based matrix math.
