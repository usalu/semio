//! 📐️ Annex E of EN 1996-1-1: the bending moment coefficient table of laterally loaded panels, one block of cells per edge condition. The cells are NOT keyed: the print is not at hand, so [`keyed`] is empty and every lookup answers [`Alpha::Unkeyed`] until a verified cell is entered in [`KEYED`] and in `./🔣️.json`. The yield-line value [`standin_alpha1`] is the documented unverified stand-in. See [`super`].

use super::numerical_models::{yield_line_pressure, Edge, Edges};

/// 🧱️ One tabulated coefficient: the orthogonal strength ratio μ, the aspect ratio h/l and the coefficient α₁ read from the print.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnnexECell {
    pub mu: f64,
    pub h_over_l: f64,
    pub alpha1: f64,
}

/// 🧱️ All cells of one edge-condition class.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnnexEClass {
    pub code: &'static str,
    pub cells: &'static [AnnexECell],
}

/// 🔖️ The verified cells; empty until the print is keyed (never fill with recalled values).
pub const KEYED: &[AnnexEClass] = &[];

/// 🔎️ Result of a coefficient lookup.
#[derive(Clone, Debug, PartialEq)]
pub enum Alpha {
    Keyed(f64),
    Unkeyed { class: String },
}

fn letter(edge: Edge) -> char {
    match edge {
        Edge::Free => 'F',
        Edge::Simple => 'S',
        Edge::Fixed => 'X',
    }
}

/// 🔤️ Canonical edge-condition class of a panel: the two horizontal edges and the two vertical edges each ordered free, simple, fixed (the table is symmetric under mirroring); letters `F` free, `S` simple, `X` fixed.
pub fn class_code(edges: Edges) -> String {
    let order = |a: Edge, b: Edge| {
        let (x, y) = (letter(a), letter(b));
        let rank = |c: char| "FSX".find(c).unwrap_or(0);
        if rank(x) <= rank(y) { [x, y] } else { [y, x] }
    };
    let [t, b] = order(edges.top, edges.bottom);
    let [l, r] = order(edges.left, edges.right);
    [t, b, l, r].iter().collect()
}

/// 🔤️ The 36 edge-condition classes in lexicographic order of the canonical codes.
pub fn classes() -> Vec<String> {
    let pairs = ["FF", "FS", "FX", "SS", "SX", "XX"];
    pairs.iter().flat_map(|h| pairs.iter().map(move |v| format!("{h}{v}"))).collect()
}

/// 🔗️ Number of supported edges (simple or fixed) of a class.
pub fn supported_edges(code: &str) -> usize {
    code.chars().filter(|c| *c != 'F').count()
}

/// 📈️ Linear interpolation of sorted `(x, y)` samples; `None` outside the sampled range (no extrapolation).
fn interpolate(samples: &[(f64, f64)], x: f64) -> Option<f64> {
    let (first, last) = (samples.first()?, samples.last()?);
    if x < first.0 || x > last.0 {
        return None;
    }
    samples.windows(2).find(|w| x <= w[1].0).map(|w| if w[1].0 > w[0].0 { w[0].1 + (w[1].1 - w[0].1) * (x - w[0].0) / (w[1].0 - w[0].0) } else { w[0].1 }).or(Some(first.1))
}

/// 🔎️ α₁ of a class from `cells` by bilinear interpolation in μ and h/l; `None` when the point lies outside the keyed grid or a needed cell is missing.
pub fn interpolate_cells(cells: &[AnnexECell], mu: f64, h_over_l: f64) -> Option<f64> {
    let mut mus: Vec<f64> = cells.iter().map(|c| c.mu).collect();
    mus.sort_by(f64::total_cmp);
    mus.dedup();
    let at_mu = |m: f64| {
        let mut row: Vec<(f64, f64)> = cells.iter().filter(|c| c.mu == m).map(|c| (c.h_over_l, c.alpha1)).collect();
        row.sort_by(|a, b| a.0.total_cmp(&b.0));
        interpolate(&row, h_over_l)
    };
    let column: Vec<(f64, f64)> = mus.iter().filter_map(|m| at_mu(*m).map(|a| (*m, a))).collect();
    if column.len() != mus.len() {
        return None;
    }
    interpolate(&column, mu)
}

const BASIS: [&str; 36] = ["ndp.en1996.annex-e.ffff", "ndp.en1996.annex-e.fffs", "ndp.en1996.annex-e.fffx", "ndp.en1996.annex-e.ffss", "ndp.en1996.annex-e.ffsx", "ndp.en1996.annex-e.ffxx", "ndp.en1996.annex-e.fsff", "ndp.en1996.annex-e.fsfs", "ndp.en1996.annex-e.fsfx", "ndp.en1996.annex-e.fsss", "ndp.en1996.annex-e.fssx", "ndp.en1996.annex-e.fsxx", "ndp.en1996.annex-e.fxff", "ndp.en1996.annex-e.fxfs", "ndp.en1996.annex-e.fxfx", "ndp.en1996.annex-e.fxss", "ndp.en1996.annex-e.fxsx", "ndp.en1996.annex-e.fxxx", "ndp.en1996.annex-e.ssff", "ndp.en1996.annex-e.ssfs", "ndp.en1996.annex-e.ssfx", "ndp.en1996.annex-e.ssss", "ndp.en1996.annex-e.sssx", "ndp.en1996.annex-e.ssxx", "ndp.en1996.annex-e.sxff", "ndp.en1996.annex-e.sxfs", "ndp.en1996.annex-e.sxfx", "ndp.en1996.annex-e.sxss", "ndp.en1996.annex-e.sxsx", "ndp.en1996.annex-e.sxxx", "ndp.en1996.annex-e.xxff", "ndp.en1996.annex-e.xxfs", "ndp.en1996.annex-e.xxfx", "ndp.en1996.annex-e.xxss", "ndp.en1996.annex-e.xxsx", "ndp.en1996.annex-e.xxxx"];

