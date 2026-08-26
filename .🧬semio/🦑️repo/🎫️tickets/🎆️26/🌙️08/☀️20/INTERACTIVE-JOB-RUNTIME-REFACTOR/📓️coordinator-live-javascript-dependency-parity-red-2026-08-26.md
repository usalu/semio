# Live JavaScript Dependency-Parity RED Checkpoint

Date: 2026-08-26  
Command: `bun ./📜️script.ts verify dependencies parity js --no-unowned-rows`  
Exit: `1`

## Live Census

- manifests: `84`
- external rows: `251`
- evidenced rows: `103`
- unowned rows: `148`
- undeclared imports: `7`
- lock workspaces: `45`
- lock mismatches: `1`
- lock fixtures: `5`

## Exact Undeclared Imports

1. Mitbestand Demonstrator Vitest config imports `vitest` without an owning declaration.
2. Sequence protocol oracle imports `ajv` without an owning declaration.
3. Stdio Semio mesh oracle imports `three` without an owning declaration.
4. Framework Kernel version-requirement oracle imports `semver` without an owning declaration.
5. Framework React class-name oracle imports `clsx` without an owning declaration.
6. Framework React style-variant oracle imports `class-variance-authority` without an owning declaration.
7. Flow schema oracle imports `ajv` without an owning declaration.

The command stops on undeclared imports before enforcing the `148` unowned direct rows, so both categories remain RED. These are not silently repaired by adding compatibility declarations: the final contract requires zero external runtime/build/test/tooling dependencies, while repository policy also requires temporary third-party parity oracles during replacement. Each oracle therefore needs an explicitly owned differential-test packet and later dependency retirement; production imports require an owned replacement. Concurrent work owns several listed paths, so this checkpoint records evidence only and makes no manifest/lock mutation.
