//! 🎨️ Themes of the visual windows: built-in light, dark and high-contrast bundles, validation of user bundles and their accessibility findings.
//!
//! 🔗️ Bundles are plain files in the schema of [`schema`]; colour maps come from `semio_s_plugin_norm_structural_scene::colour_maps`.

use semio_s_plugin_norm_structural_scene::colour_maps::{metrics, ramp, MapKind, RampId, ResultField, Variant};
use semio_s_plugin_norm_structural_scene::{LocalizedCopy, Rgb};

#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
pub use schema::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

impl ThemeVariant {
    pub fn variant(self) -> Variant {
        match self {
            Self::Light => Variant::Light,
            Self::Dark => Variant::Dark,
            Self::HighContrast => Variant::HighContrast,
        }
    }
}

fn builtin(id: &str, en: &str, de: &str, variant: ThemeVariant) -> ThemeBundle {
    ThemeBundle {
        id: id.into(),
        name: LocalizedCopy::new(en, de),
        variant,
        utilisation_ramp: "cool".into(),
        result_ramps: ResultRamps { stress: "diverging".into(), temperature: "warm".into(), pressure: "diverging".into(), settlement: "cool".into() },
        domain_max: 1.2,
        background: None,
        role_colours: Vec::new(),
        edge_style: if variant == ThemeVariant::HighContrast { EdgeStyle::Bold } else { EdgeStyle::Thin },
        label_density: crate::viewport3d::config::LabelDensity::Normal,
        sun: variant != ThemeVariant::HighContrast,
        motion: MotionPolicy::System,
        text_scale: if variant == ThemeVariant::HighContrast { 1.25 } else { 1.0 },
    }
}

/// 📦️ The built-in themes: light, dark and high contrast.
pub fn builtin_themes() -> Vec<ThemeBundle> {
    vec![builtin("light", "Light", "Hell", ThemeVariant::Light), builtin("dark", "Dark", "Dunkel", ThemeVariant::Dark), builtin("high-contrast", "High contrast", "Hoher Kontrast", ThemeVariant::HighContrast)]
}

/// 🧪️ Structural rule violations of a bundle as `path: rule` lines; empty when the bundle can be applied.
pub fn validate(theme: &ThemeBundle) -> Vec<String> {
    let mut out = Vec::new();
    let mut rule = |path: &str, ok: bool, text: &str| if !ok { out.push(format!("{path}: {text}")) };
    rule("id", !theme.id.is_empty() && theme.id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'), "id-kebab-case");
    rule("name", !theme.name.en.is_empty() && !theme.name.de.is_empty(), "name-both-languages");
    rule("utilisationRamp", RampId::parse(&theme.utilisation_ramp).is_some_and(|r| ramp(r, Variant::Light).kind == MapKind::Sequential), "ramp-not-sequential");
    let r = &theme.result_ramps;
    for (path, id, kind) in [("resultRamps.stress", &r.stress, MapKind::Diverging), ("resultRamps.pressure", &r.pressure, MapKind::Diverging), ("resultRamps.temperature", &r.temperature, MapKind::Sequential), ("resultRamps.settlement", &r.settlement, MapKind::Sequential)] {
        rule(path, RampId::parse(id).is_some_and(|r| ramp(r, Variant::Light).kind == kind), "ramp-wrong-kind");
    }
    rule("domainMax", theme.domain_max.is_finite() && theme.domain_max > 1.0, "domain-max-not-above-one");
    rule("textScale", theme.text_scale.is_finite() && (0.75..=3.0).contains(&theme.text_scale), "text-scale-out-of-range");
    for (i, c) in theme.role_colours.iter().enumerate() {
        rule(&format!("roleColours[{i}].role"), !c.role.is_empty() && theme.role_colours[..i].iter().all(|p| p.role != c.role), "role-empty-or-duplicate");
    }
    out
}

fn array(c: &Rgb) -> [u8; 3] {
    [c.r, c.g, c.b]
}

/// ♿️ Accessibility findings of a valid bundle: ramps whose steps fall under the contrast floor against the chosen background.
pub fn accessibility_findings(theme: &ThemeBundle) -> Vec<String> {
    let variant = theme.variant.variant();
    let bg = theme.background.as_ref().map_or(variant.background(), array);
    let mut out = Vec::new();
    for (path, id) in [("utilisationRamp", &theme.utilisation_ramp), ("resultRamps.stress", &theme.result_ramps.stress), ("resultRamps.temperature", &theme.result_ramps.temperature), ("resultRamps.pressure", &theme.result_ramps.pressure), ("resultRamps.settlement", &theme.result_ramps.settlement)] {
        let Some(id) = RampId::parse(id) else { continue };
        let r = ramp(id, variant);
        let n = 9;
        let worst = r.samples(n).iter().enumerate().filter(|(i, _)| r.kind != MapKind::Diverging || (*i as f64 / (n - 1) as f64 - 0.5).abs() >= 0.2).map(|(_, s)| metrics::contrast_ratio(metrics::to_srgb(*s), metrics::to_srgb(bg))).fold(f64::INFINITY, f64::min);
        if worst < variant.contrast_floor() {
            out.push(format!("{path}: contrast {worst:.2} below {:.1}", variant.contrast_floor()));
        }
    }
    let fail = ramp(RampId::Failure, variant);
    let worst = fail.samples(5).iter().map(|s| metrics::contrast_ratio(metrics::to_srgb(*s), metrics::to_srgb(bg))).fold(f64::INFINITY, f64::min);
    if worst < variant.contrast_floor() {
        out.push(format!("failure: contrast {worst:.2} below {:.1}", variant.contrast_floor()));
    }
    out
}

/// 🔤️ The colour-map field a result field uses in a theme.
pub fn ramp_for(theme: &ThemeBundle, field: ResultField) -> Option<RampId> {
    RampId::parse(match field {
        ResultField::Stress => &theme.result_ramps.stress,
        ResultField::Temperature => &theme.result_ramps.temperature,
        ResultField::Pressure => &theme.result_ramps.pressure,
        ResultField::Settlement => &theme.result_ramps.settlement,
    })
}

/// ♿️ The theme in force: the user's bundle, unless forced colours or "more contrast" demand high contrast and the bundle is not high contrast itself; without a bundle the built-in theme of the preferred variant.
pub fn effective_theme(prefs: crate::accessibility::A11yPrefs, chosen: Option<&ThemeBundle>) -> ThemeBundle {
    let demanded = crate::accessibility::resolve_variant(prefs);
    match chosen {
        Some(theme) if demanded != Variant::HighContrast || theme.variant == ThemeVariant::HighContrast => theme.clone(),
        _ => builtin_themes().into_iter().find(|theme| theme.variant.variant() == demanded).expect("a built-in theme per variant"),
    }
}

/// 🎞️ Whether animation may run: the theme's policy wins over the system preference.
pub fn motion_allowed(prefs: crate::accessibility::A11yPrefs, theme: &ThemeBundle) -> bool {
    match theme.motion {
        MotionPolicy::Reduced => false,
        MotionPolicy::Full => true,
        MotionPolicy::System => crate::accessibility::animations_allowed(prefs),
    }
}
