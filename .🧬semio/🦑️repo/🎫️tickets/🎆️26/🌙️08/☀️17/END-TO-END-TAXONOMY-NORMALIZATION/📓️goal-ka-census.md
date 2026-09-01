# Census — `🎠️kernel` and `🖼️assets`, re-measured

Baseline: `bb06c41f73f0122fbed315b7487428b976f99921` (= current HEAD, unchanged throughout).

## `🧰️framework/🔨️modules/🎠️kernel` — 50 moves, 1 unresolved (unchanged from prior slice)

```
bun ./📜️script.ts clean taxonomy plan --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION \
  --scope "🧰️framework/🔨️modules/🎠️kernel" --baseline "$B" --plan "$T/🗑️temp/🔣️ka-kernel.json" --workers 4
[clean taxonomy plan] moves=50 roots=0 relocations=0 symlinks=0 removals=0 edits=157 regenerations=2 unresolved=1
```

Row (identical to the earlier census slice's finding):

```json
{
  "code": "reference-syntax-unsupported",
  "message": "typescript unsupported-path-syntax:222:6@14159 ... \"🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts\" ...",
  "path": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/📜️script.ts"
}
```

## `🧰️framework/🔨️modules/🖼️assets` — 1 089 moves, 6 unresolved (composition shifted from the 5 on record)

```
bun ./📜️script.ts clean taxonomy plan --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION \
  --scope "🧰️framework/🔨️modules/🖼️assets" --baseline "$B" --plan "$T/🗑️temp/🔣️ka-assets.json" --workers 4
[clean taxonomy plan] moves=1089 roots=0 relocations=0 symlinks=0 removals=1 edits=54 regenerations=1 unresolved=6
```

Reproduced 3× (`ka-assets.json`, `ka-assets2.json`, `ka-assets3.json`, all deleted after use), identical every time:

| # | code | path | note |
|---|---|---|---|
| 0 | `generator-preview-invalid` | `.vscode/launch.json` (root 0 of `plugin-registry`'s `outputRoots`) | **new**, not on record before — see report |
| 1 | `reference-syntax-unsupported` | `.../☀️23/END-TO-END-TESTING-REFACTOR/las-1-0-fixture-derive/extract_positions.py` | on record: correctly blocked by an embedded package, do not retry |
| 2–5 | `reference-syntax-unsupported` (×4) | `.../📺️renderer/.../🧊️wgpu/🟦️typescript/🟨️frame-worker.js` | on record: generated wgpu bundle, see report |

All 6 rows byte-identical across all 3 plan runs (same `structuredLocation` offsets, same messages).
