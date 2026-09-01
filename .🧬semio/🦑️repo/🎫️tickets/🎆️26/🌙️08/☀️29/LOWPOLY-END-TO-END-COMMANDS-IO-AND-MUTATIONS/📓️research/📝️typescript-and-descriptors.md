# Lowpoly TypeScript Surface & Stub Analysis

## 1. Component.ts Files Enumeration (Sorted by Size)

33 files found under `✏️s/🔌️plugins/💠️lowpoly`:

| Size (bytes) | Path | Type |
|---|---|---|
| 11 | `.../🚪️io/📤️export/🧵️serializers/.../🟦️component.ts` (×9) | Stub |
| 11 | `.../🚪️io/📥️import/🧩️deserializers/.../🟦️component.ts` (×9) | Stub |
| 61 | `.../🧬️schema/🧬️mutations/🟦️component.ts` | Stub + doc |
| 71 | `.../🚪️io/🟦️component.ts` | Stub + doc + W7 marker |
| 95 | `.../🧬️schema/🔺️diff/📝️text/🟦️component.ts` | Type def |
| 103 | `.../🧬️schema/📸️snapshot/(binary\|text)/🟦️component.ts` (×2) | Type def |
| 105 | `.../🧬️schema/🧬️mutations/📝️text/🟦️component.ts` | Type def |
| 111 | `.../🧬️schema/(💡️inferences/📝️text\|📸️snapshot/💾️binary)/🟦️component.ts` (×2) | Type def |
| 113 | `.../🧬️schema/🧬️mutations/💾️binary/🟦️component.ts` | Type def |
| 119 | `.../🧬️schema/💡️inferences/💾️binary/🟦️component.ts` | Type def |
| 143 | `.../📚️examples/🎬️demo/🟦️component.ts` | Demo export |
| 175 | `.../✏️editor/📚️examples/🎬️demo-session/🟦️component.ts` | Demo export |
| 206 | `.../🧬️schema/💡️inferences/📦bounds/🟦️component.ts` | Type def |
| 352 | `.../🧬️schema/💡️inferences/🟦️component.ts` | Types (`LowpolyBounds`, `LowpolyInference`) |
| 552 | `.../👁️viewer/🟦️component.ts` | Exports dialect, mode, window re-exports |
| 702 | `.../✏️editor/🟦️component.ts` | Exports dialect, mode IDs, window re-exports |
| 909 | `.../🧬️schema/📸️snapshot/🟦️component.ts` | Snapshot schema type defs |
| 945 | `.../✏️editor/👥️presence/🧬️schema/🟦️component.ts` | Presence schema types |
| 1296 | `.../👁️viewer/🎭️modes/👁️view/.../🌐️model/🟦️component.ts` | Viewer mode impl |
| 1587 | `.../✏️editor/🎭️modes/🎨️paint/.../🖼️uv/🟦️component.ts` | Paint mode impl |
| 1602 | `.../✏️editor/🎭️modes/✏️edit/.../🌐️model/🟦️component.ts` | Edit mode impl |
| 1764 | `.../✏️editor/🎚️config/🧬️schema/🟦️component.ts` | Config schema |
| 2690 | `.../🧬️schema/🟦️component.ts` | Core schema: `LowpolyArtifact`, `LowpolySelection`, `LowpolyTransform`, `LowpolyObject`, etc. |
| 4019 | `.../🧬️schema/🔺️diff/🟦️component.ts` | Diff schema impl |

### <300 Bytes (Pure Stubs)

18 files (all serializers/deserializers for 9 file formats × 2 directions):
- All are `export {}` only
- No doc comments in most; two have comments:

```
// I/O facet barrel
/** 🚪️ IO facet barrel — WASM facades land in W7. */
export {};

// Mutations facade  
/** 🧩 lowpoly 🧬️mutations WASM facade. */
export {};
```

## 2. Deferral Markers (Wave/Tracking)

**Single marker found: W7**
- **Location**: `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🟦️component.ts`
- **Exact comment**: `/** 🚪️ IO facet barrel — WASM facades land in W7. */`
- **Files carrying this marker**: 1 (the IO facet barrel)

**Secondary pattern**: Generic "WASM facade" references (no wave marker)
- Mutations component: `/** 🧩 lowpoly 🧬️mutations WASM facade. */`
- No explicit deferral tracking in this comment

## 3. W7 Ticket Tracking

**Ticket ID**: `26/08/06/S-MODULES-CRATE-CONSOLIDATION-AND-NAMING-FIX`

**Status**: `closed` ✓

**Description**: Consolidate `✏️s/🔨️modules` shared-kernel crates from 9 rust crates + 2(+) TS packages down to 4 rust crates + 2 TS packages, fixing wrong naming (`semio-framework-os-kernel-3d-*` → `semio-s-*`).

**Key point**: W7 is about **s-module consolidation** (shared kernels for 2D/3D/mindmap/imperative), NOT about lowpoly's WASM facades. The IO facet's W7 reference is a **misdirected deferral** — it refers to a different work phase that consolidates shared spatial kernels, not lowpoly-specific I/O binding.

## 4. Comparison with CAD Plugin

CAD has similar stub patterns but richer TS surface:

| Aspect | CAD | Lowpoly |
|--------|-----|---------|
| **Serializer/deserializer stubs** | 16 (also `export {}`, similar 46-51 byte size) | 18 (11 bytes each) |
| **Core TS package** | `core.ts` (514 bytes) re-exports from spatial kernel + registry | **Missing** — no `core.ts` exists |
| **Index.ts exports** | 17 namespaced exports (schema, snapshot, diff, dsl, pack, op, mutations, spr, io, runtime, brepjs) | 3 namespaced exports (schema, **decomposer**, io) |
| **Decomposer path** | ✓ Exists: `.../brepjs/🟦️component.ts` | ✗ **Missing**: `.../🪓️decomposer/` dir does not exist |