/// 🔖️ The registry row of the cells of a class (`ndp.en1996.annex-e.<class>`, unverified while the print is not keyed), or `None` for a code that is no class; a check that reads a cell of the class lists it as its basis.
pub fn basis_id(code: &str) -> Option<&'static str> {
    classes().iter().position(|c| c == code).map(|i| BASIS[i])
}

/// 🔎️ The keyed coefficient α₁ for the edge condition, μ and h/l, or [`Alpha::Unkeyed`].
pub fn alpha1(edges: Edges, mu: f64, h_over_l: f64) -> Alpha {
    let code = class_code(edges);
    match KEYED.iter().find(|c| c.code == code).and_then(|c| interpolate_cells(c.cells, mu, h_over_l)) {
        Some(value) => Alpha::Keyed(value),
        None => Alpha::Unkeyed { class: code },
    }
}

/// 🧮️ The unverified stand-in α₁ = m₁/(w_Rd l²) of the Johansen yield-line value for a panel of aspect ratio h/l and orthogonal ratio μ = m₁/m₂ (hogging equal to sagging on fixed edges).
pub fn standin_alpha1(edges: Edges, mu: f64, h_over_l: f64) -> f64 {
    let m1 = 1.0;
    let m2 = m1 / mu.max(1e-9);
    m1 / yield_line_pressure(edges, 1.0, h_over_l, m1, m2).max(1e-12)
}

/// 🧮️ Design moment per metre `M_Ed1 = α₁ w_Ed l²` of a panel of length `l`.
pub fn design_moment(alpha1: f64, w_ed: f64, l: f64) -> f64 {
    alpha1 * w_ed * l * l
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cells() -> Vec<AnnexECell> {
        vec![AnnexECell { mu: 0.5, h_over_l: 0.5, alpha1: 1.0 }, AnnexECell { mu: 0.5, h_over_l: 1.0, alpha1: 3.0 }, AnnexECell { mu: 1.0, h_over_l: 0.5, alpha1: 2.0 }, AnnexECell { mu: 1.0, h_over_l: 1.0, alpha1: 6.0 }]
    }

    #[test]
    fn there_are_thirty_six_classes_and_the_code_is_canonical_under_mirroring() {
        assert_eq!(classes().len(), 36);
        let a = Edges::parse("SXFS").unwrap();
        let b = Edges::parse("XSSF").unwrap();
        assert_eq!(class_code(a), class_code(b));
        assert!(classes().contains(&class_code(a)));
        assert_eq!(class_code(Edges::parse("SSSS").unwrap()), "SSSS");
        assert_eq!(supported_edges("FSXX"), 3);
    }

    #[test]
    fn synthetic_cells_interpolate_bilinearly_and_never_extrapolate() {
        let c = cells();
        assert!((interpolate_cells(&c, 0.5, 0.75).unwrap() - 2.0).abs() < 1e-12);
        assert!((interpolate_cells(&c, 0.75, 0.75).unwrap() - 3.0).abs() < 1e-12);
        assert!((interpolate_cells(&c, 1.0, 1.0).unwrap() - 6.0).abs() < 1e-12);
        assert_eq!(interpolate_cells(&c, 0.4, 0.75), None);
        assert_eq!(interpolate_cells(&c, 0.75, 1.2), None);
        assert_eq!(interpolate_cells(&c[..3], 0.75, 0.75), None);
    }

    #[test]
    fn no_cell_is_keyed_so_every_lookup_is_unkeyed() {
        assert!(KEYED.is_empty());
        assert_eq!(alpha1(Edges::parse("SSSS").unwrap(), 1.0, 1.0), Alpha::Unkeyed { class: "SSSS".into() });
    }

    #[test]
    fn every_class_has_its_unverified_registry_row_as_basis() {
        for code in classes() {
            let id = basis_id(&code).unwrap();
            assert_eq!(id, format!("ndp.en1996.annex-e.{}", code.to_lowercase()));
            assert!(semio_s_plugin_norm_compliance::ndp(id).is_some(), "{id}");
        }
        assert_eq!(basis_id("ZZZZ"), None);
    }

    #[test]
    fn the_stand_in_of_a_square_simply_supported_panel_is_one_twenty_fourth() {
        let a = standin_alpha1(Edges::parse("SSSS").unwrap(), 1.0, 1.0);
        assert!((a - 1.0 / 24.0).abs() < 1e-3, "{a}");
        assert!((design_moment(a, 1000.0, 4.0) - a * 16_000.0).abs() < 1e-9);
    }

    #[test]
    fn the_json_twin_lists_every_class_with_empty_cells() {
        let json = include_str!("🔣️.json");
        for code in classes() {
            assert!(json.contains(&format!("\"code\": \"{code}\"")), "{code}");
        }
        assert!(json.contains("\"status\": \"unverified\""));
        assert!(!json.contains("\"alpha1\":"), "no cell may be keyed in the twin while KEYED is empty");
    }
}
