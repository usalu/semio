# W0 Engine Move Map

Generated: 2026-08-12
Ticket: `26/08/12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS`

## Summary

| Metric | Count |
|--------|------:|
| Standards-level `⚙️engine` directories | 94 |
| Flat engines (root component only) | 76 |
| Engines with immediate subdirs | 18 |
| Nested subdir depth ≥2 | 1 |
| Subset-level `🪆️subsets/*/⚙️engine` already present | 0 |

## Move Rule

Every standards-level engine relocates under the `✳️any` subset:

```
.../🏅️standards/<version>/⚙️engine/
  → .../🏅️standards/<version>/🪆️subsets/✳️any/⚙️engine/
```

After move, `glue.rs` shim blocks that re-export `super::standards::<ver>::engine::*` must target `super::standards::<ver>::subsets::any::engine::*` instead.

## Subset-Level Engines (Already Present)

**None found.** No `🪆️subsets/*/⚙️engine` directories exist yet — all engines are still at standards level.

## Engines With Extra Subdirs (Relocation Flags)

These engines have immediate child directories beyond the root `🦀️component.rs`. All subdirs move with the parent into `🪆️subsets/✳️any/⚙️engine/`.

### `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/⚙️engine`

**Destination:** `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine`

**Subdirs (1):** `terrain`
**Root files:** `🦀️component.rs`

### `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine`

**Destination:** `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine`

**Subdirs (8):** `⏱️rate`, `🎛️config`, `🎞️animation`, `🎥️camera`, `🎥️video`, `🎬️scene`, `📐️geometry`, `🔤️text`
**Root files:** `🦀️component.rs`

### `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/⚙️engine`

**Destination:** `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine`

**Subdirs (3):** `🎵️modal-buckling`, `🕸️meshing`, `🗺️mesh-preview`
**Root files:** `🦀️component.rs`

### `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/⚙️engine`

**Destination:** `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine`

**Subdirs (3):** `🎵️modal-buckling`, `🕸️meshing`, `🗺️mesh-preview`
**Root files:** `🦀️component.rs`

### `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/⚙️engine`

**Destination:** `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine`

**Subdirs (10):** `↔️adjacency`, `✅️validate`, `🎁️outputs`, `📄️report`, `📊️status-summary`, `📐️template`, `📤️exchange`, `🔍️search`, `🔬️analyze`, `🧭️trace`
**Root files:** `🦀️component.rs`

### `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/⚙️engine`

**Destination:** `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine`

**Subdirs (4):** `🔩️metal`, `🤖️robotic`, `🧱️concrete`, `🪵️wood`
**Root files:** `🦀️component.rs`

### `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/⚙️engine`

**Destination:** `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine`

**Subdirs (2):** `🎨️paint`, `🧵️media`
**Root files:** `🦀️component.rs`

### `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/⚙️engine`

**Destination:** `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine`

**Subdirs (1):** `🎬️scene`
**Root files:** `🦀️component.rs`

### `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/⚙️engine`

**Destination:** `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine`

**Subdirs (4):** `📥️geometry-import`, `🔄️transformation`, `🔍️construct`, `🕹️interaction`
**Root files:** `🦀️component.rs`

### `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine`

**Destination:** `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine`

**Subdirs (10):** `🌟️feature`, `🌫️dense`, `🎥️video`, `🏃️motion`, `🏭️reconstruction`, `📷️camera`, `📸️sfm`, `🖼️images`, `🗺️geo`, `🥽️mesh`
**Root files:** `🦀️component.rs`

### `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/⚙️engine`

**Destination:** `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine`

**Subdirs (50):** `air_exchange`, `air_system`, `airflow_network`, `calendar`, `coils`, `comfort`, `controls`, `curves`, `daylight`, `dispatch`, `economics`, `electrical`, `envelope`, `error`, `evaporative`, `fans`, `faults`, `fenestration`, `gains`, `geometry`, `heat_recovery`, `humidity_eq`, `hvac_topo`, `iaq`, `ideal_hvac`, `kernel`, `material`, `meters`, `metrics`, `model`, `num`, `output`, `plant`, `precompute`, `props`, `refrigeration`, `results`, `room_air`, `schedule`, `shw`, `sim`, `site`, `sizing`, `solar`, `solar_thermal`, `terminal`, `units`, `water`, `zone_air`, `zone_hvac`
**Root files:** `🟦️component.ts`, `🦀️component.rs`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/⚙️engine`

**Destination:** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/⚙️engine`

**Subdirs (2):** `🎥️h264`, `📦️boxes`
**Root files:** `🦀️component.rs`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/⚙️engine`

**Destination:** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/⚙️engine`

**Subdirs (1):** `🏛️spatial`
**Root files:** `🦀️component.rs`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/⚙️engine`

**Destination:** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/⚙️engine`

