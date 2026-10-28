# Hidden-Line Geometry

The three bodies use one shared projection and stroke style. The box has nine visible and three hidden edges; the rectangular pyramid has five visible and three hidden edges. The smooth elliptical cylinder has no invented seam: the rear bottom contour is hidden, while the top rim, front bottom contour and two tangent generators remain solid.

The neutral case and PDF verifier were authored before the drawing update. The old artifact failed with two hidden-line groups rather than six. The final artifact passes all six, with three hidden linear segments in each polyhedral rendering and one rear arc in each cylindrical rendering.

The native PGF parser required a literal coordinate for the pyramid apex; the final definition uses the same central-base apex formula as the common projection. The raw PDF dash array is in converted TeX points; the verifier accounts for that conversion and the header scaling.

Concurrent main-thread header/table edits were observed through the read-only shared geometry reference. The independent variant pins its original four-row worked table,20mm header,shape numbers and0.6 scale locally. It does not alter those shared edits. The final directed and matrix contracts are independently validated against the current shared graph geometry.

See report.md for the final artifact and checks.
