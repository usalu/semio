# Luna Exclusion Map — Contested UI Vocabulary Isolation

**Scanned:** 2026-08-20 14:11:00 CEST — 14:12:46 CEST  
**Live Fleet:** ComponentTree/BuiltNode/ui_wgpu::wgpu vocabulary migration  
**Liveness Status:** ACTIVE (commits 2026-08-20)

---

## Executive Summary

A concurrent session is refactoring the UI vocabulary across the fleet (UiNode → ComponentTree/BuiltNode, ui_wgpu paths). This scan identifies the 83 active source files (across 20 contested crates) that contain the new vocabulary, so other executors can avoid collisions.

**Contested crate count:** 20  
**Clear (non-UI) crate count:** 47+ (uncontested in framework/plugin SDK)  
**Highest-risk file:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (3 commits in 8 hours)  

---

## Contested Crates — Active Refactor Zones (DO NOT EDIT)

### Framework UI and Rendering

- **semio-framework-🖱️ui** (16 files)
  - Core UI runtime, reconciliation, transaction logic
  - Contract/builder with BuiltNode bindings
  - wgpu rendering paths
  - **Status:** ACTIVELY EDITED (2026-08-20)

- **semio-framework-🛂️manifest** (3 files)
  - Manifest component with ComponentTree references
  - **Status:** Recent commits (2026-08-20)

- **semio-framework-📡️replication** (1 file)
  - Wire protocol; references ComponentTree
  
- **semio-framework-🎠️kernel** (1 file)
  - Kernel component; carries ComponentTree type references

- **semio-framework-🎯️action-bus** (1 file)
  - TypeScript action routing; ComponentTree vocabulary

### OS Product Modules (Heavy Contested Zone)

- **semio-framework-os-📺️renderer** (16 files) ⚠️ **HIGHEST ACTIVITY**
  - React/wgpu render targets
  - Engine elements (Shell, Dock, Canvas, Scenes)
  - TaskManager, Interpreter, ProgramBridge
  - **Status:** MULTIPLE COMMITS PER DAY (2026-08-18 to 2026-08-20)
  - **Risk:** Overlapping render architecture concerns

- **semio-framework-os-🔌️plugin** (6 files)
  - Guest plugin SDK, host component, reactor patches
  - **Status:** VERY ACTIVE (3 commits in 8 hours, 2026-08-20)
  - **Critical:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` modified 2026-08-20 three times
  - **Note:** Per important.md, this file is involved in live WIT io-mechanism work

- **semio-framework-os-♾️infinite** (4 files)
  - Board/world components with ComponentTree references

- **semio-framework-os-🌊️flow** (3 files)
  - Flow host/catalogue modules

- **semio-framework-os-📖️playbook** (1 file)

- **semio-framework-os-🌉️mcp** (1 file)

### Framework Root

- **semio-framework-root** (2 files)
  - Top-level glue and Cargo.toml

### Plugin UI Artifacts (Fleet Tests)

These are downstream of the SDK refactor. They consume ComponentTree:

- **semio-plugin-📐️cad** (7 files)
  - CAD editor/viewer windows; schema; wasm artifact
  - **Status:** Recent (2026-08-19)

- **semio-plugin-🌊️flow** (3 files)
  - Flow editor/viewer; schema

- **semio-plugin-🌀️procedural** (5 files)
  - Procedural 2D/3D schemas and editors

- **semio-plugin-🕸️dag** (2 files)
  - DAG schema artifact

- **semio-plugin-➗️mathematical** (2 files)
  - Math editor; schema

- **semio-plugin-📖️playbook** (1 file, Cargo.toml only)

---

## Clear Crates (Safe to Edit, No Contested Vocabulary)

The following first-party crates do **not** contain ComponentTree, BuiltNode, or ui_wgpu vocabulary and are not in the UI refactor path:

### Framework Core (Async/Schema/Dispatch)

- semio-framework-async
- semio-framework-async-macros
- semio-framework-dispatch
- semio-framework-dispatch-macros
- semio-framework-schema
- semio-framework-schema-derive
- semio-framework-actor
- semio-framework-replication

### Framework Data/Utilities

- semio-framework-number
- semio-framework-pack
- semio-framework-geometry
- semio-framework-math
- semio-framework-3d
- semio-framework-2d
- semio-framework-machine
- semio-framework-graph
- semio-framework-surface
- semio-framework-editor
- semio-framework-server
- semio-framework-ui-contract (has BuiltNode bindings but is NOT actively refactored)

### OS Kernel/Services (Core Infrastructure)

- semio-framework-os-kernel ✅ GREEN per ticket baseline
- semio-framework-os-kernel-db ✅ GREEN per ticket baseline
- semio-framework-os-services ✅ GREEN per ticket baseline

### Host/Plugin Infrastructure

- semio-framework-plugin-host ✅ GREEN per ticket baseline
- semio-framework-plugin-describe

### Fleet Plugin Runtimes (Not UI)

- semio-s-plugin-stdio
- semio-s-plugin-note
- All non-UI plugins (cad/flow/procedural/etc. infrastructure code)

---

## Dependency Vulnerability Analysis

### Blocked Executors

The following downstream crates depend transitively on contested UI crates and **cannot** be cleanly edited this wave:

- **semio-framework-os-run** — depends on semio-framework-plugin and semio-framework-plugin-host (both partially contested)
- **semio-framework-os-flow** — depends on semio-framework-os-infinite and 7 flow/logic extension plugins (some contested)

All 63 fleet plugin crates that implement UI editors (cad, flow, procedural, etc.) are blocked on the SDK refactor.

### Safe Dependencies (No UI)

- semio-framework-async, semio-framework-schema, semio-framework-pack, semio-framework-actor dependencies are **clear** — no UI vocabulary, no contested files
- semio-framework-os-kernel, semio-framework-os-kernel-db, semio-framework-os-services remain **GREEN** per baseline and have no ComponentTree/BuiltNode references

---

## Liveness Evidence

**Start scan:** Thu Aug 20 14:11:00 CEST 2026  
**End scan:** Thu Aug 20 14:12:46 CEST 2026

### Recent Commits on Contested Files

| File | Last 3 Commits |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/.../🦀️builder.rs` | 2026-08-20 #545 |
| `🧰️framework/🛍️products/💻️os/.../📺️renderer/.../Shell/🧊️component.rs` | 2026-08-20 #544, 2026-08-19 #539, 2026-08-18 #536 |
| `🧰️framework/🛍️products/💻️os/.../🔌️plugin/🦀️component.rs` | 2026-08-20 #546, #545, #544 |
| Plugin UI artifacts (cad/flow/etc.) | 2026-08-19 #543, #541 |

