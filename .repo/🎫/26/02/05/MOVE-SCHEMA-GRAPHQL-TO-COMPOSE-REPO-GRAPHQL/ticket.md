---
goal: AI-OPTIMIZED-REPO/REPO-SERVER
---

# Ticket

## Summary

Also fixed VSCode extension codegen.ts schema path reference.

## Changes

- Removed empty typo directory `repo/grapqhl`
- Moved `repo/cli/schema.graphql` -> `repo/graphql/schema.graphql`
- Moved `repo/cli/queries/` -> `repo/graphql/queries/`
- Updated `repo/cli/gqlgen.yml` schema path to `../graphql/schema.graphql`
- Updated `repo/vscode/codegen.ts` schema path from `../go/schema.graphql` to `../graphql/schema.graphql`
