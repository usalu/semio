# MCP Reachable Cargo Membership — 2026-11-06

Read-only census follows the same dependency groups as Cargo preparation: MCP root development dependencies and every normal/build/target path dependency recursively, including inherited workspace paths. NativePoll is also audited explicitly because its package is not currently reachable from MCP declarations. Actual repository scopes and member inventories are selected through the existing first-party Cargo interface; smol-toml independently parses package workspace/dependency fields. No preparation recipe, compiler, implicit member or authority bypass runs.

Packages: 471; workspaces: 3; issues: 0.

