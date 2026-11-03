//! 🔩️ The persisted stud collection of one composite beam: the design resistance `P_Rd` of every stud (EN 1994-1-1, 6.6.3.1 with the sheeting reduction `k_t` / `k_l` of 6.6.4), the stations the studs form, the gaps between them (6.6.5.5) and the longitudinal shear force every stud has to carry.

use crate::catalogue as cat;
use crate::AnnexParams;
use semio_s_artifact_norm_en1994_model::{CompositeBeam, Stud};
use semio_s_plugin_norm_compliance::AnnexChoice;

/// 📏️ Studs closer than this along the beam axis share one station (the studs of one rib, `n_r` of EN 1994-1-1, 6.6.4.2).
pub const STATION_TOLERANCE_M: f64 = 1.0e-3;
const EDGE_EPSILON_M: f64 = 1.0e-6;

/// 🔩️ One stud with the design resistances EN 1994-1-1 gives it.
#[derive(Clone, Debug, PartialEq)]
pub struct StudResistance {
    pub id: String,
    pub x_m: f64,
    pub diameter_m: f64,
    pub height_m: f64,
    pub f_u_pa: f64,
    pub station: usize,
    pub n_r: u32,
    pub p_rd1_n: f64,
    pub p_rd2_n: f64,
    pub reduction: f64,
    pub p_rd_n: f64,
}

/// 📏️ One gap between two neighbouring stations of the designed span.
#[derive(Clone, Debug, PartialEq)]
pub struct Gap {
    pub before: String,
    pub after: String,
    pub after_x_m: f64,
    pub gap_m: f64,
    pub diameter_m: f64,
}

/// 🔩️ The studs of one beam, ascending in `x_m`, with the designed span they are checked against.
#[derive(Clone, Debug, PartialEq)]
pub struct BeamStuds {
    pub rows: Vec<StudResistance>,
    pub origin_m: f64,
    pub span_m: f64,
}

/// 🔩️ The `k_t` / `k_l` reduction (0 when the sheeting is out of scope, more than two studs per rib) and the resistances `P_Rd,1`, `P_Rd,2` of one stud.
pub fn resistance_of(beam: &CompositeBeam, stud: &Stud, n_r: u32, params: &AnnexParams) -> (f64, f64, f64, f64) {
    let (p1, p2, p) = cat::stud_resistance_n(stud.diameter_m, stud.height_m, stud.f_u_pa, beam.concrete_f_ck_pa, beam.concrete_e_cm_pa, params.gamma_v);
    let sh = &beam.sheeting;
    let k = cat::sheeting_reduction(sh.height_m, sh.rib_width_m, stud.height_m, stud.diameter_m, sh.thickness_m, n_r.max(1), sh.ribs_parallel_to_beam).unwrap_or(0.0);
    (p1, p2, k, p * k)
}

/// 🔩️ The studs `studs` of `beam` (rows of other beams are ignored) with their resistances.
pub fn of(beam: &CompositeBeam, studs: &[Stud], annex: AnnexChoice) -> BeamStuds {
    let params = AnnexParams::for_annex(annex);
    let layout = cat::continuous::Layout::of(cat::span_position(beam.support), beam.span_m, beam.adjacent_span_m);
    let mut own: Vec<&Stud> = studs.iter().filter(|s| s.beam_id == beam.id).collect();
    own.sort_by(|a, b| a.x_m.total_cmp(&b.x_m).then_with(|| a.id.cmp(&b.id)));
    let mut station_of = Vec::with_capacity(own.len());
    let mut sizes: Vec<u32> = Vec::new();
    let mut first_x = f64::NEG_INFINITY;
    for stud in &own {
        if sizes.is_empty() || stud.x_m - first_x > STATION_TOLERANCE_M {
            sizes.push(0);
            first_x = stud.x_m;
        }
        if let Some(last) = sizes.last_mut() {
            *last += 1;
        }
        station_of.push(sizes.len() - 1);
    }
    let rows = own
        .iter()
        .zip(&station_of)
        .map(|(stud, &station)| {
            let n_r = sizes[station];
            let (p_rd1_n, p_rd2_n, reduction, p_rd_n) = resistance_of(beam, stud, n_r, &params);
            StudResistance { id: stud.id.clone(), x_m: stud.x_m, diameter_m: stud.diameter_m, height_m: stud.height_m, f_u_pa: stud.f_u_pa, station, n_r, p_rd1_n, p_rd2_n, reduction, p_rd_n }
        })
        .collect();
    BeamStuds { rows, origin_m: layout.origin(layout.own), span_m: beam.span_m }
}

