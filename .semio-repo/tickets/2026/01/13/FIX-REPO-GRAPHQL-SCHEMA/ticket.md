---
slug: FIX-REPO-GRAPHQL-SCHEMA
prompt: "Fix inconsistent ticket mechanism and GraphQL schema mismatch causing VS Code extension errors"
status: open
author: GitHub Copilot
date:
  created: 2026-01-13T10:00:00Z
ignore: false
---

# Plan

1. Investigate the GraphQL schema definition in `./semio-repo/cli/graph` and `graphql/repo`.
2. Identify why `Range.start` and `Range.end` are being treated as scalars (`Int!`) instead of `Position` objects.
3. Fix the schema definition in the Go code or the GraphQL schema file.
4. Rebuild the CLI if necessary.
5. Verify the fix using the `Nodes.graphql` query.

# Log

- Created ticket.
