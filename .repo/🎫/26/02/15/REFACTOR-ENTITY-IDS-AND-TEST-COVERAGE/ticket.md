---
goal: SEMIO-REPO/SPECS
---

# Ticket

## Summary

Removed @ from project URIs and aligned ID/URI tests.

## Changes

- Updated project URI generation in `repo/cli/main.go`:
- `Project.GetURI` now returns `semiorepo://project/<NAME>` without `@`.
- `GetArtifactURI("project")` now returns `semiorepo://project/<NAME>` and trims leading `@` from source name.
- `IdToUri` now renders project IDs as `semiorepo://project/<PROJECT>`.
- `UriToId` now parses both `project/<PROJECT>` and `project/@<PROJECT>` by stripping optional `@`.
- Updated URI expectations in `repo/cli/main_test.go` for:
- `TestArtifactIDAndURI`
- `TestIdToUri`
- `TestUriToId`
- `TestIdUriRoundTrip`

## Log

- Checked repository metadata and tree output with `./repo/cli/cli tree`.
- Located all project URI builders and assertions via `git grep`.
- Applied source and test updates.
- Verified tests:
- `go test ./repo/cli -run 'TestArtifactIDAndURI|TestIdToUri|TestUriToId|TestIdUriRoundTrip'`
- Verified runtime behavior from updated source:
- `go run ./repo/cli tree semio --only-project --json | grep -o 'semiorepo://project[^\" ]*' | head -n 10`
- Observed output contains `semiorepo://project/SEMIO` and no `@` prefix.

## Todos

- [x] Remove `@` from project URI rendering.
- [x] Align URI/id conversion logic.
- [x] Update existing tests.
- [x] Run targeted tests.
- [x] Confirm runtime behavior via CLI command output.

## Plan

- Identify all project URI production and parsing points.
- Refactor project URI formatting to canonical no-`@` form.
- Update existing tests to match canonical URI.
- Validate with targeted tests and runtime CLI output.
