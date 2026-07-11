# Energy Calculation Documentation

This document outlines the energy calculation logic based on the `calculate_energy` function found in `sketchpad-rs/src/lib.rs` and the geometric mapping logic in `main_mcp.js`. It breaks down the formulas used and maps the variables to their source locations within the application state and external databases.

## 1. Building Geometry & Surface Areas

**Concept & Formula:**
The physical envelope of the building determines the surface areas exposed to the environment. The calculations compute the gross and net areas for the walls, roof, floor, and windows.

* **Total Window Area:**
  $$
  A_{window} = \sum A_{window\_orientations} \quad \text{(North, East, South, West, Horizontal)}
  $$
* **Net Wall Area:**
  $$
  A_{net\_wall} = A_{gross\_wall} - A_{vertical\_windows}
  $$

**Source Mapping:**

* **Areas:** Extracted directly from `state.geometry` (e.g., `geometry.total_floor_area`, `geometry.total_roof_area`, `geometry.envelope_data`).

### 1.1 Detailed Orientation Calculation (Windows & Walls)

The orientation of walls and windows is calculated dynamically in the front-end (`main_mcp.js`) by analyzing the 2D floor plan polygons.

1. **Polygon Edges:** The system iterates over the perimeter edges of the building's union polygon. For each edge from point $P_1$ to $P_2$, it calculates $\Delta x = P_{2x} - P_{1x}$ and $\Delta y = P_{2y} - P_{1y}$.
2. **Building Rotation:** The edge vectors are rotated by the global building rotation angle ($\theta_{rot}$):

   $$
   \Delta x_{rot} = \Delta x \cdot \cos(-\theta_{rot}) - \Delta y \cdot \sin(-\theta_{rot})
   $$

   $$
   \Delta y_{rot} = \Delta x \cdot \sin(-\theta_{rot}) + \Delta y \cdot \cos(-\theta_{rot})
   $$
3. **Surface Normal Vector:** The 2D normal vector pointing outwards from the wall is computed:

   $$
   N_x = \Delta y_{rot}
   $$

   $$
   N_y = -\Delta x_{rot}
   $$
4. **Cardinal Direction:** The angle of the normal is found using $\text{atan2}(N_x, N_y)$ and converted to degrees ($0^\circ$ to $360^\circ$). The wall and its windows are binned into cardinal directions:

   * **North (N):** $[315^\circ, 360^\circ)$ or $[0^\circ, 45^\circ)$
   * **East (E):** $[45^\circ, 135^\circ)$
   * **South (S):** $[135^\circ, 225^\circ)$
   * **West (W):** $[225^\circ, 315^\circ)$
5. **Window Matching:** It checks the unrotated original zones to find windows placed on these edges and assigns their areas to the corresponding rotated cardinal bin in `envelope_data`.

## 2. Thermal Transmittance & Components (U-Values)

**Concept & Formula:**
The U-value measures the rate of heat transfer through a structure. Base U-values are modeled via layered components in the Rust `transmission` module according to DIN V 18599-2.

### 2.1 Opaque Components (`BuildingComponent` & Custom Thickness)

Opaque components (walls, roofs, floors) are modeled as a series of material layers. If the user wants to add custom insulation, they can push a new `Layer` to the component with a specific thickness.

* **Struct `Material`:** Defines `lambda` ($\lambda$, thermal conductivity in $W/(m\cdot K)$).
* **Struct `Layer`:** Defines `thickness` ($d$ in meters) and its `Material`.
* **Thermal Resistance ($R_T$):**
  $$
  R_T = R_{si} + \sum \left( \frac{d_i}{\lambda_i} \right) + R_{se}
  $$

  * $d_i$: The user-defined thickness of the layer in meters.
  * $\lambda_i$: The material's thermal conductivity.
  * $R_{si}$ (Inner surface resistance): Standard $0.13 \text{ m}^2K/W$.
  * $R_{se}$ (Outer surface resistance): Standard $0.04 \text{ m}^2K/W$ (or $0.13$ if internal).
* **U-Value Calculation (`calculate_u_value`):**
  $$
  U = \frac{1}{R_T}
  $$

  This formula is dynamically re-calculated in `lib.rs` every time a user adds a layer or changes its thickness.

### 2.2 Transparent Components (`WindowComponent`)

Windows account for shading control systems when calculating their effective heat loss.

* **Struct `WindowComponent`:** Defines base U-value ($U_w$) and shutter U-value ($U_{w,sh}$).
* **Effective U-Value (`calculate_u_w_eff`):**
  $$
  U_{w,eff} = U_w \cdot (1 - f_{sh}) + U_{w,sh} \cdot f_{sh}
  $$

**Source Mapping:**

* **Base Components:** `TABULA_DB` fallback or customized via `state.params.custom_wall_insulation`.
* **Shutter Usage Fraction ($f_{sh}$):** Handled by `get_shutter_fraction(month, building_type, control_type)` based on `ShutterControl::Manual` or `Automated`.
  * *Manual + Non-Residential:* $f_{sh} = 0.0$
  * *Automated (All):* Ranges from $0.64$ in Dec/Jan to $0.23$ in June.
  * *Manual + Residential:* Ranges from $0.45$ in Dec/Jan to $0.16$ in June.

## 3. Transmission Heat Transfer Coefficient ($H_{tr}$)

**Concept & Formula:**
The overall transmission heat transfer coefficient ($H_T$ or $H_{tr}$) represents the total heat lost through the building envelope via transmission. It sums up the "leakiness" of the entire building by aggregating direct losses, losses to unheated spaces/ground, and thermal bridges.

* **Overall Transmission ($H_T$):**

  $$
  H_T = H_{T,D} + H_{T,iu,eff} + H_{T,WB}
  $$
