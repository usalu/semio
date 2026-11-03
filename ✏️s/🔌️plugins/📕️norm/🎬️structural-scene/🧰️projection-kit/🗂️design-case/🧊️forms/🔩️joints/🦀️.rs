//! 🔩️ Joint and detail forms of the case drawing: fastener group, fillet weld, concrete anchor, slab with opening and the criteria symbol, each with the grips that write their dimension and auxiliary inputs. See [`super`].

use super::*;
use norm::design_case::CaseForm;

/// 🔩️ The joint form of the declaration of `m`, `None` for the other forms.
pub(super) fn build<C: CaseCatalogue>(m: &Meas<'_, C>) -> Option<Built> {
    Some(match m.g.form {
        CaseForm::Fastener => fastener(m),
        CaseForm::Weld => weld(m),
        CaseForm::Anchor => anchor(m),
        CaseForm::Opening => opening(m),
        CaseForm::Symbol => symbol(m),
        _ => return None,
    })
}

const X: [f64; 3] = [1.0, 0.0, 0.0];
const Y: [f64; 3] = [0.0, 1.0, 0.0];
const Z: [f64; 3] = [0.0, 0.0, 1.0];
const HALF_DIAGONAL: f64 = std::f64::consts::FRAC_1_SQRT_2;
const MIN_RIM: f64 = 0.02;
const COUNT_MAX: f64 = 12.0;

/// 🔩️ Two lapped plates pierced by `rows` rows of `count` fasteners of `diameter` at `spacing` along the row and `spacing2` across; plate dimensions without a declared input follow the pattern.
fn fastener<C: CaseCatalogue>(m: &Meas<'_, C>) -> Built {
    let (declared_w, declared_l, t1) = (m.w(), m.l(), m.h());
    let t2 = m.aux("thickness2", t1);
    let d = m.aux("diameter", (t1 + t2).clamp(0.01, 0.2) * 0.5);
    let count = m.count("count", if m.has("spacing") { 2.0 } else { 1.0 });
    let spacing = m.aux("spacing", 3.0 * d);
    let rows = m.count("rows", if m.has("spacing2") { 2.0 } else { 1.0 });
    let spacing2 = m.aux("spacing2", 3.0 * d);
    let length = if m.g.length.is_empty() { (count - 1.0) * spacing + 6.0 * d } else { declared_l };
    let width = if m.g.width.is_empty() { (rows - 1.0) * spacing2 + 4.0 * d } else { declared_w };
    let mut parts = vec![
        Part::new("", solid(m.pose, [0.0, 0.0, 0.0], [length, width, t1]), m.role.clone(), ("Plate", "Blech")),
        Part::new("plate-b", solid(m.pose, [0.0, 0.0, t1], [length, width, t2]), m.role.clone(), ("Lapped plate", "Laschenblech")),
    ];
    for index in 0..(count * rows) as usize {
        let (row, column) = (index / count as usize, index % count as usize);
        let (x, y) = ((column as f64 - (count - 1.0) / 2.0) * spacing, (row as f64 - (rows - 1.0) / 2.0) * spacing2);
        parts.push(Part::new(format!("fastener-{index}"), pillar(m.pose, [x, y, -0.01], d / 2.0, t1 + t2 + 0.02), MaterialRole::Bolt, ("Fastener", "Verbindungsmittel")));
        parts.push(Part::new(format!("head-{index}"), pillar(m.pose, [x, y, t1 + t2], 0.9 * d, 0.6 * d), MaterialRole::Bolt, ("Fastener head", "Verbindungsmittelkopf")));
    }
    let plans = vec![
        face("length", m.g.length, 2.0, X, [length / 2.0, 0.0, t1]),
        face("width", m.g.width, 2.0, Y, [0.0, width / 2.0, t1]),
        scalar("height", m.g.height, 1.0, Z, [length / 2.0, 0.0, t1]),
        scalar("thickness-b", m.key("thickness2"), 1.0, Z, [-length / 2.0, 0.0, t1 + t2]),
        face("diameter", m.key("diameter"), 2.0, Y, [(count - 1.0) * spacing / 2.0, d / 2.0, t1 + t2 + 0.6 * d]),
        scalar("spacing", m.key("spacing"), 2.0 / (count - 1.0).max(1.0), X, [(count - 1.0) * spacing / 2.0, 0.0, t1 + t2 + 0.6 * d]),
        scalar("spacing-across", m.key("spacing2"), 2.0 / (rows - 1.0).max(1.0), Y, [0.0, (rows - 1.0) * spacing2 / 2.0, t1 + t2 + 0.6 * d]),
        stepper("count", m.key("count"), (1.0, COUNT_MAX), [(count - 1.0) * spacing / 2.0 + spacing, 0.0, t1 + t2 + 0.6 * d]),
        stepper("rows", m.key("rows"), (1.0, COUNT_MAX), [0.0, (rows - 1.0) * spacing2 / 2.0 + spacing2, t1 + t2 + 0.6 * d]),
    ];
    Built { parts, top: t1 + t2 + 0.6 * d, bottom: -0.01, half: (length / 2.0, width / 2.0), plans }
}

