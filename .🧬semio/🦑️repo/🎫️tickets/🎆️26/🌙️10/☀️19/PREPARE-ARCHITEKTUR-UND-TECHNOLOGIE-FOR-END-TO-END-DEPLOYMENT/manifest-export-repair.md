# Manifest Public Export Repair

The manifest UI axes generated source already exists at `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🎚️ui-axes/🟦️.ts`. The defect was its public package export pointing to a nonexistent generated file inside the package directory. The authored package convention is an explicit public wrapper for the actual owning source module.

The existing `@semio-tech/framework/manifest/generated/ui-axes` export now targets an authored, nongenerated package wrapper at `🛂️manifest/🎚️ui-axes/🟦️.ts`. That wrapper explicitly reexports the four actual values and three actual types from the existing generated source. The generated source and normative axes schema were not edited or duplicated.

## Source Contract and Runtime Evidence

The neutral workspace-export fixture now records the actual package manifest, public specifier, authored target, owning source and explicit value/type surface. Its new regression compares the repository public export resolver against the independent `resolve.exports` oracle, checks the wrapper's explicit source reexports using the TypeScript AST, and independently resolves the public specifier with TypeScript's module resolver.

- RED before implementation: the targeted Vitest regression failed on the old `./🛂️manifest/🤖️generated/🎚️ui-axes/🟦️.ts` package target instead of the authored wrapper target (1 failed, 19 skipped). An independent `resolve.exports` diagnostic reproduced the same old target.
- GREEN after implementation: the full current resolution suite passed, 20 tests in 1 test file, with Vitest 4.1.12 and the existing native config loader.
- Runtime: importing the actual authored wrapper with the installed Node transform-types support logged `[DEBUG] Public wrapper runtime arrays and membership guards match normative neutral axes`, with locales `en`, `de` and terminologies `native`, `reuse`. The diagnostic compared both arrays to the existing normative axes JSON, accepted all authored ids, and refused an invalid value for both guards.
- Read-only `git diff --check` passed for the four changed source/fixture/test files.

All edits used `ErrorActionPreference = 'Stop'` and before-image guards for shared files. The wrapper is authored source, not a generated-module substitute. This repair does not claim canonical TSC completion: the root agent is rerunning the current-source TypeScript program to check the previously observed eight diagnostics.

## Changed Files

- `🧰️framework/📦️packages/🟦️typescript/package.json`
- `🧰️framework/📦️packages/🟦️typescript/🛂️manifest/🎚️ui-axes/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🧫️fixtures/🔣️.json`
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/📦️resolution/🟦️.ts`
- This report.

The temporary RED and GREEN logs are under this ticket's `🗑️generated` directory and must be removed during final ticket cleanup.
