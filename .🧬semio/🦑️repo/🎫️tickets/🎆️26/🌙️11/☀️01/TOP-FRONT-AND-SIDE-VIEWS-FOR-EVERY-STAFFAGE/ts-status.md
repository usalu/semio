# TS status (test worker)

State: READY. The guard exists and runs; it is red for every asset that still lacks front or plan geometry (expected until the content workers finish).

Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = this ticket folder.

## What TS added

| Piece | Path |
|---|---|
| Vectors (coordinate contract, tolerance, hand-computed cases) | `S/🧫️fixtures/🧊️three-views/🔣️.json` |
| Gherkin (en) | `S/🧪️tests/🧊️three-views/🥒️.feature` |
| Vitest case | `S/🧪️tests/🧊️three-views/🟦️.ts` |
| Sweep and consistency rules (shared by test and ticket scripts) | `S/🧪️tests/🧊️three-views/🧰️.ts` |
| shapely oracle (run through `uv`) | `S/🧪️tests/🧊️three-views/🐍️.py` |
| Coverage table | `T/🧰️TS/coverage.py` |
| Consistency reporter (bun) | `T/🧰️TS/consistency.ts` |
| Fallback vitest config | `T/🧰️TS/vitest.three-views.config.ts` |

## Commands (from `C:\git\semio`)

Whole staffage suite (includes the new case; slow):

```
repo run @teaching/staffage:test --lang=en
```

Only the three-views case. The ticket-local config reuses the archive's vitest config and restricts `include`. This is also the fallback when `repo run @teaching/staffage:test` is broken (about 15 s on a quiet machine):

```
cd 🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript
../../../../../node_modules/.bin/vitest run --config "../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️TS/vitest.three-views.config.ts"
```

One asset only (vitest name filter, the test names start with the asset id):

```
... vitest run --config "<the config above>" -t "dairy-cow"
```

Coverage table (reads the catalog JSON only, instant; `E/F/P` are shape counts, `!` marks a representation missing a view, `unavailable` stays unavailable):

```
python .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️TS/coverage.py [--asset ID ...] [--missing-only] [--json]
```

Same table with the sweep and consistency status per asset and scale (`ok` or the failing check names; runs `consistency.ts` through `repo tools exec bun`, about a minute, writes `T/🗑️generated/TS/consistency.json` and prints every finding message):

```
python .../🧰️TS/coverage.py --consistency --asset <id> [--asset <id> ...]
```

Quickest per-asset check from a content worker's point of view (prints each finding with the scale, pose or clip instant, and view):

```
repo tools exec bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️TS/consistency.ts "" <asset-id>
```

## The three checks per asset (test names)

1. `<id> draws non-empty finite geometry in every view for every pose and clip instant at every authored scale`: per authored scale the three views must have authored non-ground shapes (`[front-missing]`, `[plan-missing]`, `[elevation-missing]`, reported once per scale), and every pose plus six instants of every clip (fractions 0, 0.2, 0.4, 0.6, 0.8, 0.999 of the duration) must evaluate each view to non-empty geometry with finite bounds (`[front-empty]`, `[plan-bounds]`, `[evaluate]` for thrown errors).
2. `<id> keeps its three views one object at the default pose at every authored scale`, tolerance max(0.02 m, 3 % of the largest dimension): `[elevation-x-plan-x]`, `[elevation-y-front-y]`, `[front-x-plan-y]` (ranges equal), `[elevation-width]` (widthM), `[elevation-height]`, `[front-width]` (depthM), `[front-height]`, `[plan-length]` (widthM), `[plan-width]` (depthM). Ground shapes are excluded. A view without geometry is skipped here (check 1 reports it).
3. Reference tests: the vectors (hand-computed), the consistency rules (including negative cases), the shapely oracle, and the dimension reading on the complete references.

## Verified reading of the dimensions (brief is right)

`widthM` is the elevation x extent (forward), `depthM` is the lateral extent (front x, plan y), `heightM` the height. Confirmed on `split-ac-indoor-unit` (widthM 0.21 projection, depthM 0.85 lateral length, heightM 0.29) and checked at every scale of both complete references by the test `reads widthM as ...`.

## One thing content workers must know: front y is relative to the rest root, not to the ground

Front shapes follow their bone by its y travel from the rest pose, and the rest pose is solved with the rig root at the origin (`restWorld` in `S/🔨️modules/🧮️evaluation/🟦️.ts`). For rigs whose root (the pelvis) is lifted to the hip height by the pose (all humans and animals: the sample figure's pelvis is 0.96 m above the rest root when standing), every front shape bound to a bone is moved up by that lift. A front shape of a standing figure must therefore be authored with the pelvis at y = 0 (feet at about y = -0.96), exactly like the elevation bone chain, not with the feet at y = 0; the evaluated default pose then has the feet at y = 0. The consistency check (`elevation y = front y`) is what tells you: if the evaluated front is one hip height too high, the authoring used ground coordinates. For rigs whose root sits at y = 0 in the pose (components, furniture, vehicles on the ground) authored front y equals ground-based y. Plan x/y are unaffected by the lift (plan follows only the bone's x travel; a planted-root clip advances the whole figure in x, like the elevation).

Hand check: the hip-height lift of the sample figure is 0.45 (thigh) + 0.43 (shank) + 0.08 (ankle) = 0.96 m; the vectors file uses it.
