# Slab Material Precision R34

The inferred ground-bearing plate exposed cumulative seams from9-decimal coordinate rounding in the native slab clipping path. Its992cells had up to1.20032e−9m² pairwise overlap and2.27169e−8m² summed pair overlap. Those individual errors fit the existing1e−8test criterion, but the input geometry can be retained more accurately without changing facets or source regions.

A new language-neutral clipping_precision case first failed: an authored12-decimal rectangular boundary lost2.460012638e−9m² compared with independent GEOS, exceeding the case's1e−11budget. _clip now accepts an explicit coordinate precision while keeping its9-decimal default for other analytical consumers. Only slab_surface and surface_difference request12digits and1e−12positive-area tolerance. No source tessellation, footprint, thickness or material region was altered; no external product dependency was added.

All4permanent SlabPartitionContract tests pass4.286s, including the new precision case, editable-hole hidden ownership rejection, portable boundary/disjoint holes and every current source slab. Independently R33fresh ground verification passes4source cases4.304s:994valid native pieces, area119.133123657304m², summed-area error−3.5527e−13m², largest pair overlap7.6979e−13m² and aggregate pair overlap9.1317e−13m². Existing96facet circles and the uniform source region are preserved.

The earlier long verification observation was not a native clipping bottleneck: profiling showed native stages complete quickly and a naive global GEOS fragment union was slow. That observation was cancelled honestly and replaced by bounded independent set proofs. No artificial source prepartition or unnecessary performance patch was made.

Changed files: model analysis/🐍️.py _clip precision parameter; model/🐍️.py slab clipping precision; model/tests/⚡️partitions.json/.py portable proof. Later coordinated curtain/ground source still requires its final whole-model check; these are identified numerical and slab checks.
