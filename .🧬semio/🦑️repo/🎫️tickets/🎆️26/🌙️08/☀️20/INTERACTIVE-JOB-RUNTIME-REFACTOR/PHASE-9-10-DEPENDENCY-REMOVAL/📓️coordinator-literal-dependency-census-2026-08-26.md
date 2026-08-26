# Coordinator Literal Dependency Census

Date: 2026-08-26

## Executed Evidence

Command: bun ./📜️script.ts verify dependencies self-test

- Exit: 0
- Hostile mutations: 17 clean

Command: bun ./📜️script.ts verify dependencies summary --format json

- Exit: 0
- Target met: false
- Raw identities: 195
- Third-party identities: 193
- First-party identities: 2
- Mandated Nx toolchain identities: 3
- Literal-external identities: 190
- Production-reachable identities: 135
- Oracle conflicts: 0
- Toolchain-owner conflicts: 0
- Toolchain audit failures: 0

## Ecosystem Ledger

| Ecosystem | Literal external | Production reachable |
| --- | ---: | ---: |
| Rust | 107 | 104 |
| JavaScript | 68 | 31 |
| Python | 15 | 0 |
| Go | 0 | 0 |
| .NET | 0 | 0 |

## Toolchain Exception

The only accepted external toolchain rows are the AGENTS-mandated root Nx packages:

- @nx/devkit
- @nx/js
- nx

All three are root-owned, Bun-lock-owned, and version-aligned at 21.6.11. No nested package receives an Nx exemption.

## Status

The dependency verifier and its hostile fixtures are green. Phases 9 and 10 remain open because 190 literal-external identities still exist; 135 are production reachable.
