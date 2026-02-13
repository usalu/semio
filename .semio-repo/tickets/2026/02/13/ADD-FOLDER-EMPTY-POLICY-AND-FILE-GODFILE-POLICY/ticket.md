---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-POLICY-MECHANISM
---

# Ticket

## Summary

Add Folder policy (Illegal/Empty, autofixable) and extend File policy (Illegal/Use Godfile) in semio-repo CLI.

## Changes

- semio-repo/cli/main.go: Add ViolationFolderIllegalEmpty and ViolationFileIllegalUseGodfile violation kinds
- semio-repo/cli/main.go: Add folder and file policy definitions
- semio-repo/cli/main.go: Implement folderPolicy and filePolicy checker functions
- semio-repo/cli/main.go: Add autofix for empty folders in applySystemAutofixes
- .semio-repo/files.json: Create godfile listing all allowed files
- semio-repo/cli/main_test.go: Add tests for both new policies

## Log

## Todos

- [x] Understand existing policy structure
- [ ] Add violation kinds
- [ ] Add policy definitions
- [ ] Implement policy checker functions
- [ ] Add autofix for empty folders
- [ ] Create .semio-repo/files.json
- [ ] Add tests
- [ ] Run tests and verify

## Plan

1. Add ViolationFolderIllegalEmpty and ViolationFileIllegalUseGodfile constants + meta
2. Add "folder" and "file" policy definitions in policies slice
3. Implement folderPolicy (walks repo dirs, checks for empty folders)
4. Implement filePolicy (reads .semio-repo/files.json, checks all tracked files against it)
5. Add folder empty autofix in applySystemAutofixes (os.Remove empty folder)
6. Create .semio-repo/files.json with all current repo files
7. Add tests in main_test.go
8. Run tests
