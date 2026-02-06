---
goal: AI-OPTIMIZED-REPO/REPO-SERVER
---

# Ticket

## Summary

Also fixed VSCode extension codegen.ts schema path reference.

## Changes

- Removed empty typo directory `@semio-repo/grapqhl`
- Moved `@semio-repo/cli/schema.graphql` -> `@semio-repo/graphql/schema.graphql`
- Moved `@semio-repo/cli/queries/` -> `@semio-repo/graphql/queries/`
- Updated `@semio-repo/cli/gqlgen.yml` schema path to `../graphql/schema.graphql`
- Updated `@semio-repo/vscode/codegen.ts` schema path from `../go/schema.graphql` to `../graphql/schema.graphql`
