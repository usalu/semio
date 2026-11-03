//! 🚄️ Resonance of rail bridges (EN 1990 A2.4.4.2.1 with EN 1991-2 6.4): the need for a dynamic analysis from the line speed and the Fig. 6.10 frequency limits, and a deck acceleration estimate by modal superposition of the shared mechanics kernel under an HSLM-A-like train. All coefficients are unverified registry values (`ndp.en1990.a2.rail.*`); the power cars of the real HSLM-A trains are not modelled. See [`super`].

use super::*;
use semio_s_plugin_norm_mechanics::{Analysis, Control, MassType, Material, Member as BeamMember, Model, Node, Section, Support};

const ELEMENTS: usize = 16;
const MODES: usize = 3;

/// 🚆️ One HSLM-A-like universal train: `coaches` coaches of length `coach_length` joined by bogies whose two axles are `axle_spacing` apart, each axle carrying `axle_load`.
#[derive(Clone, Debug, PartialEq)]
pub struct Train {
    pub name: String,
    pub coaches: usize,
    pub coach_length: f64,
    pub axle_spacing: f64,
    pub axle_load: f64,
}

impl Train {
    /// 📖️ HSLM-A`k` (k = 1..=10) from the registry.
    pub fn hslm(annex: AnnexChoice, k: usize) -> Self {
        let read = |key: &str| ndp(annex, &format!("ndp.en1990.a2.rail.hslm.a{k}.{key}"));
        Self { name: format!("HSLM-A{k}"), coaches: read("coaches") as usize, coach_length: read("coach_length"), axle_spacing: read("axle_spacing"), axle_load: read("axle_load") }
    }

    /// 📖️ All ten trains of the registry.
    pub fn spectrum(annex: AnnexChoice) -> Vec<Self> {
        (1..=10).map(|k| Self::hslm(annex, k)).collect()
    }

    /// 📍️ Axle positions behind the first axle (m), front first.
    pub fn axles(&self) -> Vec<f64> {
        (0..=self.coaches).flat_map(|bogie| {
            let centre = bogie as f64 * self.coach_length;
            [centre - self.axle_spacing / 2.0, centre + self.axle_spacing / 2.0]
        }).map(|x| x + self.axle_spacing / 2.0).collect()
    }

    pub fn length(&self) -> f64 {
        self.axles().last().copied().unwrap_or(0.0)
    }

    /// 🔁️ Resonance speeds v_i = f·D/i (m/s) of the first `orders` orders for the natural frequency `f` (Hz).
    pub fn resonance_speeds(&self, f: f64, orders: usize) -> Vec<(usize, f64)> {
        (1..=orders).map(|order| (order, f * self.coach_length / order as f64)).collect()
    }
}

/// 📐️ Fig. 6.10 limits of the natural frequency n0 (Hz) of a simply supported span `span` (m): lower (dynamic impact) and upper (track irregularities); `None` outside the registry span range.
pub fn frequency_limits(annex: AnnexChoice, span: f64) -> Option<(f64, f64)> {
    let read = |key: &str| ndp(annex, &format!("ndp.en1990.a2.rail.{key}"));
    if !(read("f_range_min")..=read("f_range_max")).contains(&span) {
        return None;
    }
    let lower = if span <= read("f_lower_break") { read("f_lower_short") / span } else { read("f_lower_a") * span.powf(read("f_lower_b")) };
    Some((lower, read("f_upper_a") * span.powf(read("f_upper_b"))))
}

/// 🧭️ Why a dynamic analysis is (not) required.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DynamicReason {
    FastLine,
    BelowLowerLimit,
    AboveUpperLimit,
    SpanOutOfRange,
    NotRequired,
}

