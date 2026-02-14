---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-POLICY-MECHANISM
---

# Ticket

## Summary

Added two new policies to semio-repo CLI: (1) Folder policy with BreachFolderIllegalEmpty (autofixable, removes empty folders) and (2) File policy with BreachFileIllegalUseGodfile (checks files against .semio-repo/files.json). Both policies only run for ScopeRepo, skip excluded dirs (.git, .semio-repo, node_modules, .venv, .nx), and have 8 passing tests. Also created .semio-repo/files.json godfile with 4670 tracked files, added autofix routing in Fix function for folder-level breachs, and fixed a pre-existing build error with interactionResourceType.

## Changes

- semio-repo/cli/main.go: Add BreachFolderIllegalEmpty and BreachFileIllegalUseGodfile statutes
- semio-repo/cli/main.go: Add folder and file policy definitions
- semio-repo/cli/main.go: Implement folderPolicy and filePolicy checker functions
- semio-repo/cli/main.go: Add autofix for empty folders in applySystemAutofixes
- .semio-repo/files.json: Create godfile listing all allowed files
- semio-repo/cli/main_test.go: Add tests for both new policies

## Log

## Todos

- [x] Understand existing policy structure
- [x] Add statutes (BreachFolderIllegalEmpty, BreachFileIllegalUseGodfile)
- [x] Add policy definitions (folder, file)
- [x] Implement folderPolicy and filePolicy checker functions
- [x] Add autofix for empty folders in applySystemAutofixes + Fix routing
- [x] Create .semio-repo/files.json godfile with 4670 tracked files
- [x] Add 8 tests covering both policies (detection, autofix, exclusions, registration)
- [x] Run tests and verify (all 8 pass, no regressions in existing policy tests)

## Plan

1. Add BreachFolderIllegalEmpty and BreachFileIllegalUseGodfile constants + meta
2. Add "folder" and "file" policy definitions in policies slice
3. Implement folderPolicy (walks repo dirs, checks for empty folders)
4. Implement filePolicy (reads .semio-repo/files.json, checks all tracked files against it)
5. Add folder empty autofix in applySystemAutofixes (os.Remove empty folder)
6. Create .semio-repo/files.json with all current repo files
7. Add tests in main_test.go
8. Run tests
