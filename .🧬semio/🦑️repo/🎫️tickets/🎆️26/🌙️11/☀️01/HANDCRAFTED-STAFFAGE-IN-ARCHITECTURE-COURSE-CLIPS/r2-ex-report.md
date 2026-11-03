# r2-ex: full cast export proof

Result: all 97 casts in `V/🧍️staffage/🔣️.json` export (97 ok, 0 failed, 15439 frames, no manifest or asset edits needed).

## Method
- Script `T/🧰️ex/run.py` (batches of 10 ids, sequential, `repo tools exec bun -- <exporter> --manifest --ids --out T/🗑️generated/ex`), per-cast wall time from the exporter's `[cast i/n]` stderr marks (start of a cast to start of the next), batch peak RSS from `[peak] N MB`. Raw numbers: `T/🧰️ex/results.json`; digest `T/🧰️ex/summary.py`.
- `--lang` no longer exists: the exporter USAGE is `--manifest <file> [--ids a,b,c] --out <dir> [--catalog <dir>] [--parent <pid>]`, so no language was passed. The contract text in `T/contract.md` and `ensure_casts` (`--lang en`, per r2 open items) are stale if lg removed it from the exporter; the exporter header comment at `🟦️.ts:2` says casts are language-independent.
- Batch peak RSS: 10 batches, 219 to 767 MB (max batch 20-29: 767 MB; others <= 468 MB). Batch wall 27 to 72 s (97 casts total about 215 s of cast time).

## Timing
- No cast over 60 s in the batched run. Slowest: `demand-primary-energy-tree` 39.4 s, `cooling-internal-gains-laptop` 17.3 s, `physics-fundamentals-pot` 15.5 s, `ventilation-volume-student` 14.7 s, `ventilation-volume-cook` 8.3 s, `cooling-summer-strategy-wheel` 6.2 s.
- Caveat: a first single-cast cold run under a loaded machine (other agents rendering) took 67 s (`cooling-internal-gains-sleeper`) and 236 s (`cooling-internal-gains-seated-worker`) per invocation including `repo tools exec` startup and contention; the same casts took seconds inside a batch. Per-invocation overhead and machine load dominate, not the cast.

## 600-frame cap (22 casts) and fractional fps (18 of them)
Fractional fps (playback must use `rate`, per contract Performance section): cooling-heating-comparison-cooler 0.998, -laptop 0.998, -laptop-summer 0.998, -toaster-summer 1.997, cooling-summer-strategy-heater 9.983, heating-internal-gains-pendant-lamp-warm-up 1.997, heating-internal-gains-server-rack 1.997, physics-fundamentals-kettle 2.995, -pot 1.580, -shower 1.997, -vacuum 0.399, shared-boiler-heating-cycle 0.666, shared-tank-charge-up 0.666, shared-toaster-winter 1.997, shared-watt-heater 0.666, shared-watt-laptop 0.998, shared-watt-toaster 1.997, ventilation-mechanical-pendulum-fan 1.426.

At the cap with integer fps (no thinning artefact): cooling-summer-strategy-wheel (2), heating-calculation-radiator (15), heating-calculation-radiator-mirrored (15), shared-radiator-thermostat-front (15).

Clips using these must pass `rate` (clip seconds / scene seconds) or show only the first seconds in coarse steps.

## Open
- Nothing failing. Possible follow-up for owners of the stale `--lang` text (contract, `ensure_casts`).
