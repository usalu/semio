# UI Footer Specific Component Retention Audit

## Scope and Classification

`Footer/🟦️component.tsx` is a coherent bottom-chrome presentation component. It splits `NavbarItem` inputs into normal and centered slots, paints shell-floor surfaces, and owns footer accessibility/introduction metadata. It is not a mixed umbrella and is not a candidate shared implementation module.

## Public Contract

- `FooterProps`: `items: NavbarItem[]` and optional `className`.
- `Footer`: bottom chrome bar with centered-overlay layout.

## Production Graph

- The React UI barrel mechanically imports and re-exports `Footer` and `FooterProps`.
- The sole active independent terminal is OS `ShellHost`, which renders `<Footer items={footerItems} />`.
- Stories, `Layout` examples, barrel inline tests, and OS test files are excluded from production-consumer proof.

Footer is a specific UI component, so the module consumer-minimum rule does not require inlining it. Its one terminal does not justify moving it to `modules`, while its cohesive semantic interaction/presentation identity justifies retaining the component.

`NavbarItem` is independently shared by Navbar, Canvas, Footer, and ShellHost. Footer's type reuse does not change that ownership.

## Boundary and Cycle Evidence

- Footer's `NavbarItem` edge is type-only and erased at runtime.
- Footer itself creates no SCC or mutable module state.
- A separate Navbar/UI-barrel runtime SCC exists because Navbar imports `NavbarTrailingFullscreenSlot`; it is not owned by Footer.
- `FooterProps` transitively exposes React adapter types through `NavbarItem`; this should be addressed with the Navbar contract owner, not by splitting Footer.

## Disposition

Retain Footer as its specific presentation component. No source lease is warranted. Current source SHA-256 is `ff901f2e47d51a0febffeb0ffdc781476617ec65f44cf0e8e2ae1421f00bd756`. The audit observed ShellHost SHA-256 `55f0a2b307bc8ab8c292b212f878a4c590dc4b94b09e47bb923f5ef4f879fa3d` and Navbar SHA-256 `2918372bf6dcee1d211db0a0082db4f7cd596db2c06ba6fb91d641d426ce024e`.
