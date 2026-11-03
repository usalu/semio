# 📈️ Diagrams D (wave 2)

Nothing compiled or run (reading only); handle-paths check (python, en1994/en1999): 0 uncovered.

## 1. Project site aggregate for document-level checks (shared)
- `🎬️structural-scene/🎯️projection/🧲️scene-kit`: `SITE_ENTITY = "project-site"` (re-exported); `canonical_order` ranks it first, as the global fragment leads in the live path (incremental equals full).
- `🧰️projection-kit/🔦️highlight`: `with_site` / `site_ref` / `worst_global`; `highlight_owners` for `Part::All|Global` adds a neutral 6x6 m ground plate (layer `site`, EN/DE names) led in the fragment whenever a non-NA check has an empty owner, coloured by the worst document-level check (never softening); the `global-checks` badge stays. Refactored shared `governs/counts/colour` helpers.
- `OwnershipAudit.site`; `is_clean` needs badge and site for globals; a failing documented global with an unlit site goes to `unlit`. All ten per-artifact ownership law tests use `ownership_audit`, so globals must now resolve to the aggregate.
- Tests: two new tests in `highlight/🧪️tests/🦀️.rs`.
- Risk: scene tests/goldens that count entities of demos with global checks (en1990 `input.*`, en1998 `site.*`, en1997 `governing.*`) may need +1; plate at origin extends scene bounds slightly.

## 2. Lower-ranked gaps
- en1996 chases and movementJoints diagrams: already present (`chase_depth`, `movement_joints`); nothing to do.
- en1994 placements: new `placements` plan diagram (diamond per placement, x drag, table twin) in `📈️diagrams`, only without focus; test added in `🔬️unit`.
- en1999 materials: new submodule `📈️diagrams/📉️stress-strain` (`stress-strain:<id>`): elastic, perfectly plastic line to f_0.2 (marker at the 0.2 % offset), f_u limit, plus HAZ-reduced line, limit and marker (rho_o,haz, rho_u,haz) when the alloy reduces; wired into `diagrams()`; two tests. Idealisation without hardening (no elongation or Ramberg-Osgood n in the repo); alloy rows unverified.
- en1995 fasteners.positions and panels.layers grips: already present (explicit px/py grips and layer t{i} grips, homes in `🧬️schema/⚖️timber/🧱️entities`); nothing to do.

Rebuild requested: en1994, en1999 (latest.md) and all builds via requests.md.
