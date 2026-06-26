# Solar Heat Gains Calculation Engine

This document explains the physics and logic used in the Rust calculation engine (`lib.rs` / `solar_gains` module) to compute the building's solar heat gains ($Q_S$) according to DIN/TS 18599-2:2025-10 (Section 6.4).

## 1. Data Flow & Architecture

The solar gains engine calculates how much solar energy enters the building. This is split into two distinct physical phenomena:

1. **Transparent Gains (** $Q_{s,w}$**):** Solar radiation passing directly through windows to heat the interior. This is heavily reduced by frames, dirty glass, angle of incidence, and shading devices.
2. **Opaque Gains & Sky Losses (** $Q_{s,op}$**):** Solar radiation warming the exterior walls/roofs, heavily offset by thermal radiation emitted from the building out into the cold night sky.

```
flowchart TD
    %% UI Inputs
    subgraph UI ["User Interface (Inputs)"]
        G["Geometry\n(Window Area, Wall Area)"]
        O["Orientation & Tilt\n(N, S, E, W, Roof)"]
        W["Window Properties\n(Glazing, Frame %)"]
        S["Shading Devices\n(Blinds, Awnings)"]
        C["Wall Color\n(Light, Medium, Dark)"]
    end

    %% Database / Norms
    subgraph DB ["DIN 18599 Physics & Climate"]
        I_S["Solar Irradiation I_s\n(Climate Data)"]
        F_W["Incidence Factor (F_w = 0.9)"]
        R_SE["Ext. Resistance (R_se = 0.04)"]
        SKY["Sky Radiation Loss\n(h_r * Delta theta_er)"]
    end

    %% Rust Engine Middle Layer
    subgraph PreCalc ["Preprocessing (Rust)"]
        EFF_A["Effective Collector Area\n(A_w * (1-F_F) * g * F_C)"]
        ALPHA["Solar Absorptance (alpha)"]
    end

    %% Detailed Calculations
    subgraph Calc ["Solar Energy Balances (Q_S)"]
        Q_W["Q_s,w\n(Transparent Gains)"]
        Q_OP["Q_s,op\n(Opaque Gains & Sky Losses)"]
    end

    %% Final Output
    QS["Q_S Total\nTotal Solar Heat Gain"]

    G --> EFF_A
    W --> EFF_A
    S --> EFF_A
    O --> I_S
    C --> ALPHA

    EFF_A --> Q_W
    I_S --> Q_W
    F_W --> Q_W

    G --> Q_OP
    I_S --> Q_OP
    ALPHA --> Q_OP
    R_SE --> Q_OP
    SKY --> Q_OP

    Q_W --> QS
    Q_OP --> QS
    QS --> |Add to Internal Gains| BALANCE["Total Building Heat Sources"]
```

## 2. Inputs Required from the UI

To accurately calculate solar gains, the UI must gather geometry and component properties mapped to their cardinal direction.

### A. Window Parameters (Transparent)

| UI Prompt                 | Determines                         | DIN Variable                           |
| ------------------------- | ---------------------------------- | -------------------------------------- |
| **"Window Area"**         | Total size of the opening ($m^2$)  | $A_w$                                  |
| **"Orientation & Tilt"**  | Dictates the sun exposure          | $I_s$                                  |
| **"Glazing Type"**        | Double, Triple, Solar Control      | $g$-value (Total Energy Transmittance) |
| **"Frame Material/Type"** | Thick vs. Thin frames              | $F_F$(Frame Fraction)                  |
| **"Shading Devices"**     | Exterior Blinds, Interior Curtains | $F_C$(Shading Reduction Factor)        |
| **"Surrounding Shadows"** | Overhangs, Neighboring Buildings   | $F_S$(Surroundings Shading Factor)     |

### B. Wall/Roof Parameters (Opaque)

| UI Prompt            | Determines                                 | DIN Variable                |
| -------------------- | ------------------------------------------ | --------------------------- |
| **"Wall/Roof Area"** | Total exposed surface ($m^2$)              | $A_{op}$                    |
| **"Surface Color"**  | Light (White), Medium (Brick), Dark (Grey) | $\alpha$(Solar Absorptance) |

## 3. Core Calculations (DIN/TS 18599-2 Section 6.4)

### 3.1 Transparent Solar Gains ($Q_{s,w}$)

This is the primary source of solar heating. The standard calculates the "Effective Solar Collecting Area" of the window and multiplies it by the irradiation.

