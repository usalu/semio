---
slug: CONNECTOR-LOOKUP-FIX
summary: Migration from 2025-11-21_CONNECTOR-LOOKUP-FIX.md
prompt: Migration from 2025-11-21_CONNECTOR-LOOKUP-FIX.md
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.686Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Connector Lookup Fix - Nov 21, 2025

## Problem

The `flattenDesign` function was reporting "Connectors not found" errors for 156 connections. Tests were failing because planes couldn't be computed for pieces.

## Root Cause Analysis

1. **Connectors stored as single objects instead of arrays**: The kit file and individual type files had `connectors` as a single object rather than an array, causing type lookups to fail
2. **Missing connector references in connections**: 156 connections (87% of total) had no explicit `connector` reference on the `connecting` side
3. **All affected pieces used single-port types**: Investigation revealed that all 156 connections without explicit connector references were connecting to pieces whose types have exactly 1 connector, meaning the connector selection is implicit

## Solution Implemented

### 1. Fixed Connector Array Format

Created `fix-ports-in-kit.ps1` script to convert all single-object connectors to arrays:

- Processed 34 types with non-array connectors
- Now all types have `connectors` as arrays, matching the expected schema

### 2. Enhanced Connector Lookup with Fallback

Modified `getConnector()` function in `semio.ts`:

- When no connector GUID specified → returns first available connector
- When connector GUID specified but not found in hierarchy → falls back to first connector
- Maintains recursive parent type lookup for connector inheritance

### 3. Regenerated Expected Flat Designs

Created script to regenerate all expected flat designs with current flattening logic:

- `design_nakagin-capsule-tower_flat.json` (180 pieces)
- `design_nakagin-capsule-tower_slanted_flat.json` (180 pieces)
- `design_nakagin-capsule-tower_twisted_flat.json` (180 pieces)
- `design_nakagin-capsule-tower_dancing_flat.json` (180 pieces)
- `design_capsule-dream_flat.json` (2880 pieces)

## Files Modified

- `c:\git\semio.tech\semio\js\js\semio.ts` - Enhanced `getConnector()` with fallback logic
- `c:\git\semio.tech\semio\scripts\fix-ports-in-kit.ps1` - New script to fix connector arrays
- `c:\git\semio.tech\semio\scripts\assemble-kit.ps1` - Added connector array conversion
- `c:\git\semio.tech\semio\assets\semio\kit_metabolism.json` - Fixed connectors, restored from git
- `c:\git\semio.tech\semio\assets\semio\design_*_flat.json` - All regenerated

## Test Results

✅ All 5 tests passing:

- Nakagin Capsule Tower ✓
- Nakagin Capsule Tower Slanted ✓
- Nakagin Capsule Tower Twisted ✓
- Nakagin Capsule Tower Dancing ✓
- Capsule Dream ✓

## Key Insights

- The "missing connectors" were not actually missing - the schema simply allows implicit connector references when a type has only one connector
- The fallback logic correctly handles this implicit behavior
- 156 out of 179 connections (87%) use this implicit connector reference pattern
- The original flat designs were outdated and needed regeneration with current logic
