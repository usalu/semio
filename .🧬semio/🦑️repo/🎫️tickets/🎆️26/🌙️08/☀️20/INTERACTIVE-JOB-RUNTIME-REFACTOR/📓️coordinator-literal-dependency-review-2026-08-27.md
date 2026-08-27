# Literal Dependency Gate

Executed `NX_DAEMON=false bun x nx run workspace:verify-dependencies-freeze --skip-nx-cache --args='literal-external'` on 2026-08-27. The gate failed as expected: **156 literal external dependencies**, of which **106 are production-reachable**. This replaces older dependency counts for this run; it does not imply an audited migration plan for every package.

| Ecosystem | Literal External | Production-Reachable |
| --- | ---: | ---: |
| Rust | 75 | 75 |
| JavaScript | 66 | 31 |
| Python | 15 | 0 |
| Go / .NET | 0 | 0 |

The mandated Bun/Nx allowance has three authorized lock-owned rows, no unauthorized rows, no oracle conflicts, and no toolchain-owner conflicts. The zero-external target remains unmet. Record: `🧪️coordinator-literal-dependencies-2026-08-27.txt`.
