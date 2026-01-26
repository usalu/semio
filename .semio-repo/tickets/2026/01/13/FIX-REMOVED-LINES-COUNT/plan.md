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
- `go/repo/repo.go`
