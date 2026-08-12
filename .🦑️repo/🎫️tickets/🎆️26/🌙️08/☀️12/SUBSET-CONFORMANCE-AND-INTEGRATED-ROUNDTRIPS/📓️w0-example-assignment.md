# W0 Example Assignment Inventory

Generated: 2026-08-12

Scope: all artifact and standard examples under `✏️s/🔌️plugins/**/📚️examples/**`, excluding `🎛️apps`.

## Summary

| Metric | Count |
|--------|-------|
| Total example units | 128 |
| Artifact-level examples | 123 |
| Standard-level examples | 5 |
| With 🧪️tests | 72 |
| Without 🧪️tests | 56 |
| Flagged for dialect/subset split | 8 |

### Assignment rules applied

1. If path is under a standard, prefer that standard's `✳️any` unless assets/code clearly target a named profile subset.
2. Otherwise assign to the artifact's primary standard's `✳️any`.
3. If multiple standards exist and example is at artifact level, note which standard's any; default to the first/primary standard directory found.
4. Flag splits needed if one example clearly spans multiple dialects or subsets.

Subset detection: parse `🪆️subsets/✳️…/` references in `🦀️component.rs` and `subset=` in `🗣️*.dsl.semio` assets.

## Per-example inventory

| plugin | artifact | current_level | current_path | standard_if_any | assigned_subset | assignment_rule | has_tests | has_assets | asset_names | notes |
|--------|----------|---------------|--------------|-----------------|-----------------|-----------------|-----------|------------|-------------|-------|
| ✒️writer | ✒️writer | artifact | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️dag-example.dsl.semio, 🗣️example.dsl.semio |  |
| ➗️mathematical | ➗️mathematical | artifact | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🌀️procedural | 🌀️procedural2d | artifact | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🌀️procedural | 🧊️procedural3d | artifact | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/🎬️box-fillet-preview` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️box-fillet-preview.pack.semio, 📡️box-fillet-preview.spr.semio, 🔧️box-fillet-preview.op.semio, … (+1) |  |
| 🌀️procedural | 🧊️procedural3d | artifact | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/🎬️box-shell-preview` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️box-shell-preview.pack.semio, 📡️box-shell-preview.spr.semio, 🔧️box-shell-preview.op.semio, … (+1) |  |
| 🌀️procedural | 🧊️procedural3d | artifact | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/🎬️face-sweep-extrude` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️face-sweep-extrude.pack.semio, 📡️face-sweep-extrude.spr.semio, 🔧️face-sweep-extrude.op.semio, … (+1) |  |
| 🌀️procedural | 🧊️procedural3d | artifact | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/🎬️hexagonal-mushroom-column` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️hexagonal-mushroom-column.pack.semio, 📡️hexagonal-mushroom-column.spr.semio, 🔧️hexagonal-mushroom-column.op.semio, … (+1) |  |
| 🌀️procedural | 🧊️procedural3d | artifact | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/🎬️rectangle-extrude-volume` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️rectangle-extrude-volume.pack.semio, 📡️rectangle-extrude-volume.spr.semio, 🔧️rectangle-extrude-volume.op.semio, … (+1) |  |
| 🌀️procedural | 🧊️procedural3d | artifact | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/🎬️rectangle-wire-preview` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️rectangle-wire-preview.pack.semio, 📡️rectangle-wire-preview.spr.semio, 🔧️rectangle-wire-preview.op.semio, … (+1) |  |
| 🌀️procedural | 🧊️procedural3d | artifact | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/🎬️sphere-box-fuse` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️sphere-box-fuse.pack.semio, 📡️sphere-box-fuse.spr.semio, 🔧️sphere-box-fuse.op.semio, … (+1) |  |
| 🌀️procedural | 🧊️procedural3d | artifact | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/🎬️sphere-cut-with-torus` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️sphere-cut-with-torus.pack.semio, 📡️sphere-cut-with-torus.spr.semio, 🔧️sphere-cut-with-torus.op.semio, … (+1) |  |
| 🌊️flow | 🌊️flow | artifact | `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🌍️gis | 🏔️gisterrain | artifact | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🌍️gis | 🗺️gismap | artifact | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🌿️vcs | 🌿️vcs | artifact | `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🎞️animate | 🎬️present | artifact | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🎥️shooting | 🎥️shooting | artifact | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🎪️demonstrator | 🎪️playground | artifact | `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🎬️sequence | 🎬️sequence | artifact | `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🏗️fem | ◻2d | artifact | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🏗️fem | 🧊️3d | artifact | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🏛️architect | 🏛️program | artifact | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🏭️process | 🧊️process3d | artifact | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 💠️lowpoly | 💠️lowpoly | artifact | `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 💡️reasoning | 🔌️wires | artifact | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 📋️forms | 📋️forms | artifact | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 📏️layout | 📏️layout | artifact | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 📐️cad | 📐️cad | artifact | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 📕️norm | 📓️iso16757 | artifact | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 📕️norm | 📔️vdi3805 | artifact | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 📕️norm | 📕️din4108 | artifact | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 📕️norm | 📗️din16798 | artifact | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 📕️norm | 📘️en1990 | artifact | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/📚️examples/📕️high-consequence-office` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️high-consequence-office.pack.semio, 📡️high-consequence-office.spr.semio, 🔧️high-consequence-office.op.semio, … (+1) |  |
| 📕️norm | 📘️en1991 | artifact | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/📚️examples/📕️retail-hydrocarbon-fire` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️retail-hydrocarbon-fire.pack.semio, 📡️retail-hydrocarbon-fire.spr.semio, 🔧️retail-hydrocarbon-fire.op.semio, … (+1) |  |
| 📕️norm | 📘️en1992 | artifact | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/📚️examples/📕️liquid-retaining-fem-anchor` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️liquid-retaining-fem-anchor.pack.semio, 📡️liquid-retaining-fem-anchor.spr.semio, 🔧️liquid-retaining-fem-anchor.op.semio, … (+1) |  |
| 📕️norm | 📘️en1993 | artifact | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/📚️examples/📕️high-strength-connection` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️high-strength-connection.pack.semio, 📡️high-strength-connection.spr.semio, 🔧️high-strength-connection.op.semio, … (+1) |  |
| 📕️norm | 📘️en1994 | artifact | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994/📚️examples/📕️composite-bridge-girder` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️composite-bridge-girder.pack.semio, 📡️composite-bridge-girder.spr.semio, 🔧️composite-bridge-girder.op.semio, … (+1) |  |
| 📕️norm | 📘️en1995 | artifact | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/📚️examples/📕️glulam-footbridge` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️glulam-footbridge.pack.semio, 📡️glulam-footbridge.spr.semio, 🔧️glulam-footbridge.op.semio, … (+1) |  |
| 📕️norm | 📘️en1996 | artifact | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/📚️examples/📕️loadbearing-wall` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️loadbearing-wall.pack.semio, 📡️loadbearing-wall.spr.semio, 🔧️loadbearing-wall.op.semio, … (+1) |  |
| 📕️norm | 📘️en1997 | artifact | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 📕️norm | 📘️en1998 | artifact | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/📚️examples/📕️seismic-rc-frame` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️seismic-rc-frame.pack.semio, 📡️seismic-rc-frame.spr.semio, 🔧️seismic-rc-frame.op.semio, … (+1) |  |
| 📕️norm | 📘️en1999 | artifact | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/📚️examples/📕️aluminium-roof-purlin` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️aluminium-roof-purlin.pack.semio, 📡️aluminium-roof-purlin.spr.semio, 🔧️aluminium-roof-purlin.op.semio, … (+1) |  |
| 📕️norm | 📙️din18599 | artifact | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 📖️playbook | 📖️playbook | artifact | `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 📜️imperative | 📜️imperative | artifact | `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 📸️remodel | 📸️remodel | artifact | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🔋️energy | 🔋️model | artifact | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🔱️trinity | ♻️rewrite | artifact | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🔱️trinity | 🔌️jack | artifact | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🕸️dag | 🕸️dag | artifact | `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🖍️draw | 🖍️draw | artifact | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🖨️raster | 🖨️raster | artifact | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🗄️stdio | ☁️las | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/📚️examples/🎬️demo` | 🔖️1.0 | ✳️any | 2: artifact-level → primary standard 🔖️1.0's ✳️any | False | True | example.las, 🎒️example.bin, 🎒️example.pack.semio, … (+1) |  |
| 🗄️stdio | ☁️ply | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/📚️examples/🎬️demo` | 🔖️1.0 | ✳️any | 2: artifact-level → primary standard 🔖️1.0's ✳️any | False | True | example.ply, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🌐️html | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/📚️examples/🎬️demo` | 🔖️5 | ✳️any | 2: artifact-level → primary standard 🔖️5's ✳️any | False | True | example.html, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🌦️epw | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📚️examples/🎬️demo` | 🔖️energyplus | ✳️any | 2: artifact-level → primary standard 🔖️energyplus's ✳️any | False | True | example.epw, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🎒️zip | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📚️examples/🎬️demo` | 🔖️2.0 | ✳️any | 2: artifact-level → primary standard 🔖️2.0's ✳️any | False | True | 🎒️example.pack.semio, 🎒️example.zip, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🎞️gif | standard | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/📚️examples/🎬️demo` | 🔖️87a | ✳️any | 1: under standard → standard's ✳️any | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🎞️gif | standard | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/📚️examples/🎬️demo` | 🔖️89a | ✳️any | 1: under standard → standard's ✳️any | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🎞️gif | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📚️examples/🎬️demo` | 🔖️87a | ✳️any | 2: artifact-level → primary standard 🔖️87a's ✳️any | False | True | example.gif, 🎒️example.pack.semio, 🗣️example.dsl.semio | multiple standards: 🔖️87a, 🔖️89a; default primary 🔖️87a; SPLIT? example at artifact level may need per-dialect copies |
| 🗄️stdio | 🎞️gif | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📚️examples/💃️dancing` | 🔖️87a | ✳️any | 2: artifact-level → primary standard 🔖️87a's ✳️any | True | True | 🎒️example.pack.semio, 🖼️dancing.gif | multiple standards: 🔖️87a, 🔖️89a; default primary 🔖️87a; SPLIT? example at artifact level may need per-dialect copies |
| 🗄️stdio | 🎞️pptx | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/📚️examples/🎬️demo` | 🔖️ecma-376 | ✳️any | 2: artifact-level → primary standard 🔖️ecma-376's ✳️any | False | True | example.pptx, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🎥️mp4 | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/📚️examples/🎬️demo` | 🔖️isobmff | ✳️any | 2: artifact-level → primary standard 🔖️isobmff's ✳️any | False | True | example.mp4, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🎨️svg | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/📚️examples/🎬️demo` | 🔖️1.1 | ✳️any | 2: artifact-level → primary standard 🔖️1.1's ✳️any | False | True | example.svg, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🎵️mp3 | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/📚️examples/🎬️demo` | 🔖️mpeg1-layer3 | ✳️any | 2: artifact-level → primary standard 🔖️mpeg1-layer3's ✳️any | False | True | example.mp3, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🏗️ifc | standard | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/📚️examples/🎬️demo` | 🔖️2x3 | ✳️any | 1: under standard → standard's ✳️any | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🏗️ifc | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📚️examples/🎬️demo` | 🔖️2x3 | ✳️any | 2: artifact-level → primary standard 🔖️2x3's ✳️any | False | True | example.ifc, 🎒️example.pack.semio, 🗣️example.dsl.semio | multiple standards: 🔖️2x3, 🔖️4; default primary 🔖️2x3; SPLIT? example at artifact level may need per-dialect copies |
| 🗄️stdio | 💬️bcf | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/📚️examples/🎬️demo` | 🔖️2.1 | ✳️any | 2: artifact-level → primary standard 🔖️2.1's ✳️any | False | True | example.bcf, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 💾️binary | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/📚️examples/🎬️demo` | 🔖️raw | ✳️any | 2: artifact-level → primary standard 🔖️raw's ✳️any | False | True | 🎒️example.bin, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 📄txt | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/📚️examples/🎬️demo` | 🔖️utf-8 | ✳️any | 2: artifact-level → primary standard 🔖️utf-8's ✳️any | False | True | 🎒️example.pack.semio, 📄example.txt, 📡️example.spr.semio, … (+1) |  |
| 🗄️stdio | 📄️pdf | standard | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/📚️examples/🎬️demo` | 🔖️1.7 | ✳️any | 1: under standard → standard's ✳️any | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 📄️pdf | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/📚️examples/🎓️bachelor-thesis` | 🔖️1.4 | ✳️any | 2: artifact-level → primary standard 🔖️1.4's ✳️any | True | True | 🎒️example.pack.semio, 📄️bachelor-thesis.pdf | multiple standards: 🔖️1.4, 🔖️1.7; default primary 🔖️1.4; SPLIT? example at artifact level may need per-dialect copies |
| 🗄️stdio | 📄️pdf | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/📚️examples/🎬️demo` | 🔖️1.4 | ✳️any | 2: artifact-level → primary standard 🔖️1.4's ✳️any | False | True | example.pdf, 🎒️example.pack.semio, 🗣️example.dsl.semio | multiple standards: 🔖️1.4, 🔖️1.7; default primary 🔖️1.4; SPLIT? example at artifact level may need per-dialect copies |
| 🗄️stdio | 📊️csv | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/📚️examples/🎬️demo` | 🔖️rfc4180 | ✳️any | 2: artifact-level → primary standard 🔖️rfc4180's ✳️any | False | True | example.csv, 🎒️example.pack.semio, 📡️example.spr.semio, … (+1) |  |
| 🗄️stdio | 📐️step | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📚️examples/🎬️demo` | 🔖️ap214 | ✳️any | 2: artifact-level → primary standard 🔖️ap214's ✳️any | False | True | example.step, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 📑️tsv | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📚️examples/🎬️demo` | 🔖️iana | ✳️any | 2: artifact-level → primary standard 🔖️iana's ✳️any | False | True | example.tsv, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 📕️xlsx | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/📚️examples/🎬️demo` | 🔖️ecma-376 | ✳️any | 2: artifact-level → primary standard 🔖️ecma-376's ✳️any | False | True | example.xlsx, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 📜️docx | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/📚️examples/🎬️demo` | 🔖️ecma-376 | ✳️any | 2: artifact-level → primary standard 🔖️ecma-376's ✳️any | False | True | example.docx, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 📝️md | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/📚️examples/🎬️demo` | 🔖️commonmark | ✳️any | 2: artifact-level → primary standard 🔖️commonmark's ✳️any | False | True | example.md, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 📰xml | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/📚️examples/🎬️demo` | 🔖️1.0 | ✳️any | 2: artifact-level → primary standard 🔖️1.0's ✳️any | False | True | example.xml, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 📷️jpg | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/📚️examples/🎬️demo` | 🔖️jfif-1.01 | ✳️any | 2: artifact-level → primary standard 🔖️jfif-1.01's ✳️any | False | True | example.jpg, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 📷️png | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/📚️examples/🎬️demo` | 🔖️1.2 | ✳️any | 2: artifact-level → primary standard 🔖️1.2's ✳️any | False | True | example.png, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 📼️avi | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/📚️examples/🎬️demo` | 🔖️1.0 | ✳️any | 2: artifact-level → primary standard 🔖️1.0's ✳️any | False | True | example.avi, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🔊️wav | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📚️examples/🎬️demo` | 🔖️riff-pcm | ✳️any | 2: artifact-level → primary standard 🔖️riff-pcm's ✳️any | False | True | example.wav, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🔣️json | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/📚️examples/🎬️demo` | 🔖️rfc8259 | ✳️any | 2: artifact-level → primary standard 🔖️rfc8259's ✳️any | False | True | 🎒️example.pack.semio, 🔣️example.json, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🖊️dwg | standard | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/📚️examples/🎬️demo` | 🔖️ac1018 | ✳️any | 1: under standard → standard's ✳️any | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🖊️dwg | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/📚️examples/🎬️demo` | 🔖️ac1018 | ✳️any | 2: artifact-level → primary standard 🔖️ac1018's ✳️any | False | True | example.dwg, 🎒️example.pack.semio, 🗣️example.dsl.semio | multiple standards: 🔖️ac1018, 🔖️ac1024; default primary 🔖️ac1018; SPLIT? example at artifact level may need per-dialect copies |
| 🗄️stdio | 🖊️dwg | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/📚️examples/🏛️architectural` | 🔖️ac1018 | ✳️any | 2: artifact-level → primary standard 🔖️ac1018's ✳️any | True | True | 🎒️example.pack.semio, 📄️architectural.dwg | multiple standards: 🔖️ac1018, 🔖️ac1024; default primary 🔖️ac1018; SPLIT? example at artifact level may need per-dialect copies |
| 🗄️stdio | 🖊️dxf | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/📚️examples/🎬️demo` | 🔖️r12 | ✳️any | 2: artifact-level → primary standard 🔖️r12's ✳️any | False | True | example.dxf, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🖼️bmp | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/📚️examples/🎬️demo` | 🔖️v3 | ✳️any | 2: artifact-level → primary standard 🔖️v3's ✳️any | False | True | example.bmp, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🖼️tiff | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/📚️examples/🎬️demo` | 🔖️6.0 | ✳️any | 2: artifact-level → primary standard 🔖️6.0's ✳️any | False | True | example.tiff, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🗜️deflate | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/📚️examples/🎬️demo` | 🔖️rfc1950 | ✳️any | 2: artifact-level → primary standard 🔖️rfc1950's ✳️any | False | True | 🎒️example.pack.semio, 🗜️example.zz, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🟪️stl | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/📚️examples/🎬️demo` | 🔖️ascii | ✳️any | 2: artifact-level → primary standard 🔖️ascii's ✳️any | False | True | example.stl, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🧊️gltf | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📚️examples/🌱️metabolism` | 🔖️2.0 | ✳️any | 2: artifact-level → primary standard 🔖️2.0's ✳️any | True | True | 🎒️example.pack.semio, 🧊️base.glb |  |
| 🗄️stdio | 🧊️gltf | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📚️examples/🎬️demo` | 🔖️2.0 | ✳️any | 2: artifact-level → primary standard 🔖️2.0's ✳️any | False | True | example.gltf, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🧊️obj | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/📚️examples/🎬️demo` | 🔖️3.0 | ✳️any | 2: artifact-level → primary standard 🔖️3.0's ✳️any | False | True | example.obj, 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🧿️semio | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🌊️pipeline` | 🔖️v1 | ✳️flow | 2: artifact-level → assets/code clearly target ✳️flow | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🧿️semio | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🌐️envelope` | 🔖️v1 | ✳️any | 2: artifact-level → code/assets name multiple subsets ['✳️any', '✳️flow']; assign first ✳️any | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio | SPLIT? spans subsets: ✳️any, ✳️flow; SPLIT? conflicting subset signals in assets vs code |
| 🗄️stdio | 🧿️semio | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🎥️clip` | 🔖️v1 | ✳️video | 2: artifact-level → assets/code clearly target ✳️video | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🧿️semio | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🎬️demo` | 🔖️v1 | ✳️any | 2: artifact-level → primary standard 🔖️v1's ✳️any | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🧿️semio | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🎵️tone` | 🔖️v1 | ✳️audio | 2: artifact-level → assets/code clearly target ✳️audio | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🧿️semio | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🏢️building` | 🔖️v1 | ✳️model | 2: artifact-level → assets/code clearly target ✳️model | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🧿️semio | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/📃️note` | 🔖️v1 | ✳️text | 2: artifact-level → assets/code clearly target ✳️text | False | True |  |  |
| 🗄️stdio | 🧿️semio | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/📄️memo` | 🔖️v1 | ✳️document | 2: artifact-level → assets/code clearly target ✳️document | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🧿️semio | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/📐️drawing` | 🔖️v1 | ✳️cad | 2: artifact-level → assets/code clearly target ✳️cad | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🧿️semio | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/📽️deck` | 🔖️v1 | ✳️presentation | 2: artifact-level → assets/code clearly target ✳️presentation | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🧿️semio | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🕸️graph` | 🔖️v1 | ✳️value | 2: artifact-level → assets/code clearly target ✳️value | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🧿️semio | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🖍️sketch` | 🔖️v1 | ✳️drawing | 2: artifact-level → assets/code clearly target ✳️drawing | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🧿️semio | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🖼️swatch` | 🔖️v1 | ✳️image | 2: artifact-level → assets/code clearly target ✳️image | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🧿️semio | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🚶️walk` | 🔖️v1 | ✳️animation | 2: artifact-level → assets/code clearly target ✳️animation | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🧿️semio | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🧊️cube` | 🔖️v1 | ✳️mesh | 2: artifact-level → assets/code clearly target ✳️mesh | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗄️stdio | 🧿️semio | artifact | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🧊️solid` | 🔖️v1 | ✳️brep | 2: artifact-level → assets/code clearly target ✳️brep | False | True | 🎒️example.pack.semio, 🗣️example.dsl.semio |  |
| 🗒️note | 🗒️note | artifact | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🧩️puzzle | ◻2d | artifact | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/📚️examples/🌲️concrete-forest` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️forest.pack.semio, 📡️forest.spr.semio, 🔧️forest.op.semio, … (+1) |  |
| 🧩️puzzle | ◻2d | artifact | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/📚️examples/🏗️nakagin-capsule-tower` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️tower.pack.semio, 📡️tower.spr.semio, 🔧️tower.op.semio, … (+1) |  |
| 🧩️puzzle | 🖐️5d | artifact | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📚️examples/🌙️capsule-dream` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️dream.pack.semio, 🏅golden-poses.json, 📡️dream.spr.semio, … (+2) |  |
| 🧩️puzzle | 🖐️5d | artifact | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📚️examples/🌲️concrete-forest` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️forest.pack.semio, 📡️forest.spr.semio, 🔧️forest.op.semio, … (+1) |  |
| 🧩️puzzle | 🖐️5d | artifact | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📚️examples/🏗️nakagin-capsule-tower` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️tower.pack.semio, 📡️tower.spr.semio, 🔧️tower.op.semio, … (+1) |  |
| 🧩️puzzle | 🧊️3d | artifact | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📚️examples/🌲️concrete-forest` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️forest.pack.semio, 📡️forest.spr.semio, 🔧️forest.op.semio, … (+1) |  |
| 🧩️puzzle | 🧊️3d | artifact | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📚️examples/🏗️nakagin-capsule-tower` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️tower.pack.semio, 📡️tower.spr.semio, 🔧️tower.op.semio, … (+1) |  |
| 🧱️block | ◻2d | artifact | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/📚️examples/🎬️hexagonal-cut-concrete-forest-left` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️hexagonal-cut-concrete-forest-left.pack.semio, 📡️hexagonal-cut-concrete-forest-left.spr.semio, 🔧️hexagonal-cut-concrete-forest-left.op.semio, … (+1) |  |
| 🧱️block | ◻2d | artifact | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/📚️examples/🎬️hexagonal-cut-concrete-forest-right` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️hexagonal-cut-concrete-forest-right.pack.semio, 📡️hexagonal-cut-concrete-forest-right.spr.semio, 🔧️hexagonal-cut-concrete-forest-right.op.semio, … (+1) |  |
| 🧱️block | 🖐️5d | artifact | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📚️examples/🎬️hexagonal-cut-concrete-forest-left` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️hexagonal-cut-concrete-forest-left.pack.semio, 📡️hexagonal-cut-concrete-forest-left.spr.semio, 🔧️hexagonal-cut-concrete-forest-left.op.semio, … (+1) |  |
| 🧱️block | 🖐️5d | artifact | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📚️examples/🎬️nakagin-capsule` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️nakagin-capsule.pack.semio, 📡️nakagin-capsule.spr.semio, 🔧️nakagin-capsule.op.semio, … (+1) |  |
| 🧱️block | 🧊️3d | artifact | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📚️examples/🎬️hexagonal-cut-concrete-forest-left` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️hexagonal-cut-concrete-forest-left.pack.semio, 📡️hexagonal-cut-concrete-forest-left.spr.semio, 🔧️hexagonal-cut-concrete-forest-left.op.semio, … (+1) |  |
| 🧱️block | 🧊️3d | artifact | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📚️examples/🎬️nakagin-capsule` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🎒️nakagin-capsule.pack.semio, 📡️nakagin-capsule.spr.semio, 🔧️nakagin-capsule.op.semio, … (+1) |  |
| 🪐️space | 🏠️home | artifact | `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |
| 🪵️sourcing | 🗂️curate | artifact | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/📚️examples/🎬️demo` | 🔖️1 | ✳️any | 2: artifact-level → primary standard 🔖️1's ✳️any | True | True | 🗣️example.dsl.semio |  |

