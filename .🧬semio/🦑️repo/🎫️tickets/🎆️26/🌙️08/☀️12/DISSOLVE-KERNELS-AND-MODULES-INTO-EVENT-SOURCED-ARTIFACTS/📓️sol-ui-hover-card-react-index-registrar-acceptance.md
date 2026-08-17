# UI Hover Card React Index Registrar Acceptance

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Shared React index pre-edit SHA-256: `6efa99283af7df14639d1f301456690d3d16860156ed8d2bf1087094a2bfc2fc`
- Terra confirmed both HoverCard source files absent and the directory without authored files before the registrar edit.

The coordinator removed only the complete HoverCard import/re-export region and the now-unused package-level `@radix-ui/react-hover-card` namespace import. No other semantic region, manifest, lockfile, generated output, or Storybook configuration changed.

Evidence:

- Shared React index post-edit SHA-256: `fa8dbb145f3c31af948dc7f18bc51a931cc7cb981fcdac3bd26086e273b99f0b`
- Index stale scan for `HoverCardPrimitive`, the direct component path, and all three exported identifiers: zero matches.
- Scoped ordinary and cached `git diff --check`: pass.

Final active-source scans and Nx validation remain Terra-owned after this serialized hash signal.