impl BeamStuds {
    /// 📍️ The middle of the designed span.
    pub fn mid_m(&self) -> f64 {
        self.origin_m + self.span_m / 2.0
    }

    /// 📍️ The end of the designed span.
    pub fn end_m(&self) -> f64 {
        self.origin_m + self.span_m
    }

    fn inside(&self, row: &StudResistance) -> bool {
        row.x_m >= self.origin_m - EDGE_EPSILON_M && row.x_m <= self.end_m() + EDGE_EPSILON_M
    }

    /// 🔩️ The studs inside the designed span.
    pub fn in_span(&self) -> impl Iterator<Item = &StudResistance> {
        self.rows.iter().filter(|r| self.inside(r))
    }

    /// 🔩️ The studs outside the designed span (they carry nothing of it).
    pub fn outside_span(&self) -> impl Iterator<Item = &StudResistance> {
        self.rows.iter().filter(|r| !self.inside(r))
    }

    /// ⚖️ The share `0`, `½` or `1` of a stud that belongs to the left half of the span (a stud on the middle belongs to both halves with half its resistance).
    fn left_weight(&self, row: &StudResistance) -> f64 {
        let d = row.x_m - self.mid_m();
        if d.abs() <= STATION_TOLERANCE_M { 0.5 } else if d < 0.0 { 1.0 } else { 0.0 }
    }

    fn weight(&self, row: &StudResistance, left: bool) -> f64 {
        if left { self.left_weight(row) } else { 1.0 - self.left_weight(row) }
    }

    /// 🔩️ The design resistance `Σ w·P_Rd` of the studs of the left (`true`) or right half of the span.
    pub fn half_resistance_n(&self, left: bool) -> f64 {
        self.in_span().map(|r| self.weight(r, left) * r.p_rd_n).sum()
    }

    /// 🔩️ The resistance of the weaker half of the span, the connection capacity `n·P_Rd` of the shear connection degree.
    pub fn capacity_n(&self) -> f64 {
        self.half_resistance_n(true).min(self.half_resistance_n(false))
    }

    /// 🔢️ The studs of the weaker half of the span (a stud on the middle counts half in each half).
    pub fn half_count(&self) -> f64 {
        let count = |left: bool| self.in_span().map(|r| self.weight(r, left)).sum::<f64>();
        count(true).min(count(false))
    }

    /// 🔩️ The mean resistance of the studs inside the span (0 without studs): the single `P_Rd` the required stud count is expressed in.
    pub fn mean_resistance_n(&self) -> f64 {
        let n = self.in_span().count();
        if n == 0 { 0.0 } else { self.in_span().map(|r| r.p_rd_n).sum::<f64>() / n as f64 }
    }

    /// 🔩️ The smallest reduction `k_t` / `k_l` of the studs inside the span (1 without studs).
    pub fn min_reduction(&self) -> f64 {
        self.in_span().map(|r| r.reduction).reduce(f64::min).unwrap_or(1.0)
    }

    /// 🔩️ The largest shank diameter inside the span (0 without studs).
    pub fn max_diameter_m(&self) -> f64 {
        self.in_span().map(|r| r.diameter_m).reduce(f64::max).unwrap_or(0.0)
    }

    /// 🔩️ The smallest overall height inside the span (0 without studs).
    pub fn min_height_m(&self) -> f64 {
        self.in_span().map(|r| r.height_m).reduce(f64::min).unwrap_or(0.0)
    }

    /// 📏️ The gaps between the neighbouring stations of the designed span.
    pub fn gaps(&self) -> Vec<Gap> {
        let mut out = Vec::new();
        let mut previous: Option<&StudResistance> = None;
        for row in self.in_span() {
            if let Some(p) = previous.filter(|p| p.station != row.station) {
                out.push(Gap { before: p.id.clone(), after: row.id.clone(), after_x_m: row.x_m, gap_m: row.x_m - p.x_m, diameter_m: p.diameter_m.max(row.diameter_m) });
            }
            previous = Some(row);
        }
        out
    }

    /// 📍️ The position of the next stud to insert: the middle of the widest gap of the weaker half, or the middle of that half when it holds no stud.
    pub fn next_position_m(&self) -> f64 {
        let left = self.half_resistance_n(true) <= self.half_resistance_n(false);
        let (lo, hi) = if left { (self.origin_m, self.mid_m()) } else { (self.mid_m(), self.end_m()) };
        let mut xs: Vec<f64> = self.in_span().filter(|r| self.weight(r, left) > 0.0).map(|r| r.x_m.clamp(lo, hi)).collect();
        xs.insert(0, lo);
        xs.push(hi);
        xs.windows(2).map(|w| (w[1] - w[0], (w[0] + w[1]) / 2.0)).fold((-1.0, (lo + hi) / 2.0), |best, g| if g.0 > best.0 { g } else { best }).1
    }

