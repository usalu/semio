"""🧪️ S3-CONTROLS — energy field-granular leaves (design §13.1, coordinator GO 2026-10-02).

Replaces the whole-record `update-site`, `update-ground-temperature`, `update-run-period` leaves of `s.energy.model` with
thirteen field-granular leaves (one per site field, per ground-temperature series month / deep value, per run-period field),
schema-first: leaf anatomy (Rust leaf, diff, inverse, descriptor, payload schema with full `x-semio-ui`, fixture tests with
placeholder quintets the Rust writer fills via `SEMIO_ENERGY_WRITE_FIXTURES=1`) and every registry (root module tree,
aggregate, KINDS, DIRECTORIES, wire probes, binary protocol, text grammars, GraphQL, protobuf, aggregate JSON schema, TS twin,
oracle catalog, the exhaustive case's feature / Python second implementation / Rust adapter). Every textual edit is an
exact replacement asserted to match once. Run once from the repository root: `python3 <this file>`.
"""

import json
import pathlib
import re
import shutil

ROOT = pathlib.Path.cwd()
ART = ROOT / "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model"
SUB = ART / "🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = SUB / "🧬️schema/🧬️mutations"
FIX = SUB / "🧫️fixtures/🧬️mutations"
CASE = SUB / "🧪️tests/🏛️mutate-energy-model-1"
REL = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
OWNER = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/" + REL
ID = "https://json.schemas.assets.semio-tech.com/s/energy/model/mutation/{}/schema.json"
OLD = [("update-site", "🌍️update-site", "UpdateSite"), ("update-ground-temperature", "🌡️update-ground-temperature", "UpdateGroundTemperature"), ("update-run-period", "📅️update-run-period", "UpdateRunPeriod")]


def ui(label, description, extra, order, group):
    """🎛️ One `x-semio-ui` annotation in the energy vocabulary's key order."""
    node = {"widget": extra.pop("widget", "stepper"), "role": "value", "label": label}
    if description:
        node["description"] = description
    node.update(extra)
    node["group"] = group
    node["order"] = order
    return node


TEMPERATURE = {"unit": "°C", "step": 0.5, "precision": 1, "softMin": -20, "softMax": 40}
MONTH = ui({"en": "Month", "de": "Monat"}, {"en": "Calendar month, 1 = January.", "de": "Kalendermonat, 1 = Januar."}, {"step": 1, "precision": 0, "softMin": 1, "softMax": 12}, 10, "ground")

