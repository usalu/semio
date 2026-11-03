//! 🏹️ Load glyphs and supports: what is drawn for actions and boundary conditions; sizes are screen-space, magnitude only scales shaft length.

use super::*;

records! {
    /// 🧾️ Load case a glyph belongs to; colour follows `kind`.
    pub struct LoadCaseRef {
        pub id: String,
        pub kind: LoadKind,
        pub psi: Option<CombinationFactors>,
    }

    /// 🎚️ Combination factors of a variable action.
    pub struct CombinationFactors {
        pub psi0: f64,
        pub psi1: f64,
        pub psi2: f64,
    }

    /// 📊️ Load intensity at a relative position along a line.
    pub struct ProfileSample {
        pub at: f64,
        pub value: Quantity,
    }

    /// 🗾️ One pressure zone on a face with its own value and label.
    pub struct PressureZone {
        pub polygon: Polygon3,
        pub value: Quantity,
        pub label: LocalizedCopy,
    }

    /// 🔗️ A mass at a point for inertial actions.
    pub struct PointMass {
        pub at: Vec3,
        pub mass: Quantity,
    }

    /// 🦾️ Support drawn at a point in its own frame.
    pub struct SupportGeometry {
        pub kind: SupportKind,
        pub at: Vec3,
        pub frame: Frame,
    }
}

choices! {
    /// 🏷️ Kind of action, driving glyph colour.
    pub enum LoadKind {
        Permanent,
        Variable,
        Accidental,
        Seismic,
        Wind,
        Snow,
        Thermal,
        Prestress,
        Other,
    }

    /// 📏️ How a glyph shaft length follows magnitude.
    pub enum LoadScaleRule {
        LogLengthByMagnitude,
        Fixed,
    }

    /// 💥️ Shape of an explosion pressure wave.
    pub enum PressureWave {
        Spherical,
        Planar,
    }
}

variants! {
    /// 📉️ Distribution of a line load along its path.
    pub enum LoadProfile {
        Uniform { value: Quantity },
        Linear { start: Quantity, end: Quantity },
        Stepped { steps: Vec<ProfileSample> },
        Sampled { samples: Vec<ProfileSample> },
    }

    /// ⬇️ Drawn representation of one action.
    pub enum LoadGlyph {
        Point { at: Vec3, direction: Vec3, magnitude: Quantity, case: LoadCaseRef },
        Moment { at: Vec3, axis: Vec3, magnitude: Quantity, case: LoadCaseRef },
        Line { path: Path3, direction: Vec3, profile: LoadProfile, case: LoadCaseRef },
        Area { polygon: Polygon3, direction: Vec3, intensity: Quantity, case: LoadCaseRef },
        Pressure { face: EntityRef, zones: Vec<PressureZone>, case: LoadCaseRef },
        Thermal { target: EntityRef, delta_t: Quantity, case: LoadCaseRef },
        Inertial { origin: Vec3, direction: Vec3, masses: Vec<PointMass>, case: LoadCaseRef },
        Impact { at: Vec3, direction: Vec3, energy_or_force: Quantity, case: LoadCaseRef },
        Explosion { centre: Vec3, pressure: Quantity, shape: PressureWave, case: LoadCaseRef },
    }

    /// 🔧️ Boundary condition kind.
    pub enum SupportKind {
        Fixed,
        Pinned,
        Roller { normal: Vec3 },
        Spring { stiffness: Vec<Quantity> },
        ElasticBed { stiffness: Quantity, extent: Polygon3 },
        PileHead,
    }
}

impl LoadGlyph {
    pub fn case(&self) -> &LoadCaseRef {
        match self {
            Self::Point { case, .. } | Self::Moment { case, .. } | Self::Line { case, .. } | Self::Area { case, .. } | Self::Pressure { case, .. }
            | Self::Thermal { case, .. } | Self::Inertial { case, .. } | Self::Impact { case, .. } | Self::Explosion { case, .. } => case,
        }
    }
}

fn direction(out: &mut Vec<Violation>, path: &str, v: &Vec3) {
    v.check(path, out);
    expect(out, path, v.length() > 0.0, "non-zero-direction");
}

impl Validate for CombinationFactors {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        expect(out, path, [self.psi0, self.psi1, self.psi2].iter().all(|v| v.is_finite() && (0.0..=1.0).contains(v)), "unit-interval");
        expect(out, path, self.psi0 >= self.psi1 && self.psi1 >= self.psi2, "non-increasing");
    }
}

