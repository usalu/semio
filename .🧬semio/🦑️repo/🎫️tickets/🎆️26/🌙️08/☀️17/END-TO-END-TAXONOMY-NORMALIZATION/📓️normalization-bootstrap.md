# End-to-End Taxonomy Normalization Bootstrap

## Immutable Inputs

- Plan source: `/Users/ueli/.codex/attachments/f0f8efe6-2a54-4a92-8280-5ddac83815e7/pasted-text.txt`
- Pinned baseline commit: `9f449b10659b95148c8bcb3f91ce583bf7446973`
- Baseline commit object verified locally: yes
- Working-tree HEAD observed at freeze: `8d9b51f081f42b36722b54f80a5c502d6322f9ca`
- Working tree observed at freeze: dirty from concurrent user and agent work; no Git state mutation is authorized.

## Frozen Scope

The normalization walker operates on every versioned and present path outside `compose/**`. The `compose/**` tree is opaque: it is not traversed by the normalization mechanism, inventoried, renamed, rewritten, validated, used as exception evidence, or followed through symlinks.

Bare `clean` retains deletion-only behavior. Taxonomy mutation is available only through the explicit `clean taxonomy apply` route with a digest-verified plan.

## Opaque Tree Digest

- Algorithm: `sha256-merkle-v1`
- Root: `compose`
- Digest: `a312d352730435c1c2053e7a82545fce53f3d6a00a32d84863f945555717e9dc`
- Files: `4969`
- Directories: `724`
- Symlinks: `3`
- Other nodes: `0`

The digest recursively hashes entry type, permission mode, byte name, file bytes or symlink target, and child digests in byte-sorted order. Symlinks are hashed without being followed.

## Decision Contract

Directory names own semantics. Renameable files own only file-kind identity and therefore use one registered file-kind emoji basename plus the compiler- or tool-required extension chain. Unknown semantic directory mappings, uncertain package roles, unresolved collisions, unsupported reference forms, and unauthorised fixed-name candidates block planning.

Package-language directories are adapter boundaries. They may contain exact external contracts, configuration, declarations, registration, import/re-export wiring, and immediately delegating entry functions; domain behavior must live under its semantic owner outside `📦️packages`.

All outputs are stable-sorted and byte-deterministic. Apply is journaled and two-phase, verifies preimages and plan digests, restores original paths and bytes on failure or cancellation, and rechecks this opaque-tree digest.

## Mutation Gate

No production-tree mutation is permitted until the parallel inventory reports cover schema and policy, references and package boundaries, and repository-wide path/collision/accounting concerns. Ticket reports are the only explorer write allowance.

## Initial Ownership Ledger

| Packet | Authority | Report |
| --- | --- | --- |
| `H-SCHEMA-POLICY` | Read-only schema, taxonomy, clean, policy, fixed-name, emoji and area-enforcement inspection | `📓️h-schema-policy.md` |
| `H-REFERENCES-PACKAGES` | Read-only reference-adapter, generator and recursive package-boundary inspection | `📓️h-references-packages.md` |
| `H-INVENTORY-COLLISION` | Read-only non-compose path accounting, owner shards, Unicode/case/path collision and compose-boundary inspection | `📓️h-inventory-collision.md` |
| `O-00` | Specification, shared interfaces, integration, verification and final decisions | This record and subsequent coordinator reports |

No packet may edit `compose/**`, `AGENTS.md`, Git state, or a production path without an explicit later writer assignment.
