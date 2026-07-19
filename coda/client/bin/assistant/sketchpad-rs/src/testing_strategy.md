# Real-World Correctness Testing Suite for DIN V 18599 Engine

## Problem

The existing `engine_tests` module validates the engine doesn't crash on edge cases. It does **not** validate that the engine produces physically correct numbers. This plan builds a correctness testing layer.

## Proposed Changes

### Reference Building Test Infrastructure

Since the tests must live inside `lib.rs` (per repo rules — no additional test files), I will add two new `#[cfg(test)]` modules:
`mod real_world_tests` and `mod sensitivity_tests` at the end of the file.

---

### Part 1: Reference Building Cases (5 cases)

Each reference case is defined as a helper function returning `(State, ExpectedOutputs)`. The expected values come from **hand calculations** using the DIN V 18599 formulas with the Potsdam reference climate already embedded in the engine.

#### Case 1: `simple_box_single_zone`

| Parameter      | Value                                 | Source                    |
| -------------- | ------------------------------------- | ------------------------- |
| Geometry       | 10m × 10m × 2.8m single story         | Hand-defined              |
| Floor Area     | 100 m²                                | 10 × 10                   |
| Wall Area      | 4 × (10 × 2.8) = 112 m²               | Perimeter × height        |
| Roof Area      | 100 m²                                | Flat roof                 |
| Windows        | 16.8 m² south (15% WWR of south wall) | WWR = 0.15                |
| U-wall         | 0.28 W/(m²K)                          | TABULA SFH 2016+ existing |
| U-roof         | 0.20 W/(m²K)                          | TABULA SFH 2016+ existing |
| U-floor        | 0.35 W/(m²K)                          | TABULA SFH 2016+ existing |
| U-window       | 1.10 W/(m²K)                          | TABULA SFH 2016+ existing |
| Climate        | Potsdam (region 4)                    | DIN V 18599-10 reference  |
| Heating system | Gas Condensing Boiler                 | Standard residential      |
| θ_i            | 20°C                                  | DIN V 18599-10 Tab.4      |
| n_min          | 0.5 1/h                               | DIN V 18599-10 Tab.4      |

**Hand-calculated expected transmission coefficient:**

- $H_{T,walls} = A_{wall,net} \cdot U_{wall} = (112 - 16.8) \cdot 0.28 \approx 26.7$ W/K
- $H_{T,roof} = 100 \cdot 0.20 = 20.0$ W/K
- $H_{T,floor} = 100 \cdot 0.35 \cdot 0.5 = 17.5$ W/K (F_x=0.5 for unheated basement)
- $H_{T,win} = 16.8 \cdot 1.10 = 18.5$ W/K
- $H_{T,total} \approx 82.7$ W/K (+ thermal bridges ~10%)
- Heating demand: ~4,000–8,000 kWh/a (well-insulated modern house, Potsdam climate)

#### Case 2: `detached_house_eh55`

Modern EH55-level house with enhanced insulation over TABULA defaults.

| Parameter                  | Value                         | Note              |
| -------------------------- | ----------------------------- | ----------------- |
| Floor Area                 | 140 m² (10m × 14m)            | Explicit override |
| Stories                    | 2                             | Explicit override |
| Custom wall insulation     | +120mm EPS (λ=0.035)          | Explicit override |
| Custom roof insulation     | +200mm mineral wool (λ=0.040) | Explicit override |
| Windows                    | Triple glazing, U=0.9         | Explicit override |
| Heating                    | Air Source Heat Pump          | Explicit override |
| Ventilation                | Mechanical, 80% heat recovery | Explicit override |
| Building Type              | SFH                           | Implicit default  |
| Year class                 | `2016-...`                    | Implicit default  |
| Scenario                   | Existing State                | Implicit default  |
| Climate                    | Potsdam (Region 4)            | Implicit default  |
| Usage Profile              | Residential                   | Implicit default  |
| Story Height               | 2.8m                          | Implicit default  |
| WWR (Window-to-Wall Ratio) | 15%                           | Implicit default  |

#### Case 3: `detached_house_altbau`

Pre-1918 unrenovated house.

