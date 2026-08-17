# Terra Graph Manifest TypeScript Zero-Consumer Acceptance

## Scope

- Ticket: `2026/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`
- Baseline HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`
- Removed `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🟦️component.ts`.
- Removed its Math TypeScript barrel export and TypeScript-program include.
- Retained the Jack DSL export and both live package scripts.
- Reduced the package description to the graph Jack DSL and retained only the `.` package export.

## Baseline Verification

| Path | Required SHA-256 | Observed SHA-256 |
| --- | --- | --- |
| `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🟦️component.ts` | `f0c0f74bed3b1b69688c5c383341d914c261cc147af4b8bcd8aeec0c9b11af53` | `f0c0f74bed3b1b69688c5c383341d914c261cc147af4b8bcd8aeec0c9b11af53` |
| `🧰️framework/🔨️modules/🧮️math/📦️packages/🟦️typescript/📦️index.ts` | `dbbd6fdea254aa8f21ab80976ad17a4de2afc60e79463d5c358c1ac5a018c40f` | `dbbd6fdea254aa8f21ab80976ad17a4de2afc60e79463d5c358c1ac5a018c40f` |
| `🧰️framework/🔨️modules/🧮️math/📦️packages/🟦️typescript/tsconfig.json` | `29f2a0c6e4f7b7c3a3f9e616702604a4b34d862ec1ca7d16de848a432c936956` | `29f2a0c6e4f7b7c3a3f9e616702604a4b34d862ec1ca7d16de848a432c936956` |
| `🧰️framework/🔨️modules/🧮️math/📦️packages/🟦️typescript/package.json` | `f32f64254a7504e9c763877320b636b9133d34aa1e87c5e99988645bce7e73e9` | `f32f64254a7504e9c763877320b636b9133d34aa1e87c5e99988645bce7e73e9` |

All four scoped ordinary and cached diffs were empty before the edit.

## Post-Edit Verification

- The authored TypeScript component path is absent.
- `validateGraphManifestArtifact`, the removed component path, and the removed `./🔣️manifest.schema.json` export have zero matches outside `.🦑️repo` and dependency metadata.
- The package manifest parsed successfully. Its description is `math · TypeScript surface of the semio math framework: the graph Jack DSL`; exports are exactly `.` → `./📦️index.ts`; scripts remain `generate` and `test`.
- The scoped ordinary diff contains only the requested deletion plus the three requested Math TypeScript manifest/assembly edits. The scoped cached diff is empty.

| Retained Path | SHA-256 After Edit |
| --- | --- |
| `🧰️framework/🔨️modules/🧮️math/📦️packages/🟦️typescript/📦️index.ts` | `c4c15027456fb246a22a0fa21f778e8fb714a90235e1ceda5282eb6ce206727d` |
| `🧰️framework/🔨️modules/🧮️math/📦️packages/🟦️typescript/tsconfig.json` | `0231eebadde9fe0b0b27f9679450fb4faa911d6e74b1a189a363c218ba60e74f` |
| `🧰️framework/🔨️modules/🧮️math/📦️packages/🟦️typescript/package.json` | `462a681faf37e78d06653a12ebdb883e44ab61b28248a9b62187611e59812363` |

## Required Validation

```text
$ bun nx run @semio-tech/framework-math-js:test-quick --skip-nx-cache

> nx run @semio-tech/framework-math-js:test-quick
> bun ./📜️script.ts test quick

RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧮️math/📦️packages/🟦️typescript

No test files found, exiting with code 0

include: ../../🕸️graph/🗣️dsl/🟦️component.ts
exclude:  **/node_modules/**, **/.git/**

NX   Successfully ran target test-quick for project @semio-tech/framework-math-js
```

Exit code: `0`. Bun also emitted its non-failing `NO_COLOR`/`FORCE_COLOR` environment warning before Vitest started.
