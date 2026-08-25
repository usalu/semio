# Phase 9/10 Z0 Dependency Verifier Truth

## Scope

This source/static packet changes only the dependency verifier regions and in-file fixture machinery in root `📜️script.ts`, plus the stale import in the repo technology's existing `📜️script.ts` policy router. It does not change `🔒️dependencies.json`, any product manifest or dependency, `bun.lock`, `go.work`, `go.sum`, `.vscode/launch.json`, or ticket metadata. No baseline was written.

The verifier now provides:

- all-ecosystem `list` support for Rust, JavaScript, Go, Python, and .NET, with `all`, `--raw`, and `--literal-external` selections;
- a `summary` report with raw, actual-third-party, first-party, Composition-scoped, mandated-toolchain, corrected/literal-external, production-reachable, and kind counts per ecosystem;
- a red-until-zero `literal-external` mode with an explicit target of zero;
- first-party Go recognition from every non-Composition `go.work` module identity and every local workspace/module `replace`;
- explicit root Composition Python scope only when `[tool.uv.workspace]` is non-empty and every member is below `compose/`; other Python manifests remain in scope;
- oracle classification only when every declaration belongs to a `🧪️oracle` or `🧪️test` owner; product declarations retain their manifest-derived runtime/build/tooling/runner kind and are reported as conflicts;
- an explicit audited Bun/Nx category derived from root `engines.bun`, `packageManager`, and the exact reviewed package set `nx`, `@nx/devkit`, and `@nx/js`;
- declaration-owner classification for that category: only a `repository-tooling` row owned by root `package.json` is authorized, every non-root row remains literal external, and a mixed-owner identity remains external while preserving its authorized-root-row evidence;
- root and non-root Nx ownership evidence reconciled against the corresponding `bun.lock` workspace snapshot. No open `@nx/*` namespace or generic JavaScript exemption exists.

## Exact Before and After Truth Census

The before census was reproduced from the pre-change verifier with `bun ./📜️script.ts verify dependencies list`.

| ecosystem  | before collector identities | before production-reachable | before kind census                       | after raw | after actual third-party | correction/scoping                                                                               | after literal-external | after production-reachable | after honest kind census                 |
| ---------- | --------------------------: | --------------------------: | ---------------------------------------- | --------: | -----------------------: | ------------------------------------------------------------------------------------------------ | ---------------------: | -------------------------: | ---------------------------------------- |
| Rust       |                          85 |                          62 | 62 runtime, 2 build, 23 oracle, 3 runner |        85 |                       85 | 20 isolated oracles remain oracle; `image`, `png`, and `zip` product owners are no longer hidden |                     85 |                         65 | 65 runtime, 2 build, 20 oracle, 3 runner |
| JavaScript |                          70 |                          31 | 31 runtime, 41 tooling, 3 oracle         |        70 |                       70 | root-only `@nx/js` is excepted; mixed-owner `nx` and `@nx/devkit` remain external                |                     69 |                         31 | 31 runtime, 41 tooling, 3 oracle         |
| Go         |                          60 |                          60 | 13 runtime, 58 build                     |        60 |                       58 | 2 first-party workspace/local-replace identities excluded                                        |                     58 |                         58 | 11 runtime, 58 build                     |
| Python     |                          15 |                           0 | 15 runner                                |        15 |                       15 | all 15 explicitly attributed to the root compose-only UV workspace                               |                      0 |                          0 | 15 runner                                |
| .NET       |                           0 |                           0 | none                                     |         0 |                        0 | none                                                                                             |                      0 |                          0 | none                                     |
| **Total**  |                     **230** |                     **153** | —                                        |   **230** |                  **228** | **2 first-party, 15 Composition-scoped, 1 fully mandated-toolchain identity**                    |                **212** |                    **154** | —                                        |

The corrected production count is 154 because the truthful classifier adds the three Rust product/oracle collisions and removes the two first-party Go identities from the former 153 count. The literal target is explicitly `0`; the live repository is therefore correctly red at `212`, not reported as closed.

The independent-audit repair changed only the owner treatment within the same raw census:

| truth view                            | raw | third-party | first-party | Composition-scoped | fully mandated identities | literal external | production-reachable literal | toolchain owner conflicts |
| ------------------------------------- | --: | ----------: | ----------: | -----------------: | ------------------------: | ---------------: | ---------------------------: | ------------------------: |
| identity-wide exception before repair | 230 |         228 |           2 |                 15 |                         3 |              210 |                          154 |                         0 |
| owner-scoped exception after repair   | 230 |         228 |           2 |                 15 |                         1 |              212 |                          154 |                         2 |

