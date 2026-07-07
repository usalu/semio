# Building Energy Calculation Engine (DIN 18599)

This document details the physics and logic used in the Rust calculation engine (`lib.rs`) for computing the building's energy balance.

# PART A: Heat Sinks (Losses)

## 6.1 Transmission Heat Transfer ($H_T$)

This document explains the physics and logic used in the Rust calculation engine (`lib.rs` / `transmission` module) to compute the building's transmission heat transfer coefficient ($H_T$) according to DIN V 18599-2 and ISO 13790.

### 1. Inputs & Boundary Conditions

To perform this calculation, the UI needs to provide two categories of data: **Automated Geometry** and **User Parameter Questions**.

### A. Automated Geometry (Calculated by the Sketchpad UI)

The UI dynamically calculates the shape of the building and provides these numbers:

- `net_wall`: Total exterior vertical wall area minus windows ($m^2$)
- `a_roof`: Total roof area ($m^2$)
- `a_ground`: Total ground/floor footprint area ($m^2$)
- `win_n`, `win_e`, `win_s`, `win_w`: Vertical window areas by direction ($m^2$)
- `win_h`: Horizontal/Roof window area ($m^2$)
- `roof_pitch_deg`: The slope angle of the roof (in degrees, 0 for flat).

### B. User Questions (Parameters)

The UI presents the following human-readable questions to determine the building physics context:

| Parameter Key               | UI Question / Prompt                                                 | Available Options                                                                                       | How Rust uses it                                                                                                                                 |
| :-------------------------- | :------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------- |
| `thermal_bridge_category` | **"What is the thermal bridge planning standard?"**            | - Standard Default- Good Planning- Excellent Planning- Internal Insulation Issues                       | Determines the$\Delta U_{WB}$ penalty (0.10, 0.05, 0.03, or 0.15).                                                                             |
| `ground_contact_type`     | **"What is below the lowest floor?"**                          | - Unheated Basement- Floor Slab On Ground- Heated Basement- Ventilated Crawl Space- Groundwater Contact | Maps to the$F_x$ temperature correction factor (e.g., 0.5 for standard unheated ground, 1.0 for groundwater).                                  |
| `shutter_control`         | **"How are window shutters controlled?"**                      | - Manual- Automated- None                                                                               | Determines the shutter usage fraction ($f_{sh}$) which improves the effective window U-value ($U_{w,eff}$).                                  |
| `climate_region`          | **"In which climate region is the building located?"**         | 15 German Cities (e.g. Potsdam, Mannheim, Fichtelberg)                                                  | Maps to a specific 12-month temperature profile to dynamically calculate the external average temperature ($\theta_e$) for the heating season. |
| `usage_profile`           | **"What is the primary usage of the building?"**               | - Residential- Single Office- Hospital Room- Gymnasium- etc.                                            | Establishes the baseline indoor heating setpoint ($\theta_{int}$) and determines the number of active heating days ($d_{hs}$).               |
| `automation_class`        | **"What class of automation/smart thermostats is installed?"** | Class A, B, C, D                                                                                        | Dynamically shifts the heating setpoint ($\theta_{int}$) down for smart energy-saving configurations (e.g., -1.5°C).                          |

---

#### C. Theoretical DIN 18599 Inputs (For Context)

To write your software, your data model must collect all of the following parameters. I have categorized them by where your system must source them.

#### Category A: Architectural Geometry (Sourced via DIN 18599-1 / CAD Model)

These are the physical dimensions of the building envelope.

- **$A_j$ (Component Area):** The area in **$m^2$** of every wall, roof, window, and door facing the exterior, the ground, or an unheated space.
- **$l_j$ (Thermal Bridge Length):** The length in meters of every linear structural joint (e.g., balcony connections, window perimeters) if doing detailed thermal bridge calculations.
- **$P$ (Perimeter Length):** The exposed perimeter of the ground slab (required for ground transmission correction).

#### Category B: Material Thermal Properties (Sourced via External Norms)

These define how well the materials resist heat flow.

- **$U_j$ (Thermal Transmittance / U-Value):** Measured in **$W/(m^2K)$**. Sourced via DIN EN ISO 6946 (opaque elements) or DIN EN ISO 10077-1 (windows/doors).
- **$\Psi_j$ (Linear Thermal Transmittance / Psi-Value):** Measured in **$W/(mK)$**. The heat leak rate of a specific joint. Sourced via DIN EN ISO 10211 or DIN 4108 Beiblatt 2.
- **$R_s$ (Surface Thermal Resistances):** Fixed constants used in U-value calculations (**$0.04$** external, **$0.13$** internal walls).

#### Category C: Structural Correction Factors (Sourced via DIN 18599-2)

These factors mathematically adjust the raw U-values based on orientation, location, or dynamic elements.

- **$f_{neig,j}$ (Inclination Factor):** Adjusts window U-values based on tilt. Found in **Part 2, Table 7** (e.g., **$1.0$** for vertical, **$1.2$** for horizontal single glazing).
- **$F_x$ (Temperature Correction Factor):** Reduces heat loss calculations for components that don't face the harsh exterior directly. Found in **Part 2, Tables 5 & 6**.
  - Examples: **$0.8$** (Unheated Attic), **$0.5$** (Unheated Basement), **$0.35$** (Unheated Staircase).
- **$\Delta U_{WB}$ (Simplified Thermal Bridge Addition):** If you don't calculate exact lengths (**$l_j \cdot \Psi_j$**), you add a flat penalty to all U-values.
  - Values: **$0.10$** (Standard), **$0.05$** (Good planning), **$0.03$** (Excellent planning), **$0.15$** (Internal insulation issues).
- **$f_{sh}$ (Shutter Fraction):** The percentage of time a night-shutter is closed, improving the window's U-value (**$U_{w,eff}$**). Found in **Part 2, Annex G**.

#### Category D: Climate & Boundary Conditions (Sourced via DIN 18599-10)

These are the exact numerical triggers that drive the equations, dictating the **$\Delta T$** (temperature difference) and operating hours.

**1. Setpoint Temperatures (The targets):**

- **$\theta_{i,h,soll}$ (Heating Setpoint):** e.g., **$20^\circ C$** (Residential), **$21^\circ C$** (Office), **$15^\circ C$** (Heavy Industry).
- **$\theta_{i,c,soll}$ (Cooling Setpoint):** e.g., **$25^\circ C$** (Residential), **$24^\circ C$** (Office).
- **$\Delta\theta_{i,NA}$ (Setback Temp):** The allowed temperature drop during nights/weekends. Almost universally **$4.0 K$**.
- **Design Extremes:** **$\theta_{i,h,min}$** (e.g., **$20^\circ C$**) and **$\theta_{i,c,max}$** (e.g., **$26^\circ C$**) used only for sizing peak equipment loads.

**2. Building Automation (The smart offsets):**

- **$\Delta\theta_{EMS}$ (Automation Shift):** If the building has smart controls, the setpoints are artificially lowered to save energy calculation-wise.
  - Class A: **$-1.5 K$** to **$-2.0 K$** | Class B: **$-1.0 K$** | Class C/D: **$0.0 K$**.
- **$f_{adapt}$:** Multiplier for adaptive control (**$1.35$** for Classes A/B, **$1.0$** for C/D).

**3. External Climate:**

- **$\theta_e$ (Monthly External Temperature):** 12 specific values depending on the climate region (e.g., Region 4 Potsdam ranges from **$0.1^\circ C$** in Jan to **$18.4^\circ C$** in July).
- **$\theta_{e,min}$ (Winter Peak Design):** Fixed at **$-12^\circ C$**.

**4. Operating Times (The duration of heat flow):**

- **$t_{h,op,d}$ (Daily Heating Hours):** e.g., **$17 \text{ h/d}$** (Residential), **$13 \text{ h/d}$** (Offices).
- **$d_{nutz}$ (Usage Days per year):** e.g., **$365 \text{ d/a}$** (Residential), **$250 \text{ d/a}$** (Offices, meaning weekends are at setback temperatures).
- **$d_{mth}$ (Days per month):** Standard calendar days (28, 30, 31).

### 2. Core Physics Calculations (DIN 18599 Formulas)

#### 1. The Core Transmission Energy Balance (**$Q_T$**)

This calculates the actual energy (in Watt-hours or kWh) lost or gained through the envelope over a specific time period (usually a month).

- **Transmission Heat Sink (Loss - when heating is needed):**

  $$
  Q_{T,sink} = \sum H_{T,j} \cdot (\theta_i - \theta_e) \cdot t
  $$

  _(Calculated for all components **$j$** where **$\theta_i > \theta_e$**)_
- **Transmission Heat Source (Gain - when cooling is needed):**

  $$
  Q_{T,source} = \sum H_{T,j} \cdot (\theta_e - \theta_i) \cdot t
  $$

  _(Calculated for all components **$j$** where **$\theta_e > \theta_i$**)_

**$\theta_i$ (Indoor Target Temperature - Heating **$\theta_{i,h,soll}$** / Cooling **$\theta_{i,c,soll}$**):**

- _Where to find:_ **Table 5** (Residential):
  - **Heating Setpoint ($\theta_{i,h,soll}$):** 20.0 °C
  - **Cooling Setpoint ($\theta_{i,c,soll}$):** 25.0 °C
  - **Building Automation Shift ($\Delta\theta_{EMS}$):**
    - Class D (No automation): 0.0 K
    - Class C (Standard): -0.5 K
    - Class B (Advanced): -1.0 K
    - Class A (High Energy Performance): -1.5 K
- and **Table 7** (Non-Residential, depending on the usage profile 1-43):

- [ ] Lfd. Nr.Nutzung (Usage Profile)Heating Temp. (°C)Cooling Temp. (°C)Automation Shift (Class D / C / B / A)1Einzelbüro21240 K / 0 K / -1.0 K / -1.5 K2Gruppenbüro (2-6 Plätze)21240 K / 0 K / -1.0 K / -1.5 K3Großraumbüro (ab 7 Plätze)21240 K / 0 K / -1.0 K / -1.5 K4Besprechung, Sitzung, Seminar21240 K / 0 K / -1.0 K / -1.5 K5Schalterhalle21240 K / 0 K / -1.0 K / -1.5 K6Einzelhandel/Kaufhaus21240 K / 0 K / -1.5 K / -2.0 K7Einzelhandel (Lebensmittel/Kühl)21240 K / 0 K / -1.5 K / -2.0 K8Klassenzimmer, Gruppenraum21240 K / 0 K / -1.2 K / -2.0 K9Hörsaal, Auditorium21240 K / 0 K / -1.5 K / -2.0 K10Bettenzimmer22240 K / 0 K / -1.0 K / -1.5 K11Hotelzimmer21240 K / 0 K / -1.0 K / -1.5 K12Kantine21240 K / 0 K / -1.5 K / -2.0 K13Restaurant21240 K / 0 K / -1.5 K / -2.0 K14Küchen in Nichtwohngebäuden21240 K / 0 K / -1.5 K / -2.0 K15Küche - Vorbereitung, Lager21240 K / 0 K / -1.5 K / -2.0 K16WC und Sanitärräume21240 K / 0 K / -0.5 K / -1.0 K17Sonstige Aufenthaltsräume21240 K / 0 K / -0.5 K / -1.0 K18Nebenflächen (ohne Aufenthalt)21240 K / 0 K / -0.5 K / -1.0 K19Verkehrsflächen21240 K / 0 K / -0.5 K / -1.0 K20Lager, Technik, Archive21240 K / 0 K / -0.5 K / -1.0 K21Rechenzentrum21240 K / 0 K / -0.5 K / -0.5 K22Gewerbliche Halle - schwere Arbeit15280 K / 0 K / -1.2 K / -1.8 K23Gewerbliche Halle - mittelschwere17260 K / 0 K / -1.2 K / -1.8 K24Gewerbliche Halle - leichte Arbeit20240 K / 0 K / -1.2 K / -1.8 K25Zuschauerbereich (Theater/Veranst.)21240 K / 0 K / -0.5 K / -1.0 K26Foyer (Theater/Veranstaltungen)21240 K / 0 K / -0.3 K / -0.5 K27Bühne (Theater/Veranstaltungen)21240 K / 0 K / -0.3 K / -0.5 K28Messe/Kongress21240 K / 0 K / -1.5 K / -2.0 K29Ausstellungsräume / Museum21240 K / 0 K / -0.75 K / -1.25 K30Bibliothek - Lesesaal21240 K / 0 K / -0.5 K / -1.0 K31Bibliothek - Freihandbereich21240 K / 0 K / -0.5 K / -1.0 K32Bibliothek - Magazin und Depot21240 K / 0 K / -0.5 K / -1.0 K33Turnhalle (ohne Zuschauer)19240 K / 0 K / -0.5 K / -1.0 K34Parkhäuser (Büro/Privat)21240 K / 0 K / -0.5 K / -1.0 K35Parkhäuser (öffentlich)21240 K / 0 K / -0.5 K / -1.0 K36Saunabereich24None0 K / 0 K / -0.5 K / -1.0 K37Fitnessraum20240 K / 0 K / -0.5 K / -1.0 K38Labor22240 K / 0 K / -0.5 K / -1.0 K39Untersuchungs-/Behandlungsräume22240 K / 0 K / -0.5 K / -1.0 K40Spezialpflegebereiche24240 K / 0 K / -0.5 K / -1.0 K41Flure des Pflegebereichs22240 K / 0 K / -0.5 K / -1.0 K42Arztpraxen/Therapeutische Praxen22240 K / 0 K / -0.5 K / -1.0 K43Lagerhallen, Logistikhallen12260 K / 0 K / -0.5 K / -1.0 K

