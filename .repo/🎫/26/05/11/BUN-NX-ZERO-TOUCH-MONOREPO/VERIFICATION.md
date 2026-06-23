# Verification (2026-05-11)

## Nx + Bun

- Set `NX_WORKSPACE_DATA_DIRECTORY` to `${workspaceFolder}/.nx/workspace-data-terminal` (IDE uses default `.nx/workspace-data`; CLI uses this path to avoid SQLite lock contention on Windows).
- Ran `bun nx run @compose/net:build`: first follow-up run showed `[local cache]` for dependent tasks.
- Ran `bun nx run @compose/net:build --skip-nx-cache` to force a fresh .NET build after Compose changes.

## Compose.NET kit JSON

- Workspace-format assets (`wip.initialKit`, `{ hash, items }` buckets, `updatedAt`) are normalized in `Utility.DeserializeKit` / `Utility.NormalizeKitDocumentJson` (`compose/net/Compose/Compose.cs`).
- `dotnet test Compose.Tests -f net8.0 --filter FullyQualifiedName~Type_Meta_From_Asset` **passed** after rebuild.

## Compose.Tests suite (full)

- Full `Compose.Tests` still reports failures that are **environmental or asset/test drift**, not specific to this change:
  - **compose-store** binary not on `PATH` (`KitWorkflow`, folder roundtrip).
  - Metabolism asset contains types named `\` / `/` (validation “empty report” cases).
  - Several designs in the workspace JSON omit `parent` (e.g. Slanted); flatten tests expect hierarchical `Parent`.
  - Drag/Move partial `design.compose.json` uses `pose`; `Piece` expects `plane`/`center` at top level unless another mapping exists.

## Repo CLI

- Runnable binary is built with: `go build -o repo/client/client.exe ./repo/mcp` (not `repo/client` alone; that tree is `package client`).
