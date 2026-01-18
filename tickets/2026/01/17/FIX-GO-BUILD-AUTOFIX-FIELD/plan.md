# Plan

## Problem
Go build fails with three errors in `go/repo/main.go`:
1. `autofixType` is undefined (line 8744)
2. `violation.Autofix` field doesn't exist on `*Violation` struct (lines 8747, 8750)

## Root Cause
The GraphQL schema has an incomplete "autofix" field definition that references:
- `autofixType` - a GraphQL type that was never defined
- `violation.Autofix` - a field that doesn't exist on the `Violation` struct

The new autofix mechanism directly applies fixes via functions rather than serializing autofix objects.

## Solution
Remove the incomplete "autofix" GraphQL field from the `violationType` definition (lines 8743-8752).

The "autofixable" boolean field (lines 8736-8741) remains valid and should be kept.

## Files to Modify
- `go/repo/main.go` - Remove lines 8743-8752