The two Go corrections are `github.com/usalu/semio/repo/client` and `github.com/usalu/semio/repo/go`. The audited toolchain is Bun `bun@1.2.5` with `engines.bun >=1.2.0`, plus the exact reviewed Nx identity set. Root `package.json` contributes three authorized rows and its root `bun.lock` snapshot owns all three at `21.6.11`. Only root-owned `@nx/js` is a fully excepted identity. `nx` and `@nx/devkit` are mixed-owner identities: their root rows remain explicit authorized evidence, while their Library rows remain literal external conflicts. The Library `bun.lock` workspace snapshot owns `nx@^21.4.1` and `@nx/devkit@21.4.1`, so neither non-root declaration is an inventory artifact.

The oracle conflicts are:

- `rust:image`, with product declarations in Animate, Draw, Surface, Infinite, and the WGPU renderer;
- `rust:png`, with product declarations in Lowpoly, Remodel, and the OS host;
- `rust:zip`, with product declarations in the OS package and OS host.

## In-File Hostile Fixtures

`verify dependencies self-test` now rejects these mutations:

1. a workspace-owned Go module retained as external;
2. a locally replaced Go module retained as external;
3. an external indirect Go requirement removed or mislabeled;
4. a direct runtime/product row hidden by an oracle registry name;
5. a genuinely isolated oracle owner not classified as oracle;
6. root Composition Python not explicitly scoped;
7. external Python elsewhere hidden;
8. a first-party Go identity reaching literal-external inventory;
9. an exact root-owned Nx runner row not receiving the narrow exception;
10. the same Nx identity at a non-root owner escaping literal-external inventory;
11. a mixed-owner Nx identity receiving an identity-wide exception instead of splitting authorized and external owner evidence;
12. root or non-root Nx owner evidence not matching its `bun.lock` workspace snapshot;
13. an illicit non-Nx root tool or an undeclared `@nx/*` name escaping literal-external inventory.
14. the repo-owned policy Library module moving or disappearing without an owned dependency-verifier diagnostic;
15. any enumerated repo policy router drifting back to a stale or non-owner import specifier;
16. an expected root, client, or Library policy-only router disappearing from discovery;
17. a newly discovered repo policy-only router remaining outside the audited enumeration.

## Owner-Scope Acceptance Gates

- `bun ./📜️script.ts verify dependencies self-test` — exit 0, `hostile-mutations=13 clean`.
- `bun ./📜️script.ts verify dependencies summary` — exit 0; raw 230, third-party 228, literal-external 212, production-reachable 154, target 0, three authorized rows, two toolchain-owner conflicts, and lock ownership `5/5`.
- `bun ./📜️script.ts verify dependencies summary --format json` — exit 0; exact per-ecosystem kinds, three oracle conflicts, exact reviewed Nx package set, three authorized root rows, two unauthorized non-root rows, and `lockOwned=true` for every audited row.
- `bun ./📜️script.ts verify dependencies list all --raw` — exit 0; Rust 85, JavaScript 70, Go 60 including exactly two first-party dispositions, Python 15, .NET 0.
- `bun ./📜️script.ts verify dependencies list all --literal-external` — exit 0; Rust 85, JavaScript 69, Go 58, Python 0, .NET 0; mixed `nx` and `@nx/devkit` identities name only their Library owners in `literalExternalUsers` and retain root evidence in `mandatedToolchainUsers`.
- `bun ./📜️script.ts verify dependencies list js --literal-external` — exit 0; 69 JavaScript identities, including mixed-owner `nx` and `@nx/devkit`, excluding only root-only `@nx/js`.
- `bun ./📜️script.ts verify dependencies literal-external` — expected exit 1; fail-closed at `current=212`, `oracle-conflicts=3`, `toolchain-owner-conflicts=2`, `toolchain-failures=0` and prints both non-root owners with lock evidence.
- TypeScript `transpileModule` parser diagnostic check over root `📜️script.ts` — exit 0, `typescript-parser-errors=0`.
- `git diff --check -- 📜️script.ts 📓️p9-z0-dependency-verifier-truth.md` — exit 0.
- `bunx prettier --check 📓️p9-z0-dependency-verifier-truth.md` — exit 0.
- `bunx prettier --check 📜️script.ts` — exit 1; the unchanged `HEAD:📜️script.ts` check also exits 1, so this is a pre-existing whole-file formatting gate rather than a Z0 regression.
- Scoped `tsc --noEmit` could not become authoritative because the repository does not install Bun type definitions and the root script/import graph already contains unrelated type failures. The Bun-executed self-test, summary, and list commands parse and execute the entire updated TypeScript file successfully.

No Cargo, Nx, Wasm, browser, product build, baseline write, or manifest mutation was run.

## Shared-Gate Import Drift Repair

