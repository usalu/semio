# Action Argument Resolution Extraction Acceptance

## Baseline

- `🧰️framework/🔨️modules/🎯️action-bus/🟦️component.ts` was clean at SHA-256 `9d17344e90aa8d9afe20fbce64d61c12a59a4e3df472c334f8b90598db629978`.
- The destination `🧰️framework/🔨️modules/🧮️action-argument-resolution/🟦️component.ts` was absent.
- Production consumers use `effectiveActionArgs` and `missingRequiredArgs` through `@semio-tech/framework`: framework `UIDialog`, OS renderer `ShellHelpers`, and the ui-react target.

## Implementation

- Removed only `effectiveActionArgs`, `missingRequiredArgs`, and the now-unused `ActionArgDef` type import from `🧰️framework/🔨️modules/🎯️action-bus/🟦️component.ts`; the remaining file owns utility and tool derivation.
- Created `🧰️framework/🔨️modules/🧮️action-argument-resolution/🟦️component.ts` with exactly the two extracted functions, its repository-owned manifest `ActionArgDef` type import, and an action-argument-resolution region/responsibility docstring.
- No forwarding export was added to the old component. The coordinator remains responsible for the direct root TypeScript assembly export.

## Validation

- `bun --check` completed successfully for both the retained action-bus and new action-argument-resolution leaves.
- The new relative manifest type-import target exists. The old source contains neither extracted helper; the new source declares both exactly once.
- `git diff --check` and an equivalent no-index whitespace check completed cleanly.
- Post-edit SHA-256:
  - `🧰️framework/🔨️modules/🎯️action-bus/🟦️component.ts`: `e3ab0e4ef72494f28c794f0d34a6ff70bae451bd6a2201ee002abe851dba9207`.
  - `🧰️framework/🔨️modules/🧮️action-argument-resolution/🟦️component.ts`: `04a708965baa5a25e7a3e6cf85c0c6011c06f4bdc5cde94ebf94aba5c9b5bc2e`.

## Consumer Evidence

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️UIDialog/🟦️component.tsx` uses both helpers to resolve dialog values and determine missing required fields.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx` uses both helpers in action execution and staged-argument flows.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` is an additional production consumer. The consumers remain read-only and continue to import through `@semio-tech/framework`.

## Handoff

- The coordinator added the direct root TypeScript assembly export at `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`; its SHA-256 is `45f9e589322aaf7001ef89750d3fc9a89c04a7bdeb4b5b647a83eabc0ac2b743` and it directly exports the new leaf.
- `bun nx run @semio-tech/framework:test-quick --skip-nx-cache` exited `0`: `2` files passed, `150` tests passed.
- `bun --check` completed successfully for the retained source, extracted source, and root framework glue. The framework project exposes no narrower declared type-check target; this is the available package-local static check.
- No Git-mutating command was executed. The retained source is unstaged and the new leaf is untracked; the coordinator-owned glue was left untouched by this lease.
