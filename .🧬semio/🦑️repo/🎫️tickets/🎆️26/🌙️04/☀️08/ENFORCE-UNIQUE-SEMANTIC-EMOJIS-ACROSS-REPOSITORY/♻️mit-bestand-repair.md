# Mit Bestand Naming Repair

## Scope

- Handpicked unique, semantic emoji prefixes for every governed entry in `♻️mit-bestand`.
- Repaired the demonstrator application, presentation sources and public assets, and Zwischenbericht sources and assets.
- Updated exact source references after each rename.
- Kept reserved names literal and limited governed entries to one emoji prefix.

## Verification

- Naming audit: 221 files, 40 directories, 250 governed entries, zero findings in every category.
- Demonstrator quick tests: 2 files and 5 tests passed.
- Zwischenbericht build: `@semio-tech/mit-bestand-bericht:build` completed successfully and produced `dist/zwischenbericht.pdf`.
- All 67 Zwischenbericht project-image paths resolve.
- Presentation project has no `test-quick` target; its build remains independently blocked in the shared framework TypeScript package.

## Compiler Boundary

TeX tooling cannot safely parse the canonical emoji-leading source and font filenames. The report build therefore creates a clean ASCII-only compiler staging tree under `dist/source`, and the print font catalog provisions ASCII font aliases in the ignored repository cache. This staging is derived output only; it does not rename or select canonical paths.