/// 🚦️ The dynamic-analysis flag of A2.4.4.2.1: required above the registry speed, or when n0 leaves the Fig. 6.10 band, or when the span lies outside its range.
pub fn dynamic_need(annex: AnnexChoice, span: f64, f0: f64, speed_kmh: f64) -> (bool, DynamicReason) {
    if speed_kmh > ndp(annex, "ndp.en1990.a2.rail.dynamic_speed") {
        return (true, DynamicReason::FastLine);
    }
    match frequency_limits(annex, span) {
        None => (true, DynamicReason::SpanOutOfRange),
        Some((lower, _)) if f0 < lower => (true, DynamicReason::BelowLowerLimit),
        Some((_, upper)) if f0 > upper => (true, DynamicReason::AboveUpperLimit),
        Some(_) => (false, DynamicReason::NotRequired),
    }
}

/// 🎵️ The first vertical modes of a simply supported deck from the kernel: frequencies (Hz) and unit-maximum shapes sampled at the nodes.
#[derive(Clone, Debug, PartialEq)]
pub struct DeckModes {
    pub span: f64,
    pub mass: f64,
    pub frequencies: Vec<f64>,
    pub shapes: Vec<Vec<f64>>,
}

impl DeckModes {
    /// 🧮️ The kernel modes of a deck of span `span` (m), linear mass `mass` (kg/m) whose bending stiffness is calibrated so that the fundamental frequency is `f0` (Hz); `None` when an input is not positive or the kernel rejects the model.
    pub fn compute(span: f64, mass: f64, f0: f64) -> Option<Self> {
        if span <= 0.0 || mass <= 0.0 || f0 <= 0.0 {
            return None;
        }
        let ei = mass * (std::f64::consts::TAU * f0).powi(2) * span.powi(4) / std::f64::consts::PI.powi(4);
        let (area, e) = (1.0, 1.0e10);
        let planar = [false, true, false, true, false, true];
        let supports: Vec<Support> = (0..=ELEMENTS)
            .map(|i| {
                let mut fixed = planar;
                fixed[0] = i == 0;
                fixed[2] = i == 0 || i == ELEMENTS;
                Support { node: format!("n{i}"), fixed, ..Default::default() }
            })
            .collect();
        let model = Model {
            nodes: (0..=ELEMENTS).map(|i| Node { id: format!("n{i}"), x: span * i as f64 / ELEMENTS as f64, y: 0.0, z: 0.0 }).collect(),
            materials: vec![Material { id: "d".into(), e, g: e / 16.0, rho: mass / area }],
            sections: vec![Section { id: "s".into(), a: area, iy: ei / e, iz: ei / e, j: ei / e, iw: 0.0, asy: 0.0, asz: 0.0 }],
            members: (0..ELEMENTS).map(|i| BeamMember { id: format!("e{i}"), node_i: format!("n{i}"), node_j: format!("n{}", i + 1), material: "d".into(), section: "s".into(), ..Default::default() }).collect(),
            supports,
            self_mass: true,
            ..Default::default()
        };
        let control = Control::none();
        let analysis = Analysis::new(&model, &control).ok()?;
        let result = analysis.modal(MODES, MassType::Consistent, &control).ok()?;
        let mut frequencies = Vec::new();
        let mut shapes = Vec::new();
        for mode in result.modes {
            let uz = |i: usize| mode.shape.iter().find(|d| d.node == format!("n{i}")).map_or(0.0, |d| d.u[2]);
            let top = (0..=ELEMENTS).map(|i| uz(i).abs()).fold(0.0, f64::max);
            if top <= 0.0 {
                return None;
            }
            let sign = (0..=ELEMENTS).map(uz).max_by(|a, b| a.abs().total_cmp(&b.abs())).map_or(1.0, f64::signum);
            frequencies.push(mode.frequency_hz);
            shapes.push((0..=ELEMENTS).map(|i| sign * uz(i) / top).collect());
        }
        (!frequencies.is_empty()).then_some(Self { span, mass, frequencies, shapes })
    }

    fn shape_at(&self, mode: usize, x: f64) -> f64 {
        let t = (x / self.span).clamp(0.0, 1.0) * ELEMENTS as f64;
        let i = (t.floor() as usize).min(ELEMENTS - 1);
        let w = t - i as f64;
        self.shapes[mode][i] * (1.0 - w) + self.shapes[mode][i + 1] * w
    }

