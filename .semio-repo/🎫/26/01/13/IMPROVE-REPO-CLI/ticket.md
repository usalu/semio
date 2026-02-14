# Ticket

## Todos
- [x] Update `GetGitAuthorGithub` to support name-based and email-based fallback lookups.
- [x] Update `CreateTicket` to validate LLM model names against an allowed list and normalize string format.
- [x] Simplify `Range` struct to use flat `Start`/`End` integer line numbers.
- [x] Update GraphQL schema to reflect `Range` changes.
- [x] Fix integration tests in `repo_test.go` to match the new schema structure.

## Changes

## Log
- Analyzed `repo.go` to understand current implementation of author lookup and ticket creation.
- Modified `GetGitAuthorGithub` to implement new lookup logic: Git Config Name -> Email -> Contributor List.
- Added `AllowedLLMs` list and validation logic in `CreateTicket`.
- Simplified `Range` struct and updated `buildSchema` for `Range` type.
- Ran tests and identified failures in `repo_test.go`.
- Systematically fixed `repo_test.go`:
  - Updated `TestSectionsEdges` and `TestDefinitionsEdges` query strings and structs to match `range { start end }`.
  - Removed `TestCommitsEdges` as `commits` field was removed from `Contributor` type.
- Verified all tests pass with `go test`.

## Summary
Improved the repository CLI by implementing stricter LLM validation, simplifying range structures for easier parsing, and enhancing author attribution logic to robustly handle git configuration and contributor lookups.