* **1. Direct Transmission (`calculate_h_t_d`):**
  Heat lost directly to the exterior environment ($F_x \approx 1.0$).

  $$
  H_{T,D} = \sum \left( A_j \cdot U_j \cdot f_{neig,j} \right)
  $$

  *Where $f_{neig,j}$ is an inclination factor derived from `WindowGlazingType` and `WindowInclinationAngle` (Table 7):*

  * **$0^\circ$ (Horizontal):** Single: $1.25$, Double: $1.21$, Triple: $1.20$
  * **$45^\circ$:** Single: $1.21$, Double: $1.15$, Triple: $1.07$
  * **$90^\circ$ (Vertical):** $1.00$ for all glazing types.
* **2. Transmission to Unheated Zones/Ground (`calculate_h_t_iu`):**
  Heat lost through surfaces not directly exposed to the outside ($F_x \neq 1.0$).

  $$
  H_{T,iu,eff} = \sum \left( A_j \cdot U_j \cdot F_{x,j} \right)
  $$
* **3. Thermal Bridges (`calculate_h_t_wb_simplified` / `calculate_detailed_h_t_wb`):**
  Simplified method adds a penalty for structural heat leaks over the entire envelope.

  $$
  H_{T,WB} = \Delta U_{WB} \cdot \sum A_{envelope}
  $$

  Detailed method: $H_{T,WB} = \sum (l_j \cdot \Psi_j \cdot f_{x,j})$
* **4. Specific Heat Transfer Coefficient (`calculate_h_t_specific`):**

  $$
  H'_T = \frac{H_T}{\sum A_{envelope}}
  $$

**Source Mapping:**

* **Temperature Adjustment Factor ($F_x$):**
  * **Enum `UnheatedSpaceType`:**
    * `Attic`, `Sunspace`, `CrawlSpaceVentilated` $\implies 0.8$
    * `AdjacentUnheatedRoom`, `UnheatedBasement`, `StaircaseExterior`, `CrawlSpaceUnventilated` $\implies 0.5$
    * `StaircaseInterior` $\implies 0.35$
  * **Enum `GroundContactType`:**
    * `GroundwaterContact` $\implies 1.0$
    * `FloorSlabOnGround`, `BasementWallShallow`, `BasementWallDeep`, `HeatedBasementFloor` $\implies 0.5$
* **Thermal Bridge Penalty ($\Delta U_{WB}$):**
  * **Enum `ThermalBridgeCategory`:**
    * `InternalInsulationIssues` $\implies 0.15$
    * `StandardDefault` $\implies 0.10$
    * `GoodPlanning` (Category A) $\implies 0.05$
    * `ExcellentPlanning` (Category B) $\implies 0.03$

## 4. Ventilation Heat Transfer Coefficient ($H_{ve}$)

**Concept & Formula:**
The standard requires resolving the baseline air requirements before calculating how much heat is lost through infiltration, windows, and mechanical systems. The core physical formula shared by all air exchange components is:

$$
H_v = n \cdot V \cdot 0.34
$$

*(where $0.34$ represents air heat capacity $c_{p,a} \cdot \rho_a$ in $Wh/(m^3K)$)*

**Total Ventilation Heat Transfer ($H_{ve}$)**
At the highest level, the total ventilation heat transfer coefficient ($H_{ve}$) is the sum of three separate components:

$$
H_{ve} = H_{V,inf} + H_{V,win} + H_{V,mech} + H_{V,ue}
$$

### 4.1 Infiltration Heat Transfer ($H_{V,inf}$)

Heat lost through unintentional leaks. The infiltration air change rate ($n_{inf}$) is calculated dynamically via `calculate_n_inf` based on whether mechanical ventilation is running ($t_{v,mech} > 0$):

* **Without Mechanical Ventilation:**

  $$
  n_{inf} = n_{50} \cdot e \cdot f_{atd}
  $$
* **With Mechanical Ventilation:**

  $$
  n_{inf} = n_{50} \cdot e \cdot f_{atd} \cdot \left(1 + (f_e - 1) \cdot \frac{t_{v,mech}}{24}\right)
  $$
* **$n_{50}$ (Air Tightness at 50 Pa):** Resolved based on building volume and `TightnessCategory` using `BuildingAirData::resolve_n50()`:

  * *If Volume $\le 1500 \text{m}^3$:* `CategoryI` (Tested): user value (default 3.0), `CategoryII` (New): 4.0, `CategoryIII`: 6.0, `CategoryIV` (Leaky): 10.0.
  * *If Volume $> 1500 \text{m}^3$:* $n_{50} = \frac{q_{50} \cdot A_{envelope}}{V}$, where $q_{50}$ is `CategoryI`: user (default 3.0), `CategoryII`: 6.0, `CategoryIII`: 9.0, `CategoryIV`: 15.0.
* **$e$ (Wind Exposure):** Hardcoded `DEFAULT_E = 0.07`.
* **$f_{atd}$ (Air Tightness Device Factor):** `calculate_f_atd()` returns $1.0$ (no ATD) or $\min\left(16.0, \frac{n_{50} + 1.5}{n_{50}}\right)$ (with ATD).
* **$f_e$ (Mechanical Imbalance Factor):** `calculate_f_e()` returns $1.0$ if perfectly balanced. If unbalanced: $f_e = \frac{1}{1 + 15.0 \cdot 0.07 \cdot \left(\frac{n_{sup} - n_{eta}}{n_{50} \cdot f_{atd}}\right)^2}$

### 4.2 Required Fresh Air ($n_{nutz}$)

The baseline fresh air requirement via `calculate_n_nutz()`:

$$
n_{nutz} = \frac{\dot{V}_A \cdot A_{NGF}}{V_e}
$$

*Where $\dot{V}_A$ is the minimum fresh air volume flow per square meter:*