The accepted verifier was reported failing before command dispatch because the repo technology policy router imported a deleted Math/Graph DSL implementation path. By the time this repair began, the exact root command was again executable, but the stale source reference remained in `🧰️framework/🛍️products/🦑️repo/📜️script.ts`. That router now imports `defineLint`, `runPolicyOnlyMain`, and `TechnologyLinter` from its existing repo-native Library owner:

`./🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts`

No compatibility shim or external package was added. Every `verify dependencies` command now discovers all repo-product `runPolicyOnlyMain` routers, requires the exact audited router set, and verifies each importer's relative specifier resolves to the same in-repository Library owner. A missing/moved module reports `[verify dependencies import-boundary]` with both owner and target instead of leaking Bun's generic missing-module exception.

The shared workspace's direct Go manifests advanced after the 230/212 owner-scope audit snapshot: the four non-Composition workspace modules now declare only two first-party inter-module requirements and no external Go requirements. The verifier therefore truthfully reports the current manifest snapshot rather than freezing the earlier census:

| ecosystem  |     raw | third-party | first-party | Composition-scoped | fully mandated | literal external | production-reachable |
| ---------- | ------: | ----------: | ----------: | -----------------: | -------------: | ---------------: | -------------------: |
| Rust       |      85 |          85 |           0 |                  0 |              0 |               85 |                   65 |
| JavaScript |      70 |          70 |           0 |                  0 |              1 |               69 |                   31 |
| Go         |       2 |           0 |           2 |                  0 |              0 |                0 |                    0 |
| Python     |      15 |          15 |           0 |                 15 |              0 |                0 |                    0 |
| .NET       |       0 |           0 |           0 |                  0 |              0 |                0 |                    0 |
| **Total**  | **172** |     **170** |       **2** |             **15** |          **1** |          **154** |               **96** |

The owner-scoped Bun/Nx result is unchanged: three authorized root rows, two unauthorized Library rows, lock ownership `5/5`, and only root-only `@nx/js` fully excepted. Oracle conflicts remain three.

### Initial Post-Drift Gates

- `bun ./📜️script.ts verify dependencies list go --literal-external --format json` — exit 0 with `[]`; the former missing-module failure is gone and the result matches the current zero external Go declarations.
- `bun 🧰️framework/🛍️products/🦑️repo/📜️script.ts` — the policy router imports successfully and reaches its owned usage diagnostic.
- `bun ./📜️script.ts verify dependencies self-test` — exit 0, initially `hostile-mutations=15 clean`; the complete router closure below supersedes this with 17.
- text and JSON `verify dependencies summary` — exit 0; raw 172, third-party 170, literal-external 154, production-reachable 96, two owner conflicts, and lock ownership `5/5`.
- `verify dependencies list all --raw` and `list all --literal-external` — exit 0.
- TypeScript parser diagnostics over both changed scripts — exit 0, `typescript-parser-errors=0`.
- scoped `git diff --check` over both scripts and this report — exit 0.

### Fresh-Audit Router Closure

The fresh audit found the client bundle policy router still importing the deleted Math/Graph implementation. The complete router enumeration then exposed another executable stale site beyond the audit finding: the Library folder router imported nonexistent `./src/index.ts`. Both now use the same repo-owned Library boundary:

| policy-only router                               | exact owned specifier                                      | result               |
| ------------------------------------------------ | ---------------------------------------------------------- | -------------------- |
| repo technology `📜️script.ts`                    | `./🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts` | imports successfully |
| client bundle `🔨️modules/💻️client/📜️script.ts`   | `../📚️library/📦️packages/🟦️typescript/📦️index.ts`          | imports successfully |
| Library folder `🔨️modules/📚️library/📜️script.ts` | `./📦️packages/🟦️typescript/📦️index.ts`                     | imports successfully |

The guard rejects both directions of set drift: a required router missing from discovery and a newly discovered router absent from the reviewed enumeration. The hostile suite is now `17 clean`.

Fresh-audit gates:

- client bundle and Library folder policy routers — their expected usage exit 1 is reached with no `Cannot find module` output;
- exact `verify dependencies list go --literal-external --format json` — exit 0, exact `[]`;
- `verify dependencies self-test` — exit 0, `hostile-mutations=17 clean`;
- text/JSON summaries and all raw/literal lists — exit 0 with the unchanged 172/170/154/96 census;
- executable repo-router stale-path scan — no deleted Math/Graph or nonexistent `./src/index.ts` import remains;
- TypeScript parser diagnostics over root verifier, all three routers, and the Library target — five files, zero errors;
- Prettier checks over all three routers and this report — exit 0;
- scoped `git diff --check` over the verifier, all three routers, and this report — exit 0.

No Cargo, Nx, Wasm, browser, product build, baseline write, manifest mutation, lock mutation, or launch mutation was run during either drift repair.