- _Details:_ Must be adjusted by the Building Automation shift ( **$\Delta\theta_{EMS}$** ) found in **Table 5/9**.

**$\theta_e$ (External Climate Temperature):**

- _Where to find:_ **Annex E, Table E.1**.
- _Details:_ Provides the 12 monthly average external temperatures for 15 German climate regions.

| Region | Referenzort            | Jan  | Feb  | Mrz  | Apr  | Mai  | Jun  | Jul  | Aug  | Sep  | Okt  | Nov  | Dez  | Jahreswert |
| :----- | :--------------------- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--------- |
| 1      | Bremerhaven            | 2,9  | 3,2  | 5,4  | 9,0  | 13,1 | 16,0 | 17,9 | 18,2 | 15,0 | 10,6 | 6,1  | 3,2  | 10,1       |
| 2      | Rostock                | 2,3  | 2,4  | 4,3  | 8,0  | 12,4 | 15,6 | 18,0 | 18,0 | 14,7 | 10,2 | 5,5  | 2,6  | 9,5        |
| 3      | Hamburg                | 2,5  | 2,7  | 4,9  | 8,5  | 12,8 | 15,5 | 17,8 | 17,8 | 14,1 | 9,8  | 5,1  | 2,3  | 9,5        |
| 4      | Potsdam (Referenz)     | 1,0  | 1,9  | 4,7  | 9,2  | 14,1 | 16,7 | 19,0 | 18,6 | 14,3 | 9,5  | 4,1  | 0,9  | 9,5        |
| 5      | Essen                  | 3,1  | 3,5  | 6,6  | 9,5  | 13,7 | 15,9 | 18,2 | 18,2 | 14,6 | 10,8 | 6,1  | 3,5  | 10,4       |
| 6      | Bad Marienberg         | 0,1  | 0,5  | 3,6  | 7,0  | 11,5 | 14,0 | 16,1 | 16,0 | 12,3 | 8,1  | 3,2  | 0,6  | 7,8        |
| 7      | Kassel                 | 1,0  | 2,1  | 5,2  | 8,8  | 13,3 | 15,9 | 18,1 | 17,8 | 13,7 | 9,5  | 4,5  | 1,7  | 9,3        |
| 8      | Braunlage              | -0,8 | -0,3 | 2,1  | 5,7  | 10,5 | 12,9 | 15,0 | 15,0 | 11,1 | 7,1  | 2,3  | -0,2 | 6,7        |
| 9      | Chemnitz               | 0,5  | 1,0  | 3,9  | 8,2  | 12,9 | 15,5 | 17,5 | 17,6 | 13,2 | 9,2  | 3,8  | 0,8  | 8,7        |
| 10     | Hof                    | -1,2 | -0,4 | 2,8  | 6,6  | 11,7 | 14,5 | 16,3 | 16,6 | 12,0 | 7,6  | 2,3  | -0,7 | 7,4        |
| 11     | Fichtelberg            | -3,3 | -3,5 | -1,3 | 2,3  | 7,4  | 9,8  | 12,2 | 12,4 | 8,1  | 4,4  | -0,6 | -2,8 | 3,8        |
| 12     | Mannheim               | 2,4  | 3,6  | 7,1  | 10,6 | 15,6 | 18,1 | 20,1 | 20,2 | 15,7 | 11,0 | 5,7  | 3,1  | 11,1       |
| 13     | Passau                 | -1,2 | 0,4  | 4,3  | 8,2  | 13,7 | 16,4 | 18,0 | 17,8 | 13,1 | 8,7  | 3,0  | -0,2 | 8,6        |
| 14     | Stötten               | -0,5 | 0,3  | 3,4  | 6,8  | 11,8 | 14,4 | 16,6 | 16,7 | 12,3 | 8,5  | 2,6  | -0,2 | 7,8        |
| 15     | Garmisch-Partenkirchen | -2,3 | -0,5 | 3,2  | 7,0  | 11,8 | 14,8 | 16,6 | 16,4 | 12,3 | 8,4  | 1,9  | -1,8 | 7,4        |

##### Design Extremes for Equipment Sizing ($\theta_{e,min}$ and $\theta_{e,max}$)

While the monthly averages (Table E.1) are used to calculate the annual energy demand ($kWh/a$), you cannot use them to calculate the peak load ($kW$) required to size the actual heating boiler or chiller.

For maximum load calculations, Table 12 provides the extreme single-day external temperatures:

- **Design Temperature for Heating ($\theta_{e,min}$):** $-12.0\ ^\circ\text{C}$ (Used universally for the winter design day in January to calculate maximum heating load).
- **Design Temperatures for Cooling ($\theta_{e,max}$):**
  - $25.0\ ^\circ\text{C}$ (Used for the summer design day in July).
  - $20.3\ ^\circ\text{C}$ (Used for the autumn design day in September).
    _(Note: The maximum cooling design temperature is relatively mild ($25.0\ ^\circ\text{C}$) because DIN 18599 balances it against simultaneous extreme solar radiation ($I_{S,max}$) of up to $927\ \text{W/m}^2$ on the same day. The combination of $25\ ^\circ\text{C}$ ambient air plus max solar gain produces the peak cooling demand)._

**$t$ (Operating Time/Duration):**

- _Where to find:_ Derived from **$d_{nutz}$** (Usage days) and **$t_{h,op,d}$** (Daily operating hours) found in **Table 5** (Res) and **Table 6** (Non-Res). Also requires **$d_{mth}$** (Days per month) from **Table 11**.

1. **Days per Month ($d_{mth}$) - From Table 11**
   These are the standard calendar days used to determine the total hours available in a given month. The standard assumes a non-leap year (365 days).

   - Jan: 31 | Feb: 28 | Mar: 31 | Apr: 30 | May: 31 | Jun: 30
   - Jul: 31 | Aug: 31 | Sep: 30 | Oct: 31 | Nov: 30 | Dec: 31
   - Total hours in a month: $t_{mth} = d_{mth} \times 24 \text{ hours}$ (e.g., January = 744 hours).
2. **Residential Buildings (Wohngebäude) - From Table 5**
   For any residential building, the operating times are strictly fixed to one profile:

   - Usage days per year ($d_{nutz,a}$): 365 days
   - Daily heating/cooling operation ($t_{h,op,d}$): 17 hours/day
   - _Application:_ This means a residential home is fully heated for 17 hours a day, 7 days a week. The remaining 7 hours of the day are calculated using the setback temperature ($\Delta\theta_{i,NA} = 4\text{ K}$).
3. **Non-Residential Buildings (Nichtwohngebäude) - From Table 6**
   For non-residential buildings, the operating times depend entirely on the usage profile. I have extracted the Usage Days per Year ($d_{nutz,a}$) and the Daily Heating Operation Hours ($t_{h,op,d}$) for all 43 profiles.
   _(Note: The daily heating operation time $t_{h,op,d}$ in the standard already includes the necessary pre-heating and post-heating times required to bring the building up to temperature before people arrive)._

| Lfd. Nr. | Nutzung (Usage Profile)             | Usage Days / Year (d_nutz) | Daily Heating Hours (t_h,op,d) |
| :------- | :---------------------------------- | :------------------------- | :----------------------------- |
| 1        | Einzelbüro                         | 250                        | 13                             |
| 2        | Gruppenbüro (2-6 Plätze)          | 250                        | 13                             |
| 3        | Großraumbüro (ab 7 Plätze)       | 250                        | 13                             |
| 4        | Besprechung, Sitzung, Seminar       | 250                        | 13                             |
| 5        | Schalterhalle                       | 250                        | 13                             |
| 6        | Einzelhandel/Kaufhaus               | 300                        | 12                             |
| 7        | Einzelhandel (Lebensmittel/Kühl)   | 300                        | 12                             |
| 8        | Klassenzimmer, Gruppenraum          | 250                        | 12                             |
| 9        | Hörsaal, Auditorium                | 250                        | 12                             |
| 10       | Bettenzimmer (Krankenhaus)          | 365                        | 24                             |
| 11       | Hotelzimmer                         | 365                        | 24                             |
| 12       | Kantine                             | 250                        | 12                             |
| 13       | Restaurant                          | 365                        | 14                             |
| 14       | Küchen in Nichtwohngebäuden       | 365                        | 14                             |
| 15       | Küche - Vorbereitung, Lager        | 365                        | 14                             |
| 16       | WC und Sanitärräume               | 250                        | 13                             |
| 17       | Sonstige Aufenthaltsräume          | 250                        | 13                             |
| 18       | Nebenflächen (ohne Aufenthalt)     | 250                        | 13                             |
| 19       | Verkehrsflächen                    | 250                        | 13                             |
| 20       | Lager, Technik, Archive             | 250                        | 13                             |
| 21       | Rechenzentrum                       | 365                        | 24                             |
| 22       | Gewerbliche Halle - schwere Arbeit  | 250                        | 14                             |
| 23       | Gewerbliche Halle - mittelschwere   | 250                        | 14                             |
| 24       | Gewerbliche Halle - leichte Arbeit  | 250                        | 14                             |
| 25       | Zuschauerbereich (Theater/Veranst.) | 200                        | 10                             |
| 26       | Foyer (Theater/Veranstaltungen)     | 200                        | 10                             |
| 27       | Bühne (Theater/Veranstaltungen)    | 200                        | 10                             |
| 28       | Messe/Kongress                      | 200                        | 14                             |
| 29       | Ausstellungsräume / Museum         | 300                        | 12                             |
| 30       | Bibliothek - Lesesaal               | 300                        | 13                             |
| 31       | Bibliothek - Freihandbereich        | 300                        | 13                             |
| 32       | Bibliothek - Magazin und Depot      | 300                        | 13                             |
| 33       | Turnhalle (ohne Zuschauer)          | 300                        | 14                             |
| 34       | Parkhäuser (Büro/Privat)          | 250                        | 13                             |
| 35       | Parkhäuser (öffentlich)           | 300                        | 14                             |
| 36       | Saunabereich                        | 365                        | 14                             |
| 37       | Fitnessraum                         | 365                        | 14                             |
| 38       | Labor                               | 250                        | 13                             |
| 39       | Untersuchungs-/Behandlungsräume    | 250                        | 13                             |
| 40       | Spezialpflegebereiche               | 365                        | 24                             |
| 41       | Flure des Pflegebereichs            | 365                        | 24                             |
| 42       | Arztpraxen/Therapeutische Praxen    | 250                        | 12                             |
| 43       | Lagerhallen, Logistikhallen         | 250                        | 14                             |

