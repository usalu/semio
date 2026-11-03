# 📋️ Fleet Brief: Staffage Multiview (Top, Front, Side)

Ticket folder (T): `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE`.
Library (L): `🎓️teaching/🏛️architecture/🧍️staffage`. Read `L/README.md` first, and `AGENTS.md` at the repo root.
Course consumer: `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals` (cast manifest `🧍️staffage/🔣️.json`, Python API, guard `🧪️tests/🐍️staffage_only.py`).

## Goal

Every asset has correct `plan` (top), `front` and `elevation` (side) views for:
- every pose;
- every clip over its whole duration;
- transitions;
- every scale band of its representations (presets 1:500, 1:200, 1:100, 1:50, plus any declared band) and every coarse `Alternative`;
- all seven views/modes: dark line, dark solid, light line, light solid, thermal, structural forces, structural moments;
- every consumer: viewer, site, React component, casts, SVG, PDF, DXF, DWG, frames and MP4/WebM.

One evaluated 3D state per instant is projected per view. A missing combination is an explicit, named validation failure. It is never a silent substitution, such as front for plan, another scale, a generic silhouette or a blank.

## Existing baseline (verify; do not assume)

- `Viewpoint` = `elevation | front | plan` already exists in `L/🧬️schema/🔣️.json` and its twin `🟦️.ts`.
- `Representation.geometry` requires all three views. **`Alternative.geometry` only requires `elevation`**, which is a known gap.
- Bones carry a lateral `z`. Offsets carry `abductionDeg` and `twistDeg`. The chain is solved in 3D. Shapes may be `revolved` or `extruded`. There are some "Plan geometry only" and "Front geometry only" stretch rules. `ViewSection` exists for front and plan.
- 149 assets: humans 28, animals 14, vegetation 16, objects 15, furniture 12, machinery 16, vehicles 9, components 15, appliances 24.

## Conventions (coordinator decision; contracts worker documents them exactly from code)

- **Side view.** The side view is the existing `elevation`, the rig plane, and it stays the canonical side.
  - World x is forward, y is up, z is the asset's left (+), as the data authors it.
  - **CORRECTION (D6):** this frame is LEFT-handed. Forward × up is the asset's right.
  - An elevation with forward to the right on paper is therefore seen from the asset's RIGHT side (the viewer stands at −z). Its near side is the right side, and the left limbs are behind.
  - contracts confirms this from code and documents it in `T/📐️conventions.md`. Asymmetry checks use this.
  - The names stay as they are: `elevation` is the side view; `view` means the 7 render modes and `viewpoint` the 3 projections. German: Seitenansicht, Vorderansicht, Draufsicht (Grundriss only for a cut plan section).
  - Front and plan bases are taken from the existing depiction code. Do not invent new ones. If the code is inconsistent, contracts reports it to the coordinator before changing it.
- **Existing views stay.** Existing additional views and sections are preserved. No new view names are added unless the coordinator agrees.
- **What a view switch may change.** Switching view changes only the camera basis. It never changes the pose, time, root motion, seed, environment, scale or attachments.
- **What scale may change.** Scale maps metres to paper mm only. Real dimensions are checked on paper as `mm = m * 1000 / denominator`.

## Ownership (one writer per shared file; others propose via the coordinator)

| Area | Owner |
|---|---|
| `L/🧬️schema/*`, `L/🔨️modules/🔗️contracts`, depiction core (`L/🔨️modules/🎨️depiction/🟦️.ts`, `📐️geometry`, `🧊️depth`, `🧩️shapes`, `✂️section`), catalog validation (`L/🔨️modules/🗂️catalog`) | sonnet-contracts |
| `L/🔨️modules/🦴️rig`, `🎞️animation`, `🧮️evaluation` | sonnet-rigs |
| `🎨️depiction/🌡️thermal-view`, `💨️transport-view`, `L/🔨️modules/🌡️thermal`, `💨️transport` | sonnet-thermal |
| `🎨️depiction/🏗️structural-view`, `L/🔨️modules/🏗️structure` | sonnet-structural |
| `L/🎯️targets/⚛️react`, `L/🌐️site` | sonnet-viewer |
| `L/🔨️modules/📤️export`, `L/🔨️modules/🖨️representation` | sonnet-exports |
| course visuals and cast manifest | sonnet-course |
| `L/🧪️tests` new coverage, regression and proof sheets | sonnet-validation |
| `L/🗂️catalog/<category>/<slug>/🔣️.json` | the category worker named on the board |
| `L/🗂️catalog/🔣️.json` manifest rows | haiku-registry, only on coordinator instruction |
| `T/*.md` docs and handoff | haiku-docs |

## Rules

- Follow `AGENTS.md`. Never run modifying git commands: commit, stash, checkout, reset, restore or clean. Never edit `AGENTS.md`.
- Other people edit the React viewer right now (uncommitted changes). Reread before every edit. Preserve their work and merge.
- Write JSON with Write or Edit, never with Bash heredocs; heredocs on Windows strip backslashes. `python -` on stdin hangs, so run a script file. Prefer bun/TS scripts.
- Put temporary scripts in `T/🛠️scripts/<your-agent>/` and generated output in `T/🗑️generated/<your-agent>/`. Put your findings in `T/📝️<your-agent>.md` and keep it updated; that file is your report.
- Handcraft, never derive. Rotating, mirroring or flattening the elevation is not a valid plan or front.
  - Author plan and front per scale band: contour simplification, grouping, line hierarchy, hatching, gaps and thin elements.
  - Keep real dimensions: widthM, depthM and heightM must match the projected extents in every view.
- Preserve asymmetry: held hand, offset parts and branches.
- Respect honest coarse-scale printability.
  - An `Alternative` symbol must be labelled as a conventional symbol.
  - An unsupported scale needs a reason.
- Do not spawn agents. Message the coordinator (`main`) through SendMessage for cross-system decisions, then continue with independent work.
- Tests run through `repo` (for example `repo run @teaching/staffage:test --lang=en`). If the repo tooling is broken, use `repo tools exec bun -- ...` or vitest with a ticket-local config, and say so in your report.
- Docstrings start with a unique emoji. Code has no comments inside definitions. Temporary logs use the `[DEBUG] ` prefix.
