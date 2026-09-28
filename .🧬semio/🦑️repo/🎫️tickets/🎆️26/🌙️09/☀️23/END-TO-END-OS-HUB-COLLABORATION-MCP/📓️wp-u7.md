# WP-U7 — Example Loaders: Typed Refusal Instead of Silent Empty Documents (Window-3 T3 Set)

Session 14c slice U7 (spawned 2026-09-28 23:3x, Opus 5.5). Coordinator = main. Scripts/inputs `wp-u7/`, captures
`wp-u7/generated/` (expendable), private target `wp-u7/target`, byte backups `wp-u7/w3-backup/`. Native cargo only via
`📜️fleet-mutex.sh native u7` (build-fleet-b, nice 15, `CARGO_INCREMENTAL=0`). No overlay (rule 23).

## Session 14

| # | Item | Status |
|---|---|---|
| 1 | Census of every example loader (`setActiveExample` reductions + helpers) | done (static) — table below; `wp-u7/u7-census.py`, raw `generated/census.{json,md}` |
| 2 | ONE typed refusal in the SDK, schema-first, en + de | designed — `ExampleRefusal` on the existing `FaultFrom` family, see Design |
| 3 | Window-3 T3 set `u7-example-loaders` | in progress |
| 4 | Laws: typed-refusal fixture law (Rust + TS + Ajv) + every seated example decodes non-empty | stdio sweep + draw demo law written (`u7-laws.py`), probe queued native (`generated/probe-1.txt`) |
| 5 | Stale assets + regen commands | in progress (probe-1 names the stdio ones; draw confirmed stale by the coordinator's live refusal) |

#### Log

- 23:3x start. Load 150+, native lane 12 tickets deep. Read AGENTS.md, preamble rules 1–24, window-3 plan, LB2 item 7.
- 23:5x census (static): 182 files with an example switch, 62 stdio `*_example_snapshot` loaders (one shape), ~50 non-stdio loaders.
- 00:00 probe-1 queued (native lane, stamp 20260929000014): lands the stdio seated-example sweep + draw demo law in-hold, keeps
  what compiles, runs both.
