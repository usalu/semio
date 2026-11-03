# 📑️ Evidence Index: staffage multiview ticket

Compiled by haiku-docs (slot 20) from a listing of the ticket folder taken 2026-11-02 at about 17:36. It covers every file in T except `🗑️generated` (excluded by the coordinator's instruction). Status values: **draft** (in progress or not yet reviewed), **final** (complete for its purpose), **superseded** (replaced by a named file), **current** (a living record). Verification is not claimed by this index: "worker-reported" means the owner says so; nothing here is coordinator-checked.

T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE`.

## 1. Files in T (excluding `🗑️generated`)

| File | Owner | Purpose | Status |
|---|---|---|---|
| `🎫️ticket.json` | coordinator | ticket record: title, goal, sessions, the 14 Sonnet and 6 Haiku fleet note | current |
| `📋️brief.md` | coordinator | fleet brief: goal, baseline, conventions, ownership, rules | final (input) |
| `🗂️work-board.md` | coordinator | fleet table (20 rows), decisions D1 to D33 | current |
| `🔗️interfaces.md` | sonnet-contracts | contract types, schema gaps, validator codes, resolver API, update §6 | draft (draft 1, updated 17:28) |
| `📐️conventions.md` | sonnet-contracts | frame, cameras, moments, sections, time, dimensions, solids, symbols | draft; worker-reported, exists |
| `🏗️structural-projection.md` | sonnet-structural | forces, moments, supports and diagrams in the three views; sign conventions | draft |
| `📖️view-documentation-inventory.md` | haiku-docs | inventory of view documentation and terminology (task 1) | superseded by `📖️terminology.md` for the terms; kept as the inventory |
| `📖️terminology.md` | haiku-docs | en/de vocabulary and the deviation list; labels T1–T26 and TS1–TS11 | final for the vocabulary (approved by the coordinator); its applications are pending |
| `📖️readme-views-draft.md` | haiku-docs | README "Views" section as a proposal, with (pending) markers | draft (proposal for the coordinator) |
| `📦️handoff.md` | haiku-docs | final handoff; sections filled as far as the sources allow; matrix and proofs TODO | draft |
| `📝️haiku-docs.md` | haiku-docs | running log of every haiku-docs task (tasks 1 to 12) | current |
| `📑️evidence-index.md` | haiku-docs | this file | current |
| `📝️haiku-inventory.md` | haiku-inventory (slot 15) | catalog inventory from Grep counts (task 1) | final (static) |
| `📝️haiku-inventory-unavailable.md` | haiku-inventory (slot 15) | audit of the 62 unavailable band decisions (task 2) | final (static) |
| `📝️haiku-spatial-audit.md` | haiku-spatial-audit (slot 16) | conventions audit and 10 spot checks (task 1); 14 more pending | draft |
| `📝️haiku-analysis-audit.md` | haiku-analysis-audit (slot 17) | thermal and structure audit (task 1) | final (static) |
| `📝️haiku-analysis-extents.md` | haiku-analysis-audit (slot 17) | classification of 211 extent failures in 53 assets, from `🗑️generated/contracts/failures.json` (task 2) | final (static) |
| `📝️haiku-analysis-structural-signs.md` | haiku-analysis-audit (slot 17) | sign conventions in the left-handed frame; coordinator rule D23 | final (static) |
| `📝️haiku-consumer-audit.md` | haiku-consumer-audit (slot 18) | consumer view propagation (task 1) | final (static) |
| `📝️haiku-consumer-callsites.md` | haiku-consumer-audit (slot 18) | course call sites: handedness, mirror, sizing; 33 files, 169 call sites (task 2) | final (static) |
| `📝️haiku-consumer-viewer-a11y.md` | haiku-consumer-audit (slot 18) | viewer i18n, accessibility and tests, read while sonnet-viewer edited | draft (static, snapshot) |
| `📝️haiku-registry.md` | haiku-registry (slot 19) | manifest and cast-viewpoint preparation (task 1); its 88-of-94 count is stale | draft (task 2 running) |
| `📝️haiku-registry-media.md` | haiku-registry (slot 19) | derivative media inventory (task 2); read-only | draft |
| `📝️sonnet-animals.md` | sonnet-animals | animals 14 and appliances B 12: per-asset status | draft (PAUSED at the D24 freeze, per the report) |
| `📝️sonnet-buildings.md` | sonnet-buildings | components 15 and appliances A 12: per-asset status, 1:500 bands, proof renders | draft (open items listed) |
| `📝️sonnet-exports.md` | sonnet-exports | exports, multiview sheet, cast schema, CLI, evidence | draft (worker-reported evidence; DWG unverified) |
| `📝️sonnet-structural.md` | sonnet-structural | 3D structure solver, structural views, unresolved cross-owner items | draft |
| `📝️sonnet-thermal.md` | sonnet-thermal | thermal and transport in 3D; task 2 open items T9 and T10 | draft (task 2 running) |
| `📝️sonnet-vegetation.md` | sonnet-vegetation | per-asset status of 16 vegetation assets; crown fill held | draft |
| `📝️sonnet-validation.md` | sonnet-validation | (expected by the handoff) | **missing** (see section 4) |

Also in T: `true`, an empty directory at the root (created 16:25, not a file of any worker). Status: stray, delete candidate. Not deleted here.

## 2. Generated output (not indexed)

`🗑️generated/` holds the outputs of the workers, including `validation/matrix.json` (10.1 MB, 17:34, worker-reported), `sonnet-validation/{full,full2,probe,sync}`, `contracts/failures.json`, `animals/`, `buildings/`, `humans/`, `machines/`, `objects/`, `rigs/`, `vegetation/`, `haiku-registry/`, `sonnet-course/`, `sonnet-exports/`, `sonnet-structural/` and `proofs-test/`. It is excluded from this index.

## 3. Scripts under `🛠️scripts`

Run convention for every script, from the ticket folder T (the working directory):
- `.ts`: `bun 🛠️scripts/<owner>/<file>.ts [arguments]`
- `.mjs` and `.cjs`: `node 🛠️scripts/<owner>/<file>` (or `bun`, to confirm per file)
- `.py`: `python -I 🛠️scripts/<owner>/<file>.py [arguments]`
- `.sh`: `bash 🛠️scripts/<owner>/<file>.sh [arguments]`
- `.json`: data, not run.

Status for every script: working script, not reviewed. Names with `fix`, `patch`, `solidify`, `add`, `addcause` and `fix-*` are one-off edits to the catalog or fixtures; they are likely **superseded** once applied, which is not verified. Arguments were not read for any script except where stated. Scripts with absolute `C:/git/semio/...` imports will not run on other machines.

Counts (files): animals 46, buildings 38, course 1, haiku-registry 7, humans 30, machines 25, objects 31, registry 3, rigs 4, sonnet-contracts 22, sonnet-course 6, sonnet-exports 5, sonnet-validation 8, sonnet-viewer 2, thermal 1, vegetation 38.

Named entry points (from the owners' reports, not verified):
- buildings: `audit.ts` (guard sweep), `deep.ts` (all poses, 11 clip samples per scale), `val.ts` (live `validateAsset`), `suit.ts` and `suit2.ts` (print suitability), `ext.ts` (per-shape extents), `render.sh`, `stack.py` and `grid.py` (contact sheets).
- vegetation: `audit4.ts` (extent audit), `bandfix.ts`, `symbols.ts`, `trunks.ts`, `fill.ts` (dry run only), `sway.ts`, `unsway.ts`, `ranges.ts`, `resid*.ts`, `plume.ts`, `proof.sh`.
- sonnet-contracts: `failures.ts` (its first argument is the output path `<out.json>`; it imports absolute `C:/git/semio/...` paths), `survey.ts`, `extent-probe.ts`.
- course: `course/tests.sh` (runs the course unittest modules through `uv`, output under `T/🗑️generated/course`); its arguments default to the guard and the staffage tests when none are given. Candidate replacement for `m9_tests.sh` of the previous ticket; not run.
- haiku-registry: `media.mjs` (cast cache walk).
- objects: `render.sh`, `rt.ts`, `validate.ts`, `symbols.ts`, `report.ts`.

### Full list

animals (`🛠️scripts/animals/`):
- `add500.py`, `addcause.py`, `anchors.ts`, `audit.ts`, `bones.ts`, `bonesol.ts`, `classify.ts`, `cons.ts`, `dump.ts`, `ext.ts`, `extremes.ts`, `fix1.py` to `fix9.py`, `flagged.ts`, `flagged2.ts`, `gullfix.py`, `handed.ts`, `lat.ts`, `lint.ts`, `mine.py`, `noassetalt.py`, `overview.mjs`, `probe.ts`, `proofs.ts`, `raw.ts`, `roundtrip.mjs`, `rt.py`, `shapes.ts`, `solidify.py`, `solidify2.py`, `solids.ts`, `spread.ts`, `sweepall.ts`, `table.ts`, `travel.ts`, `unavail.py`, `validate.ts`, `who.ts`

buildings (`🛠️scripts/buildings/`):
- `audit.ts`, `bones.ts`, `cw.ts`, `cw2.ts`, `deep.ts`, `dump.ts`, `emit.ts`, `emit2.ts`, `ext.ts`, `fix-500.ts`, `fix-causes.ts`, `fix-symbols.ts`, `fix2.ts`, `fixtext.ts`, `grid.py`, `ids.ts`, `ids2.ts`, `lib.ts` (library, not a command), `loads.ts`, `minf.ts`, `one.ts`, `parse.ts`, `patch1.ts` to `patch4.ts`, `pens.ts`, `render.sh`, `solidify.ts`, `stack.py`, `stackf.py`, `suit.ts`, `suit2.ts`, `sy.ts`, `un.ts`, `un2.ts`, `val.ts`, `vitest.config.ts` (config, not a command)

course (`🛠️scripts/course/`):
- `tests.sh`

haiku-registry (`🛠️scripts/haiku-registry/`):
- `band500.ts`, `counts.mjs`, `cross.mjs`, `cross2.mjs`, `ids.ts`, `inspect.mjs`, `media.mjs`

humans (`🛠️scripts/humans/`):
- `addz.ts`, `audit.ts`, `bands-skate.ts`, `bands-toddler.ts`, `bands.ts`, `bonecov.ts`, `cause.ts`, `diag.ts`, `extremes.ts`, `fillbones.ts`, `fix1.ts`, `fix2.ts`, `forward.ts`, `full.ts`, `lateral.ts`, `lib.ts` (library), `limbwidth.ts`, `probe.ts`, `proofplan.ts`, `rev-standing-adult.ts`, `revert-revolve.ts`, `shape.ts`, `shp.ts`, `simplify.ts`, `suit.ts`, `summary.ts`, `symbols.ts`, `undo-lateral.ts`, `validate.ts`, `wc-front.ts`

machines (`🛠️scripts/machines/`):
- `anchor-bounds.ts`, `anchors.ts`, `dry-shapes.ts`, `excavator-fix2.ts`, `excavator-slew.ts`, `excavator-solids.ts`, `excavator-unslew.ts`, `extents.ts`, `fix-extents.ts`, `fix-extents2.ts`, `follow.ts`, `guard.ts`, `inventory.ts`, `lib.ts` (library), `over.ts`, `render-symbols.ts`, `render.ts`, `shapes.ts`, `side.ts`, `solar500.ts`, `solidify.ts`, `symbols.ts`, `twist.ts`, `validate.ts`, `wheels.ts`

objects (`🛠️scripts/objects/`):
- `audit.ts`, `axes.ts`, `bands500.ts`, `bike500.ts`, `cafe.ts`, `cafe2.ts`, `clamp.ts`, `dbg.ts`, `emit.ts`, `ext.ts`, `ext2.ts`, `extents.ts`, `fix2.ts`, `grill.ts`, `grill2.ts`, `grill3.ts`, `hand.ts`, `patch.ts`, `picnic.ts`, `ranges.ts`, `redraw500.ts`, `render.sh`, `report.ts`, `rt.ts`, `snap.ts`, `summary.ts`, `support.ts`, `symbols.ts`, `symcheck.ts`, `validate.ts`

registry (`🛠️scripts/registry/`):
- `prune.py` (prunes cast cache keys; reads `prune-list.json`); `prune-list.json` (data). Owner not stated; likely haiku-registry, to confirm. Not run.

rigs (`🛠️scripts/rigs/`):
- `audit.ts`, `diagnose.ts`, `migrate-axis.ts` (the D24 axis migration), `state3d.ts`

sonnet-contracts (`🛠️scripts/sonnet-contracts/`):
- `extent-probe.ts`, `failures.ts`, `fix-fixtures.ts`, `patch-aw.ts`, `patch-axis.ts`, `patch-captions.ts`, `patch-catalog2.ts`, `patch-d27.ts`, `patch-d28.ts`, `patch-d28b.ts`, `patch-dims.ts`, `patch-dimsts.ts`, `patch-docs.ts`, `patch-header.ts`, `patch-interfaces.ts`, `patch-lateral.ts`, `patch-measure.ts`, `patch-messages.ts`, `patch-ranges.ts`, `patch-twin.ts`, `patch-wf.ts`, `survey.ts`

sonnet-course (`🛠️scripts/sonnet-course/`):
- `context.py`, `explicit_viewpoint.py`, `fix_json_newlines.py`, `inventory.py`, `rename_mirrored.py`, `show.py`

sonnet-exports (`🛠️scripts/sonnet-exports/`):
- `chk.ts`, `cli_patch.py`, `fix2.py`, `forklift.ts`

sonnet-validation (`🛠️scripts/sonnet-validation/`):
- `bench.ts`, `probe2.ts`, `probe3.ts`, `probe4.ts`, `prof.ts`, `raster.py`, `summary.ts`, `syncaudit.ts`

sonnet-viewer (`🛠️scripts/sonnet-viewer/`):
- `terminology.cjs` (terminology check, to confirm), `vitest.config.ts` (config)

thermal (`🛠️scripts/thermal/`):
- `sweep.ts`

vegetation (`🛠️scripts/vegetation/`):
- `anchorz.ts`, `audit.ts`, `audit2.ts`, `audit3.ts`, `audit4.ts`, `bandfix.ts`, `bones.ts`, `cover.ts`, `env.ts`, `fill.ts`, `fl.ts`, `flsum.ts`, `info.ts`, `inv.ts`, `lib.ts` (library), `lsshapes.ts`, `overview.ts`, `palm2.ts`, `palmgrid.ts`, `parity.ts`, `plume.ts`, `poseab.ts`, `proof.sh`, `ranges.ts`, `resid.ts`, `resid2.ts`, `resid3.ts`, `roundtrip.ts`, `rt2.ts`, `shape.ts`, `shape2.ts`, `stems.ts`, `sway.ts`, `symbols.ts`, `symlist.ts`, `t1.ts`, `trunks.ts`, `unsway.ts`

## 4. Files cited by the handoff or the reports that do not exist

- `T/📝️sonnet-validation.md`: cited by the handoff (coverage-matrix source status). **Missing** at the check.
- `T/🗑️generated/validation/matrix.json`: the handoff said it did not exist. It **now exists** (17:34). The handoff text was updated to say so; the matrix is not read into the handoff.
- `T/🗑️generated/<agent>/`: a template placeholder in the handoff, not a file. Not an error.
- `m9_tests.sh` (previous ticket, `☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS`): exists there, not in T. Cited in the handoff; the candidate replacement `T/🛠️scripts/course/tests.sh` exists in T.
- `T/🗑️generated/contracts/failures.json`: cited by `📝️haiku-analysis-extents.md`. It exists (inside the excluded `🗑️generated`).

No other citation in the handoff, terminology draft or the README draft was found missing.

## 5. Scripts with absolute `C:/git/semio` paths (cross-platform blockers)

Grep: `C:[/\\]+git[/\\]+semio` over `🛠️scripts/` (run 2026-11-02, about 17:45). Line numbers are the grep's. Owner is the folder's worker. Coordinator will route fixes to resolve paths relative to the script or the repo root.

Totals: 110 files and 152 matching lines, out of about 300 scripts.

| Owner | Folder | Files | Matching lines |
|---|---|---|---|
| sonnet-animals | `animals/` | 27 | 44 |
| sonnet-buildings | `buildings/` | 14 | 14 |
| sonnet-humans | `humans/` | 10 | 10 |
| sonnet-machines | `machines/` | 18 | 36 |
| sonnet-objects | `objects/` | 14 | 14 |
| sonnet-vegetation | `vegetation/` | 14 | 14 |
| sonnet-thermal | `thermal/` | 1 | 1 |
| sonnet-contracts | `sonnet-contracts/` | 3 | 6 |
| sonnet-course | `sonnet-course/` | 4 | 4 |
| sonnet-exports | `sonnet-exports/` | 3 | 6 |
| sonnet-viewer | `sonnet-viewer/` | 2 | 3 |

Files and lines (`file:line`):

animals (sonnet-animals):
- `animals/anchors.ts:1,2`, `animals/audit.ts:2,3`, `animals/bones.ts:2,3`, `animals/bonesol.ts:1`, `animals/classify.ts:2`, `animals/cons.ts:1`, `animals/dump.ts:1`, `animals/ext.ts:1,2`, `animals/extremes.ts:1,2`, `animals/fix3.py:23`, `animals/flagged.ts:2,3`, `animals/flagged2.ts:2,3`, `animals/handed.ts:1,2`, `animals/lat.ts:1,2`, `animals/lint.ts:1`, `animals/overview.mjs:2`, `animals/probe.ts:1,2`, `animals/proofs.ts:1,2`, `animals/raw.ts:1,2`, `animals/shapes.ts:1,2`, `animals/solids.ts:1`, `animals/spread.ts:1`, `animals/sweepall.ts:1`, `animals/table.ts:1,2`, `animals/travel.ts:1,2`, `animals/validate.ts:1,2`, `animals/who.ts:1,2`

buildings (sonnet-buildings):
- `buildings/audit.ts:2`, `buildings/bones.ts:2`, `buildings/deep.ts:2`, `buildings/env.ts:2`, `buildings/ext.ts:2`, `buildings/fix-blank.ts:2`, `buildings/fixtext.ts:2`, `buildings/lib.ts:19`, `buildings/solidify.ts:2`, `buildings/suit.ts:2`, `buildings/suit2.ts:2`, `buildings/sy.ts:2`, `buildings/val.ts:2`, `buildings/vitest.config.ts:2`

humans (sonnet-humans):
- `humans/audit.ts:3`, `humans/bonecov.ts:3`, `humans/extremes.ts:2`, `humans/full.ts:2`, `humans/lib.ts:2`, `humans/limbwidth.ts:2`, `humans/probe.ts:2`, `humans/proofplan.ts:3`, `humans/shp.ts:2`, `humans/validate.ts:2`

machines (sonnet-machines):
- `machines/anchor-bounds.ts:1,2`, `machines/anchors.ts:1,2`, `machines/dry-shapes.ts:4,5`, `machines/excavator-solids.ts:2,3`, `machines/extents.ts:1,2`, `machines/follow.ts:1,2`, `machines/guard.ts:1`, `machines/inventory.ts:1`, `machines/lib.ts:2`, `machines/over.ts:1,2`, `machines/render-symbols.ts:1,2,3`, `machines/render.ts:1,2,3`, `machines/shapes.ts:1,2`, `machines/side.ts:1,2`, `machines/solidify.ts:3,4,5`, `machines/twist.ts:1,2`, `machines/validate.ts:1,2`, `machines/wheels.ts:1,2`

objects (sonnet-objects):
- `objects/audit.ts:3`, `objects/bands500.ts:2`, `objects/dbg.ts:1`, `objects/emit.ts:1`, `objects/ext.ts:1`, `objects/ext2.ts:1`, `objects/extents.ts:1`, `objects/hand.ts:1`, `objects/patch.ts:2`, `objects/snap.ts:1`, `objects/support.ts:1`, `objects/symbols.ts:2`, `objects/symcheck.ts:1`, `objects/validate.ts:1`

vegetation (sonnet-vegetation):
- `vegetation/audit.ts:2`, `vegetation/audit2.ts:2`, `vegetation/audit3.ts:2`, `vegetation/audit4.ts:2`, `vegetation/bones.ts:2`, `vegetation/info.ts:2`, `vegetation/inv.ts:2`, `vegetation/lib.ts:2`, `vegetation/overview.ts:2`, `vegetation/ranges.ts:2`, `vegetation/roundtrip.ts:2`, `vegetation/rt2.ts:2`, `vegetation/stems.ts:2`, `vegetation/unsway.ts:2`

thermal (sonnet-thermal):
- `thermal/sweep.ts:10`

sonnet-contracts:
- `sonnet-contracts/extent-probe.ts:1,2`, `sonnet-contracts/failures.ts:1,4,5`, `sonnet-contracts/survey.ts:1`

sonnet-course:
- `sonnet-course/context.py:3`, `sonnet-course/explicit_viewpoint.py:3`, `sonnet-course/fix_json_newlines.py:5`, `sonnet-course/inventory.py:3`

sonnet-exports:
- `sonnet-exports/chk.ts:2`, `sonnet-exports/forklift.ts:1,2,3,4`, `sonnet-exports/symbols.ts:1`

sonnet-viewer:
- `sonnet-viewer/validate-walker.ts:2,3`, `sonnet-viewer/vitest.config.ts:3`

Notes:
- The folder changed since the section 3 listing: `buildings/env.ts`, `buildings/fix-blank.ts`, `sonnet-exports/symbols.ts` and `sonnet-viewer/validate-walker.ts` are new since that listing and are included here. Section 3 is not updated for them.
- Scripts that use only relative paths do not appear in this list.
- The grep matches the path text, not whether the file runs, so a hit in a comment is counted.

## 6. Open items for this index

- The stray empty directory `true` at the T root: removed by the coordinator.
- Scripts whose arguments and runner were not read: every entry in section 3 is a path and an extension, not a tested command.
- Status of scripts marked one-off (fix, patch, solidify and add names): not verified against the catalog.

Not run: no script, test or check was executed to build this index.
