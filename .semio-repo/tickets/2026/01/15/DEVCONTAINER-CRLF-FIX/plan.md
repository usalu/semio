# Plan: Fix CRLF Line Endings in post-create.sh

## Problem
The `.devcontainer/post-create.sh` script has Windows-style CRLF line endings, causing errors when executed in the Linux devcontainer:
- `$'\r': command not found` errors throughout
- All commands fail due to carriage return characters

## Solution
1. Convert the file from CRLF to LF line endings
2. Ensure the file is properly formatted for Unix/Linux execution

## Files to Modify
- `.devcontainer/post-create.sh` - Convert line endings from CRLF to LF

## Implementation Steps
1. Read current file content
2. Write file with LF line endings only
3. Verify the fix
