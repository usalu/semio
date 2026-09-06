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
        return protocol::MutationOutcome::error("mutation.invalid-payload", format!("Ground temperatures need twelve monthly values each, got {} building-surface and {} shallow.", payload.building_surface_c.len(), payload.shallow_c.len()), Vec::<String>::new());
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
        return protocol::MutationOutcome::error("mutation.invalid-payload", format!("An airflow network pairs one node id per zone id, got {} zone ids and {} node ids.", payload.zone_ids.len(), payload.node_ids.len()), Vec::<String>::new());
    }
    if !payload.present && !(payload.zone_ids.is_empty() && payload.link_ids.is_empty()) {
        return protocol::MutationOutcome::error("mutation.invalid-payload", "A detached airflow network carries no zone nodes and no links.", Vec::<String>::new());
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
        return protocol::MutationOutcome::error("mutation.duplicate", format!("Output variable \\"{}\\" is already registered for \\"{}\\".", payload.name, payload.key), [payload.name.clone(), payload.key.clone()]);
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
        return protocol::MutationOutcome::error("mutation.invalid-payload", format!("{:?} is not an artifact reference URI.", payload.target_uri), [payload.target_uri.clone()]);
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
        return protocol::MutationOutcome::error("mutation.invalid-payload", format!("{:?} is not an artifact reference URI.", payload.target_uri), [payload.target_uri.clone()]);
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
        return protocol::MutationOutcome::error("mutation.duplicate", format!("Another zone is already named \\"{}\\".", payload.new_name), [payload.id.0.to_string()]);
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
        return unchanged(before), rejected("mutation.invalid-payload", [])
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
        return unchanged(before), rejected("mutation.invalid-payload", [])
    if not payload["present"] and (zone_ids or link_ids):
        return unchanged(before), rejected("mutation.invalid-payload", [])
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
        return unchanged(before), rejected("mutation.duplicate", [name, key])
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
        return unchanged(before), rejected("mutation.invalid-payload", [payload["targetUri"]])
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
        return unchanged(before), rejected("mutation.invalid-payload", [payload["targetUri"]])
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
        return unchanged(before), rejected("mutation.duplicate", [str(entity_id)])
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
def audit() -> None:
    """🚨️ Fails loudly on the two collisions parallel group workers can actually cause: two kinds on
    one ledger number (the protobuf field number must be unique forever) and two kinds on one emoji
    (`pathEmojiPolicy.siblingNamespace` is `files-and-directories` inside `🧬️mutations/`)."""
    taken_emoji = {"💾️", "📖️", "📝️", "🔗️", "🔣️", "🛰️", "🟦️", "🦀️"}
    seen_numbers, seen_emoji, seen_slugs = {}, {}, {}
    for k in KINDS:
        assert k.emoji not in taken_emoji, f"{k.slug}: {k.emoji} is a non-kind sibling inside 🧬️mutations/"
        assert k.emoji.endswith("\ufe0f") and not k.emoji.endswith("\ufe0f\ufe0f"), f"{k.slug}: {k.emoji!r} needs exactly one trailing U+FE0F"
        assert k.dir == k.dir.__str__() and unicodedata.normalize("NFC", k.dir) == k.dir, f"{k.slug}: directory name is not NFC"
        for table, key, label in ((seen_numbers, k.number, "ledger number"), (seen_emoji, k.emoji, "emoji"), (seen_slugs, k.slug, "slug")):
            assert key not in table, f"{label} {key!r} is claimed by both {table[key]} and {k.slug}"
            table[key] = k.slug
        assert camel(k.slug) == k.variant


def main() -> None:
    import shutil

    audit()

    old = os.path.join(ROOT, MUT, "♻️replace-model")
    if os.path.isdir(old):
        shutil.rmtree(old)
        print("removed ♻️replace-model")

    for k in KINDS:
        write(f"{MUT}/{k.dir}/🦀️.rs", leaf_component(k))
        write(f"{MUT}/{k.dir}/🔺️diff/🦀️.rs", leaf_diff(k))
        write(f"{MUT}/{k.dir}/↩️inverse/🦀️.rs", leaf_inverse(k))
        write(f"{MUT}/{k.dir}/🔣️.json", leaf_descriptor(k))
        write(f"{MUT}/{k.dir}/🧬️.schema.json", leaf_payload_schema(k))
        for emoji, name, description, scenario in k.cases:
            case_dir = f"{MUT}/{k.dir}/🧪️tests/{emoji}{name}"
            write(f"{case_dir}/🦀️.rs", case_component(k, (emoji, name, description, scenario)))
            for relative in ["📸️snapshot/⬅️before/🔣️.json", "📸️snapshot/➡️after/🔣️.json", "🦠️mutation/🔣️.json", "🔺️diff/🔣️.json", "🎯️outcome/🔣️.json"]:
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
