# Findings

## Symptom

After ramping the puzzle 3d fill slider up, sliding down no longer removes objects, and sliding back up cannot add new ones.

## Root cause

`setFillCount` materializes fill objects into the document while keeping an in-memory fill plan (`fill.base` + `sequence`). Any later incidental action (`setHover`, pick, mesh sync, …) called `sync_precompute_session` → `set_scene` with that applied projection. `set_scene` treated it as a brand-new scene, ran `rebuild_queue()`, and baked the filled objects into a fresh `fill.base`. After that the slider could only plan *on top of* the already-filled scene — it could neither remove the baked objects nor replan their tail.

## Fix

In `puzzle/3d` `set_scene`: if the incoming fixture is the active fill plan's base plus applied fill objects (compared by id sets, tolerant of attraction-rederive pose drift), update non-fixture scene fields and keep the fill session instead of rebuilding.
