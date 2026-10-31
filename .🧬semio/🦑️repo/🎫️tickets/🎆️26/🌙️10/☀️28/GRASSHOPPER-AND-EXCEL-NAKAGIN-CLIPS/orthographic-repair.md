# Physical Orthographic Reference Repair

The public capsule_views interface previously rendered P/S elevations with the requested family but derived the plan opening and sheet aperture metadata from the default front-window family. The strengthened six-family regression failed on P and S: the reported centre was (0,4.034333333,1.25) instead of the actual side aperture (1.234876275,2.6,1.25). The other four families exposed omitted reference bolt parts.

The method now requests the actual family aperture, uses its normal to place the plan opening and diameter dimension on the correct wall, and includes physical attachment hardware in both reference elevations. The staged teaching capsule_axon bolt default is unchanged. All six final reference gallery views explicitly enable the four attachment heads too.

The outer plan rectangle also contradicted the actual rounded physical casing: its corners were 103.553 mm outside the R0.25 m profile. The plan now uses the shared physical-width/length rounded profile, the full-detail quarter segmentation policy and the physical back offset. Its inner rectangle remains the authoritative usable boundary, with no invented XY radius. Physical overall dimensions still measure the same bounding extents.

Portable quarter controls 1/4/8 precede the new boundary implementation. Independent SciPy convex hulls of the actual shell vertices produced 8/20/36 vertices, while the old plan always had four; all three cases failed before the change. The six-family aperture/plan-window/reference-bolt check also failed before it. After repair, the two targeted permanent methods pass in 347.231 seconds with terminal exit 0. The source audit is retained in orthographic-plan-audit.md.

These checks validate the physical reference interface; the later fragment-renderer integration, final whole-viewer suites, integrated images and native scene lifecycles retain their own final gates. No shared physical model source was changed by this correction.
