# Active Scope Census Findings

## Taxonomy Scope

| Area | State | Refactor disposition |
| --- | --- | --- |
| `🧰️framework` | mixed | Included in report mode |
| `🧰️framework/🛍️products/🦑️repo` | clean | Included |
| `✏️s/🔌️plugins` | clean area, mixed plugin state | Included in report mode |
| `✏️s/🔨️modules` | mixed | Included in report mode |
| `compose` | exempt | Structurally excluded |
| `🌎️hub` | legacy | Structurally excluded |
| `♻️mit-bestand` | legacy | Structurally excluded |

The SSOT is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` at schema version 2. The per-plugin ambiguity is resolved in favor of inclusion: a clean parent does not suppress a mixed plugin subtree.

## Inventory

- 22,292 active `component.*` files and 26,757 authored active files were detected before generated/vendor/cache/build filtering.
- Active manifests: 145 `project.json`, 70 `package.json`, 97 `Cargo.toml`, 4 `go.mod`, 1 active `pyproject.toml`, and 146 `📜️script.ts`.
- The OS development extension-module and plugin-module generated `*_component.*` output is ignored and must not be counted as authored leaves.
- `go.work` mixes four active repo modules with excluded compose modules; adapters must enumerate module directories rather than run `go list ./...` from the root.

## Lease Topology

- Framework direct module owners: 22; UI is the largest at 94 component files. Kernel, machine, platform, and OS renderer remain quarantined by the dirty lease snapshot.
- Product queues: OS (split by direct module), print, and repo (client/server/library/CLI/native). Repo library export remains a central hot path.
- `✏️s/🔨️modules`: spatial-kernel (3), FEM (16), mindmap (1), imperative (4), lang (0). Mindmap is an isolated initial Terra lease.
- Plugins: 33. Large safe boundaries are stdio by artifact (with glTF first), norm by standard family, architect by program domain, and block/puzzle by artifact/app/extension.

## Scheduling Evidence

| Plugin | Component files | Safe owner boundary |
| --- | ---: | --- |
| stdio | 6,771 | artifact root; Semio split by subset |
| norm | 3,898 | standard family |
| architect | 1,749 | bounded program domain |
| block | 1,085 | artifact/app/extension |
| puzzle | 987 | artifact/app/extension |

All remaining plugins may initially be leased as whole plugin owners, subject to their own nested instructions. This report is read-only evidence; semantic moves require the deterministic graph census and an implementation lease.

## Instruction and Conflict Notes

Twenty-six plugin roots have nested instructions; additional constraints apply in framework UI/assets/3D, product OS/repo, CAD assets, and Trinity LSP. Every worker must reread the path-specific file before mutation. The protected dirty paths are enumerated in [Semantic Module Refactor Lease Snapshot](./semantic-module-refactor-lease-snapshot.md).
