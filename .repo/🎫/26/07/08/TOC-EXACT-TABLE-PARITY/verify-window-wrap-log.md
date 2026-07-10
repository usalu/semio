# Window Wrap Enforcement Verify

## Changes

- `semio@window@depth` tracks open window contexts (Window, Table, Figure, register lists).
- `SemioTable` / `SemioTableTwo` / `SemioTableThree` error when used outside a window.
- Registered window kinds accept `hierarchy=false` to skip numbering and list-of registration while keeping window chrome.
- Meilenstein table in `zwischenbericht.tex` wrapped in `\begin{Table}[title=Meilensteine, hierarchy=false]`.

## Build

```
bun run build:mit-bestand:zwischenbericht
```

Exit code 0.
