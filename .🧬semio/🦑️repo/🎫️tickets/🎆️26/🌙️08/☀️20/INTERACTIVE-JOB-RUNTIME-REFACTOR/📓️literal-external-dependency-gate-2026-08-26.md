# Literal External Dependency Gate

## Command

```text
bun ./📜️script.ts verify dependencies literal-external --format json
```

Exit: `1`.

## Live totals

The zero target is not met. The verifier initially reported 191 literal external identities and 135 production-reachable identities. Removing the nested repo-library package's unused Nx ownership rows reduced the live total to 189 without changing the production-reachable total.

| Ecosystem | Literal external | Production reachable |
| --- | ---: | ---: |
| Rust | 106 | 104 |
| JavaScript | 68 | 31 |
| Python | 15 | 0 |
| Go | 0 | 0 |
| .NET | 0 | 0 |

The repository contains two first-party Go module identities and one explicitly mandated root Nx toolchain identity. These do not count toward the literal-external total.

## Additional gate failures

The two unauthorized nested toolchain-owner rows in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json` (`@nx/devkit@21.4.1` and `nx@^21.4.1`) were removed. Its package scripts now call the root Nx targets as required, and `bun install --lockfile-only` refreshed `bun.lock`. The follow-up audit reports three authorized, lock-owned root Nx rows and zero unauthorized rows.

There are no oracle-classification conflicts and no audited-toolchain integrity failures.

## Consequence

Phase 9 and Phase 10 cannot close. The command/tool interactivity work is only one critical path; dependency retirement remains a separate 189-identity implementation program with differential parity, performance evidence, owned defaults, and deletion gates for every replacement. Existing source reports do not override this live literal audit.
