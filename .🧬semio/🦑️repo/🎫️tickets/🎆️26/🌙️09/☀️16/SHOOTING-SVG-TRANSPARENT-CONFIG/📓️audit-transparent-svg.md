# Shooting SVG Transparent Background Audit

## Root cause

Icon shots use the shared `iconRenderPort` (`SVGRenderer` from three.js). When `scene.background` is unset (shooting omits `background` from the icon request for `""` / `transparent`), `SVGRenderer.render` still runs `clear()` with its default clear color **white** (`Color()` defaults to `#ffffff`), which becomes `style="background-color: …"` on the root `<svg>`.

## Fix

- `autoClear = false` for transparent shots.
- `finalizeIconSvgMarkup` strips any `background-color` inline style as a safety net.
- Shooting `scene.background` / per-shot `background` continue to gate opaque fills (unchanged contract).
- `scene.material.stroke` drives mesh edge outlines in icon + world-3d preview.

## Config surface

| Field | Location | Effect |
|-------|----------|--------|
| `scene.background` / `shot.background` | Document | Opaque hex or `""`/`transparent` for see-through |
| `scene.material.color` | Document | Mesh fill |
| `scene.material.stroke` | Document | Edge line color (`none`/`transparent` disables) |
| Lights / material metalness etc. | Document | Existing lighting model |
