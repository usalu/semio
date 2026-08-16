# Graph DSL TypeScript Dead-Package Packet

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`
- Graph DSL TypeScript component SHA-256: `dbc3858ce38101842457d4a53b172e7eeb18b14418604d6b650821a144142211`; clean.
- Math TypeScript package current files: package `462a681faf37e78d06653a12ebdb883e44ab61b28248a9b62187611e59812363`, tsconfig `0231eebadde9fe0b0b27f9679450fb4faa911d6e74b1a189a363c218ba60e74f`, index `c4c15027456fb246a22a0fa21f778e8fb714a90235e1ceda5282eb6ce206727d`, project `2e6466c8b0d49406cfd1f211e2d08c2d305c7ea2bd834791ebf27dd13e78a339`, script `9bc772953bf968505a9da2575d875a5f01511ccbc3a3c21f297f2f25238d481e`, Vitest `de758dba7a79424b2bf5375a1c50b4a419a875f6d1f5adbf4fcc2231dcc93666`.
- The package/index/tsconfig include released TS-01 changes; the other paths are clean.
- OS TypeScript package manifest SHA-256: `418abb9ed16f25fd014ac48fb28faafffc404551ae5028eb7ad89a206fbbcbc7`; clean.
- `bun.lock` SHA-256: `6dafbd22ee4765b5bc54d94c4d413933e1d96cf7ecc1f42890acf29e46c7c976`; clean.
- Launch configuration contains no Math TypeScript project registration.

## Consumer Evidence

Every exported TypeScript graph DSL symbol has zero reference outside its own component. The only reverse edges are the Math TypeScript barrel, tsconfig, Vitest source list, and project/package assembly. No production source imports `@semio-tech/framework-math-js`, `@semio-tech/framework-math`, or the component path. The OS TypeScript package merely declares an unused workspace dependency. Package entrypoints, project glue, tests, and dependency declarations do not qualify as terminal production consumers.

The Rust graph DSL remains live and untouched.

## Atomic Lease

Terra deletes the authored TypeScript graph DSL component and the complete now-empty Math TypeScript package directory. The Sol coordinator is the sole writer for the OS package dependency and workspace lock: remove `@semio-tech/framework-math-js` from OS dependencies, then regenerate the lock with Bun and accept only the matching workspace-package/dependency resolution removals. No launch edit is required.

Terra writable paths:

- `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🟦️component.ts`
- all six files directly inside `🧰️framework/🔨️modules/🧮️math/📦️packages/🟦️typescript`
- one unique Terra ticket Markdown

Coordinator writable paths:

- `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/package.json`
- `bun.lock`

Validation:

```text
bun install --lockfile-only
bun nx show projects
bun nx run @semio-tech/framework-os:test-quick --skip-nx-cache
```

The deleted Nx project must disappear; active reference searches and ordinary/cached diff checks must be clean. Do not touch Rust Math/graph, generated graph files, root Cargo, launch configuration, renderer, stdio, T-01, G-02, or unrelated package manifests.
