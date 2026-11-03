# 📝️ haiku-consumer-audit: Consumer View Propagation (relayed by the coordinator)

Static audit only (read-only); no tests were run. Snapshot of 2026-11-02.

Path shorthand:
- `REACT` = `🧍️staffage/🎯️targets/⚛️react/🔨️modules`
- `EXP` = `🧍️staffage/🔨️modules/📤️export`
- `COURSE` = `⚡️energy/🏗️authoring/🎨️visuals`

All 94 casts carry `viewpoint` now: 6 are front or plan, the rest elevation. The course uses the library only through the Python wrapper, and there is no `<staffage-figure>` outside 🏛️architecture.

## A. React and site (owner: sonnet-viewer)

| Consumer | Location | Defect |
|---|---|---|
| Pipeline | `REACT/🧮️pipeline:100` | `request.viewpoint ?? "elevation"` is a silent default |
| Row previews (browse, overview, playground) | `REACT/🖼️previews:117-136` | There is no viewpoint prop; previews always draw the elevation |
| Placed figures | `REACT/📍️placed:52-84` | There is no viewpoint, and the `try/catch` returns `null` silently |
| Compare | `REACT/⚖️compare:65,145`, `ComparePane` in `🗺️routing:267` | Front and plan cannot be reached, and the URL tokens carry no viewpoint |
| Viewer | `REACT/🎛️viewer:102,127` | The scene is only replaced when `metadata.view` changes, so a stale scene survives a viewpoint-only switch (low severity) |
| Figure | `REACT/🧍️figure` | A missing prop silently means elevation, and there is no `data-viewpoint` |
| Custom element | `REACT/🧩️element:149,165,135` | An invalid viewpoint falls back to elevation silently; an invalid view renders nothing; no tests |
| Routing | `🗺️routing:198,230` | An unknown `vp` falls back silently; compare and playground links carry no `vp` |
| Detail → compare link | `REACT/🔬️detail:75` | The viewpoint is dropped |
| Export panel snippet | `📤️export-panel:86,91` | The snippet has no viewpoint, and the lede shows a raw id |
| Download filenames | `REACT/📤️downloads:102-105` | The viewpoint is not in the name, so the outputs of different views collide; the test at `🧪️tests/📤️downloads:97,43` pins this behaviour |
| Playground export | `🧪️playground/📤️export:31`, `🧱️compose:90`, `💾️persistence:71` | There is no viewpoint, so the export is always the elevation |

## B. Exporters (owner: sonnet-exports)

| Consumer | Location | Defect |
|---|---|---|
| CLI viewpoint | `EXP/🎬️cli:77` | Defaults to `elevation` |
| CLI output stem | `EXP/🎬️cli:142` | The stem has no viewpoint, so a front-view run overwrites the elevation outputs |
| CLI DXF/DWG | `EXP/🎬️cli:153,158` | No analysis or diagrams are passed, so the output differs from the browser export, and the DXF has no viewpoint metadata |
| Preview CLI | `🖨️preview:168-170` (default), `:190-193` | The sheet name has no viewpoint (it collides), and preview thermal ignores `referenceEnvironment` |
| Cast `viewpointOf` | `🎭️cast:37` | The default is dead code (now removed), but README line 120 is stale |
| Cast `heightM` | `🎭️cast:199` | For plan this is the paper vertical extent, i.e. the depth; the test at `🎭️cast-export:136-137` and Python `staffage_height` both rely on it |

## C. Course (owner: sonnet-course)

| Item | Detail |
|---|---|
| Index validation | `🐍️.py:467-474` does not compare the index viewpoint with the manifest row |
| Guard | There is no viewpoint rule |
| Front radiator casts beside elevation casts | physics fundamentals 1759/1914, heating fundamentals 123, demand practice 154, recap 205, certificate 263, primary-energy 144, heating calculation 1088, conduction 1033 |
| Optimization clips | Extrema 439-440 has a front cooler with a fixed unit; experiments 904-905 has a mixed-view icon group |
| Excel program 368 | The plan cast is rotated −π/2 |
| BIM information 212-213 | The plan car is stretched non-uniformly, which breaks its real dimensions |

## D. Docs and contracts

| Item | Owner |
|---|---|
| README line 27: the catalog layout is wrong (it should be `<category>/<slug>/🔣️.json`) | haiku-docs |
| README line 120: the cast default | haiku-docs |
| README line 148: `staffage_height` | haiku-docs |
| `🖨️representation:65` and `🗂️catalog:387-388` tolerate missing views via `geometry.front ?? []` | contracts |

## E. Missing tests

- No React test covers viewpoint.
- Filename uniqueness across views is not tested in the CLI, the preview or React.
- The preview CLI is untested.
- Viewpoint handling in compare, playground and the custom element, `vp` routing, and the export-panel snippet are untested.
- The course guard has no viewpoint test.
