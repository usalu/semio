# 📋 Round 3 summary: handcrafted staffage in the architecture course clips

Skeleton by worker `doc` (W20), 2026-11-03. Each section is filled only from the r3-*-report.md files in this folder. A claim without a report is not written here. Baseline of open items: `r3-baseline.md`. Round-2 record: `summary.md`.

## 1. Clip migration matrix

Pending. Filled from the W01 inventory and the clip matrices once they exist in this folder. No r3 inventory or matrix file was present at the time of writing.

| series | clip | depicted staffage | cast status | render status | report |
|---|---|---|---|---|---|
| (pending) | | | | | |

## 2. Assets added or extended

Pending. Source: asset reports (W10 objects, W11 buildings, W08 people, W09 vegetation) and the catalog check.

Known from the open-item review, not yet a report:

- `🔌️appliances/electric-kettle/🔣️.json`: `massKg` 1.22 (OI-23), verified by grep in `r3-baseline.md`.
- `🏗️machinery/evaporative-humidifier/🔣️.json`: moved from `🔌️appliances` (OI-48), verified by grep.
- `🏗️machinery/oil-pumpjack/🔣️.json`: skid and motor are separate bones (OI-114), verified by grep, mass not confirmed.
- `🔌️appliances/toaster/🔣️.json`: casing heat changed (OI-113), verified by grep, temperature not confirmed.

## 3. Integration changes

Pending. Source: contract and integration reports (W05 provenance, W18 validation, W19 registry).

Known from the open-item review:

- `⚡️energy/🏗️authoring/📦️packages/🐍️python/pyproject.toml` `[dependency-groups] test` now declares the modules named in OI-124 (verified by Read).

## 4. Commands and results

Pending. Each entry needs the command, the exit code and the pass count from a r3-*-report.md file.

Checks run by `doc` in this pass (read-only, grep and glob only, no tests run):

| check | result |
|---|---|
| `Grep` electric-kettle `massKg` | `1.22` at line 51 |
| `Grep` `evaporative-humidifier` in `🗂️catalog` | only `🏗️machinery` path |
| `Grep` `energy_tank(` in practice clip | line 1144 present |
| `Grep` `uv run --with` under `A` | 0 files |
| `Grep` `RES-FIN-ROOF-GRAVEL-SL-R-705` in BIM model | 2 files |
| `Grep` `class ModelCase` in BIM tests | line 277 present |
| `Glob` `**/silhouettes.py` under visuals | 0 files |
| `Glob` `**/*.env` under `⚡️energy` | 0 files |

## 5. Proof media

Pending. Source: render and media reports (W17 media, W15 renders, W12 timing, W13 player). Only renders that a report says were inspected are listed.

| clip | file | what it shows | inspected by | report |
|---|---|---|---|---|
| (pending) | | | | |

## 6. Limitations

Carried from round 2 and still open at the time of writing (see `r3-baseline.md` for owners):

- BIM model reserve and setup time: OI-01, OI-02, OI-80, OI-81, OI-118.
- Native German review: OI-73. No TTS key: OI-110. Speech tooling German only: OI-111.
- POSIX and devcontainer not run: OI-83, OI-84.
- Renders after round-2 edits not confirmed: OI-115, OI-67.
- Judgement items not decided: OI-10, OI-41 (ruled, to confirm), OI-46 (ruled, to confirm), OI-112, OI-121.
- Infra: `repo dependencies ensure` (OI-100), `repo ffmpeg` (OI-101), cache state (OI-102).

## 7. Fleet record

Round 3 fleet, from the brief map. Report files appear as each worker finishes. None existed at the time of writing.

| worker | scope | report | status |
|---|---|---|---|
| W01 | inventory | (pending) | pending |
| W05 | contract (provenance) | `r3-contract.md` and report (pending) | pending |
| W06 | thermal | (pending) | pending |
| W07 | structural | (pending) | pending |
| W08 | people (OI-47, OI-117) | (pending) | pending |
| W09 | vegetation (OI-10) | (pending) | pending |
| W10 | objects (OI-04, OI-23, OI-48, OI-87, OI-113, OI-114, OI-116, OI-123) | (pending) | pending |
| W11 | buildings (OI-46) | (pending) | pending |
| W12 | timing (OI-121) | (pending) | pending |
| W13 | player (OI-128) | (pending) | pending |
| W14 | heating and cooling clips (OI-41, OI-112) | (pending) | pending |
| W15 | ventilation, demand, physics, optimization clips (OI-67, OI-115) | (pending) | pending |
| W16 | other series (OI-01, OI-02, OI-80, OI-81, OI-85, OI-118) | (pending) | pending |
| W17 | media | (pending) | pending |
| W18 | validation (OI-82, OI-86, OI-89, OI-90, OI-119, OI-124) | (pending) | pending |
| W19 | registry (OI-102, OI-125, OI-127) | (pending) | pending |
| W20 | doc (this file, `r3-baseline.md`, summary corrections) | `r3-baseline.md` | done for phase 1 and 2 skeleton |
| coordinator | OI-73, OI-74, OI-109, OI-110, OI-111 | none | summary corrections done by W20 |
| infra | OI-83, OI-84, OI-100, OI-101 | none | open |
