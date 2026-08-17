# Plugin Migration Inventory

This report provides comprehensive metrics for all 33 plugins used for planning the migration.

## Plugin Inventory Table

| Plugin | Crate | Art | Std | Sub | IO | Facets | ArtDef | Ed | Vw | Doc | Kind | Art | ArtDef | LOC | Path | Shim | Stdio | SRefs | NonTax |
|--------|-------|-----|-----|-----|----|----|--------|----|----|-----|------|-----|--------|------|------|------|-------|-------|--------|
| `✒️writer` | semio-s-plugin-writer | 1 | 1 | 1 | 10 | 1 | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 492 | 156 | 13 | yes | 17 | — |
| `➗️mathematical` | semio-s-plugin-mathematical | 1 | 1 | 1 | 8 | 1 | 1 | 2 | 1 | 0 | 2 | 0 | 0 | 550 | 172 | 13 | yes | 11 | — |
| `🌀️procedural` | semio-s-plugin-procedural | 3 | 3 | 3 | 32 | 2 | 3 | 2 | 0 | 0 | 2 | 0 | 0 | 1476 | 514 | 29 | yes | 29 | — |
| `🌊️flow` | semio-s-plugin-flow | 1 | 1 | 1 | 8 | 1 | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 564 | 193 | 13 | yes | 12 | — |
| `🌍️gis` | semio-s-plugin-gis | 2 | 2 | 2 | 32 | 2 | 2 | 2 | 1 | 0 | 3 | 0 | 0 | 1016 | 335 | 22 | yes | 37 | — |
| `🌿️vcs` | semio-s-plugin-vcs | 1 | 1 | 1 | 10 | 1 | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 459 | 146 | 13 | yes | 10 | — |
| `🎞️animate` | semio-s-plugin-animate | 1 | 1 | 1 | 14 | 1 | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 575 | 194 | 8 | yes | 49 | — |
| `🎥️shooting` | semio-s-plugin-shooting | 1 | 1 | 1 | 20 | 1 | 1 | 1 | 1 | 0 | 1 | 0 | 0 | 846 | 304 | 9 | yes | 30 | — |
| `🎪️demonstrator` | semio-s-plugin-demonstrator | 1 | 1 | 1 | 8 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 314 | 96 | 0 | yes | 10 | — |
| `🎬️sequence` | semio-s-plugin-sequence | 1 | 1 | 1 | 8 | 1 | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 462 | 149 | 8 | yes | 11 | — |
| `🏗️fem` | semio-s-plugin-fem | 2 | 2 | 2 | 20 | 2 | 2 | 2 | 0 | 0 | 3 | 0 | 0 | 1295 | 469 | 22 | yes | 34 | — |
| `🏛️architect` | semio-s-plugin-architect | 1 | 1 | 1 | 10 | 1 | 1 | 1 | 1 | 0 | 1 | 0 | 0 | 2838 | 1194 | 13 | yes | 32 | — |
| `🏭️process` | semio-s-plugin-process | 1 | 1 | 1 | 18 | 1 | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 681 | 232 | 8 | yes | 46 | — |
| `💠️lowpoly` | semio-s-plugin-lowpoly | 1 | 1 | 1 | 18 | 1 | 1 | 2 | 1 | 0 | 2 | 0 | 0 | 693 | 241 | 8 | yes | 23 | — |
| `💡️reasoning` | semio-s-plugin-reasoning-mindmap | 1 | 1 | 1 | 12 | 1 | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 516 | 171 | 8 | yes | 15 | — |
| `📋️forms` | semio-s-plugin-forms | 1 | 1 | 1 | 8 | 1 | 1 | 1 | 1 | 0 | 2 | 0 | 0 | 503 | 173 | 8 | yes | 11 | — |
| `📏️layout` | semio-s-plugin-layout | 1 | 1 | 1 | 12 | 1 | 1 | 1 | 1 | 0 | 2 | 0 | 0 | 693 | 248 | 8 | yes | 28 | — |
| `📐️cad` | semio-s-plugin-cad | 1 | 1 | 1 | 16 | 1 | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 716 | 246 | 8 | yes | 66 | — |
| `📕️norm` | semio-s-plugin-norm | 15 | 15 | 15 | 0 | 16 | 1 | 15 | 1 | 0 | 16 | 0 | 0 | 6928 | 2413 | 345 | yes | 14 | 🎚️config:       1;👥️presence:       1;📄️artifact:       1;🖥️app-surface:       1; |
| `📖️playbook` | semio-s-plugin-playbook | 1 | 1 | 1 | 10 | 1 | 1 | 1 | 1 | 0 | 2 | 0 | 0 | 492 | 157 | 8 | yes | 7 | — |
| `📜️imperative` | semio-s-plugin-imperative | 1 | 1 | 1 | 8 | 1 | 1 | 1 | 1 | 0 | 2 | 0 | 0 | 446 | 142 | 8 | yes | 16 | — |
| `📸️remodel` | semio-s-plugin-remodel | 1 | 1 | 1 | 18 | 1 | 1 | 1 | 1 | 0 | 2 | 0 | 0 | 975 | 355 | 8 | yes | 39 | — |
| `🔋️energy` | semio-s-plugin-energy | 1 | 1 | 1 | 10 | 1 | 1 | 2 | 0 | 0 | 2 | 0 | 0 | 536 | 157 | 11 | yes | 29 | — |
| `🔱️trinity` | semio-s-plugin-trinity | 2 | 2 | 2 | 22 | 2 | 2 | 2 | 1 | 0 | 3 | 0 | 0 | 1247 | 378 | 22 | yes | 30 | — |
| `🕸️dag` | semio-s-plugin-dag | 1 | 1 | 1 | 12 | 1 | 1 | 2 | 1 | 0 | 2 | 0 | 0 | 558 | 190 | 13 | yes | 16 | — |
| `🖍️draw` | semio-s-plugin-draw | 1 | 1 | 1 | 12 | 1 | 1 | 2 | 1 | 0 | 2 | 0 | 0 | 593 | 203 | 8 | yes | 10 | — |
| `🖨️raster` | semio-s-plugin-raster | 1 | 1 | 1 | 18 | 1 | 1 | 1 | 1 | 0 | 2 | 0 | 0 | 651 | 220 | 8 | yes | 25 | — |
| `🗄️stdio` | semio-s-plugin-stdio | 36 | 40 | 88 | 164 | 87 | 0 | 89 | 0 | 0 | 1 | 1 | 0 | 13970 | 4728 | 95 | yes | 1 | — |
| `🗒️note` | semio-s-plugin-note | 1 | 1 | 1 | 12 | 1 | 1 | 1 | 1 | 1 | 2 | 0 | 0 | 814 | 303 | 8 | yes | 37 | — |
| `🧩️puzzle` | semio-s-plugin-puzzle | 3 | 3 | 3 | 44 | 3 | 3 | 3 | 1 | 0 | 4 | 0 | 0 | 2507 | 951 | 33 | yes | 40 | — |
| `🧱️block` | semio-s-plugin-block | 3 | 3 | 3 | 36 | 3 | 3 | 3 | 0 | 0 | 4 | 0 | 0 | 2140 | 805 | 33 | yes | 27 | — |
| `🪐️space` | semio-s-plugin-space | 2 | 2 | 2 | 10 | 2 | 2 | 2 | 1 | 0 | 3 | 0 | 0 | 847 | 273 | 8 | yes | 15 | — |
| `🪵️sourcing` | semio-s-plugin-sourcing | 1 | 1 | 1 | 12 | 1 | 1 | 1 | 1 | 0 | 1 | 0 | 0 | 462 | 147 | 8 | yes | 18 | — |
|--------|-------|-----|-----|-----|----|----|--------|----|----|-----|------|-----|--------|------|------|------|-------|-------|--------|
| **TOTAL** | — | 92 | 96 | 144 | 652 | 143 | 42 | 148 | 19 | 1 | 74 | 1 | 0 | 47855 | 16655 | 827 | — | 795 | — |