**Subdirs (3):** `📐️part21`, `🧱️brep`, `🪜️ladder`
**Root files:** `🦀️component.rs`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/⚙️engine`

**Destination:** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/⚙️engine`

**Subdirs (2):** `🧮️geometry`, `🧰️triples`
**Root files:** `🦀️component.rs`

### `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/⚙️engine`

**Destination:** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine`

**Subdirs (5):** `🎲️board-host`, `📐️layout`, `🔗️linking`, `🔣️icons`, `🖌️brush`
**Root files:** `🦀️component.rs`

### `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/⚙️engine`

**Destination:** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine`

**Subdirs (2):** `✂️transfer`, `📐️flatten`
**Root files:** `🦀️component.rs`

### `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/⚙️engine`

**Destination:** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine`

**Subdirs (4):** `⏳️session`, `📐️geometry`, `🖌️brush`, `🪣️fill`
**Root files:** `🦀️component.rs`

### Nested Subdirs (depth ≥2)

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/⚙️engine/📐️geometry/🎛flatten`

## Notable Shared-Module Cases

| Artifact | Shared module | Notes |
|----------|---------------|-------|
| `🧿️semio` | `🧮️geometry`, `🧰️triples` | Geometry and triple-store helpers must land in `any` alongside root engine |
| `🔋️model` | 50 domain subdirs | Crate-root flat `pub mod` in `glue.rs` L31–~130; all paths need `🪆️subsets/✳️any/` inserted |
| `🎬️present` | 8 subdirs + `animate` alias tree | `glue.rs` L42–57 `#[path]` + L58–83 duplicate `animate::*` re-export aliases |
| `🧊️3d` (puzzle) | `📐️geometry/🎛flatten` | Only nested-depth-2 case in repo |

## Full Move Table by Plugin

### Plugin: `✒️writer`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `➗️mathematical`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `🌀️procedural`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `🌊️flow`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `🌍️gis`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | terrain |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `🌿️vcs`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `🎞️animate`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | ⏱️rate, 🎛️config, 🎞️animation, 🎥️camera, 🎥️video, 🎬️scene, 📐️geometry, 🔤️text |

### Plugin: `🎥️shooting`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `🎪️demonstrator`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `🎬️sequence`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `🏗️fem`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | 🎵️modal-buckling, 🕸️meshing, 🗺️mesh-preview |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | 🎵️modal-buckling, 🕸️meshing, 🗺️mesh-preview |

### Plugin: `🏛️architect`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | ↔️adjacency, ✅️validate, 🎁️outputs, 📄️report, 📊️status-summary, 📐️template, 📤️exchange, 🔍️search, 🔬️analyze, 🧭️trace |

### Plugin: `🏭️process`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | 🔩️metal, 🤖️robotic, 🧱️concrete, 🪵️wood |

### Plugin: `💠️lowpoly`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | 🎨️paint, 🧵️media |

### Plugin: `💡️reasoning`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `📋️forms`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `📏️layout`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | 🎬️scene |

### Plugin: `📐️cad`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | 📥️geometry-import, 🔄️transformation, 🔍️construct, 🕹️interaction |

### Plugin: `📕️norm`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `📖️playbook`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `📜️imperative`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `📸️remodel`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | 🌟️feature, 🌫️dense, 🎥️video, 🏃️motion, 🏭️reconstruction, 📷️camera, 📸️sfm, 🖼️images, 🗺️geo, 🥽️mesh |

### Plugin: `🔋️energy`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | air_exchange, air_system, airflow_network, calendar, coils, comfort, controls, curves, daylight, dispatch, economics, electrical, envelope, error, evaporative, fans, faults, fenestration, gains, geometry, heat_recovery, humidity_eq, hvac_topo, iaq, ideal_hvac, kernel, material, meters, metrics, model, num, output, plant, precompute, props, refrigeration, results, room_air, schedule, shw, sim, site, sizing, solar, solar_thermal, terminal, units, water, zone_air, zone_hvac |

### Plugin: `🔱️trinity`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `🕸️dag`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `🖍️draw`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `🖨️raster`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `🗄️stdio`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/⚙️engine` | 🎥️h264, 📦️boxes |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/⚙️engine` | 🏛️spatial |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/⚙️engine` | 📐️part21, 🧱️brep, 🪜️ladder |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/⚙️engine` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/⚙️engine` | 🧮️geometry, 🧰️triples |

### Plugin: `🗒️note`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `🧩️puzzle`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | 🎲️board-host, 📐️layout, 🔗️linking, 🔣️icons, 🖌️brush |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | ✂️transfer, 📐️flatten |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | ⏳️session, 📐️geometry, 🖌️brush, 🪣️fill |

### Plugin: `🧱️block`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `🪐️space`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |

### Plugin: `🪵️sourcing`

| Source | Destination | Subdirs |
|--------|-------------|---------|
| `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/⚙️engine` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | — |
