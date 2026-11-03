//! 🎨️ Theme bundle schema: a named, shareable, local-only customisation of colour maps, palettes, edges and motion; schema twins live in [`schema`](self).

use crate::viewport3d::config::LabelDensity;
use semio_s_plugin_norm_structural_scene::{LocalizedCopy, Rgb};

macro_rules! theme_choices {
    ($($item:item)*) => {$(
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
        #[value(rename_all = "camelCase")]
        $item
    )*};
}

macro_rules! theme_records {
    ($($item:item)*) => {$(
        #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
        #[value(rename_all = "camelCase")]
        $item
    )*};
}

theme_choices! {
    /// 🌗️ Base appearance a theme starts from.
    pub enum ThemeVariant {
        #[default]
        Light,
        Dark,
        HighContrast,
    }

    /// 🖍️ Outline weight of scene entities.
    pub enum EdgeStyle {
        None,
        #[default]
        Thin,
        Bold,
    }

    /// 🎞️ Whether animation follows the system, is forced off or forced on.
    pub enum MotionPolicy {
        #[default]
        System,
        Reduced,
        Full,
    }
}

theme_records! {
    /// 🎯️ Ramp id (see the colour maps) used per result field.
    pub struct ResultRamps {
        pub stress: String,
        pub temperature: String,
        pub pressure: String,
        pub settlement: String,
    }

    /// 🧱️ Colour override of one material role (`steel`, `concrete`, … as in the scene).
    pub struct RoleColour {
        pub role: String,
        pub rgb: Rgb,
    }

    /// 🎨️ A theme bundle: selects variant, colour maps, role palette, background, edges, label density, lighting, motion and text scale. Hatch and glyph redundancy are not switchable.
    pub struct ThemeBundle {
        pub id: String,
        pub name: LocalizedCopy,
        pub variant: ThemeVariant,
        pub utilisation_ramp: String,
        pub result_ramps: ResultRamps,
        pub domain_max: f64,
        pub background: Option<Rgb>,
        pub role_colours: Vec<RoleColour>,
        pub edge_style: EdgeStyle,
        pub label_density: LabelDensity,
        pub sun: bool,
        pub motion: MotionPolicy,
        pub text_scale: f64,
    }
}
