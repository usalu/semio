# DiffView Presentation Parity

## Scope

This packet aligns the React and WGPU DiffView presentations against one schema-first corpus and an actual Chromium measurement of the authored React component. It covers unified and split layout, the embedded Share Tech Mono face, semantic added and removed colors, fixed gutters, preserved-space wrapping, scrolling, and paired split-row height.

## Accepted authority

The neutral contract is `framework/ui/schema/diff-view-presentation` with its matching `framework/ui/contract/fixtures/diff-view-presentation` corpus. It records the accepted 400 × 160 surface, 3.2 px inset/gap, 11.2 px Share Tech Mono with 16 px line boxes, semantic colors, unified and split coordinates, and a narrow preserved-space wrap case.

The narrow case uses `a  b  c  d` and expects `a  b  c  ` followed by `d`. The earlier fail-first sample ended in spaces; Chromium correctly kept it on one line because terminal spaces hang under CSS `white-space: pre-wrap`. The revised corpus forces a break while retaining repeated and trailing spaces on the first line.

## Production changes

- React DiffView now uses the dedicated `diff-added` semantic token and the authored destructive alias, gives the text cells their remaining flex width, and lays split pairs in one three-column grid row so both panes advance by the maximum wrapped height.
- The styling palette declares `diff-added` as `#00d492`; generated Rust, TypeScript, Python, C#, and CSS artifacts are refreshed. WGPU `Theme` and custom document themes project the same token.
- The WGPU font atlas keys glyphs by `TextFace`, shapes and rasterizes both Sans and Mono, and exposes face-aware measurement and drawing without changing the existing Sans defaults.
- WGPU DiffView derives one shared metrics object from accepted bounds and theme tokens. Unified and split modes use Mono text, fixed number gutters, semantic ink, preserved-space wrapping, shared split-row heights, host scrolling, and scissoring.

## Evidence

- Fail-first React/Chromium run: `🗑️generated/sol-flow34/diff-react-focused.log` — schema tuple strictness and missing narrow wrap failed.
- Intermediate Chromium diagnostics: `🗑️generated/sol-flow34/diff-react-wrap-debug.log` — verified the host at 160 px, row at 153.625 px, text box at 45.6875 px, `white-space: pre-wrap`, and CSS trailing-space hanging. The temporary `[DEBUG]` statement was removed from source.
- Final React/Chromium oracle: `🗑️generated/sol-flow34/diff-react-focused-final.log` — 3/3 passed. It compiles production CSS, loads the embedded font, measures the real React host in Chromium, validates both neutral schemas, cross-checks operations with the third-party `diff` library, and asserts the actual DOM projection.
- Styling generated-artifact check: `🗑️generated/sol-flow34/diff-styling-generated-check.log` — passed, generated artifacts fresh.
- Focused WGPU Diff laws: `🗑️generated/sol-flow34/diff-native-focused.log` — launched through the scoped Nx target and currently queued behind the shared Cargo lane at this checkpoint.
- Whitespace validation: `git diff --check` over the DiffView, text, theme, styling, schema, and focused test files produced no findings.

## Remaining integration boundary

The focused Rust laws had not reached compilation at the source checkpoint because existing Flow/UI/renderer jobs held the shared Cargo queue. Root owns the fresh full native, UI, and WASM gates. The source checkpoint deliberately includes the concurrent EventFeed `TextWeight::Medium` extension while preserving the face-aware text path.
