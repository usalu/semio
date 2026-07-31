# Ticket

## Todos

# Plan

## Problem

Go build fails with three errors in `./repo/cli/main.go`:

1. `autofixType` is undefined (line 8744)
2. `breach.Autofix` field doesn't exist on `*Breach` struct (lines 8747, 8750)

## Root Cause

The GraphQL schema has an incomplete "autofix" field definition that references:

- `autofixType` - a GraphQL type that was never defined
- `breach.Autofix` - a field that doesn't exist on the `Breach` struct

The new autofix mechanism directly applies fixes via functions rather than serializing autofix objects.

## Solution

Remove the incomplete "autofix" GraphQL field from the `breachType` definition (lines 8743-8752).

The "autofixable" boolean field (lines 8736-8741) remains valid and should be kept.

## Files to Modify

- `./repo/cli/main.go` - Remove lines 8743-8752

## Changes

## Log

# Log

## Investigation

- Identified three build errors in `./repo/cli/main.go`:
  1. `autofixType` undefined at line 8744
  2. `breach.Autofix` field missing at lines 8747, 8750
- Found incomplete GraphQL "autofix" field definition referencing non-existent type and struct field
- Confirmed `Breach` struct (lines 563-571) has no `Autofix` field
- Confirmed no `autofixType` GraphQL type exists in codebase
- Dev confirmed: new autofix mechanism directly applies fixes via functions, no serialization needed

## Fix Applied

- Removed incomplete "autofix" GraphQL field from `breachType` (lines 8743-8752)
- Fixed resulting syntax issue with brace indentation
- Kept "autofixable" boolean field which correctly uses `breach.Autofixable()` method

## Verification

- `go build` completed successfully with no errors

## Summary

# Summary

Removed incomplete "autofix" GraphQL field from `breachType` in `./repo/cli/main.go`. The field referenced an undefined `autofixType` and non-existent `Breach.Autofix` struct field. The new autofix mechanism applies fixes directly via functions rather than serializing autofix objects. Build now succeeds.