* Residential: Flat $0.5 \text{ h}^{-1}$ fallback.
* Storage/Logistics: $0.5 \text{ m}^3/(\text{h}\cdot\text{m}^2)$
* Default/Average: $1.5 \text{ m}^3/(\text{h}\cdot\text{m}^2)$
* Single Office: $2.0 \text{ m}^3/(\text{h}\cdot\text{m}^2)$
* Group/Open Office, Retail, Hospital Room: $3.0 \text{ m}^3/(\text{h}\cdot\text{m}^2)$
* Gymnasium: $3.5 \text{ m}^3/(\text{h}\cdot\text{m}^2)$
* Meeting Room, Classroom: $5.0 \text{ m}^3/(\text{h}\cdot\text{m}^2)$
* Restaurant: $7.0 \text{ m}^3/(\text{h}\cdot\text{m}^2)$

### 4.3 Window Airing ($H_{V,win}$)

Calculates window airing by figuring out how much fresh air is required ($n_{nutz}$) versus how much is already provided via `WindowAiringParams`.

* **Without Mech Vent (`delta_n_win`):** Deficit $\Delta n_{win} = \max[0; n_{nutz} - (n_{nutz} - 0.2) \cdot n_{inf} - 0.1]$
* **With Mech Vent (`delta_n_win_mech`):** Complex deficit calculation comparing $n_{sup}$ and $n_{eta}$ against total infiltration and mechanical supply.
* **Final Rate (`calculate_n_win_daily`):** Interpolates between these states using $t_{v,mech}$ and building usage hours $t_{nutz}$.
  $$
  H_{V,win} = n_{win} \cdot V \cdot 0.34
  $$

### 4.4 Mechanical Ventilation ($H_{V,mech}$)

Heat loss specifically driven by the mechanical supply air system via struct `MechanicalSystem`.

* **Supply Air Rate:** $n_{mech,SUP} = \frac{\dot{V}_{mech,b}}{V}$
* **Daily Average Rate:** $n_{mech} = n_{mech,SUP} \cdot \frac{t_{v,mech}}{24}$
* **Heat Transfer:** $H_{V,mech} = n_{mech} \cdot V \cdot 0.34$

## 5. Heating/Cooling Profiles & Parameters

**Concept & Formula:**
The internal setpoint temperatures and operation hours are derived dynamically from standard profiles, affecting the final heat loss and gain calculations.

**Source Mapping:**

* **Usage Profile (`UsageProfile` enum):**
  * *Usage Days ($d_{nutz}$):* 365 (Residential, Hospital, DataCenter), 300 (Retail, Library), 250 (Office, School), 200 (Stage/Exhibition).
  * *Heating Hours ($t_{nutz}$):* 24h (Hospital/DataCenter), 17h (Residential), 13h (Office/Lab), 12h (Retail/Classroom), 10h (Stage).
  * *Heating Setpoints:* $20.0^\circ\text{C}$ (Residential), $21.0^\circ\text{C}$ (Office), $22.0^\circ\text{C}$ (Hospital), $15.0^\circ\text{C}$ (Heavy Industry).
  * *Cooling Setpoints:* $25.0^\circ\text{C}$ (Residential), $24.0^\circ\text{C}$ (Office), $28.0^\circ\text{C}$ (Heavy Industry).
* **Automation Class (`AutomationClass` enum):** Determines temperature shift:
  * *Class C/D:* $0.0\text{ K}$ (Except Residential C: $-0.5\text{ K}$).
  * *Class B:* $-1.5\text{ K}$ (Retail/Restaurant), $-1.0\text{ K}$ (Office).
  * *Class A:* $-2.0\text{ K}$ (Retail/Restaurant), $-1.5\text{ K}$ (Office).
* **Climate Region (`ClimateRegion` enum):**
  * Monthly External Temperatures ($\theta_e$) array depending on region (e.g. `Potsdam`, `Bremerhaven`, `Hamburg`).

## 6. Base Energy Losses ($Q_{ht}$)

**Concept & Formula:**
The total sensible heat lost over the heating period. The formulas use a conversion factor of $0.024$ to convert W to kWh/day.

* **Transmission Loss ($Q_{ht,tr}$):**

  $$
  Q_{ht,tr} = 0.024 \cdot H_{tr} \cdot (\theta_{int} - \theta_e) \cdot d_{hs,loss}
  $$
* 
  **Ventilation Loss ($Q_{ht,ve}$):**

  $$
  Q_{ht,ve} = 0.024 \cdot H_{ve} \cdot (\theta_{int} - \theta_e) \cdot d_{hs,loss}
  $$
* **Internal Setpoint ($\theta_{int}$):**

  $$
  \theta_{int} = \max(10.0,\; \text{heating\_setpoint} + \text{temperature\_shift})
  $$
* **Heating Period ($d_{hs,loss}$):** Fixed at $185.0$ days for the winter heating season.

## 7. Energy Gains ($Q_{gn}$)

**Concept & Formula:**
Heat gained from the sun and internal sources that offset the heating demand.

$$
Q_{gn} = Q_{sol} + Q_{int}
$$

### 7.1 Internal Gains ($Q_{int}$)

Heat introduced into the building by occupants and equipment via the `internal_gains` engine.

$$
Q_{int} = \frac{\text{net\_daily\_gain\_wh} \cdot d_{hs,gain}}{1000}
$$

*(Where $d_{hs,gain} = d_{nutz} \cdot \frac{185.0}{365.0}$)*

The `internal_gains` engine computes `net_daily_gain_wh` using `StandardGainProfile` values ($q_{i,combined}, q_{i,p}, q_{i,app}, q_{i,sink,app}$):

* **Residential Calculation (Combined):**

  $$
  \text{gain}_{res} = (q_{i,combined} \cdot A_{ngf}) \cdot t_{nutz}
  $$
* **Non-Residential Calculation (Split):**

  $$
  \text{gain}_{non\_res} = (q_{i,p} \cdot A_{ngf} \cdot t_{nutz}) + (q_{i,app} \cdot A_{ngf} \cdot t_{nutz})
  $$

  $$
  \text{sink}_{app} = q_{i,sink,app} \cdot A_{ngf} \cdot t_{nutz}
  $$

**Additional Gain Sinks:**