impl Validate for LoadCaseRef {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "id"), &self.id);
        self.psi.check(&at(path, "psi"), out);
    }
}

impl Validate for ProfileSample {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        expect(out, &at(path, "at"), self.at.is_finite() && (0.0..=1.0).contains(&self.at), "unit-interval");
        self.value.check(&at(path, "value"), out);
    }
}

impl Validate for PressureZone {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.polygon.check(&at(path, "polygon"), out);
        self.value.check(&at(path, "value"), out);
        self.label.check(&at(path, "label"), out);
    }
}

impl Validate for PointMass {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.at.check(&at(path, "at"), out);
        self.mass.check(&at(path, "mass"), out);
        expect(out, &at(path, "mass"), self.mass.value > 0.0, "positive");
    }
}

impl Validate for LoadProfile {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        match self {
            Self::Uniform { value } => value.check(&at(path, "value"), out),
            Self::Linear { start, end } => {
                start.check(&at(path, "start"), out);
                end.check(&at(path, "end"), out);
                expect(out, path, start.kind == end.kind, "same-kind");
            }
            Self::Stepped { steps: samples } | Self::Sampled { samples } => {
                expect(out, path, samples.len() >= 2, "at-least-2-samples");
                each(samples, path, "samples", out);
                expect(out, path, samples.windows(2).all(|w| w[0].at < w[1].at), "strictly-ascending");
            }
        }
    }
}

impl Validate for LoadGlyph {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.case().check(&at(path, "case"), out);
        match self {
            Self::Point { at: p, direction: d, magnitude, .. } | Self::Impact { at: p, direction: d, energy_or_force: magnitude, .. } => {
                p.check(&at(path, "at"), out);
                direction(out, &at(path, "direction"), d);
                magnitude.check(&at(path, "magnitude"), out);
            }
            Self::Moment { at: p, axis, magnitude, .. } => {
                p.check(&at(path, "at"), out);
                direction(out, &at(path, "axis"), axis);
                magnitude.check(&at(path, "magnitude"), out);
            }
            Self::Line { path: line, direction: d, profile, .. } => {
                line.check(&at(path, "path"), out);
                direction(out, &at(path, "direction"), d);
                profile.check(&at(path, "profile"), out);
            }
            Self::Area { polygon, direction: d, intensity, .. } => {
                polygon.check(&at(path, "polygon"), out);
                direction(out, &at(path, "direction"), d);
                intensity.check(&at(path, "intensity"), out);
            }
            Self::Pressure { face, zones, .. } => {
                face.check(&at(path, "face"), out);
                expect(out, &at(path, "zones"), !zones.is_empty(), "non-empty");
                each(zones, path, "zones", out);
            }
            Self::Thermal { target, delta_t, .. } => {
                target.check(&at(path, "target"), out);
                delta_t.check(&at(path, "deltaT"), out);
            }
            Self::Inertial { origin, direction: d, masses, .. } => {
                origin.check(&at(path, "origin"), out);
                direction(out, &at(path, "direction"), d);
                expect(out, &at(path, "masses"), !masses.is_empty(), "non-empty");
                each(masses, path, "masses", out);
            }
            Self::Explosion { centre, pressure, .. } => {
                centre.check(&at(path, "centre"), out);
                pressure.check(&at(path, "pressure"), out);
            }
        }
    }
}

impl Validate for SupportKind {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        match self {
            Self::Roller { normal } => direction(out, &at(path, "normal"), normal),
            Self::Spring { stiffness } => {
                expect(out, &at(path, "stiffness"), stiffness.len() == 6, "six-components");
                each(stiffness, path, "stiffness", out);
                expect(out, &at(path, "stiffness"), stiffness.iter().all(|q| q.value >= 0.0), "non-negative");
            }
            Self::ElasticBed { stiffness, extent } => {
                stiffness.check(&at(path, "stiffness"), out);
                extent.check(&at(path, "extent"), out);
            }
            Self::Fixed | Self::Pinned | Self::PileHead => {}
        }
    }
}

impl Validate for SupportGeometry {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.kind.check(&at(path, "kind"), out);
        self.at.check(&at(path, "at"), out);
        self.frame.check(&at(path, "frame"), out);
    }
}