#### 2. The Overall Transmission Heat Transfer Coefficient (**$H_T$**)

This formula sums up the "leakiness" of the entire building envelope (in W/K).

$$
H_T = H_{T,D} + \sum(F_{x,j} \cdot H_{T,iu,j}) + H_{T,WB}
$$

**$F_x$ (Temperature Correction Factor):**

- _Where to find:_ **Table 5** (Unheated rooms) and **Table 6** (Ground slabs/basements).
- _Details:_ e.g., **$F_x = 0.8$** for unheated attics, **$F_x = 0.5$** for unheated basements.
- **Table 5: Unheated Spaces ($F_x$ für unbeheizte Räume):**| Lfd. Nr. | Art des angrenzenden unbeheizten Raumes (Type of Unheated Space)    | F_x Factor |
  | :------- | :------------------------------------------------------------------ | :--------- |
  | 1        | Dachraum (Unheated Attic / Roof space)                              | 0.8        |
  | 2        | Unbeheizter Glasvorbau / Wintergarten (Unheated Sunspace)           | 0.8        |
  | 3        | Kriechkeller, stark belüftet (Crawl space, heavily ventilated)     | 0.8        |
  | 4        | Angrenzender unbeheizter Raum (Standard adjacent unheated room)     | 0.5        |
  | 5        | Unbeheizter Keller (Unheated Basement, general)                     | 0.5        |
  | 6        | Treppenhaus, außenliegend (Staircase with large exterior walls)    | 0.5        |
  | 7        | Kriechkeller, unbelüftet (Crawl space, unventilated)               | 0.5        |
  | 8        | Treppenhaus, innenliegend (Staircase mostly surrounded by building) | 0.35       |
- **Table 6: Ground Contact ($F_x$ für Bauteile gegen Erdreich):**| Lfd. Nr. | Bauteil gegen Erdreich (Component against the ground)               | F_x Factor |
  | :------- | :------------------------------------------------------------------ | :--------- |
  | 1        | Bodenplatte auf Erdreich (Floor slab directly on ground)            | 0.5        |
  | 2        | Wände gegen Erdreich, < 1,5m Tiefe (Basement walls, shallow depth) | 0.5        |
  | 3        | Wände gegen Erdreich, > 1,5m Tiefe (Basement walls, deep depth)    | 0.5        |
  | 4        | Fußboden des beheizten Kellers (Floor of a heated basement)        | 0.5        |
  | 5        | Bauteile gegen Grundwasser (Components touching groundwater)        | 1.0        |

#### 3. Direct Transmission to Exterior (**$H_{T,D}$**)

Calculates heat transfer through walls, roofs, and windows directly touching the outside air.

$$
H_{T,D} = \sum (f_{neig,j} \cdot U_j \cdot A_j)
$$

**$f_{neig,j}$ (Inclination Factor):**

- _Where to find:_ **Table 7** (18599-2)
- _Details:_ Adjusts window U-values based on their tilt angle (**$0^\circ$** to **$90^\circ$**) and glazing type (single, double, triple). Default is **$1.0$** for opaque walls.
- **Table 7: Inclination Factor ($f_{neig,j}$):**| Neigung (Grad °) | Einfachglas (Single) | Zweifachglas (Double) | Dreifachglas (Triple)\* |
  | :---------------- | :------------------- | :-------------------- | :---------------------- |
  | 0° (Horizontal)  | 1,25                 | 1,21                  | 1,20                    |
  | 15°              | 1,21                 | 1,22                  | 1,16                    |
  | 30°              | 1,19                 | 1,21                  | 1,13                    |
  | 45°              | 1,21                 | 1,15                  | 1,07                    |
  | 60°              | 1,00                 | 1,13                  | 1,05                    |
  | 75°              | 1,00                 | 1,08                  | 1,02                    |
  | 90° (Vertikal)   | 1,00                 | 1,00                  | 1,00                    |

##### 1. Das mathematische Prinzip

Der U-Wert ist der Kehrwert des gesamten Wärmedurchgangswiderstands ($R_T$):

$$
U = \frac{1}{R_T}
$$

Der gesamte Wärmedurchgangswiderstand setzt sich aus den Schichtwiderständen und den Übergangswiderständen zusammen:

$$
R_T = R_{si} + \sum \left( \frac{d_i}{\lambda_i} \right) + R_{se}
$$

Dabei ist:

- $d_i$: Dicke der Schicht $i$ in Metern ($m$).
- $\lambda_i$: Wärmeleitfähigkeit des Materials der Schicht $i$ in $W/(m \cdot K)$.
- $R_{si}, R_{se}$: Übergangswiderstände (konstant, wie von dir genannt).

#### 4. Transmission to Unheated Spaces (**$H_{T,iu}$**)

Calculates heat transfer through walls/floors touching unheated rooms (like attics or basements) or the ground.

$$
H_{T,iu} = \sum (U_j \cdot A_j)
$$

_(Note: To get the effective heat transfer, this is multiplied by the temperature correction factor **$F_x$** as shown in Formula 2)._

```rust
pub struct Material {
    pub name: String,
    pub lambda: f64, // Wärmeleitfähigkeit W/(m*K)
}

pub struct Layer {
    pub material: Material,
    pub thickness: f64, // Dicke in Metern
}

pub struct BuildingComponent {
    pub layers: Vec<Layer>,
    pub r_si: f64, // Standard 0.13
    pub r_se: f64, // Standard 0.04
}

impl BuildingComponent {
    // Berechnet den U-Wert basierend auf der aktuellen Dicke aller Schichten
    pub fn calculate_u_value(&self) -> f64 {
        let sum_r: f64 = self.layers.iter()
            .map(|l| l.thickness / l.material.lambda)
            .sum();

        let r_t = self.r_si + sum_r + self.r_se;
        1.0 / r_t
    }
}

impl BuildingComponent {
    pub fn calculate_u_value_internal(&self) -> f64 {
        // Bei Innenbauteilen: R_se wird zu R_si (meist 0.13 + 0.13)
        let sum_r: f64 = self.layers.iter()
            .map(|l| l.thickness / l.material.lambda)
            .sum();

        let r_t = self.r_si + sum_r + self.r_si;
        1.0 / r_t
    }
}
```

#### 5. Thermal Bridges (**$H_{T,WB}$**)

Calculates the extra heat lost through structural joints (balconies, window frames, corners). The standard allows for two calculation methods.

- **Simplified Method (Flat Addition):**
  $$
  H_{T,WB} = \Delta U_{WB} \cdot \sum A_j
  $$

**$\Delta U_{WB}$ (Simplified Thermal Bridge Penalty):**

- _Where to find:_ **Section 6.1.4 (Text categories)** (18599-2)
- _Details:_ Flat values of **$0.10$** (default), **$0.05$**, **$0.03$**, or **$0.15 W/(m^2K)$** depending on the construction quality and adherence to DIN 4108 Beiblatt 2.

| Penalty Value ($\Delta U_{WB}$) | Construction Condition / Requirement (Anwendungsbedingung)                                                                                                                                                  |
| :-------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 0.15 W/(m²K)                     | Increased Penalty: Must be used for buildings with internal insulation (Innendämmung) where solid floor ceilings intersect the exterior wall without thermal separation, creating massive thermal bridges. |
| 0.10 W/(m²K)                     | Standard Default (Ohne Nachweis): Used when no special thermal bridge planning is done, or if the construction details do not conform to the standard examples provided in DIN 4108 Beiblatt 2.             |
| 0.05 W/(m²K)                     | Good Planning (Kategorie A): Allowed only if it is explicitly proven that all thermal bridges in the building match the standard, energy-optimized design examples shown in DIN 4108 Beiblatt 2.            |
| 0.03 W/(m²K)                     | Excellent Planning (Kategorie B): Allowed only if proven that all thermal bridges match the highly-insulated, premium design examples (Category B) defined in DIN 4108 Beiblatt 2.                          |

#### 6. Effective Thermal Transmittance for Windows with Shutters (**$U_{w,eff}$**)

If a window has a night shutter, its **$U$**-value improves dynamically based on how long the shutter is closed.

$$
U_{w,eff} = U_w \cdot (1 - f_{sh}) + U_{w,sh} \cdot f_{sh}
$$

**$f_{sh}$ (Shutter Usage Fraction):**

- _Where to find:_ **Annex G, Tables G.1, G.2, G.3** (18599-2)
- _Details:_ The fraction of the day the shutter is closed, depending on the automation control type.
- **Table G.1: Residential Buildings (Wohngebäude):**| Monat (Month) | f_sh : Manuell (Manual Control) | f_sh : Automatisch (Automated / Motorized) |
  | :------------ | :------------------------------ | :----------------------------------------- |
  | Januar        | 0,43                            | 0,61                                       |
  | Februar       | 0,38                            | 0,54                                       |
  | März         | 0,32                            | 0,45                                       |
  | April         | 0,25                            | 0,36                                       |
  | Mai           | 0,20                            | 0,28                                       |
  | Juni          | 0,16                            | 0,23                                       |
  | Juli          | 0,18                            | 0,25                                       |
  | August        | 0,23                            | 0,33                                       |
  | September     | 0,29                            | 0,42                                       |
  | Oktober       | 0,37                            | 0,53                                       |
  | November      | 0,42                            | 0,60                                       |
  | Dezember      | 0,45                            | 0,64                                       |
- **Table G.2 & G.3: Non-Residential Buildings (Nichtwohngebäude):**| Monat (Month) | f_sh : Non-Residential (Manual) | f_sh : Non-Residential (Automated / BMS) |
  | :------------ | :------------------------------ | :--------------------------------------- |
  | Januar        | 0,00                            | 0,61                                     |
  | Februar       | 0,00                            | 0,54                                     |
  | März         | 0,00                            | 0,45                                     |
  | April         | 0,00                            | 0,36                                     |
  | Mai           | 0,00                            | 0,28                                     |
  | Juni          | 0,00                            | 0,23                                     |
  | Juli          | 0,00                            | 0,25                                     |
  | August        | 0,00                            | 0,33                                     |
  | September     | 0,00                            | 0,42                                     |
  | Oktober       | 0,00                            | 0,53                                     |
  | November      | 0,00                            | 0,60                                     |
  | Dezember      | 0,00                            | 0,64                                     |

#### 7. Specific Heat Transfer Coefficient (**$H'_T$**)

Used primarily in Annex F to evaluate the overall energy quality of the building envelope relative to its size.

$$
H'_{T} = \frac{H_{T,D} + H_{T,WB} + \sum(F_{x,j} \cdot H_{T,iu,j})}{A}
$$

#### Mathematical Execution logic

Once the inputs and TABULA U-values are gathered, the engine executes three main equations:

### 1. Direct Transmission ($H_{T,D}$)

Calculates heat transfer directly touching the outside air.
**Components involved:** Exterior Walls, Roof, Windows.
**Formula:**
$H_{T,D} = \sum (A_j \cdot U_j \cdot f_{neig,j})$

_Rust Execution:_

- The engine creates dummy `BuildingComponent` instances for the wall and roof using the TABULA $U$-values.
- It infers `WindowGlazingType` (Single/Double/Triple) by inspecting the Tabula $U_{window}$ value.
- It calculates $f_{neig}$ (inclination factor) based on `roof_pitch_deg` and Glazing Type.
- It calculates $U_{w,eff}$ (effective window U-value) factoring in the `shutter_control` fraction ($f_{sh}$).

### 2. Transmission to Unheated Zones ($H_{T,iu}$)

Calculates heat transfer through components touching the ground or unheated rooms.
**Components involved:** Ground Floor / Basement Ceiling.
**Formula:**
$H_{T,iu} = \sum (A_j \cdot U_j \cdot F_{x,j})$

