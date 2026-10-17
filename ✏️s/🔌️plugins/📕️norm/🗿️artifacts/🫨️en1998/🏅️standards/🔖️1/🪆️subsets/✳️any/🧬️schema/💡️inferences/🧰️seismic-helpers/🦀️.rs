//! 🧰️ Helpers of the EN 1998 inference: the quantity and localisation constructors, the id selector and the annex resolution. See [`super`].

use super::*;

pub(super) fn q_force(n: f64) -> Quantity { Quantity::new(QuantityKind::Force, n) }

pub(super) fn q_len(m: f64) -> Quantity { Quantity::length_m(m) }

pub(super) fn q_acc(v: f64) -> Quantity { Quantity::acceleration_m_s2(v) }

pub(super) fn q_dim(v: f64) -> Quantity { Quantity::new(QuantityKind::Dimensionless, v) }

pub(super) fn q_press(pa: f64) -> Quantity { Quantity::new(QuantityKind::Pressure, pa) }

pub(super) fn q_mom(nm: f64) -> Quantity { Quantity::new(QuantityKind::Moment, nm) }

pub(super) fn lc(en: &str, de: &str) -> LocalizedCopy { LocalizedCopy::new(en, de) }

pub(super) fn id_sel(collection: &str, id: &str) -> String {
    format!("{collection}[id={id}]")
}

pub(super) fn resolve_annex(document: &En1998Snapshot) -> Result<AnnexParams, CheckResult> {
    let annex = match document.annex.to_ascii_lowercase().as_str() {
        "en" => AnnexChoice::En,
        _ => AnnexChoice::De,
    };
    match annex {
        AnnexChoice::De => {
            let zone = na_de::SeismicZone::from(document.site.seismic_zone);
            let combo = na_de::GroundCombo::from(document.site.de_ground_combo);
            Ok(AnnexParams::De { zone, combo })
        }
        AnnexChoice::En => {
            let g = document.site.en_ground_type.chars().next().unwrap_or('B').to_ascii_uppercase();
            let type1 = !document.site.en_spectrum_type.to_ascii_lowercase().contains("2");
            Ok(AnnexParams::En { a_gr: document.site.a_gr, ground: g, spectrum_type1: type1 })
        }
    }
}