**Formula:**
$Q_{s,w} = I_s \cdot A_w \cdot (1 - F_F) \cdot F_w \cdot g \cdot F_C \cdot F_S$

- $I_s$: Solar irradiation for the given orientation and time period ($Wh/m^2$ or $kWh/m^2$).
- $A_w$: Total area of the window ($m^2$).
- $(1 - F_F)$: Transparent glass fraction (subtracting the opaque frame $F_F$).
- $F_w$: Correction factor for non-perpendicular radiation (Standard default is `0.90`).
- $g$: Total solar energy transmittance of the glass.
- $F_C$: Shading factor from operable blinds/shades.
- $F_S$: Shading factor from fixed obstacles.

### 3.2 Opaque Solar Gains & Sky Radiation ($Q_{s,op}$)

The exterior envelope absorbs solar heat during the day, reducing transmission losses. However, it also constantly radiates heat outward into the cold sky (sky radiation). In winter, the sky radiation loss often exceeds the solar gain, making $Q_{s,op}$ a net negative (a heat sink).

**Formula:**
$Q_{s,op} = A_{op} \cdot U_{op} \cdot R_{se} \cdot (\alpha \cdot I_s - F_{sky} \cdot h_r \cdot \Delta\theta_{er} \cdot t)$

- $A_{op}$: Area of the opaque component (Wall/Roof).
- $U_{op}$: Thermal transmittance (U-value) of the component.
- $R_{se}$: External surface thermal resistance (Standard is `0.04` $m^2K/W$).
- $\alpha$: Solar absorptance coefficient of the surface color.
- $F_{sky}$: Form factor to the sky (`1.0` for horizontal roofs, `0.5` for vertical walls).
- $h_r$: External radiative heat transfer coefficient (Standard is `5.0` $W/m^2K$).
- $\Delta\theta_{er}$: Temperature difference between external air and apparent sky (Standard is `11 K`).
- $t$: Time period duration (hours).

  _(Note: Because this requires the U-value (_ $U_{op}$_), opaque solar gains are intrinsically linked to the Transmission calculations)._

## 4. Reference Data & Standard Values

To execute the logic, the engine relies on strict fallback parameters provided by DIN 18599.

### Table A: Standard Glazing $g$-Values

If exact manufacturer data is missing, DIN defaults apply:

| Glazing Type                     | $g$-value       |
| -------------------------------- | --------------- |
| Single Glazing                   | `0.85`          |
| Double Glazing (Standard)        | `0.75`          |
| Double Glazing (Low-E / Thermal) | `0.60`          |
| Triple Glazing (Low-E / Thermal) | `0.50`          |
| Solar Control Glass              | `0.30`to `0.40` |

### Table B: Frame Fraction ($F_F$)

The percentage of the window area that is opaque frame.

| Window Description                              | Frame Fraction ($F_F$)       |
| ----------------------------------------------- | ---------------------------- |
| Standard Window                                 | `0.30`(30% Frame, 70% Glass) |
| Very Large Windows / Glass Facades              | `0.20`                       |
| Small Windows / Divided panes (Sprossenfenster) | `0.40`                       |

### Table C: Shading Devices ($F_C$)

Operable shading heavily reduces solar penetration.

| Shading Device                         | Reduction Factor ($F_C$) |
| -------------------------------------- | ------------------------ |
| No Shading                             | `1.00`                   |
| Interior Curtains (White / Light)      | `0.80`                   |
| Interior Curtains (Dark)               | `0.60`                   |
| Exterior Blinds / Shutters (Rollladen) | `0.25`                   |
| Exterior Awnings (Markisen)            | `0.40`                   |

### Table D: Opaque Absorptance ($\alpha$)

How much sun the exterior walls/roof absorb based on color.

| Surface Color                      | Solar Absorptance ($\alpha$) |
| ---------------------------------- | ---------------------------- |
| Light (White, very light grey)     | `0.30`                       |
| Medium (Red brick, concrete, wood) | `0.60`                       |
| Dark (Dark grey, black roof tiles) | `0.90`                       |

## 5. Detailed Rust Implementation

This Rust architecture encapsulates the exact physics required by the standard, safely isolating transparent and opaque behaviors.

