# 📝️ sonnet-humans: staffage multiview, human assets (28)

Scripts: `T/🛠️scripts/humans/`. Generated output: `T/🗑️generated/humans/`, with the original files in `bk/`. Convention used throughout: D6 (left-handed frame, left = +z, elevation seen from the right).

## Method

1. **Audit** (`audit.ts`, `summary.ts`, `bonecov.ts`, `diag.ts`):
   - per band and per view: shape counts, bone binding, projected extents against `dimensions`;
   - pose and clip sweep: elevation y against front y, elevation x against plan x, front x against plan y;
   - handedness by the sign of the lateral centre of every `-l`/`-r` shape;
   - bone coverage: limb bones drawn in some views but not in all.
2. **Validator** (`validate.ts`, `validateAsset`): run over all 28 assets. The target is zero failures of the contract codes.
3. **Fix tools** (all reproducible, all write through `lib.ts`):
   - `lateral.ts`: iterative inward or outward shift of the limb chain or the extreme shape until the lateral extent meets `depthM`.
   - `forward.ts`: match the plan forward edge to the elevation edge.
   - `fillbones.ts`: copy missing limb shapes from the nearest scale into a band that drew the limb in another view only. The z order is mapped relative to the torso; thickening to the 0.2 mm feature.
   - `symbols.ts`, `cause.ts`, `fix1.ts`, `fix2.ts`: one-off handcrafted edits.

## Decisions

- `dimensions` were changed only with physical evidence: umbrella `depthM` 1.0 to 1.05 (the canopy is a disc, `widthM` is 1.05). Everything else was fixed in the drawing.
- Symbols (`true-position`, paper mm):
  - Travelling markers (jogger, running-child, walking-commuter, wheelchair-user): plan = side (dot plus arrow or tick); front = dot with a ring (arrow towards the viewer).
  - Seated markers (office-worker, reader): a dot in every view.
  - Every `dimensionalNote` states that it is a conventional symbol, not to scale.
- `unavailable` reasons carry `cause: below-printable-feature`. D17 asks for new bands; see the status table.
- Occlusion at coarse bands: far-side (left) limbs are drawn below the torso in elevation (z mapped relative to the torso); legs hidden under the trunk in plan are drawn below it, so they appear when a stride moves them out.

## Per-asset status (updated at the end of the run)

See the table at the bottom of this file.