## Summary Statistics

### Total Counts Across All Plugins

| Metric | Count |
|--------|-------|
| Total Artifacts | 92 |
| Total Standards | 96 |
| Total Subsets | 144 |
| Total IO Leaves | 652 |
| Total derive_artifact_facets invocations | 143 |
| Total ArtifactDefinition::new | 42 |
| Total glue.rs LOC | 47855 |
| Total stdio plugin references | 795 |

## Migration Batch Recommendations

### Batch 1 (Cheapest/Most Isolated - ~7 plugins)

These plugins have minimal dependencies and simple structures:

- `🎪️demonstrator` - Single artifact, minimal IO leaves
- `🎬️sequence` - Single artifact, minimal IO leaves
- `🌿️vcs` - Single artifact, minimal IO leaves
- `📋️forms` - Single artifact, minimal IO leaves
- `📜️imperative` - Single artifact, minimal IO leaves
- `🖍️draw` - Single artifact, minimal IO leaves
- `🪵️sourcing` - Single artifact, minimal IO leaves

### Batch 2 (Moderately Complex - ~7 plugins)

Plugins with 1 artifact but more IO dependencies:

- `✒️writer` - 10 IO leaves, Multiple IO families
- `➗️mathematical` - 8 IO leaves, Multiple IO families
- `🌊️flow` - 8 IO leaves, Multiple IO families
- `🎞️animate` - 14 IO leaves, Multiple IO families
- `🏭️process` - 18 IO leaves, High IO complexity
- `💠️lowpoly` - 18 IO leaves, High IO complexity
- `📏️layout` - 12 IO leaves, Moderate IO

### Batch 3 (Multi-Artifact - ~7 plugins)

Plugins with 2-3 artifacts but no cross-artifact dependencies:

- `🌍️gis` - 2 artifacts, 32 IO leaves - GIS artifacts with separate standards
- `🏗️fem` - 2 artifacts, 20 IO leaves - FEM analysis with standards
- `🔱️trinity` - 2 artifacts, 22 IO leaves - Trinity system with standards
- `🪐️space` - 2 artifacts, 10 IO leaves - Space/3D artifacts
- `🌀️procedural` - 3 artifacts, 32 IO leaves - Procedural artifacts
- `🧩️puzzle` - 3 artifacts, 44 IO leaves - Puzzle components
- `🧱️block` - 3 artifacts, 36 IO leaves - Block structures

### Batch 4 (Complex Structures - ~8 plugins)

High-complexity plugins reserved for last due to glue.rs size and shim count:

- `🎥️shooting` - 20 IO leaves - High-complexity single artifact
- `🎪️demonstrator` - 8 IO leaves - Reference architecture
- `🏛️architect` - 10 IO leaves - Large glue.rs (2838 LOC)
- `💡️reasoning` - 12 IO leaves - Reasoning system
- `📖️playbook` - 10 IO leaves - Playbook/workflow
- `📐️cad` - 16 IO leaves - CAD system
- `📸️remodel` - 18 IO leaves - Remodeling system
- `🔋️energy` - 10 IO leaves - Energy/physics system

**Special Notes:**
- `🗄️stdio` is the foundation plugin with 36 artifacts, 88 subsets, 164 IO leaves. Migrate this FIRST or use as reference.
- `📕️norm` is highly complex with 15 artifacts and 548 pub use shims. Migrate LAST or consider breaking up.
