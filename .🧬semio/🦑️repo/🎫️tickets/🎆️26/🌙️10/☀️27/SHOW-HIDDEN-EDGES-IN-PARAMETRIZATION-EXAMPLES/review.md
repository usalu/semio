# Example Hidden Edges

This isolated ticket adds the box's three hidden rear edges and the cylinder's rear lower half ellipse using the same densely dashed style as the pyramid. The shared shape macros serve both the example table and K3. Visible geometry and dimensions are unchanged. Main table-centering work and the main PDF are outside this side request.

cases.json is a language-neutral regression; verify.mjs checks source geometry; verify-native.py independently checks native PDF graphics with pypdf and renders with PDFium. preview.tex is the isolated rendering input.

Validation completed: RED source test first rejected missing box hidden paths; GREEN source test then passed all three shape cases. Isolated native Tectonic compilation exited 0. Independent pypdf verification confirmed five dashed paths: three box edges, three pyramid edges, and the cylinder lower rear half-ellipse, all with identical PDF dash pattern [2.98883, 1.99255]. PDFium rendering was visually inspected: all hidden lines are correctly occluded and consistent. No main PDF, table-centering files or main-ticket state were changed. Temporary preview/cache/log outputs were removed after all owned processes exited.
