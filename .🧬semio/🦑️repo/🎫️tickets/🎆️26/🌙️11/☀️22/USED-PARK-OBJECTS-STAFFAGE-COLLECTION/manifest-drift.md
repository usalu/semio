# Manifest Drift — Lost Rows in the Shared Staffage Manifest

2026-11-22 ~16:55: 14 of 20 published used-park-objects assets had their catalog file `S/🗂️catalog/<category>/<slug>/🔣️.json` but no manifest row and no membership in `used-park-objects`. All five wave publishers (`used-park-objects`, `reclaimed-components`, `used-hvac-components`, `used-household-objects`, `used-instruments`) take the same `mkdir` lock `RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/publish.lock`, so the loss does not come from unlocked publishers of these waves.

Likely cause: the lock is a local folder; the replica daemon copies the worktree between machines with about one second of lag and resolves concurrent writes last-writer-wins. A publisher on another machine can take the lock before the folder arrives there, or the daemon can bring an older copy of the manifest back over a newer one. Either way rows written in that window are lost.

At the same check, 60 catalog asset files across the archive had no manifest row (plants and humans of other sessions' waves, and household and instrument assets). Those are other tickets' assets; this ticket does not republish them.

Remedy in this ticket: `reconcile.py` republishes every row-01–29 asset whose catalog file exists but whose row or membership is missing (idempotent, through `publish.ts` and its lock). It ran once (14 rows restored) and runs again after the last author finishes and before the ticket closes.

Product proposal: one manifest writer that derives rows from the asset files (the asset file is already the single source of `powerW`), so a lost row cannot occur.
