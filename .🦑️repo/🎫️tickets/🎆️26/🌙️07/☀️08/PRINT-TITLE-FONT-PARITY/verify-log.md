# Print Title Font Parity

## Change

Title pages and `SemioSerif` now use Anta (`SemioSans`), matching body text.

- `semio-fonts.sty`: `\let\SemioSerif\SemioSans`, removed Kelly Slab font family
- `semio-components.sty`: `\maketitle` / `\makecoverpages` use `\SemioSans`
- `flyer.content.tex`: hero title uses `\SemioSans`
- `script.ts`: removed unused Kelly Slab from `PRINT_FONTS`

## Verification

- `paper.pdf` / `report.pdf` build without KellySlab in log
- Title page raster captures in ticket folder