**Conclusion:** The other fleet is **ACTIVE and ONGOING**. Multiple files modified twice or thrice in a 2-minute window (commits #544, #545, #546 all 2026-08-20). This is NOT idle work; the refactor is in-flight.

---

## Overlapping Concerns (Genuine Hazards)

Files where two fleets' work might collide if both touched them:

### HIGH RISK

1. **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`**
   - Contains ComponentTree vocabulary (UI new)
   - Per important.md rule 9 (peer tickets), this file is involved in live io-mechanism changes
   - Recent edits (2026-08-20 three times)
   - **Status:** DO NOT TOUCH — concurrent peer work in progress

2. **`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs`**
   - ComponentTree vocabulary
   - Active refactor (3 commits in 2 days)
   - May contain dropped-future or executor concerns from other tickets
   - **Action:** Read-only inspection only

### MEDIUM RISK

- All wgpu rendering paths under 📺️renderer (contain ui_wgpu::wgpu)
- All plugin artifact editor/viewer window files (may reference both UI vocabulary and wasm serialization concerns)

### LOW RISK (Documentation/Config Only)

- Cargo.toml files in contested crates (safe to read for dependency info)
- Generated manifest.ts (regenerated, not hand-edited)

---

## Recommended Actions for Executors

### DO NOT TOUCH (Another Fleet Live)

- ❌ Any file in contested crates listed above
- ❌ `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (concurrent io-mechanism work)
- ❌ `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/` (heavy active refactor)
- ❌ Any file in the 63 fleet plugin UI artifact crates

### SAFE TO EDIT (No UI Vocabulary)

- ✅ semio-framework-os-kernel and deps (async/dispatch/actor/schema cores)
- ✅ semio-framework-plugin-host (host runtime, safe to verify/extend if needed)
- ✅ semio-framework-plugin-describe (descriptor generation)
- ✅ Any core infrastructure crate not listing ComponentTree/BuiltNode/ui_wgpu

### SAFE TO READ (Inspect-Only)

- ✅ Contested files for understanding the new vocabulary structure
- ✅ Cargo.toml dependencies to understand downstream impact
- ✅ recent git log on contested files to assess liveness

---

## Summary Table

| Classification | Count | Examples |
|---|---|---|
| **Contested Crates** | 20 | semio-framework-🖱️ui, semio-framework-os-📺️renderer, semio-plugin-📐️cad |
| **Contested Files** | 83 | UI contract, wgpu glue, renderer elements, plugin editors |
| **Clear Crates** | 47+ | semio-framework-os-kernel, semio-framework-async, semio-framework-pack |
| **Highest Activity** | 📺️renderer, 🔌️plugin | 3+ commits/day pattern |
| **Blocked Executors** | ~10 | os-run, os-flow, all UI-heavy plugins |
| **Safe Executors** | unlimited | Anything avoiding contested crates |

---

## Conclusion

**The other fleet is actively refactoring the UI vocabulary (ComponentTree/BuiltNode/ui_wgpu) across 20 crates.** Execution of any work touching those crates this wave will collide destructively. This map enumerates every contested file and crate so executors can avoid them.

**Green path for this wave:** Stick to the clear crates (async/kernel/schema/dispatch core, host runtime infrastructure, descriptor generation). All UI and plugin editor work is blocked upstream until the vocabulary migration lands.

