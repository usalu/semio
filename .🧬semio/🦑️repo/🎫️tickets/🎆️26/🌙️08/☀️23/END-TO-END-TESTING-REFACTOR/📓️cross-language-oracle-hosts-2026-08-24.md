# Cross-language oracle hosts — Python and npm reference libraries

Date: 2026-08-24. Ticket: 26/08/23/END-TO-END-TESTING-REFACTOR.

## What was broken

`OracleHostPackage.implementation` was dead for every value except `"rust"`. The only call site,
`contributedOraclePackages(repoRoot, discovered, "rust")` inside `materializeRustHost`, passed the
literal. A `{ "implementation": "python", … }` entry parsed, merged into the registry, and was then
discarded. `materializePythonHost` ran bare `python3` with no environment of its own, and there was
no `materializeTypescriptHost` at all.

Two further holes made the field unsafe even once it was read:

* the dependency ratchet classified only *registered oracle packages*, so a distribution reaching an
  adapter through `oracleHostPackages` was never classified;
* `oracleImportsInProduction` matched `use x` / `from "x"` / `require("x")` against every file type
  at once. It could not see `import x` or `from x import` — Python's only two import forms — and
  could not be taught them, because one shared expression would have matched the Rust crate `json`
  against every `import json` in the repository.

## What changed — framework

| File | Change |
|---|---|
| `📦️packages/🟦️typescript/📦️index.ts` | `OracleHostPackage.path` is now optional and load-bearing: **with** a path the entry is local in-repo source linked by path; **without** one it is an external distribution the host must provision. Added `version`, `module` (import name when it differs from the distribution name), `oracleHostModule`, `dependencyEcosystemOf`, `dependencyEcosystemOfRegistryValue`, `externalOracleHostPackages`, `importProbe`. `oracleImportsInProduction` now probes each ecosystem with its own import syntax in its own file types, and also probes external host packages. Recorded production debt is keyed by `(path, package)` instead of `(path, oracleId)`, so a library registered both as an oracle and as a host package is excused once. |
| `📜️script.ts` | `MaterializedHost` carries `problems`; `executeOne` reports them and refuses to run an unprovisioned host. `provisionPythonInterpreter` builds a cache-local venv; `materializeTypescriptHost` resolves declared npm packages from the repo root; `materializeRustHost` reports a path-less Rust entry rather than generating a broken manifest. `loadClassifiedBaseline` folds external host packages into the classified dependency list. |
| `🧬️schema/🔣️component.json` | Added `$defs.OracleHostPackage` and the registry's `oracleHostPackages` / `comparisonProfiles` / `migrationStatus` properties, which the manifests already used. |
| `README.md` | New section "Reaching a reference library — in any language". |
| `🔒️dependencies.json` | `python:pypdf@6.14.2`, `kinds: ["test-oracle"]`, `productionReachable: false`. |

### Python — the choice made

A virtual environment under `.🧬semio/🦑️repo/⚡️cache/tests/hosts/python-env-<digest of the declared
package set>`, created with `--system-site-packages`, reused across runs via a `🧾️packages.json`
stamp, rebuilt when the declaration changes. A declared package is accepted only when it is
importable **and** at the declared version (`importlib.metadata.version`); otherwise it is
`pip install`ed **into the environment**. The system interpreter is never written to. `--system-site-packages`
is what keeps a checkout that already has the pinned version working without a download; the version
check is what keeps that reuse honest.

If the venv cannot be created, or pip cannot satisfy a declaration, nothing is executed and the run
reports the reason and fails. Proven by pinning `pypdf==999.0.0`:

```
[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=1
[test] python oracle host: pypdf==999.0.0 is neither importable nor installable into
       .🧬semio/🦑️repo/⚡️cache/tests/hosts/python-env-a398872606f54d45445fbf3b0a710d0b —
       ERROR: Could not find a version that satisfies the requirement pypdf==999.0.0 …
```

