//! 📜️ DIN V 18599 and GEG norm tables: primary energy factors, reference U-values, hours, air change and profile constants.

use super::*;

//#region 📜️NormTables
/// ⛽ GEG Anlage 4 — Primärenergiefaktoren f_P [-].
pub mod geg_anlage4_primary_energy_factors {
    pub const NATURAL_GAS: f64 = 1.1;
    pub const HEATING_OIL: f64 = 1.1;
    pub const ELECTRICITY_GRID: f64 = 1.8;
    pub const DISTRICT_HEATING: f64 = 0.7;
    pub const BIOMASS: f64 = 0.2;
}

/// 🧱 GEG Anlage 2 — Höchstwerte der Wärmedurchgangskoeffizienten U_ref [W/(m²·K)] (Wohngebäude, Neubau).
pub mod geg_anlage2_reference_u {
    pub const WALL: f64 = 0.28;
    pub const ROOF: f64 = 0.20;
    pub const FLOOR: f64 = 0.35;
    pub const WINDOW: f64 = 1.3;
    pub const DOOR: f64 = 1.8;
}

/// 🧱 GEG Anlage 3 — mittlere U-Werte Nichtwohngebäude [W/(m²·K)].
pub mod geg_anlage3_mean_u {
    pub const WALL: f64 = 0.28;
    pub const ROOF: f64 = 0.20;
    pub const FLOOR: f64 = 0.35;
    pub const WINDOW: f64 = 1.5;
    pub const DOOR: f64 = 1.8;
}

/// 🏗️ GEG Anlage 1 Tabelle 1 — Referenzausführung Lüftung mit WRG η_WRG [-].
pub const GEG_ANLAGE1_TABELLE1_ETA_WRG_REF: f64 = 0.80;

/// 🌡️ GEG Anlage 1 / Anlage 2 — Wärmebrückenzuschlag der Referenzausführung ΔU_WB [W/(m²·K)].
pub const GEG_ANLAGE1_DELTA_U_WB_REF: f64 = 0.05;

/// 🔥 GEG Anlage 1 Tabelle 1 — Teillastwirkungsgrade der Referenz-Wärmeerzeugung/-verteilung/-speicherung/-übergabe [-].
pub mod geg_anlage1_tabelle1_heating_eta {
    pub const GENERATION: f64 = 0.95;
    pub const DISTRIBUTION: f64 = 0.95;
    pub const STORAGE: f64 = 0.98;
    pub const TRANSFER: f64 = 0.95;
    pub const SYSTEM_PRODUCT: f64 = GENERATION * DISTRIBUTION * STORAGE * TRANSFER;
}

/// 🪟 GEG Anlage 1 — Referenz-g-Wert Fenster [-].
pub const GEG_ANLAGE1_WINDOW_G_REF: f64 = 0.60;

/// ⏱️ DIN V 18599-10 Nutzungsprofile — jährliche Betriebsstunden mechanischer Lüftung [h/a].
pub mod din_v_18599_10_fan_hours {
    pub const RESIDENTIAL_WFH: f64 = 5000.0;
    pub const OFFICE: f64 = 2500.0;
    pub const SCHOOL: f64 = 2000.0;
}

/// 💡 DIN V 18599-4 — spezifische elektrische Bewertungsleistung p_LX [W/m²] (Grenzwerte je Nutzungsprofil).
pub mod din_v_18599_4_lighting_power_density {
    pub const RESIDENTIAL: f64 = 10.0;
    pub const OFFICE: f64 = 12.0;
    pub const SCHOOL: f64 = 15.0;
}

/// 🚿 DIN V 18599-10 — Nutzwärmebedarf Trinkwarmwasser q_w [kWh/(Person·a)].
pub mod din_v_18599_10_dhw_specific {
    pub const RESIDENTIAL: f64 = 500.0;
    pub const OFFICE: f64 = 100.0;
    pub const SCHOOL: f64 = 150.0;
}

/// 🚿 DIN V 18599-8 — relative Verlustzuschläge auf die Nutzenergie (Speicher / Verteilung).
pub mod din_v_18599_8_dhw_loss_fractions {
    pub const STORAGE_OF_USEFUL: f64 = 0.25;
    pub const DISTRIBUTION_OF_USEFUL: f64 = 0.20;
    pub const STORAGE_FLOOR_KWH_A: f64 = 50.0;
    pub const DISTRIBUTION_FLOOR_KWH_A: f64 = 30.0;
}

/// 📊 DIN V 18599-12:2018 Tabelle 5 — spezifische Primärenergiekennwerte q_p,tab [kWh/(m²·a)] (Tabellenverfahren, vor f_BAC).
pub mod din_v_18599_12_tabelle5_qp_specific {
    pub const RESIDENTIAL: f64 = 66.0;
    pub const OFFICE: f64 = 100.0;
    pub const SCHOOL: f64 = 85.0;
}

/// ⚙️ DIN V 18599-5 — Mindest-Gesamtaufwandzahl / Erzeugernutzungsgrad-Produkt η_sys,min [-].
pub const DIN_V_18599_5_ETA_SYS_MIN: f64 = 0.85;

