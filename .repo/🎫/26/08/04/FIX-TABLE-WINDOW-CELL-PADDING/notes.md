## Consistency analysis

| Table kind | Example | Outer inset | Inter-column inset |
|---|---|---|---|
| `\SemioTableLong` (good) | Tabelle BB.M.a | `\semio@table@border@L` + `\semio@table@padendL` | `\semio@table@padcp` ×2 |
| `\begin{Table}+\SemioTable` (broken) | Überblick, Meilensteine, … | `@{}` — text flush to vertical rules | no `padcp` — columns abut |

Short tables only narrowed column widths (`-2\SemioTablePad`) without the long-table edge pads, so text sat on cell borders while long tables matched `\semio@tab@inset` (~5.5pt).

## Fix

- `print/tex/semio-table.sty`: `\SemioTableWindowColspec{fractions}` builds the same colspec as `\SemioTableLong` (border, padend, padcp, segmented row rules).
- Zwischenbericht appendix/main short tables switched from `@{}T{…\linewidth…}@{}` to `\SemioTableWindowColspec{…}`.