LEAVES = [
    dict(kind="change-site-latitude", dir="🌍️change-site-latitude", group="site", field="latitude_deg", param="new_latitude_deg", wire="newLatitudeDeg", ty="f64",
         doc="Sets the site's geographic latitude, north positive.", what="latitude", unit="°", lo=-90.0, hi=90.0,
         en='Change site latitude to {}°', de='Breitengrad des Standorts auf {}° ändern',
         schema={"type": "number", "minimum": -90, "maximum": 90}, ui=ui({"en": "Latitude", "de": "Breitengrad"}, {"en": "North positive, -90 to 90.", "de": "Nord positiv, -90 bis 90."}, {"unit": "deg", "step": 0.001, "precision": 3, "softMin": -90, "softMax": 90}, 10, "site"),
         cases=[("✅️sets", "moves the site to Denver's latitude", "", "39.74"), ("⛔️refuses", "refuses a latitude beyond the pole", "", "123.0")]),
    dict(kind="change-site-longitude", dir="🌏️change-site-longitude", group="site", field="longitude_deg", param="new_longitude_deg", wire="newLongitudeDeg", ty="f64",
         doc="Sets the site's geographic longitude, east positive.", what="longitude", unit="°", lo=-180.0, hi=180.0,
         en='Change site longitude to {}°', de='Längengrad des Standorts auf {}° ändern',
         schema={"type": "number", "minimum": -180, "maximum": 180}, ui=ui({"en": "Longitude", "de": "Längengrad"}, {"en": "East positive, -180 to 180.", "de": "Ost positiv, -180 bis 180."}, {"unit": "deg", "step": 0.001, "precision": 3, "softMin": -180, "softMax": 180}, 10, "site"),
         cases=[("✅️sets", "moves the site to Denver's longitude", "", "-104.99"), ("⛔️refuses", "refuses a longitude beyond the date line", "", "200.0")]),
    dict(kind="change-site-elevation", dir="⛰️change-site-elevation", group="site", field="elevation_m", param="new_elevation_m", wire="newElevationM", ty="f64",
         doc="Sets the site's elevation above sea level (EnergyPlus `Site:Location`: -300 m to below 8900 m).", what="elevation", unit=" m", lo=-300.0, hi=8900.0, hi_open=True,
         en='Change site elevation to {} m', de='Höhe des Standorts auf {} m ändern',
         schema={"type": "number", "minimum": -300, "exclusiveMaximum": 8900}, ui=ui({"en": "Elevation", "de": "Höhe über NN"}, {"en": "Above sea level, -300 m to below 8900 m.", "de": "Über Normalnull, -300 m bis unter 8900 m."}, {"unit": "m", "step": 1, "precision": 0}, 10, "site"),
         cases=[("✅️sets", "lifts the site to Denver's elevation", "", "1650.0"), ("⛔️refuses", "refuses an elevation above any terrain", "", "9000.0")]),
    dict(kind="change-site-time-zone", dir="🕰️change-site-time-zone", group="site", field="time_zone_hours", param="new_time_zone_hours", wire="newTimeZoneHours", ty="f64",
         doc="Sets the site's standard time zone as an offset from UTC.", what="time zone", unit=" h", lo=-12.0, hi=14.0,
         en='Change site time zone to UTC{:+}', de='Zeitzone des Standorts auf UTC{:+} ändern',
         schema={"type": "number", "minimum": -12, "maximum": 14}, ui=ui({"en": "Time zone", "de": "Zeitzone"}, {"en": "Offset from UTC, -12 to 14 h.", "de": "Abweichung von UTC, -12 bis 14 h."}, {"unit": "h", "step": 0.25, "precision": 2, "softMin": -12, "softMax": 14}, 10, "site"),
         cases=[("✅️sets", "moves the site to Mountain Standard Time", "", "-7.0"), ("⛔️refuses", "refuses an offset no time zone has", "", "15.0")]),
    dict(kind="change-site-north-axis", dir="🔝️change-site-north-axis", group="site", field="north_axis_deg", param="new_north_axis_deg", wire="newNorthAxisDeg", ty="f64",
         doc="Sets the angle from true north to the building's north axis, clockwise.", what="north axis", unit="°", lo=None, hi=None,
         en='Change building north axis to {}°', de='Nordachse des Gebäudes auf {}° ändern',
         schema={"type": "number"}, ui=ui({"en": "North axis", "de": "Nordachse"}, {"en": "Angle from true north to the building's north axis, clockwise.", "de": "Winkel von geografisch Nord zur Nordachse des Gebäudes, im Uhrzeigersinn."}, {"widget": "dial", "unit": "deg", "step": 1, "precision": 1, "softMin": 0, "softMax": 360, "snaps": [0, 90, 180, 270, 360]}, 10, "site"),
         cases=[("✅️sets", "turns the building's north axis", "", "30.0"), ("🟰️same", "keeps an unchanged north axis as a no-op", "model.site.north_axis_deg = 30.0;", "30.0")]),
    dict(kind="change-ground-building", dir="🌡️change-ground-building", group="ground-month", series="building_surface_c", what="building-surface ground temperature",
         doc="Sets one month's ground temperature under the building, read by the ground heat transfer solve.",
         en='Change building-surface ground temperature of month {} to {} °C', de='Erdreichtemperatur unter dem Gebäude im Monat {} auf {} °C ändern',
         ui=ui({"en": "Building surface ground temperature", "de": "Erdreichtemperatur unter dem Gebäude"}, {"en": "Monthly value at the building's ground contact.", "de": "Monatswert am Erdreichkontakt des Gebäudes."}, dict(TEMPERATURE), 20, "ground"),
         cases=[("✅️sets", "warms July under the building", "", "7, 21.5"), ("⛔️refuses", "refuses a thirteenth month", "", "13, 20.0")]),
    dict(kind="change-ground-shallow", dir="🌱️change-ground-shallow", group="ground-month", series="shallow_c", what="shallow ground temperature",
         doc="Sets one month's shallow ground temperature, read by the ground heat transfer solve.",
         en='Change shallow ground temperature of month {} to {} °C', de='Oberflächennahe Erdreichtemperatur im Monat {} auf {} °C ändern',
         ui=ui({"en": "Shallow ground temperature", "de": "Oberflächennahe Erdreichtemperatur"}, {"en": "Monthly value near the surface.", "de": "Monatswert nahe der Oberfläche."}, dict(TEMPERATURE), 20, "ground"),
         cases=[("✅️sets", "cools January near the surface", "", "1, 2.5"), ("⛔️refuses", "refuses a temperature below absolute zero", "", "1, -300.0")]),
    dict(kind="change-ground-deep", dir="⛏️change-ground-deep", group="ground-deep", what="deep ground temperature",
         doc="Sets the deep ground temperature, read by the ground heat transfer solve.",
         en='Change deep ground temperature to {} °C', de='Erdreichtemperatur in der Tiefe auf {} °C ändern',
         ui=ui({"en": "Deep ground temperature", "de": "Erdreichtemperatur in der Tiefe"}, {"en": "Constant over the year.", "de": "Über das Jahr konstant."}, dict(TEMPERATURE), 10, "ground"),
         cases=[("✅️sets", "sets Denver's deep ground temperature", "", "11.0"), ("⛔️refuses", "refuses a temperature below absolute zero", "", "-300.0")]),
    dict(kind="change-run-start-month", dir="🛫️change-run-start-month", group="run", field="start_month", param="new_start_month", wire="newStartMonth", ty="u8", lo=1, hi=12, title="Start month",
         doc="Sets the month the simulation run period starts in; the period must stay a calendar interval of its year.",
         en='Change run period start month to {}', de='Startmonat des Simulationszeitraums auf {} ändern',
         ui=ui({"en": "Start month", "de": "Startmonat"}, None, {"step": 1, "precision": 0, "softMin": 1, "softMax": 12}, 10, "calendar"),
         cases=[("✅️sets", "starts the run in March", "", "3"), ("⛔️refuses", "refuses a start after the current end", "model.run_period = crate::calendar::RunPeriod { start_month: 1, start_day: 1, end_month: 1, end_day: 31, year: 2026 };", "2")]),
    dict(kind="change-run-start-day", dir="▶️change-run-start-day", group="run", field="start_day", param="new_start_day", wire="newStartDay", ty="u8", lo=1, hi=31, title="Start day",
         doc="Sets the day of month the simulation run period starts on; the period must stay a calendar interval of its year.",
         en='Change run period start day to {}', de='Starttag des Simulationszeitraums auf {} ändern',
         ui=ui({"en": "Start day", "de": "Starttag"}, None, {"step": 1, "precision": 0, "softMin": 1, "softMax": 31}, 20, "calendar"),
         cases=[("✅️sets", "starts the run mid-January", "", "15"), ("⛔️refuses", "refuses a start day February does not have", "model.run_period = crate::calendar::RunPeriod { start_month: 2, start_day: 1, end_month: 2, end_day: 28, year: 2026 };", "30")]),
    dict(kind="change-run-end-month", dir="🛬️change-run-end-month", group="run", field="end_month", param="new_end_month", wire="newEndMonth", ty="u8", lo=1, hi=12, title="End month",
         doc="Sets the month the simulation run period ends in; the period must stay a calendar interval of its year.",
         en='Change run period end month to {}', de='Endmonat des Simulationszeitraums auf {} ändern',
         ui=ui({"en": "End month", "de": "Endmonat"}, None, {"step": 1, "precision": 0, "softMin": 1, "softMax": 12}, 30, "calendar"),
         cases=[("✅️sets", "ends the run in July", "", "7"), ("⛔️refuses", "refuses an end month without the current end day", "", "2")]),
    dict(kind="change-run-end-day", dir="⏹️change-run-end-day", group="run", field="end_day", param="new_end_day", wire="newEndDay", ty="u8", lo=1, hi=31, title="End day",
         doc="Sets the day of month the simulation run period ends on; the period must stay a calendar interval of its year.",
         en='Change run period end day to {}', de='Endtag des Simulationszeitraums auf {} ändern',
         ui=ui({"en": "End day", "de": "Endtag"}, None, {"step": 1, "precision": 0, "softMin": 1, "softMax": 31}, 40, "calendar"),
         cases=[("✅️sets", "ends the run a day before New Year's Eve", "", "30"), ("⛔️refuses", "refuses an end before the current start", "model.run_period = crate::calendar::RunPeriod { start_month: 3, start_day: 15, end_month: 3, end_day: 31, year: 2026 };", "10")]),
    dict(kind="change-run-year", dir="📅️change-run-year", group="run", field="year", param="new_year", wire="newYear", ty="u16", lo=0, hi=65535, title="Year",
         doc="Sets the calendar year that fixes the run period's weekdays and leap day; the period must stay a calendar interval of that year.",
         en='Change run period year to {}', de='Jahr des Simulationszeitraums auf {} ändern',
         ui=ui({"en": "Year", "de": "Jahr"}, {"en": "Calendar year that fixes the weekdays.", "de": "Kalenderjahr, das die Wochentage festlegt."}, {"step": 1, "precision": 0}, 50, "calendar"),
         cases=[("✅️sets", "moves the run to 2027", "", "2027"), ("⛔️refuses", "refuses a year without the period's leap day", "model.run_period = crate::calendar::RunPeriod { start_month: 1, start_day: 1, end_month: 2, end_day: 29, year: 2028 };", "2027")]),
]

