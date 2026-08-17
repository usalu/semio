# Terra Packet UI-MobilePanel-01: One-Consumer Dissolution

## Preconditions

- Read root/applicable `AGENTS.md` and the audit.
- Baseline HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`.
- Use `apply_patch` only; no modifying Git.
- Rehash each Terra-owned path and require the exact hashes in the table below.
- Shared React index is coordinator-only; current announced baseline is `fa8dbb145f3c31af948dc7f18bc51a931cc7cb981fcdac3bd26086e273b99f0b`.

## Terra Writable Closure

| Path | Required SHA-256 |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📱️MobilePanel/🟦️component.tsx` | `9030ca9ecc24ac257a39b73e3c51779b74854f2030db88aa41068d27ff5d27b1` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📱️MobilePanel/🧪️story.tsx` | `09ec4401427a80fdbf7480da2f543a286bf590644d45756c730c9e520cabbdbd` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📐️Layout/🟦️component.tsx` | `28b23cd4dc78b57c6fa856a06e973a3e168431c3205c1b7a710f8ded5a699132` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️component.tsx` | `0072669a74e42a4bbb62ca688ebfcd2fb67ff2b23f41c2a9b6b21f36dde6c6a1` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️PanelTabBar/🟦️component.tsx` | `8137457e8460a1023e42e8fa3426e2220bb7b782f17d85df527fe7bb4ce8ecab` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🆔️ElementId/🟦️component.tsx` | `2ada5a026752899f8aa14f50100607eba1a720016319dbf78baa80618c9ecf0e` |

Terra also owns one unique acceptance record `📓️terra-ui-mobile-panel-layout-inline-acceptance.md`.

## Required Change

1. Move the full MobilePanel behavior into `Layout/Component.tsx` under a private hierarchical region, preserving behavior and imports.
2. Rename the private component to `LayoutMobilePanel` and the owned public contract to `LayoutMobilePanelProps`; update `LayoutProps.mobilePanel` and the one render site.
3. Delete the former MobilePanel component and exclusive story.
4. Update only stale doc links in Panel, PanelTabBar, and ElementId so they describe the private mobile panel hosted by Layout without linking to a removed component.
5. Do not touch the shared React index, `.storybook/ui-new-stories.spec.ts`, other stories, manifests, locks, generated census output, protected renderer paths, or plugins.

Send a post-source checkpoint and wait. The coordinator will remove the separate MobilePanel registrar, export `LayoutMobilePanelProps` with Layout, update the one Storybook smoke comment, and supply the new index/spec hashes.

## Validation

After registrar signal, prove old direct paths and exported `MobilePanel`/`MobilePanelProps` are absent, while `LayoutMobilePanel`, `LayoutMobilePanelProps`, and the mobile Layout render branch exist. Run scoped ordinary/cached diff checks and UI React lint, typecheck, test-quick, and build once. Record exact results and do not repair unrelated broad failures.
