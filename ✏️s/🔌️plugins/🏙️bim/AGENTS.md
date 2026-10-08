---
technology: bim
emoji: 🏙️
---

# BIM

Building information modelling as a parametric, event-sourced document.

- Hierarchy: project → site → building → storey → elements (walls, curtain walls, columns, beams, slabs, roofs, openings, stairs, railings, spaces). Types (wall, slab, roof, column, beam, window, door) and materials are libraries referenced by id.
- Parametric principle: the snapshot stores authored parameters only. Everything derivable (elevations, wall heights, thickness, length, footprints, solids, quantities) is an inference of `ModelInference`, never stored.
- Laws: mutations are sparse typed diffs with concrete inverses; only `protocol::apply_diff` applies; every leaf proves the inverse sum law; references are validated in `🔺️diff`; deletes cascade only over kinds that have a create leaf and otherwise refuse with `mutation.in-use`.
- One artifact (`🏢️model`, `s.bim.model@1/*`); no bim-specific module, no engine, no state holder.
