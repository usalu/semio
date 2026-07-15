# Print Footer Visibility Verify

Build: `bun ./script.ts build` (all 6 templates, light+dark)

## Geometry (report.pdf log)

- modes: `includehead includefoot`
- v-part: `(T,H,B)=(28.6119pt, 787.82306pt, 28.6119pt)` — top/bottom gap symmetric
- textheight: `720.46312pt` (was `771.725pt` before fix)
- footskip: `24.83995pt`

## Visual checks

| Doc               | Page | Raster                   | Footer visible | Content stops above footer |
| ----------------- | ---- | ------------------------ | -------------- | -------------------------- |
| report            | 4    | report-p4.png            | yes (page 1)   | yes                        |
| report            | 5    | report-p5.png            | yes (page 5)   | yes                        |
| paper             | 2    | paper-p2.png             | yes (page 2)   | yes                        |
| flyer             | 1    | flyer-p1.png             | yes            | yes                        |
| zwischenbericht   | 2    | zwischenbericht-p2.png   | yes (page 2)   | yes                        |
| forschungsbericht | 5    | forschungsbericht-p5.png | yes (page 5)   | yes                        |
| kompaktbericht    | 2    | kompaktbericht-p2.png    | yes            | yes                        |

## Notes

- Flyer uses `1.2cm` margin; geometry computes negative outer gap (`T=B=-8.37677pt`) because navbar chrome exceeds margin — pre-existing flyer constraint, page still renders with visible footer.
- Cover pages (e.g. zwischenbericht p1) use empty pagestyle by design; footer check applies to body pages.
