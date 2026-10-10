# r12 exec - w2-wp20-energy-io (WP-20 energy IO: gbXML 7.03 export, IFC thermal property sets, import)

`T` = this ticket folder, `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, `D` = `T/🗑️generated/w2-wp20-energy-io/draft`.

## 1. State

**Written, not in the crate yet.** At about 16:15 the coordinator froze edits inside the BIM crate until `T/r13-blessed.flag` exists (r13 bless pass); the flag was still missing at 18:05. The relay agent repeated it: keep waiting. So everything lives as drafts in `D` plus four input scripts in `T` that land it in one go. Nothing Rust has been compiled. What did run:

| Command | Result |
|---|---|
| `python T/r12-w2-wp20-energy-io-prototype.py <S>/🧫️fixtures D/gbxml_oracle.py <out>`: lxml + numpy prototype gbXML of the `box` energy fixture, then the draft gbXML oracle on it | **0 problems**: ids, references, units, ranges, geometry, closure (outward area vectors sum to 0, divergence volume 29.97 = Volume, floor 9.99 = Area), and every area and loss group against the committed inference table `💡️inferences/🌡️energy-envelope/🏠️box/💡️inference/🌡️energy-envelope/🔣️.json` |
| Mutation check of the oracle on that prototype (roof dropped, tilt 80, window moved out of its wall, wall U changed, dangling idref, Volume 31, two AdjacentSpaceIds on an exterior wall) | 7 of 7 detected; the unchanged document gives 0 problems |
| `ifcopenshell` 0.8.4.post1 templates `Pset_IFC2X3.ifc` / `Pset_IFC4_ADD2.ifc` | property names and measure types read (section 3) |
| Draft oracles re-run after the rulings (holder/construction fields, curtain wall glass opening, zero-area wall groups skipped in the regrouping) | prototype still 0 problems |
| Dry run of every anchor of `T/r12-w2-wp20-energy-io-patch.py` against today's crate | each anchor found exactly once |

## 2. Install order (run from `T`, Git Bash, after the flag)

1. `PYTHONUTF8=1 ../../../../../../.venv/Scripts/python.exe r12-w2-wp20-energy-io-fixtures.py`: copies the house with conditions (`S/🖼️assets/🏡️house/📸️snapshot.json`) to `S/🧫️fixtures/🌿️gbxml/🏠️house/📸️snapshot/🔣️.json` and `S/🧫️fixtures/🏗️ifc/🔥️energy/📸️snapshot/🔣️.json`.
2. `python r12-w2-wp20-energy-io-install.py`: copies the drafts to their places (new files only, never overwrites):
   `S/🚪️io/📤️export/🧱️holders`, `.../📤️export/🌿️gbxml/{🦀️.rs, 🧱️codec, 📐️geometry, 🔗️pairing, 📊️tables}` (each with `🧪️tests/🔬️unit/🦀️.rs`), `.../📤️export/🏗️ifc/🔥️energy`, `.../📥️import/🏗️ifc/🔥️energy`,
   `S/🧪️tests/🌿️export-bim-1-gbxml/{🥒️.feature, 🦀️.rs, 🐍️.py}`, `S/🧪️tests/🔥️export-bim-1-ifc-energy/{🥒️.feature, 🦀️.rs, 🐍️.py}`.
   These files are inert until step 3 mounts them.
3. `python r12-w2-wp20-energy-io-patch.py`: exact-text edits of existing files (it reports `NOT FOUND` and leaves the file alone when a peer changed the anchor):
   `S/🚪️io/📤️export/🦀️.rs` (mount `holders`, `gbxml`), `S/🚪️io/🦀️.rs` (`serializer_entry::<ModelSnapshot, export::gbxml::ModelIntoGbxml>`), `.../📤️export/🏗️ifc/🦀️.rs` (mount `energy`, `Links.derived`,
   stage `("energy", energy::emit)` before `"links"`), `.../🏗️ifc/🧬️data/🦀️.rs` (derived rows merged into authored sets with the same name, the rest as their own sets; type rows `UValue`, `GValue`, `FrameFraction`),
   `.../📥️import/🏗️ifc/🦀️.rs` (mount `energy`, `energy::read_conditions` before `read_templates`), `.../📥️import/🏗️ifc/🧬️data/🦀️.rs` (type thermal data, `DerivedRows` filter in `read_attached`),
   `.../📥️import/🏗️ifc/🧱️walls/🦀️.rs` (occurrence thermal data for types made from an occurrence).
   The IO hop-list test `S/🚪️io/🧪️tests/🔬️unit/🦀️.rs` needs the new `(s.bim.model, s.stdio.xml@1.0)` row by hand.
4. `"$T/🚦️gate.sh" w2-wp20-energy-io -- cargo check --manifest-path ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml -p semio-s-artifact-bim-model --lib --tests --message-format=short` and fix compile slips. Known ones: `pairing::merge` pushes `space`/`host` without deref; the test drafts `energy_ifc_import_tests.rs` (leftover `strip` helper) and the `celsius` visibility need tidying.
5. `python r12-w2-wp20-energy-io-register.py`: adds oracle rows `bim-1-lxml-gbxml` and `bim-1-ifcopenshell-ifc-energy` to `S/🔮️oracles/🔣️.json`. The manifest round-trips byte for byte at indent 2.
6. Bless my own new fixtures only: `BIM_BLESS=1 cargo test ... --lib -- gbxml::tests::the_committed_files_are_the_current_export ifc::energy::tests::the_committed_files_are_the_current_export`.
   This writes `S/🧫️fixtures/🌿️gbxml/{🏠️box,🏘️zoning,🧱️stack,🏠️house}/gbxml.xml` and `S/🧫️fixtures/🏗️ifc/🔥️energy/energy-{2x3,4}.ifc`.
7. Oracles: `python S/🧪️tests/🌿️export-bim-1-gbxml/🐍️.py write S/🧫️fixtures/🌿️gbxml` then `check`, and `python S/🧪️tests/🔥️export-bim-1-ifc-energy/🐍️.py write S/🧫️fixtures/🏗️ifc/🔥️energy` then `check`.
8. `cargo test --lib` (gbxml, holders, ifc::energy, import energy, io tests, plus the existing IFC import byte-stable tests), then the wasm32-wasip2 `--lib` check, then `bun r3-f1-check-names.ts`.
9. Delete `T/🗑️generated/w2-wp20-energy-io`.

Side effect on peers (r13 bless): with step 3 the IFC 2x3 and IFC4 exports of every model that has conditions or window/door thermal data gain the thermal sets and the type rows `UValue`, `GValue` and `FrameFraction`. The assets house has both; the committed `🏗️ifc/🏠️house` fixture snapshot has neither and is unaffected. Re-bless any IFC fixture built from a snapshot with thermal data after step 3.

## 3. Design

### gbXML 7.03 (`S/🚪️io/📤️export/🌿️gbxml`)
- **Serializer and dialect.** `ModelIntoGbxml` writes into `s.stdio.xml@1.0/*` (`XML_DIALECT`), `IoFidelity::Lossy`. The tree is the `XmlNode` model of the stdio XML artifact and the text comes from `xml_document_to_text_checked`. `🧱️codec` adds only whitespace indentation between child elements. **The XML dialect needed no extension**: the namespace is a plain `xmlns` attribute, and escaping and the declaration are the writer's.
- **Stages.** `STAGES` = zones, spaces, constructions, windows, surfaces, then `document`. `StagedGbxml` steps one stage at a time (fraction, stage name, cancellable between steps) and keeps no borrow of the inference, like `ifc::StagedExport`. The one-shot path is `export_gbxml(model)`. A model without conditions has no envelope and is refused.
- **Content.**
  - A `Campus` per site, with `Location` (lat/long/elevation, `CADModelAzimuth` 0, site name).
  - A `Building` per building, with `Area`, each `BuildingStorey` (`Level` = inferred building-relative elevation) and its `Space`s: `zoneIdRef`, `conditionType` from the set points, `buildingStoreyIdRef`, Name, Description (occupancy, schedule), Area, Volume, `PeopleNumber` (space density else zone density, times area), `LightPowerPerArea`, `EquipPowerPerArea`, `OAFlowPerArea` (L/s/m²), `CADObjectId`.
  - `WindowType` with `U-value` and `SolarHeatGainCoeff`; the frame fraction goes in its Description.
  - `Construction` per distinct (type, U), with `U-value` and `LayerId`s; a door construction carries the U-value only.
  - `Layer` → `Material` per (material, thickness): Thickness, Conductivity, Density, SpecificHeat, R-value.
  - `Zone` per thermal zone (section below), then `DocumentHistory/ProgramInfo` (no date, so the output is deterministic).
- **Surfaces.** Type mapping as in the contract. `exposedToSun` for ExteriorWall/Roof/ExposedFloor. `constructionIdRef` only when the surface has a U-value. Children: `AdjacentSpaceId` once or twice, `RectangularGeometry` (Azimuth, lower-left CartesianPoint, Tilt, Height, Width), `PlanarGeometry`, `Opening`s, then `CADObjectId` (the surface id, and the partner side's id as a second `CADObjectId`).
- **Openings.** `FixedWindow` with `windowTypeIdRef`, or `NonSlidingDoor` with `constructionIdRef`. The RectangularGeometry point is the 2-D offset in the host's right/up frame; PlanarGeometry is the 3-D polygon.
- **Pairing (`🔗️pairing`).**
  - Adjacent and adiabatic sides of the same two spaces are paired. Walls pair when they share an element, face opposite ways (±20°) and overlap ≥ 50% of the smaller side in the wall plane. A floor pairs with the ceiling under it when area (1e-4) and plan centre (1 mm) match.
  - The geometry comes from the lower space id for a wall and from the upper space's floor for a floor (type `InteriorFloor`, spaces [upper, lower]). Openings of both sides are merged by element. A side without a partner stays a surface with one `AdjacentSpaceId`; the oracle flags that as a problem.
- **Orientation.** Polygons are turned clockwise by `true_north − rotation` of the building, so +Y is true north. `Azimuth` then means the same whether or not a reader applies `CADModelAzimuth` (written as 0). Every number is rounded to 9 decimals, and azimuth and tilt come from the written polygon.
- **Thermal zones.** Grouped by (authored zone, heating, cooling). An authored zone with several set-point pairs becomes "Name (1)", "Name (2)". A space without an authored zone gets its own zone. `DesignHeatT`/`DesignCoolT` use unit `C`.
- **Holders (`🧱️holders`), per the coordinator ruling.** No derivation is repeated: the module only names the collection of `EnvelopeSurface.holder` (wall, curtain wall, slab, roof, ceiling, window, door) and reads the layer stack of the type `EnvelopeSurface.construction`. Both fields are added by the coordinator after the flag; the drafts are coded against them and do not compile before.
- **Curtain walls (ruling 2).** A `CurtainWall` surface is written as an `ExteriorWall` surface without construction that holds one `FixedWindow` opening covering its whole polygon (`CADObjectId` `<surface>/glass`), whose `WindowType` is made from the curtain wall type (`U-value`, `SolarHeatGainCoeff`, frame fraction in the Description). IFC writes `Pset_CurtainWallCommon.ThermalTransmittance` (present in both schemas) from the surfaces' authored U; g-value and frame fraction travel in the `CurtainWallType` JSON row the curtain wall export already writes. A mullion psi is not modelled: the frame fraction covers the mullion share of the area.
- **Oracle table (`📊️tables`).** `export_table(model)` returns counts; spaces; surfaces keyed by `CADObjectId` (kind, exposed, spaces, construction name, U, azimuth, tilt, width, height, polygon area, openings); constructions with their material names; window types; and the polygon area per surface type.

### IFC (`S/🚪️io/📤️export/🏗️ifc/🔥️energy`, stage `energy` before `links`)
- **Space conditions.** A space with conditions gets `Pset_SpaceThermalRequirements` with these rows:
  - `SpaceTemperatureMin` and `WinterMin` = heating set point.
  - `SpaceTemperatureMax` and `SummerMax` = cooling set point.
  - `AirConditioning` = a cooling set point is stated.
  - Outdoor air as air changes per hour: `MechanicalVentilationRate` (`IFCCOUNTMEASURE`, rate·3.6/clear height) for a conditioned space. An unconditioned space gets `NaturalVentilationRate` and `NaturalVentilation`.

  The record itself is stored exactly in the `Semio_Authoring.Conditions` row as JSON.
- **Kelvin.** Temperatures are written in **kelvin**: the file declares no temperature unit, so the SI base unit applies.
- **Property names checked with ifcopenshell.** The 2x3 template of `Pset_SpaceThermalRequirements` has no `SpaceTemperature`, `HumidityMax` or `HumidityMin`. Both schemas type the ventilation rates as `IfcCountMeasure` (air changes).
- **`ThermalTransmittance`.**
  - Walls (`Pset_WallCommon`) and slabs (`Pset_SlabCommon`) take the area-weighted U of the envelope surfaces they hold.
  - Roofs (`Pset_RoofCommon`) do so **in IFC4 only**: the 2x3 template has no such property.
  - Windows (`Pset_WindowCommon`: `ThermalTransmittance`, `GlazingAreaFraction` = 1 − frame fraction; plus `Pset_DoorWindowGlazingType.SolarHeatGainTransmittance` = g, typed `IfcPositiveRatioMeasure` in 2x3 and `IfcNormalisedRatioMeasure` in IFC4) and doors (`Pset_DoorCommon.ThermalTransmittance`) take the values of their type.
- **Authored wins.** An authored property of the same name on the element (or on the window or door type) wins. Every derived row is listed in `Semio_Authoring.DerivedRows`, and the importer drops listed rows from the user properties. The house has authored `w-g-south` `Pset_WallCommon.ThermalTransmittance` = 0.18 and an authored window value on `o-g-living`, so this path is covered by the house itself.
- **Import (`S/🚪️io/📥️import/🏗️ifc/🔥️energy`).**
  - `space_conditions` come from the `Conditions` row; a foreign file falls back to the standard set (≥ 100 is read as kelvin, otherwise °C).
  - Window and door type data come from `UValue`/`GValue`/`FrameFraction`, else from the type's `Pset_WindowCommon`/`Pset_DoorCommon`/`Pset_DoorWindowGlazingType`, else (for a type made from an occurrence in `🧱️walls`) from the occurrence's sets.
- **Subject table.** `energy::thermal_report(schema, model)` lists the thermal sets of every product with IFC type and value, plus `derived`, `conditions` and the type rows.

### Oracles
- **gbXML.** `🌿️export-bim-1-gbxml/🐍️.py` (lxml 6.1.3, shapely 2.1.2, numpy 2.5.0) runs the structure, geometry, closure and duplicate-partition audits, plus the differential against the committed `energy-envelope` table (areas and U·A losses per group) for box, zoning and stack. If the XSD is committed it is used automatically.
- **IFC.** `🔥️export-bim-1-ifc-energy/🐍️.py` (ifcopenshell) checks the derived properties against the official templates (name and primary measure type), restates the rules from the snapshot (kelvin, air changes from `Qto_SpaceBaseQuantities.Height`, type values, ISO 6946 bounds of the layer stacks, authored kept and not listed, derived listed, no roof U in 2x3), and runs the EXPRESS rules.

## 4. XSD (not downloaded)

The official `GreenBuildingXML_Ver7.03.xsd` is not in the repo, not in `.venv`, and not under `C:\Users\Ueli`, `C:\git`, `C:\ProgramData` or `C:\Program Files` (one OpenStudio-for-Revit install contains only older gbXML 0.37 sample files, which I read for element order). I did **not** download it: downloading a file needs the user's explicit approval. Once someone fetches it (source: gbxml.org, schema 7.03), commit it as `S/🧫️fixtures/🌿️gbxml/📜️schema/GreenBuildingXML_Ver7.03.xsd`; the oracle then validates every file with `lxml.etree.XMLSchema`. Without it, the element order follows the order used by OpenStudio and Revit gbXML exports; whether 7.03 enforces an order is unverified.

## 5. Needs for the coordinator and the inference owner (all agreed in the contract rulings)
- `EnvelopeSurface.holder` / `.construction` (ruling 1) and `CurtainWallType.{u_value, g_value, frame_fraction}` with inferred curtain wall U (ruling 2) must exist before step 4; the gbXML test fixtures (box, zoning, stack) contain no curtain wall, the house does.
- **Editor job not written.** The editor command for a gbXML export job (pattern `SheetsExportWork` over `StagedGbxml`) is outside my scope (editor is owned by others).
