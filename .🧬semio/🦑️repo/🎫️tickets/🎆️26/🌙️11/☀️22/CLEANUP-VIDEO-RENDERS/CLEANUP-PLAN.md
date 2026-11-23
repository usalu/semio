# Cleanup Video Renders

## Context

Laptop disk critically low (~522 MiB free on 228 GiB). User asked to remove this week's generated video renders only — keep fundamental source/teaching code.

## Safe to delete (regenerable)

| Path | Size | Why |
|------|------|-----|
| `videos/` (gitignored `/videos/`) | ~334M | Render outputs: partial `s/`, `rendered/*.mp4`, traces, quality folders, SVG frame dumps |
| `.🧬semio/🎓️teaching/⚡️cache/staffage/` | ~51M | Staffage asset cache used by renders; rebuilds on next render |
| Ticket `🗑️generated/logs` under this week's render tickets | ~few MB | Tool logs only |

## Keep (fundamental)

- Teaching scene sources under `🎓️teaching/` (Python/Manim scene definitions)
- Scene logic under calculation modules
- Ticket markdown audits (`SUMMARY.md`, `CHANGES.md`, notes)
- Teaching python env cache (not a video render artifact)

## Note

`videos/` is listed in `.gitignore` as `/videos/` — nothing here is source of truth.
