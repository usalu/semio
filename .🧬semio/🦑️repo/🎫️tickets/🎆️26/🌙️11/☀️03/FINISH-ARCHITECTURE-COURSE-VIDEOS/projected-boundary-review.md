# Projected Boundary Source Review

Root read projected_material_regions and _regions before actual fixture capture completed. Projection already converts its matrix and spatial coordinates to Fraction and computes the two-dimensional coordinates with exact sums; this is not an observed floating-point projection implementation. material_skin lifts its result back to floats at the existing output boundary. No source or model was changed.

Inference for diagnosis: an open directed walk in _trace suggests an unbalanced classified boundary graph or missing atom/node, rather than merely a different successor choice in an otherwise balanced finite directed graph. Capture and reduction must establish the actual cause; this reasoning is not a repair or a proven explanation for the failing primitive. Inspecting incident directed degrees and exact edge classifications may help reduce the captured case.
