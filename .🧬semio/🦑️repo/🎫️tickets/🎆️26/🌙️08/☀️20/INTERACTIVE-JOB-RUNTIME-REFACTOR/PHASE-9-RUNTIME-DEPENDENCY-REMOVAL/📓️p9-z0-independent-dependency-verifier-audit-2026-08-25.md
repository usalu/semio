# P9 Z0 Independent Dependency-Verifier Truth Audit — 2026-08-25

## Verdict

**RED.** The all-ecosystem collector faithfully reflects the present direct manifest identities and the red literal-zero command fails as intended. The truth-classifier is nevertheless not a valid Z0 authority: its Bun/Nx exception is identity-wide rather than root-owner-scoped, and it accepts an open-ended `@nx/*` namespace. Therefore it can hide a non-root tooling dependency merely because the same package identity occurs at the root.

No production source, manifest, lock, baseline, launch file, or test fixture was changed by this audit. The files named below are execution records only.

## Executed Gates

| Gate | Result | Evidence |
| --- | --- | --- |
| `bun ./📜️script.ts verify dependencies self-test` | GREEN — `hostile-mutations=8 clean` | `📝️p9-z0-independent-self-test-2026-08-25.txt` |
| `… verify dependencies summary --format json` | GREEN — executed and emitted current report | `📝️p9-z0-independent-summary-2026-08-25.json` |
| `… verify dependencies list all --raw` | GREEN — executed | `📝️p9-z0-independent-raw-2026-08-25.json` |
| `… verify dependencies list all --literal-external` | GREEN — executed | `📝️p9-z0-independent-literal-2026-08-25.json` |
| `… verify dependencies literal-external --format json` | GREEN fail-closed — exit 1 at `current=210`, `oracle-conflicts=3`, `toolchain-failures=0` | `📝️p9-z0-independent-zero-target-2026-08-25.{json,stderr}` |
| `… verify dependencies` | GREEN ratchet — baseline 232, current 228, no new identities; four removals | `📝️p9-z0-independent-ratchet-2026-08-25.txt` |
| independent direct-manifest identity census | GREEN — no missing verifier identities | `📝️p9-z0-independent-direct-manifest-census-2026-08-25.json` |
| `git diff --check -- 📜️script.ts` | GREEN — exit 0 | live command output |

No Cargo, Nx, Wasm, browser, product build, baseline write, manifest write, lock reconciliation, or launch-file mutation was run.

## Live Census

The report's raw and literal lists were reconciled against an independently parsed direct-manifest census. That census covered 123 non-Composition Cargo manifests, 85 non-Composition `package.json` manifests (the verifier's stricter skip set excludes six non-product tooling/static directories), four non-Composition `go.work` modules, two non-Composition Python manifests, and one non-Composition .NET project. It found 85 Rust, 70 JavaScript, 58 external Go, 15 Python, and zero .NET identities. The verifier added exactly the two genuinely present Go first-party identities; it had no identity missing from the direct census.

| Ecosystem | Raw | Third-party | First-party | Composition-scoped | Mandated-toolchain (reported) | Literal external (reported) | Production-reachable literal |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Rust | 85 | 85 | 0 | 0 | 0 | 85 | 65 |
| JavaScript | 70 | 70 | 0 | 0 | 3 | 67 | 31 |
| Go | 60 | 58 | 2 | 0 | 0 | 58 | 58 |
| Python | 15 | 15 | 0 | 15 | 0 | 0 | 0 |
| .NET | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| **Total** | **230** | **228** | **2** | **15** | **3** | **210** | **154** |

The honest kind census is Rust `65 runtime + 2 build + 3 runner + 20 oracle`; JavaScript `31 runtime + 41 tooling + 3 oracle`; Go `11 runtime + 58 build` (the categories overlap per identity); Python `15 runner`; .NET none.

The two present first-party Go identities are `github.com/usalu/semio/repo/client` and `github.com/usalu/semio/repo/go`. Their actual replacements resolve to the local CLI/library directories, and their manifest-census exclusion is correct on this snapshot. The root Python project has a non-empty `compose/py`, `compose/engine` UV workspace and all 15 detected rows belong only to root `pyproject.toml`; the separate non-Composition Python manifest has no dependency rows. The current .NET project has no `PackageReference`.

The three registered-oracle collisions remain visible as runtime entries and as conflicts: `rust:image` (five product owners), `rust:png` (three), and `rust:zip` (two). The remaining 20 Rust and three JavaScript oracle rows have only `🧪️oracle`/`🧪️test` owners. Thus the present product/oracle handling is honest and the zero-target command does fail closed.

## Decisive Z0 Failures

1. `dependencyTruthRootToolchain` approves every root `devDependencies` name satisfying `name === "nx" || name.startsWith("@nx/")`. This is an open namespace, not the exact mandated identity set. A future root `@nx/*` package is automatically removed from literal external inventory without a policy decision or hostile test.

2. `dependencyTruthReportFromEntries` checks only the merged entry name and all-tooling kind; it never requires `entry.users` to equal root `package.json`. The live raw report proves the bug now:

   - `nx`: `package.json` **and** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`;
   - `@nx/devkit`: the same two owners;
   - `@nx/js`: root-only.

   All three are reported as `mandated-toolchain`, so the non-root Library declarations are hidden. Under the required root-only exception, at least `nx` and `@nx/devkit` must remain literal external. The reported JavaScript literal count 67 and total literal count 210 are therefore understated by **at least two**; owner-scoping alone yields JavaScript **69** and total **212**. Production-reachable remains 154 because these two rows are tooling, but the literal-total and exception semantics are false.

3. Go local replacement recognition is lexical only. `dependencyParseGoModule` treats any relative, absolute, or drive-path replacement target as local and never establishes that the target exists or resolves inside the repository/workspace. The current two replacements are real, so the live census is not over-counted; however a missing/out-of-repository local-looking target would be incorrectly excluded. The existing self-test covers a relative replacement but not a non-existent or escaping local target, so the claimed general "genuinely local" invariant is unproven.

## Diff and Boundary Audit

There are no staged changes. The unstaged packet diff modifies root `📜️script.ts` dependency-verifier surfaces only: command routing, collectors/classifiers, report/self-test support, and baseline serialization. No unstaged or staged path matches `🔒️dependencies.json`, `Cargo.toml`, `Cargo.lock`, `package.json`, `bun.lock`, `go.work`, `go.sum`, `pyproject.toml`, `*.csproj`, `.vscode/launch.json`, or `📋️project.json`.

The ratchet's four reported removals (`go:github.com/usalu/semio/repo/client`, `go:github.com/usalu/semio/repo/go`, `python:pypdf`, `python:simplejson`) were not written to the baseline; the committed baseline remains 232 and current third-party inventory is 228.

## Required Repair Before GREEN

- Replace the `@nx/*` prefix rule with an explicit, reviewed toolchain identity set.
- Require a mandated-toolchain entry to have only root `package.json` as its owner; mixed-owner entries must remain literal external (or be represented per declaration rather than by globally merged identity).
- Resolve every candidate Go `replace` target relative to its declaring `go.mod`, require an existing local module inside the allowed repository boundary, and add hostile fixtures for missing, escaping, and remote replacements.
- Rerun the same source-only gates and direct-manifest census after repair. Do not write the baseline while literal-zero remains RED.
