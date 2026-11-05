# Final Read-only Audit

Reviewed current fitting implementation, motion bounds/crop semantics, new course coverage TypeScript twin and shared diagnostic fixture, and seasonal table use. No actionable correctness defect found in the reviewed edits.

## Motion Fit

The conditional motion probe in fitted_staffage is correct: _Motion inserts transform probes spanning index whole-clip boundsM, or figureBoundsM for transport casts. Normal VGroup bounds therefore contain every sampled frame; transport casts deliberately use figure-only bounds to avoid fitting airflow. A requested window provides the fitting extent directly, and _Motion crops frame paths in cast metres. The mirrored full-clip envelope has the same width and height, so omitting mirror from the temporary probe does not alter the fit scale. No new projection or scaling discrepancy identified.

## Coverage and Seasonal Table

The TypeScript twin exists and independently checks the shared fixture, actual manifest/catalog and all course source series. Missing asset-ambiguous and scale-denominator vectors are present; manifest-version is declared and explicitly tested. Valid shared manifest vectors are independently checked with Ajv.

The heating-comparison clip selects the table from the seasonal casts map and derives toaster height from that same selected table cast, preserving thermal scenario and support geometry together.

Earlier independent current-manifest audit resolved all 97 casts against 152 catalog assets with no missing asset, authored scale, selected-view geometry, pose or clip. No expensive suites were rerun for this read-only review; integration execution remains with the coordinator.

## Entire Clip Tree Scope Scan

Read-only filesystem scan applying the guard fixture selection exactly:

```json
{
  "guardRoot": "🎓️teaching",
  "clipFolder": "🎬️clip",
  "fileName": "🐍️.py",
  "skip": [
    "🧪️tests",
    ".venv",
    "node_modules",
    "__pycache__"
  ],
  "count": 186,
  "roots": {
    "⚡️energy": 98,
    "🎯️optimization": 16,
    "🏗️bim": 46,
    "📊️excel": 12,
    "🦗️grasshopper": 14
  },
  "coverageSeries": [
    "⚡️energy",
    "🎯️optimization",
    "🏗️bim",
    "📊️excel",
    "🦗️grasshopper",
    "🏢️nakagin",
    "🏙️typology",
    "🔀️morph",
    "📐️parametric",
    "🧲️fit",
    "📚️course"
  ],
  "missing": [],
  "forbiddenNames": [
    "person_glyph",
    "seated_person_glyph",
    "object_silhouette",
    "lamp_glyph",
    "respiration_parts"
  ],
  "forbiddenHits": [],
  "ruleAAllowed": {
    "⚡️energy/📊️demand/🎬️clip/⚖️regulations/🐍️.py::_north_window": "section: aperture cut through the section-material wall, glass from the shared window glyph",
    "⚡️energy/📊️demand/🎬️clip/⚖️regulations/🐍️.py::_year_clock": "diagram instrument: dial of the shared clock glyph with a redrawn month hand",
    "⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py::_valve_dial": "diagram instrument: dial mark redrawn on the anchor of the staffage valve",
    "⚡️energy/🔥️heating/🎬️clip/🧮️calculation/🐍️.py::_window_wall": "structure: wall opening of the building section",
    "⚡️energy/🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py::_lattice": "diagram: bonded and free particles of the physics lens",
    "⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py::person_bars": "chart: bar chart of person heat outputs",
    "⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py::_beam": "diagram: a straight light ray of the physics lens, not a structural beam",
    "🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py::ramp_strip": "chart: colour-ramp legend strip, not a wheelchair ramp",
    "🎯️optimization/🎬️clip/🎲️sampling/🐍️.py::_plane_group": "chart: axis lines, ticks and design-space box of a coordinate plane, not a tree",
    "🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/📋️plan/🐍️.py::_plan_ramps": "structure: IFC ramp footprints of the Savoye floor plan, drawn from model geometry",
    "🎯️optimization/🎬️clip/🎲️sampling/🐍️.py::_meters": "chart: sample meters of the optimization run",
    "🎯️optimization/🎬️clip/🏆️pareto/🐍️.py::_payback_table": "table: payback figures",
    "🎯️optimization/🎬️clip/🧪️experiments/🐍️.py::_table": "table: experiment results",
    "📊️excel/🎬️clip/📋️program/🐍️.py::table_dressing": "table: spreadsheet cell styling",
    "🦗️grasshopper/🎬️clip/🌳️trees/🐍️.py::present_trees": "data diagram: Grasshopper data trees",
    "🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py::_table": "table: capsule parameters",
    "🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py::_table": "table: component anatomy",
    "🦗️grasshopper/🎬️clip/🧱️model/🐍️.py::window": "structure: CAD window geometry of the solid model"
  },
  "ruleCAllowed": {
    "⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py::Beat2_HumanFactor.construct": "chart: activity scale with axis line, ticks and data dots",
    "⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py::Beat3_IsolateAirflow.construct": "diagram: abstract equality balance of two quantities, not a weighing device (the catalog has no balance asset)",
    "⚡️energy/📊️demand/🎬️clip/🏷️certificate/🐍️.py::_document": "diagram: certificate card with header band and ruled lines, a UI card not a paper object",
    "⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py::Beat7_SensibelLatent.construct": "diagram: fade-out group of the psychrometric chart parts, no new object",
    "🎯️optimization/🎬️clip/⚖️index/🐍️.py::Board._build": "chart: score rows and rank badges of the index board",
    "🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py::RoomVisuals.plan": "diagram: floor plan with orientation ring and compass, structure only",
    "🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py::RoomVisuals.room_section.build": "structure: room section geometry, daylight wedge, lamp halo and light cone around the pendant lamp casts",
    "🎯️optimization/🎬️clip/🎲️sampling/🐍️.py::Beat2_GridRandomSampling._orientation_glyph": "glyph: mini plan with north tick and glazed facade of a design sample",
    "🏗️bim/🎬️clip/🔗️consistency/🐍️.py::Beat6_Consistency.leak_marks": "diagram: gap, leak path and stain annotations on the build-up detail",
    "📊️excel/🎬️clip/🎨️visuals/📗️workbook/🐍️.py::Window._title_bar": "UI chrome: application title bar of the workbook window",
    "🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py::Beat2_Capsule._icon": "glyph: legend icons drawn from the clip's own widgets",
    "🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py::Beat2_Capsule._table": "table: capsule parameter table",
    "🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py::Beat1_Anatomy._eye": "UI glyph: preview eye above a Grasshopper component, struck through when off"
  }
}
```

