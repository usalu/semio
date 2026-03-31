# Summary

Rebuilt `semio/engine/dist/mcp-app.html` by running `npm run build:mcp-app` in `semio/engine/`.

The previous build was from 2026-03-30 and lacked all recent McpDesignViewer routing logic:
- `show-design` → `SemioDesign` split view (3D scene + 2D diagram)
- `show-scene` → `SemioScene` 3D only
- `mcpFlattenDesignForSemioSurface`
- `mergeRichestDesignFromCandidates` priority logic
- `splitLayout="always"` for MCP design viewer

New build: 6.4MB, contains 22 references to show-design, 16 to show-scene, mcpFlatten and mergeRichest.

## Files
- `semio/engine/dist/mcp-app.html` (rebuilt)
