# Layout Launch Audit

Inspected 2026-11-20. Read-only source and ticket review; no execution or runtime result is claimed. The older audit.md has been preserved.

## Scope and acceptance

The layout plugin is a browser document editor (✏️s/🔌️plugins/📏️layout/AGENTS.md), not an automatic object-arrangement plugin. Its document consists of pages, spreads, frames, stories, styles and assets; principal output is PDF, SVG and PNG. The active ticket additionally specifies responsive hover, linked 2D artifact placement and fast large-sheet preview, and the blueprint transform gumball. The launch kit prompt requires the built plugin to load in its actual host, execute its principal flows against actual host state, preserve state through supported lifecycle and be built and installed reproducibly.

Acceptance must therefore include (1) start and inspect the actual host; (2) open a document, edit/select/move/resize/rotate frames with transform; (3) place a linked 2D asset and inspect displayed proxy; (4) export and inspect resulting output; (5) check undo/redo, cancellation and stale selection where supported; (6) save/reopen and repeat; (7) build/install a fresh package and repeat. Record actual versions, commands, artifacts and results rather than treating source tests as host proof.

## Prioritized gaps and evidence

1. Actual host/package acceptance remains unproven by the inspected records. Historical ticket ui.md and verification-audit.md explicitly say native execution was unverified; these were authored under an earlier environment and do not prove the present runner is broken. The current launch kit demands real browser/host interaction, built artifact and installation evidence.
2. Ticket checkpoint-milestones.md explicitly says export checkpoint restore remains a testkit surface with no registered production host resume consumer; native execution remains unverified there. Treat resume as unsupported unless current production wiring is found, and verify cancellation and recoverable state against the actual host.
3. The prior audit.md and verification-audit.md describe older source, including missing IDML and TypeScript test routes; the present project manifest lists IDML tests, diff algebra, retained mutation and many more test entries. Do not use their historical absence claims as current findings without checking source.
4. Existing placement and gumball notes document tests and implementation but are not equivalent to host proof. A current rotated PNG canvas test checks a two-pixel quarter turn, while a separate invalid PNG fallback test only verifies the frame is retained; actual rendered images, arbitrary rotation, selection and export consistency remain to be exercised.

## Sources

- layout-plugin-launch-kit/prompt.txt and README.md
- ✏️s/🔌️plugins/📏️layout/AGENTS.md
- Ticket 📓️contract.md, 📓️placement.md, 📓️gumball.md, ui.md, verification-audit.md and checkpoint-milestones.md
- ✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript/📋️project.json
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖼️canvas/🧪️tests/🔬️unit/🦀️.rs