The guard uses root-wide recursive discovery rather than enumerated series. Coverage fixture series were compared against every discovered architecture clip root. Forbidden primitive allowances describe diagrams, annotations, charts and teaching metaphors; they do not grant permission for a second physical staffage catalog. Rule A and C tests require every allowance to have a reason and to correspond to a live flagged function, rejecting stale allowances. The regex forbidden-helper pass found no current named-helper hits. This independent scan validates selection and naming, while the existing Python AST suite validates actual primitive composition and documented exceptions.

## Provenance Checkpoint and Guard Reconciliation

Reviewed current casts_in checkpoint invalidation. It clears _source_digest, _catalog_bytes and _presets before resolving selected casts. asset_revision no longer has its own lru_cache. Each content key includes the complete selected manifest entry, asset bytes, revisions of every referenced environment (including timeline keys), all exporter/module/schema source bytes and bundled print profiles. The manifest reader refreshes on file modification timestamp. Remaining frame/index/checked caches are keyed by the content-addressed cast folder; changed input keys select different folders, so these do not preserve an old cast across a normal changed-input checkpoint. No stale process-lifetime source memo found after the fix. This is checkpoint invalidation, not a concurrent source-snapshot transaction.

The restored _brick_block exception is confined to a scientific conduction diagram: two touching masonry wall regions, four staggered mortar courses, temperature tracker fills and heat-transfer arrows precede a microscopic lens. It depicts section-material control volumes, with its context explicitly labeled masonry; it is an appropriate documented teaching-diagram exception rather than an alternate freestanding staffage asset. The fixture now records that exact function and rationale. Current Rule A allowance count is 19, superseding the earlier snapshot of 18.