* **Lighting:** $\text{gain}_{light} = \mu_L \cdot q_{l,f,daily}$
* **Material Transport:**
  $$
  \text{gain}_{mat} = c_p \cdot \dot{m} \cdot (\theta_{in} - \theta_{out}) \cdot t
  $$

  * `ColdGoodsSmall`/`Large`: Acts as a net heat sink (cools room).
  * `HotMetalSmall`/`Large`: Acts as a net heat source (heats room).

### 7.2 Solar Gains & Sky Loss

Computed via the solar engine.

* **Solar Gains ($Q_{sol}$):** Positive heat entering through windows.
  $$
  Q_{sol} = \frac{Q_{sol\_sources\_wh}}{1000}
  $$
* **Sky Loss ($Q_{sky\_loss}$):** Heat lost by thermal radiation to the sky. It is added to $Q_{ht,tr}$ rather than subtracted from gains.
  $$
  Q_{sky\_loss} = \frac{Q_{sol\_sinks\_wh}}{1000}
  $$

### 6.1 Code Variable Walkthrough (Aggregation)

When the engine calculates the final balance, it uses specific variables to aggregate these systems:

* **`HotMetalLarge` / `material_transport`**: For extremely heavy industrial usage profiles, the system calculates the massive amount of heat that raw materials dump into a room. The `HotMetalLarge` example assumes a mass flow of $2000 \text{ kg/h}$ cooling from $200^\circ\text{C}$ to $20^\circ\text{C}$ inside the building. This is fed into the internal gains engine.
* **`q_sol_sources_wh` and `q_sol_sinks_wh`**: The solar engine returns both the positive heat gained from the sun (sources) and the heat lost to the cold sky via thermal radiation (sinks) in Watt-hours ($Wh$).
* **`q_sol` and `q_sky_loss`**: The code divides those Watt-hour values by $1000.0$ to convert them into standard $kWh$.
* **`q_ht_tr_total`**: The `q_sky_loss` (heat radiating out into the cold sky) is added to the **Total Transmission Losses ($Q_{ht,tr}$)**, instead of being subtracted from the gains.
* **`q_int`**: The internal gains engine outputs a net daily gain in $Wh$, which is multiplied by the number of heating days ($d_{hs,gain}$) and divided by $1000.0$ to get the final internal gains in $kWh$.
* **`q_gn`**: Finally, the total Solar Gains ($Q_{sol}$) and Internal Gains ($Q_{int}$) are summed together to get the total $Q_{gn}$.

## 7. Final Heating Demand ($Q_{h,nd}$)

**Concept & Formula:**
The absolute net thermal energy required to maintain the setpoint temperature, factoring in the utilization of heat gains. This is calculated using the Master Equation inside the `EnergyBalanceEngine`:

$$
Q_{h,nd} = \max\Big(0.0,\; Q_l - (\eta \cdot Q_g)\Big)
$$

Here is exactly how the Rust engine (`energy_balance::EnergyBalanceEngine::calculate_final_heating_demand`) computes this step-by-step:

**1. Initialize the Engine (`EnergyBalanceEngine::new`)**
The engine is instantiated with the `conditioned_volume` ($V_e$) and a thermal inertia profile based on the `ConstructionWeight` enum:

* `ConstructionWeight::Heavy`: Used for Residential buildings (`SFH`, `MFH`). Simulates thick masonry that stores heat for longer periods.
* `ConstructionWeight::Light`: Used for Non-Residential buildings (e.g., Offices). Simulates lighter constructions that lose/gain heat rapidly.
* `CalculationPeriod::Seasonal`: Defines the bounds for the formulas.

**2. Aggregate Sinks and Sources**
The engine adds up all the heat the building loses, and all the heat it generates using the exact parameter mappings:

* **$Q_l$ (Total Losses)** = `q_ht_tr_total * 1000.0` (includes $Q_{sky\_loss}$) + `q_ht_ve * 1000.0`
* **$Q_g$ (Total Gains)** = `q_int * 1000.0` + `q_sol_sources_wh` (only the positive solar entering)

**3. Calculate Thermal Inertia ($\tau$)**
The engine calculates how long the building can "store" heat before it escapes. This is determined by dividing the building's effective heat capacity (mapped by `ConstructionWeight`) by the total heat transfer coefficients (`h_tr`, `h_ve`).

**4. Calculate Gain Utilization Factor ($\eta$)**
Not all gains are useful (e.g., if the sun heats the room to 25°C when the setpoint is 20°C, the extra 5°C is "wasted"). The engine calculates an efficiency factor ($\eta$) based on the ratio of gains to losses and the thermal inertia.

**5. Final Balance**
The final heating demand is equal to the total heat lost ($Q_l$), minus the portion of the free heat ($\eta \cdot Q_g$) that the building was actually able to trap and use.

## 8. Final Cooling Demand ($Q_{c,nd}$)

**Concept & Formula:**
The absolute net thermal energy required to remove heat and maintain the summer setpoint temperature (typically $25^\circ\text{C}$). This is calculated using the Master Cooling Equation inside the `EnergyBalanceEngine`:

$$
Q_{c,nd} = \max\Big(0.0,\; Q_{g,c} - (\eta_c \cdot Q_{l,c})\Big)
$$

Here is exactly how the Rust engine (`energy_balance::EnergyBalanceEngine::calculate_final_cooling_demand`) computes this step-by-step:

**1. Aggregate Sinks and Sources (The Flipped Balance)**
Unlike heating, the engine must dynamically route transmission and ventilation based on whether the outside air is hotter or cooler than the cooling setpoint:

* **$Q_{l,c}$ (Total Sinks)** = Transmission/Ventilation leaving the building (when outside is cooler than the setpoint).
* **$Q_{g,c}$ (Total Sources)** = Internal Gains ($Q_{int}$) + Solar Gains ($Q_{sol\_sources}$) + Transmission/Ventilation entering the building (when outside is hotter).

