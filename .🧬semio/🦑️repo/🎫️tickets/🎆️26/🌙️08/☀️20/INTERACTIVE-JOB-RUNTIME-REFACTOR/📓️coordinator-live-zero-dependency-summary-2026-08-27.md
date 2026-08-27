# Live Zero-Dependency Summary

Date: 2026-08-27

## Scope correction

The governing plan declares `./compose` out of scope. The dependency freeze already excluded
`compose/`, but its generic Cargo-manifest walker still admitted generated `temp/compose/`
manifests. That made 28 composition-only identities appear as new repository dependencies.

`dependencyIsCompositionManifest` now excludes both canonical and generated composition trees.
The dependency hostile suite proves both excluded forms and a nearby `temp/owned` non-composition
path; `bun ./📜️script.ts verify dependencies self-test` reports `hostile-mutations=18 clean`.

## Fresh census

`bun ./📜️script.ts verify dependencies summary` reports:

| Ecosystem | Third-party identities | Literal external | Production reachable |
| --- | ---: | ---: | ---: |
| Rust | 77 | 77 | 76 |
| JavaScript | 71 | 68 | 31 |
| Python | 15 | 15 | 0 |
| Go first-party | 0 | 0 | 0 |
| Total | 163 | 160 | 107 |

The three JavaScript rows excluded from the literal target are the exact AGENTS-mandated Nx
toolchain rows. The zero target remains `0`; `meets-target=false`.

## Freeze result

The ratchet now sees only two genuine new identities rather than the 30-row scope-contaminated
result:

- `js:fast-glob@3.3.3`, repository tooling owned by the repo TypeScript policy library;
- `rust:byteorder@1.5.0`, test-runner dependency owned by the framework plugin crate.

`bun ./📜️script.ts verify dependencies` correctly remains red. The baseline was not rewritten.
Both identities require owned replacement/removal or an exact temporary oracle disposition; neither
may be normalized into the baseline.