/// 🧊 DIN V 18599-7 — Kühlbedarf-Grenze als Anteil am Heizwärmebedarf [-].
pub const DIN_V_18599_7_COOLING_TO_HEATING_RATIO: f64 = 0.5;

/// 🪟 DIN V 18599-7 — Abminderung g-Wert bei Kühlüberschreitung (Multiplikator / Untergrenze).
pub mod din_v_18599_7_g_remedy {
    pub const REDUCE_FACTOR: f64 = 0.7;
    pub const FLOOR: f64 = 0.3;
}

/// 📐 GEG Anlage 2 — Höchstwerte H′T [W/(m²·K)] nach Gebäudegröße und Anbauart.
pub mod geg_anlage2_ht_prime_limits {
    pub const DETACHED_AN_LE_350: f64 = 0.40;
    pub const DETACHED_AN_GT_350: f64 = 0.50;
    pub const SEMI_OR_END_AN_LE_350: f64 = 0.45;
    pub const SEMI_OR_END_AN_GT_350: f64 = 0.50;
    pub const MID_TERRACE: f64 = 0.50;
    pub const AN_THRESHOLD_M2: f64 = 350.0;
}

/// 🎛️ DIN V 18599-11 — Korrekturfaktoren f_BAC auf den Endenergiebedarf nach Automationsklasse.
pub mod din_v_18599_11_automation_factor {
    pub const CLASS_A: f64 = 0.88;
    pub const CLASS_B: f64 = 0.93;
    pub const CLASS_C: f64 = 0.97;
    pub const CLASS_D: f64 = 1.00;
}

/// 🧱 DIN V 18599-2 — Temperaturkorrekturfaktoren F_x für angrenzende Bereiche [-].
pub mod din_v_18599_2_adjacency_fx {
    pub const OUTDOOR: f64 = 1.0;
    pub const GROUND: f64 = 0.6;
    pub const UNHEATED: f64 = 0.5;
    pub const HEATED: f64 = 0.0;
}

/// 🌡️ DIN V 18599-10 — Standard-Raumtemperaturen / innere Gewinne wenn keine Zone vorliegt.
pub mod din_v_18599_10_zone_defaults {
    pub const THETA_I_HEAT_C: f64 = 20.0;
    pub const THETA_I_COOL_C: f64 = 26.0;
    pub const INTERNAL_GAINS_W_M2: f64 = 3.5;
}

/// ☀️ DIN V 18599-2 — Orientierung-/Neigungsfaktoren für solare Einstrahlung (Monatsbilanz).
pub mod din_v_18599_2_solar_geometry_factors {
    pub const SOUTH_HORIZ: f64 = 1.0;
    pub const EAST_WEST_HORIZ: f64 = 0.7;
    pub const NORTH_HORIZ: f64 = 0.4;
    pub const LOW_TILT: f64 = 0.9;
    pub const STEEP_TILT: f64 = 1.0;
    pub const MID_TILT: f64 = 0.95;
}

/// 📊 DIN V 18599-2 — Ausnutzungsgrad der Gewinne (Zeitkonstante-Parameter a).
pub const DIN_V_18599_2_UTILIZATION_A: f64 = 0.95;

/// 🌬️ DIN V 18599-1 — ρ·c_a Volumenstrom-Kennwert [Wh/(m³·K)] ≡ [W/(m³/h·K)].
pub const DIN_V_18599_1_RHO_CA: f64 = 0.34;

/// ⏱️ Mittlere Monatsstunden für die Monatsbilanz [h].
pub const DIN_V_18599_1_HOURS_PER_MONTH: f64 = 730.0;

/// 🔥 Numerische Untergrenze für η_sys (Schutz vor Division durch 0) [-].
pub const DIN_V_18599_5_ETA_SYS_FLOOR: f64 = 0.05;

/// ⚖️ GEG §10 — Primärenergie-Grenzwertfaktor auf Q_P,Ref (typisch 0,55 für Wohngebäude-Neubau).
pub const GEG_SECTION10_QP_FACTOR_DEFAULT: f64 = 0.55;

/// 💡 DIN V 18599-10 — jährliche Beleuchtungsvollbenutzungsstunden t_L [h/a] je Nutzungsprofil.
pub mod din_v_18599_10_lighting_hours {
    pub const WFH: f64 = 1700.0;
    pub const OFFICE: f64 = 2500.0;
    pub const SCHOOL: f64 = 1800.0;
}

/// 🌬️ DIN V 18599-10 — Außenluftwechsel n_Außen [1/h] bezogen auf Nettovolumen.
pub mod din_v_18599_10_outdoor_air_change {
    pub const WFH: f64 = 0.5;
    pub const OFFICE: f64 = 1.0;
    pub const SCHOOL: f64 = 1.5;
}

/// 📐 DIN V 18599-1 §6.1.1 — Nettoluftvolumen / Bruttovolumen V/V_e (Standardfaktor).
pub const DIN_V_18599_1_NET_TO_GROSS_VOLUME_RATIO: f64 = 0.80;

/// 🌬️ DIN V 18599-2 — Infiltrationsluftwechsel n_inf bezogen auf V_e [1/h].
pub const DIN_V_18599_2_INFILTRATION_N_INF: f64 = 0.2;
//#endregion 📜️NormTables