for leaf in LEAVES:
    words = leaf["kind"].split("-")
    leaf["variant"] = "".join(word.capitalize() for word in words)
    leaf["camel"] = words[0] + "".join(word.capitalize() for word in words[1:])
    leaf["fn"] = leaf["kind"].replace("-", "_")
    leaf["emoji"] = re.match(r"^([^a-z]+)", leaf["dir"]).group(1)
    leaf["entity"] = {"site": "site", "ground-month": "ground-temperature", "ground-deep": "ground-temperature", "run": "run-period"}[leaf["group"]]
    leaf["display"] = " ".join(word.capitalize() for word in words)
    leaf.setdefault("title", "")
    leaf.setdefault("what", leaf["title"].lower())
    leaf.setdefault("unit", "")
    if leaf["group"] == "ground-month":
        leaf.update(param="new_temperature_c", wire="newTemperatureC", ty="f64")
    if leaf["group"] == "ground-deep":
        leaf.update(param="new_temperature_c", wire="newTemperatureC", ty="f64")


#region 🔖️RustLeaf
def payload_fields(leaf):
    """🧾️ `(name, type)` of the payload struct, address first."""
    return ([("month", "u8")] if leaf["group"] == "ground-month" else []) + [(leaf["param"], leaf["ty"])]


def label_args(leaf):
    """🏷️ The `format!` arguments of the label."""
    return ", ".join(f"self.{name}" for name, _ in payload_fields(leaf))


def leaf_rs(leaf):
    fields = payload_fields(leaf)
    params = ", ".join(f"{name}: {ty}" for name, ty in fields)
    names = ", ".join(name for name, _ in fields)
    struct = "\n".join(f"    pub {name}: {ty}," for name, ty in fields)
    record = "Changed" + leaf["variant"][len("Change"):]
    return f'''//! {leaf["emoji"]} Energy model mutation — `{leaf["variant"]}`: {leaf["doc"]}

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{{FromValue as FromValueDerive, ToValue as ToValueDerive}};

//#region 🔖️Mutation
/// {leaf["emoji"]} `{leaf["kind"]}` payload. {leaf["doc"]}
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "{leaf["kind"]}")]
pub struct {leaf["variant"]} {{
{struct}
}}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn {leaf["fn"]}({params}) -> EnergyModelMutation {{
    EnergyModelMutation::{leaf["variant"]}({leaf["variant"]} {{ {names} }})
}}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for {leaf["variant"]} {{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {{ verb: "change", entity: "{leaf["entity"]}", kind: "{leaf["kind"]}", record: "{record}" }};

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {{
        super::diff::diff(self, base)
    }}

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {{
        super::inverse::inverse(self, base)
    }}

    fn label(&self) -> protocol::LocalizedLabel {{
        protocol::LocalizedLabel::native(&format!("{leaf["en"]}", {label_args(leaf)}), &format!("{leaf["de"]}", {label_args(leaf)}))
    }}
}}
//#endregion 🔖️Mutation
'''


def rust_float(number):
    """🔢️ A Rust `f64` literal."""
    return repr(float(number))


