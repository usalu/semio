# Terra Packet: Action Group Direct-Owner Cycle Break

## Objective

Remove the residual ActionGroup-to-React-barrel runtime edge by importing three presentation values from their direct specific module owners. Make no behavioral or API change.

## Lease

Writable source path only:

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚡️ActionGroup/🟦️component.tsx`
- one unique acceptance Markdown in the active ticket

Baseline SHA-256: `1978fe34c166fceba6be70cd478d65b66a5e85e3301347cde77e077b7287e9b4`. Rehash after Flow release and abort on mismatch.

## Exact Change

- Remove `chromeControlGroupShellClass`, `loadingBorderElementClass`, and `waitingBorderElementClass` from the React barrel import.
- Import `chromeControlGroupShellClass` from `../../🔨️modules/🎛️chrome-control-presentation/🟦️component.ts`.
- Import `loadingBorderElementClass` and `waitingBorderElementClass` from `../../🔨️modules/🌀️status-border-presentation/🟦️component.ts`.
- Preserve every function, type, class composition, story, callback, and export unchanged.
- Do not edit the barrel or any registrar.

## Gates

- ActionGroup has zero React-barrel import after the edit.
- Both direct owner hashes are unchanged.
- Scoped ordinary/cached `git diff --check` pass.
- Run `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` and `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` once; classify established unrelated failures without repair.
- Record final SHA, exact diff, and gate results in the acceptance Markdown.
