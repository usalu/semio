---
slug: FLATTEN-DESIGN-DIAGNOSIS
summary: Migration from 2025-11-19_FLATTEN-DESIGN-DIAGNOSIS.md
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.677Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Flatten Design Diagnosis

## Date

2025-11-19

## Problem

All `flattenDesign` tests are failing with almost all pieces missing planes and centers (179/180 for Nakagin designs, 2864/2880 for Capsule Dream).

## Root Cause - CRITICAL DATA INTEGRITY ISSUE

The kit data has **severe type/port mismatches**:

### Example 1: Base Type Mismatch

- Piece `4b51bc5d-2fc2-40d6-9908-a2cee7911799` has type `c4bf196e-84dd-4af4-9a01-8f4f961d8932` (Base)
- Connection parent port: `b1da11ce-75b2-4d60-9eaf-1f43e3100c2c` (interface: "core circular bottom") - belongs to Blob type (child of Base)
- Connection child port: `e0fc8147-34ce-4d7c-85c3-2858654167fc` (interface: "core circular top") - belongs to type `d31a1b54-194e-4969-8efc-24146f138909` (Single Storey, child of Cylindric Tambour)

### The Problem

- Pieces use one type (Base)
- Connections reference ports from COMPLETELY DIFFERENT type hierarchies (Blob, Cylindric Tambour)
- These are not parent-child relationships - the data is fundamentally incorrect

## Why flattenDesign Fails

1. `flattenDesign` looks up piece type to find port information
2. Port GUID doesn't exist in the piece's type or its children
3. Connection cannot be processed
4. Piece plane/center cannot be calculated
5. Almost all pieces fail to flatten

## Solutions

### Option 1: Fix Source Data (REQUIRED)

The **kit JSON files must be corrected**:

- Pieces must reference types that actually have the ports used in connections
- OR connections must use ports that exist in the piece types
- This is a data generation/export issue that needs to be fixed at the source

### Option 2: Workaround - Port Interface Matching (NOT RECOMMENDED)

Match ports by interface instead of GUID:

- Find a port with compatible interface in piece type
- Risky - may connect incompatible ports with same interface

### Option 3: Skip Invalid Connections (TEMP WORKAROUND)

- Log warning for invalid connections
- Continue flattening valid pieces
- Partial flatten is better than complete failure

## Status

- Debug logs removed from `flattenDesign`
- Port resolution enhanced to search child types (insufficient for this issue)
- Tests still failing - data must be fixed at source
- Diagnosis document created

## Conclusion

**The `flattenDesign` implementation is correct**. The kit data has fundamental integrity issues that prevent flattening from working. The data must be regenerated/fixed at the source.