def admissible(leaf, value):
    """🚧️ The payload-intrinsic admissibility expression (`mutation.invariant` when false)."""
    if leaf["group"] == "site":
        if leaf["lo"] is None:
            return f"{value}.is_finite()"
        return f"({rust_float(leaf['lo'])}{'..' if leaf.get('hi_open') else '..='}{rust_float(leaf['hi'])}).contains(&{value})"
    if leaf["group"] == "ground-month":
        return f"(1..=12).contains(&payload.month) && {value}.is_finite() && {value} >= -273.15"
    if leaf["group"] == "ground-deep":
        return f"{value}.is_finite() && {value} >= -273.15"
    if leaf["field"] == "year":
        return None
    return f"({leaf['lo']}..={leaf['hi']}).contains(&{value})"


def negate(check):
    """🚫️ `!check`, parenthesized only around a conjunction."""
    return f"!({check})" if "&&" in check else f"!{check}"


def current(leaf):
    """📍️ The base value the leaf replaces."""
    if leaf["group"] == "site":
        return f"base.model.site.{leaf['field']}"
    if leaf["group"] == "ground-month":
        return f"base.model.ground_temperature.{leaf['series']}[usize::from(payload.month) - 1]"
    if leaf["group"] == "ground-deep":
        return "base.model.ground_temperature.deep_c"
    return f"base.model.run_period.{leaf['field']}"


def diff_rs(leaf):
    value = f"payload.{leaf['param']}"
    check = admissible(leaf, value)
    refusal = {
        "site": f'format!("A site {leaf["what"]} of {{}}{leaf.get("unit", "")} is not admissible.", {value})',
        "ground-month": f'format!("A {leaf["what"]} of {{}} °C for month {{}} is not admissible.", {value}, payload.month)',
        "ground-deep": f'format!("A {leaf["what"]} of {{}} °C is not admissible.", {value})',
        "run": f'format!("A run period {leaf.get("title", "").lower()} of {{}} is not admissible.", {value})',
    }[leaf["group"]]
    invariant = f'''    if {negate(check) if check else ""} {{
        return protocol::MutationOutcome::fatal("mutation.invariant", {refusal}, Vec::<String>::new());
    }}
''' if check else ""
    noop_text = {
        "site": f'format!("The site {leaf["what"]} is already {{}}{leaf.get("unit", "")}.", {value})',
        "ground-month": f'format!("The {leaf["what"]} of month {{}} is already {{}} °C.", payload.month, {value})',
        "ground-deep": f'format!("The {leaf["what"]} is already {{}} °C.", {value})',
        "run": f'format!("The run period {leaf.get("title", "").lower()} is already {{}}.", {value})',
    }[leaf["group"]]
    mismatch = f'''    if !model.run_period.is_interval() {{
        let period = base.model.run_period;
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("A {leaf.get("title", "").lower()} of {{}} does not form a calendar interval with the run period {{}}-{{}} .. {{}}-{{}} of {{}}.", {value}, period.start_month, period.start_day, period.end_month, period.end_day, period.year), Vec::<String>::new());
    }}
''' if leaf["group"] == "run" else ""
    return f'''//! 🔺️ Sparse diff builder for `{leaf["variant"]}` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::{leaf["variant"]}, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {{
{invariant}    if {current(leaf)} == {value} {{
        return protocol::MutationOutcome::empty().warning("mutation.no-op", {noop_text});
    }}
    let mut model = base.model.clone();
    {current(leaf).replace("base.model", "model")} = {value};
{mismatch}    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}}
//#endregion 🔖️Diff
'''


def inverse_rs(leaf):
    value = f"payload.{leaf['param']}"
    check = admissible(leaf, value)
    month = "payload.month, " if leaf["group"] == "ground-month" else ""
    if leaf["group"] == "run":
        guard = f"{negate(check) + ' || ' if check else ''}{current(leaf)} == {value} || !next.is_interval()"
        body = f'''    let mut next = base.model.run_period;
    next.{leaf["field"]} = {value};
    if {guard} {{
        return Vec::new();
    }}
'''
    else:
        body = f'''    if {negate(check)} || {current(leaf)} == {value} {{
        return Vec::new();
    }}
'''
    return f'''//! ↩️ Inverse for `{leaf["variant"]}` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::{leaf["variant"]}, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {{
{body}    vec![vocabulary::{leaf["fn"]}({month}{current(leaf)})]
}}
//#endregion 🔖️Inverse
'''


def test_rs(leaf, case):
    directory, summary, setup, args = case
    files = {"BEFORE": "📸️snapshot/⬅️before", "AFTER": "📸️snapshot/➡️after", "MUTATION": "🦠️mutation", "DIFF": "🔺️diff", "OUTCOME": "🎯️outcome"}
    consts = "\n".join(f'const {name}: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf["dir"]}/{directory}/{path}/🔣️.json");' for name, path in files.items())
    setup_line = f"    {setup}\n" if setup else ""
    return f'''//! 🧪️ `{leaf["kind"]}` fixture — `{directory}`: {summary}.
//!
//! The committed `(before, mutation, after, diff, outcome)` quintet beside this file IS the
//! specification; `scenario` is the typed source it was generated from
//! (`SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test -p semio-s-artifact-energy-model`), and the eight law
//! assertions below read the committed bytes back, never the scenario.

use crate::mutations::fixtures::{{self, snapshot, Case}};
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

{consts}

#[allow(unused_variables, unused_mut)]
fn scenario() -> (EnergyModelSnapshot, EnergyModelMutation) {{
    let mut model = crate::model::Model {{ name: "BESTEST 600".into(), version: "1".into(), ..crate::model::Model::default() }};
{setup_line}    (snapshot(model), super::{leaf["fn"]}({args}))
}}

fn case() -> Case {{
    Case {{ kind: "{leaf["kind"]}", directory: "{leaf["dir"]}/{directory}", before: BEFORE, after: AFTER, mutation: MUTATION, diff: DIFF, outcome: OUTCOME, scenario }}
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
'''
#endregion 🔖️RustLeaf


