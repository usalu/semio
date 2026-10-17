# Rust Command Data Delivery Qualification

The production Rust compiler port now consumes one immutable, schema-defined command record for cfg probing and compilation. The full 4,096-dependency DATA case demonstrates 4,096 indexed lookups with zero repeated linear searches, with independent Python and SQLite checks. This implements a real architecture and scaling step; output-relevant cache-key separation remains unfinished.

The frozen [checkpoint](rust-command-data-checkpoint-current.md) is SHA256 `349ab14c97a14cb255231d882b2293b7fc41d160ef9958924c6a565796206640`, 17646 bytes. It retains the complete test history, implementation files, genuine failures and source qualifications.

Latest phased matrix: **seven of eight commands pass**. New command tests and strict checks pass in both modes, prototype Source passes, and both complete artifact Source cohorts pass after the required context AST assertion migration. **Release Source still fails at its aggregate deadline (116.898 seconds).** It was not rerun unchanged or given a larger budget. The passing earlier checkpoints do not supersede that failure.

## Delivery Source Qualification

This later census rehashes only the 1,814 previously observed source/config/schema/test paths. Of 104 inherited owned identities, 103 remain stable; all ten implementation/config/schema/test files authored in this slice remain stable. Three historical observed paths are still missing. Maximum observed absolute path length is 191 UTF-16 units. This does not discover all new paths, qualify a complete current execution closure or measure physical repository size.

Two subsequent changes relative to the post-context-fix snapshot were observed:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts` (observed): `3a5eeb1c9326c535d79835b5286ac306b40367d9fd4b3ed8215d3be1dada9fcd` → `a4c9b6fb78cce71a8b804d65d2d1ff0c406dcdbc7216d6266d7fdcf72cc1e597` (164629 bytes).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🟦️.ts` (owned): `9fd15301aa21309a69adcc5cb30add6372ccf1b91f4b491fa5eba0c3bfd5c682` → `406babe06b682be6b9e5335f56d2c80dd56120a3563e930d9e64008228b24419` (16742 bytes).

Both are possible executable/input changes. The Rust graph module is an inherited previously owned runtime, not a file authored in this command-data slice. No attribution, harmlessness, unreachable-path or identical execution assumption is made. No peer changes are reverted. All recorded executions retain their loaded revisions; this census is not a new execution.

## Remaining Work

The command record still contains physical bindings and only the base environment; runtime resource overlays remain. Current selected Rust inputs still include the sealed compiler program and receipt. Complete semantic recipe normalization and actual dispatch/consumption/mutation evidence must precede changing those inputs. Infrastructure-caused broad invalidation and no-unaffected-crate recompilation remain unqualified.

Actual current Native/toolchain/Nx execution, multi-agent shared warm caches, full CPU scheduling, prototype/release behavior, crate growth, platform workflows, historical image handling and the hard allocated-byte/maximum-repository ceiling remain open. Pure DATA projection and Source tests do not prove physical enforcement. Existing Native/default-SDK/publication/compiler/cache-image/OS-inventory holds remain active.

The goal and existing ticket remain active. No AGENTS file, Git state or worktree is modified. Reports and necessary generated failure evidence are retained inside the active ticket.
