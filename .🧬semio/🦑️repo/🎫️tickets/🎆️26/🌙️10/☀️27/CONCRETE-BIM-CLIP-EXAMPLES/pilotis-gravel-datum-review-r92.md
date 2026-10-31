# Pilotis Gravel Datum Review

Read-only snapshot 24e6fd48355bb2bcd2fab4ba65b8a89047b8b517fcde6fa7f62ca1b26ae502c2. Reinspected retained Annex6 image `🗑️generated/register-r9-section23.png` visually. Both transverse views label external terrain −0.15m and coal cover −0.13m. These are actual local source controls; no grading surface or estate-wide uniformity follows.

Current1652 gravel cells all use inferred −0.04..0.00m. Their source note already labels perimeter/grading/depth inferred, but the top datum is inconsistent with extending the shown external terrain at −0.15m. Do not claim0.00m as measured original gravel. No gravel part AABB intersects the coal cover XY2.56267..3.02861/Y15.648639..16.269048, so this inspection does not establish burial of the actual cover. Atlas white patches alone are not holes: occupied ground slabs and below-cut structure require actual Z-band checks.

A uniform move to −0.15m can only be an explicitly inferred extension of the source section controls. Preserve cover−0.13m. If authoring that extension, regenerate exclusions against actual column/wall/slab material at the new band; do not rigidly reuse0.00-band exclusions or invent slopes. Typed controls and bounded uncertainty are in pilotis-gravel-datum-review-r92.json.
