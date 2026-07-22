# Remove Checked-In Dark Tex Files

Light `.tex` is the only source of truth. Dark variants are derived at build time into a gitignored `.semio-dark/` directory.

## Changes
- Deleted tracked `*-dark.tex` under `print/template/**` and `mit-bestand/bericht/zwischenbericht/`
- `deriveDarkTexSource` writes a generated banner; preserves `% !TEX` magic comments
- `writeDerivedDarkTex` emits `.semio-dark/<name>-dark.tex` next to the light source
- Compile uses light-source `workDir` so `\input` and assets resolve
- `.gitignore`: `**/.semio-dark/` and `*-dark.tex`
- Watch ignores `.semio-dark` churn

## Verified
- `deriveDarkTexSource` unit checks pass for all 7 canonical light sources
- `buildPrintDocument(report.tex)` produced light+dark PDFs; dark tex only under `.semio-dark/`
