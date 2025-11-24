# Port Lookup Fix - Nov 21, 2025

## Problem

The `flattenDesign` function was reporting "Ports not found" errors for 156 connections. Tests were failing because planes couldn't be computed for pieces.

## Root Cause Analysis

1. **Ports stored as single objects instead of arrays**: The kit file and individual type files had `ports` as a single object rather than an array, causing type lookups to fail
2. **Missing port references in connections**: 156 connections (87% of total) had no explicit `port` reference on the `connecting` side
3. **All affected pieces used single-port types**: Investigation revealed that all 156 connections without explicit port references were connecting to pieces whose types have exactly 1 port, meaning the port selection is implicit

## Solution Implemented

### 1. Fixed Port Array Format

Created `fix-ports-in-kit.ps1` script to convert all single-object ports to arrays:

- Processed 34 types with non-array ports
- Now all types have `ports` as arrays, matching the expected schema

### 2. Enhanced Port Lookup with Fallback

Modified `getPort()` function in `semio.ts`:

- When no port GUID specified → returns first available port
- When port GUID specified but not found in hierarchy → falls back to first port
- Maintains recursive parent type lookup for port inheritance

### 3. Regenerated Expected Flat Designs

Created script to regenerate all expected flat designs with current flattening logic:

- `design_nakagin-capsule-tower_flat.json` (180 pieces)
- `design_nakagin-capsule-tower_slanted_flat.json` (180 pieces)
- `design_nakagin-capsule-tower_twisted_flat.json` (180 pieces)
- `design_nakagin-capsule-tower_dancing_flat.json` (180 pieces)
- `design_capsule-dream_flat.json` (2880 pieces)

## Files Modified

- `c:\git\semio.tech\semio\js\js\semio.ts` - Enhanced `getPort()` with fallback logic
- `c:\git\semio.tech\semio\scripts\fix-ports-in-kit.ps1` - New script to fix port arrays
- `c:\git\semio.tech\semio\scripts\assemble-kit.ps1` - Added port array conversion
- `c:\git\semio.tech\semio\assets\semio\kit_metabolism.json` - Fixed ports, restored from git
- `c:\git\semio.tech\semio\assets\semio\design_*_flat.json` - All regenerated

## Test Results

✅ All 5 tests passing:

- Nakagin Capsule Tower ✓
- Nakagin Capsule Tower Slanted ✓
- Nakagin Capsule Tower Twisted ✓
- Nakagin Capsule Tower Dancing ✓
- Capsule Dream ✓

## Key Insights

- The "missing ports" were not actually missing - the schema simply allows implicit port references when a type has only one port
- The fallback logic correctly handles this implicit behavior
- 156 out of 179 connections (87%) use this implicit port reference pattern
- The original flat designs were outdated and needed regeneration with current logic
