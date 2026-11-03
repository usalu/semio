//! 🧰️ Helpers of the EN 1998 inference: the quantity and localisation constructors, the id selector and the annex resolution. See [`super`].

use super::*;

pub fn q_force(n: f64) -> Quantity { Quantity::new(QuantityKind::Force, n) }

pub fn q_len(m: f64) -> Quantity { Quantity::length_m(m) }

pub fn q_acc(v: f64) -> Quantity { Quantity::acceleration_m_s2(v) }

pub fn q_dim(v: f64) -> Quantity { Quantity::new(QuantityKind::Dimensionless, v) }

pub fn q_press(pa: f64) -> Quantity { Quantity::new(QuantityKind::Pressure, pa) }

pub fn q_mom(nm: f64) -> Quantity { Quantity::new(QuantityKind::Moment, nm) }

/// 🛡️ A resistance of zero or less is floored to a tiny positive limit so a positive demand fails instead of passing at utilisation 0.
pub fn floor_limit(limit: Quantity) -> Quantity { Quantity::new(limit.kind, limit.value.max(1e-9)) }

pub fn lc(en: &str, de: &str) -> LocalizedCopy { LocalizedCopy::new(en, de) }

pub fn id_sel(collection: &str, id: &str) -> String {
    format!("{collection}[id={id}]")
}

pub fn resolve_annex(document: &En1998Snapshot) -> Result<AnnexParams, CheckResult> {
    match document.annex {
        AnnexChoice::De => {
            let combo = na_de::GroundCombo::from(document.site.de_ground_combo);
            Ok(AnnexParams::De { s_ap_r: document.site.s_ap_r, combo })
        }
        AnnexChoice::En => {
            let g = part_1::spectrum_ground_letter_of(document);
            let type1 = document.site.en_spectrum_type == semio_s_artifact_norm_en1998_model::EnSpectrumType::Type1;
            Ok(AnnexParams::En { a_gr: document.site.a_gr, ground: g, spectrum_type1: type1 })
        }
    }
}
