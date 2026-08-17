# W6 batch1c — stdio migration + cargo proof

Ticket: `26/08/10/STDIO-ARTIFACTS-AND-IO` fan-out **batch1c** (12 plugins).

## Cargo check (exit 0)

| Plugin | Crate | Log |
|--------|-------|-----|
| imperative | `semio-s-plugin-imperative` | [`🧪w6-batch1c-semio-s-plugin-imperative.log`](🧪w6-batch1c-semio-s-plugin-imperative.log) |
| remodel | `semio-s-plugin-remodel` | [`🧪w6-batch1c-semio-s-plugin-remodel.log`](🧪w6-batch1c-semio-s-plugin-remodel.log) |
| demonstrator / playground | `semio-s-plugin-demonstrator` | [`🧪w6-batch1c-semio-s-plugin-demonstrator.log`](🧪w6-batch1c-semio-s-plugin-demonstrator.log) |
| animate / present | `semio-s-plugin-animate` | [`🧪w6-batch1c-semio-s-plugin-animate.log`](🧪w6-batch1c-semio-s-plugin-animate.log) |
| shooting | `semio-s-plugin-shooting` | [`🧪w6-batch1c-semio-s-plugin-shooting.log`](🧪w6-batch1c-semio-s-plugin-shooting.log) |
| sequence | `semio-s-plugin-sequence` | [`🧪w6-batch1c-semio-s-plugin-sequence.log`](🧪w6-batch1c-semio-s-plugin-sequence.log) |
| architect / program | `semio-s-plugin-architect` | [`🧪w6-batch1c-semio-s-plugin-architect.log`](🧪w6-batch1c-semio-s-plugin-architect.log) |
| process / process3d | `semio-s-plugin-process` | [`🧪w6-batch1c-semio-s-plugin-process.log`](🧪w6-batch1c-semio-s-plugin-process.log) |
| lowpoly | `semio-s-plugin-lowpoly` | [`🧪w6-batch1c-semio-s-plugin-lowpoly.log`](🧪w6-batch1c-semio-s-plugin-lowpoly.log) |
| reasoning / wires | **`semio-s-plugin-reasoning-mindmap`** (not `semio-s-plugin-reasoning`) | [`🧪w6-batch1c-semio-s-plugin-reasoning-mindmap.log`](🧪w6-batch1c-semio-s-plugin-reasoning-mindmap.log) |
| space / home | `semio-s-plugin-space` | [`🧪w6-batch1c-semio-s-plugin-space.log`](🧪w6-batch1c-semio-s-plugin-space.log) |
| sourcing / curate (`🪵️sourcing` → `🗂️curate`) | `semio-s-plugin-sourcing` | [`🧪w6-batch1c-semio-s-plugin-sourcing.log`](🧪w6-batch1c-semio-s-plugin-sourcing.log) |

**Proof command (all exit 0):**

```bash
for c in semio-s-plugin-imperative semio-s-plugin-remodel semio-s-plugin-demonstrator \
  semio-s-plugin-animate semio-s-plugin-shooting semio-s-plugin-sequence \
  semio-s-plugin-architect semio-s-plugin-process semio-s-plugin-lowpoly \
  semio-s-plugin-reasoning-mindmap semio-s-plugin-space semio-s-plugin-sourcing; do
  cargo check -p "$c" || exit 1
done
```

## IO / glue fixes applied

- **Batch1a/b-style stdio IO** (no fake `engine::encode_*`): pack round-trip + json/md/csv + serde wire for zip/xlsx/etc. — generator [`generators/w6_batch1c_fix_all.py`](generators/w6_batch1c_fix_all.py).
- **Schema const names:** `ARCHITECT_PROGRAM_SCHEMA`, `PROCESS_3D_SCHEMA`, `SOURCING_CURATE_SCHEMA`, `MINDMAP_WIRES_SCHEMA`, `SHomeSnapshot` / `SHomeMutation`.
- **Remodel:** builder/decomposer/IO use `RemodelSnapshot` (not `WatertightReportSnapshot`).
- **Shooting:** top-level `artifacts::shooting::pack` re-export for engine protocol paths.
- **Lowpoly:** engine re-exports for `paint` + `media` helpers (`mesh_from_mesh_document`, …).
- **Animate:** `engine::animate` + `animate_video` glue tree (nested facet modules + root re-exports).
- **Space:** `OsMediaExportResult` duplicate impl gated with `#[cfg(not(feature = "os-host-full"))]` in framework host.
- **Transitive deps for demonstrator:** procedural/puzzle/gis example `include_str!` paths (`../../../📚️examples/…`), puzzle diff text glob-import ordering, procedural `ArtifactKindSpec` comma.

## Roster note

`generators/w6-batch1c.json` crate for reasoning updated to **`semio-s-plugin-reasoning-mindmap`**.
