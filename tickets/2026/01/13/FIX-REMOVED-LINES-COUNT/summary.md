# Summary

Fixed the bug where ticket file metrics always showed removed lines as 0.

## Changes
- Added `DiffLines` struct to hold both added and removed line numbers
- Updated `GetGitDiffLines` to parse both `-start,count` (removed) and `+start,count` (added) from git diff hunk headers
- Updated `computeAffectedSections` to calculate and report both added and removed lines per section

The fix correctly parses the git unified diff format which uses `@@ -old_start,old_count +new_start,new_count @@` to indicate line changes.
