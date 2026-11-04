# Rebase onto Ueli 718

- Original base: 15d755f053c3a94e974e919772f1cd9a3c7845ec
- Target: 45ef25ecbd34d20ab312585ca5ba370b066c9117 (Ueli 718)
- Branch: feat/border-merge
- Preserved 145 modified/untracked/input files in a temporary compressed snapshot, including the ignored Storybook interaction helper.
- Original index is empty. Changes remain uncommitted; no feature commit is implied.
- Stopped this checkout's live Storybook on6011 before changing source base. Other checkout's6010 server is untouched.

Rebase completed. Detailed preservation and validation results follow.

## Preservation Verification

HEAD matches Ueli718 exactly; no branch-only feature commits existed to replay. 141 original files retain exact SHA-256 content. Three overlapping files retain both upstream and local edits:

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚡️ActionGroup/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🍱️view-chrome/🖼️chrome-frame-and-main-window/🦀️.rs`

ActionGroup conflict resolved by retaining shared-frame refs/markers and upstream h-medium height. CSS and Shell merged automatically. Original empty index restored; changes remain unstaged/uncommitted. No unmerged paths remain. The conflict safety autostash fe1486d5867 is retained rather than dropping any stash. The test-only browser environment helper was deliberately updated to reject Nix libraries requiring a newer glibc than this host.

## Validation After Ueli718

- Installed the target lockfile with frozen Bun install; package manifests and locks have no local changes.
- Relevant integration run: 20 files, 712 passing, 8 failing out of 720. Two failures were Chromium launch failures caused by newer Nix libraries requiring an incompatible glibc. Browser helper now filters unsupported libraries and prioritizes the current runtime libc directory; both browser tests pass on rerun (2 passed, 16 unrelated cases skipped). Net results across these runs: 714 passing cases, six remaining failing cases.
- Six remaining assertions concern unchanged Tree/WindowMeasuresTree source and test modules after upstream Tree markup changes: row fill; property-label wrapper; expanded-last-sibling guides; continuous guide wrappers; measure loading/hover guide; nested measure-group guides. No unrelated Tree behavior or expectations were rewritten to force a green rebase.
- Strict source TypeScript check reports six diagnostics in untouched upstream modules: shell-chrome i18n rowControlFocus missing from a Pick key union (2), Tree standard/touch comparisons (4). This check is not passing on the new base.
- Actual App story passed Chromium assertions for shared seams, utility attachment, active/hover precedence and absence of duplicate bottom strokes; browser exceptions: none.
- Quad Layout, Maximize Stack and Content Through Glass all finished their authored play phases with no browser errors. The ticket smoke script now explicitly clicks Detail rather than relying on the upstream play function to choose the active tab.
- Storybook restarted successfully at http://localhost:6011 (launcher PID 2204449). The unrelated server on 6010 was not touched.
- git diff --check passes; no unresolved merge entries and no staged changes.
- Temporary compressed snapshot removed after successful preservation verification. Canonical source files, ignored Storybook inputs, reproducible scripts and reports remain. Only the active Storybook log remains as generated output.