#region 🔖️Schemas
def payload_schema(leaf):
    properties = {"mutation": {"const": leaf["camel"]}}
    required = ["mutation"]
    if leaf["group"] == "ground-month":
        properties["month"] = {"type": "integer", "minimum": 1, "maximum": 12, "x-semio-ui": dict(MONTH)}
        required.append("month")
    if leaf["group"] in ("ground-month", "ground-deep"):
        value = {"type": "number", "minimum": -273.15}
    elif leaf["group"] == "run":
        value = {"type": "integer", "minimum": leaf["lo"], "maximum": leaf["hi"]}
    else:
        value = dict(leaf["schema"])
    value["x-semio-ui"] = leaf["ui"]
    properties[leaf["wire"]] = value
    required.append(leaf["wire"])
    return {"$schema": "http://json-schema.org/draft-07/schema#", "$id": ID.format(leaf["kind"]), "title": leaf["variant"], "type": "object", "additionalProperties": False, "required": required, "properties": properties}


def descriptor(leaf, tag):
    return {
        "schemaVersion": 1,
        "owner": f"{OWNER}/{leaf['dir']}",
        "semanticKind": leaf["kind"],
        "displayName": leaf["display"],
        "emoji": leaf["emoji"],
        "aggregateVariant": leaf["variant"],
        "payloadSchema": "🧬️schema/🔣️.json",
        "textOpcode": leaf["kind"],
        "binaryTag": tag,
        "invertibility": "explicit-mutation",
        "diffParticipation": "detect",
        "outcomeClasses": ["applied", "no-op", "rejected"],
        "composition": "atomic",
        "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"],
    }


def dump(value):
    return json.dumps(value, indent=2, ensure_ascii=False) + "\n"
#endregion 🔖️Schemas


#region 🔖️Edits
def replace(path, old, new):
    text = path.read_text()
    count = text.count(old)
    assert count == 1, f"{path}: {count} matches for {old[:120]!r}"
    path.write_text(text.replace(old, new))


def module_block(leaf):
    base = f"{REL}/{leaf['dir']}"
    tests = "".join(f'''                            #[cfg(test)]
                            #[path = "{base}/🧪️tests/{directory}/🦀️.rs"]
                            mod tests_{re.sub(r"[^a-z]", "", directory)};
''' for directory, _, _, _ in leaf["cases"])
    return f'''                        #[path = "."]
                        pub mod {leaf["fn"]} {{
                            #[path = "{base}/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "{base}/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "{base}/↩️inverse/🦀️.rs"]
                            pub mod inverse;
{tests}                        }}
'''


def old_module_block(text, module):
    start = text.index(f'                        #[path = "."]\n                        pub mod {module} {{\n')
    end = text.index("                        }\n", start) + len("                        }\n")
    return text[start:end]


def scenario_id(leaf, directory):
    return f"{leaf['kind']}-{re.sub(r'^[^a-z]+', '', directory)}"


def grammar_rule(leaf):
    parts = (['"month" "=" NUMBER'] if leaf["group"] == "ground-month" else []) + [f'"{leaf["param"].replace("_", "-")}" "=" NUMBER']
    return f'{leaf["kind"]} = "{leaf["kind"]}" ' + " ".join(parts)


def graphql_type(leaf):
    ty = {"f64": "Float!", "u8": "Int!", "u16": "Int!"}
    fields = (["  month: Int!"] if leaf["group"] == "ground-month" else []) + [f"  {leaf['wire']}: {ty[leaf['ty']]}"]
    return f"type {leaf['variant']} {{\n" + "\n".join(fields) + "\n}\n"


def proto_message(leaf):
    ty = {"f64": "double", "u8": "uint32", "u16": "uint32"}
    fields = (["  uint32 month = 1;"] if leaf["group"] == "ground-month" else []) + [f"  {ty[leaf['ty']]} {leaf['param']} = {2 if leaf['group'] == 'ground-month' else 1};"]
    return f"message {leaf['variant']} {{\n" + "\n".join(fields) + "\n}\n"


def ts_interface(leaf):
    fields = (["  readonly month: number;"] if leaf["group"] == "ground-month" else []) + [f"  readonly {leaf['wire']}: number;"]
    return f"/** {leaf['emoji']} `{leaf['kind']}` payload. */\nexport interface {leaf['variant']} {{\n  readonly mutation: \"{leaf['camel']}\";\n" + "\n".join(fields) + "\n}\n"


def probe(leaf):
    """🧵️ One representative value of the leaf for the codec round-trip corpus."""
    if leaf["group"] == "site":
        return f"{leaf['fn']}({rust_float({'latitude_deg': 52.4, 'longitude_deg': 9.7, 'elevation_m': 55.0, 'time_zone_hours': 1.0, 'north_axis_deg': 15.0}[leaf['field']])})"
    if leaf["group"] == "ground-month":
        return f"{leaf['fn']}(7, 21.5)"
    if leaf["group"] == "ground-deep":
        return f"{leaf['fn']}(8.0)"
    return f"{leaf['fn']}({ {'start_month': 2, 'start_day': 3, 'end_month': 11, 'end_day': 30, 'year': 2027}[leaf['field']] })"
#endregion 🔖️Edits


