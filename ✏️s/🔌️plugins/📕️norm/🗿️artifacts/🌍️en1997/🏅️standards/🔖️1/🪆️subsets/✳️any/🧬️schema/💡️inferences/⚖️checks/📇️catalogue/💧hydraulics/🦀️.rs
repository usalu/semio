//! 💧 Hydraulic failure beyond heave, catalogue entry HYD-02 (EN 1997-1 §10.3-10.4, DIN 1054 GZ 1A hydraulic heave, Lane and Bligh creep rules).

use super::*;

/// 📐️ Exit gradient at the downstream foot of a single sheet pile of embedment `d` in an infinitely deep homogeneous layer: i = ΔH/(π·√(x² + d²)) from the conformal map ζ = √(z² + d²) (Harr, Groundwater and Seepage); `x` is the distance from the wall.
pub fn exit_gradient_sheet_pile(delta_h: f64, d: f64, x: f64) -> f64 {
    delta_h / (std::f64::consts::PI * (x * x + d * d).sqrt())
}

fn hyd_02(c: &Case) -> R {
    let (dh, gp) = (c.positive("delta_h")?, c.positive("gamma_prime")?);
    let (lv, lh) = (c.series("lv"), c.series("lh"));
    let (l_vertical, l_horizontal) = (lv.iter().sum::<f64>(), lh.iter().sum::<f64>());
    let path = l_vertical + l_horizontal;
    let (i_exit, source) = match (c.opt("i_exit"), c.opt("d_embed")) {
        (Some(i), _) => (i, "flow net"),
        (None, Some(_)) => (exit_gradient_sheet_pile(dh, c.positive("d_embed")?, 0.0), "sheet pile ΔH/(π·d)"),
        _ => {
            gate(path > 0.0, "creep segments lv1…, lh1… or i_exit or d_embed required")?;
            (dh / path, "mean gradient along the creep path")
        }
    };
    gate(i_exit > 0.0 && i_exit.is_finite(), "the exit gradient must be positive")?;
    let (u, gw) = (c.upl(), c.gamma_w());
    let l_bligh = path;
    let l_lane = l_vertical + l_horizontal / 3.0;
    let mut out = vec![
        Outcome::force("piping", "Seepage force γH·i·γw vs γH,stb·γ′ at the exit", "Strömungskraft γH·i·γw gegen γH,stb·γ′ am Austritt", u.gamma_hyd * i_exit * gw, u.gamma_hyd_stb * gp),
        Outcome { steps: format!("i_exit = {} ({source})", f(i_exit)), ..Outcome::number("i_exit", "Exit gradient i", "Austrittsgefälle i", i_exit) },
        Outcome::number("i_crit", "Critical gradient γ′/γw", "Kritisches Gefälle γ′/γw", gp / gw),
        Outcome::number("safety", "Safety i_crit/i", "Sicherheit i_crit/i", gp / gw / i_exit),
    ];
    if path > 0.0 {
        out.push(Outcome::metre("l_bligh", "Bligh creep length ΣL_v + ΣL_h", "Bligh-Sickerweg ΣL_v + ΣL_h", l_bligh));
        out.push(Outcome::metre("l_lane", "Lane weighted creep length ΣL_v + ΣL_h/3", "Lane-Sickerweg ΣL_v + ΣL_h/3", l_lane));
        out.push(Outcome::number("c_bligh_provided", "Provided Bligh ratio L/ΔH", "Vorhandenes Bligh-Verhältnis L/ΔH", l_bligh / dh));
        out.push(Outcome::number("c_lane_provided", "Provided Lane ratio L_w/ΔH", "Vorhandenes Lane-Verhältnis L_w/ΔH", l_lane / dh));
        if let Some(required) = c.opt("c_lane") {
            out.insert(0, Outcome::ratio("lane", "Required Lane ratio vs provided L_w/ΔH", "Erforderliches Lane-Verhältnis gegen vorhandenes L_w/ΔH", required, l_lane / dh));
        }
        if let Some(required) = c.opt("c_bligh") {
            out.insert(0, Outcome::ratio("bligh", "Required Bligh ratio vs provided L/ΔH", "Erforderliches Bligh-Verhältnis gegen vorhandenes L/ΔH", required, l_bligh / dh));
        }
    }
    if let Some(k) = c.opt("k") {
        out.push(Outcome::number("v_exit", "Darcy exit velocity v = k·i (m/s)", "Darcy-Austrittsgeschwindigkeit v = k·i (m/s)", k * i_exit));
    }
    Ok(out)
}

pub const ENTRIES: &[Entry] = &[Entry {
    id: "HYD-02",
    part: "EN 1997-1",
    clause: "10.3",
    en: "Piping and internal erosion: exit gradient and creep length",
    de: "Hydraulischer Grundbruch und innere Erosion: Austrittsgefälle und Sickerweg",
    inputs: &["delta_h", "gamma_prime", "lv1", "lh1", "i_exit", "d_embed", "c_lane", "c_bligh", "k", "gamma_w", "situation"],
    eval: hyd_02,
}];
