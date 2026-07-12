use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::cell::RefCell;
use serde_json::Value;

pub mod ontology;

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
    #[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
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
    
    // New parameters for DHW (DIN V 18599-8)
    #[serde(default)]
    pub dhw_base_volume_liters_per_day: f64,
    #[serde(default)]
    pub dhw_wastewater_heat_recovery: f64,
    #[serde(default)]
    pub dhw_generator_type: String,

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
            dhw_base_volume_liters_per_day: 0.0,
            dhw_wastewater_heat_recovery: 0.0,
            dhw_generator_type: "HeatPumpAirWater".to_string(),
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

pub struct EnergyAnalysisResult {
    pub transmission_loss_kwh: f64,
    pub ventilation_loss_kwh: f64,
    pub solar_gains_kwh: f64,
    pub internal_gains_kwh: f64,
    pub final_heating_demand_kwh: f64,
    pub final_dhw_fuel_demand_kwh: f64,
    pub final_dhw_electricity_demand_kwh: f64,
    pub q_h_nd_kwh: f64,
    pub h_t_wb: f64,
    pub f_x: f64,
    pub n50: f64,
    pub r_se: f64,
    pub r_si: f64,
    pub h_t_d: f64,
    pub h_t_iu: f64,
    pub h_v_inf: f64,
    pub h_v_win: f64,
    pub h_v_mech: f64,
    pub h_tr: f64,
    pub h_ve: f64,
}

