
# Internal Heat Gains Calculation Engine

This document explains the physics and logic used in the Rust calculation engine (`lib.rs` / `internal_gains` module) to compute the building's internal heat gains ($Q_I$) and sinks according to DIN/TS 18599-2:2025-10 (Section 6.5) and DIN V 18599-10.

## 1. Data Flow & Architecture

The internal gains engine calculates the "free" heat generated inside the building (sources) and the heat actively removed by specific equipment (sinks). The engine supports two parallel pathways:

1. **The Standardized Method (DIN 18599-2 / 6.5):** Uses generic flat rates ($W/m^2$) based on usage profiles, handles material transport mass flows, and adjusts for exhaust-air lighting systems. Required for official Energy Certificates.
2. **The Detailed Custom Method:** Allows users to define the exact number of people and specific electrical equipment in a room for precise load sizing.

```
flowchart TD
    %% UI Inputs
    subgraph UI ["User Interface (Inputs)"]
        G["Geometry Engine\n(A_NGF)"]
        Q1[Usage Profile\n(Standard Method)]
        Q2[Custom Inventory\n(Detailed Method)]
        Q3[Material Transport\n(Mass Flow, Temp)]
        Q4[Lighting System\n(Standard vs Exhaust)]
    end

    %% Database / Norms
    subgraph DB ["DIN 18599-10 & Physics"]
        Q_RES["Residential: q_I"]
        Q_NRES["Non-Res: q_I,p & q_I,app"]
        Q_SINK["Heat Sinks: q_I,sink,app"]
        MU_L["Room Load Factor (mu_l)"]
        C_MAT["Specific Heat Capacity (c)"]
    end

    %% Detailed Calculations
    subgraph Calc ["Energy Balances (Daily Q_I)"]
        Q_G["Q_I,source,G (Residential Combined)"]
        Q_P["Q_I,source,p (People)"]
        Q_APP["Q_I,source,app (Equipment Sources)"]
        Q_SINK_APP["Q_I,sink,app (Equipment Sinks)"]
        Q_GOODS["Q_I,source/sink,goods (Material)"]
        Q_L["Q_I,source,l (Lighting)"]
    end

    %% Final Output
    QI["Q_I Total\nTotal Net Internal Heat Gain"]

    Q1 --> Q_RES
    Q1 --> Q_NRES
    Q1 --> Q_SINK
    Q3 --> C_MAT
    Q4 --> MU_L

    G --> Q_G
    G --> Q_P
    G --> Q_APP
    G --> Q_SINK_APP

    Q_RES --> Q_G
    Q_NRES --> Q_P
    Q_NRES --> Q_APP
    Q_SINK --> Q_SINK_APP
    C_MAT --> Q_GOODS
    MU_L --> Q_L

    Q_G --> QI
    Q_P --> QI
    Q_APP --> QI
    Q_L --> QI
    Q_GOODS --> QI
    Q_SINK_APP --> |Subtract| QI
```

## 2. Inputs Required from the UI

### A. Automated Geometry

* `a_ngf`: Net floor area / Nettogrundfläche ($A_{NGF}$ in $m^2$).

### B. User Questions (Parameters)

| Mode               | UI Prompts                                               | Available Options                                                          | How Rust uses it                                                                        |
| ------------------ | -------------------------------------------------------- | -------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| **Standard** | "What is the primary usage?"                             | Residential, Office, Retail, etc.                                          | Fetches$q_{I,p}$,$q_{I,app}$, and$q_{I,sink,app}$from DIN 18599-10.               |
| **Standard** | "What kind of lighting fixtures are installed?"          | - Standard (No Exhaust)- Exhaust via Ceiling Cavity- Exhaust via Air Ducts | Maps to the$\mu_l$room load factor (1.0, 0.75, or 0.65) to reduce lighting heat gain. |
| **Material Transport** | "Are large amounts of cold or hot materials regularly brought into this space?" | Type (e.g., Frozen Goods, Metal) & Volume (None / Pallets / Truckloads) | Infers mass flow ($\dot{m}$) and $\Delta T$ based on typical material properties and volume. |
| **Detailed** | "How many people are usually in this room?"              | Number input                                                               | Multiplies count by standard metabolic heat (e.g., 80W).                                |
| **Detailed** | "Add electrical equipment"                               | Add item (Name, Watts, Duty Cycle)                                         | Multiplies item count by wattage and duty-cycle.                                        |

