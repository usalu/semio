# Result CENSUS4-PKG-14 (prefix `c4p14`): plugin-family TS and Python test-adapter files split into cohesive children

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Package 14 of `F/MONOLITH-CENSUS-4.md` section 4 (8 files, 12,016 lines). Tools in `F/c4p14-tools/`, raw outputs in `F/🗑️generated/c4p14-*`.

## 0. Verdict

**All eight files are split, installed and verified. Nothing was left half applied.** Every root is now under 500 lines and every child is at most 652 lines (limit about 1,000). Behaviour is unchanged: the Python oracle runs, the glTF generator output and the PDF schema are byte or deep identical to the pre-edit state, and `tsc` reports exactly the same 9 diagnostics as before.

| # | Target (under `✏️s/🔌️plugins/`) | Lines before | Root after | Children | Largest child | Mode |
|---|---|---:|---:|---:|---:|---|
| 1 | `🔋️energy/…/🧪️tests/🏛️mutate-energy-model-1/🪨️fabric-and-gains/🐍️.py` | 1,072 | 462 | 1 (`👥️gain-kinds`) | 652 | python child loaded by `_load` |
| 2 | `🔋️energy/…/🧬️schema/🧬️mutations/🟦️.ts` | 2,548 | 357 | 47 (one `🟦️.ts` per existing mutation group folder, 21 to 75 lines each) | 75 | `import type` + re-export, no new folders |
| 3 | `🗄️stdio/🗿️artifacts/📖️pdf/…/🧱️base/🧬️schema/🔺️diff/🟦️.ts` | 2,730 | 381 | 0 new files; the 2,351-line inline schema literal is replaced by the existing sibling `🔣️.json` | n/a | json import |
| 4 | `🗄️stdio/…/🧊️gltf/…/🏭️generator/📜️script.ts` | 1,404 | 129 | 7 folders | 305 | module imports |
| 5 | `🗄️stdio/…/🧿️semio/…/🏛️model/🧪️tests/🏛️mutate-semio-model/🐍️.py` | 1,003 | 210 | 3 | 390 | python children |
| 6 | same, `🕸️graph/…/🌳️mutate-semio-graph/🐍️.py` | 1,011 | 187 | 3 | 356 | python children |
| 7 | same, `🖊️drawing/…/🖊️mutate-semio-drawing/🐍️.py` | 1,181 | 213 | 3 | 378 | python children |
| 8 | same, `🧊️brep/…/🧊️mutate-semio-brep/🐍️.py` | 1,067 | 186 | 3 | 463 | python children |

## 1. Found state (recovery after the two session restarts)

Two restarts cut the work. State found and checked before continuing:

- Tools complete in `F/c4p14-tools/`: `gsplit.cjs`, `pysplit.py`, `fabsplit.py`, `pyrun.py`, `pychk.py`, `pdf-check.ts`, `files.json`, `specs/`, snapshots of all eight originals in `F/🗑️generated/c4p14-snap/`.
- The tree was **fully applied, not half applied**: all eight roots already below 500 lines, all children present, no stale folder (an early `🔤️text-carrier` name listed in `taxdirs.txt` no longer exists on disk; the live children are `📝️text-carrier`, `📦️pack-carrier`, `🧬️mutation-verbs`). No `__pycache__`, `.orig` or `.bak` left in the touched trees.
- The cut came during the last verification round. Two last-round checks had failed for environmental reasons, not code reasons: the final `tsc` run died with `ENFILE: file table overflow` (system file table full, load average 57 from the fleet), and my last combined command was killed with exit 137. The energy owned-target test path I used in that command was also wrong in my own notes.
- Nothing needed rolling back. I reran the failed checks after the load dropped (section 4) and added `c4p14-tools/tschk.py` for the lossless check of the three TS targets.

## 2. Layout

Energy mutations (`🧬️schema/🧬️mutations/`): the root keeps the shared types and the union/registry wiring; each mutation group folder that already existed (`⏯️run`, `☀️pv-system`, `⚠️fault`, `⛱️shading-device`, `❄️refrigeration`, `🌞️daylight-zone`, `🌡️thermostat`, `🌪️ventilation`, `🌬️air-loop`, `🌳️shading`, `🍃️outdoor-air`, `🎚️setpoint`, `🎨️finish`, `🏗️construction`, `🏡️zone`, `🏭️plant-loop`, `👥️people-gain`, `💡️lighting-gain`, `💧️humidistat`, `💨️infiltration`, `💯️ideal-loads`, `📆️year-schedule`, `📈️time-series`, `📋️space-list`, `📍️site`, `📏️sizing`, `🔋️model`, `🔌️load-center`, `🔍️glazing`, `🔒️enclosure`, `🔥️solar-thermal`, `🔧️zone-equipment`, `🔲️boundary`, `🕐️fixed-schedule`, `🕒️day-schedule`, `🕳️leakage`, `🖥️equipment-gain`, week-schedule, `🚰️water-system`, `🚿️shw-system`, `🛋️space`, room-air and the rest) now owns a `🟦️.ts` with the payload interfaces of its own Rust twin leaves. No new folders were created, so there is no emoji or sibling-name risk.

glTF generator (`🏭️generator/`): `🏗️base-document` (113), `✂️structural-edit` (220), `🧬️recipe-registry` (29), `👪️recipes-nodes-and-scenes` (265), `🕸️recipes-geometry-and-materials` (305), `🧥️recipes-skin-animation-and-resources` (278), `🌱️real-input-rows` (97). The root `📜️script.ts` keeps only the CLI (`generate`, `list`) and imports the children; recipe families register by side-effect import in a fixed order, so `RECIPES` order is unchanged.

