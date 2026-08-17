# UI Diagram Node React Index Registrar Acceptance

## Baseline

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Shared React index pre-edit SHA-256 after the accepted Accordion registrar: `1ae126cc1dd3f5a47c201ca9af485397205d3d8b3cc48e40dd8c902de9cf5f29`
- Terra confirmed the DiagramNode component and exclusive story absent, Canvas and Diagram story consumers removed, unrelated skeleton stories retained, and scoped source diffs clean.

## Registrar Change

The coordinator removed only the complete `DiagramNode` import/re-export region, including `DiagramNode`, `PlaceholderDiagramNode`, and `DiagramNodeProps`. No protected renderer path, other semantic region, generated census, dependency manifest, or Storybook configuration was changed.

## Evidence

- Shared React index post-edit SHA-256: `6efa99283af7df14639d1f301456690d3d16860156ed8d2bf1087094a2bfc2fc`
- Index stale scan for direct DiagramNode path and all three exported identifiers: zero matches.
- Scoped ordinary and cached `git diff --check`: pass.
- The shared index remains unstaged and serialized under coordinator ownership.

Terra owns final active-source classification and registered Nx validation after this hash signal.