_Rust Execution:_

- The engine looks at `ground_contact_type` to find $F_x$. (e.g., "Unheated Basement" = 0.5).
- It multiplies the Tabula $U_{floor}$ by the Ground Area and the $F_x$ factor.

### 3. Thermal Bridges ($H_{T,WB}$)

Accounts for extra heat leaking through joints, corners, and structural intersections.
**Formula:**
$H_{T,WB} = \Delta U_{WB} \cdot \sum A_{total}$

_Rust Execution:_

- The engine uses `thermal_bridge_category` to determine the $\Delta U_{WB}$ penalty.
- It sums up the entire envelope area (Walls + Roof + Ground + Windows) and multiplies it by $\Delta U_{WB}$.

### 4. Total Transmission ($H_T$)

Finally, the three parts are summed together.
**Formula:**
$H_T = H_{T,D} + H_{T,iu} + H_{T,WB}$

This $H_T$ value is then passed into the broader ISO 13790 engine to calculate the final `Q_ht` (Total Heat Loss) alongside ventilation losses.

### 5. Dynamic Temperature Application ($Q_{ht}$)

The engine calculates the final heat loss in kWh/year using the specific Temperature Delta:
**Formula:**
$Q_{ht} = 0.024 \cdot H_T \cdot (\theta_{int} - \theta_e) \cdot d_{hs}$

_Rust Execution:_

- The engine maps `climate_region` to get the monthly temperatures and averages the 7 heating months to get $\theta_e$.
- The engine maps `usage_profile` to establish base $\theta_{int}$ and heating days $d_{hs}$.
- The engine maps `automation_class` to apply a precise energy-saving temperature shift to $\theta_{int}$.

---

### 3. Reference Tables & Standards

### 3.4 Exhaustive Lighting Requirements Database (DIN EN 12464-1:2021)

This database contains the exhaustive extraction of lighting requirements from Tables 9 to 61 of DIN EN 12464-1:2021. These values serve as the mandatory target constraints for calculating lighting energy demand ($Q_{l,f}$) in DIN V 18599-4.

**Terminology:**
* $\bar{E}_m$ (lx): Maintained illuminance (Target Lux level).
* $UGR_L$: Unified Glare Rating limit.
* $R_a$: Color Rendering Index.

#### Part 1: General and Common Areas

**Table 9: Traffic Zones (Verkehrszonen)**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ | Remarks |
| --- | --- | --- | --- | --- |
| Entrance halls | 100 | 22 | 80 | - |
| Lounges / Waiting areas | 200 | 22 | 80 | - |
| Circulation areas and corridors | 100 | 28 | 80 | 40 allowed if unoccupied |
| Stairs, escalators, moving walkways | 150 | 25 | 80 | - |
| Loading ramps / bays | 150 | 25 | 80 | - |

**Table 10: Rest, Sanitation, First Aid**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ | Remarks |
| --- | --- | --- | --- | --- |
| Canteens and Pantries | 200 | 22 | 80 | - |
| Rest rooms | 200 | 22 | 80 | - |
| Rooms for physical exercise | 300 | 22 | 80 | - |
| Cloakrooms, washrooms, bathrooms, toilets | 200 | 25 | 80 | - |
| Sick bay / First aid rooms | 500 | 19 | 80 | - |

**Table 11: Control Rooms (Kontrollräume)**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ | Remarks |
| --- | --- | --- | --- | --- |
| Plant rooms, switch gear rooms | 200 | 25 | 80 | - |
| Telex, post room, switchboard | 500 | 19 | 80 | - |

**Table 12 & 13: Storage and Racks**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ | Remarks |
| --- | --- | --- | --- | --- |
| Store and stockrooms (unmanned) | 100 | 25 | 60 | - |
| Store and stockrooms (manned) | 200 | 25 | 80 | - |
| Dispatch and packing areas | 300 | 25 | 80 | - |
| Gangways in rack systems (unmanned) | 20 | - | 40 | - |
| Gangways in rack systems (manned) | 150 | 22 | 80 | Vertical illuminance critical |

#### Part 2: Industrial and Craft Activities

**Table 14: Agriculture (Landwirtschaft)**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Loading and operating of goods | 200 | 25 | 80 |
| Buildings for livestock | 50 | 28 | 40 |
| Sick animal pens | 200 | 25 | 80 |
| Feed preparation | 200 | 25 | 80 |

**Table 15: Bakeries (Backwarenherstellung)**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Preparation and baking | 300 | 22 | 80 |
| Finishing, glazing, decorating | 500 | 22 | 80 |

**Table 16: Cement, Concrete, Bricks**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Drying | 50 | 28 | 40 |
| Preparation of materials, kilns | 200 | 25 | 80 |
| General machine work | 300 | 25 | 80 |
| Rough forms | 300 | 25 | 80 |

**Table 17: Ceramics and Glass**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ | Remarks |
| --- | --- | --- | --- | --- |
| Drying | 50 | 28 | 40 | - |
| Preparation, general machine work | 300 | 25 | 80 | - |
| Enamelling, rolling, pressing | 300 | 25 | 80 | - |
| Grinding, engraving, polishing | 750 | 19 | 80 | - |
| Precision work, decorative painting | 1000 | 16 | 90 | High precision |

**Table 18: Chemicals, Plastics, Rubber**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ | Remarks |
| --- | --- | --- | --- | --- |
| Remote-operated processing | 50 | 28 | 40 | - |
| Processing with limited manual intervention | 150 | 28 | 40 | - |
| Constantly manned work places | 300 | 25 | 80 | - |
| Precision measuring rooms, labs | 500 | 19 | 80 | - |
| Color inspection | 1000 | 16 | 90 | $T_{cp}$ ≥ 4000K |

**Table 19: Electrical and Electronics**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Cable and wire manufacture | 300 | 25 | 80 |
| Winding (large coils) | 300 | 25 | 80 |
| Winding (medium coils) | 500 | 22 | 80 |
| Winding (small coils) | 750 | 19 | 80 |
| Coil impregnating | 300 | 25 | 80 |
| Assembly work (rough) | 300 | 25 | 80 |
| Assembly work (medium) | 500 | 22 | 80 |
| Assembly work (fine) | 750 | 19 | 80 |
| Assembly work (precision) | 1000 | 16 | 80 |
| Electronic workshops, testing | 1500 | 16 | 80 |

**Table 20: Food Industry**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Workplaces/zones in breweries, malting | 200 | 25 | 80 |
| Washing, barrel filling, cleaning | 200 | 25 | 80 |
| Sorting and washing of products | 300 | 25 | 80 |
| Work on color-critical goods | 500 | 22 | 90 |

**Table 21: Foundries and Metal Casting**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Underground tunnels, cellars | 50 | 28 | 40 |
| Platforms | 100 | 25 | 40 |
| Sand preparation | 200 | 25 | 80 |
| Core making, mould making | 300 | 25 | 80 |

**Table 22: Hairdressers**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Hairdressing | 500 | 19 | 90 |

**Table 23: Jewellery**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Working with precious stones | 1500 | 16 | 90 |
| Watch making (manual) | 1500 | 16 | 80 |

**Table 24: Laundries**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Goods in, marking and sorting | 300 | 25 | 80 |
| Washing and dry cleaning | 300 | 25 | 80 |
| Ironing, pressing | 300 | 25 | 80 |
| Inspection and repairs | 750 | 19 | 80 |

**Table 25: Leather Industry**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Vats, barrels, pits | 200 | 25 | 80 |
| Fleshing, skiving, splitting | 300 | 25 | 80 |
| Saddlery, shoe manufacture | 500 | 22 | 80 |
| Quality control | 1000 | 19 | 80 |
| Color inspection | 1000 | 16 | 90 |

**Table 26: Metal Working**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Open die forging | 200 | 25 | 80 |
| Drop forging | 300 | 25 | 80 |
| Welding | 300 | 25 | 80 |
| Rough/Medium machining | 300 | 22 | 80 |
| Precision machining | 500 | 19 | 80 |
| Tool making, cutting equipment | 750 | 19 | 80 |

**Table 27: Paper Industry**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Edge runners, pulp mills | 200 | 25 | 80 |
| Paper manufacture, paper machines | 300 | 25 | 80 |
| Paper inspection | 500 | 22 | 80 |

**Table 28: Power Stations**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Fuel supply plant | 50 | 28 | 40 |
| Boiler house | 100 | 28 | 40 |
| Machine halls | 200 | 25 | 80 |
| Control rooms | 500 | 19 | 80 |

**Table 29: Printing Industry**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Cutting, gilding, embossing | 500 | 19 | 80 |
| Sorting, paper reproduction | 500 | 19 | 80 |
| Type setting, retouching | 1000 | 19 | 80 |
| Color inspection in printing | 1500 | 16 | 90 |

**Table 30: Iron and Steel Works**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Production plants without manual op | 50 | 28 | 40 |
| Production plants with manual op | 150 | 28 | 40 |
| Slab inspection | 200 | 25 | 80 |

**Table 31: Textile Industry**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Bale opening, carding, washing | 200 | 25 | 80 |
| Spinning, plying, reeling | 300 | 22 | 80 |
| Weaving, knitting | 500 | 22 | 80 |
| Sewing, fine knitting | 750 | 22 | 80 |
| Color inspection, fabric control | 1000 | 16 | 90 |

**Table 32: Vehicle Construction**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Body work and assembly | 500 | 22 | 80 |
| Painting, spraying | 750 | 22 | 80 |
| Painting inspection, repair | 1000 | 16 | 90 |

**Table 33: Wood Working**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Automatic processing (sawing) | 200 | 25 | 80 |
| Joiners bench, gluing | 300 | 25 | 80 |
| Polishing, painting | 750 | 22 | 80 |
| Quality control | 1000 | 19 | 90 |

#### Part 3: Commercial and Public Buildings

**Table 34: Offices (Büros)**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Filing, copying, circulation | 300 | 19 | 80 |
| Writing, typing, reading, data proc. | 500 | 19 | 80 |
| CAD work stations | 500 | 19 | 80 |
| Conference and meeting rooms | 500 | 19 | 80 |
| Reception desk | 300 | 22 | 80 |
| Archives | 200 | 25 | 80 |

**Table 35: Retail (Einzelhandel)**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Sales area (small) | 300 | 22 | 80 |
| Sales area (large) | 500 | 22 | 80 |
| Till area / Cashier | 500 | 19 | 80 |
| Wrapper table | 500 | 19 | 80 |

**Table 36: Restaurants and Hotels**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Reception, cashier | 300 | 22 | 80 |
| Kitchens | 500 | 22 | 80 |
| Restaurant, dining room | - | - | 80 |
| Self-service restaurants | 200 | 22 | 80 |
| Buffet | 300 | 22 | 80 |
| Conference rooms (Hotels) | 500 | 19 | 80 |

**Table 37: Exhibitions and Museums**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| General exhibitions | 300 | 22 | 80 |

**Table 38: Libraries (Bibliotheken)**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Bookshelves | 200 | 19 | 80 |
| Reading area | 500 | 19 | 80 |
| Counters | 500 | 19 | 80 |

**Table 39: Public Car Parks (Parkhäuser)**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| In/Out ramps (Day) | 300 | 25 | 40 |
| In/Out ramps (Night) | 75 | 25 | 40 |
| Traffic lanes | 75 | 25 | 40 |
| Parking areas | 75 | 28 | 40 |
| Ticket office | 300 | 19 | 80 |

#### Part 4: Educational and Healthcare Premises

