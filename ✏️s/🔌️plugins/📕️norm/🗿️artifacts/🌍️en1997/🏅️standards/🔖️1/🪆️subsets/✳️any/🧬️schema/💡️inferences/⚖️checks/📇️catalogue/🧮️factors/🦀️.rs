//! 🧮️ Partial-factor catalogue entry PF-01: the factor set an approach, annex and design situation resolve to (EN 1997-1 Annex A, DIN 1054:2021-04 Tables A 2.1-A 2.3 (values carried over from the withdrawn 2010-12 edition, unverified against 2021-04)).

use super::*;

fn pf_01(c: &Case) -> R {
    let approach = parse_design_approach(c.text.get("approach").ok_or("missing input approach")?);
    let sit = c.so("situation", "bsP");
    let p = resolve_params(approach, c.annex, sit);
    let u = resolve_upl_params(c.annex, sit);
    let n = |key: &str, en: &str, de: &str, v: f64| Outcome::number(key, en, de, v);
    Ok(vec![
        n("gamma_g", "γG unfavourable", "γG ungünstig", p.gamma_g),
        n("gamma_q", "γQ unfavourable", "γQ ungünstig", p.gamma_q),
        n("gamma_phi", "γφ′ on tan φ′", "γφ′ auf tan φ′", p.gamma_phi),
        n("gamma_c", "γc′", "γc′", p.gamma_c),
        n("gamma_cu", "γcu", "γcu", p.gamma_cu),
        n("gamma_gamma", "γγ", "γγ", p.gamma_gamma),
        n("gamma_r_v", "γR,v bearing", "γR,v Grundbruch", p.gamma_r_v),
        n("gamma_r_h", "γR,h sliding", "γR,h Gleiten", p.gamma_r_h),
        n("gamma_r_e", "γR,e passive earth pressure", "γR,e Erdwiderstand", p.gamma_r_e),
        n("gamma_b", "γb pile base", "γb Pfahlfuß", p.gamma_b),
        n("gamma_s", "γs pile shaft", "γs Pfahlmantel", p.gamma_s),
        n("gamma_t", "γt pile total resistance", "γt Pfahl Gesamtwiderstand", p.gamma_t),
        n("gamma_st", "γs,t pile tension", "γs,t Pfahlzug", p.gamma_st),
        n("gamma_a", "γa anchor", "γa Anker", p.gamma_a),
        n("equ_stb", "EQU γG,stb", "EQU γG,stb", u.gamma_equ_stb),
        n("equ_dst", "EQU γG,dst", "EQU γG,dst", u.gamma_equ_dst),
        n("equ_q", "EQU γQ,dst", "EQU γQ,dst", u.gamma_equ_q),
        n("upl_stb", "UPL γG,stb", "UPL γG,stb", u.gamma_g_stb),
        n("upl_dst", "UPL γG,dst", "UPL γG,dst", u.gamma_g_dst),
        n("upl_q", "UPL γQ,dst", "UPL γQ,dst", u.gamma_q_dst),
        n("hyd_h", "HYD γH / γG,dst", "HYD γH / γG,dst", u.gamma_hyd),
        n("hyd_stb", "HYD γG,stb", "HYD γG,stb", u.gamma_hyd_stb),
    ])
}

pub const ENTRIES: &[Entry] = &[Entry {
    id: "PF-01",
    part: "EN 1997-1",
    clause: "2.4.7",
    en: "Partial factor set (A, M, R, EQU, UPL, HYD)",
    de: "Teilsicherheitsbeiwerte (A, M, R, EQU, UPL, HYD)",
    inputs: &["approach", "situation"],
    eval: pf_01,
}];
