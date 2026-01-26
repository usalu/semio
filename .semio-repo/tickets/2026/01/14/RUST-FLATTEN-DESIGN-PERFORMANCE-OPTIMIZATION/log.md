# Log

## Analysis

Analyzed benchmark.csv showing Rust flatten_design was significantly slower than Go:

- Nakagin Capsule Tower: Rust 0.489ms vs Go 0.0094ms (~52x slower)
- Capsule Dream: Rust 72.879ms vs Go 0.7303ms (~100x slower)

## Root Cause

The Rust implementation had several performance issues:

1. Returned `Vec<FlattenedPiece>` which cloned entire Piece structs
2. Used `String` keys in HashMaps instead of `&str` references
3. Used `Result` wrapper unnecessarily
4. Did not pre-allocate HashMap capacities

Go and TypeScript return `DesignDiff` containing only plane updates, not cloned pieces.

## Changes Made

1. Changed `flatten_design` return type from `Result<Vec<FlattenedPiece>>` to `DesignDiff`
2. Changed HashMap keys from `&String` to `&str`
3. Added `HashMap::with_capacity()` pre-allocation
4. Changed helper functions to not use Result wrapper
5. Updated WASM binding to return DesignDiff
6. Added `planes_equal_approx` helper for diff detection

## Results

New benchmark results:

- Nakagin Capsule Tower: 0.261ms (was 0.489ms) - **1.9x faster**
- Capsule Dream: 7.361ms (was 72.879ms) - **10x faster**

Rust is now faster than TypeScript for all flatten operations.
