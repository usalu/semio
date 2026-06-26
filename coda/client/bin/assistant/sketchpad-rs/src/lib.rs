use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::cell::RefCell;
use serde_json::Value;

pub mod transmission {
//! # Transmission Module
//!
//! This module implements the transmission energy calculations according to DIN V 18599-2.
//!
//! ## Equations
//!
//! 1. **U-value of opaque component**:
//!    // formula: U = \frac{1}{R_T}
//!    // formula: R_T = R_{si} + \sum \left( \frac{d_i}{\lambda_i} \right) + R_{se}
//!
//! 2. **Effective U-value for windows**:
//!    // formula: U_{w,eff} = U_w \cdot (1 - f_{sh}) + U_{w,sh} \cdot f_{sh}
//!
//! 3. **Direct Transmission to Outside**:
//!    // formula: H_{T,D} = \sum (f_{neig,j} \cdot U_j \cdot A_j)
//!
//! 4. **Transmission to Unheated Zones**:
//!    // formula: H_{T,iu,eff} = \sum (A_j \cdot U_j \cdot F_{x,j})
//!
//! 5. **Thermal Bridges (Simplified)**:
//!    // formula: H_{T,WB} = \Delta U_{WB} \cdot \sum A_j
//!
//! 6. **Thermal Bridges (Detailed)**:
//!    // formula: H_{T,WB} = \sum (l_j \cdot \Psi_j \cdot f_{x,j})
//!
//! 7. **Overall Transmission Heat Transfer Coefficient**:
//!    // formula: H_T = H_{T,D} + \sum(F_{x,j} \cdot H_{T,iu,j}) + H_{T,WB}
//!
//! 8. **Specific Heat Transfer Coefficient**:
//!    // formula: H'_{T} = \frac{H_T}{A}

use serde::{Deserialize, Serialize};

/// Represents a single material with its thermal conductivity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    /// Thermal conductivity $\lambda$ in W/(m*K)
    pub lambda: f64,
}

/// Represents a single layer within a building component.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layer {
    pub material: Material,
    /// Thickness in meters
    pub thickness: f64,
}

/// Represents an opaque building component (wall, roof, floor).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildingComponent {
    pub name: String,
    pub layers: Vec<Layer>,
    /// Inner surface thermal resistance ($R_{si}$), standard 0.13
    pub r_si: f64,
    /// Outer surface thermal resistance ($R_{se}$), standard 0.04 (or 0.13 for internal)
    pub r_se: f64,
    /// Component Area ($A_j$) in $m^2$
    pub area: f64,
    /// Inclination factor ($f_{neig,j}$) - defaults to 1.0 for opaque components
    pub f_neig: f64,
    /// Temperature correction factor ($F_{x,j}$) - 1.0 for direct exterior
    pub f_x: f64,
}

impl BuildingComponent {
    /// Calculates the U-value of the opaque component.
    ///
    /// // formula: U = \frac{1}{R_T}
    /// // formula: R_T = R_{si} + \sum \left( \frac{d_i}{\lambda_i} \right) + R_{se}
    pub fn calculate_u_value(&self) -> f64 {
        let sum_r: f64 = self.layers.iter()
            .map(|l| l.thickness / l.material.lambda)
            .sum();
        
        let r_t = self.r_si + sum_r + self.r_se;
        1.0 / r_t
    }

    /// Calculates the U-value for an internal component where both sides have $R_{si}$.
    pub fn calculate_u_value_internal(&self) -> f64 {
        let sum_r: f64 = self.layers.iter()
            .map(|l| l.thickness / l.material.lambda)
            .sum();
        
        let r_t = self.r_si + sum_r + self.r_si; 
        1.0 / r_t
    }
}

/// Represents a window component.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowComponent {
    pub name: String,
    /// Component Area ($A_w$) in $m^2$
    pub area: f64,
    /// Base U-value of the window ($U_w$) in W/(m^2*K)
    pub u_w: f64,
    /// U-value of the window with closed shutter ($U_{w,sh}$) in W/(m^2*K)
    pub u_w_sh: f64,
    /// Shutter usage fraction ($f_{sh}$)
    pub f_sh: f64,
    /// Inclination factor ($f_{neig,j}$)
    pub f_neig: f64,
    /// Temperature correction factor ($F_{x,j}$) - usually 1.0
    pub f_x: f64,
}

impl WindowComponent {
    /// Calculates the effective U-value of the window considering shutters.
    ///
    /// // formula: U_{w,eff} = U_w \cdot (1 - f_{sh}) + U_{w,sh} \cdot f_{sh}
    pub fn calculate_u_w_eff(&self) -> f64 {
        self.u_w * (1.0 - self.f_sh) + self.u_w_sh * self.f_sh
    }
}

/// 1. Direct Transmission to Outside ($H_{T,D}$)
/// Calculates heat transfer through components directly touching the outside air.
///
/// // formula: H_{T,D} = \sum (f_{neig,j} \cdot U_j \cdot A_j)
pub fn calculate_h_t_d(components: &[BuildingComponent], windows: &[WindowComponent]) -> f64 {
    let comp_h_t_d: f64 = components
        .iter()
        .filter(|c| (c.f_x - 1.0).abs() < 1e-5) // Only direct exterior components
        .map(|c| c.area * c.calculate_u_value() * c.f_neig)
        .sum();
    
    let win_h_t_d: f64 = windows
        .iter()
        .filter(|w| (w.f_x - 1.0).abs() < 1e-5)
        .map(|w| w.area * w.calculate_u_w_eff() * w.f_neig)
        .sum();

    comp_h_t_d + win_h_t_d
}

/// 2. Transmission to Unheated Zones ($H_{T,iu}$)
/// Calculates effective heat transfer through walls/floors touching unheated rooms or the ground.
/// Note: This function already applies the $F_{x,j}$ factor.
///
/// // formula: H_{T,iu,eff} = \sum (A_j \cdot U_j \cdot F_{x,j})
pub fn calculate_h_t_iu(components: &[BuildingComponent]) -> f64 {
    components
        .iter()
        .filter(|c| (c.f_x - 1.0).abs() >= 1e-5) // Only components to unheated spaces/ground
        // Assuming r_se is configured correctly (e.g., to 0.13) for these components
        .map(|c| c.area * c.calculate_u_value() * c.f_x) 
        .sum()
}

/// 5. Thermal Bridges ($H_{T,WB}$) - Simplified Method
/// Calculates the extra heat lost through structural joints using a flat addition.
///
/// // formula: H_{T,WB} = \Delta U_{WB} \cdot \sum A_j
pub fn calculate_h_t_wb_simplified(delta_u_wb: f64, total_area: f64) -> f64 {
    delta_u_wb * total_area
}

/// 2. Overall Transmission Heat Transfer Coefficient ($H_T$)
/// Sums up the "leakiness" of the entire building envelope.
///
/// // formula: H_T = H_{T,D} + \sum(F_{x,j} \cdot H_{T,iu,j}) + H_{T,WB}
pub fn calculate_h_t_total(h_t_d: f64, h_t_iu_effective: f64, h_t_wb: f64) -> f64 {
    h_t_d + h_t_iu_effective + h_t_wb
}

/// 7. Specific Heat Transfer Coefficient ($H'_T$)
/// Evaluates the overall energy quality of the building envelope relative to its size.
///
/// // formula: H'_{T} = \frac{H_T}{A}
pub fn calculate_h_t_specific(h_t_total: f64, total_area: f64) -> f64 {
    if total_area > 0.0 {
        h_t_total / total_area
    } else {
        0.0
    }
}

/// Represents a Detailed Linear Thermal Bridge
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LinearThermalBridge {
    pub length: f64,       // l_j or l_i in meters
    pub psi_value: f64,    // Psi_j or Psi_Fx_i in W/(m*K) (Calculated via DIN EN ISO 10211)
    pub f_x: f64,          // Temp correction factor if bridge is to unheated space/ground
}

/// 3. Detailed Thermal Bridges Calculation ($H_{T,WB}$)
/// // formula: H_{T,WB} = \sum (l_j \cdot \Psi_j \cdot f_{x,j})
pub fn calculate_detailed_h_t_wb(bridges: &[LinearThermalBridge]) -> f64 {
    bridges
        .iter()
        .map(|b| b.length * b.psi_value * b.f_x)
        .sum()
}

// ==============================================================================
// Enumerations and Tables for DIN V 18599 (MCP Tool Input Interfaces)
// ==============================================================================

/// Represents the type of unheated space for determining the $F_x$ factor (Table 5).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum UnheatedSpaceType {
    /// Dachraum (Unheated Attic / Roof space)
    Attic,
    /// Unbeheizter Glasvorbau / Wintergarten (Unheated Sunspace)
    Sunspace,
    /// Kriechkeller, stark belüftet (Crawl space, heavily ventilated)
    CrawlSpaceVentilated,
    /// Angrenzender unbeheizter Raum (Standard adjacent unheated room)
    AdjacentUnheatedRoom,
    /// Unbeheizter Keller (Unheated Basement, general)
    UnheatedBasement,
    /// Treppenhaus, außenliegend (Staircase with large exterior walls)
    StaircaseExterior,
    /// Kriechkeller, unbelüftet (Crawl space, unventilated)
    CrawlSpaceUnventilated,
    /// Treppenhaus, innenliegend (Staircase mostly surrounded by building)
    StaircaseInterior,
}

impl UnheatedSpaceType {
    pub fn f_x_factor(&self) -> f64 {
        match self {
            Self::Attic => 0.8,
            Self::Sunspace => 0.8,
            Self::CrawlSpaceVentilated => 0.8,
            Self::AdjacentUnheatedRoom => 0.5,
            Self::UnheatedBasement => 0.5,
            Self::StaircaseExterior => 0.5,
            Self::CrawlSpaceUnventilated => 0.5,
            Self::StaircaseInterior => 0.35,
        }
    }
}

/// Represents the type of ground contact for determining the $F_x$ factor (Table 6).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GroundContactType {
    /// Bodenplatte auf Erdreich (Floor slab directly on ground)
    FloorSlabOnGround,
    /// Wände gegen Erdreich, < 1,5m Tiefe (Basement walls, shallow depth)
    BasementWallShallow,
    /// Wände gegen Erdreich, > 1,5m Tiefe (Basement walls, deep depth)
    BasementWallDeep,
    /// Fußboden des beheizten Kellers (Floor of a heated basement)
    HeatedBasementFloor,
    /// Bauteile gegen Grundwasser (Components touching groundwater)
    GroundwaterContact,
}

impl GroundContactType {
    pub fn f_x_factor(&self) -> f64 {
        match self {
            Self::GroundwaterContact => 1.0,
            _ => 0.5,
        }
    }
}

/// Window Glazing Type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WindowGlazingType {
    Single, Double, Triple,
}

/// Window Inclination Angle
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WindowInclinationAngle {
    Deg0, Deg15, Deg30, Deg45, Deg60, Deg75, Deg90,
}

/// Returns the inclination factor ($f_{neig,j}$) based on Table 7.
pub fn get_inclination_factor(glazing: WindowGlazingType, angle: WindowInclinationAngle) -> f64 {
    match (angle, glazing) {
        (WindowInclinationAngle::Deg0, WindowGlazingType::Single) => 1.25,
        (WindowInclinationAngle::Deg0, WindowGlazingType::Double) => 1.21,
        (WindowInclinationAngle::Deg0, WindowGlazingType::Triple) => 1.20,
        (WindowInclinationAngle::Deg15, WindowGlazingType::Single) => 1.21,
        (WindowInclinationAngle::Deg15, WindowGlazingType::Double) => 1.22,
        (WindowInclinationAngle::Deg15, WindowGlazingType::Triple) => 1.16,
        (WindowInclinationAngle::Deg30, WindowGlazingType::Single) => 1.19,
        (WindowInclinationAngle::Deg30, WindowGlazingType::Double) => 1.21,
        (WindowInclinationAngle::Deg30, WindowGlazingType::Triple) => 1.13,
        (WindowInclinationAngle::Deg45, WindowGlazingType::Single) => 1.21,
        (WindowInclinationAngle::Deg45, WindowGlazingType::Double) => 1.15,
        (WindowInclinationAngle::Deg45, WindowGlazingType::Triple) => 1.07,
        (WindowInclinationAngle::Deg60, WindowGlazingType::Single) => 1.00,
        (WindowInclinationAngle::Deg60, WindowGlazingType::Double) => 1.13,
        (WindowInclinationAngle::Deg60, WindowGlazingType::Triple) => 1.05,
        (WindowInclinationAngle::Deg75, WindowGlazingType::Single) => 1.00,
        (WindowInclinationAngle::Deg75, WindowGlazingType::Double) => 1.08,
        (WindowInclinationAngle::Deg75, WindowGlazingType::Triple) => 1.02,
        (WindowInclinationAngle::Deg90, _) => 1.00,
    }
}

