# Flow Extension Build & Registration Status

**Investigation Date:** 2026-08-29

## Summary

All 9 flow extensions are registered in the plugin registry, but only 7 are consumed by the procedural3d plugin. The build environment has a file lock contention issue that prevented native cargo check/build completion.

## Extension Source Locations

| Extension | Source Path | Crate Name |
|-----------|-------------|-----------|
| brep | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust` | semio-s-plugin-flow-extension-brep |
| dictionary | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/📦️packages/🦀️rust` | semio-s-plugin-flow-extension-dictionary |
| bim | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust` | semio-s-plugin-flow-extension-bim |
| logic | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/📦️packages/🦀️rust` | semio-s-plugin-flow-extension-logic |
| primitive | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/📦️packages/🦀️rust` | semio-s-plugin-flow-extension-primitive |
| math | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust` | semio-s-plugin-flow-extension-math |
| list | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/📦️packages/🦀️rust` | semio-s-plugin-flow-extension-list |
| draw | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust` | semio-s-plugin-flow-extension-draw |
| text | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/📦️packages/🦀️rust` | semio-s-plugin-flow-extension-text |

## Build Status

### Native Cargo Check
**Status:** BLOCKED - File lock contention
**Command:** `cargo check -p semio-s-plugin-flow-extension-brep ... --keep-going`
**Issue:** Build system waiting on file lock; likely another build process holding lock

### WASM Build (wasm32-wasip2)
**Status:** NOT COMPLETED - Dependent on native check
**Command:** `cargo build --target wasm32-wasip2 -p ... --keep-going`
**Note:** Cannot proceed until lock is released

## Registration Status

### plugins.json Registry
**Path:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔣️plugins.json`

**All 9 extensions present:**
- ✓ flow-extension-brep (lines 361-380)
- ✓ flow-extension-dictionary (lines 382-400)
- ✓ flow-extension-draw (lines 402-420)
- ✓ flow-extension-list (lines 422-440)
- ✓ flow-extension-logic (lines 442-460)
- ✓ flow-extension-math (lines 462-480)
- ✓ flow-extension-primitive (lines 482-500)
- ✓ flow-extension-text (lines 502-520)
- ✓ flow-extension-bim (lines 341-359)

### session.ts Registration
**Path:** `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🤖️generated/🟦️session.ts`

**Note:** File contains only playground session variant (gis2d); does not include procedural variants. This file is regenerated per playground variant.

## Procedural Plugin Extension Consumption

**Source:** `✏️s/🔌️plugins/🌀️procedural/🦀️component.rs`
**Metadata:** `package.metadata.semio.consumes = ["forms.questionKind", "flow.extension"]`

### Registered with Procedural (7/9)
✓ **brep** - Line 265: `FlowExtensionManifest::new("brep", "Brep", "0.3.0")`
✓ **math** - Line 270: `FlowExtensionManifest::new("math", "Math", "0.1.0")`
✓ **primitive** - Line 275: `FlowExtensionManifest::new("core", "Core", "0.1.0")` (labeled as "core")
✓ **logic** - Line 280: `FlowExtensionManifest::new("logic", "Logic", "0.1.0")`
✓ **dictionary** - Line 285: `FlowExtensionManifest::new("dictionary", "Dictionary", "0.1.0")`
✓ **list** - Line 290: `FlowExtensionManifest::new("list", "List", "0.1.0")`
✓ **text** - Line 295: `FlowExtensionManifest::new("text", "Text", "0.1.0")`

### NOT Registered with Procedural (2/9)
✗ **draw** - Absent from procedural component.rs
✗ **bim** - Absent from procedural component.rs

## Extension Invocation in Procedural3d

**Flow Evaluation Entry Point:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️flow-eval-tick/🦀️component.rs`

- Line 25: `effects.push(Effect::InvokeExtension { req: ..., extension_id: pending.extension_id, capability: "evaluate".into(), request_json })`
- Extensions are invoked dynamically via `pending.extension_id` determined at flow evaluation time
- Requested capability: `"evaluate"` (flow operator evaluation)
- Fallback to tessellation preview when evaluation complete

## Analysis

### Architecture Pattern
- **7 extensions actively used**: brep, math, primitive, logic, dictionary, list, text
- **2 extensions registered but unused**: draw, bim
- Flow evaluation is dynamic: extensions are invoked by name at evaluation time
- Procedural3d's 3D preview depends on flow extensions via the flow evaluation engine

### Potential Issues
1. **Unused extensions**: draw and bim are registered in the plugin registry but not mounted in procedural
   - draw might be needed for 2D operations in procedural2d (not investigated)
   - bim might be optional or pending integration
   
2. **Build lock**: Cargo check timed out waiting for build directory lock
   - Suggests active build or incomplete cleanup from prior run
   - Recommend: Clear target directory or identify holding process

### Registration Chain
1. Extensions defined in source Cargo.toml files ✓
2. Extensions registered in plugins.json (registry) ✓
3. Extensions mounted in procedural plugin.rs (partial) - 7/9
4. Extensions available at runtime via flow evaluation ✓

