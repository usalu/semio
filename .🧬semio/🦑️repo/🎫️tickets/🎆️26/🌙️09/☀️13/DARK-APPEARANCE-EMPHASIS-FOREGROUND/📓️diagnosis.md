# Dark Appearance Emphasis Foreground

## Symptom

In dark appearance, navbar edge emphasis, tree hover labels (`text-emphasized`), and other `border-emphasized-color` / `--color-emphasized` paints stayed **dark** on a dark surface.

## Cause

Production-safe semantic tokens (`--border-emphasized-color`, `--color-emphasized`, …) were re-declared only on `:root` (`html`). The OS shell scopes surface chrome to `.semio-scope.dark` and clears `.dark` from `documentElement`. Inherited custom properties use the **computed** value from `html` (still light-mode `--foreground`), so emphasis froze to the dark palette token inside the shell subtree.

## Fix

- Re-declare the unlayered semantic emphasis/border/scrollbar block on `:root, .dark`.
- Point `@theme` `--color-emphasized` at `var(--foreground)` directly.
- Align `resolveColorHex` headless fallbacks for `border-emphasized-color` / `color-emphasized` with foreground flipping.

## Verification

`🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📜️script.ts test` — suite cases `unlayered emphasis tokens…` and `resolveColorHex border-emphasized…`.
