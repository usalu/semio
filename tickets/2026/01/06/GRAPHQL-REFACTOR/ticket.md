---
slug: GRAPHQL-REFACTOR
prompt: Refactor repo (library only), cli, mcp server and vscode extension to use GraphQL. The cli uses no server but is only command wise invoked. Depending on the query it resolves more nodes (repo, bundle, folder, file, section, definition, contributor, ticket, policy, violationKind, violation). The repo should use gqlgen. The vscode extension should use urql.
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2026-01-06T01:23:14Z"
commit: be4368f95cbb7a23415abb8ab800502ce774c667
iterations:
    - prompt: Refactor repo (library only), cli, mcp server and vscode extension to use GraphQL. The cli uses no server but is only command wise invoked. Depending on the query it resolves more nodes (repo, bundle, folder, file, section, definition, contributor, ticket, policy, violationKind, violation). The repo should use gqlgen. The vscode extension should use urql.
      date:
        started: "2026-01-06T01:23:14Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      declared:
        updated:
            - path: go/repo/main.go
            - path: go/cli/main.go
            - path: go/mcp/main.go
            - path: js/vscode/extension.ts
---
# Previously

# Plan

# Changes