    /// 🧱️ The cells of the stations of each half of the span: a cell reaches from the middle of the gap to the previous station to the middle of the gap to the next one, the outer cells reach the support and the span middle; a station on the middle is a member of the last cell of the left and of the first cell of the right half with half its resistance.
    pub fn cells(&self) -> Vec<Cell> {
        let index: Vec<usize> = self.rows.iter().enumerate().filter(|(_, r)| self.inside(r)).map(|(i, _)| i).collect();
        let mut out = Vec::new();
        for left in [true, false] {
            let (lo, hi) = if left { (self.origin_m, self.mid_m()) } else { (self.mid_m(), self.end_m()) };
            let mut stations: Vec<Vec<(usize, usize)>> = Vec::new();
            for (k, &i) in index.iter().enumerate().filter(|(_, &i)| self.weight(&self.rows[i], left) > 0.0) {
                let same = stations.last().is_some_and(|l| self.rows[l[0].1].station == self.rows[i].station);
                match stations.last_mut() {
                    Some(last) if same => last.push((k, i)),
                    _ => stations.push(vec![(k, i)]),
                }
            }
            let xs: Vec<f64> = stations.iter().map(|s| self.rows[s[0].1].x_m.clamp(lo, hi)).collect();
            for (j, station) in stations.iter().enumerate() {
                let a = if j == 0 { lo } else { (xs[j - 1] + xs[j]) / 2.0 };
                let b = if j + 1 == xs.len() { hi } else { (xs[j] + xs[j + 1]) / 2.0 };
                let members: Vec<(usize, f64)> = station.iter().map(|&(k, i)| (k, self.weight(&self.rows[i], left))).collect();
                let resistance_n = station.iter().map(|&(_, i)| self.weight(&self.rows[i], left) * self.rows[i].p_rd_n).sum();
                out.push(Cell { left, a_m: a, b_m: b.max(a), members, resistance_n });
            }
        }
        out
    }

    /// ⚖️ The longitudinal shear force every stud inside the span carries when the shear flow is `flow(x)` [N/m]: the flow acts on the cell of each station and the station shares it between its studs in proportion to their resistance.
    pub fn forces(&self, flow: &dyn Fn(f64) -> f64) -> Vec<StudForce> {
        let inside: Vec<&StudResistance> = self.in_span().collect();
        let mut out: Vec<StudForce> = inside.iter().map(|r| StudForce { id: r.id.clone(), x_m: r.x_m, force_n: 0.0, resistance_n: r.p_rd_n }).collect();
        for cell in self.cells() {
            let force = flow(cell.mid_m()).abs() * cell.length_m();
            for &(k, w) in &cell.members {
                out[k].force_n += if cell.resistance_n > 1e-9 { force * w * inside[k].p_rd_n / cell.resistance_n } else { 0.0 };
            }
        }
        out
    }
}

/// 🧱️ The cell of one stud station in one half of the span.
#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub left: bool,
    pub a_m: f64,
    pub b_m: f64,
    pub members: Vec<(usize, f64)>,
    pub resistance_n: f64,
}

impl Cell {
    /// 📏️ The length of the cell [m].
    pub fn length_m(&self) -> f64 {
        self.b_m - self.a_m
    }

    /// 📍️ The middle of the cell.
    pub fn mid_m(&self) -> f64 {
        (self.a_m + self.b_m) / 2.0
    }

    /// 🔩️ The resistance per length `Σ w·P_Rd / length` [N/m] the cell offers to the shear flow.
    pub fn resistance_n_per_m(&self) -> f64 {
        self.resistance_n / self.length_m().max(1e-9)
    }
}

/// ⚖️ The longitudinal shear force one stud carries against its resistance.
#[derive(Clone, Debug, PartialEq)]
pub struct StudForce {
    pub id: String,
    pub x_m: f64,
    pub force_n: f64,
    pub resistance_n: f64,
}

impl StudForce {
    /// 📊️ The utilisation `P_Ed / P_Rd` (infinite for a stud without resistance that has to carry force).
    pub fn utilisation(&self) -> f64 {
        if self.resistance_n > 1e-9 { self.force_n / self.resistance_n } else if self.force_n > 0.0 { f64::INFINITY } else { 0.0 }
    }
}