    fn generalized_mass(&self, mode: usize) -> f64 {
        let h = self.span / ELEMENTS as f64;
        let squares: Vec<f64> = self.shapes[mode].iter().map(|v| v * v).collect();
        self.mass * h * (squares.iter().sum::<f64>() - 0.5 * (squares[0] + squares[ELEMENTS]))
    }

    /// 📈️ The largest mid-span acceleration (m/s²) while `train` crosses at `speed` (m/s): modal superposition of the kernel modes with damping ratio `zeta`, Newmark average-acceleration integration, followed by one second of free decay.
    pub fn peak_acceleration(&self, train: &Train, speed: f64, zeta: f64) -> f64 {
        if speed <= 0.0 {
            return 0.0;
        }
        let axles = train.axles();
        let duration = (self.span + train.length()) / speed + 1.0;
        let highest = self.frequencies.iter().copied().fold(0.0, f64::max);
        let dt = (1.0 / (20.0 * highest)).clamp(1.0e-4, 5.0e-3);
        let steps = (duration / dt).ceil() as usize;
        let mut peak = 0.0f64;
        let mut state: Vec<(f64, f64, f64)> = vec![(0.0, 0.0, 0.0); self.frequencies.len()];
        let masses: Vec<f64> = (0..self.frequencies.len()).map(|n| self.generalized_mass(n)).collect();
        let force = |mode: usize, time: f64| axles.iter().map(|offset| speed * time - offset).filter(|x| (0.0..=self.span).contains(x)).map(|x| train.axle_load * self.shape_at(mode, x)).sum::<f64>() / masses[mode];
        for (n, s) in state.iter_mut().enumerate() {
            s.2 = force(n, 0.0);
        }
        for step in 1..=steps {
            let time = step as f64 * dt;
            let mut total = 0.0;
            for (n, s) in state.iter_mut().enumerate() {
                let omega = std::f64::consts::TAU * self.frequencies[n];
                let c = 2.0 * zeta * omega;
                let (q, v, a) = *s;
                let f = force(n, time);
                let stiffness = omega * omega + 2.0 * c / dt + 4.0 / (dt * dt);
                let load = f + 4.0 * q / (dt * dt) + 4.0 * v / dt + a + c * (2.0 * q / dt + v);
                let q1 = load / stiffness;
                let v1 = 2.0 * (q1 - q) / dt - v;
                let a1 = 4.0 * (q1 - q) / (dt * dt) - 4.0 * v / dt - a;
                *s = (q1, v1, a1);
                total += self.shape_at(n, self.span / 2.0) * a1;
            }
            peak = peak.max(total.abs());
        }
        peak
    }
}

/// 🏁️ The governing result of the HSLM-like spectrum: train, speed (m/s) and peak acceleration.
#[derive(Clone, Debug, PartialEq)]
pub struct Governing {
    pub train: String,
    pub speed: f64,
    pub acceleration: f64,
}

/// 🏁️ The largest peak acceleration over all trains at their resonance speeds (orders 1..=3) inside `[0.4·v, v]` with `v` the verification speed (m/s) and at `v` itself.
pub fn governing_acceleration(modes: &DeckModes, trains: &[Train], verification_speed: f64, zeta: f64) -> Option<Governing> {
    let mut best: Option<Governing> = None;
    for train in trains {
        let mut speeds: Vec<f64> = train.resonance_speeds(modes.frequencies[0], 3).into_iter().map(|(_, v)| v).filter(|v| (0.4 * verification_speed..=verification_speed).contains(v)).collect();
        speeds.push(verification_speed);
        for speed in speeds {
            let acceleration = modes.peak_acceleration(train, speed, zeta);
            if best.as_ref().is_none_or(|g| acceleration > g.acceleration) {
                best = Some(Governing { train: train.name.clone(), speed, acceleration });
            }
        }
    }
    best
}