## Proposed move map

All example units move from current artifact/standard-level `📚️examples/` into `🏅️standards/<std>/🪆️subsets/<subset>/📚️examples/<slug>`.

| from_path | to_path |
|-----------|---------|
| `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📚️examples/🎬️demo` | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/📚️examples/🎬️demo` | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/🎬️box-fillet-preview` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️box-fillet-preview` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/🎬️box-shell-preview` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️box-shell-preview` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/🎬️face-sweep-extrude` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️face-sweep-extrude` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/🎬️hexagonal-mushroom-column` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️hexagonal-mushroom-column` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/🎬️rectangle-extrude-volume` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️rectangle-extrude-volume` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/🎬️rectangle-wire-preview` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️rectangle-wire-preview` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/🎬️sphere-box-fuse` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️sphere-box-fuse` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/🎬️sphere-cut-with-torus` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️sphere-cut-with-torus` |
| `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📚️examples/🎬️demo` | `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📚️examples/🎬️demo` | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📚️examples/🎬️demo` | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📚️examples/🎬️demo` | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📚️examples/🎬️demo` | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/📚️examples/🎬️demo` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/📚️examples/🎬️demo` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/📚️examples/🎬️demo` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/📚️examples/🎬️demo` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/📚️examples/📕️high-consequence-office` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️high-consequence-office` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/📚️examples/📕️retail-hydrocarbon-fire` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️retail-hydrocarbon-fire` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/📚️examples/📕️liquid-retaining-fem-anchor` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️liquid-retaining-fem-anchor` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/📚️examples/📕️high-strength-connection` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️high-strength-connection` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994/📚️examples/📕️composite-bridge-girder` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️composite-bridge-girder` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/📚️examples/📕️glulam-footbridge` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️glulam-footbridge` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/📚️examples/📕️loadbearing-wall` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️loadbearing-wall` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/📚️examples/🎬️demo` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/📚️examples/📕️seismic-rc-frame` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️seismic-rc-frame` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/📚️examples/📕️aluminium-roof-purlin` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️aluminium-roof-purlin` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/📚️examples/🎬️demo` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📚️examples/🎬️demo` | `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/📚️examples/🎬️demo` | `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/📚️examples/🎬️demo` | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📚️examples/💃️dancing` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/💃️dancing` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/📚️examples/🎓️bachelor-thesis` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/📚️examples/🏛️architectural` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📚️examples/🌱️metabolism` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/📚️examples/🌱️metabolism` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🌊️pipeline` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/📚️examples/🌊️pipeline` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🌐️envelope` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🌐️envelope` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🎥️clip` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/📚️examples/🎥️clip` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🎵️tone` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/📚️examples/🎵️tone` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🏢️building` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/📚️examples/🏢️building` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/📃️note` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/📚️examples/📃️note` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/📄️memo` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/📚️examples/📄️memo` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/📐️drawing` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/📚️examples/📐️drawing` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/📽️deck` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/📚️examples/📽️deck` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🕸️graph` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/📚️examples/🕸️graph` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🖍️sketch` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/📚️examples/🖍️sketch` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🖼️swatch` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/📚️examples/🖼️swatch` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🚶️walk` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/📚️examples/🚶️walk` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🧊️cube` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/📚️examples/🧊️cube` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🧊️solid` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/📚️examples/🧊️solid` |
| `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/📚️examples/🌲️concrete-forest` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/📚️examples/🏗️nakagin-capsule-tower` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📚️examples/🌙️capsule-dream` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌙️capsule-dream` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📚️examples/🌲️concrete-forest` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📚️examples/🏗️nakagin-capsule-tower` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📚️examples/🌲️concrete-forest` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📚️examples/🏗️nakagin-capsule-tower` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower` |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/📚️examples/🎬️hexagonal-cut-concrete-forest-left` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️hexagonal-cut-concrete-forest-left` |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/📚️examples/🎬️hexagonal-cut-concrete-forest-right` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️hexagonal-cut-concrete-forest-right` |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📚️examples/🎬️hexagonal-cut-concrete-forest-left` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️hexagonal-cut-concrete-forest-left` |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📚️examples/🎬️nakagin-capsule` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️nakagin-capsule` |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📚️examples/🎬️hexagonal-cut-concrete-forest-left` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️hexagonal-cut-concrete-forest-left` |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📚️examples/🎬️nakagin-capsule` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️nakagin-capsule` |
| `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |
| `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/📚️examples/🎬️demo` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo` |

## Split review queue

- **🗄️stdio/🎞️gif/🎬️demo**: multiple standards: 🔖️87a, 🔖️89a; default primary 🔖️87a; SPLIT? example at artifact level may need per-dialect copies
- **🗄️stdio/🎞️gif/💃️dancing**: multiple standards: 🔖️87a, 🔖️89a; default primary 🔖️87a; SPLIT? example at artifact level may need per-dialect copies
- **🗄️stdio/🏗️ifc/🎬️demo**: multiple standards: 🔖️2x3, 🔖️4; default primary 🔖️2x3; SPLIT? example at artifact level may need per-dialect copies
- **🗄️stdio/📄️pdf/🎓️bachelor-thesis**: multiple standards: 🔖️1.4, 🔖️1.7; default primary 🔖️1.4; SPLIT? example at artifact level may need per-dialect copies
- **🗄️stdio/📄️pdf/🎬️demo**: multiple standards: 🔖️1.4, 🔖️1.7; default primary 🔖️1.4; SPLIT? example at artifact level may need per-dialect copies
- **🗄️stdio/🖊️dwg/🎬️demo**: multiple standards: 🔖️ac1018, 🔖️ac1024; default primary 🔖️ac1018; SPLIT? example at artifact level may need per-dialect copies
- **🗄️stdio/🖊️dwg/🏛️architectural**: multiple standards: 🔖️ac1018, 🔖️ac1024; default primary 🔖️ac1018; SPLIT? example at artifact level may need per-dialect copies
- **🗄️stdio/🧿️semio/🌐️envelope**: SPLIT? spans subsets: ✳️any, ✳️flow; SPLIT? conflicting subset signals in assets vs code

## Plugins covered

33 plugins: ✒️writer, ➗️mathematical, 🌀️procedural, 🌊️flow, 🌍️gis, 🌿️vcs, 🎞️animate, 🎥️shooting, 🎪️demonstrator, 🎬️sequence, 🏗️fem, 🏛️architect, 🏭️process, 💠️lowpoly, 💡️reasoning, 📋️forms, 📏️layout, 📐️cad, 📕️norm, 📖️playbook, 📜️imperative, 📸️remodel, 🔋️energy, 🔱️trinity, 🕸️dag, 🖍️draw, 🖨️raster, 🗄️stdio, 🗒️note, 🧩️puzzle, 🧱️block, 🪐️space, 🪵️sourcing
