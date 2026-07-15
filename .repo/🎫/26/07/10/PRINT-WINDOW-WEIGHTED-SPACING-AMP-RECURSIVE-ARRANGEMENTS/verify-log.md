# Print Window Weighted Spacing Verify

Cover raster: cover-spacing.png
Expected gap: 4pt (~16.0px at scale 3)

## Vertical gaps

| Gap | px  | pt   | ok  |
| --- | --- | ---- | --- |
| 1   | 14  | 3.5  | yes |
| 2   | 21  | 5.25 | yes |
| 3   | 33  | 8.25 | no  |
| 4   | 24  | 6    | no  |
| 5   | 70  | 17.5 | no  |
| 6   | 15  | 3.75 | yes |
| 7   | 12  | 3    | yes |
| 8   | 7   | 1.75 | no  |
| 9   | 3   | 0.75 | no  |
| 10  | 7   | 1.75 | no  |
| 11  | 13  | 3.25 | yes |
| 12  | 14  | 3.5  | yes |

## Horizontal gaps

| Gap | px  | pt   | ok  |
| --- | --- | ---- | --- |
| 1   | 6   | 1.5  | no  |
| 2   | 2   | 0.5  | no  |
| 3   | 3   | 0.75 | no  |
| 4   | 5   | 1.25 | no  |
| 5   | 2   | 0.5  | no  |
| 6   | 3   | 0.75 | no  |
| 7   | 2   | 0.5  | no  |
| 8   | 2   | 0.5  | no  |
| 9   | 2   | 0.5  | no  |
| 10  | 8   | 2    | no  |
| 11  | 2   | 0.5  | no  |
| 12  | 8   | 2    | no  |