**2. Calculate Thermal Inertia ($\tau_c$)**
The engine calculates how long the building stores heat, using the exact same `ConstructionWeight` parameters as for heating, divided by the total heat transfer coefficients.

**3. Calculate Loss Utilization Factor ($\eta_c$)**
Not all natural cooling (like a cold night sky) is useful against a midday sun. The engine (`calculate_loss_utilization_factor`) computes exactly how much of the sinks can be utilized to offset the gains using the strict DIN V 18599-2 / ISO 13790 formulas:

* **Gain/Loss Ratio ($\gamma_c$):** The ratio of Sinks divided by Sources (the inverse of heating).

  $$
  \gamma_c = \frac{Q_{l,c}}{Q_{g,c}}
  $$
* **Curve Parameter ($a_c$):** Uses the same base constants as heating, but applied to the cooling time constant ($a_{c,0} = 1.0$, $\tau_{c,0} = 15.0 \text{ h}$).

  $$
  a_c = a_{c,0} + \frac{\tau_c}{\tau_{c,0}}
  $$
* **Final Utilization Factor ($\eta_c$):**

  $$
  \text{If } \gamma_c = 1: \quad \eta_c = \frac{a_c}{a_c + 1}
  $$

  $$
  \text{If } \gamma_c \neq 1: \quad \eta_c = \frac{1 - \gamma_c^{a_c}}{1 - \gamma_c^{a_c + 1}}
  $$

**4. Final Balance**
The final cooling demand is equal to the total heat sources ($Q_{g,c}$), minus the portion of the natural cooling sinks ($\eta_c \cdot Q_{l,c}$) that the building was actually able to utilize.

## 9. Heating System Engine (Anlagentechnik) & Final Energy Demand ($Q_{h,f}$)

**Concept & Formula:**
This module calculates the Final Energy Demand for Heating ($Q_{h,f}$) and the Auxiliary Electrical Energy ($W_h$) by working backward through the thermal chain of mechanical subsystems according to DIN V 18599-5:2018-09.

### 9.1 Subsystem 1: Emission (Wärmeübergabe)

Adds stratification, control, and wall proximity losses to the building's Net Heating Demand ($Q_{h,nd}$):

$$
Q_{h,in,ce} = Q_{h,nd} + Q_{h,ce}
$$

Where emission losses are calculated as:

$$
Q_{h,ce} = Q_{h,nd} \cdot \left[ \frac{\Delta\theta_{ce}}{\theta_{i,h} - \theta_e} \right]
$$

$$
W_{h,ce} = w_{h,ce} \cdot A_{NGF}
$$

* **Total Temperature Variation ($\Delta\theta_{ce}$):**
  $$
  \Delta\theta_{ce} = \Delta\theta_{str} + \Delta\theta_{ctr} + \Delta\theta_{emb} + \Delta\theta_{hydr} + \Delta\theta_{roomaut}
  $$
* **Stratification ($\Delta\theta_{str}$):**
  * `EmissionType::Radiator` + `TempLevel::High70_55` $\implies 1.2\text{ K}$
  * `EmissionType::Radiator` + `TempLevel::Medium55_45` $\implies 0.7\text{ K}$
  * `EmissionType::Radiator` + `TempLevel::Low35_28` $\implies 0.5\text{ K}$
  * `EmissionType::UnderfloorHeating` $\implies 0.0\text{ K}$
* **Control Deviation ($\Delta\theta_{ctr}$):**
  * `EmissionType::Radiator` + `EmissionControl::Mechanical2K` $\implies 1.2\text{ K}$
  * `EmissionType::Radiator` + `EmissionControl::Mechanical1K` $\implies 0.7\text{ K}$
  * `EmissionType::Radiator` + `EmissionControl::ElectronicPI` $\implies 0.5\text{ K}$
  * `EmissionType::UnderfloorHeating` + `EmissionControl::Mechanical2K`/`Mechanical1K` $\implies 1.2\text{ K}$
  * `EmissionType::UnderfloorHeating` + `EmissionControl::ElectronicPI` $\implies 0.9\text{ K}$
* **Adjacent Proximity Loss ($\Delta\theta_{emb}$):**
  * `AdjacentLoss::InteriorWall` $\implies 0.0\text{ K}$
  * `AdjacentLoss::SolidExteriorWall` $\implies 0.3\text{ K}$
  * `AdjacentLoss::GlassExteriorWall` $\implies 1.7\text{ K}$
* **Balancing Variation ($\Delta\theta_{hydr}$):**
  * `EmissionBalancing::NoBalancing` $\implies 0.6\text{ K}$
  * `EmissionBalancing::StaticBalancing` $\implies 0.3\text{ K}$
  * `EmissionBalancing::DynamicBalancing` $\implies 0.0\text{ K}$
* **Auxiliary Power Coefficient ($w_{h,ce}$):**
  * `EmissionControl::Mechanical2K` / `Mechanical1K` $\implies 0.0\text{ kWh/m²a}$
  * `EmissionControl::ElectronicPI` $\implies 0.6\text{ kWh/m²a}$

---

### 9.2 Subsystem 2: Distribution (Wärmeverteilung)

Calculates linear pipe losses using estimated pipe lengths and average circuit temperatures ($\theta_{HK,av}$):

$$
Q_{h,in,d} = Q_{h,in,ce} + Q_{h,d}
$$

$$
Q_{h,d} = \frac{1}{1000} \cdot U_L \cdot (\theta_{HK,av} - \theta_{amb,d}) \cdot L_{total} \cdot t_h
$$

$$
W_{h,d} = \frac{P_{Pu,d} \cdot t_h \cdot e_d}{1000}
$$

