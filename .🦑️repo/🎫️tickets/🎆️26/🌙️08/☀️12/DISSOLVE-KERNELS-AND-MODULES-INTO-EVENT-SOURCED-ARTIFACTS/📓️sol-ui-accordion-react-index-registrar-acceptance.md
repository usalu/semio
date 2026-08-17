# UI Accordion React Index Registrar Acceptance

## Baseline

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Shared React index pre-edit SHA-256: `01005e76dbc844cbaa2e9c8b2e6b7727bfd3d575f7ef887e62c3f1ce249c4a52`
- Terra confirmed both Accordion source files absent and the target directory without authored files before this registrar edit.

## Registrar Change

The coordinator removed exactly:

1. The unused package-level `@radix-ui/react-accordion` namespace import.
2. The complete `Accordion` semantic import/re-export region.

No other registrar region, component, dependency manifest, lockfile, generated output, or Storybook configuration was changed.

## Evidence

- Shared React index post-edit SHA-256: `1ae126cc1dd3f5a47c201ca9af485397205d3d8b3cc48e40dd8c902de9cf5f29`
- Index stale scan for `AccordionPrimitive`, the Accordion semantic region, direct Accordion source path, and the four exported identifiers: zero matches.
- Scoped ordinary `git diff --check`: pass.
- Scoped cached `git diff --check`: pass; the index remains unstaged.

Final active-source scans and registered Nx gates remain Terra-owned and run only after this serialized hash signal.