| Parameter                  | Value                     | Note              |
| -------------------------- | ------------------------- | ----------------- |
| Year class                 | `...1859`                 | Explicit override |
| Scenario                   | Existing State            | Explicit override |
| Floor Area                 | 120 m²                    | Explicit override |
| Heating                    | Gas Non-Condensing Boiler | Explicit override |
| No custom insulation       | Uses raw TABULA values    | Explicit override |
| Building Type              | SFH                       | Implicit default  |
| Climate                    | Potsdam (Region 4)        | Implicit default  |
| Usage Profile              | Residential               | Implicit default  |
| Story Height               | 2.8m                      | Implicit default  |
| WWR (Window-to-Wall Ratio) | 15%                       | Implicit default  |
| Ventilation                | Natural (CategoryI)       | Implicit default  |

#### Case 4: `office_building_multizone`

Multi-zone office with mechanical ventilation.

| Parameter                  | Value                                              | Note              |
| -------------------------- | -------------------------------------------------- | ----------------- |
| Zones                      | 2 zones: Einzelbüro (60m²) + Verkehrsfläche (20m²) | Explicit override |
| Year class                 | `2002-2009`                                        | Explicit override |
| Ventilation                | Mechanical balanced, 70% HR                        | Explicit override |
| Internal gains             | Per DIN V 18599-10 Table 5 profile 1               | Explicit override |
| Building Type              | SFH                                                | Implicit default  |
| Scenario                   | Existing State                                     | Implicit default  |
| Climate                    | Potsdam (Region 4)                                 | Implicit default  |
| Story Height               | 2.8m                                               | Implicit default  |
| Heating                    | Gas Condensing Boiler                              | Implicit default  |
| WWR (Window-to-Wall Ratio) | 15%                                                | Implicit default  |

#### Case 5: `school_building`

Larger building, high occupancy.

| Parameter                  | Value                     | Note              |
| -------------------------- | ------------------------- | ----------------- |
| Profile                    | Klassenzimmer (profile 8) | Explicit override |
| Floor Area                 | 500 m²                    | Explicit override |
| Year class                 | `1969-1978`               | Explicit override |
| Occupancy-driven gains     | 100 Wh/(m²d) persons      | Explicit override |
| Building Type              | SFH                       | Implicit default  |
| Scenario                   | Existing State            | Implicit default  |
| Climate                    | Potsdam (Region 4)        | Implicit default  |
| Story Height               | 2.8m                      | Implicit default  |
| Heating                    | Gas Condensing Boiler     | Implicit default  |
| Ventilation                | Natural (CategoryI)       | Implicit default  |
| WWR (Window-to-Wall Ratio) | 15%                       | Implicit default  |

---

### Part 2: Sensitivity ("Delta") Tests

For each reference case, I will implement functions that:

1. Run the baseline
2. Apply exactly **one** modification
3. Assert **direction** + **isolation** + **plausible magnitude**

#### Required Sensitivity Cases

| #   | Modification                                   | Assertion                                  |
| --- | ---------------------------------------------- | ------------------------------------------ |
| S1  | Wall U +50%                                    | `Q_H ↑`, `Q_lighting` unchanged            |
| S2  | Wall U −50%                                    | `Q_H ↓`, stays ≥ 0                         |
| S3  | Window U −30%                                  | `Q_H ↓`, solar gains unchanged             |
| S4  | Infiltration ×2                                | `Q_H ↑`, transmission unchanged            |
| S5  | Heat recovery 0→80%                            | `Q_H ↓`                                    |
| S6  | HVAC swap: gas→heat pump                       | `Q_H_nd` unchanged, primary energy changes |
| S7  | Monotonicity sweep: wall U at +0/25/50/75/100% | `Q_H` non-decreasing                       |

## Explanation on Sparse Inputs (Why default values were used)

In these tests, the inputs explicitly provided seem sparse (e.g., only defining Floor Area and Year Class). We rely on the core `Parameters::default()` and `BuildingGeometry::default()` traits in Rust to inject standard baseline defaults.