* **Average Heating Circuit Temperature ($\theta_{HK,av}$):**

  $$
  \theta_{HK,av} = 0.5 \cdot (\theta_{VL,av} + \theta_{RL,av})
  $$

  $$
  \theta_{VL,av} = (\theta_{VA} - \theta_{i,h}) \cdot \beta_{d}^{1/n} + \theta_{i,h}
  $$

  $$
  \theta_{RL,av} = (\theta_{RA} - \theta_{i,h}) \cdot \beta_{d}^{1/n} + \theta_{i,h}
  $$

  * Clamped load factor $\beta_d = \frac{Q_{h,nd} + Q_{h,ce}}{\Phi_{h,max} \cdot t_h} \cdot f_{hydr}$ (clamped to $[0, 1]$).
  * Exponent $n = 1.3$ for `Radiator` and $1.1$ for `UnderfloorHeating`.
* **Pipe Lengths ($L_{total}$):**

  $$
  L_{total} = L_A + L_S + L_V = (0.28 + 0.08 + 0.05) \cdot A_{NGF} = 0.41 \cdot A_{NGF}
  $$
* **Linear Transmittance ($U_L$):**

  * `PipeInsulation::Uninsulated` $\implies 2.20\text{ W/mK}$
  * `PipeInsulation::EnEV50` $\implies 0.40\text{ W/mK}$
  * `PipeInsulation::EnEV100` $\implies 0.25\text{ W/mK}$
* **Ambient Pipe Temperature ($\theta_{amb,d}$):**

  * `in_unheated_basement: true` $\implies 13.0^\circ\text{C}$
  * `in_unheated_basement: false` $\implies \theta_{i,h}$
* **Circulation Pump Power ($P_{Pu,d}$):**

  * $$
    P_{Pu,d} = \text{base} + \text{HE\_factor} \cdot A_{NGF}
    $$
  * `PumpControl::Uncontrolled` $\implies 20.0 + 0.15 \cdot A_{NGF}$
  * `PumpControl::RegulatedDeltaPV` $\implies 15.0 + 0.10 \cdot A_{NGF}$
  * `PumpControl::RegulatedDeltaPC` $\implies 5.0 + 0.05 \cdot A_{NGF}$
* **Pump Correction Factor ($e_d$):**

  * `PumpControl::Uncontrolled` $\implies 1.0$
  * `PumpControl::RegulatedDeltaPV` $\implies 0.4$
  * `PumpControl::RegulatedDeltaPC` $\implies 0.6$

---

### 9.3 Subsystem 3: Storage (Wärmespeicherung)

Accounts for buffer storage tank heat loss based on insulation quality:

$$
Q_{h,in,s} = Q_{h,in,d} + Q_{h,s}
$$

$$
Q_{h,s} = \frac{1}{1000} \cdot H_{s,st} \cdot (\theta_{s,av} - \theta_{amb,s}) \cdot t_h
$$

$$
W_{h,s} = w_{h,s} \cdot A_{NGF}
$$

* **Loss Coefficient ($H_{s,st}$):**
  * `BufferTank::None` $\implies 0.0\text{ W/K}$
  * `BufferTank::StandardUnheated` $\implies 0.16 \cdot \sqrt{V_s}$
  * `BufferTank::HighEffUnheated` $\implies 0.10 \cdot \sqrt{V_s}$
  * `BufferTank::HighEffHeated` $\implies 0.08 \cdot \sqrt{V_s}$
* **Storage Temperature ($\theta_{s,av}$):**
  * `BufferConnection::Series` $\implies \theta_{RL,av} + 2.0\text{ K}$ (using $\beta_s = \frac{(Q_{h,nd} + Q_{h,ce}) \cdot f_{hydr} + Q_{h,d}}{\Phi_{h,max} \cdot t_h}$)
  * `BufferConnection::Parallel` $\implies \theta_{VL,av} + 2.0\text{ K}$
* **Ambient Storage Temperature ($\theta_{amb,s}$):**
  * `inside_heated_envelope: true` $\implies 20.0^\circ\text{C}$
  * `inside_heated_envelope: false` $\implies 13.0^\circ\text{C}$
* **Storage Pump electricity ($w_{h,s}$):**
  * `BufferTank::None` $\implies 0.0\text{ kWh/m²a}$
  * Others $\implies 0.4\text{ kWh/m²a}$

---

### 9.4 Subsystem 4: Generation (Wärmeerzeugung)

Calculates final energy demand by dividing accumulated heat demand by the efficiency factor ($e_{g,h}$ or reciprocal of COP) and applying pump management correction ($f_{gen,PM}$):

$$
Q_{h,f} = Q_{h,outg} \cdot e_{g,h} \cdot f_{gen,PM}
$$

$$
Q_{h,outg} = Q_{h,nd} + Q_{h,ce} + Q_{h,d} + Q_{h,s}
$$

$$
W_{h,gen} = w_{h,g} \cdot A_{NGF}
$$

* **Expenditure / Efficiency Factor ($e_{g,h}$):**
  * `GeneratorType::OldGasBoiler` / `CondensingGasBoiler`:
    * Interpolates boiler performance based on nominal power $P_n$ ($10\text{ kW}$ to $400\text{ kW}$ logarithmic scaling):
      $$
      p_{factor} = \max\left(0.0,\; \frac{\ln(P_n / 10.0)}{\ln(40.0)}\right)
      $$
    * `OldGasBoiler` $\implies \eta_{100} = 84.0 + 4.0 \cdot p_{factor}$, $\eta_{30} = 79.0 + 6.0 \cdot p_{factor}$.
    * `CondensingGasBoiler` $\implies \eta_{100} = 96.0 + 1.0 \cdot p_{factor}$, $\eta_{30} = 103.0 + 2.0 \cdot p_{factor}$.
    * $\eta_{avg} = 0.3 \cdot \eta_{30} + 0.7 \cdot \eta_{100}$.
    * $$
      e_{g,h} = \frac{1}{\eta_{avg} / 100.0}
      $$
  * `GeneratorType::PelletBoiler` $\implies e_{g,h} = \frac{1}{0.85}$
  * `GeneratorType::AirSourceHeatPump`:
    * $$
      e_{g,h} = \frac{1}{COP} \quad \text{where} \quad COP = 0.45 \cdot \frac{\theta_{sink} + 273.15}{\theta_{sink} - \theta_{source}}
      $$
    * $\theta_{sink} = \theta_{HK,av}$ (using $\beta_{gen}$), $\theta_{source} = \theta_e$.
  * `GeneratorType::GroundSourceHeatPump`:
    * $$
      e_{g,h} = \frac{1}{COP} \quad \text{where} \quad COP = 0.52 \cdot \frac{\theta_{sink} + 273.15}{\theta_{sink} - \theta_{source}}
      $$
    * $\theta_{sink} = \theta_{HK,av}$, $\theta_{source} = 0.0^\circ\text{C}$ (Brine sole).
  * `GeneratorType::DirectElectric` $\implies e_{g,h} = 1.0$