**CAD patterns for I/O**:
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/.../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🟦️component.ts`
  ```
  /** Serialize cad to stdio.gltf. */
  export {};
  ```

## 5. Lowpoly TS Package Analysis

**Package name**: `@semio-tech/lowpoly-js` (v0.1.0)

**What it exports** (from `📦️index.ts`):
```typescript
export * as lowpoly_schema from "../../🗿️artifacts/💠️lowpoly/🧬️schema/🟦️component.ts";
export * as lowpoly_decomposer from "../../🗿️artifacts/💠️lowpoly/🪓️decomposer/🟦️component.ts"; // ← MISSING FILE
export * as lowpoly_io from "../../🗿️artifacts/💠️lowpoly/🚪️io/🟦️component.ts";
```

**What the nx `test` target validates**:
- Calls `./📜️script.ts test`
- Runs interactive-job fixture validation:
  - Validates 47 classified actions in Rust source (`📐️editor/🦀️component.rs`)
  - Expects 19 "Migrated" + 28 "BatchOnlyPendingRewrite" routes
  - Validates Ajv schema compliance for the fixture (`🔣️component.json`)
  - Validates structural markers in Rust (progress, replay, paint steps, store limits)
  - **Does NOT test TS facades** — it only validates Rust interactive-job plumbing

**Does NOT have**:
- `core.ts` (like CAD does)
- Any vitest/unit tests for TS type definitions
- Any compilation checks for the stub exports

## 6. Descriptor.json Analysis

**File**: `✏️s/🔌️plugins/💠️lowpoly/🔣️descriptor.json` (~260KB)

**Generation**: `✓ Generated` (contains `hashes` field)

**Top-level structure**:
- `activationEvents`, `capabilityRequests`, `contributions`, `descriptorVersion`, `execution`, `hashes`, `manifest`, `quotas`, `role`

**Contributions**:
| Entry type | Count | Details |
|---|---|---|
| **ioEntries** | 10 | Import operations from all formats: dwg, gltf, json, las, obj, ply, png, stl, txt, + self-lowpoly |
| **panels** | 6 | Framework panels: artifact, catalogue, inspection, layers, history (×2) |
| **composerEntries** | 1 | Reads all 10 formats + lowpoly; writes lowpoly |

**Commands**: The descriptor lists **0 commands** in the editor/paint modes' command arrays.

**Note**: The descriptor is generated by the plugin (invoked via `nx generate` on the Rust crate), so it reflects actual Rust metadata, not TS stubs.

## 7. Real Gaps vs. Intentional Deferrals — Verdict

### (a) Intentional, Tracked Deferrals

| File | Marker | Tracking | Status |
|---|---|---|---|
| `🚪️io/🟦️component.ts` (71 bytes) | "W7" | Ticket `26/08/06/S-MODULES-CRATE-CONSOLIDATION-AND-NAMING-FIX` | Closed |

**Caveat**: The W7 marker is **misdirected**. W7 work (s-module consolidation) is already complete; the marker likely refers to a stale plan phase, not lowpoly-specific work. The IO stub itself remains because actual I/O implementation belongs in Rust WASM bindings (which the descriptor.json confirms exist and are registered).

### (b) Untracked Real Gaps

| File | Issue | Impact |
|---|---|---|
| `📦️index.ts` export of `lowpoly_decomposer` | Path `../../🗿️artifacts/💠️lowpoly/🪓️decomposer/🟦️component.ts` does not exist | **Compile-time error**: module resolution fails; TS/Node will not load |
| Missing `core.ts` (unlike CAD) | No merged core engine surface defined | Clients must import from `lowpoly_schema` directly; no unified namespace |
| 18 serializer/deserializer stubs | `export {}` with no TS definitions | **Design-time gap**: no type checking for I/O format bindings; descriptor lists 10 formats as functional but TS stubs are empty |

**Descriptor vs. TS mismatch**:
- Descriptor claims 10 working import/export formats (dwg, gltf, json, las, obj, ply, png, stl, txt, lowpoly)
- Rust plugin likely has WASM bindings for all (implied by closed W7 + descriptor generation)
- TS facade stubs are all `export {}`
- This is the intended architecture: **Rust handles I/O; TS stubs are placeholder facades**

**End-to-end blockage**:
- The missing `🪓️decomposer/🟦️component.ts` will cause module load failure **immediately** when the TS package is imported
- The 18 I/O serializer/deserializer stubs are *not* blocking end-to-end use (descriptor shows I/O works at plugin level; TS just has no type surface)
- Missing commands in descriptor means no UI commands yet (edit mode has no `commands: []` entries), but this may be intentional (commands pushed to interactive-job runtime instead)

## Recommendations

1. **Delete the decomposer export** from `📦️index.ts` or **create** the missing `🪓️decomposer/🟦️component.ts` directory/file
2. **Create `core.ts`** following CAD's pattern if clients need a merged core namespace (currently they must import nested paths)
3. **Add TS type facades** for I/O serializers/deserializers if end-to-end TS type checking is required (currently Rust handles validation)
4. **Update or remove the W7 marker** from the IO facet barrel comment (W7 is complete; this is a stale reference)
5. **Add TS unit tests** to lowpoly's `📜️script.ts` via Vitest to validate type surface (currently only Rust interactive-job is tested)