/// 🔥️ A fillet weld: a base plate with an upright plate on it and a bead of the declared throat along both sides over the weld length.
fn weld<C: CaseCatalogue>(m: &Meas<'_, C>) -> Built {
    let (plate_w, length, t) = (m.w(), m.l(), m.h());
    let throat = m.aux("throat", 0.5 * t);
    let width = if m.g.width.is_empty() { 2.0 * throat + 4.0 * t } else { plate_w };
    let web = t.max(0.01);
    let upright = (4.0 * t).max(0.15);
    let diamond = throat * std::f64::consts::SQRT_2;
    let frame = |side: f64| {
        let corner = [0.0, side * web / 2.0, t];
        let z = [0.0, -HALF_DIAGONAL, HALF_DIAGONAL];
        m.pose.axes([corner[0] - z[0] * diamond / 2.0, corner[1] - z[1] * diamond / 2.0, corner[2] - z[2] * diamond / 2.0], X, [0.0, HALF_DIAGONAL, HALF_DIAGONAL], z)
    };
    let bead = |tag: &str, side: f64| Part::new(tag, SolidShape::Box { frame: frame(side), size: v3(length, diamond, diamond) }, MaterialRole::Weld, ("Fillet weld", "Kehlnaht"));
    let parts = vec![
        Part::new("", solid(m.pose, [0.0, 0.0, 0.0], [length, width, t]), m.role.clone(), ("Base plate", "Grundblech")),
        Part::new("upright", solid(m.pose, [0.0, 0.0, t], [length, web, upright]), m.role.clone(), ("Upright plate", "Steife")),
        bead("bead-a", 1.0),
        bead("bead-b", -1.0),
    ];
    let plans = vec![
        face("length", m.g.length, 2.0, X, [length / 2.0, 0.0, t]),
        face("width", m.g.width, 2.0, Y, [0.0, width / 2.0, t / 2.0]),
        scalar("height", m.g.height, 1.0, Z, [length / 2.0, width / 2.0, t]),
        scalar("throat", m.key("throat"), 1.0, Z, [0.0, web / 2.0 + throat, t + throat]),
    ];
    Built { parts, top: t + upright, bottom: 0.0, half: (length / 2.0, width / 2.0), plans }
}

