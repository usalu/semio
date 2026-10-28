# Same Table German Dark Verification

The official dark driver was derived with the repository's `deriveDarkPrintTexSource` implementation, then compiled with native Tectonic 0.16.9. The compile exited successfully and produced a 135111-byte PDF with three pages. Native expansion of the shared blank-row helper succeeded.

Independent pypdf and pdfplumber checks confirm German document metadata, the intended title and author, three pages, all text inside page bounds, and zero overfull or underfull diagnostics. The initial bare Python environment lacked PDF dependencies; rerunning through the bundled Python runtime succeeded without installing dependencies.

All three pages were rendered with PDFium at scale 2 and inspected. Page 1 shows the windowed example table with an unnumbered 16 mm form header and eight 6 mm parameter rows: four populated and four empty. The shared chrome, grid, and parameter-column alignment match the two task tables on page 2. The K3 diagram and every example/counterexample paragraph remain readable on page 1. Page 2 retains both capsule matrices, and page 3 retains the complete directed graph, twenty highlighted transitions, and writing ledger. No clipping, overlap, lost content, or pagination defect was found.

The native PDF and all renders are temporary ticket outputs under `🗑️generated`, to be removed after final delivery. Source files remained frozen throughout native compilation and review.

## Source SHA-256

- `🧾️template/📝️assignment/parametrization-geometry.tex`: `2efc46545e80874f2d77774a8ce2acc887885b00e930b22c18d14a1aab35fa37`
- `🖋️latex/semio-assignment.sty`: `21e1075cd0a64d36d28780e7d8b68e881ef3945b6c2f011bfc202a14890058dc`
- `🧾️template/📝️assignment/parametrization-de.tex`: `13994c3d71cb2951a148d7c54e9a39449bf3cba54cdbe1648432892e0faef235`
- `🧾️template/📝️assignment/parametrization-en.tex`: `857ffbedf79876ce3250cf926c3a8d3059e79d4112d3da7754e50349e8340ae7`
- `🧾️template/📝️assignment/parametrization-de.content.tex`: `a63dad4f88f1c0844b4b61c9cb463100a3b404d53c62c0aa57b94ed5d5a56422`
- `🧾️template/📝️assignment/parametrization-en.content.tex`: `c570fe2d8300b8cc42515233e4c3de3bca8e2336a54688760587f96a5c782448`
