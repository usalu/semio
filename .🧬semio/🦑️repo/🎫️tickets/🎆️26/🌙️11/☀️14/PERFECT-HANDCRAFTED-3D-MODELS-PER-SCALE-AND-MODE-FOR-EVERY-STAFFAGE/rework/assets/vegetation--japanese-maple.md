# vegetation/japanese-maple — rework report

Reference table, defects and what was rebuilt: `rework/13.md`, section 1 (batch author 13, night of 2026-11-22, script `rework-13-japanese-maple.py`).

## Publication by the umbrella finisher, 2026-11-22 22:45–23:00

State found: the batch author was stopped by the limit at 01:34. The script had written the catalog root (2D of all scales, anchors, `areaM2`; catalog 01:35) and the rebuilt volumes source (`volumes/vegetation/japanese-maple.json`, 1489 / 190 / 109 / 61 body solids), but the source was never spliced: the catalog kept the old lists (487 / 420 / 278 / 278) under the new drawings and was `INVALID` with 60 `volumes-feature` findings (sweep of 22:10).

Finished: with the source merged one finding was left (`support-base-support` of the 1:500 structure list, a 40 mm disc under the 75 mm feature); `rework-finish-500-supports.py` makes it 75 mm deep below the ground plane. Published with `volumes-insert.py`.

Proof:
- `r3-validate.ts vegetation/japanese-maple --volumes` (source merged) → `VALID … schema 0, validateAsset 0, checkVolumes 0`.
- After publishing: `r3-validate.ts vegetation/japanese-maple --catalog-only --volumes` → `VALID vegetation/japanese-maple (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`; check CLI `✓ japanese-maple (1/1)`, four scales suitable (1:500 min feature 0.219 mm). Source = catalog.

Sheets read at full size: 1:50 body six views (1489 solids), 2D 1:200. Verdict: three sinuous stems from a flared collar, limbs fanning out under three tiers and a top, layered habit with air between the tiers; the 1:200 drawing shows the same tiers and agrees with the model. Rendered, not read: 1:500 body.

Remains imperfect: the cushions are smooth domes with a toothed rim and read as parasols, a real Acer palmatum layer is looser and ragged; `rework/13.md` lists what its author left open; thermal, structure, poses, clips, 1:100 and 1:500 were not looked at by the finisher; the asset never had a second-round pass or a review.
