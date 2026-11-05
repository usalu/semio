//! 🎚️ `Viewport3dWindowConfig`: the persisted, local-only customisation of one 3D viewport window; schema twins live in [`schema`](self).

use super::scene::{LoadKind, LocalizedCopy, Vec3};
use super::*;
use semio_plugin::{World3dColorRamp, World3dText};

macro_rules! config_choices {
    ($($item:item)*) => {$(
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
        #[value(rename_all = "camelCase")]
        $item
    )*};
}

macro_rules! config_records {
    ($($item:item)*) => {$(
        #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
        #[value(rename_all = "camelCase")]
        $item
    )*};
}

config_choices! {
    /// 🎥️ Named camera direction relative to the scene bounds.
    pub enum CameraPreset {
        #[default]
        Iso,
        Plan,
        ElevationFront,
        ElevationBack,
        ElevationLeft,
        ElevationRight,
    }

    /// 🌈️ Ramp the utilisation colour map samples.
    pub enum ColourRamp {
        #[default]
        Viridis,
        Inferno,
        Coolwarm,
        Grayscale,
    }

    /// 🏷️ How many annotation labels are drawn.
    pub enum LabelDensity {
        Off,
        Low,
        #[default]
        Normal,
        High,
    }

    /// 📏️ Unit dimension labels are written in.
    pub enum LengthUnit {
        Millimetre,
        #[default]
        Metre,
    }

    /// 📱️ Device class driving level of detail, instance budget and layout.
    pub enum DeviceClass {
        #[default]
        Desktop,
        Mobile,
        Tablet,
    }
}

/// 🗣️ Language of resolved instance labels; scenes themselves carry both. It has no default: it always follows the session locale through [`Lang::of`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum Lang {
    En,
    De,
}

impl Lang {
    /// 🗣️ The language of a session locale, matched exhaustively.
    pub fn of(locale: semio_ui_locale::Locale) -> Self {
        match locale {
            semio_ui_locale::Locale::En => Self::En,
            semio_ui_locale::Locale::De => Self::De,
        }
    }
}

config_records! {
    /// ✂️ Section plane with optional cap.
    pub struct SectionPlane {
        pub origin: Vec3,
        pub normal: Vec3,
        pub cap: bool,
    }

    /// 📌️ A saved camera view.
    pub struct SavedView {
        pub name: LocalizedCopy,
        pub position: Vec3,
        pub target: Vec3,
    }

    /// 🎛️ Everything a user may customise about one viewport window.
    pub struct Viewport3dWindowConfig {
        pub preset: CameraPreset,
        pub ramp: ColourRamp,
        pub domain_max: f64,
        pub sun: bool,
        pub hidden_layers: Vec<String>,
        pub solo_layer: Option<String>,
        pub hidden_load_kinds: Vec<LoadKind>,
        pub load_glyph_scale: f64,
        pub deformation_scale: f64,
        pub deformation_phase: f64,
        pub playback_speed: f64,
        pub active_field: Option<String>,
        pub section: Option<SectionPlane>,
        pub label_density: LabelDensity,
        pub show_dimensions: bool,
        pub unit: LengthUnit,
        pub device: DeviceClass,
        pub saved_views: Vec<SavedView>,
    }
}

impl Default for Viewport3dWindowConfig {
    fn default() -> Self {
        Self {
            preset: CameraPreset::Iso,
            ramp: ColourRamp::Viridis,
            domain_max: 1.2,
            sun: true,
            hidden_layers: Vec::new(),
            solo_layer: None,
            hidden_load_kinds: Vec::new(),
            load_glyph_scale: 1.0,
            deformation_scale: 1.0,
            deformation_phase: 1.0,
            playback_speed: 1.0,
            active_field: None,
            section: None,
            label_density: LabelDensity::Normal,
            show_dimensions: true,
            unit: LengthUnit::Metre,
            device: DeviceClass::Desktop,
            saved_views: Vec::new(),
        }
    }
}

impl ColourRamp {
    pub fn framework(self) -> World3dColorRamp {
        match self {
            Self::Viridis => World3dColorRamp::Viridis,
            Self::Inferno => World3dColorRamp::Inferno,
            Self::Coolwarm => World3dColorRamp::Coolwarm,
            Self::Grayscale => World3dColorRamp::Grayscale,
        }
    }
}

/// 📱️ Level of detail, instance budget and label cap of a device class: desktop first, then mobile, then tablet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeviceProfile {
    pub lod: super::parts::Lod,
    pub instance_budget: usize,
    pub max_label_density: LabelDensity,
}

impl DeviceClass {
    pub fn profile(self) -> DeviceProfile {
        match self {
            Self::Desktop => DeviceProfile { lod: super::parts::Lod::FULL, instance_budget: 20_000, max_label_density: LabelDensity::High },
            Self::Mobile => DeviceProfile { lod: super::parts::Lod::COARSE, instance_budget: 3_000, max_label_density: LabelDensity::Low },
            Self::Tablet => DeviceProfile { lod: super::parts::Lod::REDUCED, instance_budget: 8_000, max_label_density: LabelDensity::Normal },
        }
    }
}

impl LabelDensity {
    pub fn rank(self) -> u8 {
        match self {
            Self::Off => 0,
            Self::Low => 1,
            Self::Normal => 2,
            Self::High => 3,
        }
    }
}

impl Viewport3dWindowConfig {
    /// 🧪️ Whether the config is internally consistent: positive domain and scales, finite phase, a non-degenerate section normal.
    pub fn validate(&self) -> Result<(), String> {
        let ok = self.domain_max > 0.0 && self.load_glyph_scale > 0.0 && self.deformation_scale.is_finite() && self.deformation_phase.is_finite() && self.playback_speed > 0.0 && self.playback_speed.is_finite()
            && self.section.as_ref().is_none_or(|s| s.normal.length() > 0.0);
        if ok { Ok(()) } else { Err("viewport-config-out-of-range".into()) }
    }

    pub fn legend_title() -> World3dText {
        World3dText::new("Utilisation", "Ausnutzung")
    }
}
