# Section Finish Cut

BIM objects `Beat2_ObjectOrientation` stopped in `villa_section` with `KeyError: 'outline'`.

`_reference_material_parts` publishes a finish cell as mesh metadata and drops `outline`, `z0`, and `z1`. Plan poche and axon faces already read that mesh. The section poche called `slab_cut`, which only reads a prism outline. The south elevation would have done the same in `slab_sides` on the next view.

`occupied_section_rings` keeps the prism scanline and, for any other cell, uses `part_sections`. Finish poche draws those rings. Elevation still uses `slab_sides` for upright prisms and `add_loft` for mesh cells, and it no longer asks a mesh cell for an outline while clipping structural sides.

The objects lesson did not publish. The same job continued at representations `Beat3_SameObject` with the process that had already imported the old modules. Later lesson processes load this cut. Objects still needs its own 480p15 render after the current serial job.

`section-finish-cache.py` read the render slab cache and published each record the way the scene does. Every finish cell in that cache is a mesh with no prism outline: 5979 cells, and `part_section` returned a ring through the middle of each cell on both plan axes (11928 cuts, no unreadable records).

`rerender-failed-bim.ps1` waits until the 07:36 rest process (shell 63148, orchestrator 15008) is gone, then renders `bim_objects` and `bim_representations` at 480p15 against `🗑️generated/full-slab-render`. Those two processes imported the section modules before this cut.

The representations scene was still inside `building_geometry` with those old modules, including a second model state whose slab keys miss the warm cache. That scene was stopped so the rest job could continue. It then started models `Beat4_PartialModels`, which loads this cut. Exchange and consistency follow in the same job.