## 3. Core Calculations (DIN/TS 18599-2 Section 6.5)

### 3.1 Residential Buildings (Wohngebäude)

For residential buildings, heat from persons, equipment, and lighting must be bundled into a single generalized value ($q_I$). Cooling sinks are neglected.
**Formula (Eq. 126):**
$Q_{I,source,G} = q_I \cdot A_{NGF}$

### 3.2 Non-Residential Persons & Equipment (Arbeitshilfen)

For non-residential zones, heat sources are split.

* **Persons (Eq. 127):** $Q_{I,source,p} = q_{I,p} \cdot A_{NGF}$
* **Equipment Sources (Eq. 128):** $Q_{I,source,app} = q_{I,app} \cdot A_{NGF}$

 **Equipment Sinks (** $Q_{I,sink,app}$**):**
Equipment that generates cold internally but exhausts its heat *outside* the zone (e.g., supermarket refrigerated displays with external compressors) acts as a thermal sink.

* **Equipment Sinks (Eq. 129):** $Q_{I,sink,app} = q_{I,sink,app} \cdot A_{NGF}$

### 3.3 Material Transport (Stofftransport)

If materials/goods are brought into the zone at a temperature significantly different from the room temperature, they absorb or release heat.

* $\dot{m}$: Mass flow rate ($\dot{m} = m_{24} / 24$).
* $c$: Specific heat capacity of the material.
* **Source (Hot goods enter,** $\theta_{in} > \theta_{out}$**):** $Q_{I,source,goods} = c \cdot \dot{m} \cdot (\theta_{in} - \theta_{out}) \cdot t$ (Eq. 130)
* **Sink (Cold goods enter,** $\theta_{in} < \theta_{out}$**):** $Q_{I,sink,goods} = c \cdot \dot{m} \cdot (\theta_{out} - \theta_{in}) \cdot t$ (Eq. 131)

### 3.4 Artificial Lighting (Künstliche Beleuchtung)

The heat from lighting depends on the electrical energy demand ($Q_{l,f}$) from DIN 18599-4. If the building uses exhaust air luminaires ( *Abluftleuchten* ), a portion of the heat is immediately removed via the ventilation system, reducing the room load factor ($\mu_l$).

* **Lighting Heat (Eq. 132):** $Q_{I,source,l} = \mu_l \cdot Q_{l,f}$
  *(Without exhaust luminaires,* $\mu_l = 1.0$*)*

### 3.5 Custom Inventory Calculation (Detailed Method)

If the user defines exact items, we bypass the $q_I \cdot A_{NGF}$ calculation and calculate total Wattage ($\Phi_I$) directly.

* **Persons:** $\Phi_{I,p} = N_{people} \cdot \dot{Q}_{person}$ (e.g., 80W per person).
* **Equipment:** $\Phi_{I,eq} = \sum (N_{item} \cdot P_{item} \cdot f_{duty})$

## 4. Reference Data & Standard Values

### Table A: Standardized Heat Loads (DIN 18599-10)

This table extracts the precise values specified by DIN V 18599-10 for generating official and legally binding heating and cooling load profiles.

