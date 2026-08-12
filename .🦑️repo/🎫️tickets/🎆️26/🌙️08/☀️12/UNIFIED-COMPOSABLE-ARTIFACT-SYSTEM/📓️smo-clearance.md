# SMO Clearance Snapshot (computed 2026-08-12 at ticket-open, W0)

Source: `../SEMANTIC-MUTATIONS-OVERHAUL/📓️wave1-reports/` (5 files: cad, draw, fem-3d, gismap, gisterrain) + `📓️wave2-reports/` (25 files, listed in that ticket's folder). A plugin is CLEAR only when every one of its artifact dirs has a report. **Re-check before every W4 dispatch — this goes stale fast, SMO is running concurrently.**

## CLEAR (18) — safe to schedule fan-out once W2/W3 gate is met
cad, process, procedural (2d+3d), flow, lowpoly, gis (terrain+map), reasoning, layout, draw, shooting, animate, writer, forms, architect, sequence, mathematical, vcs, demonstrator

## NOT CLEAR (15) — do not touch `🧬️mutations/**`, wait for their reports
- **remodel** — 0/1 artifacts reported
- **fem** — 1/2 (3d done, 2d missing)
- **norm** — 9/15 (din16798, din4108, en1990-1994, iso16757, vdi3805 done; missing din18599, en1995-1999)
- **dag** — 0/1
- **trinity** — 0/2 (jack, rewrite)
- **puzzle** — 0/3 (2d, 3d, 5d)
- **block** — 0/3 (2d, 3d, 5d)
- **sourcing** — 0/1
- **raster** — 0/1
- **note** — 0/1 (explicitly noted unstable by SMO wave-0)
- **playbook** — 0/1
- **energy** — 0/1
- **imperative** — 0/1
- **space** — 0/1
- **stdio** — 0/37 (expected — huge, will likely clear last or never before we need it; claim it explicitly if we must start W2 before SMO reaches it)

## Batch reassignment given this snapshot

Original plan batches assumed different clearance; reordering W4 to dispatch CLEAR plugins first regardless of original batch letter. Recompute before each dispatch round.
