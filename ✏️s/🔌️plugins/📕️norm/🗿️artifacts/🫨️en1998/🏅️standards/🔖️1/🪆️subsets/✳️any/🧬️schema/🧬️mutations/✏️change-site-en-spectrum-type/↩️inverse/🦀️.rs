//! ↩️ Inverse for `change-site-en-spectrum-type`.
use crate::change_site_en_spectrum_type::ChangeSiteEnSpectrumType;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiteEnSpectrumType, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_site_en_spectrum_type::ChangeSiteEnSpectrumType> {
    Ok(vec![Op::from(ChangeSiteEnSpectrumType { new_en_spectrum_type: base.site.en_spectrum_type })])
}
