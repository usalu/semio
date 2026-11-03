//! 🪛️ Self-tapping screws and blind rivets of cold-formed sheets in shear and tension (EN 1993-1-3 8.3, mm, N, MPa); every coefficient is a registry row `ndp.en1993.1-3.fastener.*`. See [EN 1993-1-3](https://eurocodes.jrc.ec.europa.eu/).

use semio_s_plugin_norm_compliance::ndp_common;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Screw,
    Rivet,
}

impl Kind {
    fn slug(self) -> &'static str {
        match self {
            Kind::Screw => "screw",
            Kind::Rivet => "rivet",
        }
    }
}

fn row(kind: Kind, name: &str) -> f64 {
    ndp_common(&format!("ndp.en1993.1-3.fastener.{}.{name}", kind.slug()))
}

/// 🧮️ Bearing resistance on the thinner sheet `t1` (strength `fu1`, fastener diameter `d`) against the support `t2`: `c sqrt(t1^3 d) fu1/gM2` for equal thicknesses, `c t1 d fu1/gM2` from the registry thickness ratio, linear in between.
pub fn bearing(kind: Kind, t1: f64, t2: f64, d: f64, fu1: f64, gm2: f64) -> f64 {
    let equal = row(kind, "bearing-equal") * (t1.powi(3) * d).sqrt() * fu1 / gm2;
    let thick = row(kind, "bearing-thick") * t1 * d * fu1 / gm2;
    let (ratio, top) = (t2 / t1, ndp_common("ndp.en1993.1-3.fastener.thickness-ratio"));
    if ratio <= 1.0 {
        equal
    } else if ratio >= top {
        thick
    } else {
        equal + (thick - equal) * (ratio - 1.0) / (top - 1.0)
    }
}

/// 🧮️ Net section resistance `(b - n d) t fu/gM2` of the sheet across `n` fasteners.
pub fn net_section(width: f64, n: f64, d: f64, t: f64, fu: f64, gm2: f64) -> f64 {
    ((width - n * d) * t * fu / gm2).max(0.0)
}

/// 🧮️ Pull-over resistance of a screw head or washer `d_w` in the sheet `t1`.
pub fn pull_over(dw: f64, t1: f64, fu1: f64, gm2: f64) -> f64 {
    row(Kind::Screw, "pull-over") * dw * t1 * fu1 / gm2
}

/// 🧮️ Pull-out resistance of a screw from the support `t2`.
pub fn pull_out(d: f64, t2: f64, fu2: f64, gm2: f64) -> f64 {
    row(Kind::Screw, "pull-out") * d * t2 * fu2 / gm2
}

/// 📏️ Minimum end distance, edge distance, pitch and gauge in units of the fastener diameter.
pub fn spacing(kind: Kind) -> [f64; 4] {
    [row(kind, "e1-min"), row(kind, "e2-min"), row(kind, "p1-min"), row(kind, "p2-min")]
}