* **Pump Management Factor ($f_{gen,PM}$):**
  * `PumpManagement::None` $\implies 1.00$
  * `PumpManagement::OutdoorGuided` $\implies 1.03$
  * `PumpManagement::RoomGuided` $\implies 1.06$
* **Generator Aux Power ($w_{h,g}$):**
  * `GeneratorType::OldGasBoiler` $\implies 0.5\text{ kWh/m²a}$
  * `GeneratorType::CondensingGasBoiler` $\implies 1.2\text{ kWh/m²a}$
  * `GeneratorType::PelletBoiler` $\implies 2.5\text{ kWh/m²a}$
  * Others $\implies 0.0$

### 9.5 Total Auxiliary & Final Energy Sums

* **Auxiliary electricity:**
  $$
  W_{h,total} = W_{h,ce} + W_{h,d} + W_{h,s} + W_{h,gen}
  $$
* **Total electricity:**
  * If generator consumes electricity (heat pump, direct electric):
    $$
    \text{Total Electricity} = Q_{h,f} + W_{h,total}
    $$
  * If generator is combustion-based:
    $$
    \text{Total Electricity} = W_{h,total}
    $$

## 12. Lighting Energy ($Q_{l,f}$)

**Concept & Formula:**
Calculates the final annual electrical energy demand for artificial lighting according to DIN V 18599-4. It divides the room into daylight-supplied ($A_{TL}$) and non-daylight-supplied ($A_{kTL}$) zones, and evaluates the installed power density ($p_j$) alongside factors for daylight and presence control.

**Source Mapping:**

* **Engine:** Calculated dynamically inside `lighting::LightingEngine::calculate_annual_final_energy_kwh`.

### 12.1 Room Geometry & Daylight Zones

The `RoomGeometry` struct evaluates the physical space.

* **Daylight Quotient ($D_{Rb}$):** Estimated using window area ($A_{window}$), lintel height ($h_{sturz}$), and working plane height ($h_{nutz}$).
* **Daylight Zone Split:** The engine calculates the maximum depth of daylight penetration:
  $$
  A_{TL,max} = 2.5 \cdot (h_{sturz} - h_{nutz}) \cdot \text{width}
  $$

  The floor area is divided into $A_{TL}$ (Daylight Zone) and $A_{kTL}$ (Artificial Only Zone).

### 12.2 Installed Power Density ($p_j$)

Calculated in `calculate_installed_power_density()`.

$$
p_j = p_{j,lx} \cdot E_m \cdot k_{WF} \cdot k_A \cdot k_L
$$

* **Base Power per Lux ($p_{j,lx}$):** Extracted via `calculate_base_power_per_lux()`, depending on `LightingType` (e.g., `Direct`, `Indirect`) and the Room Index ($k$).
* **Required Lux ($E_m$):** The required illuminance from the `LightingRequirement` struct (linked to the massive `RoomUsage` enum, e.g., `WritingTypingReadingDataProc` $\implies 500 \text{ lx}$).
* **Maintenance Factor ($k_{WF}$):** Constant multiplier (`K_WF_DEFAULT = 0.80 / 0.67`).
* **Task Area Reduction ($k_A$):** Hardcoded to $0.85$.
* **Lamp Technology Factor ($k_L$):** Determined by `LampTechnology` (e.g., `LEDLuminaire` $\implies 0.49$, `Halogen` $\implies 5.0$).

### 12.3 Control Factors

* **Presence Factor ($F_{Prä}$):** Calculated in `calculate_f_pra()` based on `PresenceControl` (e.g., `MotionDetector` $\implies c_{prä,kon} = 0.95$) and the relative absence fraction ($C_A$).
  $$
  F_{Prä} = 1.0 - (C_A \cdot c_{prä,kon})
  $$
* **Daylight Factor ($F_{TL}$):** Calculated in `calculate_f_tl()`. Determines how much artificial light is saved by natural daylight, depending on `DaylightControl` (e.g., `DimmedAutoOnOff`, `Manual`).

### 12.4 Final Annual Demand

The engine combines the effective lighting hours during the day ($t_{day}$) and night ($t_{night}$) across both daylight zones.

$$
Q_{l,f} = \frac{p_j}{1000} \cdot \Big(A_{TL} \cdot (t_{eff,day,TL} + t_{eff,night}) + A_{kTL} \cdot (t_{eff,day,kTL} + t_{eff,night})\Big)
$$

## 13. Domestic Hot Water (DHW) Engine

**Concept & Formula:**
This module calculates the Final Energy Demand for Domestic Hot Water ($Q_{w,f}$) and the Auxiliary Electrical Energy ($W_w$) according to DIN V 18599-8. It follows the thermal chain from base demand through distribution, storage, and generation.

### 13.1 Base Demand & Heat Recovery
The required hot water demand ($q_{w,b}$) is determined by the specific building profile and usage. 

**Source Mapping (`DHWEngine::calculate_final_energy`):**
*   **Base Thermal Demand:** Passed in as $q_{w,b\_annual}$. For standard systems, it is derived from liters per day.
*   **Shower Demand:** Estimated as $60\%$ of total demand.
    $$
    q_{w,shower} = q_{w,b} \cdot 0.60
    $$
