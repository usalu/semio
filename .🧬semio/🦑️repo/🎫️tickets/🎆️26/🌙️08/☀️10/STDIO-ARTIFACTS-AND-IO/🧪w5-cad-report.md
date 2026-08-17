# W5 Pilot Report — CAD Artifact Stdio Migration

Ticket: `26/08/10/STDIO-ARTIFACTS-AND-IO`  
Owned path: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad`  
Also patched: `📦️packages/🦀️rust/📦️glue.rs`, `📦️packages/🦀️rust/Cargo.toml`, `📦️packages/🟦️typescript/📦️index.ts`

## Gate

```
cargo check -p semio-s-plugin-cad
```

**Result: green** — 0 errors (`w5-cad-cargo-check2.log`). Warnings only.

## Tokens (`🧪tokens.json`)

| Key | Value |
|---|---|
| builder | `🏗️builder` |
| decomposer | `🪓️decomposer` |
| text | `📝️text` |
| binary | `💾️binary` |
| deserializers | `🧩️deserializers` |
| serializers | `🧵️serializers` |

## Path map (§5) — executed

| Old | New |
|---|---|
| `🗣️dsl/` | `🧬️schema/📸️snapshot/📝️text/` |
| `📸️snapshot/🎒️pack/` | `🧬️schema/📸️snapshot/💾️binary/` |
| `📸️snapshot/🧬️schema/` | `🧬️schema/📸️snapshot/` |
| `🔺️diff/` (grammar+rs+ts) | `🧬️schema/🔺️diff/📝️text/` |
| `🔺️diff/🧬️schema/` | `🧬️schema/🔺️diff/` |
| (new) | `🧬️schema/🔺️diff/💾️binary/` |
| `🔧️op/` | `🧬️schema/🧬️mutations/📝️text/` |
| `📡️spr/` | `🧬️schema/🧬️mutations/💾️binary/` |
| `🧬️mutations/<m>/` | `🧬️schema/🧬️mutations/<m>/` |
| `🚪️io/<format>/{import,export}/` | `🚪️io/{import/🧩️deserializers,export/🧵️serializers}/🗿️artifacts/<stdio>/` |
| (new) | `🏗️builder/`, `🪓️decomposer/` |

## Disk verification

`CAD.iterdir()` now:

```
⚙️engine
🎬️interaction-spec
🏗️builder
📚️examples
🚪️io
🦀️component.rs
🧬️schema
🪓️decomposer
```

Asserts (all true):

- old root facets gone: `🗣️dsl`, `📸️snapshot`, `🔺️diff`, `🔧️op`, `📡️spr`, root `🧬️mutations`
- `🏗️builder/🦀️component.rs` and `🪓️decomposer/🦀️component.rs` exist
- text leaves = 8 under snapshot/diff/mutations `📝️text`
- binary leaves = 6 under snapshot/diff/mutations `💾️binary`
- no old `🚪️io/<format>/` dirs
- IO present for: dwg, glb, gltf, ifc, json, obj, png, step, stl

## IO matrix

Each of `dwg,glb,gltf,ifc,json,obj,png,step,stl` has:

- `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/<emoji-dir>/{rs,ts}`
- `🚪️io/📤️export/🧵️serializers/🗿️artifacts/<emoji-dir>/{rs,ts}`

Typed against `semio-s-plugin-stdio` snapshots.

## SDK

- `CadBuilder` implements `ArtifactBuilder`
- `CadDecomposer` implements `ArtifactDecomposer`
- Glue keeps compatibility aliases (`dsl`, `op`, `spr`, `snapshot`, `diff`, `io::<fmt>::{import,export}`) so apps compile without extension edits

## Generators / logs (ticket only)

- `generators/w5_cad_state.json`
- `generators/w5_cad_leaves.json`
- `generators/w5_cad_stdio_dirs.json`
- `generators/w5_patch_glue.py`
- `generators/w5_fix_includes.py`
- `w5-cad-cargo-check.log`
- `w5-cad-cargo-check2.log`
