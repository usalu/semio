# Ventilation Energy Calculation Engine

This document explains the physics and logic used in the Rust calculation engine (`lib.rs` / `ventilation` module) to compute the building's ventilation heat transfer coefficient ($H_V$) and resulting energy demand ($Q_V$) according to DIN/TS 18599-2:2025-10, DIN V 18599-10, and DIN 4108-4.

## 1. Data Flow & Architecture

The ventilation engine calculates how much heat is lost (or gained) when indoor air is replaced by outdoor air. It combines the building's internal air volume, the air-tightness of the envelope, mechanical air exchange rates, supply air temperatures (heat recovery), and a highly dynamic model for occupant window-opening behavior.

```
flowchart TD
    %% UI Inputs
    subgraph UI ["User Interface (Inputs)"]
        G["Geometry Engine\n(A_NGF, V)"]
        Q1[Air Tightness / n50]
        Q3[Mech. Air Flow\n(V_mech_b, V_ETA)]
        Q4[Usage Profile\n(V_A, t_nutz, t_V,mech)]
        Q5[Heat Recovery\n(eta_t)]
    end

    %% Rust Engine Middle Layer
    subgraph PreCalc ["Preprocessing (Rust)"]
        NNUTZ["n_nutz\nRequired Air (Eq. 91)"]
        NINF["n_inf\nInfiltration Rate (Eq. 66/67)"]
        NMECH["n_mech\nDaily Mech Rate (Eq. 95-99)"]
        TVMECH["theta_V,mech\nSupply Temp (Eq. 100)"]
        NWIN["n_win\nDaily Window Airing (Eq. 80-90)"]
    end

    %% Detailed Calculations
    subgraph Calc ["Ventilation Heat Transfer Coefficients (H_V)"]
        HVINF["H_V,inf\n(Infiltration Eq. 65)"]
        HVWIN["H_V,win\n(Window Airing Eq. 75)"]
        HVMECH["H_V,mech\n(Mechanical Eq. 94)"]
        HVUE["H_V,ue\n(Unheated Zones Eq. 103)"]
    end

    %% Final Output
    HV["Q_V Total\nSum of Energy Balances"]

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

## 2. Inputs Required from the UI

To perform this highly detailed calculation, the UI needs geometry, system, and behavioral parameters.

### A. Automated Geometry (Calculated by the Sketchpad UI)

* `a_ngf`: Net floor area / Nettogrundfläche ($A_{NGF}$ in $m^2$).
* `h_room`: Average clear room height ($h_R$ in $m$).
* `v_net`: Net air volume ($V$ in $m^3$).
* `a_e`: Envelope Area ($A_E$ in $m^2$). Required if $V > 1500 m^3$.

### B. User Questions (Parameters)

| Parameter Key       | UI Question / Prompt                               | Available Options                                     | How Rust uses it                                                                           |
| ------------------- | -------------------------------------------------- | ----------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `building_type`   | **"What is the primary usage?"**             | Residential vs. Non-Residential                       | Triggers seasonal window adjustment ($f_{win,seasonal}$) per Eq. 76 & 79.                |
| `air_tightness`   | **"How airtight is the building envelope?"** | Category I, II, III, IV                               | Maps to Table 8 (DIN 18599-2) to determine default$n_{50}$or$q_{50}$.                  |
| `has_atd`         | **"Are Air Transfer Devices installed?"**    | Yes / No                                              | Calculates$f_{ATD}$(Eq. 69), increasing natural infiltration.                            |
| `mech_system`     | **"Mechanical supply and exhaust?"**         | $\dot{V}_{mech,b}$and$\dot{V}_{ETA}$(in$m^3/h$) | Used in Eq. 97 & 99 and defines if windows must be opened to balance pressure (Eq. 87-90). |
| `heat_recovery`   | **"Heat recovery efficiency?"**              | $\eta_t$value (0.0 to 1.0)                          | Used to calculate the supply air temp$\theta_{V,mech}$(Eq. 100).                         |
| `operating_times` | **"Daily usage and system hours?"**          | Sliders:$t_{nutz}$and$t_{V,mech}$                 | Time-weighting for Window Airing (Eq. 80-84) and Mech Rate (Eq. 95).                       |

## 3. Core Calculations (DIN/TS 18599-2:2025-10)

The standard requires resolving the baseline air requirements before calculating how much heat is lost through infiltration, windows, and mechanical systems.

### 1. Required Fresh Air ($n_{nutz}$)

The absolute minimum air required is based on the floor area and the specific usage profile (DIN 18599-10).
$n_{nutz} = \frac{\dot{V}_A \cdot A_{NGF}}{V}$ (Eq. 91)

### 2. Mechanical Ventilation ($H_{V,mech}$ and $\theta_{V,mech}$)

* **Supply Air Rate:** $n_{mech,SUP} = \frac{\dot{V}_{mech,b}}{V}$ (Eq. 97)
* **Exhaust Air Rate:** $n_{mech,ETA} = \frac{\dot{V}_{ETA}}{V}$ (Eq. 99)
* **Daily Average Rate:** $n_{mech} = n_{mech,SUP} \cdot \frac{t_{V,mech}}{24h}$ (Eq. 95)
* **Heat Transfer Coefficient:** $H_{V,mech} = n_{mech} \cdot V \cdot c_{p,a} \cdot \rho_a$ (Eq. 94)

 **Supply Temperature (** $\theta_{V,mech}$**):**
$\theta_{V,mech} = \theta_e + \eta_t \cdot (\theta_i - \theta_e)$ (Eq. 100)

**Energy Balance (Mechanical):**

* **Heat Sink:** $Q_{V,mech} = H_{V,mech} \cdot (\theta_i - \theta_{V,mech}) \cdot t$ (Eq. 92)
* **Heat Source:** $Q_{V,mech} = H_{V,mech} \cdot (\theta_{V,mech} - \theta_i) \cdot t$ (Eq. 93)

### 3. Window Airing Deficit ($\Delta n_{win}$)

Calculates window airing by figuring out how much fresh air is *required* ($n_{nutz}$) versus how much is *already provided* by infiltration ($n_{inf}$) and mechanical systems ($n_{SUP}$). Occupants open windows to make up the deficit.

* **Without Mech Vent:** $\Delta n_{win} = \max[0; n_{nutz} - (n_{nutz} - 0.2) \cdot n_{inf} - 0.1]$ (Eq. 81)
* **With Mech Vent:** Calculates a deficit depending on if the system pushes enough air ($n_{SUP}$) and if the system causes pressure imbalances ($n_{ETA} > n_{SUP}$).
* **Heat Transfer Coefficient:** $H_{V,win} = n_{win} \cdot V \cdot c_{p,a} \cdot \rho_a$ (Eq. 75)

### 4. Infiltration ($H_{V,inf}$)

Calculates unintentional leaks. Modified by the mechanical system's pressure ($f_e$).

* **Heat Transfer Coefficient:** $H_{V,inf} = n_{inf} \cdot V \cdot c_{p,a} \cdot \rho_a$ (Eq. 65)

### 5. Unheated Zones ($H_{V,ue}$)

Exchange rate of adjacent unheated zones to the outside:
$H_{V,ue} = c_{p,a} \cdot \rho_a \cdot n_{ue} \cdot V_u$ (Eq. 103)

## 4. Reference Data & Standard Values

To execute the logic, the engine relies on strict fallback parameters provided by DIN 18599 and DIN 4108-4. If exact values are not provided by the user, these defaults must be applied.

### 4.1 Global Constants (DIN/TS 18599-2)

* **Heat Capacity of Air (** $c_{p,a} \cdot \rho_a$**):** `0.34 Wh/(m³·K)`
* **Volume Flow Coefficient (** $e$**):** `0.07` (Standard default for building wind exposure).
* **Wind Exposure Coefficient (** $f$**):** `15.0` (Standard default used in Eq. 72).
* **Unheated Zone Infiltration (** $n_{ue}$**):** `0.6 h^-1` (Used for standard sunspaces/attics in Eq. 105).

### 4.2 Building Air Tightness ($n_{50}$ and $q_{50}$)

From  **DIN 18599-2, Table 8** . Used if no measured Blower-Door test is available.

| Tightness Category                                                                                                            | Description             | $n_{50}$(for$V \le 1500 m^3$) | $q_{50}$(for$V > 1500 m^3$) |
| ----------------------------------------------------------------------------------------------------------------------------- | ----------------------- | --------------------------------- | ------------------------------- |
| **Kategorie I**                                                                                                         | Tested to DIN 4108-7    | *Use Measured Value*            | *Use Measured Value*          |
| **Kategorie II**                                                                                                        | New buildings, untested | `4.0`                           | `6.0`                         |
| **Kategorie III**                                                                                                       | General existing stock  | `6.0`                           | `9.0`                         |
| **Kategorie IV**                                                                                                        | Obvious leaks/gaps      | `10.0`                          | `15.0`                        |
| *(Note: If* $V > 1500 m^3$*,* $q_{50}$ *must be converted via Eq. 70:* $n_{50} = \frac{q_{50} \cdot A_E}{V}$*)* |                         |                                   |                                 |

### 4.3 Component Air Permeability (DIN 4108-4, Table 8)

Extracted from  **DIN 4108-4:2020-11, Table 8** . Evaluates the leakiness of specific facade components based on construction. Used for precise component-level infiltration tracking.

| Construction Details                                                  | Air Permeability Class (DIN EN 12207) |
| --------------------------------------------------------------------- | ------------------------------------- |
| Wooden windows (incl. double windows)*without*seals                 | **Class 2**                     |
| All window constructions with age-resistant, replaceable seals        | **Class 3**                     |
| All exterior door constructions with age-resistant, replaceable seals | **Class 2**                     |

### 4.4 Minimum Window Airing ($n_{win,min}$)

Even if mechanical systems are off, a minimum limit of window-opening is assumed.

* **Residential Buildings:** `0.10 h^-1` (Subject to seasonal modification $f_{win,seasonal}$).
* **Non-Residential:** $\min(0.1, 0.1 \cdot \frac{3}{h_R})$ where $h_R$ is clear room height.

### 4.5 Usage Profiles & Fresh Air Demand ($\dot{V}_A$)

From  **DIN 18599-10** . Determines $n_{nutz}$ per Eq. 91. Also dictates default operating times ($t_{nutz}$ and $t_{V,mech}$).

| Usage Profile                      | Minimum Fresh Air$\dot{V}_A$ $[m^3/(h \cdot m^2)]$ | Typical Operating Hours ($t_{nutz}$&$t_{V,mech}$) |
| ---------------------------------- | ------------------------------------------------------ | ----------------------------------------------------- |
| Residential (Wohngebäude)         | Default$n_{nutz} = 0.5 \text{ h}^{-1}$               | `24 h/d`                                            |
| Single Office / Small Group Office | `2.0`                                                | `13 h/d`                                            |
| Open Plan Office / Retail          | `3.0`                                                | `12-13 h/d`                                         |
| Classroom / Meeting Room           | `5.0`to `6.0`                                      | `12-13 h/d`                                         |
| Restaurant                         | `7.0`                                                | `14 h/d`                                            |
| Hospital Rooms                     | `3.0`                                                | `24 h/d`                                            |
| Warehouse / Logistics              | `0.5`                                                | `14 h/d`                                            |

### 4.6 Mechanical Exhaust Default Values ($n_{mech,ETA}$)

If the mechanical system configuration is unknown or partially specified, the standard provides rules for the exhaust flow rate ($n_{mech,ETA}$) to correctly calculate the pressure balance.

* **Balanced Systems:** If unspecified, assume $n_{mech,ETA} = n_{mech,SUP}$.
* **Pure Exhaust Systems (Abluftanlagen):** $n_{mech,SUP} = 0$. The exhaust rate $n_{mech,ETA}$ defaults to the required fresh air $n_{nutz}$. The deficit is fully covered by window airing and infiltration.

### 4.7 Heat Recovery Efficiency Defaults ($\eta_t$)

If a mechanical system is present but the heat exchanger's exact efficiency is not known, fallback values can be used to estimate the supply temperature ($\theta_{V,mech}$).

| System Quality                  | Heat Recovery Efficiency ($\eta_t$) | Effect on Supply Temperature   |
| ------------------------------- | ------------------------------------- | ------------------------------ |
| Pure Exhaust System (No supply) | `0.00`(0%)                          | $\theta_{V,mech} = \theta_e$ |
| Standard / Older Systems        | `0.60`(60%)                         | Recovers 60% of delta T        |
| Modern High-Efficiency Systems  | `0.80`(80%)                         | Recovers 80% of delta T        |
| Premium / Passive House Systems | `0.90`(90%)                         | Recovers 90% of delta T        |

## Simplified UI Data Mapping (The Lungs & The Heart)

By asking chronological and physical questions, the user's brain can "walk through" their building, keeping the frontend conversational while the backend remains strictly DIN-compliant.

### The Lungs: Passive Breathing (has_atd, air_tightness)
Users don't understand "infiltration" or "ATDs", but they know what their windows look like.
*   **has_atd**: "Do your windows or exterior walls have small, built-in ventilation slits that let air trickle in even when they are closed?" Yes -> `has_atd = true`, No -> `has_atd = false`.
*   **air_tightness ($n_{50}$)**: "Has your building ever officially passed a Blower-Door pressure test?" If Yes, Category I. If No: "When were the windows and roof last replaced?" After 2000 -> Category II. Before 2000, no drafts -> Category III. Before 2000, drafts -> Category IV.

### The Heart: Active Systems (Mechanical Volumes, Heat Recovery)
Mechanical ventilation is intimidating. If a user doesn't know their exact flow rate ($m^3/h$), DIN 18599 allows us to assume the system was sized correctly to meet the minimum required fresh air rate ($n_{nutz}$).
*   **Mechanical Ventilation Volumes**: "Do you have an active, motorized ventilation system?" If yes, but they don't know the exact airflow rate ($m^3/h$), our software applies an *Estimation Trick*. It automatically calculates the minimum required fresh air ($n_{nutz}$) and assumes the system was designed correctly, setting supply and exhaust equal to $n_{nutz} \times Volume$.
*   **Heat Recovery Efficiency**: "Does your ventilation system feature 'Heat Recovery'?" If yes, and they don't know the percentage, we can safely estimate 80% (0.80) as a default fallback for modern systems.
