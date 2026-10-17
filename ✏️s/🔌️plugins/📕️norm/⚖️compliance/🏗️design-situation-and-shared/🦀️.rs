//! 🏗️ Design situations and shared classifications of the norm compliance core: the consequence classes, imposed categories, limit states, climate zones and occupancies. See [`super`].


// #region 🔖️DesignSituation
/// 🏗️ Design situation per EN 1990 Table A1.1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub enum DesignSituation {
    #[dsl(key = "persistent")]
    Persistent,
    #[dsl(key = "transient")]
    Transient,
    #[dsl(key = "accidental")]
    Accidental,
    #[dsl(key = "seismic")]
    Seismic,
}

/// 📋️ Consequence class per EN 1990.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub enum ConsequenceClass {
    Cc1,
    Cc2,
    Cc3,
}

impl ConsequenceClass {
    pub fn as_u8(self) -> u8 {
        match self {
            Self::Cc1 => 1,
            Self::Cc2 => 2,
            Self::Cc3 => 3,
        }
    }
}

/// 📊️ Variable action category per EN 1991-1-1 Table 6.1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub enum ImposedCategory {
    #[dsl(key = "a")]
    A,
    #[dsl(key = "b")]
    B,
    #[dsl(key = "c")]
    C,
    #[dsl(key = "d")]
    D,
    #[dsl(key = "e")]
    E,
    #[dsl(key = "f")]
    F,
    #[dsl(key = "g")]
    G,
    #[dsl(key = "h")]
    H,
}

impl ImposedCategory {
    pub fn q_k_kn_m2(self) -> f64 {
        match self {
            Self::A => 2.0,
            Self::B => 2.5,
            Self::C => 3.0,
            Self::D => 4.0,
            Self::E => 5.0,
            Self::F => 3.0,
            Self::G => 5.0,
            Self::H => 20.0,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::A => "residential",
            Self::B => "office",
            Self::C => "congregation",
            Self::D => "retail",
            Self::E => "storage",
            Self::F => "traffic_light",
            Self::G => "traffic_heavy",
            Self::H => "roof",
        }
    }
}
// #endregion 🔖️DesignSituation

// #region 🔖️Shared
/// ⚖️ Limit state per EN 1990.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub enum LimitState {
    Uls,
    Sls,
    Als,
    Fls,
}

/// ⏱️ Load duration class for timber and similar materials.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub enum LoadDuration {
    Permanent,
    Long,
    Medium,
    Short,
    Instantaneous,
}

/// 🌡️ Reference climate zone for thermal norms (Germany).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub enum ClimateZoneDe {
    #[dsl(key = "zone1")]
    Zone1,
    #[dsl(key = "zone2")]
    Zone2,
    #[dsl(key = "zone3")]
    Zone3,
    #[dsl(key = "zone4")]
    Zone4,
}

impl ClimateZoneDe {
    pub fn design_external_temperature_c(self) -> f64 {
        match self {
            Self::Zone1 => -16.0,
            Self::Zone2 => -14.0,
            Self::Zone3 => -12.0,
            Self::Zone4 => -10.0,
        }
    }

    pub fn summer_design_temperature_c(self) -> f64 {
        match self {
            Self::Zone1 => 26.0,
            Self::Zone2 => 28.0,
            Self::Zone3 => 30.0,
            Self::Zone4 => 32.0,
        }
    }

    pub fn heating_degree_days(self) -> f64 {
        match self {
            Self::Zone1 => 3800.0,
            Self::Zone2 => 3200.0,
            Self::Zone3 => 2600.0,
            Self::Zone4 => 2000.0,
        }
    }
}

/// 🏠️ Occupancy type for indoor environment norms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub enum OccupancyType {
    #[dsl(key = "residential")]
    Residential,
    #[dsl(key = "office")]
    Office,
    #[dsl(key = "classroom")]
    Classroom,
    #[dsl(key = "retail")]
    Retail,
    #[dsl(key = "meeting")]
    Meeting,
    #[dsl(key = "kitchen")]
    Kitchen,
    #[dsl(key = "corridor")]
    Corridor,
}
// #endregion 🔖️Shared
