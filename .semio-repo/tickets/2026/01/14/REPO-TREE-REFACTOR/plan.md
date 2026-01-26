# Plan - REPO-TREE-REFACTOR

## Backend (Go/GraphQL)
- [ ] Revert `Range` type to use `Int` for `start`/`end` instead of `Position`.
- [ ] Verify/Add `sections` field to `File` type.
- [ ] Verify/Add `definitions` field to `Section` type.
- [ ] Update `repo.go` resolvers for `Range`, `File.sections`, `Section.definitions`.

## Frontend (VS Code)
- [ ] Regenerate GraphQL types.
- [ ] Update `CodebaseProvider` (Tree layout):
    - [ ] Root should be bundles (`repo.bundles`), not the repo itself.
    - [ ] `File` items should represent children as sections.
    - [ ] `Section` items should represent children as definitions.
- [ ] Ensure `TreeItem.collapsibleState` is set to `Collapsed` for expandable items (Bundles, Folders, Files, Sections).
- [ ] Implement `getChildren` logic to fetch deeper levels lazily if not already done.

## Verification
- [ ] Check `Range` type in schema.
- [ ] Check Tree View structure in VS Code (simulated via looking at code).