/// ⚖️ The uniform plastic shear flow of ductile studs (EN 1994-1-1, 6.6.1.2 and 6.6.2.2): the required concrete force `nc_req_n` spread over the half span.
pub fn plastic_flow_n_per_m(nc_req_n: f64, span_m: f64) -> f64 {
    nc_req_n / (span_m / 2.0).max(1e-9)
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_s_artifact_norm_en1994_model::En1994Snapshot;

    fn stud(id: &str, x: f64) -> Stud {
        Stud { id: id.into(), beam_id: "beam-B1".into(), x_m: x, diameter_m: 0.019, height_m: 0.095, f_u_pa: 450e6 }
    }

    fn beam() -> CompositeBeam {
        let mut beam = En1994Snapshot::example(AnnexChoice::En).beams[0].clone();
        beam.id = "beam-B1".into();
        beam
    }

    #[test]
    fn studs_at_one_position_form_a_station_and_the_sheeting_reduction_follows_the_station_size() {
        let beam = beam();
        let studs = vec![stud("a", 0.5), stud("b", 0.5), stud("c", 1.0)];
        let rows = of(&beam, &studs, AnnexChoice::En).rows;
        assert_eq!((rows[0].n_r, rows[1].n_r, rows[2].n_r), (2, 2, 1));
        assert_eq!(rows[0].station, rows[1].station);
        assert_ne!(rows[0].station, rows[2].station);
        let params = AnnexParams::for_annex(AnnexChoice::En);
        assert!((rows[2].p_rd_n - resistance_of(&beam, &studs[2], 1, &params).3).abs() < 1e-9);
        assert!(rows[0].p_rd_n <= rows[2].p_rd_n + 1e-9);
    }

    #[test]
    fn the_forces_of_a_uniform_flow_sum_to_the_flow_times_the_span() {
        let beam = beam();
        let origin = of(&beam, &[], AnnexChoice::En).origin_m;
        let span = beam.span_m;
        let studs: Vec<Stud> = (0..10).map(|k| stud(&format!("s{k}"), origin + (k as f64 + 0.5) * span / 10.0)).collect();
        let forces = of(&beam, &studs, AnnexChoice::En).forces(&|_| 1000.0);
        let total: f64 = forces.iter().map(|f| f.force_n).sum();
        assert!((total - 1000.0 * span).abs() < 1e-6, "{total}");
        assert!(forces.iter().all(|f| (f.force_n - 1000.0 * span / 10.0).abs() < 1e-6));
    }

    #[test]
    fn a_stud_on_the_middle_belongs_to_both_halves_with_half_its_resistance() {
        let beam = beam();
        let origin = of(&beam, &[], AnnexChoice::En).origin_m;
        let span = beam.span_m;
        let studs = vec![stud("l", origin + span / 4.0), stud("m", origin + span / 2.0), stud("r", origin + 3.0 * span / 4.0)];
        let set = of(&beam, &studs, AnnexChoice::En);
        let p = set.rows[0].p_rd_n;
        assert!((set.half_resistance_n(true) - 1.5 * p).abs() < 1e-6 * p.max(1.0));
        assert!((set.capacity_n() - 1.5 * p).abs() < 1e-6 * p.max(1.0));
        assert!((set.half_count() - 1.5).abs() < 1e-12);
    }

    #[test]
    fn the_next_stud_goes_into_the_widest_gap_of_the_weaker_half() {
        let beam = beam();
        let origin = of(&beam, &[], AnnexChoice::En).origin_m;
        let span = beam.span_m;
        let studs = vec![stud("a", origin + 0.1 * span), stud("b", origin + 0.2 * span), stud("c", origin + 0.7 * span), stud("d", origin + 0.8 * span)];
        assert!((of(&beam, &studs, AnnexChoice::En).next_position_m() - (origin + 0.35 * span)).abs() < 1e-9);
        assert!((of(&beam, &[], AnnexChoice::En).next_position_m() - (origin + 0.25 * span)).abs() < 1e-9);
    }

    #[test]
    fn the_gaps_join_the_stations_of_the_span_and_the_outside_studs_carry_nothing() {
        let beam = beam();
        let origin = of(&beam, &[], AnnexChoice::En).origin_m;
        let span = beam.span_m;
        let studs = vec![stud("a", origin + 1.0), stud("b", origin + 1.0), stud("c", origin + 1.5), stud("out", origin + span + 2.0)];
        let set = of(&beam, &studs, AnnexChoice::En);
        let gaps = set.gaps();
        assert_eq!(gaps.len(), 1);
        assert!((gaps[0].gap_m - 0.5).abs() < 1e-9 && gaps[0].after == "c");
        assert_eq!(set.outside_span().count(), 1);
        assert_eq!(set.forces(&|_| 1.0).len(), 3);
    }
}