Python adapters (four semio, one energy): `📝️text-carrier`, `📦️pack-carrier`, `🧬️mutation-verbs` per semio adapter; energy gets `👥️gain-kinds` beside its existing eight children. Each root keeps the verb table wiring and loads the children with the adapter's existing `_load(alias, folder)` helper.

PDF diff: the file held the whole JSON schema as an inline `as const` literal (2,351 of 2,730 lines) that duplicated the sibling `🔣️.json`. It now does `import schemaDocument from "./🔣️.json" with { type: "json" }; export const schema = schemaDocument;` and keeps the 51 parsers and the types.

**Source-text reader (added by SPLIT-HARDENING-7).** `…/🔋️energy/…/🧬️schema/🦀️.rs` read the `mutations.typescript` facet leaf as `include_str!("🧬️mutations/🟦️.ts")`, the 357-line root only. It is now a `concat!` of the root and the 47 group leaves in the root's import order, the way the BIM mutations descriptor does it; probe `F/sh7-tools/reader-probe.py` finds `ChangeZoneVolume`, `ChangeBatteryCapacity`, `ChangeRunEndDay`, `ReplaceSurfaceVertices` and loses 0 lines of the HEAD monolith (`F/RESULT-SPLIT-HARDENING-7.md` section 2). HEAD order is not reproducible: 8 groups interleave in the monolith.

## 3. Data and tables

The only large data in scope was the pdf diff schema literal, replaced by the sibling `🔣️.json` as above (deep-equal to the old literal, section 4). Python tables (`VECTOR_ROOTS`, `VOCABULARY`, `EXTRA_INVERT`, energy `**gains` rows) were kept whole in one child each and merge in the original key order.

## 4. Verification actually run

| Check | Method | Result |
|---|---|---|
| Lossless, TS | `c4p14-tools/tschk.py`: stripped non-blank line multiset of the snapshot minus the new module tree | energy mutations: lost 0, added 95 (docstrings, `import type`, re-exports). glTF generator: 35 lines lost and 68 added, all of the form `function x` becoming `export function x` plus `Doc`/`Remap` imports and region markers (no body line lost). pdf diff: the 2,351 lost lines are the schema literal, covered by the deep-equal row below |
| Lossless, Python | `c4p14-tools/pychk.py` (`c4p14-pychk-final2.log`): compile, symtable undefined globals, top-level statement multiset, line multiset, region markers, merged tables equal in content and order | `PROBLEMS 0`; every case statements lost 0 |
| PDF schema | `c4p14-tools/pdf-check.ts` (`c4p14-final-pdf-check.log`) | exported schema deep-equals the old literal: true; 55 exports, 51 parsers; accept/refuse probes identical to the post-split baseline log |
| Python oracle, five adapters | `c4p14-tools/pyrun.py` runs each adapter (`c4p14-py-*-before.jsonl` versus `*-final2.jsonl`), compared per scenario on status, full output projection and diagnostics with temp paths normalised (`c4p14-py-compare-final2.log`) | semio_model 37/37 passed, semio_graph 55/55, semio_brep 40/40, semio_drawing 52 (50 passed, 2 errored), energy 1,189 (1,159 passed, 2 failed, 28 errored); **0 scenarios differ from before in any of the five**. The 2 drawing errors and the 30 energy non-passes are identical before and after, i.e. pre-existing (missing shared fixtures, not touched by this slice) |
| glTF generator | `SEMIO_FIXTURE_OUT=… bun ./📜️script.ts generate`, `list`, `list --json` | writes 127 bundles; `diff -r` of the output tree against the pre-edit output is empty (247 files each); both list outputs byte identical to the pre-edit lists |
| `tsc` | `node tsc -p c4p14-tsc-final.json --noEmit` over the 59 roots (all new and edited TS plus the energy owned-target test), `c4p14-tsc-final.tsc.txt` | 9 diagnostics (8 TS2304, 1 TS2345), message set **identical** to the pre-edit run `c4p14-tsc-before` (pdf diff `Cannot find name 'T'` x2, energy mutations x6, three `GLTFExporter` plugin typing). No new diagnostic. Diagnostics present before and after are not in scope |
| Energy owned-target test | `bun test ./🟦️.test.ts` in `🧬️mutations/🧪️tests/🪪️owned-target` | 1 pass, 0 fail, 8 expects |
| Schema parsers | `bun ./📜️script.ts schema parsers check` | `pending=0 dangling=0 helperless=0 drift=0 orphans=0 unowned=177`, same as the pre-edit baseline |
| Path emoji statutes | `bun test` of `🔏️path-emoji-statutes` (37 tests) | 27 pass, 10 fail; the pass/fail set per test name is identical to the pre-edit run (pre-existing failures, e.g. the gltf fixture path-shape contract) |
| Path length | longest new relative path | 189 bytes, under 256 |
| `[DEBUG]` | grep of the touched trees | none added by this slice (hits exist only in untouched sqlite snapshot tests) |

Not run (out of scope for this slice, not claimed): Rust compile of the energy or pdf Rust twins (no Rust file was edited), `nx` graph tasks.

## 5. Files touched

Edited roots: the eight in the table. Created: 47 energy group `🟦️.ts`, 7 glTF child folders with `🟦️.ts`, 13 Python child `🐍️.py` files in 13 new folders (3 in each of the 4 semio adapters, plus `👥️gain-kinds` for energy). Tool files stay in `F/c4p14-tools/`; `F/🗑️generated/c4p14-*` is generated output to be removed when the ticket closes.
