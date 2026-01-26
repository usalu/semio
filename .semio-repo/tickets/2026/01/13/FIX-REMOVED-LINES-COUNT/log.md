# Log

## Analysis
Investigated the bug where removed lines are always counted as 0 in ticket file metrics.

Root cause identified in `go/repo/repo.go`:
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
