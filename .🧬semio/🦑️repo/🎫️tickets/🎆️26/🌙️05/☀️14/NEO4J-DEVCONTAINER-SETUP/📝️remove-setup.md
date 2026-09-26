# Remove Dev Neo4j Repo Setup

Removed the root development Neo4j setup: devcontainer Neo4j/APOC installation, ports, volume, environment, and startup; native Windows/macOS/Linux bootstrap installation and graph import; the root Neo4j MCP/export/purge commands and task registrations; generated graph snapshots; and setup-specific documentation and agent grants.

Preserved the hub Neo4j implementation, framework Neo4j storage implementation and conformance tests, and the separate `♻️mit-bestand/🔎️recherche` Neo4j project and MCP configuration. The ordinary root `repo` and `semio` MCP servers remain configured.

## Verification

- Root TypeScript transpilation and project/package/launch JSON/JSONC parsing succeeded.
- `bash -n` passed for both native bootstrap and devcontainer startup scripts.
- Docker Compose configuration validation succeeded.
- VS Code reported no errors for the Windows PowerShell bootstrap; `pwsh` is unavailable in this environment.
- Active setup-reference scans found no Neo4j devcontainer/native/MCP/export/purge wiring outside the preserved hub, research, and framework storage surfaces.

## Ticket Workflow

The repository MCP ticket reopen and ticket-open calls returned `invalid tool params`. Ticket metadata was left unchanged; this note records the work in the existing `NEO4J-DEVCONTAINER-SETUP` folder.