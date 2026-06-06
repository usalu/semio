# Concrete Forest Brush Port Family Fix

## Problem

Brush on concrete forest b-* vortices could suggest c-* (column) placements. Root causes:

1. Empty `kindCompatibility` at runtime is treated as "allow all" in `vorticesAttractionCompatibleForDrag`.
2. WASM precompute scene sync omitted `kindCompatibility` from brush collision probes and from the sync fingerprint.

## Fix

- `puzzle3dSingleLetterPortFamiliesCompatible`: b-* and c-* single-letter hyphen families never mate (concrete forest beam vs column).
- `resolvePuzzle3dKindCompatibility` / `kindCompatibilityFromFixtureMeta`: fall back to fixture meta when host omits rules.
- `Canvas3D` resolves compatibility from fixture meta; brush collision probes pass rules through to WASM sync.
- Rust WASM parity for single-letter port family guard.

## Files

- `puzzle/3d/react/index.tsx`
- `puzzle/3d/rs/lib.rs`
- `puzzle/3d/play/index.ts`