| Lfd. Nr. | Usage Profile (Nutzung)              | $q_{I,p}$ $[W/m^2]$ | $q_{I,app}$ $[W/m^2]$ | $q_{I,sink,app}$ $[W/m^2]$ | $t_{nutz}$ $[h/d]$ | $d_{nutz}$ $[d/a]$ |
| -------- | ------------------------------------ | ----------------------- | ------------------------- | ------------------------------ | ---------------------- | ---------------------- |
| -        | **Residential (Wohngebäude)** | *Combined 3.75*       | -                         | `0.0`                        | `24`                 | `365`                |
| 1        | Einzelbüro (Single Office)          | `4.50`                | `7.00`                  | `0.0`                        | `13`                 | `250`                |
| 2        | Gruppenbüro (Group Office)          | `5.00`                | `7.00`                  | `0.0`                        | `13`                 | `250`                |
| 3        | Großraumbüro (Open Plan Office)    | `6.00`                | `9.00`                  | `0.0`                        | `13`                 | `250`                |
| 4        | Besprechung, Seminar (Meeting)       | `15.00`               | `2.00`                  | `0.0`                        | `13`                 | `250`                |
| 6        | Einzelhandel/Kaufhaus (Retail)       | `7.00`                | `2.00`                  | `0.0`                        | `12`                 | `300`                |
| 7        | Einzelhandel (Lebensmittel/Kühl)    | `7.00`                | `2.00`                  | `10.0`                       | `12`                 | `300`                |
| 8        | Klassenzimmer (Classroom)            | `15.00`               | `2.00`                  | `0.0`                        | `12`                 | `250`                |
| 10       | Bettenzimmer (Hospital Room)         | `4.00`                | `2.00`                  | `0.0`                        | `24`                 | `365`                |
| 11       | Hotelzimmer (Hotel Room)             | `3.00`                | `2.00`                  | `0.0`                        | `24`                 | `365`                |
| 13       | Restaurant                           | `12.00`               | `5.00`                  | `0.0`                        | `14`                 | `365`                |
| 14       | Küchen (Commercial Kitchen)         | `5.00`                | `20.00`                 | `0.0`                        | `14`                 | `365`                |
| 21       | Rechenzentrum (Data Center)          | `1.00`                | `150.00`                | `0.0`                        | `24`                 | `365`                |
| 33       | Turnhalle (Gymnasium)                | `10.00`               | `0.00`                  | `0.0`                        | `14`                 | `300`                |
| 43       | Lagerhallen (Warehouse)              | `1.00`                | `1.00`                  | `0.0`                        | `14`                 | `250`                |

### Table B: Room Load Factor for Lighting ($\mu_l$)

| Lighting Exhaust Type                       | Room Load Factor$\mu_l$      |
| ------------------------------------------- | ------------------------------ |
| Standard Luminaires (No Exhaust)            | `1.0`                        |
| Abluftleuchten (Exhaust via Ceiling Cavity) | `0.75`(Average of 0.7 - 0.8) |
| Abluftleuchten (Exhaust via Air Ducts)      | `0.65`(Average of 0.6 - 0.7) |

## 5. Detailed Rust Implementation

This Rust architecture supports the strict separation required by DIN 18599-2, handling both sources, sinks, material flows, and explicitly categorized exhaust lighting.

