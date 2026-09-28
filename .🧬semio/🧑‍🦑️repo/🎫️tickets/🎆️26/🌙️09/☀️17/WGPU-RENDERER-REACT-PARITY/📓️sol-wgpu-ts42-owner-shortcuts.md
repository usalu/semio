# WGPU TypeScript 42 Owner and Shortcut Triage

The full WGPU census reported 21 failures in three files. This packet resolves the nineteen failures owned here; the two package-integration generator failures remain with the root owner.

## Retained document owner move

The owner-move law expected seven `terminal_is_fault()` releases but production correctly contained eight. The additional release belongs to `paint_window_projection_step`, the separately retained World Projection tree introduced after the fixture's chrome-walk inventory. It removes and restores `window_projection_documents`, advances its own `UiDocumentFrameCursor`, retries nonterminal work up to `WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES`, records a fault only after terminal/exhausted work, and is invoked from the main window chrome walk. It is therefore a real named chrome walk, not a stray release or a duplicated count.

The neutral fixture `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🖌️wgpu-document-owner-move/🔣️.json` now names `paint_window_projection_step`. The law still requires every production release to occur inside exactly one named walk and every named walk to contain a release; it does not replace the structural check with a numeric allowance.

## OS command shortcuts

All eighteen failures were a test-environment mismatch. `⌨️os-command-shortcuts/🟦️.ts` constructs real DOM elements, dispatches `KeyboardEvent`, and uses Testing Library `userEvent`, while the WGPU Vitest config intentionally defaults the rest of the renderer suite to Node. The first access to `document` failed before any production shortcut assertion ran. The file now declares `// @vitest-environment jsdom`; production shortcut logic is unchanged.

## Receipt

The scoped two-file WGPU run passed 29/29 in 11.78 seconds, with 304 ms test time and 611 ms environment time. The saved receipt is `🗑️generated/sol-flow34/wgpu-owner-shortcuts42.log`.

The remaining two full-census failures are package-integration generator ownership errors for newly imported browser modules. They are unrelated to document ownership or keyboard routing and are owned by the root integration lane.