def main():
    names = {name for name in (p.name for p in MUT.iterdir() if p.is_dir())} - {old_dir for _, old_dir, _ in OLD}
    taken = {re.match(r"^([^a-z]+)", name).group(1) for name in names if re.match(r"^([^a-z]+)", name)}
    for leaf in LEAVES:
        assert leaf["emoji"] not in taken, f"{leaf['dir']}: emoji {leaf['emoji']} is taken"
        taken.add(leaf["emoji"])
    protocol = MUT / "💾️binary/📡️.protocol.semio"
    tags = [int(tag) for tag in re.findall(r"^record \S+ tag=(\d+)$", protocol.read_text(), re.M)]
    next_tag = max(tags) + 1
    for index, leaf in enumerate(LEAVES):
        leaf["tag"] = next_tag + index
    for leaf in LEAVES:
        for directory, _, _, _ in leaf["cases"]:
            for path in [f"{FIX.relative_to(ROOT)}/{leaf['dir']}/{directory}/📸️snapshot/⬅️before/🔣️.json", f"{MUT.relative_to(ROOT)}/{leaf['dir']}/🧪️tests/{directory}/🦀️.rs"]:
                assert len(path.encode()) <= 240, (path, len(path.encode()))

    for leaf in LEAVES:
        owner = MUT / leaf["dir"]
        (owner / "🔺️diff").mkdir(parents=True, exist_ok=True)
        (owner / "↩️inverse").mkdir(parents=True, exist_ok=True)
        (owner / "🧬️schema").mkdir(parents=True, exist_ok=True)
        (owner / "🦀️.rs").write_text(leaf_rs(leaf))
        (owner / "🔺️diff/🦀️.rs").write_text(diff_rs(leaf))
        (owner / "↩️inverse/🦀️.rs").write_text(inverse_rs(leaf))
        (owner / "🧬️schema/🔣️.json").write_text(dump(payload_schema(leaf)))
        (owner / "🔣️.json").write_text(dump(descriptor(leaf, leaf["tag"])))
        for case in leaf["cases"]:
            (owner / "🧪️tests" / case[0]).mkdir(parents=True, exist_ok=True)
            (owner / "🧪️tests" / case[0] / "🦀️.rs").write_text(test_rs(leaf, case))
            for path in ["📸️snapshot/⬅️before", "📸️snapshot/➡️after", "🦠️mutation", "🔺️diff", "🎯️outcome"]:
                target = FIX / leaf["dir"] / case[0] / path
                target.mkdir(parents=True, exist_ok=True)
                if not (target / "🔣️.json").exists():
                    (target / "🔣️.json").write_text("{}\n")

    root = ART / "🦀️.rs"
    text = root.read_text()
    old = "".join(old_module_block(text, module) for module in ["update_site", "update_ground_temperature", "update_run_period"])
    replace(root, old, "".join(module_block(leaf) for leaf in LEAVES))

    aggregate = MUT / "🦀️.rs"
    replace(aggregate, "pub use super::update_ground_temperature::{update_ground_temperature, UpdateGroundTemperature};\npub use super::update_run_period::{update_run_period, UpdateRunPeriod};\npub use super::update_site::{update_site, UpdateSite};\n",
            "".join(f"pub use super::{leaf['fn']}::{{{leaf['fn']}, {leaf['variant']}}};\n" for leaf in LEAVES))
    replace(aggregate, "    UpdateSite(UpdateSite),\n    UpdateGroundTemperature(UpdateGroundTemperature),\n    UpdateRunPeriod(UpdateRunPeriod),\n", "".join(f"    {leaf['variant']}({leaf['variant']}),\n" for leaf in LEAVES))
    replace(aggregate, '    "update-site",\n    "update-ground-temperature",\n    "update-run-period",\n', "".join(f'    "{leaf["kind"]}",\n' for leaf in LEAVES))
    replace(aggregate, '    ("update-site", "🌍️update-site"),\n    ("update-ground-temperature", "🌡️update-ground-temperature"),\n    ("update-run-period", "📅️update-run-period"),\n', "".join(f'    ("{leaf["kind"]}", "{leaf["dir"]}"),\n' for leaf in LEAVES))
    replace(aggregate, "        update_site(52.4, 9.7, 55.0, 1.0, 0.0),\n        update_ground_temperature(vec![10.0; 12], vec![9.0; 12], 8.0),\n        update_run_period(1, 1, 1, 31, 2026),\n", "".join(f"        {probe(leaf)},\n" for leaf in LEAVES))

    replace(protocol, "record update-site tag=2\nfield payload bytes\nrecord update-ground-temperature tag=3\nfield payload bytes\nrecord update-run-period tag=4\nfield payload bytes\n", "".join(f"record {leaf['kind']} tag={leaf['tag']}\nfield payload bytes\n" for leaf in LEAVES))

    for grammar in [MUT / "📖️.grammar.semio", MUT / "📝️text/📖️.grammar.semio"]:
        replace(grammar, " | update-site | update-ground-temperature | update-run-period | ", " | " + " | ".join(leaf["kind"] for leaf in LEAVES) + " | ")
        text = grammar.read_text()
        rules = [line for line in text.splitlines(keepends=True) if line.startswith(("update-site = ", "update-ground-temperature = ", "update-run-period = "))]
        assert len(rules) == 3, grammar
        replace(grammar, "".join(rules), "".join(grammar_rule(leaf) + "\n" for leaf in LEAVES))

    graphql = MUT / "🔗️.graphql"
    text = graphql.read_text()
    start, end = text.index("type UpdateSite {"), text.index("type ReplaceAirflowNetwork {")
    replace(graphql, text[start:end], "\n".join(graphql_type(leaf) for leaf in LEAVES) + "\n")
    replace(graphql, "  | UpdateSite\n  | UpdateGroundTemperature\n  | UpdateRunPeriod\n", "".join(f"  | {leaf['variant']}\n" for leaf in LEAVES))

    proto = MUT / "🛰️.proto"
    text = proto.read_text()
    start, end = text.index("message UpdateSite {"), text.index("message ReplaceAirflowNetwork {")
    replace(proto, text[start:end], "\n".join(proto_message(leaf) for leaf in LEAVES) + "\n")
    oneof = proto.read_text()
    numbers = [int(number) for number in re.findall(r"^    \w+ \w+ = (\d+);$", oneof[oneof.index("message EnergyModelMutation {"):], re.M)]
    first = max(numbers) + 1
    replace(proto, "    UpdateSite update_site = 3;\n    UpdateGroundTemperature update_ground_temperature = 4;\n    UpdateRunPeriod update_run_period = 5;\n", "".join(f"    {leaf['variant']} {leaf['fn']} = {first + index};\n" for index, leaf in enumerate(LEAVES)))

    schema = MUT / "🔣️.json"
    replace(schema, "".join(f'    {{\n      "$ref": "{ID.format(kind)}"\n    }},\n' for kind, _, _ in OLD), "".join(f'    {{\n      "$ref": "{ID.format(leaf["kind"])}"\n    }},\n' for leaf in LEAVES))

    twin = MUT / "🟦️.ts"
    text = twin.read_text()
    start, end = text.index("/** 🌍️ `update-site` payload. */"), text.index("/** 🫧️ `replace-airflow-network` payload. */")
    replace(twin, text[start:end], "\n".join(ts_interface(leaf) for leaf in LEAVES) + "\n")
    replace(twin, "  | UpdateSite\n  | UpdateGroundTemperature\n  | UpdateRunPeriod\n", "".join(f"  | {leaf['variant']}\n" for leaf in LEAVES))

    oracles = SUB / "🔮️oracles/🔣️.json"
    text = oracles.read_text()
    start = text.index('        {\n          "mutationId": "update-site",')
    end = text.index('        {\n          "mutationId": "replace-airflow-network",')
    catalog = "".join(f'''        {{
          "mutationId": "{leaf["kind"]}",
          "sourceMutationDirectoryName": "{leaf["dir"]}",
          "mutationDirectoryName": "{leaf["dir"]}",
          "scenarios": [
''' + ",\n".join(f'''            {{
              "id": "{scenario_id(leaf, directory)}",
              "directoryName": "{directory}"
            }}''' for directory, _, _, _ in leaf["cases"]) + '''
          ]
        },
''' for leaf in LEAVES)
    replace(oracles, text[start:end], catalog)
    replace(oracles, '        "update-site",\n        "update-ground-temperature",\n        "update-run-period",\n', "".join(f'        "{leaf["kind"]}",\n' for leaf in LEAVES))
    text = oracles.read_text()
    start = text.index('        {\n          "id": "update-site",\n          "capability"')
    end = text.index('        {\n          "id": "replace-airflow-network",\n          "capability"')
    block = text[start:text.index('        {\n          "id": "update-ground-temperature",')]
    replace(oracles, text[start:end], "".join(block.replace('"update-site"', f'"{leaf["kind"]}"').replace('"UpdateSite"', f'"{leaf["variant"]}"') for leaf in LEAVES))

    feature = CASE / "🥒️.feature"
    text = feature.read_text()
    old_rows = "".join(line for line in text.splitlines(keepends=True)[:80] if re.match(r"^      \| update-(site|ground-temperature|run-period)-", line))
    assert old_rows.count("\n") == 6, old_rows
    rows = "".join(f"      | {scenario_id(leaf, directory)} | {leaf['dir']} | {directory} |\n" for leaf in LEAVES for directory, _, _, _ in leaf["cases"])
    feature.write_text(text.replace(old_rows, rows))
    assert feature.read_text().count(rows) == text.count(old_rows) == 2, "both outlines"

    adapter = CASE / "🦀️.rs"
    replace(adapter, '    "update-site",\n    "update-ground-temperature",\n    "update-run-period",\n', "".join(f'    "{leaf["kind"]}",\n' for leaf in LEAVES))
    text = adapter.read_text()
    start = text.index('        Vector {\n            id: "update-site-relocates-to-denver",')
    end = text.index('        Vector {\n            id: "replace-airflow-network-')
    vectors = "".join(f'''        Vector {{
            id: "{scenario_id(leaf, directory)}",
            kind: "{leaf["kind"]}",
            before: include_str!("../../🧫️fixtures/🧬️mutations/{leaf["dir"]}/{directory}/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧫️fixtures/🧬️mutations/{leaf["dir"]}/{directory}/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧫️fixtures/🧬️mutations/{leaf["dir"]}/{directory}/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🧫️fixtures/🧬️mutations/{leaf["dir"]}/{directory}/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🧫️fixtures/🧬️mutations/{leaf["dir"]}/{directory}/🎯️outcome/🔣️.json"),
        }},
''' for leaf in LEAVES for directory, _, _, _ in leaf["cases"])
    replace(adapter, text[start:end], vectors)

    python = CASE / "🐍️.py"
    text = python.read_text()
    old_roots = "".join(line for line in text.splitlines(keepends=True) if re.match(r'^    "update-(site|ground-temperature|run-period)-', line))
    assert old_roots.count("\n") == 6
    replace(python, old_roots, "".join(f'    "{scenario_id(leaf, directory)}": "shared://🧬️mutations/{leaf["dir"]}/{directory}",\n' for leaf in LEAVES for directory, _, _, _ in leaf["cases"]))
    replace(python, "import copy\nimport json\nimport re\n", "import copy\nimport json\nimport math\nimport re\n")
    text = python.read_text()
    start, end = text.index("def update_site(before, payload):"), text.index("def replace_airflow_network(before, payload):")
    replace(python, text[start:end], PY_LEAVES)
    replace(python, '    "update-site": update_site,\n    "update-ground-temperature": update_ground_temperature,\n    "update-run-period": update_run_period,\n', "".join(f'    "{leaf["kind"]}": {leaf["fn"]},\n' for leaf in LEAVES))
    text = python.read_text()
    start = text.index('    if kind == "update-site":\n')
    end = text.index('    if kind == "replace-airflow-network":\n')
    replace(python, text[start:end], '''    if kind in FIELD_LEAVES:
        return [(kind, FIELD_LEAVES[kind](before, payload))]
''')

    for kind, old_dir, _ in OLD:
        shutil.rmtree(MUT / old_dir)
        shutil.rmtree(FIX / old_dir)
    print("ok", [(leaf["kind"], leaf["tag"]) for leaf in LEAVES], "proto", first)