*   **Wastewater Heat Recovery (WRG):**
    $$
    q_{w,wrg} = q_{w,shower} \cdot \text{wastewater\_heat\_recovery}
    $$
*   **Reduced Demand:**
    $$
    q_{w,b,reduced} = \max(0.0,\; q_{w,b} - q_{w,wrg})
    $$

### 13.2 Subsystem 1: Distribution (Verteilung)
Calculates pipe losses for tapping lines and circulation loops.

**Formulas (`DHWEngine::calculate_final_energy`):**
$$
Q_{w,d} = Q_{w,d,tap} + Q_{w,d,c}
$$
$$
Q_{w,d,tap} = \frac{1}{1000} \cdot U_{l,tap} \cdot L_{w,tap} \cdot (\theta_{w,t} - \theta_{amb}) \cdot t_{tap}
$$
$$
Q_{w,d,c} = \frac{1}{1000} \cdot U_{l,c} \cdot L_{w,c} \cdot (\theta_{w,c,av} - \theta_{amb}) \cdot t_{circ}
$$

**Source Mappings:**
*   **Temperatures:** $\theta_{w,t} = 60.0^\circ\text{C}$, $\theta_{w,c,av} = 57.5^\circ\text{C}$, $\theta_{amb} = 20.0^\circ\text{C}$, $t_{tap} = 30.0\text{ h/a}$.
*   **Lengths ($L_{w,tap}$, $L_{w,c}$):**
    *   `DHWSystemType::Centralized` $\implies L_{w,tap} = 0.05 \cdot A_{NGF}$
    *   `DHWSystemType::Decentralized` $\implies L_{w,tap} = 0.015 \cdot A_{NGF}$
    *   `has_circulation == true` $\implies L_{w,c} = 0.06 \cdot A_{NGF}$ (otherwise 0).
*   **Linear Transmittances ($U_{l,tap}$, $U_{l,c}$):**
    *   `PipeInsulationDHW::Uninsulated` $\implies U_{l,tap} = 1.80$, $U_{l,c} = 2.20$
    *   `PipeInsulationDHW::Insulated50Percent` $\implies U_{l,tap} = 0.35$, $U_{l,c} = 0.40$
    *   `PipeInsulationDHW::Insulated100Percent` $\implies U_{l,tap} = 0.22$, $U_{l,c} = 0.25$
*   **Circulation Hours ($t_{circ}$):**
    *   `Residential1_2Family` / `OfficesCommercial` $\implies 420.0\text{ h/month}$
    *   `ResidentialMultiFamily` / `HospitalsHotels` $\implies 744.0\text{ h/month}$

### 13.3 Subsystem 2: Storage (Speicherung)
Accounts for hot water tank heat loss based on insulation quality.

**Formulas:**
$$
Q_{w,s} = \frac{1}{1000} \cdot H_{w,st} \cdot (\theta_{w,s,av} - \theta_{amb}) \cdot t_{mth}
$$

**Source Mappings:**
*   **Loss Coefficient ($H_{w,st}$):**
    *   `TankInsulationDHW::VeryGoodClassA` $\implies 1.10 + (V_s - 100.0) \cdot 0.001$
    *   `TankInsulationDHW::StandardClassC` $\implies 1.60 + (V_s - 100.0) \cdot 0.003$
    *   `TankInsulationDHW::PoorOld` $\implies 2.50 + (V_s - 100.0) \cdot 0.005$
    *   `TankInsulationDHW::None` $\implies 0.0$
*   **Temperatures:** $\theta_{w,s,av} = 60.0^\circ\text{C}$, $\theta_{amb} = 20.0^\circ\text{C}$.

### 13.4 Subsystem 3: Generation (Wärmeerzeugung)
Calculates the final energy demand by applying the expenditure factor ($e_{g,w}$) to the total required heat output.

**Formulas:**
$$
Q_{w,outg} = q_{w,b,reduced} + Q_{w,d} + Q_{w,s}
$$
$$
Q_{w,outg,req} = \max(0.0,\; Q_{w,outg} - Q_{solar\_thermal})
$$
$$
Q_{w,f} = Q_{w,outg,req} \cdot e_{g,w}
$$

**Source Mappings ($e_{g,w}$):**
*   `ElectricInstantaneous` $\implies 1.00$
*   `ElectricSmallStorage` $\implies 1.02$
*   `ElectricStandardStorage` $\implies 1.05$
*   `GasInstantaneousNew` $\implies 1.15$
*   `GasStorageNew` $\implies 1.25$
*   `CombinedGasBoilerCondensing` $\implies$ Summer: $2.50$, Winter: $1.10$
*   `DistrictHeating` $\implies 1.02$
*   `HeatPumpAirWater` $\implies$ Calculates COP using $\eta_{carnot} = 0.38$, $\theta_{source} = 5.0^\circ\text{C}$, $\theta_{sink} = 55.0^\circ\text{C}$. $e_{g,w} = 1 / \text{COP}$.

### 13.5 Auxiliary Electricity ($W_w$)
Sum of electricity required for pumps and generation controls.
$$
W_w = W_{w,d,c} + W_{w,s} + W_{w,gen}
$$
*   **Circulation Pump:** $W_{w,d,c} = (35.0 \text{ W} \cdot t_{circ}) / 1000.0$
*   **Storage Pump:** $W_{w,s} = (45.0 \text{ W} \cdot t_{load}) / 1000.0$
*   **Generator Standby/Active:** $W_{w,gen} = (60.0 \cdot t_{load} + 10.0 \cdot (t_{mth} - t_{load})) / 1000.0$

If the generator is electrically driven (e.g. Heat Pump, Electric Instantaneous), the Final Electricity Demand is $Q_{w,f} + W_w$. Otherwise, Fuel Demand is $Q_{w,f}$ and Electricity Demand is just $W_w$.
