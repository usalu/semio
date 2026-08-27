# All-App Descriptor Freshness

Date: 2026-08-27

## Scope

The all-app gate discovers the 32 checked-in `🔣️descriptor.json` owners under `✏️s/🔌️plugins`, including four extensions. The canonical producer is each plugin Rust package's `📜️script.ts describe` command, which builds the `wasm32-wasip2` component and invokes the framework describe package. Descriptors are not hand-maintained source.

## Live Census

- App declarations: 101
- Launch-only products: 57
- Total app/product surfaces: 158
- Launch coverage: 101/101 app declarations across React, WGPU Wasm, and WGPU native
- Descriptor action rows: 4,760
- Rows serialized as `interactiveJob: migrated`: 1,979
- Rows not serialized as migrated: 2,781

Twenty plugin descriptors are stale as whole files: every action row in each file lacks the nested `semantics.execution.interactiveJob` value. Twelve descriptor owners are current; their framework-injected history, clipboard, tutorial, and interaction actions serialize the source-owned `migrated` classification correctly. A direct CAD sample proves the schema path is `semantics.execution.interactiveJob`, not a top-level action property.

## Stale Whole-File Owners

| Plugin | Non-migrated rows | Total rows |
|---|---:|---:|
| norm | 675 | 675 |
| flow | 243 | 243 |
| architect | 208 | 208 |
| space | 204 | 204 |
| remodel | 184 | 184 |
| shooting | 127 | 127 |
| lowpoly | 123 | 123 |
| note | 123 | 123 |
| sequence | 121 | 121 |
| forms | 103 | 103 |
| raster | 96 | 96 |
| layout | 85 | 85 |
| imperative | 84 | 84 |
| gis | 79 | 79 |
| dag | 75 | 75 |
| vcs | 69 | 69 |
| draw | 58 | 58 |
| mathematical | 53 | 53 |
| reasoning | 41 | 41 |
| writer | 30 | 48 |

## Current Descriptor Owners

The energy, fem, sourcing, procedural, animate, demonstrator, cad, and process descriptors have no missing serialized dispositions. The four CAD extension descriptors correctly delegate activation and contain no app rows.

## Required Closure

1. Finish each app's source classification and retained job implementation first. `BatchOnlyPendingRewrite`, `ForbiddenFromUi`, and `Deleted` are honest intermediate states but do not satisfy the final `apps --actions` gate.
2. Build each affected plugin for `wasm32-wasip2` through its `bun`/`nx` project target.
3. Run its canonical `📜️script.ts describe` command to refresh both `🛂️descriptor.semio` and `🔣️descriptor.json` from the built component.
4. Run the registry freshness check, then `bun ./📜️script.ts verify interactivity apps --actions`.
5. Do not bulk-edit descriptor JSON. That would sever the checked-in descriptor from the executable component and evade the existing describe freshness law.

## Evidence Commands

```text
bun ./📜️script.ts verify interactivity apps
bun ./📜️script.ts verify interactivity apps --actions
jq '.. | objects | select(.id? == "checkoutCheckpoint") | {id,semantics}' ✏️s/🔌️plugins/📐️cad/🔣️descriptor.json
```