PY_LEAVES = '''def _run_period_is_interval(period):
    """🗓️ Both bounds are real dates of the period's year and the start does not lie after the end."""
    year = period["year"]
    leap = (year % 4 == 0 and year % 100 != 0) or year % 400 == 0
    lengths = [31, 29 if leap else 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]

    def exists(month, day):
        return 1 <= month <= 12 and 1 <= day <= lengths[month - 1]

    start, end = (period["start_month"], period["start_day"]), (period["end_month"], period["end_day"])
    return exists(*start) and exists(*end) and start <= end


def _site_leaf(field, wire, admissible):
    """🌍️ A `change-site-<field>` leaf: one scalar of the site facet, refused outside its range."""
    def apply(before, payload):
        value = payload[wire]
        if not admissible(value):
            return unchanged(before), rejected("mutation.invariant", [])
        if before["model"]["site"][field] == value:
            return unchanged(before), no_op()
        after = copy.deepcopy(before)
        after["model"]["site"][field] = value
        return after, applied()
    return apply


def _ground_leaf(series):
    """🌡️ A monthly ground-temperature leaf (`series` = None: the deep value): nothing below absolute zero."""
    def apply(before, payload):
        value, month = payload["newTemperatureC"], payload.get("month")
        if (series is not None and not 1 <= month <= 12) or not math.isfinite(value) or value < -273.15:
            return unchanged(before), rejected("mutation.invariant", [])
        ground = before["model"]["ground_temperature"]
        if (ground[series][month - 1] if series else ground["deep_c"]) == value:
            return unchanged(before), no_op()
        after = copy.deepcopy(before)
        if series:
            after["model"]["ground_temperature"][series][month - 1] = value
        else:
            after["model"]["ground_temperature"]["deep_c"] = value
        return after, applied()
    return apply


def _run_period_leaf(field, wire, low, high):
    """📅️ A run-period field leaf: refused outside its range, and refused (`mutation.target-mismatch`) when the edited
    period stops being a calendar interval with the current other bound."""
    def apply(before, payload):
        value = payload[wire]
        if not low <= value <= high:
            return unchanged(before), rejected("mutation.invariant", [])
        if before["model"]["run_period"][field] == value:
            return unchanged(before), no_op()
        after = copy.deepcopy(before)
        after["model"]["run_period"][field] = value
        if not _run_period_is_interval(after["model"]["run_period"]):
            return unchanged(before), rejected("mutation.target-mismatch", [])
        return after, applied()
    return apply


change_site_latitude = _site_leaf("latitude_deg", "newLatitudeDeg", lambda value: -90.0 <= value <= 90.0)
change_site_longitude = _site_leaf("longitude_deg", "newLongitudeDeg", lambda value: -180.0 <= value <= 180.0)
change_site_elevation = _site_leaf("elevation_m", "newElevationM", lambda value: -300.0 <= value < 8900.0)
change_site_time_zone = _site_leaf("time_zone_hours", "newTimeZoneHours", lambda value: -12.0 <= value <= 14.0)
change_site_north_axis = _site_leaf("north_axis_deg", "newNorthAxisDeg", math.isfinite)
change_ground_building = _ground_leaf("building_surface_c")
change_ground_shallow = _ground_leaf("shallow_c")
change_ground_deep = _ground_leaf(None)
change_run_start_month = _run_period_leaf("start_month", "newStartMonth", 1, 12)
change_run_start_day = _run_period_leaf("start_day", "newStartDay", 1, 31)
change_run_end_month = _run_period_leaf("end_month", "newEndMonth", 1, 12)
change_run_end_day = _run_period_leaf("end_day", "newEndDay", 1, 31)
change_run_year = _run_period_leaf("year", "newYear", 0, 65535)


#: ↩️ The base value each field leaf's undo writes back, as that leaf's own payload.
FIELD_LEAVES = {
    "change-site-latitude": lambda before, payload: {"newLatitudeDeg": before["model"]["site"]["latitude_deg"]},
    "change-site-longitude": lambda before, payload: {"newLongitudeDeg": before["model"]["site"]["longitude_deg"]},
    "change-site-elevation": lambda before, payload: {"newElevationM": before["model"]["site"]["elevation_m"]},
    "change-site-time-zone": lambda before, payload: {"newTimeZoneHours": before["model"]["site"]["time_zone_hours"]},
    "change-site-north-axis": lambda before, payload: {"newNorthAxisDeg": before["model"]["site"]["north_axis_deg"]},
    "change-ground-building": lambda before, payload: {"month": payload["month"], "newTemperatureC": before["model"]["ground_temperature"]["building_surface_c"][payload["month"] - 1]},
    "change-ground-shallow": lambda before, payload: {"month": payload["month"], "newTemperatureC": before["model"]["ground_temperature"]["shallow_c"][payload["month"] - 1]},
    "change-ground-deep": lambda before, payload: {"newTemperatureC": before["model"]["ground_temperature"]["deep_c"]},
    "change-run-start-month": lambda before, payload: {"newStartMonth": before["model"]["run_period"]["start_month"]},
    "change-run-start-day": lambda before, payload: {"newStartDay": before["model"]["run_period"]["start_day"]},
    "change-run-end-month": lambda before, payload: {"newEndMonth": before["model"]["run_period"]["end_month"]},
    "change-run-end-day": lambda before, payload: {"newEndDay": before["model"]["run_period"]["end_day"]},
    "change-run-year": lambda before, payload: {"newYear": before["model"]["run_period"]["year"]},
}


'''

if __name__ == "__main__":
    main()