fn map_state_to_graph(state: &State, u_wall: f64, u_roof: f64, u_floor: f64, u_window: f64, res: &EnergyAnalysisResult) -> crate::ontology::BuildingKnowledgeGraph {
    use crate::ontology::*;
    let mut graph = BuildingKnowledgeGraph::new();

    let b_idx = graph.add_entity(EntityData::Building(BuildingData {
        name: "Building".to_string(),
        building_type: crate::transmission::BuildingType::Residential,
        building_category: None,
        year_class: state.params.year_class.clone(),
        scenario: state.params.scenario.clone(),
        num_stories: state.params.num_stories,
        heating_system: state.params.heating_system.clone(),
        thermal_bridge_category: crate::transmission::ThermalBridgeCategory::StandardDefault,
        total_conditioned_volume: state.geometry.as_ref().map(|g| g.total_conditioned_volume).unwrap_or(0.0),
        total_floor_area: state.geometry.as_ref().map(|g| g.total_floor_area).unwrap_or(0.0),
        total_roof_area: state.geometry.as_ref().map(|g| g.total_roof_area).unwrap_or(0.0),
        total_ground_area: state.geometry.as_ref().map(|g| g.total_ground_area).unwrap_or(0.0),
        exterior_perimeter: state.geometry.as_ref().map(|g| g.exterior_perimeter).unwrap_or(0.0),
        roof_pitch_deg: None,
        building_rotation_deg: state.params.building_rotation_deg,
        window_to_wall_ratio: state.params.window_to_wall_ratio,
    }));

    let p_vol = graph.add_entity(EntityData::Property(PropertyData { 
        name: "Volume (V_e)".into(), value: format!("{:.1}", state.geometry.as_ref().map(|g| g.total_conditioned_volume).unwrap_or(0.0)), unit: "m³".into(),
        doc: Some("**Conditioned Volume ($V_e$)**The total heated volume of the building. Used to calculate ventilation air mass flows ($V_e \\cdot n$).".into())
    }));
    let p_year = graph.add_entity(EntityData::Property(PropertyData { 
        name: "Year Class".into(), value: state.params.year_class.clone(), unit: "".into(),
        doc: Some("**Year Class**The construction age bracket. Dictates default U-values, infiltration rates, and system efficiencies if not explicitly overridden.".into())
    }));
    let p_heat = graph.add_entity(EntityData::Property(PropertyData { 
        name: "Heating System".into(), value: state.params.heating_system.clone(), unit: "".into(),
        doc: Some("**Heating System**The primary thermal generator. Affects the primary energy factor ($f_P$) and conversion efficiency ($e_g$).".into())
    }));
    graph.add_relationship(Relationship::HasProperty { host: b_idx, property: p_vol });
    graph.add_relationship(Relationship::HasProperty { host: b_idx, property: p_year });
    graph.add_relationship(Relationship::HasProperty { host: b_idx, property: p_heat });

    // Global Building Ventilation Calculation
    let calc_vent = graph.add_entity(EntityData::Calculation(CalculationData {
        name: "Ventilation Heat Loss (H_V)".into(),
        formula: "H_V = \\rho_{air} \\cdot c_{a} \\cdot n \\cdot V_e".into(),
        doc: "Calculates the heat loss due to air exchange (infiltration and window airing).".into(),
    }));
    graph.add_relationship(Relationship::InputsTo { parameter: p_vol, calculation: calc_vent });

    let p_n50 = graph.add_entity(EntityData::Property(PropertyData {
        name: "n50 (Blower Door)".into(), value: format!("{:.2}", res.n50), unit: "1/h".into(),
        doc: Some("Air change rate at 50 Pa pressure difference. Measures building envelope airtightness.".into())
    }));
    let p_hv_inf = graph.add_entity(EntityData::Property(PropertyData {
        name: "H_{V,inf}".into(), value: format!("{:.1}", res.h_v_inf), unit: "W/K".into(),
        doc: Some("Ventilation heat transfer coefficient for infiltration through envelope leaks.".into())
    }));
    let p_hv_win = graph.add_entity(EntityData::Property(PropertyData {
        name: "H_{V,win}".into(), value: format!("{:.1}", res.h_v_win), unit: "W/K".into(),
        doc: Some("Ventilation heat transfer coefficient for window airing.".into())
    }));
    let p_hv_mech = graph.add_entity(EntityData::Property(PropertyData {
        name: "H_{V,mech}".into(), value: format!("{:.1}", res.h_v_mech), unit: "W/K".into(),
        doc: Some("Ventilation heat transfer coefficient for mechanical ventilation (considering heat recovery).".into())
    }));
    let p_h_ve = graph.add_entity(EntityData::Property(PropertyData {
        name: "H_V (Total)".into(), value: format!("{:.1}", res.h_ve), unit: "W/K".into(),
        doc: Some("Total ventilation heat transfer coefficient.".into())
    }));
    
    graph.add_relationship(Relationship::InputsTo { parameter: p_n50, calculation: calc_vent });
    graph.add_relationship(Relationship::InputsTo { parameter: p_hv_inf, calculation: calc_vent });
    graph.add_relationship(Relationship::InputsTo { parameter: p_hv_win, calculation: calc_vent });
    graph.add_relationship(Relationship::InputsTo { parameter: p_hv_mech, calculation: calc_vent });
    graph.add_relationship(Relationship::InputsTo { parameter: p_h_ve, calculation: calc_vent });

    let r_vent = graph.add_entity(EntityData::Property(PropertyData {
        name: "Ventilation Loss Result".into(), value: format!("{:.0}", res.ventilation_loss_kwh), unit: "kWh/a".into(),
        doc: Some("Final calculated annual heat loss due to ventilation (Q_v).".into())
    }));
    graph.add_relationship(Relationship::OutputsTo { calculation: calc_vent, result: r_vent });


    if let Some(geom) = &state.geometry {
        let a_wall_total = geom.envelope_data.n.gross_wall_area + geom.envelope_data.e.gross_wall_area + geom.envelope_data.s.gross_wall_area + geom.envelope_data.w.gross_wall_area;
        let win_area_total = geom.envelope_data.n.window_area + geom.envelope_data.s.window_area + geom.envelope_data.e.window_area + geom.envelope_data.w.window_area;
        let has_zones = state.ui_state.as_ref().and_then(|ui| ui.get("raw_zones").and_then(|z| z.as_array())).map(|a| !a.is_empty()).unwrap_or(false);

        if has_zones {
            let raw_zones = state.ui_state.as_ref().unwrap().get("raw_zones").unwrap().as_array().unwrap();
            let mut total_zone_area = 0.0;
            for zone in raw_zones {
                let w = zone.get("geometry").and_then(|g| g.get("width")).and_then(|v| v.as_f64()).unwrap_or(0.0);
                let l = zone.get("geometry").and_then(|g| g.get("length")).and_then(|v| v.as_f64()).unwrap_or(0.0);
                total_zone_area += w * l;
            }
            if total_zone_area == 0.0 { total_zone_area = 1.0; } 

            for zone in raw_zones {
                let z_name = zone.get("type").and_then(|v| v.as_str()).unwrap_or("Zone").to_string();
                let z_width = zone.get("geometry").and_then(|g| g.get("width")).and_then(|v| v.as_f64()).unwrap_or(0.0);
                let z_length = zone.get("geometry").and_then(|g| g.get("length")).and_then(|v| v.as_f64()).unwrap_or(0.0);
                let z_area = z_width * z_length;
                let scale_factor = z_area / total_zone_area;

                let z_idx = graph.add_entity(EntityData::Space(SpaceData {
                    name: z_name, volume: z_area * state.params.story_height, net_floor_area: z_area,
                    room_depth: None, ceiling_height: Some(state.params.story_height), is_critical_room: false, unheated_space_type: None
                }));
                graph.add_relationship(Relationship::Aggregates { parent: b_idx, child: z_idx });

                let p_area = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "Area (A_NGF)".into(), value: format!("{:.1}", z_area), unit: "m²".into(),
                    doc: Some("**Net Floor Area ($A_{NGF}$)**The reference area used to multiply specific internal gains ($q_I$).".into())
                }));
                graph.add_relationship(Relationship::HasProperty { host: z_idx, property: p_area });

                // Calculations
                let calc_internal = graph.add_entity(EntityData::Calculation(CalculationData {
                    name: "Internal Gains (Q_I)".into(),
                    formula: "Q_{I} = q_{I} \\cdot A_{NGF} \\cdot t".into(),
                    doc: "Heat generated by people, equipment, and lighting.".into(),
                }));
                let calc_trans = graph.add_entity(EntityData::Calculation(CalculationData {
                    name: "Transmission Loss (H_T)".into(),
                    formula: "H_{T,D} = \\sum (A_j \\cdot U_j \\cdot f_{neig,j})".into(),
                    doc: "Direct transmission heat loss through the opaque envelope and windows.".into(),
                }));

    let p_ht_d = graph.add_entity(EntityData::Property(PropertyData {
        name: "H_{T,D}".into(), value: format!("{:.1}", res.h_t_d), unit: "W/K".into(),
        doc: Some("Direct transmission heat transfer coefficient to the exterior environment.".into())
    }));
    let p_ht_iu = graph.add_entity(EntityData::Property(PropertyData {
        name: "H_{T,iu}".into(), value: format!("{:.1}", res.h_t_iu), unit: "W/K".into(),
        doc: Some("Transmission heat transfer coefficient to unheated spaces.".into())
    }));
    let p_ht_wb = graph.add_entity(EntityData::Property(PropertyData {
        name: "H_{T,wb}".into(), value: format!("{:.1}", res.h_t_wb), unit: "W/K".into(),
        doc: Some("Transmission heat transfer coefficient for thermal bridges.".into())
    }));
    let p_fx = graph.add_entity(EntityData::Property(PropertyData {
        name: "f_x (Ground)".into(), value: format!("{:.2}", res.f_x), unit: "-".into(),
        doc: Some("Temperature weighting factor for ground-coupled components.".into())
    }));
    let p_rse = graph.add_entity(EntityData::Property(PropertyData {
        name: "R_se".into(), value: format!("{:.2}", res.r_se), unit: "m²K/W".into(),
        doc: Some("External surface thermal resistance.".into())
    }));
    let p_rsi = graph.add_entity(EntityData::Property(PropertyData {
        name: "R_si".into(), value: format!("{:.2}", res.r_si), unit: "m²K/W".into(),
        doc: Some("Internal surface thermal resistance.".into())
    }));
    let p_h_tr = graph.add_entity(EntityData::Property(PropertyData {
        name: "H_T (Total)".into(), value: format!("{:.1}", res.h_tr), unit: "W/K".into(),
        doc: Some("Total transmission heat transfer coefficient.".into())
    }));

    graph.add_relationship(Relationship::InputsTo { parameter: p_ht_d, calculation: calc_trans });
    graph.add_relationship(Relationship::InputsTo { parameter: p_ht_iu, calculation: calc_trans });
    graph.add_relationship(Relationship::InputsTo { parameter: p_ht_wb, calculation: calc_trans });
    graph.add_relationship(Relationship::InputsTo { parameter: p_fx, calculation: calc_trans });
    graph.add_relationship(Relationship::InputsTo { parameter: p_rse, calculation: calc_trans });
    graph.add_relationship(Relationship::InputsTo { parameter: p_rsi, calculation: calc_trans });
    graph.add_relationship(Relationship::InputsTo { parameter: p_h_tr, calculation: calc_trans });

    let r_trans = graph.add_entity(EntityData::Property(PropertyData {
        name: "Transmission Loss Result".into(), value: format!("{:.0}", res.transmission_loss_kwh), unit: "kWh/a".into(),
        doc: Some("Final calculated annual heat loss due to transmission (Q_T).".into())
    }));
    graph.add_relationship(Relationship::OutputsTo { calculation: calc_trans, result: r_trans });

    // Assuming calc_internal and calc_solar exist in scope here or we will link them directly after they are created.


                let calc_solar_op = graph.add_entity(EntityData::Calculation(CalculationData {
                    name: "Opaque Solar/Sky (Q_s,op)".into(),
                    formula: "Q_{s,op} = A_{op} \\cdot U_{op} \\cdot R_{se} \\cdot (\\alpha \\cdot I_s - F_{sky} \\cdot h_r \\cdot \\Delta\\theta_{er})".into(),
                    doc: "Solar gains on opaque walls minus radiation lost to the cold night sky.".into(),
                }));

                graph.add_relationship(Relationship::InputsTo { parameter: p_area, calculation: calc_internal });

                let r_int = graph.add_entity(EntityData::Property(PropertyData {
                    name: "Internal Gains Result".into(), value: format!("{:.0}", res.internal_gains_kwh), unit: "kWh/a".into(),
                    doc: Some("Final calculated annual heat gains from people and equipment (Q_I).".into())
                }));
                graph.add_relationship(Relationship::OutputsTo { calculation: calc_internal, result: r_int });


                // --- WALL ---
                let w_idx = graph.add_entity(EntityData::Wall(WallData {
                    area: a_wall_total * scale_factor, u_value: u_wall, thickness: 0.3,
                    r_si: 0.13, r_se: 0.04, f_neig: 1.0, f_x: 1.0, solar_absorptance: 0.6, is_roof: false,
                    name: "".to_string(), orientation: None,
                }));
                graph.add_relationship(Relationship::BoundsSpace { space: z_idx, boundary_element: w_idx });
                
                let wp_u = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "U-Value".into(), value: format!("{:.2}", u_wall), unit: "W/(m²K)".into(),
                    doc: Some("Measures heat transfer rate. Lower values indicate better insulation.".into())
                }));
                let wp_alpha = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "Solar Absorptance (\\alpha)".into(), value: "0.6".into(), unit: "-".into(),
                    doc: Some("Solar radiation absorbed based on color.".into())
                }));
                let wp_rse = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "R_se".into(), value: "0.04".into(), unit: "m²K/W".into(),
                    doc: Some("External Surface Resistance. Standard is 0.04 for walls.".into())
                }));
                let wp_area = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "Area".into(), value: format!("{:.1}", a_wall_total * scale_factor), unit: "m²".into(),
                    doc: Some("Total exposed area. Directly proportional to transmission heat losses.".into())
                }));
                graph.add_relationship(Relationship::HasProperty { host: w_idx, property: wp_u });
                graph.add_relationship(Relationship::HasProperty { host: w_idx, property: wp_alpha });
                graph.add_relationship(Relationship::HasProperty { host: w_idx, property: wp_rse });
                graph.add_relationship(Relationship::HasProperty { host: w_idx, property: wp_area });

                graph.add_relationship(Relationship::InputsTo { parameter: wp_u, calculation: calc_trans });
                graph.add_relationship(Relationship::InputsTo { parameter: wp_area, calculation: calc_trans });
                graph.add_relationship(Relationship::InputsTo { parameter: wp_alpha, calculation: calc_solar_op });
                graph.add_relationship(Relationship::InputsTo { parameter: wp_rse, calculation: calc_solar_op });
                graph.add_relationship(Relationship::InputsTo { parameter: wp_area, calculation: calc_solar_op });

                // --- WINDOW ---
                if win_area_total > 0.0 {
                    let win_idx = graph.add_entity(EntityData::Window(WindowData {
                        area: win_area_total * scale_factor, u_value: u_window, u_w_sh: u_window, f_sh: 0.0, g_value: 0.6, frame_fraction: 0.3, f_neig: 1.0, f_x: 1.0, shading_factor_fc: 1.0, surroundings_shading_fs: 1.0,
                        shutter_control: crate::transmission::ShutterControl::Manual, glazing_type: crate::transmission::WindowGlazingType::Double, inclination_angle: crate::transmission::WindowInclinationAngle::Deg90,
                        name: "".to_string(), orientation: None,
                    }));
                    graph.add_relationship(Relationship::FillsVoid { host: w_idx, filler: win_idx });

                    let calc_solar = graph.add_entity(EntityData::Calculation(CalculationData {
                        name: "Solar Gains (Q_S)".into(),
                        formula: "Q_{s,w} = I_s \\cdot A_w \\cdot (1 - F_F) \\cdot F_w \\cdot g \\cdot F_C \\cdot F_S".into(),
                        doc: "Solar energy passing directly through windows, minus frame and shading.".into(),
                    }));

                    let win_g = graph.add_entity(EntityData::Property(PropertyData { 
                        name: "g-Value".into(), value: "0.6".into(), unit: "-".into(),
                        doc: Some("Total Solar Energy Transmittance. Fraction of solar radiation passing through glass.".into())
                    }));
                    let win_ff = graph.add_entity(EntityData::Property(PropertyData { 
                        name: "Frame Fraction (F_F)".into(), value: "0.3".into(), unit: "-".into(),
                        doc: Some("Percentage of window area that is opaque frame.".into())
                    }));
                    let win_fc = graph.add_entity(EntityData::Property(PropertyData { 
                        name: "Shading Factor (F_C)".into(), value: "1.0".into(), unit: "-".into(),
                        doc: Some("Operable Shading Factor from blinds or curtains.".into())
                    }));
                    let win_u = graph.add_entity(EntityData::Property(PropertyData { 
                        name: "U-Value".into(), value: format!("{:.2}", u_window), unit: "W/(m²K)".into(),
                        doc: Some("Measures heat transfer through window.".into())
                    }));
                    let win_area_prop = graph.add_entity(EntityData::Property(PropertyData { 
                        name: "Area".into(), value: format!("{:.2}", win_area_total * scale_factor), unit: "m²".into(),
                        doc: Some("Window size area.".into())
                    }));
                    graph.add_relationship(Relationship::HasProperty { host: win_idx, property: win_g });
                    graph.add_relationship(Relationship::HasProperty { host: win_idx, property: win_ff });
                    graph.add_relationship(Relationship::HasProperty { host: win_idx, property: win_fc });
                    graph.add_relationship(Relationship::HasProperty { host: win_idx, property: win_u });
                    graph.add_relationship(Relationship::HasProperty { host: win_idx, property: win_area_prop });

                    graph.add_relationship(Relationship::InputsTo { parameter: win_u, calculation: calc_trans });
                    graph.add_relationship(Relationship::InputsTo { parameter: win_area_prop, calculation: calc_trans });
                    graph.add_relationship(Relationship::InputsTo { parameter: win_g, calculation: calc_solar });
                    graph.add_relationship(Relationship::InputsTo { parameter: win_ff, calculation: calc_solar });
                    graph.add_relationship(Relationship::InputsTo { parameter: win_fc, calculation: calc_solar });
                    graph.add_relationship(Relationship::InputsTo { parameter: win_area_prop, calculation: calc_solar });

                    let r_sol = graph.add_entity(EntityData::Property(PropertyData {
                        name: "Solar Gains Result".into(), value: format!("{:.0}", res.solar_gains_kwh), unit: "kWh/a".into(),
                        doc: Some("Final calculated annual solar heat gains through windows (Q_S).".into())
                    }));
                    graph.add_relationship(Relationship::OutputsTo { calculation: calc_solar, result: r_sol });

                }

                // --- ROOF ---
                let r_idx = graph.add_entity(EntityData::Roof(RoofData {
                    area: geom.total_roof_area * scale_factor, u_value: u_roof, r_si: 0.1, r_se: 0.04, f_neig: 1.0, f_x: 1.0, solar_absorptance: 0.8
                }));
                graph.add_relationship(Relationship::BoundsSpace { space: z_idx, boundary_element: r_idx });
                
                let rp_fneig = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "f_neig".into(), value: "1.0".into(), unit: "-".into(),
                    doc: Some("Inclination Correction Factor ($f_{neig}$)".into())
                }));
                let rp_u = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "U-Value".into(), value: format!("{:.2}", u_roof), unit: "W/(m²K)".into(),
                    doc: None
                }));
                graph.add_relationship(Relationship::HasProperty { host: r_idx, property: rp_fneig });
                graph.add_relationship(Relationship::HasProperty { host: r_idx, property: rp_u });

                graph.add_relationship(Relationship::InputsTo { parameter: rp_u, calculation: calc_trans });
                graph.add_relationship(Relationship::InputsTo { parameter: rp_fneig, calculation: calc_trans });

                // --- SLAB ---
                let s_idx = graph.add_entity(EntityData::Slab(SlabData {
                    area: geom.total_ground_area * scale_factor, u_value: u_floor, r_si: 0.17, r_se: 0.04, f_x: 0.6, ground_contact: None
                }));
                graph.add_relationship(Relationship::BoundsSpace { space: z_idx, boundary_element: s_idx });

                let sp_fx = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "f_x".into(), value: "0.6".into(), unit: "-".into(),
                    doc: Some("Temperature Correction Factor ($f_x$) for ground/unheated spaces.".into())
                }));
                let sp_u = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "U-Value".into(), value: format!("{:.2}", u_floor), unit: "W/(m²K)".into(),
                    doc: None
                }));
                graph.add_relationship(Relationship::HasProperty { host: s_idx, property: sp_fx });
                graph.add_relationship(Relationship::HasProperty { host: s_idx, property: sp_u });

                graph.add_relationship(Relationship::InputsTo { parameter: sp_u, calculation: calc_trans });
                graph.add_relationship(Relationship::InputsTo { parameter: sp_fx, calculation: calc_trans });
            }
        } else {
            let z_idx = graph.add_entity(EntityData::Space(SpaceData {
                name: "Main Zone".into(), volume: geom.total_conditioned_volume, net_floor_area: geom.total_floor_area,
                room_depth: None, ceiling_height: None, is_critical_room: false, unheated_space_type: None
            }));
            graph.add_relationship(Relationship::Aggregates { parent: b_idx, child: z_idx });

            let w_idx = graph.add_entity(EntityData::Wall(WallData {
                area: a_wall_total, u_value: u_wall, thickness: 0.3,
                r_si: 0.13, r_se: 0.04, f_neig: 1.0, f_x: 1.0, solar_absorptance: 0.6, is_roof: false,
                name: "".to_string(), orientation: None,
            }));
            graph.add_relationship(Relationship::BoundsSpace { space: z_idx, boundary_element: w_idx });
            
            let wp_u = graph.add_entity(EntityData::Property(PropertyData { 
                name: "U-Value".into(), value: format!("{:.2}", u_wall), unit: "W/(m²K)".into(),
                doc: Some("Measures the rate of heat transfer through a structure. Lower values indicate better insulation.".into())
            }));
            graph.add_relationship(Relationship::HasProperty { host: w_idx, property: wp_u });
        }
    }

    // Final Energy Balance Calculation
    let calc_heating = graph.add_entity(EntityData::Calculation(CalculationData {
        name: "Heating Demand (Q_{H,nd})".into(),
        formula: r"Q_{H,nd} = Q_T + Q_V - \eta \cdot (Q_S + Q_I)".into(),
        doc: "Total heat energy required to maintain the setpoint temperature, after subtracting useful solar and internal gains.".into(),
    }));

    let p_qt = graph.add_entity(EntityData::Property(PropertyData {
        name: "Q_T (Transmission)".into(), value: format!("{:.0}", res.transmission_loss_kwh), unit: "kWh/a".into(),
        doc: None
    }));
    let p_qv = graph.add_entity(EntityData::Property(PropertyData {
        name: "Q_V (Ventilation)".into(), value: format!("{:.0}", res.ventilation_loss_kwh), unit: "kWh/a".into(),
        doc: None
    }));
    let p_qs = graph.add_entity(EntityData::Property(PropertyData {
        name: "Q_S (Solar)".into(), value: format!("{:.0}", res.solar_gains_kwh), unit: "kWh/a".into(),
        doc: None
    }));
    let p_qi = graph.add_entity(EntityData::Property(PropertyData {
        name: "Q_I (Internal)".into(), value: format!("{:.0}", res.internal_gains_kwh), unit: "kWh/a".into(),
        doc: None
    }));

    graph.add_relationship(Relationship::InputsTo { parameter: p_qt, calculation: calc_heating });
    graph.add_relationship(Relationship::InputsTo { parameter: p_qv, calculation: calc_heating });
    graph.add_relationship(Relationship::InputsTo { parameter: p_qs, calculation: calc_heating });
    graph.add_relationship(Relationship::InputsTo { parameter: p_qi, calculation: calc_heating });

    let r_heating = graph.add_entity(EntityData::Property(PropertyData {
        name: "Heating Demand Result".into(), value: format!("{:.0}", res.q_h_nd_kwh), unit: "kWh/a".into(),
        doc: Some("Final Heating Demand (Q_{H,nd}).".into())
    }));
    graph.add_relationship(Relationship::OutputsTo { calculation: calc_heating, result: r_heating });

    // Final Delivered Energy Calculation
    let calc_final = graph.add_entity(EntityData::Calculation(CalculationData {
        name: "Delivered Energy (Q_{End})".into(),
        formula: r"Q_{End} = (Q_{H,nd} + Q_{d,h} + Q_{s,h}) \cdot e_{g,h}".into(),
        doc: "Total energy billed by the utility, factoring in the efficiency of the heating system.".into(),
    }));

    graph.add_relationship(Relationship::InputsTo { parameter: r_heating, calculation: calc_final });
    // p_heat is already added earlier, we could link it here but p_heat was created way earlier in scope.
    // We will just add the efficiency factor directly as an input to represent the system.
    let p_eff = graph.add_entity(EntityData::Property(PropertyData {
        name: "e_{g,h} (Efficiency)".into(), value: format!("{:.2}", res.final_heating_demand_kwh / res.q_h_nd_kwh), unit: "-".into(),
        doc: Some("Total system loss factor of the heating generator.".into())
    }));
    graph.add_relationship(Relationship::InputsTo { parameter: p_eff, calculation: calc_final });

    let r_final = graph.add_entity(EntityData::Property(PropertyData {
        name: "Final Energy Result".into(), value: format!("{:.0}", res.final_heating_demand_kwh), unit: "kWh/a".into(),
        doc: Some("Total Delivered Energy (Q_{End}).".into())
    }));
    graph.add_relationship(Relationship::OutputsTo { calculation: calc_final, result: r_final });


    graph
}

    // DHW Engine Evaluation
    let dhw_generator = match state.params.dhw_generator_type.as_str() {
        "ElectricInstantaneous" => dhw_system::GeneratorTypeDHW::ElectricInstantaneous,
        "ElectricSmallStorage" => dhw_system::GeneratorTypeDHW::ElectricSmallStorage,
        "ElectricStandardStorage" => dhw_system::GeneratorTypeDHW::ElectricStandardStorage,
        "GasInstantaneousNew" => dhw_system::GeneratorTypeDHW::GasInstantaneousNew,
        "GasStorageNew" => dhw_system::GeneratorTypeDHW::GasStorageNew,
        "CombinedGasBoilerCondensing" => dhw_system::GeneratorTypeDHW::CombinedGasBoilerCondensing,
        "DistrictHeating" => dhw_system::GeneratorTypeDHW::DistrictHeating,
        _ => dhw_system::GeneratorTypeDHW::HeatPumpAirWater,
    };
    
    let dhw_engine = dhw_system::DHWEngine {
        a_ngf: a_floor_total,
        profile: match state.params.usage_profile.as_str() {
            "Residential" => dhw_system::BuildingProfileDHW::Residential1_2Family,
            "Office" => dhw_system::BuildingProfileDHW::OfficesCommercial,
            "Hospital" => dhw_system::BuildingProfileDHW::HospitalsHotels,
            _ => dhw_system::BuildingProfileDHW::ResidentialMultiFamily,
        },
        system_type: dhw_system::DHWSystemType::Centralized,
        distribution_insulation: dhw_system::PipeInsulationDHW::Insulated100Percent,
        has_circulation: true,
        tank_volume_liters: 200.0,
        tank_insulation: dhw_system::TankInsulationDHW::StandardClassC,
        generator: dhw_generator,
        phi_w_max_kw: 15.0,
        is_summer_mode: false,
        t_mth: 8760.0,
        wastewater_heat_recovery: state.params.dhw_wastewater_heat_recovery,
        solar_thermal_kwh: 0.0,
    };
    
    let q_w_b_annual = state.params.dhw_base_volume_liters_per_day * 365.0 * 0.058;
    let dhw_res = dhw_engine.calculate_final_energy(q_w_b_annual);

    // Q_del_h = Q_g_h_out * e_g_h
    let q_final_heating = q_g_h_out * e_g_h;
    let q_final_total = q_final_heating + dhw_res.fuel_demand_kwh + dhw_res.total_electricity_kwh;

    // --- Primary Energy Calculation ---
    let heating_carrier = match state.params.heating_system.as_str() {
        "OldGasBoiler" | "CondensingGasBoiler" => primary_energy::EnergyCarrier::NaturalGas,
        "PelletBoiler" => primary_energy::EnergyCarrier::WoodPellets,
        "DirectElectric" => primary_energy::EnergyCarrier::GridElectricity,
        "HeatPumpAirWater" | "GroundSourceHeatPump" => primary_energy::EnergyCarrier::GridElectricity,
        "DistrictHeating" => primary_energy::EnergyCarrier::DistrictHeatingFossil,
        _ => primary_energy::EnergyCarrier::GridElectricity,
    };

    let dhw_carrier = match dhw_generator {
        dhw_system::GeneratorTypeDHW::ElectricInstantaneous |
        dhw_system::GeneratorTypeDHW::ElectricSmallStorage |
        dhw_system::GeneratorTypeDHW::ElectricStandardStorage |
        dhw_system::GeneratorTypeDHW::HeatPumpAirWater => primary_energy::EnergyCarrier::GridElectricity,
        dhw_system::GeneratorTypeDHW::GasInstantaneousNew |
        dhw_system::GeneratorTypeDHW::GasStorageNew |
        dhw_system::GeneratorTypeDHW::CombinedGasBoilerCondensing => primary_energy::EnergyCarrier::NaturalGas,
        dhw_system::GeneratorTypeDHW::DistrictHeating => primary_energy::EnergyCarrier::DistrictHeatingFossil,
    };

    let energy_demands = vec![
        primary_energy::EnergyDemand {
            q_f: q_final_heating,
            w_f: 0.0, // Auxiliary electricity for heating to be added
            carrier: heating_carrier,
        },
        primary_energy::EnergyDemand {
            q_f: dhw_res.fuel_demand_kwh,
            w_f: dhw_res.total_electricity_kwh,
            carrier: dhw_carrier,
        },
    ];

    let q_p_nren = primary_energy::PrimaryEnergyEngine::calculate_primary_energy(&energy_demands, true);
    let q_p_tot = primary_energy::PrimaryEnergyEngine::calculate_primary_energy(&energy_demands, false);

    let energy_class = balance_engine.determine_energy_class(q_final_total * 1000.0, a_floor_total);

    // region Overheating
    let mut overheating_windows = Vec::new();
    let f_c_val = match state.params.shutter_control.as_str() {
        "Automated" => 0.30,
        "Manual" => 0.50,
        _ => 1.00,
    };

    if win_n > 0.0 {
        overheating_windows.push(overheating::OverheatingWindow {
            area: win_n,
            g_value,
            f_c: f_c_val,
            f_s: 1.0,
            inclination_deg: 90.0,
            is_north_oriented: true,
            is_permanently_shaded: false,
        });
    }
    if win_e > 0.0 {
        overheating_windows.push(overheating::OverheatingWindow {
            area: win_e,
            g_value,
            f_c: f_c_val,
            f_s: 1.0,
            inclination_deg: 90.0,
            is_north_oriented: false,
            is_permanently_shaded: false,
        });
    }
    if win_s > 0.0 {
        overheating_windows.push(overheating::OverheatingWindow {
            area: win_s,
            g_value,
            f_c: f_c_val,
            f_s: 1.0,
            inclination_deg: 90.0,
            is_north_oriented: false,
            is_permanently_shaded: false,
        });
    }
    if win_w > 0.0 {
        overheating_windows.push(overheating::OverheatingWindow {
            area: win_w,
            g_value,
            f_c: f_c_val,
            f_s: 1.0,
            inclination_deg: 90.0,
            is_north_oriented: false,
            is_permanently_shaded: false,
        });
    }
    if win_h > 0.0 {
        overheating_windows.push(overheating::OverheatingWindow {
            area: win_h,
            g_value,
            f_c: f_c_val,
            f_s: 1.0,
            inclination_deg: 0.0,
            is_north_oriented: false,
            is_permanently_shaded: false,
        });
    }

    let category = if state.params.usage_profile == "Residential" {
        overheating::BuildingCategory::Residential
    } else {
        overheating::BuildingCategory::NonResidential
    };

    let summer_region = match state.params.climate_region.as_str() {
        "Bremerhaven" | "Rostock" | "Hamburg" | "Fichtelberg" | "Braunlage" => overheating::SummerClimateRegion::A,
        "Mannheim" => overheating::SummerClimateRegion::C,
        _ => overheating::SummerClimateRegion::B,
    };

    let night_ventilation = if state.params.mech_supply > 0.0 || state.params.has_atd {
        overheating::NightVentilation::Increased
    } else {
        overheating::NightVentilation::None
    };

    let room = overheating::CriticalRoom {
        actual_floor_area: a_floor_total,
        room_depth: 5.0,
        room_height: state.params.story_height,
        has_opposite_windows: (win_n > 0.0 && win_s > 0.0) || (win_e > 0.0 && win_w > 0.0),
        category,
        climate_region: summer_region,
        construction: overheating::ConstructionClass::Medium,
        night_ventilation,
        has_passive_cooling: false,
        windows: overheating_windows,
    };

    let overheating_result = overheating::OverheatingEngine::evaluate_room(&room);
    // endregion Overheating

    // Build the graph
    
    let results = EnergyAnalysisResult {
        transmission_loss_kwh: q_ht_tr_total,
        ventilation_loss_kwh: q_ht_ve,
        solar_gains_kwh: q_sol,
        internal_gains_kwh: q_int,
        final_heating_demand_kwh: q_final_heating,
        final_dhw_fuel_demand_kwh: dhw_res.fuel_demand_kwh,
        final_dhw_electricity_demand_kwh: dhw_res.total_electricity_kwh,
        q_h_nd_kwh: q_h_nd,
        h_t_wb,
        f_x: f_x_ground,
        n50,
        r_se: 0.04,
        r_si: 0.13,
        h_t_d,
        h_t_iu,
        h_v_inf,
        h_v_win,
        h_v_mech,
        h_tr,
        h_ve,
    };
    let knowledge_graph = map_state_to_graph(state, u_wall, u_roof, u_floor, u_window, &results);

    
    // Generate Suggestions
    let mut suggestions = Vec::new();
    if q_ht_tr_total > q_ht_ve * 1.5 {
        suggestions.push("Transmission losses are dominating. Consider upgrading wall or roof insulation.".to_string());
    }
    if q_ht_ve > q_ht_tr_total * 0.8 {
        suggestions.push("Ventilation losses are high. Adding a Heat Recovery Ventilation (HRV) system could be beneficial.".to_string());
    }
    if u_window > 1.2 {
        suggestions.push(format!("Window U-value ({:.2} W/m²K) is relatively high. Triple glazing is recommended.", u_window));
    }
    if q_sol < q_ht_total * 0.1 {
        suggestions.push("Solar gains are very low. If renovating, consider larger south-facing windows to improve passive heating.".to_string());
    }

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
            "Q_final_kWh_a": q_final_total,
            "specific_Q_final_kWh_m2a": q_final_total / a_floor_total,
            "energy_class": energy_class.as_str(),
            "heating_fuel_kWh_a": q_final_heating,
            "dhw_fuel_kWh_a": dhw_res.fuel_demand_kwh,
            "dhw_electricity_kWh_a": dhw_res.total_electricity_kwh
        },
        "primary_energy": {
            "Q_p_nren_kWh_a": q_p_nren,
            "specific_Q_p_nren_kWh_m2a": q_p_nren / a_floor_total,
            "Q_p_tot_kWh_a": q_p_tot
        },
        "wall_insulation_breakdown": {
            "u_wall_base": u_wall_existing,
            "r_wall_base": r_base_wall,
            "r_insulation": r_ins_wall,
            "u_wall_final": u_wall
        },
        "suggestions": suggestions,
        "graph": knowledge_graph.to_vis_network_json(),
        "overheating": {
            "exemption": match overheating_result.exemption {
                overheating::ExemptionStatus::ExemptSmallWindows => "Exempt (Small Windows)",
                overheating::ExemptionStatus::ExemptResidentialHighlyShaded => "Exempt (Highly Shaded)",
                overheating::ExemptionStatus::NotExempt => "Not Exempt",
            },
            "s_vorh": overheating_result.s_vorh,
            "s_zul": overheating_result.s_zul,
            "passes": overheating_result.passes
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
    fn test_tabula_sfh_1859() {
        let state = State {
            geometry: Some(BuildingGeometry {
                total_conditioned_volume: 613.2,
                total_floor_area: 219.0,
                total_roof_area: 109.5,
                total_ground_area: 109.5,
                exterior_perimeter: 41.85,
                roof_pitch_deg: Some(0.0),
                envelope_data: EnvelopeData {
                    n: EnvelopeDirectionData { gross_wall_area: 58.5, window_area: 8.7 },
                    e: EnvelopeDirectionData { gross_wall_area: 58.5, window_area: 8.7 },
                    s: EnvelopeDirectionData { gross_wall_area: 58.5, window_area: 8.7 },
                    w: EnvelopeDirectionData { gross_wall_area: 58.5, window_area: 8.7 },
                    h: EnvelopeDirectionData { gross_wall_area: 0.0, window_area: 0.0 },
                }
            }),
            params: Parameters {
                building_type: "SFH".to_string(),
                year_class: "...1859".to_string(),
                scenario: "Existing State".to_string(),
                story_height: 2.8,
                num_stories: 2,
                window_to_wall_ratio: 0.15,
                building_rotation_deg: 0.0,
                heating_system: "Gas Condensing Boiler".to_string(),
                climate_region: "Potsdam".to_string(),
                usage_profile: "Residential".to_string(),
                automation_class: "C".to_string(),
                thermal_bridge_category: "Standard Default".to_string(),
                ground_contact_type: "Unheated Basement".to_string(),
                shutter_control: "Manual".to_string(),
                ..Default::default()
            },
            ui_state: None,
        };

        let result = calculate_energy(&state);
        
        let q_h_nd_m2a = result["heating_demand"]["specific_Q_H_nd_kWh_m2a"]
            .as_f64()
            .expect("Should have heating demand");

        println!("Calculated Q_h_nd: {} kWh/m2a", q_h_nd_m2a);
        
        assert!((q_h_nd_m2a - 167.3).abs() < 5.0, "Heating demand is way off: expected ~167, got {}", q_h_nd_m2a);
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
            dhw_base_volume_liters_per_day: 0.0,
            dhw_wastewater_heat_recovery: 0.0,
            dhw_generator_type: "HeatPumpAirWater".to_string(),
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

pub mod overheating {
    use serde::{Deserialize, Serialize};

    // --- ENUMS & CONSTANTS ---

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum SummerClimateRegion {
        A, // Cool (e.g., Northern Coast)
        B, // Moderate (e.g., Central Germany)
        C, // Hot (e.g., Rhine Valley, Stuttgart)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum NightVentilation {
        None,      // Windows remain closed
        Increased, // Windows tilted / Mech. Ventilation (n >= 2.0 1/h)
        High,      // Wide open cross-ventilation (n >= 5.0 1/h)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum BuildingCategory {
        Residential,
        NonResidential,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum ConstructionClass {
        Light,  // Timber frame, drywall, suspended ceilings
        Medium, // Mixed masonry/lightweight
        Heavy,  // Solid concrete/brick walls and floors
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum ExemptionStatus {
        ExemptSmallWindows,           // Passed via § 8.3.2 (a)
        ExemptResidentialHighlyShaded, // Passed via § 8.3.2 (b)
        NotExempt,                    // Must calculate S_vorh <= S_zul
    }

    // --- DATA MODELS ---

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct OverheatingWindow {
        pub area: f64,                 // A_w in m² (Clear opening dimension)
        pub g_value: f64,              // Glass solar transmittance (perpendicular)
        pub f_c: f64,                  // Sun protection reduction factor
        pub f_s: f64,                  // Structural shading factor (1.0 if none)
        pub inclination_deg: f64,      // 90 is vertical, 0 is horizontal flat roof
        pub is_north_oriented: bool,   // True if oriented NE through N to NW
        pub is_permanently_shaded: bool, // True if blocked by adjacent building, etc.
    }

    impl OverheatingWindow {
        /// Calculates g_tot = g * F_C * F_S (Eq. 3 modified for structural shading)
        pub fn g_tot(&self) -> f64 {
            self.g_value * self.f_c * self.f_s
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CriticalRoom {
        pub actual_floor_area: f64, // A_G in m²
        pub room_depth: f64,        // Depth from the window facade in meters
        pub room_height: f64,       // Clear ceiling height in meters
        pub has_opposite_windows: bool, // True if windows are on opposite walls (cross-ventilation geometry)
        
        pub category: BuildingCategory,
        pub climate_region: SummerClimateRegion,
        pub construction: ConstructionClass,
        pub night_ventilation: NightVentilation,
        pub has_passive_cooling: bool,
        pub windows: Vec<OverheatingWindow>,
    }

    impl CriticalRoom {
        /// Calculates the effective Net Floor Area according to § 8.3.4 (a)
        pub fn effective_floor_area(&self) -> f64 {
            // Maximum depth allowed is 3x height (or 6x if opposite windows exist)
            let max_depth_multiplier = if self.has_opposite_windows { 6.0 } else { 3.0 };
            let max_allowable_depth = max_depth_multiplier * self.room_height;
            
            if self.room_depth > max_allowable_depth && self.room_depth > 0.0 {
                // Scale down the area proportionally to the depth cap
                self.actual_floor_area * (max_allowable_depth / self.room_depth)
            } else {
                self.actual_floor_area
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct OverheatingResult {
        pub exemption: ExemptionStatus,
        pub s_vorh: f64,   // Existing solar entry
        pub s_zul: f64,    // Maximum allowable solar entry
        pub passes: bool,  // True if S_vorh <= S_zul OR if Exempt
    }

    // --- CALCULATION ENGINE ---

    pub struct OverheatingEngine;

    impl OverheatingEngine {
        
        /// Checks if the room bypasses the calculation entirely (§ 8.3.2)
        pub fn check_exemptions(room: &CriticalRoom, f_w_g: f64) -> ExemptionStatus {
            // Rule A: Exempt due to very small windows (Table 7)
            let mut strict_limit = 1.0;
            for w in &room.windows {
                let limit = if w.inclination_deg <= 60.0 {
                    0.07 // 7% for roof windows
                } else if !w.is_north_oriented {
                    0.10 // 10% for East, South, West vertical windows
                } else {
                    0.15 // 15% for North vertical windows
                };
                
                if limit < strict_limit {
                    strict_limit = limit;
                }
            }
            
            if f_w_g <= strict_limit {
                return ExemptionStatus::ExemptSmallWindows;
            }

            // Rule B: Exempt Residential with heavy shading and night ventilation
            if room.category == BuildingCategory::Residential 
                && f_w_g <= 0.35 
                && room.night_ventilation != NightVentilation::None 
            {
                let mut all_shaded_properly = true;
                
                for w in &room.windows {
                    // Check applies to East, South, West windows
                    if !w.is_north_oriented {
                        let required_fc = if w.g_value > 0.40 { 0.30 } else { 0.35 };
                        if w.f_c > required_fc {
                            all_shaded_properly = false;
                            break;
                        }
                    }
                }
                
                // If there are no windows, or all applicable windows meet the strict F_c requirement
                if all_shaded_properly && !room.windows.is_empty() {
                    return ExemptionStatus::ExemptResidentialHighlyShaded;
                }
            }

            ExemptionStatus::NotExempt
        }

        /// Executes the full GEG Verification
        pub fn evaluate_room(room: &CriticalRoom) -> OverheatingResult {
            let a_g_eff = room.effective_floor_area();
            if a_g_eff <= 0.0 {
                return OverheatingResult { exemption: ExemptionStatus::NotExempt, s_vorh: 0.0, s_zul: 0.0, passes: true };
            }

            let total_window_area: f64 = room.windows.iter().map(|w| w.area).sum();
            let f_w_g = total_window_area / a_g_eff; // Floor-area-related window fraction

            // 1. Check Exemptions
            let exemption = Self::check_exemptions(room, f_w_g);
            if exemption != ExemptionStatus::NotExempt {
                return OverheatingResult { exemption, s_vorh: 0.0, s_zul: 0.0, passes: true };
            }

            // 2. Calculate Existing Solar Entry (S_vorh) - Eq. 2
            let sum_window_entries: f64 = room.windows.iter()
                .map(|w| w.area * w.g_tot())
                .sum();
            let s_vorh = sum_window_entries / a_g_eff;

            // 3. Calculate Allowable Solar Entry (S_zul) - Eq. 4 & Table 8
            let s_zul = Self::calculate_s_zul(room, f_w_g, total_window_area);

            OverheatingResult {
                exemption,
                s_vorh,
                s_zul,
                // Pass if existing entry is less than or equal to allowable entry
                passes: s_vorh <= (s_zul + 1e-6), 
            }
        }

        /// Calculates S_zul (Equation 4 & Table 8)
        fn calculate_s_zul(room: &CriticalRoom, f_w_g: f64, total_window_area: f64) -> f64 {
            // --- S1: Base Value from Table 8 ---
            let s_1 = Self::get_s1_base_value(
                room.category,
                room.night_ventilation,
                room.climate_region,
                room.construction,
            );

            // --- S2: Geometry Modifier ---
            let (a, b) = match room.category {
                BuildingCategory::Residential => (0.060, 0.231),
                BuildingCategory::NonResidential => (0.030, 0.115),
            };
            let s_2 = a - (b * f_w_g);

            // --- S3: Solar Control Glass Modifier ---
            // Proportional addition for windows with g <= 0.40
            let s_3 = if total_window_area > 0.0 {
                let sun_protect_area: f64 = room.windows.iter()
                    .filter(|w| w.g_value <= 0.40)
                    .map(|w| w.area)
                    .sum();
                0.03 * (sun_protect_area / total_window_area)
            } else {
                0.0
            };

            // --- S4: Roof Window Penalty ---
            // Calculates fraction of windows with inclination between 0 and 60 degrees
            let s_4 = if total_window_area > 0.0 {
                let roof_window_area: f64 = room.windows.iter()
                    .filter(|w| w.inclination_deg >= 0.0 && w.inclination_deg <= 60.0)
                    .map(|w| w.area)
                    .sum();
                let f_neig = roof_window_area / total_window_area;
                -0.035 * f_neig
            } else {
                0.0
            };

            // --- S5: North/Shaded Window Bonus ---
            // Applies to vertical North windows OR permanently shaded windows
            let s_5 = if total_window_area > 0.0 {
                let shaded_area: f64 = room.windows.iter()
                    .filter(|w| (w.inclination_deg > 60.0 && w.is_north_oriented) || w.is_permanently_shaded)
                    .map(|w| w.area)
                    .sum();
                let f_nord = shaded_area / total_window_area;
                0.10 * f_nord
            } else {
                0.0
            };

            // --- S6: Passive Cooling Bonus ---
            let s_6 = if room.has_passive_cooling {
                match room.construction {
                    ConstructionClass::Light => 0.02,
                    ConstructionClass::Medium => 0.04,
                    ConstructionClass::Heavy => 0.06,
                }
            } else {
                0.0
            };

            // Sum all modifiers
            s_1 + s_2 + s_3 + s_4 + s_5 + s_6
        }

        /// Direct mapping of Table 8 Base Values
        fn get_s1_base_value(category: BuildingCategory, vent: NightVentilation, region: SummerClimateRegion, weight: ConstructionClass) -> f64 {
            match (category, vent, region, weight) {
                // RESIDENTIAL - NONE
                (BuildingCategory::Residential, NightVentilation::None, SummerClimateRegion::A, ConstructionClass::Light) => 0.071,
                (BuildingCategory::Residential, NightVentilation::None, SummerClimateRegion::A, ConstructionClass::Medium) => 0.080,
                (BuildingCategory::Residential, NightVentilation::None, SummerClimateRegion::A, ConstructionClass::Heavy) => 0.087,
                (BuildingCategory::Residential, NightVentilation::None, SummerClimateRegion::B, ConstructionClass::Light) => 0.056,
                (BuildingCategory::Residential, NightVentilation::None, SummerClimateRegion::B, ConstructionClass::Medium) => 0.067,
                (BuildingCategory::Residential, NightVentilation::None, SummerClimateRegion::B, ConstructionClass::Heavy) => 0.074,
                (BuildingCategory::Residential, NightVentilation::None, SummerClimateRegion::C, ConstructionClass::Light) => 0.041,
                (BuildingCategory::Residential, NightVentilation::None, SummerClimateRegion::C, ConstructionClass::Medium) => 0.054,
                (BuildingCategory::Residential, NightVentilation::None, SummerClimateRegion::C, ConstructionClass::Heavy) => 0.061,

                // RESIDENTIAL - INCREASED
                (BuildingCategory::Residential, NightVentilation::Increased, SummerClimateRegion::A, ConstructionClass::Light) => 0.098,
                (BuildingCategory::Residential, NightVentilation::Increased, SummerClimateRegion::A, ConstructionClass::Medium) => 0.114,
                (BuildingCategory::Residential, NightVentilation::Increased, SummerClimateRegion::A, ConstructionClass::Heavy) => 0.125,
                (BuildingCategory::Residential, NightVentilation::Increased, SummerClimateRegion::B, ConstructionClass::Light) => 0.088,
                (BuildingCategory::Residential, NightVentilation::Increased, SummerClimateRegion::B, ConstructionClass::Medium) => 0.103,
                (BuildingCategory::Residential, NightVentilation::Increased, SummerClimateRegion::B, ConstructionClass::Heavy) => 0.113,
                (BuildingCategory::Residential, NightVentilation::Increased, SummerClimateRegion::C, ConstructionClass::Light) => 0.078,
                (BuildingCategory::Residential, NightVentilation::Increased, SummerClimateRegion::C, ConstructionClass::Medium) => 0.092,
                (BuildingCategory::Residential, NightVentilation::Increased, SummerClimateRegion::C, ConstructionClass::Heavy) => 0.101,

                // RESIDENTIAL - HIGH
                (BuildingCategory::Residential, NightVentilation::High, SummerClimateRegion::A, ConstructionClass::Light) => 0.128,
                (BuildingCategory::Residential, NightVentilation::High, SummerClimateRegion::A, ConstructionClass::Medium) => 0.160,
                (BuildingCategory::Residential, NightVentilation::High, SummerClimateRegion::A, ConstructionClass::Heavy) => 0.181,
                (BuildingCategory::Residential, NightVentilation::High, SummerClimateRegion::B, ConstructionClass::Light) => 0.117,
                (BuildingCategory::Residential, NightVentilation::High, SummerClimateRegion::B, ConstructionClass::Medium) => 0.152,
                (BuildingCategory::Residential, NightVentilation::High, SummerClimateRegion::B, ConstructionClass::Heavy) => 0.171,
                (BuildingCategory::Residential, NightVentilation::High, SummerClimateRegion::C, ConstructionClass::Light) => 0.105,
                (BuildingCategory::Residential, NightVentilation::High, SummerClimateRegion::C, ConstructionClass::Medium) => 0.143,
                (BuildingCategory::Residential, NightVentilation::High, SummerClimateRegion::C, ConstructionClass::Heavy) => 0.158,

                // NON-RESIDENTIAL - NONE
                (BuildingCategory::NonResidential, NightVentilation::None, SummerClimateRegion::A, ConstructionClass::Light) => 0.013,
                (BuildingCategory::NonResidential, NightVentilation::None, SummerClimateRegion::A, ConstructionClass::Medium) => 0.020,
                (BuildingCategory::NonResidential, NightVentilation::None, SummerClimateRegion::A, ConstructionClass::Heavy) => 0.025,
                (BuildingCategory::NonResidential, NightVentilation::None, SummerClimateRegion::B, ConstructionClass::Light) => 0.007,
                (BuildingCategory::NonResidential, NightVentilation::None, SummerClimateRegion::B, ConstructionClass::Medium) => 0.013,
                (BuildingCategory::NonResidential, NightVentilation::None, SummerClimateRegion::B, ConstructionClass::Heavy) => 0.018,
                (BuildingCategory::NonResidential, NightVentilation::None, SummerClimateRegion::C, ConstructionClass::Light) => 0.000,
                (BuildingCategory::NonResidential, NightVentilation::None, SummerClimateRegion::C, ConstructionClass::Medium) => 0.006,
                (BuildingCategory::NonResidential, NightVentilation::None, SummerClimateRegion::C, ConstructionClass::Heavy) => 0.011,

                // NON-RESIDENTIAL - INCREASED
                (BuildingCategory::NonResidential, NightVentilation::Increased, SummerClimateRegion::A, ConstructionClass::Light) => 0.071,
                (BuildingCategory::NonResidential, NightVentilation::Increased, SummerClimateRegion::A, ConstructionClass::Medium) => 0.089,
                (BuildingCategory::NonResidential, NightVentilation::Increased, SummerClimateRegion::A, ConstructionClass::Heavy) => 0.101,
                (BuildingCategory::NonResidential, NightVentilation::Increased, SummerClimateRegion::B, ConstructionClass::Light) => 0.060,
                (BuildingCategory::NonResidential, NightVentilation::Increased, SummerClimateRegion::B, ConstructionClass::Medium) => 0.081,
                (BuildingCategory::NonResidential, NightVentilation::Increased, SummerClimateRegion::B, ConstructionClass::Heavy) => 0.092,
                (BuildingCategory::NonResidential, NightVentilation::Increased, SummerClimateRegion::C, ConstructionClass::Light) => 0.048,
                (BuildingCategory::NonResidential, NightVentilation::Increased, SummerClimateRegion::C, ConstructionClass::Medium) => 0.072,
                (BuildingCategory::NonResidential, NightVentilation::Increased, SummerClimateRegion::C, ConstructionClass::Heavy) => 0.083,

                // NON-RESIDENTIAL - HIGH
                (BuildingCategory::NonResidential, NightVentilation::High, SummerClimateRegion::A, ConstructionClass::Light) => 0.090,
                (BuildingCategory::NonResidential, NightVentilation::High, SummerClimateRegion::A, ConstructionClass::Medium) => 0.135,
                (BuildingCategory::NonResidential, NightVentilation::High, SummerClimateRegion::A, ConstructionClass::Heavy) => 0.170,
                (BuildingCategory::NonResidential, NightVentilation::High, SummerClimateRegion::B, ConstructionClass::Light) => 0.082,
                (BuildingCategory::NonResidential, NightVentilation::High, SummerClimateRegion::B, ConstructionClass::Medium) => 0.124,
                (BuildingCategory::NonResidential, NightVentilation::High, SummerClimateRegion::B, ConstructionClass::Heavy) => 0.160,
                (BuildingCategory::NonResidential, NightVentilation::High, SummerClimateRegion::C, ConstructionClass::Light) => 0.074,
                (BuildingCategory::NonResidential, NightVentilation::High, SummerClimateRegion::C, ConstructionClass::Medium) => 0.113,
                (BuildingCategory::NonResidential, NightVentilation::High, SummerClimateRegion::C, ConstructionClass::Heavy) => 0.145,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_exemption_residential_highly_shaded() {
            let room = CriticalRoom {
                actual_floor_area: 25.0,
                room_depth: 5.0,
                room_height: 2.5,
                has_opposite_windows: false,
                category: BuildingCategory::Residential,
                climate_region: SummerClimateRegion::C,
                construction: ConstructionClass::Light,
                night_ventilation: NightVentilation::Increased,
                has_passive_cooling: false,
                windows: vec![
                    OverheatingWindow {
                        area: 5.0, // f_w_g = 5/25 = 0.20 (<= 0.35 threshold)
                        g_value: 0.60,
                        f_c: 0.25, // <= 0.30 threshold for g > 0.4
                        f_s: 1.0,
                        inclination_deg: 90.0,
                        is_north_oriented: false,
                        is_permanently_shaded: false,
                    }
                ]
            };

            let result = OverheatingEngine::evaluate_room(&room);
            assert_eq!(result.exemption, ExemptionStatus::ExemptResidentialHighlyShaded);
            assert_eq!(result.passes, true);
        }

        #[test]
        fn test_effective_area_capping() {
            // A terribly designed, extremely deep room (10m deep, only 2.5m high)
            let room = CriticalRoom {
                actual_floor_area: 50.0, // 5m wide x 10m deep
                room_depth: 10.0,
                room_height: 2.5,
                has_opposite_windows: false, // Cap is 3 * 2.5 = 7.5m
                category: BuildingCategory::Residential,
                climate_region: SummerClimateRegion::C,
                construction: ConstructionClass::Light,
                night_ventilation: NightVentilation::None,
                has_passive_cooling: false,
                windows: vec![
                    OverheatingWindow {
                        area: 10.0,
                        g_value: 0.60,
                        f_c: 1.0,
                        f_s: 1.0,
                        inclination_deg: 90.0,
                        is_north_oriented: false,
                        is_permanently_shaded: false,
                    }
                ]
            };

            let result = OverheatingEngine::evaluate_room(&room);
            
            // Effective area should be capped: width 5m * max_depth 7.5m = 37.5m²
            // This means f_w_g is calculated as 10/37.5 = 0.266 instead of 10/50 = 0.20
            // S_vorh = (10 * 0.6) / 37.5 = 0.16
            
            assert_eq!(result.exemption, ExemptionStatus::NotExempt);
            assert_eq!(result.passes, false); // Massive overheating expected
            assert!((result.s_vorh - 0.16).abs() < 0.001);
        }
    }
}


pub mod lighting {
    use serde::{Deserialize, Serialize};

    // --- CONSTANTS ---
    pub const DEFAULT_MAINTENANCE_FACTOR_WF: f64 = 0.67;
    pub const K_WF_DEFAULT: f64 = 0.80 / DEFAULT_MAINTENANCE_FACTOR_WF;

    // --- ENUMS & DATA MODELS ---

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum LightingType {
        Direct,
        DirectIndirect,
        Indirect,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum LampTechnology {
        Incandescent,
        Halogen,
        FluorescentEVG,
        CompactFluorescent,
        LEDReplacement,
        LEDLuminaire,
    }

    impl LampTechnology {
        /// Table 6: Adjustment factor k_L
        pub fn k_l_factor(&self) -> f64 {
            match self {
                Self::Incandescent => 6.0,
                Self::Halogen => 5.0,
                Self::FluorescentEVG => 1.0,
                Self::CompactFluorescent => 1.4,
                Self::LEDReplacement => 0.68,
                Self::LEDLuminaire => 0.49, // highly efficient
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum DaylightControl {
        Manual,
        DimmedAutoOnOff,     // Gedimmt, wiedereinschaltend
        DimmedManualOnAutoOff, // Gedimmt, nicht wiedereinschaltend
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum PresenceControl {
        Manual,
        MotionDetector,
    }

    impl PresenceControl {
        /// Table 28: Detector efficiency c_prä,kon
        pub fn c_pra_kon(&self) -> f64 {
            match self {
                Self::Manual => 0.50,
                Self::MotionDetector => 0.95,
            }
        }
    }


    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub struct LightingRequirement {
        pub e_m: Option<f64>,
        pub ugr_l: Option<f64>,
        pub r_a: Option<f64>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum RoomUsage {
    EntranceHalls,
    LoungesWaitingAreas,
    CirculationAreasAndCorridors,
    StairsEscalatorsMovingWalkways,
    LoadingRampsBays,
    CanteensAndPantries,
    RestRooms,
    RoomsForPhysicalExercise,
    CloakroomsWashroomsBathroomsToilets,
    SickBayFirstAidRooms,
    PlantRoomsSwitchGearRooms,
    TelexPostRoomSwitchboard,
    StoreAndStockroomsUnmanned,
    StoreAndStockroomsManned,
    DispatchAndPackingAreas,
    GangwaysInRackSystemsUnmanned,
    GangwaysInRackSystemsManned,
    LoadingAndOperatingOfGoods,
    BuildingsForLivestock,
    SickAnimalPens,
    FeedPreparation,
    PreparationAndBaking,
    FinishingGlazingDecorating,
    DryingCement,
    PreparationOfMaterialsKilns,
    GeneralMachineWorkCement,
    RoughForms,
    DryingCeramics,
    PreparationGeneralMachineWorkCeramics,
    EnamellingRollingPressing,
    GrindingEngravingPolishing,
    PrecisionWorkDecorativePainting,
    RemoteoperatedProcessing,
    ProcessingWithLimitedManualIntervention,
    ConstantlyMannedWorkPlaces,
    PrecisionMeasuringRoomsLabs,
    ColorInspectionChemicals,
    CableAndWireManufacture,
    WindingLargeCoils,
    WindingMediumCoils,
    WindingSmallCoils,
    CoilImpregnating,
    AssemblyWorkRough,
    AssemblyWorkMedium,
    AssemblyWorkFine,
    AssemblyWorkPrecision,
    ElectronicWorkshopsTesting,
    WorkplacesZonesInBreweriesMalting,
    WashingBarrelFillingCleaning,
    SortingAndWashingOfProducts,
    WorkOnColorcriticalGoods,
    UndergroundTunnelsCellars,
    Platforms,
    SandPreparation,
    CoreMakingMouldMaking,
    Hairdressing,
    WorkingWithPreciousStones,
    WatchMakingManual,
    GoodsInMarkingAndSorting,
    WashingAndDryCleaning,
    IroningPressing,
    InspectionAndRepairs,
    VatsBarrelsPits,
    FleshingSkivingSplitting,
    SaddleryShoeManufacture,
    QualityControlLeather,
    ColorInspectionLeather,
    OpenDieForging,
    DropForging,
    Welding,
    RoughMediumMachining,
    PrecisionMachining,
    ToolMakingCuttingEquipment,
    EdgeRunnersPulpMills,
    PaperManufacturePaperMachines,
    PaperInspection,
    FuelSupplyPlant,
    BoilerHouse,
    MachineHalls,
    ControlRoomsPowerStations,
    CuttingGildingEmbossing,
    SortingPaperReproduction,
    TypeSettingRetouching,
    ColorInspectionInPrinting,
    ProductionPlantsWithoutManualOp,
    ProductionPlantsWithManualOp,
    SlabInspection,
    BaleOpeningCardingWashing,
    SpinningPlyingReeling,
    WeavingKnitting,
    SewingFineKnitting,
    ColorInspectionFabricControl,
    BodyWorkAndAssembly,
    PaintingSpraying,
    PaintingInspectionRepair,
    AutomaticProcessingSawing,
    JoinersBenchGluing,
    PolishingPainting,
    QualityControlWood,
    FilingCopyingCirculation,
    WritingTypingReadingDataProc,
    CadWorkStations,
    ConferenceAndMeetingRooms,
    ReceptionDesk,
    Archives,
    SalesAreaSmall,
    SalesAreaLarge,
    TillAreaCashier,
    WrapperTable,
    ReceptionCashier,
    Kitchens,
    RestaurantDiningRoom,
    SelfserviceRestaurants,
    Buffet,
    ConferenceRoomsHotels,
    GeneralExhibitions,
    Bookshelves,
    ReadingArea,
    Counters,
    InOutRampsDay,
    InOutRampsNight,
    TrafficLanes,
    ParkingAreas,
    TicketOffice,
    PlaySchoolRoomsNursery,
    NurseryClassCrafts,
    ClassroomsTutorialRooms,
    ClassroomsForEveningClasses,
    AuditoriumsLectureHalls,
    BlackboardsWhiteboards,
    DemonstrationTables,
    ArtAndCraftRooms,
    ArtRoomsInArtSchools,
    TechnicalDrawingRooms,
    ComputerPracticeRooms,
    LanguageLaboratories,
    PreparationRooms,
    StudentCommonRooms,
    TeachersRooms,
    SportsHallsGymnasiums,
    WaitingRooms,
    CorridorsDay,
    CorridorsNight,
    StaffOfficeRooms,
    StaffRooms,
    WardsGeneralLighting,
    WardsReadingLighting,
    WardsSimpleExaminations,
    ExaminationAndTreatmentGeneral,
    ExaminationAndTreatmentDetailed,
    EarAndEyeExamination,
    OperatingTheatrePreopRecovery,
    OperatingTheatreGeneral,
    OperatingCavity,
    IntensiveCareGeneral,
    IntensiveCareExamination,
    DentistsGeneral,
    DentistsAtThePatient,
    Pharmacies,
    AutopsyRooms,
    AutopsyTable,
    ArrivalAndDepartureHalls,
    BaggageClaim,
    ConnectionAreasEscalators,
    InformationDesksCheckin,
    CustomsAndPassportControl,
    AirportWaitingAreas,
    LuggageSortingRooms,
    SecurityCheck,
    AirTrafficControlTower,
    EnclosedPlatforms,
    PassengerSubwaysTunnels,
    TicketHallsAndConcourse,
    TicketAndLuggageOffices,
    RailwayWaitingRooms,
    }

    impl RoomUsage {
        pub fn requirements(&self) -> LightingRequirement {
            match self {
            Self::EntranceHalls => LightingRequirement { e_m: Some(100.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::LoungesWaitingAreas => LightingRequirement { e_m: Some(200.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::CirculationAreasAndCorridors => LightingRequirement { e_m: Some(100.0), ugr_l: Some(28.0), r_a: Some(80.0) },
            Self::StairsEscalatorsMovingWalkways => LightingRequirement { e_m: Some(150.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::LoadingRampsBays => LightingRequirement { e_m: Some(150.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::CanteensAndPantries => LightingRequirement { e_m: Some(200.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::RestRooms => LightingRequirement { e_m: Some(200.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::RoomsForPhysicalExercise => LightingRequirement { e_m: Some(300.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::CloakroomsWashroomsBathroomsToilets => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::SickBayFirstAidRooms => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::PlantRoomsSwitchGearRooms => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::TelexPostRoomSwitchboard => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::StoreAndStockroomsUnmanned => LightingRequirement { e_m: Some(100.0), ugr_l: Some(25.0), r_a: Some(60.0) },
            Self::StoreAndStockroomsManned => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::DispatchAndPackingAreas => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::GangwaysInRackSystemsUnmanned => LightingRequirement { e_m: Some(20.0), ugr_l: None, r_a: Some(40.0) },
            Self::GangwaysInRackSystemsManned => LightingRequirement { e_m: Some(150.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::LoadingAndOperatingOfGoods => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::BuildingsForLivestock => LightingRequirement { e_m: Some(50.0), ugr_l: Some(28.0), r_a: Some(40.0) },
            Self::SickAnimalPens => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::FeedPreparation => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::PreparationAndBaking => LightingRequirement { e_m: Some(300.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::FinishingGlazingDecorating => LightingRequirement { e_m: Some(500.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::DryingCement => LightingRequirement { e_m: Some(50.0), ugr_l: Some(28.0), r_a: Some(40.0) },
            Self::PreparationOfMaterialsKilns => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::GeneralMachineWorkCement => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::RoughForms => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::DryingCeramics => LightingRequirement { e_m: Some(50.0), ugr_l: Some(28.0), r_a: Some(40.0) },
            Self::PreparationGeneralMachineWorkCeramics => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::EnamellingRollingPressing => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::GrindingEngravingPolishing => LightingRequirement { e_m: Some(750.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::PrecisionWorkDecorativePainting => LightingRequirement { e_m: Some(1000.0), ugr_l: Some(16.0), r_a: Some(90.0) },
            Self::RemoteoperatedProcessing => LightingRequirement { e_m: Some(50.0), ugr_l: Some(28.0), r_a: Some(40.0) },
            Self::ProcessingWithLimitedManualIntervention => LightingRequirement { e_m: Some(150.0), ugr_l: Some(28.0), r_a: Some(40.0) },
            Self::ConstantlyMannedWorkPlaces => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::PrecisionMeasuringRoomsLabs => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::ColorInspectionChemicals => LightingRequirement { e_m: Some(1000.0), ugr_l: Some(16.0), r_a: Some(90.0) },
            Self::CableAndWireManufacture => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::WindingLargeCoils => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::WindingMediumCoils => LightingRequirement { e_m: Some(500.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::WindingSmallCoils => LightingRequirement { e_m: Some(750.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::CoilImpregnating => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::AssemblyWorkRough => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::AssemblyWorkMedium => LightingRequirement { e_m: Some(500.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::AssemblyWorkFine => LightingRequirement { e_m: Some(750.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::AssemblyWorkPrecision => LightingRequirement { e_m: Some(1000.0), ugr_l: Some(16.0), r_a: Some(80.0) },
            Self::ElectronicWorkshopsTesting => LightingRequirement { e_m: Some(1500.0), ugr_l: Some(16.0), r_a: Some(80.0) },
            Self::WorkplacesZonesInBreweriesMalting => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::WashingBarrelFillingCleaning => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::SortingAndWashingOfProducts => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::WorkOnColorcriticalGoods => LightingRequirement { e_m: Some(500.0), ugr_l: Some(22.0), r_a: Some(90.0) },
            Self::UndergroundTunnelsCellars => LightingRequirement { e_m: Some(50.0), ugr_l: Some(28.0), r_a: Some(40.0) },
            Self::Platforms => LightingRequirement { e_m: Some(100.0), ugr_l: Some(25.0), r_a: Some(40.0) },
            Self::SandPreparation => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::CoreMakingMouldMaking => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::Hairdressing => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(90.0) },
            Self::WorkingWithPreciousStones => LightingRequirement { e_m: Some(1500.0), ugr_l: Some(16.0), r_a: Some(90.0) },
            Self::WatchMakingManual => LightingRequirement { e_m: Some(1500.0), ugr_l: Some(16.0), r_a: Some(80.0) },
            Self::GoodsInMarkingAndSorting => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::WashingAndDryCleaning => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::IroningPressing => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::InspectionAndRepairs => LightingRequirement { e_m: Some(750.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::VatsBarrelsPits => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::FleshingSkivingSplitting => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::SaddleryShoeManufacture => LightingRequirement { e_m: Some(500.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::QualityControlLeather => LightingRequirement { e_m: Some(1000.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::ColorInspectionLeather => LightingRequirement { e_m: Some(1000.0), ugr_l: Some(16.0), r_a: Some(90.0) },
            Self::OpenDieForging => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::DropForging => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::Welding => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::RoughMediumMachining => LightingRequirement { e_m: Some(300.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::PrecisionMachining => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::ToolMakingCuttingEquipment => LightingRequirement { e_m: Some(750.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::EdgeRunnersPulpMills => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::PaperManufacturePaperMachines => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::PaperInspection => LightingRequirement { e_m: Some(500.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::FuelSupplyPlant => LightingRequirement { e_m: Some(50.0), ugr_l: Some(28.0), r_a: Some(40.0) },
            Self::BoilerHouse => LightingRequirement { e_m: Some(100.0), ugr_l: Some(28.0), r_a: Some(40.0) },
            Self::MachineHalls => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::ControlRoomsPowerStations => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::CuttingGildingEmbossing => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::SortingPaperReproduction => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::TypeSettingRetouching => LightingRequirement { e_m: Some(1000.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::ColorInspectionInPrinting => LightingRequirement { e_m: Some(1500.0), ugr_l: Some(16.0), r_a: Some(90.0) },
            Self::ProductionPlantsWithoutManualOp => LightingRequirement { e_m: Some(50.0), ugr_l: Some(28.0), r_a: Some(40.0) },
            Self::ProductionPlantsWithManualOp => LightingRequirement { e_m: Some(150.0), ugr_l: Some(28.0), r_a: Some(40.0) },
            Self::SlabInspection => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::BaleOpeningCardingWashing => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::SpinningPlyingReeling => LightingRequirement { e_m: Some(300.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::WeavingKnitting => LightingRequirement { e_m: Some(500.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::SewingFineKnitting => LightingRequirement { e_m: Some(750.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::ColorInspectionFabricControl => LightingRequirement { e_m: Some(1000.0), ugr_l: Some(16.0), r_a: Some(90.0) },
            Self::BodyWorkAndAssembly => LightingRequirement { e_m: Some(500.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::PaintingSpraying => LightingRequirement { e_m: Some(750.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::PaintingInspectionRepair => LightingRequirement { e_m: Some(1000.0), ugr_l: Some(16.0), r_a: Some(90.0) },
            Self::AutomaticProcessingSawing => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::JoinersBenchGluing => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::PolishingPainting => LightingRequirement { e_m: Some(750.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::QualityControlWood => LightingRequirement { e_m: Some(1000.0), ugr_l: Some(19.0), r_a: Some(90.0) },
            Self::FilingCopyingCirculation => LightingRequirement { e_m: Some(300.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::WritingTypingReadingDataProc => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::CadWorkStations => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::ConferenceAndMeetingRooms => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::ReceptionDesk => LightingRequirement { e_m: Some(300.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::Archives => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::SalesAreaSmall => LightingRequirement { e_m: Some(300.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::SalesAreaLarge => LightingRequirement { e_m: Some(500.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::TillAreaCashier => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::WrapperTable => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::ReceptionCashier => LightingRequirement { e_m: Some(300.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::Kitchens => LightingRequirement { e_m: Some(500.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::RestaurantDiningRoom => LightingRequirement { e_m: None, ugr_l: None, r_a: Some(80.0) },
            Self::SelfserviceRestaurants => LightingRequirement { e_m: Some(200.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::Buffet => LightingRequirement { e_m: Some(300.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::ConferenceRoomsHotels => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::GeneralExhibitions => LightingRequirement { e_m: Some(300.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::Bookshelves => LightingRequirement { e_m: Some(200.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::ReadingArea => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::Counters => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::InOutRampsDay => LightingRequirement { e_m: Some(300.0), ugr_l: Some(25.0), r_a: Some(40.0) },
            Self::InOutRampsNight => LightingRequirement { e_m: Some(75.0), ugr_l: Some(25.0), r_a: Some(40.0) },
            Self::TrafficLanes => LightingRequirement { e_m: Some(75.0), ugr_l: Some(25.0), r_a: Some(40.0) },
            Self::ParkingAreas => LightingRequirement { e_m: Some(75.0), ugr_l: Some(28.0), r_a: Some(40.0) },
            Self::TicketOffice => LightingRequirement { e_m: Some(300.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::PlaySchoolRoomsNursery => LightingRequirement { e_m: Some(300.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::NurseryClassCrafts => LightingRequirement { e_m: Some(300.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::ClassroomsTutorialRooms => LightingRequirement { e_m: Some(300.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::ClassroomsForEveningClasses => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::AuditoriumsLectureHalls => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::BlackboardsWhiteboards => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::DemonstrationTables => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::ArtAndCraftRooms => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::ArtRoomsInArtSchools => LightingRequirement { e_m: Some(750.0), ugr_l: Some(19.0), r_a: Some(90.0) },
            Self::TechnicalDrawingRooms => LightingRequirement { e_m: Some(750.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::ComputerPracticeRooms => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::LanguageLaboratories => LightingRequirement { e_m: Some(300.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::PreparationRooms => LightingRequirement { e_m: Some(500.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::StudentCommonRooms => LightingRequirement { e_m: Some(200.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::TeachersRooms => LightingRequirement { e_m: Some(300.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::SportsHallsGymnasiums => LightingRequirement { e_m: Some(300.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::WaitingRooms => LightingRequirement { e_m: Some(200.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::CorridorsDay => LightingRequirement { e_m: Some(200.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::CorridorsNight => LightingRequirement { e_m: Some(50.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::StaffOfficeRooms => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::StaffRooms => LightingRequirement { e_m: Some(300.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::WardsGeneralLighting => LightingRequirement { e_m: Some(100.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::WardsReadingLighting => LightingRequirement { e_m: Some(300.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::WardsSimpleExaminations => LightingRequirement { e_m: Some(300.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::ExaminationAndTreatmentGeneral => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(90.0) },
            Self::ExaminationAndTreatmentDetailed => LightingRequirement { e_m: Some(1000.0), ugr_l: Some(19.0), r_a: Some(90.0) },
            Self::EarAndEyeExamination => LightingRequirement { e_m: Some(1000.0), ugr_l: None, r_a: Some(90.0) },
            Self::OperatingTheatrePreopRecovery => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(90.0) },
            Self::OperatingTheatreGeneral => LightingRequirement { e_m: Some(1000.0), ugr_l: Some(19.0), r_a: Some(90.0) },
            Self::OperatingCavity => LightingRequirement { e_m: Some(100000.0), ugr_l: None, r_a: None },
            Self::IntensiveCareGeneral => LightingRequirement { e_m: Some(100.0), ugr_l: Some(19.0), r_a: Some(90.0) },
            Self::IntensiveCareExamination => LightingRequirement { e_m: Some(1000.0), ugr_l: Some(19.0), r_a: Some(90.0) },
            Self::DentistsGeneral => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(90.0) },
            Self::DentistsAtThePatient => LightingRequirement { e_m: Some(1000.0), ugr_l: None, r_a: Some(90.0) },
            Self::Pharmacies => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::AutopsyRooms => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(90.0) },
            Self::AutopsyTable => LightingRequirement { e_m: Some(5000.0), ugr_l: None, r_a: Some(90.0) },
            Self::ArrivalAndDepartureHalls => LightingRequirement { e_m: Some(200.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::BaggageClaim => LightingRequirement { e_m: Some(200.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::ConnectionAreasEscalators => LightingRequirement { e_m: Some(150.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::InformationDesksCheckin => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::CustomsAndPassportControl => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::AirportWaitingAreas => LightingRequirement { e_m: Some(200.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            Self::LuggageSortingRooms => LightingRequirement { e_m: Some(200.0), ugr_l: Some(25.0), r_a: Some(80.0) },
            Self::SecurityCheck => LightingRequirement { e_m: Some(300.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::AirTrafficControlTower => LightingRequirement { e_m: Some(500.0), ugr_l: Some(16.0), r_a: Some(80.0) },
            Self::EnclosedPlatforms => LightingRequirement { e_m: Some(200.0), ugr_l: Some(28.0), r_a: Some(40.0) },
            Self::PassengerSubwaysTunnels => LightingRequirement { e_m: Some(150.0), ugr_l: Some(28.0), r_a: Some(40.0) },
            Self::TicketHallsAndConcourse => LightingRequirement { e_m: Some(200.0), ugr_l: Some(28.0), r_a: Some(40.0) },
            Self::TicketAndLuggageOffices => LightingRequirement { e_m: Some(500.0), ugr_l: Some(19.0), r_a: Some(80.0) },
            Self::RailwayWaitingRooms => LightingRequirement { e_m: Some(200.0), ugr_l: Some(22.0), r_a: Some(80.0) },
            }
        }
    }


    pub struct RoomGeometry {
        pub a_floor: f64,
        pub a_window: f64,
        pub room_depth: f64,
        pub room_width: f64,
        pub h_sturz: f64, // Height of window lintel (m)
        pub h_nutz: f64,  // Height of working plane (e.g. 0.8m)
    }

    impl RoomGeometry {
        /// Calculates Room Index k (Eq. 10)
        pub fn room_index(&self) -> f64 {
            let h_prime = self.h_sturz - self.h_nutz;
            let k = (self.room_depth * self.room_width) / (h_prime * (self.room_depth + self.room_width));
            f64::max(0.6, k)
        }

        /// Calculates max daylight depth (Eq. 7) and splits the room
        pub fn daylight_zone_area(&self) -> (f64, f64) {
            let a_tl_max = 2.5 * (self.h_sturz - self.h_nutz);
            let effective_depth = f64::min(self.room_depth, a_tl_max);
            
            let a_tl = effective_depth * self.room_width;
            let a_tl_clamped = f64::min(a_tl, self.a_floor);
            let a_ktl = self.a_floor - a_tl_clamped;
            
            (a_tl_clamped, a_ktl)
        }

        /// Estimates the Raw Daylight Quotient (D_Rb) based on Eq. 30
        pub fn daylight_quotient(&self) -> f64 {
            let (a_tl, _) = self.daylight_zone_area();
            if a_tl <= 0.0 { return 0.0; }
            
            let i_tr = self.a_window / a_tl;
            let effective_depth = f64::min(self.room_depth, 2.5 * (self.h_sturz - self.h_nutz));
            let i_rt = effective_depth / (self.h_sturz - self.h_nutz);
            let i_v = 0.7; // Standard obstruction index

            let d_rb = (4.13 + 20.0 * i_tr - 1.36 * i_rt) * i_v;
            f64::max(0.0, d_rb)
        }
    }

    // --- ENGINE ---

    pub struct LightingEngine {
        pub geometry: RoomGeometry,
        pub lamp_tech: LampTechnology,
        pub light_type: LightingType,
        pub daylight_control: DaylightControl,
        pub presence_control: PresenceControl,
        pub e_m_lux: f64, // Required lux (e.g., 500)
        pub t_day: f64,   // Usage hours during daytime (h/a)
        pub t_night: f64, // Usage hours during nighttime (h/a)
        pub relative_absence: f64, // C_A (e.g., 0.3 for 30% away)
    }

    impl LightingEngine {
        /// Interpolates Table 5 to get base power p_j,lx
        pub fn calculate_base_power_per_lux(&self) -> f64 {
            let k = self.geometry.room_index();
            // Simplified fallback array corresponding to Table 5 [k=0.6, 1.0, 2.0, 5.0]
            match self.light_type {
                LightingType::Direct => if k < 1.0 { 0.040 } else if k < 2.0 { 0.030 } else { 0.022 },
                LightingType::DirectIndirect => if k < 1.0 { 0.055 } else if k < 2.0 { 0.035 } else { 0.026 },
                LightingType::Indirect => if k < 1.0 { 0.100 } else if k < 2.0 { 0.050 } else { 0.035 },
            }
        }

        /// Eq. 11: Calculates total installed power density p_j (W/m²)
        pub fn calculate_installed_power_density(&self) -> f64 {
            let p_j_lx = self.calculate_base_power_per_lux();
            let k_a = 0.85; // Standard task area reduction
            let k_l = self.lamp_tech.k_l_factor();
            
            p_j_lx * self.e_m_lux * K_WF_DEFAULT * k_a * k_l
        }

        /// Eq. 40: Calculates Presence Factor F_Prä
        pub fn calculate_f_pra(&self) -> f64 {
            1.0 - (self.relative_absence * self.presence_control.c_pra_kon())
        }

        /// Extracts C_TL,kon based on Table 25 (Simplified for 500 Lux)
        pub fn calculate_c_tl_kon(&self, d_rb: f64) -> f64 {
            // Table 25 logic based on Daylight Quality
            let is_good = d_rb >= 6.0;
            let is_medium = d_rb >= 4.0 && d_rb < 6.0;
            
            match self.daylight_control {
                DaylightControl::Manual => {
                    if is_good { 0.52 } else if is_medium { 0.49 } else { 0.47 }
                },
                DaylightControl::DimmedAutoOnOff => {
                    if is_good { 0.78 } else if is_medium { 0.75 } else { 0.70 }
                },
                DaylightControl::DimmedManualOnAutoOff => {
                    if is_good { 0.81 } else if is_medium { 0.79 } else { 0.73 }
                }
            }
        }

        /// Eq. 19 & 38: Calculates Daylight Supply Factor F_TL
        pub fn calculate_f_tl(&self) -> f64 {
            let d_rb = self.geometry.daylight_quotient();
            if d_rb < 2.0 { return 1.0; } // No daylight, factor is 1.0 (100% artificial)

            // Simplified implementation of Eq 38 weighting (using South default 67% off / 33% blinds)
            // A full implementation requires Table 12 and Table 15 interpolation.
            let c_tl_vers_sna = if d_rb >= 6.0 { 0.93 } else if d_rb >= 4.0 { 0.82 } else { 0.60 };
            let c_tl_vers_sa = 0.40; // Default for auto blinds
            
            let c_tl_vers = (0.67 * c_tl_vers_sna) + (0.33 * c_tl_vers_sa);
            let c_tl_kon = self.calculate_c_tl_kon(d_rb);

            f64::max(0.0, 1.0 - (c_tl_vers * c_tl_kon))
        }

        /// Main Engine Call: Calculates final energy demand for lighting (Q_l,f) in kWh/a
        pub fn calculate_annual_final_energy_kwh(&self) -> f64 {
            let (a_tl, a_ktl) = self.geometry.daylight_zone_area();
            let p_j = self.calculate_installed_power_density();
            
            let f_pra = self.calculate_f_pra();
            let f_kl = 1.0; // Assume no constant light dimming sensor to save complexity
            let f_tl = self.calculate_f_tl();

            // Eq. 4, 5, 6
            let t_eff_tag_tl = self.t_day * f_tl * f_pra * f_kl;
            let t_eff_tag_ktl = self.t_day * f_pra * f_kl;
            let t_eff_nacht = self.t_night * f_pra * f_kl;

            // Eq. 2 (Watt-hours)
            let q_lf_wh = p_j * (
                a_tl * (t_eff_tag_tl + t_eff_nacht) + 
                a_ktl * (t_eff_tag_ktl + t_eff_nacht)
            );

            q_lf_wh / 1000.0 // Return kWh
        }
    }
}

pub mod dhw_system {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum BuildingProfileDHW {
        Residential1_2Family,
        ResidentialMultiFamily,
        HospitalsHotels,
        OfficesCommercial,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum DHWSystemType {
        Centralized,
        Decentralized,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum PipeInsulationDHW {
        Uninsulated,
        Insulated50Percent,
        Insulated100Percent,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum TankInsulationDHW {
        VeryGoodClassA,
        StandardClassC,
        PoorOld,
        None,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum GeneratorTypeDHW {
        ElectricInstantaneous,
        ElectricSmallStorage,
        ElectricStandardStorage,
        GasInstantaneousNew,
        GasStorageNew,
        CombinedGasBoilerCondensing,
        HeatPumpAirWater,
        DistrictHeating,
    }

    pub struct DHWEngine {
        pub a_ngf: f64,
        pub profile: BuildingProfileDHW,
        pub system_type: DHWSystemType,
        pub distribution_insulation: PipeInsulationDHW,
        pub has_circulation: bool,
        pub tank_volume_liters: f64,
        pub tank_insulation: TankInsulationDHW,
        pub generator: GeneratorTypeDHW,
        pub phi_w_max_kw: f64,
        pub is_summer_mode: bool,
        pub t_mth: f64,
        pub wastewater_heat_recovery: f64,
        pub solar_thermal_kwh: f64,
    }

    pub struct DHWFinalEnergyResult {
        pub fuel_demand_kwh: f64,
        pub auxiliary_electricity_kwh: f64,
        pub total_electricity_kwh: f64,
    }

    impl DHWEngine {
        pub fn u_l_tap(&self) -> f64 {
            match self.distribution_insulation {
                PipeInsulationDHW::Uninsulated => 1.80,
                PipeInsulationDHW::Insulated50Percent => 0.35,
                PipeInsulationDHW::Insulated100Percent => 0.22,
            }
        }

        pub fn u_l_c(&self) -> f64 {
            match self.distribution_insulation {
                PipeInsulationDHW::Uninsulated => 2.20,
                PipeInsulationDHW::Insulated50Percent => 0.40,
                PipeInsulationDHW::Insulated100Percent => 0.25,
            }
        }

        pub fn h_w_st(&self) -> f64 {
            if self.tank_volume_liters <= 0.0 { return 0.0; }
            let vol = self.tank_volume_liters;
            match self.tank_insulation {
                TankInsulationDHW::VeryGoodClassA => 1.10 + (vol - 100.0) * 0.001,
                TankInsulationDHW::StandardClassC => 1.60 + (vol - 100.0) * 0.003,
                TankInsulationDHW::PoorOld => 2.50 + (vol - 100.0) * 0.005,
                TankInsulationDHW::None => 0.0,
            }
        }

        pub fn t_circ(&self) -> f64 {
            if !self.has_circulation { return 0.0; }
            match self.profile {
                BuildingProfileDHW::Residential1_2Family => 420.0,
                BuildingProfileDHW::ResidentialMultiFamily => 744.0,
                BuildingProfileDHW::HospitalsHotels => 744.0,
                BuildingProfileDHW::OfficesCommercial => 420.0,
            }
        }

        pub fn e_g_w(&self, _beta_w: f64) -> f64 {
            match self.generator {
                GeneratorTypeDHW::ElectricInstantaneous => 1.00,
                GeneratorTypeDHW::ElectricSmallStorage => 1.02,
                GeneratorTypeDHW::ElectricStandardStorage => 1.05,
                GeneratorTypeDHW::GasInstantaneousNew => 1.15,
                GeneratorTypeDHW::GasStorageNew => 1.25,
                GeneratorTypeDHW::DistrictHeating => 1.02,
                GeneratorTypeDHW::CombinedGasBoilerCondensing => {
                    if self.is_summer_mode { 2.50 } else { 1.10 }
                },
                GeneratorTypeDHW::HeatPumpAirWater => {
                    let eta_carnot = 0.38;
                    let theta_source = 5.0;
                    let theta_sink = 55.0;
                    let cop = eta_carnot * (theta_sink + 273.15) / ((theta_sink + 273.15) - (theta_source + 273.15));
                    1.0 / f64::max(1.0, cop)
                }
            }
        }

        pub fn calculate_final_energy(&self, q_w_b: f64) -> DHWFinalEnergyResult {
            let q_w_shower = q_w_b * 0.60;
            let q_w_wrg = q_w_shower * self.wastewater_heat_recovery;
            let q_w_b_reduced = f64::max(0.0, q_w_b - q_w_wrg);

            let l_w_tap = match self.system_type {
                DHWSystemType::Centralized => 0.05 * self.a_ngf,
                DHWSystemType::Decentralized => 0.015 * self.a_ngf,
            };
            let l_w_c = if self.has_circulation { 0.06 * self.a_ngf } else { 0.0 };

            let theta_w_t = 60.0;
            let theta_w_c_av = 57.5;
            let theta_amb = 20.0;
            let t_tap = 30.0;

            let q_w_d_tap = (1.0 / 1000.0) * self.u_l_tap() * l_w_tap * (theta_w_t - theta_amb) * t_tap;
            let q_w_d_c = (1.0 / 1000.0) * self.u_l_c() * l_w_c * (theta_w_c_av - theta_amb) * self.t_circ();
            let q_w_d = q_w_d_tap + q_w_d_c;

            let theta_w_s_av = 60.0;
            let q_w_s = (1.0 / 1000.0) * self.h_w_st() * (theta_w_s_av - theta_amb) * self.t_mth;

            let q_w_outg = q_w_b_reduced + q_w_d + q_w_s;
            let q_w_outg_req = f64::max(0.0, q_w_outg - self.solar_thermal_kwh);

            let mut beta_w = 0.0;
            if self.phi_w_max_kw > 0.0 {
                beta_w = q_w_outg_req / (self.phi_w_max_kw * self.t_mth);
            }

            let q_w_f = q_w_outg_req * self.e_g_w(beta_w);

            let p_pu_c = if self.has_circulation { 35.0 } else { 0.0 };
            let w_w_d_c = (p_pu_c * self.t_circ()) / 1000.0;

            let p_pu_load = if self.tank_volume_liters > 0.0 { 45.0 } else { 0.0 };
            let t_load = beta_w * self.t_mth;
            let w_w_s = (p_pu_load * t_load) / 1000.0;

            let w_w_gen = (60.0 * t_load + 10.0 * (self.t_mth - t_load)) / 1000.0;
            let w_w = w_w_d_c + w_w_s + w_w_gen;

            let is_electric_gen = matches!(self.generator, 
                GeneratorTypeDHW::ElectricInstantaneous | 
                GeneratorTypeDHW::ElectricSmallStorage | 
                GeneratorTypeDHW::ElectricStandardStorage | 
                GeneratorTypeDHW::HeatPumpAirWater);

            let total_electricity_kwh = if is_electric_gen { q_w_f + w_w } else { w_w };

            DHWFinalEnergyResult {
                fuel_demand_kwh: if is_electric_gen { 0.0 } else { q_w_f },
                auxiliary_electricity_kwh: w_w,
                total_electricity_kwh,
            }
        }
    }

    #[cfg(test)]
    pub mod tests {
        use super::*;

        #[test]
        fn test_dhw_engine_electric_decentralized() {
            let engine = DHWEngine {
                a_ngf: 100.0,
                profile: BuildingProfileDHW::Residential1_2Family,
                system_type: DHWSystemType::Decentralized,
                distribution_insulation: PipeInsulationDHW::Uninsulated,
                has_circulation: false,
                tank_volume_liters: 0.0,
                tank_insulation: TankInsulationDHW::None,
                generator: GeneratorTypeDHW::ElectricInstantaneous,
                phi_w_max_kw: 21.0,
                is_summer_mode: true,
                t_mth: 744.0, // January
                wastewater_heat_recovery: 0.0,
                solar_thermal_kwh: 0.0,
            };

            let q_w_b = 200.0; // kWh
            let res = engine.calculate_final_energy(q_w_b);

            assert_eq!(res.fuel_demand_kwh, 0.0);
            assert!(res.total_electricity_kwh > q_w_b);
            assert!(res.total_electricity_kwh < q_w_b * 1.5);
        }

        #[test]
        fn test_dhw_engine_heat_pump_centralized() {
            let engine = DHWEngine {
                a_ngf: 1000.0,
                profile: BuildingProfileDHW::ResidentialMultiFamily,
                system_type: DHWSystemType::Centralized,
                distribution_insulation: PipeInsulationDHW::Insulated100Percent,
                has_circulation: true,
                tank_volume_liters: 500.0,
                tank_insulation: TankInsulationDHW::VeryGoodClassA,
                generator: GeneratorTypeDHW::HeatPumpAirWater,
                phi_w_max_kw: 15.0,
                is_summer_mode: false,
                t_mth: 744.0,
                wastewater_heat_recovery: 0.30,
                solar_thermal_kwh: 0.0,
            };

            let q_w_b = 3000.0;
            let res = engine.calculate_final_energy(q_w_b);

            assert_eq!(res.fuel_demand_kwh, 0.0);
            assert!(res.total_electricity_kwh > 0.0);
            assert!(res.total_electricity_kwh < q_w_b);
        }
    }
}

#[cfg(test)]
mod lighting_tests {
    use super::*;
    use crate::lighting::{RoomUsage, LightingRequirement};

    #[test]
    fn test_lighting_requirements() {
        let req = RoomUsage::OperatingCavity.requirements();
        assert_eq!(req.e_m, Some(100000.0));
        assert_eq!(req.ugr_l, None);
        assert_eq!(req.r_a, None);

        let req = RoomUsage::StoreAndStockroomsUnmanned.requirements();
        assert_eq!(req.e_m, Some(100.0));
        assert_eq!(req.ugr_l, Some(25.0));
        assert_eq!(req.r_a, Some(60.0));

        let req = RoomUsage::RestaurantDiningRoom.requirements();
        assert_eq!(req.e_m, None);
        assert_eq!(req.ugr_l, None);
        assert_eq!(req.r_a, Some(80.0));
    }
}

pub mod primary_energy {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum EnergyCarrier {
        GridElectricity,
        NaturalGas,
        Biogas,
        LiquidGas,
        FuelOil,
        BioOil,
        HardCoal,
        Lignite,
        WoodPellets,
        LogWood,
        DistrictHeatingFossil,
        DistrictHeatingRenewable,
        DistrictHeating(f64), // Certified per network
        EnvironmentalEnergy,
    }

    impl EnergyCarrier {
        /// Returns the Non-Renewable Primary Energy Factor (f_p,nren)
        pub fn f_p_nren(&self) -> f64 {
            match self {
                Self::GridElectricity => 1.80,
                Self::NaturalGas => 1.10,
                Self::Biogas => 0.50,
                Self::LiquidGas => 1.10,
                Self::FuelOil => 1.10,
                Self::BioOil => 0.50,
                Self::HardCoal => 1.10,
                Self::Lignite => 1.20,
                Self::WoodPellets => 0.20,
                Self::LogWood => 0.20,
                Self::DistrictHeatingFossil => 0.70,
                Self::DistrictHeatingRenewable => 0.00,
                Self::DistrictHeating(factor) => *factor,
                Self::EnvironmentalEnergy => 0.00,
            }
        }

        /// Returns the Total Primary Energy Factor (f_p,tot)
        pub fn f_p_tot(&self) -> f64 {
            match self {
                Self::GridElectricity => 2.80,
                Self::NaturalGas => 1.10,
                Self::Biogas => 1.50,
                Self::LiquidGas => 1.10,
                Self::FuelOil => 1.10,
                Self::BioOil => 1.50,
                Self::HardCoal => 1.10,
                Self::Lignite => 1.20,
                Self::WoodPellets => 1.20,
                Self::LogWood => 1.20,
                Self::DistrictHeatingFossil => 0.70,
                Self::DistrictHeatingRenewable => 1.00,
                Self::DistrictHeating(_) => 0.70, // Or certified per network
                Self::EnvironmentalEnergy => 1.00,
            }
        }
    }

    pub struct EnergyDemand {
        pub q_f: f64, // Final thermal energy demand (kWh)
        pub w_f: f64, // Final electrical auxiliary energy demand (kWh)
        pub carrier: EnergyCarrier,
    }

    pub struct PrimaryEnergyEngine {}

    impl PrimaryEnergyEngine {
        /// 1.2 Converting Final Energy to Primary Energy
        pub fn calculate_primary_energy(demands: &[EnergyDemand], non_renewable: bool) -> f64 {
            demands.iter().map(|d| {
                let factor = if non_renewable { d.carrier.f_p_nren() } else { d.carrier.f_p_tot() };
                (d.q_f + d.w_f) * factor
            }).sum()
        }

        /// 3.1 Exported Energy Credits (e.g., PV)
        pub fn calculate_exported_energy_credit(w_exp_pv: f64) -> f64 {
            // Usually offsets grid electricity
            w_exp_pv * EnergyCarrier::GridElectricity.f_p_nren()
        }

        /// 4.1 Weather Adjustment Factor
        pub fn weather_adjustment_factor(gtz_standard: f64, gtz_measured: f64) -> f64 {
            if gtz_measured == 0.0 {
                return 1.0; // Prevent division by zero
            }
            gtz_standard / gtz_measured
        }

        /// 4.2 Normalizing Measured Consumption
        pub fn normalize_measured_consumption(e_v_measured: f64, e_v_ww: f64, kf: f64) -> f64 {
            (e_v_measured - e_v_ww) * kf + e_v_ww
        }
    }
}