**Why didn't we use all inputs explicitly?**
Defining every single parameter (e.g., `roof_pitch`, `shading_factor`, `thermal_bridge_adjustment`, etc.) would make the test setup incredibly verbose and hard to maintain. Instead, we use "Regression by Exception." We establish a default standard baseline, and for each test, we explicitly mutate **only the variables that matter** for that archetype.

**What are the full explicit default values for the omitted inputs?**
Based on the `Default` implementation for the `Parameters` struct in `lib.rs` (lines 1391-1437), if a field is not explicitly defined in a reference case, the engine automatically injects the following baseline values:

### Core Building Properties

- **`building_type`**: `"SFH"` (Single Family House)
- **`year_class`**: `"2016-..."` (Modern standard building)
- **`scenario`**: `"Existing State"`
- **`story_height`**: `2.8` meters
- **`num_stories`**: `1`
- **`window_to_wall_ratio`**: `0.15` (15%)
- **`building_rotation_deg`**: `0.0` (North aligned)
- **`construction_class`**: `ConstructionClass::Heavy`
- **`climate_region`**: `"Potsdam"` (Region 4)
- **`usage_profile`**: `"Residential"`
- **`automation_class`**: `"C"`

### Envelope & Insulation

- **`custom_wall_insulation`**: `None`
- **`custom_roof_insulation`**: `None`
- **`custom_floor_insulation`**: `None`
- **`thermal_bridge_category`**: `StandardDefault` (Category A/B logic)
- **`detailed_thermal_bridges`**: `[]` (Empty array)
- **`ground_contact_type`**: `"Unheated Basement"`
- **`shutter_control`**: `"Manual"`

### Heating Systems (Generator, Emission, Distribution, Storage)

- **`heating_system`**: `"Gas Condensing Boiler"`
- **`heating_emission_type`**: `"Radiator"`
- **`heating_emission_control`**: `"Mechanical2K"`
- **`heating_pipe_insulation`**: `"EnEV100"`
- **`heating_pump_control`**: `"RegulatedDeltaPV"`
- **`heating_buffer_tank`**: `"StandardUnheated"`

### Ventilation & Infiltration

- **`air_tightness`**: `"CategoryII"`
- **`has_atd`**: `false`
- **`mech_supply`**: `0.0` m³/h
- **`mech_exhaust`**: `0.0` m³/h
- **`heat_recovery`**: `0.0` (0%)
- **`mech_hours`**: `0.0` h/d

### Lighting & Internal Gains

- [ ] **`lighting_exhaust`**: `"None"`
- [ ] **`lighting_room_usage`**: `"Residential"`
- [ ] **`lighting_lamp_technology`**: `"LED"`
- [ ] **`lighting_control`**: `"Manual"`
- [ ] **`material_transport`**: `"None"`
- [ ] **`custom_occupants`**: `0.0`
- [ ] **`custom_equipment`**: `0.0`

> **Note on Internal Gains & Usage Profiles:re**
> You may notice `custom_occupants` and `custom_equipment` are `0.0`. This does **not** mean the building is empty. Setting these to `0.0` instructs the engine to ignore custom overrides and instead use the normative DIN V 18599 baseline assumptions driven by the **`usage_profile`** (e.g., `"Residential"`).
>
> When `"Residential"` is the active profile, the engine automatically derives standard daily usage hours ($t_{nutz}$), operating days per year ($d_{nutz}$), average heat emitted by occupants per square meter, and average heat emitted by standard household appliances per square meter. Custom overrides are only used when explicitly defining specialized zones (like a server room with an exact equipment wattage).

### Domestic Hot Water (DHW)

- **`dhw_consumption_profile`**: `"Standard"`
- **`dhw_system_scale`**: `"StandardSFH"`
- **`dhw_wrg_technology`**: `"None"`
- **`dhw_generator_type`**: `"HeatPumpAirWater"`

By relying on these 41 comprehensive defaults, a test like `detached_house_altbau` only needs to override `year_class` to `"…1859"` and `heating_system` to `"Gas Non-Condensing Boiler"`. The engine will safely lean on everything else—like the default 2.8m story height, radiator emission systems, or LED lighting—without us explicitly typing it out for every single test case.