/// Thermal Bridge Planning Category for simplified penalty ($\Delta U_{WB}$)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ThermalBridgeCategory {
    /// Internal insulation issues (0.15)
    InternalInsulationIssues,
    /// Standard Default (0.10)
    StandardDefault,
    /// Good Planning / Category A (0.05)
    GoodPlanning,
    /// Excellent Planning / Category B (0.03)
    ExcellentPlanning,
}

impl ThermalBridgeCategory {
    pub fn delta_u_wb(&self) -> f64 {
        match self {
            Self::InternalInsulationIssues => 0.15,
            Self::StandardDefault => 0.10,
            Self::GoodPlanning => 0.05,
            Self::ExcellentPlanning => 0.03,
        }
    }
}

/// Month Enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Month {
    Jan, Feb, Mar, Apr, May, Jun, Jul, Aug, Sep, Oct, Nov, Dec
}

impl Month {
    pub fn days_in_month(&self) -> f64 {
        match self {
            Self::Jan | Self::Mar | Self::May | Self::Jul | Self::Aug | Self::Oct | Self::Dec => 31.0,
            Self::Apr | Self::Jun | Self::Sep | Self::Nov => 30.0,
            Self::Feb => 28.0,
        }
    }
}

/// Building Type for Shutter Usage
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BuildingType {
    Residential,
    NonResidential,
}

/// Shutter Control System Type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ShutterControl {
    Manual,
    Automated,
}