**Table 44: Educational Premises**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Play school rooms / Nursery | 300 | 19 | 80 |
| Nursery class (crafts) | 300 | 19 | 80 |
| Classrooms, tutorial rooms | 300 | 19 | 80 |
| Classrooms for evening classes | 500 | 19 | 80 |
| Auditoriums / Lecture halls | 500 | 19 | 80 |
| Blackboards / Whiteboards | 500 | 19 | 80 |
| Demonstration tables | 500 | 19 | 80 |
| Art and craft rooms | 500 | 19 | 80 |
| Art rooms in art schools | 750 | 19 | 90 |
| Technical drawing rooms | 750 | 19 | 80 |
| Computer practice rooms | 500 | 19 | 80 |
| Language laboratories | 300 | 19 | 80 |
| Preparation rooms | 500 | 22 | 80 |
| Student common rooms | 200 | 22 | 80 |
| Teachers rooms | 300 | 19 | 80 |
| Sports halls / Gymnasiums | 300 | 22 | 80 |

**Tables 45-54: Health Care Premises (Krankenhäuser)**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Waiting rooms | 200 | 22 | 80 |
| Corridors (Day) | 200 | 22 | 80 |
| Corridors (Night) | 50 | 22 | 80 |
| Staff office rooms | 500 | 19 | 80 |
| Staff rooms | 300 | 19 | 80 |
| Wards - General lighting | 100 | 19 | 80 |
| Wards - Reading lighting | 300 | 19 | 80 |
| Wards - Simple examinations | 300 | 19 | 80 |
| Examination and treatment (General) | 500 | 19 | 90 |
| Examination and treatment (Detailed) | 1000 | 19 | 90 |
| Ear and eye examination | 1000 | - | 90 |
| Operating theatre (Pre-op & Recovery) | 500 | 19 | 90 |
| Operating theatre (General) | 1000 | 19 | 90 |
| Operating cavity | 100000 | - | - |
| Intensive care (General) | 100 | 19 | 90 |
| Intensive care (Examination) | 1000 | 19 | 90 |
| Dentists (General) | 500 | 19 | 90 |
| Dentists (At the patient) | 1000 | - | 90 |
| Pharmacies | 500 | 19 | 80 |
| Autopsy rooms | 500 | 19 | 90 |
| Autopsy table | 5000 | - | 90 |

#### Part 5: Transport and Infrastructure

**Table 60: Airports (Flughäfen)**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Arrival and departure halls | 200 | 22 | 80 |
| Baggage claim | 200 | 22 | 80 |
| Connection areas, escalators | 150 | 22 | 80 |
| Information desks, check-in | 500 | 19 | 80 |
| Customs and passport control | 500 | 19 | 80 |
| Waiting areas | 200 | 22 | 80 |
| Luggage sorting rooms | 200 | 25 | 80 |
| Security check | 300 | 19 | 80 |
| Air traffic control tower | 500 | 16 | 80 |

**Table 61: Railway Installations (Bahnanlagen)**
| Activity / Room Type | $\bar{E}_m$ | $UGR_L$ | $R_a$ |
| --- | --- | --- | --- |
| Enclosed platforms | 200 | 28 | 40 |
| Passenger subways / tunnels | 150 | 28 | 40 |
| Ticket halls and concourse | 200 | 28 | 40 |
| Ticket and luggage offices | 500 | 19 | 80 |
| Waiting rooms | 200 | 22 | 80 |




### Implementation Summary

To build the transmission module, your software needs to:

1. Load **Category A** (Geometry) and **Category B** (U-values) to calculate the static building shell **$H_T$**.
2. Apply the modifiers from **Category C** (DIN 18599-2) to adjust for unheated spaces and bridges.
3. Run a monthly loop using the temperatures and times from **Category D** (DIN 18599-10) to figure out exactly how many Watt-hours of energy transferred through that shell over the course of the year.

### 4. Rust Engine Implementation

#### Data Flow & Architecture

The calculation engine combines user inputs from the UI with pre-calculated archetype physics data from the **TABULA** database to compute the final transmission losses without overwhelming the user with overly complex physics inputs.

```mermaid
flowchart TD
    %% UI Inputs
    subgraph UI ["User Interface (Inputs)"]
        G["Geometry Engine\n(Areas, Perimeter, Roof Pitch)"]
        Q1["Thermal Bridge Question"]
        Q2["Ground Contact Question"]
        Q3["Shutter Control Question"]
        Q4["Climate Region"]
        Q5["Usage Profile & Automation"]
    end

    %% Database
    subgraph DB ["TABULA Database"]
        T["U-Values\nWalls, Roof, Floor, Window"]
    end

    %% Rust Engine Middle Layer
    subgraph PreCalc ["Preprocessing (Rust)"]
        FX["Ground F_x Factor\n0.5 to 1.0"]
        DU["Delta U_WB Penalty\n0.03 to 0.15"]
        FSH["Shutter Fraction f_sh"]
        FNEIG["Inclination Factor f_neig"]
        TEMP["Dynamic Temperatures\ntheta_e and theta_int"]
    end

    %% Detailed Calculations
    subgraph Calc ["Transmission Calculations"]
        HTD["HT,D\n(Direct: Walls, Roof, Windows)"]
        HTIU["HT,iu\n(Unheated: Floor/Ground)"]
        HTWB["HT,WB\n(Thermal Bridges)"]
    end

    %% Final Output
    HT["HT Total\n(Transmission Heat Transfer)"]

    G --> |Net Areas| HTD
    G --> |Ground Area| HTIU
    G --> |Total Area| HTWB
    G --> |Roof Pitch| FNEIG

    Q1 --> DU
    Q2 --> FX
    Q3 --> FSH
    Q4 --> |Monthly Averages| TEMP
    Q5 --> |Setpoints & Shifts| TEMP

    T --> |U_wall, U_roof, U_window| HTD
    T --> |U_floor| HTIU
    T --> |U_window| FNEIG

    DU --> HTWB
    FX --> HTIU
    FSH --> HTD
    FNEIG --> HTD

    HTD --> HT
    HTIU --> HT
    HTWB --> HT
    HT --> |x Delta T| Q_HT["Q_HT\nTotal Transmission Heat Loss"]
    TEMP --> Q_HT
```

---

This section breaks down how the physics logic is structurally implemented in the `transmission` module inside `lib.rs`.

### The Fundamental Data Structures calculates its U-value

```rust
pub struct BuildingComponent {
    pub name: String,
    pub layers: Vec<Layer>,
    pub r_si: f64,
    pub r_se: f64,
    pub area: f64,
    pub f_neig: f64,
    pub f_x: f64,
}
```

- **Purpose:** Represents an opaque wall, roof, or floor.
- **How it works:** It natively calculates its U-value using the standard thermal resistance formula:
  $U = \frac{1}{R_T}$ where $R_T = R_{si} + \sum R_{layers} + R_{se}$

  The thermal resistance of each layer is calculated based on its thickness ($d$, in meters) and thermal conductivity ($\lambda$, in $W/(m \cdot K)$):
  $R_{layer} = \frac{d}{\lambda}$

  **Surface Resistances ($R_{si}$ and $R_{se}$):**
  DIN EN ISO 6946 dictates these constants. They almost never change unless the heat flow direction changes:

  - **External Walls (Horizontal heat flow):** $R_{si} = 0.13$, $R_{se} = 0.04$
  - **Roofs (Upward heat flow):** $R_{si} = 0.10$, $R_{se} = 0.04$
  - **Floors (Downward heat flow):** $R_{si} = 0.17$, $R_{se} = 0.04$

  _Note:_ Since we fetch exact U-values from TABULA, the engine bridges the gap by creating a "Dummy Layer" whose thickness mimics the exact TABULA thermal resistance ($R_{total} = 1.0 / U_{val}$), completely bypassing complex layer definitions for the user while preserving the DIN calculation structure.

  **Custom Insulation Overrides:**
  If the user chooses to define custom insulation for a component (adjusting only the material and thickness), the engine calculates the new U-value by fetching the "Existing State" (001 scenario) U-value from TABULA as the baseline, and then adds the thermal resistance of the custom insulation layer:
  $U_{new} = \frac{1}{\frac{1}{U_{existing}} + \frac{d}{\lambda}}$

  The UI provides four standard materials with their typical $\lambda$ values:

  1. **EPS (Expanded Polystyrene)**
     - **Use Case in TABULA:** The absolute standard for external wall insulation (ETICS/WDVS) in "Standard Refurbishment" scenarios.
     - **Standard Lambda ($\lambda$):** 0.035 - 0.040 W/(m·K)
  2. **Mineral Wool (Glass Wool / Rock Wool)**
     - **Use Case in TABULA:** The default choice for pitched roofs (between rafters), attic floors, and ventilated facades due to fire safety and acoustic properties.
     - **Standard Lambda ($\lambda$):** 0.035 - 0.040 W/(m·K)
  3. **XPS (Extruded Polystyrene)**
     - **Use Case in TABULA:** Used exclusively for ground-contact elements (perimeter insulation for basements, floor slabs) because it is moisture and pressure resistant.
     - **Standard Lambda ($\lambda$):** 0.035 W/(m·K)
  4. **PUR / PIR (Polyurethane / Polyisocyanurate Rigid Foam)**
     - **Use Case in TABULA:** Used in "Advanced Refurbishment" scenarios where high insulation is required but space is very limited (e.g., slim flat roofs or high-efficiency cavity walls).
     - **Standard Lambda ($\lambda$):** 0.022 - 0.028 W/(m·K)

```rust
pub struct WindowComponent {
    pub area: f64,
    pub u_w: f64,
    pub u_w_sh: f64,
    pub f_sh: f64,
    pub f_neig: f64,
    pub f_x: f64,
}
```

- **Purpose:** Represents transparent surfaces.
- **How it works:** It features an internal method `calculate_u_w_eff()` which dynamically blends the base U-value (`u_w`) with the shutter-closed U-value (`u_w_sh`) proportionally based on how often the user says shutters are used (`f_sh`).

### The Calculation Engine Functions

The core logic is broken into pure, side-effect-free functions that accept lists of these components:

1. **`calculate_h_t_d`**: Iterates through all `BuildingComponent`s and `WindowComponent`s. It filters exclusively for components that touch outside air directly (`f_x == 1.0`), multiplies their Area × U-Value × Inclination Factor (`f_neig`), and sums them up.
2. **`calculate_h_t_iu`**: Iterates through `BuildingComponent`s. It filters for components that touch unheated spaces or ground (`f_x < 1.0`), and multiplies their Area × U-Value × the specific Temperature Correction Factor (`f_x`) determined by the UI ground-contact question.
3. **`calculate_h_t_wb_simplified`**: A straightforward function that takes the flat penalty parameter (`delta_u_wb` selected by the UI thermal bridge question) and multiplies it by the absolute sum of all envelope areas.

### Engine Integration (`calculate_energy` inside `lib.rs`)

Inside the main `calculate_energy` loop, the program uses Rust's `match` blocks to convert the raw String parameters from the UI (`state.params`) directly into the strongly-typed Enums:

```rust
let f_x_ground = match state.params.ground_contact_type.as_str() {
    "Floor Slab On Ground" => 0.5,
    "Heated Basement" => 1.0,
    "Ventilated Crawl Space" => 0.8,
    // ...
```