Installation into the venv (never the system) was proven by temporarily declaring `six==1.17.0`
while the machine's interpreter carries 1.15.0:

```
venv six 1.17.0 …/⚡️cache/tests/hosts/python-env-bd24…/lib/python3.9/site-packages/six.py
system six 1.15.0
```

### TypeScript — the choice made

**Resolve, do not install.** The repository has one populated root `node_modules` and one lockfile,
and bun resolves a bare specifier by walking up from the repository root, which is how the existing
`semver` oracle already works. A private per-host install would give the repository two versions of
the same library and no lockfile covering one of them. What was added is verification: a declared
package that does not resolve from the repository root is reported as an unmet declaration before
the host runs, instead of surfacing as an adapter import error that never mentions the manifest.

## The proof case

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/extract-text-pdf-1-4/` — `component.feature` +
`🐍️component.py`. Oracle `pypdf-pdf-1-4-text` (ecosystem `python`, package `pypdf` 6.14.2),
registered together with the `python` host package in
`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️.json`.

Why Python genuinely helps and Rust does not: `lopdf` — the registered PDF editing and parsing
oracle — exposes the object graph and raw content streams. Reconstructing a page's reading order
means decoding the font's `/Encoding` and `/ToUnicode` CMaps, undoing glyph-index and ligature
mappings and re-assembling text-showing operators into lines. No crate linked by the stdio oracle
host does that. `pypdf` does.

Input is the real committed 6.3 MB, 65-page LaTeX bachelor thesis, read in place through `asset://`.
Two scenarios: a `@mode-conformance` table of `| page | contains |` claims about what the document
prints, and a `@mode-property` scenario that every one of the pages yields a non-empty text layer
(136,300 characters over 65 pages).

The case fails when it should: replacing the page-65 claim `BIBLIOGRAPHY` with text that is not in
the thesis produced `executed=2 passed=1 failed=1`.

## Verification (real output)

```
$ bun ./📜️script.ts contract                                   # exit 0
0 high-priority breach(es) across 0 rule(s)

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio          # exit 0
[test] level=exhaustive cases=56 executed=781 passed=781 failed=0 errored=0 parity=0/0 not-exercised=9

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case extract-text-pdf-1-4   # exit 0
[test] level=exhaustive cases=1 executed=2 passed=2 failed=0 errored=0 parity=0/0

$ bun ./📜️script.ts dependency                                 # exit 0
[dependency] ecosystems=4 entries=228 production-reachable=151 test-oracle=26
[dependency] test-oracle python:pypdf@6.14.2 (pypdf-pdf-1-4-text)

$ bun test 🧪️index.test.ts                                     # in 📦️packages/🟦️typescript
 69 pass  0 fail  1269 expect() calls
```

Baseline before this change was 55 cases / 779 scenarios and 62 unit tests.

## Domain neutrality

The framework learned nothing about PDF, text extraction or any format. `📜️script.ts` and
`📦️index.ts` name no package, no plugin and no artifact; `pypdf`, the capability
`pdf-1-4-text-extraction` and the oracle id all live in the stdio plugin's own contribution manifest.
The only vocabulary the framework gained is `path` present vs absent.

## Left open

* `ratchetDependencies` is invoked in `DependencyScript` as `ratchetDependencies(sorted, sorted, …)`,
  comparing the derived baseline with itself, so `newProduction` and `unregisteredTestDeps` can never
  fire from the CLI. The gate is real only in the unit tests. Pre-existing, untouched here.
* `productionDebt` can only be recorded on an oracle registry entry. An external host package with no
  matching oracle entry that turns out to be production-reachable therefore has no escape hatch —
  correct today, but worth a schema slot if it ever bites.
* Go and .NET hosts still provision no external packages; the same `path`-absent rule is the obvious
  extension point (`go get` into a module cache, `<PackageReference>` in the generated csproj).