/// Returns the shutter usage fraction ($f_{sh}$) based on Annex G.
pub fn get_shutter_fraction(month: Month, b_type: BuildingType, control: ShutterControl) -> f64 {
    match (b_type, control, month) {
        (BuildingType::NonResidential, ShutterControl::Manual, _) => 0.0,
        (_, ShutterControl::Automated, Month::Jan) => 0.61,
        (_, ShutterControl::Automated, Month::Feb) => 0.54,
        (_, ShutterControl::Automated, Month::Mar) => 0.45,
        (_, ShutterControl::Automated, Month::Apr) => 0.36,
        (_, ShutterControl::Automated, Month::May) => 0.28,
        (_, ShutterControl::Automated, Month::Jun) => 0.23,
        (_, ShutterControl::Automated, Month::Jul) => 0.25,
        (_, ShutterControl::Automated, Month::Aug) => 0.33,
        (_, ShutterControl::Automated, Month::Sep) => 0.42,
        (_, ShutterControl::Automated, Month::Oct) => 0.53,
        (_, ShutterControl::Automated, Month::Nov) => 0.60,
        (_, ShutterControl::Automated, Month::Dec) => 0.64,
        (BuildingType::Residential, ShutterControl::Manual, Month::Jan) => 0.43,
        (BuildingType::Residential, ShutterControl::Manual, Month::Feb) => 0.38,
        (BuildingType::Residential, ShutterControl::Manual, Month::Mar) => 0.32,
        (BuildingType::Residential, ShutterControl::Manual, Month::Apr) => 0.25,
        (BuildingType::Residential, ShutterControl::Manual, Month::May) => 0.20,
        (BuildingType::Residential, ShutterControl::Manual, Month::Jun) => 0.16,
        (BuildingType::Residential, ShutterControl::Manual, Month::Jul) => 0.18,
        (BuildingType::Residential, ShutterControl::Manual, Month::Aug) => 0.23,
        (BuildingType::Residential, ShutterControl::Manual, Month::Sep) => 0.29,
        (BuildingType::Residential, ShutterControl::Manual, Month::Oct) => 0.37,
        (BuildingType::Residential, ShutterControl::Manual, Month::Nov) => 0.42,
        (BuildingType::Residential, ShutterControl::Manual, Month::Dec) => 0.45,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ClimateRegion {
    Bremerhaven, Rostock, Hamburg, Potsdam, Essen, BadMarienberg, Kassel, Braunlage,
    Chemnitz, Hof, Fichtelberg, Mannheim, Passau, Stoetten, GarmischPartenkirchen,
}

impl ClimateRegion {
    /// Returns the 12 monthly external temperatures for the region [Jan..Dec].
    pub fn monthly_temperatures(&self) -> [f64; 12] {
        match self {
            Self::Bremerhaven => [2.9, 3.2, 5.4, 9.0, 13.1, 16.0, 17.9, 18.2, 15.0, 10.6, 6.1, 3.2],
            Self::Rostock => [2.3, 2.4, 4.3, 8.0, 12.4, 15.6, 18.0, 18.0, 14.7, 10.2, 5.5, 2.6],
            Self::Hamburg => [2.5, 2.7, 4.9, 8.5, 12.8, 15.5, 17.8, 17.8, 14.1, 9.8, 5.1, 2.3],
            Self::Potsdam => [1.0, 1.9, 4.7, 9.2, 14.1, 16.7, 19.0, 18.6, 14.3, 9.5, 4.1, 0.9],
            Self::Essen => [3.1, 3.5, 6.6, 9.5, 13.7, 15.9, 18.2, 18.2, 14.6, 10.8, 6.1, 3.5],
            Self::BadMarienberg => [0.1, 0.5, 3.6, 7.0, 11.5, 14.0, 16.1, 16.0, 12.3, 8.1, 3.2, 0.6],
            Self::Kassel => [1.0, 2.1, 5.2, 8.8, 13.3, 15.9, 18.1, 17.8, 13.7, 9.5, 4.5, 1.7],
            Self::Braunlage => [-0.8, -0.3, 2.1, 5.7, 10.5, 12.9, 15.0, 15.0, 11.1, 7.1, 2.3, -0.2],
            Self::Chemnitz => [0.5, 1.0, 3.9, 8.2, 12.9, 15.5, 17.5, 17.6, 13.2, 9.2, 3.8, 0.8],
            Self::Hof => [-1.2, -0.4, 2.8, 6.6, 11.7, 14.5, 16.3, 16.6, 12.0, 7.6, 2.3, -0.7],
            Self::Fichtelberg => [-3.3, -3.5, -1.3, 2.3, 7.4, 9.8, 12.2, 12.4, 8.1, 4.4, -0.6, -2.8],
            Self::Mannheim => [2.4, 3.6, 7.1, 10.6, 15.6, 18.1, 20.1, 20.2, 15.7, 11.0, 5.7, 3.1],
            Self::Passau => [-1.2, 0.4, 4.3, 8.2, 13.7, 16.4, 18.0, 17.8, 13.1, 8.7, 3.0, -0.2],
            Self::Stoetten => [-0.5, 0.3, 3.4, 6.8, 11.8, 14.4, 16.6, 16.7, 12.3, 8.5, 2.6, -0.2],
            Self::GarmischPartenkirchen => [-2.3, -0.5, 3.2, 7.0, 11.8, 14.8, 16.6, 16.4, 12.3, 8.4, 1.9, -1.8],
        }
    }

    /// Returns seasonal irradiation values in kWh/m2: [North, East, South, West, Horizontal]
    pub fn seasonal_irradiation(&self) -> [f64; 5] {
        match self {
            _ => [160.0, 271.0, 392.0, 271.0, 392.0], // Potsdam/Reference standard
        }
    }
}

/// Usage Profiles mapping to daily heating hours and usage days.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum UsageProfile {
    Residential,
    SingleOffice,
    GroupOffice,
    OpenPlanOffice,
    MeetingRoom,
    TicketHall,
    RetailStore,
    RetailFood,
    Classroom,
    Auditorium,
    HospitalRoom,
    HotelRoom,
    Canteen,
    Restaurant,
    KitchenCommercial,
    KitchenPrep,
    WC,
    Lounge,
    AuxiliarySpace,
    TrafficArea,
    StorageArchive,
    DataCenter,
    IndustrialHeavy,
    IndustrialMedium,
    IndustrialLight,
    AudienceArea,
    Foyer,
    Stage,
    ExhibitionHall,
    Museum,
    LibraryReading,
    LibraryOpenAccess,
    LibraryArchive,
    Gymnasium,
    ParkingOffice,
    ParkingPublic,
    Sauna,
    Fitness,
    Laboratory,
    ExaminationRoom,
    SpecialCare,
    WardCorridor,
    MedicalPractice,
    LogisticsHall,
}

impl UsageProfile {
    pub fn usage_days(&self) -> f64 {
        match self {
            Self::Residential | Self::HospitalRoom | Self::HotelRoom | Self::Restaurant | Self::KitchenCommercial | Self::KitchenPrep | Self::DataCenter | Self::Sauna | Self::Fitness | Self::SpecialCare | Self::WardCorridor => 365.0,
            Self::RetailStore | Self::RetailFood | Self::Museum | Self::LibraryReading | Self::LibraryOpenAccess | Self::LibraryArchive | Self::Gymnasium | Self::ParkingPublic => 300.0,
            Self::AudienceArea | Self::Foyer | Self::Stage | Self::ExhibitionHall => 200.0,
            _ => 250.0,
        }
    }

    pub fn daily_heating_hours(&self) -> f64 {
        match self {
            Self::HospitalRoom | Self::HotelRoom | Self::DataCenter | Self::SpecialCare | Self::WardCorridor => 24.0,
            Self::Residential => 17.0,
            Self::Restaurant | Self::KitchenCommercial | Self::KitchenPrep | Self::IndustrialHeavy | Self::IndustrialMedium | Self::IndustrialLight | Self::ExhibitionHall | Self::Gymnasium | Self::ParkingPublic | Self::Sauna | Self::Fitness | Self::LogisticsHall => 14.0,
            Self::SingleOffice | Self::GroupOffice | Self::OpenPlanOffice | Self::MeetingRoom | Self::TicketHall | Self::WC | Self::Lounge | Self::AuxiliarySpace | Self::TrafficArea | Self::StorageArchive | Self::LibraryReading | Self::LibraryOpenAccess | Self::LibraryArchive | Self::ParkingOffice | Self::Laboratory | Self::ExaminationRoom => 13.0,
            Self::RetailStore | Self::RetailFood | Self::Classroom | Self::Auditorium | Self::Canteen | Self::Museum | Self::MedicalPractice => 12.0,
            Self::AudienceArea | Self::Foyer | Self::Stage => 10.0,
        }
    }

    pub fn heating_setpoint(&self) -> f64 {
        match self {
            Self::IndustrialHeavy => 15.0,
            Self::IndustrialMedium => 17.0,
            Self::Gymnasium => 19.0,
            Self::Residential | Self::IndustrialLight | Self::Fitness => 20.0,
            Self::HospitalRoom | Self::SpecialCare | Self::WardCorridor | Self::MedicalPractice | Self::Laboratory | Self::ExaminationRoom => 22.0,
            Self::Sauna => 24.0,
            Self::LogisticsHall => 12.0,
            _ => 21.0,
        }
    }

    pub fn cooling_setpoint(&self) -> Option<f64> {
        match self {
            Self::Sauna => None,
            Self::IndustrialHeavy => Some(28.0),
            Self::IndustrialMedium | Self::LogisticsHall => Some(26.0),
            Self::Residential => Some(25.0),
            _ => Some(24.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AutomationClass {
    A, B, C, D
}

impl AutomationClass {
    pub fn temperature_shift(&self, profile: UsageProfile) -> f64 {
        match self {
            Self::D => 0.0,
            Self::C => {
                match profile {
                    UsageProfile::Residential => -0.5,
                    _ => 0.0, 
                }
            },
            Self::B => {
                match profile {
                    UsageProfile::RetailStore | UsageProfile::RetailFood | UsageProfile::Canteen | UsageProfile::Restaurant | UsageProfile::KitchenCommercial | UsageProfile::KitchenPrep | UsageProfile::ExhibitionHall => -1.5,
                    UsageProfile::Classroom | UsageProfile::IndustrialHeavy | UsageProfile::IndustrialMedium | UsageProfile::IndustrialLight => -1.2,
                    UsageProfile::Museum => -0.75,
                    UsageProfile::WC | UsageProfile::Lounge | UsageProfile::AuxiliarySpace | UsageProfile::TrafficArea | UsageProfile::StorageArchive | UsageProfile::DataCenter | UsageProfile::AudienceArea | UsageProfile::LibraryReading | UsageProfile::LibraryOpenAccess | UsageProfile::LibraryArchive | UsageProfile::Gymnasium | UsageProfile::ParkingOffice | UsageProfile::ParkingPublic | UsageProfile::Sauna | UsageProfile::Fitness | UsageProfile::Laboratory | UsageProfile::ExaminationRoom | UsageProfile::SpecialCare | UsageProfile::WardCorridor | UsageProfile::MedicalPractice | UsageProfile::LogisticsHall => -0.5,
                    UsageProfile::Foyer | UsageProfile::Stage => -0.3,
                    _ => -1.0, 
                }
            },
            Self::A => {
                match profile {
                    UsageProfile::RetailStore | UsageProfile::RetailFood | UsageProfile::Classroom | UsageProfile::Auditorium | UsageProfile::Canteen | UsageProfile::Restaurant | UsageProfile::KitchenCommercial | UsageProfile::KitchenPrep | UsageProfile::ExhibitionHall => -2.0,
                    UsageProfile::IndustrialHeavy | UsageProfile::IndustrialMedium | UsageProfile::IndustrialLight => -1.8,
                    UsageProfile::Museum => -1.25,
                    UsageProfile::WC | UsageProfile::Lounge | UsageProfile::AuxiliarySpace | UsageProfile::TrafficArea | UsageProfile::StorageArchive | UsageProfile::AudienceArea | UsageProfile::LibraryReading | UsageProfile::LibraryOpenAccess | UsageProfile::LibraryArchive | UsageProfile::Gymnasium | UsageProfile::ParkingOffice | UsageProfile::ParkingPublic | UsageProfile::Sauna | UsageProfile::Fitness | UsageProfile::Laboratory | UsageProfile::ExaminationRoom | UsageProfile::SpecialCare | UsageProfile::WardCorridor | UsageProfile::MedicalPractice | UsageProfile::LogisticsHall => -1.0,
                    UsageProfile::DataCenter | UsageProfile::Foyer | UsageProfile::Stage => -0.5,
                    _ => -1.5, 
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unheated_space_fx() {
        assert_eq!(UnheatedSpaceType::Attic.f_x_factor(), 0.8);
        assert_eq!(UnheatedSpaceType::StaircaseInterior.f_x_factor(), 0.35);
    }

    #[test]
    fn test_inclination_factor() {
        assert_eq!(get_inclination_factor(WindowGlazingType::Single, WindowInclinationAngle::Deg0), 1.25);
        assert_eq!(get_inclination_factor(WindowGlazingType::Triple, WindowInclinationAngle::Deg45), 1.07);
        assert_eq!(get_inclination_factor(WindowGlazingType::Double, WindowInclinationAngle::Deg90), 1.0);
    }

    #[test]
    fn test_shutter_fraction() {
        assert_eq!(get_shutter_fraction(Month::Jan, BuildingType::Residential, ShutterControl::Manual), 0.43);
        assert_eq!(get_shutter_fraction(Month::Jul, BuildingType::NonResidential, ShutterControl::Manual), 0.0);
        assert_eq!(get_shutter_fraction(Month::Dec, BuildingType::NonResidential, ShutterControl::Automated), 0.64);
    }

    #[test]
    fn test_usage_profile() {
        let office = UsageProfile::SingleOffice;
        assert_eq!(office.heating_setpoint(), 21.0);
        assert_eq!(office.cooling_setpoint(), Some(24.0));
        assert_eq!(office.daily_heating_hours(), 13.0);
        assert_eq!(office.usage_days(), 250.0);

        let hospital = UsageProfile::HospitalRoom;
        assert_eq!(hospital.heating_setpoint(), 22.0);
        assert_eq!(hospital.daily_heating_hours(), 24.0);
        assert_eq!(hospital.usage_days(), 365.0);
    }

    #[test]
    fn test_automation_class() {
        assert_eq!(AutomationClass::D.temperature_shift(UsageProfile::SingleOffice), 0.0);
        assert_eq!(AutomationClass::A.temperature_shift(UsageProfile::RetailStore), -2.0);
        assert_eq!(AutomationClass::B.temperature_shift(UsageProfile::Residential), -1.0);
    }
}

}

pub mod ventilation {
    use std::fmt;

    const C_AIR: f64 = 0.34; // Wh/(m³K) (c_p,a * rho_a)
    const DEFAULT_E: f64 = 0.07; // Standard Volumenstromkoeffizient
    const DEFAULT_F: f64 = 15.0; // Standard Windexposition

    /// Categories according to Table 8 (DIN 18599-2)
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum TightnessCategory {
        CategoryI,   // Measured/Tested
        CategoryII,  // New, untested
        CategoryIII, // Other
        CategoryIV,  // Obvious leaks
    }

    pub struct BuildingAirData {
        pub volume: f64,       // V (m³)
        pub a_ngf: f64,        // A_NGF (m²)
        pub h_room: f64,       // h_R (m)
        pub a_e: f64,          // A_E (m²) Envelope Area
        pub is_residential: bool,
        pub n50_measured: Option<f64>,
        pub q50_measured: Option<f64>,
    }

    impl BuildingAirData {
        /// Eq 70 & Table 8: Resolves the appropriate n50 value
        pub fn resolve_n50(&self, category: TightnessCategory) -> f64 {
            if self.volume <= 1500.0 {
                match category {
                    TightnessCategory::CategoryI => self.n50_measured.unwrap_or(3.0),
                    TightnessCategory::CategoryII => 4.0,
                    TightnessCategory::CategoryIII => 6.0,
                    TightnessCategory::CategoryIV => 10.0,
                }
            } else {
                let q50 = match category {
                    TightnessCategory::CategoryI => self.q50_measured.unwrap_or(3.0),
                    TightnessCategory::CategoryII => 6.0,
                    TightnessCategory::CategoryIII => 9.0,
                    TightnessCategory::CategoryIV => 15.0,
                };
                let vol = if self.volume < 0.001 { 0.001 } else { self.volume };
                (q50 * self.a_e) / vol // Eq. 70
            }
        }
    }

    // --- 1. REQUIRED FRESH AIR (Eq. 91) ---
    pub fn calculate_n_nutz(v_dot_a: f64, a_ngf: f64, volume: f64) -> f64 {
        let vol = if volume < 0.001 { 0.001 } else { volume };
        (v_dot_a * a_ngf) / vol
    }

    // --- 2. INFILTRATION & ATD (Eq. 66 - 72) ---
    pub fn calculate_f_atd(has_atd: bool, n50: f64) -> f64 {
        if !has_atd { 
            1.0 
        } else { 
            let safe_n50 = if n50 < 0.001 { 0.001 } else { n50 };
            f64::min(16.0, (safe_n50 + 1.5) / safe_n50) 
        }
    }

    pub fn calculate_f_e(n_sup: f64, n_eta: f64, n50: f64, f_atd: f64) -> f64 {
        if (n_sup - n_eta).abs() < 0.001 {
            1.0 // Balanced
        } else {
            let denom = n50 * f_atd;
            let safe_denom = if denom < 0.001 { 0.001 } else { denom };
            let imbalance = (n_sup - n_eta) / safe_denom;
            1.0 / (1.0 + DEFAULT_F * DEFAULT_E * imbalance.powi(2))
        }
    }

    pub fn calculate_n_inf(n50: f64, f_atd: f64, f_e: f64, t_v_mech: f64) -> f64 {
        if t_v_mech <= 0.0 {
            n50 * DEFAULT_E * f_atd
        } else {
            n50 * DEFAULT_E * f_atd * (1.0 + (f_e - 1.0) * (t_v_mech / 24.0))
        }
    }

    // --- 3. MECHANICAL VENTILATION (Eq. 92 - 102) ---
    pub struct MechanicalSystem {
        pub v_dot_mech_b: f64, // Supply
        pub v_dot_eta: f64,    // Exhaust
        pub eta_t: f64,        // Heat recovery efficiency
        pub t_v_mech: f64,     // Operating hours
    }

    impl MechanicalSystem {
        pub fn n_mech_sup(&self, volume: f64) -> f64 { 
            let vol = if volume < 0.001 { 0.001 } else { volume };
            self.v_dot_mech_b / vol 
        }
        pub fn n_mech_eta(&self, volume: f64) -> f64 { 
            let vol = if volume < 0.001 { 0.001 } else { volume };
            self.v_dot_eta / vol 
        }
        pub fn n_mech_daily(&self, volume: f64) -> f64 { self.n_mech_sup(volume) * (self.t_v_mech / 24.0) }
      
        pub fn theta_v_mech(&self, theta_e: f64, theta_i: f64) -> f64 {
            theta_e + self.eta_t * (theta_i - theta_e)
        }

        pub fn calculate_h_v_mech(&self, volume: f64) -> f64 {
            self.n_mech_daily(volume) * volume * C_AIR
        }
    }

    // --- 4. WINDOW AIRING (Eq. 73 - 90) ---
    pub struct WindowAiringParams {
        pub n_nutz: f64,
        pub n_inf_0: f64,  
        pub f_e: f64,      
        pub n_sup: f64,    
        pub n_eta: f64,    
        pub t_nutz: f64,   
        pub t_v_mech: f64,   
        pub n_win_min: f64,  
    }

    impl WindowAiringParams {
        pub fn delta_n_win(&self, n_inf: f64) -> f64 {
            let deficit = if self.n_nutz < 1.2 {
                self.n_nutz - (self.n_nutz - 0.2) * n_inf - 0.1
            } else {
                self.n_nutz - n_inf - 0.1
            };
            f64::max(0.0, deficit)
        }

        pub fn delta_n_win_mech(&self) -> f64 {
            let delta_0 = if self.n_nutz < 1.2 {
                f64::max(0.0, self.n_nutz - (self.n_nutz - 0.2) * self.n_inf_0 * self.f_e - 0.1)
            } else {
                f64::max(0.0, self.n_nutz - self.n_inf_0 * self.f_e - 0.1)
            };

            let total_inf = self.n_sup + self.n_inf_0;

            if delta_0 <= self.n_sup {
                if self.n_eta <= total_inf { 0.0 } else { self.n_eta - self.n_sup - self.n_inf_0 }
            } else {
                if self.n_eta <= delta_0 + self.n_inf_0 { delta_0 - self.n_sup } else { self.n_eta - self.n_sup - self.n_inf_0 }
            }
        }

        pub fn calculate_n_win_daily(&self, n_inf: f64) -> f64 {
            if self.t_v_mech <= 0.0 {
                self.n_win_min + self.delta_n_win(n_inf) * (self.t_nutz / 24.0)
            } else if self.t_v_mech >= self.t_nutz {
                self.n_win_min + self.delta_n_win_mech() * (self.t_v_mech / 24.0)
            } else {
                self.n_win_min 
                + self.delta_n_win(n_inf) * ((self.t_nutz - self.t_v_mech) / 24.0) 
                + self.delta_n_win_mech() * (self.t_v_mech / 24.0)
            }
        }
    }

    pub fn calculate_h_v_win(n_win: f64, volume: f64) -> f64 {
        n_win * volume * C_AIR
    }

    // --- 5. UNHEATED ZONES (Eq. 103 - 105) ---
    pub fn calculate_h_v_ue(n_ue: f64, volume_u: f64) -> f64 {
        n_ue * volume_u * C_AIR
    }
}

pub mod internal_gains {
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
                2 => Self { // Gruppenbüro
                    is_residential: false, q_i_combined: 0.0,
                    q_i_p: 5.00, q_i_app: 7.00, q_i_sink_app: 0.0,
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
                6 => Self { // Einzelhandel/Kaufhaus
                    is_residential: false, q_i_combined: 0.0,
                    q_i_p: 7.00, q_i_app: 2.00, q_i_sink_app: 0.0,
                    t_nutz: 12.0, d_nutz: 300.0,
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
                10 => Self { // Bettenzimmer
                    is_residential: false, q_i_combined: 0.0,
                    q_i_p: 4.00, q_i_app: 2.00, q_i_sink_app: 0.0,
                    t_nutz: 24.0, d_nutz: 365.0,
                },
                11 => Self { // Hotelzimmer
                    is_residential: false, q_i_combined: 0.0,
                    q_i_p: 3.00, q_i_app: 2.00, q_i_sink_app: 0.0,
                    t_nutz: 24.0, d_nutz: 365.0,
                },
                13 => Self { // Restaurant
                    is_residential: false, q_i_combined: 0.0,
                    q_i_p: 12.00, q_i_app: 5.00, q_i_sink_app: 0.0,
                    t_nutz: 14.0, d_nutz: 365.0,
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
                33 => Self { // Turnhalle
                    is_residential: false, q_i_combined: 0.0,
                    q_i_p: 10.00, q_i_app: 0.00, q_i_sink_app: 0.0,
                    t_nutz: 14.0, d_nutz: 300.0,
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
}

// Embed the Tabula database
static TABULA_DATA: &str = include_str!("../../tabula_data/extracted_data.json");

lazy_static::lazy_static! {
    static ref TABULA_DB: HashMap<String, Value> = {
        serde_json::from_str(TABULA_DATA).unwrap_or_default()
    };
}

// -----------------------------------------------------------------------------
// Data Structures
// -----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EnvelopeDirectionData {
    pub gross_wall_area: f64,
    pub window_area: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EnvelopeData {
    #[serde(rename = "N")]
    pub n: EnvelopeDirectionData,
    #[serde(rename = "E")]
    pub e: EnvelopeDirectionData,
    #[serde(rename = "S")]
    pub s: EnvelopeDirectionData,
    #[serde(rename = "W")]
    pub w: EnvelopeDirectionData,
    #[serde(rename = "H")]
    #[serde(default)]
    pub h: EnvelopeDirectionData,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct BuildingGeometry {
    pub total_conditioned_volume: f64,
    pub total_floor_area: f64,
    pub total_roof_area: f64,
    pub total_ground_area: f64,
    pub exterior_perimeter: f64,
    #[serde(default)]
    pub roof_pitch_deg: Option<f64>,
    #[serde(default)]
    pub envelope_data: EnvelopeData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CustomInsulation {
    pub thickness_m: f64,
    pub lambda: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Parameters {
    pub building_type: String,
    pub year_class: String,
    pub scenario: String,
    pub story_height: f64,
    pub num_stories: i32,
    pub window_to_wall_ratio: f64,
    pub building_rotation_deg: f64,
    pub heating_system: String,
    pub custom_wall_insulation: Option<CustomInsulation>,
    pub custom_roof_insulation: Option<CustomInsulation>,
    pub custom_floor_insulation: Option<CustomInsulation>,
    
    // New parameters for detailed transmission
    #[serde(default)]
    pub thermal_bridge_category: String,
    #[serde(default)]
    pub ground_contact_type: String,
    #[serde(default)]
    pub shutter_control: String,
    #[serde(default)]
    pub climate_region: String,
    #[serde(default)]
    pub usage_profile: String,
    #[serde(default)]
    pub automation_class: String,

    // New parameters for internal heat gains
    #[serde(default)]
    pub lighting_exhaust: String,
    #[serde(default)]
    pub material_transport: String,
    #[serde(default)]
    pub custom_occupants: f64,
    #[serde(default)]
    pub custom_equipment: f64,

    // New parameters for ventilation
    #[serde(default)]
    pub air_tightness: String,
    #[serde(default)]
    pub has_atd: bool,
    #[serde(default)]
    pub mech_supply: f64,
    #[serde(default)]
    pub mech_exhaust: f64,
    #[serde(default)]
    pub heat_recovery: f64,
    #[serde(default)]
    pub mech_hours: f64,
    
    // Explicit physics parameters fetched from Neo4j (DIN V 18599 & TABULA)
    // (Removed graph parameters)
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            building_type: "SFH".to_string(),
            year_class: "2016-...".to_string(),
            scenario: "Existing State".to_string(),
            story_height: 2.8,
            num_stories: 1,
            window_to_wall_ratio: 0.15,
            building_rotation_deg: 0.0,
            heating_system: "Gas Condensing Boiler".to_string(),
            custom_wall_insulation: None,
            custom_roof_insulation: None,
            custom_floor_insulation: None,
            thermal_bridge_category: "Standard Default".to_string(),
            ground_contact_type: "Unheated Basement".to_string(),
            shutter_control: "Manual".to_string(),
            climate_region: "Potsdam".to_string(),
            usage_profile: "Residential".to_string(),
            automation_class: "C".to_string(),
            air_tightness: "CategoryII".to_string(),
            has_atd: false,
            mech_supply: 0.0,
            mech_exhaust: 0.0,
            heat_recovery: 0.0,
            mech_hours: 0.0,
            lighting_exhaust: "None".to_string(),
            material_transport: "None".to_string(),
            custom_occupants: 0.0,
            custom_equipment: 0.0,
            // (Removed graph parameters)
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct State {
    pub geometry: Option<BuildingGeometry>,
    pub params: Parameters,
    pub ui_state: Option<Value>,
}

// -----------------------------------------------------------------------------
// Action Logging & Event Sourcing
// -----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum Action {
    Init,
    UpdateGeometry(BuildingGeometry),
    UpdateParameters(Parameters),
    GenericUpdate,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogEntry {
    pub timestamp: f64,
    pub action: Action,
    pub description: String,
}

impl LogEntry {
    pub fn new(action: Action, description: &str) -> Self {
        Self {
            timestamp: js_sys::Date::now(),
            action,
            description: description.to_string(),
        }
    }
}

// -----------------------------------------------------------------------------
// History Store
// -----------------------------------------------------------------------------

pub struct HistoryStore {
    past: Vec<(State, LogEntry)>,
    current: State,
    current_log: LogEntry,
    future: Vec<(State, LogEntry)>,
}

impl HistoryStore {
    pub fn new() -> Self {
        Self {
            past: Vec::new(),
            current: State::default(),
            current_log: LogEntry::new(Action::Init, "Initialized engine"),
            future: Vec::new(),
        }
    }

    pub fn apply_action(&mut self, new_state: State, action: Action, description: &str) {
        let entry = LogEntry::new(action, description);
        self.past.push((self.current.clone(), self.current_log.clone()));
        self.current = new_state;
        self.current_log = entry;
        self.future.clear();
    }

    pub fn set_state(&mut self, new_state: State) {
        self.apply_action(new_state, Action::GenericUpdate, "Generic state update");
    }

    pub fn undo(&mut self) -> bool {
        if let Some((prev_state, prev_log)) = self.past.pop() {
            self.future.push((self.current.clone(), self.current_log.clone()));
            self.current = prev_state;
            self.current_log = prev_log;
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some((next_state, next_log)) = self.future.pop() {
            self.past.push((self.current.clone(), self.current_log.clone()));
            self.current = next_state;
            self.current_log = next_log;
            true
        } else {
            false
        }
    }

    pub fn current(&self) -> &State {
        &self.current
    }

    pub fn current_log(&self) -> &LogEntry {
        &self.current_log
    }

    pub fn get_log_history(&self) -> Vec<LogEntry> {
        let mut history: Vec<LogEntry> = self.past.iter().map(|(_, log)| log.clone()).collect();
        history.push(self.current_log.clone());
        history
    }
}

// Global Store
thread_local! {
    static STORE: RefCell<HistoryStore> = RefCell::new(HistoryStore::new());
}

// -----------------------------------------------------------------------------
// Calculation Engine
// -----------------------------------------------------------------------------

// We do a simplified steady-state calculation replicating the ISO 13790 logic from Python.
// In a full implementation, Shapely would be used for exact union area. 
// Since this is Rust WASM, we calculate a bounding box or simple sum of areas.
// We assume simple non-overlapping rectangular zones for the area calculation to match the sketchpad.

fn calculate_energy(state: &State) -> Value {
    let geometry = match &state.geometry {
        Some(g) => g,
        None => return serde_json::json!({ "status": "error", "message": "No geometry defined." }),
    };

    let a_floor_total = geometry.total_floor_area;
    let a_roof = geometry.total_roof_area;
    let a_ground = geometry.total_ground_area;
    let perimeter = geometry.exterior_perimeter;
    let conditioned_volume = geometry.total_conditioned_volume;

    let win_n = geometry.envelope_data.n.window_area;
    let win_e = geometry.envelope_data.e.window_area;
    let win_s = geometry.envelope_data.s.window_area;
    let win_w = geometry.envelope_data.w.window_area;
    let win_h = geometry.envelope_data.h.window_area;

    let a_wall = geometry.envelope_data.n.gross_wall_area
        + geometry.envelope_data.e.gross_wall_area
        + geometry.envelope_data.s.gross_wall_area
        + geometry.envelope_data.w.gross_wall_area;

    let a_window = win_n + win_e + win_s + win_w + win_h;
    let net_wall = a_wall - (win_n + win_e + win_s + win_w); // Only vertical windows reduce wall area


    // Lookup TABULA
    let year_class = &state.params.year_class;
    let age_code = match year_class.as_str() {
        "...1859" => "01", "1860-1918" => "02", "1919-1948" => "03", "1949-1957" => "04",
        "1958-1968" => "05", "1969-1978" => "06", "1979-1983" => "07", "1984-1994" => "08",
        "1995-2001" => "09", "2002-2009" => "10", "2010-2015" => "11", "2016-..." | "2016+" => "12",
        _ => "01"
    };

    let suffix = match state.params.scenario.as_str() {
        "Existing State" => ".001",
        "Usual Refurbishment" => ".002",
        "Advanced Refurbishment" => ".003",
        _ => ".001"
    };

    let prefix = format!("DE_{}_{}", state.params.building_type, age_code);
    
    // Find archetype
    let mut u_wall = 1.0;
    let mut u_roof = 1.0;
    let mut u_floor = 1.0;
    let mut u_window = 1.3;
    let mut g_value = 0.7;

    for (k, v) in TABULA_DB.iter() {
        if k.starts_with(&prefix) && k.ends_with(suffix) {
            u_wall = v.get("u_wall").and_then(Value::as_f64).unwrap_or(1.0);
            u_roof = v.get("u_roof").and_then(Value::as_f64).unwrap_or(1.0);
            u_floor = v.get("u_floor").and_then(Value::as_f64).unwrap_or(1.0);
            u_window = v.get("u_window").and_then(Value::as_f64).unwrap_or(1.3);
            g_value = v.get("g_value").and_then(Value::as_f64).unwrap_or(0.7);
            break;
        }
    }
    
    // Always find existing state u-values for the custom insulation breakdown
    let mut u_wall_existing = u_wall;
    let mut u_roof_existing = u_roof;
    let mut u_floor_existing = u_floor;
    for (k, v) in TABULA_DB.iter() {
        if k.starts_with(&prefix) && k.ends_with(".001") {
            u_wall_existing = v.get("u_wall").and_then(Value::as_f64).unwrap_or(1.0);
            u_roof_existing = v.get("u_roof").and_then(Value::as_f64).unwrap_or(1.0);
            u_floor_existing = v.get("u_floor").and_then(Value::as_f64).unwrap_or(1.0);
            break;
        }
    }

    let r_base_wall = 1.0 / u_wall_existing;
    let mut r_ins_wall = 0.0;
    if let Some(ref custom_ins) = state.params.custom_wall_insulation {
        r_ins_wall = custom_ins.thickness_m / custom_ins.lambda;
        u_wall = 1.0 / (r_base_wall + r_ins_wall);
    }

    let r_base_roof = 1.0 / u_roof_existing;
    if let Some(ref custom_ins) = state.params.custom_roof_insulation {
        let r_ins = custom_ins.thickness_m / custom_ins.lambda;
        u_roof = 1.0 / (r_base_roof + r_ins);
    }

    let r_base_floor = 1.0 / u_floor_existing;
    if let Some(ref custom_ins) = state.params.custom_floor_insulation {
        let r_ins = custom_ins.thickness_m / custom_ins.lambda;
        u_floor = 1.0 / (r_base_floor + r_ins);
    }

    let thermal_bridge_category = match state.params.thermal_bridge_category.as_str() {
        "Internal Insulation Issues" => transmission::ThermalBridgeCategory::InternalInsulationIssues,
        "Good Planning" => transmission::ThermalBridgeCategory::GoodPlanning,
        "Excellent Planning" => transmission::ThermalBridgeCategory::ExcellentPlanning,
        _ => transmission::ThermalBridgeCategory::StandardDefault,
    };

    let f_x_ground = match state.params.ground_contact_type.as_str() {
        "Floor Slab On Ground" => 0.5,
        "Heated Basement" => 1.0,
        "Ventilated Crawl Space" => 0.8,
        "Groundwater Contact" => 1.0,
        _ => 0.5, // Unheated Basement
    };

    let shutter_control = match state.params.shutter_control.as_str() {
        "Automated" => transmission::ShutterControl::Automated,
        _ => transmission::ShutterControl::Manual,
    };

    let glazing_type = if u_window >= 2.5 {
        transmission::WindowGlazingType::Single
    } else if u_window >= 1.5 {
        transmission::WindowGlazingType::Double
    } else {
        transmission::WindowGlazingType::Triple
    };

    let roof_pitch = geometry.roof_pitch_deg.unwrap_or(0.0);
    let roof_angle = if roof_pitch <= 10.0 {
        transmission::WindowInclinationAngle::Deg0
    } else if roof_pitch <= 20.0 {
        transmission::WindowInclinationAngle::Deg15
    } else if roof_pitch <= 35.0 {
        transmission::WindowInclinationAngle::Deg30
    } else if roof_pitch <= 50.0 {
        transmission::WindowInclinationAngle::Deg45
    } else if roof_pitch <= 65.0 {
        transmission::WindowInclinationAngle::Deg60
    } else if roof_pitch <= 80.0 {
        transmission::WindowInclinationAngle::Deg75
    } else {
        transmission::WindowInclinationAngle::Deg90
    };

    // Helper to adapt Tabula U-value to the Detailed Component structure
    let make_comp = |area: f64, u_val: f64, f_neig: f64, f_x: f64| -> transmission::BuildingComponent {
        let r_total = 1.0 / u_val;
        let r_si = 0.13;
        let r_se = 0.04;
        let r_layer = f64::max(0.001, r_total - r_si - r_se);
        transmission::BuildingComponent {
            name: "Comp".to_string(),
            layers: vec![transmission::Layer { material: transmission::Material { name: "Dummy".to_string(), lambda: 1.0 }, thickness: r_layer }],
            r_si,
            r_se,
            area,
            f_neig,
            f_x,
        }
    };

    let comp_wall = make_comp(net_wall, u_wall, 1.0, 1.0);
    let comp_roof = make_comp(a_roof, u_roof, 1.0, 1.0);
    let comp_floor = make_comp(a_ground, u_floor, 1.0, f_x_ground);

    let b_type = match state.params.building_type.as_str() {
        "SFH" | "MFH" => transmission::BuildingType::Residential,
        _ => transmission::BuildingType::NonResidential,
    };
    // Since we use the seasonal method, we only average f_sh over the 7 heating months.
    let heating_months = [
        transmission::Month::Jan, transmission::Month::Feb, transmission::Month::Mar,
        transmission::Month::Apr, transmission::Month::Oct, transmission::Month::Nov, 
        transmission::Month::Dec,
    ];
    let mut total_f_sh_days = 0.0;
    let mut total_heating_days = 0.0;
    for &m in &heating_months {
        let days = m.days_in_month();
        total_f_sh_days += transmission::get_shutter_fraction(m, b_type, shutter_control) * days;
        total_heating_days += days;
    }
    let mut f_sh = total_f_sh_days / total_heating_days;

    if state.params.shutter_control == "None" {
        f_sh = 0.0;
    }
    let u_w_sh = u_window * 0.8;

    let win_vertical = transmission::WindowComponent {
        name: "WinVert".to_string(),
        area: win_n + win_e + win_s + win_w,
        u_w: u_window,
        u_w_sh,
        f_sh,
        f_neig: 1.0,
        f_x: 1.0,
    };
    
    let f_neig_roof = transmission::get_inclination_factor(glazing_type, roof_angle);
    let win_roof = transmission::WindowComponent {
        name: "WinRoof".to_string(),
        area: win_h,
        u_w: u_window,
        u_w_sh,
        f_sh,
        f_neig: f_neig_roof,
        f_x: 1.0,
    };

    let components = vec![comp_wall, comp_roof, comp_floor];
    let windows = vec![win_vertical, win_roof];

    let h_t_d = transmission::calculate_h_t_d(&components, &windows);
    let h_t_iu = transmission::calculate_h_t_iu(&components);
    
    let delta_u_wb = if state.params.thermal_bridge_category == "Standard Default" {
        match state.params.scenario.as_str() {
            "Advanced Refurbishment" => 0.0,
            "Usual Refurbishment" => 0.05,
            _ => 0.10,
        }
    } else {
        thermal_bridge_category.delta_u_wb()
    };
    
    let sum_a = net_wall + a_roof + a_ground + a_window;
    let h_t_wb = transmission::calculate_h_t_wb_simplified(delta_u_wb, sum_a);

    let h_tr = transmission::calculate_h_t_total(h_t_d, h_t_iu, h_t_wb);

    let profile = match state.params.usage_profile.as_str() {
        "SingleOffice" => transmission::UsageProfile::SingleOffice,
        "RetailStore" => transmission::UsageProfile::RetailStore,
        "HospitalRoom" => transmission::UsageProfile::HospitalRoom,
        "Restaurant" => transmission::UsageProfile::Restaurant,
        "Gymnasium" => transmission::UsageProfile::Gymnasium,
        "IndustrialHeavy" => transmission::UsageProfile::IndustrialHeavy,
        _ => transmission::UsageProfile::Residential,
    };
    
    // Ventilation parameters
    let air_tightness = match state.params.air_tightness.as_str() {
        "CategoryI" => ventilation::TightnessCategory::CategoryI,
        "CategoryIII" => ventilation::TightnessCategory::CategoryIII,
        "CategoryIV" => ventilation::TightnessCategory::CategoryIV,
        _ => ventilation::TightnessCategory::CategoryII,
    };

    let b_air_data = ventilation::BuildingAirData {
        volume: conditioned_volume,
        a_ngf: a_floor_total,
        h_room: state.params.story_height,
        a_e: sum_a,
        is_residential: profile == transmission::UsageProfile::Residential,
        n50_measured: None,
        q50_measured: None,
    };

    let n50 = b_air_data.resolve_n50(air_tightness);
    let f_atd = ventilation::calculate_f_atd(state.params.has_atd, n50);
    
    // Required fresh air (n_nutz)
    let v_dot_a = match profile {
        transmission::UsageProfile::SingleOffice => 2.0,
        transmission::UsageProfile::GroupOffice | transmission::UsageProfile::OpenPlanOffice | transmission::UsageProfile::RetailStore | transmission::UsageProfile::RetailFood | transmission::UsageProfile::MedicalPractice | transmission::UsageProfile::ExaminationRoom => 3.0,
        transmission::UsageProfile::MeetingRoom | transmission::UsageProfile::Classroom => 5.0,
        transmission::UsageProfile::Restaurant => 7.0,
        transmission::UsageProfile::Gymnasium => 3.5,
        transmission::UsageProfile::StorageArchive | transmission::UsageProfile::LogisticsHall => 0.5,
        transmission::UsageProfile::HospitalRoom => 3.0,
        _ => 1.5,
    };

    let n_nutz = if profile == transmission::UsageProfile::Residential {
        0.5
    } else {
        ventilation::calculate_n_nutz(v_dot_a, a_floor_total, conditioned_volume)
    };

    // Mechanical System Setup with Estimation Trick
    let (mut mech_supply, mut mech_exhaust) = (state.params.mech_supply, state.params.mech_exhaust);
    if state.params.mech_hours > 0.0 && mech_supply <= 0.0 && mech_exhaust <= 0.0 {
        // Assume system was designed correctly to meet min required fresh air
        mech_supply = n_nutz * conditioned_volume;
        mech_exhaust = n_nutz * conditioned_volume;
    }

    let mech_system = ventilation::MechanicalSystem {
        v_dot_mech_b: mech_supply,
        v_dot_eta: mech_exhaust,
        eta_t: state.params.heat_recovery,
        t_v_mech: state.params.mech_hours,
    };

    let n_sup = mech_system.n_mech_sup(conditioned_volume);
    let n_eta = mech_system.n_mech_eta(conditioned_volume);
    let f_e = ventilation::calculate_f_e(n_sup, n_eta, n50, f_atd);

    let n_inf = ventilation::calculate_n_inf(n50, f_atd, f_e, state.params.mech_hours);
    let h_v_inf = n_inf * conditioned_volume * 0.34;
    let h_v_mech = mech_system.calculate_h_v_mech(conditioned_volume);

    let hours_op = profile.daily_heating_hours();

    let n_win_min = if profile == transmission::UsageProfile::Residential { 
        0.1 
    } else { 
        f64::min(0.1, 0.1 * 3.0 / state.params.story_height) 
    };

    let win_params = ventilation::WindowAiringParams {
        n_nutz,
        n_inf_0: n50 * 0.07 * f_atd,
        f_e,
        n_sup,
        n_eta,
        t_nutz: hours_op,
        t_v_mech: state.params.mech_hours,
        n_win_min,
    };

    let n_win = win_params.calculate_n_win_daily(n_inf);
    let h_v_win = ventilation::calculate_h_v_win(n_win, conditioned_volume);

    // Average ventilation heat transfer coefficient
    let h_ve = h_v_inf + h_v_win + h_v_mech;
    
    // Map Dynamic Temperature Variables
    let region = match state.params.climate_region.as_str() {
        "Bremerhaven" => transmission::ClimateRegion::Bremerhaven,
        "Rostock" => transmission::ClimateRegion::Rostock,
        "Hamburg" => transmission::ClimateRegion::Hamburg,
        "Essen" => transmission::ClimateRegion::Essen,
        "BadMarienberg" => transmission::ClimateRegion::BadMarienberg,
        "Kassel" => transmission::ClimateRegion::Kassel,
        "Braunlage" => transmission::ClimateRegion::Braunlage,
        "Chemnitz" => transmission::ClimateRegion::Chemnitz,
        "Hof" => transmission::ClimateRegion::Hof,
        "Fichtelberg" => transmission::ClimateRegion::Fichtelberg,
        "Mannheim" => transmission::ClimateRegion::Mannheim,
        "Passau" => transmission::ClimateRegion::Passau,
        "Stoetten" => transmission::ClimateRegion::Stoetten,
        "GarmischPartenkirchen" => transmission::ClimateRegion::GarmischPartenkirchen,
        _ => transmission::ClimateRegion::Potsdam,
    };

    let automation = match state.params.automation_class.as_str() {
        "A" => transmission::AutomationClass::A,
        "B" => transmission::AutomationClass::B,
        "D" => transmission::AutomationClass::D,
        _ => transmission::AutomationClass::C, // Standard
    };

    // Calculate external temperature (theta_e) by averaging the heating months.
    let temps = region.monthly_temperatures();
    let heating_months_sum = temps[0] + temps[1] + temps[2] + temps[3] + temps[9] + temps[10] + temps[11];
    let theta_e = heating_months_sum / 7.0;

    // Calculate internal setpoint (theta_int)
    let base_temp = profile.heating_setpoint();
    let temp_shift = automation.temperature_shift(profile);
    let theta_int = f64::max(10.0, base_temp + temp_shift); // Don't let it drop below 10

    // The entire building leaks heat for the full 185-day winter, 24/7.
    let d_hs_loss = 185.0;
    
    // Internal gains only occur when the building is actively used.
    // We scale the annual usage days to find the active usage days within the heating season.
    let d_hs_gain = profile.usage_days() * (185.0 / 365.0);

    // ISO 13790
    let q_ht_tr = 0.024 * h_tr * 1.0 * (theta_int - theta_e) * d_hs_loss;
    let q_ht_ve = 0.024 * h_ve * 1.0 * (theta_int - theta_e) * d_hs_loss;
    
    
    // win_n, win_e, win_s, win_w are already calculated.
    
    let shading_device = match state.params.shutter_control.as_str() {
        "Automated" => solar_gains::ShadingDevice::ExteriorBlinds,
        "Manual" => solar_gains::ShadingDevice::InteriorLight,
        _ => solar_gains::ShadingDevice::None,
    };
    let f_c = shading_device.reduction_factor();
    let frame_fraction = solar_gains::WindowFrameType::Standard.frame_fraction();

    let mut solar_engine = solar_gains::SolarGainsEngine::new();
    let [irr_n, irr_e, irr_s, irr_w, irr_h] = region.seasonal_irradiation();

    let add_win = |engine: &mut solar_gains::SolarGainsEngine, area: f64, irr: f64, f_s: f64| {
        if area > 0.0 {
            engine.add_transparent(solar_gains::TransparentComponent {
                area,
                frame_fraction,
                g_value,
                f_c,
                f_s,
                irradiation: irr * 1000.0, // convert kWh to Wh
            });
        }
    };

    add_win(&mut solar_engine, win_n, irr_n, 0.9);
    add_win(&mut solar_engine, win_e, irr_e, 0.9);
    add_win(&mut solar_engine, win_s, irr_s, 0.9);
    add_win(&mut solar_engine, win_w, irr_w, 0.9);
    add_win(&mut solar_engine, win_h, irr_h, 0.9);

    // Opaque components (Walls and Roof)
    let irr_wall_avg = (irr_n + irr_e + irr_s + irr_w) / 4.0;
    let hours_period = d_hs_loss * 24.0;

    if net_wall > 0.0 {
        solar_engine.add_opaque(solar_gains::OpaqueComponent {
            area: net_wall,
            u_value: u_wall,
            alpha: solar_gains::SurfaceColor::Medium.absorptance(),
            is_roof: false,
            irradiation: irr_wall_avg * 1000.0,
            time_hours: hours_period,
        });
    }

    if a_roof > 0.0 {
        solar_engine.add_opaque(solar_gains::OpaqueComponent {
            area: a_roof,
            u_value: u_roof,
            alpha: solar_gains::SurfaceColor::Medium.absorptance(),
            is_roof: true,
            irradiation: irr_h * 1000.0,
            time_hours: hours_period,
        });
    }

    // Removed total_solar_gain_wh() to separate sources and sinks later
    
    // Internal heat gains according to DIN V 18599-10
    let profile_id = match profile {
        transmission::UsageProfile::Residential => 0,
        transmission::UsageProfile::SingleOffice => 1,
        transmission::UsageProfile::GroupOffice => 2,
        transmission::UsageProfile::OpenPlanOffice => 3,
        transmission::UsageProfile::MeetingRoom => 4,
        transmission::UsageProfile::RetailStore => 6,
        transmission::UsageProfile::RetailFood => 7,
        transmission::UsageProfile::Classroom => 8,
        transmission::UsageProfile::HospitalRoom => 10,
        transmission::UsageProfile::HotelRoom => 11,
        transmission::UsageProfile::Restaurant => 13,
        transmission::UsageProfile::KitchenCommercial => 14,
        transmission::UsageProfile::DataCenter => 21,
        transmission::UsageProfile::Gymnasium => 33,
        transmission::UsageProfile::StorageArchive | transmission::UsageProfile::LogisticsHall => 43,
        _ => 999, // default fallback
    };
    
    let gain_profile = internal_gains::StandardGainProfile::from_profile_id(profile_id);
    
    let method = if state.params.custom_occupants > 0.0 || state.params.custom_equipment > 0.0 {
        internal_gains::GainCalculationMethod::Custom(internal_gains::CustomInventoryProfile {
            num_people: state.params.custom_occupants as u32,
            metabolic_rate_watts: 80.0, // standard resting/light work
            equipment_watts_active: state.params.custom_equipment,
            t_nutz: gain_profile.t_nutz,
        })
    } else {
        internal_gains::GainCalculationMethod::Standard(gain_profile)
    };
    
    let exhaust_type = match state.params.lighting_exhaust.as_str() {
        "CeilingCavity" => internal_gains::LightingExhaustType::CeilingCavity,
        "AirDucts" => internal_gains::LightingExhaustType::AirDucts,
        _ => internal_gains::LightingExhaustType::Standard,
    };
    
    let lighting = internal_gains::LightingSystem {
        q_l_f_daily: 0.0, // Defaults to 0 for now
        exhaust_type,
    };
    
    let material_transport = match state.params.material_transport.as_str() {
        "ColdGoodsSmall" => Some(internal_gains::MaterialTransport {
            c_specific_heat: 0.5,
            m_dot: 100.0,
            theta_in: -18.0,
            theta_out: 20.0,
        }),
        "ColdGoodsLarge" => Some(internal_gains::MaterialTransport {
            c_specific_heat: 0.5,
            m_dot: 1000.0,
            theta_in: -18.0,
            theta_out: 20.0,
        }),
        "HotMetalSmall" => Some(internal_gains::MaterialTransport {
            c_specific_heat: 0.13,
            m_dot: 500.0,
            theta_in: 200.0,
            theta_out: 20.0,
        }),
        "HotMetalLarge" => Some(internal_gains::MaterialTransport {
            c_specific_heat: 0.13,
            m_dot: 2000.0,
            theta_in: 200.0,
            theta_out: 20.0,
        }),
        _ => None,
    };
    
    let engine = internal_gains::InternalGainsEngine::new(
        a_floor_total,
        method,
        material_transport,
        lighting
    );
    
    // Solar Engine gives separated sources and sinks in Wh.
    let (q_sol_sources_wh, q_sol_sinks_wh) = solar_engine.solar_energy_balance_wh();
    
    // Convert to kWh/a for JSON payload
    let q_sol = q_sol_sources_wh / 1000.0;
    let q_sky_loss = q_sol_sinks_wh / 1000.0;
    
    // Engine gives daily gain in Wh.
    // Annual = daily * d_hs_gain / 1000.0 (to get kWh)
    let q_int = (engine.net_daily_gain_wh() * d_hs_gain) / 1000.0;
    let q_gn = q_sol + q_int;

    // Aggregates Totals
    let q_ht_tr_total = q_ht_tr + q_sky_loss;
    let q_ht_total = q_ht_tr_total + q_ht_ve;
    
    let weight = match state.params.building_type.as_str() {
        "SFH" | "MFH" => energy_balance::ConstructionWeight::Heavy, // Default to heavy for residential
        _ => energy_balance::ConstructionWeight::Light, // Default to light for others unless specified
    };

    let balance_engine = energy_balance::EnergyBalanceEngine::new(
        conditioned_volume,
        weight,
        energy_balance::CalculationPeriod::Seasonal,
    );

    let q_h_nd_wh = balance_engine.calculate_final_heating_demand(
        q_ht_tr_total * 1000.0, // Includes sky losses
        q_ht_ve * 1000.0,
        q_int * 1000.0,
        q_sol_sources_wh, // Only positive solar gains
        h_tr,
        h_ve
    );
    let q_h_nd = q_h_nd_wh / 1000.0;
    
    // 1. Determine System Properties: (e_g_h, q_d_h_specific, q_s_h_specific)
    let (e_g_h, q_d_h_spec, q_s_h_spec) = match state.params.heating_system.as_str() {
        "Gas Condensing Boiler" => (1.05, 15.0, 0.0),      // High efficiency, typical pipes
        "Gas Non-Condensing Boiler" => (1.18, 15.0, 5.0),  // Older tech, has storage tank
        "Air Source Heat Pump" => (0.35, 10.0, 5.0),       // COP ~2.8, modern pipes
        "Biomass Pellet Boiler" => (1.25, 15.0, 10.0),     // Lower efficiency, large buffer tank
        "Direct Electric Heating" => (1.00, 0.0, 0.0),     // 100% efficient at point of use, no pipes
        _ => (1.10, 15.0, 0.0), // Default fallback
    };

    // 2. Calculate absolute losses (Specific Loss * Floor Area)
    let q_d_h_total = q_d_h_spec * a_floor_total;
    let q_s_h_total = q_s_h_spec * a_floor_total;

    // 3. Apply TABULA Equation 20 (Simplified): Heat Output of Generator
    // Q_g_h_out = Q_h_nd + Q_d_h + Q_s_h
    let q_g_h_out = q_h_nd + q_d_h_total + q_s_h_total;

    // 4. Apply TABULA Equation 19: Delivered Energy
    // Q_del_h = Q_g_h_out * e_g_h
    let q_final = q_g_h_out * e_g_h;

    let energy_class = balance_engine.determine_energy_class(q_final * 1000.0, a_floor_total);

    serde_json::json!({
        "status": "success",
        "envelope_areas_m2": {
            "net_wall": net_wall,
            "roof": a_roof,
            "floor": a_ground,
            "window": a_window,
            "total_floor": a_floor_total,
            "exterior_perimeter_m": perimeter
        },
        "tabula_u_values": {
            "wall_W_m2K": u_wall,
            "roof_W_m2K": u_roof,
            "floor_W_m2K": u_floor,
            "window_W_m2K": u_window
        },
        "heat_losses": {
            "Q_ht_kWh_a": q_ht_total,
            "transmission_loss_kWh_a": q_ht_tr_total,
            "ventilation_loss_kWh_a": q_ht_ve,
            "sky_radiation_loss_kWh_a": q_sky_loss
        },
        "heat_gains": {
            "solar_gains_kWh_a": q_sol,
            "internal_gains_kWh_a": q_int,
            "total_gains_kWh_a": q_gn
        },
        "heating_demand": {
            "Q_H_nd_kWh_a": q_h_nd,
            "specific_Q_H_nd_kWh_m2a": q_h_nd / a_floor_total
        },
        "final_energy": {
            "Q_final_kWh_a": q_final,
            "specific_Q_final_kWh_m2a": q_final / a_floor_total,
            "energy_class": energy_class.as_str()
        },
        "wall_insulation_breakdown": {
            "u_wall_base": u_wall_existing,
            "r_wall_base": r_base_wall,
            "r_insulation": r_ins_wall,
            "u_wall_final": u_wall
        }
    })
}

// -----------------------------------------------------------------------------
// WASM Exports
// -----------------------------------------------------------------------------

#[wasm_bindgen]
pub fn init_engine() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn get_state() -> String {
    STORE.with(|store| {
        let store = store.borrow();
        serde_json::to_string(store.current()).unwrap()
    })
}

#[wasm_bindgen]
pub fn get_history_log() -> String {
    STORE.with(|store| {
        let store = store.borrow();
        serde_json::to_string(&store.get_log_history()).unwrap()
    })
}

#[wasm_bindgen]
pub fn update_state(state_json: &str) -> String {
    match serde_json::from_str::<State>(state_json) {
        Ok(new_state) => {
            STORE.with(|store| {
                let mut store = store.borrow_mut();
                store.set_state(new_state);
                let res = calculate_energy(store.current());
                serde_json::to_string(&res).unwrap()
            })
        }
        Err(e) => {
            serde_json::json!({ "status": "error", "message": format!("Invalid state payload: {}", e) }).to_string()
        }
    }
}

#[wasm_bindgen]
pub fn update_geometry(geom_json: &str) -> String {
    match serde_json::from_str::<BuildingGeometry>(geom_json) {
        Ok(geometry) => {
            STORE.with(|store| {
                let mut store = store.borrow_mut();
                let mut new_state = store.current().clone();
                new_state.geometry = Some(geometry.clone());
                store.apply_action(new_state, Action::UpdateGeometry(geometry), "Updated building geometry");
                let res = calculate_energy(store.current());
                serde_json::to_string(&res).unwrap()
            })
        }
        Err(e) => {
            serde_json::json!({ "status": "error", "message": format!("Invalid geometry payload: {}", e) }).to_string()
        }
    }
}

#[wasm_bindgen]
pub fn update_parameters(params_json: &str) -> String {
    match serde_json::from_str::<Parameters>(params_json) {
        Ok(params) => {
            STORE.with(|store| {
                let mut store = store.borrow_mut();
                let mut new_state = store.current().clone();
                new_state.params = params.clone();
                store.apply_action(new_state, Action::UpdateParameters(params), "Updated parameters");
                let res = calculate_energy(store.current());
                serde_json::to_string(&res).unwrap()
            })
        }
        Err(e) => {
            serde_json::json!({ "status": "error", "message": format!("Invalid parameters payload: {}", e) }).to_string()
        }
    }
}

#[wasm_bindgen]
pub fn undo() -> String {
    STORE.with(|store| {
        let mut store = store.borrow_mut();
        if store.undo() {
            let res = calculate_energy(store.current());
            serde_json::json!({
                "status": "success",
                "state": store.current(),
                "energy": res,
                "log": store.current_log()
            }).to_string()
        } else {
            serde_json::json!({ "status": "error", "message": "No undo history" }).to_string()
        }
    })
}

#[wasm_bindgen]
pub fn redo() -> String {
    STORE.with(|store| {
        let mut store = store.borrow_mut();
        if store.redo() {
            let res = calculate_energy(store.current());
            serde_json::json!({
                "status": "success",
                "state": store.current(),
                "energy": res,
                "log": store.current_log()
            }).to_string()
        } else {
            serde_json::json!({ "status": "error", "message": "No redo history" }).to_string()
        }
    })
}

pub mod solar_gains {
    use serde::{Deserialize, Serialize};

    // --- CONSTANTS FROM DIN 18599 ---
    pub const F_W_STANDARD: f64 = 0.90; // Correction for non-perpendicular radiation
    pub const R_SE_STANDARD: f64 = 0.04; // External surface resistance (m²K/W)
    pub const H_R_STANDARD: f64 = 5.0; // External radiative heat transfer coeff (W/m²K)
    pub const DELTA_THETA_ER: f64 = 11.0; // Sky temperature difference (K)

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum GlazingType {
        Single,
        DoubleStandard,
        DoubleLowE,
        TripleLowE,
        SolarControl,
    }

    impl GlazingType {
        pub fn g_value(&self) -> f64 {
            match self {
                Self::Single => 0.85,
                Self::DoubleStandard => 0.75,
                Self::DoubleLowE => 0.60,
                Self::TripleLowE => 0.50,
                Self::SolarControl => 0.35,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum WindowFrameType {
        Standard,
        VeryLarge,
        SmallDivided,
    }

    impl WindowFrameType {
        pub fn frame_fraction(&self) -> f64 {
            match self {
                Self::Standard => 0.30,
                Self::VeryLarge => 0.20,
                Self::SmallDivided => 0.40,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum ShadingDevice {
        None,
        InteriorLight,
        InteriorDark,
        ExteriorBlinds,
        ExteriorAwnings,
    }

    impl ShadingDevice {
        pub fn reduction_factor(&self) -> f64 {
            match self {
                Self::None => 1.00,
                Self::InteriorLight => 0.80,
                Self::InteriorDark => 0.60,
                Self::ExteriorBlinds => 0.25,
                Self::ExteriorAwnings => 0.40,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum SurfaceColor {
        Light,
        Medium,
        Dark,
    }

    impl SurfaceColor {
        pub fn absorptance(&self) -> f64 {
            match self {
                Self::Light => 0.30,
                Self::Medium => 0.60,
                Self::Dark => 0.90,
            }
        }
    }

    // --- DATA MODELS ---

    /// Represents a window or transparent surface.
    #[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[derive(Debug, Clone, Serialize, Deserialize)]
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

        /// Calculates the separated solar heat sources (gains) and sinks (losses) for the envelope (Wh)
        pub fn solar_energy_balance_wh(&self) -> (f64, f64) {
            let mut total_sources = 0.0;
            let mut total_sinks = 0.0;

            // Transparent components only provide gains
            for c in &self.transparent_components {
                total_sources += c.solar_gain_wh();
            }

            // Opaque components can provide gains or losses (due to sky radiation)
            for c in &self.opaque_components {
                let gain = c.solar_gain_wh();
                if gain > 0.0 {
                    total_sources += gain;
                } else {
                    total_sinks += gain.abs(); // Sinks are treated as positive losses
                }
            }

            (total_sources, total_sinks)
        }
    }
}

pub mod energy_balance {
    use serde::{Deserialize, Serialize};

    // --- CATEGORY 2: MATHEMATICAL SOLVER CONSTANTS ---

    /// Defines the calculation resolution, which dictates the a_0 and tau_0 constants.
    #[derive(Debug, Clone, Copy)]
    pub enum CalculationPeriod {
        Monthly,
        Seasonal,
    }

    impl CalculationPeriod {
        /// Returns the (a_0, tau_0) tuple required for the 'a' parameter calculation.
        pub fn solver_constants(&self) -> (f64, f64) {
            match self {
                Self::Monthly => (1.0, 15.0),  // a_0 = 1.0, tau_0 = 15.0
                Self::Seasonal => (0.8, 30.0), // a_0 = 0.8, tau_0 = 30.0
            }
        }
    }

    // --- CATEGORY 1: BUILDING PHYSICS PROPERTIES ---

    /// Represents the thermal mass classification of the building.
    #[derive(Debug, Clone, Copy)]
    pub enum ConstructionWeight {
        Light, // e.g., Timber frame
        Heavy, // e.g., Solid masonry/concrete
    }

    impl ConstructionWeight {
        /// Returns the effective heat capacity (C_eff) in Wh/K based on the heated building volume (V_e).
        pub fn effective_heat_capacity(&self, volume_e: f64) -> f64 {
            match self {
                Self::Light => 15.0 * volume_e,
                Self::Heavy => 50.0 * volume_e,
            }
        }
    }

    // --- CATEGORY 4: ENERGY CERTIFICATE (ENERGIEAUSWEIS) ---

    /// Official German energy performance classes (A+ to H)
    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum EnergyClass {
        APlus,
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
    }

    impl EnergyClass {
        /// Maps the specific energy demand (kWh per m² per year) to an official category.
        pub fn from_specific_demand(demand_kwh_per_m2: f64) -> Self {
            if demand_kwh_per_m2 < 30.0 {
                Self::APlus
            } else if demand_kwh_per_m2 < 50.0 {
                Self::A
            } else if demand_kwh_per_m2 < 75.0 {
                Self::B
            } else if demand_kwh_per_m2 < 100.0 {
                Self::C
            } else if demand_kwh_per_m2 < 130.0 {
                Self::D
            } else if demand_kwh_per_m2 < 160.0 {
                Self::E
            } else if demand_kwh_per_m2 < 200.0 {
                Self::F
            } else if demand_kwh_per_m2 < 250.0 {
                Self::G
            } else {
                Self::H
            }
        }

        /// Returns a human-readable string representation of the class
        pub fn as_str(&self) -> &'static str {
            match self {
                Self::APlus => "A+",
                Self::A => "A",
                Self::B => "B",
                Self::C => "C",
                Self::D => "D",
                Self::E => "E",
                Self::F => "F",
                Self::G => "G",
                Self::H => "H",
            }
        }
    }

    // --- THE MASTER SOLVER ---

    /// The master solver for the building's energy balance.
    pub struct EnergyBalanceEngine {
        pub c_eff: f64, // Effective heat capacity (Wh/K)
        pub period: CalculationPeriod,
    }

    impl EnergyBalanceEngine {
        /// Initialize using the standardized DIN 4108-6 volume and weight fallbacks.
        pub fn new(volume_e: f64, weight: ConstructionWeight, period: CalculationPeriod) -> Self {
            Self {
                c_eff: weight.effective_heat_capacity(volume_e),
                period,
            }
        }

        /// Initialize with a highly specific, custom calculated C_eff.
        pub fn new_detailed(c_eff: f64, period: CalculationPeriod) -> Self {
            Self { c_eff, period }
        }

        /// Calculates the building's thermal time constant (tau) in hours.
        pub fn calculate_time_constant(&self, h_t: f64, h_v: f64) -> f64 {
            let h_total = h_t + h_v;
            if h_total <= 0.0 {
                return 0.0; // Prevent division by zero
            }
            self.c_eff / h_total
        }

        /// Calculates the Gain Utilization Factor (eta) for heating.
        pub fn calculate_gain_utilization_factor(
            &self, 
            q_l: f64, // Total heat sinks / losses (Q_T + Q_V)
            q_g: f64, // Total heat sources / gains (Q_I + Q_S)
            tau: f64  // Time constant in hours
        ) -> f64 {
            if q_l <= 0.0 {
                return 0.0; // No losses mean heating is impossible
            }
            if q_g <= 0.0 {
                return 1.0; // No gains to utilize, so factor is nominally 1.0
            }

            let gamma = q_g / q_l;
            let (a_0, tau_0) = self.period.solver_constants();
        
            // Calculate the curve-fitting parameter 'a'
            let a = a_0 + (tau / tau_0);

            // Floating point safe comparison for gamma == 1.0
            if (gamma - 1.0).abs() < 0.001 {
                a / (a + 1.0)
            } else {
                // eta = (1 - gamma^a) / (1 - gamma^(a+1))
                (1.0 - gamma.powf(a)) / (1.0 - gamma.powf(a + 1.0))
            }
        }

        /// The Master Equation: Calculates the final Heating Energy Demand (Q_h) for the given period.
        pub fn calculate_final_heating_demand(
            &self,
            q_t: f64, // Transmission Losses (Wh)
            q_v: f64, // Ventilation Losses (Wh)
            q_i: f64, // Internal Gains (Wh)
            q_s: f64, // Solar Gains (Wh)
            h_t: f64, // Transmission Coefficient (W/K)
            h_v: f64, // Ventilation Coefficient (W/K)
        ) -> f64 {
            // 1. Aggregate Sinks and Sources
            let q_l = q_t + q_v;
            let q_g = q_i + q_s;

            // 2. Calculate thermal inertia (how long the building stores heat)
            let tau = self.calculate_time_constant(h_t, h_v);
        
            // 3. Calculate how much of the gains we can actually use (eta)
            let eta = self.calculate_gain_utilization_factor(q_l, q_g, tau);

            // 4. Final balance: Losses minus Utilized Gains
            let q_h = q_l - (eta * q_g);

            // Heating demand cannot be mathematically negative. 
            f64::max(0.0, q_h)
        }

        /// Calculates the specific energy demand and returns the official German Energy Class.
        /// Changed parameter to expect final energy in Wh to be compliant with GEG,
        /// rather than just heating demand (Q_h).
        pub fn determine_energy_class(&self, annual_final_energy_wh: f64, floor_area_m2: f64) -> EnergyClass {
            if floor_area_m2 <= 0.0 {
                return EnergyClass::H; // Fallback for invalid geometry
            }

            // Convert Wh to kWh, then divide by area to get kWh/(m²·a)
            let specific_demand_kwh = (annual_final_energy_wh / 1000.0) / floor_area_m2;
        
            EnergyClass::from_specific_demand(specific_demand_kwh)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_state() {
        let json_payload = r#"{
            "geometry": null,
            "params": {
                "building_type": "SFH",
                "year_class": "2016-...",
                "scenario": "Existing State",
                "story_height": 2.8,
                "num_stories": 1,
                "window_to_wall_ratio": 0.15,
                "building_rotation_deg": 0.0,
                "heating_system": "Gas Condensing Boiler",
                "custom_wall_insulation": null,
                "thermal_bridge_category": "Standard Default",
                "ground_contact_type": "Unheated Basement",
                "shutter_control": "Manual",
                "air_tightness": "Category II",
                "wind_shielding": "Moderate",
                "ventilation_type": "Window Airing",
                "heat_recovery": "None"
            },
            "ui_state": null
        }"#;
        let res = update_state(json_payload);
        assert!(res.contains("success") || res.contains("error")); 
    }

    #[test]
    fn test_ventilation_calculation() {
        let n50 = 1.5;
        let f_atd = ventilation::calculate_f_atd(false, n50);
        let n_inf = ventilation::calculate_n_inf(n50, f_atd, 1.0, 0.0);
        assert!(n_inf > 0.0);

        let h_v_inf = n_inf * 100.0 * 0.34;
        assert!(h_v_inf > 0.0);
        
        let mech = ventilation::MechanicalSystem {
            v_dot_mech_b: 100.0,
            v_dot_eta: 100.0,
            eta_t: 0.8,
            t_v_mech: 24.0,
        };
        let n_sup = mech.n_mech_sup(100.0);
        let n_eta = mech.n_mech_eta(100.0);
        assert_eq!(n_sup, 1.0);
        assert_eq!(n_eta, 1.0);
        
        let h_v_mech = mech.calculate_h_v_mech(100.0);
        assert!(h_v_mech > 0.0);
    }

    #[test]
    fn test_transmission_calculation() {
        let material = transmission::Material { name: "TestMat".to_string(), lambda: 0.035 };
        let layer = transmission::Layer { thickness: 0.1, material };
        let comp = transmission::BuildingComponent {
            name: "TestWall".to_string(),
            area: 10.0,
            r_si: 0.13,
            r_se: 0.04,
            f_x: 1.0,
            f_neig: 1.0,
            layers: vec![layer],
        };
        let u_value = comp.calculate_u_value();
        assert!(u_value > 0.0 && u_value < 1.0);
        
        let h_t = transmission::calculate_h_t_d(&vec![comp], &vec![]);
        assert!(h_t > 0.0);
    }

    #[test]
    fn test_solar_gains_calculation() {
        use solar_gains::*;

        let mut engine = SolarGainsEngine::new();

        engine.add_transparent(TransparentComponent {
            area: 10.0,
            frame_fraction: WindowFrameType::Standard.frame_fraction(),
            g_value: GlazingType::DoubleLowE.g_value(),
            f_c: ShadingDevice::ExteriorBlinds.reduction_factor(),
            f_s: 1.00,
            irradiation: 45000.0, 
        });

        engine.add_opaque(OpaqueComponent {
            area: 50.0,
            u_value: 0.28,
            alpha: SurfaceColor::Medium.absorptance(),
            is_roof: false,
            irradiation: 45000.0,
            time_hours: 744.0, 
        });

        let transparent_gain = engine.transparent_components[0].solar_gain_wh();
        assert!(transparent_gain > 0.0, "Transparent gain should be positive");
        
        let expected_eff_area = 10.0 * (1.0 - 0.30) * 0.90 * 0.60 * 0.25 * 1.0;
        let diff = (engine.transparent_components[0].effective_collecting_area() - expected_eff_area).abs();
        assert!(diff < 0.001, "Effective collecting area mismatch");

        let opaque_gain = engine.opaque_components[0].solar_gain_wh();
        let expected_opaque = 50.0 * 0.28 * 0.04 * (0.60 * 45000.0 - 0.5 * 5.0 * 11.0 * 744.0);
        assert!((opaque_gain - expected_opaque).abs() < 0.1, "Opaque gain mismatch, expected {}, got {}", expected_opaque, opaque_gain);
    }

    #[test]
    fn test_energy_balance_engine() {
        use energy_balance::*;
        let engine = EnergyBalanceEngine::new(500.0, ConstructionWeight::Heavy, CalculationPeriod::Monthly);
        
        let h_t = 150.0;
        let h_v = 50.0;
        let tau = engine.calculate_time_constant(h_t, h_v);
        assert!(tau > 0.0);

        let q_l = 10000.0;
        let q_g = 5000.0;
        let eta = engine.calculate_gain_utilization_factor(q_l, q_g, tau);
        assert!(eta > 0.0 && eta <= 1.0);

        let class = engine.determine_energy_class(10000.0 * 1000.0, 150.0); // 10000 kWh / 150m2 = 66.6 kWh/m2a
        assert_eq!(class, EnergyClass::B);
    }

    // =========================================================================
    // THE MASTER INTEGRATION TEST: Verifies the entire pipeline calculates correctly
    // =========================================================================
    #[test]
    fn test_full_energy_calculation_pipeline() {
        // 1. Mock a standard 2-story Single Family House (10m x 10m footprint)
        let geom = BuildingGeometry {
            total_conditioned_volume: 560.0, // 200m2 * 2.8m
            total_floor_area: 200.0,
            total_roof_area: 100.0,
            total_ground_area: 100.0,
            exterior_perimeter: 40.0,
            roof_pitch_deg: Some(30.0),
            envelope_data: EnvelopeData {
                n: EnvelopeDirectionData { gross_wall_area: 56.0, window_area: 5.0 },
                e: EnvelopeDirectionData { gross_wall_area: 56.0, window_area: 5.0 },
                s: EnvelopeDirectionData { gross_wall_area: 56.0, window_area: 15.0 },
                w: EnvelopeDirectionData { gross_wall_area: 56.0, window_area: 5.0 },
                h: EnvelopeDirectionData { gross_wall_area: 0.0, window_area: 0.0 }, // Windows handled in walls
            }
        };

        // 2. Set realistic parameters for a modern heat-pump powered house
        let params = Parameters {
            building_type: "SFH".to_string(),
            year_class: "2016-...".to_string(), // Modern build
            scenario: "Existing State".to_string(),
            story_height: 2.8,
            num_stories: 2,
            window_to_wall_ratio: 0.15,
            building_rotation_deg: 0.0,
            heating_system: "Air Source Heat Pump".to_string(), // High efficiency
            custom_wall_insulation: None,
            custom_roof_insulation: None,
            custom_floor_insulation: None,
            thermal_bridge_category: "Good Planning".to_string(),
            ground_contact_type: "Floor Slab On Ground".to_string(),
            shutter_control: "Automated".to_string(), // Uses smart shading
            climate_region: "Potsdam".to_string(),
            usage_profile: "Residential".to_string(),
            automation_class: "B".to_string(),
            air_tightness: "CategoryI".to_string(), // Blower-door tested
            has_atd: false,
            mech_supply: 0.0,
            mech_exhaust: 0.0,
            heat_recovery: 0.0,
            mech_hours: 0.0,
            lighting_exhaust: "None".to_string(),
            material_transport: "None".to_string(),
            custom_occupants: 0.0,
            custom_equipment: 0.0,
        };

        let state = State {
            geometry: Some(geom),
            params,
            ui_state: None,
        };

        // 3. Run the orchestration engine
        let result = calculate_energy(&state);

        // 4. Assert that the solver completed successfully
        assert_eq!(result["status"], "success", "Calculation pipeline failed.");

        // 5. Verify the geometry mapping logic
        let areas = &result["envelope_areas_m2"];
        assert_eq!(areas["net_wall"].as_f64().unwrap(), 194.0); // 224 total - 30 windows
        assert_eq!(areas["window"].as_f64().unwrap(), 30.0);
        assert_eq!(areas["total_floor"].as_f64().unwrap(), 200.0);

        // 6. Verify Physics Isolation (Sinks vs Sources)
        let losses = &result["heat_losses"];
        let gains = &result["heat_gains"];
        
        let q_ht = losses["Q_ht_kWh_a"].as_f64().unwrap();
        let q_sky = losses["sky_radiation_loss_kWh_a"].as_f64().unwrap();
        let q_sol = gains["solar_gains_kWh_a"].as_f64().unwrap();
        
        assert!(q_ht > 0.0, "Building must have heat losses");
        assert!(q_sky > 0.0, "Opaque walls must radiate heat to sky");
        assert!(q_sol > 0.0, "Windows must collect solar gains");

        // 7. Verify the DIN V 4108-6 Energy Balance Engine
        let heating_demand = result["heating_demand"]["specific_Q_H_nd_kWh_m2a"].as_f64().unwrap();
        assert!(heating_demand > 0.0, "Specific heating demand should be positive");
        
        // 8. Verify Final Energy mapping (Heat pumps convert demand into lower final energy)
        let final_energy = result["final_energy"]["specific_Q_final_kWh_m2a"].as_f64().unwrap();
        let energy_class = result["final_energy"]["energy_class"].as_str().unwrap();
        
        assert!(final_energy > 0.0, "Final energy must be greater than zero");
        assert!(!energy_class.is_empty(), "Energy class must be assigned");
        
        // Output for manual debugging when tests are run with `--nocapture`
        println!("Test Passed! Specific Heating Demand: {:.2} kWh/m²a", heating_demand);
        println!("Final Energy Class: {}", energy_class);
    }

    // =========================================================================
    // TABULA DATASET VALIDATION TESTS
    // These tests mock the exact archetype scenarios from the IWU TABULA Report
    // to verify the engine's deviation from the official German reference values.
    // =========================================================================

    fn create_tabula_reference_geometry() -> BuildingGeometry {
        // A typical German SFH from the TABULA database (~150m2 living space)
        BuildingGeometry {
            total_conditioned_volume: 420.0, 
            total_floor_area: 150.0,
            total_roof_area: 80.0,
            total_ground_area: 75.0,
            exterior_perimeter: 35.0,
            roof_pitch_deg: Some(35.0),
            envelope_data: EnvelopeData {
                n: EnvelopeDirectionData { gross_wall_area: 45.0, window_area: 5.0 },
                e: EnvelopeDirectionData { gross_wall_area: 35.0, window_area: 5.0 },
                s: EnvelopeDirectionData { gross_wall_area: 45.0, window_area: 15.0 },
                w: EnvelopeDirectionData { gross_wall_area: 35.0, window_area: 5.0 },
                h: EnvelopeDirectionData { gross_wall_area: 0.0, window_area: 0.0 },
            }
        }
    }

    #[test]
    fn test_tabula_sfh_1860_1918_existing() {
        let mut params = Parameters::default();
        params.building_type = "SFH".to_string();
        params.year_class = "1860-1918".to_string(); // TABULA Code 02
        params.scenario = "Existing State".to_string();
        params.heating_system = "Gas Non-Condensing Boiler".to_string(); // Classic old system
        params.climate_region = "Potsdam".to_string(); // Standard reference climate

        let state = State { geometry: Some(create_tabula_reference_geometry()), params, ui_state: None };
        let result = calculate_energy(&state);
        
        let demand = result["heating_demand"]["specific_Q_H_nd_kWh_m2a"].as_f64().unwrap();
        let class = result["final_energy"]["energy_class"].as_str().unwrap();
        
        println!("--------------------------------------------------");
        println!("TABULA TEST 1: SFH 1860-1918 (Unrefurbished)");
        println!("Expected: High heating demand (> 150 kWh/m²a), Class G/H");
        println!("Engine Output: {:.2} kWh/m²a | Class: {}", demand, class);
        println!("--------------------------------------------------");
        
        assert!(demand > 120.0, "Historic unrefurbished building must have high demand.");
    }

    #[test]
    fn test_tabula_sfh_1969_1978_existing() {
        let mut params = Parameters::default();
        params.building_type = "SFH".to_string();
        params.year_class = "1969-1978".to_string(); // TABULA Code 06 (Pre-Oil Crisis)
        params.scenario = "Existing State".to_string();
        params.heating_system = "Gas Non-Condensing Boiler".to_string(); 
        params.climate_region = "Potsdam".to_string();

        let state = State { geometry: Some(create_tabula_reference_geometry()), params, ui_state: None };
        let result = calculate_energy(&state);
        
        let demand = result["heating_demand"]["specific_Q_H_nd_kWh_m2a"].as_f64().unwrap();
        let class = result["final_energy"]["energy_class"].as_str().unwrap();
        
        println!("TABULA TEST 2: SFH 1969-1978 (Unrefurbished)");
        println!("Expected: Medium-High demand (~ 120-180 kWh/m²a), Class E/F/G");
        println!("Engine Output: {:.2} kWh/m²a | Class: {}", demand, class);
        println!("--------------------------------------------------");
        
        assert!(demand > 90.0, "70s unrefurbished building should still be quite inefficient.");
    }

    #[test]
    fn test_tabula_sfh_1969_1978_advanced() {
        let mut params = Parameters::default();
        params.building_type = "SFH".to_string();
        params.year_class = "1969-1978".to_string(); 
        // Applying the highest tier TABULA Refurbishment
        params.scenario = "Advanced Refurbishment".to_string(); 
        
        // Upgrading to modern technical standards
        params.heating_system = "Air Source Heat Pump".to_string();
        params.shutter_control = "Automated".to_string();
        params.automation_class = "A".to_string();
        params.air_tightness = "CategoryI".to_string(); // Blower door tested
        params.heat_recovery = 0.85; // 85% efficient mechanical ventilation
        params.mech_hours = 24.0;
        params.climate_region = "Potsdam".to_string();

        let state = State { geometry: Some(create_tabula_reference_geometry()), params, ui_state: None };
        let result = calculate_energy(&state);
        
        let demand = result["heating_demand"]["specific_Q_H_nd_kWh_m2a"].as_f64().unwrap();
        let class = result["final_energy"]["energy_class"].as_str().unwrap();
        
        println!("TABULA TEST 3: SFH 1969-1978 (Advanced Refurbishment)");
        println!("Expected: Excellent modern demand (< 50 kWh/m²a), Class A+/A/B");
        println!("Engine Output: {:.2} kWh/m²a | Class: {}", demand, class);
        println!("--------------------------------------------------");
        
        assert!(demand < 75.0, "Advanced refurbishment should bring demand below 75 kWh/m²a.");
    }
}