````
Beyond geometry mapping, the engine dynamically calculates the temperature properties:
```rust
    // Calculate external temperature (theta_e) by averaging the heating months.
    let temps = region.monthly_temperatures();
    let heating_months_sum = temps[0] + temps[1] + temps[2] + temps[3] + temps[9] + temps[10] + temps[11];
    let theta_e = heating_months_sum / 7.0;

    // Calculate internal setpoint (theta_int)
    let base_temp = profile.heating_setpoint();
    let temp_shift = automation.temperature_shift(profile);
    let theta_int = f64::max(10.0, base_temp + temp_shift);
````

Once all values are mapped and calculated, it feeds the component arrays into the three `calculate_h_t` functions, sums them into `h_tr`, and applies the custom `theta_int - theta_e` delta to calculate the final `q_ht_tr` energy loss.

---

#### Simplified UI Data Mapping

By asking chronological and physical questions, the user's brain can "walk through" their building, keeping the frontend conversational while the backend remains strictly DIN-compliant.

Building age and renovation history are the strongest predictors of building physics. A building from 1960 that hasn't been renovated will almost always fall into Tightness Category IV (obvious leaks) and have specific default U-values (which we fetch from TABULA).
We manage complex transmission and insulation values by asking simple questions like "When was the building built?" and "When were the windows and roof last replaced or heavily renovated?".

## 6.2 Ventilation Heat Transfer ($H_V$)

This document explains the physics and logic used in the Rust calculation engine (`lib.rs` / `ventilation` module) to compute the building's ventilation heat transfer coefficient ($H_V$) and resulting energy demand ($Q_V$) according to DIN/TS 18599-2:2025-10, DIN V 18599-10, and DIN 4108-4.

### 1. Inputs & Boundary Conditions

To perform this highly detailed calculation, the UI needs geometry, system, and behavioral parameters.

**A. Automated Geometry (Calculated by the Sketchpad UI)**

* `a_ngf`: Net floor area / Nettogrundfläche ($A_{NGF}$ in $m^2$).
* `h_room`: Average clear room height ($h_R$ in $m$).
* `v_net`: Net air volume ($V$ in $m^3$).
* `a_e`: Envelope Area ($A_E$ in $m^2$). Required if $V > 1500 m^3$.

**B. User Questions (Parameters)**

| Parameter Key       | UI Question / Prompt                               | Available Options                                         | How Rust uses it                                                                           |
| ------------------- | -------------------------------------------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `building_type`   | **"What is the primary usage?"**             | Residential vs. Non-Residential                           | Triggers seasonal window adjustment ($f_{win,seasonal}$) per Eq. 76 & 79.                |
| `air_tightness`   | **"How airtight is the building envelope?"** | Category I, II, III, IV                                   | Maps to Table 8 (DIN 18599-2) to determine default$n_{50}$ or $q_{50}$.                |
| `has_atd`         | **"Are Air Transfer Devices installed?"**    | Yes / No                                                  | Calculates$f_{ATD}$ (Eq. 69), increasing natural infiltration.                           |
| `mech_system`     | **"Mechanical supply and exhaust?"**         | $\dot{V}_{mech,b}$ and $\dot{V}_{ETA}$ (in $m^3/h$) | Used in Eq. 97 & 99 and defines if windows must be opened to balance pressure (Eq. 87-90). |
| `heat_recovery`   | **"Heat recovery efficiency?"**              | $\eta_t$ value (0.0 to 1.0)                             | Used to calculate the supply air temp$	heta_{V,mech}$ (Eq. 100).                         |
| `operating_times` | **"Daily usage and system hours?"**          | Sliders:$t_{nutz}$ and $t_{V,mech}$                   | Time-weighting for Window Airing (Eq. 80-84) and Mech Rate (Eq. 95).                       |

### 2. Core Physics Calculations (DIN 18599 Formulas)

The standard requires resolving the baseline air requirements before calculating how much heat is lost through infiltration, windows, and mechanical systems.

**1. Required Fresh Air ($n_{nutz}$)**
The absolute minimum air required is based on the floor area and the specific usage profile (DIN 18599-10).
$n_{nutz} = \frac{\dot{V}_A \cdot A_{NGF}}{V}$ (Eq. 91)

**2. Mechanical Ventilation ($H_{V,mech}$ and $\theta_{V,mech}$)**

* **Supply Air Rate:** $n_{mech,SUP} = \frac{\dot{V}_{mech,b}}{V}$ (Eq. 97)
* **Exhaust Air Rate:** $n_{mech,ETA} = \frac{\dot{V}_{ETA}}{V}$ (Eq. 99)
* **Daily Average Rate:** $n_{mech} = n_{mech,SUP} \cdot \frac{t_{V,mech}}{24h}$ (Eq. 95)
* **Heat Transfer Coefficient:** $H_{V,mech} = n_{mech} \cdot V \cdot c_{p,a} \cdot \rho_a$ (Eq. 94)

**Supply Temperature ($\theta_{V,mech}$):**
$\theta_{V,mech} = \theta_e + \eta_t \cdot (\theta_i - \theta_e)$ (Eq. 100)

**Energy Balance (Mechanical):**

* **Heat Sink:** $Q_{V,mech} = H_{V,mech} \cdot (\theta_i - \theta_{V,mech}) \cdot t$ (Eq. 92)
* **Heat Source:** $Q_{V,mech} = H_{V,mech} \cdot (\theta_{V,mech} - \theta_i) \cdot t$ (Eq. 93)

**3. Window Airing Deficit ($\Delta n_{win}$)**
Calculates window airing by figuring out how much fresh air is *required* ($n_{nutz}$) versus how much is *already provided* by infiltration ($n_{inf}$) and mechanical systems ($n_{SUP}$). Occupants open windows to make up the deficit.

* **Without Mech Vent:** $\Delta n_{win} = \max[0; n_{nutz} - (n_{nutz} - 0.2) \cdot n_{inf} - 0.1]$ (Eq. 81)
* **With Mech Vent:** Calculates a deficit depending on if the system pushes enough air ($n_{SUP}$) and if the system causes pressure imbalances ($n_{ETA} > n_{SUP}$).
* **Heat Transfer Coefficient:** $H_{V,win} = n_{win} \cdot V \cdot c_{p,a} \cdot \rho_a$ (Eq. 75)

**4. Infiltration ($H_{V,inf}$)**
Calculates unintentional leaks. Modified by the mechanical system's pressure ($f_e$).

* **Heat Transfer Coefficient:** $H_{V,inf} = n_{inf} \cdot V \cdot c_{p,a} \cdot \rho_a$ (Eq. 65)

**5. Unheated Zones ($H_{V,ue}$)**
Exchange rate of adjacent unheated zones to the outside:
$H_{V,ue} = c_{p,a} \cdot \rho_a \cdot n_{ue} \cdot V_u$ (Eq. 103)

### 3. Reference Tables & Standards

To execute the logic, the engine relies on strict fallback parameters provided by DIN 18599 and DIN 4108-4. If exact values are not provided by the user, these defaults must be applied.

**3.1 Global Constants (DIN/TS 18599-2)**

* **Heat Capacity of Air ($c_{p,a} \cdot \rho_a$):** `0.34 Wh/(m³·K)`
* **Volume Flow Coefficient ($e$):** `0.07` (Standard default for building wind exposure).
* **Wind Exposure Coefficient ($f$):** `15.0` (Standard default used in Eq. 72).
* **Unheated Zone Infiltration ($n_{ue}$):** `0.6 h^-1` (Used for standard sunspaces/attics in Eq. 105).

**3.2 Building Air Tightness ($n_{50}$ and $q_{50}$)**
From **DIN 18599-2, Table 8**. Used if no measured Blower-Door test is available.

| Tightness Category                                                                                                | Description             | $n_{50}$ (for $V \le 1500 m^3$) | $q_{50}$ (for $V > 1500 m^3$) |
| ----------------------------------------------------------------------------------------------------------------- | ----------------------- | ----------------------------------- | --------------------------------- |
| **Kategorie I**                                                                                             | Tested to DIN 4108-7    | *Use Measured Value*              | *Use Measured Value*            |
| **Kategorie II**                                                                                            | New buildings, untested | `4.0`                             | `6.0`                           |
| **Kategorie III**                                                                                           | General existing stock  | `6.0`                             | `9.0`                           |
| **Kategorie IV**                                                                                            | Obvious leaks/gaps      | `10.0`                            | `15.0`                          |
| *(Note: If $V > 1500 m^3$, $q_{50}$ must be converted via Eq. 70: $n_{50} = \frac{q_{50} \cdot A_E}{V}$)* |                         |                                     |                                   |

**3.3 Component Air Permeability (DIN 4108-4, Table 8)**
Extracted from **DIN 4108-4:2020-11, Table 8**. Evaluates the leakiness of specific facade components based on construction.

| Construction Details                                                  | Air Permeability Class (DIN EN 12207) |
| --------------------------------------------------------------------- | ------------------------------------- |
| Wooden windows (incl. double windows)*without* seals                | **Class 2**                     |
| All window constructions with age-resistant, replaceable seals        | **Class 3**                     |
| All exterior door constructions with age-resistant, replaceable seals | **Class 2**                     |

**3.4 Minimum Window Airing ($n_{win,min}$)**
Even if mechanical systems are off, a minimum limit of window-opening is assumed.

* **Residential Buildings:** `0.10 h^-1` (Subject to seasonal modification $f_{win,seasonal}$).
* **Non-Residential:** $\min(0.1, 0.1 \cdot \frac{3}{h_R})$ where $h_R$ is clear room height.

**3.5 Usage Profiles & Fresh Air Demand ($\dot{V}_A$)**
From **DIN 18599-10**. Determines $n_{nutz}$ per Eq. 91. Also dictates default operating times ($t_{nutz}$ and $t_{V,mech}$).

| Usage Profile                      | Minimum Fresh Air$\dot{V}_A$ $[m^3/(h \cdot m^2)]$ | Typical Operating Hours ($t_{nutz}$ & $t_{V,mech}$) |
| ---------------------------------- | ------------------------------------------------------ | ------------------------------------------------------- |
| Residential (Wohngebäude)         | Default$n_{nutz} = 0.5 	ext{ h}^{-1}$                | `24 h/d`                                              |
| Single Office / Small Group Office | `2.0`                                                | `13 h/d`                                              |
| Open Plan Office / Retail          | `3.0`                                                | `12-13 h/d`                                           |
| Classroom / Meeting Room           | `5.0` to `6.0`                                     | `12-13 h/d`                                           |
| Restaurant                         | `7.0`                                                | `14 h/d`                                              |
| Hospital Rooms                     | `3.0`                                                | `24 h/d`                                              |
| Warehouse / Logistics              | `0.5`                                                | `14 h/d`                                              |

**3.6 Mechanical Exhaust Default Values ($n_{mech,ETA}$)**

* **Balanced Systems:** If unspecified, assume $n_{mech,ETA} = n_{mech,SUP}$.
* **Pure Exhaust Systems (Abluftanlagen):** $n_{mech,SUP} = 0$. The exhaust rate $n_{mech,ETA}$ defaults to the required fresh air $n_{nutz}$. The deficit is fully covered by window airing and infiltration.

**3.7 Heat Recovery Efficiency Defaults ($\eta_t$)**

| System Quality                  | Heat Recovery Efficiency ($\eta_t$) | Effect on Supply Temperature   |
| ------------------------------- | ------------------------------------- | ------------------------------ |
| Pure Exhaust System (No supply) | `0.00` (0%)                         | $\theta_{V,mech} = \theta_e$ |
| Standard / Older Systems        | `0.60` (60%)                        | Recovers 60% of delta T        |
| Modern High-Efficiency Systems  | `0.80` (80%)                        | Recovers 80% of delta T        |
| Premium / Passive House Systems | `0.90` (90%)                        | Recovers 90% of delta T        |

### 4. Rust Engine Implementation

The ventilation engine calculates how much heat is lost (or gained) when indoor air is replaced by outdoor air. It combines the building's internal air volume, the air-tightness of the envelope, mechanical air exchange rates, supply air temperatures (heat recovery), and a highly dynamic model for occupant window-opening behavior.

#### Data Flow & Architecture

```mermaid
flowchart TD
    %% UI Inputs
    subgraph UI ["User Interface (Inputs)"]
        G["Geometry Engine
(A_NGF, V)"]
        Q1[Air Tightness / n50]
        Q3[Mech. Air Flow
(V_mech_b, V_ETA)]
        Q4[Usage Profile
(V_A, t_nutz, t_V,mech)]
        Q5[Heat Recovery
(eta_t)]
    end

    %% Rust Engine Middle Layer
    subgraph PreCalc ["Preprocessing (Rust)"]
        NNUTZ["n_nutz
Required Air (Eq. 91)"]
        NINF["n_inf
Infiltration Rate (Eq. 66/67)"]
        NMECH["n_mech
Daily Mech Rate (Eq. 95-99)"]
        TVMECH["theta_V,mech
Supply Temp (Eq. 100)"]
        NWIN["n_win
Daily Window Airing (Eq. 80-90)"]
    end

    %% Detailed Calculations
    subgraph Calc ["Ventilation Heat Transfer Coefficients (H_V)"]
        HVINF["H_V,inf
(Infiltration Eq. 65)"]
        HVWIN["H_V,win
(Window Airing Eq. 75)"]
        HVMECH["H_V,mech
(Mechanical Eq. 94)"]
        HVUE["H_V,ue
(Unheated Zones Eq. 103)"]
    end

    %% Final Output
    HV["Q_V Total
Sum of Energy Balances"]

    G --> NNUTZ
    Q4 --> NNUTZ
  
    Q1 --> NINF
  
    Q3 --> NMECH
    G --> NMECH
    Q4 --> NMECH
  
    Q5 --> TVMECH
  
    NNUTZ --> NWIN
    NINF --> NWIN
    Q3 --> NWIN
  
    NINF --> HVINF
    NWIN --> HVWIN
    NMECH --> HVMECH
  
    HVINF --> |x (theta_i - theta_e)| HV
    HVWIN --> |x (theta_i - theta_e)| HV
    HVMECH --> |x (theta_i - theta_V,mech)| HV
```

## Total Heat Sinks / Heat Demand ($Q_{sink}$)

This section calculates the total building heat demand by combining the transmission and ventilation losses.
$Q_{sink} = Q_{T,sink} + Q_{V,sink}$

# PART B: Heat Sources (Gains)

## 6.4 Solar Heat Gains ($Q_S$)

### 1. Inputs & Boundary Conditions

To accurately calculate solar gains, the UI must gather geometry and component properties mapped to their cardinal direction.

### A. Window Parameters (Transparent)

| UI Prompt                       | Determines                          | DIN Variable                             |
| ------------------------------- | ----------------------------------- | ---------------------------------------- |
| **"Window Area"**         | Total size of the opening ($m^2$) | $A_w$                                  |
| **"Orientation & Tilt"**  | Dictates the sun exposure           | $I_s$                                  |
| **"Glazing Type"**        | Double, Triple, Solar Control       | $g$-value (Total Energy Transmittance) |
| **"Frame Material/Type"** | Thick vs. Thin frames               | $F_F$(Frame Fraction)                  |
| **"Shading Devices"**     | Exterior Blinds, Interior Curtains  | $F_C$(Shading Reduction Factor)        |
| **"Surrounding Shadows"** | Overhangs, Neighboring Buildings    | $F_S$(Surroundings Shading Factor)     |

### B. Wall/Roof Parameters (Opaque)

| UI Prompt                  | Determines                                 | DIN Variable                  |
| -------------------------- | ------------------------------------------ | ----------------------------- |
| **"Wall/Roof Area"** | Total exposed surface ($m^2$)            | $A_{op}$                    |
| **"Surface Color"**  | Light (White), Medium (Brick), Dark (Grey) | $\alpha$(Solar Absorptance) |

### 2. Core Physics Calculations (DIN 18599 Formulas)

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

### 3. Reference Tables & Standards

To execute the logic, the engine relies on strict fallback parameters provided by DIN 18599.

### Table A: Standard Glazing $g$-Values

If exact manufacturer data is missing, DIN defaults apply:

| Glazing Type                     | $g$-value                        |
| -------------------------------- | ---------------------------------- |
| Single Glazing                   | `0.85`                           |
| Double Glazing (Standard)        | `0.75`                           |
| Double Glazing (Low-E / Thermal) | `0.60`                           |
| Triple Glazing (Low-E / Thermal) | `0.50`                           |
| Solar Control Glass              | `0.35` (Fixed in implementation) |

### Table B: Frame Fraction ($F_F$)

The percentage of the window area that is opaque frame.

| Window Description                              | Frame Fraction ($F_F$)       |
| ----------------------------------------------- | ------------------------------ |
| Standard Window                                 | `0.30`(30% Frame, 70% Glass) |
| Very Large Windows / Glass Facades              | `0.20`                       |
| Small Windows / Divided panes (Sprossenfenster) | `0.40`                       |

### Table C: Shading Devices ($F_C$)

Operable shading heavily reduces solar penetration.

| Shading Device                         | Reduction Factor ($F_C$) |
| -------------------------------------- | -------------------------- |
| No Shading                             | `1.00`                   |
| Interior Curtains (White / Light)      | `0.80`                   |
| Interior Curtains (Dark)               | `0.60`                   |
| Exterior Blinds / Shutters (Rollladen) | `0.25`                   |
| Exterior Awnings (Markisen)            | `0.40`                   |

### Table D: Opaque Absorptance ($\alpha$)

How much sun the exterior walls/roof absorb based on color.

| Surface Color                      | Solar Absorptance ($\alpha$) |
| ---------------------------------- | ------------------------------ |
| Light (White, very light grey)     | `0.30`                       |
| Medium (Red brick, concrete, wood) | `0.60`                       |
| Dark (Dark grey, black roof tiles) | `0.90`                       |

### 4. Rust Engine Implementation

#### Data Flow & Architecture

The solar gains engine calculates how much solar energy enters the building. This is split into two distinct physical phenomena:

1. **Transparent Gains (** $Q_{s,w}$**):** Solar radiation passing directly through windows to heat the interior. This is heavily reduced by frames, dirty glass, angle of incidence, and shading devices.
2. **Opaque Gains & Sky Losses (** $Q_{s,op}$**):** Solar radiation warming the exterior walls/roofs, heavily offset by thermal radiation emitted from the building out into the cold night sky.

```mermaid
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

This Rust architecture encapsulates the exact physics required by the standard, safely isolating transparent and opaque behaviors.

```rust
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

## 6.5 Internal Heat Gains ($Q_I$)

### 1. Inputs & Boundary Conditions

### A. Automated Geometry

* `a_ngf`: Net floor area / Nettogrundfläche ($A_{NGF}$ in $m^2$).

### B. User Questions (Parameters)

| Mode                         | UI Prompts                                                                      | Available Options                                                          | How Rust uses it                                                                                 |
| ---------------------------- | ------------------------------------------------------------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| **Standard**           | "What is the primary usage?"                                                    | Residential, Office, Retail, etc.                                          | Fetches$q_{I,p}$,$q_{I,app}$, and$q_{I,sink,app}$from DIN 18599-10.                        |
| **Standard**           | "What kind of lighting fixtures are installed?"                                 | - Standard (No Exhaust)- Exhaust via Ceiling Cavity- Exhaust via Air Ducts | Maps to the$\mu_l$room load factor (1.0, 0.75, or 0.65) to reduce lighting heat gain.          |
| **Material Transport** | "Are large amounts of cold or hot materials regularly brought into this space?" | Type (e.g., Frozen Goods, Metal) & Volume (None / Pallets / Truckloads)    | Infers mass flow ($\dot{m}$) and $\Delta T$ based on typical material properties and volume. |
| **Detailed**           | "How many people are usually in this room?"                                     | Number input                                                               | Multiplies count by standard metabolic heat (e.g., 80W).                                         |
| **Detailed**           | "Add electrical equipment"                                                      | Add item (Name, Watts, Duty Cycle)                                         | Multiplies item count by wattage and duty-cycle.                                                 |

### 2. Core Physics Calculations (DIN 18599 Formulas)

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

### 3. Reference Tables & Standards

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

| Lighting Exhaust Type                       | Room Load Factor$\mu_l$          |
| ------------------------------------------- | ---------------------------------- |
| Standard Luminaires (No Exhaust)            | `1.0`                            |
| Abluftleuchten (Exhaust via Ceiling Cavity) | `0.75` (Fixed in implementation) |
| Abluftleuchten (Exhaust via Air Ducts)      | `0.65` (Fixed in implementation) |

### 4. Rust Engine Implementation

#### Data Flow & Architecture

The internal gains engine calculates the "free" heat generated inside the building (sources) and the heat actively removed by specific equipment (sinks). The engine supports two parallel pathways:

1. **The Standardized Method (DIN 18599-2 / 6.5):** Uses generic flat rates ($W/m^2$) based on usage profiles, handles material transport mass flows, and adjusts for exhaust-air lighting systems. Required for official Energy Certificates.
2. **The Detailed Custom Method:** Allows users to define the exact number of people and specific electrical equipment in a room for precise load sizing.

```mermaid
flowchart TD
    %% UI Inputs
    subgraph UI ["User Interface (Inputs)"]
        G["Geometry Engine\n(A_NGF)"]
        Q1["Usage Profile\n(Standard Method)"]
        Q2["Custom Inventory\n(Detailed Method)"]
        Q3["Material Transport\n(Mass Flow, Temp)"]
        Q4["Lighting System\n(Standard vs Exhaust)"]
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

This Rust architecture supports the strict separation required by DIN 18599-2, handling both sources, sinks, material flows, and explicitly categorized exhaust lighting.

```rust
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

#### Simplified UI Data Mapping

By asking chronological and physical questions, the user's brain can "walk through" their building, keeping the frontend conversational while the backend remains strictly DIN-compliant.

**Usage Profile & Automation Class**
Instead of asking for a specific DIN 18599-10 profile directly, we ask:
*   "What is the primary use of this space?" -> This maps to the usage profile (e.g., Residential, Office). This unlocks heating setpoints, usage hours, and internal heat gains.
*   "How do you control the heating in this space?" -> Maps to the Automation Class. For example, "Manual radiator knobs" translates to Class C, while "Smart Home system" translates to Class A or B, lowering the heating setpoint automatically in the engine.
```

## Total Heat Sources / Final Energy Balance ($Q_h$)

This section calculates the total building heat sources (gains) and applies the ISO 13790 solver (time constant $\tau$, gain utilization factor $\eta$) to find the final heating demand $Q_h$.
$Q_{source} = Q_S + Q_I$

*Note: The `energy_balance` solver is fully implemented in `lib.rs` but will be expanded with detailed documentation in future updates.*\n

# PART C: Electrical Energy Demand

## 6.3 Lighting Energy Calculation Engine ($Q_{l,f}$)

This document explains the physics and logic used in the Rust calculation engine (`lib.rs` / `lighting` module) to compute the Final Energy Demand for Lighting ($Q_{l,f}$) according to DIN V 18599-4:2018-09.

### 1. Inputs & Boundary Conditions

To perform the calculation, the system extracts data across several architectural categories. It calculates the electrical energy needed to maintain a required brightness ($\bar{E}_m$) in a room. It splits the room into Daylight Areas ($A_{TL}$) near the windows, and Non-Daylight Areas ($A_{KTL}$) deep inside the room. It then applies reduction factors based on daylight sensors, motion detectors, and window orientation.

- **Geometry**: Floor Area, Window Area, Depth.
- **Lamp & Luminaire**: Lamp Technology, Luminaire Type.
- **Sensors**: Daylight Control System, Motion Detectors.
- **Usage Profile**: Required Lux, Operating Hours.
- **Window Blinds**: Sun Protection configurations.

### 2. Core Physics Calculations (DIN 18599-4 Formulas)

**2.1 The Master Equation (Eq. 2)**
The final energy demand for a room ($Q_{l,f}$) is the installed electrical power ($p_j$) multiplied by the area and the effective operating hours.

$$
Q_{l,f} = p_j \cdot [A_{TL} \cdot (t_{eff,Tag,TL} + t_{eff,Nacht}) + A_{KTL} \cdot (t_{eff,Tag,KTL} + t_{eff,Nacht})]
$$

**2.2 Effective Operating Times (Eq. 4, 5, 6)**
The raw usage hours from DIN 18599-10 ($t_{Tag}$ and $t_{Nacht}$) are reduced by smart systems:
Daylight Zone (Day): $t_{eff,Tag,TL} = t_{Tag} \cdot F_{TL} \cdot F_{Prä} \cdot F_{KL}$
Non-Daylight Zone (Day): $t_{eff,Tag,KTL} = t_{Tag} \cdot F_{Prä} \cdot F_{KL}$
All Zones (Night): $t_{eff,Nacht} = t_{Nacht} \cdot F_{Prä} \cdot F_{KL}$

**2.3 Installed Electrical Power ($p_j$) (Eq. 11)**
If a detailed lighting plan doesn't exist, the standard estimates the installed Watts per square meter based on the required Lux and lamp technology.

$$
p_j = p_{j,lx} \cdot \bar{E}_m \cdot k_{WF} \cdot k_A \cdot k_L \cdot k_{VB}
$$

**2.4 Presence / Motion Detection ($F_{Prä}$) (Eq. 40)**

$$
F_{Prä} = 1 - C_A \cdot c_{Prä,kon}
$$

$C_A$: Relative absence rate of the room.
$c_{Prä,kon}$: Detector efficiency (Table 28: 0.5 without detector, 0.95 with detector).

**2.5 Daylight Supply Factor ($F_{TL}$) (Eq. 19 & 38)**
If daylight enters the room, the artificial lighting can be dimmed/turned off.

$$
F_{TL} = 1 - C_{TL,Vers} \cdot C_{TL,kon}
$$

Where the total daylight provision $C_{TL,Vers}$ blends the time the blinds are open vs. closed.

### 3. Reference Tables & Standards

**Table 5: Base Power per Lux ($p_{j,lx}$)**
Measured in $W/(m^2 \cdot lx)$. Depends on the Room Index $k = \frac{a \cdot b}{h \cdot (a+b)}$.

| Lighting Type             | k=0.6 | k=1.0 | k=2.0 | k=5.0 |
| ------------------------- | ----- | ----- | ----- | ----- |
| Direct (Ceiling spots)    | 0.045 | 0.033 | 0.025 | 0.021 |
| Direct/Indirect (Pendant) | 0.067 | 0.045 | 0.032 | 0.025 |
| Indirect (Wall washers)   | 0.122 | 0.071 | 0.044 | 0.033 |

**Table 6: Lamp Adjustment Factor ($k_L$)**
Converts the base value to the actual installed lamp.

| Lamp Technology                 | $k_L$ Factor |
| ------------------------------- | -------------- |
| Incandescent Bulb (Glühlampe)  | 6.0            |
| Halogen                         | 5.0            |
| Fluorescent Tube (EVG) - BASE   | 1.0            |
| Compact Fluorescent (CFL)       | 1.2 to 1.6     |
| LED Replacement Bulb (Retrofit) | 0.68           |
| Dedicated LED Luminaire         | 0.44 to 0.49   |

**Table 9 & Eq. 30: Daylight Classification**
The room's raw daylight is determined by the window size vs. room depth.
$D_{Rb} = \max(4.13 + 20.0 \cdot \frac{A_{window}}{A_{floor}} - 1.36 \cdot \frac{Depth}{Height_{window}}) \cdot 0.7$

- $D_{Rb} \ge 6\%$: Good (Gut)
- $4\% \le D_{Rb} < 6\%$: Medium (Mittel)
- $2\% \le D_{Rb} < 4\%$: Low (Gering)
- $D_{Rb} < 2\%$: None (Keine)

**Table 25: Daylight Control System Efficiency ($C_{TL,kon}$)**
How well the electrical system responds to daylight (Excerpt for 500 Lux, "Medium" Daylight class).

| Control Type               | $C_{TL,kon}$ |
| -------------------------- | -------------- |
| Manual Switch              | 0.44           |
| Dimmed, Auto-On/Off        | 0.75           |
| Dimmed, Manual-On/Auto-Off | 0.77           |

**Table 28: Presence Detector Efficiency ($c_{Prä,kon}$)**

| Detector Type                          | $c_{Prä,kon}$ |
| -------------------------------------- | ---------------- |
| No Presence Detector (Manual)          | 0.50             |
| With Presence Detector (Motion Sensor) | 0.95             |

\n
# PART C: Electrical Energy Demand

## 6.3 Lighting Energy Calculation Engine ($Q_{l,f}$)

This document explains the physics and logic used in the Rust calculation engine (`lib.rs` / `lighting` module) to compute the Final Energy Demand for Lighting ($Q_{l,f}$) according to DIN V 18599-4:2018-09.

### 1. Inputs & Boundary Conditions

To perform the calculation, the system extracts data across several architectural categories. It calculates the electrical energy needed to maintain a required brightness ($\bar{E}_m$) in a room. It splits the room into Daylight Areas ($A_{TL}$) near the windows, and Non-Daylight Areas ($A_{KTL}$) deep inside the room. It then applies reduction factors based on daylight sensors, motion detectors, and window orientation.

- **Geometry**: Floor Area, Window Area, Depth.
- **Lamp & Luminaire**: Lamp Technology, Luminaire Type.
- **Sensors**: Daylight Control System, Motion Detectors.
- **Usage Profile**: Required Lux, Operating Hours.
- **Window Blinds**: Sun Protection configurations.

### 2. Core Physics Calculations (DIN 18599-4 Formulas)

**2.1 The Master Equation (Eq. 2)**
The final energy demand for a room ($Q_{l,f}$) is the installed electrical power ($p_j$) multiplied by the area and the effective operating hours.
$$Q_{l,f} = p_j \cdot [A_{TL} \cdot (t_{eff,Tag,TL} + t_{eff,Nacht}) + A_{KTL} \cdot (t_{eff,Tag,KTL} + t_{eff,Nacht})]$$

**2.2 Effective Operating Times (Eq. 4, 5, 6)**
The raw usage hours from DIN 18599-10 ($t_{Tag}$ and $t_{Nacht}$) are reduced by smart systems:
Daylight Zone (Day): $t_{eff,Tag,TL} = t_{Tag} \cdot F_{TL} \cdot F_{Prä} \cdot F_{KL}$
Non-Daylight Zone (Day): $t_{eff,Tag,KTL} = t_{Tag} \cdot F_{Prä} \cdot F_{KL}$
All Zones (Night): $t_{eff,Nacht} = t_{Nacht} \cdot F_{Prä} \cdot F_{KL}$

**2.3 Installed Electrical Power ($p_j$) (Eq. 11)**
If a detailed lighting plan doesn't exist, the standard estimates the installed Watts per square meter based on the required Lux and lamp technology.
$$p_j = p_{j,lx} \cdot \bar{E}_m \cdot k_{WF} \cdot k_A \cdot k_L \cdot k_{VB}$$

**2.4 Presence / Motion Detection ($F_{Prä}$) (Eq. 40)**
$$F_{Prä} = 1 - C_A \cdot c_{Prä,kon}$$
$C_A$: Relative absence rate of the room.
$c_{Prä,kon}$: Detector efficiency (Table 28: 0.5 without detector, 0.95 with detector).

**2.5 Daylight Supply Factor ($F_{TL}$) (Eq. 19 & 38)**
If daylight enters the room, the artificial lighting can be dimmed/turned off.
$$F_{TL} = 1 - C_{TL,Vers} \cdot C_{TL,kon}$$
Where the total daylight provision $C_{TL,Vers}$ blends the time the blinds are open vs. closed.

### 3. Reference Tables & Standards

**Table 5: Base Power per Lux ($p_{j,lx}$)**
Measured in $W/(m^2 \cdot lx)$. Depends on the Room Index $k = \frac{a \cdot b}{h \cdot (a+b)}$.

| Lighting Type | k=0.6 | k=1.0 | k=2.0 | k=5.0 |
|---|---|---|---|---|
| Direct (Ceiling spots) | 0.045 | 0.033 | 0.025 | 0.021 |
| Direct/Indirect (Pendant) | 0.067 | 0.045 | 0.032 | 0.025 |
| Indirect (Wall washers) | 0.122 | 0.071 | 0.044 | 0.033 |

**Table 6: Lamp Adjustment Factor ($k_L$)**
Converts the base value to the actual installed lamp.

| Lamp Technology | $k_L$ Factor |
|---|---|
| Incandescent Bulb (Glühlampe) | 6.0 |
| Halogen | 5.0 |
| Fluorescent Tube (EVG) - BASE | 1.0 |
| Compact Fluorescent (CFL) | 1.2 to 1.6 |
| LED Replacement Bulb (Retrofit) | 0.68 |
| Dedicated LED Luminaire | 0.44 to 0.49 |

**Table 9 & Eq. 30: Daylight Classification**
The room's raw daylight is determined by the window size vs. room depth.
$D_{Rb} = \max(4.13 + 20.0 \cdot \frac{A_{window}}{A_{floor}} - 1.36 \cdot \frac{Depth}{Height_{window}}) \cdot 0.7$
- $D_{Rb} \ge 6\%$: Good (Gut)
- $4\% \le D_{Rb} < 6\%$: Medium (Mittel)
- $2\% \le D_{Rb} < 4\%$: Low (Gering)
- $D_{Rb} < 2\%$: None (Keine)

**Table 25: Daylight Control System Efficiency ($C_{TL,kon}$)**
How well the electrical system responds to daylight (Excerpt for 500 Lux, "Medium" Daylight class).

| Control Type | $C_{TL,kon}$ |
|---|---|
| Manual Switch | 0.44 |
| Dimmed, Auto-On/Off | 0.75 |
| Dimmed, Manual-On/Auto-Off | 0.77 |

**Table 28: Presence Detector Efficiency ($c_{Prä,kon}$)**

| Detector Type | $c_{Prä,kon}$ |
|---|---|
| No Presence Detector (Manual) | 0.50 |
| With Presence Detector (Motion Sensor) | 0.95 |

### 4. Rust Engine Implementation

This Rust architecture encapsulates the DIN V 18599-4 logic, strictly separating Daylight Zones from Non-Daylight Zones, and perfectly applying lamp technologies and smart automation logic.

#### Data Flow & Architecture
```mermaid
flowchart TD
    %% UI Inputs
    subgraph UI ["User Interface (Inputs)"]
        G["Geometry
(Floor Area, Window Area, Depth)"]
        Q1[Lamp Type & Luminaire Type]
        Q2[Sensors
(Daylight Control, Motion Detectors)]
        Q3[Usage Profile
(Required Lux, Hours)]
        Q4[Window Blinds / Sun Protection]
    end

    %% Pre-calculations
    subgraph PreCalc ["Preprocessing (Rust)"]
        PJ["p_j
Installed Power (W/m²)"]
        ZONES["Zoning
Daylight (A_TL) vs Non-Daylight (A_KTL)"]
        F_PRA["F_Prä
Presence Factor"]
        F_KL["F_KL
Constant Light Factor"]
        F_TL["F_TL
Daylight Supply Factor"]
    end

    %% Operational Times
    subgraph Times ["Effective Operating Times"]
        T_DAY_TL["t_eff,Tag,TL
(Daylight Zone, Day)"]
        T_DAY_KTL["t_eff,Tag,KTL
(Non-Daylight Zone, Day)"]
        T_NIGHT["t_eff,Nacht
(All Zones, Night)"]
    end

    %% Final Output
    QLF["Q_l,f
Final Energy Demand for Lighting (kWh)"]

    Q1 --> PJ
    Q3 --> PJ
    
    G --> ZONES
    
    Q2 --> F_PRA
    Q3 --> F_PRA
    
    Q2 --> F_KL
    
    G --> F_TL
    Q4 --> F_TL
    Q2 --> F_TL

    F_TL --> T_DAY_TL
    F_PRA --> T_DAY_TL
    F_KL --> T_DAY_TL
    Q3 --> T_DAY_TL
    
    F_PRA --> T_DAY_KTL
    F_KL --> T_DAY_KTL
    Q3 --> T_DAY_KTL
    
    F_PRA --> T_NIGHT
    F_KL --> T_NIGHT
    Q3 --> T_NIGHT

    PJ --> QLF
    ZONES --> QLF
    T_DAY_TL --> QLF
    T_DAY_KTL --> QLF
    T_NIGHT --> QLF
```
