# Grasshopper and Excel Previews (interim, 2026-10-30 22:26)

## Caption fixes
Static check `render/caption_check.py` wraps every NARRATION clause of the nine scenes with `wrap_body_lines` (the wrapping `caption_bar` uses, 11.5 wide, max 2 lines). It found 17 overflowing strings (about 150 characters or more is the limit for two lines). All were shortened in en and de, meaning kept; a second run reports `overflows: 0`.

| Scene | Clause |
| --- | --- |
| Grasshopper capsule | sliders, lift (de), frontplane, listitem |
| Grasshopper casting | text_number |
| Excel anatomy | hook |
| Excel references | hook |
| Excel energy | dims, areas |

Other crashes: none caused by our scenes. The `tower`, `trees`, `anatomy` (Grasshopper) scenes had no overflow.

## Reruns
`render/rerun_queue.py` (running in the background) reruns every failed target once the batch has launched all 18 logs and fewer than 6 previews are alive; up to 3 tries per target. Log: `🗑️generated/rerun-queue.log`.
Failed so far: Excel_anatomy-en, Excel_references-en, Excel_energy-en, Grasshopper_casting-en, Grasshopper_capsule-en (snapshot older than the fix).
Done so far: Grasshopper_anatomy-en, Grasshopper_tower-en.

## Status
The machine runs at about 100 percent CPU from several sessions; one preview takes 1 to 2 hours. Not all 18 files exist yet. Freshness criterion (mp4 newer than the scene file) is checked by the queue; renders started before the narration edits (Excel_program-en, Grasshopper_trees-en, tower-de) may need a rerun if codex edited their scenes afterwards.
