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

    let mut f_sh = transmission::get_shutter_fraction(transmission::Month::Jan, transmission::BuildingType::Residential, shutter_control);
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
    
    // Infiltration
    let is_old = match year_class.as_str() {
        "...1859" | "1860-1918" | "1919-1948" | "1949-1957" | "1958-1968" | "1969-1978" => true,
        _ => false,
    };

    let n_infiltr = match state.params.scenario.as_str() {
        "Advanced Refurbishment" => 0.05,
        "Usual Refurbishment" => 0.1,
        _ => if is_old { 0.4 } else { 0.2 },
    };

    let h_ve = 0.34 * (0.4 + n_infiltr) * conditioned_volume;
    
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

    let profile = match state.params.usage_profile.as_str() {
        "SingleOffice" => transmission::UsageProfile::SingleOffice,
        "RetailStore" => transmission::UsageProfile::RetailStore,
        "HospitalRoom" => transmission::UsageProfile::HospitalRoom,
        "Restaurant" => transmission::UsageProfile::Restaurant,
        "Gymnasium" => transmission::UsageProfile::Gymnasium,
        "IndustrialHeavy" => transmission::UsageProfile::IndustrialHeavy,
        _ => transmission::UsageProfile::Residential,
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

    // Get Heating Days (d_hs)
    let d_hs = profile.usage_days();

    // ISO 13790
    let q_ht = 0.024 * (h_tr + h_ve) * 1.0 * (theta_int - theta_e) * d_hs;
    let q_ht_tr = 0.024 * h_tr * 1.0 * (theta_int - theta_e) * d_hs;
    let q_ht_ve = 0.024 * h_ve * 1.0 * (theta_int - theta_e) * d_hs;
    
    
    // win_n, win_e, win_s, win_w are already calculated.
    
    let sol_factor_vertical = 0.6 * (1.0 - 0.3) * 0.9 * g_value;
    let sol_factor_horizontal = 0.8 * (1.0 - 0.3) * 0.9 * g_value;
    
    let q_sol_n = sol_factor_vertical * win_n * 160.0;
    let q_sol_e = sol_factor_vertical * win_e * 271.0;
    let q_sol_s = sol_factor_vertical * win_s * 392.0;
    let q_sol_w = sol_factor_vertical * win_w * 271.0;
    let q_sol_h = sol_factor_horizontal * win_h * 392.0;
    
    let q_sol = q_sol_n + q_sol_e + q_sol_s + q_sol_w + q_sol_h;
    
    // Internal heat gains according to equation (9)
    let phi_int = 3.0; // average thermal output of internal heat sources [W/m²]
    let q_int = 0.024 * phi_int * d_hs * a_floor_total;
    let q_gn = q_sol + q_int;
    
    let q_h_nd = f64::max(0.0, q_ht - 0.95 * q_gn);
    
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
            "Q_ht_kWh_a": q_ht,
            "transmission_loss_kWh_a": q_ht_tr,
            "ventilation_loss_kWh_a": q_ht_ve
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
            "specific_Q_final_kWh_m2a": q_final / a_floor_total
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
                "shutter_control": "Manual"
            },
            "ui_state": null
        }"#;
        let res = update_state(json_payload);
        println!("{}", res);
        assert!(res.contains("success"));
    }
}