/// ⚓️ A concrete block with a headed anchor of the declared diameter embedded to the declared depth below the surface.
fn anchor<C: CaseCatalogue>(m: &Meas<'_, C>) -> Built {
    let (w, l) = (m.w(), m.l());
    let embed = m.aux("embed", 0.7 * m.n.2);
    let h = if m.g.height.is_empty() { m.n.2.max(1.3 * embed) } else { m.h() };
    let d = m.aux("diameter", 0.02);
    let parts = vec![
        Part::new("", solid(m.pose, [0.0, 0.0, -h], [w, l, h]), m.role.clone(), ("Concrete block", "Betonblock")),
        Part::new("anchor", pillar(m.pose, [0.0, 0.0, -embed], d / 2.0, embed + 0.06), MaterialRole::Steel, ("Anchor", "Dübel")),
        Part::new("head", pillar(m.pose, [0.0, 0.0, -embed], d, 0.5 * d), MaterialRole::Steel, ("Anchor head", "Ankerkopf")),
    ];
    let plans = vec![
        face("width", m.g.width, 2.0, X, [w / 2.0, 0.0, -h / 2.0]),
        face("length", m.g.length, 2.0, Y, [0.0, l / 2.0, -h / 2.0]),
        scalar("height", m.g.height, -1.0, Z, [w / 2.0, l / 2.0, -h]),
        scalar("embed", m.key("embed"), -1.0, Z, [0.0, 0.0, -embed]),
        face("diameter", m.key("diameter"), 2.0, X, [d / 2.0, 0.0, 0.03]),
    ];
    Built { parts, top: 0.06, bottom: -h, half: (w / 2.0, l / 2.0), plans }
}

/// 🕳️ A slab drawn as four strips around a rectangular opening centred in plan; an opening wider than the slab keeps a rim of [`MIN_RIM`] so the opening always follows its input.
fn opening<C: CaseCatalogue>(m: &Meas<'_, C>) -> Built {
    let (w, l, t) = (m.w(), m.l(), m.h());
    let hole_w = m.aux("open_w", 0.25 * w);
    let hole_l = m.aux("open_l", 0.25 * l);
    let (side_w, side_l) = (((w - hole_w) / 2.0).max(MIN_RIM), ((l - hole_l) / 2.0).max(MIN_RIM));
    let strip = |tag: &str, x: f64, y: f64, sx: f64, sy: f64| Part::new(tag, solid(m.pose, [x, y, 0.0], [sx, sy, t]), m.role.clone(), ("Slab strip", "Plattenstreifen"));
    let parts = vec![
        strip("", -(hole_w + side_w) / 2.0, 0.0, side_w, l),
        strip("strip-right", (hole_w + side_w) / 2.0, 0.0, side_w, l),
        strip("strip-front", 0.0, -(hole_l + side_l) / 2.0, hole_w, side_l),
        strip("strip-back", 0.0, (hole_l + side_l) / 2.0, hole_w, side_l),
    ];
    let plans = vec![
        face("width", m.g.width, 2.0, X, [w / 2.0, 0.0, t / 2.0]),
        face("length", m.g.length, 2.0, Y, [0.0, l / 2.0, t / 2.0]),
        scalar("height", m.g.height, 1.0, Z, [w / 2.0, l / 2.0, t]),
        face("opening-width", m.key("open_w"), 2.0, X, [hole_w / 2.0, 0.0, t]),
        face("opening-length", m.key("open_l"), 2.0, Y, [0.0, hole_l / 2.0, t]),
    ];
    Built { parts, top: t, bottom: 0.0, half: ((hole_w + 2.0 * side_w).max(w) / 2.0, (hole_l + 2.0 * side_l).max(l) / 2.0), plans }
}

/// 🔣️ A criteria sign: a diamond on a plinth for a case without a number to draw.
fn symbol<C: CaseCatalogue>(m: &Meas<'_, C>) -> Built {
    let (w, _, h) = m.n;
    let side = 0.6 * w;
    let sign = m.pose.axes([0.0, 0.0, h - side - 0.1], [HALF_DIAGONAL, HALF_DIAGONAL, 0.0], [-HALF_DIAGONAL, HALF_DIAGONAL, 0.0], Z);
    let parts = vec![
        Part::new("", SolidShape::Box { frame: sign, size: v3(side, side, side) }, m.role.clone(), ("Criteria sign", "Kriterienzeichen")),
        Part::new("plinth", pillar(m.pose, [0.0, 0.0, 0.0], 0.15, h - side - 0.1), MaterialRole::Support, ("Plinth", "Sockel")),
    ];
    Built { parts, top: h - 0.1, bottom: 0.0, half: (w / 2.0, w / 2.0), plans: Vec::new() }
}
