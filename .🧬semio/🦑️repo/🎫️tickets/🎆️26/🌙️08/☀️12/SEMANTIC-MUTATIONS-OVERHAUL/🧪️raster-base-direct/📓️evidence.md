# Raster Base Direct Owners

## Scope

Task `TERRA-STDIO-RASTER-BASE-DIRECT-01` owns only PNG 1.2 Any, JPG JFIF-1.01 Any, BMP V3 Any, and TIFF 6.0 Any, in that order, plus their necessary base consumers and shared glue mounts. JPG and TIFF subset mutation roots are excluded from this batch. `compose/**` is never accessed. No modifying Git commands are used.

The four artifact prefixes had no pre-existing Git changes at the initial scoped `git status --short` check. The shared glue and coordinating ticket already contain concurrent work and are patched only at exact owned blocks.

## Baseline Roster

| Root | Inline Variants | Retained Operations | Removed Fallbacks |
| --- | ---: | ---: | --- |
| PNG 1.2 Any | 17 | 15 | NoMutation, SetSnapshot |
| JPG JFIF-1.01 Any | 12 | 10 | NoMutation, SetSnapshot |
| BMP V3 Any | 7 | 5 | NoMutation, SetSnapshot |
| TIFF 6.0 Any | 8 | 6 | NoMutation, SetSnapshot |

Each root also contains one physical nested `📄set-snapshot` owner. These are fallback fixtures, not additional semantic operations. Direct payload structs retain the typed sparse diff behavior and explicit inverse operations. Canonical root text/binary components will assemble the leaf-owned codec entries, not introduce another dispatch switch.

## Validation Boundary

The coordinator's Demonstrator quick suite is active. No Cargo or Nx build may start until a compile-coherent checkpoint is reported and the coordinator authorizes the shared build. Static Bun validators and the independent nightly parse oracle do not claim runtime success.

## Execution Evidence

- Applicable root, `✏️s`, and STDIO `AGENTS.md` files read; no deeper artifact AGENTS files found.
- Governing pasted brief and coordinating control plane read.
- Initial commands: scoped `rg` inventory, source reads, and read-only Git status.
