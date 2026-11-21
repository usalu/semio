import json
import math
import os
import sqlite3
import struct
import subprocess
import tempfile
from pathlib import Path


root = next(parent for parent in Path(__file__).resolve().parents if (parent / '✏️s').is_dir())
artifact = next(root.glob('✏️s/*/📕️norm/*/⚡️din18599/*/🔖️1/*/✳️any'))
fixtures = artifact / '🧪️tests/🧫️fixtures/🔮️cross-checks'
cases = json.loads((fixtures / '🏘️residential-gains.json').read_text(encoding='utf-8'))
envelope = json.loads((fixtures / '🧱️residential-envelope.json').read_text(encoding='utf-8'))
schema = json.loads((artifact / '🧬️schema/📸️snapshot/🔣️.json').read_text(encoding='utf-8'))
checks = artifact / '🧬️schema/💡️inferences/⚖️checks'

with sqlite3.connect(':memory:') as sqlite:
    for case in cases['cases']:
        row = sqlite.execute(
            'WITH dw(rate) AS (SELECT CASE WHEN :dwellings <= 2 THEN 45 ELSE 90 END) SELECT rate/24.0, rate*:area*31/1000.0, rate*:area*28/1000.0, rate*:area*365/1000.0 FROM dw',
            {'dwellings': case['dwellingCount'], 'area': cases['netFloorAreaM2']},
        ).fetchone()
        assert row == tuple(case[key] for key in ['averageWM2', 'januaryKWh', 'februaryKWh', 'annualKWh']), row
        assert case['useClass'] in schema['$defs']['UseClass']['enum']

    sqlite.executescript((artifact / '🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql').read_text(encoding='utf-8'))
    numeric_inputs = (100.0, 300.0, .55, .03)
    words = tuple(word for value in numeric_inputs for word in (struct.unpack('>q', struct.pack('>d', value))[0], 'finite'))
    insert = 'INSERT INTO din18599_document VALUES (1, ' + ', '.join('?' for _ in range(17)) + ')'

    def insert_use_class(use_class):
        sqlite.execute(insert, ('Residential', 'Detached', use_class, 'DetailedMonthly', *numeric_inputs, 'C', *words))

    for case in cases['cases']:
        insert_use_class(case['useClass'])
        assert sqlite.execute('SELECT use_class FROM din18599_document').fetchone() == (case['useClass'],)
        sqlite.execute('DELETE FROM din18599_document')

    try:
        insert_use_class('UnknownDwellingBasis')
    except sqlite3.IntegrityError:
        pass
    else:
        raise AssertionError('SQLite accepted an unknown use class')

    sqlite.execute('CREATE TABLE envelope (area REAL, factor REAL, actual REAL, reference REAL)')
    for case in envelope['cases']:
        sqlite.executemany(
            'INSERT INTO envelope VALUES (?,?,?,?)',
            [(row['areaM2'], row['factor'], actual, row['referenceUWM2k'])
             for row, actual in zip(envelope['components'], case['actualUWM2k'])],
        )
        actual, limit = sqlite.execute(
            'SELECT SUM(area * factor * actual)/SUM(area) + ?, SUM(area * factor * reference)/SUM(area) + ? FROM envelope WHERE reference IS NOT NULL',
            (envelope['deltaUWbWM2k'], 0.05),
        ).fetchone()
        assert math.isclose(actual, case['hTPrimeWM2k']) and math.isclose(limit, envelope['limitWM2k'])
        assert ('Pass' if actual <= limit else 'Fail') == case['status']
        sqlite.execute('DELETE FROM envelope')

