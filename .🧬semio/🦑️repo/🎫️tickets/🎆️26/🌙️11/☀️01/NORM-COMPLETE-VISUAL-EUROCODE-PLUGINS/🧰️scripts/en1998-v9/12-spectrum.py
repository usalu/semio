"""en1998 v9 step 12: the spectrum ground type follows the foundation soil layers (precedence documented in part_1). python -I"""
import sys; sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib import *
R = S / "🦀️.rs"
sub(R, "En1998Foundation, En1998RetainingWall, En1998Site,", "En1998Foundation, En1998Member, En1998RetainingWall, En1998Site, En1998Snapshot, MemberKind,")
sub(R, '''    /// 🌍️ Ground letter of the spectrum parameters of the site: the Table 3.1 type derived from the site's own classification inputs when it has a spectrum letter, else the declared EN ground type, else B.
    pub fn spectrum_ground_letter(site: &super::En1998Site) -> char {
        ground_type_table_3_1(&ground_inputs_of_site(site, site.v_s30_m_s))
            .and_then(GroundType::spectrum_letter)
            .or_else(|| site.en_ground_type.chars().next().map(|c| c.to_ascii_uppercase()))
            .unwrap_or('B')
    }
''', '''    /// 🌍️ Ground letter of the spectrum parameters for the v_s,30 chosen by the caller: the Table 3.1 type derived from that value and the site's other classification inputs when it has a spectrum letter, else the declared EN ground type, else B.
    pub fn spectrum_ground_letter_for(site: &super::En1998Site, v_s30_m_s: f64) -> char {
        ground_type_table_3_1(&ground_inputs_of_site(site, v_s30_m_s))
            .and_then(GroundType::spectrum_letter)
            .or_else(|| site.en_ground_type.chars().next().map(|c| c.to_ascii_uppercase()))
            .unwrap_or('B')
    }

    /// 🌍️ Ground letter of the spectrum parameters from the site's own inputs only (see [`spectrum_ground_letter_for`]); the document-level choice with foundation layers is [`spectrum_ground_letter_of`].
    pub fn spectrum_ground_letter(site: &super::En1998Site) -> char {
        spectrum_ground_letter_for(site, site.v_s30_m_s)
    }

    /// 🪨 Where the v_s,30 behind the spectrum comes from.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum GroundSource {
        FoundationLayers { foundation_id: String },
        Site,
    }

    /// 🪨 v_s,30 of the soil layers of one foundation (Eq. 3.1 over the top 30 m, layers ordered top to bottom); 0 without usable layers.
    pub fn foundation_v_s30_m_s(document: &super::En1998Snapshot, foundation_id: &str) -> f64 {
        let layers: Vec<(f64, f64)> = document.soil_layers.iter().filter(|l| l.foundation_id == foundation_id).map(|l| (l.thickness_m, l.shear_wave_velocity_m_s)).collect();
        time_averaged_over_depth(&layers, 30.0)
    }

    /// 🪨 The v_s,30 the spectrum follows, by this precedence: (1) the soil layers of the foundations, when at least one foundation has layers with a positive v_s,30, the lowest value governs because the spectrum is one document-level quantity and softer ground is the conservative choice; (2) otherwise the site's own `v_s30_m_s`. N_SPT, c_u, the soft clay, alluvium and liquefiable inputs always come from the site.
    pub fn effective_v_s30(document: &super::En1998Snapshot) -> (f64, GroundSource) {
        document.foundations.iter()
            .map(|f| (foundation_v_s30_m_s(document, &f.id), f))
            .filter(|(v, _)| *v > 0.0)
            .min_by(|a, b| a.0.total_cmp(&b.0))
            .map_or((document.site.v_s30_m_s, GroundSource::Site), |(v, f)| (v, GroundSource::FoundationLayers { foundation_id: f.id.clone() }))
    }

    /// 🌍️ Ground letter of the spectra of the document: Table 3.1 on the [`effective_v_s30`] (foundation layers before the site's own value).
    pub fn spectrum_ground_letter_of(document: &super::En1998Snapshot) -> char {
        spectrum_ground_letter_for(&document.site, effective_v_s30(document).0)
    }
''')
sub(S / "💡️inferences/🧰️seismic-helpers/🦀️.rs", "part_1::spectrum_ground_letter(&document.site)", "part_1::spectrum_ground_letter_of(document)")
sub(ANY / "🎬️scene/🧮️response/🦀️.rs", "part_1::spectrum_ground_letter(&document.site)", "part_1::spectrum_ground_letter_of(document)")
B = S / "💡️inferences/🌍️full-seismic-check/🧩️building-details/🦀️.rs"
sub(B, "    let v = document.site.v_s30_m_s;\n    let Some(derived)", "    let (v, source) = part_1::effective_v_s30(document);\n    let Some(derived)")
sub(B, '''        &format!("v_s,30 = {v:.0} m/s, N_SPT = {:.0}, c_u = {:.0} kPa give ground type {name}; stated class {stated}{}", document.site.n_spt, document.site.c_u_kpa, if class.is_none()''', '''        &format!("v_s,30 = {v:.0} m/s ({}), N_SPT = {:.0}, c_u = {:.0} kPa give ground type {name}; stated class {stated}{}", match &source { part_1::GroundSource::FoundationLayers { foundation_id } => format!("soil layers of {foundation_id}"), part_1::GroundSource::Site => "site input".to_owned() }, document.site.n_spt, document.site.c_u_kpa, if class.is_none()''')
sub(B, '''        &format!("v_s,30 = {v:.0} m/s, N_SPT = {:.0}, c_u = {:.0} kPa ergeben Baugrundklasse {name}; angegebene Klasse {stated}{}", document.site.n_spt, document.site.c_u_kpa, if class.is_none()''', '''        &format!("v_s,30 = {v:.0} m/s ({}), N_SPT = {:.0}, c_u = {:.0} kPa ergeben Baugrundklasse {name}; angegebene Klasse {stated}{}", match &source { part_1::GroundSource::FoundationLayers { foundation_id } => format!("Bodenschichten von {foundation_id}"), part_1::GroundSource::Site => "Standorteingabe".to_owned() }, document.site.n_spt, document.site.c_u_kpa, if class.is_none()''')
G = ANY / "🎬️scene/🌉️ground-and-bridges/🦀️.rs"
sub(G, "    let used = part_1::spectrum_ground_letter(site);", "    let used = part_1::spectrum_ground_letter_of(input.document);\n    let followed = part_1::effective_v_s30(input.document);")
sub(G, '''format!("v_s,30 = {v_s30:.0} m/s \\u{2192} {} {} | S =''', '''format!("v_s,30 = {v_s30:.0} m/s \\u{2192} {} {} | {} | S =''')
flush()
