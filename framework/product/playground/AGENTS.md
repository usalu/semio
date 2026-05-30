A self-contained miniframework for building playgrounds (one app, one window kind, one fixture, selection, filter, workbench, details).

- [core](./core/core.ts) — React-neutral runtime + one-app shell (`PlaygroundController`, `ProductRuntime`, declarative `UiNode` bodies, registries).
- [renderer/react](./renderer/react/index.tsx) — Shell renderer (`PlaygroundView`) and `bootPlayground`; puzzle chrome stays on `./puzzle/*` subpath exports.

## Static sites (iframe-embeddable)

Every playground is a **Vite static site** (`dist/`) meant to be embedded in iframes and hosted on GitHub Pages (or equivalent static hosting).

| Play | Nx project | Latest host (`public/CNAME`) |
| --- | --- | --- |
| Sketchpad | `@semio/play` | [play.semio-tech.com](https://play.semio-tech.com) |
| CAD spatial | `@cad/js/renderer` | [play.cad.semio-tech.com](https://play.cad.semio-tech.com) |
| Puzzle 2D | `@puzzle/2d/play` | [play.2d.semio-tech.com](https://play.2d.semio-tech.com) |
| Puzzle 3D | `@puzzle/3d/play` | [play.3d.semio-tech.com](https://play.3d.semio-tech.com) |
| Puzzle 5D | `@puzzle/5d/play` | [play.5d.semio-tech.com](https://play.5d.semio-tech.com) |

Shared build contract (see [`ui/styling/vite-elements-assets.ts`](../../ui/styling/vite-elements-assets.ts)):

- `base: "./"` — relative asset URLs (works on any subdomain and inside iframes)
- `public/.nojekyll` — disable Jekyll on GitHub Pages (Rollup chunks starting with `_` must not be stripped)
- `public/CNAME` — **latest** canonical hostname only; the repo always tracks the current version
- `Content-Security-Policy: frame-ancestors *` on HTML + dev/preview headers — explicitly iframe-embeddable

### Versioning

- **Latest** is always served at the main link above (e.g. `play.semio-tech.com`).
- **Versioned** deployments use `v{n}.<latest-host>` (e.g. `v4.play.semio-tech.com`) and are published at deploy time; older `dist/` trees are **not** kept in git.
- When cutting a release, copy the built `dist/` to the versioned subdomain, then deploy `main` to the latest host.

Build: `bun nx run <project>:build` (puzzle/cad runs wasm prep where needed).
