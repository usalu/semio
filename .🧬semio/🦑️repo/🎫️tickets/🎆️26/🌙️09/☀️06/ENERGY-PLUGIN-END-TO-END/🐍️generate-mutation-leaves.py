"""🏗️ Generator for `s.energy.model` semantic mutation leaves.

Emits, from the one spec table below, every file the scaffolding recipe requires for a mutation kind:
the leaf directory (`🔣️.json`, `🦀️.rs`, `🧬️.schema.json`, `🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs`), one
fixture-case directory per committed vector (`🦀️.rs` plus the five JSON files, seeded empty and then
materialized by the Rust `SEMIO_ENERGY_WRITE_FIXTURES=1` pass), the aggregate surfaces
(`🦀️.rs`/`🔣️.json`/`🟦️.ts`/`🔗️.graphql`/`🛰️.proto`/`📖️.grammar.semio`), the crate `#[path]` mounts,
the subset oracle catalog registration, the `🥒️.feature` example tables and the Python second
implementation.

Run from the repository root: `python3 <this file>`.
"""

from __future__ import annotations

import json
import os
import re
import unicodedata

ROOT = os.getcwd()
SUBSET = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = f"{SUBSET}/🧬️schema/🧬️mutations"
CRATE = "✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/🦀️.rs"
MOUNT_PREFIX = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"


def camel(slug: str) -> str:
    return "".join(part.capitalize() for part in slug.split("-"))


def lower_camel(name: str) -> str:
    return name[0].lower() + name[1:]


def snake(slug: str) -> str:
    return slug.replace("-", "_")


def field_camel(name: str) -> str:
    head, *rest = name.split("_")
    return head + "".join(part.capitalize() for part in rest)


JSON_TYPES = {
    "String": {"type": "string"},
    "f64": {"type": "number"},
    "u8": {"type": "integer"},
    "u16": {"type": "integer"},
    "u32": {"type": "integer"},
    "bool": {"type": "boolean"},
    "EntityId": {"type": "integer"},
    "crate::model::EntityId": {"type": "integer"},
    "Vec<f64>": {"type": "array", "items": {"type": "number"}},
    "Vec<u32>": {"type": "array", "items": {"type": "integer"}},
    "crate::model::OutputReportFrequency": {"enum": ["Timestep", "Hourly", "Daily", "Monthly", "RunPeriod"]},
    "crate::model::ScheduleId": {"type": "integer"},
    "Option<crate::model::ScheduleId>": {"type": ["integer", "null"]},
    "Option<f64>": {"type": ["number", "null"]},
    "Vec<crate::model::EntityId>": {"type": "array", "items": {"type": "integer"}},
    "crate::model::ZoneEquipmentType": {"enum": ["Baseboard", "Radiant", "FanCoil", "Ptac", "VrfTerminal", "Erv", "UnitHeater", "WaterToAirHp"]},
    "crate::model::PlantLoopType": {"enum": ["Heating", "Cooling", "Condenser"]},
    "crate::model::SizingType": {"enum": ["Heating", "Cooling", "OutdoorAir"]},
    "crate::model::DesignDayType": {"enum": ["Heating", "Cooling"]},
    "crate::model::RoomAirModelType": {"enum": ["WellMixed", "OneNodeDisplacement", "TwoNodeBuoyancy", "UnderFloorAirDistribution"]},
    "crate::model::SurfaceClass": {"enum": ["ExteriorWall", "InteriorWall", "Roof", "Ceiling", "Floor", "Interzone", "Adiabatic", "Ground"]},
    "crate::model::OutsideBoundaryKind": {"enum": ["OutdoorAir", "Ground", "OtherSideTemperature", "Adiabatic", "Interzone"]},
    "Vec<[f64; 3]>": {"type": "array", "items": {"type": "array", "items": {"type": "number"}, "minItems": 3, "maxItems": 3}},
    "Option<crate::model::EntityId>": {"type": ["integer", "null"]},
    "crate::air_exchange::InfiltrationMethod": {"enum": ["ScheduledAch", "PerExteriorArea", "EffectiveLeakageArea", "WindAndStack"]},
    "crate::model::FaultType": {"enum": ["SensorBias", "CoilFouling", "DamperStuck", "ChillerFouling", "BoilerEfficiencyDegradation"]},
    "Vec<crate::model::ScheduleId>": {"type": "array", "items": {"type": "integer"}},
    "crate::schedule::ScheduleInterpolation": {"enum": ["Continuous", "Discrete"]},
}

PROTO_TYPES = {
    "String": "string",
    "f64": "double",
    "u8": "uint32",
    "u16": "uint32",
    "u32": "uint32",
    "bool": "bool",
    "EntityId": "uint32",
    "crate::model::EntityId": "uint32",
    "Vec<f64>": "repeated double",
    "Vec<u32>": "repeated uint32",
    "crate::model::OutputReportFrequency": "string",
    "crate::model::ScheduleId": "uint32",
    "Option<crate::model::ScheduleId>": "optional uint32",
    "Option<f64>": "optional double",
    "Vec<crate::model::EntityId>": "repeated uint32",
    "crate::model::ZoneEquipmentType": "string",
    "crate::model::PlantLoopType": "string",
    "crate::model::SizingType": "string",
    "crate::model::DesignDayType": "string",
    "crate::model::RoomAirModelType": "string",
    "crate::model::SurfaceClass": "string",
    "crate::model::OutsideBoundaryKind": "string",
    "Vec<[f64; 3]>": "repeated Vertex3",
    "Option<crate::model::EntityId>": "optional uint32",
    "crate::air_exchange::InfiltrationMethod": "string",
    "crate::model::FaultType": "string",
    "Vec<crate::model::ScheduleId>": "repeated uint32",
    "crate::schedule::ScheduleInterpolation": "string",
}

TS_TYPES = {
    "String": "string",
    "f64": "number",
    "u8": "number",
    "u16": "number",
    "u32": "number",
    "bool": "boolean",
    "EntityId": "number",
    "crate::model::EntityId": "number",
    "Vec<f64>": "readonly number[]",
    "Vec<u32>": "readonly number[]",
    "crate::model::OutputReportFrequency": '"Timestep" | "Hourly" | "Daily" | "Monthly" | "RunPeriod"',
    "crate::model::ScheduleId": "number",
    "Option<crate::model::ScheduleId>": "number | null",
    "Option<f64>": "number | null",
    "Vec<crate::model::EntityId>": "readonly number[]",
    "crate::model::ZoneEquipmentType": '"Baseboard" | "Radiant" | "FanCoil" | "Ptac" | "VrfTerminal" | "Erv" | "UnitHeater" | "WaterToAirHp"',
    "crate::model::PlantLoopType": '"Heating" | "Cooling" | "Condenser"',
    "crate::model::SizingType": '"Heating" | "Cooling" | "OutdoorAir"',
    "crate::model::DesignDayType": '"Heating" | "Cooling"',
    "crate::model::RoomAirModelType": '"WellMixed" | "OneNodeDisplacement" | "TwoNodeBuoyancy" | "UnderFloorAirDistribution"',
    "crate::model::SurfaceClass": '"ExteriorWall" | "InteriorWall" | "Roof" | "Ceiling" | "Floor" | "Interzone" | "Adiabatic" | "Ground"',
    "crate::model::OutsideBoundaryKind": '"OutdoorAir" | "Ground" | "OtherSideTemperature" | "Adiabatic" | "Interzone"',
    "Vec<[f64; 3]>": "readonly (readonly [number, number, number])[]",
    "Option<crate::model::EntityId>": "number | null",
    "crate::air_exchange::InfiltrationMethod": '"ScheduledAch" | "PerExteriorArea" | "EffectiveLeakageArea" | "WindAndStack"',
    "crate::model::FaultType": '"SensorBias" | "CoilFouling" | "DamperStuck" | "ChillerFouling" | "BoilerEfficiencyDegradation"',
    "Vec<crate::model::ScheduleId>": "readonly number[]",
    "crate::schedule::ScheduleInterpolation": '"Continuous" | "Discrete"',
}

GRAPHQL_TYPES = {
    "String": "String!",
    "f64": "Float!",
    "u8": "Int!",
    "u16": "Int!",
    "u32": "Int!",
    "bool": "Boolean!",
    "EntityId": "Int!",
    "crate::model::EntityId": "Int!",
    "Vec<f64>": "[Float!]!",
    "Vec<u32>": "[Int!]!",
    "crate::model::OutputReportFrequency": "String!",
    "crate::model::ScheduleId": "Int!",
    "Option<crate::model::ScheduleId>": "Int",
    "Option<f64>": "Float",
    "Vec<crate::model::EntityId>": "[Int!]!",
    "crate::model::ZoneEquipmentType": "String!",
    "crate::model::PlantLoopType": "String!",
    "crate::model::SizingType": "String!",
    "crate::model::DesignDayType": "String!",
    "crate::model::RoomAirModelType": "String!",
    "crate::model::SurfaceClass": "String!",
    "crate::model::OutsideBoundaryKind": "String!",
    "Vec<[f64; 3]>": "[[Float!]!]!",
    "Option<crate::model::EntityId>": "Int",
    "crate::air_exchange::InfiltrationMethod": "String!",
    "crate::model::FaultType": "String!",
    "Vec<crate::model::ScheduleId>": "[Int!]!",
    "crate::schedule::ScheduleInterpolation": "String!",
}


class Kind:
    def __init__(self, number, slug, emoji, verb, entity, record, display, doc, fields, label, target, diff, inverse, cases, outcome_classes, probe=None, python=None, python_invert=None):
        self.probe = probe
        self.python = python
        self.python_invert = python_invert
        self.number = number
        self.slug = slug
        self.emoji = emoji
        self.verb = verb
        self.entity = entity
        self.record = record
        self.display = display
        self.doc = doc
        self.fields = fields
        self.label = label
        self.target = target
        self.diff = diff
        self.inverse = inverse
        self.cases = cases
        self.outcome_classes = outcome_classes

    @property
    def variant(self):
        return camel(self.slug)

    @property
    def module(self):
        return snake(self.slug)

    @property
    def dir(self):
        return f"{self.emoji}{self.slug}"

    @property
    def wire_tag(self):
        return lower_camel(self.variant)


D = "crate::artifacts::model::schema::diff::text::diff_from_model"

KINDS: list[Kind] = []


def kind(**kw):
    """🧮 Registers one kind, keeping `KINDS` in ledger-number order no matter which group's
    region declares it. The enum body, `KINDS`, `DIRECTORIES` and the wire probes are all emitted in
    this order, and that order IS the binary ordinal, so declaration order must never decide it."""
    entry = Kind(**kw)
    position = next((index for index, existing in enumerate(KINDS) if existing.number > entry.number), len(KINDS))
    KINDS.insert(position, entry)


# region 🔖️ModelRoot
kind(
    number=1,
    slug="rename-model",
    emoji="🏷️",
    verb="rename",
    entity="model",
    record="RenamedModel",
    display="Rename Model",
    doc="Sets the model's identity field.",
    fields=[("new_name", "String")],
    label='format!("Rename energy model to \\"{}\\"", self.new_name)',
    target=None,
    diff='''    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "An energy model name must not be blank.", [payload.new_name.clone()]);
    }
    if base.model.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("The energy model is already named \\"{}\\".", payload.new_name));
    }
    let mut model = base.model.clone();
    model.name = payload.new_name.clone();
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    if payload.new_name.trim().is_empty() || base.model.name == payload.new_name {
        return Vec::new();
    }
    vec![super::rename_model(base.model.name.clone())]''',
    outcome_classes=["applied", "warning", "error"],
    cases=[
        ("✅️", "renames-the-model", "renames the demo model", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), version: "1".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    (snapshot(model), super::rename_model("BESTEST 600FF".into()))'''),
        ("⛔️", "refuses-a-blank-name", "refuses a blank name", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::rename_model("   ".into()))'''),
    ],
)

kind(
    number=2,
    slug="change-model-version",
    emoji="🔢️",
    verb="change",
    entity="model",
    record="ChangedModelVersion",
    display="Change Model Version",
    doc="Sets the model document's own version string.",
    fields=[("new_version", "String")],
    label='format!("Change energy model version to \\"{}\\"", self.new_version)',
    target=None,
    diff='''    if payload.new_version.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "An energy model version must not be blank.", [payload.new_version.clone()]);
    }
    if base.model.version == payload.new_version {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("The energy model version is already \\"{}\\".", payload.new_version));
    }
    let mut model = base.model.clone();
    model.version = payload.new_version.clone();
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    if payload.new_version.trim().is_empty() || base.model.version == payload.new_version {
        return Vec::new();
    }
    vec![super::change_model_version(base.model.version.clone())]''',
    outcome_classes=["applied", "warning", "error"],
    cases=[
        ("✅️", "bumps-the-version", "bumps the version to 2", '''    let model = crate::model::Model { name: "BESTEST 600".into(), version: "1".into(), ..crate::model::Model::default() };
    (snapshot(model), super::change_model_version("2".into()))'''),
        ("⛔️", "refuses-a-blank-version", "refuses a blank version", '''    let model = crate::model::Model { name: "BESTEST 600".into(), version: "1".into(), ..crate::model::Model::default() };
    (snapshot(model), super::change_model_version(String::new()))'''),
    ],
)

kind(
    number=3,
    slug="update-site",
    emoji="🌍️",
    verb="update",
    entity="site",
    record="UpdatedSite",
    display="Update Site",
    doc="Sets the whole inseparable site facet — latitude, longitude, elevation, time zone and north axis are validated and consumed together by the solar geometry, so none of them is meaningfully set on its own.",
    fields=[("latitude_deg", "f64"), ("longitude_deg", "f64"), ("elevation_m", "f64"), ("time_zone_hours", "f64"), ("north_axis_deg", "f64")],
    label='format!("Update site to {:.4}, {:.4}", self.latitude_deg, self.longitude_deg)',
    target=None,
    diff='''    let site = crate::model::Site { latitude_deg: payload.latitude_deg, longitude_deg: payload.longitude_deg, elevation_m: payload.elevation_m, time_zone_hours: payload.time_zone_hours, north_axis_deg: payload.north_axis_deg };
    if !(-90.0..=90.0).contains(&site.latitude_deg) || !(-180.0..=180.0).contains(&site.longitude_deg) || !(-12.0..=14.0).contains(&site.time_zone_hours) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Site {:.4}, {:.4} at UTC{:+} is outside the representable earth.", site.latitude_deg, site.longitude_deg, site.time_zone_hours), Vec::<String>::new());
    }
    if base.model.site == site {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The site facet already has this value.");
    }
    let mut model = base.model.clone();
    model.site = site;
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    let site = crate::model::Site { latitude_deg: payload.latitude_deg, longitude_deg: payload.longitude_deg, elevation_m: payload.elevation_m, time_zone_hours: payload.time_zone_hours, north_axis_deg: payload.north_axis_deg };
    if base.model.site == site {
        return Vec::new();
    }
    let old = base.model.site;
    vec![super::update_site(old.latitude_deg, old.longitude_deg, old.elevation_m, old.time_zone_hours, old.north_axis_deg)]''',
    outcome_classes=["applied", "warning", "error"],
    cases=[
        ("✅️", "relocates-to-denver", "relocates the model to Denver", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::update_site(39.833, -104.65, 1650.0, -7.0, 0.0))'''),
        ("⛔️", "refuses-a-bad-latitude", "refuses an impossible latitude", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::update_site(123.0, -104.65, 1650.0, -7.0, 0.0))'''),
    ],
)

kind(
    number=4,
    slug="update-ground-temperature",
    emoji="🌡️",
    verb="update",
    entity="ground-temperature",
    record="UpdatedGroundTemperature",
    display="Update Ground Temperature",
    doc="Sets the whole inseparable ground-temperature facet — twelve monthly building-surface values, twelve monthly shallow values and one deep value are read together by the ground heat transfer solve.",
    fields=[("building_surface_c", "Vec<f64>"), ("shallow_c", "Vec<f64>"), ("deep_c", "f64")],
    label='"Update ground temperatures".to_string()',
    target=None,
    diff='''    if payload.building_surface_c.len() != 12 || payload.shallow_c.len() != 12 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Ground temperatures need twelve monthly values each, got {} building-surface and {} shallow.", payload.building_surface_c.len(), payload.shallow_c.len()), Vec::<String>::new());
    }
    let mut building_surface_c = [0.0f64; 12];
    let mut shallow_c = [0.0f64; 12];
    building_surface_c.copy_from_slice(&payload.building_surface_c);
    shallow_c.copy_from_slice(&payload.shallow_c);
    let ground = crate::model::GroundTemperatureConfig { building_surface_c, shallow_c, deep_c: payload.deep_c };
    if base.model.ground_temperature == ground {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The ground-temperature facet already has this value.");
    }
    let mut model = base.model.clone();
    model.ground_temperature = ground;
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    if payload.building_surface_c.len() != 12 || payload.shallow_c.len() != 12 {
        return Vec::new();
    }
    let old = base.model.ground_temperature.clone();
    if old.deep_c == payload.deep_c && old.building_surface_c.as_slice() == payload.building_surface_c.as_slice() && old.shallow_c.as_slice() == payload.shallow_c.as_slice() {
        return Vec::new();
    }
    vec![super::update_ground_temperature(old.building_surface_c.to_vec(), old.shallow_c.to_vec(), old.deep_c)]''',
    outcome_classes=["applied", "warning", "error"],
    cases=[
        ("✅️", "sets-denver-ground", "sets Denver monthly ground temperatures", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::update_ground_temperature(vec![1.0, 2.0, 5.0, 9.0, 14.0, 19.0, 22.0, 21.0, 17.0, 11.0, 5.0, 1.0], vec![4.0; 12], 10.0))'''),
        ("⛔️", "refuses-a-short-year", "refuses a ten-month year", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::update_ground_temperature(vec![4.0; 10], vec![4.0; 12], 10.0))'''),
    ],
)

kind(
    number=5,
    slug="update-run-period",
    emoji="📅️",
    verb="update",
    entity="run-period",
    record="UpdatedRunPeriod",
    display="Update Run Period",
    doc="Sets the whole inseparable run-period facet the simulation kernel reads out of the model. Start and end are one calendar interval: setting either alone can name an interval that does not exist.",
    fields=[("start_month", "u8"), ("start_day", "u8"), ("end_month", "u8"), ("end_day", "u8"), ("year", "u16")],
    label='format!("Update run period to {}-{} .. {}-{}", self.start_month, self.start_day, self.end_month, self.end_day)',
    target=None,
    diff='''    let run_period = crate::calendar::RunPeriod { start_month: payload.start_month, start_day: payload.start_day, end_month: payload.end_month, end_day: payload.end_day, year: payload.year };
    if !(1..=12).contains(&run_period.start_month) || !(1..=12).contains(&run_period.end_month) || !(1..=31).contains(&run_period.start_day) || !(1..=31).contains(&run_period.end_day) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Run period {}-{} .. {}-{} is not a calendar interval.", run_period.start_month, run_period.start_day, run_period.end_month, run_period.end_day), Vec::<String>::new());
    }
    if base.model.run_period == run_period {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The run period already has this value.");
    }
    let mut model = base.model.clone();
    model.run_period = run_period;
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    let run_period = crate::calendar::RunPeriod { start_month: payload.start_month, start_day: payload.start_day, end_month: payload.end_month, end_day: payload.end_day, year: payload.year };
    if base.model.run_period == run_period {
        return Vec::new();
    }
    let old = base.model.run_period;
    vec![super::update_run_period(old.start_month, old.start_day, old.end_month, old.end_day, old.year)]''',
    outcome_classes=["applied", "warning", "error"],
    cases=[
        ("✅️", "shortens-to-january", "shortens the run period to January", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::update_run_period(1, 1, 1, 31, 2026))'''),
        ("⛔️", "refuses-month-13", "refuses a thirteenth month", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::update_run_period(1, 1, 13, 31, 2026))'''),
    ],
)

kind(
    number=6,
    slug="replace-airflow-network",
    emoji="🫧️",
    verb="replace",
    entity="airflow-network",
    record="ReplacedAirflowNetwork",
    display="Replace Airflow Network",
    doc="Swaps the document-root airflow-network singleton whole. `present` false detaches it, so one kind covers both attach and detach. `zone_ids[i]` pairs with `node_ids[i]`: the payload carries the two halves of `AirflowNetworkDefinition::zone_node_ids` as parallel lists because its tuple element type has no `dsl::DslField` (see the ticket ledger's follow-up note).",
    fields=[("present", "bool"), ("zone_ids", "Vec<u32>"), ("node_ids", "Vec<u32>"), ("outdoor_node_id", "u32"), ("link_ids", "Vec<u32>")],
    label='if self.present { format!("Replace airflow network with {} zone nodes", self.zone_ids.len()) } else { "Detach the airflow network".to_string() }',
    target=None,
    diff='''    if payload.zone_ids.len() != payload.node_ids.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("An airflow network pairs one node id per zone id, got {} zone ids and {} node ids.", payload.zone_ids.len(), payload.node_ids.len()), Vec::<String>::new());
    }
    if !payload.present && !(payload.zone_ids.is_empty() && payload.link_ids.is_empty()) {
        return protocol::MutationOutcome::error("mutation.invariant", "A detached airflow network carries no zone nodes and no links.", Vec::<String>::new());
    }
    let network = payload.present.then(|| crate::model::AirflowNetworkDefinition {
        zone_node_ids: payload.zone_ids.iter().copied().zip(payload.node_ids.iter().copied()).map(|(zone, node)| (crate::model::EntityId(zone), node)).collect(),
        outdoor_node_id: payload.outdoor_node_id,
        link_ids: payload.link_ids.clone(),
    });
    if base.model.airflow_network == network {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The airflow network already has this value.");
    }
    let mut model = base.model.clone();
    model.airflow_network = network;
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    if payload.zone_ids.len() != payload.node_ids.len() || (!payload.present && !(payload.zone_ids.is_empty() && payload.link_ids.is_empty())) {
        return Vec::new();
    }
    match &base.model.airflow_network {
        Some(old) => vec![super::replace_airflow_network(true, old.zone_node_ids.iter().map(|(zone, _)| zone.0).collect(), old.zone_node_ids.iter().map(|(_, node)| *node).collect(), old.outdoor_node_id, old.link_ids.clone())],
        None => vec![super::replace_airflow_network(false, Vec::new(), Vec::new(), 0, Vec::new())],
    }''',
    outcome_classes=["applied", "warning", "error"],
    cases=[
        ("✅️", "attaches-a-network", "attaches a one-zone airflow network", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    (snapshot(model), super::replace_airflow_network(true, vec![1], vec![1], 0, vec![7]))'''),
        ("⛔️", "refuses-unpaired-nodes", "refuses unpaired zone and node ids", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::replace_airflow_network(true, vec![1, 2], vec![1], 0, Vec::new()))'''),
    ],
)

kind(
    number=7,
    slug="add-output-variable",
    emoji="📊️",
    verb="add",
    entity="output-variable",
    record="AddedOutputVariable",
    display="Add Output Variable",
    doc="Attaches one reporting registration to the document-root output-variable set. `(name, key)` is the natural composite key — `OutputVariableSpec` carries no id — so a duplicate registration is refused rather than silently doubled.",
    fields=[("name", "String"), ("key", "String"), ("reporting_frequency", "crate::model::OutputReportFrequency")],
    label='format!("Add output variable \\"{}\\" for \\"{}\\"", self.name, self.key)',
    target='vec![self.name.clone(), self.key.clone()]',
    diff='''    if payload.name.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "An output variable name must not be blank.", [payload.key.clone()]);
    }
    if base.model.output_variables.iter().any(|spec| spec.name == payload.name && spec.key == payload.key) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Output variable \\"{}\\" is already registered for \\"{}\\".", payload.name, payload.key), [payload.name.clone(), payload.key.clone()]);
    }
    let mut model = base.model.clone();
    model.output_variables.push(crate::model::OutputVariableSpec { name: payload.name.clone(), key: payload.key.clone(), reporting_frequency: payload.reporting_frequency });
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    if payload.name.trim().is_empty() || base.model.output_variables.iter().any(|spec| spec.name == payload.name && spec.key == payload.key) {
        return Vec::new();
    }
    vec![super::remove_output_variable(payload.name.clone(), payload.key.clone())]''',
    outcome_classes=["applied", "error"],
    cases=[
        ("✅️", "adds-zone-air-temp", "adds the zone mean air temperature report", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::add_output_variable("Zone Mean Air Temperature".into(), "ZONE ONE".into(), crate::model::OutputReportFrequency::Hourly))'''),
        ("⛔️", "refuses-a-duplicate", "refuses a duplicate registration", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.output_variables.push(crate::model::OutputVariableSpec { name: "Zone Mean Air Temperature".into(), key: "ZONE ONE".into(), reporting_frequency: crate::model::OutputReportFrequency::Hourly });
    (snapshot(model), super::add_output_variable("Zone Mean Air Temperature".into(), "ZONE ONE".into(), crate::model::OutputReportFrequency::Daily))'''),
    ],
)

kind(
    number=8,
    slug="remove-output-variable",
    emoji="📉️",
    verb="remove",
    entity="output-variable",
    record="RemovedOutputVariable",
    display="Remove Output Variable",
    doc="Detaches one reporting registration addressed by its `(name, key)` natural composite key.",
    fields=[("name", "String"), ("key", "String")],
    label='format!("Remove output variable \\"{}\\" for \\"{}\\"", self.name, self.key)',
    target='vec![self.name.clone(), self.key.clone()]',
    diff='''    let Some(existing) = base.model.output_variables.iter().find(|spec| spec.name == payload.name && spec.key == payload.key) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Output variable \\"{}\\" is not registered for \\"{}\\".", payload.name, payload.key), [payload.name.clone(), payload.key.clone()]);
    };
    let _ = existing;
    let mut model = base.model.clone();
    model.output_variables.retain(|spec| !(spec.name == payload.name && spec.key == payload.key));
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    match base.model.output_variables.iter().find(|spec| spec.name == payload.name && spec.key == payload.key) {
        Some(spec) => vec![super::add_output_variable(spec.name.clone(), spec.key.clone(), spec.reporting_frequency)],
        None => Vec::new(),
    }''',
    outcome_classes=["applied", "error"],
    cases=[
        ("✅️", "drops-zone-air-temp", "drops the zone mean air temperature report", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.output_variables.push(crate::model::OutputVariableSpec { name: "Zone Mean Air Temperature".into(), key: "ZONE ONE".into(), reporting_frequency: crate::model::OutputReportFrequency::Hourly });
    (snapshot(model), super::remove_output_variable("Zone Mean Air Temperature".into(), "ZONE ONE".into()))'''),
        ("⛔️", "refuses-an-absent-one", "refuses an unregistered variable", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::remove_output_variable("Zone Mean Air Temperature".into(), "ZONE ONE".into()))'''),
    ],
)

kind(
    number=9,
    slug="bind-weather-file",
    emoji="🌦️",
    verb="bind",
    entity="weather-file",
    record="BoundWeatherFile",
    display="Bind Weather File",
    doc="Attaches the `weather` link slot to an `🌦️epw` stdio artifact addressed by its `ArtifactRef` URI. The link is pinned to the target's head; the model never embeds weather bytes.",
    fields=[("target_uri", "String")],
    label='format!("Bind weather file {}", self.target_uri)',
    target='vec![self.target_uri.clone()]',
    diff='''    let Ok(target) = store::os_io::ArtifactRef::parse_uri(&payload.target_uri) else {
        return protocol::MutationOutcome::error("mutation.invariant", format!("{:?} is not an artifact reference URI.", payload.target_uri), [payload.target_uri.clone()]);
    };
    let link = store::ArtifactLink { target, pin: store::LinkPin::Head, role: super::WEATHER_LINK_ROLE.to_string() };
    if base.weather_link.as_ref() == Some(&link) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("The weather file is already bound to {}.", payload.target_uri));
    }
    protocol::MutationOutcome::new(EnergyModelDiff { weather_link: Some(EnergyLinkSlotDelta::Attached { link }), ..Default::default() })''',
    inverse='''    let Ok(target) = store::os_io::ArtifactRef::parse_uri(&payload.target_uri) else {
        return Vec::new();
    };
    let link = store::ArtifactLink { target, pin: store::LinkPin::Head, role: super::WEATHER_LINK_ROLE.to_string() };
    match &base.weather_link {
        Some(existing) if existing == &link => Vec::new(),
        Some(existing) => vec![super::bind_weather_file(existing.target.to_uri())],
        None => vec![super::unbind_weather_file()],
    }''',
    outcome_classes=["applied", "warning", "error"],
    cases=[
        ("✅️", "binds-hannover-epw", "binds the bundled Hannover weather file", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::bind_weather_file("hannover!s.stdio.semio@v1/epw".into()))'''),
        ("⛔️", "refuses-a-bad-uri", "refuses a malformed artifact URI", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::bind_weather_file("not a uri".into()))'''),
    ],
)

kind(
    number=10,
    slug="unbind-weather-file",
    emoji="🌤️",
    verb="unbind",
    entity="weather-file",
    record="UnboundWeatherFile",
    display="Unbind Weather File",
    doc="Detaches the `weather` link slot. Refused when nothing is bound, so an undo chain can never invent an unbind that had no partner.",
    fields=[],
    label='"Unbind the weather file".to_string()',
    target=None,
    diff='''    let Some(existing) = &base.weather_link else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No weather file is bound to this energy model.", Vec::<String>::new());
    };
    let _ = existing;
    protocol::MutationOutcome::new(EnergyModelDiff { weather_link: Some(EnergyLinkSlotDelta::Detached), ..Default::default() })''',
    inverse='''    let _ = payload;
    match &base.weather_link {
        Some(existing) => vec![super::bind_weather_file(existing.target.to_uri())],
        None => Vec::new(),
    }''',
    outcome_classes=["applied", "error"],
    cases=[
        ("✅️", "unbinds-the-weather", "unbinds a bound weather file", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    let mut before = snapshot(model);
    before.weather_link = Some(link("hannover!s.stdio.semio@v1/epw", "weather"));
    (before, super::unbind_weather_file())'''),
        ("⛔️", "refuses-when-unbound", "refuses when nothing is bound", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::unbind_weather_file())'''),
    ],
)

kind(
    number=11,
    slug="connect-referenced-model",
    emoji="🪢️",
    verb="connect",
    entity="referenced-model",
    record="ConnectedReferencedModel",
    display="Connect Referenced Model",
    doc="Creates the relationship between this energy model and the geometry model it was derived from, addressed by the target's `ArtifactRef` URI.",
    fields=[("target_uri", "String")],
    label='format!("Connect referenced model {}", self.target_uri)',
    target='vec![self.target_uri.clone()]',
    diff='''    let Ok(target) = store::os_io::ArtifactRef::parse_uri(&payload.target_uri) else {
        return protocol::MutationOutcome::error("mutation.invariant", format!("{:?} is not an artifact reference URI.", payload.target_uri), [payload.target_uri.clone()]);
    };
    let link = store::ArtifactLink { target, pin: store::LinkPin::Head, role: super::REFERENCED_MODEL_LINK_ROLE.to_string() };
    if base.referenced_model.as_ref() == Some(&link) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("The referenced model is already {}.", payload.target_uri));
    }
    protocol::MutationOutcome::new(EnergyModelDiff { referenced_model: Some(EnergyLinkSlotDelta::Attached { link }), ..Default::default() })''',
    inverse='''    let Ok(target) = store::os_io::ArtifactRef::parse_uri(&payload.target_uri) else {
        return Vec::new();
    };
    let link = store::ArtifactLink { target, pin: store::LinkPin::Head, role: super::REFERENCED_MODEL_LINK_ROLE.to_string() };
    match &base.referenced_model {
        Some(existing) if existing == &link => Vec::new(),
        Some(existing) => vec![super::connect_referenced_model(existing.target.to_uri())],
        None => vec![super::disconnect_referenced_model()],
    }''',
    outcome_classes=["applied", "warning", "error"],
    cases=[
        ("✅️", "connects-the-geometry", "connects the source geometry model", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::connect_referenced_model("doc-2!s.stdio.semio@v1/model".into()))'''),
        ("⛔️", "refuses-a-bad-uri", "refuses a malformed artifact URI", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::connect_referenced_model(String::new()))'''),
    ],
)

kind(
    number=12,
    slug="disconnect-referenced-model",
    emoji="✂️",
    verb="disconnect",
    entity="referenced-model",
    record="DisconnectedReferencedModel",
    display="Disconnect Referenced Model",
    doc="Removes the relationship to the geometry model. Refused when no relationship exists.",
    fields=[],
    label='"Disconnect the referenced model".to_string()',
    target=None,
    diff='''    let Some(existing) = &base.referenced_model else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No referenced model is connected to this energy model.", Vec::<String>::new());
    };
    let _ = existing;
    protocol::MutationOutcome::new(EnergyModelDiff { referenced_model: Some(EnergyLinkSlotDelta::Detached), ..Default::default() })''',
    inverse='''    let _ = payload;
    match &base.referenced_model {
        Some(existing) => vec![super::connect_referenced_model(existing.target.to_uri())],
        None => Vec::new(),
    }''',
    outcome_classes=["applied", "error"],
    cases=[
        ("✅️", "disconnects-the-geometry", "disconnects the source geometry model", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    let mut before = snapshot(model);
    before.referenced_model = Some(link("doc-2!s.stdio.semio@v1/model", "model"));
    (before, super::disconnect_referenced_model())'''),
        ("⛔️", "refuses-when-absent", "refuses when nothing is connected", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::disconnect_referenced_model())'''),
    ],
)
# endregion 🔖️ModelRoot


# region 🔖️Zones
ZONE_MISSING = '''    let Some(existing) = base.model.zones.iter().find(|zone| zone.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };'''

kind(
    number=100,
    slug="rename-zone",
    emoji="🏠️",
    verb="rename",
    entity="zone",
    record="RenamedZone",
    display="Rename Zone",
    doc="Sets one zone's identity field. Zone names are the key every EnergyPlus-class report keys on, so a duplicate is refused.",
    fields=[("id", "crate::model::EntityId"), ("new_name", "String")],
    label='format!("Rename zone {} to \\"{}\\"", self.id.0, self.new_name)',
    target='vec![self.id.0.to_string()]',
    diff=ZONE_MISSING + '''
    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "A zone name must not be blank.", [payload.id.0.to_string()]);
    }
    if base.model.zones.iter().any(|zone| zone.id != payload.id && zone.name == payload.new_name) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Another zone is already named \\"{}\\".", payload.new_name), [payload.id.0.to_string()]);
    }
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Zone {} is already named \\"{}\\".", payload.id.0, payload.new_name));
    }
    let mut model = base.model.clone();
    if let Some(zone) = model.zones.iter_mut().find(|zone| zone.id == payload.id) {
        zone.name = payload.new_name.clone();
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    match base.model.zones.iter().find(|zone| zone.id == payload.id) {
        Some(zone) if zone.name != payload.new_name && !payload.new_name.trim().is_empty() => vec![super::rename_zone(payload.id, zone.name.clone())],
        _ => Vec::new(),
    }''',
    outcome_classes=["applied", "warning", "error"],
    cases=[
        ("✅️", "renames-zone-one", "renames the first zone", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    (snapshot(model), super::rename_zone(crate::model::EntityId(1), "ZONE 1".into()))'''),
        ("⛔️", "refuses-a-missing-zone", "refuses an absent zone", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::rename_zone(crate::model::EntityId(9), "ZONE 9".into()))'''),
    ],
)

kind(
    number=101,
    slug="change-zone-volume",
    emoji="📦️",
    verb="change",
    entity="zone",
    record="ChangedZoneVolume",
    display="Change Zone Volume",
    doc="Sets one zone's air volume in cubic metres — the capacitance the zone air heat balance integrates over.",
    fields=[("id", "crate::model::EntityId"), ("new_volume_m3", "f64")],
    label='format!("Change zone {} volume to {} m³", self.id.0, self.new_volume_m3)',
    target='vec![self.id.0.to_string()]',
    diff=ZONE_MISSING + '''
    if !payload.new_volume_m3.is_finite() || payload.new_volume_m3 <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Zone {} needs a positive finite volume, got {}.", payload.id.0, payload.new_volume_m3), [payload.id.0.to_string()]);
    }
    if existing.volume_m3 == payload.new_volume_m3 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Zone {} already has volume {} m³.", payload.id.0, payload.new_volume_m3));
    }
    let mut model = base.model.clone();
    if let Some(zone) = model.zones.iter_mut().find(|zone| zone.id == payload.id) {
        zone.volume_m3 = payload.new_volume_m3;
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    match base.model.zones.iter().find(|zone| zone.id == payload.id) {
        Some(zone) if zone.volume_m3 != payload.new_volume_m3 && payload.new_volume_m3.is_finite() && payload.new_volume_m3 > 0.0 => vec![super::change_zone_volume(payload.id, zone.volume_m3)],
        _ => Vec::new(),
    }''',
    outcome_classes=["applied", "warning", "error"],
    cases=[
        ("✅️", "resizes-zone-one", "resizes the first zone", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    (snapshot(model), super::change_zone_volume(crate::model::EntityId(1), 129.6))'''),
        ("⛔️", "refuses-zero-volume", "refuses a zero volume", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    (snapshot(model), super::change_zone_volume(crate::model::EntityId(1), 0.0))'''),
    ],
)

kind(
    number=102,
    slug="change-zone-multiplier",
    emoji="✖️",
    verb="change",
    entity="zone",
    record="ChangedZoneMultiplier",
    display="Change Zone Multiplier",
    doc="Sets how many identical instances of the zone the results are scaled by.",
    fields=[("id", "crate::model::EntityId"), ("new_multiplier", "u32")],
    label='format!("Change zone {} multiplier to {}", self.id.0, self.new_multiplier)',
    target='vec![self.id.0.to_string()]',
    diff=ZONE_MISSING + '''
    if payload.new_multiplier == 0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Zone {} needs at least one instance.", payload.id.0), [payload.id.0.to_string()]);
    }
    if existing.multiplier == payload.new_multiplier {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Zone {} already has multiplier {}.", payload.id.0, payload.new_multiplier));
    }
    let mut model = base.model.clone();
    if let Some(zone) = model.zones.iter_mut().find(|zone| zone.id == payload.id) {
        zone.multiplier = payload.new_multiplier;
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    match base.model.zones.iter().find(|zone| zone.id == payload.id) {
        Some(zone) if zone.multiplier != payload.new_multiplier && payload.new_multiplier != 0 => vec![super::change_zone_multiplier(payload.id, zone.multiplier)],
        _ => Vec::new(),
    }''',
    outcome_classes=["applied", "warning", "error"],
    cases=[
        ("✅️", "stacks-four-storeys", "stacks four identical storeys", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    (snapshot(model), super::change_zone_multiplier(crate::model::EntityId(1), 4))'''),
        ("⛔️", "refuses-zero-instances", "refuses zero instances", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    (snapshot(model), super::change_zone_multiplier(crate::model::EntityId(1), 0))'''),
    ],
)

kind(
    number=103,
    slug="change-zone-conditioned",
    emoji="🌬️",
    verb="change",
    entity="zone",
    record="ChangedZoneConditioned",
    display="Change Zone Conditioned",
    doc="Sets whether the zone is served by heating or cooling equipment at all.",
    fields=[("id", "crate::model::EntityId"), ("new_conditioned", "bool")],
    label='format!("Change zone {} conditioned to {}", self.id.0, self.new_conditioned)',
    target='vec![self.id.0.to_string()]',
    diff=ZONE_MISSING + '''
    if existing.conditioned == payload.new_conditioned {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Zone {} is already conditioned={}.", payload.id.0, payload.new_conditioned));
    }
    let mut model = base.model.clone();
    if let Some(zone) = model.zones.iter_mut().find(|zone| zone.id == payload.id) {
        zone.conditioned = payload.new_conditioned;
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    match base.model.zones.iter().find(|zone| zone.id == payload.id) {
        Some(zone) if zone.conditioned != payload.new_conditioned => vec![super::change_zone_conditioned(payload.id, zone.conditioned)],
        _ => Vec::new(),
    }''',
    outcome_classes=["applied", "warning", "error"],
    cases=[
        ("✅️", "frees-the-zone", "turns the zone free-floating", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    (snapshot(model), super::change_zone_conditioned(crate::model::EntityId(1), false))'''),
        ("⛔️", "refuses-a-missing-zone", "refuses an absent zone", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::change_zone_conditioned(crate::model::EntityId(9), false))'''),
    ],
)

kind(
    number=104,
    slug="change-zone-floor-area-participation",
    emoji="📐️",
    verb="change",
    entity="zone",
    record="ChangedZoneFloorAreaParticipation",
    display="Change Zone Floor Area Participation",
    doc="Sets whether the zone's floor area counts toward the building total the normalized reports divide by.",
    fields=[("id", "crate::model::EntityId"), ("new_part_of_total_floor_area", "bool")],
    label='format!("Change zone {} floor-area participation to {}", self.id.0, self.new_part_of_total_floor_area)',
    target='vec![self.id.0.to_string()]',
    diff=ZONE_MISSING + '''
    if existing.part_of_total_floor_area == payload.new_part_of_total_floor_area {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Zone {} already has partOfTotalFloorArea={}.", payload.id.0, payload.new_part_of_total_floor_area));
    }
    let mut model = base.model.clone();
    if let Some(zone) = model.zones.iter_mut().find(|zone| zone.id == payload.id) {
        zone.part_of_total_floor_area = payload.new_part_of_total_floor_area;
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    match base.model.zones.iter().find(|zone| zone.id == payload.id) {
        Some(zone) if zone.part_of_total_floor_area != payload.new_part_of_total_floor_area => vec![super::change_zone_floor_area_participation(payload.id, zone.part_of_total_floor_area)],
        _ => Vec::new(),
    }''',
    outcome_classes=["applied", "warning", "error"],
    cases=[
        ("✅️", "excludes-the-zone", "excludes the zone from the total", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    (snapshot(model), super::change_zone_floor_area_participation(crate::model::EntityId(1), false))'''),
        ("⛔️", "refuses-a-missing-zone", "refuses an absent zone", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::change_zone_floor_area_participation(crate::model::EntityId(9), false))'''),
    ],
)
# endregion 🔖️Zones


# region 🔖️G3
G3_PAST = {"create": "Created", "delete": "Deleted", "change": "Changed", "rename": "Renamed", "add": "Added", "remove": "Removed", "replace": "Replaced"}
G3_CLONE = ("String", "Vec<crate::model::EntityId>", "crate::model::ZoneEquipmentType")
G3_ID_TYPES = ("crate::model::EntityId", "crate::model::ScheduleId")
G3_SCHEDULE_FAMILIES = ("constants", "daily", "weekly", "annual", "time_series")
G3_VARIATION_SELECTOR = "️"


def g3e(base: str) -> str:
    """🔤 One kind emoji: the pasted base character plus the mandatory U+FE0F appended here, so a
    spec row can never ship a bare or a doubled variation selector — `mutationDirectoryPattern` wants
    exactly one and many emoji render identically without it."""
    assert G3_VARIATION_SELECTOR not in base, base
    return base + G3_VARIATION_SELECTOR


def g3_clone(ty: str) -> str:
    """🧬️ The `.clone()` a non-`Copy` payload field needs, and nothing at all for the rest."""
    return ".clone()" if ty in G3_CLONE else ""


def g3_show(name: str, ty: str) -> tuple[str, str]:
    """🏷️ How one payload field prints inside a `label()`: an id newtype shows its bare number, every
    other type shows its `Debug` form."""
    return ("{}", f"self.{name}.0") if ty in G3_ID_TYPES else ("{:?}", f"self.{name}")


def g3_finite(expression: str) -> str:
    """🐍️ The second implementation's own non-finite test — Python has no `f64::is_finite`, and
    importing one would make this file depend on the numerics it is judging."""
    return f'{expression} != {expression} or {expression} in (float("inf"), float("-inf"))'


class G3Check:
    """🚧️ One refusal, stated once and emitted into the three places a refusal has to appear: the
    Rust diff's early return, the Rust inverse's "a refused forward step owes no undo" guard, and the
    Python second implementation's own branch. Every pattern function below builds these — a spec row
    never writes a refusal by hand, so the three can never drift apart."""

    def __init__(self, code: str, cond: str, message: str, path: str, py_cond: str, py_path: str):
        self.code = code
        self.cond = cond
        self.message = message
        self.path = path
        self.py_cond = py_cond
        self.py_path = py_path

    def rust(self) -> str:
        return f'''    if {self.cond} {{
        return protocol::MutationOutcome::error("{self.code}", {self.message}, [{self.path}]);
    }}'''

    def python(self) -> str:
        return f'''    if {self.py_cond}:
        return unchanged(before), rejected("{self.code}", [{self.py_path}])'''

    def final_rust(self) -> str:
        """⛔️ The same refusal as the WHOLE body of a kind that can only ever refuse, so no
        unreachable tail has to be written after it."""
        return f'    protocol::MutationOutcome::error("{self.code}", {self.message}, [{self.path}])'

    def final_python(self) -> str:
        return f'    return unchanged(before), rejected("{self.code}", [{self.py_path}])'


def g3_rust_checks(checks) -> str:
    return "\n".join(check.rust() for check in checks)


def g3_python_checks(checks) -> str:
    return "\n".join(check.python() for check in checks)


def g3_refused(checks) -> str:
    """↩️ The single boolean an inverse asks before it offers any undo at all."""
    return " || ".join(f"({check.cond})" for check in checks)


def g3_chk_when(flag: str, check: G3Check) -> G3Check:
    """🚧️ One refusal narrowed to the payloads a `present` flag says carry the value at all."""
    return G3Check(check.code, f"payload.{flag} && ({check.cond})", check.message, check.path, f'payload["{field_camel(flag)}"] and ({check.py_cond})', check.py_path)


def g3_chk_duplicate(entity) -> G3Check:
    """🚧️ An id-keyed create refuses a second row on an id the collection already holds."""
    key, cam = entity.key, field_camel(entity.key)
    return G3Check(
        "mutation.duplicate-id",
        f"base.model.{entity.coll}.iter().any(|item| item.{key} == payload.{key})",
        f'format!("{entity.display} {{}} already exists.", payload.{key}.0)',
        f"payload.{key}.0.to_string()",
        f'any(entry["{key}"] == payload["{cam}"] for entry in before["model"]["{entity.coll}"])',
        f'str(payload["{cam}"])',
    )


def g3_chk_zone(field: str) -> G3Check:
    """🚧️ A zone-valued foreign key names a zone this model really has."""
    cam = field_camel(field)
    return G3Check(
        "mutation.target-missing",
        f"!base.model.zones.iter().any(|zone| zone.id == payload.{field})",
        f'format!("Zone {{}} does not exist.", payload.{field}.0)',
        f"payload.{field}.0.to_string()",
        f'not any(entry["id"] == payload["{cam}"] for entry in before["model"]["zones"])',
        f'str(payload["{cam}"])',
    )


def g3_chk_reference(coll: str, display: str, field: str) -> G3Check:
    """🚧️ Any other id-keyed foreign key names a row this model really has."""
    cam = field_camel(field)
    return G3Check(
        "mutation.target-missing",
        f"!base.model.{coll}.iter().any(|item| item.id == payload.{field})",
        f'format!("{display} {{}} does not exist.", payload.{field}.0)',
        f"payload.{field}.0.to_string()",
        f'not any(entry["id"] == payload["{cam}"] for entry in before["model"]["{coll}"])',
        f'str(payload["{cam}"])',
    )


def g3_chk_schedule(field: str) -> G3Check:
    """🚧️ A `ScheduleId` resolves inside the model's OWN `ScheduleSet` — the five families share one
    id space, so any of them may own it, and `Model::validate` checks the same thing after the fact."""
    cam = field_camel(field)
    rust = " || ".join(f"base.model.schedules.{family}.iter().any(|schedule| schedule.id == payload.{field})" for family in G3_SCHEDULE_FAMILIES)
    families = ", ".join(f'"{family}"' for family in G3_SCHEDULE_FAMILIES)
    return G3Check(
        "mutation.target-missing",
        f"!({rust})",
        f'format!("Schedule {{}} is not defined by this model.", payload.{field}.0)',
        f"payload.{field}.0.to_string()",
        f'not any(entry["id"] == payload["{cam}"] for family in ({families},) for entry in before["model"]["schedules"][family])',
        f'str(payload["{cam}"])',
    )


def g3_chk_positive(field: str, key: str, what: str) -> G3Check:
    """🚧️ A physical quantity that only exists above zero."""
    cam, key_cam = field_camel(field), field_camel(key)
    value = f'payload["{cam}"]'
    return G3Check(
        "mutation.invariant",
        f"!payload.{field}.is_finite() || payload.{field} <= 0.0",
        f'format!("{what} must be a positive finite number, got {{}}.", payload.{field})',
        f"payload.{key}.0.to_string()",
        f'{g3_finite(value)} or {value} <= 0.0',
        f'str(payload["{key_cam}"])',
    )


def g3_chk_non_negative(field: str, key: str, what: str) -> G3Check:
    """🚧️ A physical quantity that may be zero but never negative or non-finite."""
    cam, key_cam = field_camel(field), field_camel(key)
    value = f'payload["{cam}"]'
    return G3Check(
        "mutation.invariant",
        f"!payload.{field}.is_finite() || payload.{field} < 0.0",
        f'format!("{what} must be a non-negative finite number, got {{}}.", payload.{field})',
        f"payload.{key}.0.to_string()",
        f'{g3_finite(value)} or {value} < 0.0',
        f'str(payload["{key_cam}"])',
    )


def g3_chk_range(field: str, key: str, low: str, high: str, what: str) -> G3Check:
    """🚧️ A physical quantity with a stated representable interval."""
    cam, key_cam = field_camel(field), field_camel(key)
    value = f'payload["{cam}"]'
    return G3Check(
        "mutation.invariant",
        f"!payload.{field}.is_finite() || !({low}..={high}).contains(&payload.{field})",
        f'format!("{what} must lie between {low} and {high}, got {{}}.", payload.{field})',
        f"payload.{key}.0.to_string()",
        f'{g3_finite(value)} or not {low} <= {value} <= {high}',
        f'str(payload["{key_cam}"])',
    )


def g3_chk_fraction(field: str, key: str, what: str) -> G3Check:
    """🚧️ A dimensionless fraction that only means anything inside `0..=1`."""
    return g3_chk_range(field, key, "0.0", "1.0", what)


def g3_chk_at_least_one(field: str, key: str, what: str) -> G3Check:
    """🚧️ A count or a rank that only means anything from one upwards."""
    cam, key_cam = field_camel(field), field_camel(key)
    return G3Check(
        "mutation.invariant",
        f"payload.{field} == 0",
        f'"{what} must be at least one.".to_string()',
        f"payload.{key}.0.to_string()",
        f'payload["{cam}"] == 0',
        f'str(payload["{key_cam}"])',
    )


def g3_chk_blank(field: str, key: str, what: str) -> G3Check:
    """🚧️ An identity field every report keys on is never blank."""
    cam, key_cam = field_camel(field), field_camel(key)
    return G3Check(
        "mutation.invariant",
        f"payload.{field}.trim().is_empty()",
        f'"{what} must not be blank.".to_string()',
        f"payload.{key}.0.to_string()",
        f'not payload["{cam}"].strip()',
        f'str(payload["{key_cam}"])',
    )


def g3_chk_duplicate_name(entity, field: str, exclude_self: bool) -> G3Check:
    """🚧️ Two rows of one collection never share a name, because the name is what an
    EnergyPlus-class report addresses them by."""
    key, cam, key_cam = entity.key, field_camel(field), field_camel(entity.key)
    rust_self = f"item.{key} != payload.{key} && " if exclude_self else ""
    py_self = f'entry["{key}"] != payload["{key_cam}"] and ' if exclude_self else ""
    return G3Check(
        "mutation.duplicate-id",
        f"base.model.{entity.coll}.iter().any(|item| {rust_self}item.name == payload.{field})",
        f'format!("Another {entity.noun} is already named {{:?}}.", payload.{field})',
        f"payload.{key}.0.to_string()",
        f'any({py_self}entry["name"] == payload["{cam}"] for entry in before["model"]["{entity.coll}"])',
        f'str(payload["{key_cam}"])',
    )


def g3_chk_absent_zero(present: str, value: str, key: str, what: str) -> G3Check:
    """🚧️ `Option<T>` has no `dsl::DslField`, so an optional numeric slot travels as a `present` flag
    beside its value, exactly the shape `replace-airflow-network` already uses for its own optional
    singleton. An absent slot carries zero, so one payload can never describe two documents."""
    present_cam, value_cam, key_cam = field_camel(present), field_camel(value), field_camel(key)
    return G3Check(
        "mutation.invariant",
        f"!payload.{present} && payload.{value} != 0.0",
        f'"An absent {what} carries the value zero.".to_string()',
        f"payload.{key}.0.to_string()",
        f'not payload["{present_cam}"] and payload["{value_cam}"] != 0.0',
        f'str(payload["{key_cam}"])',
    )


def g3_chk_absent_zero_id(present: str, value: str, key: str, what: str) -> G3Check:
    """🚧️ The same flattened-`Option` rule for an optional id reference."""
    present_cam, value_cam, key_cam = field_camel(present), field_camel(value), field_camel(key)
    return G3Check(
        "mutation.invariant",
        f"!payload.{present} && payload.{value}.0 != 0",
        f'"An absent {what} carries the id zero.".to_string()',
        f"payload.{key}.0.to_string()",
        f'not payload["{present_cam}"] and payload["{value_cam}"] != 0',
        f'str(payload["{key_cam}"])',
    )


class G3Entity:
    """🧱️ One id-keyed `Model` collection, declared once: the create payload, how a row is built from
    that payload, how the payload is read back off a row, and the two fixture seeds (with the row and
    without it) every one of its kinds is exercised against. `create`/`delete`/`change`/`rename`/
    `add`/`remove` are then pure functions of this — in Rust, in JSON Schema, in the wire mirrors and
    in the Python second implementation at once."""

    def __init__(self, *, coll, display, struct, noun, payload, seed, bare, probe_args, key="id", key_literal=None, ctor=None, recreate=None, py_new=None, py_recreate=None, pre="", create_slug=None, delete_slug=None):
        self.coll = coll
        self.display = display
        self.struct = struct
        self.noun = noun
        self.payload = payload
        self.seed = seed
        self.bare = bare
        self.probe_args = probe_args
        self.key = key
        self.key_literal = key_literal or probe_args.split(",")[0].strip()
        self.pre = pre
        self.create_slug = create_slug or f"create-{noun.replace(' ', '-')}"
        self.delete_slug = delete_slug or f"delete-{noun.replace(' ', '-')}"
        self.ctor = ctor or ", ".join(f"{name}: payload.{name}{g3_clone(ty)}" for name, ty in payload)
        self.recreate = recreate or ", ".join(f"item.{name}{g3_clone(ty)}" for name, ty in payload)
        self.py_new = py_new or "{" + ", ".join(f'"{name}": payload["{field_camel(name)}"]' for name, _ in payload) + "}"
        self.py_recreate = py_recreate or "{" + ", ".join(f'"{field_camel(name)}": item["{name}"]' for name, _ in payload) + "}"

    @property
    def key_camel(self) -> str:
        return field_camel(self.key)

    def missing(self) -> str:
        """🎯️ The let-else every target-addressed kind of this collection opens with."""
        return f'''    let Some(existing) = base.model.{self.coll}.iter().find(|item| item.{self.key} == payload.{self.key}) else {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{self.display} {{}} does not exist.", payload.{self.key}.0), [payload.{self.key}.0.to_string()]);
    }};'''

    def py_missing(self) -> str:
        return f'''    item = next((entry for entry in before["model"]["{self.coll}"] if entry["{self.key}"] == payload["{self.key_camel}"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["{self.key_camel}"])])'''

    def py_row(self) -> str:
        return f'''    item = next(entry for entry in before["model"]["{self.coll}"] if entry["{self.key}"] == payload["{self.key_camel}"])'''

    def scenario(self, call: str, seed=None) -> str:
        return (seed if seed is not None else self.seed) + f"\n    (snapshot(model), super::{call})"


def g3_register(*, number, emoji, slug, entity_slug, doc, fields, label, target, diff, inverse, outcome_classes, cases, probe, python, python_invert=None) -> None:
    """🧮 Registers one G3 kind, deriving the verb, the record name and the display name from the
    ledger slug so those three can never disagree with it."""
    words = slug.split("-")
    kind(
        number=number,
        slug=slug,
        emoji=g3e(emoji),
        verb=words[0],
        entity=entity_slug,
        record=G3_PAST[words[0]] + "".join(word.capitalize() for word in words[1:]),
        display=" ".join(word.capitalize() for word in words),
        doc=doc,
        fields=fields,
        label=label,
        target=target,
        diff=diff,
        inverse=inverse,
        cases=cases,
        outcome_classes=outcome_classes,
        probe=probe,
        python=python,
        python_invert=python_invert,
    )


def g3_create(*, number, emoji, entity, entity_slug, doc, checks, happy_args, happy_case, happy_desc, refusal_args, refusal_case, refusal_desc, refusal_seed=None) -> None:
    """🌱️ `create-<entity>` — one new row under a caller-minted id, because no mutation mints its own
    (vocabulary §5.3). Refuses a duplicate id and every broken reference the row would introduce; its
    undo is the matching `delete-<entity>`."""
    slug, key = entity.create_slug, entity.key
    all_checks = [g3_chk_duplicate(entity)] + list(checks)
    diff = f'''{g3_rust_checks(all_checks)}
{entity.pre}    let mut model = base.model.clone();
    model.{entity.coll}.push(crate::model::{entity.struct} {{ {entity.ctor} }});
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''
    inverse = f'''    if {g3_refused(all_checks)} {{
        return Vec::new();
    }}
    vec![super::{snake(entity.delete_slug)}(payload.{key})]'''
    python = f'''    """{g3e(emoji)} `{slug}` — one new {entity.noun} row under a caller-minted id."""
{g3_python_checks(all_checks)}
    after = copy.deepcopy(before)
    after["model"]["{entity.coll}"].append({entity.py_new})
    return after, applied()'''
    python_invert = f'''    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("{entity.delete_slug}", {{"{entity.key_camel}": payload["{entity.key_camel}"]}})]'''
    g3_register(
        number=number,
        emoji=emoji,
        slug=slug,
        entity_slug=entity_slug,
        doc=doc,
        fields=entity.payload,
        label=f'format!("Create {entity.noun} {{}}", self.{key}.0)',
        target=f"vec![self.{key}.0.to_string()]",
        diff=diff,
        inverse=inverse,
        outcome_classes=["applied", "error"],
        cases=[
            ("✅️", happy_case, happy_desc, entity.scenario(f"{snake(slug)}({happy_args})", entity.bare)),
            ("⛔️", refusal_case, refusal_desc, entity.scenario(f"{snake(slug)}({refusal_args})", refusal_seed)),
        ],
        probe=f"{snake(slug)}({entity.probe_args})",
        python=python,
        python_invert=python_invert,
    )


def g3_delete(*, number, emoji, entity, entity_slug, doc, happy_args, happy_case, happy_desc, refusal_args, refusal_case, refusal_desc, refusal_seed=None, checks=()) -> None:
    """🪓 `delete-<entity>` — drops one row addressed by its id, refusing an absent target and any row
    another collection still points at. Its undo re-creates the row read off BASE."""
    slug, key = entity.delete_slug, entity.key
    guard = f" if !({g3_refused(checks)})" if checks else ""
    diff = f'''{entity.missing()}
    let _ = existing;
{g3_rust_checks(checks)}
    let mut model = base.model.clone();
    model.{entity.coll}.retain(|item| item.{key} != payload.{key});
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''
    inverse = f'''    match base.model.{entity.coll}.iter().find(|item| item.{key} == payload.{key}) {{
        Some(item){guard} => vec![super::{snake(entity.create_slug)}({entity.recreate})],
        _ => Vec::new(),
    }}'''
    python = f'''    """{g3e(emoji)} `{slug}` — drops the addressed {entity.noun} row."""
{entity.py_missing()}
{g3_python_checks(checks)}
    after = copy.deepcopy(before)
    after["model"]["{entity.coll}"] = [entry for entry in after["model"]["{entity.coll}"] if entry["{key}"] != payload["{entity.key_camel}"]]
    return after, applied()'''
    python_invert = f'''    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
{entity.py_row()}
    return [("{entity.create_slug}", {entity.py_recreate})]'''
    g3_register(
        number=number,
        emoji=emoji,
        slug=slug,
        entity_slug=entity_slug,
        doc=doc,
        fields=[(key, "crate::model::EntityId")],
        label=f'format!("Delete {entity.noun} {{}}", self.{key}.0)',
        target=f"vec![self.{key}.0.to_string()]",
        diff=diff,
        inverse=inverse,
        outcome_classes=["applied", "error"],
        cases=[
            ("✅️", happy_case, happy_desc, entity.scenario(f"{snake(slug)}({happy_args})")),
            ("⛔️", refusal_case, refusal_desc, entity.scenario(f"{snake(slug)}({refusal_args})", refusal_seed)),
        ],
        probe=f"{snake(slug)}({entity.key_literal})",
        python=python,
        python_invert=python_invert,
    )


def g3_change(*, number, emoji, slug, entity, entity_slug, field, ty, doc, what, checks, happy, happy_case, happy_desc, probe_value, refusal=None, refusal_case=None, refusal_desc=None) -> None:
    """🔧️ `change-<entity>-<field>` — one independent scalar of one row. Refuses an absent target and
    every stated invariant, warns on a no-op, and is its own inverse with the old value read off
    BASE. `refusal=None` means the refusal vector addresses an absent row instead of a bad value."""
    key = entity.key
    name = f"new_{field}"
    cam, key_cam = field_camel(name), entity.key_camel
    spec, shown = g3_show(name, ty)
    clone = g3_clone(ty)
    diff = f'''{entity.missing()}
{g3_rust_checks(checks)}
    if existing.{field} == payload.{name} {{
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("{entity.display} {{}} already has that {what}.", payload.{key}.0));
    }}
    let mut model = base.model.clone();
    if let Some(item) = model.{entity.coll}.iter_mut().find(|item| item.{key} == payload.{key}) {{
        item.{field} = payload.{name}{clone};
    }}
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''
    guard = f" && !({g3_refused(checks)})" if checks else ""
    inverse = f'''    match base.model.{entity.coll}.iter().find(|item| item.{key} == payload.{key}) {{
        Some(item) if item.{field} != payload.{name}{guard} => vec![super::{snake(slug)}(payload.{key}, item.{field}{clone})],
        _ => Vec::new(),
    }}'''
    python = f'''    """{g3e(emoji)} `{slug}` — the {what} of one {entity.noun} row."""
{entity.py_missing()}
{g3_python_checks(checks)}
    if item["{field}"] == payload["{cam}"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["{entity.coll}"]:
        if entry["{key}"] == payload["{key_cam}"]:
            entry["{field}"] = payload["{cam}"]
    return after, applied()'''
    python_invert = f'''    """↩️ Its own inverse, with the old {what} read off BASE."""
{entity.py_row()}
    return [("{slug}", {{"{key_cam}": payload["{key_cam}"], "{cam}": item["{field}"]}})]'''
    refusal_key = entity.key_literal if refusal is not None else "crate::model::EntityId(99)"
    refusal_value = refusal if refusal is not None else happy
    g3_register(
        number=number,
        emoji=emoji,
        slug=slug,
        entity_slug=entity_slug,
        doc=doc,
        fields=[(key, "crate::model::EntityId"), (name, ty)],
        label=f'format!("Change {entity.noun} {{}} {what} to {spec}", self.{key}.0, {shown})',
        target=f"vec![self.{key}.0.to_string()]",
        diff=diff,
        inverse=inverse,
        outcome_classes=["applied", "warning", "error"],
        cases=[
            ("✅️", happy_case, happy_desc, entity.scenario(f"{snake(slug)}({entity.key_literal}, {happy})")),
            (
                "⛔️",
                refusal_case or "refuses-an-absent-row",
                refusal_desc or f"refuses an absent {entity.noun}",
                entity.scenario(f"{snake(slug)}({refusal_key}, {refusal_value})"),
            ),
        ],
        probe=f"{snake(slug)}({entity.key_literal}, {probe_value})",
        python=python,
        python_invert=python_invert,
    )


def g3_change_optional(*, number, emoji, slug, entity, entity_slug, field, doc, what, present_field, value_field, checks, happy_args, happy_case, happy_desc, refusal_args, refusal_case, refusal_desc, probe_args) -> None:
    """🫙 `change-<entity>-<optional>` — an `Option<f64>` slot. `Option<T>` has no `dsl::DslField`, so
    the slot travels as a `present` flag beside its value; `present` false is the autosized/absent
    reading and then the value has to be zero."""
    key, key_cam = entity.key, entity.key_camel
    present_cam, value_cam = field_camel(present_field), field_camel(value_field)
    all_checks = [g3_chk_absent_zero(present_field, value_field, key, what)] + [g3_chk_when(present_field, check) for check in checks]
    diff = f'''{entity.missing()}
{g3_rust_checks(all_checks)}
    let value = payload.{present_field}.then_some(payload.{value_field});
    if existing.{field} == value {{
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("{entity.display} {{}} already has that {what}.", payload.{key}.0));
    }}
    let mut model = base.model.clone();
    if let Some(item) = model.{entity.coll}.iter_mut().find(|item| item.{key} == payload.{key}) {{
        item.{field} = value;
    }}
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''
    inverse = f'''    let value = payload.{present_field}.then_some(payload.{value_field});
    match base.model.{entity.coll}.iter().find(|item| item.{key} == payload.{key}) {{
        Some(item) if item.{field} != value && !({g3_refused(all_checks)}) => vec![super::{snake(slug)}(payload.{key}, item.{field}.is_some(), item.{field}.unwrap_or(0.0))],
        _ => Vec::new(),
    }}'''
    python = f'''    """{g3e(emoji)} `{slug}` — the optional {what} of one {entity.noun} row, carried as a present
    flag beside its value because `Option<T>` has no `dsl::DslField`."""
{entity.py_missing()}
{g3_python_checks(all_checks)}
    value = payload["{value_cam}"] if payload["{present_cam}"] else None
    if item["{field}"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["{entity.coll}"]:
        if entry["{key}"] == payload["{key_cam}"]:
            entry["{field}"] = value
    return after, applied()'''
    python_invert = f'''    """↩️ Its own inverse, with the old {what} read off BASE."""
{entity.py_row()}
    old = item["{field}"]
    return [("{slug}", {{"{key_cam}": payload["{key_cam}"], "{present_cam}": old is not None, "{value_cam}": old if old is not None else 0.0}})]'''
    g3_register(
        number=number,
        emoji=emoji,
        slug=slug,
        entity_slug=entity_slug,
        doc=doc,
        fields=[(key, "crate::model::EntityId"), (present_field, "bool"), (value_field, "f64")],
        label=f'format!("Change {entity.noun} {{}} {what} to {{:?}}", self.{key}.0, self.{present_field}.then_some(self.{value_field}))',
        target=f"vec![self.{key}.0.to_string()]",
        diff=diff,
        inverse=inverse,
        outcome_classes=["applied", "warning", "error"],
        cases=[
            ("✅️", happy_case, happy_desc, entity.scenario(f"{snake(slug)}({happy_args})")),
            ("⛔️", refusal_case, refusal_desc, entity.scenario(f"{snake(slug)}({refusal_args})")),
        ],
        probe=f"{snake(slug)}({probe_args})",
        python=python,
        python_invert=python_invert,
    )


def g3_rename(*, number, emoji, slug, entity, entity_slug, doc, happy, happy_case, happy_desc, probe_value) -> None:
    """🏷️ `rename-<entity>` — the identity field, refusing a blank and a name another row already
    holds, warning on a no-op."""
    g3_change(
        number=number,
        emoji=emoji,
        slug=slug,
        entity=entity,
        entity_slug=entity_slug,
        field="name",
        ty="String",
        doc=doc,
        what="name",
        checks=[g3_chk_blank("new_name", entity.key, f"A {entity.noun} name"), g3_chk_duplicate_name(entity, "new_name", True)],
        happy=happy,
        happy_case=happy_case,
        happy_desc=happy_desc,
        refusal='"   ".into()',
        refusal_case="refuses-a-blank-name",
        refusal_desc="refuses a blank name",
        probe_value=probe_value,
    )


def g3_member_add(*, number, emoji, slug, entity, entity_slug, list_field, member, member_display, doc, checks, remove_slug, happy_args, happy_case, happy_desc, refusal_args, refusal_case, refusal_desc, probe_args, refusal_seed=None) -> None:
    """➕ `add-<owner>-<member>` — one id into an ordered membership list. The list is kept ascending,
    so a later removal's undo lands the id back in exactly the slot it left."""
    key, cam, key_cam = entity.key, field_camel(member), entity.key_camel
    present = G3Check(
        "mutation.duplicate-id",
        f"existing.{list_field}.contains(&payload.{member})",
        f'format!("{entity.display} {{}} already lists {member_display} {{}}.", payload.{key}.0, payload.{member}.0)',
        f"payload.{member}.0.to_string()",
        f'payload["{cam}"] in item["{list_field}"]',
        f'str(payload["{cam}"])',
    )
    all_checks = list(checks) + [present]
    diff = f'''{entity.missing()}
{g3_rust_checks(all_checks)}
    let mut model = base.model.clone();
    if let Some(item) = model.{entity.coll}.iter_mut().find(|item| item.{key} == payload.{key}) {{
        let position = item.{list_field}.iter().position(|entry| entry.0 > payload.{member}.0).unwrap_or(item.{list_field}.len());
        item.{list_field}.insert(position, payload.{member});
    }}
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''
    inverse = f'''    match base.model.{entity.coll}.iter().find(|item| item.{key} == payload.{key}) {{
        Some(existing) if !({g3_refused(all_checks)}) => vec![super::{snake(remove_slug)}(payload.{key}, payload.{member})],
        _ => Vec::new(),
    }}'''
    python = f'''    """{g3e(emoji)} `{slug}` — one {member_display} id into the ascending membership list."""
{entity.py_missing()}
{g3_python_checks(all_checks)}
    after = copy.deepcopy(before)
    for entry in after["model"]["{entity.coll}"]:
        if entry["{key}"] == payload["{key_cam}"]:
            members = entry["{list_field}"]
            position = next((index for index, value in enumerate(members) if value > payload["{cam}"]), len(members))
            members.insert(position, payload["{cam}"])
    return after, applied()'''
    python_invert = f'''    """↩️ The undo of an add is the removal of exactly that member."""
    return [("{remove_slug}", {{"{key_cam}": payload["{key_cam}"], "{cam}": payload["{cam}"]}})]'''
    g3_register(
        number=number,
        emoji=emoji,
        slug=slug,
        entity_slug=entity_slug,
        doc=doc,
        fields=[(key, "crate::model::EntityId"), (member, "crate::model::EntityId")],
        label=f'format!("Add {member_display} {{}} to {entity.noun} {{}}", self.{member}.0, self.{key}.0)',
        target=f"vec![self.{key}.0.to_string(), self.{member}.0.to_string()]",
        diff=diff,
        inverse=inverse,
        outcome_classes=["applied", "error"],
        cases=[
            ("✅️", happy_case, happy_desc, entity.scenario(f"{snake(slug)}({happy_args})")),
            ("⛔️", refusal_case, refusal_desc, entity.scenario(f"{snake(slug)}({refusal_args})", refusal_seed)),
        ],
        probe=f"{snake(slug)}({probe_args})",
        python=python,
        python_invert=python_invert,
    )


def g3_member_remove(*, number, emoji, slug, entity, entity_slug, list_field, member, member_display, doc, add_slug, happy_args, happy_case, happy_desc, refusal_args, refusal_case, refusal_desc, probe_args, happy_seed=None, refusal_seed=None) -> None:
    """➖ `remove-<owner>-<member>` — one id out of an ordered membership list, refusing a member the
    list does not hold."""
    key, cam, key_cam = entity.key, field_camel(member), entity.key_camel
    absent = G3Check(
        "mutation.target-missing",
        f"!existing.{list_field}.contains(&payload.{member})",
        f'format!("{entity.display} {{}} does not list {member_display} {{}}.", payload.{key}.0, payload.{member}.0)',
        f"payload.{member}.0.to_string()",
        f'payload["{cam}"] not in item["{list_field}"]',
        f'str(payload["{cam}"])',
    )
    diff = f'''{entity.missing()}
{absent.rust()}
    let mut model = base.model.clone();
    if let Some(item) = model.{entity.coll}.iter_mut().find(|item| item.{key} == payload.{key}) {{
        item.{list_field}.retain(|entry| *entry != payload.{member});
    }}
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''
    inverse = f'''    match base.model.{entity.coll}.iter().find(|item| item.{key} == payload.{key}) {{
        Some(existing) if !({absent.cond}) => vec![super::{snake(add_slug)}(payload.{key}, payload.{member})],
        _ => Vec::new(),
    }}'''
    python = f'''    """{g3e(emoji)} `{slug}` — one {member_display} id out of the membership list."""
{entity.py_missing()}
{absent.python()}
    after = copy.deepcopy(before)
    for entry in after["model"]["{entity.coll}"]:
        if entry["{key}"] == payload["{key_cam}"]:
            entry["{list_field}"] = [value for value in entry["{list_field}"] if value != payload["{cam}"]]
    return after, applied()'''
    python_invert = f'''    """↩️ The undo of a removal puts the member back at its ascending position."""
    return [("{add_slug}", {{"{key_cam}": payload["{key_cam}"], "{cam}": payload["{cam}"]}})]'''
    g3_register(
        number=number,
        emoji=emoji,
        slug=slug,
        entity_slug=entity_slug,
        doc=doc,
        fields=[(key, "crate::model::EntityId"), (member, "crate::model::EntityId")],
        label=f'format!("Remove {member_display} {{}} from {entity.noun} {{}}", self.{member}.0, self.{key}.0)',
        target=f"vec![self.{key}.0.to_string(), self.{member}.0.to_string()]",
        diff=diff,
        inverse=inverse,
        outcome_classes=["applied", "error"],
        cases=[
            ("✅️", happy_case, happy_desc, entity.scenario(f"{snake(slug)}({happy_args})", happy_seed)),
            ("⛔️", refusal_case, refusal_desc, entity.scenario(f"{snake(slug)}({refusal_args})", refusal_seed)),
        ],
        probe=f"{snake(slug)}({probe_args})",
        python=python,
        python_invert=python_invert,
    )

G3_MODEL = '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.zones.push(zone(2, "ZONE TWO"));
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(1), value: 20.0 });
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(2), value: 27.0 });'''

G3_THERMOSTAT_ROW = '''
    model.thermostats.push(crate::model::Thermostat { id: crate::model::EntityId(10), zone_id: crate::model::EntityId(1), heating_setpoint_schedule_id: crate::model::ScheduleId(1), cooling_setpoint_schedule_id: crate::model::ScheduleId(2), heating_throttle_range_k: 1.0, cooling_throttle_range_k: 1.0 });'''

THERMOSTAT = G3Entity(
    coll="thermostats",
    display="Thermostat",
    struct="Thermostat",
    noun="thermostat",
    payload=[
        ("id", "crate::model::EntityId"),
        ("zone_id", "crate::model::EntityId"),
        ("heating_setpoint_schedule_id", "crate::model::ScheduleId"),
        ("cooling_setpoint_schedule_id", "crate::model::ScheduleId"),
        ("heating_throttle_range_k", "f64"),
        ("cooling_throttle_range_k", "f64"),
    ],
    seed=G3_MODEL + G3_THERMOSTAT_ROW,
    bare=G3_MODEL,
    probe_args="crate::model::EntityId(10), crate::model::EntityId(1), crate::model::ScheduleId(1), crate::model::ScheduleId(2), 1.0, 1.0",
)

g3_create(
    number=500,
    emoji="🩺",
    entity=THERMOSTAT,
    entity_slug="thermostat",
    doc="Creates the setpoint control of one zone. The two setpoint schedules must already be defined by the model's own `ScheduleSet` and the zone must exist, so a thermostat can never be born dangling.",
    checks=[
        g3_chk_zone("zone_id"),
        g3_chk_schedule("heating_setpoint_schedule_id"),
        g3_chk_schedule("cooling_setpoint_schedule_id"),
        g3_chk_positive("heating_throttle_range_k", "id", "A heating throttle range"),
        g3_chk_positive("cooling_throttle_range_k", "id", "A cooling throttle range"),
    ],
    happy_args="crate::model::EntityId(10), crate::model::EntityId(1), crate::model::ScheduleId(1), crate::model::ScheduleId(2), 1.0, 1.0",
    happy_case="controls-zone-one",
    happy_desc="puts a thermostat on the first zone",
    refusal_args="crate::model::EntityId(10), crate::model::EntityId(99), crate::model::ScheduleId(1), crate::model::ScheduleId(2), 1.0, 1.0",
    refusal_case="refuses-an-absent-zone",
    refusal_desc="refuses a thermostat on a zone that does not exist",
    refusal_seed=G3_MODEL,
)

g3_delete(
    number=501,
    emoji="🛑",
    entity=THERMOSTAT,
    entity_slug="thermostat",
    doc="Drops one zone's setpoint control, leaving the zone free-floating. Refused when no thermostat carries the id, so an undo chain can never invent a deletion that had no partner.",
    happy_args="crate::model::EntityId(10)",
    happy_case="frees-zone-one",
    happy_desc="drops the thermostat off the first zone",
    refusal_args="crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-one",
    refusal_desc="refuses an absent thermostat",
)

g3_change(
    number=502,
    emoji="🛖",
    slug="change-thermostat-zone",
    entity=THERMOSTAT,
    entity_slug="thermostat",
    field="zone_id",
    ty="crate::model::EntityId",
    doc="Moves one thermostat to another zone. A foreign key, not a nesting field — zones do not nest, so this is a plain reassignment under the general axis and not a `move-to-<container>`.",
    what="zone",
    checks=[g3_chk_zone("new_zone_id")],
    happy="crate::model::EntityId(2)",
    happy_case="moves-to-zone-two",
    happy_desc="moves the thermostat to the second zone",
    refusal="crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-zone",
    refusal_desc="refuses a zone that does not exist",
    probe_value="crate::model::EntityId(2)",
)

g3_change(
    number=503,
    emoji="🥵",
    slug="change-thermostat-heating-setpoint-schedule",
    entity=THERMOSTAT,
    entity_slug="thermostat",
    field="heating_setpoint_schedule_id",
    ty="crate::model::ScheduleId",
    doc="Repoints the heating setpoint at another schedule the model defines. Setpoint schedules carry °C directly.",
    what="heating setpoint schedule",
    checks=[g3_chk_schedule("new_heating_setpoint_schedule_id")],
    happy="crate::model::ScheduleId(2)",
    happy_case="repoints-heating",
    happy_desc="repoints the heating setpoint at the second schedule",
    refusal="crate::model::ScheduleId(99)",
    refusal_case="refuses-an-absent-one",
    refusal_desc="refuses a schedule the model does not define",
    probe_value="crate::model::ScheduleId(2)",
)

g3_change(
    number=504,
    emoji="🐧",
    slug="change-thermostat-cooling-setpoint-schedule",
    entity=THERMOSTAT,
    entity_slug="thermostat",
    field="cooling_setpoint_schedule_id",
    ty="crate::model::ScheduleId",
    doc="Repoints the cooling setpoint at another schedule the model defines.",
    what="cooling setpoint schedule",
    checks=[g3_chk_schedule("new_cooling_setpoint_schedule_id")],
    happy="crate::model::ScheduleId(1)",
    happy_case="repoints-cooling",
    happy_desc="repoints the cooling setpoint at the first schedule",
    refusal="crate::model::ScheduleId(99)",
    refusal_case="refuses-an-absent-one",
    refusal_desc="refuses a schedule the model does not define",
    probe_value="crate::model::ScheduleId(1)",
)

g3_change(
    number=505,
    emoji="🎚",
    slug="change-thermostat-heating-throttle-range",
    entity=THERMOSTAT,
    entity_slug="thermostat",
    field="heating_throttle_range_k",
    ty="f64",
    doc="Sets the proportional band the heating setpoint is approached over, in kelvin.",
    what="heating throttle range",
    checks=[g3_chk_positive("new_heating_throttle_range_k", "id", "A heating throttle range")],
    happy="2.0",
    happy_case="widens-heating-band",
    happy_desc="widens the heating throttle band",
    refusal="0.0",
    refusal_case="refuses-a-zero-band",
    refusal_desc="refuses a zero-wide throttle band",
    probe_value="2.0",
)

g3_change(
    number=506,
    emoji="🎛",
    slug="change-thermostat-cooling-throttle-range",
    entity=THERMOSTAT,
    entity_slug="thermostat",
    field="cooling_throttle_range_k",
    ty="f64",
    doc="Sets the proportional band the cooling setpoint is approached over, in kelvin.",
    what="cooling throttle range",
    checks=[g3_chk_positive("new_cooling_throttle_range_k", "id", "A cooling throttle range")],
    happy="2.0",
    happy_case="widens-cooling-band",
    happy_desc="widens the cooling throttle band",
    refusal="-1.0",
    refusal_case="refuses-a-negative-band",
    refusal_desc="refuses a negative throttle band",
    probe_value="2.0",
)

G3_HUMIDISTAT_ROW = '''
    model.humidistats.push(crate::model::Humidistat { id: crate::model::EntityId(11), zone_id: crate::model::EntityId(1), humidifying_setpoint_schedule_id: crate::model::ScheduleId(1), dehumidifying_setpoint_schedule_id: crate::model::ScheduleId(2), humidifying_throttle_range: 5.0, dehumidifying_throttle_range: 5.0 });'''

HUMIDISTAT = G3Entity(
    coll="humidistats",
    display="Humidistat",
    struct="Humidistat",
    noun="humidistat",
    payload=[
        ("id", "crate::model::EntityId"),
        ("zone_id", "crate::model::EntityId"),
        ("humidifying_setpoint_schedule_id", "crate::model::ScheduleId"),
        ("dehumidifying_setpoint_schedule_id", "crate::model::ScheduleId"),
        ("humidifying_throttle_range", "f64"),
        ("dehumidifying_throttle_range", "f64"),
    ],
    seed=G3_MODEL + G3_HUMIDISTAT_ROW,
    bare=G3_MODEL,
    probe_args="crate::model::EntityId(11), crate::model::EntityId(1), crate::model::ScheduleId(1), crate::model::ScheduleId(2), 5.0, 5.0",
)

g3_create(
    number=507,
    emoji="🌂",
    entity=HUMIDISTAT,
    entity_slug="humidistat",
    doc="Creates the humidity control of one zone, against two setpoint schedules the model already defines.",
    checks=[
        g3_chk_zone("zone_id"),
        g3_chk_schedule("humidifying_setpoint_schedule_id"),
        g3_chk_schedule("dehumidifying_setpoint_schedule_id"),
        g3_chk_positive("humidifying_throttle_range", "id", "A humidifying throttle range"),
        g3_chk_positive("dehumidifying_throttle_range", "id", "A dehumidifying throttle range"),
    ],
    happy_args="crate::model::EntityId(11), crate::model::EntityId(1), crate::model::ScheduleId(1), crate::model::ScheduleId(2), 5.0, 5.0",
    happy_case="controls-zone-one",
    happy_desc="puts a humidistat on the first zone",
    refusal_args="crate::model::EntityId(11), crate::model::EntityId(1), crate::model::ScheduleId(99), crate::model::ScheduleId(2), 5.0, 5.0",
    refusal_case="refuses-an-absent-one",
    refusal_desc="refuses a schedule the model does not define",
    refusal_seed=G3_MODEL,
)

g3_delete(
    number=508,
    emoji="🏜",
    entity=HUMIDISTAT,
    entity_slug="humidistat",
    doc="Drops one zone's humidity control. Refused when no humidistat carries the id.",
    happy_args="crate::model::EntityId(11)",
    happy_case="drops-the-control",
    happy_desc="drops the humidistat off the first zone",
    refusal_args="crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-one",
    refusal_desc="refuses an absent humidistat",
)

g3_change(
    number=509,
    emoji="🏙",
    slug="change-humidistat-zone",
    entity=HUMIDISTAT,
    entity_slug="humidistat",
    field="zone_id",
    ty="crate::model::EntityId",
    doc="Moves one humidistat to another zone.",
    what="zone",
    checks=[g3_chk_zone("new_zone_id")],
    happy="crate::model::EntityId(2)",
    happy_case="moves-to-zone-two",
    happy_desc="moves the humidistat to the second zone",
    refusal="crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-zone",
    refusal_desc="refuses a zone that does not exist",
    probe_value="crate::model::EntityId(2)",
)

g3_change(
    number=510,
    emoji="☔",
    slug="change-humidistat-humidifying-setpoint-schedule",
    entity=HUMIDISTAT,
    entity_slug="humidistat",
    field="humidifying_setpoint_schedule_id",
    ty="crate::model::ScheduleId",
    doc="Repoints the humidifying setpoint at another schedule the model defines.",
    what="humidifying setpoint schedule",
    checks=[g3_chk_schedule("new_humidifying_setpoint_schedule_id")],
    happy="crate::model::ScheduleId(2)",
    happy_case="repoints-humidifying",
    happy_desc="repoints the humidifying setpoint",
    refusal="crate::model::ScheduleId(99)",
    refusal_case="refuses-an-absent-one",
    refusal_desc="refuses a schedule the model does not define",
    probe_value="crate::model::ScheduleId(2)",
)

g3_change(
    number=511,
    emoji="🏝",
    slug="change-humidistat-dehumidifying-setpoint-schedule",
    entity=HUMIDISTAT,
    entity_slug="humidistat",
    field="dehumidifying_setpoint_schedule_id",
    ty="crate::model::ScheduleId",
    doc="Repoints the dehumidifying setpoint at another schedule the model defines.",
    what="dehumidifying setpoint schedule",
    checks=[g3_chk_schedule("new_dehumidifying_setpoint_schedule_id")],
    happy="crate::model::ScheduleId(1)",
    happy_case="repoints-drying",
    happy_desc="repoints the dehumidifying setpoint",
    refusal="crate::model::ScheduleId(99)",
    refusal_case="refuses-an-absent-one",
    refusal_desc="refuses a schedule the model does not define",
    probe_value="crate::model::ScheduleId(1)",
)

g3_change(
    number=512,
    emoji="🌧",
    slug="change-humidistat-humidifying-throttle-range",
    entity=HUMIDISTAT,
    entity_slug="humidistat",
    field="humidifying_throttle_range",
    ty="f64",
    doc="Sets the proportional band the humidifying setpoint is approached over, in percent relative humidity.",
    what="humidifying throttle range",
    checks=[g3_chk_positive("new_humidifying_throttle_range", "id", "A humidifying throttle range")],
    happy="10.0",
    happy_case="widens-the-band",
    happy_desc="widens the humidifying band",
    refusal="0.0",
    refusal_case="refuses-a-zero-band",
    refusal_desc="refuses a zero-wide band",
    probe_value="10.0",
)

g3_change(
    number=513,
    emoji="🧻",
    slug="change-humidistat-dehumidifying-throttle-range",
    entity=HUMIDISTAT,
    entity_slug="humidistat",
    field="dehumidifying_throttle_range",
    ty="f64",
    doc="Sets the proportional band the dehumidifying setpoint is approached over, in percent relative humidity.",
    what="dehumidifying throttle range",
    checks=[g3_chk_positive("new_dehumidifying_throttle_range", "id", "A dehumidifying throttle range")],
    happy="10.0",
    happy_case="widens-the-band",
    happy_desc="widens the dehumidifying band",
    refusal="-2.0",
    refusal_case="refuses-a-negative-band",
    refusal_desc="refuses a negative band",
    probe_value="10.0",
)

G3_IDEAL_ROW = '''
    model.ideal_loads.push(crate::model::IdealLoadsSystem { id: crate::model::EntityId(12), zone_id: crate::model::EntityId(1), max_heating_supply_air_temp_c: 50.0, min_cooling_supply_air_temp_c: 13.0, max_heating_capacity_w: None, max_cooling_capacity_w: None, outdoor_air_per_person_m3_s: 0.0, outdoor_air_per_area_m3_s_m2: 0.0 });'''

IDEAL_LOADS = G3Entity(
    coll="ideal_loads",
    display="Ideal loads system",
    struct="IdealLoadsSystem",
    noun="ideal loads system",
    payload=[
        ("id", "crate::model::EntityId"),
        ("zone_id", "crate::model::EntityId"),
        ("max_heating_supply_air_temp_c", "f64"),
        ("min_cooling_supply_air_temp_c", "f64"),
        ("max_heating_capacity_present", "bool"),
        ("max_heating_capacity_w", "f64"),
        ("max_cooling_capacity_present", "bool"),
        ("max_cooling_capacity_w", "f64"),
        ("outdoor_air_per_person_m3_s", "f64"),
        ("outdoor_air_per_area_m3_s_m2", "f64"),
    ],
    ctor="id: payload.id, zone_id: payload.zone_id, max_heating_supply_air_temp_c: payload.max_heating_supply_air_temp_c, min_cooling_supply_air_temp_c: payload.min_cooling_supply_air_temp_c, max_heating_capacity_w: payload.max_heating_capacity_present.then_some(payload.max_heating_capacity_w), max_cooling_capacity_w: payload.max_cooling_capacity_present.then_some(payload.max_cooling_capacity_w), outdoor_air_per_person_m3_s: payload.outdoor_air_per_person_m3_s, outdoor_air_per_area_m3_s_m2: payload.outdoor_air_per_area_m3_s_m2",
    recreate="item.id, item.zone_id, item.max_heating_supply_air_temp_c, item.min_cooling_supply_air_temp_c, item.max_heating_capacity_w.is_some(), item.max_heating_capacity_w.unwrap_or(0.0), item.max_cooling_capacity_w.is_some(), item.max_cooling_capacity_w.unwrap_or(0.0), item.outdoor_air_per_person_m3_s, item.outdoor_air_per_area_m3_s_m2",
    py_new='{"id": payload["id"], "zone_id": payload["zoneId"], "max_heating_supply_air_temp_c": payload["maxHeatingSupplyAirTempC"], "min_cooling_supply_air_temp_c": payload["minCoolingSupplyAirTempC"], "max_heating_capacity_w": payload["maxHeatingCapacityW"] if payload["maxHeatingCapacityPresent"] else None, "max_cooling_capacity_w": payload["maxCoolingCapacityW"] if payload["maxCoolingCapacityPresent"] else None, "outdoor_air_per_person_m3_s": payload["outdoorAirPerPersonM3S"], "outdoor_air_per_area_m3_s_m2": payload["outdoorAirPerAreaM3SM2"]}',
    py_recreate='{"id": item["id"], "zoneId": item["zone_id"], "maxHeatingSupplyAirTempC": item["max_heating_supply_air_temp_c"], "minCoolingSupplyAirTempC": item["min_cooling_supply_air_temp_c"], "maxHeatingCapacityPresent": item["max_heating_capacity_w"] is not None, "maxHeatingCapacityW": item["max_heating_capacity_w"] if item["max_heating_capacity_w"] is not None else 0.0, "maxCoolingCapacityPresent": item["max_cooling_capacity_w"] is not None, "maxCoolingCapacityW": item["max_cooling_capacity_w"] if item["max_cooling_capacity_w"] is not None else 0.0, "outdoorAirPerPersonM3S": item["outdoor_air_per_person_m3_s"], "outdoorAirPerAreaM3SM2": item["outdoor_air_per_area_m3_s_m2"]}',
    seed=G3_MODEL + G3_IDEAL_ROW,
    bare=G3_MODEL,
    probe_args="crate::model::EntityId(12), crate::model::EntityId(1), 50.0, 13.0, false, 0.0, false, 0.0, 0.0, 0.0",
)

g3_create(
    number=514,
    emoji="🫁",
    entity=IDEAL_LOADS,
    entity_slug="ideal-loads-system",
    doc="Creates the ideal-loads air system that serves one zone. Both capacity limits are optional — `present` false is the autosized reading, and then the value has to be zero.",
    checks=[
        g3_chk_zone("zone_id"),
        g3_chk_range("max_heating_supply_air_temp_c", "id", "-100.0", "200.0", "A heating supply air temperature"),
        g3_chk_range("min_cooling_supply_air_temp_c", "id", "-100.0", "200.0", "A cooling supply air temperature"),
        g3_chk_non_negative("outdoor_air_per_person_m3_s", "id", "An outdoor air rate per person"),
        g3_chk_non_negative("outdoor_air_per_area_m3_s_m2", "id", "An outdoor air rate per floor area"),
        g3_chk_absent_zero("max_heating_capacity_present", "max_heating_capacity_w", "id", "heating capacity"),
        g3_chk_absent_zero("max_cooling_capacity_present", "max_cooling_capacity_w", "id", "cooling capacity"),
        g3_chk_when("max_heating_capacity_present", g3_chk_positive("max_heating_capacity_w", "id", "A stated heating capacity")),
        g3_chk_when("max_cooling_capacity_present", g3_chk_positive("max_cooling_capacity_w", "id", "A stated cooling capacity")),
    ],
    happy_args="crate::model::EntityId(12), crate::model::EntityId(1), 50.0, 13.0, false, 0.0, false, 0.0, 0.0, 0.0",
    happy_case="serves-zone-one",
    happy_desc="serves the first zone with autosized ideal loads",
    refusal_args="crate::model::EntityId(12), crate::model::EntityId(99), 50.0, 13.0, false, 0.0, false, 0.0, 0.0, 0.0",
    refusal_case="refuses-an-absent-zone",
    refusal_desc="refuses an ideal loads system on an absent zone",
    refusal_seed=G3_MODEL,
)

g3_delete(
    number=515,
    emoji="🫥",
    entity=IDEAL_LOADS,
    entity_slug="ideal-loads-system",
    doc="Drops the ideal-loads air system off its zone. Refused when no system carries the id.",
    happy_args="crate::model::EntityId(12)",
    happy_case="drops-the-system",
    happy_desc="drops the ideal loads system",
    refusal_args="crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-one",
    refusal_desc="refuses an absent ideal loads system",
)

g3_change(
    number=516,
    emoji="🏢",
    slug="change-ideal-loads-system-zone",
    entity=IDEAL_LOADS,
    entity_slug="ideal-loads-system",
    field="zone_id",
    ty="crate::model::EntityId",
    doc="Moves the ideal-loads air system to another zone.",
    what="zone",
    checks=[g3_chk_zone("new_zone_id")],
    happy="crate::model::EntityId(2)",
    happy_case="moves-to-zone-two",
    happy_desc="moves the system to the second zone",
    refusal="crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-zone",
    refusal_desc="refuses a zone that does not exist",
    probe_value="crate::model::EntityId(2)",
)

g3_change(
    number=517,
    emoji="🔴",
    slug="change-ideal-loads-system-max-heating-supply-air-temp",
    entity=IDEAL_LOADS,
    entity_slug="ideal-loads-system",
    field="max_heating_supply_air_temp_c",
    ty="f64",
    doc="Sets the warmest air the ideal-loads system may deliver, in °C.",
    what="maximum heating supply air temperature",
    checks=[g3_chk_range("new_max_heating_supply_air_temp_c", "id", "-100.0", "200.0", "A heating supply air temperature")],
    happy="45.0",
    happy_case="cools-the-supply",
    happy_desc="lowers the heating supply air limit",
    refusal="1000.0",
    refusal_case="refuses-a-hot-supply",
    refusal_desc="refuses an unrepresentable supply temperature",
    probe_value="45.0",
)

g3_change(
    number=518,
    emoji="🔵",
    slug="change-ideal-loads-system-min-cooling-supply-air-temp",
    entity=IDEAL_LOADS,
    entity_slug="ideal-loads-system",
    field="min_cooling_supply_air_temp_c",
    ty="f64",
    doc="Sets the coldest air the ideal-loads system may deliver, in °C.",
    what="minimum cooling supply air temperature",
    checks=[g3_chk_range("new_min_cooling_supply_air_temp_c", "id", "-100.0", "200.0", "A cooling supply air temperature")],
    happy="12.0",
    happy_case="lowers-the-supply",
    happy_desc="lowers the cooling supply air limit",
    refusal="-500.0",
    refusal_case="refuses-a-cold-supply",
    refusal_desc="refuses an unrepresentable supply temperature",
    probe_value="12.0",
)

g3_change_optional(
    number=519,
    emoji="⛽",
    slug="change-ideal-loads-system-max-heating-capacity",
    entity=IDEAL_LOADS,
    entity_slug="ideal-loads-system",
    field="max_heating_capacity_w",
    doc="Sets or clears the ideal-loads heating capacity limit in watts. Cleared means autosized.",
    what="maximum heating capacity",
    present_field="new_capacity_present",
    value_field="new_max_heating_capacity_w",
    checks=[g3_chk_positive("new_max_heating_capacity_w", "id", "A stated heating capacity")],
    happy_args="crate::model::EntityId(12), true, 4000.0",
    happy_case="caps-the-heating",
    happy_desc="caps the heating capacity at four kilowatts",
    refusal_args="crate::model::EntityId(12), false, 4000.0",
    refusal_case="refuses-a-stray-value",
    refusal_desc="refuses an autosized capacity that still carries a value",
    probe_args="crate::model::EntityId(12), true, 4000.0",
)

g3_change_optional(
    number=520,
    emoji="🟧",
    slug="change-ideal-loads-system-max-cooling-capacity",
    entity=IDEAL_LOADS,
    entity_slug="ideal-loads-system",
    field="max_cooling_capacity_w",
    doc="Sets or clears the ideal-loads cooling capacity limit in watts. Cleared means autosized.",
    what="maximum cooling capacity",
    present_field="new_capacity_present",
    value_field="new_max_cooling_capacity_w",
    checks=[g3_chk_positive("new_max_cooling_capacity_w", "id", "A stated cooling capacity")],
    happy_args="crate::model::EntityId(12), true, 5000.0",
    happy_case="caps-the-cooling",
    happy_desc="caps the cooling capacity at five kilowatts",
    refusal_args="crate::model::EntityId(12), false, 5000.0",
    refusal_case="refuses-a-stray-value",
    refusal_desc="refuses an autosized capacity that still carries a value",
    probe_args="crate::model::EntityId(12), true, 5000.0",
)

g3_change(
    number=521,
    emoji="🧍",
    slug="change-ideal-loads-system-outdoor-air-per-person",
    entity=IDEAL_LOADS,
    entity_slug="ideal-loads-system",
    field="outdoor_air_per_person_m3_s",
    ty="f64",
    doc="Sets the ventilation rate the system draws per occupant, in m³/s.",
    what="outdoor air rate per person",
    checks=[g3_chk_non_negative("new_outdoor_air_per_person_m3_s", "id", "An outdoor air rate per person")],
    happy="0.0025",
    happy_case="ventilates-per-head",
    happy_desc="draws outdoor air per occupant",
    refusal="-1.0",
    refusal_case="refuses-a-negative",
    refusal_desc="refuses a negative ventilation rate",
    probe_value="0.0025",
)

g3_change(
    number=522,
    emoji="🔳",
    slug="change-ideal-loads-system-outdoor-air-per-area",
    entity=IDEAL_LOADS,
    entity_slug="ideal-loads-system",
    field="outdoor_air_per_area_m3_s_m2",
    ty="f64",
    doc="Sets the ventilation rate the system draws per square metre of floor, in m³/s·m².",
    what="outdoor air rate per floor area",
    checks=[g3_chk_non_negative("new_outdoor_air_per_area_m3_s_m2", "id", "An outdoor air rate per floor area")],
    happy="0.0003",
    happy_case="ventilates-per-area",
    happy_desc="draws outdoor air per floor area",
    refusal="-1.0",
    refusal_case="refuses-a-negative",
    refusal_desc="refuses a negative ventilation rate",
    probe_value="0.0003",
)

G3_EQUIPMENT_ROW = '''
    model.zone_equipment.push(crate::model::ZoneEquipmentAssignment { id: crate::model::EntityId(13), zone_id: crate::model::EntityId(1), equipment_type: crate::model::ZoneEquipmentType::Baseboard, priority: 1, heating_capacity_w: 2000.0, cooling_capacity_w: 0.0 });'''

ZONE_EQUIPMENT = G3Entity(
    coll="zone_equipment",
    display="Zone equipment",
    struct="ZoneEquipmentAssignment",
    noun="zone equipment",
    payload=[
        ("id", "crate::model::EntityId"),
        ("zone_id", "crate::model::EntityId"),
        ("equipment_type", "crate::model::ZoneEquipmentType"),
        ("priority", "u8"),
        ("heating_capacity_w", "f64"),
        ("cooling_capacity_w", "f64"),
    ],
    seed=G3_MODEL + G3_EQUIPMENT_ROW,
    bare=G3_MODEL,
    probe_args="crate::model::EntityId(13), crate::model::EntityId(1), crate::model::ZoneEquipmentType::Baseboard, 1, 2000.0, 0.0",
)

g3_create(
    number=523,
    emoji="🛠",
    entity=ZONE_EQUIPMENT,
    entity_slug="zone-equipment",
    doc="Adds one piece of zone equipment to a zone's own equipment list. `priority` is its rank in that list, so it starts at one.",
    checks=[
        g3_chk_zone("zone_id"),
        g3_chk_at_least_one("priority", "id", "An equipment priority"),
        g3_chk_non_negative("heating_capacity_w", "id", "A heating capacity"),
        g3_chk_non_negative("cooling_capacity_w", "id", "A cooling capacity"),
    ],
    happy_args="crate::model::EntityId(13), crate::model::EntityId(1), crate::model::ZoneEquipmentType::Baseboard, 1, 2000.0, 0.0",
    happy_case="adds-a-baseboard",
    happy_desc="adds a baseboard to the first zone",
    refusal_args="crate::model::EntityId(13), crate::model::EntityId(1), crate::model::ZoneEquipmentType::Baseboard, 0, 2000.0, 0.0",
    refusal_case="refuses-rank-zero",
    refusal_desc="refuses an equipment rank of zero",
    refusal_seed=G3_MODEL,
)

g3_delete(
    number=524,
    emoji="🗑",
    entity=ZONE_EQUIPMENT,
    entity_slug="zone-equipment",
    doc="Removes one piece of zone equipment. Refused when no assignment carries the id.",
    happy_args="crate::model::EntityId(13)",
    happy_case="drops-the-baseboard",
    happy_desc="removes the baseboard",
    refusal_args="crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-one",
    refusal_desc="refuses absent zone equipment",
)

g3_change(
    number=525,
    emoji="🏬",
    slug="change-zone-equipment-zone",
    entity=ZONE_EQUIPMENT,
    entity_slug="zone-equipment",
    field="zone_id",
    ty="crate::model::EntityId",
    doc="Moves one piece of zone equipment to another zone.",
    what="zone",
    checks=[g3_chk_zone("new_zone_id")],
    happy="crate::model::EntityId(2)",
    happy_case="moves-to-zone-two",
    happy_desc="moves the equipment to the second zone",
    refusal="crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-zone",
    refusal_desc="refuses a zone that does not exist",
    probe_value="crate::model::EntityId(2)",
)

g3_change(
    number=526,
    emoji="🔧",
    slug="change-zone-equipment-type",
    entity=ZONE_EQUIPMENT,
    entity_slug="zone-equipment",
    field="equipment_type",
    ty="crate::model::ZoneEquipmentType",
    doc="Swaps the catalog type the assignment names. The eight-way enum is one scalar, not eight fields, so it changes as a whole.",
    what="equipment type",
    checks=[],
    happy="crate::model::ZoneEquipmentType::FanCoil",
    happy_case="swaps-to-a-fan-coil",
    happy_desc="swaps the baseboard for a fan coil",
    probe_value="crate::model::ZoneEquipmentType::FanCoil",
)

g3_change(
    number=527,
    emoji="🎗",
    slug="change-zone-equipment-priority",
    entity=ZONE_EQUIPMENT,
    entity_slug="zone-equipment",
    field="priority",
    ty="u8",
    doc="Sets the rank this equipment holds in its zone's own equipment list.",
    what="priority",
    checks=[g3_chk_at_least_one("new_priority", "id", "An equipment priority")],
    happy="2",
    happy_case="demotes-it",
    happy_desc="demotes the equipment to second rank",
    refusal="0",
    refusal_case="refuses-rank-zero",
    refusal_desc="refuses a rank of zero",
    probe_value="2",
)

g3_change(
    number=528,
    emoji="🧇",
    slug="change-zone-equipment-heating-capacity",
    entity=ZONE_EQUIPMENT,
    entity_slug="zone-equipment",
    field="heating_capacity_w",
    ty="f64",
    doc="Sets the heating output this equipment can deliver, in watts.",
    what="heating capacity",
    checks=[g3_chk_non_negative("new_heating_capacity_w", "id", "A heating capacity")],
    happy="3000.0",
    happy_case="uprates-heating",
    happy_desc="uprates the heating capacity",
    refusal="-1.0",
    refusal_case="refuses-a-negative",
    refusal_desc="refuses a negative capacity",
    probe_value="3000.0",
)

g3_change(
    number=529,
    emoji="🍧",
    slug="change-zone-equipment-cooling-capacity",
    entity=ZONE_EQUIPMENT,
    entity_slug="zone-equipment",
    field="cooling_capacity_w",
    ty="f64",
    doc="Sets the cooling output this equipment can deliver, in watts.",
    what="cooling capacity",
    checks=[g3_chk_non_negative("new_cooling_capacity_w", "id", "A cooling capacity")],
    happy="1500.0",
    happy_case="uprates-cooling",
    happy_desc="uprates the cooling capacity",
    refusal="-1.0",
    refusal_case="refuses-a-negative",
    refusal_desc="refuses a negative capacity",
    probe_value="1500.0",
)

G3_DAYLIGHT_ROW = '''
    model.daylight_zones.push(crate::model::DaylightZoneConfig { id: crate::model::EntityId(14), zone_id: crate::model::EntityId(1), illuminance_target_lux: 500.0, glare_limit: 22.0, window_transmittance: 0.6 });'''

DAYLIGHT_ZONE = G3Entity(
    coll="daylight_zones",
    display="Daylight zone",
    struct="DaylightZoneConfig",
    noun="daylight zone",
    payload=[
        ("id", "crate::model::EntityId"),
        ("zone_id", "crate::model::EntityId"),
        ("illuminance_target_lux", "f64"),
        ("glare_limit", "f64"),
        ("window_transmittance", "f64"),
    ],
    seed=G3_MODEL + G3_DAYLIGHT_ROW,
    bare=G3_MODEL,
    probe_args="crate::model::EntityId(14), crate::model::EntityId(1), 500.0, 22.0, 0.6",
)

g3_create(
    number=530,
    emoji="🔭",
    entity=DAYLIGHT_ZONE,
    entity_slug="daylight-zone",
    doc="Puts a daylighting control on one zone: an illuminance target the electric lighting dims toward, a glare index limit and the visible transmittance the control sees the window through.",
    checks=[
        g3_chk_zone("zone_id"),
        g3_chk_positive("illuminance_target_lux", "id", "An illuminance target"),
        g3_chk_positive("glare_limit", "id", "A glare limit"),
        g3_chk_fraction("window_transmittance", "id", "A window transmittance"),
    ],
    happy_args="crate::model::EntityId(14), crate::model::EntityId(1), 500.0, 22.0, 0.6",
    happy_case="lights-zone-one",
    happy_desc="daylights the first zone",
    refusal_args="crate::model::EntityId(14), crate::model::EntityId(1), 500.0, 22.0, 1.4",
    refusal_case="refuses-a-bad-tau",
    refusal_desc="refuses a transmittance above one",
    refusal_seed=G3_MODEL,
)

g3_delete(
    number=531,
    emoji="🌗",
    entity=DAYLIGHT_ZONE,
    entity_slug="daylight-zone",
    doc="Removes the daylighting control from its zone. Refused when no control carries the id.",
    happy_args="crate::model::EntityId(14)",
    happy_case="darkens-the-zone",
    happy_desc="removes the daylighting control",
    refusal_args="crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-one",
    refusal_desc="refuses an absent daylight zone",
)

g3_change(
    number=532,
    emoji="🏫",
    slug="change-daylight-zone-zone",
    entity=DAYLIGHT_ZONE,
    entity_slug="daylight-zone",
    field="zone_id",
    ty="crate::model::EntityId",
    doc="Moves the daylighting control to another zone.",
    what="zone",
    checks=[g3_chk_zone("new_zone_id")],
    happy="crate::model::EntityId(2)",
    happy_case="moves-to-zone-two",
    happy_desc="moves the control to the second zone",
    refusal="crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-zone",
    refusal_desc="refuses a zone that does not exist",
    probe_value="crate::model::EntityId(2)",
)

g3_change(
    number=533,
    emoji="🪔",
    slug="change-daylight-zone-illuminance-target",
    entity=DAYLIGHT_ZONE,
    entity_slug="daylight-zone",
    field="illuminance_target_lux",
    ty="f64",
    doc="Sets the working-plane illuminance the electric lighting dims toward, in lux.",
    what="illuminance target",
    checks=[g3_chk_positive("new_illuminance_target_lux", "id", "An illuminance target")],
    happy="300.0",
    happy_case="dims-the-target",
    happy_desc="lowers the illuminance target",
    refusal="0.0",
    refusal_case="refuses-a-dark-target",
    refusal_desc="refuses a zero illuminance target",
    probe_value="300.0",
)

g3_change(
    number=534,
    emoji="🕶",
    slug="change-daylight-zone-glare-limit",
    entity=DAYLIGHT_ZONE,
    entity_slug="daylight-zone",
    field="glare_limit",
    ty="f64",
    doc="Sets the maximum discomfort glare index the control tolerates.",
    what="glare limit",
    checks=[g3_chk_positive("new_glare_limit", "id", "A glare limit")],
    happy="20.0",
    happy_case="tightens-glare",
    happy_desc="tightens the glare limit",
    refusal="-5.0",
    refusal_case="refuses-a-negative",
    refusal_desc="refuses a negative glare limit",
    probe_value="20.0",
)

g3_change(
    number=535,
    emoji="🥃",
    slug="change-daylight-zone-window-transmittance",
    entity=DAYLIGHT_ZONE,
    entity_slug="daylight-zone",
    field="window_transmittance",
    ty="f64",
    doc="Sets the visible transmittance the daylight control sees its window through, as a fraction.",
    what="window transmittance",
    checks=[g3_chk_fraction("new_window_transmittance", "id", "A window transmittance")],
    happy="0.5",
    happy_case="darkens-the-glass",
    happy_desc="darkens the glazing the control sees",
    refusal="1.5",
    refusal_case="refuses-a-bad-tau",
    refusal_desc="refuses a transmittance above one",
    probe_value="0.5",
)

G3_SIZING_ROW = '''
    model.sizing_objects.push(crate::model::SizingObject { id: crate::model::EntityId(15), zone_id: crate::model::EntityId(1), sizing_type: crate::model::SizingType::Heating, design_day_type: crate::model::DesignDayType::Heating });'''

SIZING_OBJECT = G3Entity(
    coll="sizing_objects",
    display="Sizing object",
    struct="SizingObject",
    noun="sizing object",
    payload=[
        ("id", "crate::model::EntityId"),
        ("zone_id", "crate::model::EntityId"),
        ("sizing_type", "crate::model::SizingType"),
        ("design_day_type", "crate::model::DesignDayType"),
    ],
    seed=G3_MODEL + G3_SIZING_ROW,
    bare=G3_MODEL,
    probe_args="crate::model::EntityId(15), crate::model::EntityId(1), crate::model::SizingType::Heating, crate::model::DesignDayType::Heating",
)

g3_create(
    number=536,
    emoji="📶",
    entity=SIZING_OBJECT,
    entity_slug="sizing-object",
    doc="Declares that one zone is autosized against a design day: which load the sizing run solves for and which design day it reads.",
    checks=[g3_chk_zone("zone_id")],
    happy_args="crate::model::EntityId(15), crate::model::EntityId(1), crate::model::SizingType::Heating, crate::model::DesignDayType::Heating",
    happy_case="sizes-zone-one",
    happy_desc="autosizes the first zone for heating",
    refusal_args="crate::model::EntityId(15), crate::model::EntityId(99), crate::model::SizingType::Heating, crate::model::DesignDayType::Heating",
    refusal_case="refuses-an-absent-zone",
    refusal_desc="refuses a sizing object on an absent zone",
    refusal_seed=G3_MODEL,
)

g3_delete(
    number=537,
    emoji="🪒",
    entity=SIZING_OBJECT,
    entity_slug="sizing-object",
    doc="Drops one zone's sizing declaration. Refused when no sizing object carries the id.",
    happy_args="crate::model::EntityId(15)",
    happy_case="drops-the-sizing",
    happy_desc="drops the sizing object",
    refusal_args="crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-one",
    refusal_desc="refuses an absent sizing object",
)

g3_change(
    number=538,
    emoji="🏨",
    slug="change-sizing-object-zone",
    entity=SIZING_OBJECT,
    entity_slug="sizing-object",
    field="zone_id",
    ty="crate::model::EntityId",
    doc="Moves the sizing declaration to another zone.",
    what="zone",
    checks=[g3_chk_zone("new_zone_id")],
    happy="crate::model::EntityId(2)",
    happy_case="moves-to-zone-two",
    happy_desc="sizes the second zone instead",
    refusal="crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-zone",
    refusal_desc="refuses a zone that does not exist",
    probe_value="crate::model::EntityId(2)",
)

g3_change(
    number=539,
    emoji="🧾",
    slug="change-sizing-object-sizing-type",
    entity=SIZING_OBJECT,
    entity_slug="sizing-object",
    field="sizing_type",
    ty="crate::model::SizingType",
    doc="Swaps which load the sizing run solves for.",
    what="sizing type",
    checks=[],
    happy="crate::model::SizingType::Cooling",
    happy_case="sizes-for-cooling",
    happy_desc="sizes for the cooling load instead",
    probe_value="crate::model::SizingType::Cooling",
)

g3_change(
    number=540,
    emoji="🌥",
    slug="change-sizing-object-design-day-type",
    entity=SIZING_OBJECT,
    entity_slug="sizing-object",
    field="design_day_type",
    ty="crate::model::DesignDayType",
    doc="Swaps which design day the sizing run reads.",
    what="design day type",
    checks=[],
    happy="crate::model::DesignDayType::Cooling",
    happy_case="reads-a-hot-day",
    happy_desc="reads the cooling design day instead",
    probe_value="crate::model::DesignDayType::Cooling",
)

G3_ROOM_AIR_ROW = '''
    model.room_air_models.push(crate::model::RoomAirModelAssignment { zone_id: crate::model::EntityId(1), model: crate::model::RoomAirModelType::WellMixed });'''

ROOM_AIR = G3Entity(
    coll="room_air_models",
    display="Room air model assignment for zone",
    struct="RoomAirModelAssignment",
    noun="room air model assignment",
    key="zone_id",
    payload=[("zone_id", "crate::model::EntityId"), ("model", "crate::model::RoomAirModelType")],
    seed=G3_MODEL + G3_ROOM_AIR_ROW,
    bare=G3_MODEL,
    probe_args="crate::model::EntityId(1), crate::model::RoomAirModelType::WellMixed",
)

g3_create(
    number=541,
    emoji="🛏",
    entity=ROOM_AIR,
    entity_slug="room-air-model-assignment",
    doc="Overrides the room air model of one zone. This collection carries no id of its own — `zone_id` IS the key (vocabulary §5.5), so it is a keyed map and a second override for one zone is a duplicate.",
    checks=[g3_chk_zone("zone_id")],
    happy_args="crate::model::EntityId(2), crate::model::RoomAirModelType::TwoNodeBuoyancy",
    happy_case="stratifies-zone-two",
    happy_desc="gives the second zone a two-node room air model",
    refusal_args="crate::model::EntityId(1), crate::model::RoomAirModelType::WellMixed",
    refusal_case="refuses-a-second-one",
    refusal_desc="refuses a second override for one zone",
)

g3_delete(
    number=542,
    emoji="🧺",
    entity=ROOM_AIR,
    entity_slug="room-air-model-assignment",
    doc="Drops one zone's room air model override, so the zone falls back to the engine's own well-mixed default.",
    happy_args="crate::model::EntityId(1)",
    happy_case="falls-back",
    happy_desc="drops the override off the first zone",
    refusal_args="crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-one",
    refusal_desc="refuses a zone with no override",
)

g3_change(
    number=543,
    emoji="🪭",
    slug="change-room-air-model",
    entity=ROOM_AIR,
    entity_slug="room-air-model-assignment",
    field="model",
    ty="crate::model::RoomAirModelType",
    doc="Swaps the room air model one zone's override names.",
    what="room air model",
    checks=[],
    happy="crate::model::RoomAirModelType::TwoNodeBuoyancy",
    happy_case="stratifies-the-air",
    happy_desc="swaps the well-mixed model for a two-node one",
    probe_value="crate::model::RoomAirModelType::TwoNodeBuoyancy",
)


G3_SPM_NAMES = '"Scheduled" | "OutdoorAirReset" | "WarmestZone" | "ColdestZone"'
G3_SPM_PY_NAMES = '("Scheduled", "OutdoorAirReset", "WarmestZone", "ColdestZone")'


def g3_spm_kind(prefix: str) -> str:
    """🎛️ The tagged `SetpointManagerKind` rebuilt from its flattened payload — a tagged union has no
    `dsl::DslField`, so the variant travels as its wire name beside the four reset limits, and only
    the reset variant is allowed to carry them."""
    return f'''    let kind = if payload.{prefix}kind == "OutdoorAirReset" {{
        crate::model::SetpointManagerKind::OutdoorAirReset {{ low_outdoor_c: payload.{prefix}low_outdoor_c, high_outdoor_c: payload.{prefix}high_outdoor_c, low_setpoint_c: payload.{prefix}low_setpoint_c, high_setpoint_c: payload.{prefix}high_setpoint_c }}
    }} else if payload.{prefix}kind == "WarmestZone" {{
        crate::model::SetpointManagerKind::WarmestZone
    }} else if payload.{prefix}kind == "ColdestZone" {{
        crate::model::SetpointManagerKind::ColdestZone
    }} else {{
        crate::model::SetpointManagerKind::Scheduled
    }};
'''


def g3_spm_flat(field: str) -> str:
    """🎛️ One flattened component of a stored `SetpointManagerKind`, as an expression."""
    if field == "kind":
        return 'match &item.kind { crate::model::SetpointManagerKind::OutdoorAirReset { .. } => "OutdoorAirReset".to_string(), crate::model::SetpointManagerKind::WarmestZone => "WarmestZone".to_string(), crate::model::SetpointManagerKind::ColdestZone => "ColdestZone".to_string(), crate::model::SetpointManagerKind::Scheduled => "Scheduled".to_string() }'
    return f"match &item.kind {{ crate::model::SetpointManagerKind::OutdoorAirReset {{ {field}, .. }} => *{field}, _ => 0.0 }}"


def g3_py_spm_kind(prefix: str) -> str:
    """🎛️ The same flattening in the second implementation, keyed by the payload's own camelCase
    field names — the wire spelling, not the Rust one."""
    key = lambda name: field_camel(prefix + name)
    reset = ", ".join(f'"{name}": payload["{key(name)}"]' for name in ("low_outdoor_c", "high_outdoor_c", "low_setpoint_c", "high_setpoint_c"))
    return f'({{"OutdoorAirReset": {{{reset}}}}} if payload["{key("kind")}"] == "OutdoorAirReset" else payload["{key("kind")}"])'


G3_SPM_ROW = '''
    model.setpoint_managers.push(crate::model::SetpointManager { id: crate::model::EntityId(16), name: "SUPPLY SPM".into(), kind: crate::model::SetpointManagerKind::Scheduled, schedule_id: Some(crate::model::ScheduleId(1)) });'''

SETPOINT_MANAGER = G3Entity(
    coll="setpoint_managers",
    display="Setpoint manager",
    struct="SetpointManager",
    noun="setpoint manager",
    payload=[
        ("id", "crate::model::EntityId"),
        ("name", "String"),
        ("kind", "String"),
        ("low_outdoor_c", "f64"),
        ("high_outdoor_c", "f64"),
        ("low_setpoint_c", "f64"),
        ("high_setpoint_c", "f64"),
        ("schedule_present", "bool"),
        ("schedule_id", "crate::model::ScheduleId"),
    ],
    pre=g3_spm_kind(""),
    ctor="id: payload.id, name: payload.name.clone(), kind, schedule_id: payload.schedule_present.then_some(payload.schedule_id)",
    recreate="item.id, item.name.clone(), " + g3_spm_flat("kind") + ", " + g3_spm_flat("low_outdoor_c") + ", " + g3_spm_flat("high_outdoor_c") + ", " + g3_spm_flat("low_setpoint_c") + ", " + g3_spm_flat("high_setpoint_c") + ", item.schedule_id.is_some(), item.schedule_id.unwrap_or(crate::model::ScheduleId(0))",
    py_new='{"id": payload["id"], "name": payload["name"], "kind": ' + g3_py_spm_kind("") + ', "schedule_id": (payload["scheduleId"] if payload["schedulePresent"] else None)}',
    py_recreate='{"id": item["id"], "name": item["name"], "kind": ("OutdoorAirReset" if isinstance(item["kind"], dict) else item["kind"]), "lowOutdoorC": (item["kind"]["OutdoorAirReset"]["low_outdoor_c"] if isinstance(item["kind"], dict) else 0.0), "highOutdoorC": (item["kind"]["OutdoorAirReset"]["high_outdoor_c"] if isinstance(item["kind"], dict) else 0.0), "lowSetpointC": (item["kind"]["OutdoorAirReset"]["low_setpoint_c"] if isinstance(item["kind"], dict) else 0.0), "highSetpointC": (item["kind"]["OutdoorAirReset"]["high_setpoint_c"] if isinstance(item["kind"], dict) else 0.0), "schedulePresent": item["schedule_id"] is not None, "scheduleId": (item["schedule_id"] if item["schedule_id"] is not None else 0)}',
    seed=G3_MODEL + G3_SPM_ROW,
    bare=G3_MODEL,
    probe_args='crate::model::EntityId(16), "SUPPLY SPM".to_string(), "Scheduled".to_string(), 0.0, 0.0, 0.0, 0.0, true, crate::model::ScheduleId(1)',
)

G3_SPM_UNKNOWN = G3Check(
    "mutation.invariant",
    f"!matches!(payload.kind.as_str(), {G3_SPM_NAMES})",
    'format!("{:?} is not a setpoint manager kind.", payload.kind)',
    "payload.id.0.to_string()",
    f'payload["kind"] not in {G3_SPM_PY_NAMES}',
    'str(payload["id"])',
)

G3_SPM_STRAY = G3Check(
    "mutation.invariant",
    'payload.kind != "OutdoorAirReset" && !(payload.low_outdoor_c == 0.0 && payload.high_outdoor_c == 0.0 && payload.low_setpoint_c == 0.0 && payload.high_setpoint_c == 0.0)',
    '"Only an OutdoorAirReset setpoint manager carries reset limits.".to_string()',
    "payload.id.0.to_string()",
    'payload["kind"] != "OutdoorAirReset" and not (payload["lowOutdoorC"] == 0.0 and payload["highOutdoorC"] == 0.0 and payload["lowSetpointC"] == 0.0 and payload["highSetpointC"] == 0.0)',
    'str(payload["id"])',
)

G3_SPM_ORDER = G3Check(
    "mutation.invariant",
    'payload.kind == "OutdoorAirReset" && payload.high_outdoor_c <= payload.low_outdoor_c',
    '"An outdoor air reset needs a high outdoor temperature above its low one.".to_string()',
    "payload.id.0.to_string()",
    'payload["kind"] == "OutdoorAirReset" and payload["highOutdoorC"] <= payload["lowOutdoorC"]',
    'str(payload["id"])',
)

g3_create(
    number=600,
    emoji="📌",
    entity=SETPOINT_MANAGER,
    entity_slug="setpoint-manager",
    doc="Creates one loop setpoint manager. `SetpointManagerKind` is a tagged union with no `dsl::DslField`, so the variant travels as its wire name beside the four outdoor-air-reset limits, and every other variant has to carry them as zero.",
    checks=[
        g3_chk_blank("name", "id", "A setpoint manager name"),
        g3_chk_duplicate_name(SETPOINT_MANAGER, "name", False),
        G3_SPM_UNKNOWN,
        G3_SPM_STRAY,
        G3_SPM_ORDER,
        g3_chk_absent_zero_id("schedule_present", "schedule_id", "id", "setpoint manager schedule"),
        g3_chk_when("schedule_present", g3_chk_schedule("schedule_id")),
    ],
    happy_args='crate::model::EntityId(16), "SUPPLY SPM".to_string(), "Scheduled".to_string(), 0.0, 0.0, 0.0, 0.0, true, crate::model::ScheduleId(1)',
    happy_case="adds-a-scheduled-spm",
    happy_desc="adds a scheduled supply setpoint manager",
    refusal_args='crate::model::EntityId(16), "SUPPLY SPM".to_string(), "Sunny".to_string(), 0.0, 0.0, 0.0, 0.0, false, crate::model::ScheduleId(0)',
    refusal_case="refuses-a-bad-kind",
    refusal_desc="refuses a setpoint manager kind that does not exist",
    refusal_seed=G3_MODEL,
)

g3_delete(
    number=601,
    emoji="🍄",
    entity=SETPOINT_MANAGER,
    entity_slug="setpoint-manager",
    doc="Drops one loop setpoint manager. Refused when no manager carries the id.",
    happy_args="crate::model::EntityId(16)",
    happy_case="drops-the-spm",
    happy_desc="drops the supply setpoint manager",
    refusal_args="crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-one",
    refusal_desc="refuses an absent setpoint manager",
)

g3_rename(
    number=602,
    emoji="🖇",
    slug="rename-setpoint-manager",
    entity=SETPOINT_MANAGER,
    entity_slug="setpoint-manager",
    doc="Sets one setpoint manager's identity field.",
    happy='"RESET SPM".into()',
    happy_case="renames-the-spm",
    happy_desc="renames the supply setpoint manager",
    probe_value='"RESET SPM".to_string()',
)

G3_SPM_REPLACE_CHECKS = [
    G3Check(
        "mutation.invariant",
        f"!matches!(payload.new_kind.as_str(), {G3_SPM_NAMES})",
        'format!("{:?} is not a setpoint manager kind.", payload.new_kind)',
        "payload.id.0.to_string()",
        f'payload["newKind"] not in {G3_SPM_PY_NAMES}',
        'str(payload["id"])',
    ),
    G3Check(
        "mutation.invariant",
        'payload.new_kind != "OutdoorAirReset" && !(payload.new_low_outdoor_c == 0.0 && payload.new_high_outdoor_c == 0.0 && payload.new_low_setpoint_c == 0.0 && payload.new_high_setpoint_c == 0.0)',
        '"Only an OutdoorAirReset setpoint manager carries reset limits.".to_string()',
        "payload.id.0.to_string()",
        'payload["newKind"] != "OutdoorAirReset" and not (payload["newLowOutdoorC"] == 0.0 and payload["newHighOutdoorC"] == 0.0 and payload["newLowSetpointC"] == 0.0 and payload["newHighSetpointC"] == 0.0)',
        'str(payload["id"])',
    ),
    G3Check(
        "mutation.invariant",
        'payload.new_kind == "OutdoorAirReset" && payload.new_high_outdoor_c <= payload.new_low_outdoor_c',
        '"An outdoor air reset needs a high outdoor temperature above its low one.".to_string()',
        "payload.id.0.to_string()",
        'payload["newKind"] == "OutdoorAirReset" and payload["newHighOutdoorC"] <= payload["newLowOutdoorC"]',
        'str(payload["id"])',
    ),
]

g3_register(
    number=603,
    emoji="🔃",
    slug="replace-setpoint-manager-kind",
    entity_slug="setpoint-manager",
    doc="Swaps the whole tagged control law of one setpoint manager. `replace`, not `change`, because the payload shape genuinely differs per variant (taxonomy rule for tagged unions); the four outdoor-air-reset limits are carried flattened beside the variant's wire name and must be zero for every other variant.",
    fields=[
        ("id", "crate::model::EntityId"),
        ("new_kind", "String"),
        ("new_low_outdoor_c", "f64"),
        ("new_high_outdoor_c", "f64"),
        ("new_low_setpoint_c", "f64"),
        ("new_high_setpoint_c", "f64"),
    ],
    label='format!("Replace setpoint manager {} control law with {}", self.id.0, self.new_kind)',
    target="vec![self.id.0.to_string()]",
    diff=SETPOINT_MANAGER.missing() + "\n" + g3_rust_checks(G3_SPM_REPLACE_CHECKS) + "\n" + g3_spm_kind("new_") + '''    if existing.kind == kind {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Setpoint manager {} already runs that control law.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.setpoint_managers.iter_mut().find(|item| item.id == payload.id) {
        item.kind = kind;
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse=g3_spm_kind("new_") + f'''    match base.model.setpoint_managers.iter().find(|item| item.id == payload.id) {{
        Some(item) if item.kind != kind && !({g3_refused(G3_SPM_REPLACE_CHECKS)}) => vec![super::replace_setpoint_manager_kind(payload.id, {g3_spm_flat("kind")}, {g3_spm_flat("low_outdoor_c")}, {g3_spm_flat("high_outdoor_c")}, {g3_spm_flat("low_setpoint_c")}, {g3_spm_flat("high_setpoint_c")})],
        _ => Vec::new(),
    }}''',
    outcome_classes=["applied", "warning", "error"],
    cases=[
        (
            "✅️",
            "resets-on-outdoor-air",
            "swaps the scheduled law for an outdoor air reset",
            SETPOINT_MANAGER.scenario('replace_setpoint_manager_kind(crate::model::EntityId(16), "OutdoorAirReset".to_string(), -10.0, 20.0, 16.0, 12.0)'),
        ),
        (
            "⛔️",
            "refuses-stray-limits",
            "refuses reset limits on a variant that carries none",
            SETPOINT_MANAGER.scenario('replace_setpoint_manager_kind(crate::model::EntityId(16), "WarmestZone".to_string(), -10.0, 20.0, 16.0, 12.0)'),
        ),
    ],
    probe='replace_setpoint_manager_kind(crate::model::EntityId(16), "OutdoorAirReset".to_string(), -10.0, 20.0, 16.0, 12.0)',
    python='''    """🔀️ `replace-setpoint-manager-kind` — the whole tagged control law of one manager."""
''' + SETPOINT_MANAGER.py_missing() + "\n" + g3_python_checks(G3_SPM_REPLACE_CHECKS) + '''
    kind = ''' + g3_py_spm_kind("new_") + '''
    if item["kind"] == kind:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["setpoint_managers"]:
        if entry["id"] == payload["id"]:
            entry["kind"] = kind
    return after, applied()''',
    python_invert='''    """↩️ Its own inverse, with the old control law flattened back off BASE."""
''' + SETPOINT_MANAGER.py_row() + '''
    old = item["kind"]
    reset = old["OutdoorAirReset"] if isinstance(old, dict) else None
    return [("replace-setpoint-manager-kind", {"id": payload["id"], "newKind": "OutdoorAirReset" if reset is not None else old, "newLowOutdoorC": reset["low_outdoor_c"] if reset is not None else 0.0, "newHighOutdoorC": reset["high_outdoor_c"] if reset is not None else 0.0, "newLowSetpointC": reset["low_setpoint_c"] if reset is not None else 0.0, "newHighSetpointC": reset["high_setpoint_c"] if reset is not None else 0.0})]''',
)

G3_SPM_SCHEDULE_CHECKS = [
    g3_chk_absent_zero_id("new_schedule_present", "new_schedule_id", "id", "setpoint manager schedule"),
    g3_chk_when("new_schedule_present", g3_chk_schedule("new_schedule_id")),
]

g3_register(
    number=604,
    emoji="🎼",
    slug="change-setpoint-manager-schedule",
    entity_slug="setpoint-manager",
    doc="Points one setpoint manager at a schedule the model defines, or clears the slot. `Option<T>` has no `dsl::DslField`, so the optional reference travels as a `present` flag beside the id and an absent slot has to carry zero.",
    fields=[("id", "crate::model::EntityId"), ("new_schedule_present", "bool"), ("new_schedule_id", "crate::model::ScheduleId")],
    label='format!("Change setpoint manager {} schedule to {:?}", self.id.0, self.new_schedule_present.then_some(self.new_schedule_id.0))',
    target="vec![self.id.0.to_string()]",
    diff=SETPOINT_MANAGER.missing() + "\n" + g3_rust_checks(G3_SPM_SCHEDULE_CHECKS) + '''
    let value = payload.new_schedule_present.then_some(payload.new_schedule_id);
    if existing.schedule_id == value {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Setpoint manager {} already reads that schedule.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.setpoint_managers.iter_mut().find(|item| item.id == payload.id) {
        item.schedule_id = value;
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse=f'''    let value = payload.new_schedule_present.then_some(payload.new_schedule_id);
    match base.model.setpoint_managers.iter().find(|item| item.id == payload.id) {{
        Some(item) if item.schedule_id != value && !({g3_refused(G3_SPM_SCHEDULE_CHECKS)}) => vec![super::change_setpoint_manager_schedule(payload.id, item.schedule_id.is_some(), item.schedule_id.unwrap_or(crate::model::ScheduleId(0)))],
        _ => Vec::new(),
    }}''',
    outcome_classes=["applied", "warning", "error"],
    cases=[
        ("✅️", "repoints-the-spm", "repoints the manager at the second schedule", SETPOINT_MANAGER.scenario("change_setpoint_manager_schedule(crate::model::EntityId(16), true, crate::model::ScheduleId(2))")),
        ("⛔️", "refuses-an-absent-one", "refuses a schedule the model does not define", SETPOINT_MANAGER.scenario("change_setpoint_manager_schedule(crate::model::EntityId(16), true, crate::model::ScheduleId(99))")),
    ],
    probe="change_setpoint_manager_schedule(crate::model::EntityId(16), true, crate::model::ScheduleId(2))",
    python='''    """🎼️ `change-setpoint-manager-schedule` — the optional schedule reference of one manager."""
''' + SETPOINT_MANAGER.py_missing() + "\n" + g3_python_checks(G3_SPM_SCHEDULE_CHECKS) + '''
    value = payload["newScheduleId"] if payload["newSchedulePresent"] else None
    if item["schedule_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["setpoint_managers"]:
        if entry["id"] == payload["id"]:
            entry["schedule_id"] = value
    return after, applied()''',
    python_invert='''    """↩️ Its own inverse, with the old reference read off BASE."""
''' + SETPOINT_MANAGER.py_row() + '''
    old = item["schedule_id"]
    return [("change-setpoint-manager-schedule", {"id": payload["id"], "newSchedulePresent": old is not None, "newScheduleId": old if old is not None else 0})]''',
)

G3_AIR_LOOP_ROW = '''
    model.air_loops.push(crate::model::ModelAirLoop { id: crate::model::EntityId(17), name: "MAIN AIR LOOP".into(), supply_node_id: 1, return_node_id: 2, design_supply_air_flow_m3_s: 1.2, terminal_zone_ids: vec![crate::model::EntityId(1)] });'''

G3_AIR_LOOP_TWO = '''
    model.air_loops.push(crate::model::ModelAirLoop { id: crate::model::EntityId(20), name: "SPARE AIR LOOP".into(), supply_node_id: 3, return_node_id: 4, design_supply_air_flow_m3_s: 0.8, terminal_zone_ids: vec![crate::model::EntityId(2)] });'''

G3_OAS_ROW = '''
    model.outdoor_air_systems.push(crate::model::OutdoorAirSystem { id: crate::model::EntityId(19), air_loop_id: crate::model::EntityId(17), min_oa_flow_m3_s: 0.2, economizer_enabled: false });'''


def g3_chk_id_list(field: str, key: str, what: str) -> G3Check:
    """🚧️ A membership list is ascending and free of duplicates, so a removal's undo lands its id
    back in exactly the slot it left."""
    cam, key_cam = field_camel(field), field_camel(key)
    return G3Check(
        "mutation.invariant",
        f"payload.{field}.windows(2).any(|pair| pair[0].0 >= pair[1].0)",
        f'"{what} is kept ascending and free of duplicates.".to_string()',
        f"payload.{key}.0.to_string()",
        f'any(payload["{cam}"][index] >= payload["{cam}"][index + 1] for index in range(len(payload["{cam}"]) - 1))',
        f'str(payload["{key_cam}"])',
    )


def g3_chk_zone_list(field: str, key: str) -> G3Check:
    """🚧️ Every id in a zone membership list names a zone the model really has."""
    cam, key_cam = field_camel(field), field_camel(key)
    return G3Check(
        "mutation.target-missing",
        f"payload.{field}.iter().any(|entry| !base.model.zones.iter().any(|zone| zone.id == *entry))",
        f'"A terminal zone list names a zone this model does not have.".to_string()',
        f"payload.{key}.0.to_string()",
        f'any(not any(zone["id"] == value for zone in before["model"]["zones"]) for value in payload["{cam}"])',
        f'str(payload["{key_cam}"])',
    )


AIR_LOOP = G3Entity(
    coll="air_loops",
    display="Air loop",
    struct="ModelAirLoop",
    noun="air loop",
    payload=[
        ("id", "crate::model::EntityId"),
        ("name", "String"),
        ("supply_node_id", "u32"),
        ("return_node_id", "u32"),
        ("design_supply_air_flow_m3_s", "f64"),
        ("terminal_zone_ids", "Vec<crate::model::EntityId>"),
    ],
    seed=G3_MODEL + G3_AIR_LOOP_ROW,
    bare=G3_MODEL,
    probe_args='crate::model::EntityId(17), "MAIN AIR LOOP".to_string(), 1, 2, 1.2, vec![crate::model::EntityId(1)]',
)

g3_create(
    number=605,
    emoji="🛞",
    entity=AIR_LOOP,
    entity_slug="air-loop",
    doc="Creates one air loop with its supply and return node ids, its design supply air flow and the zones its terminals serve. The terminal list is kept ascending and free of duplicates so a later membership removal's undo is exact.",
    checks=[
        g3_chk_blank("name", "id", "An air loop name"),
        g3_chk_duplicate_name(AIR_LOOP, "name", False),
        g3_chk_at_least_one("supply_node_id", "id", "A supply node id"),
        g3_chk_at_least_one("return_node_id", "id", "A return node id"),
        g3_chk_positive("design_supply_air_flow_m3_s", "id", "A design supply air flow"),
        g3_chk_id_list("terminal_zone_ids", "id", "A terminal zone list"),
        g3_chk_zone_list("terminal_zone_ids", "id"),
    ],
    happy_args='crate::model::EntityId(17), "MAIN AIR LOOP".to_string(), 1, 2, 1.2, vec![crate::model::EntityId(1)]',
    happy_case="adds-a-main-loop",
    happy_desc="adds the main air loop serving the first zone",
    refusal_args='crate::model::EntityId(17), "MAIN AIR LOOP".to_string(), 1, 2, 1.2, vec![crate::model::EntityId(2), crate::model::EntityId(1)]',
    refusal_case="refuses-a-jumbled-list",
    refusal_desc="refuses a terminal zone list that is not ascending",
    refusal_seed=G3_MODEL,
)

g3_delete(
    number=606,
    emoji="🥀",
    entity=AIR_LOOP,
    entity_slug="air-loop",
    doc="Drops one air loop. Refused while an outdoor air system still names it, rather than cascading — the outdoor air system is a document of its own and its owner decides its fate.",
    checks=[
        G3Check(
            "mutation.invariant",
            "base.model.outdoor_air_systems.iter().any(|system| system.air_loop_id == payload.id)",
            'format!("Air loop {} still serves an outdoor air system.", payload.id.0)',
            "payload.id.0.to_string()",
            'any(system["air_loop_id"] == payload["id"] for system in before["model"]["outdoor_air_systems"])',
            'str(payload["id"])',
        )
    ],
    happy_args="crate::model::EntityId(17)",
    happy_case="drops-the-loop",
    happy_desc="drops the main air loop",
    refusal_args="crate::model::EntityId(17)",
    refusal_case="refuses-a-served-loop",
    refusal_desc="refuses a loop an outdoor air system still names",
    refusal_seed=G3_MODEL + G3_AIR_LOOP_ROW + G3_OAS_ROW,
)

g3_rename(
    number=607,
    emoji="📇",
    slug="rename-air-loop",
    entity=AIR_LOOP,
    entity_slug="air-loop",
    doc="Sets one air loop's identity field.",
    happy='"PRIMARY AIR LOOP".into()',
    happy_case="renames-the-loop",
    happy_desc="renames the main air loop",
    probe_value='"PRIMARY AIR LOOP".to_string()',
)

g3_change(
    number=608,
    emoji="↗",
    slug="change-air-loop-supply-node",
    entity=AIR_LOOP,
    entity_slug="air-loop",
    field="supply_node_id",
    ty="u32",
    doc="Sets the node the air loop supplies from. Node zero is the unset reading, so it is refused.",
    what="supply node",
    checks=[g3_chk_at_least_one("new_supply_node_id", "id", "A supply node id")],
    happy="5",
    happy_case="repoints-supply",
    happy_desc="repoints the supply node",
    refusal="0",
    refusal_case="refuses-node-zero",
    refusal_desc="refuses the unset node id",
    probe_value="5",
)

g3_change(
    number=609,
    emoji="↘",
    slug="change-air-loop-return-node",
    entity=AIR_LOOP,
    entity_slug="air-loop",
    field="return_node_id",
    ty="u32",
    doc="Sets the node the air loop returns to. Node zero is the unset reading, so it is refused.",
    what="return node",
    checks=[g3_chk_at_least_one("new_return_node_id", "id", "A return node id")],
    happy="6",
    happy_case="repoints-return",
    happy_desc="repoints the return node",
    refusal="0",
    refusal_case="refuses-node-zero",
    refusal_desc="refuses the unset node id",
    probe_value="6",
)

g3_change(
    number=610,
    emoji="🍥",
    slug="change-air-loop-design-supply-air-flow",
    entity=AIR_LOOP,
    entity_slug="air-loop",
    field="design_supply_air_flow_m3_s",
    ty="f64",
    doc="Sets the volumetric air flow the loop is designed around, in m³/s.",
    what="design supply air flow",
    checks=[g3_chk_positive("new_design_supply_air_flow_m3_s", "id", "A design supply air flow")],
    happy="1.6",
    happy_case="uprates-the-flow",
    happy_desc="uprates the design supply air flow",
    refusal="0.0",
    refusal_case="refuses-no-flow",
    refusal_desc="refuses a zero design flow",
    probe_value="1.6",
)

g3_member_add(
    number=611,
    emoji="🪺",
    slug="add-air-loop-terminal-zone",
    entity=AIR_LOOP,
    entity_slug="air-loop",
    list_field="terminal_zone_ids",
    member="zone_id",
    member_display="terminal zone",
    doc="Puts one more zone on the air loop's terminal list, at its ascending position.",
    checks=[g3_chk_zone("zone_id")],
    remove_slug="remove-air-loop-terminal-zone",
    happy_args="crate::model::EntityId(17), crate::model::EntityId(2)",
    happy_case="serves-zone-two",
    happy_desc="puts the second zone on the loop",
    refusal_args="crate::model::EntityId(17), crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-zone",
    refusal_desc="refuses a zone that does not exist",
    probe_args="crate::model::EntityId(17), crate::model::EntityId(2)",
)

g3_member_remove(
    number=612,
    emoji="🪹",
    slug="remove-air-loop-terminal-zone",
    entity=AIR_LOOP,
    entity_slug="air-loop",
    list_field="terminal_zone_ids",
    member="zone_id",
    member_display="terminal zone",
    doc="Takes one zone off the air loop's terminal list. Refused when the loop does not serve it.",
    add_slug="add-air-loop-terminal-zone",
    happy_args="crate::model::EntityId(17), crate::model::EntityId(1)",
    happy_case="stops-serving-one",
    happy_desc="takes the first zone off the loop",
    refusal_args="crate::model::EntityId(17), crate::model::EntityId(2)",
    refusal_case="refuses-an-unserved",
    refusal_desc="refuses a zone the loop does not serve",
    probe_args="crate::model::EntityId(17), crate::model::EntityId(1)",
)

G3_PLANT_ROW = '''
    model.plant_loops.push(crate::model::PlantLoopConfig { id: crate::model::EntityId(18), name: "HOT WATER LOOP".into(), loop_type: crate::model::PlantLoopType::Heating, supply_temperature_c: 80.0, return_temperature_c: 60.0, design_flow_kg_s: 2.0, equipment_ids: Vec::new() });'''

G3_PLANT_EQUIPMENT_OPAQUE = G3Check(
    "mutation.invariant",
    "payload.equipment_ids.iter().any(|entry| entry.0 == 0)",
    '"A plant equipment list carries no unset id.".to_string()',
    "payload.id.0.to_string()",
    'any(value == 0 for value in payload["equipmentIds"])',
    'str(payload["id"])',
)

PLANT_LOOP = G3Entity(
    coll="plant_loops",
    display="Plant loop",
    struct="PlantLoopConfig",
    noun="plant loop",
    payload=[
        ("id", "crate::model::EntityId"),
        ("name", "String"),
        ("loop_type", "crate::model::PlantLoopType"),
        ("supply_temperature_c", "f64"),
        ("return_temperature_c", "f64"),
        ("design_flow_kg_s", "f64"),
        ("equipment_ids", "Vec<crate::model::EntityId>"),
    ],
    seed=G3_MODEL + G3_PLANT_ROW,
    bare=G3_MODEL,
    probe_args='crate::model::EntityId(18), "HOT WATER LOOP".to_string(), crate::model::PlantLoopType::Heating, 80.0, 60.0, 2.0, Vec::new()',
)

g3_create(
    number=613,
    emoji="⚗",
    entity=PLANT_LOOP,
    entity_slug="plant-loop",
    doc="Creates one plant loop. ⚠️ `equipment_ids` is the ONE reference this group cannot check: `Model` carries no chiller/boiler/pump collection at all (vocabulary §5.4), so the ids are opaque here — the list is only held to being ascending, free of duplicates and free of the unset id zero. When a plant-equipment collection lands, add a `g3_chk_reference` over it; the hole is recorded in this ticket's `📓️w7-g3-hvac.md` rather than papered over.",
    checks=[
        g3_chk_blank("name", "id", "A plant loop name"),
        g3_chk_duplicate_name(PLANT_LOOP, "name", False),
        g3_chk_range("supply_temperature_c", "id", "-100.0", "300.0", "A supply temperature"),
        g3_chk_range("return_temperature_c", "id", "-100.0", "300.0", "A return temperature"),
        g3_chk_positive("design_flow_kg_s", "id", "A design mass flow"),
        g3_chk_id_list("equipment_ids", "id", "A plant equipment list"),
        G3_PLANT_EQUIPMENT_OPAQUE,
    ],
    happy_args='crate::model::EntityId(18), "HOT WATER LOOP".to_string(), crate::model::PlantLoopType::Heating, 80.0, 60.0, 2.0, Vec::new()',
    happy_case="adds-a-hot-loop",
    happy_desc="adds a hot water plant loop",
    refusal_args='crate::model::EntityId(18), "HOT WATER LOOP".to_string(), crate::model::PlantLoopType::Heating, 80.0, 60.0, 2.0, vec![crate::model::EntityId(51), crate::model::EntityId(50)]',
    refusal_case="refuses-a-jumbled-list",
    refusal_desc="refuses an equipment list that is not ascending",
    refusal_seed=G3_MODEL,
)

g3_delete(
    number=614,
    emoji="💣",
    entity=PLANT_LOOP,
    entity_slug="plant-loop",
    doc="Drops one plant loop. Refused when no loop carries the id.",
    happy_args="crate::model::EntityId(18)",
    happy_case="drops-the-loop",
    happy_desc="drops the hot water loop",
    refusal_args="crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-one",
    refusal_desc="refuses an absent plant loop",
)

g3_rename(
    number=615,
    emoji="📛",
    slug="rename-plant-loop",
    entity=PLANT_LOOP,
    entity_slug="plant-loop",
    doc="Sets one plant loop's identity field.",
    happy='"BOILER LOOP".into()',
    happy_case="renames-the-loop",
    happy_desc="renames the hot water loop",
    probe_value='"BOILER LOOP".to_string()',
)

g3_change(
    number=616,
    emoji="♻",
    slug="change-plant-loop-type",
    entity=PLANT_LOOP,
    entity_slug="plant-loop",
    field="loop_type",
    ty="crate::model::PlantLoopType",
    doc="Swaps which fluid duty the loop carries.",
    what="loop type",
    checks=[],
    happy="crate::model::PlantLoopType::Cooling",
    happy_case="turns-it-chilled",
    happy_desc="turns the hot water loop into a chilled one",
    probe_value="crate::model::PlantLoopType::Cooling",
)

g3_change(
    number=617,
    emoji="☕",
    slug="change-plant-loop-supply-temperature",
    entity=PLANT_LOOP,
    entity_slug="plant-loop",
    field="supply_temperature_c",
    ty="f64",
    doc="Sets the temperature the loop supplies at, in °C.",
    what="supply temperature",
    checks=[g3_chk_range("new_supply_temperature_c", "id", "-100.0", "300.0", "A supply temperature")],
    happy="70.0",
    happy_case="cools-the-supply",
    happy_desc="lowers the supply temperature",
    refusal="1000.0",
    refusal_case="refuses-a-hot-supply",
    refusal_desc="refuses an unrepresentable supply temperature",
    probe_value="70.0",
)

g3_change(
    number=618,
    emoji="🧫",
    slug="change-plant-loop-return-temperature",
    entity=PLANT_LOOP,
    entity_slug="plant-loop",
    field="return_temperature_c",
    ty="f64",
    doc="Sets the temperature the loop returns at, in °C.",
    what="return temperature",
    checks=[g3_chk_range("new_return_temperature_c", "id", "-100.0", "300.0", "A return temperature")],
    happy="55.0",
    happy_case="cools-the-return",
    happy_desc="lowers the return temperature",
    refusal="-500.0",
    refusal_case="refuses-a-cold-return",
    refusal_desc="refuses an unrepresentable return temperature",
    probe_value="55.0",
)

g3_change(
    number=619,
    emoji="🚤",
    slug="change-plant-loop-design-flow",
    entity=PLANT_LOOP,
    entity_slug="plant-loop",
    field="design_flow_kg_s",
    ty="f64",
    doc="Sets the mass flow the loop is designed around, in kg/s.",
    what="design mass flow",
    checks=[g3_chk_positive("new_design_flow_kg_s", "id", "A design mass flow")],
    happy="3.0",
    happy_case="uprates-the-flow",
    happy_desc="uprates the design mass flow",
    refusal="0.0",
    refusal_case="refuses-no-flow",
    refusal_desc="refuses a zero design flow",
    probe_value="3.0",
)

G3_PLANT_ROW_EQUIPPED = '''
    model.plant_loops.push(crate::model::PlantLoopConfig { id: crate::model::EntityId(18), name: "HOT WATER LOOP".into(), loop_type: crate::model::PlantLoopType::Heating, supply_temperature_c: 80.0, return_temperature_c: 60.0, design_flow_kg_s: 2.0, equipment_ids: vec![crate::model::EntityId(50)] });'''

g3_member_add(
    number=620,
    emoji="🔩",
    slug="add-plant-loop-equipment",
    entity=PLANT_LOOP,
    entity_slug="plant-loop",
    list_field="equipment_ids",
    member="equipment_id",
    member_display="plant equipment",
    doc="Puts one central-plant equipment id on a plant loop. ⚠️ This is the ONE reference in the whole HVAC group that is NOT checked against a collection: `Model` has no chiller, boiler or pump type at all (vocabulary §5.4), so the id is opaque and only the list shape is enforced. Declared and documented rather than silently validated against nothing; when a plant-equipment collection lands this gains a `g3_chk_reference` and the refusal vector below becomes a referential one.",
    checks=[
        G3Check(
            "mutation.invariant",
            "payload.equipment_id.0 == 0",
            '"Plant equipment zero is the unset id, not a reference.".to_string()',
            "payload.equipment_id.0.to_string()",
            'payload["equipmentId"] == 0',
            'str(payload["equipmentId"])',
        )
    ],
    remove_slug="remove-plant-loop-equipment",
    happy_args="crate::model::EntityId(18), crate::model::EntityId(50)",
    happy_case="names-equipment",
    happy_desc="puts one plant equipment id on the loop",
    refusal_args="crate::model::EntityId(18), crate::model::EntityId(0)",
    refusal_case="refuses-the-unset-id",
    refusal_desc="refuses the unset equipment id",
    probe_args="crate::model::EntityId(18), crate::model::EntityId(50)",
)

g3_member_remove(
    number=621,
    emoji="⚙",
    slug="remove-plant-loop-equipment",
    entity=PLANT_LOOP,
    entity_slug="plant-loop",
    list_field="equipment_ids",
    member="equipment_id",
    member_display="plant equipment",
    doc="Takes one central-plant equipment id off a plant loop — the repair a document that arrived with a dangling `equipment_ids` entry needs, since no collection backs those ids (vocabulary §5.4). Refused when the loop does not list the id.",
    add_slug="add-plant-loop-equipment",
    happy_args="crate::model::EntityId(18), crate::model::EntityId(50)",
    happy_case="drops-equipment",
    happy_desc="takes the plant equipment id off the loop",
    happy_seed=G3_MODEL + G3_PLANT_ROW_EQUIPPED,
    refusal_args="crate::model::EntityId(18), crate::model::EntityId(50)",
    refusal_case="refuses-an-unlisted",
    refusal_desc="refuses an id the loop does not list",
    probe_args="crate::model::EntityId(18), crate::model::EntityId(50)",
)

OUTDOOR_AIR = G3Entity(
    coll="outdoor_air_systems",
    display="Outdoor air system",
    struct="OutdoorAirSystem",
    noun="outdoor air system",
    payload=[
        ("id", "crate::model::EntityId"),
        ("air_loop_id", "crate::model::EntityId"),
        ("min_oa_flow_m3_s", "f64"),
        ("economizer_enabled", "bool"),
    ],
    seed=G3_MODEL + G3_AIR_LOOP_ROW + G3_AIR_LOOP_TWO + G3_OAS_ROW,
    bare=G3_MODEL + G3_AIR_LOOP_ROW + G3_AIR_LOOP_TWO,
    probe_args="crate::model::EntityId(19), crate::model::EntityId(17), 0.2, false",
)

g3_create(
    number=622,
    emoji="🌲",
    entity=OUTDOOR_AIR,
    entity_slug="outdoor-air-system",
    doc="Puts an outdoor air system on one air loop: the minimum outdoor air flow it always draws and whether its economizer may open beyond that.",
    checks=[
        g3_chk_reference("air_loops", "Air loop", "air_loop_id"),
        g3_chk_non_negative("min_oa_flow_m3_s", "id", "A minimum outdoor air flow"),
    ],
    happy_args="crate::model::EntityId(19), crate::model::EntityId(17), 0.2, false",
    happy_case="ventilates-the-loop",
    happy_desc="puts an outdoor air system on the main loop",
    refusal_args="crate::model::EntityId(19), crate::model::EntityId(99), 0.2, false",
    refusal_case="refuses-an-absent-loop",
    refusal_desc="refuses an air loop that does not exist",
    refusal_seed=G3_MODEL + G3_AIR_LOOP_ROW + G3_AIR_LOOP_TWO,
)

g3_delete(
    number=623,
    emoji="🍂",
    entity=OUTDOOR_AIR,
    entity_slug="outdoor-air-system",
    doc="Drops one outdoor air system off its loop. Refused when no system carries the id.",
    happy_args="crate::model::EntityId(19)",
    happy_case="drops-the-system",
    happy_desc="drops the outdoor air system",
    refusal_args="crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-one",
    refusal_desc="refuses an absent outdoor air system",
)

g3_change(
    number=624,
    emoji="⛓",
    slug="change-outdoor-air-system-air-loop",
    entity=OUTDOOR_AIR,
    entity_slug="outdoor-air-system",
    field="air_loop_id",
    ty="crate::model::EntityId",
    doc="Moves the outdoor air system to another air loop.",
    what="air loop",
    checks=[g3_chk_reference("air_loops", "Air loop", "new_air_loop_id")],
    happy="crate::model::EntityId(20)",
    happy_case="moves-to-the-spare",
    happy_desc="moves the system to the spare loop",
    refusal="crate::model::EntityId(99)",
    refusal_case="refuses-an-absent-loop",
    refusal_desc="refuses an air loop that does not exist",
    probe_value="crate::model::EntityId(20)",
)

g3_change(
    number=625,
    emoji="🦋",
    slug="change-outdoor-air-system-min-oa-flow",
    entity=OUTDOOR_AIR,
    entity_slug="outdoor-air-system",
    field="min_oa_flow_m3_s",
    ty="f64",
    doc="Sets the outdoor air flow the system always draws, in m³/s.",
    what="minimum outdoor air flow",
    checks=[g3_chk_non_negative("new_min_oa_flow_m3_s", "id", "A minimum outdoor air flow")],
    happy="0.35",
    happy_case="raises-the-minimum",
    happy_desc="raises the minimum outdoor air flow",
    refusal="-1.0",
    refusal_case="refuses-a-negative",
    refusal_desc="refuses a negative outdoor air flow",
    probe_value="0.35",
)

g3_change(
    number=626,
    emoji="💰",
    slug="change-outdoor-air-system-economizer-enabled",
    entity=OUTDOOR_AIR,
    entity_slug="outdoor-air-system",
    field="economizer_enabled",
    ty="bool",
    doc="Sets whether the economizer may open beyond the minimum outdoor air flow when free cooling is available.",
    what="economizer setting",
    checks=[],
    happy="true",
    happy_case="frees-the-cooling",
    happy_desc="lets the economizer open",
    probe_value="true",
)
# endregion 🔖️G3



# region 🔖️G1ZonesEnvelope
# 🧩️ Lane G1 — zones & spaces (100–199) and geometry & envelope (200–299).
#
# The pattern functions below are the reusable half of this region: `g1_emoji` normalizes the
# mandatory U+FE0F no matter how the source emoji was pasted, and `g1_scalar` / `g1_create` /
# `g1_delete` emit the three shapes the taxonomy's `change`, `create` and `delete` verbs take over an
# id-keyed `Model` collection — Rust diff, Rust inverse, Python second implementation and Python
# inverse, all four from one description, so a kind that follows one of those shapes is a table row
# rather than four hand-written bodies. Other groups may call them; they carry nothing zone- or
# surface-specific. A kind whose shape is genuinely its own (a cascade, an unkeyed edge collection,
# a link slot) still states its four bodies inline, right here.
#
# ORDERING INVARIANT the `create`/`delete` shapes rely on: every id-keyed `Model` collection is kept
# in ascending-id order, and `adjacency_pairs` in ascending `(surface_a_id, surface_b_id)` order.
# `create` therefore inserts at the position that keeps that order rather than appending, which is
# what makes `delete` ∘ `create` land the entity back where it was — the inverse law compares whole
# documents, so an append would move a re-created row to the end.

VS16 = "\ufe0f"


def g1_emoji(text: str) -> str:
    """🎨 Exactly one trailing U+FE0F. `taxonomy.json`'s `mutationDirectoryPattern` requires the
    variation selector and many emoji render identically with and without it, so the spec table
    below never depends on how a character was pasted."""
    return text.rstrip(VS16) + VS16


def g1_guard_rust(guards, address: str) -> str:
    """⛔️ The refusal ladder, in declaration order, as Rust."""
    out = ""
    for condition, code, message, _python in guards:
        out += "\n    if " + condition + " {\n        return protocol::MutationOutcome::error(\"" + code + "\", " + message + ", " + address + ");\n    }"
    return out


def g1_guard_python(guards, address: str) -> str:
    """⛔️ The same ladder, in the same order, as the second implementation's own Python."""
    out = ""
    for _condition, code, _message, python in guards:
        out += "\n    if " + python + ":\n        return unchanged(before), rejected(\"" + code + "\", " + address + ")"
    return out


def g1_refusal_or(guards) -> str:
    """↩️ The disjunction of every refusal, for the inverse's "a refused step owes nothing" test."""
    return "".join(condition + " || " for condition, _code, _message, _python in guards)


def g1_fill(text: str, **slots) -> str:
    """🧷️ `@NAME@` slot substitution — Rust and Python bodies are full of braces, so the templates
    here are deliberately not f-strings."""
    for name, value in slots.items():
        text = text.replace("@" + name + "@", value)
    return text


G1_SCALAR_DIFF = '''    let Some(existing) = base.model.@COLL@.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("@NOUN@ {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };@GUARDS@
    if existing.@FIELD@ == payload.@PAYLOAD@ {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("@NOUN@ {} already has this @UNIT@.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.@COLL@.iter_mut().find(|item| item.id == payload.id) {
        item.@FIELD@ = payload.@PAYLOAD@@CLONE@;
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''

G1_SCALAR_INVERSE = '''    let Some(existing) = base.model.@COLL@.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if @REFUSALS@existing.@FIELD@ == payload.@PAYLOAD@ {
        return Vec::new();
    }
    vec![super::@MODULE@(payload.id, existing.@FIELD@@CLONE@)]'''

G1_SCALAR_PYTHON = '''    """@EMOJI@ `@SLUG@{id,@CAMEL@}` — @DOC@"""
    entity_id, value = payload["id"], payload["@CAMEL@"]
    item = next((row for row in before["model"]["@COLL@"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])@GUARDS@
    if item["@FIELD@"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["@COLL@"]:
        if row["id"] == entity_id:
            row["@FIELD@"] = value
    return after, applied()'''

G1_SCALAR_PYTHON_INVERT = '''    item = next(row for row in before["model"]["@COLL@"] if row["id"] == payload["id"])
    return [("@SLUG@", {"id": payload["id"], "@CAMEL@": item["@FIELD@"]})]'''


def g1_scalar(*, number, slug, emoji, entity, record, display, doc, collection, noun, field, payload_name, ty, unit, label, probe, cases, guards=(), clone="", outcome_classes=("applied", "warning", "error")):
    """🔧️ `change-<entity>-<field>` (or any other one-scalar verb) over an id-keyed collection:
    refuse a missing target, run the kind's own guard ladder, treat an unchanged value as a no-op,
    otherwise write exactly that one field and nothing else."""
    camel = field_camel(payload_name)
    kind(
        number=number,
        slug=slug,
        emoji=g1_emoji(emoji),
        verb=slug.split("-")[0],
        entity=entity,
        record=record,
        display=display,
        doc=doc,
        fields=[("id", "crate::model::EntityId"), (payload_name, ty)],
        label=label,
        target="vec![self.id.0.to_string()]",
        diff=g1_fill(G1_SCALAR_DIFF, COLL=collection, NOUN=noun, GUARDS=g1_guard_rust(guards, "[payload.id.0.to_string()]"), FIELD=field, PAYLOAD=payload_name, UNIT=unit, CLONE=clone),
        inverse=g1_fill(G1_SCALAR_INVERSE, COLL=collection, REFUSALS=g1_refusal_or(guards), FIELD=field, PAYLOAD=payload_name, MODULE=snake(slug), CLONE=clone),
        cases=cases,
        outcome_classes=list(outcome_classes),
        probe=probe,
        python=g1_fill(G1_SCALAR_PYTHON, EMOJI=g1_emoji(emoji), SLUG=slug, CAMEL=camel, DOC=doc, COLL=collection, GUARDS=g1_guard_python(guards, "[str(entity_id)]"), FIELD=field),
        python_invert=g1_fill(G1_SCALAR_PYTHON_INVERT, COLL=collection, SLUG=slug, CAMEL=camel, FIELD=field),
    )


G1_CREATE_DIFF = '''    if base.model.@COLL@.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("@NOUN@ {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }@GUARDS@@PRELUDE@
    let mut model = base.model.clone();
    let position = model.@COLL@.iter().position(|item| item.id > payload.id).unwrap_or(model.@COLL@.len());
    model.@COLL@.insert(position, @LITERAL@);
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''

G1_CREATE_INVERSE = '''    if base.model.@COLL@.iter().any(|item| item.id == payload.id) || @EXTRA@@REFUSALS@false {
        return Vec::new();
    }
    vec![super::@UNDO@]'''

G1_CREATE_PYTHON = '''    """@EMOJI@ `@SLUG@` — @DOC@"""
    entity_id = payload["id"]
    if any(row["id"] == entity_id for row in before["model"]["@COLL@"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])@GUARDS@@PYPRELUDE@
    after = copy.deepcopy(before)
    created = @PYLITERAL@
    rows = after["model"]["@COLL@"]
    position = next((index for index, row in enumerate(rows) if row["id"] > entity_id), len(rows))
    rows.insert(position, created)
    return after, applied()'''


def g1_create(*, number, slug, emoji, entity, record, display, doc, collection, noun, fields, literal, py_literal, undo, py_undo, label, probe, cases, guards=(), prelude="", py_prelude="", extra_refusal="", outcome_classes=("applied", "error")):
    """🌱️ `create-<entity>`: the caller supplies the id — no mutation mints one — the guard ladder
    refuses a taken id and every domain invariant, and the row lands at its ascending-id position."""
    kind(
        number=number,
        slug=slug,
        emoji=g1_emoji(emoji),
        verb="create",
        entity=entity,
        record=record,
        display=display,
        doc=doc,
        fields=fields,
        label=label,
        target="vec![self.id.0.to_string()]",
        diff=g1_fill(G1_CREATE_DIFF, COLL=collection, NOUN=noun, GUARDS=g1_guard_rust(guards, "[payload.id.0.to_string()]"), PRELUDE=prelude, LITERAL=literal),
        inverse=g1_fill(G1_CREATE_INVERSE, COLL=collection, EXTRA=extra_refusal, REFUSALS=g1_refusal_or(guards), UNDO=undo),
        cases=cases,
        outcome_classes=list(outcome_classes),
        probe=probe,
        python=g1_fill(G1_CREATE_PYTHON, EMOJI=g1_emoji(emoji), SLUG=slug, DOC=doc, COLL=collection, GUARDS=g1_guard_python(guards, "[str(entity_id)]"), PYPRELUDE=py_prelude, PYLITERAL=py_literal),
        python_invert=py_undo,
    )


G1_DELETE_DIFF = '''    let Some(existing) = base.model.@COLL@.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("@NOUN@ {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;@GUARDS@
    let mut model = base.model.clone();
    model.@COLL@.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''

G1_DELETE_INVERSE = '''    let Some(existing) = base.model.@COLL@.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if @REFUSALS@false {
        return Vec::new();
    }
    vec![super::@UNDO@]'''

G1_DELETE_PYTHON = '''    """@EMOJI@ `@SLUG@{id}` — @DOC@"""
    entity_id = payload["id"]
    item = next((row for row in before["model"]["@COLL@"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])@GUARDS@
    after = copy.deepcopy(before)
    after["model"]["@COLL@"] = [row for row in after["model"]["@COLL@"] if row["id"] != entity_id]
    return after, applied()'''


def g1_delete(*, number, slug, emoji, entity, record, display, doc, collection, noun, undo, py_undo, label, probe, cases, guards=(), outcome_classes=("applied", "error")):
    """🗑️ `delete-<entity>` with no cascade: refuse a missing target, refuse every reference that
    would be left dangling, otherwise drop exactly that row. Its undo is the matching `create`."""
    kind(
        number=number,
        slug=slug,
        emoji=g1_emoji(emoji),
        verb="delete",
        entity=entity,
        record=record,
        display=display,
        doc=doc,
        fields=[("id", "crate::model::EntityId")],
        label=label,
        target="vec![self.id.0.to_string()]",
        diff=g1_fill(G1_DELETE_DIFF, COLL=collection, NOUN=noun, GUARDS=g1_guard_rust(guards, "[payload.id.0.to_string()]")),
        inverse=g1_fill(G1_DELETE_INVERSE, COLL=collection, REFUSALS=g1_refusal_or(guards), UNDO=undo),
        cases=cases,
        outcome_classes=list(outcome_classes),
        probe=probe,
        python=g1_fill(G1_DELETE_PYTHON, EMOJI=g1_emoji(emoji), SLUG=slug, DOC=doc, COLL=collection, GUARDS=g1_guard_python(guards, "[str(entity_id)]")),
        python_invert=py_undo,
    )


def g1_rs_positive(expr: str) -> str:
    """🔢️ Rust: "not a positive finite number"."""
    return g1_fill("!@E@.is_finite() || @E@ <= 0.0", E=expr)


def g1_py_positive(expr: str) -> str:
    """🔢️ The same predicate in Python, spelled without `math` so the oracle keeps its own
    dependency-free reading of the payload schema."""
    return g1_fill('not (@E@ == @E@ and abs(@E@) != float("inf") and @E@ > 0.0)', E=expr)


def g1_rs_non_negative(expr: str) -> str:
    """🔢️ Rust: "not a non-negative finite number"."""
    return g1_fill("!@E@.is_finite() || @E@ < 0.0", E=expr)


def g1_py_non_negative(expr: str) -> str:
    """🔢️ The same predicate in Python."""
    return g1_fill('not (@E@ == @E@ and abs(@E@) != float("inf") and @E@ >= 0.0)', E=expr)


def g1_rs_fraction(expr: str) -> str:
    """🔢️ Rust: "not a fraction in 0..=1"."""
    return g1_fill("!@E@.is_finite() || !(0.0..=1.0).contains(&@E@)", E=expr)


def g1_py_fraction(expr: str) -> str:
    """🔢️ The same predicate in Python."""
    return g1_fill('not (@E@ == @E@ and 0.0 <= @E@ <= 1.0)', E=expr)


#: 🏠️ Every `Model` collection that carries a bare `zone_id`, for `delete-zone`'s RESTRICT check.
G1_ZONE_OWNERS = ["spaces", "surfaces", "people", "lighting", "equipment", "thermostats", "humidistats", "ideal_loads", "zone_equipment", "infiltrations", "mechanical_ventilations", "sizing_objects", "daylight_zones", "room_air_models"]

G1_ZONE_IN_USE_RS = "".join(g1_fill("base.model.@C@.iter().any(|item| item.zone_id == payload.id) || ", C=name) for name in G1_ZONE_OWNERS) + "base.model.thermal_enclosures.iter().any(|item| item.zone_ids.contains(&payload.id)) || base.model.air_loops.iter().any(|item| item.terminal_zone_ids.contains(&payload.id)) || base.model.airflow_network.as_ref().is_some_and(|network| network.zone_node_ids.iter().any(|(zone, _)| *zone == payload.id))"

G1_ZONE_IN_USE_PY = 'any(row["zone_id"] == entity_id for name in (' + ", ".join('"' + name + '"' for name in G1_ZONE_OWNERS) + ') for row in before["model"][name]) or any(entity_id in row["zone_ids"] for row in before["model"]["thermal_enclosures"]) or any(entity_id in row["terminal_zone_ids"] for row in before["model"]["air_loops"]) or (before["model"]["airflow_network"] is not None and any(pair[0] == entity_id for pair in before["model"]["airflow_network"]["zone_node_ids"]))'


# region 🏠️Zones — 105–106
g1_create(
    number=105,
    slug="create-zone",
    emoji="🏘",
    entity="zone",
    record="CreatedZone",
    display="Create Zone",
    doc="Adds one thermal zone the caller has already chosen an id for — no mutation mints an id, so an undo can name the same zone again.",
    collection="zones",
    noun="Zone",
    fields=[("id", "crate::model::EntityId"), ("name", "String"), ("volume_m3", "f64"), ("multiplier", "u32"), ("conditioned", "bool"), ("part_of_total_floor_area", "bool")],
    literal="crate::model::Zone { id: payload.id, name: payload.name.clone(), volume_m3: payload.volume_m3, multiplier: payload.multiplier, conditioned: payload.conditioned, part_of_total_floor_area: payload.part_of_total_floor_area }",
    py_literal='{"id": entity_id, "name": payload["name"], "volume_m3": payload["volumeM3"], "multiplier": payload["multiplier"], "conditioned": payload["conditioned"], "part_of_total_floor_area": payload["partOfTotalFloorArea"]}',
    undo="delete_zone(payload.id)",
    py_undo='    return [("delete-zone", {"id": payload["id"]})]',
    label='format!("Create zone \\"{}\\"", self.name)',
    probe='create_zone(crate::model::EntityId(2), "ZONE TWO".to_string(), 129.6, 1, true, true)',
    guards=[
        ("payload.name.trim().is_empty()", "mutation.invariant", '"A zone name must not be blank."', 'not payload["name"].strip()'),
        ("base.model.zones.iter().any(|item| item.name == payload.name)", "mutation.duplicate-id", 'format!("Another zone is already named \\"{}\\".", payload.name)', 'any(row["name"] == payload["name"] for row in before["model"]["zones"])'),
        (g1_rs_positive("payload.volume_m3"), "mutation.invariant", 'format!("Zone {} needs a positive finite volume, got {}.", payload.id.0, payload.volume_m3)', g1_py_positive('payload["volumeM3"]')),
        ("payload.multiplier == 0", "mutation.invariant", 'format!("Zone {} needs at least one instance.", payload.id.0)', 'payload["multiplier"] == 0'),
    ],
    cases=[
        ("✅️", "adds-a-second-zone", "adds a second conditioned zone", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    (snapshot(model), super::create_zone(crate::model::EntityId(2), "ZONE TWO".into(), 129.6, 1, true, true))'''),
        ("⛔️", "refuses-a-taken-id", "refuses an id another zone already holds", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    (snapshot(model), super::create_zone(crate::model::EntityId(1), "ZONE TWO".into(), 129.6, 1, true, true))'''),
    ],
)

g1_delete(
    number=106,
    slug="delete-zone",
    emoji="🏚",
    entity="zone",
    record="DeletedZone",
    display="Delete Zone",
    doc="Removes one thermal zone. It RESTRICTS rather than cascades: while any space, surface, gain, HVAC object, grouping or airflow node still names the zone it is refused with `mutation.invariant`, because a cascade here would have to delete surfaces, which cascade again to fenestrations and adjacency pairs.",
    collection="zones",
    noun="Zone",
    undo="create_zone(existing.id, existing.name.clone(), existing.volume_m3, existing.multiplier, existing.conditioned, existing.part_of_total_floor_area)",
    py_undo='''    item = next(row for row in before["model"]["zones"] if row["id"] == payload["id"])
    return [("create-zone", {"id": item["id"], "name": item["name"], "volumeM3": item["volume_m3"], "multiplier": item["multiplier"], "conditioned": item["conditioned"], "partOfTotalFloorArea": item["part_of_total_floor_area"]})]''',
    label='format!("Delete zone {}", self.id.0)',
    probe="delete_zone(crate::model::EntityId(2))",
    guards=[
        (G1_ZONE_IN_USE_RS, "mutation.invariant", 'format!("Zone {} is still referenced by another entity; delete or reassign those first.", payload.id.0)', G1_ZONE_IN_USE_PY),
    ],
    cases=[
        ("✅️", "deletes-a-free-zone", "deletes a zone nothing references", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.zones.push(zone(2, "ZONE TWO"));
    (snapshot(model), super::delete_zone(crate::model::EntityId(2)))'''),
        ("⛔️", "refuses-a-used-zone", "refuses a zone a space still names", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.spaces.push(fixtures::space(2, "SPACE ONE", 1));
    (snapshot(model), super::delete_zone(crate::model::EntityId(1)))'''),
    ],
)
# endregion 🏠️Zones


# region 🪑️Spaces — 107–111
g1_create(
    number=107,
    slug="create-space",
    emoji="🪑",
    entity="space",
    record="CreatedSpace",
    display="Create Space",
    doc="Adds one space inside an existing zone. The owning zone must already exist, so the document never carries a space whose zone the reports cannot resolve.",
    collection="spaces",
    noun="Space",
    fields=[("id", "crate::model::EntityId"), ("name", "String"), ("zone_id", "crate::model::EntityId"), ("floor_area_m2", "f64")],
    literal="crate::model::Space { id: payload.id, name: payload.name.clone(), zone_id: payload.zone_id, floor_area_m2: payload.floor_area_m2 }",
    py_literal='{"id": entity_id, "name": payload["name"], "zone_id": payload["zoneId"], "floor_area_m2": payload["floorAreaM2"]}',
    undo="delete_space(payload.id)",
    py_undo='    return [("delete-space", {"id": payload["id"]})]',
    label='format!("Create space \\"{}\\"", self.name)',
    probe='create_space(crate::model::EntityId(2), "SPACE ONE".to_string(), crate::model::EntityId(1), 48.0)',
    guards=[
        ("payload.name.trim().is_empty()", "mutation.invariant", '"A space name must not be blank."', 'not payload["name"].strip()'),
        ("!base.model.zones.iter().any(|item| item.id == payload.zone_id)", "mutation.target-missing", 'format!("Zone {} does not exist.", payload.zone_id.0)', 'not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"])'),
        (g1_rs_non_negative("payload.floor_area_m2"), "mutation.invariant", 'format!("Space {} needs a non-negative finite floor area, got {}.", payload.id.0, payload.floor_area_m2)', g1_py_non_negative('payload["floorAreaM2"]')),
    ],
    cases=[
        ("✅️", "adds-a-space", "adds a space to the first zone", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    (snapshot(model), super::create_space(crate::model::EntityId(2), "SPACE ONE".into(), crate::model::EntityId(1), 48.0))'''),
        ("⛔️", "refuses-a-missing-zone", "refuses a space in an absent zone", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::create_space(crate::model::EntityId(2), "SPACE ONE".into(), crate::model::EntityId(9), 48.0))'''),
    ],
)

g1_delete(
    number=108,
    slug="delete-space",
    emoji="🧹",
    entity="space",
    record="DeletedSpace",
    display="Delete Space",
    doc="Removes one space. Refused while a space list still names it, so no grouping is left pointing at nothing.",
    collection="spaces",
    noun="Space",
    undo="create_space(existing.id, existing.name.clone(), existing.zone_id, existing.floor_area_m2)",
    py_undo='''    item = next(row for row in before["model"]["spaces"] if row["id"] == payload["id"])
    return [("create-space", {"id": item["id"], "name": item["name"], "zoneId": item["zone_id"], "floorAreaM2": item["floor_area_m2"]})]''',
    label='format!("Delete space {}", self.id.0)',
    probe="delete_space(crate::model::EntityId(2))",
    guards=[
        ("base.model.space_lists.iter().any(|item| item.space_ids.contains(&payload.id))", "mutation.invariant", 'format!("Space {} is still a member of a space list.", payload.id.0)', 'any(payload["id"] in row["space_ids"] for row in before["model"]["space_lists"])'),
    ],
    cases=[
        ("✅️", "deletes-a-free-space", "deletes a space no list names", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.spaces.push(fixtures::space(2, "SPACE ONE", 1));
    (snapshot(model), super::delete_space(crate::model::EntityId(2)))'''),
        ("⛔️", "refuses-a-missing-space", "refuses an absent space", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    (snapshot(model), super::delete_space(crate::model::EntityId(9)))'''),
    ],
)

g1_scalar(
    number=109,
    slug="rename-space",
    emoji="🔤",
    entity="space",
    record="RenamedSpace",
    display="Rename Space",
    doc="Sets one space's identity field; a name another space already holds is refused.",
    collection="spaces",
    noun="Space",
    field="name",
    payload_name="new_name",
    ty="String",
    unit="name",
    clone=".clone()",
    label='format!("Rename space {} to \\"{}\\"", self.id.0, self.new_name)',
    probe='rename_space(crate::model::EntityId(2), "SPACE 1".to_string())',
    guards=[
        ("payload.new_name.trim().is_empty()", "mutation.invariant", '"A space name must not be blank."', 'not value.strip()'),
        ("base.model.spaces.iter().any(|item| item.id != payload.id && item.name == payload.new_name)", "mutation.duplicate-id", 'format!("Another space is already named \\"{}\\".", payload.new_name)', 'any(row["id"] != entity_id and row["name"] == value for row in before["model"]["spaces"])'),
    ],
    cases=[
        ("✅️", "renames-a-space", "renames the first space", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.spaces.push(fixtures::space(2, "SPACE ONE", 1));
    (snapshot(model), super::rename_space(crate::model::EntityId(2), "SPACE 1".into()))'''),
        ("⛔️", "refuses-a-blank-name", "refuses a blank name", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.spaces.push(fixtures::space(2, "SPACE ONE", 1));
    (snapshot(model), super::rename_space(crate::model::EntityId(2), "  ".into()))'''),
    ],
)

g1_scalar(
    number=110,
    slug="change-space-floor-area",
    emoji="🧮",
    entity="space",
    record="ChangedSpaceFloorArea",
    display="Change Space Floor Area",
    doc="Sets one space's floor area in square metres — the denominator every per-area gain in that space is expanded against.",
    collection="spaces",
    noun="Space",
    field="floor_area_m2",
    payload_name="new_floor_area_m2",
    ty="f64",
    unit="floor area",
    label='format!("Change space {} floor area to {} m²", self.id.0, self.new_floor_area_m2)',
    probe="change_space_floor_area(crate::model::EntityId(2), 48.0)",
    guards=[
        (g1_rs_non_negative("payload.new_floor_area_m2"), "mutation.invariant", 'format!("Space {} needs a non-negative finite floor area, got {}.", payload.id.0, payload.new_floor_area_m2)', g1_py_non_negative("value")),
    ],
    cases=[
        ("✅️", "resizes-a-space", "resizes the first space", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.spaces.push(fixtures::space(2, "SPACE ONE", 1));
    (snapshot(model), super::change_space_floor_area(crate::model::EntityId(2), 96.0))'''),
        ("⛔️", "refuses-negative-area", "refuses a negative floor area", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.spaces.push(fixtures::space(2, "SPACE ONE", 1));
    (snapshot(model), super::change_space_floor_area(crate::model::EntityId(2), -1.0))'''),
    ],
)

g1_scalar(
    number=111,
    slug="change-space-zone",
    emoji="🚚",
    entity="space",
    record="ChangedSpaceZone",
    display="Change Space Zone",
    doc="Reassigns one space to another existing zone. `Space::zone_id` is a foreign key between two flat collections, not a recursive parent field, so this is the `change` verb rather than a hierarchy move.",
    collection="spaces",
    noun="Space",
    field="zone_id",
    payload_name="new_zone_id",
    ty="crate::model::EntityId",
    unit="owning zone",
    label='format!("Move space {} to zone {}", self.id.0, self.new_zone_id.0)',
    probe="change_space_zone(crate::model::EntityId(2), crate::model::EntityId(3))",
    guards=[
        ("!base.model.zones.iter().any(|item| item.id == payload.new_zone_id)", "mutation.target-missing", 'format!("Zone {} does not exist.", payload.new_zone_id.0)', 'not any(row["id"] == value for row in before["model"]["zones"])'),
    ],
    cases=[
        ("✅️", "moves-a-space", "moves the space to the second zone", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.zones.push(zone(3, "ZONE TWO"));
    model.spaces.push(fixtures::space(2, "SPACE ONE", 1));
    (snapshot(model), super::change_space_zone(crate::model::EntityId(2), crate::model::EntityId(3)))'''),
        ("⛔️", "refuses-a-missing-zone", "refuses an absent destination zone", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.spaces.push(fixtures::space(2, "SPACE ONE", 1));
    (snapshot(model), super::change_space_zone(crate::model::EntityId(2), crate::model::EntityId(9)))'''),
    ],
)
# endregion 🪑️Spaces


# region 📐️Surfaces — 200–210
G1_SURFACE_BOUNDARY_HALVES = "crate::model::OutsideBoundary::from_parts(payload.boundary, payload.interzone_surface_id).is_none() || "

G1_SURFACE_PRELUDE = '''
    let Some(boundary) = crate::model::OutsideBoundary::from_parts(payload.boundary, payload.interzone_surface_id) else {
        return protocol::MutationOutcome::error("mutation.invariant", "An interzone boundary names exactly one partner surface, and every other boundary names none.", [payload.id.0.to_string()]);
    };'''

G1_SURFACE_PY_PRELUDE = '''
    if (payload["boundary"] == "Interzone") != (payload["interzoneSurfaceId"] is not None):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    boundary = {"Interzone": payload["interzoneSurfaceId"]} if payload["interzoneSurfaceId"] is not None else payload["boundary"]'''

g1_create(
    number=200,
    slug="create-surface",
    emoji="🟫",
    entity="surface",
    record="CreatedSurface",
    display="Create Surface",
    doc="Adds one planar polygon surface to an existing zone with an existing construction. The exterior boundary arrives as its two halves — a `boundary` discriminator and the `interzoneSurfaceId` only the `Interzone` arm carries — because `dsl::DslScalar` binds unit variants only.",
    collection="surfaces",
    noun="Surface",
    fields=[("id", "crate::model::EntityId"), ("name", "String"), ("zone_id", "crate::model::EntityId"), ("class", "crate::model::SurfaceClass"), ("vertices_m", "Vec<[f64; 3]>"), ("construction_id", "crate::model::EntityId"), ("boundary", "crate::model::OutsideBoundaryKind"), ("interzone_surface_id", "Option<crate::model::EntityId>"), ("sun_exposed", "bool"), ("wind_exposed", "bool"), ("multiplier", "u32")],
    prelude=G1_SURFACE_PRELUDE,
    py_prelude=G1_SURFACE_PY_PRELUDE,
    extra_refusal=G1_SURFACE_BOUNDARY_HALVES,
    literal="crate::model::Surface { id: payload.id, name: payload.name.clone(), zone_id: payload.zone_id, class: payload.class, vertices_m: payload.vertices_m.clone(), construction_id: payload.construction_id, outside_boundary_condition: boundary, sun_exposed: payload.sun_exposed, wind_exposed: payload.wind_exposed, multiplier: payload.multiplier }",
    py_literal='{"id": entity_id, "name": payload["name"], "zone_id": payload["zoneId"], "class": payload["class"], "vertices_m": payload["verticesM"], "construction_id": payload["constructionId"], "outside_boundary_condition": boundary, "sun_exposed": payload["sunExposed"], "wind_exposed": payload["windExposed"], "multiplier": payload["multiplier"]}',
    undo="delete_surface(payload.id)",
    py_undo='    return [("delete-surface", {"id": payload["id"]})]',
    label='format!("Create surface \\"{}\\"", self.name)',
    probe='create_surface(crate::model::EntityId(3), "WALL SOUTH".to_string(), crate::model::EntityId(1), crate::model::SurfaceClass::ExteriorWall, vec![[0.0, 0.0, 0.0], [8.0, 0.0, 0.0], [8.0, 0.0, 2.7]], crate::model::EntityId(2), crate::model::OutsideBoundaryKind::OutdoorAir, None, true, true, 1)',
    guards=[
        ("payload.name.trim().is_empty()", "mutation.invariant", '"A surface name must not be blank."', 'not payload["name"].strip()'),
        ("!base.model.zones.iter().any(|item| item.id == payload.zone_id)", "mutation.target-missing", 'format!("Zone {} does not exist.", payload.zone_id.0)', 'not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"])'),
        ("!base.model.constructions.iter().any(|item| item.id == payload.construction_id)", "mutation.target-missing", 'format!("Construction {} does not exist.", payload.construction_id.0)', 'not any(row["id"] == payload["constructionId"] for row in before["model"]["constructions"])'),
        ("payload.vertices_m.len() < 3", "mutation.invariant", 'format!("A surface polygon needs at least three vertices, got {}.", payload.vertices_m.len())', 'len(payload["verticesM"]) < 3'),
        ("payload.interzone_surface_id.is_some_and(|partner| partner == payload.id || !base.model.surfaces.iter().any(|item| item.id == partner))", "mutation.target-missing", '"An interzone partner must be another surface that already exists."', '(payload["interzoneSurfaceId"] is not None and (payload["interzoneSurfaceId"] == entity_id or not any(row["id"] == payload["interzoneSurfaceId"] for row in before["model"]["surfaces"])))'),
        ("payload.multiplier == 0", "mutation.invariant", 'format!("Surface {} needs at least one instance.", payload.id.0)', 'payload["multiplier"] == 0'),
    ],
    cases=[
        ("✅️", "adds-a-south-wall", "adds the south exterior wall", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.materials.push(fixtures::material(4, "PLASTERBOARD"));
    model.constructions.push(fixtures::construction(2, "LIGHTWEIGHT WALL", 4));
    (snapshot(model), super::create_surface(crate::model::EntityId(3), "WALL SOUTH".into(), crate::model::EntityId(1), crate::model::SurfaceClass::ExteriorWall, vec![[0.0, 0.0, 0.0], [8.0, 0.0, 0.0], [8.0, 0.0, 2.7], [0.0, 0.0, 2.7]], crate::model::EntityId(2), crate::model::OutsideBoundaryKind::OutdoorAir, None, true, true, 1))'''),
        ("⛔️", "refuses-two-vertices", "refuses a polygon with two vertices", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.materials.push(fixtures::material(4, "PLASTERBOARD"));
    model.constructions.push(fixtures::construction(2, "LIGHTWEIGHT WALL", 4));
    (snapshot(model), super::create_surface(crate::model::EntityId(3), "WALL SOUTH".into(), crate::model::EntityId(1), crate::model::SurfaceClass::ExteriorWall, vec![[0.0, 0.0, 0.0], [8.0, 0.0, 0.0]], crate::model::EntityId(2), crate::model::OutsideBoundaryKind::OutdoorAir, None, true, true, 1))'''),
    ],
)

kind(
    number=201,
    slug="delete-surface",
    emoji=g1_emoji("🪚"),
    verb="delete",
    entity="surface",
    record="DeletedSurface",
    display="Delete Surface",
    doc="Removes one surface and CASCADES: every fenestration hosted on it and every adjacency pair naming it go with it, reported at info level as `mutation.cascade`. It still RESTRICTS on the one reference a cascade could not answer for — another surface naming this one as its interzone partner — because silently rewriting that surface's boundary condition is a physics decision no delete may take.",
    fields=[("id", "crate::model::EntityId")],
    label='format!("Delete surface {}", self.id.0)',
    target="vec![self.id.0.to_string()]",
    probe="delete_surface(crate::model::EntityId(3))",
    diff='''    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Surface {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    if base.model.surfaces.iter().any(|item| item.outside_boundary_condition.interzone_partner() == Some(payload.id)) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Surface {} is another surface's interzone partner; change that boundary condition first.", payload.id.0), [payload.id.0.to_string()]);
    }
    let fenestrations = base.model.fenestrations.iter().filter(|item| item.surface_id == payload.id).count();
    let pairs = base.model.adjacency_pairs.iter().filter(|item| item.surface_a_id == payload.id || item.surface_b_id == payload.id).count();
    let mut model = base.model.clone();
    model.surfaces.retain(|item| item.id != payload.id);
    model.fenestrations.retain(|item| item.surface_id != payload.id);
    model.adjacency_pairs.retain(|item| item.surface_a_id != payload.id && item.surface_b_id != payload.id);
    let outcome = protocol::MutationOutcome::new(SUPER_DIFF(model));
    if fenestrations + pairs == 0 {
        return outcome;
    }
    outcome.info("mutation.cascade", format!("Deleting surface {} also removed {fenestrations} fenestration(s) and {pairs} adjacency pair(s).", payload.id.0))''',
    inverse='''    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if base.model.surfaces.iter().any(|item| item.outside_boundary_condition.interzone_partner() == Some(payload.id)) {
        return Vec::new();
    }
    let mut steps = vec![super::create_surface(existing.id, existing.name.clone(), existing.zone_id, existing.class, existing.vertices_m.clone(), existing.construction_id, existing.outside_boundary_condition.kind(), existing.outside_boundary_condition.interzone_partner(), existing.sun_exposed, existing.wind_exposed, existing.multiplier)];
    for window in base.model.fenestrations.iter().filter(|item| item.surface_id == payload.id) {
        steps.push(super::create_fenestration(window.id, window.name.clone(), window.surface_id, window.u_value_w_m2k, window.shgc, window.vlt, window.area_m2, window.height_m, window.sill_height_m, window.frame_conductance_w_k, window.divider_conductance_w_k, window.overhang_depth_m, window.overhang_offset_m, window.fin_depth_m, window.fin_offset_m, window.glazing_construction_id));
    }
    for pair in base.model.adjacency_pairs.iter().filter(|item| item.surface_a_id == payload.id || item.surface_b_id == payload.id) {
        steps.push(super::connect_surfaces(pair.surface_a_id, pair.surface_b_id));
    }
    steps''',
    outcome_classes=["applied", "info", "error"],
    python='''    """🪚️ `delete-surface{id}` — the one cascading delete in this vocabulary: fenestrations and
    adjacency pairs go with their host surface, an interzone partner blocks it."""
    entity_id = payload["id"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if any(row["outside_boundary_condition"] == {"Interzone": entity_id} for row in before["model"]["surfaces"]):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    windows = [row for row in before["model"]["fenestrations"] if row["surface_id"] == entity_id]
    pairs = [row for row in before["model"]["adjacency_pairs"] if entity_id in (row["surface_a_id"], row["surface_b_id"])]
    after = copy.deepcopy(before)
    after["model"]["surfaces"] = [row for row in after["model"]["surfaces"] if row["id"] != entity_id]
    after["model"]["fenestrations"] = [row for row in after["model"]["fenestrations"] if row["surface_id"] != entity_id]
    after["model"]["adjacency_pairs"] = [row for row in after["model"]["adjacency_pairs"] if entity_id not in (row["surface_a_id"], row["surface_b_id"])]
    if not windows and not pairs:
        return after, applied()
    return after, applied(("info", "mutation.cascade"))''',
    python_invert='''    entity_id = payload["id"]
    item = next(row for row in before["model"]["surfaces"] if row["id"] == entity_id)
    boundary = item["outside_boundary_condition"]
    partner = boundary["Interzone"] if isinstance(boundary, dict) else None
    steps = [("create-surface", {"id": item["id"], "name": item["name"], "zoneId": item["zone_id"], "class": item["class"], "verticesM": item["vertices_m"], "constructionId": item["construction_id"], "boundary": "Interzone" if partner is not None else boundary, "interzoneSurfaceId": partner, "sunExposed": item["sun_exposed"], "windExposed": item["wind_exposed"], "multiplier": item["multiplier"]})]
    for window in before["model"]["fenestrations"]:
        if window["surface_id"] == entity_id:
            steps.append(("create-fenestration", {"id": window["id"], "name": window["name"], "surfaceId": window["surface_id"], "uValueWM2k": window["u_value_w_m2k"], "shgc": window["shgc"], "vlt": window["vlt"], "areaM2": window["area_m2"], "heightM": window["height_m"], "sillHeightM": window["sill_height_m"], "frameConductanceWK": window["frame_conductance_w_k"], "dividerConductanceWK": window["divider_conductance_w_k"], "overhangDepthM": window["overhang_depth_m"], "overhangOffsetM": window["overhang_offset_m"], "finDepthM": window["fin_depth_m"], "finOffsetM": window["fin_offset_m"], "glazingConstructionId": window["glazing_construction_id"]}))
    for pair in before["model"]["adjacency_pairs"]:
        if entity_id in (pair["surface_a_id"], pair["surface_b_id"]):
            steps.append(("connect-surfaces", {"surfaceAId": pair["surface_a_id"], "surfaceBId": pair["surface_b_id"]}))
    return steps''',
    cases=[
        ("✅️", "cascades-a-window", "deletes a wall and its window", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.materials.push(fixtures::material(4, "PLASTERBOARD"));
    model.constructions.push(fixtures::construction(2, "LIGHTWEIGHT WALL", 4));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    model.fenestrations.push(fixtures::window(5, "WINDOW SOUTH", 3));
    (snapshot(model), super::delete_surface(crate::model::EntityId(3)))'''),
        ("⛔️", "refuses-a-partner", "refuses a surface another names as its interzone partner", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.materials.push(fixtures::material(4, "PLASTERBOARD"));
    model.constructions.push(fixtures::construction(2, "LIGHTWEIGHT WALL", 4));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    let mut neighbour = fixtures::surface(6, "WALL NORTH", 1, 2);
    neighbour.outside_boundary_condition = crate::model::OutsideBoundary::Interzone(crate::model::EntityId(3));
    model.surfaces.push(neighbour);
    (snapshot(model), super::delete_surface(crate::model::EntityId(3)))'''),
    ],
)

g1_scalar(
    number=202,
    slug="rename-surface",
    emoji="🏳",
    entity="surface",
    record="RenamedSurface",
    display="Rename Surface",
    doc="Sets one surface's identity field; a name another surface already holds is refused because every surface-level report keys on it.",
    collection="surfaces",
    noun="Surface",
    field="name",
    payload_name="new_name",
    ty="String",
    unit="name",
    clone=".clone()",
    label='format!("Rename surface {} to \\"{}\\"", self.id.0, self.new_name)',
    probe='rename_surface(crate::model::EntityId(3), "WALL S".to_string())',
    guards=[
        ("payload.new_name.trim().is_empty()", "mutation.invariant", '"A surface name must not be blank."', 'not value.strip()'),
        ("base.model.surfaces.iter().any(|item| item.id != payload.id && item.name == payload.new_name)", "mutation.duplicate-id", 'format!("Another surface is already named \\"{}\\".", payload.new_name)', 'any(row["id"] != entity_id and row["name"] == value for row in before["model"]["surfaces"])'),
    ],
    cases=[
        ("✅️", "renames-a-wall", "renames the south wall", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    (snapshot(model), super::rename_surface(crate::model::EntityId(3), "WALL S".into()))'''),
        ("⛔️", "refuses-a-taken-name", "refuses a name another surface holds", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    model.surfaces.push(fixtures::surface(6, "WALL NORTH", 1, 2));
    (snapshot(model), super::rename_surface(crate::model::EntityId(3), "WALL NORTH".into()))'''),
    ],
)

g1_scalar(
    number=203,
    slug="change-surface-zone",
    emoji="🗜",
    entity="surface",
    record="ChangedSurfaceZone",
    display="Change Surface Zone",
    doc="Reassigns one surface to another existing zone — the zone whose air heat balance the surface's inside face exchanges with.",
    collection="surfaces",
    noun="Surface",
    field="zone_id",
    payload_name="new_zone_id",
    ty="crate::model::EntityId",
    unit="owning zone",
    label='format!("Move surface {} to zone {}", self.id.0, self.new_zone_id.0)',
    probe="change_surface_zone(crate::model::EntityId(3), crate::model::EntityId(7))",
    guards=[
        ("!base.model.zones.iter().any(|item| item.id == payload.new_zone_id)", "mutation.target-missing", 'format!("Zone {} does not exist.", payload.new_zone_id.0)', 'not any(row["id"] == value for row in before["model"]["zones"])'),
    ],
    cases=[
        ("✅️", "moves-a-wall", "moves the wall to the second zone", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.zones.push(zone(7, "ZONE TWO"));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    (snapshot(model), super::change_surface_zone(crate::model::EntityId(3), crate::model::EntityId(7)))'''),
        ("⛔️", "refuses-a-missing-zone", "refuses an absent destination zone", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    (snapshot(model), super::change_surface_zone(crate::model::EntityId(3), crate::model::EntityId(9)))'''),
    ],
)

g1_scalar(
    number=204,
    slug="change-surface-class",
    emoji="🧩",
    entity="surface",
    record="ChangedSurfaceClass",
    display="Change Surface Class",
    doc="Sets which of the eight boundary roles the surface plays — the role the kernel reads to decide whether it sees sky, ground or another zone.",
    collection="surfaces",
    noun="Surface",
    field="class",
    payload_name="new_class",
    ty="crate::model::SurfaceClass",
    unit="class",
    label='format!("Change surface {} class to {:?}", self.id.0, self.new_class)',
    probe="change_surface_class(crate::model::EntityId(3), crate::model::SurfaceClass::Roof)",
    cases=[
        ("✅️", "turns-a-wall-to-roof", "reclassifies the wall as a roof", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    (snapshot(model), super::change_surface_class(crate::model::EntityId(3), crate::model::SurfaceClass::Roof))'''),
        ("⛔️", "refuses-a-missing-one", "refuses an absent surface", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::change_surface_class(crate::model::EntityId(9), crate::model::SurfaceClass::Roof))'''),
    ],
)

g1_scalar(
    number=205,
    slug="replace-surface-vertices",
    emoji="🔺",
    entity="surface",
    record="ReplacedSurfaceVertices",
    display="Replace Surface Vertices",
    doc="Swaps the surface's whole polygon. Geometry is one value — moving a single corner would leave the other vertices describing a different plane — so the taxonomy's `replace` verb carries the full ring, minimum three vertices.",
    collection="surfaces",
    noun="Surface",
    field="vertices_m",
    payload_name="new_vertices_m",
    ty="Vec<[f64; 3]>",
    unit="polygon",
    clone=".clone()",
    label='format!("Replace surface {} with a {}-vertex polygon", self.id.0, self.new_vertices_m.len())',
    probe="replace_surface_vertices(crate::model::EntityId(3), vec![[0.0, 0.0, 0.0], [6.0, 0.0, 0.0], [6.0, 0.0, 2.7]])",
    guards=[
        ("payload.new_vertices_m.len() < 3", "mutation.invariant", 'format!("A surface polygon needs at least three vertices, got {}.", payload.new_vertices_m.len())', 'len(value) < 3'),
    ],
    cases=[
        ("✅️", "narrows-a-wall", "narrows the wall to six metres", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    (snapshot(model), super::replace_surface_vertices(crate::model::EntityId(3), vec![[0.0, 0.0, 0.0], [6.0, 0.0, 0.0], [6.0, 0.0, 2.7], [0.0, 0.0, 2.7]]))'''),
        ("⛔️", "refuses-a-line", "refuses a two-vertex polygon", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    (snapshot(model), super::replace_surface_vertices(crate::model::EntityId(3), vec![[0.0, 0.0, 0.0], [6.0, 0.0, 0.0]]))'''),
    ],
)

g1_scalar(
    number=206,
    slug="change-surface-construction",
    emoji="🧰",
    entity="surface",
    record="ChangedSurfaceConstruction",
    display="Change Surface Construction",
    doc="Points the surface at another existing layered construction — the layer stack the conduction transfer functions are derived from.",
    collection="surfaces",
    noun="Surface",
    field="construction_id",
    payload_name="new_construction_id",
    ty="crate::model::EntityId",
    unit="construction",
    label='format!("Change surface {} construction to {}", self.id.0, self.new_construction_id.0)',
    probe="change_surface_construction(crate::model::EntityId(3), crate::model::EntityId(8))",
    guards=[
        ("!base.model.constructions.iter().any(|item| item.id == payload.new_construction_id)", "mutation.target-missing", 'format!("Construction {} does not exist.", payload.new_construction_id.0)', 'not any(row["id"] == value for row in before["model"]["constructions"])'),
    ],
    cases=[
        ("✅️", "swaps-the-wall-stack", "swaps the wall to the heavyweight stack", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.materials.push(fixtures::material(4, "PLASTERBOARD"));
    model.constructions.push(fixtures::construction(2, "LIGHTWEIGHT WALL", 4));
    model.constructions.push(fixtures::construction(8, "HEAVYWEIGHT WALL", 4));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    (snapshot(model), super::change_surface_construction(crate::model::EntityId(3), crate::model::EntityId(8)))'''),
        ("⛔️", "refuses-a-missing-one", "refuses an absent construction", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    (snapshot(model), super::change_surface_construction(crate::model::EntityId(3), crate::model::EntityId(9)))'''),
    ],
)

kind(
    number=207,
    slug="change-surface-boundary-condition",
    emoji=g1_emoji("🚧"),
    verb="change",
    entity="surface",
    record="ChangedSurfaceBoundaryCondition",
    display="Change Surface Boundary Condition",
    doc="Sets what the surface's outside face faces. The tagged union arrives as its two halves — a `newBoundary` discriminator and the `newInterzoneSurfaceId` only the `Interzone` arm carries — and the two must agree.",
    fields=[("id", "crate::model::EntityId"), ("new_boundary", "crate::model::OutsideBoundaryKind"), ("new_interzone_surface_id", "Option<crate::model::EntityId>")],
    label='format!("Change surface {} boundary to {:?}", self.id.0, self.new_boundary)',
    target="vec![self.id.0.to_string()]",
    probe="change_surface_boundary_condition(crate::model::EntityId(3), crate::model::OutsideBoundaryKind::Ground, None)",
    diff='''    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Surface {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let Some(boundary) = crate::model::OutsideBoundary::from_parts(payload.new_boundary, payload.new_interzone_surface_id) else {
        return protocol::MutationOutcome::error("mutation.invariant", "An interzone boundary names exactly one partner surface, and every other boundary names none.", [payload.id.0.to_string()]);
    };
    if payload.new_interzone_surface_id.is_some_and(|partner| partner == payload.id || !base.model.surfaces.iter().any(|item| item.id == partner)) {
        return protocol::MutationOutcome::error("mutation.target-missing", "An interzone partner must be another surface that already exists.", [payload.id.0.to_string()]);
    }
    if existing.outside_boundary_condition == boundary {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Surface {} already has this boundary condition.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.surfaces.iter_mut().find(|item| item.id == payload.id) {
        item.outside_boundary_condition = boundary;
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let Some(boundary) = crate::model::OutsideBoundary::from_parts(payload.new_boundary, payload.new_interzone_surface_id) else {
        return Vec::new();
    };
    if payload.new_interzone_surface_id.is_some_and(|partner| partner == payload.id || !base.model.surfaces.iter().any(|item| item.id == partner)) || existing.outside_boundary_condition == boundary {
        return Vec::new();
    }
    vec![super::change_surface_boundary_condition(payload.id, existing.outside_boundary_condition.kind(), existing.outside_boundary_condition.interzone_partner())]''',
    outcome_classes=["applied", "warning", "error"],
    python='''    """🚧️ `change-surface-boundary-condition{id,newBoundary,newInterzoneSurfaceId}` — the tagged
    union's two halves, admitted only when they agree."""
    entity_id, tag, partner = payload["id"], payload["newBoundary"], payload["newInterzoneSurfaceId"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if (tag == "Interzone") != (partner is not None):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if partner is not None and (partner == entity_id or not any(row["id"] == partner for row in before["model"]["surfaces"])):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    boundary = {"Interzone": partner} if partner is not None else tag
    if item["outside_boundary_condition"] == boundary:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["outside_boundary_condition"] = boundary
    return after, applied()''',
    python_invert='''    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    boundary = item["outside_boundary_condition"]
    partner = boundary["Interzone"] if isinstance(boundary, dict) else None
    return [("change-surface-boundary-condition", {"id": payload["id"], "newBoundary": "Interzone" if partner is not None else boundary, "newInterzoneSurfaceId": partner})]''',
    cases=[
        ("✅️", "grounds-a-floor", "puts the surface on the ground", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    (snapshot(model), super::change_surface_boundary_condition(crate::model::EntityId(3), crate::model::OutsideBoundaryKind::Ground, None))'''),
        ("⛔️", "refuses-half-a-union", "refuses an interzone arm with no partner", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    (snapshot(model), super::change_surface_boundary_condition(crate::model::EntityId(3), crate::model::OutsideBoundaryKind::Interzone, None))'''),
    ],
)

g1_scalar(
    number=208,
    slug="change-surface-sun-exposed",
    emoji="🌅",
    entity="surface",
    record="ChangedSurfaceSunExposed",
    display="Change Surface Sun Exposed",
    doc="Sets whether the outside face receives beam and diffuse solar at all.",
    collection="surfaces",
    noun="Surface",
    field="sun_exposed",
    payload_name="new_sun_exposed",
    ty="bool",
    unit="sun exposure",
    label='format!("Change surface {} sun exposure to {}", self.id.0, self.new_sun_exposed)',
    probe="change_surface_sun_exposed(crate::model::EntityId(3), false)",
    cases=[
        ("✅️", "shades-a-wall", "takes the wall out of the sun", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    (snapshot(model), super::change_surface_sun_exposed(crate::model::EntityId(3), false))'''),
        ("⛔️", "refuses-a-missing-one", "refuses an absent surface", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::change_surface_sun_exposed(crate::model::EntityId(9), false))'''),
    ],
)

g1_scalar(
    number=209,
    slug="change-surface-wind-exposed",
    emoji="🍃",
    entity="surface",
    record="ChangedSurfaceWindExposed",
    display="Change Surface Wind Exposed",
    doc="Sets whether the outside face uses the wind-driven exterior convection correlation or the sheltered one.",
    collection="surfaces",
    noun="Surface",
    field="wind_exposed",
    payload_name="new_wind_exposed",
    ty="bool",
    unit="wind exposure",
    label='format!("Change surface {} wind exposure to {}", self.id.0, self.new_wind_exposed)',
    probe="change_surface_wind_exposed(crate::model::EntityId(3), false)",
    cases=[
        ("✅️", "shelters-a-wall", "shelters the wall from wind", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    (snapshot(model), super::change_surface_wind_exposed(crate::model::EntityId(3), false))'''),
        ("⛔️", "refuses-a-missing-one", "refuses an absent surface", '''    let model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    (snapshot(model), super::change_surface_wind_exposed(crate::model::EntityId(9), false))'''),
    ],
)

g1_scalar(
    number=210,
    slug="change-surface-multiplier",
    emoji="🔁",
    entity="surface",
    record="ChangedSurfaceMultiplier",
    display="Change Surface Multiplier",
    doc="Sets how many identical instances of the surface the heat balance is scaled by.",
    collection="surfaces",
    noun="Surface",
    field="multiplier",
    payload_name="new_multiplier",
    ty="u32",
    unit="multiplier",
    label='format!("Change surface {} multiplier to {}", self.id.0, self.new_multiplier)',
    probe="change_surface_multiplier(crate::model::EntityId(3), 4)",
    guards=[
        ("payload.new_multiplier == 0", "mutation.invariant", 'format!("Surface {} needs at least one instance.", payload.id.0)', "value == 0"),
    ],
    cases=[
        ("✅️", "repeats-a-wall", "repeats the wall four times", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    (snapshot(model), super::change_surface_multiplier(crate::model::EntityId(3), 4))'''),
        ("⛔️", "refuses-zero", "refuses zero instances", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    (snapshot(model), super::change_surface_multiplier(crate::model::EntityId(3), 0))'''),
    ],
)
# endregion 📐️Surfaces


# region 🪟️Fenestration — 211–220, 228–235
G1_WINDOW_FIELDS = [("id", "crate::model::EntityId"), ("name", "String"), ("surface_id", "crate::model::EntityId"), ("u_value_w_m2k", "f64"), ("shgc", "f64"), ("vlt", "f64"), ("area_m2", "f64"), ("height_m", "f64"), ("sill_height_m", "f64"), ("frame_conductance_w_k", "f64"), ("divider_conductance_w_k", "f64"), ("overhang_depth_m", "f64"), ("overhang_offset_m", "f64"), ("fin_depth_m", "f64"), ("fin_offset_m", "f64"), ("glazing_construction_id", "Option<crate::model::EntityId>")]

G1_WINDOW_SCENARIO = '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.materials.push(fixtures::material(4, "PLASTERBOARD"));
    model.constructions.push(fixtures::construction(2, "LIGHTWEIGHT WALL", 4));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    model.fenestrations.push(fixtures::window(5, "WINDOW SOUTH", 3));
'''


def g1_window_scalar(*, number, slug, emoji, record, display, doc, field, unit, label, probe, happy, refusal, ty="f64", guards=(), clone=""):
    """🪟️ A one-scalar `change` on `Model::fenestrations`, with both fixture scenarios built on the
    same §5.2 south-facing window so the committed vectors stay comparable across the ten kinds."""
    g1_scalar(
        number=number,
        slug=slug,
        emoji=emoji,
        entity="fenestration",
        record=record,
        display=display,
        doc=doc,
        collection="fenestrations",
        noun="Fenestration",
        field=field,
        payload_name="new_" + field,
        ty=ty,
        unit=unit,
        clone=clone,
        label=label,
        probe=probe,
        guards=guards,
        cases=[
            ("✅️", happy[0], happy[1], G1_WINDOW_SCENARIO + "    (snapshot(model), super::" + snake(slug) + "(crate::model::EntityId(5), " + happy[2] + "))"),
            ("⛔️", refusal[0], refusal[1], G1_WINDOW_SCENARIO + "    (snapshot(model), super::" + snake(slug) + "(crate::model::EntityId(" + refusal[3] + "), " + refusal[2] + "))"),
        ],
    )


g1_create(
    number=211,
    slug="create-fenestration",
    emoji="🪟",
    entity="fenestration",
    record="CreatedFenestration",
    display="Create Fenestration",
    doc="Adds one window, skylight or door to an existing host surface, with the full optics, geometry and attached-shading payload the entity carries — including the optional `glazingConstructionId` that supersedes the three scalar optics fields when it is set.",
    collection="fenestrations",
    noun="Fenestration",
    fields=G1_WINDOW_FIELDS,
    literal="crate::model::Fenestration { id: payload.id, name: payload.name.clone(), surface_id: payload.surface_id, u_value_w_m2k: payload.u_value_w_m2k, shgc: payload.shgc, vlt: payload.vlt, area_m2: payload.area_m2, height_m: payload.height_m, sill_height_m: payload.sill_height_m, frame_conductance_w_k: payload.frame_conductance_w_k, divider_conductance_w_k: payload.divider_conductance_w_k, overhang_depth_m: payload.overhang_depth_m, overhang_offset_m: payload.overhang_offset_m, fin_depth_m: payload.fin_depth_m, fin_offset_m: payload.fin_offset_m, glazing_construction_id: payload.glazing_construction_id }",
    py_literal='{"id": entity_id, "name": payload["name"], "surface_id": payload["surfaceId"], "u_value_w_m2k": payload["uValueWM2k"], "shgc": payload["shgc"], "vlt": payload["vlt"], "area_m2": payload["areaM2"], "height_m": payload["heightM"], "sill_height_m": payload["sillHeightM"], "frame_conductance_w_k": payload["frameConductanceWK"], "divider_conductance_w_k": payload["dividerConductanceWK"], "overhang_depth_m": payload["overhangDepthM"], "overhang_offset_m": payload["overhangOffsetM"], "fin_depth_m": payload["finDepthM"], "fin_offset_m": payload["finOffsetM"], "glazing_construction_id": payload["glazingConstructionId"]}',
    undo="delete_fenestration(payload.id)",
    py_undo='    return [("delete-fenestration", {"id": payload["id"]})]',
    label='format!("Create fenestration \\"{}\\"", self.name)',
    probe='create_fenestration(crate::model::EntityId(5), "WINDOW SOUTH".to_string(), crate::model::EntityId(3), 3.0, 0.787, 0.86, 6.0, 2.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None)',
    guards=[
        ("payload.name.trim().is_empty()", "mutation.invariant", '"A fenestration name must not be blank."', 'not payload["name"].strip()'),
        ("!base.model.surfaces.iter().any(|item| item.id == payload.surface_id)", "mutation.target-missing", 'format!("Surface {} does not exist.", payload.surface_id.0)', 'not any(row["id"] == payload["surfaceId"] for row in before["model"]["surfaces"])'),
        ("payload.glazing_construction_id.is_some_and(|glazing| !base.model.constructions.iter().any(|item| item.id == glazing))", "mutation.target-missing", '"The named glazing construction does not exist."', '(payload["glazingConstructionId"] is not None and not any(row["id"] == payload["glazingConstructionId"] for row in before["model"]["constructions"]))'),
        (g1_rs_positive("payload.u_value_w_m2k"), "mutation.invariant", 'format!("Fenestration {} needs a positive finite U-value.", payload.id.0)', g1_py_positive('payload["uValueWM2k"]')),
        (g1_rs_fraction("payload.shgc"), "mutation.invariant", 'format!("Fenestration {} needs an SHGC in 0..=1.", payload.id.0)', g1_py_fraction('payload["shgc"]')),
        (g1_rs_fraction("payload.vlt"), "mutation.invariant", 'format!("Fenestration {} needs a visible transmittance in 0..=1.", payload.id.0)', g1_py_fraction('payload["vlt"]')),
        (g1_rs_positive("payload.area_m2"), "mutation.invariant", 'format!("Fenestration {} needs a positive finite area.", payload.id.0)', g1_py_positive('payload["areaM2"]')),
        (g1_rs_positive("payload.height_m"), "mutation.invariant", 'format!("Fenestration {} needs a positive finite height.", payload.id.0)', g1_py_positive('payload["heightM"]')),
        (g1_rs_non_negative("payload.sill_height_m"), "mutation.invariant", 'format!("Fenestration {} needs a non-negative finite sill height.", payload.id.0)', g1_py_non_negative('payload["sillHeightM"]')),
    ],
    cases=[
        ("✅️", "adds-a-south-window", "adds the south window", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    (snapshot(model), super::create_fenestration(crate::model::EntityId(5), "WINDOW SOUTH".into(), crate::model::EntityId(3), 3.0, 0.787, 0.86, 6.0, 2.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None))'''),
        ("⛔️", "refuses-a-missing-host", "refuses an absent host surface", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    (snapshot(model), super::create_fenestration(crate::model::EntityId(5), "WINDOW SOUTH".into(), crate::model::EntityId(9), 3.0, 0.787, 0.86, 6.0, 2.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None))'''),
    ],
)

g1_delete(
    number=212,
    slug="delete-fenestration",
    emoji="🚪",
    entity="fenestration",
    record="DeletedFenestration",
    display="Delete Fenestration",
    doc="Removes one fenestration. Nothing in the document references a fenestration, so it neither cascades nor restricts.",
    collection="fenestrations",
    noun="Fenestration",
    undo="create_fenestration(existing.id, existing.name.clone(), existing.surface_id, existing.u_value_w_m2k, existing.shgc, existing.vlt, existing.area_m2, existing.height_m, existing.sill_height_m, existing.frame_conductance_w_k, existing.divider_conductance_w_k, existing.overhang_depth_m, existing.overhang_offset_m, existing.fin_depth_m, existing.fin_offset_m, existing.glazing_construction_id)",
    py_undo='''    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("create-fenestration", {"id": item["id"], "name": item["name"], "surfaceId": item["surface_id"], "uValueWM2k": item["u_value_w_m2k"], "shgc": item["shgc"], "vlt": item["vlt"], "areaM2": item["area_m2"], "heightM": item["height_m"], "sillHeightM": item["sill_height_m"], "frameConductanceWK": item["frame_conductance_w_k"], "dividerConductanceWK": item["divider_conductance_w_k"], "overhangDepthM": item["overhang_depth_m"], "overhangOffsetM": item["overhang_offset_m"], "finDepthM": item["fin_depth_m"], "finOffsetM": item["fin_offset_m"], "glazingConstructionId": item["glazing_construction_id"]})]''',
    label='format!("Delete fenestration {}", self.id.0)',
    probe="delete_fenestration(crate::model::EntityId(5))",
    cases=[
        ("✅️", "removes-a-window", "removes the south window", G1_WINDOW_SCENARIO + "    (snapshot(model), super::delete_fenestration(crate::model::EntityId(5)))"),
        ("⛔️", "refuses-a-missing-one", "refuses an absent fenestration", G1_WINDOW_SCENARIO + "    (snapshot(model), super::delete_fenestration(crate::model::EntityId(9)))"),
    ],
)

g1_scalar(
    number=213,
    slug="rename-fenestration",
    emoji="🏁",
    entity="fenestration",
    record="RenamedFenestration",
    display="Rename Fenestration",
    doc="Sets one fenestration's identity field; a name another fenestration already holds is refused.",
    collection="fenestrations",
    noun="Fenestration",
    field="name",
    payload_name="new_name",
    ty="String",
    unit="name",
    clone=".clone()",
    label='format!("Rename fenestration {} to \\"{}\\"", self.id.0, self.new_name)',
    probe='rename_fenestration(crate::model::EntityId(5), "WINDOW S".to_string())',
    guards=[
        ("payload.new_name.trim().is_empty()", "mutation.invariant", '"A fenestration name must not be blank."', 'not value.strip()'),
        ("base.model.fenestrations.iter().any(|item| item.id != payload.id && item.name == payload.new_name)", "mutation.duplicate-id", 'format!("Another fenestration is already named \\"{}\\".", payload.new_name)', 'any(row["id"] != entity_id and row["name"] == value for row in before["model"]["fenestrations"])'),
    ],
    cases=[
        ("✅️", "renames-a-window", "renames the south window", G1_WINDOW_SCENARIO + '    (snapshot(model), super::rename_fenestration(crate::model::EntityId(5), "WINDOW S".into()))'),
        ("⛔️", "refuses-a-blank-name", "refuses a blank name", G1_WINDOW_SCENARIO + '    (snapshot(model), super::rename_fenestration(crate::model::EntityId(5), " ".into()))'),
    ],
)

g1_scalar(
    number=214,
    slug="change-fenestration-surface",
    emoji="🧲",
    entity="fenestration",
    record="ChangedFenestrationSurface",
    display="Change Fenestration Surface",
    doc="Rehosts the fenestration on another existing surface — the surface whose orientation, tilt and zone the window then inherits.",
    collection="fenestrations",
    noun="Fenestration",
    field="surface_id",
    payload_name="new_surface_id",
    ty="crate::model::EntityId",
    unit="host surface",
    label='format!("Move fenestration {} to surface {}", self.id.0, self.new_surface_id.0)',
    probe="change_fenestration_surface(crate::model::EntityId(5), crate::model::EntityId(6))",
    guards=[
        ("!base.model.surfaces.iter().any(|item| item.id == payload.new_surface_id)", "mutation.target-missing", 'format!("Surface {} does not exist.", payload.new_surface_id.0)', 'not any(row["id"] == value for row in before["model"]["surfaces"])'),
    ],
    cases=[
        ("✅️", "rehosts-a-window", "rehosts the window on the north wall", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    model.surfaces.push(fixtures::surface(6, "WALL NORTH", 1, 2));
    model.fenestrations.push(fixtures::window(5, "WINDOW SOUTH", 3));
    (snapshot(model), super::change_fenestration_surface(crate::model::EntityId(5), crate::model::EntityId(6)))'''),
        ("⛔️", "refuses-a-missing-host", "refuses an absent host surface", G1_WINDOW_SCENARIO + "    (snapshot(model), super::change_fenestration_surface(crate::model::EntityId(5), crate::model::EntityId(9)))"),
    ],
)

g1_window_scalar(
    number=215,
    slug="change-fenestration-u-value",
    emoji="🌐",
    record="ChangedFenestrationUValue",
    display="Change Fenestration U-Value",
    doc="Sets the glazing's air-to-air thermal transmittance in W/(m²·K) — the conductance the film-free window term multiplies by area and the outdoor-to-zone temperature difference.",
    field="u_value_w_m2k",
    unit="U-value",
    label='format!("Change fenestration {} U-value to {} W/(m²·K)", self.id.0, self.new_u_value_w_m2k)',
    probe="change_fenestration_u_value(crate::model::EntityId(5), 2.74)",
    guards=[(g1_rs_positive("payload.new_u_value_w_m2k"), "mutation.invariant", 'format!("Fenestration {} needs a positive finite U-value, got {}.", payload.id.0, payload.new_u_value_w_m2k)', g1_py_positive("value"))],
    happy=("swaps-the-glazing", "moves to the EnergyPlus simple-glazing U-value", "2.74"),
    refusal=("refuses-zero-u", "refuses a zero U-value", "0.0", "5"),
)

g1_window_scalar(
    number=216,
    slug="change-fenestration-shgc",
    emoji="🌇",
    record="ChangedFenestrationShgc",
    display="Change Fenestration SHGC",
    doc="Sets the solar heat gain coefficient — the fraction of incident solar the glazing passes to the zone, directly and by re-radiation.",
    field="shgc",
    unit="SHGC",
    label='format!("Change fenestration {} SHGC to {}", self.id.0, self.new_shgc)',
    probe="change_fenestration_shgc(crate::model::EntityId(5), 0.76)",
    guards=[(g1_rs_fraction("payload.new_shgc"), "mutation.invariant", 'format!("Fenestration {} needs an SHGC in 0..=1, got {}.", payload.id.0, payload.new_shgc)', g1_py_fraction("value"))],
    happy=("dims-the-solar-gain", "moves to the EnergyPlus simple-glazing SHGC", "0.76"),
    refusal=("refuses-shgc-above-one", "refuses an SHGC above one", "1.4", "5"),
)

g1_window_scalar(
    number=217,
    slug="change-fenestration-vlt",
    emoji="🌈",
    record="ChangedFenestrationVlt",
    display="Change Fenestration VLT",
    doc="Sets the visible light transmittance — the daylight fraction the illuminance calculation reads, independent of the solar gain the SHGC governs.",
    field="vlt",
    unit="visible transmittance",
    label='format!("Change fenestration {} visible transmittance to {}", self.id.0, self.new_vlt)',
    probe="change_fenestration_vlt(crate::model::EntityId(5), 0.84)",
    guards=[(g1_rs_fraction("payload.new_vlt"), "mutation.invariant", 'format!("Fenestration {} needs a visible transmittance in 0..=1, got {}.", payload.id.0, payload.new_vlt)', g1_py_fraction("value"))],
    happy=("dims-the-daylight", "moves to the EnergyPlus simple-glazing VT", "0.84"),
    refusal=("refuses-negative-vlt", "refuses a negative transmittance", "-0.1", "5"),
)

g1_window_scalar(
    number=218,
    slug="change-fenestration-area",
    emoji="🟥",
    record="ChangedFenestrationArea",
    display="Change Fenestration Area",
    doc="Sets the glazed area in square metres — the area every window heat-balance and solar term is proportional to.",
    field="area_m2",
    unit="area",
    label='format!("Change fenestration {} area to {} m²", self.id.0, self.new_area_m2)',
    probe="change_fenestration_area(crate::model::EntityId(5), 12.0)",
    guards=[(g1_rs_positive("payload.new_area_m2"), "mutation.invariant", 'format!("Fenestration {} needs a positive finite area, got {}.", payload.id.0, payload.new_area_m2)', g1_py_positive("value"))],
    happy=("doubles-the-glazing", "doubles the glazed area", "12.0"),
    refusal=("refuses-zero-area", "refuses a zero area", "0.0", "5"),
)

g1_window_scalar(
    number=219,
    slug="change-fenestration-frame-conductance",
    emoji="🖼",
    record="ChangedFenestrationFrameConductance",
    display="Change Fenestration Frame Conductance",
    doc="Sets the frame's whole-assembly thermal conductance in W/K, added to the glazing term rather than distributed over the area.",
    field="frame_conductance_w_k",
    unit="frame conductance",
    label='format!("Change fenestration {} frame conductance to {} W/K", self.id.0, self.new_frame_conductance_w_k)',
    probe="change_fenestration_frame_conductance(crate::model::EntityId(5), 1.2)",
    guards=[(g1_rs_non_negative("payload.new_frame_conductance_w_k"), "mutation.invariant", 'format!("Fenestration {} needs a non-negative finite frame conductance, got {}.", payload.id.0, payload.new_frame_conductance_w_k)', g1_py_non_negative("value"))],
    happy=("adds-a-frame", "gives the window a conducting frame", "1.2"),
    refusal=("refuses-negative-frame", "refuses a negative frame conductance", "-1.0", "5"),
)

g1_window_scalar(
    number=220,
    slug="change-fenestration-divider-conductance",
    emoji="🧷",
    record="ChangedFenestrationDividerConductance",
    display="Change Fenestration Divider Conductance",
    doc="Sets the divider bars' whole-assembly thermal conductance in W/K, the frame term's sibling for the muntins inside the glazed area.",
    field="divider_conductance_w_k",
    unit="divider conductance",
    label='format!("Change fenestration {} divider conductance to {} W/K", self.id.0, self.new_divider_conductance_w_k)',
    probe="change_fenestration_divider_conductance(crate::model::EntityId(5), 0.4)",
    guards=[(g1_rs_non_negative("payload.new_divider_conductance_w_k"), "mutation.invariant", 'format!("Fenestration {} needs a non-negative finite divider conductance, got {}.", payload.id.0, payload.new_divider_conductance_w_k)', g1_py_non_negative("value"))],
    happy=("adds-dividers", "gives the window divider bars", "0.4"),
    refusal=("refuses-a-negative", "refuses a negative divider conductance", "-1.0", "5"),
)

g1_window_scalar(
    number=230,
    slug="change-fenestration-height",
    emoji="⬆",
    record="ChangedFenestrationHeight",
    display="Change Fenestration Height",
    doc="Sets the glazing's head-to-sill height in metres — the length the overhang and fin shadow geometry is projected against.",
    field="height_m",
    unit="height",
    label='format!("Change fenestration {} height to {} m", self.id.0, self.new_height_m)',
    probe="change_fenestration_height(crate::model::EntityId(5), 2.4)",
    guards=[(g1_rs_positive("payload.new_height_m"), "mutation.invariant", 'format!("Fenestration {} needs a positive finite height, got {}.", payload.id.0, payload.new_height_m)', g1_py_positive("value"))],
    happy=("raises-the-head", "raises the window head", "2.4"),
    refusal=("refuses-zero-height", "refuses a zero height", "0.0", "5"),
)

g1_window_scalar(
    number=231,
    slug="change-fenestration-sill-height",
    emoji="⬇",
    record="ChangedFenestrationSillHeight",
    display="Change Fenestration Sill Height",
    doc="Sets the height of the glazing's sill above the host surface's base in metres.",
    field="sill_height_m",
    unit="sill height",
    label='format!("Change fenestration {} sill height to {} m", self.id.0, self.new_sill_height_m)',
    probe="change_fenestration_sill_height(crate::model::EntityId(5), 0.9)",
    guards=[(g1_rs_non_negative("payload.new_sill_height_m"), "mutation.invariant", 'format!("Fenestration {} needs a non-negative finite sill height, got {}.", payload.id.0, payload.new_sill_height_m)', g1_py_non_negative("value"))],
    happy=("raises-the-sill", "raises the window sill", "0.9"),
    refusal=("refuses-negative-sill", "refuses a negative sill height", "-0.2", "5"),
)

g1_window_scalar(
    number=232,
    slug="change-fenestration-overhang-depth",
    emoji="🧢",
    record="ChangedFenestrationOverhangDepth",
    display="Change Fenestration Overhang Depth",
    doc="Sets how far the horizontal projection above the window head reaches out of the glazing plane, in metres — ANSI/ASHRAE 140 §5.2 cases 610/910 are exactly this field.",
    field="overhang_depth_m",
    unit="overhang depth",
    label='format!("Change fenestration {} overhang depth to {} m", self.id.0, self.new_overhang_depth_m)',
    probe="change_fenestration_overhang_depth(crate::model::EntityId(5), 1.0)",
    guards=[(g1_rs_non_negative("payload.new_overhang_depth_m"), "mutation.invariant", 'format!("Fenestration {} needs a non-negative finite overhang depth, got {}.", payload.id.0, payload.new_overhang_depth_m)', g1_py_non_negative("value"))],
    happy=("adds-case-610-shade", "adds case 610's one-metre overhang", "1.0"),
    refusal=("refuses-negative-depth", "refuses a negative overhang depth", "-1.0", "5"),
)

g1_window_scalar(
    number=233,
    slug="change-fenestration-overhang-offset",
    emoji="🎩",
    record="ChangedFenestrationOverhangOffset",
    display="Change Fenestration Overhang Offset",
    doc="Sets how far above the window head the horizontal projection sits, in metres.",
    field="overhang_offset_m",
    unit="overhang offset",
    label='format!("Change fenestration {} overhang offset to {} m", self.id.0, self.new_overhang_offset_m)',
    probe="change_fenestration_overhang_offset(crate::model::EntityId(5), 0.5)",
    guards=[(g1_rs_non_negative("payload.new_overhang_offset_m"), "mutation.invariant", 'format!("Fenestration {} needs a non-negative finite overhang offset, got {}.", payload.id.0, payload.new_overhang_offset_m)', g1_py_non_negative("value"))],
    happy=("lifts-the-overhang", "lifts the overhang half a metre", "0.5"),
    refusal=("refuses-negative-offset", "refuses a negative overhang offset", "-0.5", "5"),
)

g1_window_scalar(
    number=234,
    slug="change-fenestration-fin-depth",
    emoji="🐬",
    record="ChangedFenestrationFinDepth",
    display="Change Fenestration Fin Depth",
    doc="Sets how far the two vertical projections beside the window jambs reach out of the glazing plane, in metres — ANSI/ASHRAE 140 §5.2 cases 630/930 are exactly this field.",
    field="fin_depth_m",
    unit="fin depth",
    label='format!("Change fenestration {} fin depth to {} m", self.id.0, self.new_fin_depth_m)',
    probe="change_fenestration_fin_depth(crate::model::EntityId(5), 1.0)",
    guards=[(g1_rs_non_negative("payload.new_fin_depth_m"), "mutation.invariant", 'format!("Fenestration {} needs a non-negative finite fin depth, got {}.", payload.id.0, payload.new_fin_depth_m)', g1_py_non_negative("value"))],
    happy=("adds-case-630-fins", "adds case 630's one-metre fins", "1.0"),
    refusal=("refuses-negative-fin", "refuses a negative fin depth", "-1.0", "5"),
)

g1_window_scalar(
    number=235,
    slug="change-fenestration-fin-offset",
    emoji="🐋",
    record="ChangedFenestrationFinOffset",
    display="Change Fenestration Fin Offset",
    doc="Sets how far beside the window jambs the two vertical projections stand, in metres.",
    field="fin_offset_m",
    unit="fin offset",
    label='format!("Change fenestration {} fin offset to {} m", self.id.0, self.new_fin_offset_m)',
    probe="change_fenestration_fin_offset(crate::model::EntityId(5), 0.2)",
    guards=[(g1_rs_non_negative("payload.new_fin_offset_m"), "mutation.invariant", 'format!("Fenestration {} needs a non-negative finite fin offset, got {}.", payload.id.0, payload.new_fin_offset_m)', g1_py_non_negative("value"))],
    happy=("spreads-the-fins", "moves the fins clear of the jambs", "0.2"),
    refusal=("refuses-negative-offset", "refuses a negative fin offset", "-0.2", "5"),
)

kind(
    number=228,
    slug="bind-fenestration-glazing-construction",
    emoji=g1_emoji("🧊"),
    verb="bind",
    entity="fenestration",
    record="BoundFenestrationGlazingConstruction",
    display="Bind Fenestration Glazing Construction",
    doc="Points the fenestration's optional glazing slot at an existing layered construction, which then supersedes `uValueWM2k`/`shgc`/`vlt`. This is the schema seam the ticket's oracle comparison needed: with only the three scalars a semio→EnergyPlus translation can emit nothing richer than `WindowMaterial:SimpleGlazingSystem`, worth +5.7 to +8.1 % of annual cooling on ANSI/ASHRAE 140 cases 600/900.",
    fields=[("id", "crate::model::EntityId"), ("construction_id", "crate::model::EntityId")],
    label='format!("Bind fenestration {} to glazing construction {}", self.id.0, self.construction_id.0)',
    target="vec![self.id.0.to_string()]",
    probe="bind_fenestration_glazing_construction(crate::model::EntityId(5), crate::model::EntityId(7))",
    diff='''    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fenestration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.constructions.iter().any(|item| item.id == payload.construction_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction {} does not exist.", payload.construction_id.0), [payload.id.0.to_string()]);
    }
    if existing.glazing_construction_id == Some(payload.construction_id) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fenestration {} is already glazed with construction {}.", payload.id.0, payload.construction_id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.fenestrations.iter_mut().find(|item| item.id == payload.id) {
        item.glazing_construction_id = Some(payload.construction_id);
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !base.model.constructions.iter().any(|item| item.id == payload.construction_id) {
        return Vec::new();
    }
    match existing.glazing_construction_id {
        Some(previous) if previous == payload.construction_id => Vec::new(),
        Some(previous) => vec![super::bind_fenestration_glazing_construction(payload.id, previous)],
        None => vec![super::clear_fenestration_glazing_construction(payload.id)],
    }''',
    outcome_classes=["applied", "warning", "error"],
    python='''    """🧊️ `bind-fenestration-glazing-construction{id,constructionId}` — taxonomy.md's `bind` verb
    filling the optional glazing slot with a real layer stack."""
    entity_id, construction_id = payload["id"], payload["constructionId"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not any(row["id"] == construction_id for row in before["model"]["constructions"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["glazing_construction_id"] == construction_id:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["glazing_construction_id"] = construction_id
    return after, applied()''',
    python_invert='''    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    previous = item["glazing_construction_id"]
    if previous is None:
        return [("clear-fenestration-glazing-construction", {"id": payload["id"]})]
    return [("bind-fenestration-glazing-construction", {"id": payload["id"], "constructionId": previous})]''',
    cases=[
        ("✅️", "glazes-a-window", "gives the window a real layer stack", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.materials.push(fixtures::material(4, "PLASTERBOARD"));
    model.constructions.push(fixtures::construction(7, "DOUBLE CLEAR", 4));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    model.fenestrations.push(fixtures::window(5, "WINDOW SOUTH", 3));
    (snapshot(model), super::bind_fenestration_glazing_construction(crate::model::EntityId(5), crate::model::EntityId(7)))'''),
        ("⛔️", "refuses-no-stack", "refuses an absent construction", G1_WINDOW_SCENARIO + "    (snapshot(model), super::bind_fenestration_glazing_construction(crate::model::EntityId(5), crate::model::EntityId(9)))"),
    ],
)

kind(
    number=229,
    slug="clear-fenestration-glazing-construction",
    emoji=g1_emoji("🫗"),
    verb="clear",
    entity="fenestration",
    record="ClearedFenestrationGlazingConstruction",
    display="Clear Fenestration Glazing Construction",
    doc="Empties the fenestration's optional glazing slot, handing the optics back to `uValueWM2k`/`shgc`/`vlt`. Refused when the slot is already empty, so an undo chain can never invent a clear that had no partner.",
    fields=[("id", "crate::model::EntityId")],
    label='format!("Clear fenestration {} glazing construction", self.id.0)',
    target="vec![self.id.0.to_string()]",
    probe="clear_fenestration_glazing_construction(crate::model::EntityId(5))",
    diff='''    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fenestration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.glazing_construction_id.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fenestration {} has no glazing construction to clear.", payload.id.0), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    if let Some(item) = model.fenestrations.iter_mut().find(|item| item.id == payload.id) {
        item.glazing_construction_id = None;
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    match base.model.fenestrations.iter().find(|item| item.id == payload.id).and_then(|item| item.glazing_construction_id) {
        Some(previous) => vec![super::bind_fenestration_glazing_construction(payload.id, previous)],
        None => Vec::new(),
    }''',
    outcome_classes=["applied", "error"],
    python='''    """🫗️ `clear-fenestration-glazing-construction{id}` — `bind`'s inverse partner."""
    entity_id = payload["id"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None or item["glazing_construction_id"] is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["glazing_construction_id"] = None
    return after, applied()''',
    python_invert='''    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("bind-fenestration-glazing-construction", {"id": payload["id"], "constructionId": item["glazing_construction_id"]})]''',
    cases=[
        ("✅️", "ungazes-a-window", "hands the optics back to the scalars", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.materials.push(fixtures::material(4, "PLASTERBOARD"));
    model.constructions.push(fixtures::construction(7, "DOUBLE CLEAR", 4));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    let mut glazed = fixtures::window(5, "WINDOW SOUTH", 3);
    glazed.glazing_construction_id = Some(crate::model::EntityId(7));
    model.fenestrations.push(glazed);
    (snapshot(model), super::clear_fenestration_glazing_construction(crate::model::EntityId(5)))'''),
        ("⛔️", "refuses-empty", "refuses a slot that is already empty", G1_WINDOW_SCENARIO + "    (snapshot(model), super::clear_fenestration_glazing_construction(crate::model::EntityId(5)))"),
    ],
)
# endregion 🪟️Fenestration


def g1_py_schedule_known(expr: str) -> str:
    """📅️ Python: "the model's own `ScheduleSet` defines this id", over the five families the
    snapshot document carries — the same set `Model::validate` builds for its reference check."""
    return 'any(row["id"] == @E@ for family in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][family])'.replace("@E@", expr)


# region 🌳️ShadingSurfaces — 221–225
G1_SHADING_SCENARIO = '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.shading_surfaces.push(fixtures::shading(11, "SITE AWNING"));
'''

g1_create(
    number=221,
    slug="create-shading-surface",
    emoji="🌳",
    entity="shading-surface",
    record="CreatedShadingSurface",
    display="Create Shading Surface",
    doc="Adds one free-standing solar obstruction — the site-level sibling of a window's own attached overhang and fins, which stay on the fenestration.",
    collection="shading_surfaces",
    noun="Shading surface",
    fields=[("id", "crate::model::EntityId"), ("name", "String"), ("vertices_m", "Vec<[f64; 3]>"), ("transmittance_schedule_id", "Option<crate::model::ScheduleId>")],
    literal="crate::model::ShadingSurface { id: payload.id, name: payload.name.clone(), vertices_m: payload.vertices_m.clone(), transmittance_schedule_id: payload.transmittance_schedule_id }",
    py_literal='{"id": entity_id, "name": payload["name"], "vertices_m": payload["verticesM"], "transmittance_schedule_id": payload["transmittanceScheduleId"]}',
    undo="delete_shading_surface(payload.id)",
    py_undo='    return [("delete-shading-surface", {"id": payload["id"]})]',
    label='format!("Create shading surface \\"{}\\"", self.name)',
    probe='create_shading_surface(crate::model::EntityId(11), "SITE AWNING".to_string(), vec![[0.0, -2.0, 3.0], [8.0, -2.0, 3.0], [8.0, 0.0, 3.0]], None)',
    guards=[
        ("payload.name.trim().is_empty()", "mutation.invariant", '"A shading surface name must not be blank."', 'not payload["name"].strip()'),
        ("payload.vertices_m.len() < 3", "mutation.invariant", 'format!("A shading polygon needs at least three vertices, got {}.", payload.vertices_m.len())', 'len(payload["verticesM"]) < 3'),
        ("payload.transmittance_schedule_id.is_some_and(|schedule| !base.model.schedules.contains(schedule))", "mutation.target-missing", '"The named transmittance schedule is not defined by this model."', '(payload["transmittanceScheduleId"] is not None and not ' + g1_py_schedule_known('payload["transmittanceScheduleId"]') + ')'),
    ],
    cases=[
        ("✅️", "adds-an-awning", "adds a site awning", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    (snapshot(model), super::create_shading_surface(crate::model::EntityId(11), "SITE AWNING".into(), vec![[0.0, -2.0, 3.0], [8.0, -2.0, 3.0], [8.0, 0.0, 3.0], [0.0, 0.0, 3.0]], None))'''),
        ("⛔️", "refuses-a-line", "refuses a two-vertex polygon", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    (snapshot(model), super::create_shading_surface(crate::model::EntityId(11), "SITE AWNING".into(), vec![[0.0, -2.0, 3.0], [8.0, -2.0, 3.0]], None))'''),
    ],
)

g1_delete(
    number=222,
    slug="delete-shading-surface",
    emoji="🪵",
    entity="shading-surface",
    record="DeletedShadingSurface",
    display="Delete Shading Surface",
    doc="Removes one free-standing solar obstruction. Nothing in the document references a shading surface, so it neither cascades nor restricts.",
    collection="shading_surfaces",
    noun="Shading surface",
    undo="create_shading_surface(existing.id, existing.name.clone(), existing.vertices_m.clone(), existing.transmittance_schedule_id)",
    py_undo='''    item = next(row for row in before["model"]["shading_surfaces"] if row["id"] == payload["id"])
    return [("create-shading-surface", {"id": item["id"], "name": item["name"], "verticesM": item["vertices_m"], "transmittanceScheduleId": item["transmittance_schedule_id"]})]''',
    label='format!("Delete shading surface {}", self.id.0)',
    probe="delete_shading_surface(crate::model::EntityId(11))",
    cases=[
        ("✅️", "removes-an-awning", "removes the site awning", G1_SHADING_SCENARIO + "    (snapshot(model), super::delete_shading_surface(crate::model::EntityId(11)))"),
        ("⛔️", "refuses-a-missing-one", "refuses an absent shading surface", G1_SHADING_SCENARIO + "    (snapshot(model), super::delete_shading_surface(crate::model::EntityId(9)))"),
    ],
)

g1_scalar(
    number=223,
    slug="rename-shading-surface",
    emoji="🏕",
    entity="shading-surface",
    record="RenamedShadingSurface",
    display="Rename Shading Surface",
    doc="Sets one shading surface's identity field; a name another shading surface already holds is refused.",
    collection="shading_surfaces",
    noun="Shading surface",
    field="name",
    payload_name="new_name",
    ty="String",
    unit="name",
    clone=".clone()",
    label='format!("Rename shading surface {} to \\"{}\\"", self.id.0, self.new_name)',
    probe='rename_shading_surface(crate::model::EntityId(11), "AWNING".to_string())',
    guards=[
        ("payload.new_name.trim().is_empty()", "mutation.invariant", '"A shading surface name must not be blank."', 'not value.strip()'),
        ("base.model.shading_surfaces.iter().any(|item| item.id != payload.id && item.name == payload.new_name)", "mutation.duplicate-id", 'format!("Another shading surface is already named \\"{}\\".", payload.new_name)', 'any(row["id"] != entity_id and row["name"] == value for row in before["model"]["shading_surfaces"])'),
    ],
    cases=[
        ("✅️", "renames-an-awning", "renames the site awning", G1_SHADING_SCENARIO + '    (snapshot(model), super::rename_shading_surface(crate::model::EntityId(11), "AWNING".into()))'),
        ("⛔️", "refuses-a-blank-name", "refuses a blank name", G1_SHADING_SCENARIO + '    (snapshot(model), super::rename_shading_surface(crate::model::EntityId(11), " ".into()))'),
    ],
)

g1_scalar(
    number=224,
    slug="replace-shading-surface-vertices",
    emoji="🗺",
    entity="shading-surface",
    record="ReplacedShadingSurfaceVertices",
    display="Replace Shading Surface Vertices",
    doc="Swaps the shading surface's whole polygon — geometry is one value, so the taxonomy's `replace` verb carries the full ring, minimum three vertices.",
    collection="shading_surfaces",
    noun="Shading surface",
    field="vertices_m",
    payload_name="new_vertices_m",
    ty="Vec<[f64; 3]>",
    unit="polygon",
    clone=".clone()",
    label='format!("Replace shading surface {} with a {}-vertex polygon", self.id.0, self.new_vertices_m.len())',
    probe="replace_shading_surface_vertices(crate::model::EntityId(11), vec![[0.0, -3.0, 3.0], [8.0, -3.0, 3.0], [8.0, 0.0, 3.0]])",
    guards=[
        ("payload.new_vertices_m.len() < 3", "mutation.invariant", 'format!("A shading polygon needs at least three vertices, got {}.", payload.new_vertices_m.len())', 'len(value) < 3'),
    ],
    cases=[
        ("✅️", "deepens-an-awning", "deepens the site awning", G1_SHADING_SCENARIO + "    (snapshot(model), super::replace_shading_surface_vertices(crate::model::EntityId(11), vec![[0.0, -3.0, 3.0], [8.0, -3.0, 3.0], [8.0, 0.0, 3.0], [0.0, 0.0, 3.0]]))"),
        ("⛔️", "refuses-a-line", "refuses a two-vertex polygon", G1_SHADING_SCENARIO + "    (snapshot(model), super::replace_shading_surface_vertices(crate::model::EntityId(11), vec![[0.0, -3.0, 3.0], [8.0, -3.0, 3.0]]))"),
    ],
)

g1_scalar(
    number=225,
    slug="change-shading-surface-transmittance-schedule",
    emoji="⛱",
    entity="shading-surface",
    record="ChangedShadingSurfaceTransmittanceSchedule",
    display="Change Shading Surface Transmittance Schedule",
    doc="Points the shading surface at one of the model's own schedules for its time-varying solar transmittance, or at none for a fully opaque obstruction.",
    collection="shading_surfaces",
    noun="Shading surface",
    field="transmittance_schedule_id",
    payload_name="new_transmittance_schedule_id",
    ty="Option<crate::model::ScheduleId>",
    unit="transmittance schedule",
    label='format!("Change shading surface {} transmittance schedule", self.id.0)',
    probe="change_shading_surface_transmittance_schedule(crate::model::EntityId(11), Some(crate::model::ScheduleId(1)))",
    guards=[
        ("payload.new_transmittance_schedule_id.is_some_and(|schedule| !base.model.schedules.contains(schedule))", "mutation.target-missing", '"The named transmittance schedule is not defined by this model."', '(value is not None and not ' + g1_py_schedule_known('value') + ')'),
    ],
    cases=[
        ("✅️", "lets-light-in", "gives the awning a transmittance schedule", '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(1), value: 0.2 });
    model.shading_surfaces.push(fixtures::shading(11, "SITE AWNING"));
    (snapshot(model), super::change_shading_surface_transmittance_schedule(crate::model::EntityId(11), Some(crate::model::ScheduleId(1))))'''),
        ("⛔️", "refuses-a-ghost", "refuses an undefined schedule", G1_SHADING_SCENARIO + "    (snapshot(model), super::change_shading_surface_transmittance_schedule(crate::model::EntityId(11), Some(crate::model::ScheduleId(9))))"),
    ],
)
# endregion 🌳️ShadingSurfaces


# region 🤝️Adjacency — 226–227
G1_ADJACENCY_SCENARIO = '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    model.surfaces.push(fixtures::surface(6, "WALL NORTH", 1, 2));
'''

G1_ADJACENCY_ADDRESS = "[payload.surface_a_id.0.to_string(), payload.surface_b_id.0.to_string()]"

kind(
    number=226,
    slug="connect-surfaces",
    emoji=g1_emoji("🤝"),
    verb="connect",
    entity="surfaces",
    record="ConnectedSurfaces",
    display="Connect Surfaces",
    doc="Creates the adjacency relationship between two existing surfaces. `AdjacencyPair` carries no id of its own, so the pair itself is the address, unordered — connecting B to A when A is already connected to B is a duplicate. The row lands at its ascending `(a, b)` position so `disconnect` ∘ `connect` restores the document exactly.",
    fields=[("surface_a_id", "crate::model::EntityId"), ("surface_b_id", "crate::model::EntityId")],
    label='format!("Connect surfaces {} and {}", self.surface_a_id.0, self.surface_b_id.0)',
    target="vec![self.surface_a_id.0.to_string(), self.surface_b_id.0.to_string()]",
    probe="connect_surfaces(crate::model::EntityId(3), crate::model::EntityId(6))",
    diff='''    if payload.surface_a_id == payload.surface_b_id {
        return protocol::MutationOutcome::error("mutation.invariant", "A surface is not adjacent to itself.", @ADDRESS@);
    }
    if !base.model.surfaces.iter().any(|item| item.id == payload.surface_a_id) || !base.model.surfaces.iter().any(|item| item.id == payload.surface_b_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "An adjacency pair joins two surfaces that already exist.", @ADDRESS@);
    }
    if base.model.adjacency_pairs.iter().any(|item| (item.surface_a_id, item.surface_b_id) == (payload.surface_a_id, payload.surface_b_id) || (item.surface_a_id, item.surface_b_id) == (payload.surface_b_id, payload.surface_a_id)) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", "These two surfaces are already adjacent.", @ADDRESS@);
    }
    let mut model = base.model.clone();
    let pair = crate::model::AdjacencyPair { surface_a_id: payload.surface_a_id, surface_b_id: payload.surface_b_id };
    let position = model.adjacency_pairs.iter().position(|item| (item.surface_a_id, item.surface_b_id) > (pair.surface_a_id, pair.surface_b_id)).unwrap_or(model.adjacency_pairs.len());
    model.adjacency_pairs.insert(position, pair);
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''.replace("@ADDRESS@", G1_ADJACENCY_ADDRESS),
    inverse='''    if payload.surface_a_id == payload.surface_b_id
        || !base.model.surfaces.iter().any(|item| item.id == payload.surface_a_id)
        || !base.model.surfaces.iter().any(|item| item.id == payload.surface_b_id)
        || base.model.adjacency_pairs.iter().any(|item| (item.surface_a_id, item.surface_b_id) == (payload.surface_a_id, payload.surface_b_id) || (item.surface_a_id, item.surface_b_id) == (payload.surface_b_id, payload.surface_a_id))
    {
        return Vec::new();
    }
    vec![super::disconnect_surfaces(payload.surface_a_id, payload.surface_b_id)]''',
    outcome_classes=["applied", "error"],
    python='''    """🤝️ `connect-surfaces{surfaceAId,surfaceBId}` — taxonomy.md's `connect` verb on an edge
    collection with no id of its own, so the unordered pair is the address."""
    first, second = payload["surfaceAId"], payload["surfaceBId"]
    address = [str(first), str(second)]
    if first == second:
        return unchanged(before), rejected("mutation.invariant", address)
    known = [row["id"] for row in before["model"]["surfaces"]]
    if first not in known or second not in known:
        return unchanged(before), rejected("mutation.target-missing", address)
    if any({row["surface_a_id"], row["surface_b_id"]} == {first, second} for row in before["model"]["adjacency_pairs"]):
        return unchanged(before), rejected("mutation.duplicate-id", address)
    after = copy.deepcopy(before)
    rows = after["model"]["adjacency_pairs"]
    position = next((index for index, row in enumerate(rows) if (row["surface_a_id"], row["surface_b_id"]) > (first, second)), len(rows))
    rows.insert(position, {"surface_a_id": first, "surface_b_id": second})
    return after, applied()''',
    python_invert='    return [("disconnect-surfaces", {"surfaceAId": payload["surfaceAId"], "surfaceBId": payload["surfaceBId"]})]',
    cases=[
        ("✅️", "joins-two-walls", "makes the two walls adjacent", G1_ADJACENCY_SCENARIO + "    (snapshot(model), super::connect_surfaces(crate::model::EntityId(3), crate::model::EntityId(6)))"),
        ("⛔️", "refuses-a-self-pair", "refuses a surface adjacent to itself", G1_ADJACENCY_SCENARIO + "    (snapshot(model), super::connect_surfaces(crate::model::EntityId(3), crate::model::EntityId(3)))"),
    ],
)

kind(
    number=227,
    slug="disconnect-surfaces",
    emoji=g1_emoji("💔"),
    verb="disconnect",
    entity="surfaces",
    record="DisconnectedSurfaces",
    display="Disconnect Surfaces",
    doc="Removes the adjacency relationship between two surfaces, addressed by the unordered pair. Refused when the two are not adjacent, so an undo chain can never invent a disconnect that had no partner.",
    fields=[("surface_a_id", "crate::model::EntityId"), ("surface_b_id", "crate::model::EntityId")],
    label='format!("Disconnect surfaces {} and {}", self.surface_a_id.0, self.surface_b_id.0)',
    target="vec![self.surface_a_id.0.to_string(), self.surface_b_id.0.to_string()]",
    probe="disconnect_surfaces(crate::model::EntityId(3), crate::model::EntityId(6))",
    diff='''    let Some(existing) = base.model.adjacency_pairs.iter().find(|item| (item.surface_a_id, item.surface_b_id) == (payload.surface_a_id, payload.surface_b_id) || (item.surface_a_id, item.surface_b_id) == (payload.surface_b_id, payload.surface_a_id)) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "These two surfaces are not adjacent.", @ADDRESS@);
    };
    let _ = existing;
    let mut model = base.model.clone();
    model.adjacency_pairs.retain(|item| (item.surface_a_id, item.surface_b_id) != (payload.surface_a_id, payload.surface_b_id) && (item.surface_a_id, item.surface_b_id) != (payload.surface_b_id, payload.surface_a_id));
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''.replace("@ADDRESS@", G1_ADJACENCY_ADDRESS),
    inverse='''    match base.model.adjacency_pairs.iter().find(|item| (item.surface_a_id, item.surface_b_id) == (payload.surface_a_id, payload.surface_b_id) || (item.surface_a_id, item.surface_b_id) == (payload.surface_b_id, payload.surface_a_id)) {
        Some(existing) => vec![super::connect_surfaces(existing.surface_a_id, existing.surface_b_id)],
        None => Vec::new(),
    }''',
    outcome_classes=["applied", "error"],
    python='''    """💔️ `disconnect-surfaces{surfaceAId,surfaceBId}` — `connect`'s inverse partner."""
    first, second = payload["surfaceAId"], payload["surfaceBId"]
    if not any({row["surface_a_id"], row["surface_b_id"]} == {first, second} for row in before["model"]["adjacency_pairs"]):
        return unchanged(before), rejected("mutation.target-missing", [str(first), str(second)])
    after = copy.deepcopy(before)
    after["model"]["adjacency_pairs"] = [row for row in after["model"]["adjacency_pairs"] if {row["surface_a_id"], row["surface_b_id"]} != {first, second}]
    return after, applied()''',
    python_invert='''    item = next(row for row in before["model"]["adjacency_pairs"] if {row["surface_a_id"], row["surface_b_id"]} == {payload["surfaceAId"], payload["surfaceBId"]})
    return [("connect-surfaces", {"surfaceAId": item["surface_a_id"], "surfaceBId": item["surface_b_id"]})]''',
    cases=[
        ("✅️", "parts-two-walls", "parts the two adjacent walls", G1_ADJACENCY_SCENARIO + '''    model.adjacency_pairs.push(crate::model::AdjacencyPair { surface_a_id: crate::model::EntityId(3), surface_b_id: crate::model::EntityId(6) });
    (snapshot(model), super::disconnect_surfaces(crate::model::EntityId(3), crate::model::EntityId(6)))'''),
        ("⛔️", "refuses-a-missing-pair", "refuses two surfaces that are not adjacent", G1_ADJACENCY_SCENARIO + "    (snapshot(model), super::disconnect_surfaces(crate::model::EntityId(3), crate::model::EntityId(6)))"),
    ],
)
# endregion 🤝️Adjacency
# endregion 🔖️G1ZonesEnvelope


# region 🔖️G2
#: 🧱️ G2 — constructions & materials (`300`–`399`) and internal gains (`400`–`499`).
#:
#: Five reusable spec patterns emit the Rust leaf AND the Python second implementation from one
#: description each, so an id-keyed collection-element kind is data rather than prose:
#: `g2_scalar` (one scalar field behind a range guard), `g2_reference` (one reference field behind a
#: referential-integrity guard), `g2_rename` (the identity field behind a duplicate guard) and the
#: `g2_create`/`g2_delete` pair. The create payload carries the element's `index`, and that is what
#: makes the pair exactly invertible: `delete`'s inverse is a `create` at the position the element
#: actually held, so the two are true inverses for an element anywhere in the collection rather than
#: only for the last one. Every guard is expressible in JSON — no fixture ever carries a NaN or an
#: infinity, which `🦠️mutation/🔣️.json` could not represent.

G2_PAST = {"add": "Added", "change": "Changed", "create": "Created", "delete": "Deleted", "remove": "Removed", "rename": "Renamed", "reorder": "Reordered"}

G2_RS_TEST = {
    "positive": "!{v}.is_finite() || {v} <= 0.0",
    "nonnegative": "!{v}.is_finite() || {v} < 0.0",
    "fraction": "!(0.0..=1.0).contains(&{v})",
    "unit-positive": "!({v} > 0.0 && {v} <= 1.0)",
}

G2_PY_TEST = {
    "positive": '{v} != {v} or {v} in (float("inf"), float("-inf")) or {v} <= 0.0',
    "nonnegative": '{v} != {v} or {v} in (float("inf"), float("-inf")) or {v} < 0.0',
    "fraction": "not 0.0 <= {v} <= 1.0",
    "unit-positive": "not 0.0 < {v} <= 1.0",
}

G2_WORDS = {
    "positive": "a positive finite value",
    "nonnegative": "a finite non-negative value",
    "fraction": "a fraction in [0, 1]",
    "unit-positive": "a fraction in (0, 1]",
}

G2_SCHEDULE_EXISTS = "base.model.schedules.constants.iter().any(|schedule| schedule.id == {v}) || base.model.schedules.daily.iter().any(|schedule| schedule.id == {v}) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == {v}) || base.model.schedules.annual.iter().any(|schedule| schedule.id == {v}) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == {v})"

G2_PY_SCHEDULE_EXISTS = 'any(row["id"] == {v} for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group])'

G2_CLONED = {"String", "Vec<crate::model::EntityId>"}


def g2_titles(slug: str) -> tuple[str, str, str]:
    """🔤 The verb, the past-tense record name and the display title a G2 slug implies."""
    verb, *rest = slug.split("-")
    return verb, G2_PAST[verb] + camel("-".join(rest)), " ".join(part.capitalize() for part in slug.split("-"))


def g2_missing_rs(collection: str, noun: str) -> str:
    return f'''    let Some(existing) = base.model.{collection}.iter().find(|item| item.id == payload.id) else {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{noun} {{}} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    }};'''


def g2_missing_py(collection: str) -> str:
    return f'''    item = next((row for row in before["model"]["{collection}"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])'''


def g2_set_rs(collection: str, field: str, value: str) -> str:
    return f'''    let mut model = base.model.clone();
    if let Some(item) = model.{collection}.iter_mut().find(|item| item.id == payload.id) {{
        item.{field} = {value};
    }}
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''


def g2_set_py(collection: str, field: str, value: str) -> str:
    return f'''    after = copy.deepcopy(before)
    for row in after["model"]["{collection}"]:
        if row["id"] == payload["id"]:
            row["{field}"] = {value}
    return after, applied()'''


def g2_element_kind(*, number, slug, emoji, collection, noun, field, rust_type, doc, guard_rs, guard_py, refuse_rs, show, seed, target_id, happy, bad, bad_seed, bad_description, outcome_classes, what=None, value_rs=None, inverse_value_rs=None, python_value="value", python_inverse_value=None):
    """🧩️ The shared body of every id-keyed single-field G2 kind: target lookup, the caller's guard,
    the no-op comparison, the write, and the inverse that reads the overwritten value off BASE."""
    verb, record, display = g2_titles(slug)
    module = snake(slug)
    what = what or field
    payload_field = f"new_{field}"
    key = field_camel(payload_field)
    v = f"payload.{payload_field}"
    value_rs = value_rs or v
    inverse_value_rs = inverse_value_rs or f"item.{field}"
    python_inverse_value = python_inverse_value or f'item["{field}"]'
    diff = f'''{g2_missing_rs(collection, noun)}{guard_rs}
    if existing.{field} == {v} {{
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("{noun} {{}} already carries this {what}: {show}.", payload.id.0, {v}{".0" if show == "{}" and rust_type.endswith("Id") else ""}));
    }}
{g2_set_rs(collection, field, value_rs)}'''
    inverse = f'''    match base.model.{collection}.iter().find(|item| item.id == payload.id) {{
        Some(item) if item.{field} != {v}{refuse_rs} => vec![super::{module}(payload.id, {inverse_value_rs})],
        _ => Vec::new(),
    }}'''
    python = f'''    """{emoji} `{slug}{{id,{key}}}` — {doc}"""
    value = payload["{key}"]
{g2_missing_py(collection)}{guard_py}
    if item["{field}"] == {python_value}:
        return unchanged(before), applied(("warning", "mutation.no-op"))
{g2_set_py(collection, field, python_value)}'''
    python_invert = f'''    """↩️ The {field} the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["{collection}"] if row["id"] == payload["id"])
    return [("{slug}", {{"id": payload["id"], "{key}": {python_inverse_value}}})]'''
    kind(
        number=number,
        slug=slug,
        emoji=emoji,
        verb=verb,
        entity=noun.lower().replace(" ", "-"),
        record=record,
        display=display,
        doc=doc,
        fields=[("id", "crate::model::EntityId"), (payload_field, rust_type)],
        label=f'format!("{display} of {noun.lower()} {{}}", self.id.0)',
        target="vec![self.id.0.to_string()]",
        diff=diff,
        inverse=inverse,
        outcome_classes=outcome_classes,
        probe=f"{module}(crate::model::EntityId({target_id}), {happy})",
        python=python,
        python_invert=python_invert,
        cases=[
            ("✅️", "applies", f"moves {noun.lower()} {target_id}", f"{seed}\n    (snapshot(model), super::{module}(crate::model::EntityId({target_id}), {happy}))"),
            ("⛔️", "refuses", bad_description, f"{bad_seed}\n    (snapshot(model), super::{module}({bad}))"),
        ],
    )


def g2_scalar(*, number, slug, emoji, collection, noun, field, what, guard, seed, target_id, happy, bad, doc=None, rust_type="f64"):
    """🔢️ One numeric field of one collection element, behind a JSON-expressible range guard."""
    v = f"payload.new_{field}"
    test = G2_RS_TEST[guard]
    guard_rs = f'''
    if {test.format(v=v)} {{
        return protocol::MutationOutcome::error("mutation.invariant", format!("{noun} {{}}: {what} must be {G2_WORDS[guard]}, got {{}}.", payload.id.0, {v}), [payload.id.0.to_string()]);
    }}'''
    guard_py = f'''
    if {G2_PY_TEST[guard].format(v="value")}:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])'''
    g2_element_kind(
        number=number,
        slug=slug,
        emoji=emoji,
        collection=collection,
        noun=noun,
        field=field,
        rust_type=rust_type,
        doc=doc or f"Sets {what} on one {noun.lower()}, addressed by id.",
        what=what,
        guard_rs=guard_rs,
        guard_py=guard_py,
        refuse_rs=f" && !({test.format(v=v)})",
        show="{}",
        seed=seed,
        target_id=target_id,
        happy=happy,
        bad=f"crate::model::EntityId({target_id}), {bad}",
        bad_seed=seed,
        bad_description=f"refuses {what} outside {G2_WORDS[guard]}",
        outcome_classes=["applied", "warning", "error"],
    )


def g2_reference(*, number, slug, emoji, collection, noun, field, ref, ref_noun, seed, target_id, happy, bad, doc=None):
    """🔗️ One reference field of one collection element, behind a referential-integrity guard: a
    reference the document does not define is refused rather than dangled."""
    v = f"payload.new_{field}"
    rust_type = "crate::model::EntityId" if ref == "zone" else "crate::model::ScheduleId"
    exists_rs = f"base.model.zones.iter().any(|zone| zone.id == {v})" if ref == "zone" else f"({G2_SCHEDULE_EXISTS.format(v=v)})"
    exists_py = 'any(row["id"] == value for row in before["model"]["zones"])' if ref == "zone" else G2_PY_SCHEDULE_EXISTS.format(v="value")
    guard_rs = f'''
    if !{exists_rs} {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{ref_noun} {{}} does not exist.", {v}.0), [{v}.0.to_string()]);
    }}'''
    guard_py = f'''
    if not {exists_py}:
        return unchanged(before), rejected("mutation.target-missing", [str(value)])'''
    g2_element_kind(
        number=number,
        slug=slug,
        emoji=emoji,
        collection=collection,
        noun=noun,
        field=field,
        rust_type=rust_type,
        doc=doc or f"Re-points one {noun.lower()}'s {ref_noun.lower()} reference; a {ref_noun.lower()} the document does not define is refused.",
        what=f"{ref_noun.lower()} reference",
        guard_rs=guard_rs,
        guard_py=guard_py,
        refuse_rs=f" && {exists_rs}",
        show="{}",
        seed=seed,
        target_id=target_id,
        happy=happy,
        bad=f"crate::model::EntityId({target_id}), {bad}",
        bad_seed=seed,
        bad_description=f"refuses an undefined {ref_noun.lower()}",
        outcome_classes=["applied", "warning", "error"],
    )


def g2_rename(*, number, slug, emoji, collection, noun, seed, target_id, happy, taken, doc=None):
    """🏷️ The identity field of one collection element; a name another sibling already holds is
    refused, because every EnergyPlus-class report keys on it."""
    v = "payload.new_name"
    duplicate = f"base.model.{collection}.iter().any(|other| other.id != payload.id && other.name == {v})"
    guard_rs = f'''
    if {v}.trim().is_empty() {{
        return protocol::MutationOutcome::error("mutation.invariant", "A {noun.lower()} name must not be blank.", [payload.id.0.to_string()]);
    }}
    if {duplicate} {{
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Another {noun.lower()} is already named \\"{{}}\\".", {v}), [payload.id.0.to_string()]);
    }}'''
    guard_py = f'''
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(row["id"] != payload["id"] and row["name"] == value for row in before["model"]["{collection}"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])'''
    g2_element_kind(
        number=number,
        slug=slug,
        emoji=emoji,
        collection=collection,
        noun=noun,
        field="name",
        rust_type="String",
        doc=doc or f"Sets one {noun.lower()}'s identity field; a name a sibling already holds is refused.",
        what="name",
        guard_rs=guard_rs,
        guard_py=guard_py,
        refuse_rs=f" && !{v}.trim().is_empty() && !{duplicate}",
        show="{}",
        seed=seed,
        target_id=target_id,
        happy=happy,
        bad=f"crate::model::EntityId({target_id}), {taken}",
        bad_seed=seed,
        bad_description="refuses a name a sibling already holds",
        outcome_classes=["applied", "warning", "error"],
        value_rs=f"{v}.clone()",
        inverse_value_rs="item.name.clone()",
    )


def g2_ref_guard_rs(refs: list[tuple[str, str, str]]) -> str:
    blocks = []
    for kind_of, field, ref_noun in refs:
        if kind_of == "zone":
            blocks.append(f'''    if !base.model.zones.iter().any(|zone| zone.id == payload.{field}) {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{ref_noun} {{}} does not exist.", payload.{field}.0), [payload.{field}.0.to_string()]);
    }}''')
        elif kind_of == "schedule":
            blocks.append(f'''    if !({G2_SCHEDULE_EXISTS.format(v=f"payload.{field}")}) {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{ref_noun} {{}} does not exist.", payload.{field}.0), [payload.{field}.0.to_string()]);
    }}''')
        else:
            blocks.append(f'''    if let Some(missing) = payload.{field}.iter().find(|id| !base.model.materials.iter().any(|material| material.id == **id)) {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{ref_noun} {{}} does not exist.", missing.0), [missing.0.to_string()]);
    }}''')
    return ("\n" + "\n".join(blocks)) if blocks else ""


def g2_ref_refused_rs(refs: list[tuple[str, str, str]]) -> str:
    parts = []
    for kind_of, field, _noun in refs:
        if kind_of == "zone":
            parts.append(f"!base.model.zones.iter().any(|zone| zone.id == payload.{field})")
        elif kind_of == "schedule":
            parts.append(f"!({G2_SCHEDULE_EXISTS.format(v=f'payload.{field}')})")
        else:
            parts.append(f"payload.{field}.iter().any(|id| !base.model.materials.iter().any(|material| material.id == *id))")
    return "".join(f" || {part}" for part in parts)


def g2_ref_guard_py(refs: list[tuple[str, str, str]]) -> str:
    blocks = []
    for kind_of, field, _noun in refs:
        key = field_camel(field)
        if kind_of == "zone":
            blocks.append(f'''    if not any(row["id"] == payload["{key}"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["{key}"])])''')
        elif kind_of == "schedule":
            blocks.append(f'''    if not {G2_PY_SCHEDULE_EXISTS.format(v=f'payload["{key}"]')}:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["{key}"])])''')
        else:
            blocks.append(f'''    missing = next((identifier for identifier in payload["{key}"] if not any(row["id"] == identifier for row in before["model"]["materials"])), None)
    if missing is not None:
        return unchanged(before), rejected("mutation.target-missing", [str(missing)])''')
    return ("\n" + "\n".join(blocks)) if blocks else ""


def g2_create(*, number, slug, emoji, collection, noun, struct, fields, refs, seed, doc, happy_args, duplicate_args, delete_slug):
    """➕️ Inserts one element at a stated `index`. The index is payload data, not a convenience: it
    is what lets `delete`'s inverse restore the element at the position it actually held."""
    verb, record, display = g2_titles(slug)
    module = snake(slug)
    ctor = ", ".join(f"{name}: payload.{name}{'.clone()' if ty in G2_CLONED else ''}" for name, ty in fields)
    diff = f'''    if base.model.{collection}.iter().any(|item| item.id == payload.id) {{
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("{noun} {{}} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }}
    if payload.index as usize > base.model.{collection}.len() {{
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {{}} is past the end of the model's {{}} {collection}.", payload.index, base.model.{collection}.len()), [payload.id.0.to_string()]);
    }}{g2_ref_guard_rs(refs)}
    let mut model = base.model.clone();
    model.{collection}.insert(payload.index as usize, {struct} {{ {ctor} }});
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''
    inverse = f'''    if base.model.{collection}.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.{collection}.len(){g2_ref_refused_rs(refs)} {{
        return Vec::new();
    }}
    vec![super::{snake(delete_slug)}(payload.id)]'''
    row = ", ".join(f'"{name}": payload["{field_camel(name)}"]' for name, _ty in fields)
    python = f'''    """{emoji} `{slug}{{index,…}}` — {doc}"""
    rows = before["model"]["{collection}"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])]){g2_ref_guard_py(refs)}
    after = copy.deepcopy(before)
    after["model"]["{collection}"].insert(payload["index"], {{{row}}})
    return after, applied()'''
    python_invert = f'''    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("{delete_slug}", {{"id": payload["id"]}})]'''
    kind(
        number=number,
        slug=slug,
        emoji=emoji,
        verb=verb,
        entity=noun.lower().replace(" ", "-"),
        record=record,
        display=display,
        doc=doc,
        fields=[("index", "u32"), *fields],
        label=f'format!("{display} {{}} at index {{}}", self.id.0, self.index)',
        target="vec![self.id.0.to_string()]",
        diff=diff,
        inverse=inverse,
        outcome_classes=["applied", "error"],
        probe=f"{module}({happy_args})",
        python=python,
        python_invert=python_invert,
        cases=[
            ("✅️", "applies", f"inserts a new {noun.lower()}", f"{seed}\n    (snapshot(model), super::{module}({happy_args}))"),
            ("⛔️", "refuses", "refuses an id the model already carries", f"{seed}\n    (snapshot(model), super::{module}({duplicate_args}))"),
        ],
    )


def g2_delete(*, number, slug, emoji, collection, noun, fields, seed, doc, target_id, create_slug, in_use=None, bad_seed=None, bad_id=None):
    """➖️ Removes one element by id. Refuses while another entity still references it, so a delete
    never dangles a reference and never cascades silently."""
    verb, record, display = g2_titles(slug)
    module = snake(slug)
    guard_rs = ""
    guard_py = ""
    early_rs = ""
    if in_use:
        condition, message, py_condition = in_use
        guard_rs = f'''
    if {condition} {{
        return protocol::MutationOutcome::error("mutation.invariant", format!("{message}", payload.id.0), [payload.id.0.to_string()]);
    }}'''
        early_rs = f'''
    if {condition} {{
        return Vec::new();
    }}'''
        guard_py = f'''
    if {py_condition}:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])'''
    args = ", ".join(f"existing.{name}{'.clone()' if ty in G2_CLONED else ''}" for name, ty in fields)
    diff = f'''{g2_missing_rs(collection, noun)}
    let _ = existing;{guard_rs}
    let mut model = base.model.clone();
    model.{collection}.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''
    inverse = f'''    let Some(index) = base.model.{collection}.iter().position(|item| item.id == payload.id) else {{
        return Vec::new();
    }};{early_rs}
    let existing = &base.model.{collection}[index];
    vec![super::{snake(create_slug)}(index as u32, {args})]'''
    row = ", ".join(f'"{field_camel(name)}": item["{name}"]' for name, _ty in fields)
    python = f'''    """{emoji} `{slug}{{id}}` — {doc}"""
{g2_missing_py(collection)}{guard_py}
    after = copy.deepcopy(before)
    after["model"]["{collection}"] = [row for row in after["model"]["{collection}"] if row["id"] != payload["id"]]
    return after, applied()'''
    python_invert = f'''    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["{collection}"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("{create_slug}", {{"index": index, {row}}})]'''
    bad_case = (
        ("⛔️", "refuses", "refuses while another entity still references it", f"{bad_seed}\n    (snapshot(model), super::{module}(crate::model::EntityId({bad_id})))")
        if in_use
        else ("⛔️", "refuses", "refuses an absent element", f"{seed}\n    (snapshot(model), super::{module}(crate::model::EntityId(999)))")
    )
    kind(
        number=number,
        slug=slug,
        emoji=emoji,
        verb=verb,
        entity=noun.lower().replace(" ", "-"),
        record=record,
        display=display,
        doc=doc,
        fields=[("id", "crate::model::EntityId")],
        label=f'format!("{display} {{}}", self.id.0)',
        target="vec![self.id.0.to_string()]",
        diff=diff,
        inverse=inverse,
        outcome_classes=["applied", "error"],
        probe=f"{module}(crate::model::EntityId({target_id}))",
        python=python,
        python_invert=python_invert,
        cases=[
            ("✅️", "applies", f"removes {noun.lower()} {target_id}", f"{seed}\n    (snapshot(model), super::{module}(crate::model::EntityId({target_id})))"),
            bad_case,
        ],
    )


# region 🔖️G2Seeds
G2_BASE = '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.zones.push(zone(2, "ZONE TWO"));
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(1), value: 1.0 });
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(2), value: 120.0 });
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(3), value: 0.5 });'''

G2_MATERIALS = G2_BASE + '''
    model.materials.push(crate::model::Material { id: crate::model::EntityId(1), name: "PLASTERBOARD".into(), thickness_m: 0.012, conductivity_w_m_k: 0.16, density_kg_m3: 950.0, specific_heat_j_kg_k: 840.0, thermal_absorptance: 0.9, solar_absorptance: 0.6, visible_absorptance: 0.6 });
    model.materials.push(crate::model::Material { id: crate::model::EntityId(2), name: "FIBERGLASS QUILT".into(), thickness_m: 0.066, conductivity_w_m_k: 0.04, density_kg_m3: 12.0, specific_heat_j_kg_k: 840.0, thermal_absorptance: 0.9, solar_absorptance: 0.6, visible_absorptance: 0.6 });'''

G2_CONSTRUCTIONS = G2_MATERIALS + '''
    model.constructions.push(crate::model::Construction { id: crate::model::EntityId(10), name: "LTWALL".into(), layer_material_ids: vec![crate::model::EntityId(1), crate::model::EntityId(2)] });
    model.constructions.push(crate::model::Construction { id: crate::model::EntityId(11), name: "HWWALL".into(), layer_material_ids: vec![crate::model::EntityId(2)] });'''

G2_CONSTRUCTION_IN_USE = G2_CONSTRUCTIONS + '''
    model.surfaces.push(crate::model::Surface { id: crate::model::EntityId(70), name: "SOUTH WALL".into(), zone_id: crate::model::EntityId(1), class: crate::model::SurfaceClass::ExteriorWall, vertices_m: vec![[0.0, 0.0, 0.0], [8.0, 0.0, 0.0], [8.0, 0.0, 2.7], [0.0, 0.0, 2.7]], construction_id: crate::model::EntityId(10), outside_boundary_condition: crate::model::OutsideBoundary::OutdoorAir, sun_exposed: true, wind_exposed: true, multiplier: 1 });'''

G2_PEOPLE = G2_BASE + '''
    model.people.push(crate::model::PeopleGain { id: crate::model::EntityId(20), zone_id: crate::model::EntityId(1), schedule_id: crate::model::ScheduleId(1), activity_schedule_id: crate::model::ScheduleId(2), people_per_area: 0.05, sensible_fraction: 0.6, latent_fraction: 0.4, radiant_fraction: 0.3 });'''

G2_LIGHTING = G2_BASE + '''
    model.lighting.push(crate::model::LightingGain { id: crate::model::EntityId(30), zone_id: crate::model::EntityId(1), schedule_id: crate::model::ScheduleId(1), watts_per_area: 10.0, radiant_fraction: 0.6, visible_fraction: 0.2, return_air_fraction: 0.0 });'''

G2_EQUIPMENT = G2_BASE + '''
    model.equipment.push(crate::model::EquipmentGain { id: crate::model::EntityId(40), zone_id: crate::model::EntityId(1), schedule_id: crate::model::ScheduleId(1), watts_per_area: 4.1667, radiant_fraction: 0.6, latent_fraction: 0.0 });'''

G2_INFILTRATION = G2_BASE + '''
    model.infiltrations.push(crate::model::Infiltration { id: crate::model::EntityId(50), zone_id: crate::model::EntityId(1), schedule_id: crate::model::ScheduleId(1), method: crate::air_exchange::InfiltrationMethod::ScheduledAch, design_flow_ach: 0.5, flow_per_exterior_area_m3_s_m2: 0.0, effective_leakage_area_m2: 0.0, discharge_coefficient: 1.0, stack_height_m: 2.7, constant_term_coefficient: 1.0, temperature_term_coefficient: 0.0, velocity_term_coefficient: 0.0, velocity_squared_term_coefficient: 0.0 });'''

G2_VENTILATION = G2_BASE + '''
    model.mechanical_ventilations.push(crate::model::MechanicalVentilation { id: crate::model::EntityId(60), zone_id: crate::model::EntityId(1), schedule_id: crate::model::ScheduleId(1), design_flow_m3_s: 0.05, fan_total_efficiency: 0.7, fan_delta_pressure_pa: 300.0 });'''

G2_MATERIAL_FIELDS = [("id", "crate::model::EntityId"), ("name", "String"), ("thickness_m", "f64"), ("conductivity_w_m_k", "f64"), ("density_kg_m3", "f64"), ("specific_heat_j_kg_k", "f64"), ("thermal_absorptance", "f64"), ("solar_absorptance", "f64"), ("visible_absorptance", "f64")]
G2_CONSTRUCTION_FIELDS = [("id", "crate::model::EntityId"), ("name", "String"), ("layer_material_ids", "Vec<crate::model::EntityId>")]
G2_PEOPLE_FIELDS = [("id", "crate::model::EntityId"), ("zone_id", "crate::model::EntityId"), ("schedule_id", "crate::model::ScheduleId"), ("activity_schedule_id", "crate::model::ScheduleId"), ("people_per_area", "f64"), ("sensible_fraction", "f64"), ("latent_fraction", "f64"), ("radiant_fraction", "f64")]
G2_LIGHTING_FIELDS = [("id", "crate::model::EntityId"), ("zone_id", "crate::model::EntityId"), ("schedule_id", "crate::model::ScheduleId"), ("watts_per_area", "f64"), ("radiant_fraction", "f64"), ("visible_fraction", "f64"), ("return_air_fraction", "f64")]
G2_EQUIPMENT_FIELDS = [("id", "crate::model::EntityId"), ("zone_id", "crate::model::EntityId"), ("schedule_id", "crate::model::ScheduleId"), ("watts_per_area", "f64"), ("radiant_fraction", "f64"), ("latent_fraction", "f64")]
G2_INFILTRATION_FIELDS = [("id", "crate::model::EntityId"), ("zone_id", "crate::model::EntityId"), ("schedule_id", "crate::model::ScheduleId"), ("method", "crate::air_exchange::InfiltrationMethod"), ("design_flow_ach", "f64"), ("flow_per_exterior_area_m3_s_m2", "f64"), ("effective_leakage_area_m2", "f64"), ("discharge_coefficient", "f64"), ("stack_height_m", "f64"), ("constant_term_coefficient", "f64"), ("temperature_term_coefficient", "f64"), ("velocity_term_coefficient", "f64"), ("velocity_squared_term_coefficient", "f64")]
G2_VENTILATION_FIELDS = [("id", "crate::model::EntityId"), ("zone_id", "crate::model::EntityId"), ("schedule_id", "crate::model::ScheduleId"), ("design_flow_m3_s", "f64"), ("fan_total_efficiency", "f64"), ("fan_delta_pressure_pa", "f64")]
# endregion 🔖️G2Seeds


# region 🔖️G2Materials
g2_create(
    number=300,
    slug="create-material",
    emoji="🧱️",
    collection="materials",
    noun="Material",
    struct="crate::model::Material",
    fields=G2_MATERIAL_FIELDS,
    refs=[],
    seed=G2_MATERIALS,
    doc="Adds one opaque material layer definition at a stated position in the model's material list. Thickness, conductivity, density and specific heat are the four the conduction transfer functions integrate; the three absorptances close the surface radiation balance.",
    happy_args='2, crate::model::EntityId(3), "CONCRETE SLAB".into(), 0.08, 1.13, 1400.0, 1000.0, 0.9, 0.6, 0.6',
    duplicate_args='2, crate::model::EntityId(1), "CONCRETE SLAB".into(), 0.08, 1.13, 1400.0, 1000.0, 0.9, 0.6, 0.6',
    delete_slug="delete-material",
)

g2_delete(
    number=301,
    slug="delete-material",
    emoji="🪨️",
    collection="materials",
    noun="Material",
    fields=G2_MATERIAL_FIELDS,
    seed=G2_MATERIALS,
    doc="Removes one material definition. Refused while any construction still names it as a layer — the alternative would be to cascade into constructions, which cascade into surfaces.",
    target_id=1,
    create_slug="create-material",
    in_use=("base.model.constructions.iter().any(|construction| construction.layer_material_ids.contains(&payload.id))", "Material {} is still a layer of a construction.", 'any(payload["id"] in row["layer_material_ids"] for row in before["model"]["constructions"])'),
    bad_seed=G2_CONSTRUCTIONS,
    bad_id=1,
)

g2_rename(
    number=302,
    slug="rename-material",
    emoji="🪧️",
    collection="materials",
    noun="Material",
    seed=G2_MATERIALS,
    target_id=1,
    happy='"TIMBER FLOORING".to_string()',
    taken='"FIBERGLASS QUILT".to_string()',
)

g2_scalar(number=303, slug="change-material-thickness", emoji="📏️", collection="materials", noun="Material", field="thickness_m", what="thickness (m)", guard="positive", seed=G2_MATERIALS, target_id=1, happy="0.02", bad="0.0")
g2_scalar(number=304, slug="change-material-conductivity", emoji="🔥️", collection="materials", noun="Material", field="conductivity_w_m_k", what="conductivity (W/m·K)", guard="positive", seed=G2_MATERIALS, target_id=1, happy="0.25", bad="-0.5")
g2_scalar(number=305, slug="change-material-density", emoji="⚖️", collection="materials", noun="Material", field="density_kg_m3", what="density (kg/m³)", guard="positive", seed=G2_MATERIALS, target_id=1, happy="1200.0", bad="0.0")
g2_scalar(number=306, slug="change-material-specific-heat", emoji="♨️", collection="materials", noun="Material", field="specific_heat_j_kg_k", what="specific heat (J/kg·K)", guard="positive", seed=G2_MATERIALS, target_id=1, happy="1000.0", bad="0.0")
g2_scalar(number=307, slug="change-material-thermal-absorptance", emoji="🔆️", collection="materials", noun="Material", field="thermal_absorptance", what="thermal absorptance", guard="fraction", seed=G2_MATERIALS, target_id=1, happy="0.85", bad="1.4")
g2_scalar(number=308, slug="change-material-solar-absorptance", emoji="☀️", collection="materials", noun="Material", field="solar_absorptance", what="solar absorptance", guard="fraction", seed=G2_MATERIALS, target_id=1, happy="0.7", bad="-0.1")
g2_scalar(number=309, slug="change-material-visible-absorptance", emoji="👁️", collection="materials", noun="Material", field="visible_absorptance", what="visible absorptance", guard="fraction", seed=G2_MATERIALS, target_id=1, happy="0.75", bad="2.0")
# endregion 🔖️G2Materials


# region 🔖️G2Constructions
g2_create(
    number=310,
    slug="create-construction",
    emoji="🏗️",
    collection="constructions",
    noun="Construction",
    struct="crate::model::Construction",
    fields=G2_CONSTRUCTION_FIELDS,
    refs=[("material-list", "layer_material_ids", "Material")],
    seed=G2_CONSTRUCTIONS,
    doc="Adds one layered construction, outside-to-inside layer order. Every layer must already name a material the document defines, so a construction is never born dangling.",
    happy_args='2, crate::model::EntityId(12), "ROOF".into(), vec![crate::model::EntityId(2), crate::model::EntityId(1)]',
    duplicate_args='2, crate::model::EntityId(10), "ROOF".into(), vec![crate::model::EntityId(2)]',
    delete_slug="delete-construction",
)

g2_delete(
    number=311,
    slug="delete-construction",
    emoji="🧨️",
    collection="constructions",
    noun="Construction",
    fields=G2_CONSTRUCTION_FIELDS,
    seed=G2_CONSTRUCTIONS,
    doc="Removes one construction. Refused while any surface still names it, because a surface without a construction has no heat path at all.",
    target_id=11,
    create_slug="create-construction",
    in_use=("base.model.surfaces.iter().any(|surface| surface.construction_id == payload.id)", "Construction {} is still assigned to a surface.", 'any(row["construction_id"] == payload["id"] for row in before["model"]["surfaces"])'),
    bad_seed=G2_CONSTRUCTION_IN_USE,
    bad_id=10,
)

g2_rename(
    number=312,
    slug="rename-construction",
    emoji="🪪️",
    collection="constructions",
    noun="Construction",
    seed=G2_CONSTRUCTIONS,
    target_id=10,
    happy='"LTWALL INSULATED".to_string()',
    taken='"HWWALL".to_string()',
)

kind(
    number=313,
    slug="add-construction-layer",
    emoji="➕️",
    verb="add",
    entity="construction-layer",
    record="AddedConstructionLayer",
    display="Add Construction Layer",
    doc="Inserts one material layer into a construction at a stated position, outside-to-inside. Layer order is physically load-bearing — the same layers in a different order are a different wall — so the position is payload data, not an append.",
    fields=[("id", "crate::model::EntityId"), ("index", "u32"), ("material_id", "crate::model::EntityId")],
    label='format!("Add layer {} to construction {} at index {}", self.material_id.0, self.id.0, self.index)',
    target="vec![self.id.0.to_string()]",
    diff='''    let Some(existing) = base.model.constructions.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.materials.iter().any(|material| material.id == payload.material_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material {} does not exist.", payload.material_id.0), [payload.material_id.0.to_string()]);
    }
    if payload.index as usize > existing.layer_material_ids.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Construction {} has {} layers, so index {} is past its end.", payload.id.0, existing.layer_material_ids.len(), payload.index), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    if let Some(construction) = model.constructions.iter_mut().find(|item| item.id == payload.id) {
        construction.layer_material_ids.insert(payload.index as usize, payload.material_id);
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    match base.model.constructions.iter().find(|item| item.id == payload.id) {
        Some(construction) if base.model.materials.iter().any(|material| material.id == payload.material_id) && payload.index as usize <= construction.layer_material_ids.len() => vec![super::remove_construction_layer(payload.id, payload.index)],
        _ => Vec::new(),
    }''',
    outcome_classes=["applied", "error"],
    probe="add_construction_layer(crate::model::EntityId(10), 2, crate::model::EntityId(1))",
    python='''    """➕️ `add-construction-layer{id,index,materialId}` — one layer inserted at a stated position,
    outside-to-inside; layer order is physically load-bearing."""
    construction = next((row for row in before["model"]["constructions"] if row["id"] == payload["id"]), None)
    if construction is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["materialId"] for row in before["model"]["materials"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["materialId"])])
    if payload["index"] > len(construction["layer_material_ids"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    for row in after["model"]["constructions"]:
        if row["id"] == payload["id"]:
            row["layer_material_ids"].insert(payload["index"], payload["materialId"])
    return after, applied()''',
    python_invert='''    """↩️ Undone by removing the layer at exactly the index it was inserted at."""
    return [("remove-construction-layer", {"id": payload["id"], "index": payload["index"]})]''',
    cases=[
        ("✅️", "applies", "inserts a third layer", f'''{G2_CONSTRUCTIONS}
    (snapshot(model), super::add_construction_layer(crate::model::EntityId(10), 2, crate::model::EntityId(1)))'''),
        ("⛔️", "refuses", "refuses an undefined material", f'''{G2_CONSTRUCTIONS}
    (snapshot(model), super::add_construction_layer(crate::model::EntityId(10), 0, crate::model::EntityId(999)))'''),
    ],
)

kind(
    number=314,
    slug="remove-construction-layer",
    emoji="➖️",
    verb="remove",
    entity="construction-layer",
    record="RemovedConstructionLayer",
    display="Remove Construction Layer",
    doc="Removes the material layer a construction holds at a stated position. Addressed by position rather than by material id because the same material may legitimately appear in a construction more than once.",
    fields=[("id", "crate::model::EntityId"), ("index", "u32")],
    label='format!("Remove layer {} from construction {}", self.index, self.id.0)',
    target="vec![self.id.0.to_string()]",
    diff='''    let Some(existing) = base.model.constructions.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.layer_material_ids.get(payload.index as usize).is_none() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Construction {} has no layer at index {}.", payload.id.0, payload.index), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    if let Some(construction) = model.constructions.iter_mut().find(|item| item.id == payload.id) {
        construction.layer_material_ids.remove(payload.index as usize);
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    match base.model.constructions.iter().find(|item| item.id == payload.id) {
        Some(construction) => match construction.layer_material_ids.get(payload.index as usize) {
            Some(material_id) => vec![super::add_construction_layer(payload.id, payload.index, *material_id)],
            None => Vec::new(),
        },
        None => Vec::new(),
    }''',
    outcome_classes=["applied", "error"],
    probe="remove_construction_layer(crate::model::EntityId(10), 1)",
    python='''    """➖️ `remove-construction-layer{id,index}` — addressed by position, because one material may
    legitimately appear in a construction more than once."""
    construction = next((row for row in before["model"]["constructions"] if row["id"] == payload["id"]), None)
    if construction is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["index"] >= len(construction["layer_material_ids"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    for row in after["model"]["constructions"]:
        if row["id"] == payload["id"]:
            del row["layer_material_ids"][payload["index"]]
    return after, applied()''',
    python_invert='''    """↩️ Re-inserts exactly the material that stood at that index, read off BASE."""
    construction = next(row for row in before["model"]["constructions"] if row["id"] == payload["id"])
    return [("add-construction-layer", {"id": payload["id"], "index": payload["index"], "materialId": construction["layer_material_ids"][payload["index"]]})]''',
    cases=[
        ("✅️", "applies", "removes the inner layer", f'''{G2_CONSTRUCTIONS}
    (snapshot(model), super::remove_construction_layer(crate::model::EntityId(10), 1))'''),
        ("⛔️", "refuses", "refuses an index past the last layer", f'''{G2_CONSTRUCTIONS}
    (snapshot(model), super::remove_construction_layer(crate::model::EntityId(10), 7))'''),
    ],
)

kind(
    number=315,
    slug="reorder-construction-layers",
    emoji="🔀️",
    verb="reorder",
    entity="construction-layers",
    record="ReorderedConstructionLayers",
    display="Reorder Construction Layers",
    doc="Restates a construction's whole layer sequence. The new sequence must be a permutation of the one the construction already holds — adding or dropping a layer is `add-construction-layer`'s and `remove-construction-layer`'s job, so a reorder can never change which materials a wall is made of.",
    fields=[("id", "crate::model::EntityId"), ("new_layer_material_ids", "Vec<crate::model::EntityId>")],
    label='format!("Reorder construction {} into {} layers", self.id.0, self.new_layer_material_ids.len())',
    target="vec![self.id.0.to_string()]",
    diff='''    let Some(existing) = base.model.constructions.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let mut wanted: Vec<u32> = payload.new_layer_material_ids.iter().map(|id| id.0).collect();
    let mut held: Vec<u32> = existing.layer_material_ids.iter().map(|id| id.0).collect();
    wanted.sort_unstable();
    held.sort_unstable();
    if wanted != held {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Construction {}'s layers can only be reordered, not exchanged.", payload.id.0), [payload.id.0.to_string()]);
    }
    if existing.layer_material_ids == payload.new_layer_material_ids {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Construction {} already holds its layers in this order.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(construction) = model.constructions.iter_mut().find(|item| item.id == payload.id) {
        construction.layer_material_ids = payload.new_layer_material_ids.clone();
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    match base.model.constructions.iter().find(|item| item.id == payload.id) {
        Some(construction) => {
            let mut wanted: Vec<u32> = payload.new_layer_material_ids.iter().map(|id| id.0).collect();
            let mut held: Vec<u32> = construction.layer_material_ids.iter().map(|id| id.0).collect();
            wanted.sort_unstable();
            held.sort_unstable();
            if wanted != held || construction.layer_material_ids == payload.new_layer_material_ids {
                return Vec::new();
            }
            vec![super::reorder_construction_layers(payload.id, construction.layer_material_ids.clone())]
        }
        None => Vec::new(),
    }''',
    outcome_classes=["applied", "warning", "error"],
    probe="reorder_construction_layers(crate::model::EntityId(10), vec![crate::model::EntityId(2), crate::model::EntityId(1)])",
    python='''    """🔀️ `reorder-construction-layers{id,newLayerMaterialIds}` — a permutation of the layers the
    construction already holds; exchanging a layer is a different kind's job."""
    construction = next((row for row in before["model"]["constructions"] if row["id"] == payload["id"]), None)
    if construction is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    wanted = payload["newLayerMaterialIds"]
    if sorted(wanted) != sorted(construction["layer_material_ids"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if construction["layer_material_ids"] == wanted:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["constructions"]:
        if row["id"] == payload["id"]:
            row["layer_material_ids"] = list(wanted)
    return after, applied()''',
    python_invert='''    """↩️ The order BASE held, restated."""
    construction = next(row for row in before["model"]["constructions"] if row["id"] == payload["id"])
    return [("reorder-construction-layers", {"id": payload["id"], "newLayerMaterialIds": list(construction["layer_material_ids"])})]''',
    cases=[
        ("✅️", "applies", "turns the wall inside out", f'''{G2_CONSTRUCTIONS}
    (snapshot(model), super::reorder_construction_layers(crate::model::EntityId(10), vec![crate::model::EntityId(2), crate::model::EntityId(1)]))'''),
        ("⛔️", "refuses", "refuses a layer exchange disguised as a reorder", f'''{G2_CONSTRUCTIONS}
    (snapshot(model), super::reorder_construction_layers(crate::model::EntityId(10), vec![crate::model::EntityId(2), crate::model::EntityId(2)]))'''),
    ],
)
# endregion 🔖️G2Constructions


# region 🔖️G2PeopleGains
g2_create(
    number=400,
    slug="create-people-gain",
    emoji="👤️",
    collection="people",
    noun="People Gain",
    struct="crate::model::PeopleGain",
    fields=G2_PEOPLE_FIELDS,
    refs=[("zone", "zone_id", "Zone"), ("schedule", "schedule_id", "Schedule"), ("schedule", "activity_schedule_id", "Schedule")],
    seed=G2_PEOPLE,
    doc="Adds one occupancy gain to a zone: the occupant density, the occupancy schedule, the metabolic activity schedule and the sensible/latent/radiant split the zone heat balance needs.",
    happy_args="1, crate::model::EntityId(21), crate::model::EntityId(2), crate::model::ScheduleId(1), crate::model::ScheduleId(2), 0.1, 0.6, 0.4, 0.3",
    duplicate_args="1, crate::model::EntityId(20), crate::model::EntityId(2), crate::model::ScheduleId(1), crate::model::ScheduleId(2), 0.1, 0.6, 0.4, 0.3",
    delete_slug="delete-people-gain",
)

g2_delete(number=401, slug="delete-people-gain", emoji="🚷️", collection="people", noun="People Gain", fields=G2_PEOPLE_FIELDS, seed=G2_PEOPLE, doc="Removes one occupancy gain. Nothing references a gain, so this never has to refuse for use.", target_id=20, create_slug="create-people-gain")
g2_reference(number=402, slug="change-people-gain-zone", emoji="🚶️", collection="people", noun="People Gain", field="zone_id", ref="zone", ref_noun="Zone", seed=G2_PEOPLE, target_id=20, happy="crate::model::EntityId(2)", bad="crate::model::EntityId(999)")
g2_reference(number=403, slug="change-people-gain-schedule", emoji="⏰️", collection="people", noun="People Gain", field="schedule_id", ref="schedule", ref_noun="Schedule", seed=G2_PEOPLE, target_id=20, happy="crate::model::ScheduleId(3)", bad="crate::model::ScheduleId(999)")
g2_reference(number=404, slug="change-people-gain-activity-schedule", emoji="🏃️", collection="people", noun="People Gain", field="activity_schedule_id", ref="schedule", ref_noun="Schedule", seed=G2_PEOPLE, target_id=20, happy="crate::model::ScheduleId(3)", bad="crate::model::ScheduleId(999)")
g2_scalar(number=405, slug="change-people-gain-people-per-area", emoji="👥️", collection="people", noun="People Gain", field="people_per_area", what="occupant density (people/m²)", guard="nonnegative", seed=G2_PEOPLE, target_id=20, happy="0.1", bad="-0.2")
g2_scalar(number=406, slug="change-people-gain-sensible-fraction", emoji="🌞️", collection="people", noun="People Gain", field="sensible_fraction", what="sensible fraction", guard="fraction", seed=G2_PEOPLE, target_id=20, happy="0.7", bad="1.5")
g2_scalar(number=407, slug="change-people-gain-latent-fraction", emoji="💧️", collection="people", noun="People Gain", field="latent_fraction", what="latent fraction", guard="fraction", seed=G2_PEOPLE, target_id=20, happy="0.3", bad="-0.2")
g2_scalar(number=408, slug="change-people-gain-radiant-fraction", emoji="📡️", collection="people", noun="People Gain", field="radiant_fraction", what="radiant fraction", guard="fraction", seed=G2_PEOPLE, target_id=20, happy="0.5", bad="1.2")
# endregion 🔖️G2PeopleGains


# region 🔖️G2LightingGains
g2_create(
    number=409,
    slug="create-lighting-gain",
    emoji="💡️",
    collection="lighting",
    noun="Lighting Gain",
    struct="crate::model::LightingGain",
    fields=G2_LIGHTING_FIELDS,
    refs=[("zone", "zone_id", "Zone"), ("schedule", "schedule_id", "Schedule")],
    seed=G2_LIGHTING,
    doc="Adds one lighting gain to a zone: the installed power density, its schedule and the radiant/visible/return-air split that decides how much of it reaches the zone air directly.",
    happy_args="1, crate::model::EntityId(31), crate::model::EntityId(2), crate::model::ScheduleId(1), 8.0, 0.6, 0.2, 0.0",
    duplicate_args="1, crate::model::EntityId(30), crate::model::EntityId(2), crate::model::ScheduleId(1), 8.0, 0.6, 0.2, 0.0",
    delete_slug="delete-lighting-gain",
)

g2_delete(number=410, slug="delete-lighting-gain", emoji="🕯️", collection="lighting", noun="Lighting Gain", fields=G2_LIGHTING_FIELDS, seed=G2_LIGHTING, doc="Removes one lighting gain.", target_id=30, create_slug="create-lighting-gain")
g2_reference(number=411, slug="change-lighting-gain-zone", emoji="🔦️", collection="lighting", noun="Lighting Gain", field="zone_id", ref="zone", ref_noun="Zone", seed=G2_LIGHTING, target_id=30, happy="crate::model::EntityId(2)", bad="crate::model::EntityId(999)")
g2_reference(number=412, slug="change-lighting-gain-schedule", emoji="⏱️", collection="lighting", noun="Lighting Gain", field="schedule_id", ref="schedule", ref_noun="Schedule", seed=G2_LIGHTING, target_id=30, happy="crate::model::ScheduleId(3)", bad="crate::model::ScheduleId(999)")
g2_scalar(number=413, slug="change-lighting-gain-watts-per-area", emoji="🔌️", collection="lighting", noun="Lighting Gain", field="watts_per_area", what="installed power density (W/m²)", guard="nonnegative", seed=G2_LIGHTING, target_id=30, happy="12.5", bad="-1.0")
g2_scalar(number=414, slug="change-lighting-gain-radiant-fraction", emoji="🌟️", collection="lighting", noun="Lighting Gain", field="radiant_fraction", what="radiant fraction", guard="fraction", seed=G2_LIGHTING, target_id=30, happy="0.7", bad="1.3")
g2_scalar(number=415, slug="change-lighting-gain-visible-fraction", emoji="🔅️", collection="lighting", noun="Lighting Gain", field="visible_fraction", what="visible fraction", guard="fraction", seed=G2_LIGHTING, target_id=30, happy="0.25", bad="-0.4")
g2_scalar(number=416, slug="change-lighting-gain-return-air-fraction", emoji="🎐️", collection="lighting", noun="Lighting Gain", field="return_air_fraction", what="return-air fraction", guard="fraction", seed=G2_LIGHTING, target_id=30, happy="0.1", bad="1.8")
# endregion 🔖️G2LightingGains


# region 🔖️G2EquipmentGains
g2_create(
    number=417,
    slug="create-equipment-gain",
    emoji="🖥️",
    collection="equipment",
    noun="Equipment Gain",
    struct="crate::model::EquipmentGain",
    fields=G2_EQUIPMENT_FIELDS,
    refs=[("zone", "zone_id", "Zone"), ("schedule", "schedule_id", "Schedule")],
    seed=G2_EQUIPMENT,
    doc="Adds one electric equipment gain to a zone — ANSI/ASHRAE 140 §5.2's 200 W internal load is exactly this entity at 60 % radiative.",
    happy_args="1, crate::model::EntityId(41), crate::model::EntityId(2), crate::model::ScheduleId(1), 2.5, 0.5, 0.0",
    duplicate_args="1, crate::model::EntityId(40), crate::model::EntityId(2), crate::model::ScheduleId(1), 2.5, 0.5, 0.0",
    delete_slug="delete-equipment-gain",
)

g2_delete(number=418, slug="delete-equipment-gain", emoji="🧯️", collection="equipment", noun="Equipment Gain", fields=G2_EQUIPMENT_FIELDS, seed=G2_EQUIPMENT, doc="Removes one electric equipment gain.", target_id=40, create_slug="create-equipment-gain")
g2_reference(number=419, slug="change-equipment-gain-zone", emoji="🖨️", collection="equipment", noun="Equipment Gain", field="zone_id", ref="zone", ref_noun="Zone", seed=G2_EQUIPMENT, target_id=40, happy="crate::model::EntityId(2)", bad="crate::model::EntityId(999)")
g2_reference(number=420, slug="change-equipment-gain-schedule", emoji="⌛️", collection="equipment", noun="Equipment Gain", field="schedule_id", ref="schedule", ref_noun="Schedule", seed=G2_EQUIPMENT, target_id=40, happy="crate::model::ScheduleId(3)", bad="crate::model::ScheduleId(999)")
g2_scalar(number=421, slug="change-equipment-gain-watts-per-area", emoji="⚡️", collection="equipment", noun="Equipment Gain", field="watts_per_area", what="equipment power density (W/m²)", guard="nonnegative", seed=G2_EQUIPMENT, target_id=40, happy="5.0", bad="-3.0")
g2_scalar(number=422, slug="change-equipment-gain-radiant-fraction", emoji="🌠️", collection="equipment", noun="Equipment Gain", field="radiant_fraction", what="radiant fraction", guard="fraction", seed=G2_EQUIPMENT, target_id=40, happy="0.4", bad="1.1")
g2_scalar(number=423, slug="change-equipment-gain-latent-fraction", emoji="💦️", collection="equipment", noun="Equipment Gain", field="latent_fraction", what="latent fraction", guard="fraction", seed=G2_EQUIPMENT, target_id=40, happy="0.2", bad="-0.5")
# endregion 🔖️G2EquipmentGains


# region 🔖️G2Infiltration
g2_create(
    number=424,
    slug="create-infiltration",
    emoji="💨️",
    collection="infiltrations",
    noun="Infiltration",
    struct="crate::model::Infiltration",
    fields=G2_INFILTRATION_FIELDS,
    refs=[("zone", "zone_id", "Zone"), ("schedule", "schedule_id", "Schedule")],
    seed=G2_INFILTRATION,
    doc="Adds one zone infiltration specification: the method plus every parameter each method needs, so the kernel maps the entity onto one `air_exchange::InfiltrationSpec` without pinning a method or smuggling the flow through a coefficient.",
    happy_args="1, crate::model::EntityId(51), crate::model::EntityId(2), crate::model::ScheduleId(1), crate::air_exchange::InfiltrationMethod::ScheduledAch, 0.5, 0.0, 0.0, 1.0, 2.7, 1.0, 0.0, 0.0, 0.0",
    duplicate_args="1, crate::model::EntityId(50), crate::model::EntityId(2), crate::model::ScheduleId(1), crate::air_exchange::InfiltrationMethod::ScheduledAch, 0.5, 0.0, 0.0, 1.0, 2.7, 1.0, 0.0, 0.0, 0.0",
    delete_slug="delete-infiltration",
)

g2_delete(number=425, slug="delete-infiltration", emoji="🧽️", collection="infiltrations", noun="Infiltration", fields=G2_INFILTRATION_FIELDS, seed=G2_INFILTRATION, doc="Removes one zone infiltration specification, leaving the zone airtight.", target_id=50, create_slug="create-infiltration")
g2_reference(number=426, slug="change-infiltration-zone", emoji="🌀️", collection="infiltrations", noun="Infiltration", field="zone_id", ref="zone", ref_noun="Zone", seed=G2_INFILTRATION, target_id=50, happy="crate::model::EntityId(2)", bad="crate::model::EntityId(999)")
g2_reference(number=427, slug="change-infiltration-schedule", emoji="⏳️", collection="infiltrations", noun="Infiltration", field="schedule_id", ref="schedule", ref_noun="Schedule", seed=G2_INFILTRATION, target_id=50, happy="crate::model::ScheduleId(3)", bad="crate::model::ScheduleId(999)")
g2_scalar(number=428, slug="change-infiltration-flow-per-exterior-area", emoji="🌫️", collection="infiltrations", noun="Infiltration", field="flow_per_exterior_area_m3_s_m2", what="flow per exterior area (m³/s·m²)", guard="nonnegative", seed=G2_INFILTRATION, target_id=50, happy="0.0003", bad="-0.001")
g2_scalar(number=429, slug="change-infiltration-constant-term-coefficient", emoji="🅰️", collection="infiltrations", noun="Infiltration", field="constant_term_coefficient", what="the constant term coefficient A", guard="nonnegative", seed=G2_INFILTRATION, target_id=50, happy="0.606", bad="-1.0", doc="Sets EnergyPlus's `ZoneInfiltration:DesignFlowRate` constant term coefficient A. The four coefficients are non-negative by that object's own convention (BLAST 0.606/0.03636/0.1177/0, DOE-2 0/0/0.224/0).")
g2_scalar(number=430, slug="change-infiltration-temperature-term-coefficient", emoji="🅱️", collection="infiltrations", noun="Infiltration", field="temperature_term_coefficient", what="the temperature term coefficient B", guard="nonnegative", seed=G2_INFILTRATION, target_id=50, happy="0.03636", bad="-0.5")
g2_scalar(number=431, slug="change-infiltration-velocity-term-coefficient", emoji="🆎️", collection="infiltrations", noun="Infiltration", field="velocity_term_coefficient", what="the wind velocity term coefficient C", guard="nonnegative", seed=G2_INFILTRATION, target_id=50, happy="0.1177", bad="-0.2")
g2_scalar(number=432, slug="change-infiltration-velocity-squared-term-coefficient", emoji="🆑️", collection="infiltrations", noun="Infiltration", field="velocity_squared_term_coefficient", what="the squared wind velocity term coefficient D", guard="nonnegative", seed=G2_INFILTRATION, target_id=50, happy="0.000103", bad="-0.1")

g2_element_kind(
    number=440,
    slug="change-infiltration-method",
    emoji="🔬️",
    collection="infiltrations",
    noun="Infiltration",
    field="method",
    rust_type="crate::air_exchange::InfiltrationMethod",
    what="infiltration method",
    doc="Selects which of `air_exchange::InfiltrationMethod`'s four flow calculations the kernel runs for one infiltration object. The parameters every method needs are already carried side by side, so switching the method never has to move data.",
    guard_rs="",
    guard_py="",
    refuse_rs="",
    show="{:?}",
    seed=G2_INFILTRATION,
    target_id=50,
    happy="crate::air_exchange::InfiltrationMethod::PerExteriorArea",
    bad="crate::model::EntityId(999), crate::air_exchange::InfiltrationMethod::PerExteriorArea",
    bad_seed=G2_INFILTRATION,
    bad_description="refuses an absent infiltration object",
    outcome_classes=["applied", "warning", "error"],
)

g2_scalar(number=441, slug="change-infiltration-design-flow-ach", emoji="🔄️", collection="infiltrations", noun="Infiltration", field="design_flow_ach", what="design flow (air changes per hour)", guard="nonnegative", seed=G2_INFILTRATION, target_id=50, happy="1.0", bad="-0.5")
g2_scalar(number=442, slug="change-infiltration-effective-leakage-area", emoji="🕳️", collection="infiltrations", noun="Infiltration", field="effective_leakage_area_m2", what="effective leakage area (m²)", guard="nonnegative", seed=G2_INFILTRATION, target_id=50, happy="0.05", bad="-0.01")
g2_scalar(number=443, slug="change-infiltration-discharge-coefficient", emoji="🚰️", collection="infiltrations", noun="Infiltration", field="discharge_coefficient", what="the orifice discharge coefficient", guard="positive", seed=G2_INFILTRATION, target_id=50, happy="0.65", bad="0.0")
g2_scalar(number=444, slug="change-infiltration-stack-height", emoji="🏭️", collection="infiltrations", noun="Infiltration", field="stack_height_m", what="stack height (m)", guard="nonnegative", seed=G2_INFILTRATION, target_id=50, happy="3.0", bad="-1.0")
# endregion 🔖️G2Infiltration


# region 🔖️G2Ventilation
g2_create(
    number=433,
    slug="create-mechanical-ventilation",
    emoji="🌪️",
    collection="mechanical_ventilations",
    noun="Mechanical Ventilation",
    struct="crate::model::MechanicalVentilation",
    fields=G2_VENTILATION_FIELDS,
    refs=[("zone", "zone_id", "Zone"), ("schedule", "schedule_id", "Schedule")],
    seed=G2_VENTILATION,
    doc="Adds one mechanical ventilation specification to a zone: the design supply flow, its schedule and the fan work the supply air arrives with.",
    happy_args="1, crate::model::EntityId(61), crate::model::EntityId(2), crate::model::ScheduleId(1), 0.08, 0.65, 250.0",
    duplicate_args="1, crate::model::EntityId(60), crate::model::EntityId(2), crate::model::ScheduleId(1), 0.08, 0.65, 250.0",
    delete_slug="delete-mechanical-ventilation",
)

g2_delete(number=434, slug="delete-mechanical-ventilation", emoji="🚫️", collection="mechanical_ventilations", noun="Mechanical Ventilation", fields=G2_VENTILATION_FIELDS, seed=G2_VENTILATION, doc="Removes one mechanical ventilation specification.", target_id=60, create_slug="create-mechanical-ventilation")
g2_reference(number=435, slug="change-mechanical-ventilation-zone", emoji="🧭️", collection="mechanical_ventilations", noun="Mechanical Ventilation", field="zone_id", ref="zone", ref_noun="Zone", seed=G2_VENTILATION, target_id=60, happy="crate::model::EntityId(2)", bad="crate::model::EntityId(999)")
g2_reference(number=436, slug="change-mechanical-ventilation-schedule", emoji="📆️", collection="mechanical_ventilations", noun="Mechanical Ventilation", field="schedule_id", ref="schedule", ref_noun="Schedule", seed=G2_VENTILATION, target_id=60, happy="crate::model::ScheduleId(3)", bad="crate::model::ScheduleId(999)")
g2_scalar(number=437, slug="change-mechanical-ventilation-design-flow", emoji="🚿️", collection="mechanical_ventilations", noun="Mechanical Ventilation", field="design_flow_m3_s", what="design supply flow (m³/s)", guard="nonnegative", seed=G2_VENTILATION, target_id=60, happy="0.12", bad="-0.05")
g2_scalar(number=438, slug="change-mechanical-ventilation-fan-total-efficiency", emoji="💠️", collection="mechanical_ventilations", noun="Mechanical Ventilation", field="fan_total_efficiency", what="fan total efficiency", guard="unit-positive", seed=G2_VENTILATION, target_id=60, happy="0.8", bad="0.0")
g2_scalar(number=439, slug="change-mechanical-ventilation-fan-delta-pressure", emoji="🎈️", collection="mechanical_ventilations", noun="Mechanical Ventilation", field="fan_delta_pressure_pa", what="fan pressure rise (Pa)", guard="nonnegative", seed=G2_VENTILATION, target_id=60, happy="500.0", bad="-100.0")
# endregion 🔖️G2Ventilation
# endregion 🔖️G2



# region 🔖️G4
#: 🔢️ Range guards this group needs that the shared numeric table does not carry: a tilt is a
#: zenith angle, an azimuth a compass bearing, a service-hot-water setpoint a liquid-water
#: temperature, and a fixture/case count an integer that must not be zero. Each is expressed twice —
#: once as the Rust refusal test, once as the Python one — because the second implementation is
#: written against the payload schema, not against the Rust.
G4_RS_TEST = {
    "finite": "!{v}.is_finite()",
    "tilt": "!{v}.is_finite() || !(0.0..=90.0).contains(&{v})",
    "azimuth": "!{v}.is_finite() || !(0.0..=360.0).contains(&{v})",
    "hot-water": "!{v}.is_finite() || !(0.0..=100.0).contains(&{v})",
}

G4_PY_TEST = {
    "finite": '{v} != {v} or {v} in (float("inf"), float("-inf"))',
    "tilt": "{v} != {v} or not 0.0 <= {v} <= 90.0",
    "azimuth": "{v} != {v} or not 0.0 <= {v} <= 360.0",
    "hot-water": "{v} != {v} or not 0.0 <= {v} <= 100.0",
}

G4_WORDS = {
    "finite": "a finite value",
    "tilt": "a zenith angle in [0, 90] degrees",
    "azimuth": "a compass bearing in [0, 360] degrees",
    "hot-water": "a liquid-water temperature in [0, 100] °C",
}


def g4_exists_rs(collection: str, value: str) -> str:
    """🔗️ Whether the document defines the referenced entity. `"schedules"` means any of the five
    schedule groups, because a `ScheduleId` is one namespace spanning all of them."""
    if collection == "schedules":
        return f"({G2_SCHEDULE_EXISTS.format(v=value)})"
    return f"base.model.{collection}.iter().any(|row| row.id == {value})"


def g4_exists_py(collection: str, value: str) -> str:
    if collection == "schedules":
        return G2_PY_SCHEDULE_EXISTS.format(v=value)
    return f'any(row["id"] == {value} for row in before["model"]["{collection}"])'


def g4_ref_guard_rs(refs) -> str:
    """🔗️ Referential-integrity refusals for a create payload. `refs` entries are
    `(shape, field, collection, noun)` with shape `"one"` (a single id) or `"many"` (a list of ids);
    this generalizes the shared zone/schedule/material-only guard to every collection in the
    document, which is what the electrical, grouping and schedule groups address."""
    blocks = []
    for shape, field, collection, noun in refs:
        if shape == "many":
            blocks.append(f'''    if let Some(missing) = payload.{field}.iter().find(|candidate| !{g4_exists_rs(collection, "**candidate")}) {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{noun} {{}} does not exist.", missing.0), [missing.0.to_string()]);
    }}''')
        else:
            blocks.append(f'''    if !{g4_exists_rs(collection, f"payload.{field}")} {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{noun} {{}} does not exist.", payload.{field}.0), [payload.{field}.0.to_string()]);
    }}''')
    return ("\n" + "\n".join(blocks)) if blocks else ""


def g4_ref_refused_rs(refs) -> str:
    parts = []
    for shape, field, collection, _noun in refs:
        if shape == "many":
            parts.append(f"payload.{field}.iter().any(|candidate| !{g4_exists_rs(collection, '*candidate')})")
        else:
            parts.append(f"!{g4_exists_rs(collection, f'payload.{field}')}")
    return "".join(f" || {part}" for part in parts)


def g4_ref_guard_py(refs) -> str:
    blocks = []
    for shape, field, collection, _noun in refs:
        key = field_camel(field)
        if shape == "many":
            blocks.append(f'''    missing = next((candidate for candidate in payload["{key}"] if not {g4_exists_py(collection, "candidate")}), None)
    if missing is not None:
        return unchanged(before), rejected("mutation.target-missing", [str(missing)])''')
        else:
            blocks.append(f'''    if not {g4_exists_py(collection, f'payload["{key}"]')}:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["{key}"])])''')
    return ("\n" + "\n".join(blocks)) if blocks else ""


def g4_create(*, number, slug, emoji, collection, noun, struct, fields, refs, seed, doc, happy_args, duplicate_args, delete_slug):
    """➕️ Inserts one element at a stated `index`, behind a general referential-integrity guard. The
    index is payload data, not a convenience: it is what lets `delete`'s inverse put the element back
    at the position it actually held, which the committed after-snapshot compares byte for byte."""
    verb, record, display = g2_titles(slug)
    module = snake(slug)
    ctor = ", ".join(f"{name}: payload.{name}{'.clone()' if ty in G2_CLONED else ''}" for name, ty in fields)
    diff = f'''    if base.model.{collection}.iter().any(|item| item.id == payload.id) {{
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("{noun} {{}} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }}
    if payload.index as usize > base.model.{collection}.len() {{
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {{}} is past the end of the model's {{}} {collection}.", payload.index, base.model.{collection}.len()), [payload.id.0.to_string()]);
    }}{g4_ref_guard_rs(refs)}
    let mut model = base.model.clone();
    model.{collection}.insert(payload.index as usize, {struct} {{ {ctor} }});
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''
    inverse = f'''    if base.model.{collection}.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.{collection}.len(){g4_ref_refused_rs(refs)} {{
        return Vec::new();
    }}
    vec![super::{snake(delete_slug)}(payload.id)]'''
    row = ", ".join(f'"{name}": payload["{field_camel(name)}"]' for name, _ty in fields)
    python = f'''    """{emoji} `{slug}{{index,…}}` — {doc}"""
    rows = before["model"]["{collection}"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])]){g4_ref_guard_py(refs)}
    after = copy.deepcopy(before)
    after["model"]["{collection}"].insert(payload["index"], {{{row}}})
    return after, applied()'''
    python_invert = '''    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("%s", {"id": payload["id"]})]''' % delete_slug
    kind(
        number=number,
        slug=slug,
        emoji=emoji,
        verb=verb,
        entity=noun.lower().replace(" ", "-"),
        record=record,
        display=display,
        doc=doc,
        fields=[("index", "u32"), *fields],
        label=f'format!("{display} {{}} at index {{}}", self.id.0, self.index)',
        target="vec![self.id.0.to_string()]",
        diff=diff,
        inverse=inverse,
        outcome_classes=["applied", "error"],
        probe=f"{module}({happy_args})",
        python=python,
        python_invert=python_invert,
        cases=[
            ("✅️", "applies", f"inserts a new {noun.lower()}", f"{seed}\n    (snapshot(model), super::{module}({happy_args}))"),
            ("⛔️", "refuses", "refuses an id the model already carries", f"{seed}\n    (snapshot(model), super::{module}({duplicate_args}))"),
        ],
    )


def g4_scalar(*, number, slug, emoji, collection, noun, field, what, guard, seed, target_id, happy, bad, doc=None):
    """📐️ One numeric field behind a G4 range guard (tilt, azimuth, water temperature, finiteness)."""
    v = f"payload.new_{field}"
    test = G4_RS_TEST[guard]
    guard_rs = f'''
    if {test.format(v=v)} {{
        return protocol::MutationOutcome::error("mutation.invariant", format!("{noun} {{}}: {what} must be {G4_WORDS[guard]}, got {{}}.", payload.id.0, {v}), [payload.id.0.to_string()]);
    }}'''
    guard_py = f'''
    if {G4_PY_TEST[guard].format(v="value")}:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])'''
    g2_element_kind(
        number=number,
        slug=slug,
        emoji=emoji,
        collection=collection,
        noun=noun,
        field=field,
        rust_type="f64",
        doc=doc or f"Sets {what} on one {noun.lower()}, addressed by id.",
        guard_rs=guard_rs,
        guard_py=guard_py,
        refuse_rs=f" && !({test.format(v=v)})",
        show="{}",
        seed=seed,
        target_id=target_id,
        happy=happy,
        bad=f"crate::model::EntityId({target_id}), {bad}",
        bad_seed=seed,
        bad_description=f"refuses {what} outside {G4_WORDS[guard]}",
        outcome_classes=["applied", "warning", "error"],
    )


def g4_count(*, number, slug, emoji, collection, noun, field, what, seed, target_id, happy, doc=None):
    """🔢️ One `u32` population count that must stay at one or more — a zero-count system is a system
    the document should not be carrying at all, so it is refused rather than silently inert."""
    v = f"payload.new_{field}"
    guard_rs = f'''
    if {v} == 0 {{
        return protocol::MutationOutcome::error("mutation.invariant", format!("{noun} {{}} needs at least one {what}.", payload.id.0), [payload.id.0.to_string()]);
    }}'''
    guard_py = '''
    if value == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])'''
    g2_element_kind(
        number=number,
        slug=slug,
        emoji=emoji,
        collection=collection,
        noun=noun,
        field=field,
        rust_type="u32",
        doc=doc or f"Sets how many {what}s one {noun.lower()} carries.",
        guard_rs=guard_rs,
        guard_py=guard_py,
        refuse_rs=f" && {v} != 0",
        show="{}",
        seed=seed,
        target_id=target_id,
        happy=happy,
        bad=f"crate::model::EntityId({target_id}), 0",
        bad_seed=seed,
        bad_description=f"refuses a zero {what} count",
        outcome_classes=["applied", "warning", "error"],
    )


def g4_entity_reference(*, number, slug, emoji, collection, noun, field, target_collection, ref_noun, seed, target_id, happy, bad, doc=None):
    """🔗️ One `EntityId` reference field pointing into an arbitrary collection — the shared reference
    helper only speaks zones and schedules, and this group re-points a fault at an ideal-loads system."""
    v = f"payload.new_{field}"
    exists_rs = g4_exists_rs(target_collection, v)
    guard_rs = f'''
    if !{exists_rs} {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{ref_noun} {{}} does not exist.", {v}.0), [{v}.0.to_string()]);
    }}'''
    guard_py = f'''
    if not {g4_exists_py(target_collection, "value")}:
        return unchanged(before), rejected("mutation.target-missing", [str(value)])'''
    g2_element_kind(
        number=number,
        slug=slug,
        emoji=emoji,
        collection=collection,
        noun=noun,
        field=field,
        rust_type="crate::model::EntityId",
        doc=doc or f"Re-points one {noun.lower()} at another {ref_noun.lower()}; a target the document does not define is refused.",
        guard_rs=guard_rs,
        guard_py=guard_py,
        refuse_rs=f" && {exists_rs}",
        show="{}",
        seed=seed,
        target_id=target_id,
        happy=happy,
        bad=f"crate::model::EntityId({target_id}), {bad}",
        bad_seed=seed,
        bad_description=f"refuses an undefined {ref_noun.lower()}",
        outcome_classes=["applied", "warning", "error"],
    )


def g4_enum(*, number, slug, emoji, collection, noun, field, rust_type, what, seed, target_id, happy, bad_seed, bad, doc=None):
    """🔤️ One closed-vocabulary field. There is no range to guard — every value the payload schema
    admits is admissible — so the only refusal left is an absent target, and the second vector is a
    no-op-free `mutation.target-missing`."""
    g2_element_kind(
        number=number,
        slug=slug,
        emoji=emoji,
        collection=collection,
        noun=noun,
        field=field,
        rust_type=rust_type,
        doc=doc or f"Sets one {noun.lower()}'s {what}.",
        guard_rs="",
        guard_py="",
        refuse_rs="",
        show="{:?}",
        seed=seed,
        target_id=target_id,
        happy=happy,
        bad=bad,
        bad_seed=bad_seed,
        bad_description="refuses an absent target",
        outcome_classes=["applied", "warning", "error"],
    )


def g4_membership(*, number, slug, emoji, mode, collection, noun, field, member_field, member_collection, member_noun, partner_slug, seed, doc, owner_id, happy_extra, bad_args, bad_description, bad_seed=None):
    """🧩️ One set-like `Vec<EntityId>` membership edge. The list is a set semantically (the
    vocabulary gives it no `reorder`), but a JSON array positionally, so `add` carries the FINAL
    index and `remove`'s inverse reads the BASE position off the document — otherwise an undo would
    put the member back at the end and the committed before-snapshot would not compare equal."""
    verb, record, display = g2_titles(slug)
    module = snake(slug)
    partner_module = snake(partner_slug)
    member_key = field_camel(member_field)
    missing_rs = g2_missing_rs(collection, noun)
    missing_py = g2_missing_py(collection)
    member_exists_rs = f"base.model.{member_collection}.iter().any(|row| row.id == payload.{member_field})"
    member_exists_py = f'any(row["id"] == payload["{member_key}"] for row in before["model"]["{member_collection}"])'
    if mode == "add":
        diff = f'''{missing_rs}
    if !{member_exists_rs} {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{member_noun} {{}} does not exist.", payload.{member_field}.0), [payload.{member_field}.0.to_string()]);
    }}
    if payload.index as usize > existing.{field}.len() {{
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {{}} is past the end of {noun} {{}}'s {{}} members.", payload.index, payload.id.0, existing.{field}.len()), [payload.id.0.to_string()]);
    }}
    if existing.{field}.contains(&payload.{member_field}) {{
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("{member_noun} {{}} already belongs to {noun} {{}}.", payload.{member_field}.0, payload.id.0));
    }}
    let mut model = base.model.clone();
    if let Some(item) = model.{collection}.iter_mut().find(|item| item.id == payload.id) {{
        item.{field}.insert(payload.index as usize, payload.{member_field});
    }}
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''
        inverse = f'''    match base.model.{collection}.iter().find(|item| item.id == payload.id) {{
        Some(item) if !item.{field}.contains(&payload.{member_field}) && payload.index as usize <= item.{field}.len() && {member_exists_rs} => vec![super::{partner_module}(payload.id, payload.{member_field})],
        _ => Vec::new(),
    }}'''
        python = f'''    """{emoji} `{slug}{{id,index,{member_key}}}` — {doc}"""
{missing_py}
    if not {member_exists_py}:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["{member_key}"])])
    if payload["index"] > len(item["{field}"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["{member_key}"] in item["{field}"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["{collection}"]:
        if row["id"] == payload["id"]:
            row["{field}"].insert(payload["index"], payload["{member_key}"])
    return after, applied()'''
        python_invert = f'''    """↩️ The added member is taken back out; the owner keeps every other member in place."""
    return [("{partner_slug}", {{"id": payload["id"], "{member_key}": payload["{member_key}"]}})]'''
        fields = [("id", "crate::model::EntityId"), ("index", "u32"), (member_field, "crate::model::EntityId")]
        label = f'format!("Add {member_noun.lower()} {{}} to {noun.lower()} {{}}", self.{member_field}.0, self.id.0)'
        happy_args = f"crate::model::EntityId({owner_id}), {happy_extra}"
        outcome_classes = ["applied", "warning", "error"]
    else:
        diff = f'''{missing_rs}
    if !existing.{field}.contains(&payload.{member_field}) {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{member_noun} {{}} is not a member of {noun} {{}}.", payload.{member_field}.0, payload.id.0), [payload.{member_field}.0.to_string()]);
    }}
    let mut model = base.model.clone();
    if let Some(item) = model.{collection}.iter_mut().find(|item| item.id == payload.id) {{
        item.{field}.retain(|candidate| *candidate != payload.{member_field});
    }}
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''
        inverse = f'''    let Some(item) = base.model.{collection}.iter().find(|item| item.id == payload.id) else {{
        return Vec::new();
    }};
    let Some(position) = item.{field}.iter().position(|candidate| *candidate == payload.{member_field}) else {{
        return Vec::new();
    }};
    vec![super::{partner_module}(payload.id, position as u32, payload.{member_field})]'''
        python = f'''    """{emoji} `{slug}{{id,{member_key}}}` — {doc}"""
{missing_py}
    if payload["{member_key}"] not in item["{field}"]:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["{member_key}"])])
    after = copy.deepcopy(before)
    for row in after["model"]["{collection}"]:
        if row["id"] == payload["id"]:
            row["{field}"] = [candidate for candidate in row["{field}"] if candidate != payload["{member_key}"]]
    return after, applied()'''
        python_invert = f'''    """↩️ The member goes back at the position it actually held in BASE, not at the end."""
    item = next(row for row in before["model"]["{collection}"] if row["id"] == payload["id"])
    return [("{partner_slug}", {{"id": payload["id"], "index": item["{field}"].index(payload["{member_key}"]), "{member_key}": payload["{member_key}"]}})]'''
        fields = [("id", "crate::model::EntityId"), (member_field, "crate::model::EntityId")]
        label = f'format!("Remove {member_noun.lower()} {{}} from {noun.lower()} {{}}", self.{member_field}.0, self.id.0)'
        happy_args = f"crate::model::EntityId({owner_id}), {happy_extra}"
        outcome_classes = ["applied", "error"]
    kind(
        number=number,
        slug=slug,
        emoji=emoji,
        verb=verb,
        entity=noun.lower().replace(" ", "-"),
        record=record,
        display=display,
        doc=doc,
        fields=fields,
        label=label,
        target="vec![self.id.0.to_string()]",
        diff=diff,
        inverse=inverse,
        outcome_classes=outcome_classes,
        probe=f"{module}({happy_args})",
        python=python,
        python_invert=python_invert,
        cases=[
            ("✅️", "applies", f"moves {noun.lower()} {owner_id}'s membership", f"{seed}\n    (snapshot(model), super::{module}({happy_args}))"),
            ("⛔️", "refuses", bad_description, f"{bad_seed or seed}\n    (snapshot(model), super::{module}({bad_args}))"),
        ],
    )


# region 🔖️G4Seeds
G4_BASE = '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.zones.push(zone(2, "ZONE TWO"));
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(1), value: 1.0 });
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(2), value: 0.5 });'''

G4_ELECTRIC = G4_BASE + '''
    model.pv_systems.push(crate::model::PvSystemAssignment { id: crate::model::EntityId(300), dc_capacity_w: 5000.0, area_m2: 30.0, tilt_deg: 30.0, azimuth_deg: 180.0, module_efficiency: 0.18, inverter_efficiency: 0.96 });
    model.pv_systems.push(crate::model::PvSystemAssignment { id: crate::model::EntityId(301), dc_capacity_w: 3000.0, area_m2: 18.0, tilt_deg: 20.0, azimuth_deg: 135.0, module_efficiency: 0.17, inverter_efficiency: 0.95 });
    model.battery_storage.push(crate::model::BatteryAssignment { id: crate::model::EntityId(310), capacity_kwh: 13.5, max_charge_w: 5000.0, max_discharge_w: 5000.0, round_trip_efficiency: 0.9 });'''

G4_LOAD_CENTERS = G4_ELECTRIC + '''
    model.electrical_load_centers.push(crate::model::ElectricalLoadCenter { id: crate::model::EntityId(320), name: "MAIN PANEL".into(), generator_ids: Vec::new(), pv_ids: vec![crate::model::EntityId(300)], battery_ids: Vec::new() });
    model.electrical_load_centers.push(crate::model::ElectricalLoadCenter { id: crate::model::EntityId(321), name: "SUB PANEL".into(), generator_ids: Vec::new(), pv_ids: Vec::new(), battery_ids: vec![crate::model::EntityId(310)] });'''

G4_SHW = G4_BASE + '''
    model.shw_systems.push(crate::model::ShwSystemConfig { id: crate::model::EntityId(400), heater_capacity_w: 4500.0, storage_volume_m3: 0.3, setpoint_c: 55.0, schedule_id: crate::model::ScheduleId(1) });'''

G4_SOLAR = G4_BASE + '''
    model.solar_thermal_systems.push(crate::model::SolarThermalConfig { id: crate::model::EntityId(410), collector_area_m2: 4.0, efficiency: 0.55, storage_volume_m3: 0.3, tilt_deg: 45.0, azimuth_deg: 180.0 });'''

G4_REFRIGERATION = G4_BASE + '''
    model.refrigeration_systems.push(crate::model::RefrigerationConfig { id: crate::model::EntityId(420), case_count: 4, design_load_w: 12000.0, defrost_schedule_id: crate::model::ScheduleId(1) });'''

G4_WATER = G4_BASE + '''
    model.water_systems.push(crate::model::WaterSystemConfig { id: crate::model::EntityId(430), fixture_count: 6, peak_flow_l_s: 0.25, schedule_id: crate::model::ScheduleId(1) });'''

G4_FAULTS = G4_BASE + '''
    model.ideal_loads.push(crate::model::IdealLoadsSystem { id: crate::model::EntityId(500), zone_id: crate::model::EntityId(1), max_heating_supply_air_temp_c: 50.0, min_cooling_supply_air_temp_c: 13.0, max_heating_capacity_w: None, max_cooling_capacity_w: None, outdoor_air_per_person_m3_s: 0.0, outdoor_air_per_area_m3_s_m2: 0.0 });
    model.ideal_loads.push(crate::model::IdealLoadsSystem { id: crate::model::EntityId(501), zone_id: crate::model::EntityId(2), max_heating_supply_air_temp_c: 50.0, min_cooling_supply_air_temp_c: 13.0, max_heating_capacity_w: None, max_cooling_capacity_w: None, outdoor_air_per_person_m3_s: 0.0, outdoor_air_per_area_m3_s_m2: 0.0 });
    model.faults.push(crate::model::FaultDefinition { id: crate::model::EntityId(440), target_equipment_id: crate::model::EntityId(500), fault_type: crate::model::FaultType::CoilFouling, severity: 0.3, start_schedule_id: crate::model::ScheduleId(1) });'''

G4_SPACES = G4_BASE + '''
    model.spaces.push(crate::model::Space { id: crate::model::EntityId(200), name: "SPACE ONE".into(), zone_id: crate::model::EntityId(1), floor_area_m2: 48.0 });
    model.spaces.push(crate::model::Space { id: crate::model::EntityId(201), name: "SPACE TWO".into(), zone_id: crate::model::EntityId(2), floor_area_m2: 48.0 });'''

G4_SPACE_LISTS = G4_SPACES + '''
    model.space_lists.push(crate::model::SpaceList { id: crate::model::EntityId(210), name: "CORE".into(), space_ids: vec![crate::model::EntityId(200)] });
    model.space_lists.push(crate::model::SpaceList { id: crate::model::EntityId(211), name: "PERIMETER".into(), space_ids: Vec::new() });'''

G4_ENCLOSURES = G4_BASE + '''
    model.thermal_enclosures.push(crate::model::ThermalEnclosure { id: crate::model::EntityId(220), name: "NORTH BLOCK".into(), zone_ids: vec![crate::model::EntityId(1)] });
    model.thermal_enclosures.push(crate::model::ThermalEnclosure { id: crate::model::EntityId(221), name: "SOUTH BLOCK".into(), zone_ids: Vec::new() });'''

G4_LOAD_CENTER_FIELDS = [("id", "crate::model::EntityId"), ("name", "String"), ("generator_ids", "Vec<crate::model::EntityId>"), ("pv_ids", "Vec<crate::model::EntityId>"), ("battery_ids", "Vec<crate::model::EntityId>")]
G4_PV_FIELDS = [("id", "crate::model::EntityId"), ("dc_capacity_w", "f64"), ("area_m2", "f64"), ("tilt_deg", "f64"), ("azimuth_deg", "f64"), ("module_efficiency", "f64"), ("inverter_efficiency", "f64")]
G4_BATTERY_FIELDS = [("id", "crate::model::EntityId"), ("capacity_kwh", "f64"), ("max_charge_w", "f64"), ("max_discharge_w", "f64"), ("round_trip_efficiency", "f64")]
G4_SHW_FIELDS = [("id", "crate::model::EntityId"), ("heater_capacity_w", "f64"), ("storage_volume_m3", "f64"), ("setpoint_c", "f64"), ("schedule_id", "crate::model::ScheduleId")]
G4_SOLAR_FIELDS = [("id", "crate::model::EntityId"), ("collector_area_m2", "f64"), ("efficiency", "f64"), ("storage_volume_m3", "f64"), ("tilt_deg", "f64"), ("azimuth_deg", "f64")]
G4_REFRIGERATION_FIELDS = [("id", "crate::model::EntityId"), ("case_count", "u32"), ("design_load_w", "f64"), ("defrost_schedule_id", "crate::model::ScheduleId")]
G4_WATER_FIELDS = [("id", "crate::model::EntityId"), ("fixture_count", "u32"), ("peak_flow_l_s", "f64"), ("schedule_id", "crate::model::ScheduleId")]
G4_FAULT_FIELDS = [("id", "crate::model::EntityId"), ("target_equipment_id", "crate::model::EntityId"), ("fault_type", "crate::model::FaultType"), ("severity", "f64"), ("start_schedule_id", "crate::model::ScheduleId")]
G4_SPACE_LIST_FIELDS = [("id", "crate::model::EntityId"), ("name", "String"), ("space_ids", "Vec<crate::model::EntityId>")]
G4_ENCLOSURE_FIELDS = [("id", "crate::model::EntityId"), ("name", "String"), ("zone_ids", "Vec<crate::model::EntityId>")]
# endregion 🔖️G4Seeds


# region 🔖️G4ElectricalLoadCentres
g4_create(
    number=700,
    slug="create-electrical-load-center",
    emoji="🏦️",
    collection="electrical_load_centers",
    noun="Electrical load center",
    struct="crate::model::ElectricalLoadCenter",
    fields=G4_LOAD_CENTER_FIELDS,
    refs=[("many", "pv_ids", "pv_systems", "PV system"), ("many", "battery_ids", "battery_storage", "Battery")],
    seed=G4_LOAD_CENTERS,
    doc="Adds one electrical load centre — the node that sums on-site generation and storage against the building's electrical demand. `pvIds` and `batteryIds` are checked against the document; `generatorIds` is carried verbatim and NOT checked, because `Model` has no generator collection for it to reference (vocabulary §5.4), which is also why this group ships no `add-electrical-load-center-generator`.",
    happy_args='2, crate::model::EntityId(322), "ROOF PANEL".into(), Vec::new(), vec![crate::model::EntityId(301)], Vec::new()',
    duplicate_args='2, crate::model::EntityId(320), "ROOF PANEL".into(), Vec::new(), Vec::new(), Vec::new()',
    delete_slug="delete-electrical-load-center",
)

g2_delete(
    number=701,
    slug="delete-electrical-load-center",
    emoji="🔻️",
    collection="electrical_load_centers",
    noun="Electrical load center",
    fields=G4_LOAD_CENTER_FIELDS,
    seed=G4_LOAD_CENTERS,
    doc="Removes one electrical load centre. Nothing in the document references a load centre — it is the referencing end of every generation edge — so this never has to refuse for use and never cascades.",
    target_id=321,
    create_slug="create-electrical-load-center",
)

g2_rename(
    number=702,
    slug="rename-electrical-load-center",
    emoji="🖊️",
    collection="electrical_load_centers",
    noun="Electrical load center",
    seed=G4_LOAD_CENTERS,
    target_id=320,
    happy='"MAIN DISTRIBUTION".to_string()',
    taken='"SUB PANEL".to_string()',
)

g4_membership(
    number=705,
    slug="add-electrical-load-center-pv",
    emoji="☄️",
    mode="add",
    collection="electrical_load_centers",
    noun="Electrical load center",
    field="pv_ids",
    member_field="pv_id",
    member_collection="pv_systems",
    member_noun="PV system",
    partner_slug="remove-electrical-load-center-pv",
    seed=G4_LOAD_CENTERS,
    doc="Attaches one PV system to a load centre, so its DC output is inverted onto that centre's bus.",
    owner_id=320,
    happy_extra="1, crate::model::EntityId(301)",
    bad_args="crate::model::EntityId(320), 1, crate::model::EntityId(999)",
    bad_description="refuses a PV system the document does not define",
)

g4_membership(
    number=706,
    slug="remove-electrical-load-center-pv",
    emoji="🌘️",
    mode="remove",
    collection="electrical_load_centers",
    noun="Electrical load center",
    field="pv_ids",
    member_field="pv_id",
    member_collection="pv_systems",
    member_noun="PV system",
    partner_slug="add-electrical-load-center-pv",
    seed=G4_LOAD_CENTERS,
    doc="Detaches one PV system from a load centre; the PV system itself survives, unattached.",
    owner_id=320,
    happy_extra="crate::model::EntityId(300)",
    bad_args="crate::model::EntityId(321), crate::model::EntityId(300)",
    bad_description="refuses a PV system the load centre does not carry",
)

g4_membership(
    number=707,
    slug="add-electrical-load-center-battery",
    emoji="🔋️",
    mode="add",
    collection="electrical_load_centers",
    noun="Electrical load center",
    field="battery_ids",
    member_field="battery_id",
    member_collection="battery_storage",
    member_noun="Battery",
    partner_slug="remove-electrical-load-center-battery",
    seed=G4_LOAD_CENTERS,
    doc="Attaches one battery to a load centre, so the centre may charge and discharge it against the net bus balance.",
    owner_id=320,
    happy_extra="0, crate::model::EntityId(310)",
    bad_args="crate::model::EntityId(320), 0, crate::model::EntityId(999)",
    bad_description="refuses a battery the document does not define",
)

g4_membership(
    number=708,
    slug="remove-electrical-load-center-battery",
    emoji="🪝️",
    mode="remove",
    collection="electrical_load_centers",
    noun="Electrical load center",
    field="battery_ids",
    member_field="battery_id",
    member_collection="battery_storage",
    member_noun="Battery",
    partner_slug="add-electrical-load-center-battery",
    seed=G4_LOAD_CENTERS,
    doc="Detaches one battery from a load centre; the battery itself survives, unattached.",
    owner_id=321,
    happy_extra="crate::model::EntityId(310)",
    bad_args="crate::model::EntityId(320), crate::model::EntityId(310)",
    bad_description="refuses a battery the load centre does not carry",
)
# endregion 🔖️G4ElectricalLoadCentres


# region 🔖️G4Photovoltaics
g4_create(
    number=709,
    slug="create-pv-system",
    emoji="✨️",
    collection="pv_systems",
    noun="PV system",
    struct="crate::model::PvSystemAssignment",
    fields=G4_PV_FIELDS,
    refs=[],
    seed=G4_ELECTRIC,
    doc="Adds one photovoltaic array. Capacity, aperture area and the two efficiencies fix the DC-to-AC chain; tilt and azimuth place the plane the incident-solar model integrates over.",
    happy_args="2, crate::model::EntityId(302), 8000.0, 44.0, 25.0, 200.0, 0.19, 0.97",
    duplicate_args="2, crate::model::EntityId(300), 8000.0, 44.0, 25.0, 200.0, 0.19, 0.97",
    delete_slug="delete-pv-system",
)

g2_delete(
    number=710,
    slug="delete-pv-system",
    emoji="🌒️",
    collection="pv_systems",
    noun="PV system",
    fields=G4_PV_FIELDS,
    seed=G4_ELECTRIC,
    doc="Removes one photovoltaic array. Refused while a load centre still lists it, so the electrical bus never sums over an array the document no longer defines.",
    target_id=300,
    create_slug="create-pv-system",
    in_use=("base.model.electrical_load_centers.iter().any(|centre| centre.pv_ids.contains(&payload.id))", "PV system {} is still attached to an electrical load centre.", 'any(payload["id"] in row["pv_ids"] for row in before["model"]["electrical_load_centers"])'),
    bad_seed=G4_LOAD_CENTERS,
    bad_id=300,
)

g2_scalar(number=711, slug="change-pv-system-dc-capacity", emoji="⚛️", collection="pv_systems", noun="PV system", field="dc_capacity_w", what="DC capacity (W)", guard="positive", seed=G4_ELECTRIC, target_id=300, happy="7000.0", bad="0.0")
g2_scalar(number=712, slug="change-pv-system-area", emoji="🟨️", collection="pv_systems", noun="PV system", field="area_m2", what="aperture area (m²)", guard="positive", seed=G4_ELECTRIC, target_id=300, happy="42.0", bad="-1.0")
g4_scalar(number=713, slug="change-pv-system-tilt", emoji="📈️", collection="pv_systems", noun="PV system", field="tilt_deg", what="tilt", guard="tilt", seed=G4_ELECTRIC, target_id=300, happy="35.0", bad="120.0")
g4_scalar(number=714, slug="change-pv-system-azimuth", emoji="🧿️", collection="pv_systems", noun="PV system", field="azimuth_deg", what="azimuth", guard="azimuth", seed=G4_ELECTRIC, target_id=300, happy="170.0", bad="400.0")
g2_scalar(number=715, slug="change-pv-system-module-efficiency", emoji="🎖️", collection="pv_systems", noun="PV system", field="module_efficiency", what="module efficiency", guard="unit-positive", seed=G4_ELECTRIC, target_id=300, happy="0.21", bad="0.0")
g2_scalar(number=716, slug="change-pv-system-inverter-efficiency", emoji="♌️", collection="pv_systems", noun="PV system", field="inverter_efficiency", what="inverter efficiency", guard="unit-positive", seed=G4_ELECTRIC, target_id=300, happy="0.98", bad="1.5")
# endregion 🔖️G4Photovoltaics


# region 🔖️G4Batteries
g4_create(
    number=717,
    slug="create-battery",
    emoji="🪙️",
    collection="battery_storage",
    noun="Battery",
    struct="crate::model::BatteryAssignment",
    fields=G4_BATTERY_FIELDS,
    refs=[],
    seed=G4_ELECTRIC,
    doc="Adds one electrical storage unit. Capacity bounds the state of charge; the two power limits bound each timestep's charge and discharge, and the round-trip efficiency is what the stored energy is debited by.",
    happy_args="1, crate::model::EntityId(311), 27.0, 7000.0, 7000.0, 0.92",
    duplicate_args="1, crate::model::EntityId(310), 27.0, 7000.0, 7000.0, 0.92",
    delete_slug="delete-battery",
)

g2_delete(
    number=718,
    slug="delete-battery",
    emoji="♒️",
    collection="battery_storage",
    noun="Battery",
    fields=G4_BATTERY_FIELDS,
    seed=G4_ELECTRIC,
    doc="Removes one electrical storage unit. Refused while a load centre still lists it.",
    target_id=310,
    create_slug="create-battery",
    in_use=("base.model.electrical_load_centers.iter().any(|centre| centre.battery_ids.contains(&payload.id))", "Battery {} is still attached to an electrical load centre.", 'any(payload["id"] in row["battery_ids"] for row in before["model"]["electrical_load_centers"])'),
    bad_seed=G4_LOAD_CENTERS,
    bad_id=310,
)

g2_scalar(number=719, slug="change-battery-capacity", emoji="🥫️", collection="battery_storage", noun="Battery", field="capacity_kwh", what="storage capacity (kWh)", guard="positive", seed=G4_ELECTRIC, target_id=310, happy="20.0", bad="0.0")
g2_scalar(number=720, slug="change-battery-max-charge", emoji="⏫️", collection="battery_storage", noun="Battery", field="max_charge_w", what="maximum charge power (W)", guard="positive", seed=G4_ELECTRIC, target_id=310, happy="7000.0", bad="-100.0")
g2_scalar(number=721, slug="change-battery-max-discharge", emoji="⏬️", collection="battery_storage", noun="Battery", field="max_discharge_w", what="maximum discharge power (W)", guard="positive", seed=G4_ELECTRIC, target_id=310, happy="6000.0", bad="0.0")
g2_scalar(number=722, slug="change-battery-round-trip-efficiency", emoji="🥉️", collection="battery_storage", noun="Battery", field="round_trip_efficiency", what="round-trip efficiency", guard="unit-positive", seed=G4_ELECTRIC, target_id=310, happy="0.95", bad="1.2")
# endregion 🔖️G4Batteries


# region 🔖️G4ServiceHotWater
g4_create(
    number=723,
    slug="create-shw-system",
    emoji="🛀️",
    collection="shw_systems",
    noun="Service hot water system",
    struct="crate::model::ShwSystemConfig",
    fields=G4_SHW_FIELDS,
    refs=[("one", "schedule_id", "schedules", "Schedule")],
    seed=G4_SHW,
    doc="Adds one service-hot-water system: a storage tank, the heater that keeps it at setpoint, and the draw schedule the load profile is read from.",
    happy_args="1, crate::model::EntityId(401), 6000.0, 0.2, 60.0, crate::model::ScheduleId(2)",
    duplicate_args="1, crate::model::EntityId(400), 6000.0, 0.2, 60.0, crate::model::ScheduleId(2)",
    delete_slug="delete-shw-system",
)

g2_delete(
    number=724,
    slug="delete-shw-system",
    emoji="🚱️",
    collection="shw_systems",
    noun="Service hot water system",
    fields=G4_SHW_FIELDS,
    seed=G4_SHW,
    doc="Removes one service-hot-water system. Nothing in the document references one, so this never refuses for use.",
    target_id=400,
    create_slug="create-shw-system",
)

g2_scalar(number=725, slug="change-shw-system-heater-capacity", emoji="🍵️", collection="shw_systems", noun="Service hot water system", field="heater_capacity_w", what="heater capacity (W)", guard="positive", seed=G4_SHW, target_id=400, happy="6000.0", bad="0.0")
g2_scalar(number=726, slug="change-shw-system-storage-volume", emoji="🛢️", collection="shw_systems", noun="Service hot water system", field="storage_volume_m3", what="storage volume (m³)", guard="positive", seed=G4_SHW, target_id=400, happy="0.2", bad="-0.1")
g4_scalar(number=727, slug="change-shw-system-setpoint", emoji="🏹️", collection="shw_systems", noun="Service hot water system", field="setpoint_c", what="tank setpoint (°C)", guard="hot-water", seed=G4_SHW, target_id=400, happy="60.0", bad="250.0")
g2_reference(number=728, slug="change-shw-system-schedule", emoji="🕐️", collection="shw_systems", noun="Service hot water system", field="schedule_id", ref="schedule", ref_noun="Schedule", seed=G4_SHW, target_id=400, happy="crate::model::ScheduleId(2)", bad="crate::model::ScheduleId(99)")
# endregion 🔖️G4ServiceHotWater


# region 🔖️G4SolarThermal
g4_create(
    number=729,
    slug="create-solar-thermal-system",
    emoji="🌄️",
    collection="solar_thermal_systems",
    noun="Solar thermal system",
    struct="crate::model::SolarThermalConfig",
    fields=G4_SOLAR_FIELDS,
    refs=[],
    seed=G4_SOLAR,
    doc="Adds one solar thermal collector loop: aperture, conversion efficiency, the buffer it charges, and the plane the incident-solar model integrates over.",
    happy_args="1, crate::model::EntityId(411), 6.0, 0.6, 0.4, 40.0, 190.0",
    duplicate_args="1, crate::model::EntityId(410), 6.0, 0.6, 0.4, 40.0, 190.0",
    delete_slug="delete-solar-thermal-system",
)

g2_delete(
    number=730,
    slug="delete-solar-thermal-system",
    emoji="🌆️",
    collection="solar_thermal_systems",
    noun="Solar thermal system",
    fields=G4_SOLAR_FIELDS,
    seed=G4_SOLAR,
    doc="Removes one solar thermal collector loop. Nothing in the document references one, so this never refuses for use.",
    target_id=410,
    create_slug="create-solar-thermal-system",
)

g2_scalar(number=731, slug="change-solar-thermal-system-collector-area", emoji="🟩️", collection="solar_thermal_systems", noun="Solar thermal system", field="collector_area_m2", what="collector area (m²)", guard="positive", seed=G4_SOLAR, target_id=410, happy="6.0", bad="0.0")
g2_scalar(number=732, slug="change-solar-thermal-system-efficiency", emoji="🏅️", collection="solar_thermal_systems", noun="Solar thermal system", field="efficiency", what="collector efficiency", guard="unit-positive", seed=G4_SOLAR, target_id=410, happy="0.62", bad="1.4")
g2_scalar(number=733, slug="change-solar-thermal-system-storage-volume", emoji="🧃️", collection="solar_thermal_systems", noun="Solar thermal system", field="storage_volume_m3", what="buffer volume (m³)", guard="positive", seed=G4_SOLAR, target_id=410, happy="0.5", bad="-2.0")
g4_scalar(number=734, slug="change-solar-thermal-system-tilt", emoji="🔼️", collection="solar_thermal_systems", noun="Solar thermal system", field="tilt_deg", what="tilt", guard="tilt", seed=G4_SOLAR, target_id=410, happy="35.0", bad="95.0")
g4_scalar(number=735, slug="change-solar-thermal-system-azimuth", emoji="⛵️", collection="solar_thermal_systems", noun="Solar thermal system", field="azimuth_deg", what="azimuth", guard="azimuth", seed=G4_SOLAR, target_id=410, happy="200.0", bad="-30.0")
# endregion 🔖️G4SolarThermal


# region 🔖️G4Refrigeration
g4_create(
    number=736,
    slug="create-refrigeration-system",
    emoji="❄️",
    collection="refrigeration_systems",
    noun="Refrigeration system",
    struct="crate::model::RefrigerationConfig",
    fields=G4_REFRIGERATION_FIELDS,
    refs=[("one", "defrost_schedule_id", "schedules", "Schedule")],
    seed=G4_REFRIGERATION,
    doc="Adds one refrigeration system: the display cases it serves, their design load, and the defrost schedule that periodically reverses it.",
    happy_args="1, crate::model::EntityId(421), 2, 8000.0, crate::model::ScheduleId(2)",
    duplicate_args="1, crate::model::EntityId(420), 2, 8000.0, crate::model::ScheduleId(2)",
    delete_slug="delete-refrigeration-system",
)

g2_delete(
    number=737,
    slug="delete-refrigeration-system",
    emoji="🫠️",
    collection="refrigeration_systems",
    noun="Refrigeration system",
    fields=G4_REFRIGERATION_FIELDS,
    seed=G4_REFRIGERATION,
    doc="Removes one refrigeration system. Nothing in the document references one, so this never refuses for use.",
    target_id=420,
    create_slug="create-refrigeration-system",
)

g4_count(number=738, slug="change-refrigeration-system-case-count", emoji="🗄️", collection="refrigeration_systems", noun="Refrigeration system", field="case_count", what="display case", seed=G4_REFRIGERATION, target_id=420, happy="6")
g2_scalar(number=739, slug="change-refrigeration-system-design-load", emoji="🏋️", collection="refrigeration_systems", noun="Refrigeration system", field="design_load_w", what="design load (W)", guard="positive", seed=G4_REFRIGERATION, target_id=420, happy="15000.0", bad="0.0")
g2_reference(number=740, slug="change-refrigeration-system-defrost-schedule", emoji="🕑️", collection="refrigeration_systems", noun="Refrigeration system", field="defrost_schedule_id", ref="schedule", ref_noun="Schedule", seed=G4_REFRIGERATION, target_id=420, happy="crate::model::ScheduleId(2)", bad="crate::model::ScheduleId(99)")
# endregion 🔖️G4Refrigeration


# region 🔖️G4WaterSystems
g4_create(
    number=741,
    slug="create-water-system",
    emoji="🚽️",
    collection="water_systems",
    noun="Water system",
    struct="crate::model::WaterSystemConfig",
    fields=G4_WATER_FIELDS,
    refs=[("one", "schedule_id", "schedules", "Schedule")],
    seed=G4_WATER,
    doc="Adds one cold-water use system: the fixtures it serves, their combined peak flow, and the draw schedule the profile is read from.",
    happy_args="1, crate::model::EntityId(431), 4, 0.15, crate::model::ScheduleId(2)",
    duplicate_args="1, crate::model::EntityId(430), 4, 0.15, crate::model::ScheduleId(2)",
    delete_slug="delete-water-system",
)

g2_delete(
    number=742,
    slug="delete-water-system",
    emoji="🧼️",
    collection="water_systems",
    noun="Water system",
    fields=G4_WATER_FIELDS,
    seed=G4_WATER,
    doc="Removes one cold-water use system. Nothing in the document references one, so this never refuses for use.",
    target_id=430,
    create_slug="create-water-system",
)

g4_count(number=743, slug="change-water-system-fixture-count", emoji="🪣️", collection="water_systems", noun="Water system", field="fixture_count", what="fixture", seed=G4_WATER, target_id=430, happy="9")
g2_scalar(number=744, slug="change-water-system-peak-flow", emoji="🚾️", collection="water_systems", noun="Water system", field="peak_flow_l_s", what="peak flow (L/s)", guard="positive", seed=G4_WATER, target_id=430, happy="0.4", bad="0.0")
g2_reference(number=745, slug="change-water-system-schedule", emoji="🕒️", collection="water_systems", noun="Water system", field="schedule_id", ref="schedule", ref_noun="Schedule", seed=G4_WATER, target_id=430, happy="crate::model::ScheduleId(2)", bad="crate::model::ScheduleId(99)")
# endregion 🔖️G4WaterSystems


# region 🔖️G4Faults
g4_create(
    number=746,
    slug="create-fault",
    emoji="⚠️",
    collection="faults",
    noun="Fault",
    struct="crate::model::FaultDefinition",
    fields=G4_FAULT_FIELDS,
    refs=[("one", "target_equipment_id", "ideal_loads", "Ideal loads system"), ("one", "start_schedule_id", "schedules", "Schedule")],
    seed=G4_FAULTS,
    doc="Adds one equipment fault. `targetEquipmentId` is checked against `ideal_loads` because that is the collection the kernel actually matches it against (`SystemSubstepStage::Fault` compares `fault.target_equipment_id == ideal.id`); the field's own type carries no discriminator, so this is the only referent the engine gives it.",
    happy_args="1, crate::model::EntityId(441), crate::model::EntityId(501), crate::model::FaultType::SensorBias, 0.5, crate::model::ScheduleId(2)",
    duplicate_args="1, crate::model::EntityId(440), crate::model::EntityId(501), crate::model::FaultType::SensorBias, 0.5, crate::model::ScheduleId(2)",
    delete_slug="delete-fault",
)

g2_delete(
    number=747,
    slug="delete-fault",
    emoji="🩹️",
    collection="faults",
    noun="Fault",
    fields=G4_FAULT_FIELDS,
    seed=G4_FAULTS,
    doc="Removes one equipment fault, restoring the equipment it degraded to its rated behaviour. Nothing references a fault, so this never refuses for use.",
    target_id=440,
    create_slug="create-fault",
)

g4_entity_reference(number=748, slug="change-fault-target-equipment", emoji="🎣️", collection="faults", noun="Fault", field="target_equipment_id", target_collection="ideal_loads", ref_noun="Ideal loads system", seed=G4_FAULTS, target_id=440, happy="crate::model::EntityId(501)", bad="crate::model::EntityId(999)")
g4_enum(number=749, slug="change-fault-type", emoji="🐛️", collection="faults", noun="Fault", field="fault_type", rust_type="crate::model::FaultType", what="degradation mechanism", seed=G4_FAULTS, target_id=440, happy="crate::model::FaultType::SensorBias", bad_seed=G4_FAULTS, bad="crate::model::EntityId(999), crate::model::FaultType::SensorBias")
g2_scalar(number=750, slug="change-fault-severity", emoji="🌶️", collection="faults", noun="Fault", field="severity", what="severity", guard="fraction", seed=G4_FAULTS, target_id=440, happy="0.6", bad="1.4")
g2_reference(number=751, slug="change-fault-start-schedule", emoji="🕓️", collection="faults", noun="Fault", field="start_schedule_id", ref="schedule", ref_noun="Schedule", seed=G4_FAULTS, target_id=440, happy="crate::model::ScheduleId(2)", bad="crate::model::ScheduleId(99)")
# endregion 🔖️G4Faults


# region 🔖️G4SpaceLists
g4_create(
    number=800,
    slug="create-space-list",
    emoji="📋️",
    collection="space_lists",
    noun="Space list",
    struct="crate::model::SpaceList",
    fields=G4_SPACE_LIST_FIELDS,
    refs=[("many", "space_ids", "spaces", "Space")],
    seed=G4_SPACE_LISTS,
    doc="Adds one named grouping of spaces — the handle a report or an assignment addresses many spaces by at once.",
    happy_args='2, crate::model::EntityId(212), "ATRIUM".into(), vec![crate::model::EntityId(201)]',
    duplicate_args='2, crate::model::EntityId(210), "ATRIUM".into(), Vec::new()',
    delete_slug="delete-space-list",
)

g2_delete(
    number=801,
    slug="delete-space-list",
    emoji="🗒️",
    collection="space_lists",
    noun="Space list",
    fields=G4_SPACE_LIST_FIELDS,
    seed=G4_SPACE_LISTS,
    doc="Removes one space grouping. The spaces themselves are untouched — a list owns membership, not the members.",
    target_id=211,
    create_slug="create-space-list",
)

g2_rename(
    number=802,
    slug="rename-space-list",
    emoji="🪶️",
    collection="space_lists",
    noun="Space list",
    seed=G4_SPACE_LISTS,
    target_id=210,
    happy='"CORE SPACES".to_string()',
    taken='"PERIMETER".to_string()',
)

g4_membership(
    number=803,
    slug="add-space-list-member",
    emoji="➡️",
    mode="add",
    collection="space_lists",
    noun="Space list",
    field="space_ids",
    member_field="space_id",
    member_collection="spaces",
    member_noun="Space",
    partner_slug="remove-space-list-member",
    seed=G4_SPACE_LISTS,
    doc="Puts one space into a grouping.",
    owner_id=210,
    happy_extra="1, crate::model::EntityId(201)",
    bad_args="crate::model::EntityId(210), 1, crate::model::EntityId(999)",
    bad_description="refuses a space the document does not define",
)

g4_membership(
    number=804,
    slug="remove-space-list-member",
    emoji="🪤️",
    mode="remove",
    collection="space_lists",
    noun="Space list",
    field="space_ids",
    member_field="space_id",
    member_collection="spaces",
    member_noun="Space",
    partner_slug="add-space-list-member",
    seed=G4_SPACE_LISTS,
    doc="Takes one space back out of a grouping; the space itself survives.",
    owner_id=210,
    happy_extra="crate::model::EntityId(200)",
    bad_args="crate::model::EntityId(211), crate::model::EntityId(200)",
    bad_description="refuses a space the grouping does not hold",
)
# endregion 🔖️G4SpaceLists


# region 🔖️G4ThermalEnclosures
g4_create(
    number=805,
    slug="create-thermal-enclosure",
    emoji="🏟️",
    collection="thermal_enclosures",
    noun="Thermal enclosure",
    struct="crate::model::ThermalEnclosure",
    fields=G4_ENCLOSURE_FIELDS,
    refs=[("many", "zone_ids", "zones", "Zone")],
    seed=G4_ENCLOSURES,
    doc="Adds one thermal enclosure — the named set of zones that share one continuous envelope boundary, which is what an envelope-area report normalizes over.",
    happy_args='2, crate::model::EntityId(222), "ROOF BLOCK".into(), vec![crate::model::EntityId(2)]',
    duplicate_args='2, crate::model::EntityId(220), "ROOF BLOCK".into(), Vec::new()',
    delete_slug="delete-thermal-enclosure",
)

g2_delete(
    number=806,
    slug="delete-thermal-enclosure",
    emoji="🏯️",
    collection="thermal_enclosures",
    noun="Thermal enclosure",
    fields=G4_ENCLOSURE_FIELDS,
    seed=G4_ENCLOSURES,
    doc="Removes one thermal enclosure. The zones themselves are untouched — an enclosure owns membership, not the members.",
    target_id=221,
    create_slug="create-thermal-enclosure",
)

g2_rename(
    number=807,
    slug="rename-thermal-enclosure",
    emoji="🖋️",
    collection="thermal_enclosures",
    noun="Thermal enclosure",
    seed=G4_ENCLOSURES,
    target_id=220,
    happy='"NORTH ENCLOSURE".to_string()',
    taken='"SOUTH BLOCK".to_string()',
)

g4_membership(
    number=808,
    slug="add-thermal-enclosure-zone",
    emoji="🔒️",
    mode="add",
    collection="thermal_enclosures",
    noun="Thermal enclosure",
    field="zone_ids",
    member_field="zone_id",
    member_collection="zones",
    member_noun="Zone",
    partner_slug="remove-thermal-enclosure-zone",
    seed=G4_ENCLOSURES,
    doc="Puts one zone inside a thermal enclosure.",
    owner_id=220,
    happy_extra="1, crate::model::EntityId(2)",
    bad_args="crate::model::EntityId(220), 1, crate::model::EntityId(999)",
    bad_description="refuses a zone the document does not define",
)

g4_membership(
    number=809,
    slug="remove-thermal-enclosure-zone",
    emoji="🔓️",
    mode="remove",
    collection="thermal_enclosures",
    noun="Thermal enclosure",
    field="zone_ids",
    member_field="zone_id",
    member_collection="zones",
    member_noun="Zone",
    partner_slug="add-thermal-enclosure-zone",
    seed=G4_ENCLOSURES,
    doc="Takes one zone back out of a thermal enclosure; the zone itself survives.",
    owner_id=220,
    happy_extra="crate::model::EntityId(1)",
    bad_args="crate::model::EntityId(221), crate::model::EntityId(1)",
    bad_description="refuses a zone the enclosure does not hold",
)
# endregion 🔖️G4ThermalEnclosures


# region 🔖️G4Schedules
#: 🔗️ Whether the document already defines this `ScheduleId`. One `ScheduleId` namespace spans all
#: five groups — `ScheduleSet::lookup` searches every group in turn — so a create refuses an id that
#: any group already holds, not merely its own.
G4_SCHEDULE_TAKEN_RS = G2_SCHEDULE_EXISTS.format(v="payload.id")
G4_SCHEDULE_TAKEN_PY = G2_PY_SCHEDULE_EXISTS.format(v='payload["id"]')

#: 🚧️ Whether anything still resolves this `ScheduleId`. Deleting a schedule out from under a
#: reference would dangle it, and `Model::validate` reports exactly that as a severe diagnostic, so
#: the delete refuses instead. The list is every `ScheduleId`-typed slot in the document, schedule
#: groups that reference other schedules included.
G4_SCHEDULE_IN_USE_RS = " || ".join([
    "base.model.people.iter().any(|row| row.schedule_id == payload.id || row.activity_schedule_id == payload.id)",
    "base.model.lighting.iter().any(|row| row.schedule_id == payload.id)",
    "base.model.equipment.iter().any(|row| row.schedule_id == payload.id)",
    "base.model.infiltrations.iter().any(|row| row.schedule_id == payload.id)",
    "base.model.mechanical_ventilations.iter().any(|row| row.schedule_id == payload.id)",
    "base.model.thermostats.iter().any(|row| row.heating_setpoint_schedule_id == payload.id || row.cooling_setpoint_schedule_id == payload.id)",
    "base.model.humidistats.iter().any(|row| row.humidifying_setpoint_schedule_id == payload.id || row.dehumidifying_setpoint_schedule_id == payload.id)",
    "base.model.setpoint_managers.iter().any(|row| row.schedule_id == Some(payload.id))",
    "base.model.shading_surfaces.iter().any(|row| row.transmittance_schedule_id == Some(payload.id))",
    "base.model.shw_systems.iter().any(|row| row.schedule_id == payload.id)",
    "base.model.refrigeration_systems.iter().any(|row| row.defrost_schedule_id == payload.id)",
    "base.model.water_systems.iter().any(|row| row.schedule_id == payload.id)",
    "base.model.faults.iter().any(|row| row.start_schedule_id == payload.id)",
    "base.model.schedules.weekly.iter().any(|row| row.daily_schedule_ids.contains(&payload.id))",
    "base.model.schedules.annual.iter().any(|row| row.default_daily_schedule_id == payload.id || row.holiday_daily_schedule_id == Some(payload.id) || row.rules.iter().any(|rule| rule.daily_schedule_id == payload.id))",
])

G4_SCHEDULE_IN_USE_PY = '''    consumers = (("people", ("schedule_id", "activity_schedule_id")), ("lighting", ("schedule_id",)), ("equipment", ("schedule_id",)), ("infiltrations", ("schedule_id",)), ("mechanical_ventilations", ("schedule_id",)), ("thermostats", ("heating_setpoint_schedule_id", "cooling_setpoint_schedule_id")), ("humidistats", ("humidifying_setpoint_schedule_id", "dehumidifying_setpoint_schedule_id")), ("setpoint_managers", ("schedule_id",)), ("shading_surfaces", ("transmittance_schedule_id",)), ("shw_systems", ("schedule_id",)), ("refrigeration_systems", ("defrost_schedule_id",)), ("water_systems", ("schedule_id",)), ("faults", ("start_schedule_id",)))
    in_use = any(row[name] == payload["id"] for collection, names in consumers for row in before["model"][collection] for name in names)
    in_use = in_use or any(payload["id"] in row["daily_schedule_ids"] for row in before["model"]["schedules"]["weekly"])
    in_use = in_use or any(row["default_daily_schedule_id"] == payload["id"] or row["holiday_daily_schedule_id"] == payload["id"] or any(rule["daily_schedule_id"] == payload["id"] for rule in row["rules"]) for row in before["model"]["schedules"]["annual"])'''

G4_DAILY_EXISTS_RS = "base.model.schedules.daily.iter().any(|row| row.id == {v})"
G4_DAILY_EXISTS_PY = 'any(row["id"] == {v} for row in before["model"]["schedules"]["daily"])'


#: 🔤️ The past-tense record names the shared table does not carry — the schedule group is the first
#: to use `insert` (ordered rule list) and `replace` (whole-body swap) as verbs.
G4_PAST = dict(G2_PAST, insert="Inserted", replace="Replaced")


def g4_titles(slug: str):
    verb, *rest = slug.split("-")
    return verb, G4_PAST[verb] + camel("-".join(rest)), " ".join(part.capitalize() for part in slug.split("-"))


def g4_schedule_missing_rs(group: str, noun: str) -> str:
    return f'''    let Some(existing) = base.model.schedules.{group}.iter().find(|item| item.id == payload.id) else {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{noun} {{}} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    }};'''


def g4_schedule_missing_py(group: str) -> str:
    return f'''    item = next((row for row in before["model"]["schedules"]["{group}"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])'''


def g4_schedule_create(*, number, slug, emoji, group, struct, noun, fields, ctor, py_row, seed, doc, happy_args, duplicate_args, delete_slug, validate_rs="", validate_refused_rs="", validate_py=""):
    """📅️ One schedule definition, inserted at a stated `index` inside its own group."""
    verb, record, display = g4_titles(slug)
    module = snake(slug)
    diff = f'''    if {G4_SCHEDULE_TAKEN_RS} {{
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Schedule {{}} is already defined.", payload.id.0), [payload.id.0.to_string()]);
    }}
    if payload.index as usize > base.model.schedules.{group}.len() {{
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {{}} is past the end of the model's {{}} {group} schedules.", payload.index, base.model.schedules.{group}.len()), [payload.id.0.to_string()]);
    }}{validate_rs}
    let mut model = base.model.clone();
    model.schedules.{group}.insert(payload.index as usize, crate::schedule::{struct} {{ {ctor} }});
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''
    inverse = f'''    if {G4_SCHEDULE_TAKEN_RS} || payload.index as usize > base.model.schedules.{group}.len(){validate_refused_rs} {{
        return Vec::new();
    }}
    vec![super::{snake(delete_slug)}(payload.id)]'''
    python = f'''    """{emoji} `{slug}{{index,…}}` — {doc}"""
    rows = before["model"]["schedules"]["{group}"]
    if {G4_SCHEDULE_TAKEN_PY}:
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])]){validate_py}
    after = copy.deepcopy(before)
    after["model"]["schedules"]["{group}"].insert(payload["index"], {py_row})
    return after, applied()'''
    python_invert = '''    """↩️ Creation is undone by deleting exactly the schedule it defined."""
    return [("%s", {"id": payload["id"]})]''' % delete_slug
    kind(
        number=number,
        slug=slug,
        emoji=emoji,
        verb=verb,
        entity=noun.lower().replace(" ", "-"),
        record=record,
        display=display,
        doc=doc,
        fields=[("index", "u32"), *fields],
        label=f'format!("{display} {{}} at index {{}}", self.id.0, self.index)',
        target="vec![self.id.0.to_string()]",
        diff=diff,
        inverse=inverse,
        outcome_classes=["applied", "error"],
        probe=f"{module}({happy_args})",
        python=python,
        python_invert=python_invert,
        cases=[
            ("✅️", "applies", f"defines a new {noun.lower()}", f"{seed}\n    (snapshot(model), super::{module}({happy_args}))"),
            ("⛔️", "refuses", "refuses a schedule id the document already defines", f"{seed}\n    (snapshot(model), super::{module}({duplicate_args}))"),
        ],
    )


def g4_schedule_delete(*, number, slug, emoji, group, noun, create_slug, create_args_rs, create_row_py, seed, target_id, bad_seed, bad_id, doc):
    """🗑️ One schedule definition, removed by id. Refused while anything still resolves it."""
    verb, record, display = g4_titles(slug)
    module = snake(slug)
    diff = f'''{g4_schedule_missing_rs(group, noun)}
    let _ = existing;
    if {G4_SCHEDULE_IN_USE_RS} {{
        return protocol::MutationOutcome::error("mutation.invariant", format!("Schedule {{}} is still referenced by the document.", payload.id.0), [payload.id.0.to_string()]);
    }}
    let mut model = base.model.clone();
    model.schedules.{group}.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''
    inverse = f'''    let Some(index) = base.model.schedules.{group}.iter().position(|item| item.id == payload.id) else {{
        return Vec::new();
    }};
    if {G4_SCHEDULE_IN_USE_RS} {{
        return Vec::new();
    }}
    let existing = &base.model.schedules.{group}[index];
    vec![super::{snake(create_slug)}(index as u32, {create_args_rs})]'''
    python = f'''    """{emoji} `{slug}{{id}}` — {doc}"""
{g4_schedule_missing_py(group)}
{G4_SCHEDULE_IN_USE_PY}
    if in_use:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["{group}"] = [row for row in after["model"]["schedules"]["{group}"] if row["id"] != payload["id"]]
    return after, applied()'''
    python_invert = f'''    """↩️ Re-defines the removed schedule at the index it actually held."""
    rows = before["model"]["schedules"]["{group}"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("{create_slug}", {{"index": index, {create_row_py}}})]'''
    kind(
        number=number,
        slug=slug,
        emoji=emoji,
        verb=verb,
        entity=noun.lower().replace(" ", "-"),
        record=record,
        display=display,
        doc=doc,
        fields=[("id", "crate::model::ScheduleId")],
        label=f'format!("{display} {{}}", self.id.0)',
        target="vec![self.id.0.to_string()]",
        diff=diff,
        inverse=inverse,
        outcome_classes=["applied", "error"],
        probe=f"{module}(crate::model::ScheduleId({target_id}))",
        python=python,
        python_invert=python_invert,
        cases=[
            ("✅️", "applies", f"removes {noun.lower()} {target_id}", f"{seed}\n    (snapshot(model), super::{module}(crate::model::ScheduleId({target_id})))"),
            ("⛔️", "refuses", "refuses while the document still resolves it", f"{bad_seed}\n    (snapshot(model), super::{module}(crate::model::ScheduleId({bad_id})))"),
        ],
    )


def g4_schedule_element(*, number, slug, emoji, group, noun, field, rust_type, doc, guard_rs, guard_py, refuse_rs, show, seed, target_id, happy, bad, bad_seed, bad_description, outcome_classes, compare_rs=None, value_rs=None, inverse_value_rs=None, python_compare=None, python_value="value", python_inverse_value=None):
    """🔧️ One field of one schedule definition — the schedule-group twin of the shared element
    helper, which only addresses top-level `Model` collections keyed by `EntityId`."""
    verb, record, display = g4_titles(slug)
    module = snake(slug)
    payload_field = f"new_{field}"
    key = field_camel(payload_field)
    v = f"payload.{payload_field}"
    compare_rs = compare_rs or "{o}." + field + " == " + v
    value_rs = value_rs or v
    inverse_value_rs = inverse_value_rs or f"item.{field}"
    python_compare = python_compare or f'item["{field}"] == {python_value}'
    python_inverse_value = python_inverse_value or f'item["{field}"]'
    diff = f'''{g4_schedule_missing_rs(group, noun)}{guard_rs}
    if {compare_rs.format(o="existing")} {{
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("{noun} {{}} already carries this {field}: {show}.", payload.id.0, {v}));
    }}
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.{group}.iter_mut().find(|item| item.id == payload.id) {{
        item.{field} = {value_rs};
    }}
    protocol::MutationOutcome::new(SUPER_DIFF(model))'''
    inverse = f'''    match base.model.schedules.{group}.iter().find(|item| item.id == payload.id) {{
        Some(item) if !({compare_rs.format(o="item")}){refuse_rs} => vec![super::{module}(payload.id, {inverse_value_rs})],
        _ => Vec::new(),
    }}'''
    python = f'''    """{emoji} `{slug}{{id,{key}}}` — {doc}"""
    value = payload["{key}"]
{g4_schedule_missing_py(group)}{guard_py}
    if {python_compare}:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["{group}"]:
        if row["id"] == payload["id"]:
            row["{field}"] = {python_value}
    return after, applied()'''
    python_invert = f'''    """↩️ The {field} the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["{group}"] if row["id"] == payload["id"])
    return [("{slug}", {{"id": payload["id"], "{key}": {python_inverse_value}}})]'''
    kind(
        number=number,
        slug=slug,
        emoji=emoji,
        verb=verb,
        entity=noun.lower().replace(" ", "-"),
        record=record,
        display=display,
        doc=doc,
        fields=[("id", "crate::model::ScheduleId"), (payload_field, rust_type)],
        label=f'format!("{display} of {noun.lower()} {{}}", self.id.0)',
        target="vec![self.id.0.to_string()]",
        diff=diff,
        inverse=inverse,
        outcome_classes=outcome_classes,
        probe=f"{module}(crate::model::ScheduleId({target_id}), {happy})",
        python=python,
        python_invert=python_invert,
        cases=[
            ("✅️", "applies", f"moves {noun.lower()} {target_id}", f"{seed}\n    (snapshot(model), super::{module}(crate::model::ScheduleId({target_id}), {happy}))"),
            ("⛔️", "refuses", bad_description, f"{bad_seed}\n    (snapshot(model), super::{module}({bad}))"),
        ],
    )


G4_SCHEDULES = '''    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(1), value: 1.0 });
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(2), value: 0.5 });
    model.schedules.daily.push(crate::schedule::DailySchedule { id: crate::model::ScheduleId(10), hourly_values: [20.0; 24], interpolation: crate::schedule::ScheduleInterpolation::Continuous, limits: None });
    model.schedules.daily.push(crate::schedule::DailySchedule { id: crate::model::ScheduleId(11), hourly_values: [27.0; 24], interpolation: crate::schedule::ScheduleInterpolation::Discrete, limits: Some(crate::schedule::ScheduleLimits { min: 0.0, max: 100.0 }) });
    model.schedules.daily.push(crate::schedule::DailySchedule { id: crate::model::ScheduleId(12), hourly_values: [1.0; 24], interpolation: crate::schedule::ScheduleInterpolation::Continuous, limits: None });
    model.schedules.weekly.push(crate::schedule::WeeklySchedule { id: crate::model::ScheduleId(20), daily_schedule_ids: [crate::model::ScheduleId(10); 7] });
    model.schedules.annual.push(crate::schedule::AnnualSchedule { id: crate::model::ScheduleId(30), rules: vec![crate::schedule::CompactScheduleRule { start_month: 1, start_day: 1, end_month: 6, end_day: 30, daily_schedule_id: crate::model::ScheduleId(10) }, crate::schedule::CompactScheduleRule { start_month: 7, start_day: 1, end_month: 12, end_day: 31, daily_schedule_id: crate::model::ScheduleId(11) }], default_daily_schedule_id: crate::model::ScheduleId(11), holiday_daily_schedule_id: None, holiday_dates: vec![(2026, 12, 25)] });
    model.schedules.time_series.push(crate::schedule::TimeSeriesSchedule { id: crate::model::ScheduleId(40), values: vec![1.0, 0.5, 0.25], timestep_seconds: 3600 });'''

G4_SCHEDULES_IN_USE = G4_SCHEDULES + '''
    model.shw_systems.push(crate::model::ShwSystemConfig { id: crate::model::EntityId(400), heater_capacity_w: 4500.0, storage_volume_m3: 0.3, setpoint_c: 55.0, schedule_id: crate::model::ScheduleId(1) });
    model.water_systems.push(crate::model::WaterSystemConfig { id: crate::model::EntityId(430), fixture_count: 6, peak_flow_l_s: 0.25, schedule_id: crate::model::ScheduleId(20) });
    model.refrigeration_systems.push(crate::model::RefrigerationConfig { id: crate::model::EntityId(420), case_count: 4, design_load_w: 12000.0, defrost_schedule_id: crate::model::ScheduleId(30) });
    model.mechanical_ventilations.push(crate::model::MechanicalVentilation { id: crate::model::EntityId(60), zone_id: crate::model::EntityId(1), schedule_id: crate::model::ScheduleId(40), design_flow_m3_s: 0.05, fan_total_efficiency: 0.7, fan_delta_pressure_pa: 300.0 });'''
# endregion 🔖️G4Schedules


# region 🔖️G4ConstantSchedules
g4_schedule_create(
    number=900,
    slug="create-constant-schedule",
    emoji="🕜️",
    group="constants",
    struct="ConstantSchedule",
    noun="Constant schedule",
    fields=[("id", "crate::model::ScheduleId"), ("value", "f64")],
    ctor="id: payload.id, value: payload.value",
    py_row='{"id": payload["id"], "value": payload["value"]}',
    seed=G4_SCHEDULES,
    doc="Defines one schedule that holds the same value at every timestep — the shape a fixed setpoint, a fixed fraction or an always-on availability takes.",
    validate_rs='''
    if !payload.value.is_finite() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Schedule {} needs a finite value, got {}.", payload.id.0, payload.value), [payload.id.0.to_string()]);
    }''',
    validate_refused_rs=" || !payload.value.is_finite()",
    validate_py='''
    if payload["value"] != payload["value"] or payload["value"] in (float("inf"), float("-inf")):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])''',
    happy_args="2, crate::model::ScheduleId(3), 0.75",
    duplicate_args="2, crate::model::ScheduleId(1), 0.75",
    delete_slug="delete-constant-schedule",
)

g4_schedule_delete(
    number=901,
    slug="delete-constant-schedule",
    emoji="📍️",
    group="constants",
    noun="Constant schedule",
    create_slug="create-constant-schedule",
    create_args_rs="existing.id, existing.value",
    create_row_py='"id": item["id"], "value": item["value"]',
    seed=G4_SCHEDULES,
    target_id=2,
    bad_seed=G4_SCHEDULES_IN_USE,
    bad_id=1,
    doc="Removes one constant schedule. Refused while any gain, thermostat, system or other schedule still resolves its id — the document would otherwise carry a reference `Model::validate` reports as severe.",
)

g4_schedule_element(
    number=902,
    slug="change-constant-schedule-value",
    emoji="🕝️",
    group="constants",
    noun="Constant schedule",
    field="value",
    rust_type="f64",
    doc="Sets the one value a constant schedule returns at every timestep.",
    guard_rs='''
    if !payload.new_value.is_finite() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Schedule {} needs a finite value, got {}.", payload.id.0, payload.new_value), [payload.id.0.to_string()]);
    }''',
    guard_py='''
    if value != value or value in (float("inf"), float("-inf")):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])''',
    refuse_rs=" && payload.new_value.is_finite()",
    show="{}",
    seed=G4_SCHEDULES,
    target_id=1,
    happy="0.25",
    bad="crate::model::ScheduleId(99), 0.25",
    bad_seed=G4_SCHEDULES,
    bad_description="refuses a schedule the document does not define",
    outcome_classes=["applied", "warning", "error"],
)
# endregion 🔖️G4ConstantSchedules


# region 🔖️G4DailySchedules
G4_DAILY_VALIDATE_RS = '''
    if payload.hourly_values.len() != 24 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A daily schedule carries twenty-four hourly values, got {}.", payload.hourly_values.len()), [payload.id.0.to_string()]);
    }
    if payload.hourly_values.iter().any(|value| !value.is_finite()) {
        return protocol::MutationOutcome::error("mutation.invariant", "Every hourly schedule value must be finite.", [payload.id.0.to_string()]);
    }
    if payload.limits_min.is_some() != payload.limits_max.is_some() {
        return protocol::MutationOutcome::error("mutation.invariant", "Schedule limits are a lower and an upper bound together, or neither.", [payload.id.0.to_string()]);
    }
    if let (Some(min), Some(max)) = (payload.limits_min, payload.limits_max) {
        if !min.is_finite() || !max.is_finite() || min > max {
            return protocol::MutationOutcome::error("mutation.invariant", format!("Schedule limits {} .. {} are not an interval.", min, max), [payload.id.0.to_string()]);
        }
    }'''

G4_DAILY_REFUSED_RS = " || payload.hourly_values.len() != 24 || payload.hourly_values.iter().any(|value| !value.is_finite()) || payload.limits_min.is_some() != payload.limits_max.is_some() || matches!((payload.limits_min, payload.limits_max), (Some(min), Some(max)) if !min.is_finite() || !max.is_finite() || min > max)"

G4_DAILY_VALIDATE_PY = '''
    if len(payload["hourlyValues"]) != 24:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry != entry or entry in (float("inf"), float("-inf")) for entry in payload["hourlyValues"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if (payload["limitsMin"] is None) != (payload["limitsMax"] is None):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["limitsMin"] is not None and payload["limitsMin"] > payload["limitsMax"]:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])'''

g4_schedule_create(
    number=903,
    slug="create-daily-schedule",
    emoji="🕞️",
    group="daily",
    struct="DailySchedule",
    noun="Daily schedule",
    fields=[("id", "crate::model::ScheduleId"), ("hourly_values", "Vec<f64>"), ("interpolation", "crate::schedule::ScheduleInterpolation"), ("limits_min", "Option<f64>"), ("limits_max", "Option<f64>")],
    ctor="id: payload.id, hourly_values: { let mut values = [0.0f64; 24]; values.copy_from_slice(&payload.hourly_values); values }, interpolation: payload.interpolation, limits: match (payload.limits_min, payload.limits_max) { (Some(min), Some(max)) => Some(crate::schedule::ScheduleLimits { min, max }), _ => None }",
    py_row='{"id": payload["id"], "hourly_values": list(payload["hourlyValues"]), "interpolation": payload["interpolation"], "limits": ({"min": payload["limitsMin"], "max": payload["limitsMax"]} if payload["limitsMin"] is not None else None)}',
    seed=G4_SCHEDULES,
    doc="Defines one twenty-four-hour profile. The optional lower and upper bound are one facet — both together or neither — and the values are clamped to them on every lookup, which is why a half-stated pair is refused rather than half-applied.",
    validate_rs=G4_DAILY_VALIDATE_RS,
    validate_refused_rs=G4_DAILY_REFUSED_RS,
    validate_py=G4_DAILY_VALIDATE_PY,
    happy_args="3, crate::model::ScheduleId(13), vec![21.0; 24], crate::schedule::ScheduleInterpolation::Continuous, None, None",
    duplicate_args="3, crate::model::ScheduleId(10), vec![21.0; 24], crate::schedule::ScheduleInterpolation::Continuous, None, None",
    delete_slug="delete-daily-schedule",
)

g4_schedule_delete(
    number=904,
    slug="delete-daily-schedule",
    emoji="🌓️",
    group="daily",
    noun="Daily schedule",
    create_slug="create-daily-schedule",
    create_args_rs="existing.id, existing.hourly_values.to_vec(), existing.interpolation, existing.limits.map(|limits| limits.min), existing.limits.map(|limits| limits.max)",
    create_row_py='"id": item["id"], "hourlyValues": item["hourly_values"], "interpolation": item["interpolation"], "limitsMin": (item["limits"]["min"] if item["limits"] is not None else None), "limitsMax": (item["limits"]["max"] if item["limits"] is not None else None)',
    seed=G4_SCHEDULES,
    target_id=12,
    bad_seed=G4_SCHEDULES,
    bad_id=10,
    doc="Removes one daily profile. Refused while a weekly or annual schedule — or any consumer — still names it.",
)

g4_schedule_element(
    number=905,
    slug="replace-daily-schedule-hourly-values",
    emoji="🕔️",
    group="daily",
    noun="Daily schedule",
    field="hourly_values",
    rust_type="Vec<f64>",
    doc="Swaps a daily profile's whole twenty-four-value body. The hours are one shape, not twenty-four independent scalars, so they move together.",
    guard_rs='''
    if payload.new_hourly_values.len() != 24 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A daily schedule carries twenty-four hourly values, got {}.", payload.new_hourly_values.len()), [payload.id.0.to_string()]);
    }
    if payload.new_hourly_values.iter().any(|value| !value.is_finite()) {
        return protocol::MutationOutcome::error("mutation.invariant", "Every hourly schedule value must be finite.", [payload.id.0.to_string()]);
    }''',
    guard_py='''
    if len(value) != 24:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry != entry or entry in (float("inf"), float("-inf")) for entry in value):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])''',
    refuse_rs=" && payload.new_hourly_values.len() == 24 && payload.new_hourly_values.iter().all(|value| value.is_finite())",
    show="{:?}",
    seed=G4_SCHEDULES,
    target_id=10,
    happy="vec![22.0; 24]",
    bad="crate::model::ScheduleId(10), vec![22.0; 12]",
    bad_seed=G4_SCHEDULES,
    bad_description="refuses a profile that is not twenty-four hours long",
    outcome_classes=["applied", "warning", "error"],
    compare_rs="{o}.hourly_values.as_slice() == payload.new_hourly_values.as_slice()",
    value_rs="{ let mut values = [0.0f64; 24]; values.copy_from_slice(&payload.new_hourly_values); values }",
    inverse_value_rs="item.hourly_values.to_vec()",
    python_compare='item["hourly_values"] == list(value)',
    python_value="list(value)",
    python_inverse_value='item["hourly_values"]',
)

g4_schedule_element(
    number=906,
    slug="change-daily-schedule-interpolation",
    emoji="🕕️",
    group="daily",
    noun="Daily schedule",
    field="interpolation",
    rust_type="crate::schedule::ScheduleInterpolation",
    doc="Sets whether a daily profile steps between its hourly values or ramps across them.",
    guard_rs="",
    guard_py="",
    refuse_rs="",
    show="{:?}",
    seed=G4_SCHEDULES,
    target_id=10,
    happy="crate::schedule::ScheduleInterpolation::Discrete",
    bad="crate::model::ScheduleId(99), crate::schedule::ScheduleInterpolation::Discrete",
    bad_seed=G4_SCHEDULES,
    bad_description="refuses a schedule the document does not define",
    outcome_classes=["applied", "warning", "error"],
)

kind(
    number=907,
    slug="change-daily-schedule-limits",
    emoji="🕟️",
    verb="change",
    entity="daily-schedule",
    record="ChangedDailyScheduleLimits",
    display="Change Daily Schedule Limits",
    doc="Sets the optional clamp a daily profile's lookups are bounded by. Lower and upper are one inseparable pair — a half-stated pair is refused — and stating neither clears the clamp entirely.",
    fields=[("id", "crate::model::ScheduleId"), ("new_limits_min", "Option<f64>"), ("new_limits_max", "Option<f64>")],
    label='format!("Change daily schedule {} limits", self.id.0)',
    target="vec![self.id.0.to_string()]",
    diff='''    let Some(existing) = base.model.schedules.daily.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_limits_min.is_some() != payload.new_limits_max.is_some() {
        return protocol::MutationOutcome::error("mutation.invariant", "Schedule limits are a lower and an upper bound together, or neither.", [payload.id.0.to_string()]);
    }
    if let (Some(min), Some(max)) = (payload.new_limits_min, payload.new_limits_max) {
        if !min.is_finite() || !max.is_finite() || min > max {
            return protocol::MutationOutcome::error("mutation.invariant", format!("Schedule limits {} .. {} are not an interval.", min, max), [payload.id.0.to_string()]);
        }
    }
    let limits = match (payload.new_limits_min, payload.new_limits_max) {
        (Some(min), Some(max)) => Some(crate::schedule::ScheduleLimits { min, max }),
        _ => None,
    };
    if existing.limits == limits {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Daily schedule {} already carries these limits.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.daily.iter_mut().find(|item| item.id == payload.id) {
        item.limits = limits;
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    let limits = match (payload.new_limits_min, payload.new_limits_max) {
        (Some(min), Some(max)) => Some(crate::schedule::ScheduleLimits { min, max }),
        _ => None,
    };
    if payload.new_limits_min.is_some() != payload.new_limits_max.is_some() || matches!((payload.new_limits_min, payload.new_limits_max), (Some(min), Some(max)) if !min.is_finite() || !max.is_finite() || min > max) {
        return Vec::new();
    }
    match base.model.schedules.daily.iter().find(|item| item.id == payload.id) {
        Some(item) if item.limits != limits => vec![super::change_daily_schedule_limits(payload.id, item.limits.map(|old| old.min), item.limits.map(|old| old.max))],
        _ => Vec::new(),
    }''',
    outcome_classes=["applied", "warning", "error"],
    probe="change_daily_schedule_limits(crate::model::ScheduleId(10), Some(0.0), Some(40.0))",
    python='''    """🚧️ `change-daily-schedule-limits{id,newLimitsMin,newLimitsMax}` — the optional clamp, set or cleared as one pair."""
    item = next((row for row in before["model"]["schedules"]["daily"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if (payload["newLimitsMin"] is None) != (payload["newLimitsMax"] is None):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["newLimitsMin"] is not None and payload["newLimitsMin"] > payload["newLimitsMax"]:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    limits = {"min": payload["newLimitsMin"], "max": payload["newLimitsMax"]} if payload["newLimitsMin"] is not None else None
    if item["limits"] == limits:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["daily"]:
        if row["id"] == payload["id"]:
            row["limits"] = limits
    return after, applied()''',
    python_invert='''    """↩️ The clamp the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["daily"] if row["id"] == payload["id"])
    old = item["limits"]
    return [("change-daily-schedule-limits", {"id": payload["id"], "newLimitsMin": (old["min"] if old is not None else None), "newLimitsMax": (old["max"] if old is not None else None)})]''',
    cases=[
        ("✅️", "applies", "clamps the profile to a stated interval", G4_SCHEDULES + "\n    (snapshot(model), super::change_daily_schedule_limits(crate::model::ScheduleId(10), Some(0.0), Some(40.0)))"),
        ("⛔️", "refuses", "refuses a half-stated pair of limits", G4_SCHEDULES + "\n    (snapshot(model), super::change_daily_schedule_limits(crate::model::ScheduleId(10), Some(5.0), None))"),
    ],
)
# endregion 🔖️G4DailySchedules


# region 🔖️G4WeeklySchedules
g4_schedule_create(
    number=908,
    slug="create-weekly-schedule",
    emoji="🗓️",
    group="weekly",
    struct="WeeklySchedule",
    noun="Weekly schedule",
    fields=[("id", "crate::model::ScheduleId"), ("daily_schedule_ids", "Vec<crate::model::ScheduleId>")],
    ctor="id: payload.id, daily_schedule_ids: { let mut ids = [crate::model::ScheduleId(0); 7]; ids.copy_from_slice(&payload.daily_schedule_ids); ids }",
    py_row='{"id": payload["id"], "daily_schedule_ids": list(payload["dailyScheduleIds"])}',
    seed=G4_SCHEDULES,
    doc="Defines one week as seven daily profiles, Sunday first — the shape `ScheduleSet::weekly_value` indexes by day of week before it reads the hour.",
    validate_rs='''
    if payload.daily_schedule_ids.len() != 7 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A weekly schedule names seven daily profiles, got {}.", payload.daily_schedule_ids.len()), [payload.id.0.to_string()]);
    }
    if let Some(missing) = payload.daily_schedule_ids.iter().find(|candidate| !base.model.schedules.daily.iter().any(|row| row.id == **candidate)) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", missing.0), [missing.0.to_string()]);
    }''',
    validate_refused_rs=" || payload.daily_schedule_ids.len() != 7 || payload.daily_schedule_ids.iter().any(|candidate| !base.model.schedules.daily.iter().any(|row| row.id == *candidate))",
    validate_py='''
    if len(payload["dailyScheduleIds"]) != 7:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    missing = next((candidate for candidate in payload["dailyScheduleIds"] if not any(row["id"] == candidate for row in before["model"]["schedules"]["daily"])), None)
    if missing is not None:
        return unchanged(before), rejected("mutation.target-missing", [str(missing)])''',
    happy_args="1, crate::model::ScheduleId(21), vec![crate::model::ScheduleId(11); 7]",
    duplicate_args="1, crate::model::ScheduleId(20), vec![crate::model::ScheduleId(11); 7]",
    delete_slug="delete-weekly-schedule",
)

g4_schedule_delete(
    number=909,
    slug="delete-weekly-schedule",
    emoji="🕖️",
    group="weekly",
    noun="Weekly schedule",
    create_slug="create-weekly-schedule",
    create_args_rs="existing.id, existing.daily_schedule_ids.to_vec()",
    create_row_py='"id": item["id"], "dailyScheduleIds": item["daily_schedule_ids"]',
    seed=G4_SCHEDULES,
    target_id=20,
    bad_seed=G4_SCHEDULES_IN_USE,
    bad_id=20,
    doc="Removes one weekly schedule. Refused while any consumer still resolves its id.",
)

kind(
    number=910,
    slug="change-weekly-schedule-day",
    emoji="🕗️",
    verb="change",
    entity="weekly-schedule",
    record="ChangedWeeklyScheduleDay",
    display="Change Weekly Schedule Day",
    doc="Points one day of a week at a different daily profile. The address is the pair (schedule id, day index 0–6); a day outside the week and a profile the document does not define are both refused.",
    fields=[("id", "crate::model::ScheduleId"), ("day_index", "u8"), ("new_daily_schedule_id", "crate::model::ScheduleId")],
    label='format!("Change weekly schedule {} day {}", self.id.0, self.day_index)',
    target="vec![self.id.0.to_string()]",
    diff='''    let Some(existing) = base.model.schedules.weekly.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Weekly schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.day_index > 6 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A week has seven days indexed 0 to 6, got {}.", payload.day_index), [payload.id.0.to_string()]);
    }
    if !base.model.schedules.daily.iter().any(|row| row.id == payload.new_daily_schedule_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", payload.new_daily_schedule_id.0), [payload.new_daily_schedule_id.0.to_string()]);
    }
    if existing.daily_schedule_ids[payload.day_index as usize] == payload.new_daily_schedule_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Weekly schedule {} day {} already names that daily profile.", payload.id.0, payload.day_index));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.weekly.iter_mut().find(|item| item.id == payload.id) {
        item.daily_schedule_ids[payload.day_index as usize] = payload.new_daily_schedule_id;
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    if payload.day_index > 6 || !base.model.schedules.daily.iter().any(|row| row.id == payload.new_daily_schedule_id) {
        return Vec::new();
    }
    match base.model.schedules.weekly.iter().find(|item| item.id == payload.id) {
        Some(item) if item.daily_schedule_ids[payload.day_index as usize] != payload.new_daily_schedule_id => vec![super::change_weekly_schedule_day(payload.id, payload.day_index, item.daily_schedule_ids[payload.day_index as usize])],
        _ => Vec::new(),
    }''',
    outcome_classes=["applied", "warning", "error"],
    probe="change_weekly_schedule_day(crate::model::ScheduleId(20), 2, crate::model::ScheduleId(11))",
    python='''    """🕗️ `change-weekly-schedule-day{id,dayIndex,newDailyScheduleId}` — one day of the week re-pointed."""
    item = next((row for row in before["model"]["schedules"]["weekly"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["dayIndex"] > 6:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not any(row["id"] == payload["newDailyScheduleId"] for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newDailyScheduleId"])])
    if item["daily_schedule_ids"][payload["dayIndex"]] == payload["newDailyScheduleId"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["weekly"]:
        if row["id"] == payload["id"]:
            row["daily_schedule_ids"][payload["dayIndex"]] = payload["newDailyScheduleId"]
    return after, applied()''',
    python_invert='''    """↩️ The daily profile that day named in BASE."""
    item = next(row for row in before["model"]["schedules"]["weekly"] if row["id"] == payload["id"])
    return [("change-weekly-schedule-day", {"id": payload["id"], "dayIndex": payload["dayIndex"], "newDailyScheduleId": item["daily_schedule_ids"][payload["dayIndex"]]})]''',
    cases=[
        ("✅️", "applies", "re-points Tuesday at another profile", G4_SCHEDULES + "\n    (snapshot(model), super::change_weekly_schedule_day(crate::model::ScheduleId(20), 2, crate::model::ScheduleId(11)))"),
        ("⛔️", "refuses", "refuses an eighth day of the week", G4_SCHEDULES + "\n    (snapshot(model), super::change_weekly_schedule_day(crate::model::ScheduleId(20), 9, crate::model::ScheduleId(11)))"),
    ],
)
# endregion 🔖️G4WeeklySchedules


# region 🔖️G4AnnualSchedules
g4_schedule_create(
    number=911,
    slug="create-annual-schedule",
    emoji="📚️",
    group="annual",
    struct="AnnualSchedule",
    noun="Annual schedule",
    fields=[("id", "crate::model::ScheduleId"), ("default_daily_schedule_id", "crate::model::ScheduleId"), ("holiday_daily_schedule_id", "Option<crate::model::ScheduleId>")],
    ctor="id: payload.id, rules: Vec::new(), default_daily_schedule_id: payload.default_daily_schedule_id, holiday_daily_schedule_id: payload.holiday_daily_schedule_id, holiday_dates: Vec::new()",
    py_row='{"id": payload["id"], "rules": [], "default_daily_schedule_id": payload["defaultDailyScheduleId"], "holiday_daily_schedule_id": payload["holidayDailyScheduleId"], "holiday_dates": []}',
    seed=G4_SCHEDULES,
    doc="Defines one rule-based year. It starts with no date rules and no holidays — those are ordered and set-like collections of their own, added by `insert-annual-schedule-rule` and `add-annual-schedule-holiday` — so a create states only the two fallbacks every lookup ends at.",
    validate_rs='''
    if !base.model.schedules.daily.iter().any(|row| row.id == payload.default_daily_schedule_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", payload.default_daily_schedule_id.0), [payload.default_daily_schedule_id.0.to_string()]);
    }
    if let Some(holiday) = payload.holiday_daily_schedule_id {
        if !base.model.schedules.daily.iter().any(|row| row.id == holiday) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", holiday.0), [holiday.0.to_string()]);
        }
    }''',
    validate_refused_rs=" || !base.model.schedules.daily.iter().any(|row| row.id == payload.default_daily_schedule_id) || payload.holiday_daily_schedule_id.is_some_and(|holiday| !base.model.schedules.daily.iter().any(|row| row.id == holiday))",
    validate_py='''
    if not any(row["id"] == payload["defaultDailyScheduleId"] for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["defaultDailyScheduleId"])])
    if payload["holidayDailyScheduleId"] is not None and not any(row["id"] == payload["holidayDailyScheduleId"] for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["holidayDailyScheduleId"])])''',
    happy_args="1, crate::model::ScheduleId(31), crate::model::ScheduleId(10), None",
    duplicate_args="1, crate::model::ScheduleId(30), crate::model::ScheduleId(10), None",
    delete_slug="delete-annual-schedule",
)

kind(
    number=912,
    slug="delete-annual-schedule",
    emoji="📕️",
    verb="delete",
    entity="annual-schedule",
    record="DeletedAnnualSchedule",
    display="Delete Annual Schedule",
    doc="Removes one rule-based year. Refused while any consumer still resolves its id. Its inverse is a cascade: the create that re-defines the year, then one insert per date rule in order, then one add per holiday — because the rules and the holidays are collections of their own and the create does not carry them.",
    fields=[("id", "crate::model::ScheduleId")],
    label='format!("Delete annual schedule {}", self.id.0)',
    target="vec![self.id.0.to_string()]",
    diff='''    let Some(existing) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    if ''' + G4_SCHEDULE_IN_USE_RS + ''' {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Schedule {} is still referenced by the document.", payload.id.0), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.schedules.annual.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    let Some(index) = base.model.schedules.annual.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if ''' + G4_SCHEDULE_IN_USE_RS + ''' {
        return Vec::new();
    }
    let existing = &base.model.schedules.annual[index];
    let mut steps = vec![super::create_annual_schedule(index as u32, existing.id, existing.default_daily_schedule_id, existing.holiday_daily_schedule_id)];
    for (position, rule) in existing.rules.iter().enumerate() {
        steps.push(super::insert_annual_schedule_rule(existing.id, position as u32, rule.start_month, rule.start_day, rule.end_month, rule.end_day, rule.daily_schedule_id));
    }
    for (position, holiday) in existing.holiday_dates.iter().enumerate() {
        steps.push(super::add_annual_schedule_holiday(existing.id, position as u32, holiday.0, holiday.1, holiday.2));
    }
    steps''',
    outcome_classes=["applied", "error"],
    probe="delete_annual_schedule(crate::model::ScheduleId(30))",
    python='''    """📕️ `delete-annual-schedule{id}` — the rule-based year, removed whole."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
''' + G4_SCHEDULE_IN_USE_PY + '''
    if in_use:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["annual"] = [row for row in after["model"]["schedules"]["annual"] if row["id"] != payload["id"]]
    return after, applied()''',
    python_invert='''    """↩️ Re-defines the year, then replays its rules in order and its holidays in order."""
    rows = before["model"]["schedules"]["annual"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    steps = [("create-annual-schedule", {"index": index, "id": item["id"], "defaultDailyScheduleId": item["default_daily_schedule_id"], "holidayDailyScheduleId": item["holiday_daily_schedule_id"]})]
    for position, rule in enumerate(item["rules"]):
        steps.append(("insert-annual-schedule-rule", {"id": item["id"], "index": position, "startMonth": rule["start_month"], "startDay": rule["start_day"], "endMonth": rule["end_month"], "endDay": rule["end_day"], "dailyScheduleId": rule["daily_schedule_id"]}))
    for position, holiday in enumerate(item["holiday_dates"]):
        steps.append(("add-annual-schedule-holiday", {"id": item["id"], "index": position, "year": holiday[0], "month": holiday[1], "day": holiday[2]}))
    return steps''',
    cases=[
        ("✅️", "applies", "removes the year and its rules together", G4_SCHEDULES + "\n    (snapshot(model), super::delete_annual_schedule(crate::model::ScheduleId(30)))"),
        ("⛔️", "refuses", "refuses while the document still resolves it", G4_SCHEDULES_IN_USE + "\n    (snapshot(model), super::delete_annual_schedule(crate::model::ScheduleId(30)))"),
    ],
)

kind(
    number=913,
    slug="insert-annual-schedule-rule",
    emoji="📗️",
    verb="insert",
    entity="annual-schedule",
    record="InsertedAnnualScheduleRule",
    display="Insert Annual Schedule Rule",
    doc="Places one date rule at a stated position in a year's ordered rule list. The order is load-bearing, not cosmetic — `ScheduleSet::annual_value` returns the FIRST rule whose date range contains the day — so this is `insert` with a FINAL-state index, not a set-like `add`.",
    fields=[("id", "crate::model::ScheduleId"), ("index", "u32"), ("start_month", "u8"), ("start_day", "u8"), ("end_month", "u8"), ("end_day", "u8"), ("daily_schedule_id", "crate::model::ScheduleId")],
    label='format!("Insert rule at {} of annual schedule {}", self.index, self.id.0)',
    target="vec![self.id.0.to_string()]",
    diff='''    let Some(existing) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.index as usize > existing.rules.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of annual schedule {}'s {} rules.", payload.index, payload.id.0, existing.rules.len()), [payload.id.0.to_string()]);
    }
    if !(1..=12).contains(&payload.start_month) || !(1..=12).contains(&payload.end_month) || !(1..=31).contains(&payload.start_day) || !(1..=31).contains(&payload.end_day) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Rule {}-{} .. {}-{} is not a calendar interval.", payload.start_month, payload.start_day, payload.end_month, payload.end_day), [payload.id.0.to_string()]);
    }
    if !base.model.schedules.daily.iter().any(|row| row.id == payload.daily_schedule_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", payload.daily_schedule_id.0), [payload.daily_schedule_id.0.to_string()]);
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.annual.iter_mut().find(|item| item.id == payload.id) {
        item.rules.insert(payload.index as usize, crate::schedule::CompactScheduleRule { start_month: payload.start_month, start_day: payload.start_day, end_month: payload.end_month, end_day: payload.end_day, daily_schedule_id: payload.daily_schedule_id });
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    let Some(item) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if payload.index as usize > item.rules.len() || !(1..=12).contains(&payload.start_month) || !(1..=12).contains(&payload.end_month) || !(1..=31).contains(&payload.start_day) || !(1..=31).contains(&payload.end_day) || !base.model.schedules.daily.iter().any(|row| row.id == payload.daily_schedule_id) {
        return Vec::new();
    }
    vec![super::remove_annual_schedule_rule(payload.id, payload.index)]''',
    outcome_classes=["applied", "error"],
    probe="insert_annual_schedule_rule(crate::model::ScheduleId(30), 1, 7, 1, 8, 31, crate::model::ScheduleId(12))",
    python='''    """📗️ `insert-annual-schedule-rule{id,index,…}` — one date rule at a FINAL-state position."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["index"] > len(item["rules"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not (1 <= payload["startMonth"] <= 12 and 1 <= payload["endMonth"] <= 12 and 1 <= payload["startDay"] <= 31 and 1 <= payload["endDay"] <= 31):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not any(row["id"] == payload["dailyScheduleId"] for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["dailyScheduleId"])])
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            row["rules"].insert(payload["index"], {"start_month": payload["startMonth"], "start_day": payload["startDay"], "end_month": payload["endMonth"], "end_day": payload["endDay"], "daily_schedule_id": payload["dailyScheduleId"]})
    return after, applied()''',
    python_invert='''    """↩️ The inserted rule is taken back out at the position it was placed."""
    return [("remove-annual-schedule-rule", {"id": payload["id"], "index": payload["index"]})]''',
    cases=[
        ("✅️", "applies", "adds a second-half-year rule", G4_SCHEDULES + "\n    (snapshot(model), super::insert_annual_schedule_rule(crate::model::ScheduleId(30), 1, 7, 1, 8, 31, crate::model::ScheduleId(12)))"),
        ("⛔️", "refuses", "refuses a rule naming an undefined daily profile", G4_SCHEDULES + "\n    (snapshot(model), super::insert_annual_schedule_rule(crate::model::ScheduleId(30), 1, 7, 1, 8, 31, crate::model::ScheduleId(99)))"),
    ],
)

kind(
    number=914,
    slug="remove-annual-schedule-rule",
    emoji="📙️",
    verb="remove",
    entity="annual-schedule",
    record="RemovedAnnualScheduleRule",
    display="Remove Annual Schedule Rule",
    doc="Takes one date rule out of a year's ordered rule list, addressed by its BASE-state index. Every later rule moves up one, and a day the removed rule used to answer falls through to the next matching rule or to the default profile.",
    fields=[("id", "crate::model::ScheduleId"), ("index", "u32")],
    label='format!("Remove rule {} of annual schedule {}", self.index, self.id.0)',
    target="vec![self.id.0.to_string()]",
    diff='''    let Some(existing) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.index as usize >= existing.rules.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} has no rule at index {}.", payload.id.0, payload.index), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.annual.iter_mut().find(|item| item.id == payload.id) {
        item.rules.remove(payload.index as usize);
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    let Some(item) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let Some(rule) = item.rules.get(payload.index as usize) else {
        return Vec::new();
    };
    vec![super::insert_annual_schedule_rule(payload.id, payload.index, rule.start_month, rule.start_day, rule.end_month, rule.end_day, rule.daily_schedule_id)]''',
    outcome_classes=["applied", "error"],
    probe="remove_annual_schedule_rule(crate::model::ScheduleId(30), 0)",
    python='''    """📙️ `remove-annual-schedule-rule{id,index}` — one date rule at a BASE-state position."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["index"] >= len(item["rules"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            del row["rules"][payload["index"]]
    return after, applied()''',
    python_invert='''    """↩️ The removed rule goes back at the index it held."""
    item = next(row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"])
    rule = item["rules"][payload["index"]]
    return [("insert-annual-schedule-rule", {"id": payload["id"], "index": payload["index"], "startMonth": rule["start_month"], "startDay": rule["start_day"], "endMonth": rule["end_month"], "endDay": rule["end_day"], "dailyScheduleId": rule["daily_schedule_id"]})]''',
    cases=[
        ("✅️", "applies", "drops the first-half-year rule", G4_SCHEDULES + "\n    (snapshot(model), super::remove_annual_schedule_rule(crate::model::ScheduleId(30), 0))"),
        ("⛔️", "refuses", "refuses an index the rule list does not reach", G4_SCHEDULES + "\n    (snapshot(model), super::remove_annual_schedule_rule(crate::model::ScheduleId(30), 5))"),
    ],
)

kind(
    number=915,
    slug="reorder-annual-schedule-rules",
    emoji="🗂️",
    verb="reorder",
    entity="annual-schedule",
    record="ReorderedAnnualScheduleRules",
    display="Reorder Annual Schedule Rules",
    doc="Moves one date rule to another position in the year's rule list. This is the one `reorder` the schedule vocabulary carries, and it earns it: precedence between two rules whose date ranges overlap IS their list order.",
    fields=[("id", "crate::model::ScheduleId"), ("from", "u32"), ("to", "u32")],
    label='format!("Reorder annual schedule {} rule {} to {}", self.id.0, self.from, self.to)',
    target="vec![self.id.0.to_string()]",
    diff='''    let Some(existing) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.from as usize >= existing.rules.len() || payload.to as usize >= existing.rules.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Annual schedule {} has {} rules, so {} .. {} is not a move.", payload.id.0, existing.rules.len(), payload.from, payload.to), [payload.id.0.to_string()]);
    }
    if payload.from == payload.to {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Annual schedule {} rule {} is already at that position.", payload.id.0, payload.from));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.annual.iter_mut().find(|item| item.id == payload.id) {
        let rule = item.rules.remove(payload.from as usize);
        item.rules.insert(payload.to as usize, rule);
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    match base.model.schedules.annual.iter().find(|item| item.id == payload.id) {
        Some(item) if payload.from != payload.to && (payload.from as usize) < item.rules.len() && (payload.to as usize) < item.rules.len() => vec![super::reorder_annual_schedule_rules(payload.id, payload.to, payload.from)],
        _ => Vec::new(),
    }''',
    outcome_classes=["applied", "warning", "error"],
    probe="reorder_annual_schedule_rules(crate::model::ScheduleId(30), 0, 1)",
    python='''    """🗂️ `reorder-annual-schedule-rules{id,from,to}` — rule precedence IS list order."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["from"] >= len(item["rules"]) or payload["to"] >= len(item["rules"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["from"] == payload["to"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            rule = row["rules"].pop(payload["from"])
            row["rules"].insert(payload["to"], rule)
    return after, applied()''',
    python_invert='''    """↩️ The same move, back the other way."""
    return [("reorder-annual-schedule-rules", {"id": payload["id"], "from": payload["to"], "to": payload["from"]})]''',
    cases=[
        ("✅️", "applies", "gives the summer rule precedence", G4_SCHEDULES + "\n    (snapshot(model), super::reorder_annual_schedule_rules(crate::model::ScheduleId(30), 0, 1))"),
        ("⛔️", "refuses", "refuses a position the rule list does not reach", G4_SCHEDULES + "\n    (snapshot(model), super::reorder_annual_schedule_rules(crate::model::ScheduleId(30), 5, 0))"),
    ],
)

g4_schedule_element(
    number=916,
    slug="change-annual-schedule-default-daily-schedule",
    emoji="🎌️",
    group="annual",
    noun="Annual schedule",
    field="default_daily_schedule_id",
    rust_type="crate::model::ScheduleId",
    doc="Re-points the profile a year falls back to on every day no rule matches.",
    guard_rs='''
    if !base.model.schedules.daily.iter().any(|row| row.id == payload.new_default_daily_schedule_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", payload.new_default_daily_schedule_id.0), [payload.new_default_daily_schedule_id.0.to_string()]);
    }''',
    guard_py='''
    if not any(row["id"] == value for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])''',
    refuse_rs=" && base.model.schedules.daily.iter().any(|row| row.id == payload.new_default_daily_schedule_id)",
    show="{:?}",
    seed=G4_SCHEDULES,
    target_id=30,
    happy="crate::model::ScheduleId(10)",
    bad="crate::model::ScheduleId(30), crate::model::ScheduleId(99)",
    bad_seed=G4_SCHEDULES,
    bad_description="refuses a daily profile the document does not define",
    outcome_classes=["applied", "warning", "error"],
)

g4_schedule_element(
    number=917,
    slug="change-annual-schedule-holiday-daily-schedule",
    emoji="🎄️",
    group="annual",
    noun="Annual schedule",
    field="holiday_daily_schedule_id",
    rust_type="Option<crate::model::ScheduleId>",
    doc="Re-points — or clears, with a null — the profile a year uses on the dates it holds as holidays.",
    guard_rs='''
    if let Some(holiday) = payload.new_holiday_daily_schedule_id {
        if !base.model.schedules.daily.iter().any(|row| row.id == holiday) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", holiday.0), [holiday.0.to_string()]);
        }
    }''',
    guard_py='''
    if value is not None and not any(row["id"] == value for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])''',
    refuse_rs=" && !payload.new_holiday_daily_schedule_id.is_some_and(|holiday| !base.model.schedules.daily.iter().any(|row| row.id == holiday))",
    show="{:?}",
    seed=G4_SCHEDULES,
    target_id=30,
    happy="Some(crate::model::ScheduleId(11))",
    bad="crate::model::ScheduleId(30), Some(crate::model::ScheduleId(99))",
    bad_seed=G4_SCHEDULES,
    bad_description="refuses a holiday profile the document does not define",
    outcome_classes=["applied", "warning", "error"],
)

kind(
    number=918,
    slug="add-annual-schedule-holiday",
    emoji="🎉️",
    verb="add",
    entity="annual-schedule",
    record="AddedAnnualScheduleHoliday",
    display="Add Annual Schedule Holiday",
    doc="Marks one calendar date as a holiday of a year, so lookups on it take the holiday profile instead of the matching rule. The dates are a set, but a JSON array positionally, so the payload carries the position the date takes.",
    fields=[("id", "crate::model::ScheduleId"), ("index", "u32"), ("year", "u16"), ("month", "u8"), ("day", "u8")],
    label='format!("Add holiday {}-{}-{} to annual schedule {}", self.year, self.month, self.day, self.id.0)',
    target="vec![self.id.0.to_string()]",
    diff='''    let Some(existing) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(1..=12).contains(&payload.month) || !(1..=31).contains(&payload.day) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("{}-{}-{} is not a calendar date.", payload.year, payload.month, payload.day), [payload.id.0.to_string()]);
    }
    if payload.index as usize > existing.holiday_dates.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of annual schedule {}'s {} holidays.", payload.index, payload.id.0, existing.holiday_dates.len()), [payload.id.0.to_string()]);
    }
    if existing.holiday_dates.contains(&(payload.year, payload.month, payload.day)) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Annual schedule {} already holds {}-{}-{} as a holiday.", payload.id.0, payload.year, payload.month, payload.day));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.annual.iter_mut().find(|item| item.id == payload.id) {
        item.holiday_dates.insert(payload.index as usize, (payload.year, payload.month, payload.day));
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    match base.model.schedules.annual.iter().find(|item| item.id == payload.id) {
        Some(item) if (1..=12).contains(&payload.month) && (1..=31).contains(&payload.day) && payload.index as usize <= item.holiday_dates.len() && !item.holiday_dates.contains(&(payload.year, payload.month, payload.day)) => vec![super::remove_annual_schedule_holiday(payload.id, payload.year, payload.month, payload.day)],
        _ => Vec::new(),
    }''',
    outcome_classes=["applied", "warning", "error"],
    probe="add_annual_schedule_holiday(crate::model::ScheduleId(30), 1, 2026, 1, 1)",
    python='''    """🎉️ `add-annual-schedule-holiday{id,index,year,month,day}` — one dated holiday."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not (1 <= payload["month"] <= 12 and 1 <= payload["day"] <= 31):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["index"] > len(item["holiday_dates"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    date = [payload["year"], payload["month"], payload["day"]]
    if date in item["holiday_dates"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            row["holiday_dates"].insert(payload["index"], date)
    return after, applied()''',
    python_invert='''    """↩️ The added holiday is taken back out."""
    return [("remove-annual-schedule-holiday", {"id": payload["id"], "year": payload["year"], "month": payload["month"], "day": payload["day"]})]''',
    cases=[
        ("✅️", "applies", "marks new year's day a holiday", G4_SCHEDULES + "\n    (snapshot(model), super::add_annual_schedule_holiday(crate::model::ScheduleId(30), 1, 2026, 1, 1))"),
        ("⛔️", "refuses", "refuses a thirteenth month", G4_SCHEDULES + "\n    (snapshot(model), super::add_annual_schedule_holiday(crate::model::ScheduleId(30), 1, 2026, 13, 1))"),
    ],
)

kind(
    number=919,
    slug="remove-annual-schedule-holiday",
    emoji="🎊️",
    verb="remove",
    entity="annual-schedule",
    record="RemovedAnnualScheduleHoliday",
    display="Remove Annual Schedule Holiday",
    doc="Takes one calendar date back out of a year's holiday set, so lookups on it fall back to the matching rule again.",
    fields=[("id", "crate::model::ScheduleId"), ("year", "u16"), ("month", "u8"), ("day", "u8")],
    label='format!("Remove holiday {}-{}-{} from annual schedule {}", self.year, self.month, self.day, self.id.0)',
    target="vec![self.id.0.to_string()]",
    diff='''    let Some(existing) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !existing.holiday_dates.contains(&(payload.year, payload.month, payload.day)) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not hold {}-{}-{} as a holiday.", payload.id.0, payload.year, payload.month, payload.day), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.annual.iter_mut().find(|item| item.id == payload.id) {
        item.holiday_dates.retain(|holiday| *holiday != (payload.year, payload.month, payload.day));
    }
    protocol::MutationOutcome::new(SUPER_DIFF(model))''',
    inverse='''    let Some(item) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let Some(position) = item.holiday_dates.iter().position(|holiday| *holiday == (payload.year, payload.month, payload.day)) else {
        return Vec::new();
    };
    vec![super::add_annual_schedule_holiday(payload.id, position as u32, payload.year, payload.month, payload.day)]''',
    outcome_classes=["applied", "error"],
    probe="remove_annual_schedule_holiday(crate::model::ScheduleId(30), 2026, 12, 25)",
    python='''    """🎊️ `remove-annual-schedule-holiday{id,year,month,day}` — one dated holiday, taken back out."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    date = [payload["year"], payload["month"], payload["day"]]
    if date not in item["holiday_dates"]:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            row["holiday_dates"] = [holiday for holiday in row["holiday_dates"] if holiday != date]
    return after, applied()''',
    python_invert='''    """↩️ The holiday goes back at the position it held in BASE."""
    item = next(row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"])
    date = [payload["year"], payload["month"], payload["day"]]
    return [("add-annual-schedule-holiday", {"id": payload["id"], "index": item["holiday_dates"].index(date), "year": payload["year"], "month": payload["month"], "day": payload["day"]})]''',
    cases=[
        ("✅️", "applies", "unmarks christmas day", G4_SCHEDULES + "\n    (snapshot(model), super::remove_annual_schedule_holiday(crate::model::ScheduleId(30), 2026, 12, 25))"),
        ("⛔️", "refuses", "refuses a date the year does not hold", G4_SCHEDULES + "\n    (snapshot(model), super::remove_annual_schedule_holiday(crate::model::ScheduleId(30), 2026, 1, 1))"),
    ],
)
# endregion 🔖️G4AnnualSchedules


# region 🔖️G4TimeSeriesSchedules
g4_schedule_create(
    number=920,
    slug="create-time-series-schedule",
    emoji="🪗️",
    group="time_series",
    struct="TimeSeriesSchedule",
    noun="Time series schedule",
    fields=[("id", "crate::model::ScheduleId"), ("values", "Vec<f64>"), ("timestep_seconds", "u32")],
    ctor="id: payload.id, values: payload.values.clone(), timestep_seconds: payload.timestep_seconds",
    py_row='{"id": payload["id"], "values": list(payload["values"]), "timestep_seconds": payload["timestepSeconds"]}',
    seed=G4_SCHEDULES,
    doc="Defines one externally measured series, indexed by timestep rather than by calendar — the shape a metered profile or a co-simulation trace takes.",
    validate_rs='''
    if payload.values.is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "A time series schedule carries at least one value.", [payload.id.0.to_string()]);
    }
    if payload.values.iter().any(|value| !value.is_finite()) {
        return protocol::MutationOutcome::error("mutation.invariant", "Every time series value must be finite.", [payload.id.0.to_string()]);
    }
    if payload.timestep_seconds == 0 {
        return protocol::MutationOutcome::error("mutation.invariant", "A time series schedule needs a timestep of at least one second.", [payload.id.0.to_string()]);
    }''',
    validate_refused_rs=" || payload.values.is_empty() || payload.values.iter().any(|value| !value.is_finite()) || payload.timestep_seconds == 0",
    validate_py='''
    if not payload["values"]:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry != entry or entry in (float("inf"), float("-inf")) for entry in payload["values"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["timestepSeconds"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])''',
    happy_args="1, crate::model::ScheduleId(41), vec![1.0, 0.9], 900",
    duplicate_args="1, crate::model::ScheduleId(40), vec![1.0, 0.9], 900",
    delete_slug="delete-time-series-schedule",
)

g4_schedule_delete(
    number=921,
    slug="delete-time-series-schedule",
    emoji="🎞️",
    group="time_series",
    noun="Time series schedule",
    create_slug="create-time-series-schedule",
    create_args_rs="existing.id, existing.values.clone(), existing.timestep_seconds",
    create_row_py='"id": item["id"], "values": item["values"], "timestepSeconds": item["timestep_seconds"]',
    seed=G4_SCHEDULES,
    target_id=40,
    bad_seed=G4_SCHEDULES_IN_USE,
    bad_id=40,
    doc="Removes one measured series. Refused while any consumer still resolves its id.",
)

g4_schedule_element(
    number=922,
    slug="replace-time-series-schedule-values",
    emoji="🕘️",
    group="time_series",
    noun="Time series schedule",
    field="values",
    rust_type="Vec<f64>",
    doc="Swaps a measured series' whole body. The samples are one measurement, not independent scalars, so they move together.",
    guard_rs='''
    if payload.new_values.is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "A time series schedule carries at least one value.", [payload.id.0.to_string()]);
    }
    if payload.new_values.iter().any(|value| !value.is_finite()) {
        return protocol::MutationOutcome::error("mutation.invariant", "Every time series value must be finite.", [payload.id.0.to_string()]);
    }''',
    guard_py='''
    if not value:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry != entry or entry in (float("inf"), float("-inf")) for entry in value):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])''',
    refuse_rs=" && !payload.new_values.is_empty() && payload.new_values.iter().all(|value| value.is_finite())",
    show="{:?}",
    seed=G4_SCHEDULES,
    target_id=40,
    happy="vec![1.0, 0.8, 0.6, 0.4]",
    bad="crate::model::ScheduleId(40), Vec::new()",
    bad_seed=G4_SCHEDULES,
    bad_description="refuses an empty series",
    outcome_classes=["applied", "warning", "error"],
    compare_rs="{o}.values == payload.new_values",
    value_rs="payload.new_values.clone()",
    inverse_value_rs="item.values.clone()",
    python_compare='item["values"] == list(value)',
    python_value="list(value)",
    python_inverse_value='item["values"]',
)

g4_schedule_element(
    number=923,
    slug="change-time-series-schedule-timestep",
    emoji="🕙️",
    group="time_series",
    noun="Time series schedule",
    field="timestep_seconds",
    rust_type="u32",
    doc="Sets how many seconds one sample of a measured series covers — what turns its index into a clock.",
    guard_rs='''
    if payload.new_timestep_seconds == 0 {
        return protocol::MutationOutcome::error("mutation.invariant", "A time series schedule needs a timestep of at least one second.", [payload.id.0.to_string()]);
    }''',
    guard_py='''
    if value == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])''',
    refuse_rs=" && payload.new_timestep_seconds != 0",
    show="{}",
    seed=G4_SCHEDULES,
    target_id=40,
    happy="900",
    bad="crate::model::ScheduleId(40), 0",
    bad_seed=G4_SCHEDULES,
    bad_description="refuses a zero-second timestep",
    outcome_classes=["applied", "warning", "error"],
)
# endregion 🔖️G4TimeSeriesSchedules
# endregion 🔖️G4


def write(path: str, text: str) -> None:
    full = os.path.join(ROOT, path)
    os.makedirs(os.path.dirname(full), exist_ok=True)
    with open(full, "w", encoding="utf-8") as handle:
        handle.write(text)


def builder_params(k: Kind) -> str:
    return ", ".join(f"{name}: {ty}" for name, ty in k.fields)


def builder_args(k: Kind) -> str:
    return ", ".join(name for name, _ in k.fields)


# region 🔖️Leaf
def leaf_component(k: Kind) -> str:
    struct_fields = "\n".join(f"    pub {name}: {ty}," for name, ty in k.fields)
    body = "{\n" + struct_fields + "\n}" if k.fields else "{}"
    target_fn = f"\n\n    fn target(&self) -> Vec<String> {{\n        {k.target}\n    }}" if k.target else ""
    builder = f"""/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn {k.module}({builder_params(k)}) -> EnergyModelMutation {{
    EnergyModelMutation::{k.variant}({k.variant} {{ {builder_args(k)} }})
}}""" if k.fields else f"""/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn {k.module}() -> EnergyModelMutation {{
    EnergyModelMutation::{k.variant}({k.variant} {{}})
}}"""
    return f"""//! {k.emoji} Energy model mutation — `{k.variant}`: {k.doc}

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{{FromValue as FromValueDerive, ToValue as ToValueDerive}};

//#region 🔖️Mutation
/// {k.emoji} `{k.slug}` payload. {k.doc}
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "{k.slug}")]
pub struct {k.variant} {body}

{builder}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for {k.variant} {{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {{ verb: "{k.verb}", entity: "{k.entity}", kind: "{k.slug}", record: "{k.record}" }};

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {{
        super::diff::diff(self, base)
    }}

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {{
        super::inverse::inverse(self, base)
    }}

    fn label(&self) -> String {{
        {k.label}
    }}{target_fn}
}}
//#endregion 🔖️Mutation
"""


def leaf_diff(k: Kind) -> str:
    body = k.diff.replace("SUPER_DIFF", D).replace("super::", "vocabulary::")
    imports = ["use crate::artifacts::model::diff::EnergyModelDiff;", "use crate::artifacts::model::EnergyModelSnapshot;"]
    if "EnergyLinkSlotDelta" in body:
        imports.append("use crate::artifacts::model::diff::EnergyLinkSlotDelta;")
    if "vocabulary::" in body:
        imports.append("use crate::artifacts::model::mutations as vocabulary;")
    header = "\n".join(sorted(imports))
    return f"""//! 🔺️ Sparse diff builder for `{k.variant}` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

{header}

//#region 🔖️Diff
pub fn diff(payload: &super::{k.variant}, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {{
{body}
}}
//#endregion 🔖️Diff
"""


def leaf_inverse(k: Kind) -> str:
    body = k.inverse.replace("super::", "vocabulary::")
    return f"""//! ↩️ Inverse for `{k.variant}` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::{k.variant}, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {{
{body}
}}
//#endregion 🔖️Inverse
"""


def leaf_payload_schema(k: Kind) -> str:
    props = {field_camel(name): JSON_TYPES[ty] for name, ty in k.fields}
    doc = {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": k.variant,
        "type": "object",
        "additionalProperties": False,
        "required": [field_camel(name) for name, _ in k.fields],
        "properties": props,
    }
    return json.dumps(doc, ensure_ascii=False, indent=2) + "\n"


def leaf_descriptor(k: Kind) -> str:
    doc = {
        "schemaVersion": 1,
        "owner": f"{MUT}/{k.dir}",
        "semanticKind": k.slug,
        "displayName": k.display,
        "emoji": k.emoji,
        "aggregateVariant": k.variant,
        "payloadSchema": "🧬️.schema.json",
        "textOpcode": k.slug,
        "binaryTag": None,
        "invertibility": "explicit-mutation",
        "diffParticipation": "detect",
        "outcomeClasses": k.outcome_classes,
        "composition": "atomic",
        "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"],
    }
    return json.dumps(doc, ensure_ascii=False, indent=2) + "\n"
# endregion 🔖️Leaf


# region 🔖️Fixtures
def case_component(k: Kind, case) -> str:
    emoji, name, description, scenario = case
    return f"""//! 🧪️ `{k.slug}` fixture — `{emoji}{name}`: {description}.
//!
//! The committed `(before, mutation, after, diff, outcome)` quintet beside this file IS the
//! specification; `scenario` is the typed source it was generated from
//! (`SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test -p semio-s-plugin-energy`), and the eight law
//! assertions below read the committed bytes back, never the scenario.

use crate::artifacts::model::mutations::fixtures::{{self, link, snapshot, zone, Case}};
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

#[allow(unused_variables, unused_mut)]
fn scenario() -> (EnergyModelSnapshot, EnergyModelMutation) {{
{scenario}
}}

fn case() -> Case {{
    Case {{ kind: "{k.slug}", directory: "{k.dir}/🧪️tests/{emoji}{name}", before: BEFORE, after: AFTER, mutation: MUTATION, diff: DIFF, outcome: OUTCOME, scenario }}
}}

#[semio_framework_async_macros::async_test]
async fn writes_the_committed_vector_when_requested() {{
    fixtures::write_when_requested(&case());
}}

#[semio_framework_async_macros::async_test]
async fn forward_reaches_the_committed_after_snapshot() {{
    fixtures::assert_forward(&case());
}}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_committed_before_snapshot() {{
    fixtures::assert_inverse(&case());
}}

#[semio_framework_async_macros::async_test]
async fn committed_documents_are_canonical() {{
    fixtures::assert_canonical(&case());
}}

#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {{
    fixtures::assert_outcome(&case());
}}

#[semio_framework_async_macros::async_test]
async fn produces_the_committed_diff() {{
    fixtures::assert_diff(&case());
}}

#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {{
    fixtures::assert_diff_canonical(&case());
}}

#[semio_framework_async_macros::async_test]
async fn committed_diff_alone_carries_before_to_after() {{
    fixtures::assert_diff_applies(&case());
}}

#[semio_framework_async_macros::async_test]
async fn semantic_descriptor_and_inverse_are_complete() {{
    fixtures::assert_semantics(&case()).await;
}}

#[semio_framework_async_macros::async_test]
async fn inverse_and_absorb_laws_hold() {{
    fixtures::assert_laws(&case()).await;
}}
"""


# endregion 🔖️Fixtures


# region 🔖️FixturesModule
FIXTURES_RS = r'''
//#region 🧰️Fixtures
/// 🧰️ Shared fixture-case machinery: the eight laws every committed `(before, mutation, after,
/// diff, outcome)` vector is held to, plus the `SEMIO_ENERGY_WRITE_FIXTURES=1` generator that
/// materializes those five JSON files from one typed scenario. Every `<kind>/🧪️tests/<case>/🦀️.rs`
/// is a thin declaration over this module, so a new kind's fixture case is data, not code.
#[cfg(test)]
pub mod fixtures {
    use super::EnergyModelMutation;
    use crate::artifacts::model::diff::EnergyModelDiff;
    use crate::artifacts::model::EnergyModelSnapshot;
    use protocol::{Mutation, MutationDiff, SemanticMutation};
    use semio_framework_os_kernel::ToValue;

    /// 🧫️ One committed specification vector and the typed scenario it was generated from.
    pub struct Case {
        pub kind: &'static str,
        pub directory: &'static str,
        pub before: &'static str,
        pub after: &'static str,
        pub mutation: &'static str,
        pub diff: &'static str,
        pub outcome: &'static str,
        pub scenario: fn() -> (EnergyModelSnapshot, EnergyModelMutation),
    }

    /// 🏠️ One conditioned zone with the BESTEST 600 envelope's own air volume.
    pub fn zone(id: u32, name: &str) -> crate::model::Zone {
        crate::model::Zone { id: crate::model::EntityId(id), name: name.to_string(), volume_m3: 129.6, multiplier: 1, conditioned: true, part_of_total_floor_area: true }
    }

    /// 📸️ The persisted snapshot a typed model lives in, both composed child handles minted.
    pub fn snapshot(model: crate::model::Model) -> EnergyModelSnapshot {
        crate::artifacts::model::energy_snapshot_with_state(crate::artifacts::model::ENERGY_MODEL_DOCUMENT_SCHEMA, &model, None)
    }

    /// 🔗️ A head-pinned forward link to another artifact.
    pub fn link(uri: &str, role: &str) -> store::ArtifactLink {
        store::ArtifactLink { target: store::os_io::ArtifactRef::parse_uri(uri).expect("fixture link uri parses"), pin: store::LinkPin::Head, role: role.to_string() }
    }

    //#region 🧰️G1Constructors
    /// 🪑️ One space inside a zone.
    pub fn space(id: u32, name: &str, zone_id: u32) -> crate::model::Space {
        crate::model::Space { id: crate::model::EntityId(id), name: name.to_string(), zone_id: crate::model::EntityId(zone_id), floor_area_m2: 48.0 }
    }

    /// 🧱️ One opaque material layer — the BESTEST 600 lightweight wall's plasterboard.
    pub fn material(id: u32, name: &str) -> crate::model::Material {
        crate::model::Material { id: crate::model::EntityId(id), name: name.to_string(), thickness_m: 0.012, conductivity_w_m_k: 0.16, density_kg_m3: 950.0, specific_heat_j_kg_k: 840.0, thermal_absorptance: 0.9, solar_absorptance: 0.6, visible_absorptance: 0.6 }
    }

    /// 🧱️ One single-layer construction over `material_id`.
    pub fn construction(id: u32, name: &str, material_id: u32) -> crate::model::Construction {
        crate::model::Construction { id: crate::model::EntityId(id), name: name.to_string(), layer_material_ids: vec![crate::model::EntityId(material_id)] }
    }

    /// 📐️ One sun- and wind-exposed 8 m × 2.7 m exterior wall on `zone_id`.
    pub fn surface(id: u32, name: &str, zone_id: u32, construction_id: u32) -> crate::model::Surface {
        crate::model::Surface { id: crate::model::EntityId(id), name: name.to_string(), zone_id: crate::model::EntityId(zone_id), class: crate::model::SurfaceClass::ExteriorWall, vertices_m: vec![[0.0, 0.0, 0.0], [8.0, 0.0, 0.0], [8.0, 0.0, 2.7], [0.0, 0.0, 2.7]], construction_id: crate::model::EntityId(construction_id), outside_boundary_condition: crate::model::OutsideBoundary::OutdoorAir, sun_exposed: true, wind_exposed: true, multiplier: 1 }
    }

    /// 🪟️ One 3 m × 2 m window carrying ANSI/ASHRAE 140 §5.2's own quoted optics.
    pub fn window(id: u32, name: &str, surface_id: u32) -> crate::model::Fenestration {
        crate::model::Fenestration { id: crate::model::EntityId(id), name: name.to_string(), surface_id: crate::model::EntityId(surface_id), u_value_w_m2k: 3.0, shgc: 0.787, vlt: 0.86, area_m2: 6.0, height_m: 2.0, sill_height_m: 0.5, frame_conductance_w_k: 0.0, divider_conductance_w_k: 0.0, overhang_depth_m: 0.0, overhang_offset_m: 0.0, fin_depth_m: 0.0, fin_offset_m: 0.0, glazing_construction_id: None }
    }

    /// 🌳️ One free-standing site shading surface.
    pub fn shading(id: u32, name: &str) -> crate::model::ShadingSurface {
        crate::model::ShadingSurface { id: crate::model::EntityId(id), name: name.to_string(), vertices_m: vec![[0.0, -2.0, 3.0], [8.0, -2.0, 3.0], [8.0, 0.0, 3.0], [0.0, 0.0, 3.0]], transmittance_schedule_id: None }
    }
    //#endregion 🧰️G1Constructors

    fn committed(case: &Case, label: &str, text: &str) -> pack::json::Value {
        pack::json::parse(text).unwrap_or_else(|error| panic!("{}/{}: committed {label} is not valid JSON: {error}", case.kind, case.directory))
    }

    fn decode(case: &Case, label: &str, text: &str) -> EnergyModelSnapshot {
        pack::json::from_json_str(text).unwrap_or_else(|error| panic!("{}/{}: committed {label} does not decode: {error}", case.kind, case.directory))
    }

    fn built(case: &Case) -> protocol::MutationOutcome<EnergyModelDiff> {
        let base = decode(case, "before-snapshot", case.before);
        let mutation: EnergyModelMutation = pack::json::from_json_str(case.mutation).expect("committed mutation payload decodes");
        <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::diff(&mutation, &base)
    }

    fn level_name(level: protocol::Severity) -> &'static str {
        match level {
            protocol::Severity::Info => "info",
            protocol::Severity::Warning => "warning",
            protocol::Severity::Error => "error",
            protocol::Severity::Fatal => "fatal",
        }
    }

    /// 🎯️ The declared outcome document a produced `MutationOutcome` corresponds to: a refusal names
    /// one fault code and the offending address, an application names its ordered message list.
    fn outcome_document(outcome: &protocol::MutationOutcome<EnergyModelDiff>) -> pack::json::Value {
        let rejected = outcome.worst_level().is_some_and(|level| level >= protocol::Severity::Error);
        if rejected {
            let first = outcome.messages().iter().find(|message| message.level >= protocol::Severity::Error).expect("a refusal carries its own message");
            return pack::json::object([
                ("status".to_string(), pack::json::Value::String("rejected".to_string())),
                ("code".to_string(), pack::json::Value::String(first.code.0.clone())),
                ("path".to_string(), pack::json::Value::Array(first.target.iter().map(|entry| pack::json::Value::String(entry.clone())).collect())),
            ]);
        }
        pack::json::object([
            ("status".to_string(), pack::json::Value::String("applied".to_string())),
            (
                "messages".to_string(),
                pack::json::Value::Array(
                    outcome
                        .messages()
                        .iter()
                        .map(|message| pack::json::object([("level".to_string(), pack::json::Value::String(level_name(message.level).to_string())), ("code".to_string(), pack::json::Value::String(message.code.0.clone()))]))
                        .collect(),
                ),
            ),
        ])
    }

    fn write_file(path: std::path::PathBuf, text: &str) {
        std::fs::create_dir_all(path.parent().expect("fixture file has a parent")).expect("fixture directory is writable");
        std::fs::write(path, format!("{text}\n")).expect("fixture file is writable");
    }

    /// 🏗️ Materializes the five committed JSON files from the typed scenario. Off unless
    /// `SEMIO_ENERGY_WRITE_FIXTURES=1`, so an ordinary test run never touches the source tree.
    pub fn write_when_requested(case: &Case) {
        if std::env::var("SEMIO_ENERGY_WRITE_FIXTURES").ok().as_deref() != Some("1") {
            return;
        }
        let (before, mutation) = (case.scenario)();
        let outcome = <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::diff(&mutation, &before);
        let after = MutationDiff::apply(outcome.diff(), &before).expect("the scenario's forward diff applies");
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations").join(case.directory);
        let json = |value: pack::json::Value| pack::json::to_string_pretty(&value);
        write_file(root.join("📸️snapshot/⬅️before/🔣️.json"), &json(pack::json::from_dsl_value(&before.to_value())));
        write_file(root.join("📸️snapshot/➡️after/🔣️.json"), &json(pack::json::from_dsl_value(&after.to_value())));
        write_file(root.join("🦠️mutation/🔣️.json"), &json(pack::json::from_dsl_value(&mutation.to_value())));
        write_file(root.join("🔺️diff/🔣️.json"), &json(pack::json::from_dsl_value(&outcome.diff().to_value())));
        write_file(root.join("🎯️outcome/🔣️.json"), &json(outcome_document(&outcome)));
    }

    /// ▶️ Applying the committed mutation to the committed before-snapshot reproduces the committed
    /// after-snapshot exactly, composed child handles included.
    pub fn assert_forward(case: &Case) {
        let base = decode(case, "before-snapshot", case.before);
        let applied = MutationDiff::apply(built(case).diff(), &base).expect("committed mutation applies to its committed before-snapshot");
        assert_eq!(applied, decode(case, "after-snapshot", case.after), "{}/{}: the applied document is not the committed after-snapshot", case.kind, case.directory);
        assert_eq!((applied.structure.child_id.as_str(), applied.zones.child_id.as_str()), (base.structure.child_id.as_str(), base.zones.child_id.as_str()), "{}/{}: structure and zones must stay one scene", case.kind, case.directory);
    }

    /// ↩️ Applying the committed mutation and then its OWN computed inverse restores the committed
    /// before-snapshot; an inverse step that is itself refused fails here rather than passing quietly.
    pub fn assert_inverse(case: &Case) {
        let base = decode(case, "before-snapshot", case.before);
        let mutation: EnergyModelMutation = pack::json::from_json_str(case.mutation).expect("committed mutation payload decodes");
        let mut snapshot = MutationDiff::apply(built(case).diff(), &base).expect("committed mutation applies");
        for step in <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::inverse(&mutation, &base) {
            let outcome = <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::diff(&step, &snapshot);
            assert!(!outcome.worst_level().is_some_and(|level| level >= protocol::Severity::Error), "{}/{}: an inverse step was itself refused", case.kind, case.directory);
            snapshot = MutationDiff::apply(outcome.diff(), &snapshot).expect("the inverse step applies");
        }
        assert_eq!(snapshot, base, "{}/{}: undoing did not land back on the committed before-snapshot", case.kind, case.directory);
    }

    /// 🔣️ Both committed documents and the committed payload re-encode to themselves.
    pub fn assert_canonical(case: &Case) {
        for (label, text) in [("before", case.before), ("after", case.after)] {
            let decoded = decode(case, label, text);
            let reencoded = pack::json::from_dsl_value(&decoded.to_value());
            assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &committed(case, label, text)), "{}/{}: committed {label} snapshot is not canonical", case.kind, case.directory);
        }
        let mutation: EnergyModelMutation = pack::json::from_json_str(case.mutation).expect("committed mutation payload decodes");
        let reencoded = pack::json::from_dsl_value(&mutation.to_value());
        assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &committed(case, "mutation", case.mutation)), "{}/{}: committed mutation payload is not canonical", case.kind, case.directory);
    }

    /// 🎯️ The diagnostics the implementation raises are the ones the committed outcome declares.
    pub fn assert_outcome(case: &Case) {
        let produced = outcome_document(&built(case));
        let declared = committed(case, "outcome", case.outcome);
        assert!(pack::json::value_eq_ignoring_object_order(&produced, &declared), "{}/{}: produced outcome {produced:?} differs from the committed one {declared:?}", case.kind, case.directory);
    }

    /// 🔺️ The produced delta is the committed delta — which pins WHICH fields the kind may touch,
    /// not merely where the document ended up.
    pub fn assert_diff(case: &Case) {
        let produced = pack::json::from_dsl_value(&built(case).diff().to_value());
        let declared = committed(case, "diff", case.diff);
        assert!(pack::json::value_eq_ignoring_object_order(&produced, &declared), "{}/{}: produced diff {produced:?} differs from the committed 🔺️diff/🔣️.json {declared:?}", case.kind, case.directory);
    }

    /// 🔣️ The committed delta decodes to the real `EnergyModelDiff` and re-encodes unchanged.
    pub fn assert_diff_canonical(case: &Case) {
        let decoded: EnergyModelDiff = pack::json::from_json_str(case.diff).expect("committed diff decodes");
        let reencoded = pack::json::from_dsl_value(&decoded.to_value());
        assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &committed(case, "diff", case.diff)), "{}/{}: committed diff is not canonical", case.kind, case.directory);
    }

    /// 🩹 The committed delta ALONE carries the before-document to the after-document.
    pub fn assert_diff_applies(case: &Case) {
        let decoded: EnergyModelDiff = pack::json::from_json_str(case.diff).expect("committed diff decodes");
        let produced = MutationDiff::apply(&decoded, &decode(case, "before-snapshot", case.before)).expect("committed diff applies to the before-document");
        assert_eq!(produced, decode(case, "after-snapshot", case.after), "{}/{}: the committed diff did not carry before to after", case.kind, case.directory);
    }

    /// 🧭️ The kind's semantic descriptor uses an approved verb and its inverse closes the loop.
    pub async fn assert_semantics(case: &Case) {
        let base = decode(case, "before-snapshot", case.before);
        let mutation: EnergyModelMutation = pack::json::from_json_str(case.mutation).expect("committed mutation payload decodes");
        let descriptor = SemanticMutation::semantics(&mutation);
        assert!(protocol::is_approved_verb(descriptor.verb), "{}: {:?} is not an approved verb", case.kind, descriptor.verb);
        assert_eq!(descriptor.kind, case.kind);
        assert_eq!(<EnergyModelMutation as SemanticMutation<EnergyModelSnapshot>>::kinds().len(), super::KINDS.len());
        assert_inverse(case);
        let _ = base;
    }

    /// ⚖️ The framework's own inverse and diff-absorb laws, run in role against this vector.
    pub async fn assert_laws(case: &Case) {
        let base = decode(case, "before-snapshot", case.before);
        let mutation: EnergyModelMutation = pack::json::from_json_str(case.mutation).expect("committed mutation payload decodes");
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
        let first = built(case).diff().clone();
        let second = built(case).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, first, second).await;
    }

    /// 📝️ Every declared kind survives the text and binary op codecs the `dsl::DslEnum` derive
    /// generates — one loop, no per-kind round-trip test to forget.
    #[semio_framework_async_macros::async_test]
    async fn every_kind_round_trips_through_text_and_binary() {
        for operation in super::wire_probes() {
            store::os_store::test_support::assert_op_line_round_trip(&operation);
            store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        }
    }
}
//#endregion 🧰️Fixtures
'''
# endregion 🔖️FixturesModule


# region 🔖️AggregateTemplate
AGG_TEMPLATE = r'''//! 🧬️ Transparent energy-model semantic mutation aggregate. Every variant is a single-field tuple
//! wrapping a handcrafted `protocol::MutationKind` payload (the `🧬️mutations/<slug>/` leaves);
//! `#[derive(dsl::Mutations)]` generates the `protocol::Mutation`/`protocol::SemanticMutation`
//! dispatch and `#[derive(dsl::DslEnum)]` generates the `dsl::DslVariants` binding that
//! `📝️text/🦀️.rs`'s op codecs are written against — so a new kind costs one enum variant and one
//! re-export line here, never a hand-written match arm, an opcode constant or a tag registry
//! (ticket 26/09/06/ENERGY-PLUGIN-END-TO-END, adopting `📸️remodel`'s architecture).
//!
//! There is deliberately NO whole-document replace in this enum: `📓️derivation-rules.md` rule 6
//! routes file-open / import / load-example through `store::ArtifactStore::reset`, outside history.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️LinkRoles
/// 🌦️ The `weather_link` slot's role name — one constant shared by the bind/unbind pair.
pub const WEATHER_LINK_ROLE: &str = "weather";
/// 🪢️ The `referenced_model` slot's role name — one constant shared by the connect/disconnect pair.
pub const REFERENCED_MODEL_LINK_ROLE: &str = "model";
//#endregion 🔖️LinkRoles

//#region 🔖️Reexports
@@REEXPORTS@@
//#endregion 🔖️Reexports

//#region 🔖️Aggregate
/// 🧬️ Closed semantic mutation vocabulary for an energy model.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = EnergyModelSnapshot, diff = EnergyModelDiff, schema = "energy.model")]
pub enum EnergyModelMutation {
@@VARIANTS@@
}

/// 🏷️ Direct semantic roster exported for the language-neutral test adapter, in aggregate
/// declaration order — which is also the binary ordinal order `dsl::variants_binary` writes.
pub const KINDS: &[&str] = &[
@@KINDS@@
];

/// 🗂️ `(semanticKind, leaf directory name)` for every declared kind — the single place the
/// emoji-carrying directory names are stated in Rust.
pub const DIRECTORIES: &[(&str, &str)] = &[
@@DIRECTORIES@@
];
//#endregion 🔖️Aggregate

//#region 🌉️TestBridge
/// 🔮️ Reports the forward and inverse behavior of one committed language-neutral vector.
pub fn energy_model_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    use semio_framework_os_kernel::ToValue;
    let decode_snapshot = |text: &str| -> Result<EnergyModelSnapshot, String> { pack::json::from_json_str::<EnergyModelSnapshot>(text).map_err(|error| error.to_string()) };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: EnergyModelMutation = pack::json::from_json_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    // 🌉️ `MutationMessage` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`) is a
    // framework-owned type that has not itself gained `ToValue`/`FromValue` — its two call sites
    // here go through the PRE-EXISTING `protocol::to_dsl_value` serde bridge (framework-internal,
    // exempt) and land in `pack::json::Value` via `pack::json::from_dsl_value`.
    let messages_json = protocol::to_dsl_value(forward.messages()).map(|value| pack::json::from_dsl_value(&value)).map_err(|error| error.to_string())?;
    let inverse_messages_json = protocol::to_dsl_value(&inverse_messages).map(|value| pack::json::from_dsl_value(&value)).map_err(|error| error.to_string())?;
    let report = pack::json::object([
        ("base".to_string(), pack::json::from_dsl_value(&base.to_value())),
        ("expectedSnapshot".to_string(), pack::json::from_dsl_value(&expected.to_value())),
        ("snapshot".to_string(), pack::json::from_dsl_value(&applied.to_value())),
        ("diff".to_string(), pack::json::from_dsl_value(&forward.diff().to_value())),
        ("messages".to_string(), messages_json),
        ("inverseSteps".to_string(), pack::json::from_dsl_value(&inverse.to_value())),
        ("inverseSnapshot".to_string(), pack::json::from_dsl_value(&undone.to_value())),
        ("inverseMessages".to_string(), inverse_messages_json),
    ]);
    Ok(pack::json::to_string(&report))
}
//#endregion 🌉️TestBridge

//#region 🧵️WireProbes
/// 🧵️ One representative value per declared variant — the codec round-trip corpus, so wire coverage
/// grows with the enum instead of with a hand-maintained list of tests.
#[cfg(test)]
pub fn wire_probes() -> Vec<EnergyModelMutation> {
    vec![
@@PROBES@@
    ]
}
//#endregion 🧵️WireProbes
@@FIXTURES@@
//#region 🧪️StructuralCorrespondence
#[cfg(test)]
mod structural_correspondence_tests {
    use super::*;
    use protocol::SemanticMutation;

    /// 🧭️ Every declared kind owns a leaf directory whose descriptor, payload schema and behaviour
    /// facets agree with the aggregate enum AND with the subset's language-neutral oracle catalog —
    /// which lives at `✳️any/🔮️oracle/🔣️.json`, never at a flat `🔣️oracle.json`.
    #[test]
    fn direct_owner_descriptors_and_catalog_correspond() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let catalog_source = std::fs::read_to_string(mutation_root.join("../../🔮️oracle/🔣️.json")).expect("language-neutral oracle catalog");
        let catalog: pack::json::Value = pack::json::parse(&catalog_source).expect("language-neutral oracle catalog must be valid JSON");
        let catalog_kinds: Vec<String> = catalog["mutationCatalogs"][0]["kinds"].as_array().expect("catalog kinds").iter().map(|kind| kind.as_str().expect("catalog kind is a string").to_string()).collect();
        let descriptors = EnergyModelMutation::kinds();
        assert_eq!(descriptors.len(), KINDS.len());
        assert_eq!(descriptors.iter().map(|descriptor| descriptor.kind).collect::<Vec<_>>(), KINDS.to_vec());
        assert_eq!(catalog_kinds, KINDS.iter().map(|kind| (*kind).to_string()).collect::<Vec<_>>());
        assert_eq!(DIRECTORIES.len(), KINDS.len());
        for (kind, directory) in DIRECTORIES {
            let owner = mutation_root.join(directory);
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor_source = std::fs::read_to_string(owner.join("🔣️.json")).expect("direct language-neutral descriptor");
            let descriptor: pack::json::Value = pack::json::parse(&descriptor_source).expect("direct descriptor must be valid JSON");
            let payload_schema_source = std::fs::read_to_string(owner.join("🧬️.schema.json")).expect("direct payload schema");
            let payload_schema: pack::json::Value = pack::json::parse(&payload_schema_source).expect("direct payload schema must be valid JSON");
            assert!(source.contains("protocol::MutationKind"), "{kind} owns no MutationKind impl");
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"].as_str(), Some(*kind));
            assert_eq!(descriptor["textOpcode"].as_str(), Some(*kind));
            assert_eq!(descriptor["payloadSchema"].as_str(), Some("🧬️.schema.json"));
            assert_eq!(payload_schema["title"].as_str(), descriptor["aggregateVariant"].as_str());
            assert!(owner.join("🔺️diff/🦀️.rs").exists(), "{kind} owns no diff leaf");
            assert!(owner.join("↩️inverse/🦀️.rs").exists(), "{kind} owns no inverse leaf");
        }
    }
}
//#endregion 🧪️StructuralCorrespondence
'''


PROBE_LITERALS = {
    "rename-model": 'rename_model("Probe".to_string())',
    "change-model-version": 'change_model_version("2".to_string())',
    "update-site": "update_site(52.4, 9.7, 55.0, 1.0, 0.0)",
    "update-ground-temperature": "update_ground_temperature(vec![10.0; 12], vec![9.0; 12], 8.0)",
    "update-run-period": "update_run_period(1, 1, 1, 31, 2026)",
    "replace-airflow-network": "replace_airflow_network(true, vec![1], vec![1], 0, vec![7])",
    "add-output-variable": 'add_output_variable("Zone Mean Air Temperature".to_string(), "ZONE ONE".to_string(), crate::model::OutputReportFrequency::Hourly)',
    "remove-output-variable": 'remove_output_variable("Zone Mean Air Temperature".to_string(), "ZONE ONE".to_string())',
    "bind-weather-file": 'bind_weather_file("hannover!s.stdio.semio@v1/epw".to_string())',
    "unbind-weather-file": "unbind_weather_file()",
    "connect-referenced-model": 'connect_referenced_model("doc-2!s.stdio.semio@v1/model".to_string())',
    "disconnect-referenced-model": "disconnect_referenced_model()",
    "rename-zone": 'rename_zone(crate::model::EntityId(1), "ZONE 1".to_string())',
    "change-zone-volume": "change_zone_volume(crate::model::EntityId(1), 129.6)",
    "change-zone-multiplier": "change_zone_multiplier(crate::model::EntityId(1), 4)",
    "change-zone-conditioned": "change_zone_conditioned(crate::model::EntityId(1), false)",
    "change-zone-floor-area-participation": "change_zone_floor_area_participation(crate::model::EntityId(1), false)",
}


def probe_literal(k: Kind) -> str:
    """🧵 One representative builder call per kind. A kind states its own `probe=` in the spec
    table; `PROBE_LITERALS` above stays as the model-root group's original inline table."""
    return k.probe if k.probe else PROBE_LITERALS[k.slug]


def aggregate_component() -> str:
    text = AGG_TEMPLATE
    text = text.replace("@@REEXPORTS@@", "\n".join(f"pub use super::{k.module}::{{{k.module}, {k.variant}}};" for k in KINDS))
    text = text.replace("@@VARIANTS@@", "\n".join(f"    {k.variant}({k.variant})," for k in KINDS))
    text = text.replace("@@KINDS@@", "\n".join(f'    "{k.slug}",' for k in KINDS))
    text = text.replace("@@DIRECTORIES@@", "\n".join(f'    ("{k.slug}", "{k.dir}"),' for k in KINDS))
    text = text.replace("@@PROBES@@", "\n".join(f"        {probe_literal(k)}," for k in KINDS))
    text = text.replace("@@FIXTURES@@", FIXTURES_RS)
    return text


AGG_TEXT_RS = '''//! 📝️ Energy-model mutation text framing — the handcrafted `OpText`/`OpBinary` pair over the
//! `dsl::DslVariants` binding `#[derive(dsl::DslEnum)]` emits for `EnergyModelMutation`. P6: the
//! derive no longer emits these traits, so they are written once here for the whole aggregate and
//! never per kind (identical shape to `📸️remodel`'s own facet).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this mutation facet.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::model::mutations::EnergyModelMutation;

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for EnergyModelMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(candidate, _)| candidate == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for EnergyModelMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs
'''

AGG_BINARY_RS = '''//! ⚖️ EnergyModel artifact — the binary operation surface (`spr`) and its laws. Tags are the
//! aggregate's own variant ordinals, emitted by `dsl::variants_binary` from the `dsl::DslEnum`
//! derive: there is no hand-maintained tag registry to collide on.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::model::schema::mutations::text::EnergyModelMutation;
use protocol::OpBinary;

/// 📦️ Encodes an `EnergyModelMutation` to its binary state-patch form.
pub fn encode_op(operation: &EnergyModelMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes an `EnergyModelMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<EnergyModelMutation, protocol::ProtocolError> {
    EnergyModelMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn every_kind_round_trips_through_this_codec() {
        for operation in crate::artifacts::model::mutations::wire_probes() {
            let bytes = encode_op(&operation).expect("encode");
            assert_eq!(decode_op(&bytes).expect("decode"), operation);
        }
    }
}
//#endregion 🧪️Tests
'''
# endregion 🔖️AggregateTemplate


# region 🔖️AggregateMirrors
def aggregate_schema_json() -> str:
    defs = {}
    for k in KINDS:
        props = {"mutation": {"const": k.slug}}
        props.update({field_camel(name): JSON_TYPES[ty] for name, ty in k.fields})
        defs[k.variant] = {
            "title": k.variant,
            "type": "object",
            "additionalProperties": False,
            "required": ["mutation"] + [field_camel(name) for name, _ in k.fields],
            "properties": props,
        }
    doc = {
        "$id": "https://semio.tech/schema/s/energy/model/mutation.json",
        "title": "EnergyModelMutation",
        "description": "One handcrafted semantic mutation kind per $defs entry - the tagged union EnergyModelMutation dispatches on.",
        "oneOf": [{"$ref": f"#/$defs/{k.variant}"} for k in KINDS],
        "$defs": defs,
    }
    return json.dumps(doc, ensure_ascii=False, indent=2) + "\n"


def aggregate_ts() -> str:
    blocks = []
    for k in KINDS:
        fields = "\n".join(f"  readonly {field_camel(name)}: {TS_TYPES[ty]};" for name, ty in k.fields)
        body = ("\n" + fields + "\n") if fields else "\n"
        blocks.append(f"/** {k.emoji} `{k.slug}` payload. */\nexport interface {k.variant} {{\n  readonly mutation: \"{k.wire_tag}\";{body}}}")
    union = "\n  | ".join(k.variant for k in KINDS)
    return (
        "/** 🧬️ Energy-model mutation vocabulary — TypeScript twin of `🧬️mutations/🦀️.rs`.\n"
        " *\n"
        " *  `EnergyModelMutation` carries `#[value(tag = \"mutation\", rename_all = \"camelCase\")]`, so the\n"
        " *  wire tag is the camelCase form of the Rust variant name (`renameModel`), never the kebab-case\n"
        " *  `#[dsl(keyword)]` slug used for the directory names and for `SemanticDescriptor.kind`.\n"
        " */\n\n"
        + "\n\n".join(blocks)
        + f"\n\nexport type EnergyModelMutation =\n  | {union};\n"
    )


def aggregate_graphql() -> str:
    blocks = []
    for k in KINDS:
        fields = "\n".join(f"  {field_camel(name)}: {GRAPHQL_TYPES[ty]}" for name, ty in k.fields)
        body = ("\n" + fields + "\n") if fields else "\n  mutation: String!\n"
        blocks.append(f"type {k.variant} {{{body}}}")
    union = "\n  | ".join(k.variant for k in KINDS)
    return "# 🧬️ EnergyModel mutation schema — one type per handcrafted semantic mutation kind.\n\n" + "\n\n".join(blocks) + f"\n\nunion EnergyModelMutation =\n  {union}\n"


def aggregate_proto() -> str:
    blocks = []
    for k in KINDS:
        lines = []
        index = 1
        for name, ty in k.fields:
            lines.append(f"  {PROTO_TYPES[ty]} {name} = {index};")
            index += 1
        body = "\n".join(lines)
        blocks.append(f"message {k.variant} {{\n{body}\n}}" if body else f"message {k.variant} {{}}")
    oneof = "\n".join(f"    {k.variant} {snake(k.slug)} = {k.number};" for k in KINDS)
    return (
        'syntax = "proto3";\npackage semio.s.energy.model.mutation;\n\n'
        "// 🧬️ EnergyModel mutation schema - one message per handcrafted semantic mutation kind.\n"
        "// `oneof` field numbers are allocated by the ticket ledger's per-group ranges\n"
        "// (📓️mutation-tag-ledger.md), never sequentially, so concurrent groups never collide.\n\n"
        "// 📐️ Shared fixed-arity numeric tuple — protobuf has no nested repeated scalar, so every\n"
        "// `Vec<[f64; N]>` payload field (surface and shading polygons) repeats this instead.\n"
        "message Vertex3 {\n  double x = 1;\n  double y = 2;\n  double z = 3;\n}\n\n"
        + "\n\n".join(blocks)
        + f"\n\nmessage EnergyModelMutation {{\n  oneof kind {{\n{oneof}\n  }}\n}}\n"
    )


def kebab(name: str) -> str:
    return name.replace("_", "-")


#: 🔤 The terminal a field type prints as in the handcrafted grammar. Anything absent is a number.
GRAMMAR_TEXT_TYPES = {"String"} | {ty for ty in JSON_TYPES if "enum" in JSON_TYPES[ty]}


def aggregate_grammar() -> str:
    ops = " | ".join(k.slug for k in KINDS)
    rules = []
    for k in KINDS:
        if k.fields:
            args = " ".join(f'"{kebab(name)}" "=" {"TEXT" if ty in GRAMMAR_TEXT_TYPES else "NUMBER"}' for name, ty in k.fields)
            rules.append(f'{k.slug} = "{k.slug}" {args}')
        else:
            rules.append(f'{k.slug} = "{k.slug}"')
    body = "\n".join(rules)
    return (
        "dialect grammar\n"
        "grammar energy.model.mutations\n"
        "extension energy\n"
        "start mutation\n\n"
        'artifact-mark = "energy.model-mutations"\n'
        "mutation = artifact-mark op+\n"
        f"op = {ops}\n"
        f"{body}\n"
    )
# endregion 🔖️AggregateMirrors


# region 🔖️Mounts
def mount_block() -> str:
    one = " " * 32
    two = " " * 36
    lines = [
        f'{one}#[path = "{MOUNT_PREFIX}/🦀️.rs"]',
        f"{one}mod component;",
        f"{one}pub use component::*;",
        f'{one}#[path = "{MOUNT_PREFIX}/💾️binary/🦀️.rs"]',
        f"{one}pub mod binary;",
        f'{one}#[path = "{MOUNT_PREFIX}/📝️text/🦀️.rs"]',
        f"{one}pub mod text;",
    ]
    for k in KINDS:
        lines.append(f'{one}#[path = "."]')
        lines.append(f"{one}pub mod {k.module} {{")
        lines.append(f'{two}#[path = "{MOUNT_PREFIX}/{k.dir}/🦀️.rs"]')
        lines.append(f"{two}mod component;")
        lines.append(f"{two}pub use component::*;")
        lines.append(f'{two}#[path = "{MOUNT_PREFIX}/{k.dir}/🔺️diff/🦀️.rs"]')
        lines.append(f"{two}pub mod diff;")
        lines.append(f'{two}#[path = "{MOUNT_PREFIX}/{k.dir}/↩️inverse/🦀️.rs"]')
        lines.append(f"{two}pub mod inverse;")
        for emoji, name, _description, _scenario in k.cases:
            lines.append(f"{two}#[cfg(test)]")
            lines.append(f'{two}#[path = "{MOUNT_PREFIX}/{k.dir}/🧪️tests/{emoji}{name}/🦀️.rs"]')
            lines.append(f"{two}mod tests_{snake(name)};")
        lines.append(f"{one}}}")
    return "\n".join(lines)


def rewrite_mounts() -> None:
    path = os.path.join(ROOT, CRATE)
    with open(path, encoding="utf-8") as handle:
        source = handle.read()
    opener = '                            #[path = "."]\n                            pub mod mutations {\n'
    start = source.index(opener)
    cursor = start + len(opener)
    depth = 1
    while depth > 0:
        character = source[cursor]
        if character == "{":
            depth += 1
        elif character == "}":
            depth -= 1
        cursor += 1
    end = cursor
    replacement = opener + mount_block() + "\n" + " " * 28 + "}"
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(source[:start] + replacement + source[end:])
# endregion 🔖️Mounts


# region 🔖️OracleCatalog
def vectors():
    for k in KINDS:
        for emoji, name, description, _scenario in k.cases:
            yield k, emoji, name, description


def rewrite_oracle_catalog() -> None:
    path = os.path.join(ROOT, f"{SUBSET}/🔮️oracle/🔣️.json")
    with open(path, encoding="utf-8") as handle:
        document = json.load(handle)
    document["_comment"] = (
        "🧩️ This subset's own contribution. `s.energy.model` persists a typed `Model` document plus two "
        "composed child handles regenerated together from it, so every mutation kind below is a real "
        "field-granular semantic edit — the whole-model `replace-model` this file used to record is GONE "
        "(📓️derivation-rules.md rule 6: whole-document load goes through `ArtifactStore::reset`, outside "
        "history). Semio-NATIVE, no third party. The Python second implementation beside "
        "`../../../🧪️tests/🏛️mutate-energy-model-1/🥒️.feature` covers every kind with a happy and a "
        "refusal vector, so nothing here is UNOBSERVABLE any more."
    )
    catalog_vectors = []
    for k in KINDS:
        catalog_vectors.append(
            {
                "mutationId": k.slug,
                "sourceMutationDirectoryName": k.dir,
                "mutationDirectoryName": k.dir,
                "scenarios": [{"id": f"{k.slug}-{name}", "directoryName": f"{emoji}{name}"} for emoji, name, _d, _s in k.cases],
            }
        )
    document["mutationCatalogs"] = [
        {
            "id": "energy-model-1-any",
            "capability": "energy-model-1-mutate",
            "standardDirectoryName": "🔖️1",
            "subsetDirectoryName": "✳️any",
            "vectors": catalog_vectors,
            "kinds": [k.slug for k in KINDS],
        }
    ]
    document["mutationManifests"] = [
        {
            "schema": "semio.repository-test.mutation-manifest/v2",
            "artifact": "s.energy.model",
            "standard": "1",
            "subset": "any",
            "standardDirectoryName": "🔖️1",
            "subsetDirectoryName": "✳️any",
            "mutations": [
                {
                    "id": k.slug,
                    "capability": "energy-model-1-mutate",
                    "payloadSchema": "🧬️.schema.json",
                    "outcomes": ["applied", "rejected"],
                    "productionDispatch": {"operation": k.slug, "bridgeVersion": 1, "variant": k.variant},
                    "oracleRequirements": [{"capability": "energy-model-1-mutate", "qualifyingKind": "verified-native-second-implementation"}],
                }
                for k in KINDS
            ],
        }
    ]
    oracle = document["oracles"][0]
    oracle["nativeSecondImplementation"]["fixtureCoverage"] = {"vectors": sum(len(k.cases) for k in KINDS), "capabilitiesCovered": ["energy-model-1-mutate"]}
    oracle["nativeSecondImplementation"]["specificationSource"] = (
        ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md; "
        ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️derivation-rules.md; "
        "../🧬️schema/📸️snapshot/🔣️.json; ../🧬️schema/🧬️mutations/🔣️.json; ../🧬️schema/🧬️mutations/<kind>/🧬️.schema.json"
    )
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
# endregion 🔖️OracleCatalog


# region 🔖️Feature
FEATURE_HEAD = '''@capability-energy-model-1-mutate
@oracle-energy-model-1-python-independent
@comparison-ordered-json-v1
@mutations-energy-model-1-any
Feature: Apply every typed s.energy.model mutation against an independent Python implementation

  `s.energy.model` is a semio-NATIVE artifact and no third party reads or writes `.dsl.semio` — the
  recorded survey (kept verbatim in `🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json`'s history)
  named and DECLINED EnergyPlus and OpenStudio, and the `energyplus` weather reader already
  registered under `✏️s/🔌️plugins/🗄️stdio`'s `🌦️epw` subset is deliberately NOT reused here. The
  second producer a differential comparison needs is therefore a second IMPLEMENTATION, and
  `🐍️.py` beside this file is it: written in Python from this subset's own committed
  `🧬️schema/📸️snapshot/🔣️.json`, its per-kind payload schemas, and
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/`'s
  `📓️taxonomy.md` verb table and `📓️derivation-rules.md` shape rules. It imports nothing from the
  Rust it judges and transliterates none of it.

  The honest boundary this feature used to carry is GONE. `replace-model` — a whole-document swap
  the taxonomy bans outright — has been removed from the vocabulary; whole-model load now goes
  through `store::ArtifactStore::reset`, outside history. In its place every kind below carries two
  committed specification vectors, one that really moves the document and one that is really
  refused, so `UNOBSERVABLE` is empty and no kind's coverage is manufactured.

  Both implementations read the SAME committed bytes: the `(before, mutation, after, diff, outcome)`
  quintet under `🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/`, declared as `asset://` fixtures so
  the plan pins their digests and a Python reference can resolve them at all.

  Where the assertions live. `mutate-<id>` and `inverse-<id>` dispatch BOTH an oracle role (the
  Python implementation, reached through this plugin's `oracleHostPackages` entry) and a subject
  role (this repository's own `energy_model_mutation_report_json`), each independently asserting the
  forward/inverse laws in role before the two are compared byte for byte.
  `identity-round-trip` keeps asserting through the shared law module
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs` that the stdio subsets use.
'''


def feature() -> str:
    rows = []
    for k, emoji, name, _description in vectors():
        rows.append(f"      | {k.slug}-{name} | {k.dir} | {emoji}{name} |")
    table = "\n".join(rows)
    mutate = f'''
  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
      | id | dir | fixture |
{table}
'''
    inverse = f'''
  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
      | id | dir | fixture |
{table}
'''
    identity = '''
  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed energy model document, print it back and cross it against its binary encoding
    Given the real committed document asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the document is parsed, printed back to canonical DSL, parsed again, and separately encoded to a pack and decoded back
    Then every decoding agrees on one snapshot, and printing the canonical text a second time reproduces it byte for byte as ArtifactDsl's own fixpoint law requires
'''
    return FEATURE_HEAD + mutate + inverse + identity
# endregion 🔖️Feature


# region 🔖️SubjectAdapter
ADAPTER_PREFIX = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"


def adapter_rs() -> str:
    rows = []
    for k, emoji, name, _description in vectors():
        base = f"{ADAPTER_PREFIX}/{k.dir}/🧪️tests/{emoji}{name}"
        rows.append(
            f'''        Vector {{
            id: "{k.slug}-{name}",
            kind: "{k.slug}",
            before: include_str!("{base}/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("{base}/🦠️mutation/🔣️.json"),
            after: include_str!("{base}/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("{base}/🔺️diff/🔣️.json"),
            outcome: include_str!("{base}/🎯️outcome/🔣️.json"),
        }},'''
        )
    table = "\n".join(rows)
    kinds = "\n".join(f'    "{k.slug}",' for k in KINDS)
    return ADAPTER_TEMPLATE.replace("@@VECTORS@@", table).replace("@@KINDS@@", kinds)


ADAPTER_TEMPLATE = r'''//! 🔋️ `s.energy.model` exhaustive mutation case — Rust adapter. Every declared kind carries two
//! committed specification vectors, one that really moves the document and one that is really
//! refused, so `UNOBSERVABLE` is empty: the weak single-no-op evidence this case used to record
//! disappeared together with the `replace-model` whole-document swap it was about (ticket
//! 26/09/06/ENERGY-PLUGIN-END-TO-END; whole-model load is now `store::ArtifactStore::reset`).
//!
//! **Where the assertions live.** The oracle role is the Python second implementation beside this
//! file, reached through the feature's `@oracle-` tag; the subject role is this repository's own
//! `energy_model_mutation_report_json`. Each asserts the forward and inverse laws in role, through
//! the shared law module `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs`, before the two are compared
//! byte for byte. The subject half is gated behind the generated host's `sut` feature so an
//! oracle-only run never compiles the local implementation.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `KINDS` in `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. That
/// file's own `direct_owner_descriptors_and_catalog_correspond` keeps the list honest against both
/// the enum and the catalog.
const KINDS: &[&str] = &[
@@KINDS@@
];

/// 👁️ Kinds whose COMMITTED specification vector cannot exhibit a forward effect. Empty: every kind
/// owns a vector that moves the document.
const UNOBSERVABLE: &[&str] = &[];

/// 🗣️ The real committed document this artifact ships as its own example.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ One committed `(before, mutation, after, diff, outcome)` specification vector, read literally
/// via `include_str!` — never recomputed here, never restated as a Rust literal.
struct Vector {
    id: &'static str,
    kind: &'static str,
    before: &'static str,
    mutation: &'static str,
    after: &'static str,
    diff: &'static str,
    outcome: &'static str,
}

const VECTORS: &[Vector] = &[
@@VECTORS@@
];

fn vector(id: &str) -> &'static Vector {
    VECTORS.iter().find(|vector| vector.id == id).unwrap_or_else(|| panic!("mutate-energy-model-1: no committed specification vector is registered for {id:?}"))
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("mutate-energy-model-1: a committed fixture must be valid JSON: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed after-snapshot, read literally.
fn mutate_oracle_for(id: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let after = vector(id).after;
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed before-snapshot.
fn inverse_oracle_for(id: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let before = vector(id).before;
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{canonical, vector, DSL_ASSET, UNOBSERVABLE};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_energy::artifacts::model::standards::v1::subsets::any::schema::mutations::energy_model_mutation_report_json;
    use semio_s_plugin_energy::artifacts::model::standards::v1::subsets::any::schema::snapshot::energy_model_identity_report_json;
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️Report
    fn member<'a>(report: &'a Json, key: &str) -> Result<&'a Json, String> {
        report.get(key).ok_or_else(|| format!("the report carries no {key:?} member"))
    }

    fn members(report: &Json, key: &str) -> Result<Vec<Json>, String> {
        match member(report, key)? {
            Json::Array(items) => Ok(items.clone()),
            other => Err(format!("the report's {key:?} member is {}, not an array", other.to_string())),
        }
    }

    fn text(report: &Json, key: &str) -> Result<String, String> {
        match member(report, key)? {
            Json::String(value) => Ok(value.clone()),
            other => Err(format!("the report's {key:?} member is {}, not a string", other.to_string())),
        }
    }

    fn strings(value: &Json, key: &str) -> Vec<String> {
        value
            .array(key)
            .iter()
            .map(|segment| match segment {
                Json::String(text) => text.clone(),
                other => other.to_string(),
            })
            .collect()
    }

    fn level_of(word: &str) -> String {
        if word == "warn" {
            "warning".to_string()
        } else {
            word.to_string()
        }
    }

    /// 🎯️ Checks the produced diagnostics against the ones the committed `🎯️outcome` declares.
    fn declared_outcome_holds(id: &str, produced: &[Json], outcome: &Json) -> Result<(), String> {
        let codes: Vec<String> = produced.iter().map(|message| message.str("code")).collect();
        let levels: Vec<String> = produced.iter().map(|message| level_of(&message.str("level"))).collect();
        if outcome.str("status") == "rejected" {
            let expected = outcome.str("code");
            if codes != vec![expected.clone()] {
                return Err(format!("mutate-{id}: the vector declares a rejection with code {expected:?}, the implementation raised {codes:?}"));
            }
            if !levels.iter().any(|level| level == "error" || level == "fatal") {
                return Err(format!("mutate-{id}: the vector declares a rejection, but the implementation raised it at {levels:?}"));
            }
            let path = strings(outcome, "path");
            let target = strings(&produced[0], "target");
            if !path.is_empty() && target != path {
                return Err(format!("mutate-{id}: the vector declares the offending address {path:?}, the implementation reported {target:?}"));
            }
            return Ok(());
        }
        let expected: Vec<String> = outcome.array("messages").iter().map(|message| message.str("code")).collect();
        if codes != expected {
            return Err(format!("mutate-{id}: the vector declares the diagnostics {expected:?}, the implementation raised {codes:?}"));
        }
        match levels.iter().find(|level| level.as_str() == "error" || level.as_str() == "fatal") {
            Some(level) => Err(format!("mutate-{id}: the vector declares an applied outcome, but the implementation raised a {level}")),
            None => Ok(()),
        }
    }
    //#endregion 🔖️Report

    //#region 🔖️Handlers
    /// 🎯️ Applies the vector and asserts the resulting document, the produced delta and the declared
    /// diagnostics all match what the vector commits to.
    pub fn mutate(id: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let committed = vector(id);
            let report = parse_json(&energy_model_mutation_report_json(committed.before, committed.mutation, committed.after).map_err(|error| format!("mutate-{id}: the committed vector did not reach this subset's own codec: {error}"))?)?;
            let applied = member(&report, "snapshot")?;
            let expected = member(&report, "expectedSnapshot")?;
            if let Some(first) = law::divergence(applied, expected) {
                return Err(format!("mutate-{id}: the applied document is not the committed after-snapshot — {first}"));
            }
            if let Some(first) = law::divergence(member(&report, "diff")?, &canonical(committed.diff)) {
                return Err(format!("mutate-{id}: the produced delta is not the committed 🔺️diff — {first}"));
            }
            declared_outcome_holds(id, &members(&report, "messages")?, &canonical(committed.outcome))?;
            if canonical(committed.outcome).str("status") != "rejected" {
                law::mutation_is_observable(committed.kind, applied, member(&report, "base")?, UNOBSERVABLE)?;
            }
            Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied.clone()))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the vector and then its OWN computed inverse must
    /// restore the committed before-snapshot exactly.
    pub fn inverse(id: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let committed = vector(id);
            let report = parse_json(&energy_model_mutation_report_json(committed.before, committed.mutation, committed.after).map_err(|error| format!("inverse-{id}: the committed vector did not reach this subset's own codec: {error}"))?)?;
            let faults: Vec<String> = members(&report, "inverseMessages")?
                .iter()
                .filter(|message| {
                    let level = message.str("level");
                    level == "error" || level == "fatal"
                })
                .map(|message| message.str("code"))
                .collect();
            if !faults.is_empty() {
                return Err(format!("inverse-{id}: an inverse step was rejected with {faults:?}, so the document never got the chance to return"));
            }
            let restored = member(&report, "inverseSnapshot")?;
            law::inverse_restores(committed.kind, restored, member(&report, "base")?)?;
            Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored.clone()))
        }
    }

    /// 🔁️ The real committed document through this subset's own two codecs.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text_bytes = String::from_utf8(ctx.fixture_bytes(DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed example is not UTF-8: {error}"))?;
        let report = parse_json(&energy_model_identity_report_json(&text_bytes).map_err(|error| format!("identity-round-trip: the committed example did not reach this subset's own codec: {error}"))?)?;
        let parsed = member(&report, "parsed")?;
        law::round_trip_preserves(member(&report, "reparsed")?, parsed)?;
        law::carrier_is_exact(text(&report, "canonicalTextAgain")?.as_bytes(), text(&report, "canonicalText")?.as_bytes())?;
        if let Some(first) = law::divergence(member(&report, "packDecoded")?, parsed) {
            return Err(format!("identity-round-trip: the binary codec decodes to a different document than the text codec — {first}"));
        }
        Ok(Outcome::with_raw(parsed.to_string().into_bytes(), parsed.clone()))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration is by FULL expanded scenario id, so this loop mirrors the feature's `Examples`
/// tables exactly; `identity-round-trip` is subject-only because turning the committed example's DSL
/// bytes into a document needs this subset's own codec.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    debug_assert!(KINDS.iter().all(|kind| VECTORS.iter().any(|vector| vector.kind == *kind)));
    for vector in VECTORS {
        built = built.oracle(&format!("mutate-{}", vector.id), mutate_oracle_for(vector.id)).oracle(&format!("inverse-{}", vector.id), inverse_oracle_for(vector.id));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{}", vector.id), subject::mutate(vector.id)).subject(&format!("inverse-{}", vector.id), subject::inverse(vector.id));
        }
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
'''
# endregion 🔖️SubjectAdapter


# region 🔖️PythonOracle
PYTHON_ORACLE = r'''"""🐍️ `s.energy.model`'s second, independent implementation of its own mutation vocabulary.

No third-party library reads or writes `.dsl.semio` — the recorded survey named and DECLINED
EnergyPlus and OpenStudio, and the `energyplus` weather reader already registered under
`✏️s/🔌️plugins/🗄️stdio`'s `🌦️epw` subset reads a different format for a different purpose, so it is
deliberately not reused here. The reference is therefore a second IMPLEMENTATION, written from this
subset's own committed `../../🧬️schema/📸️snapshot/🔣️.json`, each kind's own
`../../🧬️schema/🧬️mutations/<dir>/🧬️.schema.json`, and
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/`'s `📓️taxonomy.md`
verb table and `📓️derivation-rules.md` shape rules. It imports nothing from the Rust it judges and
transliterates none of it.

The honest boundary this file used to carry is gone with `replace-model`: a whole-document swap is
not a mutation at all (rule 6), so there is no longer a kind whose only vector is a no-op. Every kind
below is exercised by one vector that moves the document and one that is refused.
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome
# endregion 🔖️Imports


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
@@VECTOR_ROOTS@@
}


def _read_json(ctx: Context, uri: str):
    """🧫️ One declared fixture, parsed."""
    return json.loads(ctx.fixture_bytes(uri))


def _vector(ctx: Context, scenario: str):
    root = VECTOR_ROOTS[scenario]
    return (
        _read_json(ctx, f"{root}/📸️snapshot/⬅️before/🔣️.json"),
        _read_json(ctx, f"{root}/🦠️mutation/🔣️.json"),
        _read_json(ctx, f"{root}/📸️snapshot/➡️after/🔣️.json"),
        _read_json(ctx, f"{root}/🎯️outcome/🔣️.json"),
    )
# endregion 🔖️Fixtures


# region 🔖️Wire
def unwrap(wire):
    """📨 Splits the committed mutation document into its kind tag and its argument object. The wire
    tag is the lowerCamel spelling of the Rust variant (`renameModel`), not the kebab catalog id."""
    if isinstance(wire, dict) and isinstance(wire.get("mutation"), str):
        return wire["mutation"], {key: value for key, value in wire.items() if key != "mutation"}
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))


def wire_tag(kind: str) -> str:
    """🔤 The catalog id (kebab) a wire tag maps to, read off the committed manifest's
    `productionDispatch`, never transliterated from a Rust variant name."""
    head, *rest = kind.split("-")
    return head + "".join(part.capitalize() for part in rest)
# endregion 🔖️Wire


# region 🔖️Outcomes
def applied(*messages):
    """🎯️ An applied outcome and its ordered diagnostics."""
    return {"status": "applied", "messages": [{"level": level, "code": code} for level, code in messages]}


def rejected(code, path):
    """⛔️ A refusal: one fault code and the offending address."""
    return {"status": "rejected", "code": code, "path": list(path)}


def unchanged(before):
    """🪞 A refused or no-op step leaves the document exactly where it was."""
    return copy.deepcopy(before)
# endregion 🔖️Outcomes


# region 🔖️Links
def parse_uri(uri):
    """🔗️ `<artifactId>!<artifactKind>@<standard>/<subset>` — the flattened `ArtifactRef` form the
    snapshot schema's link slots carry. Anything else is not a reference."""
    if not isinstance(uri, str) or "!" not in uri:
        return None
    artifact_id, rest = uri.split("!", 1)
    if "@" not in rest or "/" not in rest:
        return None
    artifact_kind, rest = rest.split("@", 1)
    standard, subset = rest.split("/", 1)
    if not artifact_id or not artifact_kind or not standard or not subset or "/" in subset:
        return None
    return {"artifactId": artifact_id, "dialect": {"artifactKind": artifact_kind, "standard": standard, "subset": subset}}


def head_link(target, role):
    """🔗️ A head-pinned link filling one named slot."""
    return {"target": target, "pin": {"kind": "head"}, "role": role}
# endregion 🔖️Links


# region 🔖️Vocabulary
def rename_model(before, payload):
    """🏷️ `rename-model{newName}` — taxonomy.md's `rename` verb on the document identity field."""
    name = payload["newName"]
    if not name.strip():
        return unchanged(before), rejected("mutation.invariant", [name])
    if before["model"]["name"] == name:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["name"] = name
    return after, applied()


def change_model_version(before, payload):
    """🔢️ `change-model-version{newVersion}` — one scalar field."""
    version = payload["newVersion"]
    if not version.strip():
        return unchanged(before), rejected("mutation.invariant", [version])
    if before["model"]["version"] == version:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["version"] = version
    return after, applied()


def update_site(before, payload):
    """🌍️ `update-site` — one inseparable five-field facet, all fields required every time."""
    site = {
        "latitude_deg": payload["latitudeDeg"],
        "longitude_deg": payload["longitudeDeg"],
        "elevation_m": payload["elevationM"],
        "time_zone_hours": payload["timeZoneHours"],
        "north_axis_deg": payload["northAxisDeg"],
    }
    if not -90.0 <= site["latitude_deg"] <= 90.0 or not -180.0 <= site["longitude_deg"] <= 180.0 or not -12.0 <= site["time_zone_hours"] <= 14.0:
        return unchanged(before), rejected("mutation.invariant", [])
    if before["model"]["site"] == site:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["site"] = site
    return after, applied()


def update_ground_temperature(before, payload):
    """🌡️ `update-ground-temperature` — twelve monthly values per series, read together."""
    building = payload["buildingSurfaceC"]
    shallow = payload["shallowC"]
    if len(building) != 12 or len(shallow) != 12:
        return unchanged(before), rejected("mutation.invariant", [])
    ground = {"building_surface_c": building, "shallow_c": shallow, "deep_c": payload["deepC"]}
    if before["model"]["ground_temperature"] == ground:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["ground_temperature"] = ground
    return after, applied()


def update_run_period(before, payload):
    """📅️ `update-run-period` — start and end are one calendar interval."""
    run_period = {
        "start_month": payload["startMonth"],
        "start_day": payload["startDay"],
        "end_month": payload["endMonth"],
        "end_day": payload["endDay"],
        "year": payload["year"],
    }
    months_ok = 1 <= run_period["start_month"] <= 12 and 1 <= run_period["end_month"] <= 12
    days_ok = 1 <= run_period["start_day"] <= 31 and 1 <= run_period["end_day"] <= 31
    if not months_ok or not days_ok:
        return unchanged(before), rejected("mutation.invariant", [])
    if before["model"]["run_period"] == run_period:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["run_period"] = run_period
    return after, applied()


def replace_airflow_network(before, payload):
    """🫧️ `replace-airflow-network` — a whole-value swap of the document-root singleton; `present`
    false detaches it, so one kind covers attach and detach (taxonomy.md's `replace` verb)."""
    zone_ids = payload["zoneIds"]
    node_ids = payload["nodeIds"]
    link_ids = payload["linkIds"]
    if len(zone_ids) != len(node_ids):
        return unchanged(before), rejected("mutation.invariant", [])
    if not payload["present"] and (zone_ids or link_ids):
        return unchanged(before), rejected("mutation.invariant", [])
    network = None
    if payload["present"]:
        network = {"zone_node_ids": [[zone, node] for zone, node in zip(zone_ids, node_ids)], "outdoor_node_id": payload["outdoorNodeId"], "link_ids": link_ids}
    if before["model"]["airflow_network"] == network:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["airflow_network"] = network
    return after, applied()


def add_output_variable(before, payload):
    """📊️ `add-output-variable` — set-like membership keyed by the natural `(name, key)` pair."""
    name, key = payload["name"], payload["key"]
    if not name.strip():
        return unchanged(before), rejected("mutation.invariant", [key])
    if any(spec["name"] == name and spec["key"] == key for spec in before["model"]["output_variables"]):
        return unchanged(before), rejected("mutation.duplicate-id", [name, key])
    after = copy.deepcopy(before)
    after["model"]["output_variables"].append({"name": name, "key": key, "reporting_frequency": payload["reportingFrequency"]})
    return after, applied()


def remove_output_variable(before, payload):
    """📉️ `remove-output-variable` — the `add` verb's inverse partner."""
    name, key = payload["name"], payload["key"]
    if not any(spec["name"] == name and spec["key"] == key for spec in before["model"]["output_variables"]):
        return unchanged(before), rejected("mutation.target-missing", [name, key])
    after = copy.deepcopy(before)
    after["model"]["output_variables"] = [spec for spec in after["model"]["output_variables"] if not (spec["name"] == name and spec["key"] == key)]
    return after, applied()


def bind_weather_file(before, payload):
    """🌦️ `bind-weather-file{targetUri}` — taxonomy.md's `bind` verb attaching a parameterization."""
    target = parse_uri(payload["targetUri"])
    if target is None:
        return unchanged(before), rejected("mutation.invariant", [payload["targetUri"]])
    link = head_link(target, "weather")
    if before.get("weatherLink") == link:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["weatherLink"] = link
    return after, applied()


def unbind_weather_file(before, payload):
    """🌤️ `unbind-weather-file` — `bind`'s inverse partner; refused when nothing is bound."""
    del payload
    if not before.get("weatherLink"):
        return unchanged(before), rejected("mutation.target-missing", [])
    after = copy.deepcopy(before)
    after["weatherLink"] = None
    return after, applied()


def connect_referenced_model(before, payload):
    """🪢️ `connect-referenced-model{targetUri}` — taxonomy.md's `connect` verb on a relationship."""
    target = parse_uri(payload["targetUri"])
    if target is None:
        return unchanged(before), rejected("mutation.invariant", [payload["targetUri"]])
    link = head_link(target, "model")
    if before.get("referencedModel") == link:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["referencedModel"] = link
    return after, applied()


def disconnect_referenced_model(before, payload):
    """✂️ `disconnect-referenced-model` — `connect`'s inverse partner."""
    del payload
    if not before.get("referencedModel"):
        return unchanged(before), rejected("mutation.target-missing", [])
    after = copy.deepcopy(before)
    after["referencedModel"] = None
    return after, applied()


def _zone(before, entity_id):
    for zone in before["model"]["zones"]:
        if zone["id"] == entity_id:
            return zone
    return None


def _with_zone(before, entity_id, field, value):
    after = copy.deepcopy(before)
    for zone in after["model"]["zones"]:
        if zone["id"] == entity_id:
            zone[field] = value
    return after


def rename_zone(before, payload):
    """🏠️ `rename-zone{id,newName}` — id-keyed identity field; a duplicate name is refused because
    every report keys on it."""
    entity_id, name = payload["id"], payload["newName"]
    zone = _zone(before, entity_id)
    if zone is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not name.strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if any(other["id"] != entity_id and other["name"] == name for other in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if zone["name"] == name:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    return _with_zone(before, entity_id, "name", name), applied()


def change_zone_volume(before, payload):
    """📦️ `change-zone-volume{id,newVolumeM3}` — the zone air capacitance."""
    entity_id, volume = payload["id"], payload["newVolumeM3"]
    zone = _zone(before, entity_id)
    if zone is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if volume != volume or volume in (float("inf"), float("-inf")) or volume <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if zone["volume_m3"] == volume:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    return _with_zone(before, entity_id, "volume_m3", volume), applied()


def change_zone_multiplier(before, payload):
    """✖️ `change-zone-multiplier{id,newMultiplier}` — identical zone instances."""
    entity_id, multiplier = payload["id"], payload["newMultiplier"]
    zone = _zone(before, entity_id)
    if zone is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if multiplier == 0:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if zone["multiplier"] == multiplier:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    return _with_zone(before, entity_id, "multiplier", multiplier), applied()


def change_zone_conditioned(before, payload):
    """🌬️ `change-zone-conditioned{id,newConditioned}` — whether equipment serves the zone."""
    entity_id, conditioned = payload["id"], payload["newConditioned"]
    zone = _zone(before, entity_id)
    if zone is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if zone["conditioned"] == conditioned:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    return _with_zone(before, entity_id, "conditioned", conditioned), applied()


def change_zone_floor_area_participation(before, payload):
    """📐️ `change-zone-floor-area-participation{id,newPartOfTotalFloorArea}` — whether the zone's
    floor area counts toward the building total the normalized reports divide by."""
    entity_id, participates = payload["id"], payload["newPartOfTotalFloorArea"]
    zone = _zone(before, entity_id)
    if zone is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if zone["part_of_total_floor_area"] == participates:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    return _with_zone(before, entity_id, "part_of_total_floor_area", participates), applied()


@@EXTRA_VOCABULARY@@
#: 🗺️ Catalog id -> this file's own implementation of that kind.
VOCABULARY = {
    "rename-model": rename_model,
    "change-model-version": change_model_version,
    "update-site": update_site,
    "update-ground-temperature": update_ground_temperature,
    "update-run-period": update_run_period,
    "replace-airflow-network": replace_airflow_network,
    "add-output-variable": add_output_variable,
    "remove-output-variable": remove_output_variable,
    "bind-weather-file": bind_weather_file,
    "unbind-weather-file": unbind_weather_file,
    "connect-referenced-model": connect_referenced_model,
    "disconnect-referenced-model": disconnect_referenced_model,
    "rename-zone": rename_zone,
    "change-zone-volume": change_zone_volume,
    "change-zone-multiplier": change_zone_multiplier,
    "change-zone-conditioned": change_zone_conditioned,
    "change-zone-floor-area-participation": change_zone_floor_area_participation,
@@EXTRA_VOCABULARY_ROWS@@}

#: ↩️ Catalog id -> the undo steps that kind owes, for every kind whose spec row states its own.
EXTRA_INVERT = {
@@EXTRA_INVERT_ROWS@@}
# endregion 🔖️Vocabulary


# region 🔖️Inverse
def invert(kind, before, payload):
    """↩️ The undo steps a kind owes, always read off BASE — never by inverting a delta. A refused or
    no-op forward step owes nothing (taxonomy.md's addressing convention)."""
    after, outcome = VOCABULARY[kind](before, payload)
    if outcome["status"] == "rejected" or after == before:
        return []
    if kind in EXTRA_INVERT:
        return EXTRA_INVERT[kind](before, payload)
    if kind == "rename-model":
        return [("rename-model", {"newName": before["model"]["name"]})]
    if kind == "change-model-version":
        return [("change-model-version", {"newVersion": before["model"]["version"]})]
    if kind == "update-site":
        site = before["model"]["site"]
        return [("update-site", {"latitudeDeg": site["latitude_deg"], "longitudeDeg": site["longitude_deg"], "elevationM": site["elevation_m"], "timeZoneHours": site["time_zone_hours"], "northAxisDeg": site["north_axis_deg"]})]
    if kind == "update-ground-temperature":
        ground = before["model"]["ground_temperature"]
        return [("update-ground-temperature", {"buildingSurfaceC": ground["building_surface_c"], "shallowC": ground["shallow_c"], "deepC": ground["deep_c"]})]
    if kind == "update-run-period":
        run_period = before["model"]["run_period"]
        return [("update-run-period", {"startMonth": run_period["start_month"], "startDay": run_period["start_day"], "endMonth": run_period["end_month"], "endDay": run_period["end_day"], "year": run_period["year"]})]
    if kind == "replace-airflow-network":
        network = before["model"]["airflow_network"]
        if network is None:
            return [("replace-airflow-network", {"present": False, "zoneIds": [], "nodeIds": [], "outdoorNodeId": 0, "linkIds": []})]
        return [("replace-airflow-network", {"present": True, "zoneIds": [pair[0] for pair in network["zone_node_ids"]], "nodeIds": [pair[1] for pair in network["zone_node_ids"]], "outdoorNodeId": network["outdoor_node_id"], "linkIds": network["link_ids"]})]
    if kind == "add-output-variable":
        return [("remove-output-variable", {"name": payload["name"], "key": payload["key"]})]
    if kind == "remove-output-variable":
        spec = next(spec for spec in before["model"]["output_variables"] if spec["name"] == payload["name"] and spec["key"] == payload["key"])
        return [("add-output-variable", {"name": spec["name"], "key": spec["key"], "reportingFrequency": spec["reporting_frequency"]})]
    if kind in ("bind-weather-file", "unbind-weather-file"):
        existing = before.get("weatherLink")
        return [("bind-weather-file", {"targetUri": _uri_of(existing)})] if existing else [("unbind-weather-file", {})]
    if kind in ("connect-referenced-model", "disconnect-referenced-model"):
        existing = before.get("referencedModel")
        return [("connect-referenced-model", {"targetUri": _uri_of(existing)})] if existing else [("disconnect-referenced-model", {})]
    zone = _zone(before, payload["id"])
    if kind == "rename-zone":
        return [("rename-zone", {"id": payload["id"], "newName": zone["name"]})]
    if kind == "change-zone-volume":
        return [("change-zone-volume", {"id": payload["id"], "newVolumeM3": zone["volume_m3"]})]
    if kind == "change-zone-multiplier":
        return [("change-zone-multiplier", {"id": payload["id"], "newMultiplier": zone["multiplier"]})]
    if kind == "change-zone-conditioned":
        return [("change-zone-conditioned", {"id": payload["id"], "newConditioned": zone["conditioned"]})]
    if kind == "change-zone-floor-area-participation":
        return [("change-zone-floor-area-participation", {"id": payload["id"], "newPartOfTotalFloorArea": zone["part_of_total_floor_area"]})]
    raise AssertionError(f"no inverse is written for {kind!r}")


def _uri_of(link):
    """🔗️ Flattens a link's `ArtifactRef` back to the URI form `bind`/`connect` payloads carry."""
    target = link["target"]
    dialect = target["dialect"]
    return f"{target['artifactId']}!{dialect['artifactKind']}@{dialect['standard']}/{dialect['subset']}"
# endregion 🔖️Inverse


# region 🔖️Oracle
def _kind_of(scenario, wire):
    tag, payload = unwrap(wire)
    for kind in VOCABULARY:
        if wire_tag(kind) == tag:
            return kind, payload
    raise AssertionError(f"unexpected wire tag {tag!r} for scenario {scenario!r}")


def _mutate_for(scenario):
    def handler(ctx: Context) -> Outcome:
        before, wire, expected_after, expected_outcome = _vector(ctx, scenario)
        kind, payload = _kind_of(scenario, wire)
        after, outcome = VOCABULARY[kind](before, payload)
        assert after == expected_after, f"mutate-{scenario}: {after} != committed after-snapshot {expected_after}"
        assert outcome == expected_outcome, f"mutate-{scenario}: {outcome} != committed outcome {expected_outcome}"
        return Outcome(projection=after, raw=json.dumps(after, sort_keys=True, separators=(",", ":")).encode("utf-8"))

    return handler


def _inverse_for(scenario):
    def handler(ctx: Context) -> Outcome:
        before, wire, _expected_after, _expected_outcome = _vector(ctx, scenario)
        kind, payload = _kind_of(scenario, wire)
        after, _outcome = VOCABULARY[kind](before, payload)
        restored = after
        for step_kind, step_payload in invert(kind, before, payload):
            restored, step_outcome = VOCABULARY[step_kind](restored, step_payload)
            assert step_outcome["status"] != "rejected", f"inverse-{scenario}: the undo step {step_kind} was itself refused"
        assert restored == before, f"inverse-{scenario}: {restored} != committed before-snapshot {before}"
        return Outcome(projection=restored, raw=json.dumps(restored, sort_keys=True, separators=(",", ":")).encode("utf-8"))

    return handler
# endregion 🔖️Oracle


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, so this mirrors the feature's `Examples`
    tables exactly. Oracle role only: registering these handlers as subjects too would make the
    reference its own subject and manufacture a guaranteed-green self-comparison."""
    built = Adapter("python")
    for scenario in VECTOR_ROOTS:
        built = built.oracle(f"mutate-{scenario}", _mutate_for(scenario)).oracle(f"inverse-{scenario}", _inverse_for(scenario))
    return built
# endregion 🔖️Registration
'''


def python_extras() -> tuple[str, str, str]:
    """🐍️ The second implementation's per-kind parts, for every kind whose spec row carries its own
    `python=` body (and `python_invert=` undo body). A group adds rows, never a hand edit here."""
    functions, rows, invert_rows = [], [], []
    for k in KINDS:
        if not k.python:
            continue
        functions.append(f"def {k.module}(before, payload):\n{k.python}\n")
        rows.append(f'    "{k.slug}": {k.module},')
        if k.python_invert:
            functions.append(f"def _invert_{k.module}(before, payload):\n{k.python_invert}\n")
            invert_rows.append(f'    "{k.slug}": _invert_{k.module},')
    joined = "\n\n".join(functions)
    return (joined + "\n\n" if joined else ""), ("\n".join(rows) + "\n" if rows else ""), ("\n".join(invert_rows) + "\n" if invert_rows else "")


def python_oracle() -> str:
    roots = "\n".join(f'    "{k.slug}-{name}": "asset://🧬️schema/🧬️mutations/{k.dir}/🧪️tests/{emoji}{name}",' for k, emoji, name, _d in vectors())
    vocabulary, rows, invert_rows = python_extras()
    text = PYTHON_ORACLE.replace("@@VECTOR_ROOTS@@", roots)
    text = text.replace("@@EXTRA_VOCABULARY@@\n", vocabulary)
    text = text.replace("@@EXTRA_VOCABULARY_ROWS@@", rows)
    text = text.replace("@@EXTRA_INVERT_ROWS@@", invert_rows)
    return text
# endregion 🔖️PythonOracle


# region 🔖️Main
TAXONOMY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"

LEDGER_RANGES = [
    (1, 99, "model root"),
    (100, 199, "zones & spaces"),
    (200, 299, "geometry & envelope"),
    (300, 399, "constructions & materials"),
    (400, 499, "internal gains"),
    (500, 599, "HVAC zone-level"),
    (600, 699, "HVAC loop-level & plant"),
    (700, 799, "electrical, renewables & other systems"),
    (800, 899, "grouping collections"),
    (900, 999, "schedules"),
]

# \U0001f5a5\ufe0f Windows MAX_PATH headroom, measured against the repository as it actually is rather
# than against `26/04/08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS`'s one-off `\U0001fa9f\ufe0fshorten-long-paths.ts`,
# whose 190-unit target ceiling is NOT a live gate: 1616 tracked paths already exceed it and the
# repository maximum is 227 (\U0001f9e9\ufe0fpuzzle's OWN mutation fixtures reach 225). The rule enforced
# here is therefore "never become the repository's new worst path" -- which still refuses the
# forty-character case name a group would otherwise invent, while not failing this vocabulary's own
# long slugs (nothing under `\U0001f4d0\ufe0fchange-zone-floor-area-participation` can fit in 190 units,
# no matter how short its case name is).
PATH_UTF16_BUDGET = 227

# \U0001f4c1 The five committed fixture documents every case directory owns, seeded empty by `main`
# and materialized by the crate's `SEMIO_ENERGY_WRITE_FIXTURES=1` pass. Also what the path budget is
# measured against, since the sweep measures a case directory by its longest member.
CASE_FIXTURE_FILES = [
    "\U0001f4f8\ufe0fsnapshot/\u2b05\ufe0fbefore/\U0001f523\ufe0f.json",
    "\U0001f4f8\ufe0fsnapshot/\u27a1\ufe0fafter/\U0001f523\ufe0f.json",
    "\U0001f9a0\ufe0fmutation/\U0001f523\ufe0f.json",
    "\U0001f53a\ufe0fdiff/\U0001f523\ufe0f.json",
    "\U0001f3af\ufe0foutcome/\U0001f523\ufe0f.json",
]


# \U0001f9ca\ufe0f `\U0001f4cb\ufe0fcontract-freeze.md` \u00a7C2's SEVEN frozen message codes, enforced repo-wide by
# `bun nx run mutation-outcome-law` (`\U0001f4dc\ufe0fscript.ts`'s `POLICY_MUTATION_FROZEN_CODES`). There is no
# per-plugin code, ever: a refusal's specificity belongs in its MESSAGE, not in a private code. The
# gate reads the GENERATED `\U0001f53a\ufe0fdiff/\U0001f980\ufe0f.rs` files, so a bad code in a spec row is only
# reported minutes later and in someone else's run -- checked here instead, at the row.
FROZEN_MESSAGE_CODES = {
    "mutation.target-missing",
    "mutation.no-op",
    "mutation.partial",
    "mutation.clamped",
    "mutation.duplicate-id",
    "mutation.invariant",
    "mutation.cascade",
}

MESSAGE_CODE_CALL = re.compile(r'(?:MutationOutcome::(?:error|fatal)|MutationMessage::(?:info|warn|error|fatal)|\.(?:info|warn))\s*\(\s*"([^"]*)"')


def utf16_units(text: str) -> int:
    """\U0001f9f5 The unit the repo-wide path shortener counts in, which is not `len` for astral emoji."""
    return len(text.encode("utf-16-le")) // 2


def mutation_directory_pattern():
    """\U0001f9ed The one authority on a mutation directory's name, read from `taxonomy.json` itself
    rather than copied: `\U0001f50d\ufe0fdiscovery/\U0001f7e6\ufe0f.ts:2783` tests exactly this
    against every sibling of `\U0001f9ec\ufe0fmutations/`, and a directory that fails it is INVISIBLE
    to every taxonomy tool instead of being reported as broken."""
    with open(os.path.join(ROOT, TAXONOMY), encoding="utf-8") as handle:
        source = json.load(handle)["mutationDirectoryPattern"]
    return re.compile(source)


def audit() -> None:
    """\U0001f6a8\ufe0f Fails loudly on everything four concurrently-appending group workers can get
    wrong, before a single file is written: two kinds on one ledger number (the protobuf field number
    must be unique forever), two kinds on one emoji (`pathEmojiPolicy.siblingNamespace` is
    `files-and-directories` inside `\U0001f9ec\ufe0fmutations/`), a directory name `taxonomy.json`'s
    own `mutationDirectoryPattern` rejects, a ledger number outside every range the ledger declares,
    a fixture pair that is not one happy plus one refusal, and a case path over the budget at which
    `26/04/08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS`' repo-wide shortener silently renames the directory
    without fixing the `#[path]` literal that names it."""
    taken_emoji = {"\U0001f4be\ufe0f", "\U0001f4d6\ufe0f", "\U0001f4dd\ufe0f", "\U0001f517\ufe0f", "\U0001f523\ufe0f", "\U0001f6f0\ufe0f", "\U0001f7e6\ufe0f", "\U0001f980\ufe0f"}
    pattern = mutation_directory_pattern()
    seen_numbers, seen_emoji, seen_slugs = {}, {}, {}
    for k in KINDS:
        assert k.emoji not in taken_emoji, f"{k.slug}: {k.emoji} is a non-kind sibling inside \U0001f9ec\ufe0fmutations/"
        assert k.emoji.endswith("\ufe0f") and not k.emoji.endswith("\ufe0f\ufe0f"), f"{k.slug}: {k.emoji!r} needs exactly one trailing U+FE0F"
        assert unicodedata.normalize("NFC", k.dir) == k.dir, f"{k.slug}: directory name is not NFC"
        assert pattern.fullmatch(k.dir), f"{k.slug}: directory {k.dir!r} does not match taxonomy.json's mutationDirectoryPattern {pattern.pattern!r}"
        assert any(low <= k.number <= high for low, high, _ in LEDGER_RANGES), f"{k.slug}: ledger number {k.number} is outside every range in the ledger's \u00a72"
        for table, key, label in ((seen_numbers, k.number, "ledger number"), (seen_emoji, k.emoji, "emoji"), (seen_slugs, k.slug, "slug")):
            assert key not in table, f"{label} {key!r} is claimed by both {table[key]} and {k.slug}"
            table[key] = k.slug
        assert camel(k.slug) == k.variant, f"{k.slug}: dsl::Mutations requires kebab({k.variant}) == {k.slug}"
        case_emoji = [emoji for emoji, _n, _d, _s in k.cases]
        assert "\u2705\ufe0f" in case_emoji and "\u26d4\ufe0f" in case_emoji, f"{k.slug}: needs at least one \u2705\ufe0f happy and one \u26d4\ufe0f refusal fixture, got {case_emoji}"
        assert len({f"{emoji}{name}" for emoji, name, _d, _s in k.cases}) == len(k.cases), f"{k.slug}: two fixture cases share one directory name"
        for body, label in ((k.diff, "diff"), (k.inverse, "inverse")):
            for code in MESSAGE_CODE_CALL.findall(body or ""):
                assert code in FROZEN_MESSAGE_CODES, f"{k.slug}: {label} reports message code {code!r}, which is not one of the seven frozen codes ({sorted(FROZEN_MESSAGE_CODES)}) \u2014 `mutation-outcome-law` refuses it repo-wide"
        assert any(code in (k.diff or "") for code in FROZEN_MESSAGE_CODES), f"{k.slug}: diff returns a MutationOutcome but never reports one of the seven frozen codes \u2014 every verb family owes a real refusal/warning detection"
        for emoji, name, _description, _scenario in k.cases:
            path = max((f"{MUT}/{k.dir}/\U0001f9ea\ufe0ftests/{emoji}{name}/{leaf}" for leaf in CASE_FIXTURE_FILES + ["\U0001f980\ufe0f.rs"]), key=utf16_units)
            assert utf16_units(path) <= PATH_UTF16_BUDGET, f"{k.slug}/{emoji}{name}: {utf16_units(path)} UTF-16 units exceeds the {PATH_UTF16_BUDGET}-unit path budget"


LEDGER = ".\U0001f9ecsemio/\U0001f991\ufe0frepo/\U0001f3ab\ufe0ftickets/\U0001f386\ufe0f26/\U0001f319\ufe0f09/\u2600\ufe0f06/ENERGY-PLUGIN-END-TO-END/\U0001f4d3\ufe0fmutation-tag-ledger.md"

LEDGER_ROW = re.compile(r"^\|\s*(\d+)\s*\|\s*`([a-z0-9-]+)`\s*\|\s*(\S+)\s*\|\s*(.*?)\s*\|\s*$", re.M)


def report_ledger_drift() -> None:
    """\U0001f4d3\ufe0f The ledger is the ALLOCATION authority and the spec table is what actually ships, so
    the two drifting apart is how one group picks an emoji another already took \u2014 which happened
    live this ticket (`\U0001f4cc\ufe0f` claimed by both `create-setpoint-manager` and
    `create-constant-schedule`). Reported, never enforced: a stale ledger row is a bookkeeping debt
    owed by the group that landed the kind, and blocking every other group's generator run on it
    would be a worse trade than telling them exactly which line to fix."""
    path = os.path.join(ROOT, LEDGER)
    if not os.path.exists(path):
        print(f"NOTE: {LEDGER} is missing \u2014 ledger drift not checked")
        return
    rows = {int(number): (slug, emoji, status) for number, slug, emoji, status in LEDGER_ROW.findall(open(path, encoding="utf-8").read())}
    drift = []
    for k in KINDS:
        row = rows.get(k.number)
        if row is None:
            drift.append(f"{k.number} {k.slug}: no ledger row at all")
        elif row[0] != k.slug:
            drift.append(f"{k.number} {k.slug}: the ledger gives that number to {row[0]!r}")
        elif row[1] != k.emoji:
            drift.append(f"{k.number} {k.slug}: ledger says {row[1]}, spec table says {k.emoji}")
        elif "reserved" in row[2].lower():
            drift.append(f"{k.number} {k.slug}: landed, but the ledger still says {row[2]!r}")
    for line in drift[:40]:
        print(f"NOTE ledger drift: {line}")
    if len(drift) > 40:
        print(f"NOTE ledger drift: \u2026 and {len(drift) - 40} more")
    if drift:
        print(f"NOTE: {len(drift)} of {len(KINDS)} kinds have a stale or missing row in {LEDGER} \u2014 the group that landed each one owes that line")


def prune_stale_case_directories() -> None:
    """\U0001f9f9\ufe0f Removes fixture case directories a spec row no longer declares, so re-running after a
    group renames a case leaves no orphan `\U0001f9ea\ufe0ftests/` directory behind — an orphan is invisible
    to the aggregate's `DIRECTORIES` table but still a tracked path the taxonomy walker reports.
    Kind directories themselves are only REPORTED, never deleted: a directory with no spec row is
    either another group's half-landed work or a real rename, and guessing between them by deleting
    is how one worker destroys another's in-flight files."""
    import shutil

    declared = {k.dir: {f"{emoji}{name}" for emoji, name, _d, _s in k.cases} for k in KINDS}
    root = os.path.join(ROOT, MUT)
    if not os.path.isdir(root):
        return
    for entry in sorted(os.listdir(root)):
        full = os.path.join(root, entry)
        if not os.path.isdir(full) or entry in {"\U0001f4be\ufe0fbinary", "\U0001f4dd\ufe0ftext"}:
            continue
        if entry not in declared:
            print(f"NOTE: {MUT}/{entry} has no spec row (another group's in-flight kind, or a stale rename) \u2014 left untouched")
            continue
        tests = os.path.join(full, "\U0001f9ea\ufe0ftests")
        if not os.path.isdir(tests):
            continue
        for case in sorted(os.listdir(tests)):
            if os.path.isdir(os.path.join(tests, case)) and case not in declared[entry]:
                shutil.rmtree(os.path.join(tests, case))
                print(f"removed stale fixture case {entry}/{case}")


def main() -> None:
    import shutil

    audit()

    old = os.path.join(ROOT, MUT, "♻️replace-model")
    if os.path.isdir(old):
        shutil.rmtree(old)
        print("removed ♻️replace-model")

    prune_stale_case_directories()
    report_ledger_drift()

    for k in KINDS:
        write(f"{MUT}/{k.dir}/🦀️.rs", leaf_component(k))
        write(f"{MUT}/{k.dir}/🔺️diff/🦀️.rs", leaf_diff(k))
        write(f"{MUT}/{k.dir}/↩️inverse/🦀️.rs", leaf_inverse(k))
        write(f"{MUT}/{k.dir}/🔣️.json", leaf_descriptor(k))
        write(f"{MUT}/{k.dir}/🧬️.schema.json", leaf_payload_schema(k))
        for emoji, name, description, scenario in k.cases:
            case_dir = f"{MUT}/{k.dir}/🧪️tests/{emoji}{name}"
            write(f"{case_dir}/🦀️.rs", case_component(k, (emoji, name, description, scenario)))
            for relative in CASE_FIXTURE_FILES:
                target = os.path.join(ROOT, case_dir, relative)
                if not os.path.exists(target):
                    write(f"{case_dir}/{relative}", "{}\n")

    write(f"{MUT}/🦀️.rs", aggregate_component())
    write(f"{MUT}/📝️text/🦀️.rs", AGG_TEXT_RS)
    write(f"{MUT}/💾️binary/🦀️.rs", AGG_BINARY_RS)
    write(f"{MUT}/🔣️.json", aggregate_schema_json())
    write(f"{MUT}/🟦️.ts", aggregate_ts())
    write(f"{MUT}/🔗️.graphql", aggregate_graphql())
    write(f"{MUT}/🛰️.proto", aggregate_proto())
    write(f"{MUT}/📖️.grammar.semio", aggregate_grammar())
    write(f"{MUT}/📝️text/📖️.grammar.semio", aggregate_grammar())

    rewrite_mounts()
    rewrite_oracle_catalog()
    write(f"{SUBSET}/🧪️tests/🏛️mutate-energy-model-1/🥒️.feature", feature())
    write(f"{SUBSET}/🧪️tests/🏛️mutate-energy-model-1/🦀️.rs", adapter_rs())
    write(f"{SUBSET}/🧪️tests/🏛️mutate-energy-model-1/🐍️.py", python_oracle())

    print(f"{len(KINDS)} kinds, {sum(len(k.cases) for k in KINDS)} fixture cases emitted")


if __name__ == "__main__":
    main()
# endregion 🔖️Main
