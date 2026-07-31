# Brepjs Attribute Sidecar Maps

**Repo MCP:** unavailable in this session (`ticket_open` / `repo://goals` not registered).

## Summary

brepjs shapes carry no user-data / attributes. Spatial keeps semantic fields on `Model.metadata` (`AttributeStore`) and maps brep topology to stable `FaceRef` / `EdgeRef` inside `@spatial/js-kernel-brepjs` so `MeshTransfer` never exposes OCCT `getHashCode` ids.

## Files

- `spatial/js/core/index.ts`
- `spatial/js/kernel-brepjs/index.ts`
- `spatial/js/renderer-r3f/index.tsx`