model = r'''
#![allow(dead_code, unused_imports)]
#[derive(Clone, Copy, PartialEq)]
enum BuildingCategory { Residential, NonResidential }
#[derive(Clone, Copy, PartialEq)]
enum CalculationMethod { DetailedMonthly, Tabular }
#[derive(Clone, Copy, PartialEq)]
enum UsageProfile { WFH, Office, School }
#[derive(Clone, Copy, PartialEq)]
enum UseClass { Residential, Office, School, ResidentialUpToTwoDwellings, ResidentialMultifamily }
#[derive(Clone, Copy, PartialEq)]
enum Attachment { Detached, SemiDetached, EndTerrace, MidTerrace }
#[derive(Clone, Copy, PartialEq)]
enum AutomationClass { A, B, C, D }
#[derive(Clone, Copy, PartialEq)]
enum Adjacency { Outdoor, Ground, Unheated, Heated, UnheatedAttic }
#[derive(Clone, Copy, PartialEq)]
enum ElementKind { Wall, Roof, Floor, Door, Window, CurtainWall, RoofGlazing, RoofDome, GlazedDoor, GroundSlabPerimeter, GroundSlabInterior, RoofWindow, RooflightStrip }
#[derive(Clone, Copy, PartialEq)]
enum ThermalMassClass { Light, Medium, Heavy, Custom }

impl ElementKind {
    fn transmits_solar_radiation(self) -> bool {
        matches!(self, Self::Window | Self::CurtainWall | Self::RoofGlazing | Self::RoofDome | Self::GlazedDoor | Self::RoofWindow | Self::RooflightStrip)
    }
}

struct EnvelopeElement {
    adjacency: Adjacency, kind: ElementKind, zone_id: String, area_m2: f64, u_value_w_m2k: f64,
    orientation_deg: f64, tilt_deg: f64, g_value: f64, fc: f64,
}
struct ThermalZone {
    id: String, usage_profile: UsageProfile, occupants: u32, area_m2: f64, volume_m3: f64,
    theta_i_heat_c: f64, theta_i_cool_c: f64, internal_gains_w_m2: f64, lighting_power_w_m2: f64,
    thermal_mass_class: ThermalMassClass, c_wirk_wh_m2k: f64,
}
struct MonthlyClimate { theta_e_c: [f64; 12], g_h_w_m2: [f64; 12] }
extern crate self as semio_s_artifact_norm_din18599_model;
pub mod schema { pub mod snapshot {
    pub const POTSDAM_HORIZONTAL_IRRADIANCE_W_M2: [f64; 12] = [29.0,44.0,97.0,189.0,221.0,241.0,210.0,180.0,127.0,77.0,31.0,17.0];
}}
struct Ventilation { airflow_m3_h: f64, heat_recovery_eta: f64 }
struct Lighting { control_factor: f64 }
struct Heating { generation_efficiency: f64, distribution_efficiency: f64, storage_efficiency: f64, transfer_efficiency: f64 }
struct Dhw { dwelling_count: u32 }
struct Din18599Snapshot {
    dhw: Dhw,
    building_category: BuildingCategory, use_class: UseClass, method: CalculationMethod,
    elements: Vec<EnvelopeElement>, zones: Vec<ThermalZone>, delta_u_wb_w_m2k: f64,
    heated_volume_m3: f64, climate: MonthlyClimate, ventilation: Ventilation, lighting: Lighting,
    heating: Heating,
}
'''
main = r'''
fn main() {
    let mut doc = Din18599Snapshot {
        building_category: BuildingCategory::Residential, method: CalculationMethod::DetailedMonthly,
        dhw: Dhw { dwelling_count: 0 }, use_class: UseClass::Residential, elements: vec![], zones: vec![ThermalZone {
            id: "living".to_owned(), usage_profile: UsageProfile::WFH, occupants: 3, area_m2: AREA,
            volume_m3: 300.0, theta_i_heat_c: 20.0, theta_i_cool_c: 26.0, internal_gains_w_m2: 100.0,
            lighting_power_w_m2: 0.0, thermal_mass_class: ThermalMassClass::Medium, c_wirk_wh_m2k: 90.0,
        }], delta_u_wb_w_m2k: 0.03, heated_volume_m3: 300.0,
        climate: MonthlyClimate { theta_e_c: [0.0; 12], g_h_w_m2: [0.0; 12] },
        ventilation: Ventilation { airflow_m3_h: 50.0, heat_recovery_eta: 0.0 },
        lighting: Lighting { control_factor: 1.0 },
        heating: Heating { generation_efficiency: 1.0, distribution_efficiency: 1.0, storage_efficiency: 1.0, transfer_efficiency: 1.0 },
    };
    assert_eq!(residential_daily_gain_wh_m2(&doc), None);

    for (count, daily, average, jan, feb, annual) in CASES {
        doc.dhw.dwelling_count = count;
        let (_, months) = &zone_heating_months(&doc)[0];
        assert_eq!(residential_daily_gain_wh_m2(&doc), Some(daily));
        assert_eq!(internal_gains_w_m2(&doc, &doc.zones[0]), average);
        assert_eq!(months[0].q_i_kwh, jan);
        assert_eq!(months[1].q_i_kwh, feb);
        assert_eq!(months.iter().map(|month| month.q_i_kwh).sum::<f64>(), annual);
        assert!(months.iter().all(|month| month.q_h_kwh.is_finite() && month.q_c_kwh.is_finite()));
    }

    doc.zones[0].usage_profile = UsageProfile::Office;
    assert_eq!(internal_gains_w_m2(&doc, &doc.zones[0]), 100.0);
    doc.zones[0].usage_profile = UsageProfile::WFH;
    doc.method = CalculationMethod::Tabular;
    assert_eq!(residential_daily_gain_wh_m2(&doc), None);
    doc.method = CalculationMethod::DetailedMonthly;
    doc.building_category = BuildingCategory::NonResidential;
    assert_eq!(residential_daily_gain_wh_m2(&doc), None);

    doc.building_category = BuildingCategory::Residential;
    doc.delta_u_wb_w_m2k = ENVELOPE_BRIDGES;
    for (actual, expected, pass) in ENVELOPE_CASES {
        doc.elements = ENVELOPE_COMPONENTS.into_iter().zip(actual).map(|((adjacency, area), u)| EnvelopeElement {
            adjacency, area_m2: area, u_value_w_m2k: u, kind: ElementKind::Wall,
            zone_id: "living".to_owned(), orientation_deg: 0.0, tilt_deg: 90.0, g_value: 0.0, fc: 1.0,
        }).collect();
        let limit = geg_ht_prime_limit(&doc).unwrap();
        assert!((limit - ENVELOPE_LIMIT).abs() < 1e-12);
        assert!((h_t_prime(&doc) - expected).abs() < 1e-12);
        assert_eq!(h_t_prime(&doc) <= limit, pass);
    }
    doc.elements.retain(|e| e.adjacency == Adjacency::Heated);
    assert!(geg_ht_prime_limit(&doc).is_none());
}
'''
module_paths = {
    'norm_tables': checks / '📜️norm-tables/🦀️.rs',
    'residential_gains': checks / '🏘️residential-gains/🦀️.rs',
    'balance_model': checks / '🧮️balance-model/🦀️.rs',
    'potsdam_irradiance': checks / '☀️potsdam-irradiance/🦀️.rs',
}
modules = '\n'.join(
    f'#[path = {json.dumps(str(path), ensure_ascii=False)}] mod {name}; use {name}::*;' for name, path in module_paths.items()
)
rows = ','.join(
    '(' + str(case['dwellingCount']) + ',' + ','.join(
        str(float(case[key])) for key in ['dailyWhM2', 'averageWM2', 'januaryKWh', 'februaryKWh', 'annualKWh']
    ) + ')'
    for case in cases['cases']
)
vectors = f"const AREA: f64 = {float(cases['netFloorAreaM2'])};\nconst CASES: [(u32,f64,f64,f64,f64,f64); {len(cases['cases'])}] = [{rows}];"
envelope_components = ','.join(f"(Adjacency::{row['adjacency']},{float(row['areaM2'])})" for row in envelope['components'])
envelope_cases = ','.join(
    f"({[float(x) for x in case['actualUWM2k']]},{float(case['hTPrimeWM2k'])},{str(case['status'] == 'Pass').lower()})"
    for case in envelope['cases']
)
vectors += f'''
const ENVELOPE_BRIDGES: f64 = {float(envelope["deltaUWbWM2k"])};
const ENVELOPE_LIMIT: f64 = {float(envelope["limitWM2k"])};
const ENVELOPE_COMPONENTS: [(Adjacency, f64); {len(envelope["components"])}] = [{envelope_components}];
const ENVELOPE_CASES: [([f64; {len(envelope["components"])}], f64, bool); {len(envelope["cases"])}] = [{envelope_cases}];
'''
generated = Path(__file__).parent / '🗑️generated'
generated.mkdir(exist_ok=True)
with tempfile.TemporaryDirectory(prefix='din18599-', dir=generated) as tmp:
    source = Path(tmp) / 'main.rs'
    binary = Path(tmp) / ('residential-gains.exe' if os.name == 'nt' else 'residential-gains')
    source.write_text('\n'.join((model, modules, vectors, main)), encoding='utf-8')
    subprocess.run(['repo', 'tools', 'exec', '--light', 'rustup', '--', 'run', 'stable', 'rustc', '--edition', '2024', str(source), '-o', str(binary)], check=True, cwd=root)
    subprocess.run([str(binary)], check=True, cwd=root)
print('[DEBUG] The Table 4 gains and GEG § 16 envelope worked cases matched production Rust and independent SQLite.')
