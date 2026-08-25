# P9 Z0 Second Independent Owner-Scope Audit — 2026-08-25

## Verdict

**GREEN for the owner-scoped truth remediation.** The dependency verifier no longer grants an identity-wide Bun/Nx exemption. The only fully exempt identity is root-owned `@nx/js`; `nx` and `@nx/devkit` retain their root authorization evidence while their non-root Library declarations are literal external and explicit conflicts.

The repository remains intentionally **RED against the literal-zero target**: `literal-external` exits 1 at `current=212` (target `0`), with three product/oracle conflicts and two toolchain-owner conflicts. This is the required fail-closed outcome, not an audit failure.

No source, baseline, manifest, lock, or launch configuration was changed by this audit. No Cargo, Nx, Wasm, browser, or product build was run.

## Reproduced Gates

| Gate | Result | Evidence |
| --- | --- | --- |
| `verify dependencies self-test` | GREEN — `hostile-mutations=13 clean` | `📝️p9-z0-second-self-test-2026-08-25.txt` |
| `verify dependencies summary` and JSON summary | GREEN | `📝️p9-z0-second-summary-2026-08-25.{txt,json}` |
| `list all --raw` | GREEN — 230 | `📝️p9-z0-second-all-raw-2026-08-25.json` |
| `list all --literal-external` | GREEN — 212 | `📝️p9-z0-second-all-literal-2026-08-25.json` |
| `list js --raw` / `list js --literal-external` | GREEN — 70 / 69 | `📝️p9-z0-second-js-{raw,literal}-2026-08-25.json` |
| `literal-external --format json` | GREEN fail-closed — exit 1 | `📝️p9-z0-second-zero-target-2026-08-25.{json,stderr}` |
| direct-manifest declaration census | GREEN — 230 unique identities, 853 declaration rows, zero missing source declarations | `📝️p9-z0-second-direct-manifest-census-2026-08-25.json` |
| TypeScript `transpileModule` diagnostic check | GREEN — `typescript-parser-errors=0` | `📝️p9-z0-second-typescript-parser-2026-08-25.txt` |
| `git diff --check -- 📜️script.ts` | GREEN | `📝️p9-z0-second-diff-check-2026-08-25.txt` |

## Exact Live Truth

| Ecosystem | Raw | Third-party | First-party | Composition-scoped | Fully mandated | Literal external | Production-reachable |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Rust | 85 | 85 | 0 | 0 | 0 | 85 | 65 |
| JavaScript | 70 | 70 | 0 | 0 | 1 | 69 | 31 |
| Go | 60 | 58 | 2 | 0 | 0 | 58 | 58 |
| Python | 15 | 15 | 0 | 15 | 0 | 0 | 0 |
| .NET | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| **Total** | **230** | **228** | **2** | **15** | **1** | **212** | **154** |

Kind census remains Rust `65 runtime + 2 build + 3 runner + 20 oracle`, JavaScript `31 runtime + 41 tooling + 3 oracle`, Go `11 runtime + 58 build`, and Python `15 runner`.

## Owner-Scoped Bun/Nx Evidence

The implementation has an exact set: `nx`, `@nx/devkit`, and `@nx/js`; it authorizes only declaration rows owned by root `package.json` and classed `repository-tooling`. It records `literalExternalUsers` and `mandatedToolchainUsers` separately for a mixed identity.

Root `package.json` declares Bun `bun@1.2.5`, `engines.bun >=1.2.0`, and all three Nx identities at `21.6.11`. Its three rows are authorized and each is owned by the root `bun.lock` workspace snapshot. The Library TypeScript package separately declares `nx@^21.4.1` and `@nx/devkit@21.4.1`; both corresponding Library lock rows exist and match. The resulting lock ownership is **5/5**.

| Identity | Root authorization | Non-root declaration | Literal result |
| --- | --- | --- | --- |
| `@nx/js` | root only | none | fully mandated; excluded from literal list |
| `@nx/devkit` | root `21.6.11` | Library `21.4.1` | literal external; both owner evidences retained |
| `nx` | root `21.6.11` | Library `^21.4.1` | literal external; both owner evidences retained |

The raw JS list contains all three, the literal JS list contains only `nx` and `@nx/devkit`, and the mandated list contains only `@nx/js`. The hostile fixtures explicitly reject a non-root Nx escape, identity-wide mixed ownership, unowned lock evidence, an undeclared `@nx/*` name, and an illicit root tooling package.

## Preserved Classifier Truth

- The first-party Go identities are `github.com/usalu/semio/repo/client` and `github.com/usalu/semio/repo/go`. Their live replacement targets resolve to existing in-repository CLI and Library modules whose declared identities match; neither reaches literal external inventory.
- Root Python is scoped only because its UV workspace has non-empty members `compose/py` and `compose/engine`. The hostile fixture keeps an external `product/pyproject.toml` dependency literal external.
- Product use cannot hide behind an oracle registration. The live fail-closed output names Rust `image` (five product owners), `png` (three), and `zip` (two) as oracle conflicts; their product/runtime classifications remain present.
- Zero mode requires literal external to equal zero and also requires zero oracle conflicts, zero toolchain owner conflicts, and zero toolchain audit failures. The live command reports `target=0, current=212, oracle-conflicts=3, toolchain-owner-conflicts=2, toolchain-failures=0` and exits 1.

## Boundary Check

The packet diff is restricted to root `📜️script.ts` (316 additions, 44 deletions). Both unstaged and staged boundary checks found no change to `🔒️dependencies.json`, package manifests, `bun.lock`, Cargo or Go manifests/locks, Python manifests, .NET projects, or `.vscode/launch.json`. The baseline remains untouched.