```
// --- DATA MODELS ---

/// Represents the standard usage profile data derived strictly from DIN V 18599-10.
#[derive(Debug, Clone, Copy)]
pub struct StandardGainProfile {
    pub is_residential: bool,
    pub q_i_combined: f64, // Used only if residential (Eq. 126)
    pub q_i_p: f64,        // Heat from persons (W/m²)
    pub q_i_app: f64,      // Heat from equipment/work aids (W/m²)
    pub q_i_sink_app: f64, // Sinks from equipment (W/m²)
    pub t_nutz: f64,       // Daily usage hours (h/d)
    pub d_nutz: f64,       // Annual usage days (d/a)
}

impl StandardGainProfile {
    /// Factory method to load standard DIN 18599-10 profiles
    pub fn from_profile_id(id: u32) -> Self {
        match id {
            0 => Self { // 0 = Residential fallback
                is_residential: true,
                q_i_combined: 3.75,
                q_i_p: 0.0, q_i_app: 0.0, q_i_sink_app: 0.0,
                t_nutz: 24.0, d_nutz: 365.0,
            },
            1 => Self { // Einzelbüro
                is_residential: false, q_i_combined: 0.0,
                q_i_p: 4.50, q_i_app: 7.00, q_i_sink_app: 0.0,
                t_nutz: 13.0, d_nutz: 250.0,
            },
            3 => Self { // Großraumbüro
                is_residential: false, q_i_combined: 0.0,
                q_i_p: 6.00, q_i_app: 9.00, q_i_sink_app: 0.0,
                t_nutz: 13.0, d_nutz: 250.0,
            },
            4 => Self { // Besprechung
                is_residential: false, q_i_combined: 0.0,
                q_i_p: 15.00, q_i_app: 2.00, q_i_sink_app: 0.0,
                t_nutz: 13.0, d_nutz: 250.0,
            },
            7 => Self { // Einzelhandel (Kühl)
                is_residential: false, q_i_combined: 0.0,
                q_i_p: 7.00, q_i_app: 2.00, q_i_sink_app: 10.0,
                t_nutz: 12.0, d_nutz: 300.0,
            },
            8 => Self { // Klassenzimmer
                is_residential: false, q_i_combined: 0.0,
                q_i_p: 15.00, q_i_app: 2.00, q_i_sink_app: 0.0,
                t_nutz: 12.0, d_nutz: 250.0,
            },
            14 => Self { // Küchen (Gewerblich)
                is_residential: false, q_i_combined: 0.0,
                q_i_p: 5.00, q_i_app: 20.00, q_i_sink_app: 0.0,
                t_nutz: 14.0, d_nutz: 365.0,
            },
            21 => Self { // Rechenzentrum
                is_residential: false, q_i_combined: 0.0,
                q_i_p: 1.00, q_i_app: 150.00, q_i_sink_app: 0.0,
                t_nutz: 24.0, d_nutz: 365.0,
            },
            43 => Self { // Lagerhallen
                is_residential: false, q_i_combined: 0.0,
                q_i_p: 1.00, q_i_app: 1.00, q_i_sink_app: 0.0,
                t_nutz: 14.0, d_nutz: 250.0,
            },
            // Default generic non-residential fallback
            _ => Self {
                is_residential: false, q_i_combined: 0.0,
                q_i_p: 3.0, q_i_app: 3.0, q_i_sink_app: 0.0,
                t_nutz: 12.0, d_nutz: 250.0,
            }
        }
    }
}

/// Represents the transport of physical goods/materials in and out of the zone.
#[derive(Debug, Clone, Copy)]
pub struct MaterialTransport {
    pub c_specific_heat: f64, // c in Wh/(kg*K)
    pub m_dot: f64,           // mass flow rate (kg/h)
    pub theta_in: f64,        // Temperature of goods entering
    pub theta_out: f64,       // Temperature of goods leaving (usually room temp)
}

/// Categorizes the type of lighting exhaust based on DIN 18599-2 Table 13.
#[derive(Debug, Clone, Copy)]
pub enum LightingExhaustType {
    Standard,      // No exhaust, standard luminaires
    CeilingCavity, // Abluftleuchten über Deckenhohlraum
    AirDucts,      // Abluftleuchten über Luftleitung
}

impl LightingExhaustType {
    /// Returns the room load factor (mu_l) associated with the exhaust type.
    pub fn room_load_factor(&self) -> f64 {
        match self {
            Self::Standard => 1.0,
            Self::CeilingCavity => 0.75, // Average range (0.7 - 0.8)
            Self::AirDucts => 0.65,      // Average range (0.6 - 0.7)
        }
    }
}

/// Represents the artificial lighting system.
#[derive(Debug, Clone, Copy)]
pub struct LightingSystem {
    pub q_l_f_daily: f64,                 // Daily electrical energy demand for lighting (Wh)
    pub exhaust_type: LightingExhaustType, // Simplifies user input to specific categories
}

impl LightingSystem {
    /// Calculates the effective room load factor based on the system type.
    pub fn mu_l(&self) -> f64 {
        self.exhaust_type.room_load_factor()
    }
}

/// Represents the custom detailed inventory of a room.
#[derive(Debug, Clone)]
pub struct CustomInventoryProfile {
    pub num_people: u32,
    pub metabolic_rate_watts: f64,
    pub equipment_watts_active: f64, // Sum of (Count * Power * DutyCycle)
    pub t_nutz: f64,
}

// --- ENGINE ---

pub enum GainCalculationMethod {
    Standard(StandardGainProfile),
    Custom(CustomInventoryProfile),
}

/// The core engine struct to calculate total internal gains
pub struct InternalGainsEngine {
    pub a_ngf: f64, // Net floor area (m²)
    pub method: GainCalculationMethod,
    pub material_transport: Option<MaterialTransport>,
    pub lighting: LightingSystem,
}

impl InternalGainsEngine {
    pub fn new(
        a_ngf: f64, 
        method: GainCalculationMethod, 
        material_transport: Option<MaterialTransport>,
        lighting: LightingSystem
    ) -> Self {
        Self { a_ngf, method, material_transport, lighting }
    }

    /// Calculates the daily internal energy gains and sinks (Wh/day).
    pub fn daily_energy_balance_wh(&self) -> (f64, f64) {
        let mut total_sources_wh = 0.0;
        let mut total_sinks_wh = 0.0;

        match &self.method {
            GainCalculationMethod::Standard(profile) => {
                if profile.is_residential {
                    // Eq. 126: Combined residential source
                    total_sources_wh += (profile.q_i_combined * self.a_ngf) * profile.t_nutz;
                } else {
                    // Eq. 127 & 128: Split non-residential sources
                    total_sources_wh += (profile.q_i_p * self.a_ngf) * profile.t_nutz;
                    total_sources_wh += (profile.q_i_app * self.a_ngf) * profile.t_nutz;
                    // Eq. 129: Equipment sinks
                    total_sinks_wh += (profile.q_i_sink_app * self.a_ngf) * profile.t_nutz;
                }
            },
            GainCalculationMethod::Custom(inventory) => {
                // Custom wattage * hours
                let person_heat = (inventory.num_people as f64) * inventory.metabolic_rate_watts;
                total_sources_wh += (person_heat + inventory.equipment_watts_active) * inventory.t_nutz;
            }
        }

        // Eq. 132: Artificial Lighting (adjusted dynamically by mu_l)
        total_sources_wh += self.lighting.mu_l() * self.lighting.q_l_f_daily;

        // Eq. 130 & 131: Material Transport (Stofftransport)
        if let Some(mat) = &self.material_transport {
            // t = 24h as per the mass flow definition in standard
            let t = 24.0; 
            if mat.theta_in > mat.theta_out {
                // Eq. 130: Source
                total_sources_wh += mat.c_specific_heat * mat.m_dot * (mat.theta_in - mat.theta_out) * t;
            } else if mat.theta_in < mat.theta_out {
                // Eq. 131: Sink
                total_sinks_wh += mat.c_specific_heat * mat.m_dot * (mat.theta_out - mat.theta_in) * t;
            }
        }

        (total_sources_wh, total_sinks_wh)
    }

    /// Returns the net daily internal gain (Sources - Sinks)
    pub fn net_daily_gain_wh(&self) -> f64 {
        let (sources, sinks) = self.daily_energy_balance_wh();
        // Prevent negative gains from overturning the balance completely, 
        // though strictly DIN tracks them separately for heating vs cooling balances.
        f64::max(0.0, sources - sinks)
    }
}

## Simplified UI Data Mapping (The Daily Rhythm)

By asking chronological and physical questions, the user's brain can "walk through" their building, keeping the frontend conversational while the backend remains strictly DIN-compliant.

**Usage Profile & Automation Class**
Instead of asking for a specific DIN 18599-10 profile directly, we ask:
*   "What is the primary use of this space?" -> This maps to the usage profile (e.g., Residential, Office). This unlocks heating setpoints, usage hours, and internal heat gains.
*   "How do you control the heating in this space?" -> Maps to the Automation Class. For example, "Manual radiator knobs" translates to Class C, while "Smart Home system" translates to Class A or B, lowering the heating setpoint automatically in the engine.
```
