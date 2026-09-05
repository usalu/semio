# Print Product Emoji Repair

The Print product was repaired by inspecting each of its 36 initial missing-emoji findings individually.

## Reserved TeX identities

The seven `semio-*.sty` package names and the `semio.cls` and `zukunftbau.cls` class names remain literal. LaTeX resolves those basenames through `RequirePackage` and `documentclass`, so nine exact-path fixed-filename contracts now document and enforce that reservation.

## Handpicked identities

- `demo-strip.png` became `🎞️demo-strip.png`.
- The font catalog became `🔤️font`, with distinct family directories `🅰️anta`, `🧱️kelly-slab`, `😀️noto-emoji`, and `🖥️share-tech-mono`. Each TTF received one family-specific emoji.
- Zukunftbau template siblings use distinct research (`🔬️`, `📑️`), compact (`🗜️`, `📝️`), bibliography (`📚️`), appendix (`📎️`), content (`🧩️`), and interim (`🚧️`) identities.
- Paper, report, and flyer template siblings use distinct content, bibliography, appendix, and document-role identities.
- Every live TypeScript and TeX reference was updated to the selected physical path.

## Verification

- Scoped audit: 46 files, 28 directories, 62 governed entries, zero findings in every category.
- Taxonomy validation: no problems.
- `bun nx run @semio-tech/print:test-quick`: passed.

The verification also exposed two stale pre-existing Print references: the package router still imported an obsolete flat verification-command path, and the paint loader pointed below the actual styling-token file. Both were corrected to the existing physical authorities before the passing run.
