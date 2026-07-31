# Ticket

## Todos

# Plan

1. [x] Analyze benchmark results - identify Rust is 50-100x slower than Go
2. [x] Read Go and Rust flatten_design implementations to compare
3. [x] Identify key performance issues: cloning pieces, Result wrapper, String allocations
4. [x] Refactor Rust flatten_design to return DesignDiff instead of Vec<FlattenedPiece>
5. [x] Use &str references instead of String clones in HashMap keys
6. [x] Pre-allocate HashMap capacities
7. [x] Fix WASM binding to use new return type
8. [x] Verify code compiles with cargo check
9. [x] Run benchmarks and verify performance improvement
10. [x] Document changes

## Changes

## Log

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

## Summary

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