```
// --- CONSTANTS FROM DIN 18599 ---
const F_W_STANDARD: f64 = 0.90; // Correction for non-perpendicular radiation
const R_SE_STANDARD: f64 = 0.04; // External surface resistance (m²K/W)
const H_R_STANDARD: f64 = 5.0; // External radiative heat transfer coeff (W/m²K)
const DELTA_THETA_ER: f64 = 11.0; // Sky temperature difference (K)

// --- DATA MODELS ---

/// Represents a window or transparent surface.
#[derive(Debug, Clone)]
pub struct TransparentComponent {
    pub area: f64,          // A_w (m²)
    pub frame_fraction: f64, // F_F (e.g., 0.30)
    pub g_value: f64,       // Total energy transmittance (e.g., 0.60)
    pub f_c: f64,           // Shading device factor (e.g., 0.25 for ext. blinds)
    pub f_s: f64,           // Surroundings shading (e.g., 0.90)
    pub irradiation: f64,   // I_s (Wh/m² for the calculated time period)
}

impl TransparentComponent {
    /// Calculates the effective solar collecting area (m²)
    pub fn effective_collecting_area(&self) -> f64 {
        self.area * (1.0 - self.frame_fraction) * F_W_STANDARD * self.g_value * self.f_c * self.f_s
    }

    /// Calculates the total solar heat gain (Wh) for this time period
    pub fn solar_gain_wh(&self) -> f64 {
        self.irradiation * self.effective_collecting_area()
    }
}

/// Represents a wall or roof (opaque surface).
#[derive(Debug, Clone)]
pub struct OpaqueComponent {
    pub area: f64,          // A_op (m²)
    pub u_value: f64,       // U-value (W/m²K)
    pub alpha: f64,         // Solar absorptance (e.g., 0.60 for medium color)
    pub is_roof: bool,      // Determines F_sky (1.0 for roof, 0.5 for walls)
    pub irradiation: f64,   // I_s (Wh/m² for the calculated time period)
    pub time_hours: f64,    // Duration of the time period (h)
}

impl OpaqueComponent {
    /// Calculates the net solar gain (which is often negative due to sky radiation) in Wh
    pub fn solar_gain_wh(&self) -> f64 {
        let f_sky = if self.is_roof { 1.0 } else { 0.5 };

        // Solar energy absorbed by the surface
        let absorbed_solar = self.alpha * self.irradiation;

        // Energy lost to the cold night sky
        let sky_radiation_loss = f_sky * H_R_STANDARD * DELTA_THETA_ER * self.time_hours;

        // Net gain is converted into a building heat load via U-value and R_se
        self.area * self.u_value * R_SE_STANDARD * (absorbed_solar - sky_radiation_loss)
    }
}

// --- ENGINE ---

/// The core engine struct to calculate total solar gains
pub struct SolarGainsEngine {
    pub transparent_components: Vec<TransparentComponent>,
    pub opaque_components: Vec<OpaqueComponent>,
}

impl SolarGainsEngine {
    pub fn new() -> Self {
        Self {
            transparent_components: Vec::new(),
            opaque_components: Vec::new(),
        }
    }

    pub fn add_transparent(&mut self, comp: TransparentComponent) {
        self.transparent_components.push(comp);
    }

    pub fn add_opaque(&mut self, comp: OpaqueComponent) {
        self.opaque_components.push(comp);
    }

    /// Calculates the total solar heat gain (Q_S) for the entire envelope (Wh)
    pub fn total_solar_gain_wh(&self) -> f64 {
        let transparent_gain: f64 = self.transparent_components.iter()
            .map(|c| c.solar_gain_wh())
            .sum();

        let opaque_gain: f64 = self.opaque_components.iter()
            .map(|c| c.solar_gain_wh())
            .sum();

        // Total Solar Gain (Q_S)
        transparent_gain + opaque_gain
    }
}

// --- USAGE EXAMPLE ---
// let mut engine = SolarGainsEngine::new();
//
// // Add a South-facing Double-Glazed Window with Exterior Blinds
// engine.add_transparent(TransparentComponent {
//     area: 10.0,
//     frame_fraction: 0.30,
//     g_value: 0.60,
//     f_c: 0.25, // Exterior blinds
//     f_s: 1.00, // No surrounding shadows
//     irradiation: 45000.0, // e.g., 45 kWh/m² for a month
// });
//
// // Add a South-facing Medium-colored Brick Wall
// engine.add_opaque(OpaqueComponent {
//     area: 50.0,
//     u_value: 0.28,
//     alpha: 0.60, // Medium color
//     is_roof: false,
//     irradiation: 45000.0,
//     time_hours: 744.0, // Hours in a 31-day month
// });
//
// let q_s_total = engine.total_solar_gain_wh();
```
