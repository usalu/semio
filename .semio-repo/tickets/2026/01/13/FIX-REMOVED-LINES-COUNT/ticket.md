# Ticket

## Todos

# Plan

## Problem Analysis

The ticket closing functionality incorrectly reports removed lines as 0 because:

1. `GetGitDiffLines` (repo.go:5148) only parses the `+start,count` portion of the diff hunk header, ignoring the `-start,count` portion which contains removed line information
2. `computeAffectedSections` (repo.go:5096) only sets `Added: len(exclusiveLines)`, leaving `Removed` at its default value of 0

## Solution

### Step 1: Create DiffLines struct and modify GetGitDiffLines

- Create `DiffLines` struct with `Added []int` and `Removed []int` fields
- Change return type from `map[string][]int` to `map[string]*DiffLines`
- Update the regex to capture both `-start,count` and `+start,count` from hunk headers
- Parse and store both sets of line numbers

### Step 2: Update ComputeTicketFiles to use new return structure

- Adapt to use the new `DiffLines` struct
- Pass both added and removed lines to `computeAffectedSections`

### Step 3: Update computeAffectedSections to calculate removed lines

- Accept both added and removed line arrays
- Calculate removed lines for each section
- Set both `Added` and `Removed` in the `LineMetrics` struct

### Complexity Note

Removed lines reference positions in the OLD file, while sections are parsed from the NEW file. For simplicity, we'll count removed lines based on the hunk's position in the new file (which is approximate but useful for metrics).

## Files to Modify

- `./semio-repo/cli/cli.go`

## Changes

## Log

# Log

## Analysis

Investigated the bug where removed lines are always counted as 0 in ticket file metrics.

Root cause identified in `./semio-repo/cli/cli.go`:

1. `GetGitDiffLines` function (line 5148) only parses the `+start,count` part of diff hunk headers, ignoring `-start,count`
2. `computeAffectedSections` function (line 5096) only sets `Added` field, leaving `Removed` as default 0

## Implementation

1. Created `DiffLines` struct with `Added []int` and `Removed []int` fields (line 148)
2. Updated `GetGitDiffLines` to:
   - Return `map[string]*DiffLines` instead of `map[string][]int`
   - Updated regex to capture both `-start,count` and `+start,count`: `^@@\s+-(\d+)(?:,(\d+))?\s+\+(\d+)(?:,(\d+))?\s+@@`
   - Parse and store both old file (removed) and new file (added) line numbers
3. Updated `ComputeTicketFiles` to use the new `*DiffLines` type
4. Updated `computeAffectedSections` to:
   - Accept `*DiffLines` instead of `[]int`
   - Calculate both added and removed lines for each section
   - Set both `Added` and `Removed` in `LineMetrics`

## Testing

- All repo tests pass
- CLI builds successfully

## Summary

Bulk close

## Changes

- Added `DiffLines` struct to hold both added and removed line numbers
- Updated `GetGitDiffLines` to parse both `-start,count` (removed) and `+start,count` (added) from git diff hunk headers
- Updated `computeAffectedSections` to calculate and report both added and removed lines per section

The fix correctly parses the git unified diff format which uses `@@ -old_start,old_count +new_start,new_count @@` to indicate line changes.
