//! 📐️ Quantities, clauses and localized text of the norm compliance core. See [`super`].

use super::*;

// #region 🔖️Quantity
/// 📐️ Physical quantity kind for SI-normalized norm computations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub enum QuantityKind {
    Dimensionless,
    Length,
    Area,
    Volume,
    Mass,
    Time,
    Temperature,
    Force,
    Pressure,
    Stress,
    Moment,
    Energy,
    Power,
    ThermalConductivity,
    ThermalResistance,
    HeatTransferCoefficient,
    AirPermeability,
    VentilationRate,
    Acceleration,
    Velocity,
}

/// 📊️ A scalar value tagged with its physical quantity kind (SI units).
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub struct Quantity {
    pub kind: QuantityKind,
    pub value: f64,
}

impl Quantity {
    pub fn new(kind: QuantityKind, value: f64) -> Self {
        Self { kind, value }
    }

    pub fn length_m(value: f64) -> Self {
        Self::new(QuantityKind::Length, value)
    }

    pub fn area_m2(value: f64) -> Self {
        Self::new(QuantityKind::Area, value)
    }

    pub fn force_kn(value: f64) -> Self {
        Self::new(QuantityKind::Force, value * 1_000.0)
    }

    pub fn stress_mpa(value: f64) -> Self {
        Self::new(QuantityKind::Stress, value * 1_000_000.0)
    }

    pub fn thermal_resistance_m2k_w(value: f64) -> Self {
        Self::new(QuantityKind::ThermalResistance, value)
    }

    pub fn u_value_w_m2k(value: f64) -> Self {
        Self::new(QuantityKind::HeatTransferCoefficient, value)
    }

    pub fn acceleration_m_s2(value: f64) -> Self {
        Self::new(QuantityKind::Acceleration, value)
    }

    pub fn velocity_m_s(value: f64) -> Self {
        Self::new(QuantityKind::Velocity, value)
    }
}
// #endregion 🔖️Quantity

#[path = "🔢️units/🦀️.rs"]
mod units;
pub use units::*;

// #region 🔖️Clause
/// 📑️ Stable clause identifier within a norm family (e.g. `EN 1992-1-1` §6.1).
#[derive(Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub struct ClauseId {
    pub family: String,
    pub part: String,
    pub section: String,
}

impl ClauseId {
    pub fn new(family: impl Into<String>, part: impl Into<String>, section: impl Into<String>) -> Self {
        Self { family: family.into(), part: part.into(), section: section.into() }
    }
}

/// 🌐️ Bilingual designator vocabulary of clause labels: each row is the canonical English word and its German edition; a label such as `Annex B` or `EN 772-1 Table A.1` is stored once and rendered per locale by [`designator`].
pub const DESIGNATOR_WORDS: &[(&str, &str)] = &[("Annex", "Anhang"), ("Tables", "Tabellen"), ("Table", "Tabelle"), ("Figure", "Bild"), ("Eq.", "Gl."), ("Expression", "Ausdruck"), ("Design case", "Bemessungsfall"), ("Section", "Abschnitt")];

/// 🏷️ A clause or table designator in a locale: every whole word of [`DESIGNATOR_WORDS`] is translated, numbers, letters, standard codes and symbols stay.
pub fn designator(label: &str, locale: &semio_framework_ui_locale::Locale) -> String {
    match locale {
        semio_framework_ui_locale::Locale::En => label.to_owned(),
        semio_framework_ui_locale::Locale::De => {
            let mut out = String::with_capacity(label.len());
            let mut rest = label;
            'scan: while !rest.is_empty() {
                let at_word_start = out.chars().last().map_or(true, |c| !c.is_alphanumeric());
                if at_word_start {
                    for (en, de) in DESIGNATOR_WORDS {
                        if let Some(tail) = rest.strip_prefix(en) {
                            if tail.chars().next().map_or(true, |c| !c.is_alphanumeric()) {
                                out.push_str(de);
                                rest = tail;
                                continue 'scan;
                            }
                        }
                    }
                }
                let mut chars = rest.chars();
                out.extend(chars.next());
                rest = chars.as_str();
            }
            out
        }
    }
}

/// 🅰️ The canonical label of an annex, `Annex B`; rendered `Anhang B` in German by [`designator`].
pub fn annex(letter: &str) -> String {
    format!("Annex {letter}")
}

impl ClauseId {
    /// 🔗️ The clause as one line in a locale, `EN 1993 1-1 §6.2.5` / `EN 1993 1-1 Abschn. 6.2.5`, with annex and table words translated.
    pub fn text(&self, locale: &semio_framework_ui_locale::Locale) -> String {
        let (family, part, section) = (designator(&self.family, locale), designator(&self.part, locale), designator(&self.section, locale));
        match locale {
            semio_framework_ui_locale::Locale::En => format!("{family} {part} §{section}"),
            semio_framework_ui_locale::Locale::De => format!("{family} {part} Abschn. {section}"),
        }
    }
}

/// 📍️ The source locator of an NDP row in a locale: `Annex B` as `Anhang B`, `EN 772-1 Table A.1` as `EN 772-1 Tabelle A.1`.
pub fn locator_text(source: &NdpSource, locale: &semio_framework_ui_locale::Locale) -> String {
    designator(source.locator, locale)
}

impl fmt::Display for ClauseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} §{}", self.family, self.part, self.section)
    }
}

#[cfg(test)]
mod designator_tests {
    use super::*;
    use semio_framework_ui_locale::Locale;

    #[test]
    fn german_labels_translate_whole_words_only() {
        assert_eq!(designator("Annex B", &Locale::De), "Anhang B");
        assert_eq!(designator("EN 772-1 Table A.1", &Locale::De), "EN 772-1 Tabelle A.1");
        assert_eq!(designator("Annexes", &Locale::De), "Annexes");
        assert_eq!(designator("Annex B", &Locale::En), "Annex B");
        assert_eq!(designator("Design case", &Locale::De), "Bemessungsfall");
        assert_eq!(annex("E"), "Annex E");
    }

    #[test]
    fn clause_text_reads_in_both_languages() {
        let clause = ClauseId::new("EN 1991-1-2", "Annex E", "E.1");
        assert_eq!(clause.text(&Locale::En), "EN 1991-1-2 Annex E §E.1");
        assert_eq!(clause.text(&Locale::De), "EN 1991-1-2 Anhang E Abschn. E.1");
    }
}
// #endregion 🔖️Clause

// #region 🔖️LocalizedText
/// 🌐️ Locale-tagged text — the ONE canonical shape shared by every norm artifact (kills the
/// `LocalizedText` duplication formerly hand-maintained separately in `vdi3805` and `iso16757`,
/// see UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM's `📓️design-full-plan.md` §4 norm line). Structurally
/// isomorphic to stdio's `✳️text` subset's `SemioTextRun{language,content,..}` (that subset's own
/// doc comment: "Absorbs the duplicated `LocalizedText` types that currently exist twice inside
/// the norm plugin") — kept as a local plain value type here rather than a composed
/// `store::os_store::artifact_child::ArtifactChild<SemioTextSnapshot>` because it is nested by the DOZENS as an ordinary
/// leaf field inside `Names`/`Subject`/`CatalogueProduct` etc., not a top-level content slot; a
/// composed child handle is for a single large content slot with its own working-scene cache,
/// not a value type reused pervasively as a struct field.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub struct LocalizedText {
    pub locale: String,
    pub text: String,
}

impl LocalizedText {
    pub fn new(locale: impl Into<String>, text: impl Into<String>) -> Self {
        Self { locale: locale.into(), text: text.into() }
    }
}
// #endregion 🔖️LocalizedText
