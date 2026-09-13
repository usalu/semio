# Inactive Devcontainer Helper Preservation

Three unreferenced helper sources are now preserved byte-for-byte under semantic ticket input concerns, with anonymous shell leaves. The current devcontainer lifecycle stays bound to its existing Bun/Nx creation command and start/attach hooks. The README no longer advertises the unused compatibility entrypoint. No new helper, fallback or script route was added.

## Current Consumer Evidence

The completed hidden-path repository ripgrep probe excluded Git metadata, node_modules, repository cache and ticket history. Neither Neo4j helper had a current text caller; compose-entrypoint appeared only in its README heading/compatibility paragraph. The inspected Dockerfile contains no helper COPY or ENTRYPOINT, the Compose service explicitly runs sleep infinity without an entrypoint, and the mounted workspace's post-start hook already owns Neo4j configuration/startup/Bolt readiness. Root dev mcp neo4j already owns the MCP delegation used by client configuration. The three retained sources remain historical authored inputs, not supported alternate entrypoints. The GitKraken launcher and active post-start/post-attach hooks were not moved by this disposition.

## Schema-First And Native Controls

A strict language-neutral fixture records the three source/destination/role identities and current lifecycle projection. Installed Ajv validates its schema, installed TypeScript parses the actual JSONC devcontainer configuration, and installed yaml parses the actual Compose configuration. The closure verifier first failed with0/3 completed moves, then the identical verifier passed3/3 after the changes. Native bash -n accepts all three original and preserved inputs. The scripts were never sourced or executed.

The exact preserved lifecycle is:

- Creation: bun nx run workspace:deps-javascript.
- Start: bash .devcontainer/post-start.sh.
- Attach: bash .devcontainer/post-attach.sh.
- Container command: sleep infinity, with no explicit entrypoint.

Before atomic renames, every original was checked as a regular non-symlink file with unchanged observed bytes and an absent destination. The final verifier confirms absent original paths, regular destinations, direct byte equality to captured originals, native Bash syntax acceptance and zero findings from the actual anonymous-basename classifier. The output carries DEBUG markers. This preserves file content and rename metadata without reformatting. No container build, Neo4j connection, installation, service startup, or shell runtime feature is claimed.

| Original Source | Preserved Input Relative To Ticket | Bytes | Observed Source SHA-256 |
| --- | --- | ---: | --- |
| `.devcontainer/compose-entrypoint.sh` | `📋️devcontainer-intake/🚀️command-forwarding/🐚️.sh` | 257 | `1f42903df4848e70dca85e7a2d72f8ea5b64b33bd7990b1f26088246da0ce62d` |
| `.devcontainer/neo4j-host-forward.sh` | `📋️devcontainer-intake/🔌️bolt-readiness/🐚️.sh` | 852 | `4d310e9a86e0e3d3ef1d3c5e5da0a439e18ba39d61c55026d9eb266e70b3200b` |
| `.devcontainer/neo4j-mcp-run.sh` | `📋️devcontainer-intake/🧩️mcp-delegation/🐚️.sh` | 474 | `922bdde112689e974ad32e8d913830c5fa1120690e0759f1590cfd04d46d2475` |

Hashes identify observed archival provenance, not expected source-body behavior. The latest global filename census predates these three moves and the two Cursor input moves; no unexecuted global count is claimed. Active lifecycle/body ownership remains queued separately.

## Exact Owned Paths

Removed repository sources:

- `.devcontainer/compose-entrypoint.sh`
- `.devcontainer/neo4j-host-forward.sh`
- `.devcontainer/neo4j-mcp-run.sh`

Updated repository documentation:

- `.devcontainer/README.md` — removed only the obsolete entrypoint heading and compatibility paragraph.

Retained ticket-relative authored paths:

- `📋️devcontainer-intake/🚀️command-forwarding/🐚️.sh`
- `📋️devcontainer-intake/🔌️bolt-readiness/🐚️.sh`
- `📋️devcontainer-intake/🧩️mcp-delegation/🐚️.sh`
- `📋️devcontainer-intake/🔣️.json`
- `📋️devcontainer-intake/🧬️schema/🔣️.json`
- `📓️coordinator-devcontainer-input-preservation-2026-09-13.md`

Scoped whitespace check passes. Exact private generated/coordinator/devcontainer-intake scratch was removed after evidence retention; all six authored ticket paths remain.
